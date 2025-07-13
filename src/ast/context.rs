use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Context {
    pub current_target: OutputTarget,
    pub program: Program,
    // / To generate code from expressions on the fly during AST parsing.
    // pub code: Vec<NodeOp>,

    // pub density: usize,

    // pub definitions: FxHashMap<String, DefineObject>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            current_target: OutputTarget::Globals,
            program: Program::default(),
        }
    }

    pub fn emit(&mut self, op: NodeOp) {
        match &self.current_target {
            OutputTarget::Globals => self.program.globals.push(op),
            OutputTarget::Init => self.program.init.push(op),
            OutputTarget::Definitions(name) => {
                self.program
                    .definitons
                    .entry(name.clone())
                    .or_default()
                    .push(op);
            }
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
