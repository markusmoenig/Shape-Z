use crate::prelude::*;
use crate::zero_expr_float;
use std::path::PathBuf;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    current_line: usize,
    path: PathBuf,
    verifier: IdVerifier,

    variable_counter: u32,
    variable_map: FxHashMap<String, u32>,

    materials: IndexMap<String, BSDFMaterial>,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
            current_line: 0,
            path: PathBuf::new(),
            verifier: IdVerifier::default(),

            variable_counter: 0,
            variable_map: FxHashMap::default(),

            materials: IndexMap::default(),
        }
    }

    /// Compile the main source module.
    pub fn compile(&mut self, path: PathBuf) -> Result<Module, ParseError> {
        if let Ok(source) = std::fs::read_to_string(path.clone()) {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                self.compile_module(stem.to_string(), source, path)
            } else {
                Err(ParseError::new("Could not read file", 0, &path))
            }
        } else {
            Err(ParseError::new("Could not read file", 0, &path))
        }
    }

    /// Compile a module with the given name, source code, and path.
    pub fn compile_module(
        &mut self,
        name: String,
        source: String,
        path: PathBuf,
    ) -> Result<Module, ParseError> {
        // Extract all tokens from the scanner
        let mut scanner = Scanner::new(source.clone());

        let mut tokens = vec![];
        loop {
            let token = scanner.scan_token();
            if token.kind == TokenType::Eof {
                //tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        self.tokens = tokens;
        self.path = path.clone();

        // Collect statements
        let mut statements = vec![];

        while !self.is_at_end() {
            let stmt = self.declaration()?;
            statements.push(Box::new(stmt));
        }

        let module = Module::new(
            name,
            source,
            self.path.clone(),
            statements,
            self.variable_map.clone(),
        );

        Ok(module)
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(vec![TokenType::Material]) {
            return self.material_declaration();
        }
        if self.match_token(vec![TokenType::Voxel]) {
            return self.voxel_declaration();
        }
        if self.match_token(vec![TokenType::Shape]) {
            return self.shape_declaration();
        }
        if self.match_token(vec![TokenType::Segment]) {
            return self.segment_declaration();
        }
        if self.match_token(vec![TokenType::Place]) {
            return self.place_declaration();
        }
        if self.match_token(vec![TokenType::Let]) {
            return self.var_declaration();
        }

        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let line = self.current_line;
        let var_name = self
            .consume(TokenType::Identifier, "Expect variable name", line)?
            .lexeme;
        _ = self.verifier.define_var(&var_name, false)?;
        self.variable_map
            .insert(var_name.clone(), self.variable_counter);
        self.variable_counter += 1;

        let mut initializer = None;
        if self.match_token(vec![TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        let init = if let Some(i) = initializer {
            Box::new(i)
        } else {
            return Err(ParseError::new(
                "Variable declaration cannot be empty",
                line,
                &self.path,
            ));
        };

        /*
        if self.check(TokenType::Comma) {
            self.consume(
                TokenType::Comma,
                &format!(
                    "Expect ',' after variable declaration, found '{}'",
                    self.lexeme(),
                ),
                line,
            )?;
            self.open_var_declaration = Some(static_type.clone());
        } else {
            self.open_var_declaration = None;
            if !self.inside_for_initializer {
                self.consume(
                    TokenType::Semicolon,
                    &format!(
                        "Expect ';' after variable declaration, found '{}'",
                        self.lexeme(),
                    ),
                    line,
                )?;
            }
        }*/

        Ok(Stmt::VarDeclaration(
            var_name,
            ASTValue::None,
            init,
            self.create_loc(line),
        ))
    }

    /// Material declaration
    fn material_declaration(&mut self) -> Result<Stmt, ParseError> {
        let line = self.current_line;
        self.consume(
            TokenType::Identifier,
            "Expected identifier after 'material''",
            self.current_line,
        )?;

        let id = self.previous().unwrap().lexeme.clone();
        let mut params = FxHashMap::default();

        while self.match_token(vec![TokenType::Identifier]) {
            let id = self.previous().unwrap().lexeme.clone();

            self.consume(
                TokenType::Equal,
                "Expected '=' after voxel identifier",
                self.current_line,
            )?;

            let value = self.expression()?;

            params.insert(id, Box::new(value));
        }

        self.consume(
            TokenType::LeftBrace,
            "Expected '{' after material header",
            self.current_line,
        )?;

        // -- Read Body Statements

        let mut blocks = FxHashMap::default();

        while self.match_token(vec![TokenType::Identifier]) {
            let id = self.previous().unwrap().lexeme.clone();

            self.consume(
                TokenType::LeftBrace,
                "Expected '{' after pattern identifier",
                self.current_line,
            )?;

            let block = self.block()?;

            blocks.insert(id, Box::new(block));
        }

        self.consume(
            TokenType::RightBrace,
            "Expected '}' after pattern block",
            self.current_line,
        )?;

        self.materials.insert(id.clone(), BSDFMaterial::default());

        Ok(Stmt::Material(
            MaterialD::new(id, params, blocks),
            self.create_loc(line),
        ))
    }

    /// Voxel declaration
    fn voxel_declaration(&mut self) -> Result<Stmt, ParseError> {
        let line = self.current_line;
        self.consume(
            TokenType::Identifier,
            "Expected identifier after 'voxel''",
            self.current_line,
        )?;

        let id = self.previous().unwrap().lexeme.clone();
        let mut params = FxHashMap::default();

        while self.match_token(vec![TokenType::Identifier]) {
            let id = self.previous().unwrap().lexeme.clone();

            self.consume(
                TokenType::Equal,
                "Expected '=' after voxel identifier",
                self.current_line,
            )?;

            let value = self.expression()?;

            params.insert(id, Box::new(value));
        }

        self.consume(
            TokenType::LeftBrace,
            "Expected '{' after voxel header",
            self.current_line,
        )?;

        let block = self.block()?;

        Ok(Stmt::Voxel(
            VoxelD::new(id, params, Box::new(block)),
            self.create_loc(line),
        ))
    }

    /// Shape declaration
    fn shape_declaration(&mut self) -> Result<Stmt, ParseError> {
        let line = self.current_line;
        self.consume(
            TokenType::Identifier,
            "Expected identifier after 'shape''",
            self.current_line,
        )?;

        let valid_shape_ids = vec!["Rect", "Disc"];

        let id = self.previous().unwrap().lexeme.clone();

        if !valid_shape_ids.contains(&id.as_str()) {
            return Err(ParseError::new(
                format!("Invalid shape id: {:?}", id),
                line,
                &self.path,
            ));
        }

        let mut params = FxHashMap::default();

        while self.match_token(vec![TokenType::Identifier]) {
            let id = self.previous().unwrap().lexeme.clone();

            self.consume(
                TokenType::Equal,
                "Expected '=' after voxel identifier",
                self.current_line,
            )?;

            let value = self.expression()?;
            params.insert(id, Box::new(value));

            if self.tokens[self.current].kind == TokenType::Comma {
                self.advance();
            }
        }

        self.consume(
            TokenType::LeftBrace,
            "Expected '{' after shape header",
            self.current_line,
        )?;

        let block = self.block()?;

        Ok(Stmt::Shape(
            ShapeD::new(id, params, Box::new(block)),
            self.create_loc(line),
        ))
    }

    /// Segment declaration
    fn segment_declaration(&mut self) -> Result<Stmt, ParseError> {
        let line = self.current_line;
        self.consume(
            TokenType::Identifier,
            "Expected identifier after 'segment''",
            self.current_line,
        )?;

        // let valid_shape_ids = vec!["Rect"];

        let id = self.previous().unwrap().lexeme.clone();

        // if !valid_shape_ids.contains(&id.as_str()) {
        //     return Err(ParseError::new(
        //         format!("Invalid shape id: {:?}", id),
        //         line,
        //         &self.path,
        //     ));
        // }

        let mut params = FxHashMap::default();

        while self.match_token(vec![TokenType::Identifier]) {
            let id = self.previous().unwrap().lexeme.clone();

            self.consume(
                TokenType::Equal,
                "Expected '=' after voxel identifier",
                self.current_line,
            )?;

            let value = self.expression()?;

            params.insert(id, Box::new(value));
        }

        self.consume(
            TokenType::LeftBrace,
            "Expected '{' after segment header",
            self.current_line,
        )?;

        let block = self.block()?;

        Ok(Stmt::Segment(
            SegmentD::new(id, params, Box::new(block)),
            self.create_loc(line),
        ))
    }

    /// Place declaration
    fn place_declaration(&mut self) -> Result<Stmt, ParseError> {
        let line = self.current_line;
        self.consume(
            TokenType::Identifier,
            "Expected identifier after 'place''",
            self.current_line,
        )?;

        let id = self.previous().unwrap().lexeme.clone();
        let mut params = FxHashMap::default();

        while self.match_token(vec![TokenType::Identifier]) {
            let id = self.previous().unwrap().lexeme.clone();

            self.consume(
                TokenType::Equal,
                "Expected '=' after define identifier",
                self.current_line,
            )?;

            let value = self.expression()?;

            params.insert(id, Box::new(value));
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after place statement",
            line,
        )?;

        Ok(Stmt::Place(id, params, self.create_loc(line)))
    }

    fn block(&mut self) -> Result<Stmt, ParseError> {
        let mut statements = vec![];

        self.verifier.begin_scope();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => {
                    statements.push(Box::new(stmt));
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }

        self.verifier.end_scope();

        let line = self.current_line;

        self.consume(TokenType::RightBrace, "Expect '}}' after block", line)?;

        Ok(Stmt::Block(statements, self.create_loc(line)))
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(vec![TokenType::If]) {
            self.if_statement()
        } else if self.match_token(vec![TokenType::Pattern]) {
            self.pattern_statement()
        }
        /*
        else if self.match_token(vec![TokenType::Print]) {
            self.print_statement()
        } else if self.match_token(vec![TokenType::While]) {
            self.while_statement()
        } else if self.match_token(vec![TokenType::For]) {
            self.for_statement()
        } else if self.match_token(vec![TokenType::Return]) {
            self.return_statement()
        } else if self.match_token(vec![TokenType::Break]) {
            self.break_statement()
        }*/
        else if self.match_token(vec![TokenType::LeftBrace]) {
            self.block()
        } else {
            self.expression_statement()
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let value: Expr = self.expression()?;
        let line = self.current_line;
        self.consume(TokenType::Semicolon, "Expect ';' after expression", line)?;
        Ok(Stmt::Expression(Box::new(value), self.create_loc(line)))
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        let line = self.current_line;
        let condition = self.expression()?;
        let then_branch = self.statement()?;
        let else_branch = if self.match_token(vec![TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(
            Box::new(condition),
            Box::new(then_branch),
            else_branch,
            self.create_loc(line),
        ))
    }

    fn pattern_statement(&mut self) -> Result<Stmt, ParseError> {
        let line = self.current_line;
        self.consume(
            TokenType::Identifier,
            "Expected identifier after 'pattern''",
            self.current_line,
        )?;
        let id = self.previous().unwrap().lexeme.clone();

        let mut params = FxHashMap::default();

        while self.match_token(vec![TokenType::Identifier]) {
            let id = self.previous().unwrap().lexeme.clone();

            self.consume(
                TokenType::Equal,
                "Expected '=' after voxel identifier",
                self.current_line,
            )?;

            let value = self.expression()?;

            params.insert(id, Box::new(value));
        }

        self.consume(
            TokenType::LeftBrace,
            "Expected '{' after pattern header",
            self.current_line,
        )?;

        // -- Read Body Statements

        let mut blocks = FxHashMap::default();

        while self.match_token(vec![TokenType::Identifier]) {
            let id = self.previous().unwrap().lexeme.clone();

            self.consume(
                TokenType::LeftBrace,
                "Expected '{' after pattern identifier",
                self.current_line,
            )?;

            let block = self.block()?;

            blocks.insert(id, Box::new(block));
        }

        self.consume(
            TokenType::RightBrace,
            "Expected '}' after pattern block",
            self.current_line,
        )?;

        Ok(Stmt::Pattern(
            PatternD::new(id, params, blocks),
            self.create_loc(line),
        ))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.check(TokenType::Plus)
            && self.match_token(vec![TokenType::Plus])
            && self.match_token(vec![TokenType::Equal])
        {
            let equals = self.previous().unwrap();
            let value = self.assignment()?;

            if let Expr::Variable(name, swizzle, field_path, _loc) = expr {
                return Ok(Expr::VariableAssignment(
                    name,
                    AssignmentOperator::AddAssign,
                    swizzle.clone(),
                    field_path.clone(),
                    Box::new(value),
                    self.create_loc(equals.line),
                ));
            }

            return Err(ParseError::new(
                format!("Invalid assignment target: '{:?}'", equals.lexeme),
                equals.line,
                &self.path,
            ));
        } else if self.check(TokenType::Minus)
            && self.match_token(vec![TokenType::Minus])
            && self.match_token(vec![TokenType::Equal])
        {
            let equals = self.previous().unwrap();
            let value = self.assignment()?;

            if let Expr::Variable(name, swizzle, field_path, _loc) = expr {
                return Ok(Expr::VariableAssignment(
                    name,
                    AssignmentOperator::SubtractAssign,
                    swizzle.clone(),
                    field_path.clone(),
                    Box::new(value),
                    self.create_loc(equals.line),
                ));
            }

            return Err(ParseError::new(
                format!("Invalid assignment target: '{:?}'", equals.lexeme),
                equals.line,
                &self.path,
            ));
        } else if self.check(TokenType::Star)
            && self.match_token(vec![TokenType::Star])
            && self.match_token(vec![TokenType::Equal])
        {
            let equals = self.previous().unwrap();
            let value = self.assignment()?;

            if let Expr::Variable(name, swizzle, field_path, _loc) = expr {
                return Ok(Expr::VariableAssignment(
                    name,
                    AssignmentOperator::MultiplyAssign,
                    swizzle.clone(),
                    field_path.clone(),
                    Box::new(value),
                    self.create_loc(equals.line),
                ));
            }

            return Err(ParseError::new(
                format!("Invalid assignment target: '{:?}'", equals.lexeme),
                equals.line,
                &self.path,
            ));
        } else if self.check(TokenType::Slash)
            && self.match_token(vec![TokenType::Slash])
            && self.match_token(vec![TokenType::Equal])
        {
            let equals = self.previous().unwrap();
            let value = self.assignment()?;

            if let Expr::Variable(name, swizzle, field_path, _loc) = expr {
                return Ok(Expr::VariableAssignment(
                    name,
                    AssignmentOperator::DivideAssign,
                    swizzle.clone(),
                    field_path.clone(),
                    Box::new(value),
                    self.create_loc(equals.line),
                ));
            }

            return Err(ParseError::new(
                format!("Invalid assignment target: '{:?}'", equals.lexeme),
                equals.line,
                &self.path,
            ));
        } else if self.match_token(vec![TokenType::Equal]) {
            let equals = self.previous().unwrap();
            let value = self.assignment()?;

            if let Expr::Variable(name, swizzle, field_path, _loc) = expr {
                return Ok(Expr::VariableAssignment(
                    name,
                    AssignmentOperator::Assign,
                    swizzle.clone(),
                    field_path.clone(),
                    Box::new(value),
                    self.create_loc(equals.line),
                ));
            }

            return Err(ParseError::new(
                format!("Invalid assignment target: '{:?}'", equals.lexeme),
                equals.line,
                &self.path,
            ));
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.match_token(vec![TokenType::Or]) {
            let operator = self.previous().unwrap();
            let right = self.and()?;
            expr = Expr::Logical(
                Box::new(expr),
                Self::operator_to_logical(operator.kind),
                Box::new(right),
                self.create_loc(operator.line),
            );
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.ternary()?;

        while self.match_token(vec![TokenType::And]) {
            let operator = self.previous().unwrap();
            let right = self.equality()?;
            expr = Expr::Logical(
                Box::new(expr),
                Self::operator_to_logical(operator.kind),
                Box::new(right),
                self.create_loc(operator.line),
            );
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        let line = self.current_line;

        while self.match_token(vec![TokenType::TernaryOperator]) {
            let then_branch = self.expression()?;

            self.consume(
                TokenType::Colon,
                "Expect ':' after condition for ternary",
                line,
            )?;

            let else_branch = self.expression()?;

            expr = Expr::Ternary(
                Box::new(expr),
                Box::new(then_branch),
                Box::new(else_branch),
                self.create_loc(line),
            );
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().unwrap();
            let right = self.comparison()?;
            expr = Expr::Equality(
                Box::new(expr),
                Self::operator_to_equality(operator.kind),
                Box::new(right),
                self.create_loc(operator.line),
            );
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().unwrap();
            let right = self.term()?;
            expr = Expr::Comparison(
                Box::new(expr),
                Self::operator_to_comparison(operator.kind),
                Box::new(right),
                self.create_loc(operator.line),
            );
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        if (self.check(TokenType::Minus) || self.check(TokenType::Plus))
            && !self.check_next(TokenType::Equal)
        {
            while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
                let operator = self.previous().unwrap();
                let right = self.factor()?;
                expr = Expr::Binary(
                    Box::new(expr),
                    Self::operator_to_binary(operator.kind),
                    Box::new(right),
                    self.create_loc(operator.line),
                );
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        if (self.check(TokenType::Slash) || self.check(TokenType::Star))
            && !self.check_next(TokenType::Equal)
        {
            while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
                let operator = self.previous().unwrap();
                let right = self.unary()?;
                expr = Expr::Binary(
                    Box::new(expr),
                    Self::operator_to_binary(operator.kind),
                    Box::new(right),
                    self.create_loc(operator.line),
                );
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().unwrap();
            let right = self.unary()?;
            return Ok(Expr::Unary(
                Self::operator_to_unary(operator.kind),
                Box::new(right),
                self.create_loc(operator.line),
            ));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments = vec![];
        let line = self.current_line;

        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(ParseError::new(
                        "Cannot have more than 255 arguments",
                        line,
                        &self.path,
                    ));
                }

                arguments.push(Box::new(self.expression()?));

                if !self.match_token(vec![TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(
            TokenType::RightParen,
            "Expect ')' after function arguments",
            line,
        )?;

        let swizzle = vec![];
        let field_path = vec![];
        /*
        if self.check(TokenType::Dot) {
            if self.is_swizzle_valid_at_current() {
                swizzle = self.get_swizzle_at_current();
            } else {
                field_path = self.get_field_path_at_current();
            }
        }*/
        Ok(Expr::FunctionCall(
            Box::new(callee),
            swizzle,
            field_path,
            arguments,
            self.create_loc(paren.line),
        ))
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.peek();
        match token.kind {
            TokenType::False => {
                self.advance();
                Ok(Expr::Value(
                    ASTValue::Boolean(false),
                    vec![],
                    vec![],
                    self.create_loc(token.line),
                ))
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Value(
                    ASTValue::Boolean(true),
                    vec![],
                    vec![],
                    self.create_loc(token.line),
                ))
            }
            TokenType::Void => {
                self.advance();
                Ok(Expr::Value(
                    ASTValue::None,
                    vec![],
                    vec![],
                    self.create_loc(token.line),
                ))
            }
            TokenType::Semicolon => Ok(Expr::Value(
                ASTValue::None,
                vec![],
                vec![],
                self.create_loc(token.line),
            )),
            TokenType::IntegerNumber => {
                self.advance();
                if let Ok(number) = token.lexeme.parse::<i32>() {
                    // if self.force_floats {
                    Ok(Expr::Value(
                        ASTValue::Float(number as f32),
                        vec![],
                        vec![],
                        self.create_loc(token.line),
                    ))
                    // } else {
                    //     Ok(Expr::Value(
                    //         ASTValue::Int(None, number),
                    //         vec![],
                    //         vec![],
                    //         self.create_loc(token.line),
                    //     ))
                    // }
                } else {
                    Err(ParseError::new(
                        "Invalid integer number",
                        token.line,
                        &self.path,
                    ))
                }
            }
            /*
            TokenType::Int2 => {
                self.advance();
                if self.match_token(vec![TokenType::LeftParen]) {
                    let comps = self.read_vec_components(2, token.line, false)?;
                    let swizzle: Vec<u8> = self.get_swizzle_at_current();

                    Ok(Expr::Value(
                        ASTValue::Int2(
                            Some(format!("{}", comps.len())),
                            if !comps.is_empty() {
                                Box::new(comps[0].clone())
                            } else {
                                zero_expr_int!()
                            },
                            if comps.len() > 1 {
                                Box::new(comps[1].clone())
                            } else {
                                zero_expr_int!()
                            },
                        ),
                        swizzle,
                        vec![],
                        self.create_loc(token.line),
                    ))
                } else {
                    Err(RPUError::new("Expected '(' after ivec2", token.line))
                }
            }
            TokenType::Int3 => {
                self.advance();
                if self.match_token(vec![TokenType::LeftParen]) {
                    let comps = self.read_vec_components(3, token.line, false)?;
                    let swizzle: Vec<u8> = self.get_swizzle_at_current();

                    Ok(Expr::Value(
                        ASTValue::Int3(
                            Some(format!("{}", comps.len())),
                            if !comps.is_empty() {
                                Box::new(comps[0].clone())
                            } else {
                                zero_expr_int!()
                            },
                            if comps.len() > 1 {
                                Box::new(comps[1].clone())
                            } else {
                                zero_expr_int!()
                            },
                            if comps.len() > 2 {
                                Box::new(comps[2].clone())
                            } else {
                                zero_expr_int!()
                            },
                        ),
                        swizzle,
                        vec![],
                        self.create_loc(token.line),
                    ))
                } else {
                    Err(RPUError::new("Expected '(' after ivec3", token.line))
                }
            }
            TokenType::Int4 => {
                self.advance();
                if self.match_token(vec![TokenType::LeftParen]) {
                    let comps = self.read_vec_components(4, token.line, false)?;
                    let swizzle: Vec<u8> = self.get_swizzle_at_current();

                    Ok(Expr::Value(
                        ASTValue::Int4(
                            Some(format!("{}", comps.len())),
                            if !comps.is_empty() {
                                Box::new(comps[0].clone())
                            } else {
                                zero_expr_int!()
                            },
                            if comps.len() > 1 {
                                Box::new(comps[1].clone())
                            } else {
                                zero_expr_int!()
                            },
                            if comps.len() > 2 {
                                Box::new(comps[2].clone())
                            } else {
                                zero_expr_int!()
                            },
                            if comps.len() > 3 {
                                Box::new(comps[3].clone())
                            } else {
                                zero_expr_int!()
                            },
                        ),
                        swizzle,
                        vec![],
                        self.create_loc(token.line),
                    ))
                } else {
                    Err(RPUError::new("Expected '(' after ivec4", token.line))
                }
            }*/
            TokenType::FloatNumber => {
                self.advance();
                if let Ok(number) = token.lexeme.parse::<f32>() {
                    Ok(Expr::Value(
                        ASTValue::Float(number),
                        vec![],
                        vec![],
                        self.create_loc(token.line),
                    ))
                } else {
                    Err(ParseError::new(
                        "Invalid float number",
                        token.line,
                        &self.path,
                    ))
                }
            }
            TokenType::Float2 => {
                self.advance();
                if self.match_token(vec![TokenType::LeftParen]) {
                    let comps = self.read_vec_components(2, token.line)?;
                    let swizzle: Vec<u8> = self.get_swizzle_at_current();

                    Ok(Expr::Value(
                        ASTValue::Float2(
                            if !comps.is_empty() {
                                Box::new(comps[0].clone())
                            } else {
                                zero_expr_float!()
                            },
                            if comps.len() > 1 {
                                Box::new(comps[1].clone())
                            } else {
                                zero_expr_float!()
                            },
                        ),
                        swizzle,
                        vec![],
                        self.create_loc(token.line),
                    ))
                } else {
                    Err(ParseError::new(
                        "Expected '(' after vec2",
                        token.line,
                        &self.path,
                    ))
                }
            }
            TokenType::Float3 => {
                self.advance();
                if self.match_token(vec![TokenType::LeftParen]) {
                    let comps = self.read_vec_components(3, token.line)?;
                    let swizzle: Vec<u8> = self.get_swizzle_at_current();

                    Ok(Expr::Value(
                        ASTValue::Float3(
                            if !comps.is_empty() {
                                Box::new(comps[0].clone())
                            } else {
                                zero_expr_float!()
                            },
                            if comps.len() > 1 {
                                Box::new(comps[1].clone())
                            } else {
                                zero_expr_float!()
                            },
                            if comps.len() > 2 {
                                Box::new(comps[2].clone())
                            } else {
                                zero_expr_float!()
                            },
                        ),
                        swizzle,
                        vec![],
                        self.create_loc(token.line),
                    ))
                } else {
                    Err(ParseError::new(
                        "Expected '(' after vec3",
                        token.line,
                        &self.path,
                    ))
                }
            }
            /*
            TokenType::Float4 => {
                self.advance();
                if self.match_token(vec![TokenType::LeftParen]) {
                    let comps = self.read_vec_components(4, token.line, true)?;
                    let swizzle: Vec<u8> = self.get_swizzle_at_current();

                    Ok(Expr::Value(
                        ASTValue::Float4(
                            Some(format!("{}", comps.len())),
                            if !comps.is_empty() {
                                Box::new(comps[0].clone())
                            } else {
                                zero_expr_float!()
                            },
                            if comps.len() > 1 {
                                Box::new(comps[1].clone())
                            } else {
                                zero_expr_float!()
                            },
                            if comps.len() > 2 {
                                Box::new(comps[2].clone())
                            } else {
                                zero_expr_float!()
                            },
                            if comps.len() > 3 {
                                Box::new(comps[3].clone())
                            } else {
                                zero_expr_float!()
                            },
                        ),
                        swizzle,
                        vec![],
                        self.create_loc(token.line),
                    ))
                } else {
                    Err(RPUError::new("Expected '(' after vec4", token.line))
                }
            }
            TokenType::Mat2 => {
                self.advance();
                if self.match_token(vec![TokenType::LeftParen]) {
                    let comps = self.read_vec_components(4, token.line, true)?;
                    //let swizzle: Vec<u8> = self.get_swizzle_at_current();

                    if comps.len() != 4 {
                        return Err(RPUError::new("Expected 4 components for mat2", token.line));
                    }

                    let mut c = vec![];
                    for comp in &comps {
                        c.push(Box::new(comp.clone()));
                    }

                    Ok(Expr::Value(
                        ASTValue::Mat2(Some(format!("{}", comps.len())), c),
                        vec![],
                        vec![],
                        self.create_loc(token.line),
                    ))
                } else {
                    Err(RPUError::new("Expected '(' after mat2", token.line))
                }
            }
            TokenType::Mat3 => {
                self.advance();
                if self.match_token(vec![TokenType::LeftParen]) {
                    let comps = self.read_vec_components(9, token.line, true)?;
                    //let swizzle: Vec<u8> = self.get_swizzle_at_current();

                    if comps.len() != 9 && comps.len() != 3 {
                        return Err(RPUError::new(
                            "Expected 9 or 3 components for mat3",
                            token.line,
                        ));
                    }

                    let mut c = vec![];
                    for comp in &comps {
                        c.push(Box::new(comp.clone()));
                    }

                    Ok(Expr::Value(
                        ASTValue::Mat3(Some(format!("{}", comps.len())), c),
                        vec![],
                        vec![],
                        self.create_loc(token.line),
                    ))
                } else {
                    Err(RPUError::new("Expected '(' after mat3", token.line))
                }
            }
            TokenType::Mat4 => {
                self.advance();
                if self.match_token(vec![TokenType::LeftParen]) {
                    let comps = self.read_vec_components(16, token.line, true)?;
                    //let swizzle: Vec<u8> = self.get_swizzle_at_current();

                    if comps.len() != 16 {
                        return Err(RPUError::new("Expected 16 components for mat4", token.line));
                    }

                    let mut c = vec![];
                    for comp in &comps {
                        c.push(Box::new(comp.clone()));
                    }

                    Ok(Expr::Value(
                        ASTValue::Mat4(Some(format!("{}", comps.len())), c),
                        vec![],
                        vec![],
                        self.create_loc(token.line),
                    ))
                } else {
                    Err(RPUError::new("Expected '(' after mat2", token.line))
                }
            }*/
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                if self.match_token(vec![TokenType::RightParen]) {
                    Ok(Expr::Grouping(Box::new(expr), self.create_loc(token.line)))
                } else {
                    Err(ParseError::new(
                        "Expected ')' after expression",
                        token.line,
                        &self.path,
                    ))
                }
            }
            TokenType::Identifier => {
                /*

                // Struct initialization ?
                if let Some(strct) = self.structs.get(&token.lexeme).cloned() {
                    self.advance();

                    if !self.match_token(vec![TokenType::LeftParen]) {
                        return Err(RPUError::new(
                            format!("Expected '(' after '{}'", token.lexeme),
                            token.line,
                        ));
                    }

                    let mut fields = vec![];

                    for (i, (_name, _value)) in strct.iter().enumerate() {
                        let expr = self.expression()?;
                        fields.push(Box::new(expr));

                        if i < strct.len() - 1 && !self.match_token(vec![TokenType::Comma]) {
                            return Err(RPUError::new(
                                "Expected ',' after struct field",
                                token.line,
                            ));
                        }
                    }

                    if !self.match_token(vec![TokenType::RightParen]) {
                        return Err(RPUError::new(
                            "Expected ')' after struct definition",
                            token.line,
                        ));
                    }

                    let field_path = self.get_field_path_at_current();

                    Ok(Expr::Value(
                        ASTValue::Struct(token.lexeme, Some("Instantiation".to_string()), fields),
                        vec![],
                        field_path,
                        self.create_loc(token.line),
                    ))
                } else {
                 */
                // Variable reference ?
                self.advance();

                let mut swizzle = vec![];
                let field_path = vec![];
                if self.check(TokenType::Dot) {
                    if self.is_swizzle_valid_at_current() {
                        swizzle = self.get_swizzle_at_current();
                    }
                    // else {
                    //     field_path = self.get_field_path_at_current();
                    // }
                }

                if token.lexeme == "local"
                    || token.lexeme == "world"
                    || token.lexeme == "u"
                    || token.lexeme == "v"
                    || token.lexeme == "d"
                    || token.lexeme == "hash"
                    || token.lexeme == "Clear"
                {
                    Ok(Expr::Variable(
                        token.lexeme.clone(),
                        swizzle,
                        field_path,
                        self.create_loc(token.line),
                    ))
                } else if let Some(_) = self.materials.get_index_of(&token.lexeme) {
                    Ok(Expr::MaterialReference(
                        token.lexeme.clone(),
                        self.create_loc(token.line),
                    ))
                } else if let Some(_) = self.verifier.get_var_name(&token.lexeme) {
                    Ok(Expr::Variable(
                        token.lexeme,
                        swizzle,
                        field_path,
                        self.create_loc(token.line),
                    ))
                } else {
                    // Check against inbuilt functions
                    Err(ParseError::new(
                        format!("Unknown identifier '{}'", token.lexeme),
                        token.line,
                        &self.path,
                    ))
                }
            }
            _ => Err(ParseError::new(
                format!("Unknown identifier '{}'", token.lexeme),
                token.line,
                &self.path,
            )),
        }
    }

    /// Reads the components of a vector up to `max_comps` components. Can terminate early if closing parenthesis is found.
    /// Check for component validity is done in the compiler.
    fn read_vec_components(
        &mut self,
        max_comps: usize,
        line: usize,
    ) -> Result<Vec<Expr>, ParseError> {
        let mut components = vec![];
        let mut count = 0;

        if self.match_token(vec![TokenType::RightParen]) {
            return Ok(components);
        }

        while count < max_comps {
            let expr = self.expression()?;

            components.push(expr);
            count += 1;

            if !self.match_token(vec![TokenType::Comma]) {
                if !self.match_token(vec![TokenType::RightParen]) {
                    return Err(ParseError::new(
                        "Expected ')' after vector components",
                        line,
                        &self.path,
                    ));
                }
                break;
            }
        }

        Ok(components)
    }

    /// Returns the swizzle at the current token if any.
    pub fn get_swizzle_at_current(&mut self) -> Vec<u8> {
        let mut swizzle: Vec<u8> = vec![];

        if self.current + 2 < self.tokens.len()
            && self.tokens[self.current].kind == TokenType::Dot
            && self.tokens[self.current + 1].kind == TokenType::Identifier
            && self.tokens[self.current + 2].kind != TokenType::Dot
        {
            let swizzle_token = self.tokens[self.current + 1].lexeme.clone();
            if swizzle_token
                .chars()
                .all(|c| matches!(c, 'x' | 'y' | 'z' | 'w'))
            {
                swizzle = swizzle_token
                    .chars()
                    .map(|c| match c {
                        'x' => 0,
                        'y' => 1,
                        'z' => 2,
                        'w' => 3,
                        _ => unreachable!(),
                    })
                    .collect();
                self.current += 2;
            }
        }

        swizzle
    }

    /// Returns true if a swizzle is valid at the current token.
    pub fn is_swizzle_valid_at_current(&self) -> bool {
        if self.current + 1 < self.tokens.len()
            && self.tokens[self.current].kind == TokenType::Dot
            && self.tokens[self.current + 1].kind == TokenType::Identifier
        {
            let swizzle_token = &self.tokens[self.current + 1].lexeme;
            swizzle_token
                .chars()
                .all(|c| matches!(c, 'x' | 'y' | 'z' | 'w'))
        } else {
            false
        }
    }

    /// Extract a potential swizzle from the variable name.
    fn _extract_swizzle(input: &str) -> (&str, Vec<u8>) {
        if let Some(pos) = input.rfind('.') {
            let (base, swizzle) = input.split_at(pos);
            let swizzle = &swizzle[1..]; // Skip the dot

            // Check if all characters in the swizzle are 'x', 'y', 'z', or 'w'
            if swizzle.chars().all(|c| matches!(c, 'x' | 'y' | 'z' | 'w')) {
                // Map 'x', 'y', 'z', 'w' to 0, 1, 2, 3 respectively
                let swizzle_bytes = swizzle
                    .chars()
                    .map(|c| match c {
                        'x' => 0,
                        'y' => 1,
                        'z' => 2,
                        'w' => 3,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<u8>>();

                return (base, swizzle_bytes);
            }
        }
        (input, Vec::new())
    }

    /// For debugging only
    fn _print_current(&self) {
        println!("Current: {:?}", self.tokens[self.current]);
    }

    // Consumes the next token if it matches the expected kind, otherwise returns a parse error.
    fn consume(
        &mut self,
        kind: TokenType,
        message: &str,
        line: usize,
    ) -> Result<Token, ParseError> {
        if self.check(kind) {
            Ok(self.advance().unwrap())
        } else {
            Err(ParseError::new(message, line, &self.path))
        }
    }

    // Advances if the next token matches any in the expected list, returns true if matched.
    fn match_token(&mut self, expected: Vec<TokenType>) -> bool {
        if expected.iter().any(|&kind| self.check(kind)) {
            self.advance();
            true
        } else {
            false
        }
    }

    // Advances and returns the matched token type if any in the expected list matches.
    fn _match_token_and_return(&mut self, expected: Vec<TokenType>) -> Option<TokenType> {
        for &kind in &expected {
            if self.check(kind) {
                self.advance();
                return Some(kind);
            }
        }
        None
    }

    // Returns the lexeme of the current token.
    fn _lexeme(&self) -> String {
        if self.current < self.tokens.len() {
            self.tokens[self.current].lexeme.clone()
        } else {
            "".to_string()
        }
    }

    // Checks if the current token matches the given kind.
    fn check(&self, kind: TokenType) -> bool {
        self.current < self.tokens.len() && self.tokens[self.current].kind == kind
    }

    // Checks if the next token matches the given kind.
    fn check_next(&self, kind: TokenType) -> bool {
        self.current + 1 < self.tokens.len() && self.tokens[self.current + 1].kind == kind
    }

    // Advances to the next token and returns the previous token.
    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current_line = self.tokens[self.current].line;
            self.current += 1;
        }
        self.previous()
    }

    // Returns true if all tokens have been consumed.
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    // Returns the current token or an EOF token if at the end.
    fn peek(&self) -> Token {
        if self.is_at_end() {
            Token {
                kind: TokenType::Eof,
                lexeme: "".to_string(),
                line: 0,
            }
        } else {
            self.tokens[self.current].clone()
        }
    }

    // Returns the previous token if available.
    fn previous(&self) -> Option<Token> {
        if self.current > 0 {
            Some(self.tokens[self.current - 1].clone())
        } else {
            None
        }
    }

    fn operator_to_unary(operator: TokenType) -> UnaryOperator {
        match operator {
            TokenType::Bang => UnaryOperator::Negate,
            TokenType::Minus => UnaryOperator::Minus,
            _ => unreachable!(),
        }
    }

    fn operator_to_binary(operator: TokenType) -> BinaryOperator {
        match operator {
            TokenType::Plus => BinaryOperator::Add,
            TokenType::Minus => BinaryOperator::Subtract,
            TokenType::Star => BinaryOperator::Multiply,
            TokenType::Slash => BinaryOperator::Divide,
            TokenType::Percent => BinaryOperator::Mod,
            _ => unreachable!(),
        }
    }

    fn operator_to_comparison(operator: TokenType) -> ComparisonOperator {
        match operator {
            TokenType::Greater => ComparisonOperator::Greater,
            TokenType::GreaterEqual => ComparisonOperator::GreaterEqual,
            TokenType::Less => ComparisonOperator::Less,
            TokenType::LessEqual => ComparisonOperator::LessEqual,
            _ => unreachable!(),
        }
    }

    fn operator_to_equality(operator: TokenType) -> EqualityOperator {
        match operator {
            TokenType::BangEqual => EqualityOperator::NotEqual,
            TokenType::EqualEqual => EqualityOperator::Equal,
            _ => unreachable!(),
        }
    }

    fn operator_to_logical(operator: TokenType) -> LogicalOperator {
        match operator {
            TokenType::And => LogicalOperator::And,
            TokenType::Or => LogicalOperator::Or,
            _ => unreachable!(),
        }
    }

    /// Create a location for the given line number.
    fn create_loc(&self, line: usize) -> Location {
        Location {
            line,
            path: self.path.clone(),
        }
    }
}
