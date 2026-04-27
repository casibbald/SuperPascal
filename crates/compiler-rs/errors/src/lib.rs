//! SuperPascal Error Handling
//!
//! This crate provides error types and error reporting for the SuperPascal compiler.
//! Errors are designed to match FreePascal's format while providing enhanced diagnostics.

use tokens::Span;

/// Error severity levels (matching FreePascal)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Informational note (not an error)
    Note,
    /// Hint for improvement
    Hint,
    /// Warning (compilation continues)
    Warning,
    /// Error (compilation continues to find more)
    Error,
    /// Fatal error (compilation stops)
    Fatal,
}

impl ErrorSeverity {
    /// Get severity as string (FPC format)
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorSeverity::Note => "Note",
            ErrorSeverity::Hint => "Hint",
            ErrorSeverity::Warning => "Warning",
            ErrorSeverity::Error => "Error",
            ErrorSeverity::Fatal => "Fatal",
        }
    }
}

/// Related location (for enhanced diagnostics)
#[derive(Debug, Clone, PartialEq)]
pub struct RelatedLocation {
    /// Message describing the relationship
    pub message: String,
    /// Location span
    pub span: Span,
    /// Optional filename (if different from main error)
    pub file: Option<String>,
}

/// Code snippet for error display
#[derive(Debug, Clone, PartialEq)]
pub struct CodeSnippet {
    /// Lines of code: (line_number, content)
    pub lines: Vec<(usize, String)>,
    /// Span to highlight
    pub highlight_span: Span,
}

/// Comprehensive diagnostic (matches FPC format, with enhancements)
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    /// Error severity
    pub severity: ErrorSeverity,
    /// FPC-compatible message
    pub message: String,
    /// Error location
    pub span: Span,
    /// Optional filename
    pub file: Option<String>,

    // Enhanced fields (optional)
    /// Context information (e.g., "In procedure 'X'...")
    pub context: Option<String>,
    /// Suggestion for fixing the error
    pub suggestion: Option<String>,
    /// Related locations (declarations, usages, etc.)
    pub related_locations: Vec<RelatedLocation>,
    /// Code snippet with highlighting
    pub code_snippet: Option<CodeSnippet>,
    /// Detailed explanation
    pub explanation: Option<String>,
}

impl Diagnostic {
    /// Create a new diagnostic
    pub fn new(severity: ErrorSeverity, message: String, span: Span) -> Self {
        Self {
            severity,
            message,
            span,
            file: None,
            context: None,
            suggestion: None,
            related_locations: vec![],
            code_snippet: None,
            explanation: None,
        }
    }

    /// Set the filename
    pub fn with_file(mut self, file: String) -> Self {
        self.file = Some(file);
        self
    }

    /// Add context information
    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }

    /// Add a suggestion
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    /// Add a related location
    pub fn with_related_location(mut self, location: RelatedLocation) -> Self {
        self.related_locations.push(location);
        self
    }

    /// Add a code snippet
    pub fn with_code_snippet(mut self, snippet: CodeSnippet) -> Self {
        self.code_snippet = Some(snippet);
        self
    }

    /// Add an explanation
    pub fn with_explanation(mut self, explanation: String) -> Self {
        self.explanation = Some(explanation);
        self
    }

    /// Format as FPC-compatible message
    pub fn format_fpc(&self) -> String {
        let file = self.file.as_deref().unwrap_or("unknown");
        format!(
            "{}({},{}) {}: {}",
            file,
            self.span.line,
            self.span.column,
            self.severity.as_str(),
            self.message
        )
    }

    /// Format as enhanced message (default)
    pub fn format_enhanced(&self) -> String {
        let mut output = self.format_fpc();

        // Add context
        if let Some(context) = &self.context {
            output.push_str(&format!("\n  └─ {}", context));
        }

        // Add suggestion
        if let Some(suggestion) = &self.suggestion {
            output.push_str(&format!("\n  └─ Suggestion: {}", suggestion));
        }

        // Add related locations
        for location in &self.related_locations {
            let file = location.file.as_deref().unwrap_or("unknown");
            output.push_str(&format!(
                "\n  └─ Related: {} ({}({},{}))",
                location.message, file, location.span.line, location.span.column
            ));
        }

        // Add code snippet
        if let Some(snippet) = &self.code_snippet {
            output.push_str("\n");
            for (line_num, content) in &snippet.lines {
                let marker = if snippet.highlight_span.line == *line_num {
                    ">"
                } else {
                    " "
                };
                output.push_str(&format!("{} {} | {}\n", marker, line_num, content));
            }
        }

        output
    }

    /// Format as verbose message (with full details)
    pub fn format_verbose(&self) -> String {
        let mut output = self.format_enhanced();

        // Add detailed explanation if available (not included in enhanced format)
        if let Some(explanation) = &self.explanation {
            output.push_str(&format!("\n\n  Explanation: {}", explanation));
        }

        output
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Default to enhanced format
        write!(f, "{}", self.format_enhanced())
    }
}

