# Compiler Pipeline

- Status: partially-verified
- Source docs: `docs/COMPILER_IMPLEMENTATION_STRATEGY.md`, `languageSpecification/02_Grammar.md`, `languageSpecification/05_ABI_Concepts.md`
- Code anchors: `crates/compiler-rs/driver/src/compiler.rs`

## Pipeline phases

1. **Lexer** (`lexer/src/lib.rs`) — tokenizes source into token stream
2. **Parser** (`parser/src/lib.rs`) — recursive descent parser producing AST
3. **Semantic Analyzer** (`semantics/src/lib.rs`) — symbol tables, type inference, feature checking
4. **IR Generation** (`ir/src/lib.rs`) — IR1 (linear low-level Pascal) or IR2 (three-address form)
5. **Backend Codegen** (`backends/backend-zealz80/src/lib.rs`) — target-specific code generation
6. **Object File** (`objects/object-zealz80/src/lib.rs`) — Z80 object format output
7. **Linker** — produces `.ZOF`, `.ZOU`, `.ZPK` bundles

## Current status

| Phase | Status | Notes |
|-------|--------|-------|
| Lexer | Complete | Token definitions in `tokens/`, tokenizer in `lexer/` |
| Parser | Complete | 50/50 Modern Pascal features; classes, properties, generics, lambdas, attributes |
| Semantic Analysis | Complete | 7-module modularization; expressions, statements, declarations, types, constants, L-values, feature checker |
| AST | Complete | AST nodes in `ast/src/lib.rs` |
| IR Generation | **Incomplete** | Only `build_var_decl`, `build_assign_stmt`, basic `build_expression` implemented. See `NEXT-STEPS.md` §1 |
| Backend | Partial | ZealZ80 backend exists but needs IR coverage |
| Object File | Partial | ZealZ80 object format exists |
| Linker | Not implemented | TBD |
