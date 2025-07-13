use crate::prelude::*;
pub mod execution;
pub mod program;
pub mod value;

#[derive(Debug, Clone, Copy)]
pub enum NodeOp {
    Push(Value),
    Add,
    Sub,
    Mul,
    Div,
    Length,
    Abs,
}
