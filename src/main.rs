pub mod ast;
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

    pub use crate::ast::context::Context;
    pub use crate::ast::environment::Environment;
    pub use crate::ast::error::{ParseError, RuntimeError};
    pub use crate::ast::execute::ExecuteVisitor;
    pub use crate::ast::idverifier::IdVerifier;
    pub use crate::ast::module::Module;
    pub use crate::ast::parser::Parser;
    pub use crate::ast::scanner::{Scanner, Token, TokenType};
    pub use crate::ast::value::*;
    pub use crate::ast::*;

    pub use crate::voxel::brushshape::BrushShape;
    pub use crate::voxel::camera::Camera;
    pub use crate::voxel::camera::iso::Iso;
    pub use crate::voxel::camera::orbit::Orbit;
    pub use crate::voxel::camera::pinhole::Pinhole;
    pub use crate::voxel::grid::VoxelGrid;
    pub use crate::voxel::palette::{Material, Palette};
    pub use crate::voxel::ray::Ray;
    pub use crate::voxel::rect::VoxelRect;
    pub use crate::voxel::renderbuffer::RenderBuffer;
    pub use crate::voxel::renderer::Renderer;
    pub use crate::voxel::renderer::model::Model;
    pub use crate::voxel::renderer::pbr::PBR;
    pub use crate::voxel::tile::Tile;
    pub use crate::voxel::{Coord, Face, HitRecord, HitType};
}

// ---

use prelude::*;

fn main() {
    let mut path = std::path::PathBuf::new();
    path.push("main.shpz");

    let camera: Arc<RwLock<Box<dyn Camera>>> = Arc::new(RwLock::new(Box::new(Iso::new())));
    let renderer: Arc<Box<dyn Renderer>> = Arc::new(Box::new(PBR::new()));
    let mut buffer = Arc::new(Mutex::new(RenderBuffer::new(800, 800)));
    let grid = Arc::new(RwLock::new(VoxelGrid::default()));
    let palette = Arc::new(RwLock::new(Palette::default()));
    let tracer = tracer::Tracer::new();

    {
        // let mut c = camera.write().unwrap();
        // c.set_origin(Vec3::new(0.0, 0.0, 2.0));
        // c.set_center(Vec3::zero());
    }

    let mut parser = Parser::new();
    match parser.compile(path.clone()) {
        Ok(module) => {
            println!("Module compiled successfully: {:?}", module.name);
        }
        Err(e) => {
            eprintln!("Error compiling module: {}", e);
            return;
        }
    }

    {
        let mut grid = grid.write().unwrap();
        let mut palette = palette.write().unwrap();
        palette.materials[1].base_color = Vec3::new(1.0, 0.0, 0.0);

        let rect = VoxelRect {
            origin: Vec3::new(0.0, 0.0, 0.0),
            size: Vec3::new(1.0, 1.0, 1.0),
        };

        rect.fill(&mut grid, 1); // Fill with material ID 1
        grid.update_bboxes();
    }

    tracer.render(&mut buffer, &grid, &palette, &renderer, &camera);

    let b = buffer.lock().unwrap();

    let mut path = std::path::PathBuf::new();
    path.push("main.png");

    b.save_srgb(path);
}
