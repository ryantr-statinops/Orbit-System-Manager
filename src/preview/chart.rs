use super::model::HistoryPoint;

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

#[cfg(test)]
mod tests {
    use super::normalize_history;
    use crate::preview::HistoryPoint;

    #[test]
    fn empty_history_produces_no_coordinates() {
        assert!(normalize_history(&[], 60, 100.0).is_empty());
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
}
