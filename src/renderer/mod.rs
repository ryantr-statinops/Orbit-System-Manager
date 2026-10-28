pub mod camera;
pub mod mesh;

use camera::OrbitCamera;
use mesh::{viridis_color, TerrainMesh};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};

const PROCESSOR_AXIS_COLOR: [u8; 4] = [99, 214, 232, 255];
const TIME_AXIS_COLOR: [u8; 4] = [103, 183, 255, 255];
const UTILIZATION_AXIS_COLOR: [u8; 4] = [253, 231, 37, 255];

#[derive(Clone, Debug, PartialEq)]
pub struct TerrainAxisLabel {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub text: String,
    pub color: [u8; 3],
    pub is_axis_title: bool,
}

struct AxisLabelSpec {
    point: (f32, f32, f32),
    text: String,
    width: i32,
    offset: (i32, i32),
    color: [u8; 3],
    is_axis_title: bool,
}

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
            self.fit_camera_to_viewport();
        }
    }

    pub fn fit_camera_to_viewport(&mut self) {
        let (span_x, span_z) = terrain_spans(self.width, self.height);
        self.camera
            .fit_to_bounds(self.width, self.height, span_x, span_z, 12.0);
    }

    pub fn projected_axis_labels(&self) -> Vec<TerrainAxisLabel> {
        let row_count = self.mesh.sample_count();
        let core_count = self.mesh.grid.first().map_or(0, Vec::len);
        if row_count == 0 || core_count == 0 {
            return Vec::new();
        }
        let (span_x, span_z) = terrain_spans(self.width, self.height);
        let floor_y = -0.75;
        let floor_half_x = span_x * 0.54;
        let floor_half_z = span_z * 0.54;
        let mut labels = Vec::new();

        for percent in [0, 25, 50, 75, 100] {
            let height = floor_y + (12.0 - floor_y) * percent as f32 / 100.0;
            append_projected_label(
                &mut labels,
                &self.camera,
                (self.width, self.height),
                AxisLabelSpec {
                    point: (-floor_half_x, height, floor_half_z),
                    text: format!("{percent}%"),
                    width: 38,
                    offset: (-40, -7),
                    color: [173, 187, 204],
                    is_axis_title: false,
                },
            );
        }
        let mut processor_ticks = vec![0, core_count / 2, core_count - 1];
        processor_ticks.dedup();
        for processor_id in processor_ticks {
            let amount = if core_count > 1 {
                processor_id as f32 / (core_count - 1) as f32
            } else {
                0.5
            };
            let x = (amount - 0.5) * span_x;
            append_projected_label(
                &mut labels,
                &self.camera,
                (self.width, self.height),
                AxisLabelSpec {
                    point: (x, floor_y, floor_half_z),
                    text: processor_id.to_string(),
                    width: 28,
                    offset: (-14, 4),
                    color: [173, 187, 204],
                    is_axis_title: false,
                },
            );
        }
        let history_seconds = self.mesh.history_seconds();
        let time_ticks = if history_seconds >= 2 {
            vec![history_seconds, history_seconds / 2, 0]
        } else if history_seconds == 1 {
            vec![1, 0]
        } else {
            vec![0]
        };
        for seconds_ago in time_ticks {
            let amount = if history_seconds > 0 {
                1.0 - seconds_ago as f32 / history_seconds as f32
            } else {
                1.0
            };
            let z = (amount - 0.5) * span_z;
            append_projected_label(
                &mut labels,
                &self.camera,
                (self.width, self.height),
                AxisLabelSpec {
                    point: (-floor_half_x, floor_y, z),
                    text: format!("{seconds_ago}s"),
                    width: 36,
                    offset: (-18, 4),
                    color: [173, 187, 204],
                    is_axis_title: false,
                },
            );
        }

        append_projected_label(
            &mut labels,
            &self.camera,
            (self.width, self.height),
            AxisLabelSpec {
                point: (0.0, floor_y, floor_half_z + 4.0),
                text: "Processor".into(),
                width: 86,
                offset: (-43, 17),
                color: [99, 214, 232],
                is_axis_title: true,
            },
        );
        append_projected_label(
            &mut labels,
            &self.camera,
            (self.width, self.height),
            AxisLabelSpec {
                point: (-floor_half_x - 4.0, floor_y, 0.0),
                text: "Time".into(),
                width: 56,
                offset: (-28, 16),
                color: [103, 183, 255],
                is_axis_title: true,
            },
        );
        append_projected_label(
            &mut labels,
            &self.camera,
            (self.width, self.height),
            AxisLabelSpec {
                point: (-floor_half_x, 12.0, floor_half_z),
                text: "Utilization".into(),
                width: 88,
                offset: (-90, -20),
                color: [253, 231, 37],
                is_axis_title: true,
            },
        );
        labels
    }

    pub fn render_frame(&mut self) -> Image {
        let w = self.width;
        let h = self.height;
        let pixels = self.buffer.make_mut_slice();

        // Keep the renderer backdrop aligned with Theme.viewport_bg (#111111).
        for pixel in pixels.iter_mut() {
            *pixel = Rgba8Pixel {
                r: 17,
                g: 17,
                b: 17,
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

        let (span_x, span_z) = terrain_spans(w, h);

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
                    let shade = surface_light_factor(&render_grid, row, column, span_x, span_z);
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
                        [61, 117, 137, 255],
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
                        [61, 117, 137, 255],
                    );
                }
            }
        }

        let floor_y = -0.75;
        let floor_half_x = span_x * 0.54;
        let floor_half_z = span_z * 0.54;
        let floor_color = [40, 59, 69, 255];
        for step in 0..=8 {
            let x = -floor_half_x + floor_half_x * 2.0 * step as f32 / 8.0;
            if let (Some(start), Some(end)) = (
                self.camera.project(x, floor_y, -floor_half_z, w, h),
                self.camera.project(x, floor_y, floor_half_z, w, h),
            ) {
                draw_depth_line(pixels, &mut depth_buffer, (w, h), start, end, floor_color);
            }
            let z = -floor_half_z + floor_half_z * 2.0 * step as f32 / 8.0;
            if let (Some(start), Some(end)) = (
                self.camera.project(-floor_half_x, floor_y, z, w, h),
                self.camera.project(floor_half_x, floor_y, z, w, h),
            ) {
                draw_depth_line(pixels, &mut depth_buffer, (w, h), start, end, floor_color);
            }
        }

        if let (Some(start), Some(end)) = (
            self.camera
                .project(-span_x * 0.5, floor_y, floor_half_z, w, h),
            self.camera
                .project(span_x * 0.5, floor_y, floor_half_z, w, h),
        ) {
            draw_depth_line(
                pixels,
                &mut depth_buffer,
                (w, h),
                start,
                end,
                PROCESSOR_AXIS_COLOR,
            );
        }
        if let (Some(start), Some(end)) = (
            self.camera
                .project(-floor_half_x, floor_y, -span_z * 0.5, w, h),
            self.camera
                .project(-floor_half_x, floor_y, span_z * 0.5, w, h),
        ) {
            draw_depth_line(
                pixels,
                &mut depth_buffer,
                (w, h),
                start,
                end,
                TIME_AXIS_COLOR,
            );
        }
        if let (Some(start), Some(end)) = (
            self.camera
                .project(-floor_half_x, floor_y, floor_half_z, w, h),
            self.camera.project(-floor_half_x, 12.0, floor_half_z, w, h),
        ) {
            draw_depth_line(
                pixels,
                &mut depth_buffer,
                (w, h),
                start,
                end,
                UTILIZATION_AXIS_COLOR,
            );
        }

        if row_count == 1 && core_count == 1 {
            if let Some((x, y, _)) = projected[0][0] {
                draw_point(
                    pixels,
                    (w, h),
                    (x, y),
                    viridis_color(render_grid[0][0] / 100.0),
                );
            }
        }

        Image::from_rgba8(self.buffer.clone())
    }
}

