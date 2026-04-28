//! Constant folding and evaluation

use crate::SemanticAnalyzer;
use ast::Node;
use symbols::{ConstantValue, SymbolKind};

impl SemanticAnalyzer {
    /// Evaluate a constant expression, returning the constant value if the expression is constant.
    /// Returns None if the expression is not constant (contains variables, function calls, etc.).
    pub(crate) fn evaluate_constant_expression(&self, expr: &Node) -> Option<ConstantValue> {
        match expr {
            Node::LiteralExpr(lit) => match &lit.value {
                ast::LiteralValue::Integer(i) => Some(ConstantValue::Integer(*i as i16)),
                ast::LiteralValue::Boolean(b) => Some(ConstantValue::Boolean(*b)),
                ast::LiteralValue::Char(c) => Some(ConstantValue::Char(*c)),
                ast::LiteralValue::String(s) => Some(ConstantValue::String(s.clone())),
            },
            Node::IdentExpr(i) => {
                // Check if identifier is a constant
                if let Some(symbol) = self.core.symbol_table.lookup(&i.name) {
                    if let SymbolKind::Constant {
                        value: Some(cv), ..
                    } = &symbol.kind
                    {
                        Some(cv.clone())
                    } else {
                        None // Not a constant or constant value not yet computed
                    }
                } else {
                    None // Identifier not found
                }
            }
            Node::BinaryExpr(bin) => {
                // Evaluate both operands
                let left = self.evaluate_constant_expression(&bin.left)?;
                let right = self.evaluate_constant_expression(&bin.right)?;

                // Evaluate binary operation
                match bin.op {
                    ast::BinaryOp::Add => self.eval_add(&left, &right),
                    ast::BinaryOp::Subtract => self.eval_subtract(&left, &right),
                    ast::BinaryOp::Multiply => self.eval_multiply(&left, &right),
                    ast::BinaryOp::Divide | ast::BinaryOp::Div => self.eval_divide(&left, &right),
                    ast::BinaryOp::Mod => self.eval_mod(&left, &right),
                    ast::BinaryOp::Equal => Some(ConstantValue::Boolean(left == right)),
                    ast::BinaryOp::NotEqual => Some(ConstantValue::Boolean(left != right)),
                    ast::BinaryOp::Less => self.eval_less(&left, &right),
                    ast::BinaryOp::LessEqual => self.eval_less_equal(&left, &right),
                    ast::BinaryOp::Greater => self.eval_greater(&left, &right),
                    ast::BinaryOp::GreaterEqual => self.eval_greater_equal(&left, &right),
                    ast::BinaryOp::And => self.eval_and(&left, &right),
                    ast::BinaryOp::Or => self.eval_or(&left, &right),
                    ast::BinaryOp::In => {
                        // Set membership: IN operator evaluation not yet implemented for constant expressions
                        None
                    }
                    ast::BinaryOp::Is => {
                        // Type checking: IS operator evaluation not yet implemented for constant expressions
                        None
                    }
                    ast::BinaryOp::As => {
                        // Type casting: AS operator evaluation not yet implemented for constant expressions
                        None
                    }
                }
            }
            Node::UnaryExpr(unary) => {
                let operand = self.evaluate_constant_expression(&unary.expr)?;
                match unary.op {
                    ast::UnaryOp::Plus => Some(operand), // Unary plus is no-op
                    ast::UnaryOp::Minus => self.eval_unary_minus(&operand),
                    ast::UnaryOp::Not => self.eval_not(&operand),
                    ast::UnaryOp::AddressOf => {
                        // Address-of operator: @variable
                        // Cannot be evaluated at compile time
                        None
                    }
                }
            }
            _ => None, // Not a constant expression
        }
    }

