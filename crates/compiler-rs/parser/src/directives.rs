//! Directive evaluation for conditional compilation
//!
//! This module handles evaluation of compiler directives like {$IFDEF}, {$DEFINE}, etc.
//! It maintains a symbol table of defined symbols and evaluates conditional compilation blocks.

use std::collections::HashSet;
use errors::{ParserError, ParserResult};
use tokens::Span;

/// Directive type parsed from directive content
#[derive(Debug, Clone, PartialEq)]
pub enum DirectiveType {
    /// {$IFDEF symbol} - if symbol is defined
    IfDef(String),
    /// {$IFNDEF symbol} - if symbol is not defined
    IfNDef(String),
    /// {$IF expression} - if expression evaluates to true
    If(String), // Expression string to evaluate
    /// {$ELSEIF expression} - else if expression evaluates to true
    ElseIf(String), // Expression string to evaluate
    /// {$ELSE} - else branch
    Else,
    /// {$ENDIF} - end conditional block
    EndIf,
    /// {$DEFINE symbol} - define a symbol
    Define(String),
    /// {$UNDEF symbol} - undefine a symbol
    Undef(String),
    /// {$INCLUDE 'filename'} - include a file
    Include(String),
    /// Other directives (passed through without evaluation)
    Other(String),
}

/// Directive evaluator for conditional compilation
pub struct DirectiveEvaluator {
    /// Set of defined symbols
    defined_symbols: HashSet<String>,
    /// Stack of conditional compilation states (true = active, false = inactive)
    conditional_stack: Vec<bool>,
    /// Whether we're currently in an active branch
    is_active: bool,
}

impl DirectiveEvaluator {
    /// Create a new directive evaluator
    pub fn new() -> Self {
        Self {
            defined_symbols: HashSet::new(),
            conditional_stack: Vec::new(),
            is_active: true, // Start active (no conditionals yet)
        }
    }

    /// Create a new directive evaluator with predefined symbols
    pub fn with_symbols(symbols: Vec<String>) -> Self {
        let mut evaluator = Self::new();
        for symbol in symbols {
            evaluator.defined_symbols.insert(symbol.to_uppercase());
        }
        evaluator
    }

    /// Parse directive content into a DirectiveType
    pub fn parse_directive(content: &str) -> DirectiveType {
        let content = content.trim();
        let parts: Vec<&str> = content.split_whitespace().collect();
        
        if parts.is_empty() {
            return DirectiveType::Other(content.to_string());
        }

        let directive_name = parts[0].to_uppercase();
        
        match directive_name.as_str() {
            "IFDEF" => {
                if parts.len() >= 2 {
                    DirectiveType::IfDef(parts[1].to_uppercase())
                } else {
                    DirectiveType::Other(content.to_string())
                }
            }
            "IFNDEF" => {
                if parts.len() >= 2 {
                    DirectiveType::IfNDef(parts[1].to_uppercase())
                } else {
                    DirectiveType::Other(content.to_string())
                }
            }
            "IF" => {
                // Extract everything after "IF" as the expression
                if let Some(expr_start) = content[2..].trim_start().find(|c: char| !c.is_whitespace()) {
                    let expr = content[2 + expr_start..].trim().to_string();
                    DirectiveType::If(expr)
                } else {
                    DirectiveType::Other(content.to_string())
                }
            }
            "ELSEIF" => {
                // Extract everything after "ELSEIF" as the expression
                if let Some(expr_start) = content[6..].trim_start().find(|c: char| !c.is_whitespace()) {
                    let expr = content[6 + expr_start..].trim().to_string();
                    DirectiveType::ElseIf(expr)
                } else {
                    DirectiveType::Other(content.to_string())
                }
            }
            "ELSE" => DirectiveType::Else,
            "ENDIF" | "END" => DirectiveType::EndIf,
            "DEFINE" => {
                if parts.len() >= 2 {
                    DirectiveType::Define(parts[1].to_uppercase())
                } else {
                    DirectiveType::Other(content.to_string())
                }
            }
            "UNDEF" | "UNDEFINE" => {
                if parts.len() >= 2 {
                    DirectiveType::Undef(parts[1].to_uppercase())
                } else {
                    DirectiveType::Other(content.to_string())
                }
            }
            "INCLUDE" | "I" => {
                // Extract filename from string literal or identifier
                if parts.len() >= 2 {
                    let filename = parts[1]
                        .trim_matches('\'')
                        .trim_matches('"')
                        .to_string();
                    DirectiveType::Include(filename)
                } else {
                    DirectiveType::Other(content.to_string())
                }
            }
            _ => DirectiveType::Other(content.to_string()),
        }
    }

