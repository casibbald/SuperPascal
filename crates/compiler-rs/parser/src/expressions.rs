//! Expression parsing
//!
//! This module handles parsing of expressions using a Pratt parser.

use ast;
use ast::Node;
use errors::{ParserError, ParserResult};
use tokens::{Span, TokenKind};

/// Expression parsing functionality
impl super::Parser {
    /// Parse expression (using Pratt parser for precedence)
    pub(super) fn parse_expression(&mut self) -> ParserResult<Node> {
        self.parse_expression_precedence(0)
    }

    /// Parse expression with precedence (Pratt parser)
    fn parse_expression_precedence(&mut self, min_precedence: u8) -> ParserResult<Node> {
        // Parse left operand (prefix)
        let mut left = self.parse_prefix()?;

        // Parse binary operators (infix)
        while let Some(op) = self.parse_binary_operator() {
            let precedence = self.get_precedence(&op);
            if precedence < min_precedence {
                break;
            }
            self.advance()?;
            let right = self.parse_expression_precedence(precedence + 1)?;
            let span = left.span().merge(right.span());
            left = Node::BinaryExpr(ast::BinaryExpr {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            });
        }

        Ok(left)
    }

    /// Parse prefix expression (unary operators, literals, identifiers, etc.)
    fn parse_prefix(&mut self) -> ParserResult<Node> {
        let start_span = self
            .current()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::at(0, 1, 1));

