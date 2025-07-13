use crate::prelude::*;

/// Values in the AST
#[derive(Clone, Debug)]
pub enum Value {
    None,
    Boolean(bool),
    Float(f32),
    Float2(Box<Expr>, Box<Expr>),
    Float3(Box<Expr>, Box<Expr>, Box<Expr>),
    String(String),
    Function(String, Vec<Value>, Box<Value>),
}

impl Value {
    /// Returns the value as a float if it is one.
    pub fn to_float(&self) -> Option<f32> {
        match self {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// The truthiness of the value.
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Float(i) => *i != 0.0,
            Value::Float2(_, _) => true,
            Value::Float3(_, _, _) => true,
            Value::String(s) => !s.is_empty(),
            _ => false,
        }
    }

    // The components of the value.
    pub fn components(&self) -> usize {
        match self {
            Value::Float(_) => 1,
            Value::Float2(_, _) => 2,
            Value::Float3(_, _, _) => 3,
            _ => 0,
        }
    }
}
