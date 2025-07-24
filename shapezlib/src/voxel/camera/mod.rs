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

    /// Set the camera iso scale.
    fn set_scale(&mut self, scale: F) {}

    /// Create a ray.
    fn create_ray(&self, uv: Vec2<F>, screen_size: Vec2<F>, offset: Vec2<F>) -> Ray;

    /// Execute the camera config parameters.
    fn apply_config(&mut self, execution: &mut Execution, context: &Context) {
        if let Some(camerad) = &context.camera_config {
            let mut program = context.program.clone();
            if let Some(code) = camerad.codes.get("center").cloned() {
                execution.execute(&code, &mut program);
                if let Some(value) = execution.stack.pop() {
                    self.set_center(value.as_vec3());
                }
            }
            if let Some(code) = camerad.codes.get("origin").cloned() {
                execution.execute(&code, &mut program);
                if let Some(value) = execution.stack.pop() {
                    self.set_origin(value.as_vec3());
                }
            }
            if let Some(code) = camerad.codes.get("scale").cloned() {
                execution.execute(&code, &mut program);
                if let Some(value) = execution.stack.pop() {
                    self.set_scale(value.as_float());
                }
            }
            if let Some(code) = camerad.codes.get("fov").cloned() {
                execution.execute(&code, &mut program);
                if let Some(value) = execution.stack.pop() {
                    self.set_fov(value.as_float());
                }
            }
        }
    }
}
