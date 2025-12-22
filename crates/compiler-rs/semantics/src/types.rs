//! Type analysis (named types, arrays, records, etc.)

use ast::Node;
use symbols::SymbolKind;
use ::types::{Field, Type};
use crate::SemanticAnalyzer;
use std::collections::HashMap;

impl SemanticAnalyzer {
    /// Validate that a type argument satisfies its constraint
    /// Returns true if the constraint is satisfied, false otherwise
    fn validate_generic_constraint(&mut self, arg_type: &Type, constraint: &Type, param_name: &str, span: tokens::Span) -> bool {
        // If arg_type is Error, skip constraint validation (error already reported)
        if *arg_type == Type::Error {
            return true; // Don't add additional constraint errors
        }

        // Handle special constraint keywords
        match constraint {
            Type::Named { name } => {
                match name.as_str() {
                    "class" => {
                        // T: class - arg_type must be a class type
                        // For now, we check if it's a Named type (classes are typically Named types)
                        // TODO: More sophisticated class type checking when class types are fully implemented
                        match arg_type {
                            Type::Named { .. } => true,
                            _ => {
                                self.core.add_error(
                                    format!(
                                        "Generic type parameter '{}' must be a class type, but '{}' is not",
                                        param_name,
                                        crate::core::CoreAnalyzer::format_type(arg_type)
                                    ),
                                    span,
                                );
                                false
                            }
                        }
                    }
                    "record" => {
                        // T: record - arg_type must be a record type
                        match arg_type {
                            Type::Record { .. } => true,
                            _ => {
                                self.core.add_error(
                                    format!(
                                        "Generic type parameter '{}' must be a record type, but '{}' is not",
                                        param_name,
                                        crate::core::CoreAnalyzer::format_type(arg_type)
                                    ),
                                    span,
                                );
                                false
                            }
                        }
                    }
                    "constructor" => {
                        // T: constructor - arg_type must have a constructor
                        // For now, we assume all class types have constructors
                        // TODO: More sophisticated constructor checking
                        match arg_type {
                            Type::Named { .. } => true,
                            _ => {
                                self.core.add_error(
                                    format!(
                                        "Generic type parameter '{}' must have a constructor, but '{}' does not",
                                        param_name,
                                        crate::core::CoreAnalyzer::format_type(arg_type)
                                    ),
                                    span,
                                );
                                false
                            }
                        }
                    }
                    _ => {
                        // Interface constraint: T: IInterface
                        // Check if arg_type implements the interface
                        // For now, we check if arg_type is the same Named type or if it's assignable
                        // TODO: More sophisticated interface implementation checking
                        match arg_type {
                            Type::Named { name: arg_name } => {
                                if arg_name == name {
                                    true
                                } else {
                                    // Check if arg_type implements the interface
                                    // This is a simplified check - in a full implementation, we'd check
                                    // the class's interface list
                                    // For now, we'll allow it and let the runtime handle it
                                    // or we could check the symbol table for interface implementation
                                    true // TODO: Implement proper interface checking
                                }
                            }
                            _ => {
                                self.core.add_error(
                                    format!(
                                        "Generic type parameter '{}' must implement '{}', but '{}' does not",
                                        param_name,
                                        name,
                                        crate::core::CoreAnalyzer::format_type(arg_type)
                                    ),
                                    span,
                                );
                                false
                            }
                        }
                    }
                }
            }
            _ => {
                // Constraint is a type - check if arg_type is assignable to constraint
                if arg_type.is_assignable_to(constraint) {
                    true
                } else {
                    self.core.add_error(
                        format!(
                            "Generic type parameter '{}' must be assignable to '{}', but '{}' is not",
                            param_name,
                            crate::core::CoreAnalyzer::format_type(constraint),
                            crate::core::CoreAnalyzer::format_type(arg_type)
                        ),
                        span,
                    );
                    false
                }
            }
        }
    }