    /// Evaluate a directive and update state
    /// Returns (should_include_code, should_skip_until_else_or_endif)
    pub fn evaluate(&mut self, directive: &DirectiveType, span: Span) -> ParserResult<(bool, bool)> {
        match directive {
            DirectiveType::IfDef(symbol) => {
                let is_defined = self.defined_symbols.contains(symbol);
                self.conditional_stack.push(self.is_active);
                self.is_active = self.is_active && is_defined;
                Ok((self.is_active, !self.is_active))
            }
            DirectiveType::IfNDef(symbol) => {
                let is_defined = self.defined_symbols.contains(symbol);
                self.conditional_stack.push(self.is_active);
                self.is_active = self.is_active && !is_defined;
                Ok((self.is_active, !self.is_active))
            }
            DirectiveType::If(expr) => {
                let expr_result = self.evaluate_expression(expr)?;
                self.conditional_stack.push(self.is_active);
                self.is_active = self.is_active && expr_result;
                Ok((self.is_active, !self.is_active))
            }
            DirectiveType::ElseIf(expr) => {
                if self.conditional_stack.is_empty() {
                    return Err(ParserError::InvalidSyntax {
                        message: "{$ELSEIF} without matching {$IF}, {$IFDEF}, or {$IFNDEF}".to_string(),
                        span,
                    });
                }
                // If we're already in an active branch, this ELSEIF is inactive
                // If we're in an inactive branch, check if this expression is true
                let parent_active = *self.conditional_stack.last().unwrap();
                if parent_active {
                    if self.is_active {
                        // We're already active, so this ELSEIF branch is inactive
                        self.is_active = false;
                    } else {
                        // We're inactive, check if this expression makes us active
                        let expr_result = self.evaluate_expression(expr)?;
                        self.is_active = expr_result;
                    }
                } else {
                    // Parent is inactive, so we stay inactive
                    self.is_active = false;
                }
                Ok((self.is_active, !self.is_active))
            }
            DirectiveType::Else => {
                if self.conditional_stack.is_empty() {
                    return Err(ParserError::InvalidSyntax {
                        message: "{$ELSE} without matching {$IFDEF} or {$IFNDEF}".to_string(),
                        span,
                    });
                }
                // Toggle active state: if we were active, become inactive, and vice versa
                // But only if the parent condition was active
                let parent_active = *self.conditional_stack.last().unwrap();
                if parent_active {
                    self.is_active = !self.is_active;
                } else {
                    self.is_active = false;
                }
                Ok((self.is_active, !self.is_active))
            }
            DirectiveType::EndIf => {
                if self.conditional_stack.is_empty() {
                    return Err(ParserError::InvalidSyntax {
                        message: "{$ENDIF} without matching {$IFDEF} or {$IFNDEF}".to_string(),
                        span,
                    });
                }
                self.conditional_stack.pop();
                // Restore active state from parent
                if let Some(&parent_active) = self.conditional_stack.last() {
                    self.is_active = parent_active;
                } else {
                    self.is_active = true; // No more conditionals, we're active
                }
                Ok((true, false)) // ENDIF itself is always processed
            }
            DirectiveType::Define(symbol) => {
                if self.is_active {
                    self.defined_symbols.insert(symbol.clone());
                }
                Ok((true, false)) // DEFINE is always processed if active
            }
            DirectiveType::Undef(symbol) => {
                if self.is_active {
                    self.defined_symbols.remove(symbol);
                }
                Ok((true, false)) // UNDEF is always processed if active
            }
            DirectiveType::Include(_) => {
                // Include handling will be done separately
                Ok((self.is_active, !self.is_active))
            }
            DirectiveType::Other(_) => {
                // Other directives are passed through
                Ok((self.is_active, !self.is_active))
            }
        }
    }

    /// Check if we're currently in an active compilation branch
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Check if a symbol is defined
    #[allow(dead_code)] // Public API method, may be used by external code
    pub fn is_defined(&self, symbol: &str) -> bool {
        self.defined_symbols.contains(&symbol.to_uppercase())
    }

