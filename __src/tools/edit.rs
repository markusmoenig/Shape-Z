use crate::{
    editor::{NODEEDITOR, PALETTE, PATTERNS},
    prelude::*,
};

pub struct EditTool {
    id: TheId,
}

impl Tool for EditTool {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            id: TheId::named("Edit Tool"),
        }
    }
    fn id(&self) -> TheId {
        self.id.clone()
    }
    fn info(&self) -> String {
        str!("Edit the current mode (E).")
    }
    fn icon_name(&self) -> String {
        str!("hand-pointing")
    }
    fn accel(&self) -> Option<char> {
        Some('e')
    }

    fn tool_event(
        &mut self,
        tool_event: ToolEvent,
        ui: &mut TheUI,
        ctx: &mut TheContext,
        context: &mut Context,
    ) -> bool {
        let mut redraw = false;
        if tool_event == ToolEvent::Activate {
            if let Some(layout) = ui.get_hlayout("Tool Params") {
                layout.clear();
            }

            if ToolMode::Palette == context.mode {
                let palette = PALETTE.read().unwrap();
                let mut editor = NODEEDITOR.write().unwrap();
                editor.set_graph(
                    NodeContext::Color(context.palette_index),
                    palette.graphs[context.palette_index as usize].clone(),
                    ui,
                    ctx,
                    context,
                );
                redraw = true;
            } else if ToolMode::Pattern == context.mode {
                let patterns = PATTERNS.read().unwrap();
                let mut editor = NODEEDITOR.write().unwrap();
                editor.set_graph(
                    NodeContext::Color(context.palette_index),
                    patterns.graphs[context.palette_index as usize].clone(),
                    ui,
                    ctx,
                    context,
                );
                redraw = true;
            }
        }

        redraw
    }

    fn handle_event(
        &mut self,
        event: &TheEvent,
        ui: &mut TheUI,
        ctx: &mut TheContext,
        context: &mut Context,
    ) -> bool {
        let mut redraw = false;
        #[allow(clippy::single_match)]
        match event {
            TheEvent::PaletteIndexChanged(id, index) => {
                if id.name == "PalettePicker" {
                    if ToolMode::Palette == context.mode {
                        let palette = PALETTE.read().unwrap();
                        let mut editor = NODEEDITOR.write().unwrap();
                        editor.set_graph(
                            NodeContext::Color(*index as u8),
                            palette.graphs[*index as usize].clone(),
                            ui,
                            ctx,
                            context,
                        );
                        redraw = true
                    } else {
                        let mut editor = NODEEDITOR.write().unwrap();
                        editor.palette_index_changed(*index as u8, ui, ctx, context);
                    }
                } else if id.name == "PatternPicker" {
                    let patterns = PATTERNS.read().unwrap();
                    let mut editor = NODEEDITOR.write().unwrap();
                    editor.set_graph(
                        NodeContext::Pattern(*index as u8),
                        patterns.graphs[*index as usize].clone(),
                        ui,
                        ctx,
                        context,
                    );
                    redraw = true
                }
            }
            _ => {}
        }
        redraw
    }
}
