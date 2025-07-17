use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Context {
    pub current_target: OutputTarget,
    pub program: Program,
}

impl Context {
    pub fn new(size: Vec3<i32>, density: usize) -> Self {
        Self {
            current_target: OutputTarget::Globals,
            program: Program::new(size, density),
        }
    }

    pub fn set_target(&mut self, target: OutputTarget) {
        self.current_target = target;
        self.program.custom.clear();
    }

    pub fn emit(&mut self, op: NodeOp) {
        match &self.current_target {
            OutputTarget::Globals => self.program.globals.push(op),
            OutputTarget::Custom => self.program.custom.push(op),
            OutputTarget::Voxels(name, id) => {
                if let Some(voxel) = self.program.voxels.get_mut(name) {
                    voxel.emit(op, id);
                }
            }
        }
    }

    pub fn get_output_voxel(&mut self) -> Option<&mut VoxelD> {
        if let OutputTarget::Voxels(id, _) = &self.current_target {
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
