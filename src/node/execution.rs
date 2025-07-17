use crate::prelude::*;

pub struct Execution {
    /// The world coordinate of the current voxel
    pub world: Value,
    /// The segment transformed local coordinate
    pub local: Value,
    /// The segment computed u value
    pub u: Value,
    /// The segment computed v value
    pub v: Value,
    /// The segment computed d (depth) value
    pub d: Value,

    /// The current bbox of the model space. Gets subdivided by segments during
    /// execution.
    pub bbox: Aabb<f32>,

    /// The execution stack.
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
            world: Value::zero(),
            local: Value::zero(),
            u: Value::zero(),
            v: Value::zero(),
            d: Value::zero(),
            bbox: Aabb {
                min: Vec3::zero(),
                max: Vec3::one(),
            },
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
                NodeOp::World => {
                    self.stack.push(self.world);
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
                }
            }
        }
    }

    /// Place a voxel box into the world. We first model into a separate grid
    /// and than merge back into the program grid.
    pub fn place(&mut self, id: &String, program: &mut Program) {
        let voxel = match program.voxels.get(id) {
            Some(defined) => defined.clone(),
            None => return,
        };

        let mut grid = VoxelGrid::empty(program.grid.read().unwrap().density);

        let mut execution = Execution::default();

        let mut size = Vec3::new(1.0, 1.0, 1.0);
        execution.execute(&voxel.size, program);
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

            // Set up the voxel coordinates
            execution.world = Value::from_vec3(world);
            execution.local = Value::from_vec3(local);

            // Inital bbox for the grid which the shapes adhere to and segments subdivide.
            execution.bbox = Aabb {
                min: -size / 2.0,
                max: size / 2.0,
            };

            // Execute the voxel body
            execution.execute(&voxel.body, program);

            // Recursively execute voxels shapes
            for shape in voxel.shapes.iter() {
                shape.execute(&mut execution, program);
            }

            // Set the result.
            if let Some(value) = execution.stack.last() {
                let result = value.x();
                if result >= 0.0 {
                    grid.set_create(world, result as u8);
                }
            }
        }

        program.grid.write().unwrap().merge(&grid);
    }
}
