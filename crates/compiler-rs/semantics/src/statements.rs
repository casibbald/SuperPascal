//! Statement analysis (if, while, for, repeat, case, assign, call, etc.)

use crate::SemanticAnalyzer;
use crate::core;
use ::types::Type;
use ast::Node;
use symbols::{ConstantValue, SymbolKind};

impl SemanticAnalyzer {
    /// Analyze statement (dispatcher)
    pub(crate) fn analyze_statement(&mut self, stmt: &Node) {
        match stmt {
            Node::AssignStmt(a) => self.analyze_assignment(a),
            Node::CallStmt(c) => self.analyze_call_stmt(c),
            Node::IfStmt(i) => self.analyze_if_stmt(i),
            Node::WhileStmt(w) => self.analyze_while_stmt(w),
            Node::ForStmt(f) => self.analyze_for_stmt(f),
            Node::RepeatStmt(r) => self.analyze_repeat_stmt(r),
            Node::CaseStmt(c) => self.analyze_case_stmt(c),
            Node::Block(blk) => {
                // BEGIN...END block: analyze all contained statements
                for s in &blk.statements {
                    self.analyze_statement(s);
                }
            }
            _ => {
                self.core
                    .add_error("Unsupported statement type".to_string(), stmt.span());
            }
        }
    }

    /// Analyze assignment statement
    pub(crate) fn analyze_assignment(&mut self, assign: &ast::AssignStmt) {
        // Analyze target (lvalue)
        let target_type = self.analyze_lvalue(&assign.target);

        // Analyze value (rvalue)
        let mut value_type = self.analyze_expression(&assign.value);

        // Special handling: If target is procedural type and value is a function/procedure name,
        // convert the function/procedure to a procedural type
        if let Type::Procedural { .. } = target_type {
            if let Node::IdentExpr(ident) = assign.value.as_ref() {
                if let Some(symbol) = self.core.symbol_table.lookup(&ident.name) {
                    match &symbol.kind {
                        SymbolKind::Function {
                            return_type,
                            params,
                            ..
                        } => {
                            // Convert function to procedural type
                            let param_types: Vec<Type> =
                                params.iter().map(|p| p.param_type.clone()).collect();
                            value_type = Type::procedural(
                                true, // is_function
                                param_types,
                                Some(return_type.clone()),
                                false, // is_method_pointer (TODO: check if method)
                            );
                        }
                        SymbolKind::Procedure { params, .. } => {
                            // Convert procedure to procedural type
                            let param_types: Vec<Type> =
                                params.iter().map(|p| p.param_type.clone()).collect();
                            value_type = Type::procedural(
                                false, // is_function
                                param_types,
                                None,  // no return type
                                false, // is_method_pointer (TODO: check if method)
                            );
                        }
                        _ => {
                            // Not a function/procedure, will be caught by type checking below
                        }
                    }
                }
            }
        }

        // Check assignment compatibility
        if !value_type.is_assignable_to(&target_type) {
            self.core.add_error(
                format!(
                    "Type mismatch: cannot assign {} to {}",
                    core::CoreAnalyzer::format_type(&value_type),
                    core::CoreAnalyzer::format_type(&target_type)
                ),
                assign.span,
            );
        }
    }

    /// Analyze call statement (procedure call)
    pub(crate) fn analyze_call_stmt(&mut self, call: &ast::CallStmt) {
        // Look up procedure
        let params_opt = self
            .core
            .symbol_table
            .lookup(&call.name)
            .and_then(|symbol| {
                if let SymbolKind::Procedure { params, .. } = &symbol.kind {
                    Some(params.clone())
                } else {
                    None
                }
            });

        if let Some(params) = params_opt {
            // Check argument count
            if call.args.len() != params.len() {
                self.core.add_error(
                    format!(
                        "Procedure '{}' expects {} arguments, found {}",
                        call.name,
                        params.len(),
                        call.args.len()
                    ),
                    call.span,
                );
                return;
            }

            // Check argument types
            for (arg, param) in call.args.iter().zip(params.iter()) {
                let arg_type = self.analyze_expression(arg);
                if !arg_type.is_assignable_to(&param.param_type) {
                    self.core.add_error(
                        format!(
                            "Argument type mismatch: expected {}, found {}",
                            core::CoreAnalyzer::format_type(&param.param_type),
                            core::CoreAnalyzer::format_type(&arg_type)
                        ),
                        arg.span(),
                    );
                }
            }
        } else if self.core.symbol_table.lookup(&call.name).is_some() {
            self.core
                .add_error(format!("'{}' is not a procedure", call.name), call.span);
        } else {
            self.core
                .add_error(format!("Procedure '{}' not found", call.name), call.span);
        }
    }