// ===== Legacy Parser Errors (for backward compatibility) =====

/// Parser error (legacy, use Diagnostic instead)
#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    /// Unexpected token
    UnexpectedToken {
        expected: String,
        found: String,
        span: Span,
    },
    /// Unexpected end of file
    UnexpectedEof {
        expected: String,
        span: Span,
    },
    /// Invalid syntax
    InvalidSyntax {
        message: String,
        span: Span,
    },
}

impl ParserError {
    /// Convert to Diagnostic
    pub fn to_diagnostic(&self, file: Option<String>) -> Diagnostic {
        match self {
            ParserError::UnexpectedToken { expected, found, span } => {
                Diagnostic::new(
                    ErrorSeverity::Error,
                    format!("Syntax error, \"{}\" expected but \"{}\" found", expected, found),
                    *span,
                )
                .with_file(file.unwrap_or_else(|| "unknown".to_string()))
                .with_suggestion(format!("Replace \"{}\" with \"{}\"", found, expected))
            }
            ParserError::UnexpectedEof { expected, span } => {
                Diagnostic::new(
                    ErrorSeverity::Fatal,
                    format!("Unexpected end of file, \"{}\" expected", expected),
                    *span,
                )
                .with_file(file.unwrap_or_else(|| "unknown".to_string()))
            }
            ParserError::InvalidSyntax { message, span } => {
                Diagnostic::new(
                    ErrorSeverity::Error,
                    message.clone(),
                    *span,
                )
                .with_file(file.unwrap_or_else(|| "unknown".to_string()))
            }
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format as FPC-compatible for backward compatibility
        let diagnostic = self.to_diagnostic(None);
        write!(f, "{}", diagnostic.format_fpc())
    }
}

impl std::error::Error for ParserError {}

/// Result type for parser operations (legacy)
pub type ParserResult<T> = Result<T, ParserError>;

/// Collection of parser errors with recovery support
#[derive(Debug, Clone, Default)]
pub struct ParseErrors {
    /// All accumulated errors
    pub errors: Vec<ParserError>,
}

impl ParseErrors {
    /// Create a new empty error collection
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Add an error to the collection
    pub fn push(&mut self, error: ParserError) {
        self.errors.push(error);
    }

    /// Check if there are any errors
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get the number of errors
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Get diagnostics for all errors
    pub fn diagnostics(&self, file: Option<String>) -> Vec<Diagnostic> {
        self.errors
            .iter()
            .map(|e| e.to_diagnostic(file.clone()))
            .collect()
    }

