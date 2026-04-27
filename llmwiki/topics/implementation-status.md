# Implementation Status

- Status: partially-verified
- Source docs: `NEXT-STEPS.md`, `tasks/modern-pascal-compliance-todo.md`, `README.md`
- Code anchors: `crates/compiler-rs/`, `languageSpecification/`

## Overall status

| Area | Status | Progress |
|------|--------|----------|
| Language Spec | Complete | v0.1–v0.8 |
| Parser | Complete | 50/50 Modern Pascal features |
| Semantic Analysis | Complete | 50/50 features |
| AST | Complete | All node types defined |
| IR Generation | **Incomplete** | Only variant decl, assign stmt, basic expr |
| Backend Codegen | Partial | Z80 backend skeleton exists |
| Stdlib (Pascal) | Draft | `lib/` has 10 modules |
| Educational Book | Draft | 36 chapters in `book/` |
| Compliance Tests | Partial | Tree-sitter corpus + FPC comparison |

## Parser compliance

From `NEXT-STEPS.md` (2024-12-23):
- **50/50 Modern Pascal features** claimed complete
- Recently completed: Class Methods, Enumerated Types, For..In Loops, Semantics modularization
- Missing features: Generics (40-60h), Dynamic Arrays (8-16h), Preprocessor (24-40h), Class Properties/Variables (4-8h)

## IR bridge gap

The AST→IR conversion is the highest priority gap. Currently implemented:
- `build_var_decl` — variable declaration
- `build_assign_stmt` — assignment statement
- `build_expression` — basic expression building

Still needs implementation:
- `build_if_stmt` — control flow with labels and jumps
- `build_while_stmt` — loop with condition
- `build_for_stmt` — loop with counter
- `build_repeat_stmt` — loop with condition at end
- `build_case_stmt` — switch/case
- `build_call_stmt` — procedure/function calls
- `build_block` — all declaration types

Estimated: 40-60 hours

## Compliance testing

- Tree-sitter corpus: 13 test files covering statements, modules, declarations, routines, expressions, literals, lambdas, attributes, preprocessor, generics (FPC/Delphi), inline variables, Pascalocoa
- FPC compliance: patched FPC with `-dt` flag for tokenizer comparison, `tests/compliance/` test cases
