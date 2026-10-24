// 3D Terrain Heightmap Mesh & Viridis Color Mapping

pub const TIME_WINDOW: usize = 30;
use std::collections::VecDeque;
use std::time::Instant;

pub struct TerrainMesh {
    pub grid: Vec<Vec<f32>>,
    pub render_grid: Vec<Vec<f32>>,
    sample_times: VecDeque<Instant>,
    core_count: usize,
}

impl TerrainMesh {
    pub fn new() -> Self {
        Self {
            grid: Vec::new(),
            render_grid: Vec::new(),
            sample_times: VecDeque::new(),
            core_count: 0,
        }
    }

    pub fn push_sample(&mut self, cores: &[f32]) {
        if cores.len() != self.core_count {
            self.core_count = cores.len();
            self.grid.clear();
            self.render_grid.clear();
            self.sample_times.clear();
        }
        let new_row = cores
            .iter()
            .map(|value| value.clamp(0.0, 100.0))
            .collect::<Vec<_>>();
        let render_row = self
            .render_grid
            .last()
            .cloned()
            .unwrap_or_else(|| new_row.clone());
        self.grid.push(new_row);
        self.render_grid.push(render_row);
        self.sample_times.push_back(Instant::now());
        if self.grid.len() > TIME_WINDOW {
            self.grid.remove(0);
            self.render_grid.remove(0);
            self.sample_times.pop_front();
        }
    }

    pub fn update_interpolation(&mut self, alpha: f32) -> bool {
        let mut moving = false;
        for (target_row, render_row) in self.grid.iter().zip(&mut self.render_grid) {
            for (target, current) in target_row.iter().zip(render_row) {
                let delta = *target - *current;
                if delta.abs() > 0.1 {
                    *current += alpha.clamp(0.0, 1.0) * delta;
                    moving = true;
                } else {
                    *current = *target;
                }
            }
        }
        moving
    }

    pub fn sample_count(&self) -> usize {
        self.grid.len()
    }

    pub fn history_seconds(&self) -> u64 {
        self.sample_times
            .front()
            .zip(self.sample_times.back())
            .map(|(first, last)| last.duration_since(*first).as_secs())
            .unwrap_or_default()
    }

    /// Creates a denser display-only grid. Recorded telemetry remains in `grid`;
    /// Catmull-Rom interpolation is used only to soften the rendered surface.
    pub fn smoothed_grid(&self, subdivisions: usize) -> Vec<Vec<f32>> {
        let rows = &self.render_grid;
        let row_count = rows.len();
        let Some(first_row) = rows.first() else {
            return Vec::new();
        };
        let column_count = first_row.len();
        if column_count == 0 {
            return vec![Vec::new(); row_count];
        }

        let steps = subdivisions.max(1);
        let output_rows = if row_count > 1 {
            (row_count - 1) * steps + 1
        } else {
            1
        };
        let output_columns = if column_count > 1 {
            (column_count - 1) * steps + 1
        } else {
            1
        };

        (0..output_rows)
            .map(|row_index| {
                let row_segment = row_index / steps;
                let row_t = if row_count > 1 && row_index < output_rows - 1 {
                    (row_index % steps) as f32 / steps as f32
                } else {
                    0.0
                };
                (0..output_columns)
                    .map(|column_index| {
                        let column_segment = column_index / steps;
                        let column_t = if column_count > 1 && column_index < output_columns - 1 {
                            (column_index % steps) as f32 / steps as f32
                        } else {
                            0.0
                        };
                        let sample_row = |offset: isize| {
                            (row_segment as isize + offset).clamp(0, row_count as isize - 1)
                                as usize
                        };
                        let sample_column = |offset: isize| {
                            (column_segment as isize + offset).clamp(0, column_count as isize - 1)
                                as usize
                        };
                        let row_values = [-1, 0, 1, 2].map(|offset| {
                            let source_row = &rows[sample_row(offset)];
                            let value = if column_count == 1 {
                                source_row[0]
                            } else {
                                catmull_rom(
                                    source_row[sample_column(-1)],
                                    source_row[sample_column(0)],
                                    source_row[sample_column(1)],
                                    source_row[sample_column(2)],
                                    column_t,
                                )
                            };
                            value
                        });
                        if row_count == 1 {
                            row_values[1].clamp(0.0, 100.0)
                        } else {
                            catmull_rom(
                                row_values[0],
                                row_values[1],
                                row_values[2],
                                row_values[3],
                                row_t,
                            )
                            .clamp(0.0, 100.0)
                        }
                    })
                    .collect()
            })
            .collect()
    }
}

