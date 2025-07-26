use clap::{Command, arg};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use shapezlib::prelude::*;
use std::collections::HashSet;
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, channel},
    },
    time::Duration,
};

fn cli() -> Command {
    Command::new("shpz")
        .about("Shape-Z - Compile, render or polygonize '.shpz' files.")
        .author("Markus Moenig")
        .version("0.1.0")
        .allow_external_subcommands(true)
        .arg(arg!([FILE] "Input '.shpz' file").default_value("main.shpz"))
        .arg(
            arg!(-r --resolution <RES> "Output resolution (WIDTHxHEIGHT)").default_value("800x800"),
        )
        .arg(
            arg!(-i --iter <N> "Path-tracing iterations")
                .default_value("50")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(arg!(--watch "Watches the source file and recompiles on change"))
        .subcommand(Command::new("render").about("Render to PNG (default)"))
        .subcommand(
            Command::new("polygonize")
                .about("Polygonize to OBJ")
                .visible_alias("poly"),
        )
}

/// Render pass
fn run_render(
    path: &PathBuf,
    polygonize: bool,
    width: usize,
    height: usize,
    iterations: usize,
    running: &AtomicBool,
    rx: &Receiver<notify::Result<notify::Event>>,
) -> (bool, Vec<PathBuf>) {
    let mut shapez = ShapeZ::default();
    if width != 800 || height != 800 {
        shapez.set_resolution(width, height);
    }

    let _module = match shapez.parse(path.clone()) {
        Ok(module) => match shapez.compile(&module) {
            Ok(()) => {
                println!("Module '{}' compiled successfully.", module.name);
            }
            Err(e) => {
                eprintln!("Error compiling module: {e}");
                return (false, vec![path.clone()]);
            }
        },
        Err(e) => {
            eprintln!("Error parsing module: {e}");
            return (false, vec![path.clone()]);
        }
    };

    // Collect files to watch: main + imports
    let mut watched = Vec::new();
    watched.push(path.clone());
    watched.extend(shapez.imported_paths().iter().cloned());

    let t0 = shapez.get_time();
    shapez.execute();
    let (voxels, mem) = shapez.stats();
    let t1 = shapez.get_time();
    println!(
        "Compiled {voxels} voxels ({mem} MB) in {:.2}s",
        (t1 - t0) as f32 / 1000.0
    );

    if polygonize {
        shapez.write_obj();
        return (false, watched);
    }

    for i in 0..iterations {
        if let Ok(Ok(ev)) = rx.try_recv() {
            if matches!(ev.kind, EventKind::Modify(_)) {
                running.store(false, Ordering::SeqCst);
            }
        }

        if !running.load(Ordering::SeqCst) {
            println!("File changed. Recompiling ...");
            return (true, watched);
        }

        shapez.sample();
        if i % 5 == 0 {
            shapez.write_image();
        }
    }

    (false, watched)
}

/// Watcher
fn watch_and_render(
    path: PathBuf,
    polygonize: bool,
    width: usize,
    height: usize,
    iterations: usize,
) {
    // shared channel from watcher to main thread
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_millis(300)),
    )
    .expect("Failed to create file watcher");
    watcher
        .watch(&path, RecursiveMode::NonRecursive)
        .expect("Failed to watch file");

    // track currently watched files
    let mut currently_watched: HashSet<PathBuf> = HashSet::from([path.clone()]);

    loop {
        // Ensure no stale events remain
        while rx.try_recv().is_ok() {}

        let running = Arc::new(AtomicBool::new(true));
        let (interrupted, watched_files) =
            run_render(&path, polygonize, width, height, iterations, &running, &rx);

        // Reconfigure watcher set (unwatch removed, watch new)
        let desired: HashSet<PathBuf> = watched_files.into_iter().collect();

        // Unwatch removed
        for p in currently_watched.difference(&desired) {
            let _ = watcher.unwatch(p);
        }
        // Watch new
        for p in desired.difference(&currently_watched) {
            let _ = watcher.watch(p, RecursiveMode::NonRecursive);
        }
        currently_watched = desired;

        if interrupted {
            // a change happened during the render – restart immediately
            continue;
        }

        // Finished all iterations – waiting for changes
        println!("Ready - Waiting ...");
        loop {
            match rx.recv() {
                Ok(Ok(ev)) if matches!(ev.kind, EventKind::Modify(_)) => break,
                Ok(Err(e)) => eprintln!("Watch error: {e:?}"),
                _ => {}
            }
        }
    }
}

fn main() {
    let matches = cli().get_matches();

    let iterations = *matches.get_one::<usize>("iter").unwrap();
    let polygonize = matches.subcommand_name() == Some("polygonize");
    let watch = matches.get_flag("watch");

    let (width, height) = matches
        .get_one::<String>("resolution")
        .and_then(|r| {
            r.split_once('x')
                .and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?)))
        })
        .unwrap_or((800, 800));

    let path = PathBuf::from(matches.get_one::<String>("FILE").unwrap());

    if watch {
        watch_and_render(path, polygonize, width, height, iterations);
    } else {
        let running = AtomicBool::new(true);
        let (_tx, dummy_rx) = channel(); // unused without --watch
        let _ = run_render(
            &path, polygonize, width, height, iterations, &running, &dummy_rx,
        );
    }
}
