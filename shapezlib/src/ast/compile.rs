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
            "tan".to_string(),
            ASTFunction {
                name: "tan".to_string(),
                arguments: 1,
                op: NodeOp::Tan,
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
                op: NodeOp::Ceil,
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

        Self {
            environment: Environment::default(),
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
        ctx.add_custom_target();
        if let Some(block) = &objectd.block {
            block.accept(self, ctx)?;
        }

        if let Some(code) = ctx.take_last_custom_target() {
            match objectd.name.as_str() {
                "Rect" => {
                    ctx.emit(NodeOp::ShapeRect(code));
                }
                "Disc" => {
                    ctx.emit(NodeOp::ShapeDisc(code));
                }
                _ => {}
            }
        }

        Ok(ASTValue::None)
    }

    // Create a segment
    fn segment(
        &mut self,
        objectd: &SegmentD,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        // Parameters

        let mut depth = vec![];
        if let Some(depth_ast) = objectd.params.get("depth") {
            ctx.add_custom_target();
            _ = depth_ast.accept(self, ctx)?;
            if let Some(code) = ctx.take_last_custom_target() {
                depth = code;
            }
        }

        // Body
        ctx.add_custom_target();
        if let Some(block) = &objectd.block {
            _ = block.accept(self, ctx)?;
        }
        if let Some(code) = ctx.take_last_custom_target() {
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
                _ => {}
            }
        }

        Ok(ASTValue::None)
    }

    // Create a pattern
    fn pattern(
        &mut self,
        objectd: &PatternD,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        let mut codes = FxHashMap::default();

        // Compile all blocks
        for (name, stmts) in &objectd.blocks {
            ctx.add_custom_target();
            _ = stmts.accept(self, ctx)?;
            if let Some(code) = ctx.take_last_custom_target() {
                codes.insert(name.clone(), code);
            }
        }

        match objectd.name.as_str() {
            "Modulo" => {
                let even = codes.get("even".into()).cloned();
                let odd = codes.get("odd".into()).cloned();
                ctx.emit(NodeOp::PatternModulo(even, odd));
            }
            "Bricks" => {
                let brick = codes.get("brick".into()).cloned();
                let cement = codes.get("cement".into()).cloned();
                ctx.emit(NodeOp::PatternBricks(brick, cement));
            }
            _ => {}
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
                    "transmission" => material_ops.push(NodeOp::MaterialTransmission),
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

        ctx.materials.insert(objectd.name.clone(), material_ops);

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
        _static_type: &ASTValue,
        expression: &Expr,
        _loc: &Location,
        ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        _ = expression.accept(self, ctx)?;

        if let Some(index) = ctx.variables.get(name) {
            ctx.emit(NodeOp::Store(*index as usize));
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
        _ = expression.accept(self, ctx)?; // RHS is now on stack

        if let Some(&index) = ctx.variables.get(&name) {
            let index = index as usize;
            if swizzle.is_empty() {
                // Non-swizzled assignment
                match op {
                    AssignmentOperator::Assign => {
                        ctx.emit(NodeOp::Store(index));
                    }
                    AssignmentOperator::AddAssign => {
                        ctx.emit(NodeOp::Load(index));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Add);
                        ctx.emit(NodeOp::Store(index));
                    }
                    AssignmentOperator::SubtractAssign => {
                        ctx.emit(NodeOp::Load(index));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Sub);
                        ctx.emit(NodeOp::Store(index));
                    }
                    AssignmentOperator::MultiplyAssign => {
                        ctx.emit(NodeOp::Load(index));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Mul);
                        ctx.emit(NodeOp::Store(index));
                    }
                    AssignmentOperator::DivideAssign => {
                        ctx.emit(NodeOp::Load(index));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Div);
                        ctx.emit(NodeOp::Store(index));
                    }
                }
            } else {
                // Swizzled assignment
                match op {
                    AssignmentOperator::Assign => {
                        ctx.emit(NodeOp::Load(index));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::SetComponents(swizzle.into()));
                        ctx.emit(NodeOp::Store(index));
                    }
                    AssignmentOperator::AddAssign => {
                        ctx.emit(NodeOp::Load(index));
                        ctx.emit(NodeOp::Dup);
                        ctx.emit(NodeOp::GetComponents(swizzle.to_vec()));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Add);
                        ctx.emit(NodeOp::SetComponents(swizzle.to_vec()));
                        ctx.emit(NodeOp::Store(index));
                    }
                    AssignmentOperator::SubtractAssign => {
                        ctx.emit(NodeOp::Load(index));
                        ctx.emit(NodeOp::Dup);
                        ctx.emit(NodeOp::GetComponents(swizzle.into()));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Sub);
                        ctx.emit(NodeOp::SetComponents(swizzle.into()));
                        ctx.emit(NodeOp::Store(index));
                    }
                    AssignmentOperator::MultiplyAssign => {
                        ctx.emit(NodeOp::Load(index));
                        ctx.emit(NodeOp::Dup);
                        ctx.emit(NodeOp::GetComponents(swizzle.into()));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Mul);
                        ctx.emit(NodeOp::SetComponents(swizzle.into()));
                        ctx.emit(NodeOp::Store(index));
                    }
                    AssignmentOperator::DivideAssign => {
                        ctx.emit(NodeOp::Load(index));
                        ctx.emit(NodeOp::Dup);
                        ctx.emit(NodeOp::GetComponents(swizzle.into()));
                        ctx.emit(NodeOp::Swap);
                        ctx.emit(NodeOp::Div);
                        ctx.emit(NodeOp::SetComponents(swizzle.into()));
                        ctx.emit(NodeOp::Store(index));
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
        if name == "local" {
            ctx.emit(NodeOp::Local);
        } else if name == "world" {
            ctx.emit(NodeOp::World);
        } else if name == "u" {
            ctx.emit(NodeOp::U);
        } else if name == "v" {
            ctx.emit(NodeOp::V);
        } else if name == "d" {
            ctx.emit(NodeOp::D);
        } else if name == "hash" {
            ctx.emit(NodeOp::Hash);
        } else {
            if let Some(index) = ctx.variables.get(&name) {
                ctx.emit(NodeOp::Load(*index as usize));
                if !swizzle.is_empty() {
                    ctx.emit(NodeOp::GetComponents(swizzle.to_vec()));
                }
            }
        }

        // else if let Some(vv) = self.environment.get(&name) {
        //     rc = vv;
        // } else if let Some(ASTValue::Function(name, args, body)) = self.ast_functions.get(&name) {
        //     rc = ASTValue::Function(name.clone(), args.clone(), body.clone());
        // }

        Ok(ASTValue::None)
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
        _name: &str,
        _args: &[ASTValue],
        _body: &[Box<Stmt>],
        _returns: &ASTValue,
        _export: &bool,
        _loc: &Location,
        _ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        // Keeping this around for possible later function support

        Ok(ASTValue::None)
    }

    fn return_stmt(
        &mut self,
        _expr: &Expr,
        _loc: &Location,
        _ctx: &mut Context,
    ) -> Result<ASTValue, RuntimeError> {
        // let rc = expr.accept(self, ctx)?;

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
