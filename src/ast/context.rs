use std::sync::RwLock;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Context {
    pub density: usize,

    pub definitions: FxHashMap<String, DefineObject>,
}

impl Context {
    pub fn new(density: usize) -> Self {
        Self {
            density,
            definitions: FxHashMap::default(),
        }
    }
}
