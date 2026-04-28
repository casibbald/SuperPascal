//! Example demonstrating enhanced error diagnostics
//!
//! Run with: cargo run --example enhanced_errors --package errors

use errors::{CodeSnippet, Diagnostic, ErrorSeverity, RelatedLocation};
use tokens::Span;

fn main() {
    println!("=== FPC-Compatible Format ===\n");

    let span = Span::new(0, 10, 10, 15);
    let diag = Diagnostic::new(
        ErrorSeverity::Error,
        "Identifier not found \"xyz\"".to_string(),
        span,
    )
    .with_file("test.pas".to_string());

    println!("{}", diag.format_fpc());
    println!();

    println!("=== Enhanced Format (Default) ===\n");

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
    .with_context("In procedure 'DoSomething' (declared at test.pas:5,3)".to_string())
    .with_suggestion("Did you mean 'xy'? (declared at test.pas:2,10)".to_string())
    .with_related_location(related);

    println!("{}", diag.format_enhanced());
    println!();

    println!("=== Enhanced Format with Code Snippet ===\n");

    let snippet = CodeSnippet {
        lines: vec![
            (8, "  var xyz: integer;".to_string()),
            (9, "  begin".to_string()),
            (10, "    writeln(xyz);  // Error here".to_string()),
        ],
        highlight_span: Span::new(0, 10, 10, 15),
    };

    let diag = Diagnostic::new(
        ErrorSeverity::Error,
        "Identifier not found \"xyz\"".to_string(),
        span,
    )
    .with_file("test.pas".to_string())
    .with_context("In procedure 'DoSomething' (declared at test.pas:5,3)".to_string())
    .with_suggestion("Did you mean 'xy'? (declared at test.pas:2,10)".to_string())
    .with_code_snippet(snippet);

    println!("{}", diag.format_enhanced());
    println!();

    println!("=== Verbose Format ===\n");

    let diag = Diagnostic::new(
        ErrorSeverity::Error,
        "Identifier not found \"xyz\"".to_string(),
        span,
    )
    .with_file("test.pas".to_string())
    .with_context("In procedure 'DoSomething' (declared at test.pas:5,3)".to_string())
    .with_suggestion("Did you mean 'xy'? (declared at test.pas:2,10)".to_string())
    .with_explanation(
        "The identifier 'xyz' is not visible in this scope. \
         The variable 'xyz' was declared in procedure 'OtherProc' but is \
         not accessible from 'DoSomething'. Consider passing it as a parameter \
         or declaring it in a shared scope."
            .to_string(),
    );

    println!("{}", diag.format_verbose());
    println!();

    println!("=== Different Severity Levels ===\n");

    let warnings = vec![
        (
            "Warning",
            ErrorSeverity::Warning,
            "Variable 'x' does not seem to be initialized",
        ),
        (
            "Hint",
            ErrorSeverity::Hint,
            "Local variable 'y' is assigned but never used",
        ),
        (
            "Note",
            ErrorSeverity::Note,
            "Procedure 'DoSomething' is never called",
        ),
    ];

    for (name, severity, message) in warnings {
        let diag = Diagnostic::new(severity, message.to_string(), Span::new(0, 5, 5, 10))
            .with_file("test.pas".to_string());

        println!("{}: {}", name, diag.format_fpc());
    }
}
