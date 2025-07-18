use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct VoxelD {
    pub id: Uuid,
    pub name: String,
    pub params: FxHashMap<String, Box<Expr>>,
    pub block: Option<Box<Stmt>>,

    pub size: Vec<NodeOp>,
    pub body: Vec<NodeOp>,
}

impl VoxelD {
    pub fn new(name: String, params: FxHashMap<String, Box<Expr>>, block: Box<Stmt>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            params,
            block: Some(block),

            size: vec![NodeOp::Push(Value::from_components(1.0, 1.0, 1.0))],
            body: vec![],
        }
    }

    /// Recursively emit the op to all shapes.
    pub fn emit(&mut self, op: NodeOp) {
        self.body.push(op);
    }
}

#[derive(Clone, Debug)]
pub struct ShapeD {
    pub name: String,
    pub params: FxHashMap<String, Box<Expr>>,
    pub block: Option<Box<Stmt>>,

    pub size: Vec<NodeOp>,

    pub body: Vec<NodeOp>,
}

impl ShapeD {
    pub fn new(name: String, params: FxHashMap<String, Box<Expr>>, block: Box<Stmt>) -> Self {
        Self {
            name,
            params,
            block: Some(block),

            size: vec![NodeOp::Push(Value::from_components(1.0, 1.0, 1.0))],
            body: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct SegmentD {
    pub name: String,
    pub params: FxHashMap<String, Box<Expr>>,
    pub block: Option<Box<Stmt>>,

    pub size: Vec<NodeOp>,

    pub body: Vec<NodeOp>,
}

impl SegmentD {
    pub fn new(name: String, params: FxHashMap<String, Box<Expr>>, block: Box<Stmt>) -> Self {
        Self {
            name,
            params,
            block: Some(block),

            size: vec![NodeOp::Push(Value::from_components(1.0, 1.0, 1.0))],
            body: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct PatternD {
    pub name: String,
    pub params: FxHashMap<String, Box<Expr>>,
    pub blocks: FxHashMap<String, Box<Stmt>>,
}

impl Default for PatternD {
    fn default() -> Self {
        Self::empty()
    }
}

impl PatternD {
    pub fn empty() -> Self {
        Self {
            name: String::new(),
            params: FxHashMap::default(),
            blocks: FxHashMap::default(),
        }
    }

    pub fn new(
        name: String,
        params: FxHashMap<String, Box<Expr>>,
        blocks: FxHashMap<String, Box<Stmt>>,
    ) -> Self {
        Self {
            name,
            params,
            blocks,
        }
    }
}
