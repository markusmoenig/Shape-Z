use crate::prelude::*;
use rayon::prelude::*;
use vek::{Aabb, Vec3};

#[derive(Clone, Debug)]
pub struct VoxelGrid {
    pub tiles: FxHashMap<Coord, Tile>,
    pub density: usize,
    pub density_f: F,
    pub bounds: [F; 3],
    /// AABB that encloses every non-empty tile.
    pub active_bbox: Aabb<F>,
    /// List of volumetric materials.
    pub volumetric: Vec<u8>,
}

impl VoxelGrid {
    pub fn empty(density: usize) -> Self {
        Self {
            tiles: FxHashMap::default(),
            density,
            density_f: density as F,
            bounds: [0.0, 0.0, 0.0],
            active_bbox: {
                let h = Vec3::from([0.0, 0.0, 0.0]) * 0.5;
                Aabb { min: -h, max: h }
            },
            volumetric: vec![],
        }
    }

    pub fn new(bounds: [F; 3], density: usize) -> Self {
        let mut tiles = FxHashMap::default();

        let x_tiles = bounds[0].ceil() as i32;
        let y_tiles = bounds[1].ceil() as i32;
        let z_tiles = bounds[2].ceil() as i32;

        let x_start = -x_tiles / 2;
        let y_start = -y_tiles / 2;
        let z_start = -z_tiles / 2;

        for tz in 0..z_tiles {
            for ty in 0..y_tiles {
                for tx in 0..x_tiles {
                    let mut tile = Tile::new(density);
                    tile.update_bbox();
                    tiles.insert((x_start + tx, y_start + ty, z_start + tz), tile);
                }
            }
        }

        Self {
            tiles,
            density,
            density_f: density as F,
            bounds,
            active_bbox: {
                let h = Vec3::from(bounds) * 0.5;
                Aabb { min: -h, max: h }
            },
            volumetric: vec![],
        }
    }

    /// Returns the size of a single voxel in world-space units.
    #[inline(always)]
    pub fn voxel_size(&self) -> f32 {
        1.0 / self.density_f
    }

    /// Update the bounding boxes of the tiles (needed after editing)
    pub fn update_bboxes(&mut self, handle_tiles: bool) {
        if handle_tiles {
            // Update the tiles bboxes
            self.tiles.par_iter_mut().for_each(|(_, tile)| {
                tile.update_bbox();
            });
        }

        // Compute a union of all non-empty tile bboxes
        let empty = Aabb {
            min: Vec3::broadcast(F::INFINITY),
            max: Vec3::broadcast(-F::INFINITY),
        };

        let tight = self
            .tiles
            .par_iter()
            .filter_map(|((tx, ty, tz), tile)| {
                if tile.is_empty() {
                    return None;
                }

                // tile local → world space:
                //   world_min = tile_origin + tile_bbox.min / density
                //   world_max = tile_origin + tile_bbox.max / density
                let origin = Vec3::new(*tx as F, *ty as F, *tz as F);
                let scale = 1.0 / self.density_f;
                Some(Aabb {
                    min: origin + tile.bbox.min * scale,
                    max: origin + tile.bbox.max * scale,
                })
            })
            .reduce(|| empty, |a, b| a.union(b));

        // Store it
        self.active_bbox = if tight.min.x.is_finite() {
            tight
        } else {
            let h = Vec3::from(self.bounds) * 0.5;
            Aabb { min: -h, max: h }
        };
    }

    /// Get a voxel at the given world coordinate
    #[inline(always)]
    pub fn get(&self, wc: Vec3<f32>) -> Option<Voxel> {
        let (tile_key, local_key) = self.to_tile_coord(wc);
        self.tiles.get(&tile_key)?.get(local_key)
    }

    /// Set a voxel at the given world coordinate
    #[inline(always)]
    pub fn set(&mut self, wc: Vec3<f32>, voxel: Voxel) {
        let (tile_key, local_key) = self.to_tile_coord(wc);
        if let Some(tile) = self.tiles.get_mut(&tile_key) {
            tile.set(local_key, voxel);
        }
    }

    /// Set a voxel at the given world coordinate and create a new tile if necessary.
    #[inline(always)]
    pub fn set_create(&mut self, wc: Vec3<f32>, voxel: Voxel) {
        let (tile_key, local_key) = self.to_tile_coord(wc);
        self.tiles
            .entry(tile_key)
            .or_insert_with(|| Tile::new(self.density))
            .set(local_key, voxel);
    }

    /// Merges this grid with another grid.
    pub fn merge(&mut self, other: &VoxelGrid) {
        // Merge
        for (tile_key, src_tile) in &other.tiles {
            let dst_tile = self
                .tiles
                .entry(*tile_key)
                .or_insert_with(|| Tile::new(self.density));

            let d = src_tile.density as i32; // side length per axis
            let area = d * d; // d², pre-compute

            // Iterate over every voxel in the dense array.
            for (idx, &maybe_mat) in src_tile.voxels.iter().enumerate() {
                if let Some(mat) = maybe_mat {
                    // Reconstruct (x,y,z) from flat index.
                    let idx = idx as i32;
                    let z = idx / area;
                    let y = (idx - z * area) / d;
                    let x = idx - z * area - y * d;

                    dst_tile.set((x, y, z), mat); // overwrite policy
                }
            }
        }
    }

