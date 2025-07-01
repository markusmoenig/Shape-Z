pub mod camera;
pub mod grid;
pub mod palette;
pub mod patterns;
pub mod ray;
pub mod renderbuffer;
pub mod renderer;
pub mod tile;

use crate::F;
use vek::Vec3;

/// Face
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Face {
    PX,
    NX,
    PY,
    NY,
    PZ,
    NZ,
}

/// Coordinate for both Tiles and the Grid
pub type Coord = (i32, i32, i32);

/// HitType
#[derive(Debug, Clone, PartialEq)]
pub enum HitType {
    Voxel(u8),        // Material ID
    BBox((f32, f32)), // BBox hit (t_min, t_far)
    Outside,          // Ray didn't enter grid
}

/// HitRecord
#[derive(PartialEq, Debug, Clone)]
pub struct HitRecord {
    pub hitpoint: Vec3<F>,
    pub normal: Vec3<F>,
    pub face: Face,
    pub hit: HitType,

    pub distance: F,
    pub local_key: Coord,
    pub tile_key: Coord,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            hitpoint: Vec3::zero(),
            normal: Vec3::zero(),
            face: Face::NX,
            hit: HitType::Outside,
            distance: 0.0,
            local_key: (0, 0, 0),
            tile_key: (0, 0, 0),
        }
    }
}
