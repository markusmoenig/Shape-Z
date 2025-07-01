//! material.rs  – one struct that covers **Disney + OpenPBR**
#![allow(clippy::upper_case_acronyms)]

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use theframework::prelude::*;
use vek::Vec3;

/// Passed to pattern evaluation.
#[derive(Clone, Debug)]
pub struct PatternContext {
    pub result: u8,

    pub cell_scale: f32,
    pub world: Vec3<f32>,
    pub uv: Vec2<i32>,
    pub normal: Vec3<f32>,
    pub layer: i32,
    pub max_layer: i32,
}

/// A palette that stores exactly 256 materials
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Patterns {
    pub graphs: Vec<NodeFXGraph>,
}

impl Default for Patterns {
    fn default() -> Self {
        let mut graphs = vec![];
        for _ in 0..256 {
            let mut graph = NodeFXGraph::default();
            let mut node = NodeFX::new(NodeFXRole::PatternUV);
            node.position = Vec2::new(10, 10);
            graph.nodes.push(node);

            graphs.push(graph);
        }

        Self { graphs }
    }
}

impl Patterns {
    /// Convenience accessor with bounds-check
    pub fn get(&self, index: u8) -> &NodeFXGraph {
        &self.graphs[index as usize]
    }
    /// Mutable accessor
    pub fn get_mut(&mut self, index: u8) -> &mut NodeFXGraph {
        &mut self.graphs[index as usize]
    }
}
