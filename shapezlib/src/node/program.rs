use crate::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]

pub enum OutputTarget {
    Globals,
    Custom,
    Voxels(String),
}

#[derive(Clone)]
pub struct Program {
    pub globals: Vec<NodeOp>,
    pub custom: Vec<NodeOp>,
    pub voxels: FxHashMap<String, VoxelD>,

    /// The output grid
    pub grid: Arc<RwLock<VoxelGrid>>,

    /// Code of all user defined functions.
    pub user_functions: Vec<Arc<[NodeOp]>>,

    /// The camera,
    pub camera: Arc<RwLock<Box<dyn Camera>>>,
}

impl Program {
    pub fn new(size: Vec3<i32>, density: usize) -> Self {
        Self {
            globals: Vec::new(),
            custom: Vec::new(),
            voxels: FxHashMap::default(),
            grid: Arc::new(RwLock::new(VoxelGrid::new(
                [size.x as F, size.y as F, size.z as F],
                density,
            ))),
            user_functions: vec![],
            camera: Arc::new(RwLock::new(Box::new(Iso::new()))),
        }
    }
}