fn catmull_rom(p0: f32, p1: f32, p2: f32, p3: f32, t: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;
    0.5 * ((2.0 * p1)
        + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
}

impl Default for TerrainMesh {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_tracks_arbitrary_core_counts_and_keeps_last_thirty_samples() {
        for count in [0, 1, 4, 16, 32, 64] {
            let mut mesh = TerrainMesh::new();
            mesh.push_sample(&vec![42.0; count]);
            assert_eq!(mesh.grid[0].len(), count);
            assert_eq!(mesh.sample_count(), 1);
            for _ in 1..35 {
                mesh.push_sample(&vec![42.0; count]);
            }
            assert_eq!(mesh.sample_count(), TIME_WINDOW);
            assert_eq!(mesh.grid.last().unwrap().len(), count);
        }
    }

    #[test]
    fn history_starts_empty_and_only_contains_real_samples() {
        let mut mesh = TerrainMesh::new();
        assert_eq!(mesh.sample_count(), 0);
        assert!(mesh.grid.is_empty());
        mesh.push_sample(&[12.5, 75.0]);
        assert_eq!(mesh.grid, vec![vec![12.5, 75.0]]);
        assert_eq!(mesh.render_grid, mesh.grid);
    }

    #[test]
    fn changing_core_count_resets_history_and_interpolation_converges() {
        let mut mesh = TerrainMesh::new();
        mesh.push_sample(&[0.0; 16]);
        mesh.push_sample(&[100.0; 16]);
        assert!(mesh.update_interpolation(0.5));
        mesh.push_sample(&[50.0; 32]);
        assert_eq!(mesh.sample_count(), 1);
        assert_eq!(mesh.render_grid[0].len(), 32);
        for _ in 0..20 {
            mesh.update_interpolation(0.5);
        }
        assert!(!mesh.update_interpolation(0.5));
    }

    #[test]
    fn viridis_palette_uses_shared_anchor_colors() {
        assert_eq!(viridis_color(0.0), [68, 1, 84, 255]);
        assert_eq!(viridis_color(0.25), [59, 82, 139, 255]);
        assert_eq!(viridis_color(0.5), [33, 145, 140, 255]);
        assert_eq!(viridis_color(0.75), [94, 201, 98, 255]);
        assert_eq!(viridis_color(1.0), [253, 231, 37, 255]);
    }

    #[test]
    fn smoothed_display_grid_preserves_samples_and_bounds_interpolation() {
        let mut mesh = TerrainMesh::new();
        mesh.push_sample(&[0.0, 100.0]);
        mesh.push_sample(&[100.0, 0.0]);
        mesh.update_interpolation(1.0);
        let dense = mesh.smoothed_grid(3);
        assert_eq!(dense.len(), 4);
        assert_eq!(dense[0].len(), 4);
        assert_eq!(dense[0][0], 0.0);
        assert_eq!(dense[0][3], 100.0);
        assert_eq!(dense[3][0], 100.0);
        assert_eq!(dense[3][3], 0.0);
        assert!(dense
            .iter()
            .flatten()
            .all(|value| (0.0..=100.0).contains(value)));
        assert_eq!(mesh.grid, vec![vec![0.0, 100.0], vec![100.0, 0.0]]);
    }

    #[test]
    fn smoothed_grid_handles_empty_and_single_dimension_history() {
        let empty = TerrainMesh::new();
        assert!(empty.smoothed_grid(3).is_empty());
        let mut one = TerrainMesh::new();
        one.push_sample(&[42.0]);
        assert_eq!(one.smoothed_grid(3), vec![vec![42.0]]);
        one.push_sample(&[64.0]);
        assert_eq!(one.smoothed_grid(3).len(), 4);
        assert!(one.smoothed_grid(3).iter().all(|row| row.len() == 1));
    }

    #[test]
    fn invalid_loads_are_clamped_without_creating_extra_samples() {
        let mut mesh = TerrainMesh::new();
        assert_eq!(mesh.sample_count(), 0);
        assert!(mesh.smoothed_grid(4).is_empty());
        mesh.push_sample(&[-10.0, 140.0]);
        assert_eq!(mesh.sample_count(), 1);
        assert_eq!(mesh.grid, vec![vec![0.0, 100.0]]);
        let display = mesh.smoothed_grid(4);
        assert_eq!(display, vec![vec![0.0, 100.0]]);
    }
}

// Convert height factor (0.0 - 1.0) to Viridis Palette [R, G, B, A]
pub fn viridis_color(t: f32) -> [u8; 4] {
    let t = t.clamp(0.0, 1.0);
    // Key color anchors of the Viridis palette:
    // 0.0 -> Dark Purple (68, 1, 84)
    // 0.25 -> Deep Blue (59, 82, 139)
    // 0.50 -> Viridis Teal (33, 145, 140)
    // 0.75 -> Light Green (94, 201, 98)
    // 1.00 -> Bright Yellow (253, 231, 37)
    let (r, g, b) = if t < 0.25 {
        let f = t / 0.25;
        (
            68.0 + f * (59.0 - 68.0),
            1.0 + f * (82.0 - 1.0),
            84.0 + f * (139.0 - 84.0),
        )
    } else if t < 0.50 {
        let f = (t - 0.25) / 0.25;
        (
            59.0 + f * (33.0 - 59.0),
            82.0 + f * (145.0 - 82.0),
            139.0 + f * (140.0 - 139.0),
        )
    } else if t < 0.75 {
        let f = (t - 0.50) / 0.25;
        (
            33.0 + f * (94.0 - 33.0),
            145.0 + f * (201.0 - 145.0),
            140.0 + f * (98.0 - 140.0),
        )
    } else {
        let f = (t - 0.75) / 0.25;
        (
            94.0 + f * (253.0 - 94.0),
            201.0 + f * (231.0 - 201.0),
            98.0 + f * (37.0 - 98.0),
        )
    };

    [r as u8, g as u8, b as u8, 255]
}
