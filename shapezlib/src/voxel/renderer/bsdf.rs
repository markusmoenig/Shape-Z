use crate::prelude::*;

use rand::Rng;

pub struct BSDF {
    background_color: Vec3<F>,
    sun_dir: Option<Vec3<F>>,
    sun_emission: Vec3<F>,
    execution: Execution,
}

impl Renderer for BSDF {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            background_color: Vec3::broadcast(0.8),
            sun_dir: None,
            sun_emission: Vec3::new(1.0, 0.95, 0.9),
            execution: Execution::new(0),
        }
    }

    fn name(&self) -> &str {
        "BSDF"
    }

    /// Get the background color.
    fn background_color(&mut self) -> Vec3<F> {
        self.background_color
    }

    /// Set the background color.
    fn set_background_color(&mut self, color: Vec3<F>) {
        self.background_color = color;
    }

    // Set the sun_dir.
    fn set_sun_dir(&mut self, dir: Vec3<F>) {
        self.sun_dir = Some(dir);
    }

    // Set the sun_emission.
    fn set_sun_emission(&mut self, emission: Vec3<F>) {
        self.sun_emission = emission;
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
        // project: Arc<Project>,
        grid: &VoxelGrid,
        materials: &[Vec<NodeOp>],
        camera: &Box<dyn Camera>,
    ) -> Vec4<F> {
        let program = &mut Program::new(Vec3::zero(), 0);
        let mut rng = rand::rng();

        let mut radiance = Vec3::zero();
        let mut throughput = Vec3::one();

        let mut state = BSDFState::default();
        let mut scatter_sample = BSDFScatterSampleRec::default();

        // For medium tracking
        let mut _in_medium = false;
        let mut _medium_sampled = false;
        let mut _surface_scatter = false;

        let mut ray = camera.create_ray(uv, resolution, Vec2::new(rng.random(), rng.random()));

        let mut curr_volumetric_material: Option<u8> = None;

        for depth in 0..8 {
            let hit = grid.dda(&ray, curr_volumetric_material);

            if hit.hit == HitType::Outside {
                radiance += self.srgb_to_linear(self.background_color) * throughput;
                break;
            } else if matches!(hit.hit, HitType::BBox(_)) {
                radiance += self.srgb_to_linear(self.background_color) * throughput;
                break;
            }

            if let HitType::Voxel(voxel) = hit.hit {
                curr_volumetric_material = hit.volumetric;
                let mut execution = self.execution.clone();
                execution.hash = voxel.hash as f32 / 255.0;
                execution.world = Value::from_vec3(hit.hitpoint);
                execution.local = execution.world;
                execution.execute(&materials[voxel.material as usize], program);

                state.depth = depth;
                state.mat.clone_from(&execution.material);
                state.mat.base_color = execution.material.base_color_linear();

                state.mat.roughness = state.mat.roughness.max(0.001);
                // Remapping from clearcoat gloss to roughness
                state.mat.clearcoat_roughness = lerp(0.1, 0.001, state.mat.clearcoat_roughness);

                state.hit_dist = hit.distance;
                state.fhp = hit.hitpoint;

                state.normal = hit.normal;
                state.ffnormal = if state.normal.dot(ray.dir) <= 0.0 {
                    state.normal
                } else {
                    -state.normal
                };

                state.eta = if ray.dir.dot(state.normal) < 0.0 {
                    1.0 / state.mat.ior
                } else {
                    state.mat.ior
                };

                onb(state.normal, &mut state.tangent, &mut state.bitangent);

                let aspect = (1.0 - state.mat.anisotropic * 0.9).sqrt();
                state.mat.ax = (state.mat.roughness / aspect).max(0.001);
                state.mat.ay = state.mat.roughness * aspect.max(0.001);

                _surface_scatter = true;

                // Emissive materials
                radiance += state.mat.emission * state.mat.base_color * throughput;

                // Sample sunlight if set
                if let Some(sun_dir) = &self.sun_dir {
                    let mut light_sample = BSDFLightSampleRec::default();
                    let mut scatter_sample = BSDFScatterSampleRec::default();

                    let scatter_pos = state.fhp + state.normal * 0.006;

                    let l = BSDFLight {
                        position: *sun_dir,
                        emission: self.sun_emission,
                        radius: 0.0,
                        type_: 1.0,
                        u: Vec3::zero(),
                        v: Vec3::zero(),
                        area: 0.0,
                    };

                    sample_distant_light(&l, scatter_pos, &mut light_sample, 1);

                    let li = light_sample.emission;

                    let mut in_sun_shadow = 1.0;
                    let sun_ray = Ray::new(hit.hitpoint, *sun_dir).advanced(0.006);

                    let mut volume = curr_volumetric_material;
                    loop {
                        let hit = grid.dda(&sun_ray, volume);

                        if let HitType::Voxel(_) = hit.hit {
                            if hit.volumetric.is_none() {
                                in_sun_shadow = 0.0;
                                break;
                            } else {
                                volume = hit.volumetric;
                            }
                        } else {
                            break;
                        }
                    }

                    if in_sun_shadow > 0.0 {
                        scatter_sample.f = disney_eval(
                            &state,
                            -ray.dir,
                            state.ffnormal,
                            light_sample.direction,
                            &mut scatter_sample.pdf,
                        );

                        let mis_weight = 1.0;
                        if scatter_sample.pdf > 0.0 {
                            radiance += (mis_weight * li * scatter_sample.f / light_sample.pdf)
                                * throughput
                                * in_sun_shadow;
                        }
                    }
                }

                // Sample BSDF for color and outgoing direction
                scatter_sample.f = disney_sample(
                    &state,
                    -ray.dir,
                    state.ffnormal,
                    &mut scatter_sample.l,
                    &mut scatter_sample.pdf,
                    &mut rng,
                );
                if scatter_sample.pdf > 0.0 {
                    throughput *= scatter_sample.f / scatter_sample.pdf;
                } else {
                    break;
                }

                ray = Ray::new(state.fhp, scatter_sample.l).advanced(0.006);
            }
        }

        Vec4::new(radiance.x, radiance.y, radiance.z, 1.0)
    }
}
