use crate::prelude::*;

#[derive(Clone)]
pub struct ASTFunction {
    pub name: String,
    pub arguments: i32,
    pub op: NodeOp,
}

/// ExecuteVisitor
pub struct CompileVisitor {
    pub environment: Environment,
    functions: FxHashMap<String, ASTFunction>,

    user_functions: IndexMap<String, (usize, IndexMap<String, Option<Vec<NodeOp>>>, usize)>,

    /// List of local variables which are in scope (inside functions)
    locals: IndexSet<String>,
}

impl Visitor for CompileVisitor {
    fn new() -> Self
    where
        Self: Sized,
    {
        let mut functions: FxHashMap<String, ASTFunction> = FxHashMap::default();
        functions.insert(
            "length".to_string(),
            ASTFunction {
                name: "length".to_string(),
                arguments: 1,
                op: NodeOp::Length,
            },
        );
        functions.insert(
            "abs".to_string(),
            ASTFunction {
                name: "abs".to_string(),
                arguments: 1,
                op: NodeOp::Abs,
            },
        );
        functions.insert(
            "sin".to_string(),
            ASTFunction {
                name: "sin".to_string(),
                arguments: 1,
                op: NodeOp::Sin,
            },
        );
        functions.insert(
            "cos".to_string(),
            ASTFunction {
                name: "cos".to_string(),
                arguments: 1,
                op: NodeOp::Cos,
            },
        );
        functions.insert(
            "normalize".to_string(),
            ASTFunction {
                name: "normalize".to_string(),
                arguments: 1,
                op: NodeOp::Normalize,
            },
        );
        functions.insert(
            "tan".to_string(),
            ASTFunction {
                name: "tan".to_string(),
                arguments: 1,
                op: NodeOp::Tan,
            },
        );
        functions.insert(
            "atan".to_string(),
            ASTFunction {
                name: "atan".to_string(),
                arguments: 1,
                op: NodeOp::Atan,
            },
        );
        functions.insert(
            "atan2".to_string(),
            ASTFunction {
                name: "atan2".to_string(),
                arguments: 2,
                op: NodeOp::Atan2,
            },
        );
        functions.insert(
            "dot".to_string(),
            ASTFunction {
                name: "dot".to_string(),
                arguments: 2,
                op: NodeOp::Dot,
            },
        );
        functions.insert(
            "cross".to_string(),
            ASTFunction {
                name: "cross".to_string(),
                arguments: 2,
                op: NodeOp::Cross,
            },
        );
        functions.insert(
            "floor".to_string(),
            ASTFunction {
                name: "floor".to_string(),
                arguments: 1,
                op: NodeOp::Floor,
            },
        );
        functions.insert(
            "ceil".to_string(),
            ASTFunction {
                name: "ceil".to_string(),
                arguments: 1,
                op: NodeOp::Ceil,
            },
        );
        functions.insert(
            "fract".to_string(),
            ASTFunction {
                name: "fract".to_string(),
                arguments: 1,
                op: NodeOp::Fract,
            },
        );
        functions.insert(
            "radians".to_string(),
            ASTFunction {
                name: "radians".to_string(),
                arguments: 1,
                op: NodeOp::Radians,
            },
        );
        functions.insert(
            "degrees".to_string(),
            ASTFunction {
                name: "degrees".to_string(),
                arguments: 1,
                op: NodeOp::Degrees,
            },
        );
        functions.insert(
            "white_noise".to_string(),
            ASTFunction {
                name: "white_noise".to_string(),
                arguments: 0,
                op: NodeOp::WhiteNoise,
            },
        );
        functions.insert(
            "value_noise".to_string(),
            ASTFunction {
                name: "value_noise".to_string(),
                arguments: 2,
                op: NodeOp::ValueNoise,
            },
        );
        functions.insert(
            "min".to_string(),
            ASTFunction {
                name: "min".to_string(),
                arguments: 2,
                op: NodeOp::Min,
            },
        );
        functions.insert(
            "max".to_string(),
            ASTFunction {
                name: "max".to_string(),
                arguments: 2,
                op: NodeOp::Max,
            },
        );
        functions.insert(
            "mix".to_string(),
            ASTFunction {
                name: "mix".to_string(),
                arguments: 3,
                op: NodeOp::Mix,
            },
        );
        functions.insert(
            "smoothstep".to_string(),
            ASTFunction {
                name: "smoothstep".to_string(),
                arguments: 3,
                op: NodeOp::Smoothstep,
            },
        );
        functions.insert(
            "step".to_string(),
            ASTFunction {
                name: "step".to_string(),
                arguments: 2,
                op: NodeOp::Smoothstep,
            },
        );
        functions.insert(
            "mod".to_string(),
            ASTFunction {
                name: "mod".to_string(),
                arguments: 2,
                op: NodeOp::Mod,
            },
        );
        functions.insert(
            "clamp".to_string(),
            ASTFunction {
                name: "clamp".to_string(),
                arguments: 3,
                op: NodeOp::Clamp,
            },
        );
        functions.insert(
            "sqrt".to_string(),
            ASTFunction {
                name: "sqrt".to_string(),
                arguments: 1,
                op: NodeOp::Sqrt,
            },
        );
        functions.insert(
            "log".to_string(),
            ASTFunction {
                name: "log".to_string(),
                arguments: 1,
                op: NodeOp::Log,
            },
        );
        functions.insert(
            "pow".to_string(),
            ASTFunction {
                name: "pow".to_string(),
                arguments: 2,
                op: NodeOp::Pow,
            },
        );
        functions.insert(
            "smooth_union".to_string(),
            ASTFunction {
                name: "smooth_union".to_string(),
                arguments: 3,
                op: NodeOp::SmoothUnion,
            },
        );
        functions.insert(
            "point_at".to_string(),
            ASTFunction {
                name: "point_at".to_string(),
                arguments: 1,
                op: NodeOp::PointAt,
            },
        );
        functions.insert(
            "print".to_string(),
            ASTFunction {
                name: "print".to_string(),
                arguments: 1,
                op: NodeOp::Print,
            },
        );
        functions.insert(
            "slope".to_string(),
            ASTFunction {
                name: "slope".to_string(),
                arguments: 1,
                op: NodeOp::Slope,
            },
        );

        Self {
            environment: Environment::default(),
            functions,
            user_functions: IndexMap::default(),
            locals: IndexSet::default(),
        }
    }

