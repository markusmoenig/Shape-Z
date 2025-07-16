use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct VoxelD {
    pub name: String,
    pub params: FxHashMap<String, Box<Expr>>,
    pub block: Option<Box<Stmt>>,

    pub size: Vec<NodeOp>,
    pub body: Vec<NodeOp>,

    pub shapes: Vec<Box<dyn Shape>>,
}

impl Default for VoxelD {
    fn default() -> Self {
        Self::empty()
    }
}

impl VoxelD {
    pub fn empty() -> Self {
        Self {
            name: String::new(),
            params: FxHashMap::default(),
            block: None,

            size: vec![NodeOp::Push(Value::from_components(1.0, 1.0, 1.0))],
            body: vec![],

            shapes: vec![],
        }
    }

    pub fn new(name: String, params: FxHashMap<String, Box<Expr>>, block: Box<Stmt>) -> Self {
        Self {
            name,
            params,
            block: Some(block),

            size: vec![NodeOp::Push(Value::from_components(1.0, 1.0, 1.0))],
            body: vec![],

            shapes: vec![],
        }
    }

    pub fn emit(&mut self, op: NodeOp, rec: Vec<usize>) {
        if rec.is_empty() {
            self.body.push(op);
        } else {
            if let Some(shape) = self.shapes.get_mut(rec[0]) {
                shape.emit(op, 0, rec);
            }
        }
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

impl Default for ShapeD {
    fn default() -> Self {
        Self::empty()
    }
}

impl ShapeD {
    pub fn empty() -> Self {
        Self {
            name: String::new(),
            params: FxHashMap::default(),
            block: None,

            size: vec![NodeOp::Push(Value::from_components(1.0, 1.0, 1.0))],
            body: vec![],
        }
    }

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
