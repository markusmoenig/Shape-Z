use crate::prelude::*;

use fast_surface_nets::ndshape::{RuntimeShape, Shape};
use fast_surface_nets::{SurfaceNetsBuffer, surface_nets};

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

type FaceMaterials = Option<Vec<u8>>;

/// Returns (positions, indices)
pub fn mesh_voxel_grid(grid: &VoxelGrid) -> (Vec<[f32; 3]>, Vec<u32>) {
    let aabb = grid.active_bbox;
    let step = grid.voxel_size();
    let dims = (
        ((aabb.max.x - aabb.min.x) / step).ceil() as u32 + 3,
        ((aabb.max.y - aabb.min.y) / step).ceil() as u32 + 3,
        ((aabb.max.z - aabb.min.z) / step).ceil() as u32 + 3,
    );
    let shape = RuntimeShape::<u32, 3>::new([dims.0, dims.1, dims.2]);

    let mut sdf = vec![1.0f32; shape.usize()];
    let pad = 1;
    rayon::scope(|s| {
        s.spawn(|_| {
            for z in pad..dims.2 - pad {
                for y in pad..dims.1 - pad {
                    for x in pad..dims.0 - pad {
                        let world = aabb.min
                            + Vec3::new(
                                (x - pad) as f32 + 0.5,
                                (y - pad) as f32 + 0.5,
                                (z - pad) as f32 + 0.5,
                            ) * step;

                        let inside = grid.get(world).is_some();
                        let idx = shape.linearize([x, y, z]) as usize;
                        sdf[idx] = if inside { -1.0 } else { 1.0 };
                    }
                }
            }
        });
    });

    let mut buf = SurfaceNetsBuffer::default();
    surface_nets(
        &sdf,
        &shape,
        [1, 1, 1],
        [dims.0 - 2, dims.1 - 2, dims.2 - 2],
        &mut buf,
    ); //  [oai_citation:0‡Docs.rs](https://docs.rs/fast-surface-nets/latest/fast_surface_nets/fn.surface_nets.html)

    for p in &mut buf.positions {
        p[0] = p[0] * step + aabb.min.x;
        p[1] = p[1] * step + aabb.min.y;
        p[2] = p[2] * step + aabb.min.z;
    }

    (buf.positions, buf.indices)
}
// ─────────────────────────────────────────────────────────────────────────────

/// Returns (positions, indices, face_materials)
pub fn mesh_voxel_grid_with_materials(grid: &VoxelGrid) -> (Vec<[f32; 3]>, Vec<u32>, Vec<u8>) {
    let bbox = grid.active_bbox;
    let step = grid.voxel_size();
    let dims = (
        ((bbox.max.x - bbox.min.x) / step).ceil() as u32 + 3,
        ((bbox.max.y - bbox.min.y) / step).ceil() as u32 + 3,
        ((bbox.max.z - bbox.min.z) / step).ceil() as u32 + 3,
    );
    let shape = RuntimeShape::<u32, 3>::new([dims.0, dims.1, dims.2]);

    let mut sdf = vec![1.0f32; shape.usize()];
    let mut mat_grid = vec![0u8; shape.usize()];

    let pad = 1;
    for z in pad..dims.2 - pad {
        for y in pad..dims.1 - pad {
            for x in pad..dims.0 - pad {
                let world = bbox.min
                    + Vec3::new(
                        (x - pad) as f32 + 0.5,
                        (y - pad) as f32 + 0.5,
                        (z - pad) as f32 + 0.5,
                    ) * step;

                let idx = shape.linearize([x, y, z]) as usize;

                if let Some(v) = grid.get(world) {
                    sdf[idx] = -1.0; // inside
                    mat_grid[idx] = v.material; // remember voxel material
                } else {
                    // leave as +1.0 / 0 material
                }
            }
        }
    }

    let mut buf = SurfaceNetsBuffer::default();
    surface_nets(
        &sdf,
        &shape,
        [1, 1, 1],
        [dims.0 - 2, dims.1 - 2, dims.2 - 2],
        &mut buf,
    );

    for p in &mut buf.positions {
        p[0] = p[0] * step + bbox.min.x;
        p[1] = p[1] * step + bbox.min.y;
        p[2] = p[2] * step + bbox.min.z;
    }

    let mut face_mats = Vec::with_capacity(buf.indices.len() / 3);
    for tri in buf.indices.chunks_exact(3) {
        let v0 = buf.positions[tri[0] as usize];
        let v1 = buf.positions[tri[1] as usize];
        let v2 = buf.positions[tri[2] as usize];

        // face centroid in world space
        let c = [
            (v0[0] + v1[0] + v2[0]) / 3.0,
            (v0[1] + v1[1] + v2[1]) / 3.0,
            (v0[2] + v1[2] + v2[2]) / 3.0,
        ];

        // Tiny inward offset so we’re *inside* the solid when we sample
        let world = Vec3::from(c) - Vec3::broadcast(step * 0.2);

        let mat = grid.get(world).map(|v| v.material).unwrap_or(0);
        face_mats.push(mat);
    }

    (buf.positions, buf.indices, face_mats)
}

