use crate::prelude::*;
use theframework::prelude::*;
use vek::Vec2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeFXGraph {
    pub id: Uuid,
    pub nodes: Vec<NodeFX>,

    /// The node connections: Source node index, source terminal, dest node index, dest terminal
    pub connections: Vec<(u16, u8, u16, u8)>,

    pub selected_node: Option<usize>,

    pub scroll_offset: Vec2<i32>,
    pub zoom: f32,
}

impl Default for NodeFXGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeFXGraph {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            nodes: vec![],
            connections: vec![],
            selected_node: None,
            scroll_offset: Vec2::zero(),
            zoom: 1.0,
        }
    }

    /// Evaluate a material graph
    pub fn evaluate_material(&self) -> Material {
        let mut material = Material::default();

        let mut curr_index = 0_usize;
        let mut curr_terminal = 0_usize;

        self.nodes[0].evaluate_material(&mut material, (self, 0));

        let mut steps = 0;
        while steps < 16 {
            if let Some((next_node, next_terminal)) =
                self.find_connected_input_node(curr_index, curr_terminal)
            {
                self.nodes[next_node as usize]
                    .evaluate_material(&mut material, (self, next_node as usize));
                curr_index = next_node as usize;
                curr_terminal = next_terminal as usize;
                steps += 1;
            } else {
                break;
            }
        }

        material
    }

    /// Evaluate a shape graph
    pub fn evaluate_shape(&self, hit: &HitRecord, context: &Context) -> VoxelGrid {
        let mut grid = VoxelGrid::default();

        let mut curr_index = 0_usize;
        let mut curr_terminal = 0_usize;

        self.nodes[0].evaluate_shape(&mut grid, hit, (self, 0), context);

        let mut steps = 0;
        while steps < 16 {
            if let Some((next_node, next_terminal)) =
                self.find_connected_input_node(curr_index, curr_terminal)
            {
                self.nodes[next_node as usize].evaluate_shape(
                    &mut grid,
                    hit,
                    (self, next_node as usize),
                    context,
                );
                curr_index = next_node as usize;
                curr_terminal = next_terminal as usize;
                steps += 1;
            } else {
                break;
            }
        }

        grid
    }

    /// Returns the connected input node and terminal for the given output node and terminal.
    pub fn find_connected_input_node(
        &self,
        node: usize,
        terminal_index: usize,
    ) -> Option<(u16, u8)> {
        for (o, ot, i, it) in &self.connections {
            if *o == node as u16 && *ot == terminal_index as u8 {
                return Some((*i, *it));
            }
        }
        None
    }

    /// Returns the connected output node for the given input node and terminal.
    pub fn find_connected_output_node(&self, node: usize, terminal_index: usize) -> Option<usize> {
        for (o, _, i, it) in &self.connections {
            if *i == node as u16 && *it == terminal_index as u8 {
                return Some(*o as usize);
            }
        }
        None
    }
}
