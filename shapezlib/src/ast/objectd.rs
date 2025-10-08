use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct VoxelD {
    pub name: String,
    pub params: FxHashMap<String, Box<Expr>>,
    pub block: Option<Box<Stmt>>,

    pub size: Vec<NodeOp>,
    pub body: Vec<NodeOp>,
}

impl VoxelD {
    pub fn new(name: String, params: FxHashMap<String, Box<Expr>>, block: Box<Stmt>) -> Self {
        Self {
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
    pub mods: FxHashMap<String, Box<Stmt>>,

    pub size: Vec<NodeOp>,

    pub body: Vec<NodeOp>,
}

impl ShapeD {
    pub fn new(
        name: String,
        params: FxHashMap<String, Box<Expr>>,
        block: Box<Stmt>,
        mods: FxHashMap<String, Box<Stmt>>,
    ) -> Self {
        Self {
            name,
            params,
            block: Some(block),
            mods,

            size: vec![NodeOp::Push(Value::from_components(1.0, 1.0, 1.0))],
            body: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct DistanceD {
    pub name: String,
    pub params: FxHashMap<String, Box<Expr>>,
    pub block: Option<Box<Stmt>>,

    pub offset: Option<Box<Stmt>>,
    pub scale: Option<Box<Stmt>>,

    pub size: Vec<NodeOp>,
    pub body: Vec<NodeOp>,
}

impl DistanceD {
    pub fn new(
        name: String,
        params: FxHashMap<String, Box<Expr>>,
        block: Box<Stmt>,
        offset: Option<Box<Stmt>>,
        scale: Option<Box<Stmt>>,
    ) -> Self {
        Self {
            name,
            params,
            block: Some(block),
            offset,
            scale,

            size: vec![NodeOp::Push(Value::from_components(1.0, 1.0, 1.0))],
            body: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct VolumeD {
    pub params: FxHashMap<String, Box<Expr>>,
    pub block: Option<Box<Stmt>>,

    pub size: Vec<NodeOp>,
    pub body: Vec<NodeOp>,
}

impl VolumeD {
    pub fn new(params: FxHashMap<String, Box<Expr>>, block: Box<Stmt>) -> Self {
        Self {
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

impl PatternD {
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

#[derive(Clone, Debug)]
pub struct CameraD {
    pub name: String,
    pub blocks: FxHashMap<String, Box<Stmt>>,
    pub codes: FxHashMap<String, Vec<NodeOp>>,
}

impl CameraD {
    pub fn new(name: String, blocks: FxHashMap<String, Box<Stmt>>) -> Self {
        Self {
            name,
            blocks,
            codes: FxHashMap::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MediumD {
    pub name: String,
    pub blocks: FxHashMap<String, Box<Stmt>>,
    pub code: Vec<NodeOp>,
}

impl MediumD {
    pub fn new(name: String, blocks: FxHashMap<String, Box<Stmt>>) -> Self {
        Self {
            name,
            blocks,
            code: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct MaterialD {
    pub name: String,
    pub params: FxHashMap<String, Box<Expr>>,
    pub blocks: FxHashMap<String, Box<Stmt>>,
    pub mediumd: Option<MediumD>,

    pub body: Vec<NodeOp>,
}

impl MaterialD {
    pub fn new(
        name: String,
        params: FxHashMap<String, Box<Expr>>,
        blocks: FxHashMap<String, Box<Stmt>>,
        mediumd: Option<MediumD>,
    ) -> Self {
        Self {
            name,
            params,
            blocks,
            body: vec![],
            mediumd,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FunctionD {
    pub name: String,
    pub arity: usize,
    pub locals: IndexMap<String, Option<Box<Expr>>>,
    pub block: Box<Stmt>,
    pub body: Vec<NodeOp>,
}

impl FunctionD {
    pub fn new(
        name: String,
        arity: usize,
        locals: IndexMap<String, Option<Box<Expr>>>,
        block: Box<Stmt>,
    ) -> Self {
        Self {
            name,
            arity,
            locals,
            block: block,

            body: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct RecursiveD {
    pub params: FxHashMap<String, Box<Expr>>,
    pub block: Box<Stmt>,
}

impl RecursiveD {
    pub fn new(params: FxHashMap<String, Box<Expr>>, block: Box<Stmt>) -> Self {
        Self { params, block }
    }
}