    /// Substitute generic type parameters with concrete types
    /// This is used when instantiating a generic type (e.g., TList<integer>)
    fn substitute_type_params(&self, template: &Type, substitutions: &HashMap<String, Type>) -> Type {
        match template {
            Type::Named { name } => {
                // If this is a generic parameter, substitute it
                if let Some(subst_type) = substitutions.get(name) {
                    subst_type.clone()
                } else {
                    // Not a generic parameter, keep as-is
                    template.clone()
                }
            }
            Type::Array { index_type, element_type, size } => {
                Type::Array {
                    index_type: Box::new(self.substitute_type_params(index_type, substitutions)),
                    element_type: Box::new(self.substitute_type_params(element_type, substitutions)),
                    size: *size,
                }
            }
            Type::DynamicArray { element_type } => {
                Type::DynamicArray {
                    element_type: Box::new(self.substitute_type_params(element_type, substitutions)),
                }
            }
            Type::Record { fields, size } => {
                let substituted_fields: Vec<Field> = fields
                    .iter()
                    .map(|f| Field {
                        name: f.name.clone(),
                        field_type: Box::new(self.substitute_type_params(&f.field_type, substitutions)),
                        offset: f.offset,
                    })
                    .collect();
                let mut record = Type::Record {
                    fields: substituted_fields,
                    size: *size,
                };
                record.calculate_record_offsets();
                record
            }
            Type::Pointer { base_type } => {
                Type::Pointer {
                    base_type: Box::new(self.substitute_type_params(base_type, substitutions)),
                }
            }
            // Primitive types, Error, Generic, Instantiated don't need substitution
            _ => template.clone(),
        }
    }

    /// Analyze type expression with generic parameter context
    /// This is used when analyzing generic type declarations where generic parameters
    /// should be treated as valid type names (Type::Named) rather than errors
    pub(crate) fn analyze_type_with_generic_params(&mut self, type_expr: &Node, generic_params: &[String]) -> Type {
        match type_expr {
            Node::NamedType(n) => {
                // Check if this is a generic parameter
                if generic_params.contains(&n.name) {
                    // This is a generic parameter - represent it as a NamedType placeholder
                    return Type::Named {
                        name: n.name.clone(),
                    };
                }
                // Not a generic parameter, analyze normally
                self.analyze_type(type_expr)
            }
            Node::ArrayType(a) => {
                let index_type = self.analyze_type_with_generic_params(&a.index_type, generic_params);
                let element_type = self.analyze_type_with_generic_params(&a.element_type, generic_params);
                Type::array(index_type, element_type)
            }
            Node::DynamicArrayType(d) => {
                let element_type = self.analyze_type_with_generic_params(&d.element_type, generic_params);
                Type::dynamic_array(element_type)
            }
            Node::RecordType(r) => {
                let fields: Vec<Field> = r
                    .fields
                    .iter()
                    .map(|f| {
                        let field_type = self.analyze_type_with_generic_params(&f.type_expr, generic_params);
                        Field {
                            name: f.names[0].clone(),
                            field_type: Box::new(field_type),
                            offset: None,
                        }
                    })
                    .collect();
                let mut record = Type::record(fields);
                record.calculate_record_offsets();
                record
            }
            _ => self.analyze_type(type_expr),
        }
    }

