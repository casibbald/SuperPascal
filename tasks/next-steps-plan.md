# Next Steps Plan - SuperPascal Development

**Last Updated:** 2024-12-19  
**Current Status:** 96% Modern Pascal Compliance (48/50 features) - Parser: 100% (50/50)  
**Codebase Status:** ✅ Clean (zero warnings, all tests passing)

---

## 🎯 Strategic Priorities

### Phase 1: Quick Wins (High Value, Low Effort)
**Goal:** Reach 80%+ compliance quickly

#### 1. Class Properties & Variables (4-8 hours)
**Why:** Completes class feature set, relatively simple
**Impact:** +2% compliance (40/50 features)

**Tasks:**
- [ ] Add `class property` parsing to `parser/src/declarations.rs`
- [ ] Add `class var` parsing to `parser/src/declarations.rs`
- [ ] Update AST (`ast/src/lib.rs`) with `is_class_property` and `is_class_var` flags
- [ ] Add semantic analysis in `semantics/src/declarations.rs`
- [ ] Add tests (refer to `tree-sitter-corpus/` for examples)
- [ ] Update `runtime-spec/src/capabilities.rs` to include these features

**Files to Modify:**
- `parser/src/declarations.rs`
- `ast/src/lib.rs`
- `semantics/src/declarations.rs`
- `tokens/src/lib.rs` (if new keywords needed)
- `runtime-spec/src/capabilities.rs`

---

#### 2. Dynamic Arrays (8-16 hours)
**Why:** Very common in modern Pascal, needed for real-world code
**Impact:** +1% compliance (41/50 features)

**Tasks:**
- [ ] Parse `array of T` syntax (distinguish from `array[0..9] of T`)
- [ ] Add `DynamicArrayType` to AST
- [ ] Update type system in `types/src/lib.rs`
- [ ] Add semantic analysis for dynamic arrays
- [ ] Handle `SetLength()` intrinsic (semantic analysis)
- [ ] Add tests

**Files to Modify:**
- `parser/src/types.rs`
- `ast/src/lib.rs`
- `types/src/lib.rs`
- `semantics/src/types.rs`
- `semantics/src/expressions.rs` (for SetLength)

---

#### 3. Update Documentation (1-2 hours)
**Why:** Keep compliance tracking accurate
**Impact:** Better project visibility

**Tasks:**
- [ ] Update `docs/parser-modern-pascal-comparison.md` (mark Class Methods as ✅)
- [ ] Update `tasks/modern-pascal-compliance-todo.md` (mark Class Methods complete)
- [ ] Update compliance percentage (currently shows 76%, should be 78% with Class Methods)

---

### Phase 2: High-Value Features (Medium Effort)
**Goal:** Reach 85%+ compliance

#### 4. Preprocessor Directives ✅ **COMPLETE** (2024-12-19)
**Why:** Critical for cross-platform code, conditional compilation
**Impact:** +1% compliance (42/50 features) ✅ **ACHIEVED**

**Implementation:**
- ✅ Add compiler directive lexing (`{$...}` and `(*$...*)`) in `lexer/src/lib.rs`
- ✅ Parse common directives:
  - ✅ `{$IFDEF}`, `{$IFNDEF}`, `{$ELSE}`, `{$ENDIF}`
  - ✅ `{$DEFINE}`, `{$UNDEF}`
  - ✅ `{$INCLUDE}` (parsing complete)
  - ⚠️ `{$IF}`, `{$ELSEIF}` with expressions (future enhancement)
  - ⚠️ `{$MODE}`, `{$H+}`, `{$H-}` (future enhancement)
- ✅ Implement conditional compilation logic with DirectiveEvaluator
- ✅ Add directive storage and evaluation
- ✅ Add comprehensive tests (17 unit tests + 6 integration tests, all passing)

**Files Modified:**
- `lexer/src/lib.rs` (directive tokenization)
- `parser/src/lib.rs` (directive parsing)
- `parser/src/directives.rs` (directive evaluation) - NEW
- `parser/src/declarations.rs` (directive parsing integration)
- `ast/src/lib.rs` (directive AST nodes)
- `tokens/src/lib.rs` (directive token kind)

---

### Phase 3: Complex Features (High Effort, High Value)
**Goal:** Reach 90%+ compliance

#### 5. Generics (40-60 hours)
**Why:** Essential for modern libraries, most complex feature
**Impact:** +1% compliance (43/50 features)

**Tasks:**
- [ ] Add generic type parameter parsing (`<T>`, `<T: constraint>`)
- [ ] Add generic type instantiation syntax (`TList<integer>`)
- [ ] Add generic constraints (`where T: class`)
- [ ] Add generic procedure/function support
- [ ] Update AST to support generic types
- [ ] Add semantic analysis for generics (type checking, instantiation)
- [ ] Add tests (refer to `tree-sitter-corpus/generics-*.txt`)

**Files to Modify:**
- `parser/src/types.rs` (generic type parsing)
- `parser/src/declarations.rs` (generic routine parsing)
- `ast/src/lib.rs` (generic AST nodes)
- `semantics/src/types.rs` (generic type checking)
- `semantics/src/declarations.rs` (generic instantiation)

---

### Phase 4: Parser Polish (Ongoing)
**Goal:** Improve developer experience and performance

#### 6. Error Recovery - Source Snippets (2-4 hours)
**Why:** Quick win, significantly improves error messages
**Impact:** Better developer experience

**Tasks:**
- [ ] Add `source: String` field to `Parser` struct
- [ ] Store source in `Parser::new()` and `Parser::new_with_file()`
- [ ] Implement `get_source_snippet()` to extract lines around span
- [ ] Create `CodeSnippet` with line numbers and highlighting
- [ ] Add tests

