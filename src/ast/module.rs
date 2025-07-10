// use crate::prelude::*;
use std::path::PathBuf;

pub struct Module {
    pub name: String,
    pub source: String,
    pub path: PathBuf,
}

impl Module {
    pub fn new(name: String, source: String, path: PathBuf) -> Self {
        Self { name, source, path }
    }
}
