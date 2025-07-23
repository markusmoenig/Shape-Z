pub mod ast;
pub mod node;
pub mod shapez;
pub mod tracer;
pub mod voxel;

pub type F = f32;
pub const F_PI: F = std::f32::consts::PI;
pub const F_TAU: F = std::f32::consts::TAU;
pub const F_FRAC_PI_2: F = std::f32::consts::FRAC_PI_2;
pub const F_FRAC_1_PI: F = std::f32::consts::FRAC_1_PI;
pub const F_E: F = std::f32::consts::E;
pub const F_SQRT_2: F = std::f32::consts::SQRT_2;
pub const F_MIN: F = f32::MIN;
pub const F_MAX: F = f32::MAX;

pub type Color = [F; 4];

#[allow(ambiguous_glob_reexports)]
pub mod prelude {

    pub use crate::{Color, F};
    pub use rustc_hash::{FxHashMap, FxHashSet};
    pub use std::sync::{Arc, Mutex};
    pub use std::sync::{LazyLock, RwLock};

    pub use vek::{Aabb, Vec2, Vec3, Vec4};

    pub use indexmap::IndexMap;

    pub use crate::shapez::ShapeZ;
    pub use crate::tracer::Tracer;

    pub use crate::node::execution::*;
    pub use crate::node::program::*;
    pub use crate::node::value::Value;
    pub use crate::node::*;

    pub use crate::ast::compile::CompileVisitor;
    pub use crate::ast::context::Context;
    pub use crate::ast::environment::Environment;
    pub use crate::ast::error::{ParseError, RuntimeError};
    pub use crate::ast::idverifier::IdVerifier;
    pub use crate::ast::module::Module;
    pub use crate::ast::obectd::*;
    pub use crate::ast::parser::Parser;
    pub use crate::ast::scanner::{Scanner, Token, TokenType};
    pub use crate::ast::value::*;
    pub use crate::ast::*;

    pub use crate::voxel::camera::Camera;
    pub use crate::voxel::camera::iso::Iso;
    pub use crate::voxel::camera::orbit::Orbit;
    pub use crate::voxel::camera::pinhole::Pinhole;
    pub use crate::voxel::grid::VoxelGrid;
    pub use crate::voxel::ray::Ray;
    pub use crate::voxel::rect::VoxelRect;
    pub use crate::voxel::renderbuffer::RenderBuffer;
    pub use crate::voxel::renderer::Renderer;
    pub use crate::voxel::renderer::bsdf::BSDF;
    pub use crate::voxel::renderer::bsdf_helper::*;
    pub use crate::voxel::renderer::pbr::PBR;
    pub use crate::voxel::renderer::raw::Raw;
    pub use crate::voxel::tile::Tile;
    pub use crate::voxel::{Coord, Face, HitRecord, HitType, Voxel};
}
