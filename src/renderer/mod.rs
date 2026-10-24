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

        let render_grid = self.mesh.smoothed_grid(3);
        let row_count = render_grid.len();
        let core_count = render_grid.first().map_or(0, Vec::len);
        if row_count == 0 || core_count == 0 {
            return Image::from_rgba8(self.buffer.clone());
        }
        let mut projected = vec![vec![None; core_count]; row_count];

        let aspect = w as f32 / h.max(1) as f32;
        let span_x = 34.0 * aspect.sqrt().clamp(1.0, 1.8);
        let span_z = 38.0;

        for (z, row) in projected.iter_mut().enumerate() {
            for (x, point) in row.iter_mut().enumerate() {
                let wx = ((x as f32 / core_count.saturating_sub(1).max(1) as f32) - 0.5) * span_x;
                let wz = ((z as f32 / row_count.saturating_sub(1).max(1) as f32) - 0.5) * span_z;
                let wy = (render_grid[z][x] / 100.0) * 12.0;

                if let Some((sx, sy, depth)) = self.camera.project(wx, wy, wz, w, h) {
                    *point = Some((sx, sy, depth));
                }
            }
        }

        let mut depth_buffer = vec![f32::INFINITY; pixels.len()];
        if row_count > 1 && core_count > 1 {
            for row in 0..row_count - 1 {
                for column in 0..core_count - 1 {
                    let (Some(a), Some(b), Some(c), Some(d)) = (
                        projected[row][column],
                        projected[row][column + 1],
                        projected[row + 1][column],
                        projected[row + 1][column + 1],
                    ) else {
                        continue;
                    };
                    let load = (render_grid[row][column]
                        + render_grid[row][column + 1]
                        + render_grid[row + 1][column]
                        + render_grid[row + 1][column + 1])
                        / 400.0;
                    let shade = 0.68 + (row as f32 / row_count as f32) * 0.16;
                    let color = shade_color(viridis_color(load), shade);
                    raster_triangle(pixels, &mut depth_buffer, (w, h), a, b, c, color);
                    raster_triangle(pixels, &mut depth_buffer, (w, h), b, d, c, color);
                }
            }
        }

        // Draw a subtle sample grid over the interpolated surface.
        #[allow(clippy::needless_range_loop)]
        for x in (0..core_count).step_by(3) {
            for z in 0..row_count.saturating_sub(1) {
                if let (Some(p1), Some(p2)) = (projected[z][x], projected[z + 1][x]) {
                    draw_line(
                        pixels,
                        (w, h),
                        (p1.0, p1.1),
                        (p2.0, p2.1),
                        [75, 104, 128, 255],
                    );
                }
            }
        }

        #[allow(clippy::needless_range_loop)]
        for z in (0..row_count).step_by(3) {
            for x in 0..core_count.saturating_sub(1) {
                if let (Some(p1), Some(p2)) = (projected[z][x], projected[z][x + 1]) {
                    draw_line(
                        pixels,
                        (w, h),
                        (p1.0, p1.1),
                        (p2.0, p2.1),
                        [75, 104, 128, 255],
                    );
                }
            }
        }

        Image::from_rgba8(self.buffer.clone())
    }
}

fn shade_color(mut color: [u8; 4], factor: f32) -> [u8; 4] {
    for channel in &mut color[..3] {
        *channel = (*channel as f32 * factor.clamp(0.0, 1.0)) as u8;
    }
    color
}

