use super::model::HistoryPoint;
use std::fmt::Write;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalizedPoint {
    pub x: f32,
    pub y: Option<f32>,
}

pub fn normalize_history(
    history: &[HistoryPoint],
    window_seconds: u8,
    max_value: f32,
) -> Vec<NormalizedPoint> {
    let window = window_seconds as f32;
    history
        .iter()
        .map(|point| NormalizedPoint {
            x: if window > 0.0 {
                (1.0 - point.seconds_ago as f32 / window).clamp(0.0, 1.0)
            } else {
                1.0
            },
            y: point.value.and_then(|value| {
                (max_value.is_finite() && max_value > 0.0 && value.is_finite())
                    .then(|| 1.0 - value.clamp(0.0, max_value) / max_value)
            }),
        })
        .collect()
}

pub fn network_history_max_value(received: &[HistoryPoint], sent: &[HistoryPoint]) -> f32 {
    let maximum = received
        .iter()
        .chain(sent)
        .filter_map(|point| point.value)
        .filter(|value| value.is_finite())
        .fold(0.0_f32, f32::max);
    (maximum * 1.1).max(1.0)
}

pub fn line_path_commands(points: &[NormalizedPoint]) -> String {
    let mut commands = String::new();
    let mut segment = Vec::new();
    for point in points {
        let Some(y) = point.y else {
            append_line_segment(&mut commands, &segment);
            segment.clear();
            continue;
        };
        segment.push((point.x, y));
    }
    append_line_segment(&mut commands, &segment);
    commands
}

fn append_line_segment(commands: &mut String, segment: &[(f32, f32)]) {
    let Some(&(first_x, first_y)) = segment.first() else {
        return;
    };
    if !commands.is_empty() {
        commands.push(' ');
    }
    let _ = write!(commands, "M {:.6} {:.6}", first_x, first_y);
    for (x, y) in &segment[1..] {
        let _ = write!(commands, " L {:.6} {:.6}", x, y);
    }
    if segment.len() == 1 {
        let marker_end = if first_x >= 0.992 {
            first_x - 0.008
        } else {
            first_x + 0.008
        };
        let _ = write!(commands, " L {:.6} {:.6}", marker_end, first_y);
    }
}

pub fn area_path_commands(points: &[NormalizedPoint]) -> String {
    let mut commands = String::new();
    let mut segment = Vec::new();

    for point in points {
        if let Some(y) = point.y {
            segment.push((point.x, y));
        } else {
            append_area_segment(&mut commands, &segment);
            segment.clear();
        }
    }
    append_area_segment(&mut commands, &segment);
    commands
}

fn append_area_segment(commands: &mut String, segment: &[(f32, f32)]) {
    if segment.len() < 2 {
        return;
    }
    let (first_x, first_y) = segment[0];
    let (last_x, _) = segment[segment.len() - 1];
    let _ = write!(commands, "M {:.6} {:.6}", first_x, first_y);
    for (x, y) in &segment[1..] {
        let _ = write!(commands, " L {:.6} {:.6}", x, y);
    }
    let _ = write!(
        commands,
        " L {:.6} 1.000000 L {:.6} 1.000000 Z ",
        last_x, first_x
    );
}

#[cfg(test)]
mod tests {
    use super::{
        area_path_commands, line_path_commands, network_history_max_value, normalize_history,
        NormalizedPoint,
    };
    use crate::preview::HistoryPoint;

    #[test]
    fn empty_history_produces_no_coordinates() {
        let points = normalize_history(&[], 60, 100.0);
        assert!(points.is_empty());
        assert!(line_path_commands(&points).is_empty());
        assert!(area_path_commands(&points).is_empty());
    }

    #[test]
    fn one_sample_is_placed_at_now_and_values_are_clamped() {
        let points = normalize_history(
            &[
                HistoryPoint {
                    seconds_ago: 0,
                    value: Some(50.0),
                },
                HistoryPoint {
                    seconds_ago: 0,
                    value: Some(120.0),
                },
            ],
            60,
            100.0,
        );
        assert_eq!(points[0].x, 1.0);
        assert_eq!(points[0].y, Some(0.5));
        assert_eq!(points[1].y, Some(0.0));
    }

