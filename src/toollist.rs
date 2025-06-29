use crate::prelude::*;
use crate::tools::brush::BrushTool;
use crate::tools::edit::EditTool;

use crate::editor::{CAMERA, VOXELGRID};
use std::sync::Arc;

pub struct ToolList {
    pub tools: Vec<Box<dyn Tool>>,

    pub curr_tool: String,

    drag_coord: Vec2<i32>,
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
            drag_coord: Vec2::zero(),
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

    pub fn get_hit(&self, ui: &mut TheUI, coord: Vec2<i32>) -> Option<HitRecord> {
        if let Some(render_view) = ui.get_render_view("ModelView") {
            let dim = *render_view.dim();

            let uv = Vec2::new(
                coord.x as f32 / dim.width as f32,
                1.0 - (coord.y as f32 / dim.height as f32),
            );
            let camera = Arc::clone(&CAMERA);
            let camera = camera.write().unwrap();
            let ray = camera.create_ray(
                uv,
                Vec2::new(dim.width as f32, dim.height as f32),
                Vec2::zero(),
            );

            // --

            let grid = Arc::clone(&VOXELGRID);
            let mut grid = grid.write().unwrap();

            grid.preview = None;
            let hit = grid.dda(&ray);

            Some(hit)
        } else {
            None
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
            TheEvent::KeyDown(TheValue::Char(c)) => {
                let acc = !ui.focus_widget_supports_text_input(ctx);

                if acc {
                    let mut tool_uuid = None;
                    for (index, tool) in self.tools.iter().enumerate() {
                        if tool.accel() == Some(*c) {
                            tool_uuid = Some(tool.id().uuid);
                            // ctx.ui.set_widget_state(
                            //     self.tools[index].id().name,
                            //     TheWidgetState::None,
                            // );
                            ctx.ui
                                .set_widget_state(tool.id().name, TheWidgetState::Selected);
                        }
                    }
                    if let Some(uuid) = tool_uuid {
                        self.set_tool(uuid, ui, ctx, context);
                    }
                }
            }
            TheEvent::RenderViewHoverChanged(id, coord) => {
                if id.name == "ModelView" {
                    if let Some(render_view) = ui.get_render_view("ModelView") {
                        let dim = *render_view.dim();

                        let uv = Vec2::new(
                            coord.x as f32 / dim.width as f32,
                            1.0 - (coord.y as f32 / dim.height as f32),
                        );
                        let camera = Arc::clone(&CAMERA);
                        let mut camera = camera.write().unwrap();
                        let ray = camera.create_ray(
                            uv,
                            Vec2::new(dim.width as f32, dim.height as f32),
                            Vec2::zero(),
                        );

                        // --

                        if ui.alt {
                            camera.zoom((*coord - self.drag_coord).y as f32);
                        } else if ui.logo || ui.ctrl {
                            camera.rotate((*coord - self.drag_coord).map(|v| -v as f32 * 2.0));
                            self.drag_coord = *coord;
                        } else {
                            let hit;

                            {
                                let grid = Arc::clone(&VOXELGRID);
                                let mut grid = grid.write().unwrap();

                                grid.preview = None;
                                hit = grid.dda(&ray);
                            }

                            self.get_current_tool().tool_event(
                                ToolEvent::HitHover(hit),
                                ui,
                                ctx,
                                context,
                            );
                        }
                        crate::utils::reset_render();
                    }
                }
            }
            TheEvent::RenderViewClicked(id, _) => {
                if id.name == "ModelView" {
                    self.get_current_tool()
                        .tool_event(ToolEvent::HitClick, ui, ctx, context);
                }
            }
            TheEvent::RenderViewDragged(id, coord) => {
                if id.name == "ModelView" {
                    if let Some(hit) = self.get_hit(ui, *coord) {
                        self.get_current_tool().tool_event(
                            ToolEvent::HitDrag(hit),
                            ui,
                            ctx,
                            context,
                        );
                        crate::utils::reset_render();
                    }
                }
            }
            _ => {}
        }

        if !redraw {
            redraw = self
                .get_current_tool()
                .handle_event(event, ui, ctx, context);
        }

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
        let mut old_tool_index = 0;
        for (index, tool) in self.tools.iter().enumerate() {
            if tool.id().uuid == tool_id && tool.id().name != self.curr_tool {
                switched_tool = true;
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
