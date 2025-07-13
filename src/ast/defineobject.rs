use crate::{expr_float, prelude::*};

#[derive(Clone, Debug)]
pub struct DefineObject {
    pub name: String,
    pub params: FxHashMap<String, Box<Expr>>,
    pub block: Option<Box<Stmt>>,
}

impl DefineObject {
    pub fn empty() -> Self {
        Self {
            name: String::new(),
            params: FxHashMap::default(),
            block: None,
        }
    }

    pub fn new(name: String, params: FxHashMap<String, Box<Expr>>, block: Box<Stmt>) -> Self {
        Self {
            name,
            params,
            block: Some(block),
        }
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

        for world in rect.iter_voxels(&grid) {
            let local = rect.world_to_local(world);

            visitor.environment.define(
                "local".into(),
                ASTValue::Float3(
                    expr_float!(local.x),
                    expr_float!(local.y),
                    expr_float!(local.z),
                ),
            );

            // visitor.local = Value::Float3(
            //     expr_float!(local.x),
            //     expr_float!(local.y),
            //     expr_float!(local.z),
            // );

            if let Some(block) = &self.block {
                let rc = block.accept(&mut visitor, ctx);
                // println!("Block executed with result: {:?}", rc);
                if let Ok(ASTValue::Float(v)) = rc {
                    if v <= 0.0 {
                        grid.set_create(world, 0);
                    }
                }
            }
            // if (local - Vec3::new(0.0, 0.0, 0.0)).magnitude() - 0.5 <= 0.0 {
            //     grid.set_create(world, 0);
            // }
        }

        //rect.fill(&mut grid, 1); // Fill with material ID 1

        grid
    }
}
