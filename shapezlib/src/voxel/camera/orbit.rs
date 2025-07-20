use crate::prelude::*;
use vek::{Vec2, Vec3};

pub struct Orbit {
    pub center: Vec3<F>,
    pub distance: F,
    pub azimuth: F,
    pub elevation: F,
    pub up: Vec3<F>,

    pub fov: F,
    pub near: F,
    pub far: F,
}

impl Camera for Orbit {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            center: Vec3::zero(),
            distance: 5.0,
            azimuth: std::f32::consts::PI / 2.0,
            elevation: 0.698,
            up: Vec3::unit_y(),

            fov: 75.0,
            near: 0.01,
            far: 100.0,
        }
    }

    fn name(&self) -> &str {
        "Orbit"
    }

    fn set_center(&mut self, center: Vec3<F>) {
        self.center = center;
    }

    fn set_fov(&mut self, fov: F) {
        self.fov = fov;
    }

    /// Rotate the camera around its center point using mouse delta in screen space.
    /// delta: mouse delta in pixels (x, y)
    fn rotate(&mut self, delta: Vec2<f32>) {
        // Sensitivity values (tweak as needed)
        let sensitivity = 0.005;

        self.azimuth -= delta.x * sensitivity;
        self.elevation += delta.y * sensitivity;

        // Clamp elevation to avoid flipping (just below ±90°)
        // let epsilon = 0.01;
        // let max_elevation = std::f32::consts::FRAC_PI_2 - epsilon;
        // self.elevation = self.elevation.clamp(-max_elevation, max_elevation);

        let min_elevation = 0.01;
        let max_elevation = std::f32::consts::FRAC_PI_2 - 0.01;

        self.elevation = self.elevation.clamp(min_elevation, max_elevation);
    }

    /// Zoom the camera in or out based on vertical mouse delta
    fn zoom(&mut self, delta: f32) {
        let zoom_sensitivity = 0.05;

        // Compute zoom factor (make sure it's always > 0)
        let zoom_factor = (1.0 - delta * zoom_sensitivity).clamp(0.5, 2.0);

        self.distance *= zoom_factor;

        self.distance = self.distance.clamp(0.1, 100.0);
    }

    /// Get the current eye position from orbit params
    fn origin(&self) -> Vec3<F> {
        let x = self.distance * self.azimuth.cos() * self.elevation.cos();
        let y = self.distance * self.elevation.sin();
        let z = self.distance * self.azimuth.sin() * self.elevation.cos();
        Vec3::new(x, y, z) + self.center
    }

    /// Zoom toward a given world-space point (e.g. voxel under the mouse)
    fn zoom_towards(&mut self, target: Vec3<F>, delta: f32) {
        let zoom_sensitivity = 0.05;

        // Clamp zoom factor
        let zoom_factor = (1.0 - delta * zoom_sensitivity).clamp(0.5, 2.0);

        // Current camera position
        let eye = self.origin();

        // Move camera closer to the target
        let new_eye = target + (eye - target) * zoom_factor;

        // Update distance and center accordingly
        self.distance = (self.center - new_eye).magnitude();
        self.center = target;

        // Clamp distance to safe values
        self.distance = self.distance.clamp(0.1, 100.0);
    }

    /// Create a camera ray.
    fn create_ray(&self, uv: Vec2<F>, screen_size: Vec2<F>, offset: Vec2<F>) -> Ray {
        let aspect = screen_size.x / screen_size.y;
        let pixel_size = Vec2::new(1.0 / screen_size.x, 1.0 / screen_size.y);

        // Orbit camera position
        let x = self.distance * self.azimuth.cos() * self.elevation.cos();
        let y = self.distance * self.elevation.sin();
        let z = self.distance * self.azimuth.sin() * self.elevation.cos();
        let position = Vec3::new(x, y, z) + self.center;

        // Compute correct basis
        let forward = (self.center - position).normalized(); // from eye to center
        let right = forward.cross(self.up).normalized();
        let up = right.cross(forward);

        // Screen plane height/width
        let half_height = (self.fov.to_radians() * 0.5).tan();
        let half_width = half_height * aspect;

        // Now build the ray
        let pixel_ndc = Vec2::new(
            (pixel_size.x * offset.x + uv.x) * 2.0 - 1.0, // [-1..1]
            (pixel_size.y * offset.y + (1.0 - uv.y)) * 2.0 - 1.0,
        );

        let dir = (forward + right * pixel_ndc.x * half_width - up * pixel_ndc.y * half_height) // ← minus Y because screen Y usually goes down
            .normalized();

        Ray::new(position, dir)
    }
}
