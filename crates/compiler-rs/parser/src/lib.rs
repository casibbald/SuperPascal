//! SuperPascal Parser
//!
//! This crate implements a recursive descent parser for SuperPascal.
//! It builds an AST from tokens produced by the lexer.

mod classes;
mod core;
mod declarations;
mod directives;
mod expressions;
pub mod incremental;
mod properties;
pub mod query;
mod statements;
mod types;
mod units;

use ast::Node;
use errors::{CodeSnippet, Diagnostic, ParseErrors, ParserError, ParserResult};
use lexer::Lexer;
use tokens::{Span, Token, TokenKind};

use crate::directives::DirectiveEvaluator;

/// Parser for SuperPascal programs
pub struct Parser {
    source: String,
    lexer: Lexer,
    current: Option<Token>,
    peek: Option<Token>,
    filename: Option<String>,
    directive_evaluator: DirectiveEvaluator,
    /// Track included files to prevent circular includes
    included_files: std::collections::HashSet<String>,
    /// Include search paths for resolving relative file paths
    include_paths: Vec<String>,
    /// Number of lines of context to include above/below error line
    context_lines: usize,
}

impl Parser {
    /// Create a new parser from source code
    pub fn new(source: &str) -> ParserResult<Self> {
        Self::new_with_file(source, None)
    }

    /// Create a new parser from source code with filename
    pub fn new_with_file(source: &str, filename: Option<String>) -> ParserResult<Self> {
        Self::new_with_file_and_symbols(source, filename, vec![])
    }

    /// Create a new parser from source code with filename and predefined symbols
    pub fn new_with_file_and_symbols(
        source: &str,
        filename: Option<String>,
        predefined_symbols: Vec<String>,
    ) -> ParserResult<Self> {
        let lexer = Lexer::new(source);
        let mut included_files = std::collections::HashSet::new();
        // Add current file to included files to prevent self-inclusion
        if let Some(ref fname) = filename {
            included_files.insert(fname.clone());
        }
        let mut parser = Self {
            source: source.to_string(),
            lexer,
            current: None,
            peek: None,
            filename: filename.clone(),
            directive_evaluator: DirectiveEvaluator::with_symbols(predefined_symbols),
            included_files,
            include_paths: vec![],
            context_lines: 1,
        };
        // Prime the parser with first two tokens
        parser.advance()?;
        parser.advance()?;
        Ok(parser)
    }

    /// Add an include search path
    pub fn add_include_path(&mut self, path: String) {
        self.include_paths.push(path);
    }

    /// Set include search paths
    pub fn set_include_paths(&mut self, paths: Vec<String>) {
        self.include_paths = paths;
    }

    /// Get mutable reference to directive evaluator
    pub(crate) fn directive_evaluator_mut(&mut self) -> &mut DirectiveEvaluator {
        &mut self.directive_evaluator
    }

    /// Get reference to directive evaluator
    pub(crate) fn directive_evaluator(&self) -> &DirectiveEvaluator {
        &self.directive_evaluator
    }