/// Write an OBJ file.
/// * `positions`  – vertex positions in world space.
/// * `indices`    – triangles, 0-based, exactly 3 indices per face.
/// * `normals`    – optional per-vertex normals (same length as `positions`).
pub fn write_obj<P: AsRef<std::path::Path>>(
    path: P,
    positions: &[[f32; 3]],
    indices: &[u32],
    normals: Option<&[[f32; 3]]>,
) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut w = BufWriter::new(file);

    // vertices
    for v in positions {
        writeln!(w, "v {} {} {}", v[0], v[1], v[2])?;
    }

    // normals
    if let Some(ns) = normals {
        for n in ns {
            writeln!(w, "vn {} {} {}", n[0], n[1], n[2])?;
        }
    }

    // faces
    for tri in indices.chunks_exact(3) {
        let v1 = tri[0] + 1;
        let v2 = tri[1] + 1;
        let v3 = tri[2] + 1;

        if normals.is_some() {
            writeln!(w, "f {0}//{0} {1}//{1} {2}//{2}", v1, v2, v3)?;
        } else {
            writeln!(w, "f {} {} {}", v1, v2, v3)?;
        }
    }
    Ok(())
}

/// Write OBJ with materials
pub fn write_obj_with_mtl<P: AsRef<Path>>(
    base_path: P,
    positions: &[[f32; 3]],
    indices: &[u32],
    face_mats: FaceMaterials, // see above
) -> std::io::Result<()> {
    let obj_path = base_path.as_ref().with_extension("obj");
    let mtl_path = base_path.as_ref().with_extension("mtl");
    let mtl_name = mtl_path.file_name().unwrap().to_string_lossy();

    let mut obj = BufWriter::new(File::create(&obj_path)?);

    // header + link to mtl
    writeln!(obj, "# generated by Shape-Z")?;
    writeln!(obj, "mtllib {}", mtl_name)?;

    // vertices
    for v in positions {
        writeln!(obj, "v {} {} {}", v[0], v[1], v[2])?;
    }

    // faces, grouped by materials
    let mut buckets: HashMap<u8, Vec<[u32; 3]>> = HashMap::new();

    for (t, tri) in indices.chunks_exact(3).enumerate() {
        let mat = face_mats
            .as_ref()
            .and_then(|mats| mats.get(t).copied())
            .unwrap_or(0); // default mat id 0

        buckets
            .entry(mat)
            .or_default()
            .push([tri[0] + 1, tri[1] + 1, tri[2] + 1]); // OBJ is 1-based
    }

    // deterministic material order
    let mut keys: Vec<u8> = buckets.keys().copied().collect();
    keys.sort_unstable();

    for mat_id in &keys {
        writeln!(obj, "usemtl material_{mat_id}")?;
        for tri in &buckets[&mat_id] {
            writeln!(obj, "f {} {} {}", tri[0], tri[1], tri[2])?;
        }
    }

    // Write mtl
    let mut mtl = BufWriter::new(File::create(&mtl_path)?);
    writeln!(
        mtl,
        "# materials for {}",
        obj_path.file_name().unwrap().to_string_lossy()
    )?;

    for mat_id in keys {
        // Pick a colour for preview: hash the id into a pastel RGB
        let hue = mat_id as f32 * 0.618_033_988; // golden ratio
        let (r, g, b) = hsv_to_rgb(hue.fract(), 0.4, 0.9);

        writeln!(mtl, "newmtl material_{mat_id}")?;
        writeln!(mtl, "Kd {:.3} {:.3} {:.3}", r, g, b)?;
        writeln!(mtl)?; // blank line
    }

    Ok(())
}

/// very light HSV→RGB helper
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let i = (h * 6.0).floor() as i32;
    let f = h * 6.0 - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    match i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}
