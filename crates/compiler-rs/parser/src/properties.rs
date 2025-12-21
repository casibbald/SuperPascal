//! Property parsing
//!
//! This module handles parsing of Object Pascal properties.

use ast;
use ast::Node;
use errors::{ParserError, ParserResult};
use tokens::{Span, TokenKind};

/// Parse property declaration: [CLASS] PROPERTY identifier [ [ index_params ] ] : type [ READ identifier ] [ WRITE identifier ] [ INDEX expr ] [ DEFAULT expr ] [ STORED expr ] [ ; default ]
pub(crate) fn parse_property_decl(parser: &mut super::Parser) -> ParserResult<Node> {
        let start_span = parser
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        // Check for CLASS keyword (class property)
        let is_class_property = if parser.check(&TokenKind::KwClass) {
            parser.advance()?; // consume CLASS
            true
        } else {
            false
        };

        parser.consume(TokenKind::KwProperty, "PROPERTY")?;

        let name_token = parser.consume(TokenKind::Identifier(String::new()), "identifier")?;
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return Err(ParserError::InvalidSyntax {
                message: "Expected identifier".to_string(),
                span: name_token.span,
            }),
        };

        // Optional index parameters: [ param1, param2: type; param3: type ]
        let mut index_params = vec![];
        if parser.check(&TokenKind::LeftBracket) {
            parser.advance()?; // consume [
            loop {
                index_params.push(parser.parse_param()?);
                if !parser.check(&TokenKind::Semicolon) {
                    break;
                }
                parser.advance()?; // consume semicolon
            }
            parser.consume(TokenKind::RightBracket, "]")?;
        }

        // Type
        parser.consume(TokenKind::Colon, ":")?;
        let property_type = parser.parse_type()?;

        // Optional READ accessor
        let read_accessor = if parser.check(&TokenKind::KwRead) {
            parser.advance()?; // consume READ
            let read_token = parser.consume(TokenKind::Identifier(String::new()), "identifier")?;
            match &read_token.kind {
                TokenKind::Identifier(name) => Some(name.clone()),
                _ => return Err(ParserError::InvalidSyntax {
                    message: "Expected identifier after READ".to_string(),
                    span: read_token.span,
                }),
            }
        } else {
            None
        };

        // Optional WRITE accessor
        let write_accessor = if parser.check(&TokenKind::KwWrite) {
            parser.advance()?; // consume WRITE
            let write_token = parser.consume(TokenKind::Identifier(String::new()), "identifier")?;
            match &write_token.kind {
                TokenKind::Identifier(name) => Some(name.clone()),
                _ => return Err(ParserError::InvalidSyntax {
                    message: "Expected identifier after WRITE".to_string(),
                    span: write_token.span,
                }),
            }
        } else {
            None
        };

        // Optional INDEX expression
        let index_expr = if parser.check(&TokenKind::KwIndex) {
            parser.advance()?; // consume INDEX
            Some(Box::new(parser.parse_expression()?))
        } else {
            None
        };

        // Optional DEFAULT expression
        let default_expr = if parser.check(&TokenKind::KwDefault) {
            parser.advance()?; // consume DEFAULT
            // Check if it's followed by an expression or just a semicolon (default;)
            if !parser.check(&TokenKind::Semicolon) {
                Some(Box::new(parser.parse_expression()?))
            } else {
                None
            }
        } else {
            None
        };

        // Optional STORED expression
        let stored_expr = if parser.check(&TokenKind::KwStored) {
            parser.advance()?; // consume STORED
            Some(Box::new(parser.parse_expression()?))
        } else {
            None
        };

        // Consume semicolon after property attributes
        parser.consume(TokenKind::Semicolon, ";")?;

        // Check for default; after semicolon - this marks it as a default property
        let is_default = if parser.check(&TokenKind::KwDefault) {
            parser.advance()?; // consume DEFAULT
            parser.consume(TokenKind::Semicolon, ";")?; // consume semicolon after default
            true
        } else {
            false
        };

        let end_span = parser
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));
        let span = start_span.merge(end_span);

        Ok(Node::PropertyDecl(ast::PropertyDecl {
            name,
            index_params,
            property_type: Box::new(property_type),
            read_accessor,
            write_accessor,
            index_expr,
            default_expr,
            stored_expr,
            is_default,
            is_class_property,
            span,
        }))
}

#[cfg(test)]
mod tests {
    use super::super::Parser;
    use ast::Node;

    // ===== Property Declaration Tests =====

    #[test]
    fn test_parse_property_simple() {
        let source = r#"
            unit TestUnit;
            interface
                property Name: string read FName write SetName;
            implementation
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Unit(unit)) = result {
            if let Some(interface) = &unit.interface {
                assert_eq!(interface.property_decls.len(), 1);
                if let Node::PropertyDecl(prop) = &interface.property_decls[0] {
                    assert_eq!(prop.name, "Name");
                    assert_eq!(prop.read_accessor, Some("FName".to_string()));
                    assert_eq!(prop.write_accessor, Some("SetName".to_string()));
                }
            }
        }
    }

    #[test]
    fn test_parse_property_read_only() {
        let source = r#"
            unit TestUnit;
            interface
                property Value: integer read FValue;
            implementation
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Unit(unit)) = result {
            if let Some(interface) = &unit.interface {
                if let Node::PropertyDecl(prop) = &interface.property_decls[0] {
                    assert_eq!(prop.read_accessor, Some("FValue".to_string()));
                    assert_eq!(prop.write_accessor, None);
                }
            }
        }
    }

    #[test]
    fn test_parse_property_with_index_params() {
        let source = r#"
            unit TestUnit;
            interface
                property Items[i: integer]: string read GetItem write SetItem;
            implementation
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Unit(unit)) = result {
            if let Some(interface) = &unit.interface {
                if let Node::PropertyDecl(prop) = &interface.property_decls[0] {
                    assert_eq!(prop.name, "Items");
                    assert_eq!(prop.index_params.len(), 1);
                }
            }
        }
    }

    #[test]
    fn test_parse_class_property() {
        let source = r#"
            program Test;
            type
                TMyClass = class
                    class property Count: integer read FCount;
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
                        // Find the property member
                        let prop_member = class_type.members.iter()
                            .find(|(_, m)| matches!(m, ast::ClassMember::Property(_)));
                        assert!(prop_member.is_some(), "Class property not found");
                        if let Some((_, ast::ClassMember::Property(prop))) = prop_member {
                            if let Node::PropertyDecl(prop_decl) = prop {
                                assert_eq!(prop_decl.name, "Count");
                                assert!(prop_decl.is_class_property, "Should be class property");
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_property_default_property() {
        let source = r#"
            unit TestUnit;
            interface
                property Items[i: integer]: string read GetItem write SetItem; default;
            implementation
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Unit(unit)) = result {
            if let Some(interface) = &unit.interface {
                if let Node::PropertyDecl(prop) = &interface.property_decls[0] {
                    assert!(prop.is_default);
                }
            }
        }
    }
}