    fn print(
        &mut self,
        expression: &Expr,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        print!("-- Print ");
        expression.accept(self, ctx)?;
        println!(" --");

        Ok(ASTValue::None)
    }

    fn block(
        &mut self,
        list: &[Box<Stmt>],
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let mut value = ASTValue::None;

        self.environment.begin_scope(ASTValue::None, false);
        for stmt in list {
            value = stmt.accept(self, ctx)?;
        }
        self.environment.end_scope();

        Ok(value)
    }

    fn expression(
        &mut self,
        expression: &Expr,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        expression.accept(self, ctx)
    }

    /// Create a voxel box
    fn voxel(
        &mut self,
        objectd: &VoxelD,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let mut cpy = objectd.clone();

        if let Some(size) = objectd.params.get("size") {
            ctx.set_target(OutputTarget::Custom);
            _ = size.accept(self, ctx)?;

            cpy.size = ctx.program.custom.clone();
        }

        ctx.program.voxels.insert(cpy.name.clone(), cpy);
        ctx.set_target(OutputTarget::Voxels(objectd.name.clone()));

        if let Some(block) = &objectd.block {
            block.accept(self, ctx)?;
        }

        ctx.set_target(OutputTarget::Globals);

        Ok(ASTValue::None)
    }

    /// Create a shape
    fn shape(
        &mut self,
        objectd: &ShapeD,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        // Compile params
        let mut cp = FxHashMap::default();
        for (name, ast) in &objectd.params {
            ctx.add_custom_target();
            _ = ast.accept(self, ctx)?;
            if let Some(code) = ctx.take_last_custom_target() {
                cp.insert(name.clone(), code);
            }
        }

        // Body
        ctx.add_custom_target();
        if let Some(block) = &objectd.block {
            block.accept(self, ctx)?;
        }
        if let Some(code) = ctx.take_last_custom_target() {
            match objectd.name.as_str() {
                "Rect" => {
                    ctx.emit(NodeOp::ShapeRect(
                        cp.get("at").cloned().unwrap_or(vec![]),
                        cp.get("size").cloned().unwrap_or(vec![]),
                        code,
                    ));
                }
                "Disc" => {
                    ctx.emit(NodeOp::ShapeDisc(
                        cp.get("at").cloned().unwrap_or(vec![]),
                        cp.get("radius").cloned().unwrap_or(vec![]),
                        code,
                    ));
                }
                "Terrain" => {
                    ctx.add_custom_target();
                    if let Some(ast) = objectd.mods.get("heightmap") {
                        _ = ast.accept(self, ctx)?;
                        if let Some(heightmap) = ctx.take_last_custom_target() {
                            ctx.emit(NodeOp::ShapeTerrain(
                                cp.get("at").cloned().unwrap_or(vec![]),
                                Arc::new(heightmap),
                                code,
                            ));
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(ASTValue::None)
    }

    // Create a Distance
    fn distance(
        &mut self,
        objectd: &DistanceD,
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        // Compile params
        let mut cp = FxHashMap::default();
        for (name, ast) in &objectd.params {
            ctx.add_custom_target();
            _ = ast.accept(self, ctx)?;
            if let Some(code) = ctx.take_last_custom_target() {
                cp.insert(name.clone(), code);
            }
        }

        let mut offset = vec![];
        if let Some(off) = &objectd.offset {
            ctx.add_custom_target();
            _ = off.accept(self, ctx)?;
            if let Some(code) = ctx.take_last_custom_target() {
                offset = code;
            }
        }

        let mut scale = vec![];
        if let Some(scal) = &objectd.scale {
            ctx.add_custom_target();
            _ = scal.accept(self, ctx)?;
            if let Some(code) = ctx.take_last_custom_target() {
                scale = code;
            }
        }

        // Body
        ctx.add_custom_target();
        if let Some(block) = &objectd.block {
            _ = block.accept(self, ctx)?;
        }
        if let Some(code) = ctx.take_last_custom_target() {
            let at = cp.get("at").cloned().unwrap_or(vec![]);
            let size = cp.get("size").cloned().unwrap_or(vec![]);
            let radius = cp.get("radius").cloned().unwrap_or(vec![]);
            let from = cp.get("from").cloned().unwrap_or(vec![]);
            let to = cp.get("to").cloned().unwrap_or(vec![]);
            match objectd.name.as_str() {
                "Sphere" => {
                    ctx.emit(NodeOp::DistanceSphere(at, radius, offset, scale, code));
                }
                "Box" => {
                    ctx.emit(NodeOp::DistanceBox(
                        at,
                        size,
                        cp.get("rounding").cloned().unwrap_or(vec![]),
                        offset,
                        scale,
                        code,
                    ));
                }
                "Capsule" => {
                    ctx.emit(NodeOp::DistanceCapsule(
                        from, to, radius, offset, scale, code,
                    ));
                }
                "Cylinder" => {
                    ctx.emit(NodeOp::DistanceCylinder(
                        from, to, radius, offset, scale, code,
                    ));
                }
                "Cone" => {
                    ctx.emit(NodeOp::DistanceCone(
                        from,
                        to,
                        cp.get("base").cloned().unwrap_or(vec![]),
                        cp.get("top").cloned().unwrap_or(vec![]),
                        offset,
                        scale,
                        code,
                    ));
                }
                "Vesica" => {
                    ctx.emit(NodeOp::DistanceVesica(
                        from,
                        to,
                        cp.get("width").cloned().unwrap_or(vec![]),
                        offset,
                        scale,
                        code,
                    ));
                }
                "NgonFrustum" => {
                    ctx.emit(NodeOp::DistanceNgonFrustum(
                        from,
                        to,
                        cp.get("sides").cloned().unwrap_or(vec![]),
                        cp.get("base").cloned().unwrap_or(vec![]),
                        cp.get("top").cloned().unwrap_or(vec![]),
                        cp.get("phase").cloned().unwrap_or(vec![]),
                        offset,
                        scale,
                        code,
                    ));
                }
                "RectFrustum" => {
                    ctx.emit(NodeOp::DistanceRectFrustum(
                        from,
                        to,
                        cp.get("base").cloned().unwrap_or(vec![]),
                        cp.get("top").cloned().unwrap_or(vec![]),
                        cp.get("angle").cloned().unwrap_or(vec![]),
                        offset,
                        scale,
                        code,
                    ));
                }
                other => {
                    return Err(RuntimeError::new(
                        format!("Unknown distance: {}", other),
                        loc,
                    ));
                }
            }
        }

        Ok(ASTValue::None)
    }

    // Create a Volume
    fn volume(
        &mut self,
        objectd: &VolumeD,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        // Compile params
        // let mut cp = FxHashMap::default();
        // for (name, ast) in &objectd.params {
        //     ctx.add_custom_target();
        //     _ = ast.accept(self, ctx)?;
        //     if let Some(code) = ctx.take_last_custom_target() {
        //         cp.insert(name.clone(), code);
        //     }
        // }

        // Body
        ctx.add_custom_target();
        if let Some(block) = &objectd.block {
            _ = block.accept(self, ctx)?;
        }
        if let Some(code) = ctx.take_last_custom_target() {
            ctx.emit(NodeOp::Volume(code));
        }

        Ok(ASTValue::None)
    }

    // Create a segment
    fn segment(
        &mut self,
        objectd: &SegmentD,
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        // Compile params
        let mut cp = FxHashMap::default();
        for (name, ast) in &objectd.params {
            ctx.add_custom_target();
            _ = ast.accept(self, ctx)?;
            if let Some(code) = ctx.take_last_custom_target() {
                cp.insert(name.clone(), code);
            }
        }

        // Body
        ctx.add_custom_target();
        if let Some(block) = &objectd.block {
            _ = block.accept(self, ctx)?;
        }
        if let Some(code) = ctx.take_last_custom_target() {
            let depth = cp.get("depth").cloned().unwrap_or(vec![]);
            match objectd.name.as_str() {
                "Left" => {
                    ctx.emit(NodeOp::SegmentLeft(depth, code));
                }
                "Back" => {
                    ctx.emit(NodeOp::SegmentBack(depth, code));
                }
                "Floor" => {
                    ctx.emit(NodeOp::SegmentFloor(depth, code));
                }
                "Right" => {
                    ctx.emit(NodeOp::SegmentRight(depth, code));
                }
                "Front" => {
                    ctx.emit(NodeOp::SegmentFront(depth, code));
                }
                "Ceiling" => {
                    ctx.emit(NodeOp::SegmentCeiling(depth, code));
                }
                other => {
                    return Err(RuntimeError::new(
                        format!("Unknown segment: {}", other),
                        loc,
                    ));
                }
            }
        }

        Ok(ASTValue::None)
    }

    // Create a pattern
    fn pattern(
        &mut self,
        objectd: &PatternD,
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        // Compile params
        let mut cp = FxHashMap::default();
        for (name, ast) in &objectd.params {
            ctx.add_custom_target();
            _ = ast.accept(self, ctx)?;
            if let Some(code) = ctx.take_last_custom_target() {
                cp.insert(name.clone(), code);
            }
        }

        // Compile all blocks
        let mut codes = FxHashMap::default();
        for (name, stmts) in &objectd.blocks {
            ctx.add_custom_target();
            _ = stmts.accept(self, ctx)?;
            if let Some(code) = ctx.take_last_custom_target() {
                codes.insert(name.clone(), code);
            }
        }

        match objectd.name.as_str() {
            "Modulo" => {
                let size = cp.get("size").cloned().unwrap_or(vec![]);
                let even = codes.get("even").cloned();
                let odd = codes.get("odd").cloned();
                ctx.emit(NodeOp::PatternModulo(size, even, odd));
            }
            "Bricks" => {
                let size = cp.get("size").cloned().unwrap_or(vec![]);
                let gap = cp.get("gap").cloned().unwrap_or(vec![]);
                let rounding = cp.get("rounding").cloned().unwrap_or(vec![]);
                let brick = codes.get("brick").cloned();
                let cement = codes.get("cement").cloned();
                ctx.emit(NodeOp::PatternBricks(size, gap, rounding, brick, cement));
            }
            other => {
                return Err(RuntimeError::new(
                    format!("Unknown pattern: {}", other),
                    loc,
                ));
            }
        }

        Ok(ASTValue::None)
    }

    // Create a camera
    fn camera(
        &mut self,
        objectd: &CameraD,
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        // Compile all blocks
        let mut codes = FxHashMap::default();
        for (name, stmts) in &objectd.blocks {
            ctx.add_custom_target();
            _ = stmts.accept(self, ctx)?;
            if let Some(code) = ctx.take_last_custom_target() {
                codes.insert(name.clone(), code);
            }
        }

        let mut config = objectd.clone();
        config.codes = codes;
        ctx.camera_config = Some(config);

        match objectd.name.as_str() {
            "Isometric" => {}
            "Pinhole" => ctx.program.camera = Arc::new(RwLock::new(Box::new(Pinhole::new()))),
            other => {
                return Err(RuntimeError::new(format!("Unknown camera: {}", other), loc));
            }
        }

        Ok(ASTValue::None)
    }

    // Create a material
    fn material(
        &mut self,
        objectd: &MaterialD,
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let mut material_ops: Vec<NodeOp> = vec![];

        // Compile all blocks
        for (name, stmts) in &objectd.blocks {
            ctx.add_custom_target();
            _ = stmts.accept(self, ctx)?;
            if let Some(codes) = ctx.take_last_custom_target() {
                material_ops.extend(codes);

                match name.as_str() {
                    "albedo" => material_ops.push(NodeOp::MaterialAlbedo),
                    "subsurface" => material_ops.push(NodeOp::MaterialSubsurface),
                    "metallic" => material_ops.push(NodeOp::MaterialMetallic),
                    "specular_tint" => material_ops.push(NodeOp::MaterialSpecularTint),
                    "roughness" => material_ops.push(NodeOp::MaterialRoughness),
                    "anisotropic" => material_ops.push(NodeOp::MaterialAnisotropic),
                    "sheen" => material_ops.push(NodeOp::MaterialSheen),
                    "sheen_tint" => material_ops.push(NodeOp::MaterialSheenTint),
                    "clearcoat" => material_ops.push(NodeOp::MaterialClearcoat),
                    "clearcoat_gloss" => material_ops.push(NodeOp::MaterialClearcoatGloss),
                    "ior" => material_ops.push(NodeOp::MaterialIOR),
                    "transmission" => {
                        material_ops.push(NodeOp::MaterialTransmission);
                        let idx = ctx.materials.len();
                        ctx.program.grid.write().unwrap().volumetric.push(idx as u8);
                    }
                    "emission" => material_ops.push(NodeOp::MaterialEmission),
                    other => {
                        return Err(RuntimeError::new(
                            format!("Unknown material property: {}", other),
                            loc,
                        ));
                    }
                }
            }
        }

        if let Some(mediumd) = &objectd.mediumd {
            let idx = ctx.materials.len();
            ctx.program.grid.write().unwrap().volumetric.push(idx as u8);

            match mediumd.name.as_str() {
                "Absorb" => material_ops.push(NodeOp::MediumAbsorb),
                "Scatter" => material_ops.push(NodeOp::MediumScatter),
                "Emissive" => material_ops.push(NodeOp::MediumEmissive),
                other => {
                    return Err(RuntimeError::new(format!("Unknown medium: {}", other), loc));
                }
            }

            // Compile all blocks
            for (name, stmts) in &mediumd.blocks {
                ctx.add_custom_target();
                _ = stmts.accept(self, ctx)?;
                if let Some(codes) = ctx.take_last_custom_target() {
                    material_ops.extend(codes);

                    match name.as_str() {
                        "density" => material_ops.push(NodeOp::MediumDensity),
                        "color" => material_ops.push(NodeOp::MediumColor),
                        "anisotropy" => material_ops.push(NodeOp::MediumAnisotropy),
                        other => {
                            return Err(RuntimeError::new(
                                format!("Unknown medium property: {}", other),
                                loc,
                            ));
                        }
                    }
                }
            }
        }

        if !ctx.materials.contains_key(&objectd.name) {
            ctx.materials.insert(objectd.name.clone(), material_ops);
        }

        Ok(ASTValue::None)
    }

    // A material reference, we add the index of the material to the code.
    fn material_reference(
        &mut self,
        name: &String,
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        if let Some(index) = ctx.materials.get_index_of(name) {
            ctx.emit(NodeOp::Push(Value::from_float(index as f32)));
        } else {
            return Err(RuntimeError::new(
                format!("Unknown material: {}", name),
                loc,
            ));
        }

        Ok(ASTValue::None)
    }

    fn import(
        &mut self,
        module: &Option<Module>,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        // Execute the statements in the imported module
        if let Some(module) = module {
            ctx.imported_paths.push(module.path.clone());
            let mut visitor = CompileVisitor::new();
            for statement in module.stmts.clone() {
                _ = statement.accept(&mut visitor, ctx);
            }
        }

        Ok(ASTValue::None)
    }

    fn config(
        &mut self,
        id: &String,
        block: &Box<Stmt>,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        ctx.add_custom_target();
        _ = block.accept(self, ctx)?;
        if let Some(code) = ctx.take_last_custom_target() {
            ctx.global_config.insert(id.clone(), code);
        }

        Ok(ASTValue::None)
    }

    fn place(
        &mut self,
        id: &String,
        params: &FxHashMap<String, Box<Expr>>,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let mut at = vec![];
        if let Some(at_param) = params.get("at") {
            ctx.add_custom_target();
            _ = at_param.accept(self, ctx)?;
            if let Some(code) = ctx.take_last_custom_target() {
                at = code;
            }
        }

        if ctx.program.voxels.contains_key(id) {
            ctx.emit(NodeOp::Place(id.clone(), at));
        }

        Ok(ASTValue::None)
    }

    fn var_declaration(
        &mut self,
        name: &str,
        _static_type: &ASTValue,
        expression: &Expr,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        _ = expression.accept(self, ctx)?;

        if let Some(index) = self.locals.get_index_of(name) {
            ctx.emit(NodeOp::StoreLocal(index));
        } else if let Some(index) = ctx.globals.get(name) {
            ctx.emit(NodeOp::StoreGlobal(*index as usize));
        }

        // self.environment.define(name.to_string(), v);

        Ok(ASTValue::None)
    }

    fn variable_assignment(
        &mut self,
        name: String,
        op: &AssignmentOperator,
        swizzle: &[u8],
        _field_path: &[String],
        expression: &Expr,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        // NOTE: We intentionally DO NOT evaluate `expression` up-front for swizzled
        // compound assignments, because we need the target value on the stack
        // first to preserve order for SetComponents.

        if let Some(index) = self.locals.get_index_of(&name) {
            let index = index as usize;
            if swizzle.is_empty() {
                // Non-swizzled assignment to a local
                match op {
                    AssignmentOperator::Assign => {
                        _ = expression.accept(self, ctx)?; // RHS
                        ctx.emit(NodeOp::StoreLocal(index));
                    }
                    AssignmentOperator::AddAssign => {
                        _ = expression.accept(self, ctx)?; // RHS
                        ctx.emit(NodeOp::LoadLocal(index));
                        ctx.emit(NodeOp::Swap); // [lhs, rhs] → [rhs, lhs] → then Add uses (lhs + rhs)?
                        ctx.emit(NodeOp::Add);
                        ctx.emit(NodeOp::StoreLocal(index));
                    }
                    AssignmentOperator::SubtractAssign => {
                        _ = expression.accept(self, ctx)?; // RHS
                        ctx.emit(NodeOp::LoadLocal(index));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Sub);
                        ctx.emit(NodeOp::StoreLocal(index));
                    }
                    AssignmentOperator::MultiplyAssign => {
                        _ = expression.accept(self, ctx)?; // RHS
                        ctx.emit(NodeOp::LoadLocal(index));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Mul);
                        ctx.emit(NodeOp::StoreLocal(index));
                    }
                    AssignmentOperator::DivideAssign => {
                        _ = expression.accept(self, ctx)?; // RHS
                        ctx.emit(NodeOp::LoadLocal(index));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Div);
                        ctx.emit(NodeOp::StoreLocal(index));
                    }
                }
            } else {
                // Swizzled assignment to a local
                match op {
                    AssignmentOperator::Assign => {
                        _ = expression.accept(self, ctx)?; // RHS
                        ctx.emit(NodeOp::LoadLocal(index)); // target
                        ctx.emit(NodeOp::Swap); // [target, rhs]
                        ctx.emit(NodeOp::SetComponents(swizzle.to_vec()));
                        ctx.emit(NodeOp::StoreLocal(index));
                    }
                    AssignmentOperator::AddAssign => {
                        // Want: t.swizzle = t.swizzle + rhs
                        ctx.emit(NodeOp::LoadLocal(index)); // t
                        ctx.emit(NodeOp::Dup); // t, t
                        ctx.emit(NodeOp::GetComponents(swizzle.to_vec())); // t, a
                        _ = expression.accept(self, ctx)?; // t, a, rhs
                        ctx.emit(NodeOp::Add); // t, (a+rhs)
                        ctx.emit(NodeOp::SetComponents(swizzle.to_vec())); // t'
                        ctx.emit(NodeOp::StoreLocal(index));
                    }
                    AssignmentOperator::SubtractAssign => {
                        ctx.emit(NodeOp::LoadLocal(index));
                        ctx.emit(NodeOp::Dup);
                        ctx.emit(NodeOp::GetComponents(swizzle.to_vec())); // t, a
                        _ = expression.accept(self, ctx)?; // t, a, rhs
                        ctx.emit(NodeOp::Sub); // t, (a-rhs)
                        ctx.emit(NodeOp::SetComponents(swizzle.to_vec())); // t'
                        ctx.emit(NodeOp::StoreLocal(index));
                    }
                    AssignmentOperator::MultiplyAssign => {
                        ctx.emit(NodeOp::LoadLocal(index));
                        ctx.emit(NodeOp::Dup);
                        ctx.emit(NodeOp::GetComponents(swizzle.to_vec())); // t, a
                        _ = expression.accept(self, ctx)?; // t, a, rhs
                        ctx.emit(NodeOp::Mul); // t, (a*rhs)
                        ctx.emit(NodeOp::SetComponents(swizzle.to_vec())); // t'
                        ctx.emit(NodeOp::StoreLocal(index));
                    }
                    AssignmentOperator::DivideAssign => {
                        ctx.emit(NodeOp::LoadLocal(index));
                        ctx.emit(NodeOp::Dup);
                        ctx.emit(NodeOp::GetComponents(swizzle.to_vec())); // t, a
                        _ = expression.accept(self, ctx)?; // t, a, rhs
                        ctx.emit(NodeOp::Div); // t, (a/rhs)
                        ctx.emit(NodeOp::SetComponents(swizzle.to_vec())); // t'
                        ctx.emit(NodeOp::StoreLocal(index));
                    }
                }
            }
        } else if let Some(&index) = ctx.globals.get(&name) {
            let index = index as usize;
            if swizzle.is_empty() {
                // Non-swizzled assignment to a global
                match op {
                    AssignmentOperator::Assign => {
                        _ = expression.accept(self, ctx)?; // RHS
                        ctx.emit(NodeOp::StoreGlobal(index));
                    }
                    AssignmentOperator::AddAssign => {
                        _ = expression.accept(self, ctx)?; // RHS
                        ctx.emit(NodeOp::LoadGlobal(index));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Add);
                        ctx.emit(NodeOp::StoreGlobal(index));
                    }
                    AssignmentOperator::SubtractAssign => {
                        _ = expression.accept(self, ctx)?; // RHS
                        ctx.emit(NodeOp::LoadGlobal(index));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Sub);
                        ctx.emit(NodeOp::StoreGlobal(index));
                    }
                    AssignmentOperator::MultiplyAssign => {
                        _ = expression.accept(self, ctx)?; // RHS
                        ctx.emit(NodeOp::LoadGlobal(index));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Mul);
                        ctx.emit(NodeOp::StoreGlobal(index));
                    }
                    AssignmentOperator::DivideAssign => {
                        _ = expression.accept(self, ctx)?; // RHS
                        ctx.emit(NodeOp::LoadGlobal(index));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Div);
                        ctx.emit(NodeOp::StoreGlobal(index));
                    }
                }
            } else {
                // Swizzled assignment to a global
                match op {
                    AssignmentOperator::Assign => {
                        _ = expression.accept(self, ctx)?; // RHS
                        ctx.emit(NodeOp::LoadGlobal(index));
                        ctx.emit(NodeOp::Swap); // [target, rhs]
                        ctx.emit(NodeOp::SetComponents(swizzle.to_vec()));
                        ctx.emit(NodeOp::StoreGlobal(index));
                    }
                    AssignmentOperator::AddAssign => {
                        ctx.emit(NodeOp::LoadGlobal(index)); // t
                        ctx.emit(NodeOp::Dup); // t, t
                        ctx.emit(NodeOp::GetComponents(swizzle.to_vec())); // t, a
                        _ = expression.accept(self, ctx)?; // t, a, rhs
                        ctx.emit(NodeOp::Add); // t, (a+rhs)
                        ctx.emit(NodeOp::SetComponents(swizzle.to_vec())); // t'
                        ctx.emit(NodeOp::StoreGlobal(index));
                    }
                    AssignmentOperator::SubtractAssign => {
                        ctx.emit(NodeOp::LoadGlobal(index));
                        ctx.emit(NodeOp::Dup);
                        ctx.emit(NodeOp::GetComponents(swizzle.to_vec())); // t, a
                        _ = expression.accept(self, ctx)?; // t, a, rhs
                        ctx.emit(NodeOp::Sub); // t, (a-rhs)
                        ctx.emit(NodeOp::SetComponents(swizzle.to_vec())); // t'
                        ctx.emit(NodeOp::StoreGlobal(index));
                    }
                    AssignmentOperator::MultiplyAssign => {
                        ctx.emit(NodeOp::LoadGlobal(index));
                        ctx.emit(NodeOp::Dup);
                        ctx.emit(NodeOp::GetComponents(swizzle.to_vec())); // t, a
                        _ = expression.accept(self, ctx)?; // t, a, rhs
                        ctx.emit(NodeOp::Mul); // t, (a*rhs)
                        ctx.emit(NodeOp::SetComponents(swizzle.to_vec())); // t'
                        ctx.emit(NodeOp::StoreGlobal(index));
                    }
                    AssignmentOperator::DivideAssign => {
                        ctx.emit(NodeOp::LoadGlobal(index));
                        ctx.emit(NodeOp::Dup);
                        ctx.emit(NodeOp::GetComponents(swizzle.to_vec())); // t, a
                        _ = expression.accept(self, ctx)?; // t, a, rhs
                        ctx.emit(NodeOp::Div); // t, (a/rhs)
                        ctx.emit(NodeOp::SetComponents(swizzle.to_vec())); // t'
                        ctx.emit(NodeOp::StoreGlobal(index));
                    }
                }
            }
        }

        Ok(ASTValue::None)
    }

    fn variable(
        &mut self,
        name: String,
        swizzle: &[u8],
        _field_path: &[String],
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let mut rc = ASTValue::None;

        if name == "local" {
            ctx.emit(NodeOp::Local);
            if !swizzle.is_empty() {
                ctx.emit(NodeOp::GetComponents(swizzle.to_vec()));
            }
        } else if name == "world" {
            ctx.emit(NodeOp::World);
            if !swizzle.is_empty() {
                ctx.emit(NodeOp::GetComponents(swizzle.to_vec()));
            }
        } else if name == "u" {
            ctx.emit(NodeOp::U);
        } else if name == "v" {
            ctx.emit(NodeOp::V);
        } else if name == "d" {
            ctx.emit(NodeOp::D);
        } else if name == "sdf" {
            ctx.emit(NodeOp::SDF);
        } else if name == "inside" {
            ctx.emit(NodeOp::Inside);
        } else if name == "hash" {
            ctx.emit(NodeOp::Hash);
        } else if name == "Clear" {
            ctx.emit(NodeOp::Clear);
        } else if self.functions.contains_key(&name) {
            rc = ASTValue::Function(name.clone(), vec![], Box::new(ASTValue::None));
        } else if self.user_functions.contains_key(&name) {
            rc = ASTValue::Function(name.clone(), vec![], Box::new(ASTValue::None));
        } else if let Some(index) = self.locals.get_index_of(&name) {
            ctx.emit(NodeOp::LoadLocal(index));
        } else {
            if let Some(index) = ctx.globals.get(&name) {
                ctx.emit(NodeOp::LoadGlobal(*index as usize));
                if !swizzle.is_empty() {
                    ctx.emit(NodeOp::GetComponents(swizzle.to_vec()));
                }
            }
        }
        // else if let Some(vv) = self.environment.get(&name) {
        //     rc = vv;

        Ok(rc)
    }

    fn value(
        &mut self,
        value: ASTValue,
        _swizzle: &[u8],
        _field_path: &[String],
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        match &value {
            ASTValue::Boolean(b) => {
                ctx.emit(NodeOp::Push(Value::from_bool(*b)));
            }
            ASTValue::Float(f) => {
                ctx.emit(NodeOp::Push(Value::from_float(*f)));
            }
            ASTValue::Float2(x, y) => {
                _ = x.accept(self, ctx)?.to_float().unwrap_or_default();
                _ = y.accept(self, ctx)?.to_float().unwrap_or_default();
                ctx.emit(NodeOp::Pack2);
            }
            ASTValue::Float3(x, y, z) => {
                _ = x.accept(self, ctx)?.to_float().unwrap_or_default();
                _ = y.accept(self, ctx)?.to_float().unwrap_or_default();
                _ = z.accept(self, ctx)?.to_float().unwrap_or_default();

                ctx.emit(NodeOp::Pack3);
            }
            _ => {}
        };

        Ok(ASTValue::None)
    }

    fn unary(
        &mut self,
        op: &UnaryOperator,
        expr: &Expr,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        _ = expr.accept(self, ctx)?;

        match op {
            UnaryOperator::Negate => ctx.emit(NodeOp::Not),
            UnaryOperator::Minus => ctx.emit(NodeOp::Neg),
        }

        Ok(ASTValue::None)
    }

    fn equality(
        &mut self,
        left: &Expr,
        op: &EqualityOperator,
        right: &Expr,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        _ = left.accept(self, ctx)?;
        _ = right.accept(self, ctx)?;

        match op {
            EqualityOperator::NotEqual => ctx.emit(NodeOp::Ne),
            EqualityOperator::Equal => ctx.emit(NodeOp::Eq),
        }

        Ok(ASTValue::None)
    }

    fn comparison(
        &mut self,
        left: &Expr,
        op: &ComparisonOperator,
        right: &Expr,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        _ = left.accept(self, ctx)?;
        _ = right.accept(self, ctx)?;

        match op {
            ComparisonOperator::Greater => ctx.emit(NodeOp::Gt),
            ComparisonOperator::GreaterEqual => ctx.emit(NodeOp::Ge),
            ComparisonOperator::Less => ctx.emit(NodeOp::Lt),
            ComparisonOperator::LessEqual => ctx.emit(NodeOp::Le),
        }

        Ok(ASTValue::None)
    }

    fn binary(
        &mut self,
        left: &Expr,
        op: &BinaryOperator,
        right: &Expr,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        _ = left.accept(self, ctx)?;
        _ = right.accept(self, ctx)?;

        match op {
            BinaryOperator::Add => {
                ctx.emit(NodeOp::Add);
            }
            BinaryOperator::Subtract => {
                ctx.emit(NodeOp::Sub);
            }
            BinaryOperator::Multiply => {
                ctx.emit(NodeOp::Mul);
            }
            BinaryOperator::Divide => {
                ctx.emit(NodeOp::Div);
            }
            BinaryOperator::Mod => {
                ctx.emit(NodeOp::Mod);
            }
        }

        Ok(ASTValue::None)
    }

    fn grouping(
        &mut self,
        expression: &Expr,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        expression.accept(self, ctx)
    }

    fn func_call(
        &mut self,
        callee: &Expr,
        _swizzle: &[u8],
        _field_path: &[String],
        args: &[Box<Expr>],
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let callee = callee.accept(self, ctx)?;

        if let ASTValue::Function(name, _func_args, _returns) = callee {
            if let Some(func) = &self.functions.get(&name).cloned() {
                if func.arguments as usize == args.len() {
                    for arg in args {
                        _ = arg.accept(self, ctx)?;
                    }
                    ctx.emit(func.op.clone());
                } else {
                    return Err(RuntimeError::new(
                        format!(
                            "Wrong amount of arguments for '{}', expected '{}' got '{}'",
                            name,
                            func.arguments as usize,
                            args.len(),
                        ),
                        loc,
                    ));
                }
            } else if let Some((arity, params, index)) = self.user_functions.get(&name) {
                let func_index = *index;
                let total_locals = params.len();
                if *arity != args.len() {
                    return Err(RuntimeError::new(
                        format!(
                            "Wrong amount of arguments for '{}', expected '{}' got '{}'",
                            name,
                            arity,
                            args.len()
                        ),
                        loc,
                    ));
                }

                for arg in args {
                    _ = arg.accept(self, ctx)?;
                }
                ctx.emit(NodeOp::FunctionCall(
                    args.len() as u8,
                    total_locals as u8,
                    func_index,
                ));
            } else {
                return Err(RuntimeError::new(
                    format!("Unknown function '{}'", name),
                    loc,
                ));
            }
        } else {
            return Err(RuntimeError::new(format!("Unknown function ''"), loc));
        }

        Ok(ASTValue::None)
    }

    fn struct_declaration(
        &mut self,
        _name: &str,
        _fields: &[(String, ASTValue)],
        _loc: &Location,
        _ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        /*
        let mut size: usize = 0;

        for (_, field) in fields {
            size += field.components() * ctx.precision.size();
        }

        ctx.structs
            .insert(name.to_string(), fields.to_vec().clone());

        ctx.struct_sizes.insert(name.to_string(), size);

        Ok(ASTValue::Struct("".to_string(), None, vec![]))
        */
        Ok(ASTValue::None)
    }

    /// Create a voxel box
    fn function_declaration(
        &mut self,
        objectd: &FunctionD,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        self.locals.clear();

        // Compile locals (and their optional default values)
        let mut cp: IndexMap<String, Option<Vec<NodeOp>>> = IndexMap::default();
        for (name, ast) in &objectd.locals {
            let mut def: Option<Vec<NodeOp>> = None;
            if let Some(ast) = ast {
                ctx.add_custom_target();
                _ = ast.accept(self, ctx)?;
                if let Some(code) = ctx.take_last_custom_target() {
                    def = Some(code);
                }
            }
            cp.insert(name.clone(), def);
            self.locals.insert(name.clone());
        }

        ctx.add_custom_target();

        let index = ctx.program.user_functions.len();

        self.user_functions
            .insert(objectd.name.clone(), (objectd.arity, cp.clone(), index));

        objectd.block.accept(self, ctx)?;
        if let Some(codes) = ctx.take_last_custom_target() {
            ctx.program
                .user_functions
                .push(Arc::from(codes.into_boxed_slice()));
        }

        self.locals.clear();

        Ok(ASTValue::None)
    }

    fn return_stmt(
        &mut self,
        expr: &Expr,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        _ = expr.accept(self, ctx)?;
        ctx.emit(NodeOp::Return);

        Ok(ASTValue::None)
    }

    fn if_stmt(
        &mut self,
        cond: &Expr,
        then_stmt: &Stmt,
        else_stmt: &Option<Box<Stmt>>,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        ctx.add_custom_target();
        _ = then_stmt.accept(self, ctx)?;
        let mut then_code = vec![];
        if let Some(code) = ctx.take_last_custom_target() {
            then_code = code;
        }

        let mut else_code = None;

        if let Some(else_stmt) = else_stmt {
            ctx.add_custom_target();
            _ = else_stmt.accept(self, ctx)?;
            if let Some(code) = ctx.take_last_custom_target() {
                else_code = Some(code);
            }
        }

        _ = cond.accept(self, ctx)?;
        ctx.emit(NodeOp::If(then_code, else_code));

        Ok(ASTValue::None)
    }

    fn ifclear_stmt(
        &mut self,
        block: &Stmt,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        ctx.add_custom_target();
        _ = block.accept(self, ctx)?;
        if let Some(code) = ctx.take_last_custom_target() {
            ctx.emit(NodeOp::IfClear(code));
        }

        Ok(ASTValue::None)
    }

    fn ternary(
        &mut self,
        _cond: &Expr,
        then_expr: &Expr,
        _else_expr: &Expr,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        /*
        ctx.add_line();
        let _rc = cond.accept(self, ctx)?;

        let param_name = format!("$_rpu_ternary_{}", ctx.ternary_counter);
        ctx.ternary_counter += 1;

        let instr = "(if".to_string();
        ctx.add_wat(&instr);
        ctx.add_indention();

        let instr = "(then".to_string();
        ctx.add_wat(&instr);
        ctx.add_indention();

        if let Some(d) = self.break_depth.last() {
            self.break_depth.push(d + 2);
        }*/

        let then_returns = then_expr.accept(self, ctx)?;

        /*
        let def_array = then_returns.write_definition("local", &param_name, &ctx.pr);
        for d in def_array {
            let c = format!("        {}\n", d);
            ctx.wat_locals.push_str(&c);
        }

        let a_set = then_returns.write_access("local.set", &param_name);
        for a in a_set.iter().rev() {
            ctx.add_wat(a);
        }

        ctx.remove_indention();
        ctx.add_wat(")");

        if let Some(d) = self.break_depth.last() {
            self.break_depth.push(d - 2);
        }

        if let Some(d) = self.break_depth.last() {
            self.break_depth.push(d + 2);
        }
        let instr = "(else".to_string();
        ctx.add_wat(&instr);
        ctx.add_indention();

        let else_returns = else_expr.accept(self, ctx)?;
        let b_set = else_returns.write_access("local.set", &param_name);
        for b in b_set.iter().rev() {
            ctx.add_wat(b);
        }

        ctx.remove_indention();
        ctx.add_wat(")");
        if let Some(d) = self.break_depth.last() {
            self.break_depth.push(d - 2);
        }

        ctx.remove_indention();
        ctx.add_wat(")");
        //ctx.add_line();

        let a_get = then_returns.write_access("local.get", &param_name);
        for a in a_get {
            ctx.add_wat(&a);
        }
        */

        Ok(then_returns)
    }

    fn for_stmt(
        &mut self,
        _init: &[Box<Stmt>],
        _conditions: &[Box<Expr>],
        _incr: &[Box<Expr>],
        _body_stmt: &Stmt,
        _loc: &Location,
        _ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        /*
        ctx.add_line();

        for i in init {
            let _rc = i.accept(self, ctx)?;
        }

        let instr = "(block".to_string();
        ctx.add_wat(&instr);
        ctx.add_indention();

        let instr = "(loop".to_string();
        ctx.add_wat(&instr);
        ctx.add_indention();

        self.break_depth.push(0);

        for cond in conditions {
            let _rc = cond.accept(self, ctx)?;

            let instr = "(i32.eqz)".to_string();
            ctx.add_wat(&instr);

            let instr = "(br_if 1)".to_string();
            ctx.add_wat(&instr);
        }

        let _rc = body_stmt.accept(self, ctx)?;

        for i in incr {
            let _rc = i.accept(self, ctx)?;
        }

        let instr = "(br 0)".to_string();
        ctx.add_wat(&instr);

        self.break_depth.pop();

        ctx.remove_indention();
        ctx.add_wat(")");

        ctx.remove_indention();
        ctx.add_wat(")");
        */
        Ok(ASTValue::None)
    }

    fn while_stmt(
        &mut self,
        _cond: &Expr,
        _body_stmt: &Stmt,
        _loc: &Location,
        _ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        /*
                ctx.add_line();

                let instr = "(block".to_string();
                ctx.add_wat(&instr);
                ctx.add_indention();

                let instr = "(loop".to_string();
                ctx.add_wat(&instr);
                ctx.add_indention();

                self.break_depth.push(0);

                let _rc = cond.accept(self, ctx)?;

                let instr = "(i32.eqz)".to_string();
                ctx.add_wat(&instr);

                let instr = "(br_if 1)".to_string();
                ctx.add_wat(&instr);

                let _rc = body_stmt.accept(self, ctx)?;

                let instr = "(br 0)".to_string();
                ctx.add_wat(&instr);

                self.break_depth.pop();

                ctx.remove_indention();
                ctx.add_wat(")");

                ctx.remove_indention();
                ctx.add_wat(")");
        */
        Ok(ASTValue::None)
    }

    fn break_stmt(
        &mut self,
        _loc: &Location,
        _ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        Ok(ASTValue::None)
    }

    fn empty_stmt(&mut self, _ctx: &mut Context) -> Result<ASTValue, RuntimeError> {
        Ok(ASTValue::None)
    }

    fn logical_expr(
        &mut self,
        left: &Expr,
        op: &LogicalOperator,
        right: &Expr,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let _l = left.accept(self, ctx)?;
        let _r = right.accept(self, ctx)?;

        match op {
            LogicalOperator::And => {
                ctx.emit(NodeOp::And);
            }
            LogicalOperator::Or => {
                ctx.emit(NodeOp::Or);
            }
        }

        Ok(ASTValue::None)
    }
}