    /// Format all errors as FPC-compatible messages
    pub fn format_fpc(&self, file: Option<String>) -> String {
        if self.errors.is_empty() {
            return String::new();
        }

        let count = self.errors.len();
        let mut output = String::new();

        for (i, diag) in self.diagnostics(file).iter().enumerate() {
            if i > 0 {
                output.push('\n');
            }
            output.push_str(&diag.format_fpc());
        }

        // Add summary
        let error_count = self
            .errors
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    ParserError::UnexpectedToken { .. } | ParserError::InvalidSyntax { .. }
                )
            })
            .count();

        if error_count > 0 {
            output.push_str(&format!(
                "\n{} error(s) found",
                error_count
            ));
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Error Severity Tests =====

    #[test]
    fn test_error_severity_strings() {
        assert_eq!(ErrorSeverity::Note.as_str(), "Note");
        assert_eq!(ErrorSeverity::Hint.as_str(), "Hint");
        assert_eq!(ErrorSeverity::Warning.as_str(), "Warning");
        assert_eq!(ErrorSeverity::Error.as_str(), "Error");
        assert_eq!(ErrorSeverity::Fatal.as_str(), "Fatal");
    }

    #[test]
    fn test_error_severity_ordering() {
        assert!(ErrorSeverity::Error > ErrorSeverity::Warning);
        assert!(ErrorSeverity::Warning > ErrorSeverity::Hint);
        assert!(ErrorSeverity::Hint > ErrorSeverity::Note);
        assert!(ErrorSeverity::Fatal > ErrorSeverity::Error);
    }

    #[test]
    fn test_error_severity_equality() {
        assert_eq!(ErrorSeverity::Error, ErrorSeverity::Error);
        assert_ne!(ErrorSeverity::Error, ErrorSeverity::Warning);
    }

    // ===== Diagnostic Creation Tests =====

    #[test]
    fn test_diagnostic_new() {
        let span = Span::new(0, 10, 10, 15);
        let diag = Diagnostic::new(
            ErrorSeverity::Error,
            "Test message".to_string(),
            span,
        );

        assert_eq!(diag.severity, ErrorSeverity::Error);
        assert_eq!(diag.message, "Test message");
        assert_eq!(diag.span, span);
        assert_eq!(diag.file, None);
        assert_eq!(diag.context, None);
        assert_eq!(diag.suggestion, None);
        assert!(diag.related_locations.is_empty());
        assert_eq!(diag.code_snippet, None);
        assert_eq!(diag.explanation, None);
    }

    #[test]
    fn test_diagnostic_builder_chain() {
        let span = Span::new(0, 10, 10, 15);
        let diag = Diagnostic::new(
            ErrorSeverity::Warning,
            "Variable 'x' does not seem to be initialized".to_string(),
            span,
        )
        .with_file("test.pas".to_string())
        .with_context("In procedure 'Test'".to_string())
        .with_suggestion("Initialize 'x' before use".to_string())
        .with_explanation("Uninitialized variables can cause undefined behavior".to_string());

        assert_eq!(diag.severity, ErrorSeverity::Warning);
        assert_eq!(diag.file, Some("test.pas".to_string()));
        assert_eq!(diag.context, Some("In procedure 'Test'".to_string()));
        assert_eq!(diag.suggestion, Some("Initialize 'x' before use".to_string()));
        assert_eq!(diag.explanation, Some("Uninitialized variables can cause undefined behavior".to_string()));
    }

    // ===== FPC Format Tests =====

    #[test]
    fn test_diagnostic_fpc_format_basic() {
        let span = Span::new(0, 10, 10, 15);
        let diag = Diagnostic::new(
            ErrorSeverity::Error,
            "Identifier not found \"xyz\"".to_string(),
            span,
        )
        .with_file("test.pas".to_string());

        let fpc_format = diag.format_fpc();
        assert_eq!(fpc_format, "test.pas(10,15) Error: Identifier not found \"xyz\"");
    }

    #[test]
    fn test_diagnostic_fpc_format_no_file() {
        let span = Span::new(0, 10, 10, 15);
        let diag = Diagnostic::new(
            ErrorSeverity::Error,
            "Test message".to_string(),
            span,
        );

        let fpc_format = diag.format_fpc();
        assert_eq!(fpc_format, "unknown(10,15) Error: Test message");
    }

    #[test]
    fn test_diagnostic_fpc_format_all_severities() {
        let span = Span::new(0, 10, 10, 15);
        let severities = vec![
            (ErrorSeverity::Note, "Note"),
            (ErrorSeverity::Hint, "Hint"),
            (ErrorSeverity::Warning, "Warning"),
            (ErrorSeverity::Error, "Error"),
            (ErrorSeverity::Fatal, "Fatal"),
        ];

        for (severity, expected_str) in severities {
            let diag = Diagnostic::new(severity, "Test".to_string(), span)
                .with_file("test.pas".to_string());
            let formatted = diag.format_fpc();
            assert!(formatted.contains(expected_str), "Expected {} in {}", expected_str, formatted);
        }
    }

    // ===== Enhanced Format Tests =====

    #[test]
    fn test_diagnostic_enhanced_format_basic() {
        let span = Span::new(0, 10, 10, 15);
        let diag = Diagnostic::new(
            ErrorSeverity::Error,
            "Identifier not found \"xyz\"".to_string(),
            span,
        )
        .with_file("test.pas".to_string())
        .with_context("In procedure 'DoSomething' (declared at test.pas:5,3)".to_string())
        .with_suggestion("Did you mean 'xy'? (declared at test.pas:2,10)".to_string());

        let enhanced = diag.format_enhanced();
        assert!(enhanced.contains("test.pas(10,15) Error:"));
        assert!(enhanced.contains("In procedure 'DoSomething'"));
        assert!(enhanced.contains("Did you mean 'xy'?"));
    }

    #[test]
    fn test_diagnostic_enhanced_format_with_related_location() {
        let span = Span::new(0, 10, 10, 15);
        let related = RelatedLocation {
            message: "Variable 'xyz' was used here".to_string(),
            span: Span::new(0, 5, 8, 12),
            file: Some("test.pas".to_string()),
        };

        let diag = Diagnostic::new(
            ErrorSeverity::Error,
            "Identifier not found \"xyz\"".to_string(),
            span,
        )
        .with_file("test.pas".to_string())
        .with_related_location(related);

        let enhanced = diag.format_enhanced();
        assert!(enhanced.contains("Related: Variable 'xyz' was used here"));
        assert!(enhanced.contains("test.pas(8,12)"));
    }

    #[test]
    fn test_diagnostic_enhanced_format_multiple_related_locations() {
        let span = Span::new(0, 10, 10, 15);
        let related1 = RelatedLocation {
            message: "First usage".to_string(),
            span: Span::new(0, 5, 8, 12),
            file: Some("test.pas".to_string()),
        };
        let related2 = RelatedLocation {
            message: "Second usage".to_string(),
            span: Span::new(0, 5, 9, 13),
            file: Some("test.pas".to_string()),
        };

        let diag = Diagnostic::new(
            ErrorSeverity::Error,
            "Test message".to_string(),
            span,
        )
        .with_file("test.pas".to_string())
        .with_related_location(related1)
        .with_related_location(related2);

        let enhanced = diag.format_enhanced();
        assert!(enhanced.contains("First usage"));
        assert!(enhanced.contains("Second usage"));
        assert_eq!(diag.related_locations.len(), 2);
    }

    #[test]
    fn test_diagnostic_enhanced_format_with_code_snippet() {
        let span = Span::new(0, 10, 10, 15);
        let snippet = CodeSnippet {
            lines: vec![
                (8, "  var xyz: integer;".to_string()),
                (9, "  begin".to_string()),
                (10, "    writeln(xyz);".to_string()),
            ],
            highlight_span: Span::new(0, 10, 10, 15),
        };

        let diag = Diagnostic::new(
            ErrorSeverity::Error,
            "Test message".to_string(),
            span,
        )
        .with_file("test.pas".to_string())
        .with_code_snippet(snippet);

        let enhanced = diag.format_enhanced();
        assert!(enhanced.contains("8 |"));
        assert!(enhanced.contains("9 |"));
        assert!(enhanced.contains("10 |"));
        assert!(enhanced.contains("> 10 |")); // Highlight marker
    }

    #[test]
    fn test_diagnostic_enhanced_format_no_enhancements() {
        // Enhanced format should still work even without enhancements
        let span = Span::new(0, 10, 10, 15);
        let diag = Diagnostic::new(
            ErrorSeverity::Error,
            "Simple error".to_string(),
            span,
        )
        .with_file("test.pas".to_string());

        let enhanced = diag.format_enhanced();
        // Should be same as FPC format when no enhancements
        assert_eq!(enhanced, diag.format_fpc());
    }

    // ===== Verbose Format Tests =====

    #[test]
    fn test_diagnostic_verbose_format() {
        let span = Span::new(0, 10, 10, 15);
        let diag = Diagnostic::new(
            ErrorSeverity::Error,
            "Test message".to_string(),
            span,
        )
        .with_file("test.pas".to_string())
        .with_explanation("This is a detailed explanation".to_string());

        let verbose = diag.format_verbose();
        assert!(verbose.contains("Test message"));
        assert!(verbose.contains("This is a detailed explanation"));
    }

    #[test]
    fn test_diagnostic_verbose_format_no_explanation() {
        // Verbose should be same as enhanced if no explanation
        let span = Span::new(0, 10, 10, 15);
        let diag = Diagnostic::new(
            ErrorSeverity::Error,
            "Test message".to_string(),
            span,
        )
        .with_file("test.pas".to_string());

        let verbose = diag.format_verbose();
        let enhanced = diag.format_enhanced();
        assert_eq!(verbose, enhanced);
    }

    // ===== Parser Error Conversion Tests =====

    #[test]
    fn test_parser_error_unexpected_token_to_diagnostic() {
        let span = Span::new(0, 10, 10, 15);
        let parser_err = ParserError::UnexpectedToken {
            expected: "identifier".to_string(),
            found: "keyword".to_string(),
            span,
        };

        let diag = parser_err.to_diagnostic(Some("test.pas".to_string()));
        assert_eq!(diag.severity, ErrorSeverity::Error);
        assert!(diag.message.contains("Syntax error"));
        assert!(diag.message.contains("identifier"));
        assert!(diag.message.contains("keyword"));
        assert!(diag.suggestion.is_some());
        assert_eq!(diag.file, Some("test.pas".to_string()));
    }

    #[test]
    fn test_parser_error_unexpected_eof_to_diagnostic() {
        let span = Span::new(0, 10, 10, 15);
        let parser_err = ParserError::UnexpectedEof {
            expected: "end".to_string(),
            span,
        };

        let diag = parser_err.to_diagnostic(Some("test.pas".to_string()));
        assert_eq!(diag.severity, ErrorSeverity::Fatal);
        assert!(diag.message.contains("Unexpected end of file"));
        assert!(diag.message.contains("end"));
        assert_eq!(diag.file, Some("test.pas".to_string()));
    }

    #[test]
    fn test_parser_error_invalid_syntax_to_diagnostic() {
        let span = Span::new(0, 10, 10, 15);
        let parser_err = ParserError::InvalidSyntax {
            message: "Invalid syntax here".to_string(),
            span,
        };

        let diag = parser_err.to_diagnostic(Some("test.pas".to_string()));
        assert_eq!(diag.severity, ErrorSeverity::Error);
        assert_eq!(diag.message, "Invalid syntax here");
        assert_eq!(diag.file, Some("test.pas".to_string()));
    }

    #[test]
    fn test_parser_error_display() {
        // Test backward compatibility - Display should use FPC format
        let span = Span::new(0, 10, 10, 15);
        let parser_err = ParserError::UnexpectedToken {
            expected: "identifier".to_string(),
            found: "keyword".to_string(),
            span,
        };

        let display_str = format!("{}", parser_err);
        assert!(display_str.contains("test.pas") || display_str.contains("unknown"));
        assert!(display_str.contains("Error:") || display_str.contains("Syntax error"));
    }

    // ===== Related Location Tests =====

    #[test]
    fn test_related_location_creation() {
        let related = RelatedLocation {
            message: "Test message".to_string(),
            span: Span::new(0, 5, 8, 12),
            file: Some("test.pas".to_string()),
        };

        assert_eq!(related.message, "Test message");
        assert_eq!(related.span, Span::new(0, 5, 8, 12));
        assert_eq!(related.file, Some("test.pas".to_string()));
    }

    #[test]
    fn test_related_location_no_file() {
        let related = RelatedLocation {
            message: "Test message".to_string(),
            span: Span::new(0, 5, 8, 12),
            file: None,
        };

        assert_eq!(related.file, None);
    }

    // ===== Code Snippet Tests =====

    #[test]
    fn test_code_snippet_creation() {
        let snippet = CodeSnippet {
            lines: vec![
                (1, "line 1".to_string()),
                (2, "line 2".to_string()),
            ],
            highlight_span: Span::new(0, 5, 2, 5),
        };

        assert_eq!(snippet.lines.len(), 2);
        assert_eq!(snippet.highlight_span.line, 2);
    }

    #[test]
    fn test_code_snippet_empty() {
        let snippet = CodeSnippet {
            lines: vec![],
            highlight_span: Span::new(0, 0, 1, 1),
        };

        assert!(snippet.lines.is_empty());
    }

    // ===== Edge Cases =====

    #[test]
    fn test_diagnostic_empty_message() {
        let span = Span::new(0, 10, 10, 15);
        let diag = Diagnostic::new(
            ErrorSeverity::Error,
            String::new(),
            span,
        )
        .with_file("test.pas".to_string());

        let formatted = diag.format_fpc();
        assert!(formatted.contains("test.pas(10,15) Error:"));
    }

    #[test]
    fn test_diagnostic_multiline_message() {
        let span = Span::new(0, 10, 10, 15);
        let diag = Diagnostic::new(
            ErrorSeverity::Error,
            "Line 1\nLine 2".to_string(),
            span,
        )
        .with_file("test.pas".to_string());

        let formatted = diag.format_fpc();
        assert!(formatted.contains("Line 1"));
    }

    #[test]
    fn test_diagnostic_all_optional_fields() {
        let span = Span::new(0, 10, 10, 15);
        let related = RelatedLocation {
            message: "Related".to_string(),
            span: Span::new(0, 5, 8, 12),
            file: Some("test.pas".to_string()),
        };
        let snippet = CodeSnippet {
            lines: vec![(10, "code".to_string())],
            highlight_span: span,
        };

        let diag = Diagnostic::new(
            ErrorSeverity::Error,
            "Message".to_string(),
            span,
        )
        .with_file("test.pas".to_string())
        .with_context("Context".to_string())
        .with_suggestion("Suggestion".to_string())
        .with_related_location(related)
        .with_code_snippet(snippet)
        .with_explanation("Explanation".to_string());

        // All fields should be set
        assert!(diag.file.is_some());
        assert!(diag.context.is_some());
        assert!(diag.suggestion.is_some());
        assert!(!diag.related_locations.is_empty());
        assert!(diag.code_snippet.is_some());
        assert!(diag.explanation.is_some());

        // All formats should work
        let _fpc = diag.format_fpc();
        let _enhanced = diag.format_enhanced();
        let _verbose = diag.format_verbose();
    }

    #[test]
    fn test_diagnostic_display_trait() {
        // Test that Diagnostic implements Display (uses enhanced format)
        let span = Span::new(0, 10, 10, 15);
        let diag = Diagnostic::new(
            ErrorSeverity::Error,
            "Test".to_string(),
            span,
        )
        .with_file("test.pas".to_string());

        let display_str = format!("{}", diag);
        // Display should use enhanced format by default
        assert_eq!(display_str, diag.format_enhanced());
    }
}
