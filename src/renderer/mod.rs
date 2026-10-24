pub mod camera;
pub mod mesh;

use camera::OrbitCamera;
use mesh::{viridis_color, TerrainMesh};
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
        let width = width.clamp(1, 1920);
        let height = height.clamp(1, 1920);
        let buffer = SharedPixelBuffer::new(width, height);
        Self {
            camera: OrbitCamera::new(),
            mesh: TerrainMesh::new(),
            width,
            height,
            buffer,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let width = width.clamp(1, 1920);
        let height = height.clamp(1, 1920);
        if (width, height) != (self.width, self.height) {
            self.width = width;
            self.height = height;
            self.buffer = SharedPixelBuffer::new(width, height);
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

        let row_count = self.mesh.render_grid.len();
        let core_count = self.mesh.render_grid.first().map_or(0, Vec::len);
        if row_count == 0 || core_count == 0 {
            return Image::from_rgba8(self.buffer.clone());
        }
        let mut projected = vec![vec![None; core_count]; row_count];

        let span_x = 34.0;
        let span_z = 38.0;

        for (z, row) in projected.iter_mut().enumerate() {
            for (x, point) in row.iter_mut().enumerate() {
                let wx = ((x as f32 / core_count.saturating_sub(1).max(1) as f32) - 0.5) * span_x;
                let wz = ((z as f32 / row_count.saturating_sub(1).max(1) as f32) - 0.5) * span_z;
                let wy = (self.mesh.render_grid[z][x] / 100.0) * 12.0;

                if let Some((sx, sy, _depth)) = self.camera.project(wx, wy, wz, w, h) {
                    *point = Some((sx, sy));
                }
            }
        }

        // 3. Draw Z-lines (time-series ribbon lines)
        #[allow(clippy::needless_range_loop)]
        for x in 0..core_count {
            for z in 0..row_count.saturating_sub(1) {
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
        #[allow(clippy::needless_range_loop)]
        for z in 0..row_count {
            for x in 0..core_count.saturating_sub(1) {
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
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: [u8; 4],
) {
    let Some((x0, y0, x1, y1)) = clip_line(x0, y0, x1, y1, w as i32, h as i32) else {
        return;
    };
    let (mut x0, mut y0) = (x0, y0);
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let w_i32 = w as i32;
    let h_i32 = h as i32;

    loop {
        if (0..w_i32).contains(&x0) && (0..h_i32).contains(&y0) {
            let idx = (y0 as u32 * w + x0 as u32) as usize;
            pixels[idx] = Rgba8Pixel {
                r: color[0],
                g: color[1],
                b: color[2],
                a: color[3],
            };
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

fn clip_line(
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    width: i32,
    height: i32,
) -> Option<(i32, i32, i32, i32)> {
    if width <= 0 || height <= 0 {
        return None;
    }
    let (x0f, y0f, x1f, y1f) = (x0 as f64, y0 as f64, x1 as f64, y1 as f64);
    let (dx, dy) = (x1f - x0f, y1f - y0f);
    let (mut t0, mut t1): (f64, f64) = (0.0, 1.0);
    for (p, q) in [
        (-dx, x0f),
        (dx, width as f64 - 1.0 - x0f),
        (-dy, y0f),
        (dy, height as f64 - 1.0 - y0f),
    ] {
        if p == 0.0 {
            if q < 0.0 {
                return None;
            }
        } else {
            let r = q / p;
            if p < 0.0 {
                t0 = t0.max(r);
            } else {
                t1 = t1.min(r);
            }
            if t0 > t1 {
                return None;
            }
        }
    }
    Some((
        (x0f + t0 * dx).round() as i32,
        (y0f + t0 * dy).round() as i32,
        (x0f + t1 * dx).round() as i32,
        (y0f + t1 * dy).round() as i32,
    ))
}

#[cfg(test)]
mod tests {
    use super::clip_line;

    #[test]
    fn clips_inside_outside_crossing_and_zero_length_lines() {
        assert_eq!(clip_line(2, 2, 5, 5, 10, 10), Some((2, 2, 5, 5)));
        assert_eq!(clip_line(-5, -5, -1, -1, 10, 10), None);
        assert_eq!(clip_line(-5, 5, 15, 5, 10, 10), Some((0, 5, 9, 5)));
        assert_eq!(clip_line(4, 4, 4, 4, 10, 10), Some((4, 4, 4, 4)));
    }
}
