use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Context {
    pub current_target: OutputTarget,
    pub custom_targets: Vec<Vec<NodeOp>>,

    pub variables: FxHashMap<String, u32>,
    pub program: Program,
}

impl Context {
    pub fn new(size: Vec3<i32>, density: usize, variables: FxHashMap<String, u32>) -> Self {
        Self {
            current_target: OutputTarget::Globals,
            custom_targets: vec![],
            program: Program::new(size, density),
            variables,
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

    // pub fn new(density: usize) -> Self {
    //     Self {
    //         code: Vec::new(),

    //         density,
    //         definitions: FxHashMap::default(),
    //     }
    // }

    // /// Clear the code generation array.
    // pub fn clean_code(&mut self) {
    //     self.code.clear();
    // }
}
