//! Type parsing
//!
//! This module handles parsing of type declarations and type expressions.

use ast;
use ast::Node;
use errors::{ParserError, ParserResult};
use tokens::{Span, Token, TokenKind};

/// Type parsing functionality
impl super::Parser {
    /// Parse type: identifier | ^type | ARRAY [ index_type ] OF element_type | RECORD field_list END | CLASS ...
    pub(super) fn parse_type(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        // Check for pointer type: ^type
        if self.check(&TokenKind::Caret) {
            self.advance()?; // consume ^
            let base_type = self.parse_type()?; // Recursively parse the base type
            let span = start_span.merge(base_type.span());
            Ok(Node::PointerType(ast::PointerType {
                base_type: Box::new(base_type),
                span,
            }))
        } else if self.check(&TokenKind::KwArray) {
            self.advance()?;
            self.consume(TokenKind::LeftBracket, "[")?;
            let index_type = self.parse_type()?;
            self.consume(TokenKind::RightBracket, "]")?;
            self.consume(TokenKind::KwOf, "OF")?;
            let element_type = self.parse_type()?;
            let span = start_span.merge(element_type.span());
            Ok(Node::ArrayType(ast::ArrayType {
                index_type: Box::new(index_type),
                element_type: Box::new(element_type),
                span,
            }))
        } else if self.check(&TokenKind::KwRecord) {
            self.advance()?;
            let mut fields = vec![];
            while !self.check(&TokenKind::KwEnd) {
                fields.push(self.parse_field_decl()?);
                self.consume(TokenKind::Semicolon, ";")?;
            }
            let end_token = self.consume(TokenKind::KwEnd, "END")?;
            let span = start_span.merge(end_token.span);
            Ok(Node::RecordType(ast::RecordType {
                fields,
                span,
            }))
        } else if self.check(&TokenKind::KwClass) {
            // Class parsing is in classes.rs
            self.parse_class_type()
        } else {
            // Accept either identifier or primitive type keywords
            let name_token = if matches!(self.current().map(|t| &t.kind), Some(TokenKind::Identifier(_))) {
                self.consume(TokenKind::Identifier(String::new()), "type identifier")?
            } else if self.check(&TokenKind::KwInteger) {
                let token = self.current().unwrap().clone();
                self.advance()?;
                Token {
                    kind: TokenKind::Identifier("integer".to_string()),
                    span: token.span,
                }
            } else if self.check(&TokenKind::KwBoolean) {
                let token = self.current().unwrap().clone();
                self.advance()?;
                Token {
                    kind: TokenKind::Identifier("boolean".to_string()),
                    span: token.span,
                }
            } else if self.check(&TokenKind::KwChar) {
                let token = self.current().unwrap().clone();
                self.advance()?;
                Token {
                    kind: TokenKind::Identifier("char".to_string()),
                    span: token.span,
                }
            } else if self.check(&TokenKind::KwByte) {
                let token = self.current().unwrap().clone();
                self.advance()?;
                Token {
                    kind: TokenKind::Identifier("byte".to_string()),
                    span: token.span,
                }
            } else if self.check(&TokenKind::KwWord) {
                let token = self.current().unwrap().clone();
                self.advance()?;
                Token {
                    kind: TokenKind::Identifier("word".to_string()),
                    span: token.span,
                }
            } else {
                return Err(ParserError::InvalidSyntax {
                    message: format!(
                        "Expected type identifier, found: {:?}",
                        self.current().map(|t| &t.kind)
                    ),
                    span: self.current().map(|t| t.span).unwrap_or_else(|| Span::at(0, 1, 1)),
                });
            };
            
            let name = match &name_token.kind {
                TokenKind::Identifier(name) => name.clone(),
                _ => return Err(ParserError::InvalidSyntax {
                    message: "Expected type identifier".to_string(),
                    span: name_token.span,
                }),
            };
            Ok(Node::NamedType(ast::NamedType {
                name,
                span: name_token.span,
            }))
        }
    }

