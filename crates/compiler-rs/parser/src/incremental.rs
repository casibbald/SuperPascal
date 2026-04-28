//! Incremental Parsing Support
//!
//! Provides basic incremental parsing capabilities for editor integration.

use ast::Node;
use tokens::Span;

/// Represents a change in the source code
#[derive(Debug, Clone, PartialEq)]
pub struct SourceChange {
    /// Start position of the change
    pub start: usize,
    /// End position of the change (before replacement)
    pub end: usize,
    /// New content (empty for deletion)
    pub new_content: String,
    /// Line number where change starts
    pub line: usize,
    /// Column number where change starts
    pub column: usize,
}

/// Incremental parser state
pub struct IncrementalParser {
    /// Last parsed AST
    last_ast: Option<Node>,
    /// Source code
    source: String,
    /// Whether a full reparse is needed
    needs_full_reparse: bool,
}

impl IncrementalParser {
    /// Create a new incremental parser
    pub fn new(source: String) -> Self {
        Self {
            last_ast: None,
            source,
            needs_full_reparse: true,
        }
    }

    /// Apply a source change and determine if incremental parsing is possible
    pub fn apply_change(&mut self, change: SourceChange) -> IncrementalParseResult {
        // Update source
        let before = &self.source[..change.start];
        let after = &self.source[change.end..];
        self.source = format!("{}{}{}", before, change.new_content, after);

        // For now, mark as needing full reparse
        // TODO: Implement actual incremental parsing logic
        self.needs_full_reparse = true;

        IncrementalParseResult {
            needs_full_reparse: true,
            affected_span: Span {
                start: change.start,
                end: change.start + change.new_content.len(),
                line: change.line,
                column: change.column,
            },
        }
    }

    /// Parse incrementally (or fully if needed)
    pub fn parse(&mut self) -> Result<Node, String> {
        use super::Parser;

        if self.needs_full_reparse {
            let mut parser =
                Parser::new(&self.source).map_err(|e| format!("Parse error: {:?}", e))?;
            let ast = parser
                .parse()
                .map_err(|e| format!("Parse error: {:?}", e))?;
            self.last_ast = Some(ast.clone());
            self.needs_full_reparse = false;
            Ok(ast)
        } else {
            // TODO: Implement incremental update
            // For now, fall back to full reparse
            let mut parser =
                Parser::new(&self.source).map_err(|e| format!("Parse error: {:?}", e))?;
            let ast = parser
                .parse()
                .map_err(|e| format!("Parse error: {:?}", e))?;
            self.last_ast = Some(ast.clone());
            Ok(ast)
        }
    }

    /// Get the current source
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Check if a full reparse is needed
    pub fn needs_full_reparse(&self) -> bool {
        self.needs_full_reparse
    }
}

/// Result of incremental parse attempt
#[derive(Debug, Clone, PartialEq)]
pub struct IncrementalParseResult {
    /// Whether a full reparse is needed
    pub needs_full_reparse: bool,
    /// Span affected by the change
    pub affected_span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incremental_parser_basic() {
        let source = "program Test; begin end.".to_string();
        let mut parser = IncrementalParser::new(source);

        let ast = parser.parse().unwrap();
        assert!(matches!(ast, Node::Program(_)));
    }

    #[test]
    fn test_incremental_parser_change() {
        let source = "program Test; begin end.".to_string();
        let mut parser = IncrementalParser::new(source);

        // Initial parse
        let _ = parser.parse().unwrap();

        // Apply change
        let change = SourceChange {
            start: 20,
            end: 20,
            new_content: "x := 1; ".to_string(),
            line: 1,
            column: 20,
        };

        let result = parser.apply_change(change);
        assert!(result.needs_full_reparse);

        // Re-parse
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, Node::Program(_)));
    }
}
