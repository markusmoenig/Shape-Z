use crate::expr_float;
use crate::prelude::*;

/// ExecuteVisitor
pub struct CompileVisitor {
    pub environment: Environment,
    functions: FxHashMap<String, ASTValue>,
    break_depth: Vec<i32>,
}

impl Visitor for CompileVisitor {
    fn new() -> Self
    where
        Self: Sized,
    {
        let mut functions: FxHashMap<String, ASTValue> = FxHashMap::default();

        functions.insert(
            "length".to_string(),
            ASTValue::Function(
                "length".to_string(),
                vec![ASTValue::None],
                Box::new(ASTValue::None),
            ),
        );

        /*

        functions.insert(
            "normalize".to_string(),
            ASTValue::Function(
                "normalize".to_string(),
                vec![ASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "smoothstep".to_string(),
            ASTASTValue::Function(
                "smoothstep".to_string(),
                vec![ASTASTValue::None, ASTASTValue::None, ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "mix".to_string(),
            ASTASTValue::Function(
                "mix".to_string(),
                vec![ASTASTValue::None, ASTASTValue::None, ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "dot".to_string(),
            ASTASTValue::Function(
                "dot".to_string(),
                vec![ASTASTValue::None, ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "cross".to_string(),
            ASTASTValue::Function(
                "cross".to_string(),
                vec![ASTASTValue::None, ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "sqrt".to_string(),
            ASTASTValue::Function(
                "sqrt".to_string(),
                vec![ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "sin".to_string(),
            ASTASTValue::Function(
                "sin".to_string(),
                vec![ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "cos".to_string(),
            ASTASTValue::Function(
                "cos".to_string(),
                vec![ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "ceil".to_string(),
            ASTASTValue::Function(
                "ceil".to_string(),
                vec![ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "floor".to_string(),
            ASTASTValue::Function(
                "floor".to_string(),
                vec![ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "fract".to_string(),
            ASTASTValue::Function(
                "fract".to_string(),
                vec![ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "abs".to_string(),
            ASTASTValue::Function(
                "abs".to_string(),
                vec![ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "tan".to_string(),
            ASTASTValue::Function(
                "tan".to_string(),
                vec![ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "atan".to_string(),
            ASTASTValue::Function(
                "atan".to_string(),
                vec![ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "degrees".to_string(),
            ASTASTValue::Function(
                "degrees".to_string(),
                vec![ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "radians".to_string(),
            ASTASTValue::Function(
                "radians".to_string(),
                vec![ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "sign".to_string(),
            ASTASTValue::Function(
                "sign".to_string(),
                vec![ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "exp".to_string(),
            ASTASTValue::Function(
                "exp".to_string(),
                vec![ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "log".to_string(),
            ASTASTValue::Function(
                "log".to_string(),
                vec![ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "rand".to_string(),
            ASTASTValue::Function("rand".to_string(), vec![], Box::new(ASTASTValue::None)),
        );

        functions.insert(
            "max".to_string(),
            ASTASTValue::Function(
                "max".to_string(),
                vec![ASTASTValue::None, ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "min".to_string(),
            ASTASTValue::Function(
                "min".to_string(),
                vec![ASTASTValue::None, ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "pow".to_string(),
            ASTASTValue::Function(
                "pow".to_string(),
                vec![ASTASTValue::None, ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "mod".to_string(),
            ASTASTValue::Function(
                "mod".to_string(),
                vec![ASTASTValue::None, ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "step".to_string(),
            ASTASTValue::Function(
                "step".to_string(),
                vec![ASTASTValue::None, ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );

        functions.insert(
            "clamp".to_string(),
            ASTASTValue::Function(
                "clamp".to_string(),
                vec![ASTASTValue::None, ASTASTValue::None, ASTASTValue::None],
                Box::new(ASTASTValue::None),
            ),
        );*/

        Self {
            environment: Environment::default(),
            functions,
            break_depth: vec![],
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

    fn define(
        &mut self,
        define_object: &DefineObject,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        //expression.accept(self, ctx)

        // ctx.definitions
        //     .insert(define_object.name.clone(), define_object.clone());

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
            return Ok(ASTASTValue::None);
        }

        match &v {
            ASTASTValue::Int(_, _) => {
                let instr = format!("(local ${} i{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));

                let instr = format!("local.set ${}", name);
                ctx.add_wat(&instr);
            }
            ASTASTValue::Int2(_, _, _) => {
                let instr = format!("(local ${}_x i{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));
                let instr = format!("(local ${}_y i{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));

                let instr = format!("local.set ${}_y", name);
                ctx.add_wat(&instr);
                let instr = format!("local.set ${}_x", name);
                ctx.add_wat(&instr);
            }
            ASTASTValue::Int3(_, _, _, _) => {
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
            ASTASTValue::Int4(_, _, _, _, _) => {
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
            ASTASTValue::Float(_, _) => {
                let instr = format!("(local ${} f{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));

                let instr = format!("local.set ${}", name);
                ctx.add_wat(&instr);
            }
            ASTASTValue::Float2(_, _, _) => {
                let instr = format!("(local ${}_x f{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));
                let instr = format!("(local ${}_y f{})", name, ctx.pr);
                ctx.wat_locals.push_str(&format!("        {}\n", instr));

                let instr = format!("local.set ${}_y", name);
                ctx.add_wat(&instr);
                let instr = format!("local.set ${}_x", name);
                ctx.add_wat(&instr);
            }
            ASTASTValue::Float3(_, _, _, _) => {
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
            ASTASTValue::Float4(_, _, _, _, _) => {
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
            ASTASTValue::Mat2(_, _) | ASTASTValue::Mat3(_, _) | ASTASTValue::Mat4(_, _) => {
                let comps = v.write_definition("local", name, &ctx.pr);
                for c in comps {
                    ctx.wat_locals.push_str(&format!("        {}\n", c));
                }
                let comps = v.write_access("local.set", name);
                for c in comps.iter().rev() {
                    ctx.add_wat(c);
                }
            }
            ASTASTValue::Struct(_, _, _) => {
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
            ASTASTValue::Int(_, _) | ASTASTValue::Float(_, _) => match op {
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
            ASTASTValue::Int2(_, _, _) | ASTASTValue::Float2(_, _, _) => {
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
            ASTASTValue::Int3(_, _, _, _) | ASTASTValue::Float3(_, _, _, _) => {
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
            ASTASTValue::Int4(_, _, _, _, _) | ASTASTValue::Float4(_, _, _, _, _) => {
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
            ASTASTValue::Struct(struct_name, _, _) => {
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
                ASTASTValue::Int(_, _) | ASTASTValue::Float(_, _) => {
                    let instr = format!("{}.get ${}", scope, name);
                    ctx.add_wat(&instr);
                    rc = v.clone();
                }
                ASTASTValue::Int2(_, _, _) | ASTASTValue::Float2(_, _, _) => {
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
                ASTASTValue::Int3(_, _, _, _) | ASTASTValue::Float3(_, _, _, _) => {
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
                ASTASTValue::Int4(_, _, _, _, _) | ASTASTValue::Float4(_, _, _, _, _) => {
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
                ASTASTValue::Mat2(_, _) | ASTASTValue::Mat3(_, _) | ASTASTValue::Mat4(_, _) => {
                    let instr = format!("{}.get", scope);
                    let comps = v.write_access(&instr, &name);

                    for c in comps {
                        ctx.add_wat(&c);
                    }

                    rc = v;
                }
                ASTASTValue::Struct(struct_name, _, _) => {
                    rc = ctx.access_struct(&name, struct_name, field_path, false, loc)?;
                }

                _ => {}
            }
        } else if let Some(ASTASTValue::Function(name, args, body)) = self.functions.get(&name) {
            rc = ASTASTValue::Function(name.clone(), args.clone(), body.clone());
        } else {
            return Err(RPUError::loc(format!("Unknown identifier '{}'", name), loc));
        }

        if !instr.is_empty() {
            ctx.add_wat(&instr);
        }
        */

        if let Some(vv) = self.environment.get(&name) {
            rc = vv;
        } else if let Some(ASTValue::Function(name, args, body)) = self.functions.get(&name) {
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
        swizzle: &[u8],
        _field_path: &[String],
        args: &[Box<Expr>],
        loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let callee = callee.accept(self, ctx)?;
        let mut rc = ASTValue::None;

        // println!(
        //     "func_call: callee: {:?}, swizzle: {:?}, args: {:?}",
        //     callee, swizzle, args
        // );

        if let ASTValue::Function(name, func_args, returns) = callee {
            if func_args.len() != args.len() {
                // return Err(RPUError::loc(
                //     format!(
                //         "Function '{}' expects {} arguments, but {} were provided",
                //         name,
                //         func_args.len(),
                //         args.len()
                //     ),
                //     loc,
                // ));
            }

            if name == "length" {
                let v = args[0].accept(self, ctx)?;

                match v {
                    ASTValue::Float3(x, y, z) => {
                        let x_val = x.accept(self, ctx)?.to_float().unwrap_or_default();
                        let y_val = y.accept(self, ctx)?.to_float().unwrap_or_default();
                        let z_val = z.accept(self, ctx)?.to_float().unwrap_or_default();

                        let r = Vec3::new(x_val, y_val, z_val).magnitude();
                        rc = ASTValue::Float(r);
                    }
                    _ => {}
                }

                /*
                let components = v.components();
                if !(1..=4).contains(&components) {
                    return Err(RPUError::loc(
                        format!("Invalid number of components {}", components),
                        loc,
                    ));
                }
                let func_name = ctx.gen_vec_length(v.components() as u32);
                let instr = format!("(call ${})", func_name);
                ctx.add_wat(&instr);
                rc = ASTASTValue::Float(None, 0.0);*/
            }
        }

        /*
        if let ASTValue::Function(name, func_args, returns) = callee {
            if func_args.len() != args.len() {
                return Err(RPUError::loc(
                    format!(
                        "Function '{}' expects {} arguments, but {} were provided",
                        name,
                        func_args.len(),
                        args.len()
                    ),
                    loc,
                ));
            }

            if name == "length" {
                let v = args[0].accept(self, ctx)?;
                let components = v.components();
                if !(1..=4).contains(&components) {
                    return Err(RPUError::loc(
                        format!("Invalid number of components {}", components),
                        loc,
                    ));
                }
                let func_name = ctx.gen_vec_length(v.components() as u32);
                let instr = format!("(call ${})", func_name);
                ctx.add_wat(&instr);
                rc = ASTASTValue::Float(None, 0.0);
            }
        }*/
        /*
        if let ASTASTValue::Function(name, func_args, returns) = callee {
            if func_args.len() != args.len() {
                return Err(RPUError::loc(
                    format!(
                        "Function '{}' expects {} arguments, but {} were provided",
                        name,
                        func_args.len(),
                        args.len()
                    ),
                    loc,
                ));
            }

            if name == "length" {
                let v = args[0].accept(self, ctx)?;
                let components = v.components();
                if !(1..=4).contains(&components) {
                    return Err(RPUError::loc(
                        format!("Invalid number of components {}", components),
                        loc,
                    ));
                }
                let func_name = ctx.gen_vec_length(v.components() as u32);
                let instr = format!("(call ${})", func_name);
                ctx.add_wat(&instr);
                rc = ASTASTValue::Float(None, 0.0);
            } else if name == "normalize" {
                let v = args[0].accept(self, ctx)?;
                let components = v.components();
                if !(1..=4).contains(&components) {
                    return Err(RPUError::loc(
                        format!("Invalid number of components {}", components),
                        loc,
                    ));
                }
                let func_name = ctx.gen_vec_normalize(v.components() as u32);
                let instr = format!("(call ${})", func_name);
                ctx.add_wat(&instr);
                rc = v;
            } else if name == "rand" {
                if !args.is_empty() {
                    return Err(RPUError::loc("'rand' does not take any arguments", loc));
                }
                let instr = "(call $_rpu_rand)";
                ctx.add_wat(instr);
                ctx.imports_hash.insert("$_rpu_rand".to_string());
                rc = ASTASTValue::Float(None, 0.0);
            } else if name == "sqrt"
                || name == "sin"
                || name == "cos"
                || name == "ceil"
                || name == "floor"
                || name == "fract"
                || name == "abs"
                || name == "tan"
                || name == "atan"
                || name == "degrees"
                || name == "radians"
                || name == "sign"
                || name == "exp"
                || name == "log"
            {
                let v = args[0].accept(self, ctx)?;
                let components = v.components();
                if !(1..=4).contains(&components) {
                    return Err(RPUError::loc(
                        format!("Invalid number of components '{}'", components),
                        loc,
                    ));
                }
                if !v.is_float_based() {
                    return Err(RPUError::loc(
                        format!("'{}' expects a float based parameter", name),
                        loc,
                    ));
                }
                let func_name = ctx.gen_vec_operation(v.components() as u32, &name);
                let instr = format!("(call ${})", func_name);
                ctx.add_wat(&instr);
                rc = v;
            } else if name == "max"
                || name == "min"
                || name == "pow"
                || name == "mod"
                || name == "step"
            {
                let v = args[0].accept(self, ctx)?;

                if func_args.len() != args.len() {
                    return Err(RPUError::loc(
                        format!(
                            "Function '{}' expects {} arguments, but {} were provided",
                            name,
                            func_args.len(),
                            args.len()
                        ),
                        loc,
                    ));
                }

                let components = v.components();
                if !(1..=4).contains(&components) {
                    return Err(RPUError::loc(
                        format!("Invalid number of components '{}'", components),
                        loc,
                    ));
                }
                if !v.is_float_based() {
                    return Err(RPUError::loc(
                        format!("'{}' expects a float based parameter", name),
                        loc,
                    ));
                }

                let b = args[1].accept(self, ctx)?;
                if b.components() != 1 {
                    return Err(RPUError::loc(
                        format!("Invalid second parameter for '{}' (scalars only)", name),
                        loc,
                    ));
                }

                let func_name = ctx.gen_vec_operation_scalar(v.components() as u32, &name);
                let instr = format!("(call ${})", func_name);
                ctx.add_wat(&instr);
                rc = v;
            } else if name == "clamp" {
                let v = args[0].accept(self, ctx)?;

                if func_args.len() != args.len() {
                    return Err(RPUError::loc(
                        format!(
                            "Function '{}' expects {} arguments, but {} were provided",
                            name,
                            func_args.len(),
                            args.len()
                        ),
                        loc,
                    ));
                }

                let components = v.components();
                if !(1..=4).contains(&components) {
                    return Err(RPUError::loc(
                        format!("Invalid number of components '{}'", components),
                        loc,
                    ));
                }
                if !v.is_float_based() {
                    return Err(RPUError::loc(
                        format!("'{}' expects a float based parameter", name),
                        loc,
                    ));
                }

                let b = args[1].accept(self, ctx)?;
                if b.components() != 1 {
                    return Err(RPUError::loc(
                        format!("Invalid second parameter for '{}' (scalars only)", name),
                        loc,
                    ));
                }

                let _ = args[2].accept(self, ctx)?;
                if b.components() != 1 {
                    return Err(RPUError::loc(
                        format!("Invalid second parameter for '{}' (scalars only)", name),
                        loc,
                    ));
                }

                let func_name = ctx.gen_vec_operation_scalar_scalar(v.components() as u32, &name);
                let instr = format!("(call ${})", func_name);
                ctx.add_wat(&instr);
                rc = v;
            } else if name == "smoothstep" {
                let a1 = args[0].accept(self, ctx)?;
                let components = a1.components();
                if !(1..=4).contains(&components) {
                    return Err(RPUError::loc(
                        format!("Invalid number of components {}", components),
                        loc,
                    ));
                }
                let a2 = args[1].accept(self, ctx)?;

                if a1.to_type() != a2.to_type() || !a1.is_float_based() {
                    return Err(RPUError::loc(
                        format!(
                            "'smoothstep' expects the first two arguments to be the same float type, but '{}' and '{}' were provided",
                            a1.to_type(),
                            a2.to_type()
                        ),
                        loc,
                    ));
                }

                let a3 = args[2].accept(self, ctx)?;
                if a3.to_type() != "float" {
                    return Err(RPUError::loc(
                        format!(
                            "'smoothstep' expects the third argument to be of type 'float', but '{}' was provided",
                            a3.to_type()
                        ),
                        loc,
                    ));
                }

                let func_name = ctx.gen_vec_smoothstep(components as u32);

                let instr = format!("(call ${})", func_name);
                ctx.add_wat(&instr);
                rc = a1;
            } else if name == "mix" {
                let a1 = args[0].accept(self, ctx)?;
                let components = a1.components();
                if !(1..=4).contains(&components) {
                    return Err(RPUError::loc(
                        format!("Invalid number of components {}", components),
                        loc,
                    ));
                }
                let a2 = args[1].accept(self, ctx)?;

                if a1.to_type() != a2.to_type() || !a1.is_float_based() {
                    return Err(RPUError::loc(
                        format!(
                            "'mix' expects the first two arguments to be the same float type, but '{}' and '{}' were provided",
                            a1.to_type(),
                            a2.to_type()
                        ),
                        loc,
                    ));
                }

                let a3 = args[2].accept(self, ctx)?;
                if a3.to_type() != "float" {
                    return Err(RPUError::loc(
                        format!(
                            "'mix' expects the third argument to be of type 'float', but '{}' was provided",
                            a3.to_type()
                        ),
                        loc,
                    ));
                }

                let func_name = ctx.gen_vec_mix(components as u32);

                let instr = format!("(call ${})", func_name);
                ctx.add_wat(&instr);
                rc = a1;
            } else if name == "dot" {
                let a1 = args[0].accept(self, ctx)?;
                let components = a1.components();
                if !(2..=4).contains(&components) {
                    return Err(RPUError::loc(
                        format!("Invalid number of components {} for 'dot'", components),
                        loc,
                    ));
                }
                let a2 = args[1].accept(self, ctx)?;

                if a1.to_type() != a2.to_type() || !a1.is_float_based() {
                    return Err(RPUError::loc(
                        format!(
                            "'dot' expects the first two arguments to be the same float type, but '{}' and '{}' were provided",
                            a1.to_type(),
                            a2.to_type()
                        ),
                        loc,
                    ));
                }

                let func_name = ctx.gen_vec_dot_product(components as u32);

                let instr = format!("(call ${})", func_name);
                ctx.add_wat(&instr);
                rc = ASTASTValue::Float(None, 0.0);
            } else if name == "cross" {
                let a1 = args[0].accept(self, ctx)?;
                let components = a1.components();
                if components != 3 {
                    return Err(RPUError::loc(
                        format!("Invalid number of components {} for 'dot'", components),
                        loc,
                    ));
                }
                let a2 = args[1].accept(self, ctx)?;

                if a1.to_type() != a2.to_type() || !a1.is_float_based() {
                    return Err(RPUError::loc(
                        format!(
                            "'dot' expects the first two arguments to be the same float type, but '{}' and '{}' were provided",
                            a1.to_type(),
                            a2.to_type()
                        ),
                        loc,
                    ));
                }

                let func_name = ctx.gen_vec_cross_product();

                let instr = format!("(call ${})", func_name);
                ctx.add_wat(&instr);
                rc = a1;
            } else {
                for index in 0..args.len() {
                    let rc = args[index].accept(self, ctx)?;
                    if rc.to_type() != func_args[index].to_type() {
                        return Err(RPUError::loc(
                            format!(
                                "Function '{}' expects argument {} to be of type '{}', but '{}' was provided",
                                name,
                                index,
                                func_args[index].to_type(),
                                rc.to_type()
                            ),
                            loc,
                        ));
                    }
                }

                let instr = format!("(call ${})", name);
                ctx.add_wat(&instr);
                rc = *returns;
            }

            if !swizzle.is_empty() {
                ctx.swizzle_it(&rc, swizzle, loc)?;
                rc = ctx.create_value_from_swizzle(&rc, swizzle.len());
            }
        }*/

        Ok(rc)
    }

    fn struct_declaration(
        &mut self,
        name: &str,
        fields: &[(String, ASTValue)],
        _loc: &Location,
        ctx: &mut Context,
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
                    ASTASTValue::Function(name.to_string(), args.to_vec(), Box::new(returns.clone())),
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
                        ASTASTValue::Int(name, _) => {
                            params += &format!(
                                "(param ${} i{})",
                                name.clone().unwrap(),
                                ctx.precision.describe()
                            );
                        }
                        ASTASTValue::Int2(name, _, _) => {
                            params += &format!(
                                "(param ${}_x i{}) (param ${}_y i{})",
                                name.clone().unwrap(),
                                ctx.precision.describe(),
                                name.clone().unwrap(),
                                ctx.precision.describe()
                            );
                        }
                        ASTASTValue::Int3(name, _, _, _) => {
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
                        ASTASTValue::Int4(name, _, _, _, _) => {
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
                        ASTASTValue::Float(name, _) => {
                            params += &format!(
                                "(param ${} f{})",
                                name.clone().unwrap(),
                                ctx.precision.describe()
                            );
                        }
                        ASTASTValue::Float2(name, _, _) => {
                            params += &format!(
                                "(param ${}_x f{}) (param ${}_y f{})",
                                name.clone().unwrap(),
                                ctx.precision.describe(),
                                name.clone().unwrap(),
                                ctx.precision.describe()
                            );
                        }
                        ASTASTValue::Float3(name, _, _, _) => {
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
                        ASTASTValue::Float4(name, _, _, _, _) => {
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
                        ASTASTValue::Struct(_, param_name, _) => {
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

                let mut last_value = ASTASTValue::None;
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
