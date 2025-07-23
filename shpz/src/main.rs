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
        .arg(
            arg!(-r --resolution <RES> "Output resolution")
                .required(false)
                .default_value("800x800"),
        )
        .subcommand(
            Command::new("render").about("Renders the input to an PNG image. Used by default."),
        )
        .subcommand(
            Command::new("polygonize")
                .about("Polygonize the input to an OBJ file.")
                .visible_alias("poly"),
        )
}

fn main() {
    let iterations = 500;

    let matches = cli().get_matches();

    // Read arguments
    let polygonize = matches.subcommand_name() == Some("polygonize");
    let (width, height) = matches
        .get_one::<String>("resolution")
        .map(|res| {
            res.split_once('x')
                .and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?)))
                .unwrap_or_else(|| {
                    eprintln!("Invalid resolution format. Use WIDTHxHEIGHT, e.g. 800x400.");
                    std::process::exit(1);
                })
        })
        .unwrap_or((800, 800));

    println!(
        "{} mode | Resolution: {}x{}",
        if polygonize { "Polygonize" } else { "Render" },
        width,
        height
    );

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
    let (voxels, mem) = shapez.stats();
    let _stop = shapez.get_time();
    println!(
        "Compiled {} voxels ({} MB) in {:.2} seconds.",
        voxels,
        mem,
        (_stop - _start) as f32 / 1000.0
    );

    if polygonize {
        shapez.write_obj();
    } else {
        // Render loop
        for i in 0..iterations {
            shapez.sample();

            if i % 10 == 0 {
                shapez.write_image();
            }
        }
    }
}
