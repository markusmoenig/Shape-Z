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

    pub hover_hitpoint: Option<Vec3<f32>>,
    pub density: usize,

    pub palette_index: u8,
    pub shape_index: usize,

    pub snap: f32,
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

            hover_hitpoint: None,
            density: 96,

            palette_index: 0,
            shape_index: 0,

            snap: 0.5,
        }
    }
}