    /// Analyze if statement
    pub(crate) fn analyze_if_stmt(&mut self, if_stmt: &ast::IfStmt) {
        let condition_type = self.analyze_expression(&if_stmt.condition);
        if !condition_type.equals(&Type::boolean()) {
            self.core.add_error(
                "If condition must be boolean".to_string(),
                if_stmt.condition.span(),
            );
            return;
        }

        // Constant folding: evaluate constant conditions
        if let Some(constant_value) = self.evaluate_constant_expression(&if_stmt.condition) {
            if let ConstantValue::Boolean(condition_value) = constant_value {
                if condition_value {
                    // Condition is always true - only analyze then branch
                    self.analyze_statement(&if_stmt.then_block);
                    // Note: Dead code elimination would skip else branch in code generation
                } else {
                    // Condition is always false - only analyze else branch (if present)
                    if let Some(else_block) = &if_stmt.else_block {
                        self.analyze_statement(else_block);
                    }
                    // Note: Dead code elimination would skip then branch in code generation
                }
                return;
            }
        }

        // Non-constant condition - analyze both branches
        self.analyze_statement(&if_stmt.then_block);
        if let Some(else_block) = &if_stmt.else_block {
            self.analyze_statement(else_block);
        }
    }

    /// Analyze while statement
    pub(crate) fn analyze_while_stmt(&mut self, while_stmt: &ast::WhileStmt) {
        let condition_type = self.analyze_expression(&while_stmt.condition);
        if !condition_type.equals(&Type::boolean()) {
            self.core.add_error(
                "While condition must be boolean".to_string(),
                while_stmt.condition.span(),
            );
            return;
        }

        // Constant folding: evaluate constant conditions
        if let Some(constant_value) = self.evaluate_constant_expression(&while_stmt.condition) {
            if let ConstantValue::Boolean(condition_value) = constant_value {
                if !condition_value {
                    // Condition is always false - loop body never executes
                    // Note: Dead code elimination would skip loop body in code generation
                    return;
                }
                // Condition is always true - infinite loop (warn in future)
                // For now, still analyze body for semantic correctness
            }
        }

        // Analyze loop body (non-constant or true constant condition)
        self.analyze_statement(&while_stmt.body);
    }

    /// Analyze for statement
    pub(crate) fn analyze_for_stmt(&mut self, for_stmt: &ast::ForStmt) {
        // Check loop variable exists and is assignable
        let var_type_opt = self
            .core
            .symbol_table
            .lookup(&for_stmt.var_name)
            .and_then(|symbol| {
                if let SymbolKind::Variable { var_type, .. } = &symbol.kind {
                    Some(var_type.clone())
                } else {
                    None
                }
            });

        if let Some(var_type) = var_type_opt {
            let start_type = self.analyze_expression(&for_stmt.start_expr);
            let end_type = self.analyze_expression(&for_stmt.end_expr);

            if !start_type.is_assignable_to(&var_type) {
                self.core.add_error(
                    format!(
                        "For loop start value type {} not compatible with loop variable type {}",
                        core::CoreAnalyzer::format_type(&start_type),
                        core::CoreAnalyzer::format_type(&var_type)
                    ),
                    for_stmt.start_expr.span(),
                );
            }

            if !end_type.is_assignable_to(&var_type) {
                self.core.add_error(
                    format!(
                        "For loop end value type {} not compatible with loop variable type {}",
                        core::CoreAnalyzer::format_type(&end_type),
                        core::CoreAnalyzer::format_type(&var_type)
                    ),
                    for_stmt.end_expr.span(),
                );
            }
        } else if self.core.symbol_table.lookup(&for_stmt.var_name).is_some() {
            self.core.add_error(
                format!("'{}' is not a variable", for_stmt.var_name),
                for_stmt.span,
            );
        } else {
            self.core.add_error(
                format!("Variable '{}' not found", for_stmt.var_name),
                for_stmt.span,
            );
        }

        self.analyze_statement(&for_stmt.body);
    }

    /// Analyze repeat statement
    pub(crate) fn analyze_repeat_stmt(&mut self, repeat_stmt: &ast::RepeatStmt) {
        for stmt in &repeat_stmt.statements {
            self.analyze_statement(stmt);
        }
        let condition_type = self.analyze_expression(&repeat_stmt.condition);
        if !condition_type.equals(&Type::boolean()) {
            self.core.add_error(
                "Repeat condition must be boolean".to_string(),
                repeat_stmt.condition.span(),
            );
        }
    }

    /// Analyze case statement
    pub(crate) fn analyze_case_stmt(&mut self, case_stmt: &ast::CaseStmt) {
        let expr_type = self.analyze_expression(&case_stmt.expr);
        // Case expression must be ordinal type (integer, byte, word, char, boolean)
        if !matches!(expr_type, Type::Primitive(_) | Type::Named { .. }) {
            self.core.add_error(
                "Case expression must be an ordinal type".to_string(),
                case_stmt.expr.span(),
            );
        }

        for case_branch in &case_stmt.cases {
            for value in &case_branch.values {
                let value_type = self.analyze_expression(value);
                if !value_type.equals(&expr_type) {
                    self.core.add_error(
                        format!(
                            "Case value type {} does not match expression type {}",
                            core::CoreAnalyzer::format_type(&value_type),
                            core::CoreAnalyzer::format_type(&expr_type)
                        ),
                        value.span(),
                    );
                }
            }
            self.analyze_statement(&case_branch.statement);
        }

        if let Some(else_stmt) = &case_stmt.else_branch {
            self.analyze_statement(else_stmt);
        }
    }
}
