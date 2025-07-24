use crate::prelude::*;

/// The context during script and voxel compilation.
#[derive(Clone)]
pub struct Context {
    /// The current code output target.
    pub current_target: OutputTarget,

    /// Custom targets needed for recursive nesting.
    pub custom_targets: Vec<Vec<NodeOp>>,

    /// Holds the variable names and their indices into the global / flat array.
    pub variables: FxHashMap<String, u32>,

    /// Holds all declared materials and their code which gets executed by the renderer on hit.
    pub materials: IndexMap<String, Vec<NodeOp>>,

    /// Holds the global configuration code (density, background, etc.)
    pub global_config: FxHashMap<String, Vec<NodeOp>>,

    /// The optional camera config
    pub camera_config: Option<CameraD>,

    /// Holds the grid and the programs NodeOps.
    pub program: Program,
}

impl Context {
    pub fn new(size: Vec3<i32>, density: usize, variables: FxHashMap<String, u32>) -> Self {
        Self {
            current_target: OutputTarget::Globals,
            custom_targets: vec![],
            program: Program::new(size, density),
            variables,
            materials: IndexMap::default(),
            global_config: FxHashMap::default(),
            camera_config: None,
        }
    }

    pub fn set_target(&mut self, target: OutputTarget) {
        self.current_target = target;
        self.program.custom.clear();
    }

    pub fn add_custom_target(&mut self) {
        self.custom_targets.push(vec![]);
    }

    pub fn take_last_custom_target(&mut self) -> Option<Vec<NodeOp>> {
        self.custom_targets.pop()
    }

    pub fn emit(&mut self, op: NodeOp) {
        if let Some(custom) = self.custom_targets.last_mut() {
            custom.push(op.clone());
            return;
        }

        match &self.current_target {
            OutputTarget::Globals => self.program.globals.push(op),
            OutputTarget::Custom => self.program.custom.push(op),
            OutputTarget::Voxels(name) => {
                if let Some(voxel) = self.program.voxels.get_mut(name) {
                    voxel.emit(op);
                }
            }
        }
    }

    pub fn get_output_voxel(&mut self) -> Option<&mut VoxelD> {
        if let OutputTarget::Voxels(id) = &self.current_target {
            self.program.voxels.get_mut(id)
        } else {
            None
        }
    }
}
