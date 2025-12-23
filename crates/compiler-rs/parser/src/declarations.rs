//! Declaration parsing
//!
//! This module handles parsing of variable, constant, type, procedure, and function declarations.

use ast;
use ast::Node;
use errors::{ParserError, ParserResult};
use tokens::{Span, TokenKind};

use crate::directives::{DirectiveEvaluator, DirectiveType};

/// Declaration parsing functionality
impl super::Parser {
    /// Parse program: PROGRAM identifier ; block .
    pub(super) fn parse_program(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        // Collect directives before PROGRAM keyword
        // Also handle conditional compilation - directives may wrap PROGRAM
        // Note: {$INCLUDE} directives are handled in parse_block, not here
        let mut directives = vec![];
        loop {
            // Check for directives first
            if self.check(&TokenKind::Directive(String::new())) {
                if let Some(directive) = self.parse_directive()? {
                    // If it's an included block, we need to merge it into the program's block later
                    // For now, just store it as a directive - parse_block will handle merging
                    directives.push(directive);
                }
                // Continue to check for more directives or PROGRAM
                continue;
            }
            
            // If we're in an inactive conditional branch, skip tokens
            if !self.directive_evaluator().is_active() {
                if self.check(&TokenKind::Eof) {
                    // Reached EOF while in inactive branch - this is an error
                    return Err(ParserError::InvalidSyntax {
                        message: "Unmatched {$IFDEF} or {$IFNDEF} - reached end of file".to_string(),
                        span: self.current().map(|t| t.span).unwrap_or_else(|| Span::at(0, 1, 1)),
                    });
                }
                // Skip this token (including PROGRAM if it's in an inactive branch)
                self.advance()?;
                continue;
            }
            
            // We're active and not at a directive - check if we're at PROGRAM
            if self.check(&TokenKind::KwProgram) {
                break; // Found PROGRAM, exit loop
            }
            
            // Not a directive, not PROGRAM, and we're active - this is unexpected
            // This might be whitespace or comments, but PROGRAM should be next
            // Let's check if we should break or continue
            if self.check(&TokenKind::Eof) {
                return Err(ParserError::InvalidSyntax {
                    message: "Expected PROGRAM, UNIT, or LIBRARY".to_string(),
                    span: self.current().map(|t| t.span).unwrap_or_else(|| Span::at(0, 1, 1)),
                });
            }
            
            // Skip non-directive, non-PROGRAM tokens (whitespace/comments should be handled by lexer)
            // But if we get here, something unexpected happened
            self.advance()?;
        }

        // PROGRAM keyword
        self.consume(TokenKind::KwProgram, "PROGRAM")?;

        // Program name
        let name_token = self.consume(TokenKind::Identifier(String::new()), "identifier")?;
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return Err(ParserError::InvalidSyntax {
                message: "Expected identifier after PROGRAM".to_string(),
                span: name_token.span,
            }),
        };

        // Semicolon
        self.consume(TokenKind::Semicolon, ";")?;

        // Block
        let mut block = self.parse_block()?;
        
        // Merge any included blocks from directives collected before PROGRAM
        if let Node::Block(ref mut program_block) = block {
            for directive in &directives {
                if let Node::Block(included_block) = directive {
                    // Merge included block's content into program block
                    program_block.directives.extend(included_block.directives.clone());
                    program_block.label_decls.extend(included_block.label_decls.clone());
                    program_block.const_decls.extend(included_block.const_decls.clone());
                    program_block.type_decls.extend(included_block.type_decls.clone());
                    program_block.var_decls.extend(included_block.var_decls.clone());
                    program_block.threadvar_decls.extend(included_block.threadvar_decls.clone());
                    program_block.proc_decls.extend(included_block.proc_decls.clone());
                    program_block.func_decls.extend(included_block.func_decls.clone());
                    program_block.operator_decls.extend(included_block.operator_decls.clone());
                    program_block.statements.extend(included_block.statements.clone());
                } else {
                    // Regular directive - add to directives list
                    program_block.directives.push(directive.clone());
                }
            }
        }

        // Period
        self.consume(TokenKind::Dot, ".")?;

        // Check for EOF or directives (allow directives like {$ENDIF} after program)
        // Skip any remaining tokens that are directives or whitespace/comments
        while let Some(token) = self.current() {
            // If we see EOF, we're done
            if matches!(token.kind, TokenKind::Eof) {
                break;
            }
            // If we see a directive, process it (e.g., {$ENDIF} closing conditional block)
            if matches!(token.kind, TokenKind::Directive(_)) {
                if let Some(directive) = self.parse_directive()? {
                    directives.push(directive);
                }
                continue;
            }
            // Otherwise, there's unexpected content
            return Err(ParserError::InvalidSyntax {
                message: "Unexpected tokens after program end".to_string(),
                span: token.span,
            });
        }

