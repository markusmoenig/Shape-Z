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
                NodeOp::If(then_code, else_code) => {
                    let value = self.stack.pop().unwrap().truthy();
                    if value {
                        self.execute(then_code, program);
                    } else if let Some(else_code) = else_code {
                        self.execute(else_code, program);
                    }
                }
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
                NodeOp::U => {
                    self.stack.push(self.u);
                }
                NodeOp::V => {
                    self.stack.push(self.v);
                }
                NodeOp::D => {
                    self.stack.push(self.d);
                }
                // Math
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
                // Comparison
                NodeOp::Eq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::from_bool(a.x() == b.x()));
                }
                NodeOp::Ne => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::from_bool(a.x() != b.x()));
                }
                NodeOp::Lt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::from_bool(a.x() < b.x()));
                }
                NodeOp::Le => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::from_bool(a.x() <= b.x()));
                }
                NodeOp::Gt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::from_bool(a.x() > b.x()));
                }
                NodeOp::Ge => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::from_bool(a.x() >= b.x()));
                }
                // Logical
                NodeOp::And => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack
                        .push(Value::from_bool(a.x() != 0.0 && b.x() != 0.0));
                }
                NodeOp::Or => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack
                        .push(Value::from_bool(a.x() != 0.0 || b.x() != 0.0));
                }
                // Unary
                NodeOp::Not => {
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::from_bool(a.x() == 0.0));
                }
                NodeOp::Neg => {
                    let a = self.stack.pop().unwrap();
                    self.stack
                        .push(Value::from_components(-a.x(), -a.y(), -a.z()));
                }
                // Shapes
                NodeOp::ShapeRect(body) => {
                    let cl = self.bbox.clone();
                    self.execute(body, program);
                    self.bbox = cl;
                }
                NodeOp::ShapeDisc(body) => {
                    let local = self.local.as_vec3();

                    // Get center and radius from bbox in XZ plane
                    let bbox = &self.bbox;
                    let center_x = (bbox.min.x + bbox.max.x) * 0.5;
                    let center_z = (bbox.min.z + bbox.max.z) * 0.5;
                    let radius_x = (bbox.max.x - bbox.min.x) * 0.5;
                    let radius_z = (bbox.max.z - bbox.min.z) * 0.5;
                    let radius = radius_x.min(radius_z); // use smallest to stay inside bounds

                    let dx = local.x - center_x;
                    let dz = local.z - center_z;
                    let r = (dx * dx + dz * dz).sqrt();

                    if r > radius {
                        return;
                    }

                    let theta = dz.atan2(dx); // angle around Y axis (radians)
                    let height = local.y;

                    self.u = Value::from_float(theta); // angle in radians
                    self.v = Value::from_float(height); // Y height
                    self.d = Value::from_float(r); // radial depth

                    let bbox_backup = self.bbox.clone();
                    self.execute(body, program);
                    self.bbox = bbox_backup;
                }
                // Segments
                NodeOp::SegmentLeft(body) => {
                    let old_max_x = self.bbox.max.x;

                    let local = self.local.as_vec3();

                    let thickness = 0.1;
                    self.bbox.max.x = self.bbox.min.x + thickness;

                    if self.bbox.contains_point(local) {
                        self.u = Value::from_float(local.z - self.bbox.min.z); // u = Z
                        self.v = Value::from_float(local.y - self.bbox.min.y); // v = Y
                        self.d = Value::from_float(local.x - self.bbox.min.x); // d = X (depth)

                        self.execute(body, program);
                    }

                    self.bbox.max.x = old_max_x;
                }
                NodeOp::SegmentBack(body) => {
                    let old_max_z = self.bbox.max.z;

                    let local = self.local.as_vec3();

                    let thickness = 0.1;
                    self.bbox.max.z = self.bbox.min.z + thickness;

                    if self.bbox.contains_point(local) {
                        self.u = Value::from_float(local.x - self.bbox.min.x); // u = X
                        self.v = Value::from_float(local.y - self.bbox.min.y); // v = Y
                        self.d = Value::from_float(local.z - self.bbox.min.z); // d = Z (depth)

                        self.execute(body, program);
                    }

                    self.bbox.max.z = old_max_z;
                }
                // Pattern
                NodeOp::PatternModulo(even, odd) => {
                    let u = self.u.as_float();
                    let v = self.v.as_float();

                    let size = 0.1;

                    let cell_u = (u / size).floor() as i32;
                    let cell_v = (v / size).floor() as i32;

                    let is_even = (cell_u ^ cell_v) % 2 == 0;
                    if is_even {
                        if let Some(even) = even {
                            self.execute(even, program);
                        }
                    } else {
                        if let Some(odd) = odd {
                            self.execute(odd, program);
                        }
                    }
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
            // for shape in voxel.shapes.iter() {
            //     shape.execute(&mut execution, program);
            // }

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
