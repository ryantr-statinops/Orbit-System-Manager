// 3D Terrain Heightmap Mesh & Viridis Color Mapping

pub const CORE_COUNT: usize = 16;
pub const TIME_WINDOW: usize = 30;

pub struct TerrainMesh {
    // 30 time slices x 16 cores (0.0 to 100.0)
    pub grid: [[f32; CORE_COUNT]; TIME_WINDOW],
    pub render_grid: [[f32; CORE_COUNT]; TIME_WINDOW],
}

impl TerrainMesh {
    pub fn new() -> Self {
        let mut mesh = Self {
            grid: [[0.0; CORE_COUNT]; TIME_WINDOW],
            render_grid: [[0.0; CORE_COUNT]; TIME_WINDOW],
        };

        // Initialize with default subtle wave for preview
        for (z, row) in mesh.grid.iter_mut().enumerate() {
            for (x, val) in row.iter_mut().enumerate() {
                let wave = ((x as f32 * 0.4).sin() + (z as f32 * 0.3).cos()) * 15.0 + 25.0;
                *val = wave.clamp(0.0, 100.0);
            }
        }
        mesh.render_grid = mesh.grid;
        mesh
    }

    pub fn push_sample(&mut self, cores: &[f32]) {
        // Shift time slices backward
        for i in 0..TIME_WINDOW - 1 {
            self.grid[i] = self.grid[i + 1];
        }

        // Push new cores row at latest time slice
        let mut new_row = [0.0; CORE_COUNT];
        for i in 0..CORE_COUNT {
            if i < cores.len() {
                new_row[i] = cores[i].clamp(0.0, 100.0);
            }
        }
        self.grid[TIME_WINDOW - 1] = new_row;
    }

    pub fn update_interpolation(&mut self, alpha: f32) {
        // Smooth lerp towards target grid
        for z in 0..TIME_WINDOW {
            for x in 0..CORE_COUNT {
                let target = self.grid[z][x];
                let current = self.render_grid[z][x];
                self.render_grid[z][x] = current + alpha * (target - current);
            }
        }
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
