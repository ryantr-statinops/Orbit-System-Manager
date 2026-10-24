pub mod camera;
pub mod mesh;

use camera::OrbitCamera;
use mesh::{viridis_color, TerrainMesh, CORE_COUNT, TIME_WINDOW};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};

pub struct OffscreenRenderer {
    pub camera: OrbitCamera,
    pub mesh: TerrainMesh,
    width: u32,
    height: u32,
    buffer: SharedPixelBuffer<Rgba8Pixel>,
}

impl OffscreenRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        let buffer = SharedPixelBuffer::new(width, height);
        Self {
            camera: OrbitCamera::new(),
            mesh: TerrainMesh::new(),
            width,
            height,
            buffer,
        }
    }

    pub fn render_frame(&mut self) -> Image {
        let w = self.width;
        let h = self.height;
        let pixels = self.buffer.make_mut_slice();

        // 1. Clear background to Dark Slate (#090d13)
        for pixel in pixels.iter_mut() {
            *pixel = Rgba8Pixel {
                r: 9,
                g: 13,
                b: 19,
                a: 255,
            };
        }

        // 2. Project all 16x30 3D vertices into 2D screen space
        let mut projected: [[Option<(i32, i32)>; CORE_COUNT]; TIME_WINDOW] =
            [[None; CORE_COUNT]; TIME_WINDOW];

        let span_x = 34.0;
        let span_z = 38.0;

        for z in 0..TIME_WINDOW {
            for x in 0..CORE_COUNT {
                let wx = ((x as f32 / (CORE_COUNT - 1) as f32) - 0.5) * span_x;
                let wz = ((z as f32 / (TIME_WINDOW - 1) as f32) - 0.5) * span_z;
                let wy = (self.mesh.render_grid[z][x] / 100.0) * 12.0;

                if let Some((sx, sy, _depth)) = self.camera.project(wx, wy, wz, w, h) {
                    projected[z][x] = Some((sx, sy));
                }
            }
        }

        // 3. Draw Z-lines (time-series ribbon lines)
        for x in 0..CORE_COUNT {
            for z in 0..TIME_WINDOW - 1 {
                if let (Some(p1), Some(p2)) = (projected[z][x], projected[z + 1][x]) {
                    let avg_load = (self.mesh.render_grid[z][x] + self.mesh.render_grid[z + 1][x])
                        * 0.5
                        / 100.0;
                    let color = viridis_color(avg_load);
                    draw_line(pixels, w, h, p1.0, p1.1, p2.0, p2.1, color);
                }
            }
        }

        // 4. Draw X-lines (cross-core ribbons)
        for z in 0..TIME_WINDOW {
            for x in 0..CORE_COUNT - 1 {
                if let (Some(p1), Some(p2)) = (projected[z][x], projected[z][x + 1]) {
                    let avg_load = (self.mesh.render_grid[z][x] + self.mesh.render_grid[z][x + 1])
                        * 0.5
                        / 100.0;
                    let color = viridis_color(avg_load);
                    draw_line(pixels, w, h, p1.0, p1.1, p2.0, p2.1, color);
                }
            }
        }

        Image::from_rgba8(self.buffer.clone())
    }
}

// Fast integer Bresenham line drawer with boundary clipping
fn draw_line(
    pixels: &mut [Rgba8Pixel],
    w: u32,
    h: u32,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    color: [u8; 4],
) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let w_i32 = w as i32;
    let h_i32 = h as i32;

    loop {
        if x0 >= 0 && x0 < w_i32 && y0 >= 0 && y0 < h_i32 {
            let idx = (y0 as u32 * w + x0 as u32) as usize;
            if idx < pixels.len() {
                pixels[idx] = Rgba8Pixel {
                    r: color[0],
                    g: color[1],
                    b: color[2],
                    a: color[3],
                };
            }
        }

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}
