pub mod ast;
pub mod node;
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
    pub use uuid::Uuid;

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
    pub use crate::voxel::material::Material;
    pub use crate::voxel::ray::Ray;
    pub use crate::voxel::rect::VoxelRect;
    pub use crate::voxel::renderbuffer::RenderBuffer;
    pub use crate::voxel::renderer::Renderer;
    pub use crate::voxel::renderer::model::Model;
    pub use crate::voxel::renderer::pbr::PBR;
    pub use crate::voxel::tile::Tile;
    pub use crate::voxel::{Coord, Face, HitRecord, HitType, Voxel};
}

// ---

use prelude::*;

fn main() {
    let mut path = std::path::PathBuf::new();
    path.push("main.shpz");

    let size = Vec3::new(10, 4, 10);
    let density = 96;
    let iterations = 50;

    let camera: Arc<RwLock<Box<dyn Camera>>> = Arc::new(RwLock::new(Box::new(Iso::new())));
    let renderer: Arc<Box<dyn Renderer>> = Arc::new(Box::new(PBR::new()));
    let mut buffer = Arc::new(Mutex::new(RenderBuffer::new(800, 800)));
    // let palette = Arc::new(RwLock::new(Palette::default()));
    let tracer = tracer::Tracer::new();

    // palette.write().unwrap().materials[0].base_color = Vec3::new(1.0, 0.0, 0.0);
    // palette.write().unwrap().materials[1].base_color = Vec3::new(0.0, 1.0, 0.0);
    // palette.write().unwrap().materials[2].base_color = Vec3::new(0.0, 0.0, 1.0);

    {
        // let mut c = camera.write().unwrap();
        // c.set_origin(Vec3::new(0.0, 0.0, 2.0));
        // c.set_center(Vec3::zero());
    }

    // Parse and compile

    let mut parser = Parser::new();
    let module = match parser.compile(path.clone()) {
        Ok(module) => module,
        Err(e) => {
            eprintln!("Error compiling module: {}", e);
            return;
        }
    };

    // Compile the AST

    let mut visitor = CompileVisitor::new();
    let mut ctx = Context::new(size, density, module.variables);

    // println!("{:?}", ctx.variables);

    for statement in module.stmts {
        match statement.accept(&mut visitor, &mut ctx) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
    }

    println!("Module '{}' compiled successfully.", module.name);

    // Model by executing the VM

    let _start: u128 = tracer.get_time();

    let mut execution = Execution::new(ctx.variables.len());
    execution.execute(&ctx.program.globals.clone(), &mut ctx.program);

    let materials: Arc<RwLock<Vec<Vec<NodeOp>>>> =
        Arc::new(RwLock::new(ctx.materials.values().cloned().collect()));
    ctx.program.grid.write().unwrap().update_bboxes();

    let _stop = tracer.get_time();
    println!("Compile time: {:?} ms.", _stop - _start);

    // Render loop

    let mut path = std::path::PathBuf::new();
    path.push("main.png");

    for i in 0..iterations {
        // Render the output grid

        tracer.render(
            &mut buffer,
            &ctx.program.grid,
            &materials,
            &renderer,
            &camera,
        );

        if i % 10 == 0 {
            let b = buffer.lock().unwrap();
            b.save_srgb(path.clone());
        }
    }
}
