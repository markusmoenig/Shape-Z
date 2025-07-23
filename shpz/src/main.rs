// ---

use clap::{Command, arg};
use shapezlib::prelude::*;

fn cli() -> Command {
    Command::new("shpz")
        .about("Shape-Z. Compiles, renders or polygonizes '.shpz' source files.")
        .author("Markus Moenig")
        .version("0.1.0")
        .subcommand_required(false)
        .allow_external_subcommands(true)
        .arg(arg!([FILE] "The input '.shpz' file").default_value("main.shpz"))
        .subcommand(
            Command::new("render").about("Renders the input to an PNG image. Used by default."),
        )
        .subcommand(Command::new("polygonize").about("Polygonize the input to an OBJ file."))
}

fn main() {
    let iterations = 50;

    let matches = cli().get_matches();

    let file_name = matches.get_one::<String>("FILE").unwrap();
    // println!("file_name {}", file_name);

    let path = std::path::PathBuf::from(file_name);

    let mut shapez = ShapeZ::default();
    let module = match shapez.compile(path.clone()) {
        Ok(module) => module,
        Err(e) => {
            eprintln!("Error compiling module: {}", e);
            return;
        }
    };

    println!("Module '{}' compiled successfully.", module.name);

    // Compile the voxels

    let _start: u128 = shapez.get_time();
    shapez.execute();
    let _stop = shapez.get_time();
    println!("Compile time: {:?} ms.", _stop - _start);

    // Render loop

    for i in 0..iterations {
        shapez.sample();

        if i % 10 == 0 {
            shapez.write_image();
        }
    }
}
