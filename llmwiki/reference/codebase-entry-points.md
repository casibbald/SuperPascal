# Codebase Entry Points

- Status: partially-verified
- Source docs: `docs/QUICK_START_COMPILER.md`, `docs/COMPILER_IMPLEMENTATION_STRATEGY.md`
- Code anchors: `crates/compiler-rs/`

## Primary entry points

| Entry | File | Purpose |
|-------|------|---------|
| Compiler driver | `driver/src/main.rs` | CLI entry point |
| Compiler pipeline | `driver/src/compiler.rs` | Main compilation orchestration |
| Parser facade | `parser/src/lib.rs` | Parser entry, `Parser::new()`, `Parser::parse()` |
| Lexer | `lexer/src/lib.rs` | Tokenizer entry |
| Semantic analysis | `semantics/src/lib.rs` | Module-level entry (delegates to 7 submodules) |
| IR builder | `ir/src/lib.rs` | IR generation entry |
| Runtime library | `runtime/src/lib.rs` | Runtime functions entry |
| Z80 backend | `backends/backend-zealz80/src/lib.rs` | Backend codegen |
| Z80 object format | `objects/object-zealz80/src/lib.rs` | Object file output |

## Call chains

### Compilation flow
```
main.rs
  └── compiler.rs (run_pipeline)
        ├── lexer (tokenize)
        ├── parser (parse)
        │     ├── tokens
        │     └── ast
        ├── semantics
        │     ├── expressions
        │     ├── statements
        │     ├── declarations
        │     ├── types
        │     ├── constants
        │     ├── lvalues
        │     └── feature_checker
        ├── types
        ├── symbols
        ├── ir (build_*)
        ├── backends/backend-zealz80
        └── objects/object-zealz80
```

### Parser internals
```
parser/lib.rs
  ├── core.rs          — Parser struct, error handling
  ├── statements.rs    — if, while, for, repeat, case, begin/end
  ├── expressions.rs   — binary, unary, call, field access
  ├── declarations.rs  — type, var, procedure, function, class
  ├── types.rs         — type expressions, arrays, records, classes
  ├── classes.rs       — class definitions, methods
  ├── properties.rs    — property declarations
  ├── units.rs         — unit/module declarations
  ├── directives.rs    — compiler directives ({$...})
  ├── query.rs         — AST query system
  └── incremental.rs   — incremental parsing support
```

## Test entry points

| Test type | Location | Command |
|-----------|----------|---------|
| Unit tests | `crates/compiler-rs/*/tests/` | `cargo test --lib` |
| Integration | `crates/compiler-rs/tests/` | `cargo nextest run --test '*'` |
| Compliance | `tests/compliance/` | `cargo nextest run --test compliance` |
| Tree-sitter corpus | `tests/tree-sitter-corpus/` | corpus-validator.rs |

## Key files by area

### Lexer & Tokens
- `tokens/src/lib.rs` — `TokenKind` enum, token definition
- `lexer/src/lib.rs` — `Lexer` struct, `next_token()` method

### Parser
- `parser/src/lib.rs` — `Parser` struct, `parse()` method
- `parser/src/core.rs` — error recovery, source tracking
- `parser/src/expressions.rs` — expression parsing (highest priority)
- `parser/src/statements.rs` — statement parsing
- `parser/src/declarations.rs` — declaration parsing
- `parser/src/types.rs` — type expression parsing

### Semantic Analysis
- `semantics/src/lib.rs` — module entry (re-exports submodules)
- `semantics/src/feature_checker.rs` — feature gate checking
- `semantics/MODULARIZATION_PLAN.md` — architecture doc

### IR
- `ir/src/lib.rs` — `IRBuilder` struct, `build_*` methods