    /// Get all defined symbols (for testing/debugging)
    pub fn defined_symbols(&self) -> &HashSet<String> {
        &self.defined_symbols
    }

    /// Check if there are unmatched conditionals
    #[allow(dead_code)] // Public API method, may be used by external code
    pub fn has_unmatched_conditionals(&self) -> bool {
        !self.conditional_stack.is_empty()
    }

    /// Evaluate a preprocessor expression
    /// Supports: Defined(SYMBOL), integer comparisons, boolean operators
    fn evaluate_expression(&self, expr: &str) -> ParserResult<bool> {
        let expr = expr.trim();
        
        // Try to parse as boolean expression with AND/OR first (they can contain other expressions)
        if let Some(boolean_result) = self.evaluate_boolean_expression(expr) {
            return Ok(boolean_result);
        }
        
        // Handle Defined(SYMBOL) function
        if expr.starts_with("Defined(") && expr.ends_with(')') {
            let symbol = expr[8..expr.len()-1].trim().to_uppercase();
            return Ok(self.defined_symbols.contains(&symbol));
        }
        
        // Handle NOT Defined(SYMBOL)
        if expr.starts_with("NOT ") {
            let rest = expr[4..].trim();
            if rest.starts_with("Defined(") && rest.ends_with(')') {
                let symbol = rest[8..rest.len()-1].trim().to_uppercase();
                return Ok(!self.defined_symbols.contains(&symbol));
            }
        }
        
        // Handle boolean literals
        if expr.eq_ignore_ascii_case("TRUE") {
            return Ok(true);
        }
        if expr.eq_ignore_ascii_case("FALSE") {
            return Ok(false);
        }
        
        // Handle integer comparisons (e.g., "VER >= 200")
        // For now, we'll support simple comparisons with predefined constants
        // In a full implementation, we'd parse and evaluate arithmetic expressions
        
        // Try to parse as integer comparison
        if let Some(comparison_result) = self.evaluate_integer_comparison(expr) {
            return Ok(comparison_result);
        }
        
        // Default: treat undefined symbols as false, defined as true
        // This allows simple symbol checks like "{$IF DEBUG}"
        let symbol = expr.to_uppercase();
        Ok(self.defined_symbols.contains(&symbol))
    }
    
    /// Evaluate integer comparison expression (e.g., "VER >= 200")
    fn evaluate_integer_comparison(&self, expr: &str) -> Option<bool> {
        // Simple pattern matching for common cases
        // In a full implementation, we'd have a proper expression parser
        
        // Check for comparison operators
        let operators = [">=", "<=", ">", "<", "=", "==", "<>", "!="];
        for op in &operators {
            if let Some(pos) = expr.find(op) {
                let left = expr[..pos].trim();
                let right = expr[pos + op.len()..].trim();
                
                // Try to parse as integers
                if let (Ok(left_val), Ok(right_val)) = (left.parse::<i32>(), right.parse::<i32>()) {
                    return Some(match *op {
                        ">=" => left_val >= right_val,
                        "<=" => left_val <= right_val,
                        ">" => left_val > right_val,
                        "<" => left_val < right_val,
                        "=" | "==" => left_val == right_val,
                        "<>" | "!=" => left_val != right_val,
                        _ => return None,
                    });
                }
                
                // Check if left is a predefined constant (like VER)
                // For now, we'll just return None and let the caller handle it
                // In a full implementation, we'd have a constants table
            }
        }
        
        None
    }
    