    /// Parse field declaration: identifier_list : type
    pub(super) fn parse_field_decl(&mut self) -> ParserResult<ast::FieldDecl> {
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
        Ok(ast::FieldDecl {
            names,
            type_expr: Box::new(type_expr),
            span,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::Parser;
    use ast::Node;

    // ===== Pointer Type Tests =====

    #[test]
    fn test_parse_pointer_type() {
        let source = r#"
            program Test;
            var p: ^integer;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::VarDecl(var_decl) = &block.var_decls[0] {
                    if let Node::PointerType(ptr_type) = var_decl.type_expr.as_ref() {
                        if let Node::NamedType(named) = ptr_type.base_type.as_ref() {
                            assert_eq!(named.name, "integer");
                        } else {
                            panic!("Expected NamedType in pointer base");
                        }
                    } else {
                        panic!("Expected PointerType");
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_pointer_to_pointer() {
        let source = r#"
            program Test;
            var p: ^^integer;
            begin
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::VarDecl(var_decl) = &block.var_decls[0] {
                    if let Node::PointerType(ptr_type) = var_decl.type_expr.as_ref() {
                        if let Node::PointerType(inner_ptr) = ptr_type.base_type.as_ref() {
                            if let Node::NamedType(named) = inner_ptr.base_type.as_ref() {
                                assert_eq!(named.name, "integer");
                            } else {
                                panic!("Expected NamedType in inner pointer base");
                            }
                        } else {
                            panic!("Expected inner PointerType");
                        }
                    } else {
                        panic!("Expected PointerType");
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_pointer_dereference() {
        let source = r#"
            program Test;
            var p: ^integer;
            begin
                p^ := 42;
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::AssignStmt(assign) = &block.statements[0] {
                    if let Node::DerefExpr(deref) = assign.target.as_ref() {
                        if let Node::IdentExpr(ident) = deref.pointer.as_ref() {
                            assert_eq!(ident.name, "p");
                        } else {
                            panic!("Expected IdentExpr in dereference");
                        }
                    } else {
                        panic!("Expected DerefExpr");
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_pointer_dereference_chain() {
        let source = r#"
            program Test;
            var p: ^^integer;
            begin
                p^^ := 42;
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::AssignStmt(assign) = &block.statements[0] {
                    if let Node::DerefExpr(outer_deref) = assign.target.as_ref() {
                        if let Node::DerefExpr(inner_deref) = outer_deref.pointer.as_ref() {
                            if let Node::IdentExpr(ident) = inner_deref.pointer.as_ref() {
                                assert_eq!(ident.name, "p");
                            } else {
                                panic!("Expected IdentExpr in inner dereference");
                            }
                        } else {
                            panic!("Expected inner DerefExpr");
                        }
                    } else {
                        panic!("Expected outer DerefExpr");
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_pointer_in_expression() {
        let source = r#"
            program Test;
            var p: ^integer;
            begin
                writeln(p^);
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::CallStmt(call) = &block.statements[0] {
                    assert_eq!(call.name, "writeln");
                    assert_eq!(call.args.len(), 1);
                    if let Node::DerefExpr(deref) = &call.args[0] {
                        if let Node::IdentExpr(ident) = deref.pointer.as_ref() {
                            assert_eq!(ident.name, "p");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_pointer_field_access() {
        let source = r#"
            program Test;
            type Rec = record
                x: integer;
            end;
            var p: ^Rec;
            begin
                p^.x := 10;
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::AssignStmt(assign) = &block.statements[0] {
                    if let Node::FieldExpr(field) = assign.target.as_ref() {
                        assert_eq!(field.field, "x");
                        if let Node::DerefExpr(deref) = field.record.as_ref() {
                            if let Node::IdentExpr(ident) = deref.pointer.as_ref() {
                                assert_eq!(ident.name, "p");
                            }
                        } else {
                            panic!("Expected DerefExpr in field access");
                        }
                    } else {
                        panic!("Expected FieldExpr");
                    }
                }
            }
        }
    }
}
