use crate::prelude::*;
use std::path::PathBuf;

pub struct Module {
    pub name: String,
    pub source: String,
    pub path: PathBuf,

    pub stmts: Vec<Box<Stmt>>,
}

impl Module {
    pub fn new(name: String, source: String, path: PathBuf, stmts: Vec<Box<Stmt>>) -> Self {
        Self {
            name,
            source,
            path,
            stmts,
        }
    }
}