    /// Evaluate boolean expression with AND/OR operators
    /// This is called from evaluate_expression, so it should not call evaluate_expression recursively
    fn evaluate_boolean_expression(&self, expr: &str) -> Option<bool> {
        let expr_upper = expr.to_uppercase();
        
        // Handle NOT first (before AND/OR)
        if expr_upper.starts_with("NOT ") {
            let rest = expr_upper[4..].trim();
            // Recursively evaluate the rest (but not through evaluate_expression to avoid circular call)
            if let Some(val) = self.evaluate_boolean_expression(rest) {
                return Some(!val);
            }
            // If not a boolean expression, try simple cases
            if rest.starts_with("Defined(") && rest.ends_with(')') {
                let symbol = rest[8..rest.len()-1].trim().to_uppercase();
                return Some(!self.defined_symbols.contains(&symbol));
            }
            if rest.eq_ignore_ascii_case("TRUE") {
                return Some(false);
            }
            if rest.eq_ignore_ascii_case("FALSE") {
                return Some(true);
            }
            // Check if it's a simple symbol
            if self.defined_symbols.contains(&rest.to_uppercase()) {
                return Some(false);
            }
            return Some(true); // NOT undefined symbol = true
        }
        
        // Split by OR first (lower precedence)
        if expr_upper.contains(" OR ") {
            let parts: Vec<&str> = expr_upper.split(" OR ").collect();
            let mut result = false;
            for part in parts {
                // Each part might contain AND, so evaluate it recursively
                let part_result = if part.contains(" AND ") {
                    self.evaluate_boolean_expression(part.trim())
                } else {
                    self.evaluate_simple_expression(part.trim())
                };
                if let Some(val) = part_result {
                    result = result || val;
                } else {
                    return None;
                }
            }
            return Some(result);
        }
        
        // Split by AND (higher precedence)
        if expr_upper.contains(" AND ") {
            let parts: Vec<&str> = expr_upper.split(" AND ").collect();
            let mut result = true;
            for part in parts {
                if let Some(val) = self.evaluate_simple_expression(part.trim()) {
                    result = result && val;
                } else {
                    return None;
                }
            }
            return Some(result);
        }
        
        None
    }
    
    /// Evaluate a simple expression (no AND/OR operators)
    /// This is a helper to avoid circular calls
    fn evaluate_simple_expression(&self, expr: &str) -> Option<bool> {
        let expr = expr.trim();
        let expr_upper = expr.to_uppercase();
        
        // Handle Defined(SYMBOL) function (case-insensitive for "Defined")
        if expr_upper.starts_with("DEFINED(") && expr.ends_with(')') {
            // Find the opening parenthesis (case-insensitive)
            let open_paren = expr_upper.find('(').unwrap_or(0);
            let close_paren = expr.len() - 1;
            let symbol = expr[open_paren + 1..close_paren].trim().to_uppercase();
            return Some(self.defined_symbols.contains(&symbol));
        }
        
        // Handle boolean literals
        if expr_upper == "TRUE" {
            return Some(true);
        }
        if expr_upper == "FALSE" {
            return Some(false);
        }
        
        // Handle integer comparisons
        if let Some(comparison_result) = self.evaluate_integer_comparison(expr) {
            return Some(comparison_result);
        }
        
        // Default: treat as symbol check
        Some(self.defined_symbols.contains(&expr_upper))
    }
}