    /// Convert a ParserError to an enhanced Diagnostic
    pub fn error_to_diagnostic(&self, error: &ParserError) -> Diagnostic {
        let mut diag = error.to_diagnostic(self.filename.clone());

        // Add enhanced context based on error type
        match error {
            ParserError::UnexpectedToken {
                expected,
                found,
                span,
            } => {
                // Add suggestion for common mistakes
                if expected == "identifier" && found.contains("keyword") {
                    diag = diag.with_suggestion(format!(
                        "Keywords cannot be used as identifiers. Use a different name."
                    ));
                } else if expected == ";" && found.contains("end") {
                    diag = diag.with_suggestion(format!(
                        "Missing semicolon before 'end'. Add ';' after the previous statement."
                    ));
                } else if expected == ":" && found.contains("=") {
                    diag = diag.with_suggestion(format!(
                        "Did you mean ':' instead of '='? Variable declarations use ':' for type."
                    ));
                } else if expected == "=" && found.contains(":") {
                    diag = diag.with_suggestion(format!(
                        "Did you mean '=' instead of ':'? Assignments use '=' (or ':=' in Pascal)."
                    ));
                } else if expected == ":=" && found.contains("=") {
                    diag = diag.with_suggestion(format!(
                        "Pascal uses ':=' for assignment, not '='. Change '=' to ':='."
                    ));
                } else if expected == "BEGIN" && found.contains("begin") {
                    diag = diag.with_suggestion(
                        format!("Pascal is case-sensitive for keywords. Use 'BEGIN' (uppercase) or ensure your lexer handles case correctly.")
                    );
                } else if expected == "END" && found.contains("end") {
                    diag = diag.with_suggestion(
                        format!("Pascal is case-sensitive for keywords. Use 'END' (uppercase) or ensure your lexer handles case correctly.")
                    );
                } else if expected == ")" && found.contains("]") {
                    diag = diag.with_suggestion(format!(
                        "Mismatched bracket. Use ')' to close a function call or expression."
                    ));
                } else if expected == "]" && found.contains(")") {
                    diag = diag.with_suggestion(format!(
                        "Mismatched bracket. Use ']' to close an array index."
                    ));
                } else if expected == "THEN" && found.contains("then") {
                    diag = diag.with_suggestion(format!(
                        "Pascal is case-sensitive. Use 'THEN' (uppercase) after IF condition."
                    ));
                } else if expected == "DO" && found.contains("do") {
                    diag = diag.with_suggestion(format!(
                        "Pascal is case-sensitive. Use 'DO' (uppercase) after FOR/WHILE condition."
                    ));
                }

                // Add code snippet if we have source available
                if let Some(source) = self.get_source_snippet(*span) {
                    diag = diag.with_code_snippet(source);
                }
            }
            ParserError::UnexpectedEof { expected, span: _ } => {
                let suggestion = match expected.as_str() {
                    "END" => "Add 'END' to close the block, procedure, or function.",
                    ";" => "Add ';' to terminate the statement.",
                    "." => "Add '.' to end the program.",
                    ")" => "Add ')' to close the expression or parameter list.",
                    "]" => "Add ']' to close the array index.",
                    "}" => "Add '}' to close the comment.",
                    "THEN" => "Add 'THEN' after the IF condition.",
                    "DO" => "Add 'DO' after the FOR/WHILE condition.",
                    _ => &format!("Add '{}' before the end of file.", expected),
                };
                diag = diag.with_suggestion(suggestion.to_string());

                // Add explanation
                diag = diag.with_explanation(
                    format!(
                        "The file ended unexpectedly while the parser was expecting '{}'. \
                         This usually means a block, statement, or expression was not properly closed.",
                        expected
                    )
                );
            }
            ParserError::InvalidSyntax { message, span } => {
                // Try to provide helpful context
                if message.contains("statement") {
                    diag = diag.with_suggestion(
                        "Check that all statements are properly terminated with semicolons."
                            .to_string(),
                    );
                } else if message.contains("expression") {
                    diag = diag.with_suggestion(
                        "Check that the expression is complete and all parentheses are balanced."
                            .to_string(),
                    );
                } else if message.contains("type") {
                    diag = diag.with_suggestion(
                        "Check that the type name is correct and the type is declared.".to_string(),
                    );
                } else if message.contains("Lexer error") {
                    diag = diag.with_suggestion(
                        "Check for invalid characters or syntax in the source code.".to_string(),
                    );
                }

                // Add code snippet if available
                if let Some(source) = self.get_source_snippet(*span) {
                    diag = diag.with_code_snippet(source);
                }
            }
        }

        diag
    }

    /// Get a code snippet around the error location for display
    fn get_source_snippet(&self, span: Span) -> Option<CodeSnippet> {
        if self.source.is_empty() {
            return None;
        }

        let lines: Vec<&str> = self.source.lines().collect();

        // Find which line the error starts on (span.line is 1-based)
        let error_line_idx = (span.line as usize).saturating_sub(1);
        if error_line_idx >= lines.len() {
            return None;
        }

        // Determine line range to include (with context)
        let start_line = error_line_idx.saturating_sub(self.context_lines);
        let end_line = (error_line_idx + 1 + self.context_lines).min(lines.len());

        // Build the snippet lines
        let mut snippet_lines = Vec::new();
        for i in start_line..end_line {
            let line_num = i + 1; // 1-based line numbers
            let line_content = lines[i].to_string();
            snippet_lines.push((line_num, line_content));
        }

        // Calculate the column offset for highlighting
        // span.column is 1-based
        let _highlight_start_col = span.column - 1;

        // Build the highlight span using the same line numbers
        let highlight_span = Span {
            start: span.start,
            end: span.end,
            line: span.line,
            column: span.column,
        };

        Some(CodeSnippet {
            lines: snippet_lines,
            highlight_span,
        })
    }

