use theframework::prelude::*;

use crate::tools::{BrushShape, VoxelGrid};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ToolMode {
    Palette,
    Point,
    Pattern,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
pub enum ToolAttachMode {
    Add,
    Remove,
}

// ToolAttachMode → i32
impl From<ToolAttachMode> for i32 {
    fn from(mode: ToolAttachMode) -> Self {
        match mode {
            ToolAttachMode::Add => 0,
            ToolAttachMode::Remove => 1,
        }
    }
}

// i32 → ToolAttachMode
impl From<i32> for ToolAttachMode {
    fn from(value: i32) -> Self {
        match value {
            1 => ToolAttachMode::Remove,
            _ => ToolAttachMode::Add,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    pub mode: ToolMode,

    pub hover_hitpoint: Option<Vec3<f32>>,
    pub density: usize,

    pub brush_shape: BrushShape,

    pub palette_index: u8,
    pub pattern_index: u8,
    pub shape_index: usize,

    // Tool defaults
    pub attach_mode: ToolAttachMode,

    pub snap: f32,
    pub brush_size: f32,
    pub brush_depth: f32,
    pub brush_border: f32,
    pub brush_falloff: f32,

    pub undo_grid: VoxelGrid,
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

            brush_shape: BrushShape::Rect,

            palette_index: 0,
            pattern_index: 0,
            shape_index: 0,

            attach_mode: ToolAttachMode::Add,
            snap: 0.1,
            brush_size: 1.0,
            brush_depth: 0.1,
            brush_border: 0.0,
            brush_falloff: 2.5,

            undo_grid: VoxelGrid::new([0.0, 0.0, 0.0], 96),
        }
    }

    /// Take the undo grid and clean up afterwards.
    pub fn take_undo_grid(&mut self) -> VoxelGrid {
        let cl = self.undo_grid.clone();
        self.undo_grid = VoxelGrid::new([0.0, 0.0, 0.0], 96);
        cl
    }
}
