use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct DefineObject {
    pub name: String,
    pub params: FxHashMap<String, Box<Expr>>,
}

impl DefineObject {
    pub fn empty() -> Self {
        Self {
            name: String::new(),
            params: FxHashMap::default(),
        }
    }

    pub fn new(name: String, params: FxHashMap<String, Box<Expr>>) -> Self {
        Self { name, params }
    }

    pub fn place(&mut self, at: Vec3<f32>, ctx: &mut Context) -> VoxelGrid {
        let mut grid = VoxelGrid::empty(ctx.density);

        let mut visitor = ExecuteVisitor::new();
        let mut size = Vec3::new(1.0, 1.0, 1.0);

        if let Some(size_expr) = self.params.get("size") {
            if let Some(size_value) = size_expr.to_vec3(&mut visitor, ctx) {
                size = size_value;
            } else {
                eprintln!("Error: 'size' parameter must be a Vec3.");
            }
        }

        println!("size = {:?}", size);

        let rect = VoxelRect {
            origin: at,
            size: Vec3::new(1.0, 1.0, 1.0),
        };

        rect.fill(&mut grid, 1); // Fill with material ID 1

        grid
    }
}
