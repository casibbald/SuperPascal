# Phase 4 Parser Work - TODO

## Overview

Phase 4 of the parser improvement plan focused on **polish and optimization**:
1. Error Recovery
2. Performance Optimization
3. Incremental Parsing
4. AST Query System

## Status Summary

### ✅ Completed

1. **Enhanced Error Diagnostics** (`lib.rs::error_to_diagnostic`)
   - ✅ Context-specific suggestions for common errors
   - ✅ Enhanced error messages with explanations
   - ✅ Support for multiple error formats (FPC, VSCode, enhanced)

2. **AST Query System** (`query.rs`)
   - ✅ Basic query API implemented
   - ✅ `find_all`, `find_first`, `find_by_type`
   - ✅ `find_procedures`, `find_variables`, `find_identifiers`
   - ✅ `find_at_span`, `get_parent_child_pairs`
   - ✅ Basic tests

3. **Performance Benchmarks** (`benches/parser_benchmark.rs`)
   - ✅ Benchmark infrastructure set up
   - ✅ Benchmarks for simple programs, large programs, nested blocks, expressions

4. **Incremental Parsing Infrastructure** (`incremental.rs`)
   - ✅ Module structure created
   - ✅ `SourceChange`, `IncrementalParser`, `IncrementalParseResult` types
   - ✅ Basic API structure

---

## TODO: Remaining Work

### 🔴 High Priority

#### 1. Error Recovery - Source Snippets

**File:** `crates/compiler-rs/parser/src/lib.rs::get_source_snippet()`

**Current Status:**
```rust
fn get_source_snippet(&self, _span: Span) -> Option<CodeSnippet> {
    // TODO: Store source in Parser struct and extract lines around error
    None
}
```

**Tasks:**
- [ ] Add `source: String` field to `Parser` struct
- [ ] Store source in `Parser::new()` and `Parser::new_with_file()`
- [ ] Implement `get_source_snippet()` to extract lines around span (3 lines before, error line, 3 lines after)
- [ ] Create `CodeSnippet` with line numbers and highlighting
- [ ] Add tests for source snippet extraction

**Estimated:** 2-4 hours

---

#### 2. Error Recovery - Actual Recovery

**Current Status:**
- Parser stops on first error
- No recovery mechanism to continue parsing

**Tasks:**
- [ ] Add `recover_from_error()` method to `Parser`
- [ ] Implement sync points:
  - [ ] Skip to next statement (`;`)
  - [ ] Skip to next declaration (VAR, CONST, TYPE, PROCEDURE, FUNCTION)
  - [ ] Skip to next block boundary (BEGIN/END)
- [ ] Modify parsing functions to attempt recovery
- [ ] Collect multiple errors instead of stopping at first
- [ ] Add tests for error recovery scenarios:
  - [ ] Multiple errors in same file
  - [ ] Recovery from missing semicolon
  - [ ] Recovery from missing END
  - [ ] Recovery from syntax errors

**Estimated:** 8-16 hours

---

### 🟡 Medium Priority

#### 3. AST Query System - Complete Coverage

**File:** `crates/compiler-rs/parser/src/query.rs::get_children()`

**Current Status:**
- Basic node types covered
- Many node types return `Vec::new()` (not fully implemented)

**Tasks:**
- [ ] Review all AST node types in `ast/src/lib.rs`
- [ ] Implement `get_children()` for missing node types:
  - [ ] Unit, Library, UsesClause
  - [ ] InterfaceSection, ImplementationSection
  - [ ] TryStmt, WithStmt, CaseStmt, RepeatStmt
  - [ ] ForInStmt, GotoStmt, LabeledStmt, AsmStmt
  - [ ] RaiseStmt, CallStmt
  - [ ] All type nodes (RecordType, ArrayType, SetType, StringType, etc.)
  - [ ] All declaration types (VarDecl, ConstDecl, TypeDecl, etc.)
  - [ ] All expression types (LiteralExpr, IdentExpr, etc.)
- [ ] Add tests for each node type
- [ ] Verify query system works for all AST structures

**Estimated:** 4-8 hours

---

#### 4. Performance Optimization

**Current Status:**
- Benchmarks exist but optimizations not yet applied
- No performance profiling done

**Tasks:**
- [ ] Run `cargo bench` to establish baseline
- [ ] Profile with `perf` or `cargo-flamegraph`
- [ ] Identify bottlenecks:
  - [ ] Token lookups (consider caching)
  - [ ] Expression parsing (Pratt parser optimization)
  - [ ] Type checking (consider memoization)
- [ ] Apply optimizations incrementally:
  - [ ] Memory optimization (reduce AST node cloning)
  - [ ] Use references where possible
  - [ ] Consider arena allocation for AST nodes
- [ ] Re-benchmark to verify improvements
- [ ] Document performance characteristics

**Estimated:** 16-32 hours (iterative)

---

### 🟢 Low Priority (Future Enhancement)

#### 5. Incremental Parsing - Actual Implementation

**File:** `crates/compiler-rs/parser/src/incremental.rs`

**Current Status:**
```rust
// TODO: Implement actual incremental parsing logic
self.needs_full_reparse = true;
```

**Tasks:**
- [ ] Implement change impact analysis (which nodes are affected)
- [ ] Implement incremental re-parsing for affected regions
- [ ] Implement AST merging (combine old and new AST)
- [ ] Handle edge cases:
  - [ ] Deletions
  - [ ] Insertions
  - [ ] Multi-line changes
- [ ] Implement incremental lexer (re-tokenize only changed regions)
- [ ] AST diff/merge algorithm
- [ ] Add comprehensive tests

**Complexity:** High - This is a significant feature requiring:
- AST node tracking (which source spans map to which nodes)
- Dependency tracking (which nodes depend on changed nodes)
- Incremental lexer (re-tokenize only changed regions)
- AST diff/merge algorithm

**Estimated:** 40-80 hours

---

## Implementation Order (Recommended)

1. **Start with Quick Wins:**
   - [ ] Error Recovery - Source Snippets (2-4 hours)
   - [ ] AST Query System - Complete Coverage (4-8 hours)
   - [ ] Basic Error Recovery (8-16 hours)

2. **Then Performance:**
   - [ ] Profile and optimize (16-32 hours, iterative)

3. **Finally Complex Features:**
   - [ ] Incremental Parsing (40-80 hours, if needed for IDE)

---

## Testing Requirements

For each remaining feature:
- [ ] Unit tests for core functionality
- [ ] Integration tests for end-to-end scenarios
- [ ] Performance tests (for optimization work)
- [ ] Edge case tests (for error recovery)

---

## Notes

- **Error Recovery - Source Snippets**: Quick win, improves developer experience significantly
- **Error Recovery - Actual Recovery**: Critical for good error messages, allows finding multiple errors
- **AST Query System**: Needed for IDE features (go to definition, refactoring)
- **Performance Optimization**: Important for large files, iterative process
- **Incremental Parsing**: Complex feature, mainly for IDE integration, can be deferred

---

**Last Updated:** 2024-12-19
**Status:** In Progress
**Phase:** 4 of 4 (Polish & Optimization)

