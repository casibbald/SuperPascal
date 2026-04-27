# Next Steps Plan

- Status: partially-verified
- Source docs: `NEXT-STEPS.md`, `tasks/next-steps-plan.md`
- Code anchors: `crates/compiler-rs/`

## Strategic priorities (from NEXT-STEPS.md)

### Phase 1: Quick Wins (8-24 hours total)
- Class Properties & Variables (4-8h) — `parser/src/declarations.rs`
- Dynamic Arrays (8-16h) — `parser/src/types.rs`

### Phase 2: Medium Effort (24-40h)
- Preprocessor Directives (24-40h) — `lexer/src/lib.rs`, `parser/src/lib.rs`

### Phase 3: Complex Features (40-60h)
- Generics (40-60h) — `parser/src/types.rs`, `parser/src/declarations.rs`

### Phase 4: Parser Polish (14-28h)
- Error Recovery - Source Snippets (2-4h)
- Error Recovery - Actual Recovery (8-16h)
- AST Query System - Complete Coverage (4-8h)

### Phase 5: IR Bridge (40-60h) — **HIGHEST PRIORITY**
- Complete `ir/src/lib.rs` `build_*` methods
- Implement: if, while, for, repeat, case, call statements

### Phase 6: Backend Codegen (40-80h)
- Complete Z80 backend for all IR types
- Add other platform backends

### Phase 7: Stdlib (60-120h)
- Core I/O, strings, math, file I/O

## Current compliance status

| Target | Current | Goal |
|--------|---------|------|
| Modern Pascal features | 50/50 (claimed) | 100% |
| Parser | 100% | 100% |
| Semantic analysis | 100% | 100% |
| IR bridge | Partial | Complete |
| Codegen | Partial | Complete |

## High-priority missing features

1. Generics (40-60h)
2. Dynamic Arrays (8-16h)
3. Preprocessor (24-40h)
4. Class Properties/Variables (4-8h)

## Medium-priority missing

5. Anonymous Functions (16-24h)
6. Class Helpers (16-24h)
7. Nested Classes (8-16h)

## Low-priority missing

8. Variant Type (16-24h)
9. Old-Style Objects (8-16h)
10. Reference-Counted Interfaces (runtime)

## Dependencies

- Class Properties/Variables → depends on Class Methods (✅ done)
- Dynamic Arrays → independent
- Preprocessor → independent
- Generics → benefits from preprocessor
- IR bridge → depends on parser completion (✅ done)
- Backend → depends on IR bridge completion
