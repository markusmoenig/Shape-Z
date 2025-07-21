use crate::prelude::*;
pub mod execution;
pub mod program;
pub mod value;

#[derive(Clone, Copy, Debug)]
pub enum Plane {
    XY, // Back
    YZ, // u = Y, v = Z
    XZ, // Floor
    ZY, // Left
}

#[derive(Debug, Clone)]
pub enum NodeOp {
    Load(usize),
    Store(usize),
    Swap,
    GetComponents(Vec<u8>),
    SetComponents(Vec<u8>),
    If(Vec<NodeOp>, Option<Vec<NodeOp>>),
    Place(String),
    Push(Value),
    Clear,
    Dup,
    World,
    Local,
    U,
    V,
    D,
    Hash,
    Pack2,
    Pack3,
    Add,
    Sub,
    Mul,
    Div,
    Length,
    Abs,
    Sin,
    Cos,
    Tan,
    Floor,
    Ceil,
    Fract,
    Mod,
    Degrees,
    Radians,
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
    ShapeRect(Vec<NodeOp>, Vec<NodeOp>, Vec<NodeOp>),
    ShapeDisc(Vec<NodeOp>, Vec<NodeOp>, Vec<NodeOp>),
    SegmentLeft(Vec<NodeOp>, Vec<NodeOp>),
    SegmentBack(Vec<NodeOp>, Vec<NodeOp>),
    SegmentFloor(Vec<NodeOp>, Vec<NodeOp>),
    PatternModulo(Option<Vec<NodeOp>>, Option<Vec<NodeOp>>),
    PatternBricks(Option<Vec<NodeOp>>, Option<Vec<NodeOp>>),
    MaterialAlbedo,
    MaterialSubsurface,
    MaterialMetallic,
    MaterialSpecularTint,
    MaterialRoughness,
    MaterialAnisotropic,
    MaterialSheen,
    MaterialSheenTint,
    MaterialClearcoat,
    MaterialClearcoatGloss,
    MaterialIOR,
    MaterialTransmission,
    MaterialEmission,
}
