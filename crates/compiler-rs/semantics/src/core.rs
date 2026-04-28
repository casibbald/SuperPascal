//! Core semantic analyzer functionality

use ::types::Type;
use errors::Diagnostic;
use symbols::SymbolTable;
use tokens::Span;

/// Core semantic analyzer functionality
pub struct CoreAnalyzer {
    pub symbol_table: SymbolTable,
    pub diagnostics: Vec<Diagnostic>,
    pub filename: Option<String>,
}

impl CoreAnalyzer {
    /// Create a new core analyzer
    pub(super) fn new(filename: Option<String>) -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            diagnostics: vec![],
            filename,
        }
    }

    /// Add an error diagnostic
    pub fn add_error(&mut self, message: String, span: Span) {
        use errors::ErrorSeverity;
        let diag = Diagnostic::new(ErrorSeverity::Error, message, span).with_file(
            self.filename
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
        );
        self.diagnostics.push(diag);
    }

    /// Format a type for error messages
    pub(super) fn format_type(ty: &Type) -> String {
        match ty {
            Type::Primitive(p) => format!("{:?}", p),
            Type::Array { element_type, .. } => {
                format!("array of {}", Self::format_type(element_type))
            }
            Type::DynamicArray { element_type } => {
                format!("dynamic array of {}", Self::format_type(element_type))
            }
            Type::Record { fields, .. } => {
                format!("record with {} fields", fields.len())
            }
            Type::Pointer { base_type } => {
                format!("pointer to {}", Self::format_type(base_type))
            }
            Type::Error => "error".to_string(),
            Type::Named { name, .. } => name.clone(),
            Type::Generic {
                name, param_names, ..
            } => {
                format!("{}<{}>", name, param_names.join(", "))
            }
            Type::Instantiated { generic_name, args } => {
                let arg_strs: Vec<String> = args.iter().map(|t| Self::format_type(t)).collect();
                format!("{}<{}>", generic_name, arg_strs.join(", "))
            }
            Type::Variant => "Variant".to_string(),
            Type::Procedural {
                is_function,
                params,
                return_type,
                is_method_pointer,
            } => {
                let kind = if *is_function {
                    "function"
                } else {
                    "procedure"
                };
                let method = if *is_method_pointer { " of object" } else { "" };
                let param_strs: Vec<String> = params.iter().map(|t| Self::format_type(t)).collect();
                let params_str = if param_strs.is_empty() {
                    "".to_string()
                } else {
                    format!("({})", param_strs.join(", "))
                };
                let return_str = if let Some(rt) = return_type {
                    format!(": {}", Self::format_type(rt))
                } else {
                    "".to_string()
                };
                format!("{}{}{}{}", kind, params_str, return_str, method)
            }
        }
    }
}
