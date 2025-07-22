use crate::prelude::*;
use vek::{Vec2, Vec3};

#[derive(Clone)]
pub struct Iso {
    pub center: Vec3<f32>,

    scale: f32,
    yaw: f32,
    pitch: f32,
}

impl Camera for Iso {
    fn new() -> Self {
        Self {
            center: Vec3::zero(),

            scale: 60.0,
            yaw: std::f32::consts::FRAC_PI_4,
            pitch: -0.61548, // classic iso
                             // pitch: -0.87266, // UO
        }
    }

    fn name(&self) -> &str {
        "Iso"
    }

    /// Zoom the camera in or out based on vertical mouse delta
    fn zoom(&mut self, delta: f32) {
        let zoom_sensitivity = 0.05;

        let zoom_factor = (1.0 - delta * zoom_sensitivity).clamp(0.5, 2.0);

        self.scale *= zoom_factor;
        self.scale = self.scale.clamp(2.0, 70.0);
    }

    fn set_center(&mut self, center: Vec3<F>) {
        self.center = center;
    }

    fn rotate(&mut self, delta: Vec2<f32>) {
        self.yaw += delta.x * 0.01;
    }

    fn create_ray(&self, uv: Vec2<f32>, screen: Vec2<f32>, random: Vec2<f32>) -> Ray {
        let ratio = screen.x / screen.y;
        let pixel_size = Vec2::new(1.0 / screen.x, 1.0 / screen.y);

        // Orbit distance
        let radius = 8.0;

        // Compute rotated offset in XZ plane
        let yaw = self.yaw;
        let pitch = self.pitch;

        // Compute spherical orbit offset
        let x = radius * pitch.cos() * yaw.cos();
        let y = radius * pitch.sin();
        let z = radius * pitch.cos() * yaw.sin();
        let offset = Vec3::new(x, y.abs(), z);

        let cam_origin = self.center + offset;
        let cam_look_at = self.center;

        let half_width = ((100.0 + self.scale).to_radians() * 0.5).tan();
        let half_height = half_width / ratio;

        let up_vector = Vec3::unit_y(); // (0.0, 1.0, 0.0)

        let w = (cam_origin - cam_look_at).normalized();
        let u = up_vector.cross(w).normalized();
        let v = w.cross(u).normalized();

        let horizontal = u * half_width * 2.0;
        let vertical = v * half_height * 2.0;

        let mut out_origin = cam_origin;
        out_origin += horizontal * (pixel_size.x * random.x + uv.x - 0.5);
        out_origin += vertical * (pixel_size.y * random.y + uv.y - 0.5);

        Ray::new(out_origin, (-w).normalized())
    }
}
