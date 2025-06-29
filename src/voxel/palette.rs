//! material.rs  – one struct that covers **Disney + OpenPBR**
#![allow(clippy::upper_case_acronyms)]

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use theframework::prelude::*;
use vek::Vec3;

/// A material definition
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Material {
    /* ─────── Disney “principled” core ─────── */
    pub base_color: Vec3<f32>,
    pub subsurface: f32,
    pub metallic: f32,
    pub specular: f32,
    pub specular_tint: f32,
    pub roughness: f32,
    pub anisotropic: f32,
    pub anisotropic_rot: f32,
    pub sheen: f32,
    pub sheen_tint: f32,
    pub clearcoat: f32,
    pub clearcoat_gloss: f32,
    pub ior: f32,
    pub transmission: f32,
    pub transmission_roughness: f32,
    pub emission_color: Vec3<f32>,
    pub emission_strength: f32,

    /* ─────── Optional OpenPBR extensions ─────── */
    #[serde(default)]
    pub coat_weight: f32,
    #[serde(default)]
    pub coat_color: Vec3<f32>,
    #[serde(default)]
    pub coat_roughness: f32,
    #[serde(default)]
    pub coat_ior: f32,

    #[serde(default)]
    pub thin_walled: bool,
    #[serde(default)]
    pub thin_film_thickness: Option<f32>,
    #[serde(default)]
    pub thin_film_ior: Option<f32>,

    #[serde(default)]
    pub specular_edge_color: Option<Vec3<f32>>,
    #[serde(default)]
    pub specular_weight: f32,

    #[serde(default)]
    pub transmission_depth: Option<f32>,
    #[serde(default)]
    pub volume_scatter_color: Option<Vec3<f32>>,
    #[serde(default)]
    pub volume_absorption_color: Option<Vec3<f32>>,
    #[serde(default)]
    pub volume_anisotropy: Option<f32>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            /* Disney defaults – match Blender’s “Principled BSDF” */
            base_color: Vec3::one(),
            subsurface: 0.0,
            metallic: 0.0,
            specular: 0.5,
            specular_tint: 0.0,
            roughness: 0.5,
            anisotropic: 0.0,
            anisotropic_rot: 0.0,
            sheen: 0.0,
            sheen_tint: 0.5,
            clearcoat: 0.0,
            clearcoat_gloss: 1.0,
            ior: 1.45,
            transmission: 0.0,
            transmission_roughness: 0.0,
            emission_color: Vec3::zero(),
            emission_strength: 0.0,

            /* OpenPBR extras – all zero / None */
            coat_weight: 0.0,
            coat_color: Vec3::one(),
            coat_roughness: 0.1,
            coat_ior: 1.5,

            thin_walled: false,
            thin_film_thickness: None,
            thin_film_ior: None,

            specular_edge_color: None,
            specular_weight: 1.0,

            transmission_depth: None,
            volume_scatter_color: None,
            volume_absorption_color: None,
            volume_anisotropy: None,
        }
    }
}

impl Material {
    pub fn base_color_linear(&self) -> Vec3<F> {
        fn srgb_to_linear(c: F) -> F {
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }

        Vec3::new(
            srgb_to_linear(self.base_color.x),
            srgb_to_linear(self.base_color.y),
            srgb_to_linear(self.base_color.z),
        )
    }
}

/// A palette that stores exactly 256 materials
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Palette {
    pub materials: Vec<Material>,
    pub graphs: Vec<NodeFXGraph>,
}

impl Default for Palette {
    fn default() -> Self {
        let mut graphs = vec![];
        for _ in 0..256 {
            let mut graph = NodeFXGraph::default();
            let mut node = NodeFX::new(NodeFXRole::Color);
            node.position = Vec2::new(10, 10);
            graph.nodes.push(node);
            graphs.push(graph);
        }

        Self {
            materials: vec![Material::default(); 256],
            graphs,
        }
    }
}

impl Palette {
    /// Convenience accessor with bounds-check
    pub fn get(&self, index: u8) -> &Material {
        &self.materials[index as usize]
    }
    /// Mutable accessor
    pub fn get_mut(&mut self, index: u8) -> &mut Material {
        &mut self.materials[index as usize]
    }

    /// Fill this palette from the text of a Paint-NET “.txt” palette file.
    pub fn load_paintnet(&mut self, src: &str) -> std::io::Result<()> {
        use std::io::{Error, ErrorKind};

        // keep the old materials’ *other* fields
        let mut idx = 0usize;

        for line in src.lines() {
            let line = line.trim();

            // skip comments / empty lines
            if line.is_empty() || line.starts_with(';') {
                continue;
            }

            // “AARRGGBB” → u32
            if line.len() != 8 || !line.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("invalid colour: {line}"),
                ));
            }
            let argb = u32::from_str_radix(line, 16)
                .map_err(|_| Error::new(ErrorKind::InvalidData, "hex parse"))?;

            // unpack & convert to linear 0-1 f32
            // let a = ((argb >> 24) & 0xFF) as f32 / 255.0;
            let r = ((argb >> 16) & 0xFF) as f32 / 255.0;
            let g = ((argb >> 8) & 0xFF) as f32 / 255.0;
            let b = (argb & 0xFF) as f32 / 255.0;

            if idx < self.materials.len() {
                self.materials[idx].base_color = Vec3::new(r, g, b);
                self.graphs[idx].nodes[0].values[0] = r;
                self.graphs[idx].nodes[0].values[1] = g;
                self.graphs[idx].nodes[0].values[2] = b;

                idx += 1;
            } else {
                break; // palette already full
            }
        }

        Ok(())
    }
}
