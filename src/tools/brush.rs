use crate::{
    editor::{NODEEDITOR, SHAPES, UNDOMANAGER, VOXELGRID},
    prelude::*,
};
use std::sync::Arc;

pub struct BrushTool {
    id: TheId,
}

impl Tool for BrushTool {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            id: TheId::named("Attach Tool"),
        }
    }
    fn id(&self) -> TheId {
        self.id.clone()
    }
    fn info(&self) -> String {
        str!("Attach / Erase voxels using brushes.")
    }
    fn icon_name(&self) -> String {
        str!("stack")
    }
    fn accel(&self) -> Option<char> {
        Some('b')
    }

    fn tool_event(
        &mut self,
        tool_event: ToolEvent,
        ui: &mut TheUI,
        ctx: &mut TheContext,
        context: &mut Context,
    ) -> bool {
        match tool_event {
            ToolEvent::Activate => {
                if let Some(layout) = ui.get_hlayout("Tool Params") {
                    layout.clear();
                }
                let shapes = SHAPES.read().unwrap();
                let mut editor = NODEEDITOR.write().unwrap();
                editor.set_graph(
                    NodeContext::Shape(context.shape_index),
                    shapes[context.shape_index].clone(),
                    ui,
                    ctx,
                    context,
                );
            }
            ToolEvent::HitClick => {
                let grid = Arc::clone(&VOXELGRID);
                let mut grid = grid.write().unwrap();
                context.undo_grid = grid.merge_preview();
            }
            ToolEvent::HitHover(hit) => {
                let shapes = SHAPES.read().unwrap();
                let preview = shapes[context.shape_index].evaluate_shape(&hit, context);

                let grid = Arc::clone(&VOXELGRID);
                let mut grid = grid.write().unwrap();
                grid.preview = Some(Box::new(preview));
            }
            ToolEvent::HitDrag(hit) => {
                let shapes = SHAPES.read().unwrap();
                let preview = shapes[context.shape_index].evaluate_shape(&hit, context);

                let grid = Arc::clone(&VOXELGRID);
                let mut grid = grid.write().unwrap();
                grid.preview = Some(Box::new(preview));
                let changes = grid.merge_preview();
                context.undo_grid.merge(&changes);
            }
            ToolEvent::HitUp => {
                if !context.undo_grid.tiles.is_empty() {
                    let grid = Arc::clone(&VOXELGRID);
                    let grid = grid.read().unwrap();

                    let redo = grid.copy_tiles_new(&context.undo_grid);

                    let atom =
                        UndoAtom::GridEdit(Box::new(context.take_undo_grid()), Box::new(redo));
                    UNDOMANAGER.write().unwrap().add_undo(atom, ctx);
                }
            }
            _ => {}
        }

        false
    }

    // fn handle_event(
    //     &mut self,
    //     event: &TheEvent,
    //     ui: &mut TheUI,
    //     ctx: &mut TheContext,
    //     context: &mut Context,
    // ) -> bool {
    //     let mut redraw = false;
    //     #[allow(clippy::single_match)]
    //     match event {
    //         TheEvent::PaletteIndexChanged(_, index) => {
    //             if ToolMode::Palette == context.mode {
    //                 let palette = PALETTE.read().unwrap();
    //                 let mut editor = NODEEDITOR.write().unwrap();
    //                 editor.set_graph(
    //                     NodeContext::Color(*index as u8),
    //                     palette.graphs[*index as usize].clone(),
    //                     ui,
    //                     ctx,
    //                 );
    //                 false = true;
    //             }
    //         }
    //         _ => {}
    //     }
    //     redraw
    // }
}
