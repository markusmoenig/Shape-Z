// ---

use clap::{Command, arg};
use shapezlib::prelude::*;

fn cli() -> Command {
    Command::new("shpz")
        .about("Shape-Z. Compiles, renders or polygonizes '.shpz' source files.")
        .author("Markus Moenig")
        .version("0.1.0")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(arg!(<FILE> "The input '.shpz' file"))
        .arg_required_else_help(false)
        .subcommand(
            Command::new("render").about("Renders the input to an PNG image. Used by default."),
        )
        .subcommand(Command::new("polygonize").about("Polygonize the input to an OBJ file."))
}

fn main() {
    let matches = cli().get_matches();

    let file_name = matches.get_one::<String>("FILE").unwrap();
    // println!("file_name {}", file_name);

    let mut path = std::path::PathBuf::from(file_name);

    let size = Vec3::new(10, 4, 10);
    let density = 96;
    let iterations = 50;

    let camera: Arc<RwLock<Box<dyn Camera>>> = Arc::new(RwLock::new(Box::new(Iso::new())));
    let renderer: Arc<Box<dyn Renderer>> = Arc::new(Box::new(BSDF::new()));
    let mut buffer = Arc::new(Mutex::new(RenderBuffer::new(800, 800)));
    let tracer = Tracer::new();

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

    path.set_extension("png");

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
