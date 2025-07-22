use crate::prelude::*;
use vek::{Vec2, Vec3, Vec4};

use rand::Rng;

pub struct Raw {
    background_color: Vec3<F>,
    execution: Execution,
}

impl Renderer for Raw {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            background_color: Vec3::broadcast(0.2),
            execution: Execution::new(0),
        }
    }

    fn name(&self) -> &str {
        "Raw"
    }

    /// Get the background color.
    fn background_color(&mut self) -> Vec3<F> {
        self.background_color
    }

    /// Set the background color.
    fn set_background_color(&mut self, color: Vec3<F>) {
        self.background_color = color;
    }

    /// Set the execution.
    fn set_execution(&mut self, execution: Execution) {
        self.execution = execution;
    }

    /// Render the pixel at the given screen position.
    fn render(
        &self,
        uv: Vec2<F>,
        resolution: Vec2<F>,
        grid: &VoxelGrid,
        materials: &[Vec<NodeOp>],
        camera: &Box<dyn Camera>,
    ) -> Vec4<F> {
        let mut rng = rand::rng();

        let ray = camera.create_ray(uv, resolution, Vec2::new(rng.random(), rng.random()));

        let hit = grid.dda(&ray);
        match hit.hit {
            HitType::Outside => {
                // background
                let c = self.background_color;
                Vec4::new(c.x, c.y, c.z, 1.0)
            }
            HitType::BBox(_) => {
                // background
                let c = self.background_color;
                Vec4::new(c.x, c.y, c.z, 1.0)
            }
            HitType::Voxel(voxel) => {
                let mut execution = Execution::new(0);
                let program = &mut Program::new(Vec3::zero(), 0);
                execution.hash = voxel.hash as f32 / 255.0;
                execution.execute(&materials[voxel.material as usize], program);

                let material = &execution.material;
                let color = material.base_color_linear();

                Vec4::new(color.x, color.y, color.z, 1.0)
            }
        }
    }
}
