use crate::prelude::*;
use std::path::PathBuf;

pub struct ShapeZ {
    path: PathBuf,
    context: Context,

    camera: Arc<RwLock<Box<dyn Camera>>>,
    renderer: Arc<RwLock<Box<dyn Renderer>>>,
    buffer: Arc<Mutex<RenderBuffer>>,

    materials: Arc<RwLock<Vec<Vec<NodeOp>>>>,
    // execution: Execution,
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
            context: Context::new(Vec3::zero(), 96, FxHashMap::default()),

            camera: Arc::new(RwLock::new(Box::new(Iso::new()))),
            renderer: Arc::new(RwLock::new(Box::new(BSDF::new()))),
            buffer: Arc::new(Mutex::new(RenderBuffer::new(800, 800))),

            materials: Arc::new(RwLock::new(vec![])),
            // execution: Execution::new(0),
            tracer: Tracer::new(),
        }
    }

    // Parse and compile the source code into a module.
    pub fn compile(&mut self, path: PathBuf) -> Result<Module, ParseError> {
        self.path = path.clone();

        let mut parser = Parser::new();
        let module = parser.compile(path.clone())?;

        // Compile the AST

        let mut visitor = CompileVisitor::new();
        self.context = Context::new(Vec3::zero(), 96, module.variables.clone());

        for statement in module.stmts.clone() {
            _ = statement.accept(&mut visitor, &mut self.context);
        }

        Ok(module)
    }

    pub fn execute(&mut self) {
        let mut execution = Execution::new(self.context.variables.len());
        execution.execute(
            &self.context.program.globals.clone(),
            &mut self.context.program,
        );

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

    pub fn sample(&mut self) {
        self.tracer.render(
            &mut self.buffer,
            &self.context.program.grid,
            &self.materials,
            &self.renderer,
            &self.camera,
        );
    }

    pub fn write_image(&self) {
        let mut path = self.path.clone();
        path.set_extension("png");

        let b = self.buffer.lock().unwrap();
        b.save_srgb(path.clone());
    }

    pub fn get_time(&self) -> u128 {
        self.tracer.get_time()
    }
}
