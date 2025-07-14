use crate::prelude::*;

pub struct Execution {
    pub local: Value,
    pub stack: Vec<Value>,
}

impl Default for Execution {
    fn default() -> Self {
        Self::new()
    }
}

impl Execution {
    pub fn new() -> Self {
        Self {
            local: Value::zero(),
            stack: Vec::with_capacity(32),
        }
    }

    pub fn execute(&mut self, code: &[NodeOp], program: &mut Program) {
        for op in code {
            match op {
                NodeOp::Place(id) => self.place(id, program),
                NodeOp::Push(v) => self.stack.push(*v),
                NodeOp::Pack3 => {
                    let z = self.stack.pop().unwrap();
                    let y = self.stack.pop().unwrap();
                    let x = self.stack.pop().unwrap();
                    self.stack.push(Value::from_components(x.x(), y.x(), z.x()));
                }
                NodeOp::Local => {
                    self.stack.push(self.local);
                }
                NodeOp::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.add(b));
                }
                NodeOp::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.sub(b));
                }
                NodeOp::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.mul(b));
                }
                NodeOp::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.div(b));
                }
                NodeOp::Length => {
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.length());
                }
                NodeOp::Abs => {
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.abs());
                } // NodeOp::CallShape(name) => {
                  //     if let Some(shape_code) = self.program.definitons.get(name) {
                  //         self.execute(shape_code);
                  //     } else {
                  //         panic!("Undefined shape: {}", name);
                  //     }
                  // }
            }
        }
    }

    pub fn place(&mut self, id: &String, program: &mut Program) {
        let defined = match program.definitons.get(id) {
            Some(defined) => defined.clone(),
            None => return,
        };

        let mut grid = VoxelGrid::empty(program.grid.read().unwrap().density);

        let mut execution = Execution::default();

        let mut size = Vec3::new(1.0, 1.0, 1.0);
        execution.execute(&defined.size, program);
        if let Some(value) = execution.stack.last() {
            size = value.as_vec3();
        }

        let rect = VoxelRect {
            origin: Vec3::zero(),
            size,
        };

        for world in rect.iter_voxels(&grid) {
            let local = rect.world_to_local(world);

            execution.stack.clear();
            execution.local = Value::from_vec3(local);
            execution.execute(&defined.body, program);

            if let Some(value) = execution.stack.last() {
                if value.x() < 0.0 {
                    grid.set_create(world, 1);
                }
            }

            // visitor.environment.define(
            //     "local".into(),
            //     ASTValue::Float3(
            //         expr_float!(local.x),
            //         expr_float!(local.y),
            //         expr_float!(local.z),
            //     ),
            // );

            // visitor.local = Value::Float3(
            //     expr_float!(local.x),
            //     expr_float!(local.y),
            //     expr_float!(local.z),
            // );

            // if let Some(block) = &self.block {
            //     let rc = block.accept(&mut visitor, ctx);
            //     // println!("Block executed with result: {:?}", rc);
            //     if let Ok(ASTValue::Float(v)) = rc {
            //         if v <= 0.0 {
            //             grid.set_create(world, 0);
            //         }
            //     }
            // }
            // if (local - Vec3::new(0.0, 0.0, 0.0)).magnitude() - 0.5 <= 0.0 {
            //     grid.set_create(world, 0);
            // }
        }

        program.grid.write().unwrap().merge(&grid);
    }
}
