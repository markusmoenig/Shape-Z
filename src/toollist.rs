use crate::prelude::*;
use crate::tools::brush::BrushTool;
use crate::tools::edit::EditTool;

pub struct ToolList {
    pub tools: Vec<Box<dyn Tool>>,

    pub curr_tool: String,
}

impl Default for ToolList {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolList {
    pub fn new() -> Self {
        let tools: Vec<Box<dyn Tool>> = vec![Box::new(EditTool::new()), Box::new(BrushTool::new())];
        Self {
            tools,
            curr_tool: "Edit Tool".into(),
        }
    }

    /// Add the tools
    pub fn add_tools(&mut self, list: &mut dyn TheVLayoutTrait, ctx: &mut TheContext) {
        ctx.ui.relayout = true;

        for tool in self.tools.iter() {
            let mut b = TheToolListButton::new(tool.id());

            b.set_icon_name(tool.icon_name());
            b.set_status_text(&tool.info());
            if tool.id().name == self.curr_tool {
                b.set_state(TheWidgetState::Selected);
            }
            list.add_widget(Box::new(b));
        }
    }

    pub fn handle_event(
        &mut self,
        event: &TheEvent,
        ui: &mut TheUI,
        ctx: &mut TheContext,
        context: &mut Context,
    ) -> bool {
        let mut redraw = false;
        match event {
            TheEvent::StateChanged(id, state) => {
                if id.name.contains("Tool") && *state == TheWidgetState::Selected {
                    redraw = self.set_tool(id.uuid, ui, ctx, context);
                }
            }
            _ => {}
        }

        redraw = self
            .get_current_tool()
            .handle_event(event, ui, ctx, context);

        redraw
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_tool(
        &mut self,
        tool_id: Uuid,
        ui: &mut TheUI,
        ctx: &mut TheContext,
        context: &mut Context,
    ) -> bool {
        let mut redraw = false;
        let mut switched_tool = false;
        let layout_name = "Tool Params";
        let mut old_tool_name = "".to_string();
        let mut old_tool_index = 0;
        for (index, tool) in self.tools.iter().enumerate() {
            if tool.id().uuid == tool_id && tool.id().name != self.curr_tool {
                switched_tool = true;
                old_tool_name = self.curr_tool.clone();
                old_tool_index = index;
                self.curr_tool = tool.id().name.clone();
                redraw = true;
            }
        }
        if switched_tool {
            for tool in self.tools.iter() {
                if tool.id().uuid != tool_id {
                    ctx.ui
                        .set_widget_state(tool.id().name.clone(), TheWidgetState::None);
                }
            }
            self.tools[old_tool_index].tool_event(ToolEvent::DeActivate, ui, ctx, context);
        }

        if let Some(layout) = ui.get_hlayout(layout_name) {
            layout.clear();
            layout.set_reverse_index(None);
            ctx.ui.redraw_all = true;
        }

        self.get_current_tool()
            .tool_event(ToolEvent::Activate, ui, ctx, context);

        ctx.ui.relayout = true;

        redraw
    }

    /// Returns the curently active tool.
    pub fn get_current_tool(&mut self) -> &mut Box<dyn Tool> {
        let curr_tool_name = &self.curr_tool;
        let len = self.tools.len();

        for i in 0..len {
            if self.tools[i].id().name == *curr_tool_name {
                return &mut self.tools[i];
            }
        }

        // fallback
        &mut self.tools[0]
    }
}
