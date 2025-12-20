//! Declaration parsing
//!
//! This module handles parsing of variable, constant, type, procedure, and function declarations.

use ast;
use ast::Node;
use errors::{ParserError, ParserResult};
use tokens::{Span, TokenKind};

/// Declaration parsing functionality
impl super::Parser {
    /// Parse program: PROGRAM identifier ; block .
    pub(super) fn parse_program(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

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
        let block = self.parse_block()?;

        // Period
        self.consume(TokenKind::Dot, ".")?;

        // Check for EOF (allow whitespace/comments after period)
        // Skip any remaining tokens that are just whitespace/comments
        while let Some(token) = self.current() {
            // If we see EOF, we're done
            if matches!(token.kind, TokenKind::Eof) {
                break;
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
            block: Box::new(block),
            span,
        }))
    }

    /// Parse block: [declarations] BEGIN statements END
    pub(crate) fn parse_block(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        let mut const_decls = vec![];
        let mut type_decls = vec![];
        let mut var_decls = vec![];
        let mut proc_decls = vec![];
        let mut func_decls = vec![];

        // Parse declarations (const, type, var, procedures, functions)
        loop {
            if self.check(&TokenKind::KwConst) {
                const_decls.extend(self.parse_const_decls()?);
            } else if self.check(&TokenKind::KwType) {
                type_decls.extend(self.parse_type_decls()?);
            } else if self.check(&TokenKind::KwVar) {
                var_decls.extend(self.parse_var_decls()?);
            } else if self.check(&TokenKind::KwProcedure) {
                proc_decls.push(self.parse_procedure_decl()?);
            } else if self.check(&TokenKind::KwFunction) {
                func_decls.push(self.parse_function_decl()?);
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
            const_decls,
            type_decls,
            var_decls,
            proc_decls,
            func_decls,
            statements,
            span,
        }))
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
            if !self.check(&TokenKind::Identifier(String::new())) {
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
            span,
        }))
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
            if !self.check(&TokenKind::Identifier(String::new())) {
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

        self.consume(TokenKind::Equal, "=")?;
        let type_expr = self.parse_type()?;

        let span = start_span.merge(type_expr.span());
        Ok(Node::TypeDecl(ast::TypeDecl {
            name,
            type_expr: Box::new(type_expr),
            span,
        }))
    }

    /// Parse variable declarations: VAR var_decl { ; var_decl }
    pub(crate) fn parse_var_decls(&mut self) -> ParserResult<Vec<Node>> {
        self.consume(TokenKind::KwVar, "VAR")?;
        let mut decls = vec![];
        loop {
            decls.push(self.parse_var_decl()?);
            if !self.check(&TokenKind::Semicolon) {
                break;
            }
            self.advance()?;
            if !self.check(&TokenKind::Identifier(String::new())) {
                break;
            }
        }
        Ok(decls)
    }

    /// Parse single variable declaration: identifier_list : type
    fn parse_var_decl(&mut self) -> ParserResult<Node> {
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

        let span = start_span.merge(type_expr.span());
        Ok(Node::VarDecl(ast::VarDecl {
            names,
            type_expr: Box::new(type_expr),
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

        let params = if self.check(&TokenKind::LeftParen) {
            self.parse_params()?
        } else {
            vec![]
        };

        self.consume(TokenKind::Semicolon, ";")?;

        // Create an empty block for forward declarations
        let empty_block = Node::Block(ast::Block {
            const_decls: vec![],
            type_decls: vec![],
            var_decls: vec![],
            proc_decls: vec![],
            func_decls: vec![],
            statements: vec![],
            span: start_span,
        });

        let span = start_span;
        Ok(Node::ProcDecl(ast::ProcDecl {
            name,
            class_name,
            params,
            block: Box::new(empty_block),
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
            const_decls: vec![],
            type_decls: vec![],
            var_decls: vec![],
            proc_decls: vec![],
            func_decls: vec![],
            statements: vec![],
            span: start_span,
        });

        let span = start_span.merge(return_type.span());
        Ok(Node::FuncDecl(ast::FuncDecl {
            name,
            class_name,
            params,
            return_type: Box::new(return_type),
            block: Box::new(empty_block),
            span,
        }))
    }

    /// Parse procedure declaration: PROCEDURE [ClassName.]identifier [ ( params ) ] ; block ;
    pub(crate) fn parse_procedure_decl(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        self.consume(TokenKind::KwProcedure, "PROCEDURE")?;

        // Parse method name: ClassName.MethodName or just MethodName
        let (class_name, name) = self.parse_qualified_name()?;

        let params = if self.check(&TokenKind::LeftParen) {
            self.parse_params()?
        } else {
            vec![]
        };

        self.consume(TokenKind::Semicolon, ";")?;
        let block = self.parse_block()?;
        self.consume(TokenKind::Semicolon, ";")?;

        let span = start_span.merge(block.span());
        Ok(Node::ProcDecl(ast::ProcDecl {
            name,
            class_name,
            params,
            block: Box::new(block),
            span,
        }))
    }

    /// Parse function declaration: FUNCTION [ClassName.]identifier [ ( params ) ] : type ; block ;
    pub(crate) fn parse_function_decl(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        self.consume(TokenKind::KwFunction, "FUNCTION")?;

        // Parse method name: ClassName.MethodName or just MethodName
        let (class_name, name) = self.parse_qualified_name()?;

        let params = if self.check(&TokenKind::LeftParen) {
            self.parse_params()?
        } else {
            vec![]
        };

        self.consume(TokenKind::Colon, ":")?;
        let return_type = self.parse_type()?;
        self.consume(TokenKind::Semicolon, ";")?;
        let block = self.parse_block()?;
        self.consume(TokenKind::Semicolon, ";")?;

        let span = start_span.merge(block.span());
        Ok(Node::FuncDecl(ast::FuncDecl {
            name,
            class_name,
            params,
            return_type: Box::new(return_type),
            block: Box::new(block),
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

    /// Parse parameter: [VAR | CONST] identifier_list : type
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

        let span = start_span.merge(type_expr.span());
        Ok(ast::Param {
            names,
            param_type,
            type_expr: Box::new(type_expr),
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
}
