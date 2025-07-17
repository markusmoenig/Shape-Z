use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct VoxelD {
    pub id: Uuid,
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
            id: Uuid::new_v4(),
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
            id: Uuid::new_v4(),
            name,
            params,
            block: Some(block),

            size: vec![NodeOp::Push(Value::from_components(1.0, 1.0, 1.0))],
            body: vec![],

            shapes: vec![],
        }
    }

    /// Recursively emit the op to all shapes.
    pub fn emit(&mut self, op: NodeOp, id: &Uuid) {
        if *id == self.id {
            self.body.push(op);
        } else {
            for shape in &mut self.shapes {
                shape.emit(&op, id);
            }
        }
    }

    /// Add a segment
    pub fn add_segment(&mut self, segment: &Box<dyn Segment>, id: &Uuid) {
        for shape in &mut self.shapes {
            shape.add_segment(segment, id);
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

#[derive(Clone, Debug)]
pub struct SegmentD {
    pub name: String,
    pub params: FxHashMap<String, Box<Expr>>,
    pub block: Option<Box<Stmt>>,

    pub size: Vec<NodeOp>,

    pub body: Vec<NodeOp>,
}

impl Default for SegmentD {
    fn default() -> Self {
        Self::empty()
    }
}

impl SegmentD {
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
