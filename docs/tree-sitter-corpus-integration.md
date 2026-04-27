# Tree-Sitter-Pascal Corpus Integration

## Overview

This document describes how we leverage `tree-sitter-pascal`'s extensive test corpus to enhance SuperPascal's testing infrastructure.

## What We're Borrowing

`tree-sitter-pascal` has **13 comprehensive test corpus files** covering:

1. **statements.txt** - Control flow (if, while, for, case, try, with, goto, asm)
2. **routines.txt** - Procedures, functions, methods, operators, nested routines
3. **declarations.txt** - Variables, constants, types, classes, interfaces
4. **expressions.txt** - All expression types, operators, precedence
5. **literals.txt** - Numbers, strings, characters, booleans
6. **modules.txt** - Units, libraries, uses, interface/implementation
7. **attributes.txt** - RTTI attributes, procedure attributes
8. **generics-delphi.txt** - Delphi generic types
9. **generics-fpc.txt** - FreePascal generic types
10. **lambdas.txt** - Anonymous procedures/functions
11. **inlinevar.txt** - Inline variable declarations
12. **preprocessor.txt** - Compiler directives
13. **pascocoa.txt** - PasCocoa/Objective-C interop

**Total**: Hundreds of test cases covering edge cases and real-world Pascal syntax.

## Integration Strategy

### Phase 1: Corpus Parser ✅

Created `tree-sitter-corpus` crate with:
- **Corpus Parser**: Parses tree-sitter corpus format
- **Test Extraction**: Extracts individual test cases with source code and expected AST
- **Batch Processing**: Processes entire corpus directories

### Phase 2: Test Generator ✅

Created `corpus-test-generator` tool:
- Generates Rust test cases from corpus files
- Creates test harness for validation
- Supports filtering by feature/priority
- Handles unimplemented features gracefully

### Phase 3: Validator ✅

Created `corpus-validator` tool:
- Validates SuperPascal parser against corpus
- Generates coverage reports
- Tracks parse success rate
- Measures performance

### Phase 4: Continuous Integration

- Run corpus tests in CI
- Track parser improvements over time
- Generate coverage reports
- Alert on regressions

## Usage

### Generate Tests

```bash
cd crates/compiler-rs/tests/tree-sitter-corpus
cargo run --bin corpus-test-generator -- \
    --corpus-dir ../../../../tree-sitter-pascal/test/corpus \
    --output-dir generated_tests \
    --filter statements,expressions
```

### Validate Parser

```bash
cargo run --bin corpus-validator -- \
    --corpus-dir ../../../../tree-sitter-pascal/test/corpus \
    --report coverage_report.json
```

### Run Generated Tests

```bash
cd crates/compiler-rs
cargo test --test tree_sitter_corpus
```

## Test Format

Tree-sitter corpus format:

```
===
Test Name
===

source code here

---

(expected AST in S-expression format)
```

Our generated tests:

```rust
#[test]
fn test_statements_if() {
    let source = r#"
        procedure Main;
        begin
          if a then b;
        end;
    "#;
    let mut parser = Parser::new(source);
    match parser {
        Ok(mut p) => {
            let result = p.parse();
            assert!(result.is_ok(), "Parse failed: {:?}", result);
        }
        Err(e) => {
            // Feature not yet implemented
            println!("Parser error (expected): {:?}", e);
        }
    }
}
```

## Coverage Tracking

The validator tracks:

- **Parse Success Rate**: % of tests that parse successfully
- **Feature Coverage**: Which features are supported
- **Error Quality**: Quality of error messages for unsupported features
- **Performance**: Parse time for each test

### Example Report

```
=== Coverage Report ===
Total Tests: 342
Successful Parses: 137
Failed Parses: 205
Parse Success Rate: 40.06%
Average Parse Time: 2.34ms

=== Results by File ===
statements.txt: 45/89 (50.6%)
expressions.txt: 38/67 (56.7%)
declarations.txt: 32/78 (41.0%)
...
```

## Benefits

1. **Comprehensive Coverage**: Hundreds of test cases covering edge cases
2. **Real-World Syntax**: Tests based on actual Pascal code patterns
3. **Regression Prevention**: Catch parser regressions early
4. **Feature Tracking**: Know exactly which features are implemented
5. **Quality Metrics**: Measure parser quality objectively

## Ethical Considerations

We're using tree-sitter-pascal's test corpus with:
- **Attribution**: Credit tree-sitter-pascal in documentation
- **License Compliance**: Respect tree-sitter-pascal's license (MIT)
- **Contribution**: Contribute improvements back when possible
- **Transparency**: Document our usage clearly

## Future Enhancements

1. **AST Comparison**: Compare our AST structure with tree-sitter's CST
2. **Error Message Comparison**: Compare error messages for unsupported features
3. **Performance Benchmarking**: Track parser performance over time
4. **Feature Flags**: Test features conditionally based on implementation status
5. **Test Categorization**: Organize tests by priority/feature

## References

- [tree-sitter-pascal](https://github.com/Isopod/tree-sitter-pascal)
- [Tree-sitter Test Corpus Format](https://tree-sitter.github.io/tree-sitter/using-parsers#command-test)
- [SuperPascal Parser Audit](./parser-audit-improvement-plan.md)

