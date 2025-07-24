use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Tile {
    pub voxels: Vec<Option<Voxel>>,
    pub density: usize,
    pub bbox: Aabb<F>,
    pub voxel_counter: u32,
}

impl Tile {
    pub fn new(density: usize) -> Self {
        let total = density * density * density;

        Self {
            voxels: vec![None; total],
            density,
            bbox: Aabb {
                min: Vec3::zero(),
                max: Vec3::zero(),
            },
            voxel_counter: 0,
        }
    }

    pub fn update_bbox(&mut self) {
        let mut min = Vec3::new(i32::MAX, i32::MAX, i32::MAX);
        let mut max = Vec3::new(i32::MIN, i32::MIN, i32::MIN);
        let d = self.density as i32;

        self.voxel_counter = 0;

        for z in 0..d {
            for y in 0..d {
                for x in 0..d {
                    if self.get((x, y, z)).is_some() {
                        self.voxel_counter += 1;
                        min.x = min.x.min(x);
                        min.y = min.y.min(y);
                        min.z = min.z.min(z);
                        max.x = max.x.max(x);
                        max.y = max.y.max(y);
                        max.z = max.z.max(z);
                    }
                }
            }
        }

        if self.voxel_counter > 0 {
            self.bbox = Aabb {
                min: min.map(|v| v as F),
                max: max.map(|v| v as F + 1.0),
            };
        } else {
            self.bbox = Aabb {
                min: Vec3::zero(),
                max: Vec3::zero(),
            };
        }
    }

    #[inline(always)]
    fn index(&self, (x, y, z): Coord) -> Option<usize> {
        if x >= 0 && y >= 0 && z >= 0 {
            let d = self.density as i32;
            if x < d && y < d && z < d {
                let i = (z * d * d + y * d + x) as usize;
                return Some(i);
            }
        }
        None
    }

    #[inline]
    pub fn get(&self, coord: Coord) -> Option<Voxel> {
        self.index(coord).and_then(|i| self.voxels[i])
    }

    #[inline]
    pub fn set(&mut self, coord: Coord, mat: Voxel) {
        if let Some(i) = self.index(coord) {
            self.voxels[i] = Some(mat);
        }
    }

    #[inline]
    pub fn clear(&mut self, coord: Coord) {
        if let Some(i) = self.index(coord) {
            self.voxels[i] = None;
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.voxel_counter == 0
    }

    pub fn dda(&self, ray: &Ray, hit: &mut HitRecord) {
        let (mut t_min, t_max) = match ray.intersect_aabb(&self.bbox) {
            Some(b) => b,
            None => return,
        };

        t_min = (t_min - 0.002).max(0.0);

        let ro = ray.at(t_min);
        let rd = ray.dir;

        #[inline(always)]
        fn equal(l: f32, r: Vec3<f32>) -> Vec3<f32> {
            r.map(|v| if l == v { 1.0 } else { 0.0 })
        }

        let mut i = ro.map(|v| v.floor());
        let srd = rd.map(|v| v.signum());
        let rdi = Vec3::broadcast(1.0) / (rd * 2.0);
        let mut normal = Vec3::zero();

        let mut t = t_min;
        while t <= t_max {
            let key = {
                let vi = i.map(|v| v as i32);
                (vi.x, vi.y, vi.z)
            };

            if let Some(voxel) = self.get(key) {
                if Some(voxel.material) != hit.volumetric {
                    hit.hit = HitType::Voxel(voxel);
                    hit.hitpoint = ray.at(t);
                    hit.distance = t_min + t;
                    hit.normal = -normal;
                    hit.local_key = key;
                    return;
                }
            }

            let plane = (Vec3::broadcast(1.0) + srd - 2.0 * (ro - i)) * rdi;
            t = plane.x.min(plane.y.min(plane.z));
            normal = equal(t, plane) * srd;
            i += normal;
        }
    }
}