    // Helper functions for constant evaluation
    pub(crate) fn eval_add(
        &self,
        left: &ConstantValue,
        right: &ConstantValue,
    ) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(l), ConstantValue::Integer(r)) => {
                Some(ConstantValue::Integer(l.saturating_add(*r)))
            }
            (ConstantValue::Byte(l), ConstantValue::Byte(r)) => {
                Some(ConstantValue::Byte(l.saturating_add(*r)))
            }
            (ConstantValue::Word(l), ConstantValue::Word(r)) => {
                Some(ConstantValue::Word(l.saturating_add(*r)))
            }
            _ => None,
        }
    }

    pub(crate) fn eval_subtract(
        &self,
        left: &ConstantValue,
        right: &ConstantValue,
    ) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(l), ConstantValue::Integer(r)) => {
                Some(ConstantValue::Integer(l.saturating_sub(*r)))
            }
            (ConstantValue::Byte(l), ConstantValue::Byte(r)) => {
                Some(ConstantValue::Byte(l.saturating_sub(*r)))
            }
            (ConstantValue::Word(l), ConstantValue::Word(r)) => {
                Some(ConstantValue::Word(l.saturating_sub(*r)))
            }
            _ => None,
        }
    }

    pub(crate) fn eval_multiply(
        &self,
        left: &ConstantValue,
        right: &ConstantValue,
    ) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(l), ConstantValue::Integer(r)) => {
                Some(ConstantValue::Integer(l.saturating_mul(*r)))
            }
            (ConstantValue::Byte(l), ConstantValue::Byte(r)) => {
                Some(ConstantValue::Byte(l.saturating_mul(*r)))
            }
            (ConstantValue::Word(l), ConstantValue::Word(r)) => {
                Some(ConstantValue::Word(l.saturating_mul(*r)))
            }
            _ => None,
        }
    }

    pub(crate) fn eval_divide(
        &self,
        left: &ConstantValue,
        right: &ConstantValue,
    ) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(l), ConstantValue::Integer(r)) => {
                if *r == 0 {
                    None // Division by zero
                } else {
                    Some(ConstantValue::Integer(l / r))
                }
            }
            (ConstantValue::Word(l), ConstantValue::Word(r)) => {
                if *r == 0 {
                    None
                } else {
                    Some(ConstantValue::Word(l / r))
                }
            }
            _ => None,
        }
    }

    pub(crate) fn eval_mod(
        &self,
        left: &ConstantValue,
        right: &ConstantValue,
    ) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(l), ConstantValue::Integer(r)) => {
                if *r == 0 {
                    None // Modulo by zero
                } else {
                    Some(ConstantValue::Integer(l % r))
                }
            }
            (ConstantValue::Word(l), ConstantValue::Word(r)) => {
                if *r == 0 {
                    None
                } else {
                    Some(ConstantValue::Word(l % r))
                }
            }
            _ => None,
        }
    }

    pub(crate) fn eval_less(
        &self,
        left: &ConstantValue,
        right: &ConstantValue,
    ) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(l), ConstantValue::Integer(r)) => {
                Some(ConstantValue::Boolean(l < r))
            }
            (ConstantValue::Byte(l), ConstantValue::Byte(r)) => Some(ConstantValue::Boolean(l < r)),
            (ConstantValue::Word(l), ConstantValue::Word(r)) => Some(ConstantValue::Boolean(l < r)),
            _ => None,
        }
    }

    pub(crate) fn eval_less_equal(
        &self,
        left: &ConstantValue,
        right: &ConstantValue,
    ) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(l), ConstantValue::Integer(r)) => {
                Some(ConstantValue::Boolean(l <= r))
            }
            (ConstantValue::Byte(l), ConstantValue::Byte(r)) => {
                Some(ConstantValue::Boolean(l <= r))
            }
            (ConstantValue::Word(l), ConstantValue::Word(r)) => {
                Some(ConstantValue::Boolean(l <= r))
            }
            _ => None,
        }
    }

    pub(crate) fn eval_greater(
        &self,
        left: &ConstantValue,
        right: &ConstantValue,
    ) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(l), ConstantValue::Integer(r)) => {
                Some(ConstantValue::Boolean(l > r))
            }
            (ConstantValue::Byte(l), ConstantValue::Byte(r)) => Some(ConstantValue::Boolean(l > r)),
            (ConstantValue::Word(l), ConstantValue::Word(r)) => Some(ConstantValue::Boolean(l > r)),
            _ => None,
        }
    }

    pub(crate) fn eval_greater_equal(
        &self,
        left: &ConstantValue,
        right: &ConstantValue,
    ) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(l), ConstantValue::Integer(r)) => {
                Some(ConstantValue::Boolean(l >= r))
            }
            (ConstantValue::Byte(l), ConstantValue::Byte(r)) => {
                Some(ConstantValue::Boolean(l >= r))
            }
            (ConstantValue::Word(l), ConstantValue::Word(r)) => {
                Some(ConstantValue::Boolean(l >= r))
            }
            _ => None,
        }
    }

    pub(crate) fn eval_and(
        &self,
        left: &ConstantValue,
        right: &ConstantValue,
    ) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Boolean(l), ConstantValue::Boolean(r)) => {
                Some(ConstantValue::Boolean(*l && *r))
            }
            _ => None,
        }
    }

    pub(crate) fn eval_or(
        &self,
        left: &ConstantValue,
        right: &ConstantValue,
    ) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Boolean(l), ConstantValue::Boolean(r)) => {
                Some(ConstantValue::Boolean(*l || *r))
            }
            _ => None,
        }
    }

    pub(crate) fn eval_unary_minus(&self, operand: &ConstantValue) -> Option<ConstantValue> {
        match operand {
            ConstantValue::Integer(i) => Some(ConstantValue::Integer(-i)),
            _ => None,
        }
    }

    pub(crate) fn eval_not(&self, operand: &ConstantValue) -> Option<ConstantValue> {
        match operand {
            ConstantValue::Boolean(b) => Some(ConstantValue::Boolean(!b)),
            _ => None,
        }
    }
}
