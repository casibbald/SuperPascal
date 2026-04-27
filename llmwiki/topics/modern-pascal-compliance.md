# Modern Pascal Compliance

- Status: partially-verified
- Source docs: `tasks/modern-pascal-compliance-todo.md`, `tasks/function-name-usage-analysis.md`, `languageSpecification/16_ModernPascalFeatures.md`
- Code anchors: `crates/compiler-rs/parser/`, `crates/compiler-rs/semantics/`

## Feature checklist

50 Modern Pascal features tracked. Current status:

### Recently completed
- ✅ Class Methods
- ✅ Enumerated Types
- ✅ For..In Loops
- ✅ Semantics modularization (7 modules)
- ✅ All compiler warnings fixed

### High priority missing
| Feature | Est. hours | Parser | Semantics | Tests |
|---------|-----------|--------|-----------|-------|
| Generics | 40-60 | ❌ | ❌ | ❌ |
| Dynamic Arrays | 8-16 | ❌ | ❌ | ❌ |
| Preprocessor | 24-40 | ❌ | ❌ | ❌ |
| Class Properties | 4-8 | ❌ | ❌ | ❌ |
| Class Variables | 4-8 | ❌ | ❌ | ❌ |

### Medium priority missing
| Feature | Est. hours | Parser | Semantics | Tests |
|---------|-----------|--------|-----------|-------|
| Anonymous Functions | 16-24 | ❌ | ❌ | ❌ |
| Class Helpers | 16-24 | ❌ | ❌ | ❌ |
| Nested Classes | 8-16 | ❌ | ❌ | ❌ |

### Low priority missing
| Feature | Est. hours | Notes |
|---------|-----------|-------|
| Variant Type | 16-24 | Runtime support needed |
| Old-Style Objects | 8-16 | Legacy Pascal |
| Reference-Counted Interfaces | — | Runtime, not parser |

## Compliance testing

Compliance is tested against:
1. **FreePascal fork** — tokenizer comparison via `-dt` flag
2. **Tree-sitter corpus** — 13 test files covering grammar features
3. **Unit tests** — individual parser/semantic modules
4. **Integration tests** — cross-crate compilation pipeline

## Tracking

- `tasks/modern-pascal-compliance-todo.md` — detailed TODO list
- `tasks/function-name-usage-analysis.md` — function naming analysis
- `tasks/string-interpolation-specification.md` — string interpolation spec
- `tasks/modern-pascal-documentation-status.md` — documentation status

## Feature implementation pattern

For each new feature:
1. Add token if needed (`tokens/src/lib.rs`)
2. Implement parser (`parser/src/*.rs`)
3. Update AST (`ast/src/lib.rs`) — flags or new node types
4. Add semantic analysis (`semantics/src/*.rs`)
5. Update capabilities (`runtime-spec/src/capabilities.rs`)
6. Add tests (tree-sitter corpus + unit tests)
7. Update compliance tracking docs