        let span = start_span.merge(block.span());
        Ok(Node::Program(ast::Program {
            name,
            directives,
            block: Box::new(block),
            span,
        }))
    }

    /// Parse a compiler directive and evaluate it
    pub(crate) fn parse_directive(&mut self) -> ParserResult<Option<Node>> {
        let token = self.consume(TokenKind::Directive(String::new()), "directive")?;
        let content = match &token.kind {
            TokenKind::Directive(content) => content.clone(),
            _ => return Err(ParserError::InvalidSyntax {
                message: "Expected directive".to_string(),
                span: token.span,
            }),
        };

        // Parse directive type
        let directive_type = DirectiveEvaluator::parse_directive(&content);
        
        // Evaluate directive
        let (should_include, should_skip) = self.directive_evaluator_mut().evaluate(&directive_type, token.span)?;
        
        // Handle conditional compilation - skip tokens if needed
        if should_skip {
            let stopped_at_else = self.skip_until_conditional_end()?;
            // If we stopped at ELSE, we've already evaluated it in skip_until_conditional_end
            // Get the ELSE token and return it
            if stopped_at_else {
                let is_directive = self.current()
                    .map(|t| matches!(t.kind, TokenKind::Directive(_)))
                    .unwrap_or(false);
                if is_directive {
                    let else_token = self.current().unwrap();
                    let else_content = match &else_token.kind {
                        TokenKind::Directive(content) => content.clone(),
                        _ => return Ok(None),
                    };
                    let else_span = else_token.span;
                    // Consume the ELSE token
                    self.advance()?;
                    return Ok(Some(Node::Directive(ast::Directive {
                        content: else_content,
                        span: else_span,
                    })));
                }
            }
            // If we stopped at ENDIF, it was already consumed, return None
            return Ok(None);
        }
        
        // Handle INCLUDE directive specially - read and parse the file
        if let DirectiveType::Include(filename) = &directive_type {
            if should_include {
                // Read and parse the included file
                return self.handle_include_directive(filename, token.span);
            } else {
                // Include is in inactive branch, skip it
                return Ok(None);
            }
        }
        
        // Only include directive in AST if it's active or if it's a control directive
        // Control directives (IFDEF, IFNDEF, IF, ELSEIF, ELSE, ENDIF) are included for debugging
        // DEFINE/UNDEF are included if active
        let include_in_ast = match directive_type {
            DirectiveType::IfDef(_)
            | DirectiveType::IfNDef(_)
            | DirectiveType::If(_)
            | DirectiveType::ElseIf(_)
            | DirectiveType::Else
            | DirectiveType::EndIf => true, // Always include control directives
            DirectiveType::Define(_)
            | DirectiveType::Undef(_) => should_include, // Only if active
            _ => should_include, // Other directives only if active
        };
        
        if include_in_ast {
            Ok(Some(Node::Directive(ast::Directive {
                content,
                span: token.span,
            })))
        } else {
            Ok(None) // Directive processed but not included in AST
        }
    }

    /// Handle {$INCLUDE} directive - read file and parse it
    fn handle_include_directive(&mut self, filename: &str, span: tokens::Span) -> ParserResult<Option<Node>> {
        use std::fs;
        
        // Resolve file path
        let file_path = self.resolve_include_path(filename)?;
        
        // Check for circular includes
        let canonical_path = fs::canonicalize(&file_path)
            .map_err(|e| ParserError::InvalidSyntax {
                message: format!("Cannot resolve include path '{}': {}", filename, e),
                span,
            })?;
        let canonical_str = canonical_path.to_string_lossy().to_string();
        
        if self.included_files.contains(&canonical_str) {
            return Err(ParserError::InvalidSyntax {
                message: format!("Circular include detected: '{}'", filename),
                span,
            });
        }
        
        // Read the file
        let file_content = fs::read_to_string(&file_path)
            .map_err(|e| ParserError::InvalidSyntax {
                message: format!("Cannot read include file '{}': {}", filename, e),
                span,
            })?;
        
        // Mark file as included
        self.included_files.insert(canonical_str.clone());
        
        // Create a new parser for the included file
        let included_filename = Some(file_path.to_string_lossy().to_string());
        let mut included_parser = super::Parser::new_with_file_and_symbols(
            &file_content,
            included_filename.clone(),
            self.directive_evaluator().defined_symbols().iter().cloned().collect(),
        )?;
        
        // Copy include paths and included files to the new parser
        included_parser.include_paths = self.include_paths.clone();
        included_parser.included_files = self.included_files.clone();
        
        // Parse the included file - it can contain:
        // 1. A block (declarations and statements with BEGIN...END)
        // 2. Just declarations (for header files)
        // 3. Just statements (for code files)
        // Try to parse as declarations-only first (most common for header files)
        let included_ast = included_parser.parse_declarations_only()?;
        
        // Return the included content
        // The included block will be merged into the current context by the caller
        Ok(Some(included_ast))
    }
    
    /// Resolve include file path (check current directory, then include paths)
    fn resolve_include_path(&self, filename: &str) -> ParserResult<std::path::PathBuf> {
        use std::path::PathBuf;
        
        // If filename is absolute, use it directly
        let path = PathBuf::from(filename);
        if path.is_absolute() {
            if path.exists() {
                return Ok(path);
            }
        }
        
        // Try relative to current file's directory
        if let Some(ref current_file) = self.filename {
            if let Some(parent) = std::path::Path::new(current_file).parent() {
                let candidate = parent.join(filename);
                if candidate.exists() {
                    return Ok(candidate);
                }
            }
        }
        
        // Try include paths
        for include_path in &self.include_paths {
            let candidate = PathBuf::from(include_path).join(filename);
            if candidate.exists() {
                return Ok(candidate);
            }
        }
        
        // Try current directory
        let candidate = PathBuf::from(filename);
        if candidate.exists() {
            return Ok(candidate);
        }
        
        // Not found
        Err(ParserError::InvalidSyntax {
            message: format!("Include file not found: '{}'", filename),
            span: tokens::Span::at(0, 1, 1),
        })
    }

    /// Skip tokens until we reach ELSE or ENDIF (for conditional compilation)
    /// Returns true if we stopped at ELSE (need to process it), false if we stopped at ENDIF
    fn skip_until_conditional_end(&mut self) -> ParserResult<bool> {
        let mut depth = 1; // We're already inside one conditional
        while depth > 0 {
            let token_span = self.current().map(|t| t.span).unwrap_or_else(|| Span::at(0, 1, 1));
            let is_directive = self.current()
                .map(|t| matches!(t.kind, TokenKind::Directive(_)))
                .unwrap_or(false);
            
            if is_directive {
                let content = match &self.current().unwrap().kind {
                    TokenKind::Directive(content) => content.clone(),
                    _ => {
                        self.advance()?;
                        continue;
                    }
                };
                let directive_type = DirectiveEvaluator::parse_directive(&content);
                
                // Evaluate the directive to update state
                let (_, _) = self.directive_evaluator_mut().evaluate(&directive_type, token_span)?;
                
                match directive_type {
                    DirectiveType::IfDef(_)
                    | DirectiveType::IfNDef(_) => {
                        depth += 1; // Nested conditional
                        self.advance()?;
                    }
                        DirectiveType::Else => {
                            if depth == 1 {
                                // This ELSE matches our IFDEF - stop skipping
                                // Don't advance - the ELSE will be processed normally
                                return Ok(true); // Stopped at ELSE
                            }
                            self.advance()?;
                        }
                        DirectiveType::EndIf => {
                            depth -= 1;
                            if depth == 0 {
                                // This ENDIF matches our IFDEF - consume it and stop skipping
                                self.advance()?;
                                return Ok(false); // Stopped at ENDIF
                            }
                            self.advance()?;
                        }
                    _ => {
                        // Other directives - just skip
                        self.advance()?;
                    }
                }
            } else {
                // Not a directive - skip this token
                if self.current().is_none() {
                    // EOF - error, unmatched conditional
                    return Err(ParserError::InvalidSyntax {
                        message: "Unmatched {$IFDEF} or {$IFNDEF} - reached end of file".to_string(),
                        span: token_span,
                    });
                }
                self.advance()?;
            }
        }
        Ok(false) // Should not reach here normally
    }

    /// Parse declarations only (no BEGIN...END) - used for included header files
    fn parse_declarations_only(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        let mut directives = vec![];
        let mut label_decls = vec![];
        let mut const_decls = vec![];
        let mut type_decls = vec![];
        let mut var_decls = vec![];
        let mut threadvar_decls = vec![];
        let mut proc_decls = vec![];
        let mut func_decls = vec![];
        let mut operator_decls = vec![];
        let mut statements = vec![];

        // Parse declarations and optionally statements (if BEGIN is present)
        loop {
            // Check for directives first
            if self.check(&TokenKind::Directive(String::new())) {
                if let Some(directive) = self.parse_directive()? {
                    // Handle nested includes
                    if let Node::Block(included_block) = directive {
                        directives.extend(included_block.directives);
                        label_decls.extend(included_block.label_decls);
                        const_decls.extend(included_block.const_decls);
                        type_decls.extend(included_block.type_decls);
                        var_decls.extend(included_block.var_decls);
                        threadvar_decls.extend(included_block.threadvar_decls);
                        proc_decls.extend(included_block.proc_decls);
                        func_decls.extend(included_block.func_decls);
                        operator_decls.extend(included_block.operator_decls);
                        statements.extend(included_block.statements);
                    } else {
                        directives.push(directive);
                    }
                }
                continue;
            }
            
            // Skip tokens if we're in an inactive conditional branch
            if !self.directive_evaluator().is_active() {
                self.advance()?;
                continue;
            }
            
            // Check for BEGIN - if present, parse statements
            if self.check(&TokenKind::KwBegin) {
                self.advance()?; // consume BEGIN
                while !self.check(&TokenKind::KwEnd) && !self.check(&TokenKind::Eof) {
                    statements.push(self.parse_statement()?);
                    if self.check(&TokenKind::Semicolon) {
                        self.advance()?;
                    }
                }
                if self.check(&TokenKind::KwEnd) {
                    self.advance()?; // consume END
                }
                break;
            }
            
            // Parse declarations
            if self.check(&TokenKind::KwLabel) {
                label_decls.extend(self.parse_label_decls()?);
            } else if self.check(&TokenKind::KwConst) {
                const_decls.extend(self.parse_const_decls()?);
            } else if self.check(&TokenKind::KwResourcestring) {
                const_decls.extend(self.parse_resourcestring_decls()?);
            } else if self.check(&TokenKind::KwType) {
                type_decls.extend(self.parse_type_decls()?);
            } else if self.check(&TokenKind::KwVar) {
                var_decls.extend(self.parse_var_decls()?);
            } else if self.check(&TokenKind::KwThreadvar) {
                threadvar_decls.extend(self.parse_threadvar_decls()?);
            } else if self.check(&TokenKind::KwProcedure) {
                proc_decls.push(self.parse_procedure_decl()?);
            } else if self.check(&TokenKind::KwFunction) {
                func_decls.push(self.parse_function_decl()?);
            } else if self.check(&TokenKind::KwOperator) {
                operator_decls.push(self.parse_operator_decl()?);
            } else if self.check(&TokenKind::Eof) {
                // End of file - done
                break;
            } else {
                // Unknown token - might be a statement (for code-only includes)
                // Try to parse as statement, but if it fails, we're done
                let _saved_pos = self.current().map(|t| t.span);
                match self.parse_statement() {
                    Ok(stmt) => {
                        statements.push(stmt);
                        if self.check(&TokenKind::Semicolon) {
                            self.advance()?;
                        }
                    }
                    Err(_) => {
                        // Not a statement - we're done parsing
                        break;
                    }
                }
            }
        }

        let end_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));
        let span = start_span.merge(end_span);

        Ok(Node::Block(ast::Block {
            directives,
            label_decls,
            const_decls,
            type_decls,
            var_decls,
            threadvar_decls,
            proc_decls,
            func_decls,
            operator_decls,
            statements,
            span,
        }))
    }

    /// Parse block: [declarations] BEGIN statements END
    pub(crate) fn parse_block(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        let mut directives = vec![];
        let mut label_decls = vec![];
        let mut const_decls = vec![];
        let mut type_decls = vec![];
        let mut var_decls = vec![];
        let mut threadvar_decls = vec![];
        let mut proc_decls = vec![];
        let mut func_decls = vec![];
        let mut operator_decls = vec![];

        // Parse declarations (directives, label, const, resourcestring, type, var, threadvar, procedures, functions, operators)
        loop {
            // Check for directives first
            if self.check(&TokenKind::Directive(String::new())) {
                if let Some(directive) = self.parse_directive()? {
                    // Handle included blocks specially - merge their content into current block
                    if let Node::Block(included_block) = directive {
                        // Merge included block's content into current block
                        directives.extend(included_block.directives);
                        label_decls.extend(included_block.label_decls);
                        const_decls.extend(included_block.const_decls);
                        type_decls.extend(included_block.type_decls);
                        var_decls.extend(included_block.var_decls);
                        threadvar_decls.extend(included_block.threadvar_decls);
                        proc_decls.extend(included_block.proc_decls);
                        func_decls.extend(included_block.func_decls);
                        operator_decls.extend(included_block.operator_decls);
                        // Note: included block's statements will be added when we parse the statements section
                    } else {
                        directives.push(directive);
                    }
                }
                continue;
            }
            
            // Skip tokens if we're in an inactive conditional branch
            if !self.directive_evaluator().is_active() {
                // Skip this token and continue
                self.advance()?;
                continue;
            }
            if self.check(&TokenKind::KwLabel) {
                label_decls.extend(self.parse_label_decls()?);
            } else if self.check(&TokenKind::KwConst) {
                const_decls.extend(self.parse_const_decls()?);
            } else if self.check(&TokenKind::KwResourcestring) {
                const_decls.extend(self.parse_resourcestring_decls()?);
            } else if self.check(&TokenKind::KwType) {
                type_decls.extend(self.parse_type_decls()?);
            } else if self.check(&TokenKind::KwVar) {
                var_decls.extend(self.parse_var_decls()?);
            } else if self.check(&TokenKind::KwThreadvar) {
                threadvar_decls.extend(self.parse_threadvar_decls()?);
            } else if self.check(&TokenKind::KwProcedure) {
                proc_decls.push(self.parse_procedure_decl()?);
            } else if self.check(&TokenKind::KwFunction) {
                func_decls.push(self.parse_function_decl()?);
            } else if self.check(&TokenKind::KwOperator) {
                operator_decls.push(self.parse_operator_decl()?);
            } else {
                break;
            }
        }

        // BEGIN
        self.consume(TokenKind::KwBegin, "BEGIN")?;

        // Statements
        // Note: parse_statement is in statements.rs module
        let mut statements = vec![];
        while !self.check(&TokenKind::KwEnd) {
            statements.push(self.parse_statement()?);
            // Optional semicolon between statements
            if self.check(&TokenKind::Semicolon) {
                self.advance()?;
            }
        }

        // END
        let end_token = self.consume(TokenKind::KwEnd, "END")?;
        let span = start_span.merge(end_token.span);

        Ok(Node::Block(ast::Block {
            directives,
            label_decls,
            const_decls,
            type_decls,
            var_decls,
            threadvar_decls,
            proc_decls,
            func_decls,
            operator_decls,
            statements,
            span,
        }))
    }

    /// Parse label declarations: LABEL label { , label } ;
    pub(crate) fn parse_label_decls(&mut self) -> ParserResult<Vec<Node>> {
        self.consume(TokenKind::KwLabel, "LABEL")?;
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));
        
        let mut labels = vec![];
        loop {
            // Labels can be identifiers or integer literals
            let label_token = self.current().ok_or_else(|| ParserError::UnexpectedEof {
                expected: "label (identifier or integer)".to_string(),
                span: start_span,
            })?;
            
            let label_name = match &label_token.kind {
                TokenKind::Identifier(name) => name.clone(),
                TokenKind::IntegerLiteral { value, .. } => value.to_string(),
                _ => return Err(ParserError::InvalidSyntax {
                    message: "Expected identifier or integer literal for label".to_string(),
                    span: label_token.span,
                }),
            };
            labels.push(label_name);
            self.advance()?;
            
            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance()?; // consume comma
        }
        
        self.consume(TokenKind::Semicolon, ";")?;
        
        let end_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| start_span);
        let span = start_span.merge(end_span);
        
        Ok(vec![Node::LabelDecl(ast::LabelDecl {
            labels,
            span,
        })])
    }

    /// Parse constant declarations: CONST const_decl { ; const_decl }
    pub(crate) fn parse_const_decls(&mut self) -> ParserResult<Vec<Node>> {
        self.consume(TokenKind::KwConst, "CONST")?;
        let mut decls = vec![];
        loop {
            decls.push(self.parse_const_decl()?);
            if !self.check(&TokenKind::Semicolon) {
                break;
            }
            self.advance()?;
            if !matches!(self.current().map(|t| &t.kind), Some(TokenKind::Identifier(_))) {
                break;
            }
        }
        Ok(decls)
    }

    /// Parse single constant declaration: identifier = expression
    fn parse_const_decl(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        let name_token = self.consume(TokenKind::Identifier(String::new()), "identifier")?;
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return Err(ParserError::InvalidSyntax {
                message: "Expected identifier".to_string(),
                span: name_token.span,
            }),
        };

        self.consume(TokenKind::Equal, "=")?;
        let value = self.parse_expression()?;

        let span = start_span.merge(value.span());
        Ok(Node::ConstDecl(ast::ConstDecl {
            name,
            value: Box::new(value),
            is_resourcestring: false, // Set to true when parsing RESOURCESTRING section
            span,
        }))
    }

    /// Parse threadvar declarations: THREADVAR var_decl { ; var_decl }
    pub(crate) fn parse_threadvar_decls(&mut self) -> ParserResult<Vec<Node>> {
        self.consume(TokenKind::KwThreadvar, "THREADVAR")?;
        let mut decls = vec![];
        loop {
            decls.push(self.parse_var_decl()?);
            if !self.check(&TokenKind::Semicolon) {
                break;
            }
            self.advance()?;
            if !matches!(self.current().map(|t| &t.kind), Some(TokenKind::Identifier(_))) {
                break;
            }
        }
        Ok(decls)
    }

    /// Parse resourcestring declarations: RESOURCESTRING const_decl { ; const_decl }
    pub(crate) fn parse_resourcestring_decls(&mut self) -> ParserResult<Vec<Node>> {
        self.consume(TokenKind::KwResourcestring, "RESOURCESTRING")?;
        let mut decls = vec![];
        loop {
            let mut const_decl = self.parse_const_decl()?;
            // Mark as resourcestring
            if let Node::ConstDecl(ref mut c) = const_decl {
                c.is_resourcestring = true;
            }
            decls.push(const_decl);
            if !self.check(&TokenKind::Semicolon) {
                break;
            }
            self.advance()?;
            if !matches!(self.current().map(|t| &t.kind), Some(TokenKind::Identifier(_))) {
                break;
            }
        }
        Ok(decls)
    }

    /// Parse type declarations: TYPE type_decl { ; type_decl }
    pub(crate) fn parse_type_decls(&mut self) -> ParserResult<Vec<Node>> {
        self.consume(TokenKind::KwType, "TYPE")?;
        let mut decls = vec![];
        loop {
            decls.push(self.parse_type_decl()?);
            if !self.check(&TokenKind::Semicolon) {
                break;
            }
            self.advance()?;
            if !matches!(self.current().map(|t| &t.kind), Some(TokenKind::Identifier(_))) {
                break;
            }
        }
        Ok(decls)
    }

    /// Parse single type declaration: identifier = type
    fn parse_type_decl(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        let name_token = self.consume(TokenKind::Identifier(String::new()), "identifier")?;
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return Err(ParserError::InvalidSyntax {
                message: "Expected identifier".to_string(),
                span: name_token.span,
            }),
        };

        // Check for generic type parameters: <T> or <T: constraint>
        let generic_params = if self.check(&TokenKind::Less) {
            self.parse_generic_type_parameters()?
        } else {
            vec![]
        };

        self.consume(TokenKind::Equal, "=")?;
        let type_expr = self.parse_type()?;

        let span = start_span.merge(type_expr.span());
        Ok(Node::TypeDecl(ast::TypeDecl {
            name,
            generic_params,
            type_expr: Box::new(type_expr),
            span,
        }))
    }

    /// Parse variable declarations: VAR var_decl { ; var_decl }
    pub(crate) fn parse_var_decls(&mut self) -> ParserResult<Vec<Node>> {
        self.parse_var_decls_with_class_flag(false)
    }

    /// Parse variable declarations with optional class var flag
    pub(crate) fn parse_var_decls_with_class_flag(&mut self, is_class_var: bool) -> ParserResult<Vec<Node>> {
        self.consume(TokenKind::KwVar, "VAR")?;
        let mut decls = vec![];
        loop {
            decls.push(self.parse_var_decl_with_class_flag(is_class_var)?);
            if !self.check(&TokenKind::Semicolon) {
                break;
            }
            self.advance()?;
            if !matches!(self.current().map(|t| &t.kind), Some(TokenKind::Identifier(_))) {
                break;
            }
        }
        Ok(decls)
    }

    /// Parse single variable declaration: identifier_list : type [ABSOLUTE expression]
    fn parse_var_decl(&mut self) -> ParserResult<Node> {
        self.parse_var_decl_with_class_flag(false)
    }

    /// Parse single variable declaration with optional class var flag
    fn parse_var_decl_with_class_flag(&mut self, is_class_var: bool) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        let mut names = vec![];
        loop {
            let name_token = self.consume(TokenKind::Identifier(String::new()), "identifier")?;
            let name = match &name_token.kind {
                TokenKind::Identifier(name) => name.clone(),
                _ => return Err(ParserError::InvalidSyntax {
                    message: "Expected identifier".to_string(),
                    span: name_token.span,
                }),
            };
            names.push(name);

            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance()?;
        }

        self.consume(TokenKind::Colon, ":")?;
        let type_expr = self.parse_type()?;

        // Optional ABSOLUTE address: ABSOLUTE expression
        let absolute_address = if self.check(&TokenKind::KwAbsolute) {
            self.advance()?; // consume ABSOLUTE
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        let end_span = absolute_address.as_ref()
            .map(|a| a.span())
            .unwrap_or_else(|| type_expr.span());
        let span = start_span.merge(end_span);
        Ok(Node::VarDecl(ast::VarDecl {
            names,
            type_expr: Box::new(type_expr),
            absolute_address,
            is_class_var,
            span,
        }))
    }

    /// Parse qualified name: ClassName.MethodName or just MethodName
    /// Returns (class_name, method_name) where class_name is None if not present
    pub(crate) fn parse_qualified_name(&mut self) -> ParserResult<(Option<String>, String)> {
        let name_token = self.consume(TokenKind::Identifier(String::new()), "identifier")?;
        let first_name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return Err(ParserError::InvalidSyntax {
                message: "Expected identifier".to_string(),
                span: name_token.span,
            }),
        };

        // Check if there's a dot (ClassName.MethodName)
        if self.check(&TokenKind::Dot) {
            self.advance()?; // consume .
            let method_token = self.consume(TokenKind::Identifier(String::new()), "identifier")?;
            let method_name = match &method_token.kind {
                TokenKind::Identifier(name) => name.clone(),
                _ => return Err(ParserError::InvalidSyntax {
                    message: "Expected identifier after dot".to_string(),
                    span: method_token.span,
                }),
            };
            Ok((Some(first_name), method_name))
        } else {
            Ok((None, first_name))
        }
    }

    /// Parse procedure forward declaration: PROCEDURE [ClassName.]identifier [ ( params ) ] ;
    pub(crate) fn parse_procedure_forward_decl(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        self.consume(TokenKind::KwProcedure, "PROCEDURE")?;

        // Parse method name: ClassName.MethodName or just MethodName
        let (class_name, name) = self.parse_qualified_name()?;

        // Check for generic type parameters: <T> or <T: constraint>
        let generic_params = if self.check(&TokenKind::Less) {
            self.parse_generic_type_parameters()?
        } else {
            vec![]
        };

        let params = if self.check(&TokenKind::LeftParen) {
            self.parse_params()?
        } else {
            vec![]
        };

        self.consume(TokenKind::Semicolon, ";")?;

        // Create an empty block for forward declarations
        let empty_block = Node::Block(ast::Block {
            directives: vec![],
            label_decls: vec![],
            const_decls: vec![],
            type_decls: vec![],
            var_decls: vec![],
            threadvar_decls: vec![],
            proc_decls: vec![],
            func_decls: vec![],
            operator_decls: vec![],
            statements: vec![],
            span: start_span,
        });

        let span = start_span;
        Ok(Node::ProcDecl(ast::ProcDecl {
            name,
            class_name,
            generic_params,
            params,
            block: Box::new(empty_block),
            is_forward: false,
            is_external: false,
            external_name: None,
            is_class_method: false, // Forward declarations can't be class methods
            span,
        }))
    }

    /// Parse function forward declaration: FUNCTION [ClassName.]identifier [ ( params ) ] : type ;
    pub(crate) fn parse_function_forward_decl(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        self.consume(TokenKind::KwFunction, "FUNCTION")?;

        // Parse method name: ClassName.MethodName or just MethodName
        let (class_name, name) = self.parse_qualified_name()?;

        // Check for generic type parameters: <T> or <T: constraint>
        let generic_params = if self.check(&TokenKind::Less) {
            self.parse_generic_type_parameters()?
        } else {
            vec![]
        };

        let params = if self.check(&TokenKind::LeftParen) {
            self.parse_params()?
        } else {
            vec![]
        };

        self.consume(TokenKind::Colon, ":")?;
        let return_type = self.parse_type()?;
        self.consume(TokenKind::Semicolon, ";")?;

        // Create an empty block for forward declarations
        let empty_block = Node::Block(ast::Block {
            directives: vec![],
            label_decls: vec![],
            const_decls: vec![],
            type_decls: vec![],
            var_decls: vec![],
            threadvar_decls: vec![],
            proc_decls: vec![],
            func_decls: vec![],
            operator_decls: vec![],
            statements: vec![],
            span: start_span,
        });

        let span = start_span.merge(return_type.span());
        Ok(Node::FuncDecl(ast::FuncDecl {
            name,
            class_name,
            generic_params,
            params,
            return_type: Box::new(return_type),
            block: Box::new(empty_block),
            is_forward: false,
            is_external: false,
            external_name: None,
            is_class_method: false, // Forward declarations can't be class methods
            span,
        }))
    }

    /// Parse procedure declaration: PROCEDURE [ClassName.]identifier [ ( params ) ] ; [block | FORWARD | EXTERNAL [name]] ;
    /// 
    /// If `in_class_context` is true, procedures without explicit blocks are treated as forward declarations.
    /// Otherwise, they may be nested routines (if followed by declarations/BEGIN).
    pub(crate) fn parse_procedure_decl(&mut self) -> ParserResult<Node> {
        self.parse_procedure_decl_impl(false)
    }

    /// Parse procedure declaration in class context (always forward if no explicit block)
    pub(crate) fn parse_procedure_decl_in_class(&mut self) -> ParserResult<Node> {
        self.parse_procedure_decl_impl(true)
    }

    /// Internal implementation with context flag
    fn parse_procedure_decl_impl(&mut self, in_class_context: bool) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        // Check for CLASS keyword (class procedure)
        let is_class_method = if self.check(&TokenKind::KwClass) {
            self.advance()?; // consume CLASS
            true
        } else {
            false
        };

        self.consume(TokenKind::KwProcedure, "PROCEDURE")?;

        // Parse method name: ClassName.MethodName or just MethodName
        let (class_name, name) = self.parse_qualified_name()?;

        // Check for generic type parameters: <T> or <T: constraint>
        let generic_params = if self.check(&TokenKind::Less) {
            self.parse_generic_type_parameters()?
        } else {
            vec![]
        };

        let params = if self.check(&TokenKind::LeftParen) {
            self.parse_params()?
        } else {
            vec![]
        };

        self.consume(TokenKind::Semicolon, ";")?;
        
        // Check for FORWARD or EXTERNAL keyword
        let (is_forward, is_external, external_name) = if self.check(&TokenKind::KwForward) {
            self.advance()?; // consume FORWARD
            self.consume(TokenKind::Semicolon, ";")?;
            (true, false, None)
        } else if self.check(&TokenKind::KwExternal) {
            self.advance()?; // consume EXTERNAL
            // Optional external name: EXTERNAL 'name' or EXTERNAL name
            let ext_name = if let Some(token) = self.current() {
                match &token.kind {
                    TokenKind::StringLiteral(_s) => {
                        let name_token = self.advance_and_get_token()?;
                        match name_token.kind {
                            TokenKind::StringLiteral(s) => Some(s),
                            _ => None,
                        }
                    }
                    TokenKind::Identifier(_s) => {
                        let name_token = self.advance_and_get_token()?;
                        match name_token.kind {
                            TokenKind::Identifier(s) => Some(s),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            } else {
                None
            };
            self.consume(TokenKind::Semicolon, ";")?;
            (false, true, ext_name)
        } else if self.check(&TokenKind::KwBegin) {
            // Regular procedure with block
            let block = self.parse_block()?;
            self.consume(TokenKind::Semicolon, ";")?;
            let span = start_span.merge(block.span());
            return Ok(Node::ProcDecl(ast::ProcDecl {
                name,
                class_name,
                generic_params,
                params,
                block: Box::new(block),
                is_forward: false,
                is_external: false,
                external_name: None,
                is_class_method,
                span,
            }));
        } else if self.check(&TokenKind::KwLabel) ||
                   self.check(&TokenKind::KwConst) ||
                   self.check(&TokenKind::KwResourcestring) ||
                   self.check(&TokenKind::KwType) ||
                   self.check(&TokenKind::KwVar) ||
                   self.check(&TokenKind::KwThreadvar) ||
                   self.check(&TokenKind::KwOperator) {
            // Procedure with nested declarations (VAR, CONST, TYPE, etc. before BEGIN) - parse block
            let block = self.parse_block()?;
            self.consume(TokenKind::Semicolon, ";")?;
            let span = start_span.merge(block.span());
            return Ok(Node::ProcDecl(ast::ProcDecl {
                name,
                class_name,
                generic_params,
                params,
                block: Box::new(block),
                is_forward: false,
                is_external: false,
                external_name: None,
                is_class_method,
                span,
            }));
        } else if in_class_context {
            // In class context, PROCEDURE/FUNCTION without explicit block is forward declaration
            (true, false, None)
        } else if self.check(&TokenKind::KwProcedure) || self.check(&TokenKind::KwFunction) {
            // PROCEDURE/FUNCTION - try parsing as nested routine
            // parse_block will handle nested PROCEDURE/FUNCTION declarations
            let block = self.parse_block()?;
            self.consume(TokenKind::Semicolon, ";")?;
            let span = start_span.merge(block.span());
            return Ok(Node::ProcDecl(ast::ProcDecl {
                name,
                class_name,
                generic_params,
                params,
                block: Box::new(block),
                is_forward: false,
                is_external: false,
                external_name: None,
                is_class_method,
                span,
            }));
        } else {
            // Forward declaration (no block, no FORWARD keyword - common in classes)
            (true, false, None)
        };

        // Create empty block for forward/external declarations
        let empty_block = Node::Block(ast::Block {
            directives: vec![],
            label_decls: vec![],
            const_decls: vec![],
            type_decls: vec![],
            var_decls: vec![],
            threadvar_decls: vec![],
            proc_decls: vec![],
            func_decls: vec![],
            operator_decls: vec![],
            statements: vec![],
            span: start_span,
        });

        let span = start_span;
        Ok(Node::ProcDecl(ast::ProcDecl {
            name,
            class_name,
            generic_params,
            params,
            block: Box::new(empty_block),
            is_forward,
            is_external,
            external_name,
            is_class_method,
            span,
        }))
    }

    /// Parse function declaration: FUNCTION [ClassName.]identifier [ ( params ) ] : type ; block ;
    pub(crate) fn parse_function_decl(&mut self) -> ParserResult<Node> {
        self.parse_function_decl_impl(false)
    }

    /// Parse function declaration in class context (always forward if no explicit block)
    pub(crate) fn parse_function_decl_in_class(&mut self) -> ParserResult<Node> {
        self.parse_function_decl_impl(true)
    }

    /// Internal implementation with context flag
    fn parse_function_decl_impl(&mut self, in_class_context: bool) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        // Check for CLASS keyword (class function)
        let is_class_method = if self.check(&TokenKind::KwClass) {
            self.advance()?; // consume CLASS
            true
        } else {
            false
        };

        self.consume(TokenKind::KwFunction, "FUNCTION")?;

        // Parse method name: ClassName.MethodName or just MethodName
        let (class_name, name) = self.parse_qualified_name()?;

        // Check for generic type parameters: <T> or <T: constraint>
        let generic_params = if self.check(&TokenKind::Less) {
            self.parse_generic_type_parameters()?
        } else {
            vec![]
        };

        let params = if self.check(&TokenKind::LeftParen) {
            self.parse_params()?
        } else {
            vec![]
        };

        self.consume(TokenKind::Colon, ":")?;
        let return_type = self.parse_type()?;
        self.consume(TokenKind::Semicolon, ";")?;
        
        // Check for FORWARD or EXTERNAL keyword
        let (is_forward, is_external, external_name) = if self.check(&TokenKind::KwForward) {
            self.advance()?; // consume FORWARD
            self.consume(TokenKind::Semicolon, ";")?;
            (true, false, None)
        } else if self.check(&TokenKind::KwExternal) {
            self.advance()?; // consume EXTERNAL
            // Optional external name: EXTERNAL 'name' or EXTERNAL name
            let ext_name = if let Some(token) = self.current() {
                match &token.kind {
                    TokenKind::StringLiteral(_s) => {
                        let name_token = self.advance_and_get_token()?;
                        match name_token.kind {
                            TokenKind::StringLiteral(s) => Some(s),
                            _ => None,
                        }
                    }
                    TokenKind::Identifier(_s) => {
                        let name_token = self.advance_and_get_token()?;
                        match name_token.kind {
                            TokenKind::Identifier(s) => Some(s),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            } else {
                None
            };
            self.consume(TokenKind::Semicolon, ";")?;
            (false, true, ext_name)
        } else if self.check(&TokenKind::KwBegin) {
            // Regular function with block
            let block = self.parse_block()?;
            self.consume(TokenKind::Semicolon, ";")?;
            let span = start_span.merge(block.span());
            return Ok(Node::FuncDecl(ast::FuncDecl {
                name,
                class_name,
                generic_params,
                params,
                return_type: Box::new(return_type),
                block: Box::new(block),
                is_forward: false,
                is_external: false,
                external_name: None,
                is_class_method,
                span,
            }));
        } else if self.check(&TokenKind::KwLabel) ||
                   self.check(&TokenKind::KwConst) ||
                   self.check(&TokenKind::KwResourcestring) ||
                   self.check(&TokenKind::KwType) ||
                   self.check(&TokenKind::KwVar) ||
                   self.check(&TokenKind::KwThreadvar) ||
                   self.check(&TokenKind::KwOperator) {
            // Function with nested declarations (VAR, CONST, TYPE, etc. before BEGIN) - parse block
            let block = self.parse_block()?;
            self.consume(TokenKind::Semicolon, ";")?;
            let span = start_span.merge(block.span());
            return Ok(Node::FuncDecl(ast::FuncDecl {
                name,
                class_name,
                generic_params,
                params,
                return_type: Box::new(return_type),
                block: Box::new(block),
                is_forward: false,
                is_external: false,
                external_name: None,
                is_class_method,
                span,
            }));
        } else if in_class_context {
            // In class context, PROCEDURE/FUNCTION without explicit block is forward declaration
            (true, false, None)
        } else if self.check(&TokenKind::KwProcedure) || self.check(&TokenKind::KwFunction) {
            // PROCEDURE/FUNCTION - try parsing as nested routine
            // parse_block will handle nested PROCEDURE/FUNCTION declarations
            let block = self.parse_block()?;
            self.consume(TokenKind::Semicolon, ";")?;
            let span = start_span.merge(block.span());
            return Ok(Node::FuncDecl(ast::FuncDecl {
                name,
                class_name,
                generic_params,
                params,
                return_type: Box::new(return_type),
                block: Box::new(block),
                is_forward: false,
                is_external: false,
                external_name: None,
                is_class_method,
                span,
            }));
        } else {
            // Forward declaration (no block, no FORWARD keyword - common in classes)
            (true, false, None)
        };

        // Create empty block for forward/external declarations
        let empty_block = Node::Block(ast::Block {
            directives: vec![],
            label_decls: vec![],
            const_decls: vec![],
            type_decls: vec![],
            var_decls: vec![],
            threadvar_decls: vec![],
            proc_decls: vec![],
            func_decls: vec![],
            operator_decls: vec![],
            statements: vec![],
            span: start_span,
        });

        let span = start_span.merge(return_type.span());
        Ok(Node::FuncDecl(ast::FuncDecl {
            name,
            class_name,
            generic_params,
            params,
            return_type: Box::new(return_type),
            block: Box::new(empty_block),
            is_forward,
            is_external,
            external_name,
            is_class_method,
            span,
        }))
    }

    /// Parse operator name: [ClassName.]operator_name
    /// The operator_name can be a symbol (+, -, *, etc.) or an identifier (sub, add, etc.)
    /// Returns (class_name, operator_name) where class_name is None if not present
    fn parse_operator_name(&mut self) -> ParserResult<(Option<String>, String)> {
        // Check if we have a class name prefix (ClassName.)
        let class_name = if let Some(token) = self.current() {
            if matches!(token.kind, TokenKind::Identifier(_)) {
                let name_token = self.advance_and_get_token()?;
                let first_name = match name_token.kind {
                    TokenKind::Identifier(name) => name,
                    _ => unreachable!(),
                };
                
                // Check if there's a dot
                if self.check(&TokenKind::Dot) {
                    self.advance()?; // consume .
                    Some(first_name)
                } else {
                    // No dot, so this is the operator name itself (identifier)
                    return Ok((None, first_name));
                }
            } else {
                None
            }
        } else {
            None
        };

        // Now parse the operator name (can be symbol or identifier)
        let operator_name = if let Some(token) = self.current() {
            match &token.kind {
                // Operator symbols
                TokenKind::Plus => {
                    self.advance()?;
                    "+".to_string()
                }
                TokenKind::Minus => {
                    self.advance()?;
                    "-".to_string()
                }
                TokenKind::Star => {
                    self.advance()?;
                    "*".to_string()
                }
                TokenKind::Slash => {
                    self.advance()?;
                    "/".to_string()
                }
                TokenKind::Equal => {
                    self.advance()?;
                    "=".to_string()
                }
                TokenKind::NotEqual => {
                    self.advance()?;
                    "<>".to_string()
                }
                TokenKind::Less => {
                    self.advance()?;
                    "<".to_string()
                }
                TokenKind::LessEqual => {
                    self.advance()?;
                    "<=".to_string()
                }
                TokenKind::Greater => {
                    self.advance()?;
                    ">".to_string()
                }
                TokenKind::GreaterEqual => {
                    self.advance()?;
                    ">=".to_string()
                }
                TokenKind::Dot => {
                    self.advance()?;
                    ".".to_string()
                }
                TokenKind::Caret => {
                    self.advance()?;
                    "^".to_string()
                }
                // Identifier operator name
                TokenKind::Identifier(_name) => {
                    let name_token = self.advance_and_get_token()?;
                    match name_token.kind {
                        TokenKind::Identifier(name) => name,
                        _ => unreachable!(),
                    }
                }
                _ => {
                    return Err(ParserError::InvalidSyntax {
                        message: "Expected operator symbol or identifier".to_string(),
                        span: token.span,
                    });
                }
            }
        } else {
            let span = self.peek_token()
                .map(|t| t.span)
                .unwrap_or_else(|| Span::at(0, 1, 1));
            return Err(ParserError::UnexpectedEof {
                expected: "operator name".to_string(),
                span,
            });
        };

        Ok((class_name, operator_name))
    }

    /// Parse operator declaration: OPERATOR [ClassName.]operator_name [ ( params ) ] : return_type ; [block | FORWARD | EXTERNAL] ;
    pub(crate) fn parse_operator_decl(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        self.consume(TokenKind::KwOperator, "OPERATOR")?;

        // Parse operator name: [ClassName.]operator_name
        let (class_name, operator_name) = self.parse_operator_name()?;

        // Optional parameters
        let params = if self.check(&TokenKind::LeftParen) {
            self.parse_params()?
        } else {
            vec![]
        };

        // Return type (required for operators)
        self.consume(TokenKind::Colon, ":")?;
        let return_type = self.parse_type()?;
        self.consume(TokenKind::Semicolon, ";")?;

        // Check for FORWARD or EXTERNAL keyword
        let (is_forward, is_external, external_name) = if self.check(&TokenKind::KwForward) {
            self.advance()?; // consume FORWARD
            self.consume(TokenKind::Semicolon, ";")?;
            (true, false, None)
        } else if self.check(&TokenKind::KwExternal) {
            self.advance()?; // consume EXTERNAL
            // Optional external name: EXTERNAL 'name' or EXTERNAL name
            let ext_name = if let Some(token) = self.current() {
                match &token.kind {
                    TokenKind::StringLiteral(_s) => {
                        let name_token = self.advance_and_get_token()?;
                        match name_token.kind {
                            TokenKind::StringLiteral(s) => Some(s),
                            _ => None,
                        }
                    }
                    TokenKind::Identifier(_s) => {
                        let name_token = self.advance_and_get_token()?;
                        match name_token.kind {
                            TokenKind::Identifier(s) => Some(s),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            } else {
                None
            };
            self.consume(TokenKind::Semicolon, ";")?;
            (false, true, ext_name)
        } else {
            // Regular operator with block
            let block = self.parse_block()?;
            self.consume(TokenKind::Semicolon, ";")?;
            let span = start_span.merge(block.span());
            return Ok(Node::OperatorDecl(ast::OperatorDecl {
                operator_name,
                class_name,
                params,
                return_type: Box::new(return_type),
                block: Box::new(block),
                is_forward: false,
                is_external: false,
                external_name: None,
                span,
            }));
        };

        // Create empty block for forward/external declarations
        let empty_block = Node::Block(ast::Block {
            directives: vec![],
            label_decls: vec![],
            const_decls: vec![],
            type_decls: vec![],
            var_decls: vec![],
            threadvar_decls: vec![],
            proc_decls: vec![],
            func_decls: vec![],
            operator_decls: vec![],
            statements: vec![],
            span: start_span,
        });

        let span = start_span.merge(return_type.span());
        Ok(Node::OperatorDecl(ast::OperatorDecl {
            operator_name,
            class_name,
            params,
            return_type: Box::new(return_type),
            block: Box::new(empty_block),
            is_forward,
            is_external,
            external_name,
            span,
        }))
    }

    /// Parse parameter list: ( param { ; param } )
    pub(crate) fn parse_params(&mut self) -> ParserResult<Vec<ast::Param>> {
        self.consume(TokenKind::LeftParen, "(")?;
        let mut params = vec![];

        if !self.check(&TokenKind::RightParen) {
            loop {
                params.push(self.parse_param()?);
                if !self.check(&TokenKind::Semicolon) {
                    break;
                }
                self.advance()?;
            }
        }

        self.consume(TokenKind::RightParen, ")")?;
        Ok(params)
    }

    /// Parse parameter: [VAR | CONST | CONSTREF | OUT] identifier_list : type [= default_value]
    pub(crate) fn parse_param(&mut self) -> ParserResult<ast::Param> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        let param_type = if self.check(&TokenKind::KwVar) {
            self.advance()?;
            ast::ParamType::Var
        } else if self.check(&TokenKind::KwConst) {
            self.advance()?;
            ast::ParamType::Const
        } else if self.check(&TokenKind::KwConstref) {
            self.advance()?;
            ast::ParamType::ConstRef
        } else if self.check(&TokenKind::KwOut) {
            self.advance()?;
            ast::ParamType::Out
        } else {
            ast::ParamType::Value
        };

        let mut names = vec![];
        loop {
            let name_token = self.consume(TokenKind::Identifier(String::new()), "identifier")?;
            let name = match &name_token.kind {
                TokenKind::Identifier(name) => name.clone(),
                _ => return Err(ParserError::InvalidSyntax {
                    message: "Expected identifier".to_string(),
                    span: name_token.span,
                }),
            };
            names.push(name);

            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance()?;
        }

        self.consume(TokenKind::Colon, ":")?;
        let type_expr = self.parse_type()?;

        // Optional default value: = expression
        let default_value = if self.check(&TokenKind::Equal) {
            self.advance()?; // consume =
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        let end_span = default_value.as_ref()
            .map(|v| v.span())
            .unwrap_or_else(|| type_expr.span());
        let span = start_span.merge(end_span);
        Ok(ast::Param {
            names,
            param_type,
            type_expr: Box::new(type_expr),
            default_value,
            span,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::Parser;
    use ast::Node;

    #[test]
    fn test_parse_simple_program() {
        let source = r#"
            program Hello;
            begin
                writeln('Hello, World!');
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        if let Err(e) = &result {
            eprintln!("Parse error: {}", e);
        }
        assert!(result.is_ok(), "Parse failed: {:?}", result);
    }

    // ===== Nested Routines Tests =====

    #[test]
    fn test_parse_nested_function_in_procedure() {
        let source = r#"
            program Test;
            procedure Outer;
                function Inner: integer;
                begin
                    Inner := 42;
                end;
            begin
                writeln(Inner);
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                assert_eq!(block.proc_decls.len(), 1);
                if let Node::ProcDecl(outer_proc) = &block.proc_decls[0] {
                    if let Node::Block(proc_block) = outer_proc.block.as_ref() {
                        // Should have one nested function
                        assert_eq!(proc_block.func_decls.len(), 1);
                        if let Node::FuncDecl(inner_func) = &proc_block.func_decls[0] {
                            assert_eq!(inner_func.name, "Inner");
                        } else {
                            panic!("Expected FuncDecl");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_nested_procedure_in_function() {
        let source = r#"
            program Test;
            function Outer: integer;
                procedure Inner;
                begin
                    writeln('Inner');
                end;
            begin
                Inner;
                Outer := 10;
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                assert_eq!(block.func_decls.len(), 1);
                if let Node::FuncDecl(outer_func) = &block.func_decls[0] {
                    if let Node::Block(func_block) = outer_func.block.as_ref() {
                        // Should have one nested procedure
                        assert_eq!(func_block.proc_decls.len(), 1);
                        if let Node::ProcDecl(inner_proc) = &func_block.proc_decls[0] {
                            assert_eq!(inner_proc.name, "Inner");
                        } else {
                            panic!("Expected ProcDecl");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_deeply_nested_routines() {
        let source = r#"
            program Test;
            procedure Level1;
                function Level2: integer;
                    procedure Level3;
                    begin
                    end;
                begin
                    Level2 := 1;
                end;
            begin
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(level1) = &block.proc_decls[0] {
                    if let Node::Block(level1_block) = level1.block.as_ref() {
                        if let Node::FuncDecl(level2) = &level1_block.func_decls[0] {
                            if let Node::Block(level2_block) = level2.block.as_ref() {
                                assert_eq!(level2_block.proc_decls.len(), 1);
                                if let Node::ProcDecl(level3) = &level2_block.proc_decls[0] {
                                    assert_eq!(level3.name, "Level3");
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_nested_routine_with_local_vars() {
        let source = r#"
            program Test;
            procedure Outer;
                var x: integer;
                function Inner: integer;
                    var y: integer;
                begin
                    Inner := x + y;
                end;
            begin
                x := Inner;
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(outer_proc) = &block.proc_decls[0] {
                    if let Node::Block(proc_block) = outer_proc.block.as_ref() {
                        // Should have local var and nested function
                        assert_eq!(proc_block.var_decls.len(), 1);
                        assert_eq!(proc_block.func_decls.len(), 1);
                        // Nested function should also have local var
                        if let Node::FuncDecl(inner_func) = &proc_block.func_decls[0] {
                            if let Node::Block(inner_block) = inner_func.block.as_ref() {
                                assert_eq!(inner_block.var_decls.len(), 1);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_multiple_nested_routines() {
        let source = r#"
            program Test;
            procedure Outer;
                procedure Helper1;
                begin
                end;
                function Helper2: integer;
                begin
                    Helper2 := 2;
                end;
            begin
                Helper1;
                writeln(Helper2);
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(outer_proc) = &block.proc_decls[0] {
                    if let Node::Block(proc_block) = outer_proc.block.as_ref() {
                        // Should have both nested routines
                        assert_eq!(proc_block.proc_decls.len(), 1);
                        assert_eq!(proc_block.func_decls.len(), 1);
                        if let Node::ProcDecl(helper1) = &proc_block.proc_decls[0] {
                            assert_eq!(helper1.name, "Helper1");
                        }
                        if let Node::FuncDecl(helper2) = &proc_block.func_decls[0] {
                            assert_eq!(helper2.name, "Helper2");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_nested_routine_with_params() {
        let source = r#"
            program Test;
            procedure Outer;
                function Inner(x: integer): integer;
                begin
                    Inner := x * 2;
                end;
            begin
                writeln(Inner(5));
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(outer_proc) = &block.proc_decls[0] {
                    if let Node::Block(proc_block) = outer_proc.block.as_ref() {
                        if let Node::FuncDecl(inner_func) = &proc_block.func_decls[0] {
                            assert_eq!(inner_func.name, "Inner");
                            assert_eq!(inner_func.params.len(), 1);
                        }
                    }
                }
            }
        }
    }

    // ===== Method Declaration Tests =====

    #[test]
    fn test_parse_method_procedure() {
        let source = r#"
            program Test;
            procedure MyClass.MyMethod;
            begin
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                assert_eq!(block.proc_decls.len(), 1);
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.name, "MyMethod");
                    assert_eq!(proc.class_name, Some("MyClass".to_string()));
                }
            }
        }
    }

    #[test]
    fn test_parse_method_function() {
        let source = r#"
            program Test;
            function MyClass.GetValue: integer;
            begin
                GetValue := 42;
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                assert_eq!(block.func_decls.len(), 1);
                if let Node::FuncDecl(func) = &block.func_decls[0] {
                    assert_eq!(func.name, "GetValue");
                    assert_eq!(func.class_name, Some("MyClass".to_string()));
                }
            }
        }
    }

    #[test]
    fn test_parse_method_with_params() {
        let source = r#"
            program Test;
            procedure MyClass.SetValue(x: integer);
            begin
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.name, "SetValue");
                    assert_eq!(proc.class_name, Some("MyClass".to_string()));
                    assert_eq!(proc.params.len(), 1);
                }
            }
        }
    }

    #[test]
    fn test_parse_regular_procedure_still_works() {
        let source = r#"
            program Test;
            procedure RegularProc;
            begin
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.name, "RegularProc");
                    assert_eq!(proc.class_name, None);
                }
            }
        }
    }

    #[test]
    fn test_parse_multiple_methods_same_class() {
        let source = r#"
            program Test;
            procedure MyClass.Method1;
            begin
            end;
            function MyClass.Method2: integer;
            begin
                Method2 := 1;
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                assert_eq!(block.proc_decls.len(), 1);
                assert_eq!(block.func_decls.len(), 1);
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.class_name, Some("MyClass".to_string()));
                    assert_eq!(proc.name, "Method1");
                }
                if let Node::FuncDecl(func) = &block.func_decls[0] {
                    assert_eq!(func.class_name, Some("MyClass".to_string()));
                    assert_eq!(func.name, "Method2");
                }
            }
        }
    }

    #[test]
    fn test_parse_methods_different_classes() {
        let source = r#"
            program Test;
            procedure ClassA.MethodA;
            begin
            end;
            procedure ClassB.MethodB;
            begin
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                assert_eq!(block.proc_decls.len(), 2);
                if let Node::ProcDecl(proc1) = &block.proc_decls[0] {
                    assert_eq!(proc1.class_name, Some("ClassA".to_string()));
                    assert_eq!(proc1.name, "MethodA");
                }
                if let Node::ProcDecl(proc2) = &block.proc_decls[1] {
                    assert_eq!(proc2.class_name, Some("ClassB".to_string()));
                    assert_eq!(proc2.name, "MethodB");
                }
            }
        }
    }

    #[test]
    fn test_parse_nested_routines_with_all_declarations() {
        let source = r#"
            program Test;
            procedure Outer;
                const C = 10;
                type T = integer;
                var v: integer;
                procedure Nested;
                begin
                end;
            begin
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(outer_proc) = &block.proc_decls[0] {
                    if let Node::Block(proc_block) = outer_proc.block.as_ref() {
                        // Should have all declaration types
                        assert_eq!(proc_block.const_decls.len(), 1);
                        assert_eq!(proc_block.type_decls.len(), 1);
                        assert_eq!(proc_block.var_decls.len(), 1);
                        assert_eq!(proc_block.proc_decls.len(), 1);
                    }
                }
            }
        }
    }

    // ========== FORWARD Declaration Tests ==========

    #[test]
    fn test_parse_forward_procedure() {
        let source = r#"
            program Test;
            procedure MyProc; forward;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.name, "MyProc");
                    assert!(proc.is_forward, "Procedure should be marked as forward");
                    assert!(!proc.is_external, "Procedure should not be external");
                    assert_eq!(proc.external_name, None);
                }
            }
        }
    }

    #[test]
    fn test_parse_forward_function() {
        let source = r#"
            program Test;
            function MyFunc: integer; forward;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::FuncDecl(func) = &block.func_decls[0] {
                    assert_eq!(func.name, "MyFunc");
                    assert!(func.is_forward, "Function should be marked as forward");
                    assert!(!func.is_external, "Function should not be external");
                    assert_eq!(func.external_name, None);
                }
            }
        }
    }

    #[test]
    fn test_parse_forward_procedure_with_params() {
        let source = r#"
            program Test;
            procedure MyProc(x: integer; y: string); forward;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.name, "MyProc");
                    assert_eq!(proc.params.len(), 2);
                    assert!(proc.is_forward);
                }
            }
        }
    }

    #[test]
    fn test_parse_forward_function_with_params() {
        let source = r#"
            program Test;
            function MyFunc(a: integer; b: boolean): string; forward;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::FuncDecl(func) = &block.func_decls[0] {
                    assert_eq!(func.name, "MyFunc");
                    assert_eq!(func.params.len(), 2);
                    assert!(func.is_forward);
                }
            }
        }
    }

    #[test]
    fn test_parse_forward_method() {
        let source = r#"
            program Test;
            procedure MyClass.MyMethod; forward;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.name, "MyMethod");
                    assert_eq!(proc.class_name, Some("MyClass".to_string()));
                    assert!(proc.is_forward);
                }
            }
        }
    }

    // ========== EXTERNAL Declaration Tests ==========

    #[test]
    fn test_parse_external_procedure() {
        let source = r#"
            program Test;
            procedure MyProc; external;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.name, "MyProc");
                    assert!(!proc.is_forward, "Procedure should not be forward");
                    assert!(proc.is_external, "Procedure should be marked as external");
                    assert_eq!(proc.external_name, None);
                }
            }
        }
    }

    #[test]
    fn test_parse_external_function() {
        let source = r#"
            program Test;
            function MyFunc: integer; external;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::FuncDecl(func) = &block.func_decls[0] {
                    assert_eq!(func.name, "MyFunc");
                    assert!(!func.is_forward, "Function should not be forward");
                    assert!(func.is_external, "Function should be marked as external");
                    assert_eq!(func.external_name, None);
                }
            }
        }
    }

    #[test]
    fn test_parse_external_procedure_with_string_name() {
        let source = r#"
            program Test;
            procedure MyProc; external 'external_proc';
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.name, "MyProc");
                    assert!(proc.is_external);
                    assert_eq!(proc.external_name, Some("external_proc".to_string()));
                }
            }
        }
    }

    #[test]
    fn test_parse_external_function_with_string_name() {
        let source = r#"
            program Test;
            function MyFunc: integer; external 'external_func';
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::FuncDecl(func) = &block.func_decls[0] {
                    assert_eq!(func.name, "MyFunc");
                    assert!(func.is_external);
                    assert_eq!(func.external_name, Some("external_func".to_string()));
                }
            }
        }
    }

    #[test]
    fn test_parse_external_procedure_with_identifier_name() {
        let source = r#"
            program Test;
            procedure MyProc; external ExternalName;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.name, "MyProc");
                    assert!(proc.is_external);
                    assert_eq!(proc.external_name, Some("ExternalName".to_string()));
                }
            }
        }
    }

    #[test]
    fn test_parse_external_function_with_identifier_name() {
        let source = r#"
            program Test;
            function MyFunc: integer; external ExternalFuncName;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::FuncDecl(func) = &block.func_decls[0] {
                    assert_eq!(func.name, "MyFunc");
                    assert!(func.is_external);
                    assert_eq!(func.external_name, Some("ExternalFuncName".to_string()));
                }
            }
        }
    }

    #[test]
    fn test_parse_external_procedure_with_params() {
        let source = r#"
            program Test;
            procedure MyProc(x: integer; y: string); external;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.name, "MyProc");
                    assert_eq!(proc.params.len(), 2);
                    assert!(proc.is_external);
                }
            }
        }
    }

    #[test]
    fn test_parse_external_function_with_params() {
        let source = r#"
            program Test;
            function MyFunc(a: integer; b: boolean): string; external 'lib_func';
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::FuncDecl(func) = &block.func_decls[0] {
                    assert_eq!(func.name, "MyFunc");
                    assert_eq!(func.params.len(), 2);
                    assert!(func.is_external);
                    assert_eq!(func.external_name, Some("lib_func".to_string()));
                }
            }
        }
    }

    #[test]
    fn test_parse_external_method() {
        let source = r#"
            program Test;
            procedure MyClass.MyMethod; external 'C_method';
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.name, "MyMethod");
                    assert_eq!(proc.class_name, Some("MyClass".to_string()));
                    assert!(proc.is_external);
                    assert_eq!(proc.external_name, Some("C_method".to_string()));
                }
            }
        }
    }

    #[test]
    fn test_parse_regular_procedure_not_forward_or_external() {
        let source = r#"
            program Test;
            procedure MyProc;
            begin
                writeln('Hello');
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.name, "MyProc");
                    assert!(!proc.is_forward, "Regular procedure should not be forward");
                    assert!(!proc.is_external, "Regular procedure should not be external");
                    assert_eq!(proc.external_name, None);
                }
            }
        }
    }

    #[test]
    fn test_parse_regular_function_not_forward_or_external() {
        let source = r#"
            program Test;
            function MyFunc: integer;
            begin
                MyFunc := 42;
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::FuncDecl(func) = &block.func_decls[0] {
                    assert_eq!(func.name, "MyFunc");
                    assert!(!func.is_forward, "Regular function should not be forward");
                    assert!(!func.is_external, "Regular function should not be external");
                    assert_eq!(func.external_name, None);
                }
            }
        }
    }

    #[test]
    fn test_parse_multiple_forward_and_external() {
        let source = r#"
            program Test;
            procedure ForwardProc; forward;
            function ForwardFunc: integer; forward;
            procedure ExternalProc; external 'ext_proc';
            function ExternalFunc: string; external 'ext_func';
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                assert_eq!(block.proc_decls.len(), 2);
                assert_eq!(block.func_decls.len(), 2);
                
                if let Node::ProcDecl(forward_proc) = &block.proc_decls[0] {
                    assert_eq!(forward_proc.name, "ForwardProc");
                    assert!(forward_proc.is_forward);
                    assert!(!forward_proc.is_external);
                }
                
                if let Node::FuncDecl(forward_func) = &block.func_decls[0] {
                    assert_eq!(forward_func.name, "ForwardFunc");
                    assert!(forward_func.is_forward);
                    assert!(!forward_func.is_external);
                }
                
                if let Node::ProcDecl(ext_proc) = &block.proc_decls[1] {
                    assert_eq!(ext_proc.name, "ExternalProc");
                    assert!(!ext_proc.is_forward);
                    assert!(ext_proc.is_external);
                    assert_eq!(ext_proc.external_name, Some("ext_proc".to_string()));
                }
                
                if let Node::FuncDecl(ext_func) = &block.func_decls[1] {
                    assert_eq!(ext_func.name, "ExternalFunc");
                    assert!(!ext_func.is_forward);
                    assert!(ext_func.is_external);
                    assert_eq!(ext_func.external_name, Some("ext_func".to_string()));
                }
            }
        }
    }

    // ========== Operator Declaration Tests ==========

    #[test]
    fn test_parse_operator_simple() {
        let source = r#"
            program Test;
            operator +(a, b: integer): integer;
            begin
                Result := a + b;
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                assert_eq!(block.operator_decls.len(), 1);
                if let Node::OperatorDecl(op) = &block.operator_decls[0] {
                    assert_eq!(op.operator_name, "+");
                    assert_eq!(op.class_name, None);
                    assert_eq!(op.params.len(), 1);
                    assert_eq!(op.params[0].names.len(), 2); // a, b
                    assert!(!op.is_forward);
                    assert!(!op.is_external);
                }
            }
        }
    }

    #[test]
    fn test_parse_operator_class() {
        let source = r#"
            program Test;
            operator MyClass.+(a, b: integer): integer;
            begin
                Result := a + b;
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                assert_eq!(block.operator_decls.len(), 1);
                if let Node::OperatorDecl(op) = &block.operator_decls[0] {
                    assert_eq!(op.operator_name, "+");
                    assert_eq!(op.class_name, Some("MyClass".to_string()));
                }
            }
        }
    }

    #[test]
    fn test_parse_operator_named() {
        let source = r#"
            program Test;
            operator sub(a, b: integer): integer;
            begin
                Result := a - b;
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                assert_eq!(block.operator_decls.len(), 1);
                if let Node::OperatorDecl(op) = &block.operator_decls[0] {
                    assert_eq!(op.operator_name, "sub");
                    assert_eq!(op.class_name, None);
                }
            }
        }
    }

    #[test]
    fn test_parse_operator_forward() {
        let source = r#"
            program Test;
            operator +(a, b: integer): integer; forward;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::OperatorDecl(op) = &block.operator_decls[0] {
                    assert_eq!(op.operator_name, "+");
                    assert!(op.is_forward);
                    assert!(!op.is_external);
                }
            }
        }
    }

    #[test]
    fn test_parse_operator_external() {
        let source = r#"
            program Test;
            operator +(a, b: integer): integer; external 'add_func';
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::OperatorDecl(op) = &block.operator_decls[0] {
                    assert_eq!(op.operator_name, "+");
                    assert!(!op.is_forward);
                    assert!(op.is_external);
                    assert_eq!(op.external_name, Some("add_func".to_string()));
                }
            }
        }
    }

    #[test]
    fn test_parse_operator_multiple_symbols() {
        let source = r#"
            program Test;
            operator +(a, b: integer): integer;
            begin
            end;
            operator -(a, b: integer): integer;
            begin
            end;
            operator *(a, b: integer): integer;
            begin
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                assert_eq!(block.operator_decls.len(), 3);
                if let Node::OperatorDecl(op1) = &block.operator_decls[0] {
                    assert_eq!(op1.operator_name, "+");
                }
                if let Node::OperatorDecl(op2) = &block.operator_decls[1] {
                    assert_eq!(op2.operator_name, "-");
                }
                if let Node::OperatorDecl(op3) = &block.operator_decls[2] {
                    assert_eq!(op3.operator_name, "*");
                }
            }
        }
    }

    // ========== Advanced Declarations Tests ==========

    #[test]
    fn test_parse_threadvar() {
        let source = r#"
            program Test;
            threadvar
                x: integer;
                y, z: word;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                assert_eq!(block.threadvar_decls.len(), 2);
                if let Node::VarDecl(v1) = &block.threadvar_decls[0] {
                    assert_eq!(v1.names, vec!["x"]);
                }
                if let Node::VarDecl(v2) = &block.threadvar_decls[1] {
                    assert_eq!(v2.names, vec!["y", "z"]);
                }
            }
        }
    }

    #[test]
    fn test_parse_resourcestring() {
        let source = r#"
            program Test;
            resourcestring
                msg1 = 'Hello';
                msg2 = 'World';
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                // RESOURCESTRING constants are added to const_decls
                assert!(block.const_decls.len() >= 2);
                if let Node::ConstDecl(c) = &block.const_decls[0] {
                    assert_eq!(c.name, "msg1");
                    assert!(c.is_resourcestring);
                }
            }
        }
    }

    #[test]
    fn test_parse_constref_parameter() {
        let source = r#"
            program Test;
            procedure Proc(constref x: integer);
            begin
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.params.len(), 1);
                    assert_eq!(proc.params[0].param_type, ast::ParamType::ConstRef);
                }
            }
        }
    }

    #[test]
    fn test_parse_out_parameter() {
        let source = r#"
            program Test;
            procedure Proc(out x: integer);
            begin
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.params.len(), 1);
                    assert_eq!(proc.params[0].param_type, ast::ParamType::Out);
                }
            }
        }
    }

    #[test]
    fn test_parse_absolute_variable() {
        let source = r#"
            program Test;
            var
                StatusReg: byte absolute $8000;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                assert_eq!(block.var_decls.len(), 1);
                if let Node::VarDecl(v1) = &block.var_decls[0] {
                    assert_eq!(v1.names, vec!["StatusReg"]);
                    assert!(v1.absolute_address.is_some());
                }
            }
        }
    }

    #[test]
    fn test_parse_default_parameter() {
        let source = r#"
            program Test;
            procedure Proc(x: integer = 10; y: word = 20);
            begin
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.params.len(), 2);
                    assert!(proc.params[0].default_value.is_some());
                    assert!(proc.params[1].default_value.is_some());
                }
            }
        }
    }

    #[test]
    fn test_parse_mixed_parameter_modes() {
        let source = r#"
            program Test;
            procedure Proc(
                a: integer;
                var b: integer;
                const c: integer;
                constref d: integer;
                out e: integer
            );
            begin
            end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::ProcDecl(proc) = &block.proc_decls[0] {
                    assert_eq!(proc.params.len(), 5);
                    assert_eq!(proc.params[0].param_type, ast::ParamType::Value);
                    assert_eq!(proc.params[1].param_type, ast::ParamType::Var);
                    assert_eq!(proc.params[2].param_type, ast::ParamType::Const);
                    assert_eq!(proc.params[3].param_type, ast::ParamType::ConstRef);
                    assert_eq!(proc.params[4].param_type, ast::ParamType::Out);
                }
            }
        }
    }

    #[test]
    fn test_parse_class_var() {
        let source = r#"
            program Test;
            type
                TMyClass = class
                    class var SharedCounter: integer;
                    class var SharedName: string;
                end;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(prog)) = result {
            if let Node::Block(block) = &*prog.block {
                if let Node::TypeDecl(type_decl) = &block.type_decls[0] {
                    if let Node::ClassType(class_type) = &*type_decl.type_expr {
                        // Find class variable members
                        let class_var_members: Vec<_> = class_type.members.iter()
                            .filter_map(|(_, m)| {
                                if let ast::ClassMember::Field(field) = m {
                                    if let Node::VarDecl(var_decl) = field {
                                        if var_decl.is_class_var {
                                            Some(var_decl)
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                            .collect();
                        assert_eq!(class_var_members.len(), 2, "Should have 2 class variables");
                        assert_eq!(class_var_members[0].names, vec!["SharedCounter"]);
                        assert_eq!(class_var_members[1].names, vec!["SharedName"]);
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_with_ifdef_active() {
        let source = r#"
            {$DEFINE DEBUG}
            {$IFDEF DEBUG}
            program Test;
            var x: integer;
            begin
                x := 42;
            end.
            {$ENDIF}
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            assert_eq!(program.name, "Test");
        } else {
            panic!("Expected Program node");
        }
    }

    #[test]
    fn test_parse_with_ifdef_inactive() {
        let source = r#"
            {$IFDEF DEBUG}
            program Test;
            var x: integer;
            begin
                x := 42;
            end.
            {$ENDIF}
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        // Should fail because there's no PROGRAM when DEBUG is not defined
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_ifndef_active() {
        let source = r#"
            {$IFNDEF RELEASE}
            program Test;
            var x: integer;
            begin
                x := 42;
            end.
            {$ENDIF}
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            assert_eq!(program.name, "Test");
        } else {
            panic!("Expected Program node");
        }
    }

    #[test]
    fn test_parse_with_else_branch() {
        let source = r#"
            {$IFDEF DEBUG}
            program Test1;
            begin end.
            {$ELSE}
            program Test2;
            begin end.
            {$ENDIF}
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            // Should parse Test2 (ELSE branch) since DEBUG is not defined
            assert_eq!(program.name, "Test2");
        } else {
            panic!("Expected Program node");
        }
    }

    #[test]
    fn test_parse_with_define() {
        let source = r#"
            {$DEFINE DEBUG}
            {$IFDEF DEBUG}
            program Test;
            begin end.
            {$ENDIF}
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            assert_eq!(program.name, "Test");
        } else {
            panic!("Expected Program node");
        }
    }

    #[test]
    fn test_parse_with_predefined_symbols() {
        let source = r#"
            {$IFDEF DEBUG}
            program Test;
            begin end.
            {$ENDIF}
        "#;
        let mut parser = Parser::new_with_file_and_symbols(
            source,
            None,
            vec!["DEBUG".to_string()],
        ).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            assert_eq!(program.name, "Test");
        } else {
            panic!("Expected Program node");
        }
    }

    #[test]
    fn test_parse_include_directive() {
        use std::fs;
        use std::path::Path;
        
        // Create a unique temporary include directory for this test
        let include_dir = Path::new("test_includes_directive");
        // Ensure directory exists, ignore error if it already exists
        let _ = fs::create_dir_all(include_dir);
        let include_file = include_dir.join("test_header.pas");
        fs::write(&include_file, "const TestConst = 42;\n")
            .expect("Failed to write include file");
        
        let source = r#"
            program Test;
            {$INCLUDE 'test_includes_directive/test_header.pas'}
            begin end.
        "#;
        
        let mut parser = Parser::new_with_file_and_symbols(
            source,
            Some("test_main.pas".to_string()),
            vec![],
        ).unwrap();
        parser.include_paths.push(".".to_string());
        
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            assert_eq!(program.name, "Test");
            // Check that the included constant is in the block
            if let Node::Block(block) = program.block.as_ref() {
                assert!(!block.const_decls.is_empty(), "Should have included constant declaration");
            }
        } else {
            panic!("Expected Program node, got: {:?}", result);
        }
        
        // Cleanup
        fs::remove_file(&include_file).ok();
        fs::remove_dir(include_dir).ok();
    }

    #[test]
    fn test_parse_include_with_quotes() {
        use std::fs;
        use std::path::Path;
        
        let include_dir = Path::new("test_includes_quotes");
        let _ = fs::create_dir_all(include_dir);
        let include_file = include_dir.join("utils.pas");
        fs::write(&include_file, "var GlobalVar: integer;\n")
            .expect("Failed to write include file");
        
        let source = r#"
            {$INCLUDE "test_includes_quotes/utils.pas"}
            program Test;
            begin end.
        "#;
        
        let mut parser = Parser::new_with_file_and_symbols(
            source,
            Some("test_main.pas".to_string()),
            vec![],
        ).unwrap();
        parser.include_paths.push(".".to_string());
        
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        // Cleanup
        let _ = fs::remove_file(&include_file);
        let _ = fs::remove_dir(include_dir);
    }

    #[test]
    fn test_parse_include_circular_detection() {
        use std::fs;
        use std::path::Path;
        
        let include_dir = Path::new("test_includes_circular");
        let _ = fs::create_dir_all(include_dir);
        let include_file1 = include_dir.join("file1.pas");
        let include_file2 = include_dir.join("file2.pas");
        
        // file1 includes file2
        fs::write(&include_file1, "{$INCLUDE 'test_includes_circular/file2.pas'}\n")
            .expect("Failed to write include file1");
        // file2 includes file1 (circular)
        fs::write(&include_file2, "{$INCLUDE 'test_includes_circular/file1.pas'}\n")
            .expect("Failed to write include file2");
        
        let source = r#"
            {$INCLUDE 'test_includes_circular/file1.pas'}
            program Test;
            begin end.
        "#;
        
        let mut parser = Parser::new_with_file_and_symbols(
            source,
            Some("test_main.pas".to_string()),
            vec![],
        ).unwrap();
        parser.include_paths.push(".".to_string());
        
        let result = parser.parse();
        // Should detect circular include and return an error
        assert!(result.is_err(), "Should detect circular include");
        
        if let Err(e) = result {
            assert!(format!("{:?}", e).contains("circular") || format!("{:?}", e).contains("Circular"), 
                "Error should mention circular include: {:?}", e);
        }
        
        // Cleanup
        let _ = fs::remove_file(&include_file1);
        let _ = fs::remove_file(&include_file2);
        let _ = fs::remove_dir(include_dir);
    }

    #[test]
    fn test_parse_include_with_symbols() {
        use std::fs;
        use std::path::Path;
        
        let include_dir = Path::new("test_includes_symbols");
        let _ = fs::create_dir_all(include_dir);
        let include_file = include_dir.join("config.pas");
        // Simple include file - conditional compilation in includes is tested elsewhere
        fs::write(&include_file, "const ConfigValue = 100;\n")
            .expect("Failed to write include file");
        
        let source = r#"
            program Test;
            {$INCLUDE 'test_includes_symbols/config.pas'}
            begin end.
        "#;
        
        let mut parser = Parser::new_with_file_and_symbols(
            source,
            Some("test_main.pas".to_string()),
            vec!["DEBUG".to_string()], // Predefine symbols (not used in this simple test)
        ).unwrap();
        parser.include_paths.push(".".to_string());
        
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        // Cleanup
        let _ = fs::remove_file(&include_file);
        let _ = fs::remove_dir(include_dir);
    }

    #[test]
    fn test_parse_include_nested() {
        use std::fs;
        use std::path::Path;
        
        let include_dir = Path::new("test_includes_nested");
        let _ = fs::create_dir_all(include_dir);
        let include_file1 = include_dir.join("header1.pas");
        let include_file2 = include_dir.join("header2.pas");
        
        fs::write(&include_file1, "const Const1 = 1;\n{$INCLUDE 'test_includes_nested/header2.pas'}\n")
            .expect("Failed to write include file1");
        fs::write(&include_file2, "const Const2 = 2;\n")
            .expect("Failed to write include file2");
        
        let source = r#"
            {$INCLUDE 'test_includes_nested/header1.pas'}
            program Test;
            begin end.
        "#;
        
        let mut parser = Parser::new_with_file_and_symbols(
            source,
            Some("test_main.pas".to_string()),
            vec![],
        ).unwrap();
        parser.include_paths.push(".".to_string());
        
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        // Cleanup
        let _ = fs::remove_file(&include_file1);
        let _ = fs::remove_file(&include_file2);
        let _ = fs::remove_dir(include_dir);
    }
}