fn raster_triangle(
    pixels: &mut [Rgba8Pixel],
    depth_buffer: &mut [f32],
    viewport: (u32, u32),
    a: (i32, i32, f32),
    b: (i32, i32, f32),
    c: (i32, i32, f32),
    color: [u8; 4],
) {
    let (width, height) = viewport;
    let edge = |p: (i32, i32, f32), q: (i32, i32, f32), x: f32, y: f32| {
        (q.0 - p.0) as f32 * (y - p.1 as f32) - (q.1 - p.1) as f32 * (x - p.0 as f32)
    };
    let area = edge(a, b, c.0 as f32, c.1 as f32);
    if area.abs() < f32::EPSILON || width == 0 || height == 0 {
        return;
    }
    let min_x_signed = a.0.min(b.0).min(c.0);
    let max_x_signed = a.0.max(b.0).max(c.0);
    let min_y_signed = a.1.min(b.1).min(c.1);
    let max_y_signed = a.1.max(b.1).max(c.1);
    if max_x_signed < 0
        || max_y_signed < 0
        || min_x_signed >= width as i32
        || min_y_signed >= height as i32
    {
        return;
    }
    let min_x = min_x_signed.max(0) as u32;
    let max_x = max_x_signed.min(width as i32 - 1) as u32;
    let min_y = min_y_signed.max(0) as u32;
    let max_y = max_y_signed.min(height as i32 - 1) as u32;
    if min_x > max_x || min_y > max_y {
        return;
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let w0 = edge(b, c, px, py) / area;
            let w1 = edge(c, a, px, py) / area;
            let w2 = edge(a, b, px, py) / area;
            if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                continue;
            }
            let depth = a.2 * w0 + b.2 * w1 + c.2 * w2;
            let index = (y * width + x) as usize;
            if depth < depth_buffer[index] {
                depth_buffer[index] = depth;
                pixels[index] = Rgba8Pixel {
                    r: color[0],
                    g: color[1],
                    b: color[2],
                    a: color[3],
                };
            }
        }
    }
}

// Fast integer Bresenham line drawer with boundary clipping
fn draw_line(
    pixels: &mut [Rgba8Pixel],
    viewport: (u32, u32),
    start: (i32, i32),
    end: (i32, i32),
    color: [u8; 4],
) {
    let (w, h) = viewport;
    let (x0, y0) = start;
    let (x1, y1) = end;
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
    use super::{clip_line, raster_triangle, OffscreenRenderer};
    use slint::{Rgba8Pixel, SharedPixelBuffer};

    #[test]
    fn rasterizer_fills_a_triangle_inside_the_viewport() {
        let width = 8;
        let height = 8;
        let background = Rgba8Pixel {
            r: 9,
            g: 13,
            b: 19,
            a: 255,
        };
        let mut pixels = vec![background; (width * height) as usize];
        let mut depth = vec![f32::INFINITY; pixels.len()];
        raster_triangle(
            &mut pixels,
            &mut depth,
            (width, height),
            (1, 1, 5.0),
            (6, 1, 5.0),
            (1, 6, 5.0),
            [90, 120, 160, 255],
        );
        assert!(pixels.iter().any(|pixel| pixel.r == 90));
        raster_triangle(
            &mut pixels,
            &mut depth,
            (width, height),
            (-8, 1, 5.0),
            (-2, 2, 5.0),
            (-4, 6, 5.0),
            [1, 2, 3, 255],
        );
    }

    #[test]
    fn headless_renderer_handles_empty_and_synthetic_telemetry() {
        let mut renderer = OffscreenRenderer::new(320, 180);
        let empty = renderer.render_frame();
        assert_eq!(empty.size().width, 320);
        assert_eq!(empty.size().height, 180);

        for values in [[10.0, 35.0, 70.0], [20.0, 60.0, 90.0], [5.0, 42.0, 80.0]] {
            renderer.mesh.push_sample(&values);
        }
        let frame = renderer.render_frame();
        let pixels: SharedPixelBuffer<Rgba8Pixel> = frame.to_rgba8().unwrap();
        assert!(pixels
            .as_slice()
            .iter()
            .any(|pixel| { (pixel.r, pixel.g, pixel.b) != (9, 13, 19) }));
    }

    #[test]
    fn clips_inside_outside_crossing_and_zero_length_lines() {
        assert_eq!(clip_line(2, 2, 5, 5, 10, 10), Some((2, 2, 5, 5)));
        assert_eq!(clip_line(-5, -5, -1, -1, 10, 10), None);
        assert_eq!(clip_line(-5, 5, 15, 5, 10, 10), Some((0, 5, 9, 5)));
        assert_eq!(clip_line(4, 4, 4, 4, 10, 10), Some((4, 4, 4, 4)));
    }
}
