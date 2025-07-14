use crate::prelude::*;
pub mod execution;
pub mod program;
pub mod value;

#[derive(Debug, Clone)]
pub enum NodeOp {
    Place(String),
    Push(Value),
    Local,
    Pack3,
    Add,
    Sub,
    Mul,
    Div,
    Length,
    Abs,
}
