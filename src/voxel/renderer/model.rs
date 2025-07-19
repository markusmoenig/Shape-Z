use crate::prelude::*;
use vek::{Vec2, Vec3, Vec4};

use rand::Rng;

pub struct Model {
    pub background_color: Vec3<F>,
}

impl Renderer for Model {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            background_color: Vec3::broadcast(0.2),
        }
    }

    fn name(&self) -> &str {
        "EditShader"
    }

    /// Get the background color.
    fn background_color(&mut self) -> Vec3<F> {
        self.background_color
    }

    /// Set the background color.
    fn set_background_color(&mut self, color: Vec3<F>) {
        self.background_color = color;
    }

    /// Render the pixel at the given screen position.
    fn render(
        &self,
        uv: Vec2<F>,
        resolution: Vec2<F>,
        grid: &VoxelGrid,
        _materials: &[Vec<NodeOp>],
        camera: &Box<dyn Camera>,
    ) -> Vec4<F> {
        let mut rng = rand::rng();

        // let mut acc = Vec3::<F>::zero();
        // let mut mask = Vec3::<F>::one();

        let ray = camera.create_ray(uv, resolution, Vec2::new(rng.random(), rng.random()));

        // -------------- voxel DDA -------------------------------------------
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
            HitType::Voxel(_m) => {
                // -------------- palette lookup --------------------------------------
                //let mat = palette.get(m);
                let mut color = Vec3::zero(); //mat.base_color; // linear 0-1

                // -------------- basic diffuse lighting ------------------------------
                let light_dir = Vec3::new(-0.5, 1.0, -0.5).normalized();
                let n = hit.normal.normalized();
                let diff = n.dot(light_dir).max(0.0);
                let mut shade = 0.20 /*ambient*/ + 0.80 * diff;

                // -------------- tiny edge/specular accent ---------------------------
                let view_dir = -ray.dir; // pointing to camera
                let spec_boost = view_dir.dot(n).max(0.0).powf(20.0);
                shade += 0.15 * spec_boost; // up to +15 %

                // -------------- 6-tap ambient-occlusion -----------------------------
                let vs = grid.voxel_size(); // voxel size in world space

                let offsets = [
                    Vec3::new(vs, 0.0, 0.0),
                    Vec3::new(-vs, 0.0, 0.0),
                    Vec3::new(0.0, vs, 0.0),
                    Vec3::new(0.0, -vs, 0.0),
                    Vec3::new(0.0, 0.0, vs),
                    Vec3::new(0.0, 0.0, -vs),
                ];

                let p = hit.hitpoint;
                let empty = offsets
                    .iter()
                    .filter(|&&o| grid.get(p + o).is_none())
                    .count() as F;

                let ao = 1.0 + empty * 0.09;
                shade *= ao;

                let solid_neighbors = offsets
                    .iter()
                    .filter(|&&o| grid.get(p + o).is_some())
                    .count();

                let edge_factor = match solid_neighbors {
                    6 => 1.0, // fully enclosed → no edge
                    5 => 0.85,
                    4 => 0.70,
                    3 => 0.55,
                    _ => 0.4, // exposed from multiple sides → stronger edge
                };
                shade *= edge_factor;

                // -------------- final colour ----------------------------------------
                color *= shade.clamp(0.0, 1.5); // keep sane
                Vec4::new(color.x, color.y, color.z, 1.0)
            }
        }
    }
}
