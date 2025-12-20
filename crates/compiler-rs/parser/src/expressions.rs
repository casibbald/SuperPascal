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
            Some(TokenKind::LeftParen) => {
                self.advance()?;
                let expr = self.parse_expression()?;
                self.consume(TokenKind::RightParen, ")")?;
                Ok(expr)
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
            _ => None,
        }
    }

    /// Get operator precedence
    fn get_precedence(&self, op: &ast::BinaryOp) -> u8 {
        match op {
            // Logical operators (lowest precedence)
            ast::BinaryOp::Or => 1,
            ast::BinaryOp::And => 2,
            // Relational operators
            ast::BinaryOp::Equal | ast::BinaryOp::NotEqual | ast::BinaryOp::Less
            | ast::BinaryOp::LessEqual | ast::BinaryOp::Greater | ast::BinaryOp::GreaterEqual => 3,
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
