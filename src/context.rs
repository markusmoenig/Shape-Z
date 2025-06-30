use theframework::prelude::*;

use crate::tools::VoxelGrid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ToolMode {
    Palette,
    Point,
    History,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub mode: ToolMode,

    pub hover_hitpoint: Option<Vec3<f32>>,
    pub density: usize,

    pub palette_index: u8,
    pub shape_index: usize,

    pub snap: f32,

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

            palette_index: 0,
            shape_index: 0,

            snap: 0.1,

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
