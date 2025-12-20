//! Unit and module parsing
//!
//! This module handles parsing of units, libraries, interfaces, and implementations.

use ast;
use ast::Node;
use errors::{ParserError, ParserResult};
use tokens::{Span, TokenKind};

/// Unit parsing functionality
impl super::Parser {
    /// Parse unit: UNIT identifier ; [interface] [implementation] [initialization] [finalization] END .
    pub(crate) fn parse_unit(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        self.consume(TokenKind::KwUnit, "UNIT")?;

        // Parse unit name (can be qualified like "a.b.c")
        let name = self.parse_qualified_unit_name()?;

        self.consume(TokenKind::Semicolon, ";")?;

        // Parse interface section (optional but usually present)
        let interface = if self.check(&TokenKind::KwInterface) {
            Some(self.parse_interface_section()?)
        } else {
            None
        };

        // Parse implementation section (optional but usually present)
        let implementation = if self.check(&TokenKind::KwImplementation) {
            Some(self.parse_implementation_section()?)
        } else {
            None
        };

        // Parse initialization section (optional)
        let initialization = if self.check(&TokenKind::KwInitialization) {
            self.advance()?; // consume INITIALIZATION
            let block = self.parse_block()?;
            // Consume semicolon after initialization block
            if self.check(&TokenKind::Semicolon) {
                self.advance()?;
            }
            Some(Box::new(block))
        } else {
            None
        };

        // Parse finalization section (optional)
        let finalization = if self.check(&TokenKind::KwFinalization) {
            self.advance()?; // consume FINALIZATION
            let block = self.parse_block()?;
            // Consume semicolon after finalization block
            if self.check(&TokenKind::Semicolon) {
                self.advance()?;
            }
            Some(Box::new(block))
        } else {
            None
        };

        // END
        self.consume(TokenKind::KwEnd, "END")?;

        // Period
        self.consume(TokenKind::Dot, ".")?;

        let end_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));
        let span = start_span.merge(end_span);

        Ok(Node::Unit(ast::Unit {
            name,
            interface,
            implementation,
            initialization,
            finalization,
            span,
        }))
    }

    /// Parse library: LIBRARY identifier ; [block] END .
    pub(crate) fn parse_library(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        self.consume(TokenKind::KwLibrary, "LIBRARY")?;

        let name_token = self.consume(TokenKind::Identifier(String::new()), "identifier")?;
        let name = match &name_token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return Err(ParserError::InvalidSyntax {
                message: "Expected identifier after LIBRARY".to_string(),
                span: name_token.span,
            }),
        };

        self.consume(TokenKind::Semicolon, ";")?;

        // Optional block
        let block = if self.check(&TokenKind::KwBegin) {
            let parsed_block = self.parse_block()?;
            // Consume semicolon after block if present
            if self.check(&TokenKind::Semicolon) {
                self.advance()?;
            }
            Some(Box::new(parsed_block))
        } else {
            None
        };

        // END
        self.consume(TokenKind::KwEnd, "END")?;

        // Period
        self.consume(TokenKind::Dot, ".")?;

        let end_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));
        let span = start_span.merge(end_span);

        Ok(Node::Library(ast::Library {
            name,
            block,
            span,
        }))
    }

    /// Parse qualified unit name: identifier { . identifier }
    fn parse_qualified_unit_name(&mut self) -> ParserResult<String> {
        let mut parts = vec![];
        
        loop {
            let token = self.consume(TokenKind::Identifier(String::new()), "identifier")?;
            let part = match &token.kind {
                TokenKind::Identifier(name) => name.clone(),
                _ => return Err(ParserError::InvalidSyntax {
                    message: "Expected identifier".to_string(),
                    span: token.span,
                }),
            };
            parts.push(part);

            if !self.check(&TokenKind::Dot) {
                break;
            }
            self.advance()?; // consume the dot
        }

        Ok(parts.join("."))
    }

    /// Parse uses clause: USES identifier_list ;
    pub(crate) fn parse_uses_clause(&mut self) -> ParserResult<ast::UsesClause> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        self.consume(TokenKind::KwUses, "USES")?;

        let mut units = vec![];
        loop {
            let unit_name = self.parse_qualified_unit_name()?;
            units.push(unit_name);

            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance()?; // consume comma
        }

        self.consume(TokenKind::Semicolon, ";")?;

        let end_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));
        let span = start_span.merge(end_span);

        Ok(ast::UsesClause { units, span })
    }

    /// Parse interface section: INTERFACE [uses] [declarations]
    pub(crate) fn parse_interface_section(&mut self) -> ParserResult<ast::InterfaceSection> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        self.consume(TokenKind::KwInterface, "INTERFACE")?;

        // Optional uses clause
        let uses = if self.check(&TokenKind::KwUses) {
            Some(self.parse_uses_clause()?)
        } else {
            None
        };

        // Parse declarations (const, type, var, procedures, functions, properties)
        let mut const_decls = vec![];
        let mut type_decls = vec![];
        let mut var_decls = vec![];
        let mut proc_decls = vec![];
        let mut func_decls = vec![];
        let mut property_decls = vec![];

        loop {
            if self.check(&TokenKind::KwConst) {
                const_decls.extend(self.parse_const_decls()?);
            } else if self.check(&TokenKind::KwType) {
                type_decls.extend(self.parse_type_decls()?);
            } else if self.check(&TokenKind::KwVar) {
                var_decls.extend(self.parse_var_decls()?);
            } else if self.check(&TokenKind::KwProcedure) {
                proc_decls.push(self.parse_procedure_forward_decl()?);
            } else if self.check(&TokenKind::KwFunction) {
                func_decls.push(self.parse_function_forward_decl()?);
            } else if self.check(&TokenKind::KwProperty) {
                property_decls.push(super::properties::parse_property_decl(self)?);
            } else {
                break;
            }
        }

        let end_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));
        let span = start_span.merge(end_span);

        Ok(ast::InterfaceSection {
            uses,
            const_decls,
            type_decls,
            var_decls,
            proc_decls,
            func_decls,
            property_decls,
            span,
        })
    }

    /// Parse implementation section: IMPLEMENTATION [uses] [declarations]
    pub(crate) fn parse_implementation_section(&mut self) -> ParserResult<ast::ImplementationSection> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        self.consume(TokenKind::KwImplementation, "IMPLEMENTATION")?;

        // Optional uses clause
        let uses = if self.check(&TokenKind::KwUses) {
            Some(self.parse_uses_clause()?)
        } else {
            None
        };

        // Parse declarations (const, type, var, procedures, functions, properties)
        let mut const_decls = vec![];
        let mut type_decls = vec![];
        let mut var_decls = vec![];
        let mut proc_decls = vec![];
        let mut func_decls = vec![];
        let mut property_decls = vec![];

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
            } else if self.check(&TokenKind::KwProperty) {
                property_decls.push(super::properties::parse_property_decl(self)?);
            } else {
                break;
            }
        }

        let end_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));
        let span = start_span.merge(end_span);

        Ok(ast::ImplementationSection {
            uses,
            const_decls,
            type_decls,
            var_decls,
            proc_decls,
            func_decls,
            property_decls,
            span,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::Parser;
    use ast::Node;

    // ===== Unit/Module Tests =====

    #[test]
    fn test_parse_empty_unit() {
        let source = r#"
            unit TestUnit;
            interface
            implementation
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Unit(unit)) = result {
            assert_eq!(unit.name, "TestUnit");
            assert!(unit.interface.is_some());
            assert!(unit.implementation.is_some());
            assert!(unit.initialization.is_none());
            assert!(unit.finalization.is_none());
        } else {
            panic!("Expected Unit node");
        }
    }

    #[test]
    fn test_parse_unit_with_uses() {
        let source = r#"
            unit TestUnit;
            interface
            uses UnitA, UnitB;
            implementation
            uses UnitC;
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Unit(unit)) = result {
            if let Some(interface) = &unit.interface {
                assert!(interface.uses.is_some());
                if let Some(uses) = &interface.uses {
                    assert_eq!(uses.units.len(), 2);
                    assert_eq!(uses.units[0], "UnitA");
                    assert_eq!(uses.units[1], "UnitB");
                }
            }
            if let Some(implementation) = &unit.implementation {
                assert!(implementation.uses.is_some());
                if let Some(uses) = &implementation.uses {
                    assert_eq!(uses.units.len(), 1);
                    assert_eq!(uses.units[0], "UnitC");
                }
            }
        }
    }

    #[test]
    fn test_parse_unit_with_qualified_name() {
        let source = r#"
            unit MyNamespace.MyModule;
            interface
            implementation
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Unit(unit)) = result {
            assert_eq!(unit.name, "MyNamespace.MyModule");
        }
    }

    #[test]
    fn test_parse_unit_with_declarations() {
        let source = r#"
            unit TestUnit;
            interface
                const C = 10;
                type T = integer;
                procedure Proc;
                function Func: integer;
            implementation
                procedure Proc;
                begin
                end;
                function Func: integer;
                begin
                    Func := 42;
                end;
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Unit(unit)) = result {
            if let Some(interface) = &unit.interface {
                assert_eq!(interface.const_decls.len(), 1);
                assert_eq!(interface.type_decls.len(), 1);
                assert_eq!(interface.proc_decls.len(), 1);
                assert_eq!(interface.func_decls.len(), 1);
            }
            if let Some(implementation) = &unit.implementation {
                assert_eq!(implementation.proc_decls.len(), 1);
                assert_eq!(implementation.func_decls.len(), 1);
            }
        }
    }

    #[test]
    fn test_parse_unit_with_initialization_finalization() {
        let source = r#"
            unit TestUnit;
            interface
            implementation
            initialization
            begin
                writeln('Init');
            end;
            finalization
            begin
                writeln('Finalize');
            end;
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Unit(unit)) = result {
            assert!(unit.initialization.is_some());
            assert!(unit.finalization.is_some());
        }
    }

    #[test]
    fn test_parse_library() {
        let source = r#"
            library TestLib;
            begin
                writeln('Library');
            end;
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Library(lib)) = result {
            assert_eq!(lib.name, "TestLib");
            assert!(lib.block.is_some());
        } else {
            panic!("Expected Library node");
        }
    }

    #[test]
    fn test_parse_library_empty() {
        let source = r#"
            library TestLib;
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Library(lib)) = result {
            assert_eq!(lib.name, "TestLib");
            assert!(lib.block.is_none());
        }
    }

    #[test]
    fn test_parse_uses_qualified_names() {
        let source = r#"
            unit TestUnit;
            interface
            uses System.Classes, System.SysUtils, MyUnit;
            implementation
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Unit(unit)) = result {
            if let Some(interface) = &unit.interface {
                if let Some(uses) = &interface.uses {
                    assert_eq!(uses.units.len(), 3);
                    assert_eq!(uses.units[0], "System.Classes");
                    assert_eq!(uses.units[1], "System.SysUtils");
                    assert_eq!(uses.units[2], "MyUnit");
                }
            }
        }
    }
}