fn terrain_spans(width: u32, height: u32) -> (f32, f32) {
    let aspect = width as f32 / height.max(1) as f32;
    (34.0 * aspect.sqrt().clamp(1.0, 1.8), 38.0)
}

fn append_projected_label(
    labels: &mut Vec<TerrainAxisLabel>,
    camera: &OrbitCamera,
    viewport: (u32, u32),
    spec: AxisLabelSpec,
) {
    let (viewport_width, viewport_height) = viewport;
    let Some((projected_x, projected_y, _)) = camera.project(
        spec.point.0,
        spec.point.1,
        spec.point.2,
        viewport_width,
        viewport_height,
    ) else {
        return;
    };
    let x = (projected_x + spec.offset.0)
        .max(0)
        .min((viewport_width as i32 - spec.width).max(0));
    let preferred_y = (projected_y + spec.offset.1)
        .max(0)
        .min((viewport_height as i32 - 14).max(0));
    let max_y = (viewport_height as i32 - 14).max(0);
    for offset in [0, 14, -14, 28, -28, 42, -42] {
        let y = (preferred_y + offset).clamp(0, max_y);
        let overlaps = labels.iter().any(|label| {
            x < label.x + label.width
                && x + spec.width > label.x
                && y < label.y + 14
                && y + 14 > label.y
        });
        if !overlaps {
            labels.push(TerrainAxisLabel {
                x,
                y,
                width: spec.width,
                text: spec.text,
                color: spec.color,
                is_axis_title: spec.is_axis_title,
            });
            return;
        }
    }
}

