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
        str!("Attach / Erase voxels using brushes (B).")
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

                    let mut attach_switch =
                        TheGroupButton::new(TheId::named("Brush Attach Switch"));
                    attach_switch
                        .add_text_status("Attach".to_string(), "Pick and place tiles.".to_string());
                    attach_switch.add_text_status(
                        "Remove".to_string(),
                        "Pick and place procedural materials.".to_string(),
                    );

                    attach_switch.set_item_width(80);
                    attach_switch.set_index(context.attach_mode.into());
                    layout.add_widget(Box::new(attach_switch));

                    let mut spacer = TheSpacer::new(TheId::empty());
                    spacer.limiter_mut().set_max_width(40);
                    layout.add_widget(Box::new(spacer));

                    let mut size_edit = TheTextLineEdit::new(TheId::named("Brush Size"));
                    size_edit.set_value(TheValue::Float(context.brush_size));
                    size_edit.set_info_text(Some("Brush Size".to_string()));
                    size_edit.set_range(TheValue::RangeF32(0.01..=5.0));
                    size_edit.set_continuous(false);
                    size_edit.set_status_text("The subdivision level of the grid.");
                    size_edit.limiter_mut().set_max_width(150);
                    layout.add_widget(Box::new(size_edit));

                    let mut depth_edit = TheTextLineEdit::new(TheId::named("Brush Depth"));
                    depth_edit.set_value(TheValue::Float(context.brush_depth));
                    depth_edit.set_info_text(Some("Brush Depth".to_string()));
                    depth_edit.set_range(TheValue::RangeF32(0.01..=5.0));
                    depth_edit.set_continuous(false);
                    depth_edit.set_status_text("The subdivision level of the grid.");
                    depth_edit.limiter_mut().set_max_width(150);
                    layout.add_widget(Box::new(depth_edit));

                    let mut border_edit = TheTextLineEdit::new(TheId::named("Brush Border"));
                    border_edit.set_value(TheValue::Float(context.brush_border));
                    border_edit.set_info_text(Some("Brush Border".to_string()));
                    border_edit.set_range(TheValue::RangeF32(0.0..=1.0));
                    border_edit.set_continuous(false);
                    border_edit.set_status_text("The subdivision level of the grid.");
                    border_edit.limiter_mut().set_max_width(150);
                    layout.add_widget(Box::new(border_edit));

                    let mut spacer = TheSpacer::new(TheId::empty());
                    spacer.limiter_mut().set_max_width(40);
                    layout.add_widget(Box::new(spacer));

                    let mut snap_edit = TheTextLineEdit::new(TheId::named("Brush Snap"));
                    snap_edit.set_value(TheValue::Float(context.snap));
                    snap_edit.set_info_text(Some("Grid Snap".to_string()));
                    snap_edit.set_range(TheValue::RangeF32(0.01..=1.0));
                    snap_edit.set_continuous(false);
                    snap_edit.set_status_text("The subdivision level of the grid.");
                    snap_edit.limiter_mut().set_max_width(150);
                    layout.add_widget(Box::new(snap_edit));
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
                let preview = shapes[context.shape_index].evaluate_brush(&hit, context);

                let grid = Arc::clone(&VOXELGRID);
                let mut grid = grid.write().unwrap();
                grid.preview = Some(Box::new(preview));
            }
            ToolEvent::HitDrag(hit) => {
                let shapes = SHAPES.read().unwrap();
                let preview = shapes[context.shape_index].evaluate_brush(&hit, context);

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

    fn handle_event(
        &mut self,
        event: &TheEvent,
        _ui: &mut TheUI,
        _ctx: &mut TheContext,
        context: &mut Context,
    ) -> bool {
        let redraw = false;
        #[allow(clippy::single_match)]
        match event {
            TheEvent::IndexChanged(id, index) => {
                if id.name == "Brush Attach Switch" {
                    let mut grid = VOXELGRID.write().unwrap();
                    context.attach_mode = ToolAttachMode::from(*index as i32);
                    grid.preview_mode = context.attach_mode;
                }
            }
            TheEvent::ValueChanged(id, value) => {
                if id.name == "Brush Size" {
                    if let Some(v) = value.to_f32() {
                        context.brush_size = v;
                    }
                }
                if id.name == "Brush Depth" {
                    if let Some(v) = value.to_f32() {
                        context.brush_depth = v;
                    }
                }
                if id.name == "Brush Snap" {
                    if let Some(v) = value.to_f32() {
                        context.snap = v;
                    }
                }
                if id.name == "Brush Border" {
                    if let Some(v) = value.to_f32() {
                        context.brush_border = v;
                    }
                }
            }
            _ => {}
        }
        redraw
    }
}