impl Default for DirectiveEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ifdef() {
        let directive = DirectiveEvaluator::parse_directive("IFDEF DEBUG");
        assert!(matches!(directive, DirectiveType::IfDef(ref s) if s == "DEBUG"));
    }

    #[test]
    fn test_parse_ifndef() {
        let directive = DirectiveEvaluator::parse_directive("IFNDEF RELEASE");
        assert!(matches!(directive, DirectiveType::IfNDef(ref s) if s == "RELEASE"));
    }

    #[test]
    fn test_parse_else() {
        let directive = DirectiveEvaluator::parse_directive("ELSE");
        assert!(matches!(directive, DirectiveType::Else));
    }

    #[test]
    fn test_parse_endif() {
        let directive = DirectiveEvaluator::parse_directive("ENDIF");
        assert!(matches!(directive, DirectiveType::EndIf));
    }

    #[test]
    fn test_parse_define() {
        let directive = DirectiveEvaluator::parse_directive("DEFINE FOO");
        assert!(matches!(directive, DirectiveType::Define(ref s) if s == "FOO"));
    }

    #[test]
    fn test_parse_undef() {
        let directive = DirectiveEvaluator::parse_directive("UNDEF FOO");
        assert!(matches!(directive, DirectiveType::Undef(ref s) if s == "FOO"));
    }

    #[test]
    fn test_evaluate_ifdef_true() {
        let mut evaluator = DirectiveEvaluator::with_symbols(vec!["DEBUG".to_string()]);
        let directive = DirectiveEvaluator::parse_directive("IFDEF DEBUG");
        let (include, skip) = evaluator.evaluate(&directive, Span::at(0, 1, 1)).unwrap();
        assert!(include);
        assert!(!skip);
        assert!(evaluator.is_active());
    }

    #[test]
    fn test_evaluate_ifdef_false() {
        let mut evaluator = DirectiveEvaluator::new();
        let directive = DirectiveEvaluator::parse_directive("IFDEF DEBUG");
        let (include, skip) = evaluator.evaluate(&directive, Span::at(0, 1, 1)).unwrap();
        assert!(!include);
        assert!(skip);
        assert!(!evaluator.is_active());
    }

    #[test]
    fn test_evaluate_ifndef_true() {
        let mut evaluator = DirectiveEvaluator::new();
        let directive = DirectiveEvaluator::parse_directive("IFNDEF DEBUG");
        let (include, skip) = evaluator.evaluate(&directive, Span::at(0, 1, 1)).unwrap();
        assert!(include);
        assert!(!skip);
        assert!(evaluator.is_active());
    }

    #[test]
    fn test_evaluate_ifndef_false() {
        let mut evaluator = DirectiveEvaluator::with_symbols(vec!["DEBUG".to_string()]);
        let directive = DirectiveEvaluator::parse_directive("IFNDEF DEBUG");
        let (include, skip) = evaluator.evaluate(&directive, Span::at(0, 1, 1)).unwrap();
        assert!(!include);
        assert!(skip);
        assert!(!evaluator.is_active());
    }

    #[test]
    fn test_evaluate_else() {
        let mut evaluator = DirectiveEvaluator::new();
        // Start with IFDEF that's false
        let ifdef = DirectiveEvaluator::parse_directive("IFDEF DEBUG");
        evaluator.evaluate(&ifdef, Span::at(0, 1, 1)).unwrap();
        assert!(!evaluator.is_active());
        
        // ELSE should make us active
        let else_directive = DirectiveEvaluator::parse_directive("ELSE");
        let (include, skip) = evaluator.evaluate(&else_directive, Span::at(0, 1, 1)).unwrap();
        assert!(include);
        assert!(!skip);
        assert!(evaluator.is_active());
    }

    #[test]
    fn test_evaluate_endif() {
        let mut evaluator = DirectiveEvaluator::new();
        let ifdef = DirectiveEvaluator::parse_directive("IFDEF DEBUG");
        evaluator.evaluate(&ifdef, Span::at(0, 1, 1)).unwrap();
        assert!(!evaluator.is_active());
        
        let endif = DirectiveEvaluator::parse_directive("ENDIF");
        let (include, skip) = evaluator.evaluate(&endif, Span::at(0, 1, 1)).unwrap();
        assert!(include);
        assert!(!skip);
        assert!(evaluator.is_active()); // Back to active after ENDIF
        assert!(!evaluator.has_unmatched_conditionals());
    }

    #[test]
    fn test_evaluate_define() {
        let mut evaluator = DirectiveEvaluator::new();
        let directive = DirectiveEvaluator::parse_directive("DEFINE FOO");
        evaluator.evaluate(&directive, Span::at(0, 1, 1)).unwrap();
        assert!(evaluator.is_defined("FOO"));
    }

    #[test]
    fn test_evaluate_undef() {
        let mut evaluator = DirectiveEvaluator::with_symbols(vec!["FOO".to_string()]);
        assert!(evaluator.is_defined("FOO"));
        let directive = DirectiveEvaluator::parse_directive("UNDEF FOO");
        evaluator.evaluate(&directive, Span::at(0, 1, 1)).unwrap();
        assert!(!evaluator.is_defined("FOO"));
    }

    #[test]
    fn test_nested_conditionals() {
        let mut evaluator = DirectiveEvaluator::with_symbols(vec!["OUTER".to_string()]);
        
        // Outer IFDEF (true)
        let outer = DirectiveEvaluator::parse_directive("IFDEF OUTER");
        evaluator.evaluate(&outer, Span::at(0, 1, 1)).unwrap();
        assert!(evaluator.is_active());
        
        // Inner IFDEF (false)
        let inner = DirectiveEvaluator::parse_directive("IFDEF INNER");
        evaluator.evaluate(&inner, Span::at(0, 1, 1)).unwrap();
        assert!(!evaluator.is_active());
        
        // End inner
        let endif_inner = DirectiveEvaluator::parse_directive("ENDIF");
        evaluator.evaluate(&endif_inner, Span::at(0, 1, 1)).unwrap();
        assert!(evaluator.is_active()); // Back to outer active state
        
        // End outer
        let endif_outer = DirectiveEvaluator::parse_directive("ENDIF");
        evaluator.evaluate(&endif_outer, Span::at(0, 1, 1)).unwrap();
        assert!(evaluator.is_active());
        assert!(!evaluator.has_unmatched_conditionals());
    }

    #[test]
    fn test_else_without_ifdef() {
        let mut evaluator = DirectiveEvaluator::new();
        let directive = DirectiveEvaluator::parse_directive("ELSE");
        let result = evaluator.evaluate(&directive, Span::at(0, 1, 1));
        assert!(result.is_err());
    }

    #[test]
    fn test_endif_without_ifdef() {
        let mut evaluator = DirectiveEvaluator::new();
        let directive = DirectiveEvaluator::parse_directive("ENDIF");
        let result = evaluator.evaluate(&directive, Span::at(0, 1, 1));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_if() {
        let directive = DirectiveEvaluator::parse_directive("IF Defined(DEBUG)");
        assert!(matches!(directive, DirectiveType::If(ref s) if s == "Defined(DEBUG)"));
    }

    #[test]
    fn test_parse_elseif() {
        let directive = DirectiveEvaluator::parse_directive("ELSEIF VER >= 200");
        assert!(matches!(directive, DirectiveType::ElseIf(ref s) if s == "VER >= 200"));
    }

    #[test]
    fn test_evaluate_if_defined() {
        let mut evaluator = DirectiveEvaluator::with_symbols(vec!["DEBUG".to_string()]);
        let directive = DirectiveEvaluator::parse_directive("IF Defined(DEBUG)");
        let (include, skip) = evaluator.evaluate(&directive, Span::at(0, 1, 1)).unwrap();
        assert!(include);
        assert!(!skip);
        assert!(evaluator.is_active());
    }

    #[test]
    fn test_evaluate_if_not_defined() {
        let mut evaluator = DirectiveEvaluator::new();
        let directive = DirectiveEvaluator::parse_directive("IF Defined(DEBUG)");
        let (include, skip) = evaluator.evaluate(&directive, Span::at(0, 1, 1)).unwrap();
        assert!(!include);
        assert!(skip);
        assert!(!evaluator.is_active());
    }

    #[test]
    fn test_evaluate_if_not_defined_symbol() {
        let mut evaluator = DirectiveEvaluator::new();
        let directive = DirectiveEvaluator::parse_directive("IF NOT Defined(DEBUG)");
        let (include, skip) = evaluator.evaluate(&directive, Span::at(0, 1, 1)).unwrap();
        assert!(include);
        assert!(!skip);
        assert!(evaluator.is_active());
    }

    #[test]
    fn test_evaluate_if_boolean_true() {
        let mut evaluator = DirectiveEvaluator::new();
        let directive = DirectiveEvaluator::parse_directive("IF TRUE");
        let (include, skip) = evaluator.evaluate(&directive, Span::at(0, 1, 1)).unwrap();
        assert!(include);
        assert!(!skip);
        assert!(evaluator.is_active());
    }

    #[test]
    fn test_evaluate_if_boolean_false() {
        let mut evaluator = DirectiveEvaluator::new();
        let directive = DirectiveEvaluator::parse_directive("IF FALSE");
        let (include, skip) = evaluator.evaluate(&directive, Span::at(0, 1, 1)).unwrap();
        assert!(!include);
        assert!(skip);
        assert!(!evaluator.is_active());
    }

    #[test]
    fn test_evaluate_if_integer_comparison() {
        let mut evaluator = DirectiveEvaluator::new();
        let directive = DirectiveEvaluator::parse_directive("IF 200 >= 100");
        let (include, skip) = evaluator.evaluate(&directive, Span::at(0, 1, 1)).unwrap();
        assert!(include);
        assert!(!skip);
        assert!(evaluator.is_active());
    }

    #[test]
    fn test_evaluate_elseif() {
        let mut evaluator = DirectiveEvaluator::new();
        // Start with IF that's false
        let if_directive = DirectiveEvaluator::parse_directive("IF Defined(DEBUG)");
        evaluator.evaluate(&if_directive, Span::at(0, 1, 1)).unwrap();
        assert!(!evaluator.is_active());
        
        // ELSEIF with true expression should make us active
        let elseif_directive = DirectiveEvaluator::parse_directive("ELSEIF TRUE");
        let (include, skip) = evaluator.evaluate(&elseif_directive, Span::at(0, 1, 1)).unwrap();
        assert!(include);
        assert!(!skip);
        assert!(evaluator.is_active());
    }

    #[test]
    fn test_evaluate_elseif_after_active() {
        let mut evaluator = DirectiveEvaluator::with_symbols(vec!["DEBUG".to_string()]);
        // Start with IF that's true
        let if_directive = DirectiveEvaluator::parse_directive("IF Defined(DEBUG)");
        evaluator.evaluate(&if_directive, Span::at(0, 1, 1)).unwrap();
        assert!(evaluator.is_active());
        
        // ELSEIF should be inactive since we're already active
        let elseif_directive = DirectiveEvaluator::parse_directive("ELSEIF TRUE");
        let (include, skip) = evaluator.evaluate(&elseif_directive, Span::at(0, 1, 1)).unwrap();
        assert!(!include);
        assert!(skip);
        assert!(!evaluator.is_active());
    }

    #[test]
    fn test_evaluate_if_and_expression() {
        let mut evaluator = DirectiveEvaluator::with_symbols(vec!["DEBUG".to_string(), "TEST".to_string()]);
        let directive = DirectiveEvaluator::parse_directive("IF Defined(DEBUG) AND Defined(TEST)");
        let (include, skip) = evaluator.evaluate(&directive, Span::at(0, 1, 1)).unwrap();
        assert!(include);
        assert!(!skip);
        assert!(evaluator.is_active());
    }

    #[test]
    fn test_evaluate_if_or_expression() {
        let mut evaluator = DirectiveEvaluator::with_symbols(vec!["DEBUG".to_string()]);
        let directive = DirectiveEvaluator::parse_directive("IF Defined(DEBUG) OR Defined(RELEASE)");
        let (include, skip) = evaluator.evaluate(&directive, Span::at(0, 1, 1)).unwrap();
        assert!(include);
        assert!(!skip);
        assert!(evaluator.is_active());
    }

    #[test]
    fn test_if_elseif_endif_flow() {
        let mut evaluator = DirectiveEvaluator::new();
        
        // IF FALSE - inactive
        let if_directive = DirectiveEvaluator::parse_directive("IF FALSE");
        let (include, skip) = evaluator.evaluate(&if_directive, Span::at(0, 1, 1)).unwrap();
        assert!(!include);
        assert!(skip);
        assert!(!evaluator.is_active());
        
        // ELSEIF TRUE - should become active
        let elseif_directive = DirectiveEvaluator::parse_directive("ELSEIF TRUE");
        let (include, skip) = evaluator.evaluate(&elseif_directive, Span::at(0, 1, 1)).unwrap();
        assert!(include);
        assert!(!skip);
        assert!(evaluator.is_active());
        
        // ENDIF - back to active
        let endif_directive = DirectiveEvaluator::parse_directive("ENDIF");
        let (include, skip) = evaluator.evaluate(&endif_directive, Span::at(0, 1, 1)).unwrap();
        assert!(include);
        assert!(!skip);
        assert!(evaluator.is_active());
        assert!(!evaluator.has_unmatched_conditionals());
    }

    #[test]
    fn test_if_elseif_elseif_flow() {
        let mut evaluator = DirectiveEvaluator::new();
        
        // IF FALSE - inactive
        let if_directive = DirectiveEvaluator::parse_directive("IF FALSE");
        evaluator.evaluate(&if_directive, Span::at(0, 1, 1)).unwrap();
        assert!(!evaluator.is_active());
        
        // ELSEIF FALSE - still inactive
        let elseif1 = DirectiveEvaluator::parse_directive("ELSEIF FALSE");
        let (include, skip) = evaluator.evaluate(&elseif1, Span::at(0, 1, 1)).unwrap();
        assert!(!include);
        assert!(skip);
        assert!(!evaluator.is_active());
        
        // ELSEIF TRUE - should become active
        let elseif2 = DirectiveEvaluator::parse_directive("ELSEIF TRUE");
        let (include, skip) = evaluator.evaluate(&elseif2, Span::at(0, 1, 1)).unwrap();
        assert!(include);
        assert!(!skip);
        assert!(evaluator.is_active());
    }
}