    // Core functionality is in core.rs

    /// Parse a complete program, unit, or library (fails on first error)
    pub fn parse(&mut self) -> ParserResult<Node> {
        // Handle directives before PROGRAM/UNIT/LIBRARY
        // Directives may wrap the program declaration
        while self.check(&TokenKind::Directive(String::new())) {
            // Process directive but don't collect it here - let parse_program/parse_unit handle it
            let _ = self.parse_directive()?;
        }

        // Skip tokens if we're in an inactive conditional branch
        while !self.directive_evaluator().is_active() {
            if self.check(&TokenKind::Directive(String::new())) {
                let _ = self.parse_directive()?;
                continue;
            } else if self.check(&TokenKind::Eof) {
                let span = self
                    .current()
                    .map(|t| t.span)
                    .unwrap_or_else(|| Span::at(0, 1, 1));
                return Err(ParserError::InvalidSyntax {
                    message: "Unmatched {$IFDEF} or {$IFNDEF} - reached end of file".to_string(),
                    span,
                });
            } else {
                self.advance()?;
            }
        }

        // Check what we're parsing: PROGRAM, UNIT, or LIBRARY
        if self.check(&TokenKind::KwProgram) {
            self.parse_program()
        } else if self.check(&TokenKind::KwUnit) {
            self.parse_unit()
        } else if self.check(&TokenKind::KwLibrary) {
            self.parse_library()
        } else {
            let span = self
                .current()
                .map(|t| t.span)
                .unwrap_or_else(|| Span::at(0, 1, 1));
            Err(ParserError::InvalidSyntax {
                message: "Expected PROGRAM, UNIT, or LIBRARY".to_string(),
                span,
            })
        }
    }

    // Program parsing is in declarations.rs
    // Unit/module parsing is in units.rs

    // Declaration parsing is in declarations.rs
    // All declaration functions have been moved to declarations.rs module

    // Function, parameter parsing is in declarations.rs

    // Type parsing is in types.rs

    // Class parsing is in classes.rs

    // Field declaration parsing is in types.rs

    // Statement parsing is in statements.rs

    /// Parse a complete program, unit, or library, collecting all errors instead of failing on first
    pub fn parse_with_errors(&mut self) -> ParseErrors {
        let mut errors = ParseErrors::new();

        // Handle directives
        let mut has_error = false;
        while self.check(&TokenKind::Directive(String::new())) {
            if let Err(e) = self.parse_directive() {
                errors.push(e);
                has_error = true;
            }
        }

        // Skip tokens if we're in an inactive conditional branch
        while !self.directive_evaluator().is_active() {
            if self.check(&TokenKind::Directive(String::new())) {
                if let Err(e) = self.parse_directive() {
                    errors.push(e);
                }
                continue;
            } else if self.check(&TokenKind::Eof) {
                let span = self
                    .current()
                    .map(|t| t.span)
                    .unwrap_or_else(|| Span::at(0, 1, 1));
                errors.push(ParserError::InvalidSyntax {
                    message: "Unmatched {$IFDEF} or {$IFNDEF} - reached end of file".to_string(),
                    span,
                });
                has_error = true;
                break;
            } else {
                let _ = self.advance();
            }
        }

        // Check what we're parsing: PROGRAM, UNIT, or LIBRARY
        if self.check(&TokenKind::KwProgram) {
            if let Err(e) = self.parse_program() {
                errors.push(e);
                has_error = true;
            }
        } else if self.check(&TokenKind::KwUnit) {
            if let Err(e) = self.parse_unit() {
                errors.push(e);
                has_error = true;
            }
        } else if self.check(&TokenKind::KwLibrary) {
            if let Err(e) = self.parse_library() {
                errors.push(e);
                has_error = true;
            }
        } else {
            let span = self
                .current()
                .map(|t| t.span)
                .unwrap_or_else(|| Span::at(0, 1, 1));
            errors.push(ParserError::InvalidSyntax {
                message: "Expected PROGRAM, UNIT, or LIBRARY".to_string(),
                span,
            });
            has_error = true;
        }

        // If there were errors, try to recover by skipping to a sync point and continuing
        if has_error && !errors.is_empty() {
            self.skip_to_sync_point();
        }

        errors
    }

