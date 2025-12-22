//! Declaration analysis (const, type, var, proc, func)

use ast::Node;
use symbols::{Parameter, ParameterMode, Symbol, SymbolKind};
use crate::SemanticAnalyzer;

impl SemanticAnalyzer {
    /// Analyze constant declaration
    pub(crate) fn analyze_const_decl(&mut self, decl: &Node) {
        if let Node::ConstDecl(c) = decl {
            // Check if constant already exists
            if self.core.symbol_table.exists_in_current_scope(&c.name) {
                self.core.add_error(
                    format!("Constant '{}' already declared", c.name),
                    c.span,
                );
                return;
            }

            // Analyze the constant value expression
            let const_type = self.analyze_expression(&c.value);

            // Evaluate constant value (constant folding)
            let const_value = self.evaluate_constant_expression(&c.value);

            // Create and insert symbol
            let symbol = Symbol {
                kind: SymbolKind::Constant {
                    name: c.name.clone(),
                    const_type: const_type.clone(),
                    value: const_value, // Store evaluated constant value
                    span: c.span,
                },
                scope_level: self.core.symbol_table.scope_level(),
            };

            if let Err(e) = self.core.symbol_table.insert(symbol) {
                self.core.add_error(e, c.span);
            }
        }
    }

    /// Analyze type declaration
    pub(crate) fn analyze_type_decl(&mut self, decl: &Node) {
        if let Node::TypeDecl(t) = decl {
            // Check if type already exists
            if self.core.symbol_table.exists_in_current_scope(&t.name) {
                self.core.add_error(
                    format!("Type '{}' already declared", t.name),
                    t.span,
                );
                return;
            }

            // Check for generic type parameters
            if !t.generic_params.is_empty() {
                // Generic type declaration: TList<T>
                let param_names: Vec<String> = t.generic_params
                    .iter()
                    .map(|p| p.name.clone())
                    .collect();

                // Analyze the type expression (may contain generic parameters as NamedType nodes)
                // Generic parameters should be represented as Type::Named nodes in the template
                // We need to allow generic parameter names during analysis
                let template_type = self.analyze_type_with_generic_params(&t.type_expr, &param_names);

                // Create and insert generic type symbol
                let symbol = Symbol {
                    kind: SymbolKind::GenericType {
                        name: t.name.clone(),
                        param_names: param_names.clone(),
                        template_type,
                        span: t.span,
                    },
                    scope_level: self.core.symbol_table.scope_level(),
                };

                if let Err(e) = self.core.symbol_table.insert(symbol) {
                    self.core.add_error(e, t.span);
                }
                return;
            }

            // Non-generic type declaration
            // Analyze the type expression
            let type_expr = self.analyze_type(&t.type_expr);

            // Create and insert symbol
            let symbol = Symbol {
                kind: SymbolKind::TypeAlias {
                    name: t.name.clone(),
                    aliased_type: type_expr,
                    span: t.span,
                },
                scope_level: self.core.symbol_table.scope_level(),
            };

            if let Err(e) = self.core.symbol_table.insert(symbol) {
                self.core.add_error(e, t.span);
            }
        }
    }

    /// Analyze variable declaration
    pub(crate) fn analyze_var_decl(&mut self, decl: &Node) {
        if let Node::VarDecl(v) = decl {
            // Analyze the type
            let var_type = self.analyze_type(&v.type_expr);

            // Create symbols for each variable name
            for name in &v.names {
                // Check if variable already exists
                if self.core.symbol_table.exists_in_current_scope(name) {
                    self.core.add_error(
                        format!("Variable '{}' already declared", name),
                        v.span,
                    );
                    continue;
                }

                let symbol = Symbol {
                    kind: SymbolKind::Variable {
                        name: name.clone(),
                        var_type: var_type.clone(),
                        span: v.span,
                    },
                    scope_level: self.core.symbol_table.scope_level(),
                };

                if let Err(e) = self.core.symbol_table.insert(symbol) {
                    self.core.add_error(e, v.span);
                }
            }
        }
    }

