use crate::prelude::*;

pub struct VoxelRect {
    pub origin: Vec3<F>, // world-space position
    pub size: Vec3<F>,   // world-space size
}

impl VoxelRect {
    /// Iterate all voxel positions within this rectangle.
    pub fn iter_voxels(&self, grid: &VoxelGrid) -> impl Iterator<Item = Vec3<F>> + '_ {
        let voxel_size = grid.voxel_size();

        let start = self.origin.map(|v| (v / voxel_size).floor() as i32);
        let end = (self.origin + self.size).map(|v| (v / voxel_size).ceil() as i32);

        let (sx, sy, sz) = (start.x, start.y, start.z);
        let (ex, ey, ez) = (end.x, end.y, end.z);

        let center_x = self.origin.x + self.size.x * 0.5;
        let center_y = self.origin.y + self.size.y * 0.5;
        let center_z = self.origin.z + self.size.z * 0.5;
        // let bottom_y = self.origin.y;

        (sx..ex).flat_map(move |x| {
            (sy..ey).flat_map(move |y| {
                (sz..ez).map(move |z| {
                    let voxel_half = voxel_size * 0.5;
                    let world = Vec3::new(x as F, y as F, z as F) * voxel_size
                        + Vec3::broadcast(voxel_half);
                    Vec3::new(world.x - center_x, world.y - center_y, world.z - center_z)
                })
            })
        })
    }

    /// Convert a world coordinate to local
    #[inline(always)]
    pub fn world_to_local(&self, world: Vec3<F>) -> Vec3<F> {
        world - self.origin
    }

    /// Set all voxels inside this region in the grid
    pub fn fill(&self, grid: &mut VoxelGrid, voxel: Voxel) {
        for world in self.iter_voxels(grid) {
            let local = self.world_to_local(world);

            if (local - Vec3::new(0.0, 0.0, 0.0)).magnitude() - 0.5 <= 0.0 {
                grid.set_create(world, voxel);
            }
        }
    }
}