    /// Converts the hit keys to a world coordinate
    #[inline(always)]
    pub fn to_world_coord(&self, tile: Coord, local: Coord) -> Vec3<f32> {
        let mut wc = Vec3::new(tile.0 as f32, tile.1 as f32, tile.2 as f32);

        wc.x += local.0 as f32 / self.density_f;
        wc.y += local.1 as f32 / self.density_f;
        wc.z += local.2 as f32 / self.density_f;

        wc
    }

    /// Converts the world coordinate to hit keys
    #[inline(always)]
    pub fn to_tile_coord(&self, wc: Vec3<f32>) -> (Coord, Coord) {
        let tile = (
            wc.x.floor() as i32,
            wc.y.floor() as i32,
            wc.z.floor() as i32,
        );
        let local = (
            ((wc.x - tile.0 as f32) * self.density_f).floor() as i32,
            ((wc.y - tile.1 as f32) * self.density_f).floor() as i32,
            ((wc.z - tile.2 as f32) * self.density_f).floor() as i32,
        );
        (tile, local)
    }

    /// World-space Aabb of the whole grid
    #[inline]
    fn bbox(&self) -> Aabb<F> {
        // let h = Vec3::from(self.bounds) * 0.5;
        // Aabb { min: -h, max: h }
        self.active_bbox
    }

    /// Returns a string with the amount of voxels inside the grid and the memory used.
    pub fn stats(&self) -> (String, String) {
        /// Formats an integer with `.` as thousands separator (e.g. 1234567 → "1.234.567")
        fn format_number(mut n: usize) -> String {
            let mut result = String::new();
            let mut digits = 0;

            while n > 0 {
                if digits != 0 && digits % 3 == 0 {
                    result.insert(0, '.');
                }
                result.insert(0, char::from_digit((n % 10) as u32, 10).unwrap());
                n /= 10;
                digits += 1;
            }

            if result.is_empty() {
                result.push('0');
            }

            result
        }

        let mut voxel_count = 0usize;
        let mut mem_bytes = 0usize;

        for tile in self.tiles.values() {
            for v in &tile.voxels {
                if v.is_some() {
                    voxel_count += 1;
                }
            }

            // Approximate memory usage:
            mem_bytes += std::mem::size_of_val(tile); // base tile
            mem_bytes += tile.voxels.capacity() * std::mem::size_of::<Option<Voxel>>(); // voxel buffer
        }

        mem_bytes += self.tiles.capacity()
            * (std::mem::size_of::<(i32, i32, i32)>() + std::mem::size_of::<Tile>());
        mem_bytes += self.volumetric.capacity();

        let mb = mem_bytes as f64 / (1024.0 * 1024.0);
        (format_number(voxel_count), format!("{:.2}", mb))
    }

    /// Recursively dda the tiles
    pub fn dda(&self, ray: &Ray, volumetric: Option<u8>) -> HitRecord {
        let mut hit = HitRecord::default();
        hit.volumetric = volumetric;

        #[inline(always)]
        fn equal(l: f32, r: Vec3<f32>) -> Vec3<f32> {
            r.map(|v| if l == v { 1.0 } else { 0.0 })
        }

        let (mut t_min, t_max) = match ray.intersect_aabb(&self.bbox()) {
            Some(b) => b,
            None => return hit,
        };

        t_min = (t_min - 0.1).max(0.0);
        let mut t = t_min;

        let ro = ray.at(t);
        let rd = ray.dir;

        let mut i = ro.map(|v| v.floor());
        let srd = rd.map(|v| v.signum());
        let rdi = Vec3::broadcast(1.0) / (rd * 2.0);

        while t <= t_max {
            let key = {
                let vi = i.map(|v| v as i32);
                (vi.x, vi.y, vi.z)
            };

            if let Some(tile) = self.tiles.get(&key) {
                let mut lro = ray.at(t);
                lro -= i; // subtract tile origin
                lro *= tile.density as f32; // scale to voxel grid
                // lro -= rd * 0.01;

                if !tile.is_empty() {
                    // DDA the tile
                    tile.dda(&Ray::new(lro, rd), &mut hit);
                    if let HitType::Voxel(voxel) = hit.hit {
                        if self.volumetric.contains(&voxel.material) {
                            hit.volumetric = Some(voxel.material);
                        } else {
                            hit.volumetric = None;
                        }
                        hit.tile_key = key;
                        hit.hitpoint = ray.at(t + hit.distance / self.density_f);
                        hit.distance = t;
                        return hit;
                    }
                };
            }

            let plane = (Vec3::broadcast(1.0) + srd - 2.0 * (ro - i)) * rdi;
            t = plane.x.min(plane.y.min(plane.z));
            let normal = equal(t, plane) * srd;
            i += normal;
        }

        HitRecord {
            hit: HitType::BBox((t_min, t_max)),
            ..Default::default()
        }
    }
}
