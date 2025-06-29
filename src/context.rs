use theframework::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ToolMode {
    Palette,
    Point,
    History,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Context {
    pub mode: ToolMode,

    pub shape_index: usize,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            mode: ToolMode::Palette,

            shape_index: 0,
        }
    }
}
