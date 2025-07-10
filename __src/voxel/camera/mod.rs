pub mod iso;
pub mod orbit;
pub mod pinhole;

use crate::prelude::*;
use vek::{Vec2, Vec3};

#[allow(unused)]
pub trait Camera: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    /// Returns the name of the camera.
    fn name(&self) -> &str;

    fn origin(&self) -> Vec3<F> {
        Vec3::zero()
    }

    /// Set the origin of the camera.
    fn set_origin(&mut self, origin: Vec3<F>) {}

    /// Set the center of the camera.
    fn set_center(&mut self, center: Vec3<F>) {}

    /// Set the fov of the camera.
    fn set_fov(&mut self, fov: F) {}

    /// Rotate the camera around its center point using mouse delta in screen space.
    fn rotate(&mut self, delta: Vec2<f32>) {}

    /// Zoom the camera in or out based on vertical mouse delta
    fn zoom(&mut self, delta: f32) {}

    /// Zoom the camera towards a target position.
    fn zoom_towards(&mut self, target: Vec3<F>, delta: f32) {}

    /// Create a ray.
    fn create_ray(&self, uv: Vec2<F>, screen_size: Vec2<F>, offset: Vec2<F>) -> Ray;
}