fn shade_color(mut color: [u8; 4], factor: f32) -> [u8; 4] {
    for channel in &mut color[..3] {
        *channel = (*channel as f32 * factor.clamp(0.0, 1.0)) as u8;
    }
    color
}

fn surface_light_factor(
    grid: &[Vec<f32>],
    row: usize,
    column: usize,
    span_x: f32,
    span_z: f32,
) -> f32 {
    let row_count = grid.len();
    let column_count = grid.first().map_or(0, Vec::len);
    if row_count == 0 || column_count == 0 {
        return 0.9;
    }
    let left = grid[row][column.saturating_sub(1)];
    let right = grid[row][(column + 1).min(column_count - 1)];
    let previous = grid[row.saturating_sub(1)][column];
    let next = grid[(row + 1).min(row_count - 1)][column];
    let sample_width = span_x / column_count.saturating_sub(1).max(1) as f32;
    let sample_depth = span_z / row_count.saturating_sub(1).max(1) as f32;
    let slope_x = ((right - left) / 100.0 * 12.0) / sample_width.max(f32::EPSILON);
    let slope_z = ((next - previous) / 100.0 * 12.0) / sample_depth.max(f32::EPSILON);
    let normal_length = (slope_x * slope_x + 1.0 + slope_z * slope_z).sqrt();
    let light_length = (0.35_f32.powi(2) + 0.82_f32.powi(2) + 0.45_f32.powi(2)).sqrt();
    let diffuse =
        ((slope_x * 0.35 + 0.82 - slope_z * 0.45) / (normal_length * light_length)).clamp(0.0, 1.0);
    0.62 + diffuse * 0.38
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

fn draw_point(pixels: &mut [Rgba8Pixel], viewport: (u32, u32), center: (i32, i32), color: [u8; 4]) {
    let (width, height) = viewport;
    for offset_y in -2..=2 {
        for offset_x in -2..=2 {
            if offset_x * offset_x + offset_y * offset_y > 4 {
                continue;
            }
            let x = center.0 + offset_x;
            let y = center.1 + offset_y;
            if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
                continue;
            }
            let pixel = &mut pixels[(y as u32 * width + x as u32) as usize];
            *pixel = Rgba8Pixel {
                r: color[0],
                g: color[1],
                b: color[2],
                a: color[3],
            };
        }
    }
}