    /// Analyze procedure declaration
    pub(crate) fn analyze_proc_decl(&mut self, decl: &Node) {
        if let Node::ProcDecl(p) = decl {
            // Check if procedure already exists
            if self.core.symbol_table.exists_in_current_scope(&p.name) {
                self.core.add_error(
                    format!("Procedure '{}' already declared", p.name),
                    p.span,
                );
                return;
            }

            // Analyze parameters
            let params = self.analyze_params(&p.params);

            // Create symbol
            let symbol = Symbol {
                kind: SymbolKind::Procedure {
                    name: p.name.clone(),
                    params: params.clone(),
                    span: p.span,
                },
                scope_level: self.core.symbol_table.scope_level(),
            };

            if let Err(e) = self.core.symbol_table.insert(symbol) {
                self.core.add_error(e, p.span);
            }

            // Analyze procedure body (enter new scope)
            self.core.symbol_table.enter_scope();
            // Add parameters to scope
            for param in &params {
                for name in &param.name.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>() {
                    if !name.is_empty() {
                        let param_symbol = Symbol {
                            kind: SymbolKind::Variable {
                                name: name.clone(),
                                var_type: param.param_type.clone(),
                                span: param.span,
                            },
                            scope_level: self.core.symbol_table.scope_level(),
                        };
                        let _ = self.core.symbol_table.insert(param_symbol);
                    }
                }
            }
            self.analyze_block(&p.block);
            self.core.symbol_table.exit_scope();
        }
    }

    /// Analyze function declaration
    pub(crate) fn analyze_func_decl(&mut self, decl: &Node) {
        if let Node::FuncDecl(f) = decl {
            // Check if function already exists
            if self.core.symbol_table.exists_in_current_scope(&f.name) {
                self.core.add_error(
                    format!("Function '{}' already declared", f.name),
                    f.span,
                );
                return;
            }

            // Analyze parameters
            let params = self.analyze_params(&f.params);
            let params_clone = params.clone();

            // Analyze return type
            let return_type = self.analyze_type(&f.return_type);
            let return_type_clone = return_type.clone();

            // Create symbol
            let symbol = Symbol {
                kind: SymbolKind::Function {
                    name: f.name.clone(),
                    params: params_clone.clone(),
                    return_type: return_type_clone,
                    span: f.span,
                },
                scope_level: self.core.symbol_table.scope_level(),
            };

            if let Err(e) = self.core.symbol_table.insert(symbol) {
                self.core.add_error(e, f.span);
            }

            // Analyze function body (enter new scope)
            self.core.symbol_table.enter_scope();
            // Add parameters to scope
            for param in &params_clone {
                for name in &param.name.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>() {
                    if !name.is_empty() {
                        let param_symbol = Symbol {
                            kind: SymbolKind::Variable {
                                name: name.clone(),
                                var_type: param.param_type.clone(),
                                span: param.span,
                            },
                            scope_level: self.core.symbol_table.scope_level(),
                        };
                        let _ = self.core.symbol_table.insert(param_symbol);
                    }
                }
            }
            self.analyze_block(&f.block);
            self.core.symbol_table.exit_scope();
        }
    }

    /// Analyze parameters
    pub(crate) fn analyze_params(&mut self, params: &[ast::Param]) -> Vec<Parameter> {
        params
            .iter()
            .map(|p| {
                let param_type = self.analyze_type(&p.type_expr);
                let passing_mode = match p.param_type {
                    ast::ParamType::Value => ParameterMode::Value,
                    ast::ParamType::Var => ParameterMode::Var,
                    ast::ParamType::Const => ParameterMode::Const,
                    ast::ParamType::ConstRef => ParameterMode::Const, // ConstRef is similar to Const
                    ast::ParamType::Out => ParameterMode::Var,        // Out is similar to Var (reference)
                };
                Parameter {
                    name: p.names.join(", "), // Join multiple names
                    param_type,
                    passing_mode,
                    span: p.span,
                }
            })
            .collect()
    }
}
