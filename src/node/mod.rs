use crate::prelude::*;
pub mod execution;
pub mod program;
pub mod value;

#[derive(Debug, Clone)]
pub enum NodeOp {
    If(Vec<NodeOp>, Option<Vec<NodeOp>>),
    Place(String),
    Push(Value),
    World,
    Local,
    Pack3,
    Add,
    Sub,
    Mul,
    Div,
    Length,
    Abs,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Not,
    Neg,
}
