use crate::prelude::*;

#[derive(Debug, Clone)]

pub enum OutputTarget {
    Globals,
    Init,
    Definitions(String),
    // Function(String),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub globals: Vec<NodeOp>,
    pub definitons: FxHashMap<String, Vec<NodeOp>>,
    pub functions: FxHashMap<String, Vec<NodeOp>>,
    pub init: Vec<NodeOp>, // optional setup code
}

impl Program {
    pub fn new() -> Self {
        Self {
            globals: Vec::new(),
            definitons: FxHashMap::default(),
            functions: FxHashMap::default(),
            init: Vec::new(),
        }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}