    #[test]
    fn history_runs_from_oldest_left_to_now_right_and_preserves_gaps() {
        let points = normalize_history(
            &[
                HistoryPoint {
                    seconds_ago: 60,
                    value: Some(0.0),
                },
                HistoryPoint {
                    seconds_ago: 30,
                    value: None,
                },
                HistoryPoint {
                    seconds_ago: 0,
                    value: Some(100.0),
                },
            ],
            60,
            100.0,
        );
        assert_eq!(points[0].x, 0.0);
        assert_eq!(points[1].x, 0.5);
        assert_eq!(points[1].y, None);
        assert_eq!(points[2].x, 1.0);
        assert_eq!(points[2].y, Some(0.0));
    }

    #[test]
    fn zero_or_non_finite_scale_never_divides_by_zero() {
        for max_value in [0.0, f32::INFINITY, f32::NAN] {
            let points = normalize_history(
                &[HistoryPoint {
                    seconds_ago: 0,
                    value: Some(50.0),
                }],
                0,
                max_value,
            );
            assert_eq!(points[0].x, 1.0);
            assert_eq!(points[0].y, None);
        }
    }

    #[test]
    fn memory_history_uses_total_capacity_as_its_vertical_scale() {
        let points = normalize_history(
            &[
                HistoryPoint {
                    seconds_ago: 60,
                    value: Some(0.0),
                },
                HistoryPoint {
                    seconds_ago: 30,
                    value: Some(8.0),
                },
                HistoryPoint {
                    seconds_ago: 0,
                    value: Some(16.0),
                },
            ],
            60,
            16.0,
        );
        assert_eq!(
            points.iter().map(|point| point.y).collect::<Vec<_>>(),
            vec![Some(1.0), Some(0.5), Some(0.0)]
        );
    }

    #[test]
    fn gpu_history_uses_the_fixed_utilization_percentage_scale() {
        let gpu = crate::preview::generate_gpu(30);
        let points = normalize_history(&gpu.history, 60, 100.0);
        assert_eq!(points.len(), 61);
        assert_eq!(points.first().unwrap().x, 0.0);
        assert_eq!(points.last().unwrap().x, 1.0);
        assert!(points.iter().all(|point| {
            point
                .y
                .is_some_and(|normalized| (0.0..=1.0).contains(&normalized))
        }));
    }

    #[test]
    fn network_history_scale_adds_headroom_and_has_a_minimum() {
        let received = [HistoryPoint {
            seconds_ago: 0,
            value: Some(10.0),
        }];
        let sent = [HistoryPoint {
            seconds_ago: 0,
            value: Some(20.0),
        }];
        assert!((network_history_max_value(&received, &sent) - 22.0).abs() < f32::EPSILON);
        assert_eq!(network_history_max_value(&[], &[]), 1.0);
        assert_eq!(
            network_history_max_value(
                &[HistoryPoint {
                    seconds_ago: 0,
                    value: None,
                }],
                &[]
            ),
            1.0
        );
    }

    #[test]
    fn line_path_starts_a_new_subpath_after_missing_data() {
        let commands = line_path_commands(&[
            NormalizedPoint {
                x: 0.0,
                y: Some(0.25),
            },
            NormalizedPoint { x: 0.5, y: None },
            NormalizedPoint {
                x: 1.0,
                y: Some(0.75),
            },
        ]);
        assert_eq!(
            commands,
            "M 0.000000 0.250000 L 0.008000 0.250000 M 1.000000 0.750000 L 0.992000 0.750000"
        );
    }

    #[test]
    fn single_history_sample_is_rendered_as_a_small_visible_mark() {
        let commands = line_path_commands(&[NormalizedPoint {
            x: 1.0,
            y: Some(0.4),
        }]);
        assert_eq!(commands, "M 1.000000 0.400000 L 0.992000 0.400000");
        assert!(area_path_commands(&[NormalizedPoint {
            x: 1.0,
            y: Some(0.4),
        }])
        .is_empty());
    }

    #[test]
    fn area_path_closes_only_contiguous_segments_with_two_or_more_points() {
        let commands = area_path_commands(&[
            NormalizedPoint {
                x: 0.0,
                y: Some(0.25),
            },
            NormalizedPoint {
                x: 0.25,
                y: Some(0.5),
            },
            NormalizedPoint { x: 0.5, y: None },
            NormalizedPoint {
                x: 0.75,
                y: Some(0.5),
            },
            NormalizedPoint {
                x: 1.0,
                y: Some(0.25),
            },
        ]);
        assert_eq!(commands.matches('Z').count(), 2);
        assert!(area_path_commands(&[NormalizedPoint {
            x: 1.0,
            y: Some(0.5),
        }])
        .is_empty());
    }
}
