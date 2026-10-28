// 3D Camera Orbit Controls & Perspective Projection

pub struct OrbitCamera {
    pub yaw: f32,                // Radians (horizontal rotation)
    pub pitch: f32,              // Radians (vertical elevation)
    pub distance: f32,           // Distance from target
    pub target: (f32, f32, f32), // Look-at point
    pub pan_x: f32,
    pub pan_y: f32,
    fit_distance: f32,
    zoom_factor: f32,
}

impl OrbitCamera {
    pub fn new() -> Self {
        Self {
            yaw: 0.785,   // ~45 degrees
            pitch: 0.610, // ~35 degrees isometric tilt
            distance: 55.0,
            target: (0.0, 0.0, 0.0),
            pan_x: 0.0,
            pan_y: 0.0,
            fit_distance: 55.0,
            zoom_factor: 1.0,
        }
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw += delta_x * 0.01;
        self.pitch = (self.pitch + delta_y * 0.01).clamp(0.05, 1.50);
    }

    pub fn zoom(&mut self, delta: f32) {
        self.zoom_factor = (self.zoom_factor * (-delta * 0.0015).exp()).clamp(0.35, 3.0);
        self.distance = self.fit_distance * self.zoom_factor;
    }

    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        self.pan_x += delta_x * 0.05;
        self.pan_y += delta_y * 0.05;
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn fit_to_bounds(&mut self, width: u32, height: u32, span_x: f32, span_z: f32, max_y: f32) {
        if width == 0 || height == 0 {
            return;
        }
        self.target = (0.0, max_y * 0.5, 0.0);

        let mut lower = 0.1;
        let mut upper = span_x.max(span_z).max(max_y).max(1.0);
        while !self.bounds_fit(upper, width, height, span_x, span_z, max_y) && upper < 65_536.0 {
            upper *= 2.0;
        }
        for _ in 0..28 {
            let middle = (lower + upper) * 0.5;
            if self.bounds_fit(middle, width, height, span_x, span_z, max_y) {
                upper = middle;
            } else {
                lower = middle;
            }
        }
        self.fit_distance = upper * 1.02;
        self.distance = self.fit_distance * self.zoom_factor;
    }

    fn bounds_fit(
        &mut self,
        distance: f32,
        width: u32,
        height: u32,
        span_x: f32,
        span_z: f32,
        max_y: f32,
    ) -> bool {
        self.distance = distance;
        let margin_x = width as f32 * 0.08;
        let margin_y = height as f32 * 0.08;
        for x in [-span_x * 0.5, span_x * 0.5] {
            for y in [0.0, max_y] {
                for z in [-span_z * 0.5, span_z * 0.5] {
                    let Some((screen_x, screen_y, _)) = self.project(x, y, z, width, height) else {
                        return false;
                    };
                    if (screen_x as f32) < margin_x
                        || (screen_x as f32) > width as f32 - margin_x
                        || (screen_y as f32) < margin_y
                        || (screen_y as f32) > height as f32 - margin_y
                    {
                        return false;
                    }
                }
            }
        }
        true
    }

    // Projects 3D world coordinate (x, y, z) into 2D viewport space (screen_x, screen_y)
    pub fn project(
        &self,
        x: f32,
        y: f32,
        z: f32,
        width: u32,
        height: u32,
    ) -> Option<(i32, i32, f32)> {
        // 1. Translate to target center
        let tx = x - self.target.0;
        let ty = y - self.target.1;
        let tz = z - self.target.2;

        // 2. Rotate by Yaw around Y-axis
        let cos_yaw = self.yaw.cos();
        let sin_yaw = self.yaw.sin();
        let x1 = tx * cos_yaw - tz * sin_yaw;
        let z1 = tx * sin_yaw + tz * cos_yaw;

        // 3. Rotate by Pitch around X-axis
        let cos_pitch = self.pitch.cos();
        let sin_pitch = self.pitch.sin();
        let y2 = ty * cos_pitch - z1 * sin_pitch;
        let z2 = ty * sin_pitch + z1 * cos_pitch + self.distance;

        // Avoid division by zero or behind camera
        if z2 <= 1.0 {
            return None;
        }

        // 4. Perspective Projection
        let fov_factor = (height as f32) * 1.3;
        let screen_x = (x1 / z2) * fov_factor + (width as f32 * 0.5) + self.pan_x * 10.0;
        let screen_y = (-y2 / z2) * fov_factor + (height as f32 * 0.52) + self.pan_y * 10.0;

        Some((screen_x as i32, screen_y as i32, z2))
    }
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::OrbitCamera;

    #[test]
    fn fitted_camera_keeps_surface_bounds_inside_various_viewports() {
        for (width, height) in [(360, 600), (800, 600), (1200, 600)] {
            let mut camera = OrbitCamera::new();
            camera.fit_to_bounds(width, height, 34.0, 38.0, 12.0);
            for x in [-17.0, 17.0] {
                for y in [0.0, 12.0] {
                    for z in [-19.0, 19.0] {
                        let (screen_x, screen_y, _) =
                            camera.project(x, y, z, width, height).unwrap();
                        assert!(
                            (width as i32 * 8 / 100..=width as i32 * 92 / 100).contains(&screen_x)
                        );
                        assert!((height as i32 * 8 / 100..=height as i32 * 92 / 100)
                            .contains(&screen_y));
                    }
                }
            }
        }
    }

    #[test]
    fn refitting_preserves_user_rotation_and_relative_zoom() {
        let mut camera = OrbitCamera::new();
        camera.fit_to_bounds(800, 600, 34.0, 38.0, 12.0);
        camera.rotate(40.0, -12.0);
        camera.zoom(120.0);
        let yaw = camera.yaw;
        let pitch = camera.pitch;
        let zoom_factor = camera.zoom_factor;

        camera.fit_to_bounds(1200, 600, 34.0, 38.0, 12.0);

        assert_eq!(camera.yaw, yaw);
        assert_eq!(camera.pitch, pitch);
        assert_eq!(camera.zoom_factor, zoom_factor);
        assert!((camera.distance - camera.fit_distance * zoom_factor).abs() < 0.001);
    }
}