fn draw_depth_line(
    pixels: &mut [Rgba8Pixel],
    depth_buffer: &mut [f32],
    viewport: (u32, u32),
    start: (i32, i32, f32),
    end: (i32, i32, f32),
    color: [u8; 4],
) {
    let (width, height) = viewport;
    let Some((x0, y0, x1, y1)) =
        clip_line(start.0, start.1, end.0, end.1, width as i32, height as i32)
    else {
        return;
    };
    let dx = (end.0 - start.0) as f32;
    let dy = (end.1 - start.1) as f32;
    let length_squared = dx * dx + dy * dy;
    let step_x = if x0 < x1 { 1 } else { -1 };
    let step_y = if y0 < y1 { 1 } else { -1 };
    let (mut x, mut y) = (x0, y0);
    let (delta_x, delta_y) = ((x1 - x0).abs(), -(y1 - y0).abs());
    let mut error = delta_x + delta_y;
    loop {
        let amount = if length_squared > 0.0 {
            (((x - start.0) as f32 * dx + (y - start.1) as f32 * dy) / length_squared)
                .clamp(0.0, 1.0)
        } else {
            0.0
        };
        let depth = start.2 + (end.2 - start.2) * amount;
        let index = (y as u32 * width + x as u32) as usize;
        if depth <= depth_buffer[index] {
            depth_buffer[index] = depth;
            pixels[index] = Rgba8Pixel {
                r: color[0],
                g: color[1],
                b: color[2],
                a: color[3],
            };
        }
        if x == x1 && y == y1 {
            break;
        }
        let twice_error = 2 * error;
        if twice_error >= delta_y {
            error += delta_y;
            x += step_x;
        }
        if twice_error <= delta_x {
            error += delta_x;
            y += step_y;
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
    fn headless_render_draws_the_processor_time_and_utilization_axes() {
        let mut renderer = OffscreenRenderer::new(640, 380);
        for sample in [[10.0, 35.0, 70.0], [20.0, 60.0, 90.0], [5.0, 42.0, 80.0]] {
            renderer.mesh.push_sample(&sample);
        }
        renderer.fit_camera_to_viewport();
        let frame = renderer.render_frame();
        let pixels: SharedPixelBuffer<Rgba8Pixel> = frame.to_rgba8().unwrap();
        for expected in [(99, 214, 232), (103, 183, 255), (253, 231, 37)] {
            assert!(pixels
                .as_slice()
                .iter()
                .any(|pixel| { (pixel.r, pixel.g, pixel.b) == expected }));
        }
    }

    #[test]
    fn surface_lighting_stays_bounded_and_responds_to_sample_slopes() {
        let flat = vec![vec![50.0; 3]; 3];
        let ridge = vec![vec![0.0, 50.0, 100.0]; 3];
        let flat_shade = surface_light_factor(&flat, 1, 1, 34.0, 38.0);
        let ridge_shade = surface_light_factor(&ridge, 1, 1, 34.0, 38.0);
        assert!((0.62..=1.0).contains(&flat_shade));
        assert!((0.62..=1.0).contains(&ridge_shade));
        assert_ne!(flat_shade, ridge_shade);
    }

    #[test]
    fn projected_axis_labels_use_dynamic_processor_ticks_and_fit_viewport() {
        for core_count in [1, 16, 64] {
            let mut renderer = OffscreenRenderer::new(640, 380);
            renderer.mesh.push_sample(&vec![42.0; core_count]);
            renderer.fit_camera_to_viewport();
            let labels = renderer.projected_axis_labels();
            assert!(labels.iter().any(|label| label.text == "Processor"));
            assert!(labels.iter().any(|label| label.text == "Time"));
            assert!(labels.iter().any(|label| label.text == "Utilization"));
            assert!(labels.iter().any(|label| label.text == "0%"));
            assert!(labels.iter().any(|label| label.text == "100%"));
            assert!(labels
                .iter()
                .any(|label| label.text == (core_count - 1).to_string()));
            assert!(labels.iter().all(|label| {
                label.x >= 0
                    && label.y >= 0
                    && label.x + label.width <= renderer.width as i32
                    && label.y + 14 <= renderer.height as i32
            }));
            for (index, label) in labels.iter().enumerate() {
                assert!(labels[index + 1..].iter().all(|other| {
                    label.x + label.width <= other.x
                        || other.x + other.width <= label.x
                        || label.y + 14 <= other.y
                        || other.y + 14 <= label.y
                }));
            }
        }
    }

    #[test]
    fn single_processor_sample_draws_a_point_without_a_surface() {
        let mut renderer = OffscreenRenderer::new(320, 180);
        renderer.mesh.push_sample(&[50.0]);
        renderer.fit_camera_to_viewport();
        let frame = renderer.render_frame();
        let pixels: SharedPixelBuffer<Rgba8Pixel> = frame.to_rgba8().unwrap();
        assert!(pixels
            .as_slice()
            .iter()
            .any(|pixel| (pixel.r, pixel.g, pixel.b) == (33, 145, 140)));
    }

    #[test]
    fn clips_inside_outside_crossing_and_zero_length_lines() {
        assert_eq!(clip_line(2, 2, 5, 5, 10, 10), Some((2, 2, 5, 5)));
        assert_eq!(clip_line(-5, -5, -1, -1, 10, 10), None);
        assert_eq!(clip_line(-5, 5, 15, 5, 10, 10), Some((0, 5, 9, 5)));
        assert_eq!(clip_line(4, 4, 4, 4, 10, 10), Some((4, 4, 4, 4)));
    }
}