        let token_kind = self.current().map(|t| t.kind.clone());
        match token_kind.as_ref() {
            Some(TokenKind::IntegerLiteral { value, .. }) => {
                let token = self.current().unwrap().clone();
                let value = *value;
                self.advance()?;
                Ok(Node::LiteralExpr(ast::LiteralExpr {
                    value: ast::LiteralValue::Integer(value),
                    span: token.span,
                }))
            }
            Some(TokenKind::CharLiteral(value)) => {
                let token = self.current().unwrap().clone();
                let value = *value;
                self.advance()?;
                Ok(Node::LiteralExpr(ast::LiteralExpr {
                    value: ast::LiteralValue::Char(value),
                    span: token.span,
                }))
            }
            Some(TokenKind::StringLiteral(value)) => {
                let token = self.current().unwrap().clone();
                let value_clone = value.clone();
                self.advance()?;
                Ok(Node::LiteralExpr(ast::LiteralExpr {
                    value: ast::LiteralValue::String(value_clone),
                    span: token.span,
                }))
            }
            Some(TokenKind::BooleanLiteral(value)) => {
                let token = self.current().unwrap().clone();
                let value = *value;
                self.advance()?;
                Ok(Node::LiteralExpr(ast::LiteralExpr {
                    value: ast::LiteralValue::Boolean(value),
                    span: token.span,
                }))
            }
            Some(TokenKind::Plus) => {
                self.advance()?;
                let expr = self.parse_prefix()?;
                let span = start_span.merge(expr.span());
                Ok(Node::UnaryExpr(ast::UnaryExpr {
                    op: ast::UnaryOp::Plus,
                    expr: Box::new(expr),
                    span,
                }))
            }
            Some(TokenKind::Minus) => {
                self.advance()?;
                let expr = self.parse_prefix()?;
                let span = start_span.merge(expr.span());
                Ok(Node::UnaryExpr(ast::UnaryExpr {
                    op: ast::UnaryOp::Minus,
                    expr: Box::new(expr),
                    span,
                }))
            }
            Some(TokenKind::KwNot) => {
                self.advance()?;
                let expr = self.parse_prefix()?;
                let span = start_span.merge(expr.span());
                Ok(Node::UnaryExpr(ast::UnaryExpr {
                    op: ast::UnaryOp::Not,
                    expr: Box::new(expr),
                    span,
                }))
            }
            Some(TokenKind::At) => {
                // Address-of operator: @variable
                self.advance()?; // consume @
                let target = self.parse_prefix()?;
                let span = start_span.merge(target.span());
                Ok(Node::AddressOfExpr(ast::AddressOfExpr {
                    target: Box::new(target),
                    span,
                }))
            }
            Some(TokenKind::KwInherited) => {
                // INHERITED [method_name] [args]
                self.advance()?; // consume INHERITED
                let method_name = if matches!(self.current().map(|t| &t.kind), Some(TokenKind::Identifier(_))) {
                    let name_token = self.current().unwrap().clone();
                    let name = match &name_token.kind {
                        TokenKind::Identifier(name) => name.clone(),
                        _ => unreachable!(),
                    };
                    self.advance()?;
                    Some(name)
                } else {
                    None
                };
                
                let args = if self.check(&TokenKind::LeftParen) {
                    self.parse_args()?
                } else {
                    vec![]
                };
                
                let span = if let Some(ref last_arg) = args.last() {
                    start_span.merge(last_arg.span())
                } else {
                    start_span
                };
                
                Ok(Node::InheritedExpr(ast::InheritedExpr {
                    method_name,
                    args,
                    span,
                }))
            }
            Some(TokenKind::LeftParen) => {
                self.advance()?;
                let expr = self.parse_expression()?;
                self.consume(TokenKind::RightParen, ")")?;
                Ok(expr)
            }
            Some(TokenKind::LeftBracket) => {
                // Set literal: [element1, element2, ...] or [element1..element2]
                self.advance()?; // consume [
                let mut elements = vec![];
                
                if !self.check(&TokenKind::RightBracket) {
                    loop {
                        let start_expr = self.parse_expression()?;
                        
                        // Check for range: element1..element2
                        if self.check(&TokenKind::DotDot) {
                            self.advance()?; // consume ..
                            let end_expr = self.parse_expression()?;
                            elements.push(ast::SetElement::Range {
                                start: Box::new(start_expr),
                                end: Box::new(end_expr),
                            });
                        } else {
                            // Single value
                            elements.push(ast::SetElement::Value(Box::new(start_expr)));
                        }
                        
                        if !self.check(&TokenKind::Comma) {
                            break;
                        }
                        self.advance()?; // consume ,
                    }
                }
                
                let end_token = self.consume(TokenKind::RightBracket, "]")?;
                let span = start_span.merge(end_token.span);
                Ok(Node::SetLiteral(ast::SetLiteral {
                    elements,
                    span,
                }))
            }
            Some(TokenKind::KwFunction) => {
                // Anonymous function: function(params): return_type begin ... end
                let start_token = self.current().unwrap().clone();
                self.advance()?; // consume 'function'
                
                // Parse parameters (optional)
                let params = if self.check(&TokenKind::LeftParen) {
                    self.parse_params()?
                } else {
                    vec![]
                };
                
                // Parse return type (required for functions)
                self.consume(TokenKind::Colon, ":")?;
                let return_type = Box::new(self.parse_type()?);
                
                // Parse body (block)
                let block = Box::new(self.parse_block()?);
                
                let span = start_token.span.merge(block.span());
                Ok(Node::AnonymousFunction(ast::AnonymousFunction {
                    params,
                    return_type,
                    block,
                    span,
                }))
            }
            Some(TokenKind::KwProcedure) => {
                // Anonymous procedure: procedure(params) begin ... end
                let start_token = self.current().unwrap().clone();
                self.advance()?; // consume 'procedure'
                
                // Parse parameters (optional)
                let params = if self.check(&TokenKind::LeftParen) {
                    self.parse_params()?
                } else {
                    vec![]
                };
                
                // Parse body (block)
                let block = Box::new(self.parse_block()?);
                
                let span = start_token.span.merge(block.span());
                Ok(Node::AnonymousProcedure(ast::AnonymousProcedure {
                    params,
                    block,
                    span,
                }))
            }
            Some(TokenKind::Identifier(_)) => {
                // Could be identifier, function call, or array/record access
                let name_token = self.current().unwrap().clone();
                let name = match &name_token.kind {
                    TokenKind::Identifier(name) => name.clone(),
                    _ => unreachable!(),
                };
                self.advance()?;

                if self.check(&TokenKind::LeftParen) {
                    // Function call
                    let args = self.parse_args()?;
                    let span = if let Some(last_arg) = args.last() {
                        name_token.span.merge(last_arg.span())
                    } else {
                        name_token.span
                    };
                    Ok(Node::CallExpr(ast::CallExpr {
                        name,
                        args,
                        span,
                    }))
                } else {
                    // Start with identifier, then parse postfix (indexing, field access)
                    let mut expr: Node = Node::IdentExpr(ast::IdentExpr {
                        name,
                        span: name_token.span,
                    });
                    expr = self.parse_postfix(expr)?;
                    Ok(expr)
                }
            }
            _ => {
                let span = self
                    .current()
                    .map(|t| t.span)
                    .unwrap_or_else(|| Span::at(0, 1, 1));
                Err(ParserError::InvalidSyntax {
                    message: "Expected expression".to_string(),
                    span,
                })
            }
        }
    }

    /// Parse postfix (array indexing, field access, pointer dereference)
    fn parse_postfix(&mut self, mut expr: Node) -> ParserResult<Node> {
        loop {
            if self.check(&TokenKind::LeftBracket) {
                self.advance()?;
                let index = self.parse_expression()?;
                self.consume(TokenKind::RightBracket, "]")?;
                let span = expr.span().merge(index.span());
                expr = Node::IndexExpr(ast::IndexExpr {
                    array: Box::new(expr),
                    index: Box::new(index),
                    span,
                });
            } else if self.check(&TokenKind::Dot) {
                self.advance()?;
                let field_token = self.consume(TokenKind::Identifier(String::new()), "identifier")?;
                let field = match &field_token.kind {
                    TokenKind::Identifier(name) => name.clone(),
                    _ => return Err(ParserError::InvalidSyntax {
                        message: "Expected identifier".to_string(),
                        span: field_token.span,
                    }),
                };
                let span = expr.span().merge(field_token.span);
                expr = Node::FieldExpr(ast::FieldExpr {
                    record: Box::new(expr),
                    field,
                    span,
                });
            } else if self.check(&TokenKind::Caret) {
                // Pointer dereference: expr^
                self.advance()?; // consume ^
                let caret_span = self
                    .current()
                    .map(|t| t.span)
                    .unwrap_or_else(|| Span::at(0, 1, 1));
                let span = expr.span().merge(caret_span);
                expr = Node::DerefExpr(ast::DerefExpr {
                    pointer: Box::new(expr),
                    span,
                });
            } else {
                break;
            }
        }
        Ok(expr)
    }

    /// Parse binary operator (if present)
    fn parse_binary_operator(&self) -> Option<ast::BinaryOp> {
        match self.current().map(|t| &t.kind) {
            Some(TokenKind::Plus) => Some(ast::BinaryOp::Add),
            Some(TokenKind::Minus) => Some(ast::BinaryOp::Subtract),
            Some(TokenKind::Star) => Some(ast::BinaryOp::Multiply),
            Some(TokenKind::Slash) => Some(ast::BinaryOp::Divide),
            Some(TokenKind::KwDiv) => Some(ast::BinaryOp::Div),
            Some(TokenKind::KwMod) => Some(ast::BinaryOp::Mod),
            Some(TokenKind::Equal) => Some(ast::BinaryOp::Equal),
            Some(TokenKind::NotEqual) => Some(ast::BinaryOp::NotEqual),
            Some(TokenKind::Less) => Some(ast::BinaryOp::Less),
            Some(TokenKind::LessEqual) => Some(ast::BinaryOp::LessEqual),
            Some(TokenKind::Greater) => Some(ast::BinaryOp::Greater),
            Some(TokenKind::GreaterEqual) => Some(ast::BinaryOp::GreaterEqual),
            Some(TokenKind::KwAnd) => Some(ast::BinaryOp::And),
            Some(TokenKind::KwOr) => Some(ast::BinaryOp::Or),
            Some(TokenKind::KwIn) => Some(ast::BinaryOp::In),
            Some(TokenKind::KwIs) => Some(ast::BinaryOp::Is),
            Some(TokenKind::KwAs) => Some(ast::BinaryOp::As),
            _ => None,
        }
    }

    /// Get operator precedence
    fn get_precedence(&self, op: &ast::BinaryOp) -> u8 {
        match op {
            // Logical operators (lowest precedence)
            ast::BinaryOp::Or => 1,
            ast::BinaryOp::And => 2,
            // Relational operators (including set membership and type operations)
            ast::BinaryOp::Equal | ast::BinaryOp::NotEqual | ast::BinaryOp::Less
            | ast::BinaryOp::LessEqual | ast::BinaryOp::Greater | ast::BinaryOp::GreaterEqual
            | ast::BinaryOp::In | ast::BinaryOp::Is | ast::BinaryOp::As => 3,
            // Additive operators
            ast::BinaryOp::Add | ast::BinaryOp::Subtract => 4,
            // Multiplicative operators (highest precedence)
            ast::BinaryOp::Multiply | ast::BinaryOp::Divide | ast::BinaryOp::Div | ast::BinaryOp::Mod => 5,
        }
    }

    /// Parse argument list: ( expression { , expression } )
    pub(crate) fn parse_args(&mut self) -> ParserResult<Vec<Node>> {
        self.consume(TokenKind::LeftParen, "(")?;
        let mut args = vec![];

        if !self.check(&TokenKind::RightParen) {
            loop {
                args.push(self.parse_expression()?);
                if !self.check(&TokenKind::Comma) {
                    break;
                }
                self.advance()?;
            }
        }

        self.consume(TokenKind::RightParen, ")")?;
        Ok(args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Parser;
    use ast::{self, Node};

    // ===== Set Literal Tests =====

    #[test]
    fn test_parse_set_literal_simple() {
        let source = r#"
            program Test;
            begin
                x := [1, 2, 3];
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::AssignStmt(assign) = &block.statements[0] {
                    if let Node::SetLiteral(set_lit) = assign.value.as_ref() {
                        assert_eq!(set_lit.elements.len(), 3);
                        // Check first element
                        if let ast::SetElement::Value(value) = &set_lit.elements[0] {
                            if let Node::LiteralExpr(lit) = value.as_ref() {
                                if let ast::LiteralValue::Integer(v) = lit.value {
                                    assert_eq!(v, 1);
                                } else {
                                    panic!("Expected Integer literal");
                                }
                            }
                        } else {
                            panic!("Expected Value element");
                        }
                    } else {
                        panic!("Expected SetLiteral");
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_set_literal_with_range() {
        let source = r#"
            program Test;
            begin
                x := [1..5, 10];
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::AssignStmt(assign) = &block.statements[0] {
                    if let Node::SetLiteral(set_lit) = assign.value.as_ref() {
                        assert_eq!(set_lit.elements.len(), 2);
                        // Check first element is a range
                        if let ast::SetElement::Range { start, end } = &set_lit.elements[0] {
                            if let Node::LiteralExpr(lit) = start.as_ref() {
                                if let ast::LiteralValue::Integer(v) = lit.value {
                                    assert_eq!(v, 1);
                                }
                            }
                            if let Node::LiteralExpr(lit) = end.as_ref() {
                                if let ast::LiteralValue::Integer(v) = lit.value {
                                    assert_eq!(v, 5);
                                }
                            }
                        } else {
                            panic!("Expected Range element");
                        }
                        // Check second element is a value
                        if let ast::SetElement::Value(value) = &set_lit.elements[1] {
                            if let Node::LiteralExpr(lit) = value.as_ref() {
                                if let ast::LiteralValue::Integer(v) = lit.value {
                                    assert_eq!(v, 10);
                                }
                            }
                        } else {
                            panic!("Expected Value element");
                        }
                    } else {
                        panic!("Expected SetLiteral");
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_set_literal_empty() {
        let source = r#"
            program Test;
            begin
                x := [];
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::AssignStmt(assign) = &block.statements[0] {
                    if let Node::SetLiteral(set_lit) = assign.value.as_ref() {
                        assert_eq!(set_lit.elements.len(), 0);
                    } else {
                        panic!("Expected SetLiteral");
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_in_operator() {
        let source = r#"
            program Test;
            begin
                if x in [1, 2, 3] then
                    writeln('Found');
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::IfStmt(if_stmt) = &block.statements[0] {
                    if let Node::BinaryExpr(bin_expr) = if_stmt.condition.as_ref() {
                        assert_eq!(bin_expr.op, ast::BinaryOp::In);
                        if let Node::IdentExpr(ident) = bin_expr.left.as_ref() {
                            assert_eq!(ident.name, "x");
                        } else {
                            panic!("Expected IdentExpr on left side of IN");
                        }
                        if let Node::SetLiteral(_) = bin_expr.right.as_ref() {
                            // OK - right side is a set literal
                        } else {
                            panic!("Expected SetLiteral on right side of IN");
                        }
                    } else {
                        panic!("Expected BinaryExpr with IN operator");
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_in_operator_with_set_variable() {
        let source = r#"
            program Test;
            var s: set of integer;
            begin
                if 5 in s then
                    writeln('Found');
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::IfStmt(if_stmt) = &block.statements[0] {
                    if let Node::BinaryExpr(bin_expr) = if_stmt.condition.as_ref() {
                        assert_eq!(bin_expr.op, ast::BinaryOp::In);
                        if let Node::LiteralExpr(lit) = bin_expr.left.as_ref() {
                            if let ast::LiteralValue::Integer(v) = lit.value {
                                assert_eq!(v, 5);
                            }
                        } else {
                            panic!("Expected LiteralExpr on left side of IN");
                        }
                        if let Node::IdentExpr(ident) = bin_expr.right.as_ref() {
                            assert_eq!(ident.name, "s");
                        } else {
                            panic!("Expected IdentExpr on right side of IN");
                        }
                    } else {
                        panic!("Expected BinaryExpr with IN operator");
                    }
                }
            }
        }
    }

    // ===== Advanced Expression Tests =====

    #[test]
    fn test_parse_inherited_without_method() {
        let source = r#"
            program Test;
            begin
                inherited;
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::CallStmt(call) = &block.statements[0] {
                    // INHERITED without method name is parsed as CallStmt with empty name
                    assert_eq!(call.name, "");
                    assert_eq!(call.args.len(), 0);
                } else {
                    panic!("Expected CallStmt for INHERITED statement");
                }
            }
        }
    }

    #[test]
    fn test_parse_inherited_with_method() {
        let source = r#"
            program Test;
            begin
                inherited DoSomething;
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::CallStmt(call) = &block.statements[0] {
                    assert_eq!(call.name, "DoSomething");
                    assert_eq!(call.args.len(), 0);
                } else {
                    panic!("Expected CallStmt for INHERITED statement");
                }
            }
        }
    }

    #[test]
    fn test_parse_inherited_with_args() {
        let source = r#"
            program Test;
            begin
                inherited DoSomething(x, y);
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::CallStmt(call) = &block.statements[0] {
                    assert_eq!(call.name, "DoSomething");
                    assert_eq!(call.args.len(), 2);
                } else {
                    panic!("Expected CallStmt for INHERITED statement");
                }
            }
        }
    }

    #[test]
    fn test_parse_address_of_operator() {
        let source = r#"
            program Test;
            var x: integer;
            begin
                writeln(@x);
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::CallStmt(call) = &block.statements[0] {
                    assert_eq!(call.args.len(), 1);
                    if let Node::AddressOfExpr(addr) = &call.args[0] {
                        if let Node::IdentExpr(ident) = addr.target.as_ref() {
                            assert_eq!(ident.name, "x");
                        } else {
                            panic!("Expected IdentExpr in address-of target");
                        }
                    } else {
                        panic!("Expected AddressOfExpr");
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_is_operator() {
        let source = r#"
            program Test;
            var obj: TObject;
            begin
                if obj is TMyClass then
                    writeln('Yes');
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::IfStmt(if_stmt) = &block.statements[0] {
                    if let Node::BinaryExpr(bin_expr) = if_stmt.condition.as_ref() {
                        assert_eq!(bin_expr.op, ast::BinaryOp::Is);
                        if let Node::IdentExpr(ident) = bin_expr.left.as_ref() {
                            assert_eq!(ident.name, "obj");
                        }
                        if let Node::IdentExpr(ident) = bin_expr.right.as_ref() {
                            assert_eq!(ident.name, "TMyClass");
                        }
                    } else {
                        panic!("Expected BinaryExpr with IS operator");
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_as_operator() {
        let source = r#"
            program Test;
            var obj: TObject;
            var result: TMyClass;
            begin
                result := obj as TMyClass;
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::AssignStmt(assign) = &block.statements[0] {
                    if let Node::BinaryExpr(bin_expr) = assign.value.as_ref() {
                        assert_eq!(bin_expr.op, ast::BinaryOp::As);
                        if let Node::IdentExpr(ident) = bin_expr.left.as_ref() {
                            assert_eq!(ident.name, "obj");
                        }
                        if let Node::IdentExpr(ident) = bin_expr.right.as_ref() {
                            assert_eq!(ident.name, "TMyClass");
                        }
                    } else {
                        panic!("Expected BinaryExpr with AS operator");
                    }
                } else {
                    panic!("Expected AssignStmt");
                }
            }
        }
    }

    #[test]
    fn test_parse_anonymous_procedure() {
        let source = r#"
            program Test;
            var a: procedure;
            begin
                a := procedure
                begin
                end;
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::AssignStmt(assign) = &block.statements[0] {
                    if let Node::AnonymousProcedure(anon_proc) = assign.value.as_ref() {
                        assert_eq!(anon_proc.params.len(), 0);
                    } else {
                        panic!("Expected AnonymousProcedure, got: {:?}", assign.value);
                    }
                } else {
                    panic!("Expected AssignStmt");
                }
            }
        }
    }

    #[test]
    fn test_parse_anonymous_function() {
        let source = r#"
            program Test;
            var b: function(a: integer; b: integer): integer;
            begin
                b := function(a: integer; b: integer): integer
                begin
                    Result := 42;
                end;
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::AssignStmt(assign) = &block.statements[0] {
                    if let Node::AnonymousFunction(anon_func) = assign.value.as_ref() {
                        assert_eq!(anon_func.params.len(), 2, "Expected 2 parameters, got {}", anon_func.params.len());
                        // Check return type
                        if let Node::NamedType(named_type) = anon_func.return_type.as_ref() {
                            assert_eq!(named_type.name, "integer");
                        } else {
                            panic!("Expected NamedType for return type");
                        }
                    } else {
                        panic!("Expected AnonymousFunction, got: {:?}", assign.value);
                    }
                } else {
                    panic!("Expected AssignStmt");
                }
            }
        }
    }

    #[test]
    fn test_parse_anonymous_function_immediate_call() {
        // Note: Immediate calls like (function...)(args) require special handling
        // For now, we'll test a simpler case where the function is assigned first
        let source = r#"
            program Test;
            var c: integer;
            var fn: function(a: integer): integer;
            begin
                fn := function(a: integer): integer
                begin
                    Result := a;
                end;
                c := fn(42);
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
    }

    #[test]
    fn test_parse_anonymous_procedure_with_params() {
        let source = r#"
            program Test;
            var proc: procedure(x: integer);
            begin
                proc := procedure(x: integer)
                begin
                    writeln(x);
                end;
            end.
        "#;
        let mut parser = Parser::new(source).unwrap();
        let result = parser.parse();
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        
        if let Ok(Node::Program(program)) = result {
            if let Node::Block(block) = program.block.as_ref() {
                if let Node::AssignStmt(assign) = &block.statements[0] {
                    if let Node::AnonymousProcedure(anon_proc) = assign.value.as_ref() {
                        assert_eq!(anon_proc.params.len(), 1);
                        assert_eq!(anon_proc.params[0].names[0], "x");
                    } else {
                        panic!("Expected AnonymousProcedure, got: {:?}", assign.value);
                    }
                }
            }
        }
    }
}
