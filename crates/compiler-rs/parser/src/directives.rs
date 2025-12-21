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
    pub fn is_defined(&self, symbol: &str) -> bool {
        self.defined_symbols.contains(&symbol.to_uppercase())
    }

    /// Get all defined symbols (for testing/debugging)
    pub fn defined_symbols(&self) -> &HashSet<String> {
        &self.defined_symbols
    }

    /// Check if there are unmatched conditionals
    pub fn has_unmatched_conditionals(&self) -> bool {
        !self.conditional_stack.is_empty()
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
}

