// 3D Camera Orbit Controls & Perspective Projection

pub struct OrbitCamera {
    pub yaw: f32,                // Radians (horizontal rotation)
    pub pitch: f32,              // Radians (vertical elevation)
    pub distance: f32,           // Distance from target
    pub target: (f32, f32, f32), // Look-at point
    pub pan_x: f32,
    pub pan_y: f32,
}

impl OrbitCamera {
    pub fn new() -> Self {
        Self {
            yaw: 0.785,   // ~45 degrees
            pitch: 0.610, // ~35 degrees isometric tilt
            distance: 55.0,
            target: (0.0, 0.0, 0.0),
            pan_x: 0.0,
            pan_y: -2.0,
        }
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw += delta_x * 0.01;
        self.pitch = (self.pitch + delta_y * 0.01).clamp(0.05, 1.50);
    }

    pub fn zoom(&mut self, delta: f32) {
        self.distance = (self.distance - delta * 0.05).clamp(20.0, 150.0);
    }

    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        self.pan_x += delta_x * 0.05;
        self.pan_y += delta_y * 0.05;
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
