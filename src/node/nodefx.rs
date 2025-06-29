use crate::prelude::*;
use std::str::FromStr;
use theframework::prelude::*;
use vek::Vec2;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeFXParam {
    /// Id, Name, Status, Value, Range
    Float(String, String, String, f32, std::ops::RangeInclusive<f32>),
    /// Id, Name, Status, Value, Range
    Int(String, String, String, i32, std::ops::RangeInclusive<i32>),
    /// Id, Name, Status, Value
    PaletteIndex(String, String, String, i32),
    /// Id, Name, Status, Options, Value
    Selector(String, String, String, Vec<String>, i32),
    /// Id, Name, Status, Value
    Color(String, String, String, TheColor),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NodeFXRole {
    Color,
    Shape,
}

use NodeFXRole::*;

impl FromStr for NodeFXRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Color" => Ok(NodeFXRole::Color),
            "Brush" => Ok(NodeFXRole::Shape),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeFX {
    pub id: Uuid,
    pub role: NodeFXRole,
    pub values: Vec<F>,

    pub position: Vec2<i32>,
}

impl NodeFX {
    pub fn new(role: NodeFXRole) -> Self {
        let values = match role {
            Color => {
                vec![0.5, 0.5, 0.5]
            }
            Shape => {
                vec![0.0]
            }
        };

        Self {
            id: Uuid::new_v4(),
            role,
            values,
            position: Vec2::new(20, 20),
        }
    }

    pub fn name(&self) -> String {
        match self.role {
            Color => "Color".into(),
            Shape => "Shape".into(),
        }
    }

    pub fn inputs(&self) -> Vec<TheNodeTerminal> {
        match self.role {
            Color => {
                vec![]
            }
            _ => {
                vec![TheNodeTerminal {
                    name: "in".into(),
                    category_name: "ShapeFX".into(),
                }]
            }
        }
    }

    pub fn outputs(&self) -> Vec<TheNodeTerminal> {
        match self.role {
            _ => {
                vec![TheNodeTerminal {
                    name: "out".into(),
                    category_name: "ShapeFX".into(),
                }]
            }
        }
    }

    /// The parameters for the NodeFX
    pub fn params(&self) -> Vec<NodeFXParam> {
        let mut params = vec![];
        match self.role {
            Color => {
                params.push(NodeFXParam::Color(
                    "color".into(),
                    "".into(),
                    "Base color of the palette index".into(),
                    TheColor::from(Vec3::new(self.values[0], self.values[1], self.values[2])),
                ));
            }
            _ => {}
        }
        params
    }

    /// Evaluate the node in a material context
    pub fn evaluate_material(&self, material: &mut Material, _graph_node: (&NodeFXGraph, usize)) {
        match self.role {
            Color => {
                material.base_color[0] = self.values[0];
                material.base_color[1] = self.values[1];
                material.base_color[2] = self.values[2];
            }
            _ => {}
        }
    }

    /// Evaluate the node in a shape context
    pub fn evaluate_shape(
        &self,
        preview: &mut VoxelGrid,
        hit: &HitRecord,
        _graph_node: (&NodeFXGraph, usize),
        context: &Context,
    ) {
        let hit_point: Option<Vec3<f32>> = match hit.hit {
            HitType::Outside => None,
            HitType::BBox((_t_near, _t_far)) => None, //Some(ray.at(t_far)),
            HitType::Voxel(_) => Some(hit.hitpoint),
        };

        fn rotation_from_y(normal: Vec3<F>) -> Mat3<F> {
            let up = Vec3::unit_y();
            let normal = normal.normalized();

            if (normal - up).magnitude_squared() < 1e-6 {
                return Mat3::identity();
            }

            if (normal + up).magnitude_squared() < 1e-6 {
                // 180° flip
                return Mat3 {
                    cols: Vec3::new(
                        Vec3::new(1.0, 0.0, 0.0),
                        Vec3::new(0.0, -1.0, 0.0),
                        Vec3::new(0.0, 0.0, -1.0),
                    ),
                };
            }

            let tangent = up.cross(normal).normalized();
            let bitangent = normal.cross(tangent);

            Mat3 {
                cols: Vec3::new(
                    tangent,   // X
                    normal,    // Y
                    bitangent, // Z
                ),
            }
        }

        #[inline]
        fn stamp_circle(local: Vec3<F>, r_vox: i32) -> bool {
            let dist_sq = local.x.powi(2) + local.z.powi(2);
            dist_sq <= (r_vox as F + 0.5).powi(2)
        }

        #[inline]
        fn stamp_rect(local: Vec3<F>, half_vox: i32) -> bool {
            local.x.abs() <= half_vox as F &&           // |x| ≤ half-width
            local.z.abs() <= half_vox as F // |z| ≤ half-height
        }

        /// Full 3-D sphere (useful when you want a true ball, not just a disc).
        #[inline]
        fn stamp_sphere(local: Vec3<F>, r_vox: i32) -> bool {
            local.magnitude_squared() <= (r_vox as F + 0.5).powi(2)
        }

        if let Some(hit_point) = hit_point {
            let step = 1.0 / preview.density_f;
            let depth = 1;
            let normal = hit.normal;
            let outside = hit.normal * 0.5 * step;

            let r_tile: F = 1.0;
            let r_vox: i32 = (r_tile * preview.density_f / 2.0).round() as i32;
            let rot = rotation_from_y(normal);

            // Extrude in slices
            for dx in -r_vox..=r_vox {
                for dz in -r_vox..=r_vox {
                    let local = Vec3::new(dx as F, 0.0, dz as F);
                    if !stamp_rect(local, r_vox) {
                        continue;
                    }

                    let base = rot * local;

                    for d in 0..depth {
                        let pos = hit_point + outside + normal * (d as F) * step + base * step;
                        preview.set_create(pos, context.palette_index);
                    }
                }
            }

            /*
            // Extrude in half 3D shapes (Sphere)
            for dx in -r_vox..=r_vox {
                for dy in -r_vox..=r_vox {
                    for dz in -r_vox..=r_vox {
                        let local = Vec3::new(dx as F, dy as F, dz as F);

                        if !stamp_sphere(local, r_vox) {
                            // true 3-D check
                            continue;
                        }

                        // keep only voxels on or in front of the hit plane (hemisphere)
                        let rotated = rot * local;
                        if rotated.dot(normal) < 0.0 {
                            continue;
                        }

                        let pos = hit_point + outside + rotated * step;
                        preview.set_create(pos, context.palette_index);
                    }
                }
            }*/
        }
    }
}
