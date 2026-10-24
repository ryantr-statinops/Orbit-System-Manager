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
