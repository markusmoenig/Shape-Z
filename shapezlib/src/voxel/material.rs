//! material.rs  – one struct that covers **Disney + OpenPBR**
#![allow(clippy::upper_case_acronyms)]

use crate::prelude::*;

/// A material definition
#[derive(Clone, Copy, Debug)]
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
    pub coat_weight: f32,
    pub coat_color: Vec3<f32>,
    pub coat_roughness: f32,
    pub coat_ior: f32,

    pub thin_walled: bool,
    pub thin_film_thickness: Option<f32>,
    pub thin_film_ior: Option<f32>,

    pub specular_edge_color: Option<Vec3<f32>>,
    pub specular_weight: f32,

    pub transmission_depth: Option<f32>,
    pub volume_scatter_color: Option<Vec3<f32>>,
    pub volume_absorption_color: Option<Vec3<f32>>,
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
