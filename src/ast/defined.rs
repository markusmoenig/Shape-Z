use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Defined {
    pub name: String,
    pub params: FxHashMap<String, Box<Expr>>,
    pub block: Option<Box<Stmt>>,

    pub size: Vec<NodeOp>,

    pub body: Vec<NodeOp>,
}

impl Default for Defined {
    fn default() -> Self {
        Self::empty()
    }
}

impl Defined {
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
