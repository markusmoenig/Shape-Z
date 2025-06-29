use crate::{editor::PALETTE, prelude::*};

use NodeFXParam::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeContext {
    Empty,
    Color(u8),
    Shape(usize),
}

pub struct NodeEditor {
    pub context: NodeContext,
    pub graph: NodeFXGraph,

    pub categories: FxHashMap<String, TheColor>,
}

#[allow(clippy::new_without_default)]
impl NodeEditor {
    pub fn new() -> Self {
        let mut categories: FxHashMap<String, TheColor> = FxHashMap::default();
        categories.insert("ShapeFX".into(), TheColor::from("#c49a00")); // Warm gold — represents pixel-level artistic material control
        categories.insert("Render".into(), TheColor::from("#e53935")); // Bright red — strong signal for core rendering pipeline
        categories.insert("Modifier".into(), TheColor::from("#00bfa5")); // Teal green — evokes transformation, procedural mesh edits
        categories.insert("FX".into(), TheColor::from("#7e57c2")); // Purple — expressive and magical for particles, screen fx, etc.
        categories.insert("Shape".into(), TheColor::from("#4285F4")); // Vivid blue — represents structure, geometric form, and stability

        Self {
            context: NodeContext::Empty,
            graph: NodeFXGraph::default(),
            categories,
        }
    }

    /// Set the context and graph.
    pub fn set_graph(
        &mut self,
        context: NodeContext,
        graph: NodeFXGraph,
        ui: &mut TheUI,
        ctx: &mut TheContext,
    ) {
        self.context = context;
        self.graph = graph;

        if self.graph.selected_node.is_none() {
            self.graph.selected_node = Some(0);
        }

        self.set_node_ui(ui, ctx);
        let canvas = self.to_canvas();
        ui.set_node_canvas("NodeView", canvas);
    }

    /// Graph has been changed, apply it to the current context
    fn graph_changed(&self, ui: &mut TheUI, ctx: &mut TheContext) {
        #[allow(clippy::single_match)]
        match self.context {
            NodeContext::Color(index) => {
                {
                    let mut palette = PALETTE.write().unwrap();
                    palette.graphs[index as usize] = self.graph.clone();
                    palette.materials[index as usize] = self.graph.evaluate_material();
                }
                crate::utils::update_palette_ui(ui, ctx);
            }
            _ => {}
        }
    }

    /// Create the NodeCanvas for display
    pub fn to_canvas(&mut self) -> TheNodeCanvas {
        let mut canvas = TheNodeCanvas {
            node_width: 136,
            selected_node: self.graph.selected_node,
            offset: self.graph.scroll_offset,
            connections: self.graph.connections.clone(),
            categories: self.categories.clone(),
            ..Default::default()
        };

        // let width = 111;
        // let height = 104;

        // let mut buffer = TheRGBABuffer::new(TheDim::sized(width as i32, height));

        for (index, node) in self.graph.nodes.iter().enumerate() {
            let n = TheNode {
                name: node.name(),
                position: node.position,
                inputs: node.inputs(),
                outputs: node.outputs(),
                preview: TheRGBABuffer::default(),
                supports_preview: true,
                preview_is_open: true,
                can_be_deleted: index != 0,
            };
            canvas.nodes.push(n);
        }

        canvas
    }