**Files to Modify:**
- `parser/src/core.rs` (add source field)
- `parser/src/lib.rs` (implement get_source_snippet)

---

#### 7. Error Recovery - Actual Recovery (8-16 hours)
**Why:** Allows finding multiple errors, critical for good UX
**Impact:** Much better error reporting

**Tasks:**
- [ ] Add `recover_from_error()` method to `Parser`
- [ ] Implement sync points (skip to `;`, next declaration, BEGIN/END)
- [ ] Modify parsing functions to attempt recovery
- [ ] Collect multiple errors instead of stopping at first
- [ ] Add tests for error recovery scenarios

**Files to Modify:**
- `parser/src/core.rs` (recovery logic)
- `parser/src/lib.rs` (integrate recovery)
- All parsing modules (add recovery points)

---

#### 8. AST Query System - Complete Coverage (4-8 hours)
**Why:** Needed for IDE features (go to definition, refactoring)
**Impact:** Enables IDE integration

**Tasks:**
- [ ] Review all AST node types
- [ ] Implement `get_children()` for missing node types
- [ ] Add tests for each node type
- [ ] Verify query system works for all AST structures

**Files to Modify:**
- `parser/src/query.rs`

---

## 📊 Implementation Roadmap

### Week 1: Quick Wins
1. ✅ Update documentation (Class Methods status)
2. Class Properties & Variables (4-8 hours)
3. Dynamic Arrays (8-16 hours)

**Expected Outcome:** 80%+ compliance (41/50 features)

### Week 2-3: Preprocessor ✅ **COMPLETE**
4. ✅ Preprocessor Directives (24-40 hours) - Completed 2024-12-19

**Expected Outcome:** 84% compliance (42/50 features) ✅ **ACHIEVED**

### Week 4-6: Generics
5. Generics (40-60 hours)

**Expected Outcome:** 84%+ compliance (43/50 features)

### Ongoing: Parser Polish
6. Error Recovery - Source Snippets (2-4 hours)
7. Error Recovery - Actual Recovery (8-16 hours)
8. AST Query System - Complete Coverage (4-8 hours)

---

## 🎯 Success Metrics

### Compliance Targets
- **Current:** 86% (43/50 features) ✅ **ACHIEVED**
- **Phase 1 Goal:** 80% (40/50 features) ✅ **EXCEEDED**
- **Phase 2 Goal:** 84% (42/50 features) ✅ **EXCEEDED**
- **Phase 3 Goal:** 86% (43/50 features) ✅ **ACHIEVED**
- **Next Goal:** 88% (44/50 features) - Semantic analysis for generics or next feature
- **Ultimate Goal:** 100% (50/50 features)

### Code Quality Targets
- ✅ Zero compiler warnings (achieved)
- ✅ 65%+ test coverage (current: ~65%)
- ✅ All tests passing (achieved: 431/431)
- ✅ Clean, modular codebase (achieved)

---

## 🔍 Current Status Summary

### ✅ Completed Recently
- ✅ Semantics modularization (7 modules, 74% reduction in lib.rs size)
- ✅ Class Methods implementation
- ✅ Class Properties & Variables
- ✅ Dynamic Arrays
- ✅ Enumerated Types
- ✅ For..In Loops
- ✅ **Preprocessor Directives** (2024-12-19) - Conditional compilation complete
- ✅ **Generics Parsing** (2024-12-19) - Full generic syntax support, 13 tests
- ✅ All compiler warnings fixed
- ✅ Phase 1-3 parser features complete

### ⚠️ Needs Verification
- Class Methods (implemented but TODO not updated)
- Record Methods (mentioned as "need to verify")

### ❌ High Priority Missing
1. **Generics** (40-60 hours) - Next target
2. ~~Dynamic Arrays~~ ✅ **COMPLETE**
3. ~~Preprocessor~~ ✅ **COMPLETE**
4. ~~Class Properties/Variables~~ ✅ **COMPLETE**

### ❌ Medium Priority Missing
5. Anonymous Functions (16-24 hours)
6. Class Helpers (16-24 hours)
7. Nested Classes (8-16 hours)

### ❌ Low Priority Missing
8. Variant Type (16-24 hours)
9. Old-Style Objects (8-16 hours)
10. Reference-Counted Interfaces (runtime, not parser)

---

## 📝 Notes

### Immediate Next Steps (This Week)
1. ✅ **Update documentation** - Mark Preprocessor as complete
2. **Generics** - Next high-priority feature (40-60 hours)
   - Most complex remaining feature
   - Essential for modern Pascal libraries
   - Will bring compliance to 86% (43/50)

### Why This Order?
- **Quick wins first:** Build momentum, visible progress
- **Preprocessor before Generics:** Preprocessor is more straightforward, needed for real-world code
- **Generics last (in Phase 3):** Most complex, requires solid foundation
- **Parser polish ongoing:** Can be done in parallel, improves DX

### Dependencies
- Class Properties/Variables: Depends on Class Methods (✅ done)
- Dynamic Arrays: Independent
- Preprocessor: Independent
- Generics: Can benefit from preprocessor for conditional compilation

---

## 🚀 Ready to Start

**Recommended First Task:** ✅ Documentation updated, now implementing Generics

**Estimated Time to 80% Compliance:** ✅ **ACHIEVED** (84% current)

**Estimated Time to 85% Compliance:** 40-60 hours (Generics implementation)

**Estimated Time to 90% Compliance:** 80-120 hours (Generics + additional features)

