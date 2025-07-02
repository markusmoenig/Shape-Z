use crate::{
    editor::{PALETTE, PATTERNS},
    prelude::*,
};
use rayon::prelude::*;
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
    BaseColor,
    Brush,
    MaterialIndex,
    PatternUV,
    Checker,
}

use NodeFXRole::*;

impl FromStr for NodeFXRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BaseColor" => Ok(NodeFXRole::BaseColor),
            "Brush" => Ok(NodeFXRole::Brush),
            "Checker" => Ok(NodeFXRole::Checker),
            "MaterialIndex" => Ok(NodeFXRole::MaterialIndex),
            "PatternUV" => Ok(NodeFXRole::PatternUV),
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
            BaseColor => {
                vec![0.5, 0.5, 0.5]
            }
            Brush => {
                vec![0.0]
            }
            Checker => {
                vec![0.0]
            }
            MaterialIndex => {
                vec![0.0]
            }
            PatternUV => {
                vec![0.2] // Cell Scale
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
            BaseColor => "Base Color".into(),
            Brush => "Brush".into(),
            Checker => "Checker".into(),
            MaterialIndex => "Material".into(),
            PatternUV => "Pattern UV".into(),
        }
    }

    pub fn inputs(&self) -> Vec<TheNodeTerminal> {
        match self.role {
            BaseColor | PatternUV => {
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
            Checker => {
                vec![
                    TheNodeTerminal {
                        name: "mat1".into(),
                        category_name: "ShapeFX".into(),
                    },
                    TheNodeTerminal {
                        name: "mat2".into(),
                        category_name: "ShapeFX".into(),
                    },
                ]
            }
            _ => {
                vec![TheNodeTerminal {
                    name: "out".into(),
                    category_name: "ShapeFX".into(),
                }]
            }
        }
    }

    /// Set the palette index
    pub fn set_palette_index(&mut self, index: u8) -> bool {
        #[allow(clippy::single_match)]
        match self.role {
            MaterialIndex => {
                if self.values[0] != index as f32 {
                    self.values[0] = index as f32;
                    return true;
                }
            }
            _ => {}
        }
        false
    }

    /// Set the palette index
    pub fn set_value(&mut self, _name: &str, value: f32) {
        // println!("set_value {}: {}", name, value);
        self.values[0] = value;
    }

    /// The parameters for the NodeFX
    pub fn params(&self) -> Vec<NodeFXParam> {
        let mut params = vec![];
        match self.role {
            BaseColor => {
                params.push(NodeFXParam::Color(
                    "color".into(),
                    "".into(),
                    "Base color of the palette index".into(),
                    TheColor::from(Vec3::new(self.values[0], self.values[1], self.values[2])),
                ));
            }
            PatternUV => {
                params.push(NodeFXParam::Float(
                    "cell_scale".into(),
                    "Cell Scale".into(),
                    "Pattern scale relative to one tile. Values < 1.0 mean smaller, denser pattern, > 1.0 mean larger pattern spanning multiple tiles."
                        .into(),
                    self.values[0],
                    0.001..=5.0,
                ));
            }
            _ => {}
        }
        params
    }

    /// Evaluate the node in a pattern context
    pub fn evaluate_pattern(
        &self,
        pattern_ctx: &mut PatternContext,
        _graph_node: (&NodeFXGraph, usize),
    ) {
        match self.role {
            PatternUV => {
                pattern_ctx.cell_scale = self.values[0];
            }
            Checker => {
                let size = 1;
                let (ux, uy) = (pattern_ctx.uv.x, pattern_ctx.uv.y);

                let cell_x = ux.div_euclid(size);
                let cell_y = uy.div_euclid(size);

                let v = (cell_x ^ cell_y) & 1;
                // pattern_ctx.result = if v == 0 { 100 } else { 101 };
                if v != 0 {
                    pattern_ctx.result = 100;
                }
            }
            _ => {}
        }
    }

    /// Evaluate the node in a material context
    pub fn evaluate_material(
        &self,
        material: &mut Material,
        _graph_node: (&NodeFXGraph, usize),
        context: &Context,
    ) {
        match self.role {
            BaseColor => {
                material.base_color[0] = self.values[0];
                material.base_color[1] = self.values[1];
                material.base_color[2] = self.values[2];
            }
            _ => {}
        }
    }

    /// Evaluate the node in a brush context
    pub fn evaluate_brush(
        &self,
        preview: &mut VoxelGrid,
        hit: &HitRecord,
        _graph_node: (&NodeFXGraph, usize),
        context: &mut Context,
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

        #[inline(always)]
        fn rim_thickness(r_vox: i32, border_frac: f32) -> i32 {
            let f = border_frac.clamp(0.0, 1.0);
            ((f * r_vox as f32).round()) as i32 // 0 → 0  …  1 → r_vox
        }

        #[inline(always)]
        fn stamp_circle_border(local: Vec3<F>, r_vox: i32, border_frac: f32) -> bool {
            let dist2 = local.x.powi(2) + local.z.powi(2);
            let outer2 = (r_vox as F + 0.5).powi(2);
            if dist2 > outer2 {
                return false;
            } // outside brush

            let rim = rim_thickness(r_vox, border_frac);
            if rim == 0 {
                return true;
            } // solid

            let inner_r = (r_vox - rim).max(0);
            let inner2 = (inner_r as F + 0.5).powi(2);
            dist2 > inner2 // true only for rim
        }

        #[inline]
        fn stamp_rect(local: Vec3<F>, half_vox: i32) -> bool {
            local.x.abs() <= half_vox as F &&           // |x| ≤ half-width
            local.z.abs() <= half_vox as F // |z| ≤ half-height
        }

        if let Some(hit_point) = hit_point {
            let patterns = PATTERNS.read().unwrap();

            let mut normal = hit.normal;
            if context.attach_mode == ToolAttachMode::Remove {
                normal = -normal;
            }

            // Exact voxel-size constants
            let vox_size = 1.0 / preview.density_f; // = step
            let density_i = preview.density as i32; // e.g. 96

            // Integer origin: which tile and which voxel inside that tile
            let snapped = hit_point; // already snapped + ½-voxel push
            let (tile0, loc0) = preview.to_tile_coord(snapped);

            // Integer basis vectors
            let rot = rotation_from_y(normal);
            let tan_vox = rot.cols[0].map(|v| v.round() as i32); // (±1,0,0) / (0,0,±1)
            let bit_vox = rot.cols[2].map(|v| v.round() as i32);
            let nor_vox = normal.map(|v| v.round() as i32); // (0,±1,0) etc.

            let tangent_f = rot.cols[0]; // 3-D vector length 1
            let bitangent_f = rot.cols[2];

            // The cell_world scales the pattern relative to one tile.
            let cell_world =
                patterns.graphs[context.pattern_index as usize].nodes[0].values[0].max(vox_size);
            let inv_cell = 1.0 / cell_world;

            let vox_size = 1.0 / preview.density_f;
            let diam_vox = (context.brush_size / vox_size).max(1.0);
            let r_vox = ((diam_vox - 1.0) * 0.5).ceil() as i32;

            let depth_vox = (context.brush_depth / vox_size).ceil() as i32;
            let depth = depth_vox.max(1);

            let border = context.brush_border;

            // Carry overflow/underflow between local and tile coordinates
            let carry = |mut tile: Coord, mut loc: Coord| -> (Coord, Coord) {
                for (t, l) in [
                    (&mut tile.0, &mut loc.0),
                    (&mut tile.1, &mut loc.1),
                    (&mut tile.2, &mut loc.2),
                ] {
                    if *l >= density_i {
                        *t += *l / density_i;
                        *l %= density_i;
                    } else if *l < 0 {
                        *t += (*l - (density_i - 1)) / density_i;
                        *l = ((*l % density_i) + density_i) % density_i;
                    }
                }
                (tile, loc)
            };

            for dx in -r_vox..=r_vox {
                for dz in -r_vox..=r_vox {
                    if !stamp_circle_border(Vec3::new(dx as F, 0.0, dz as F), r_vox, border) {
                        continue;
                    }

                    for d in 0..depth {
                        // offset in local voxel space
                        let off = tan_vox * dx + bit_vox * dz + nor_vox * d;

                        let loc = (loc0.0 + off.x, loc0.1 + off.y, loc0.2 + off.z);
                        let (tile, loc) = carry(tile0, loc);

                        // exact world centre of that voxel
                        let mut pos = preview.to_world_coord(tile, loc);
                        pos += Vec3::broadcast(vox_size * 0.5);

                        let rel = pos;
                        let u = (rel.dot(tangent_f) * inv_cell).floor() as i32;
                        let v = (rel.dot(bitangent_f) * inv_cell).floor() as i32;

                        // Create the material from the pattern
                        let mut pattern_ctx = PatternContext {
                            result: context.palette_index,
                            cell_scale: 1.0,
                            world: pos,
                            // uv: Vec2::new(dx, dz),
                            uv: Vec2::new(u, v),
                            normal: hit.normal,
                            layer: d,
                            max_layer: depth,
                        };
                        let index = patterns.graphs[context.pattern_index as usize]
                            .evaluate_pattern(&mut pattern_ctx);
                        preview.set_create(pos, index);
                    }
                }
            }
        }
    }

    /// Evaluate the node in a terrain brush context
    pub fn evaluate_terrain_brush(
        &self,
        preview: &mut VoxelGrid,
        hit: &HitRecord,
        grid: &VoxelGrid,
        _graph_node: (&NodeFXGraph, usize),
        context: &mut Context,
    ) {
        let hit_point = match hit.hit {
            HitType::Voxel(_) => hit.hitpoint,
            _ => return,
        };

        let vox_size = 1.0 / preview.density_f;
        let density_i = preview.density as i32;

        let (tile0, loc0) = preview.to_tile_coord(hit_point);

        let carry = |mut tile: Coord, mut loc: Coord| -> (Coord, Coord) {
            for (t, l) in [
                (&mut tile.0, &mut loc.0),
                (&mut tile.1, &mut loc.1),
                (&mut tile.2, &mut loc.2),
            ] {
                if *l >= density_i {
                    *t += *l / density_i;
                    *l %= density_i;
                } else if *l < 0 {
                    *t += (*l - (density_i - 1)) / density_i;
                    *l = ((*l % density_i) + density_i) % density_i;
                }
            }
            (tile, loc)
        };

        #[inline]
        fn layers_for_radius(dist2: i32, r_vox: i32, depth: i32, falloff: f32) -> i32 {
            if falloff <= 0.0 {
                return depth; // hard edge
            }
            let r = r_vox as f32 + 0.5;
            let d = (dist2 as f32).sqrt(); // distance in voxels
            let t = (1.0 - d / r).clamp(0.0, 1.0); // 1 at centre → 0 at rim
            let t_pow = t.powf(falloff); // adjustable curve
            (depth as f32 * t_pow).ceil() as i32
        }

        let r_vox = ((context.brush_size / vox_size - 1.0) * 0.5).ceil() as i32; // radius
        let depth_max = (context.brush_depth / vox_size).ceil() as i32; // layers
        let falloff = context.brush_falloff;

        let patterns = PATTERNS.read().unwrap();
        let vox_size = 1.0 / preview.density_f;

        let cell_world =
            patterns.graphs[context.pattern_index as usize].nodes[0].values[0].max(vox_size);
        let inv_cell = 1.0 / cell_world;

        // tangent / bitangent for a terrain brush (XZ plane)
        let tangent_f: Vec3<f32> = Vec3::unit_x(); // (1,0,0)
        let bitangent_f: Vec3<f32> = Vec3::unit_z(); // (0,0,1)
        let normal_f: Vec3<f32> = Vec3::unit_y(); // (0,1,0)

        for dx in -r_vox..=r_vox {
            for dz in -r_vox..=r_vox {
                let dist2 = dx * dx + dz * dz;
                if dist2 > (r_vox + 1).pow(2) {
                    continue;
                }

                // How many layers should this column get?
                let layers_here = layers_for_radius(dist2, r_vox, depth_max, falloff);
                if layers_here == 0 {
                    continue;
                }

                // Vertical scan
                let top_vox = loc0.1 + depth_max; // scan band still ±depth_max
                let bottom_vox = loc0.1 - depth_max;

                let mut placed = false;

                for y in (bottom_vox..=top_vox).rev() {
                    let loc = (loc0.0 + dx, y, loc0.2 + dz);
                    let (tile, loc) = carry(tile0, loc);

                    let mut centre = grid.to_world_coord(tile, loc);
                    centre += Vec3::broadcast(vox_size * 0.5);

                    if grid.get(centre).is_some() {
                        // place up to N layers above the first solid voxel
                        let mut loc_above = (loc.0, loc.1 + 1, loc.2);
                        for depth in 0..layers_here {
                            let (tile_a, loc_a) = carry(tile, loc_above);

                            let mut world = preview.to_world_coord(tile_a, loc_a);
                            world += Vec3::broadcast(vox_size * 0.5);

                            let mut pattern_ctx = PatternContext {
                                result: context.palette_index, // default
                                cell_scale: 1.0,
                                world,
                                uv: Vec2::new(
                                    (world.dot(tangent_f) * inv_cell).floor() as i32,
                                    (world.dot(bitangent_f) * inv_cell).floor() as i32,
                                ),
                                normal: normal_f,
                                layer: depth,
                                max_layer: layers_here,
                            };

                            let index = patterns.graphs[context.pattern_index as usize]
                                .evaluate_pattern(&mut pattern_ctx);
                            preview.set_create(world, index);

                            loc_above.1 += 1; // next layer
                        }
                        placed = true;
                        break;
                    }
                }

                if !placed {
                    let mut loc = (loc0.0 + dx, bottom_vox, loc0.2 + dz);
                    for depth in 0..layers_here {
                        let (tile_b, loc_b) = carry(tile0, loc);

                        let mut world = preview.to_world_coord(tile_b, loc_b);
                        world += Vec3::broadcast(vox_size * 0.5);

                        let mut pattern_ctx = PatternContext {
                            result: context.palette_index, // default
                            cell_scale: 1.0,
                            world,
                            uv: Vec2::new(
                                (world.dot(tangent_f) * inv_cell).floor() as i32,
                                (world.dot(bitangent_f) * inv_cell).floor() as i32,
                            ),
                            normal: normal_f,
                            layer: depth,
                            max_layer: layers_here,
                        };

                        let index = patterns.graphs[context.pattern_index as usize]
                            .evaluate_pattern(&mut pattern_ctx);
                        preview.set_create(world, index);

                        loc.1 += 1;
                    }
                }
            }
        }
    }

    /// Evaluate the node in a shape context
    pub fn preview(
        &self,
        buffer: &mut TheRGBABuffer,
        _graph_node: (&NodeFXGraph, usize),
        context: &mut Context,
    ) {
        let width = buffer.dim().width as usize;
        let height = buffer.dim().height;

        let r_vox = 50;
        let px_per_voxel = 1.0; // UI scale – tweak as you like
        // let r_px = (r_vox as f32 * px_per_voxel).ceil() as i32;
        let cx = (width as i32) / 2; // window centre
        let cy = (height) / 2;

        #[inline(always)]
        fn rim_thickness(r_vox: i32, border_frac: f32) -> i32 {
            let f = border_frac.clamp(0.0, 1.0);
            ((f * r_vox as f32).round()) as i32 // 0 → 0  …  1 → r_vox
        }

        #[inline(always)]
        fn stamp_circle(local: Vec3<F>, r_vox: i32, border_frac: f32) -> bool {
            let dist2 = local.x.powi(2) + local.z.powi(2);
            let outer2 = (r_vox as F + 0.5).powi(2);
            if dist2 > outer2 {
                return false;
            } // outside brush

            let rim = rim_thickness(r_vox, border_frac);
            if rim == 0 {
                return true;
            } // solid

            let inner_r = (r_vox - rim).max(0);
            let inner2 = (inner_r as F + 0.5).powi(2);
            dist2 > inner2 // true only for rim
        }

        #[inline(always)]
        fn stamp_rect(local: Vec3<F>, half_vox: i32, border_frac: f32) -> bool {
            let ax = local.x.abs() as i32;
            let az = local.z.abs() as i32;
            if ax > half_vox || az > half_vox {
                return false;
            }

            let rim = rim_thickness(half_vox, border_frac);
            if rim == 0 {
                return true;
            }

            ax >= half_vox - rim + 1 || az >= half_vox - rim + 1
        }

        let fc = 0.5;

        let palette = PALETTE.read().unwrap();

        buffer
            .pixels_mut()
            .par_rchunks_exact_mut(width * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {
                    let i = j * width + i;

                    let x = (i % width) as f32;
                    let y = (i / width) as f32;

                    let mut color = Vec4::zero();
                    #[allow(clippy::single_match)]
                    match self.role {
                        Brush => {
                            let dx = ((x as i32) - cx) as f32 / px_per_voxel;
                            let dz = ((y as i32) - cy) as f32 / px_per_voxel;

                            let inside = stamp_circle(Vec3::new(dx, 0.0, dz), r_vox, 1.0);
                            if inside {
                                color.x = fc;
                                color.y = fc;
                                color.z = fc;
                                color.w = 1.0;
                            }
                        }
                        Checker => {
                            let cell_px = 15.0;

                            let rel_px_x = (x - cx as f32);
                            let rel_px_y = (y - cy as f32);

                            let u = (rel_px_x / cell_px).floor() as i32;
                            let v = (rel_px_y / cell_px).floor() as i32;

                            let mut pattern_ctx = PatternContext {
                                result: context.palette_index,
                                cell_scale: 1.0,
                                world: Vec3::zero(),
                                uv: Vec2::new(u, v),
                                normal: Vec3::unit_y(),
                                layer: 0,
                                max_layer: 1,
                            };
                            self.evaluate_pattern(&mut pattern_ctx, _graph_node);

                            let c = palette.materials[pattern_ctx.result as usize].base_color;
                            color.x = c.x;
                            color.y = c.y;
                            color.z = c.z;
                            color.w = 1.0;
                        }
                        MaterialIndex => {
                            let index = self.values[0] as usize;
                            let c = palette.materials[index].base_color;
                            color.x = c.x;
                            color.y = c.y;
                            color.z = c.z;
                            color.w = 1.0;
                        }
                        _ => {}
                    }
                    pixel.copy_from_slice(&TheColor::from_vec4f(color).to_u8_array());
                }
            });
    }
}

// // ------------------------------------------------------------
// fn sample_pattern(p: &Pattern, uv: Vec2<i32>) -> Option<u8> {
//     match &p.kind {
//         PatternKind::None => None,
//         PatternKind::Checker{size, mat_a, mat_b} => {
//             let v = ((uv.x / *size) ^ (uv.y / *size)) & 1;
//             Some(if v == 0 { *mat_a } else { *mat_b })
//         }
//         PatternKind::Brick{w, h, mort, mat_brick, mat_mortar} => {
//             let (bx, by) = (uv.x % (w+*mort), uv.y % (h+*mort));
//             Some(if bx < *w && by < *h { *mat_brick } else { *mat_mortar })
//         }
//         // Noise etc…
//     }
// }
