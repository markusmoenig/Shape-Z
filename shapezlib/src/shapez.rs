use crate::prelude::*;
use std::path::PathBuf;

// Default density
const DENSITY: usize = 40;

pub struct ShapeZ {
    path: PathBuf,
    context: Context,

    renderer: Arc<RwLock<Box<dyn Renderer>>>,
    buffer: Arc<Mutex<RenderBuffer>>,

    materials: Arc<RwLock<Vec<Vec<NodeOp>>>>,
    tracer: Tracer,
}

impl Default for ShapeZ {
    fn default() -> Self {
        Self::new()
    }
}

impl ShapeZ {
    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
            context: Context::new(Vec3::zero(), DENSITY, FxHashMap::default()),

            renderer: Arc::new(RwLock::new(Box::new(BSDF::new()))),
            buffer: Arc::new(Mutex::new(RenderBuffer::new(800, 800))),

            materials: Arc::new(RwLock::new(vec![])),
            tracer: Tracer::new(),
        }
    }

    pub fn set_resolution(&mut self, width: usize, height: usize) {
        self.buffer = Arc::new(Mutex::new(RenderBuffer::new(width, height)));
    }

    // Parse the source code into a module.
    pub fn parse(&mut self, path: PathBuf) -> Result<Module, ParseError> {
        self.path = path.clone();

        let mut parser = Parser::new();
        let module = parser.compile(path.clone())?;

        Ok(module)
    }

    // Parse the source code into a module.
    pub fn parse_str(&mut self, str: String) -> Result<Module, ParseError> {
        self.path = PathBuf::from("string_based.shpz");

        let mut parser = Parser::new();
        let module = parser.compile_module("main".into(), str, self.path.clone())?;

        Ok(module)
    }

    // Compile the source code
    pub fn compile(&mut self, module: &Module) -> Result<(), RuntimeError> {
        let mut visitor = CompileVisitor::new();
        self.context = Context::new(Vec3::zero(), DENSITY, module.variables.clone());

        for statement in module.stmts.clone() {
            _ = statement.accept(&mut visitor, &mut self.context)?;
        }

        Ok(())
    }

    /// Compile the voxels into the VoxelGrid.
    pub fn execute(&mut self) {
        let mut execution = Execution::new(self.context.variables.len());

        // Execute relevant global configs before voxel compilation

        // If user specified a density create a new grid
        if let Some(code) = self.context.global_config.get("density").cloned() {
            execution.execute(&code, &mut self.context.program);
            if let Some(density) = execution.stack.pop() {
                let density = (density.x() as usize).clamp(0, 200);
                let volumetric = self.context.program.grid.read().unwrap().volumetric.clone();
                self.context.program.grid = Arc::new(RwLock::new(VoxelGrid::empty(density)));
                self.context.program.grid.write().unwrap().volumetric = volumetric;
            }
        }

        // If user specified a background set it to the renderer.
        if let Some(code) = self.context.global_config.get("background").cloned() {
            execution.execute(&code, &mut self.context.program);
            if let Some(back) = execution.stack.pop() {
                self.renderer
                    .write()
                    .unwrap()
                    .set_background_color(back.as_vec3());
            }
        }

        // If user specified a sun_dir set it to the renderer.
        if let Some(code) = self.context.global_config.get("sun_dir").cloned() {
            execution.execute(&code, &mut self.context.program);
            if let Some(back) = execution.stack.pop() {
                self.renderer
                    .write()
                    .unwrap()
                    .set_sun_dir(back.as_vec3().normalized());
            }
        }

        // If user specified a sun_emission set it to the renderer.
        if let Some(code) = self.context.global_config.get("sun_emission").cloned() {
            execution.execute(&code, &mut self.context.program);
            if let Some(back) = execution.stack.pop() {
                self.renderer
                    .write()
                    .unwrap()
                    .set_sun_emission(back.as_vec3());
            }
        }

        self.context
            .program
            .camera
            .write()
            .unwrap()
            .apply_config(&mut execution, &self.context);

        // Execute the main program to compile all voxels.
        execution.execute(
            &self.context.program.globals.clone(),
            &mut self.context.program,
        );

        // Extract the materials from the context.
        self.materials = Arc::new(RwLock::new(
            self.context.materials.values().cloned().collect(),
        ));

        self.context
            .program
            .grid
            .write()
            .unwrap()
            .update_bboxes(true);

        self.renderer.write().unwrap().set_execution(execution);
    }

    /// Compute a sample of the image.
    pub fn sample(&mut self) {
        self.tracer.render(
            &mut self.buffer,
            &self.context.program.grid,
            &self.materials,
            &self.renderer,
            &self.context.program.camera,
        );
    }

    /// Write the grid into an obj file.
    pub fn write_obj(&self) {
        let mut path = self.path.clone();
        path.set_extension("obj");

        let (verts, indices, mats) =
            // crate::mesh::mesh_voxel_grid(&self.context.program.grid.read().unwrap());
            crate::mesh::mesh_voxel_grid_with_materials(&self.context.program.grid.read().unwrap());

        // _ = crate::mesh::write_obj(path, &verts, &indices, None);
        _ = crate::mesh::write_obj_with_mtl(path.clone(), &verts, &indices, Some(mats));

        println!("OBJ written to: {:?}.", path);
    }

    /// Write the image to disc.
    pub fn write_image(&self) {
        let mut path = self.path.clone();
        path.set_extension("png");

        let b = self.buffer.lock().unwrap();
        b.save_srgb(path.clone());
    }

    /// Write the image to an u array.
    pub fn write_image_to_array(&self) -> Vec<u8> {
        let b = self.buffer.lock().unwrap();
        b.as_rgb_bytes()
    }

    /// Get the current time in ms.
    pub fn get_time(&self) -> u128 {
        self.tracer.get_time()
    }

    /// Get a summary of the voxel grid.
    pub fn stats(&self) -> (String, String) {
        self.context.program.grid.read().unwrap().stats()
    }

    /// Imported paths
    pub fn imported_paths(&self) -> Vec<PathBuf> {
        self.context.imported_paths.clone()
    }
}
