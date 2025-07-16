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
    ast_functions: FxHashMap<String, ASTValue>,
    functions: FxHashMap<String, ASTFunction>,
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

        let mut ast_functions: FxHashMap<String, ASTValue> = FxHashMap::default();
        ast_functions.insert(
            "length".to_string(),
            ASTValue::Function(
                "length".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "normalize".to_string(),
            ASTValue::Function(
                "normalize".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "smoothstep".to_string(),
            ASTValue::Function(
                "smoothstep".to_string(),
                vec![ASTValue::None, ASTValue::None, ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "mix".to_string(),
            ASTValue::Function(
                "mix".to_string(),
                vec![ASTValue::None, ASTValue::None, ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "dot".to_string(),
            ASTValue::Function(
                "dot".to_string(),
                vec![ASTValue::None, ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "cross".to_string(),
            ASTValue::Function(
                "cross".to_string(),
                vec![ASTValue::None, ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "sqrt".to_string(),
            ASTValue::Function(
                "sqrt".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "sin".to_string(),
            ASTValue::Function(
                "sin".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "cos".to_string(),
            ASTValue::Function(
                "cos".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "ceil".to_string(),
            ASTValue::Function(
                "ceil".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "floor".to_string(),
            ASTValue::Function(
                "floor".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "fract".to_string(),
            ASTValue::Function(
                "fract".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "abs".to_string(),
            ASTValue::Function(
                "abs".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "tan".to_string(),
            ASTValue::Function(
                "tan".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "atan".to_string(),
            ASTValue::Function(
                "atan".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "degrees".to_string(),
            ASTValue::Function(
                "degrees".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "radians".to_string(),
            ASTValue::Function(
                "radians".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "sign".to_string(),
            ASTValue::Function(
                "sign".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "exp".to_string(),
            ASTValue::Function(
                "exp".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "log".to_string(),
            ASTValue::Function(
                "log".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "rand".to_string(),
            ASTValue::Function("rand".to_string(), vec![], Box::new(ASTValue::None)),
        );
        ast_functions.insert(
            "max".to_string(),
            ASTValue::Function(
                "max".to_string(),
                vec![ASTValue::None, ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "min".to_string(),
            ASTValue::Function(
                "min".to_string(),
                vec![ASTValue::None, ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "pow".to_string(),
            ASTValue::Function(
                "pow".to_string(),
                vec![ASTValue::None, ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "mod".to_string(),
            ASTValue::Function(
                "mod".to_string(),
                vec![ASTValue::None, ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "step".to_string(),
            ASTValue::Function(
                "step".to_string(),
                vec![ASTValue::None, ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );
        ast_functions.insert(
            "clamp".to_string(),
            ASTValue::Function(
                "clamp".to_string(),
                vec![ASTValue::None, ASTValue::None, ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );

        Self {
            environment: Environment::default(),
            ast_functions,
            functions,
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
        // let instr = "(block".to_string();
        // ctx.add_wat(&instr);
        // ctx.add_indention();

        // if let Some(d) = self.break_depth.last() {
        //     self.break_depth.push(d + 1);
        // }

        let mut value = ASTValue::None;

        self.environment.begin_scope(ASTValue::None, false);
        for stmt in list {
            value = stmt.accept(self, ctx)?;
            //println!("Block statement executed with result: {:?}", rc);
        }
        self.environment.end_scope();

        // if let Some(d) = self.break_depth.last() {
        //     self.break_depth.push(d - 1);
        // }

        // ctx.remove_indention();
        // ctx.add_wat(")");

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

    fn voxel(
        &mut self,
        define_object: &VoxelD,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let mut cpy = define_object.clone();

        if let Some(size) = define_object.params.get("size") {
            ctx.set_target(OutputTarget::Custom);
            _ = size.accept(self, ctx)?;

            cpy.size = ctx.program.custom.clone();
        }

        ctx.program.voxels.insert(cpy.name.clone(), cpy);
        ctx.set_target(OutputTarget::Voxels(define_object.name.clone(), vec![]));

        if let Some(block) = &define_object.block {
            block.accept(self, ctx)?;
        }

        ctx.set_target(OutputTarget::Globals);

        Ok(ASTValue::None)
    }

    fn shape(
        &mut self,
        objectd: &ShapeD,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let shape_index = if let Some(voxel) = ctx.get_output_voxel() {
            voxel.shapes.len()
        } else {
            0
        };

        if objectd.name == "Rect" {
            let shape = Rect::new();

            if let Some(voxel) = ctx.get_output_voxel() {
                voxel.shapes.push(Box::new(shape));
            }
        }

        let target_cpy = ctx.current_target.clone();

        if let OutputTarget::Voxels(id, rec) = &ctx.current_target {
            let mut rec = rec.clone();
            rec.push(shape_index);
            ctx.set_target(OutputTarget::Voxels(id.clone(), rec));
        }

        if let Some(block) = &objectd.block {
            block.accept(self, ctx)?;
        }

        ctx.set_target(target_cpy);

        // if let Some(size) = define_object.params.get("size") {
        //     ctx.set_target(OutputTarget::Custom);
        //     _ = size.accept(self, ctx)?;

        //     cpy.size = ctx.program.custom.clone();
        // }

        // ctx.program.definitons.insert(cpy.name.clone(), cpy);
        // ctx.set_target(OutputTarget::Definitions(define_object.name.clone()));

        // if let Some(block) = &define_object.block {
        //     block.accept(self, ctx)?;
        // }

        // ctx.set_target(OutputTarget::Globals);

        Ok(ASTValue::None)
    }

    fn place(
        &mut self,
        id: &String,
        _params: &FxHashMap<String, Box<Expr>>,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        if ctx.program.voxels.contains_key(id) {
            ctx.emit(NodeOp::Place(id.clone()));
        }

        Ok(ASTValue::None)
    }

    fn var_declaration(
        &mut self,
        name: &str,
        static_type: &ASTValue,
        expression: &Expr,
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let v = expression.accept(self, ctx)?;

        /*
        // Compare incoming expression type with the static type.
        if v.to_type() != static_type.to_type() {
            return Err(RPUError::loc(
                format!(
                    "Variable '{}' has type '{}', but expression has type '{}'",
                    ctx.remove_trailing_var_identifier(name),
                    static_type.to_type(),
                    v.to_type()
                ),
                loc,
            ));
        }

        // Global function definition. We write these out in the module header in gen_wat().
        if self.environment.is_global_scope() {
            ctx.globals.insert(name.to_string(), v.clone());
            return Ok(ASTValue::None);
        }

        match &v {
            ASTValue::Int(_, _) => {
                let instr = format!("(local ${} i{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));

                let instr = format!("local.set ${}", name);
                ctx.add_wat(&instr);
            }
            ASTValue::Int2(_, _, _) => {
                let instr = format!("(local ${}_x i{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));
                let instr = format!("(local ${}_y i{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));

                let instr = format!("local.set ${}_y", name);
                ctx.add_wat(&instr);
                let instr = format!("local.set ${}_x", name);
                ctx.add_wat(&instr);
            }
            ASTValue::Int3(_, _, _, _) => {
                let instr = format!("(local ${}_x i{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));
                let instr = format!("(local ${}_y i{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));
                let instr = format!("(local ${}_z i{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));

                let instr = format!("local.set ${}_z", name);
                ctx.add_wat(&instr);
                let instr = format!("local.set ${}_y", name);
                ctx.add_wat(&instr);
                let instr = format!("local.set ${}_x", name);
                ctx.add_wat(&instr);
            }
            ASTValue::Int4(_, _, _, _, _) => {
                let instr = format!("(local ${}_x i{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));
                let instr = format!("(local ${}_y i{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));
                let instr = format!("(local ${}_z i{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));
                let instr = format!("(local ${}_w i{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));

                let instr = format!("local.set ${}_w", name);
                ctx.add_wat(&instr);
                let instr = format!("local.set ${}_z", name);
                ctx.add_wat(&instr);
                let instr = format!("local.set ${}_y", name);
                ctx.add_wat(&instr);
                let instr = format!("local.set ${}_x", name);
                ctx.add_wat(&instr);
            }
            ASTValue::Float(_, _) => {
                let instr = format!("(local ${} f{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));

                let instr = format!("local.set ${}", name);
                ctx.add_wat(&instr);
            }
            ASTValue::Float2(_, _, _) => {
                let instr = format!("(local ${}_x f{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));
                let instr = format!("(local ${}_y f{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));

                let instr = format!("local.set ${}_y", name);
                ctx.add_wat(&instr);
                let instr = format!("local.set ${}_x", name);
                ctx.add_wat(&instr);
            }
            ASTValue::Float3(_, _, _, _) => {
                let instr = format!("(local ${}_x f{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));
                let instr = format!("(local ${}_y f{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));
                let instr = format!("(local ${}_z f{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));

                let instr = format!("local.set ${}_z", name);
                ctx.add_wat(&instr);
                let instr = format!("local.set ${}_y", name);
                ctx.add_wat(&instr);
                let instr = format!("local.set ${}_x", name);
                ctx.add_wat(&instr);
            }
            ASTValue::Float4(_, _, _, _, _) => {
                let instr = format!("(local ${}_x f{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));
                let instr = format!("(local ${}_y f{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));
                let instr = format!("(local ${}_z f{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));
                let instr = format!("(local ${}_w f{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));

                let instr = format!("local.set ${}_w", name);
                ctx.add_wat(&instr);
                let instr = format!("local.set ${}_z", name);
                ctx.add_wat(&instr);
                let instr = format!("local.set ${}_y", name);
                ctx.add_wat(&instr);
                let instr = format!("local.set ${}_x", name);
                ctx.add_wat(&instr);
            }
            ASTValue::Mat2(_, _) | ASTValue::Mat3(_, _) | ASTValue::Mat4(_, _) => {
                let comps = v.write_definition("local", name, &ctx.pr);
                for c in comps {
                    ctx.wat_locals.push_str(&format!("        {}\n", c));
                }
                let comps = v.write_access("local.set", name);
                for c in comps.iter().rev() {
                    ctx.add_wat(c);
                }
            }
            ASTValue::Struct(_, _, _) => {
                // Copy the incoming struct mem ptr to the variable
                let comps = v.write_definition("local", name, &ctx.pr);
                for c in comps {
                    ctx.wat_locals.push_str(&format!("        {}\n", c));
                }
                let comps = v.write_access("local.set", name);
                for c in comps.iter().rev() {
                    ctx.add_wat(c);
                }
            }
            _ => {}
        }
        */
        self.environment.define(name.to_string(), v);

        Ok(ASTValue::None)
    }

    fn variable_assignment(
        &mut self,
        name: String,
        op: &AssignmentOperator,
        swizzle: &[u8],
        field_path: &[String],
        expression: &Expr,
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let mut v = expression.accept(self, ctx)?;
        /*
        let incoming_components = v.components();

        if field_path.is_empty() {
            // Use the type of the variable
            if let Some(vv) = self.environment.get(&name) {
                if swizzle.is_empty() && incoming_components != vv.components() {
                    return Err(RPUError::loc(
                        format!(
                            "Variable '{}' has {} component(s), but expression has {}",
                            ctx.remove_trailing_var_identifier(&name),
                            v.components(),
                            incoming_components
                        ),
                        loc,
                    ));
                }
                v = vv;
            }

            if swizzle.is_empty() {
                if incoming_components != v.components() {
                    return Err(RPUError::loc(
                        format!(
                            "Variable '{}' has {} component(s), but expression has {}",
                            ctx.remove_trailing_var_identifier(&name),
                            v.components(),
                            incoming_components
                        ),
                        loc,
                    ));
                }
            } else if incoming_components != swizzle.len() {
                return Err(RPUError::loc(
                    format!(
                        "Variable '{}' has {} swizzle, but expression has {} component(s)",
                        ctx.remove_trailing_var_identifier(&name),
                        swizzle.len(),
                        incoming_components
                    ),
                    loc,
                ));
            }
        } else {
            // For structs
            if let Some(vv) = self.environment.get(&name) {
                v = vv;
            }
        }

        match &v {
            ASTValue::Int(_, _) | ASTValue::Float(_, _) => match op {
                AssignmentOperator::Assign => {
                    let instr = format!("local.set ${}", name);
                    ctx.add_wat(&instr);
                }
                _ => {
                    let instr = format!("local.get ${}", name);
                    ctx.add_wat(&instr);
                    let instr = format!(
                        "{}.{}",
                        v.to_wat_component_type(&ctx.pr),
                        op.to_wat_type(&v)
                    );
                    ctx.add_wat(&instr);
                    let instr = format!("local.set ${}", name);
                    ctx.add_wat(&instr);
                }
            },
            ASTValue::Int2(_, _, _) | ASTValue::Float2(_, _, _) => {
                if swizzle.is_empty() {
                    match op {
                        AssignmentOperator::Assign => {
                            let instr = format!("local.set ${}_y", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_x", name);
                            ctx.add_wat(&instr);
                        }
                        _ => {
                            let temp = ctx.add_temporary(&v);

                            let instr = format!("local.set ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}_y", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}", temp);
                            ctx.add_wat(&instr);

                            let instr = format!(
                                "{}.{}",
                                v.to_wat_component_type(&ctx.pr),
                                op.to_wat_type(&v)
                            );
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_y", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}_x", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!(
                                "{}.{}",
                                v.to_wat_component_type(&ctx.pr),
                                op.to_wat_type(&v)
                            );
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_x", name);
                            ctx.add_wat(&instr);
                        }
                    }
                } else {
                    for s in swizzle.iter().rev() {
                        match s {
                            0 => {
                                let instr = format!("(local.set ${}_x)", name);
                                ctx.add_wat(&instr);
                            }
                            1 => {
                                let instr = format!("(local.set ${}_y)", name);
                                ctx.add_wat(&instr);
                            }
                            _ => {
                                return Err(RPUError::loc(
                                    format!(
                                        "Swizzle '{}' out of range for '{}'",
                                        ctx.deswizzle(*s),
                                        name
                                    ),
                                    loc,
                                ));
                            }
                        }
                    }
                }
            }
            ASTValue::Int3(_, _, _, _) | ASTValue::Float3(_, _, _, _) => {
                if swizzle.is_empty() {
                    match op {
                        AssignmentOperator::Assign => {
                            let instr = format!("local.set ${}_z", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_y", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_x", name);
                            ctx.add_wat(&instr);
                        }
                        _ => {
                            let temp = ctx.add_temporary(&v);

                            let instr = format!("local.set ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}_z", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!(
                                "{}.{}",
                                v.to_wat_component_type(&ctx.pr),
                                op.to_wat_type(&v)
                            );
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_z", name);
                            ctx.add_wat(&instr);

                            let instr = format!("local.set ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}_y", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!(
                                "{}.{}",
                                v.to_wat_component_type(&ctx.pr),
                                op.to_wat_type(&v)
                            );
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_y", name);
                            ctx.add_wat(&instr);

                            let instr = format!("local.set ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}_x", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!(
                                "{}.{}",
                                v.to_wat_component_type(&ctx.pr),
                                op.to_wat_type(&v)
                            );
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_x", name);
                            ctx.add_wat(&instr);
                        }
                    }
                } else {
                    for s in swizzle.iter().rev() {
                        match s {
                            0 => {
                                let instr = format!("local.set ${}_x", name);
                                ctx.add_wat(&instr);
                            }
                            1 => {
                                let instr = format!("local.set ${}_y", name);
                                ctx.add_wat(&instr);
                            }
                            2 => {
                                let instr = format!("local.set ${}_z", name);
                                ctx.add_wat(&instr);
                            }
                            _ => {
                                return Err(RPUError::loc(
                                    format!(
                                        "Swizzle '{}' out of range for '{}'",
                                        ctx.deswizzle(*s),
                                        name
                                    ),
                                    loc,
                                ));
                            }
                        }
                    }
                }
            }
            ASTValue::Int4(_, _, _, _, _) | ASTValue::Float4(_, _, _, _, _) => {
                if swizzle.is_empty() {
                    match op {
                        AssignmentOperator::Assign => {
                            let instr = format!("local.set ${}_w", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_z", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_y", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_x", name);
                            ctx.add_wat(&instr);
                        }
                        _ => {
                            let temp = ctx.add_temporary(&v);

                            let instr = format!("local.set ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}_w", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!(
                                "{}.{}",
                                v.to_wat_component_type(&ctx.pr),
                                op.to_wat_type(&v)
                            );
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_w", name);
                            ctx.add_wat(&instr);

                            let instr = format!("local.set ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}_z", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!(
                                "{}.{}",
                                v.to_wat_component_type(&ctx.pr),
                                op.to_wat_type(&v)
                            );
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_z", name);
                            ctx.add_wat(&instr);

                            let instr = format!("local.set ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}_y", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!(
                                "{}.{}",
                                v.to_wat_component_type(&ctx.pr),
                                op.to_wat_type(&v)
                            );
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_y", name);
                            ctx.add_wat(&instr);

                            let instr = format!("local.set ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}_x", name);
                            ctx.add_wat(&instr);
                            let instr = format!("local.get ${}", temp);
                            ctx.add_wat(&instr);
                            let instr = format!(
                                "{}.{}",
                                v.to_wat_component_type(&ctx.pr),
                                op.to_wat_type(&v)
                            );
                            ctx.add_wat(&instr);
                            let instr = format!("local.set ${}_x", name);
                            ctx.add_wat(&instr);
                        }
                    }
                } else {
                    for s in swizzle.iter().rev() {
                        match s {
                            0 => {
                                let instr = format!("local.set ${}_x", name);
                                ctx.add_wat(&instr);
                            }
                            1 => {
                                let instr = format!("local.set ${}_y", name);
                                ctx.add_wat(&instr);
                            }
                            2 => {
                                let instr = format!("local.set ${}_z", name);
                                ctx.add_wat(&instr);
                            }
                            3 => {
                                let instr = format!("local.set ${}_w", name);
                                ctx.add_wat(&instr);
                            }
                            _ => {
                                return Err(RPUError::loc(
                                    format!(
                                        "Swizzle '{}' out of range for '{}'",
                                        ctx.deswizzle(*s),
                                        name
                                    ),
                                    loc,
                                ));
                            }
                        }
                    }
                }
            }
            ASTValue::Struct(struct_name, _, _) => {
                if field_path.is_empty() {
                    // We got an incoming complete struct, just copy the mem ptr
                    // TODO Check if the struct types are the same
                    let instr = format!("(local.set ${})", name);
                    ctx.add_wat(&instr);
                } else {
                    // We got a field path, so we need to copy the field
                    _ = ctx.access_struct(&name, struct_name, field_path, true, loc)?;
                }
            }
            _ => {}
        }
        */
        self.environment.assign(&name, v);

        Ok(ASTValue::None)
    }

    fn variable(
        &mut self,
        name: String,
        swizzle: &[u8],
        field_path: &[String],
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let instr = String::new();
        let mut rc = ASTValue::None;

        /*
        if swizzle.len() > 4 {
            return Err(RPUError::loc(
                format!(
                    "Maximal swizzle length is 4, got {} for '{}'",
                    swizzle.len(),
                    name
                ),
                loc,
            ));
        }

        let mut scope = "local";

        // Check if the variable is in the environment
        let mut vv = self.environment.get(&name);

        // Check if the variable is a global
        if vv.is_none() {
            if let Some(global_value) = ctx.globals.get(&name) {
                scope = "global";
                vv = Some(global_value.clone());
            }
        }

        fn process_swizzle(
            v: &ASTValue,
            swizzle: &[u8],
            scope: &str,
            name: &str,
            ctx: &mut Context,
        ) {
            if !swizzle.is_empty() {
                let components = v.components();
                for s in swizzle {
                    if *s < components as u8 {
                        let instr = format!("{}.get ${}_{}", scope, name, ctx.deswizzle(*s));
                        ctx.add_wat(&instr);
                    }
                }
            }
        }

        if let Some(v) = vv {
            if !swizzle.is_empty() {
                rc = ctx.create_value_from_swizzle(&v, swizzle.len());
            }
            match &v {
                ASTValue::Int(_, _) | ASTValue::Float(_, _) => {
                    let instr = format!("{}.get ${}", scope, name);
                    ctx.add_wat(&instr);
                    rc = v.clone();
                }
                ASTValue::Int2(_, _, _) | ASTValue::Float2(_, _, _) => {
                    if swizzle.is_empty() {
                        let instr = format!("{}.get ${}_x", scope, name);
                        ctx.add_wat(&instr);
                        let instr = format!("{}.get ${}_y", scope, name);
                        ctx.add_wat(&instr);
                        rc = v.clone();
                    } else {
                        process_swizzle(&v, swizzle, scope, &name, ctx);
                    }
                }
                ASTValue::Int3(_, _, _, _) | ASTValue::Float3(_, _, _, _) => {
                    if swizzle.is_empty() {
                        let instr = format!("{}.get ${}_x", scope, name);
                        ctx.add_wat(&instr);
                        let instr = format!("{}.get ${}_y", scope, name);
                        ctx.add_wat(&instr);
                        let instr = format!("{}.get ${}_z", scope, name);
                        ctx.add_wat(&instr);
                        rc = v.clone();
                    } else {
                        process_swizzle(&v, swizzle, scope, &name, ctx);
                    }
                }
                ASTValue::Int4(_, _, _, _, _) | ASTValue::Float4(_, _, _, _, _) => {
                    if swizzle.is_empty() {
                        let instr = format!("{}.get ${}_x", scope, name);
                        ctx.add_wat(&instr);
                        let instr = format!("{}.get ${}_y", scope, name);
                        ctx.add_wat(&instr);
                        let instr = format!("{}.get ${}_z", scope, name);
                        ctx.add_wat(&instr);
                        let instr = format!("{}.get ${}_w", scope, name);
                        ctx.add_wat(&instr);
                        rc = v.clone();
                    } else {
                        process_swizzle(&v, swizzle, scope, &name, ctx);
                    }
                }
                ASTValue::Mat2(_, _) | ASTValue::Mat3(_, _) | ASTValue::Mat4(_, _) => {
                    let instr = format!("{}.get", scope);
                    let comps = v.write_access(&instr, &name);

                    for c in comps {
                        ctx.add_wat(&c);
                    }

                    rc = v;
                }
                ASTValue::Struct(struct_name, _, _) => {
                    rc = ctx.access_struct(&name, struct_name, field_path, false, loc)?;
                }

                _ => {}
            }
        } else if let Some(ASTValue::Function(name, args, body)) = self.functions.get(&name) {
            rc = ASTValue::Function(name.clone(), args.clone(), body.clone());
        } else {
            return Err(RPUError::loc(format!("Unknown identifier '{}'", name), loc));
        }

        if !instr.is_empty() {
            ctx.add_wat(&instr);
        }
        */

        if name == "local" {
            ctx.emit(NodeOp::Local);
        }

        if let Some(vv) = self.environment.get(&name) {
            rc = vv;
        } else if let Some(ASTValue::Function(name, args, body)) = self.ast_functions.get(&name) {
            rc = ASTValue::Function(name.clone(), args.clone(), body.clone());
        }

        Ok(rc)
    }

    fn value(
        &mut self,
        value: ASTValue,
        swizzle: &[u8],
        _field_path: &[String],
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        match &value {
            ASTValue::Boolean(b) => {
                ctx.emit(NodeOp::Push(Value::from_bool(*b)));
            }
            ASTValue::Float(f) => {
                ctx.emit(NodeOp::Push(Value::from_float(*f)));
            }
            ASTValue::Float3(x, y, z) => {
                let x = x.accept(self, ctx)?.to_float().unwrap_or_default();
                let y = y.accept(self, ctx)?.to_float().unwrap_or_default();
                let z = z.accept(self, ctx)?.to_float().unwrap_or_default();

                ctx.emit(NodeOp::Pack3);
            }
            _ => {}
        };

        Ok(ASTValue::None)
    }

    fn unary(
        &mut self,
        _op: &UnaryOperator,
        expr: &Expr,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let v = expr.accept(self, ctx)?;

        // !, - have the same behavior right now.
        // let func_name = ctx.gen_vec_operation(v.components() as u32, "neg");
        // let instr = format!("(call ${})", func_name);
        // ctx.add_wat(&instr);

        Ok(v)
    }

    fn equality(
        &mut self,
        left: &Expr,
        op: &EqualityOperator,
        right: &Expr,
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let left_value = left.accept(self, ctx)?;
        let right_value = right.accept(self, ctx)?;

        /*
        if left_value.to_type() != right_value.to_type() {
            return Err(RPUError::loc(
                format!(
                    "Type mismatch for '{}' operator: '{}' and '{}'",
                    op.describe(),
                    left_value.to_type(),
                    right_value.to_type()
                ),
                loc,
            ));
        }

        let instr = if !left_value.is_float_based() {
            match op {
                EqualityOperator::NotEqual => format!("(i{}.ne)", ctx.pr),
                EqualityOperator::Equal => format!("(i{}.eq)", ctx.pr),
            }
        } else {
            match op {
                EqualityOperator::NotEqual => format!("(f{}.ne)", ctx.pr),
                EqualityOperator::Equal => format!("(f{}.eq)", ctx.pr),
            }
        };
        ctx.add_wat(&instr);
        */

        Ok(ASTValue::None)
    }

    fn comparison(
        &mut self,
        left: &Expr,
        op: &ComparisonOperator,
        right: &Expr,
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let left_value = left.accept(self, ctx)?;
        let right_value = right.accept(self, ctx)?;

        /*
        if left_value.to_type() != right_value.to_type() {
            return Err(RPUError::loc(
                format!(
                    "Type mismatch for '{}' operator: '{}' and '{}'",
                    op.describe(),
                    left_value.to_type(),
                    right_value.to_type()
                ),
                loc,
            ));
        }

        let is_float_based = left_value.is_float_based();

        let instr = if !is_float_based {
            match op {
                ComparisonOperator::Greater => format!("(i{}.gt_s)", ctx.pr),
                ComparisonOperator::GreaterEqual => format!("(i{}.ge_s)", ctx.pr),
                ComparisonOperator::Less => format!("(i{}.lt_s)", ctx.pr),
                ComparisonOperator::LessEqual => format!("(i{}.le_s)", ctx.pr),
            }
        } else {
            match op {
                ComparisonOperator::Greater => format!("(f{}.gt)", ctx.pr),
                ComparisonOperator::GreaterEqual => format!("(f{}.ge)", ctx.pr),
                ComparisonOperator::Less => format!("(f{}.lt)", ctx.pr),
                ComparisonOperator::LessEqual => format!("(f{}.le)", ctx.pr),
            }
        };

        ctx.add_wat(&instr);*/

        Ok(left_value)
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
            _ => {}
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

        let functions = self.functions.clone();
        if let ASTValue::Function(name, _func_args, _returns) = callee {
            if let Some(func) = &self.functions.get(&name).cloned() {
                if func.arguments as usize == args.len() {
                    for arg in args {
                        _ = arg.accept(self, ctx)?;
                    }
                    ctx.emit(func.op.clone());
                } else {
                    return Err(RuntimeError::new(
                        format!("Wrong amount of arguments for '{}'", name),
                        loc,
                    ));
                }
            }
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

    fn func_declaration(
        &mut self,
        name: &str,
        args: &[ASTValue],
        body: &[Box<Stmt>],
        returns: &ASTValue,
        export: &bool,
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        /*
                self.functions.insert(
                    name.to_string(),
                    ASTValue::Function(name.to_string(), args.to_vec(), Box::new(returns.clone())),
                );

                let mut params = String::new();

                ctx.clear_locals();
                self.environment.begin_scope(returns.clone(), true);

                for param in args {
                    // Save the param into the environment
                    if let Some(name) = param.name() {
                        self.environment.define(name, param.clone());
                    }

                    match param {
                        ASTValue::Int(name, _) => {
                            params += &format!(
                                "(param ${} i{})",
                                name.clone().unwrap(),
                                ctx.precision.describe()
                            );
                        }
                        ASTValue::Int2(name, _, _) => {
                            params += &format!(
                                "(param ${}_x i{}) (param ${}_y i{})",
                                name.clone().unwrap(),
                                ctx.precision.describe(),
                                name.clone().unwrap(),
                                ctx.precision.describe()
                            );
                        }
                        ASTValue::Int3(name, _, _, _) => {
                            params += &format!(
                                "(param ${}_x i{}) (param ${}_y i{}) (param ${}_z i{})",
                                name.clone().unwrap(),
                                ctx.precision.describe(),
                                name.clone().unwrap(),
                                ctx.precision.describe(),
                                name.clone().unwrap(),
                                ctx.precision.describe()
                            );
                        }
                        ASTValue::Int4(name, _, _, _, _) => {
                            params += &format!(
                                "(param ${}_x i{}) (param ${}_y i{}) (param ${}_z i{}) (param ${}_w i{})",
                                name.clone().unwrap(),
                                ctx.precision.describe(),
                                name.clone().unwrap(),
                                ctx.precision.describe(),
                                name.clone().unwrap(),
                                ctx.precision.describe(),
                                name.clone().unwrap(),
                                ctx.precision.describe()
                            );
                        }
                        ASTValue::Float(name, _) => {
                            params += &format!(
                                "(param ${} f{})",
                                name.clone().unwrap(),
                                ctx.precision.describe()
                            );
                        }
                        ASTValue::Float2(name, _, _) => {
                            params += &format!(
                                "(param ${}_x f{}) (param ${}_y f{})",
                                name.clone().unwrap(),
                                ctx.precision.describe(),
                                name.clone().unwrap(),
                                ctx.precision.describe()
                            );
                        }
                        ASTValue::Float3(name, _, _, _) => {
                            params += &format!(
                                "(param ${}_x f{}) (param ${}_y f{}) (param ${}_z f{})",
                                name.clone().unwrap(),
                                ctx.precision.describe(),
                                name.clone().unwrap(),
                                ctx.precision.describe(),
                                name.clone().unwrap(),
                                ctx.precision.describe()
                            );
                        }
                        ASTValue::Float4(name, _, _, _, _) => {
                            params += &format!(
                                "(param ${}_x f{}) (param ${}_y f{}) (param ${}_z f{}) (param ${}_w f{})",
                                name.clone().unwrap(),
                                ctx.precision.describe(),
                                name.clone().unwrap(),
                                ctx.precision.describe(),
                                name.clone().unwrap(),
                                ctx.precision.describe(),
                                name.clone().unwrap(),
                                ctx.precision.describe()
                            );
                        }
                        ASTValue::Struct(_, param_name, _) => {
                            params += &format!("(param ${} i32)", param_name.clone().unwrap());
                        }
                        _ => {}
                    }
                }

                let mut return_type = String::new();

                if let Some(r) = returns.to_wat_type(&ctx.pr) {
                    return_type = format!("(result {})", r);
                }

                let export_str = if *export {
                    format!(" (export \"{}\")", name)
                } else {
                    "".to_string()
                };

                let instr = format!("(func ${}{} {} {}", name, export_str, params, return_type);

                ctx.add_line();
                ctx.add_wat(&format!(";; function '{}'", name));
                ctx.add_wat(&instr);
                ctx.add_indention();

                ctx.wat.push_str("__LOCALS__");

                let mut last_value = ASTValue::None;
                for stmt in body {
                    last_value = stmt.accept(self, ctx)?;
                }

                if let Some(ret) = self.environment.get_return() {
                    if ret.to_type() != "void" && last_value.to_type() != ret.to_type() {
                        return Err(RPUError::loc(
                            format!("Function '{}' does not end with a 'return' statement", name),
                            loc,
                        ));
                    }
                }

                self.environment.end_scope();

                ctx.wat = ctx.wat.replace("__LOCALS__", &ctx.wat_locals);
                ctx.wat_locals = String::new();

                ctx.remove_indention();
                ctx.add_wat(")");
        */
        Ok(ASTValue::None)
    }

    fn return_stmt(
        &mut self,
        expr: &Expr,
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let rc = expr.accept(self, ctx)?;

        /*
        if let Some(ret) = self.environment.get_return() {
            if rc.to_type() != ret.to_type() {
                return Err(RPUError::loc(
                    format!(
                        "Invalid return type '{}', should be '{}'",
                        rc.to_type(),
                        ret.to_type()
                    ),
                    loc,
                ));
            }
        }

        ctx.add_wat("(return)");
        */

        Ok(rc)
    }

    fn if_stmt(
        &mut self,
        cond: &Expr,
        then_stmt: &Stmt,
        else_stmt: &Option<Box<Stmt>>,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        /*
        ctx.add_line();
        let _rc = cond.accept(self, ctx)?;

        let instr = "(if".to_string();
        ctx.add_wat(&instr);
        ctx.add_indention();

        let instr = "(then".to_string();
        ctx.add_wat(&instr);
        ctx.add_indention();

        if let Some(d) = self.break_depth.last() {
            self.break_depth.push(d + 2);
        }

        let _ = then_stmt.accept(self, ctx)?;

        ctx.remove_indention();
        ctx.add_wat(")");

        if let Some(d) = self.break_depth.last() {
            self.break_depth.push(d - 2);
        }

        if let Some(es) = else_stmt {
            if let Some(d) = self.break_depth.last() {
                self.break_depth.push(d + 2);
            }
            let instr = "(else".to_string();
            ctx.add_wat(&instr);
            ctx.add_indention();
            let _ = es.accept(self, ctx)?;
            ctx.remove_indention();
            ctx.add_wat(")");
            if let Some(d) = self.break_depth.last() {
                self.break_depth.push(d - 2);
            }
        }

        ctx.remove_indention();
        ctx.add_wat(")");
        //ctx.add_line();
        */
        Ok(ASTValue::None)
    }

    fn ternary(
        &mut self,
        cond: &Expr,
        then_expr: &Expr,
        else_expr: &Expr,
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
        init: &[Box<Stmt>],
        conditions: &[Box<Expr>],
        incr: &[Box<Expr>],
        body_stmt: &Stmt,
        _loc: &Location,
        ctx: &mut Context,
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
        cond: &Expr,
        body_stmt: &Stmt,
        _loc: &Location,
        ctx: &mut Context,
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

    fn break_stmt(&mut self, _loc: &Location, ctx: &mut Context) -> Result<ASTValue, RuntimeError> {
        // if let Some(d) = self.break_depth.last() {
        //     let instr = format!("(br {})", d);
        //     ctx.add_wat(&instr);
        // }

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

        /*
        match op {
            LogicalOperator::And => {
                let instr = "(i32.and)".to_string();
                ctx.add_wat(&instr);
            }
            LogicalOperator::Or => {
                let instr = "(i32.or)".to_string();
                ctx.add_wat(&instr);
            }
        }*/

        Ok(ASTValue::None)
    }
}