    #[allow(clippy::too_many_arguments)]
    pub fn handle_event(
        &mut self,
        event: &TheEvent,
        ui: &mut TheUI,
        ctx: &mut TheContext,
        context: &mut Context,
    ) -> bool {
        let redraw = false;
        #[allow(clippy::single_match)]
        match event {
            /*
            TheEvent::ContextMenuSelected(id, item) => {
                if (id.name == "ShapeFX Nodes"
                    || id.name == "Modifier Nodes"
                    || id.name == "Render Nodes"
                    || id.name == "FX Nodes"
                    || id.name == "Shape Nodes")
                    && !self.graph.nodes.is_empty()
                {
                    if let Ok(role) = item.name.parse::<ShapeFXRole>() {
                        let mut effect = ShapeFX::new(role);

                        effect.position = Vec2::new(
                            self.graph.scroll_offset.x + 220,
                            self.graph.scroll_offset.y + 10,
                        );
                        self.graph.nodes.push(effect);
                        self.graph.selected_node = Some(self.graph.nodes.len() - 1);

                        let canvas = self.to_canvas();
                        ui.set_node_canvas("ShapeFX NodeCanvas", canvas);

                        if let Some(map) = project.get_map_mut(server_ctx) {
                            map.shapefx_graphs.insert(self.graph.id, self.graph.clone());
                        }
                        self.set_selected_node_ui(project, ui, ctx, true);
                    }
                }
            }
            TheEvent::StateChanged(id, TheWidgetState::Clicked) => {
                if id.name == "Create Graph Button" {
                    // println!("{:?}", server_ctx.curr_map_context);
                    //
                    if server_ctx.curr_map_context == MapContext::Material
                        || server_ctx.profile_view.is_some()
                    {
                        {
                            self.graph = ShapeFXGraph {
                                nodes: vec![ShapeFX::new(ShapeFXRole::MaterialGeometry)],
                                ..Default::default()
                            };
                            self.context = NodeContext::Material;
                        }
                    } else if server_ctx.curr_map_context == MapContext::Character
                        || server_ctx.curr_map_context == MapContext::Item
                    {
                        self.graph = ShapeFXGraph {
                            nodes: vec![ShapeFX::new(ShapeFXRole::MaterialGeometry)],
                            ..Default::default()
                        };
                        self.context = NodeContext::Shape;
                    } else if self.context == NodeContext::Region {
                        if server_ctx.curr_map_tool_type == MapToolType::Sector {
                            self.graph = ShapeFXGraph {
                                nodes: vec![ShapeFX::new(ShapeFXRole::SectorGeometry)],
                                ..Default::default()
                            };
                        } else if server_ctx.curr_map_tool_type == MapToolType::Linedef {
                            self.graph = ShapeFXGraph {
                                nodes: vec![ShapeFX::new(ShapeFXRole::LinedefGeometry)],
                                ..Default::default()
                            };
                        }
                        self.context = NodeContext::Region;
                    }

                    let canvas = self.to_canvas();
                    ui.set_node_canvas("ShapeFX NodeCanvas", canvas);
                    self.graph_changed(project, ui, ctx, server_ctx);
                }
            }*/
            TheEvent::NodeSelectedIndexChanged(id, index) => {
                if id.name == "NodeView" {
                    self.graph.selected_node = *index;
                    self.set_node_ui(ui, ctx);
                    // if let Some(map) = project.get_map_mut(server_ctx) {
                    //     map.changed += 1;
                    //     map.shapefx_graphs.insert(self.graph.id, self.graph.clone());
                    // }
                }
            }
            TheEvent::NodeDragged(id, index, position) => {
                if id.name == "NodeView" {
                    self.graph.nodes[*index].position = *position;
                    // if let Some(map) = project.get_map_mut(server_ctx) {
                    //     // let prev = map.clone();
                    //     map.changed += 1;
                    //     map.shapefx_graphs.insert(self.graph.id, self.graph.clone());
                    // }
                }
            }
            TheEvent::NodeConnectionAdded(id, connections)
            | TheEvent::NodeConnectionRemoved(id, connections) => {
                if id.name == "NodeView" {
                    // self.graph.connections.clone_from(connections);
                    // self.graph_changed(project, ui, ctx, server_ctx);
                }
            }
            TheEvent::NodeDeleted(id, deleted_node_index, connections) => {
                if id.name == "NodeView" {
                    // self.graph.nodes.remove(*deleted_node_index);
                    // self.graph.connections.clone_from(connections);

                    // self.graph_changed(project, ui, ctx, server_ctx);
                }
            }
            TheEvent::NodeViewScrolled(id, offset) => {
                if id.name == "NodeView" {
                    self.graph.scroll_offset = *offset;
                }
            }
            TheEvent::ValueChanged(id, value) => {
                if id.name.starts_with("nodefx") {
                    let snake_case = self.transform_to_snake_case(&id.name, "nodefx");
                    if let Some(index) = self.graph.selected_node {
                        if let Some(node) = self.graph.nodes.get_mut(index) {
                            match value {
                                TheValue::FloatRange(v, _) => {
                                    // node.values.set(&snake_case, rusterix::Value::Float(*v))
                                }
                                TheValue::IntRange(v, _) => {
                                    // node.values.set(&snake_case, rusterix::Value::Int(*v))
                                }
                                TheValue::Int(v) => {
                                    // node.values.set(&snake_case, rusterix::Value::Int(*v))
                                }
                                TheValue::ColorObject(v) => {
                                    node.values[0] = v.r;
                                    node.values[1] = v.g;
                                    node.values[2] = v.b;
                                }
                                _ => {}
                            }
                        }
                        self.graph_changed(ui, ctx);
                    }
                }
            }
            _ => {}
        }

        redraw
    }

