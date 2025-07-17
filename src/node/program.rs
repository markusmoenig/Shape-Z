use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]

pub enum OutputTarget {
    Globals,
    Custom,
    // Name of the Voxel, id of the recursive element (Shape, Segment, Pattern)
    Voxels(String, Uuid),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub globals: Vec<NodeOp>,
    pub custom: Vec<NodeOp>,
    pub voxels: FxHashMap<String, VoxelD>,
    pub functions: FxHashMap<String, Vec<NodeOp>>,

    pub grid: Arc<RwLock<VoxelGrid>>,
}

impl Program {
    pub fn new(size: Vec3<i32>, density: usize) -> Self {
        Self {
            globals: Vec::new(),
            custom: Vec::new(),
            voxels: FxHashMap::default(),
            functions: FxHashMap::default(),

            grid: Arc::new(RwLock::new(VoxelGrid::new(
                [size.x as F, size.y as F, size.z as F],
                density,
            ))),
        }
    }
}