    /// Analyze type expression
    pub(crate) fn analyze_type(&mut self, type_expr: &Node) -> Type {
        match type_expr {
            Node::NamedType(n) => {
                // Check for generic type arguments
                if !n.generic_args.is_empty() {
                    // Generic type instantiation: TList<integer>
                    // Look up the generic type template
                    if let Some(symbol) = self.core.symbol_table.lookup(&n.name) {
                        // Clone necessary data before mutable borrow
                        let (param_names, param_constraints, template_type) = if let SymbolKind::GenericType { param_names, param_constraints, template_type, .. } = &symbol.kind {
                            (param_names.clone(), param_constraints.clone(), template_type.clone())
                        } else {
                            self.core.add_error(
                                format!("'{}' is not a generic type", n.name),
                                n.span,
                            );
                            return Type::Error;
                        };

                        // Check argument count matches parameter count
                        if n.generic_args.len() != param_names.len() {
                            self.core.add_error(
                                format!(
                                    "Generic type '{}' expects {} type arguments, found {}",
                                    n.name,
                                    param_names.len(),
                                    n.generic_args.len()
                                ),
                                n.span,
                            );
                            return Type::Error;
                        }

                        // Analyze all type arguments
                        let mut arg_types = Vec::new();
                        for arg in &n.generic_args {
                            let arg_type = self.analyze_type(arg);
                            if arg_type == Type::Error {
                                return Type::Error;
                            }
                            arg_types.push(arg_type);
                        }

                        // Validate constraints
                        let mut all_constraints_satisfied = true;
                        for (i, (param_name, arg_type)) in param_names.iter().zip(arg_types.iter()).enumerate() {
                            if let Some(constraint) = param_constraints.get(i).and_then(|c| c.as_ref()) {
                                if !self.validate_generic_constraint(arg_type, constraint, param_name, n.span) {
                                    all_constraints_satisfied = false;
                                }
                            }
                        }

                        if !all_constraints_satisfied {
                            return Type::Error;
                        }

                        // Create substitution map: param_name -> arg_type
                        let mut substitutions = HashMap::new();
                        for (param_name, arg_type) in param_names.iter().zip(arg_types.iter()) {
                            substitutions.insert(param_name.clone(), arg_type.clone());
                        }

                        // Substitute type parameters in the template
                        let instantiated_type = self.substitute_type_params(&template_type, &substitutions);
                        
                        // Return the instantiated (substituted) type
                        instantiated_type
                    } else {
                        self.core.add_error(
                            format!("Generic type '{}' not found", n.name),
                            n.span,
                        );
                        Type::Error
                    }
                } else {
                    // Non-generic type lookup
                    // Look up named type in symbol table
                    if let Some(symbol) = self.core.symbol_table.lookup(&n.name) {
                        match &symbol.kind {
                            SymbolKind::TypeAlias { aliased_type, .. } => aliased_type.clone(),
                            SymbolKind::GenericType { name, param_names, .. } => {
                                // Reference to generic type without arguments
                                Type::Generic {
                                    name: name.clone(),
                                    param_names: param_names.clone(),
                                    template: Box::new(Type::Error), // Template not needed for reference
                                }
                            }
                            _ => {
                                self.core.add_error(
                                    format!("'{}' is not a type", n.name),
                                    n.span,
                                );
                                Type::Error
                            }
                        }
                    } else {
                        // Check for built-in types
                        match n.name.as_str() {
                            "integer" => Type::integer(),
                            "byte" => Type::byte(),
                            "word" => Type::word(),
                            "boolean" => Type::boolean(),
                            "char" => Type::char(),
                            _ => {
                                self.core.add_error(
                                    format!("Type '{}' not found", n.name),
                                    n.span,
                                );
                                Type::Error
                            }
                        }
                    }
                }
            }
            Node::ArrayType(a) => {
                let index_type = self.analyze_type(&a.index_type);
                let element_type = self.analyze_type(&a.element_type);
                Type::array(index_type, element_type)
            }
            Node::DynamicArrayType(d) => {
                let element_type = self.analyze_type(&d.element_type);
                Type::dynamic_array(element_type)
            }
            Node::RecordType(r) => {
                let fields: Vec<Field> = r
                    .fields
                    .iter()
                    .map(|f| {
                        let field_type = self.analyze_type(&f.type_expr);
                        Field {
                            name: f.names[0].clone(), // Use first name for now
                            field_type: Box::new(field_type),
                            offset: None,
                        }
                    })
                    .collect();
                let mut record = Type::record(fields);
                record.calculate_record_offsets();
                record
            }
            _ => {
                self.core.add_error("Invalid type expression".to_string(), type_expr.span());
                Type::Error
            }
        }
    }
}