    /// Create the UI for the selected node.
    pub fn set_node_ui(&mut self, ui: &mut TheUI, ctx: &mut TheContext) {
        let mut nodeui = TheNodeUI::default();
        let mut node_name = "Node".to_string();

        if let Some(index) = self.graph.selected_node {
            if let Some(node) = self.graph.nodes.get(index) {
                node_name = node.name();
                for param in node.params() {
                    match param {
                        Float(id, name, status, value, range) => {
                            let item = TheNodeUIItem::FloatEditSlider(
                                format!(
                                    "nodefx{}",
                                    id.get(0..1).unwrap_or("").to_uppercase()
                                        + id.get(1..).unwrap_or("")
                                ),
                                name.clone(),
                                status.clone(),
                                value,
                                range,
                                false,
                            );
                            nodeui.add_item(item);
                        }
                        Int(id, name, status, value, range) => {
                            let item = TheNodeUIItem::IntEditSlider(
                                format!(
                                    "nodefx{}",
                                    id.get(0..1).unwrap_or("").to_uppercase()
                                        + id.get(1..).unwrap_or("")
                                ),
                                name.clone(),
                                status.clone(),
                                value,
                                range,
                                false,
                            );
                            nodeui.add_item(item);
                        }
                        NodeFXParam::Color(id, name, status, value) => {
                            let item = TheNodeUIItem::ColorPicker(
                                format!(
                                    "nodefx{}",
                                    id.get(0..1).unwrap_or("").to_uppercase()
                                        + id.get(1..).unwrap_or("")
                                ),
                                name.clone(),
                                status.clone(),
                                value,
                                false,
                            );
                            nodeui.add_item(item);
                        }
                        // PaletteIndex(id, name, status, value) => {
                        //     let item = TheNodeUIItem::PaletteSlider(
                        //         format!(
                        //             "shapefx{}",
                        //             id.get(0..1).unwrap_or("").to_uppercase()
                        //                 + id.get(1..).unwrap_or("")
                        //         ),
                        //         name.clone(),
                        //         status.clone(),
                        //         value,
                        //         project.palette.clone(),
                        //         false,
                        //     );
                        //     nodeui.add_item(item);
                        // }
                        Selector(id, name, status, options, value) => {
                            let item = TheNodeUIItem::Selector(
                                format!(
                                    "nodefx{}",
                                    id.get(0..1).unwrap_or("").to_uppercase()
                                        + id.get(1..).unwrap_or("")
                                ),
                                name.clone(),
                                status.clone(),
                                options.clone(),
                                value,
                            );
                            nodeui.add_item(item);
                        }
                        _ => {}
                    }
                }
            }
        }

        if let Some(layout) = ui.get_text_layout("Node Settings") {
            nodeui.apply_to_text_layout(layout);
            // layout.relayout(ctx);
            ctx.ui.relayout = true;
        }
    }

    fn transform_to_snake_case(&self, input: &str, strip_prefix: &str) -> String {
        // Strip the prefix if it exists
        let stripped = if let Some(remainder) = input.strip_prefix(strip_prefix) {
            remainder
        } else {
            input
        };

        // Convert to snake_case
        let mut snake_case = String::new();
        for (i, c) in stripped.chars().enumerate() {
            if c.is_uppercase() {
                // Add an underscore before uppercase letters (except for the first character)
                if i > 0 {
                    snake_case.push('_');
                }
                snake_case.push(c.to_ascii_lowercase());
            } else {
                snake_case.push(c);
            }
        }

        snake_case
    }
}
