# Crate Layout

- Status: partially-verified
- Source docs: `crates/compiler-rs/Cargo.toml`, `docs/RUST_CRATE_LAYOUT_PROPOSAL.md`
- Code anchors: `crates/compiler-rs/`

## Workspace members (14 crates)

| Crate | Crate root | Purpose |
|-------|-----------|---------|
| `tokens` | `tokens/src/lib.rs` | Token kind definitions (keywords, operators, literals, identifiers) |
| `lexer` | `lexer/src/lib.rs` | Lexical analyzer — source → token stream |
| `ast` | `ast/src/lib.rs` | AST node types — expressions, statements, declarations, types |
| `errors` | `errors/src/lib.rs` | Error types, diagnostic reporting |
| `parser` | `parser/src/lib.rs` | Recursive descent parser — AST generation |
| `types` | `types/src/lib.rs` | Type system — type definitions, type checking infrastructure |
| `symbols` | `symbols/src/lib.rs` | Symbol table — scopes, identifiers, resolution |
| `semantics` | `semantics/src/lib.rs` | Semantic analysis (7 modules) |
| `ir` | `ir/src/lib.rs` | Intermediate representation — IR1 linear, IR2 three-address |
| `runtime-spec` | `runtime-spec/src/lib.rs` | Runtime capabilities spec |
| `runtime` | `runtime/src/lib.rs` | Runtime library — array, closure, interface, variant |
| `backends/backend-zealz80` | `backends/backend-zealz80/src/lib.rs` | Z80 codegen backend |
| `objects/object-zealz80` | `objects/object-zealz80/src/lib.rs` | Z80 object file format (.ZOF) |
| `driver` | `driver/src/compiler.rs` | Compiler driver — pipeline orchestration |

## Semantic analysis submodule structure

`semantics/src/` was modularized from a single monolithic `lib.rs`:

| Module | Responsibility |
|--------|---------------|
| `expressions.rs` | Expression type checking, inference |
| `statements.rs` | Statement analysis (if, while, for, repeat, case, call, block) |
| `declarations.rs` | Type, procedure, variable, class declarations |
| `types.rs` | Type system rules, type compatibility |
| `constants.rs` | Constant folding, literal evaluation |
| `lvalues.rs` | L-value analysis (assignment targets) |
| `feature_checker.rs` | Modern Pascal feature gate checking |

## Cargo workspace config

- `resolver = "3"` (edition 2024)
- `version = "0.1.0"`
- No external dependencies yet (core workspace, dependencies to be added as needed)