    /// Skip tokens until we find a sync point for error recovery
    /// Sync points are keywords that mark known safe places to restart parsing
    fn skip_to_sync_point(&mut self) {
        let sync_keywords = [
            TokenKind::KwEnd,
            TokenKind::KwBegin,
            TokenKind::KwVar,
            TokenKind::KwConst,
            TokenKind::KwType,
            TokenKind::KwProcedure,
            TokenKind::KwFunction,
            TokenKind::KwProgram,
            TokenKind::KwUnit,
            TokenKind::KwLibrary,
            TokenKind::KwUses,
            TokenKind::KwImplementation,
            TokenKind::KwInterface,
        ];

        while !self.check(&TokenKind::Eof) {
            if sync_keywords.iter().any(|k| self.check(k)) {
                return; // Found sync point
            }
            let _ = self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use errors::{CodeSnippet, Diagnostic, ParserError};
    use tokens::Span;

    // Declaration tests (nested routines, method declarations, simple program) are in declarations.rs
    // Unit/library tests are in units.rs

    // Property tests are in properties.rs
    // Pointer type tests are in types.rs
    // Class type tests are in classes.rs

    #[test]
    fn test_get_source_snippet_returns_single_line() {
        let source = "PROGRAM Hello;\nVAR X: INTEGER;\nBEGIN\n  X := 10;\nEND.\n";
        let mut parser = Parser::new_with_file(source, Some("test.pas".to_string())).unwrap();
        // Set context_lines = 0 to test bare single-line snippet
        parser.context_lines = 0;

        // Error on line 4, column 3 (X := 10)
        let span = Span::new(30, 35, 4, 3);
        let snippet = parser
            .get_source_snippet(span)
            .expect("should return snippet");

        assert_eq!(snippet.lines.len(), 1);
        assert_eq!(snippet.lines[0].0, 4);
        assert_eq!(snippet.lines[0].1, "  X := 10;");
    }

    #[test]
    fn test_get_source_snippet_with_context() {
        let source = "PROGRAM Test;\nVAR A: INTEGER;\nBEGIN\n  A := \"hello\";\nEND.\n";
        let parser = Parser::new_with_file(source, Some("test.pas".to_string())).unwrap();

        // Set context_lines to 1
        let mut parser = parser;
        parser.context_lines = 1;

        // Error on line 4 (A := "hello")
        let span = Span::new(28, 37, 4, 3);
        let snippet = parser
            .get_source_snippet(span)
            .expect("should return snippet");

        // Should include 1 line above + error line + 1 line below = 3 lines
        assert_eq!(snippet.lines.len(), 3);
        assert_eq!(snippet.lines[0].0, 3);
        assert_eq!(snippet.lines[0].1, "BEGIN");
        assert_eq!(snippet.lines[1].0, 4);
        assert!(snippet.lines[1].1.contains(":= \"hello\""));
        assert_eq!(snippet.lines[2].0, 5);
        assert_eq!(snippet.lines[2].1, "END.");
    }

    #[test]
    fn test_get_source_snippet_at_start() {
        use tokens::Span;

        let source = "PROGRAM Test;\nVAR X: INTEGER;\nBEGIN\n  X := 10;\nEND.\n";
        let parser = Parser::new_with_file(source, None).unwrap();

        // Error on first line
        let span = Span::new(0, 6, 1, 1);
        let snippet = parser
            .get_source_snippet(span)
            .expect("should return snippet");

        assert_eq!(snippet.lines.len(), 2); // program line + context line below
        assert_eq!(snippet.lines[0].0, 1);
        assert_eq!(snippet.lines[0].1, "PROGRAM Test;");
    }

    #[test]
    fn test_error_to_diagnostic_includes_snippet() {
        let source = "PROGRAM Test;\nBEGIN\n  X := \"hello\";\nEND.\n";
        let parser = Parser::new_with_file(source, Some("test.pas".to_string())).unwrap();

        // Simulate an UnexpectedToken error
        let error = ParserError::UnexpectedToken {
            expected: ";".to_string(),
            found: "\"hello\"".to_string(),
            span: Span::new(20, 30, 3, 10),
        };

        let diag = parser.error_to_diagnostic(&error);

        // The diagnostic should now include a code snippet
        assert!(
            diag.code_snippet.is_some(),
            "Diagnostic should include a code snippet"
        );
        if let Some(ref snippet) = diag.code_snippet {
            assert!(!snippet.lines.is_empty(), "Snippet should have lines");
            // Should include the error line with context
            assert!(
                snippet
                    .lines
                    .iter()
                    .any(|(_, line)| line.contains("X := \"hello\"")),
                "Snippet should include the error line"
            );
        }
    }

    #[test]
    fn test_error_diagnostic_formatting() {
        let source = "PROGRAM BadType;\nVAR X: INTEGER;\nBEGIN\n  X := \"hello\";\nEND.\n";
        let parser = Parser::new_with_file(source, Some("test.pas".to_string())).unwrap();

        let error = ParserError::UnexpectedToken {
            expected: ";".to_string(),
            found: "\"hello\"".to_string(),
            span: Span::new(25, 35, 4, 10),
        };

        let diag = parser.error_to_diagnostic(&error);

        // Verify the diagnostic has all expected fields populated
        assert!(diag.file.as_ref().is_some_and(|f| f == "test.pas"));
        assert!(
            diag.suggestion.is_some(),
            "Should have a suggestion for unexpected token"
        );
        assert!(diag.code_snippet.is_some(), "Should have a code snippet");
        assert!(!diag.message.is_empty(), "Message should not be empty");
    }

    #[test]
    fn test_parse_with_errors_collects_multiple_errors() {
        // Missing semicolons aren't errors for the parser, but missing END/period is
        let source = "PROGRAM MultiError;\nVAR X: INTEGER;\nBEGIN\n  X := 1;\n  X := 2\nEND"; // Missing period at end
        let mut parser = Parser::new_with_file(source, Some("multi.pas".to_string())).unwrap();
        let errors = parser.parse_with_errors();

        // Should have found at least one error (missing period after END)
        assert!(!errors.is_empty(), "Should have collected errors");
    }

    #[test]
    fn test_parse_with_errors_no_errors() {
        let source = "PROGRAM Valid;\nVAR X: INTEGER;\nBEGIN\n  X := 10;\nEND.\n";
        let mut parser = Parser::new_with_file(source, Some("valid.pas".to_string())).unwrap();
        let errors = parser.parse_with_errors();

        assert!(errors.is_empty(), "Should have no errors for valid program");
    }

    #[test]
    fn test_parse_with_errors_returns_error_count() {
        let source = "PROGRAM Bad;\nBEGIN"; // Missing everything after BEGIN
        let mut parser = Parser::new_with_file(source, Some("bad.pas".to_string())).unwrap();
        let errors = parser.parse_with_errors();

        assert!(!errors.is_empty(), "Should detect at least one error");
        assert!(errors.len() >= 1);
    }

    #[test]
    fn test_parse_with_errors_empty_source() {
        let source = "";
        let mut parser = Parser::new_with_file(source, None).unwrap();
        let errors = parser.parse_with_errors();

        assert!(
            !errors.is_empty(),
            "Empty source should produce an error (expected PROGRAM)"
        );
    }

    #[test]
    fn test_parse_errors_format_fpc() {
        let source = "PROGRAM Bad;\nBEGIN";
        let mut parser = Parser::new_with_file(source, Some("test.pas".to_string())).unwrap();
        let errors = parser.parse_with_errors();

        if !errors.is_empty() {
            let formatted = errors.format_fpc(Some("test.pas".to_string()));
            assert!(formatted.contains("Error"));
            assert!(formatted.contains("test.pas"));
        }
    }
}
