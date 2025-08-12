use crate::prelude::*;

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
    pub functions: IndexMap<String, (usize, IndexMap<String, Option<Vec<NodeOp>>>, Vec<NodeOp>)>,

    /// The output grid
    pub grid: Arc<RwLock<VoxelGrid>>,

    /// The camera,
    pub camera: Arc<RwLock<Box<dyn Camera>>>,
}

impl Program {
    pub fn new(size: Vec3<i32>, density: usize) -> Self {
        Self {
            globals: Vec::new(),
            custom: Vec::new(),
            voxels: FxHashMap::default(),
            functions: IndexMap::default(),

            grid: Arc::new(RwLock::new(VoxelGrid::new(
                [size.x as F, size.y as F, size.z as F],
                density,
            ))),

            camera: Arc::new(RwLock::new(Box::new(Iso::new()))),
        }
    }
}
