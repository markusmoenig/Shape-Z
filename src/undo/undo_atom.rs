use crate::prelude::*;
use std::sync::Arc;
use theframework::prelude::*;

use crate::editor::{PALETTE, VOXELGRID};

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UndoAtom {
    PaletteEdit(u8, Box<NodeFXGraph>, Box<NodeFXGraph>),
    GridEdit(Box<VoxelGrid>, Box<VoxelGrid>),
}

use UndoAtom::*;

impl UndoAtom {
    pub fn undo(&self, ui: &mut TheUI, ctx: &mut TheContext, _context: &mut Context) {
        match self {
            PaletteEdit(index, prev, _) => {
                {
                    let mut palette = PALETTE.write().unwrap();
                    palette.graphs[*index as usize] = *prev.clone();
                    palette.materials[*index as usize] = prev.evaluate_material();
                }
                crate::utils::update_palette_ui(ui, ctx);
                crate::utils::reset_render();
            }
            GridEdit(prev, _) => {
                let grid = Arc::clone(&VOXELGRID);
                let mut grid = grid.write().unwrap();
                grid.replace_tiles(prev);
                grid.update_bboxes();
                crate::utils::reset_render();
            }
        }
    }
    pub fn redo(&self, ui: &mut TheUI, ctx: &mut TheContext, _context: &mut Context) {
        match self {
            PaletteEdit(index, _, next) => {
                {
                    let mut palette = PALETTE.write().unwrap();
                    palette.graphs[*index as usize] = *next.clone();
                    palette.materials[*index as usize] = next.evaluate_material();
                }
                crate::utils::update_palette_ui(ui, ctx);
                crate::utils::reset_render();
            }
            GridEdit(_, next) => {
                let grid = Arc::clone(&VOXELGRID);
                let mut grid = grid.write().unwrap();
                grid.replace_tiles(next);
                grid.update_bboxes();
                crate::utils::reset_render();
            }
        }
    }
}
