# Phase 4 Parser Work - Remaining Tasks

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

### ❌ Remaining Work

#### 1. Error Recovery - Source Snippets

**File:** `parser/src/lib.rs::get_source_snippet()`

**Current Status:**
```rust
fn get_source_snippet(&self, _span: Span) -> Option<CodeSnippet> {
    // TODO: Store source in Parser struct and extract lines around error
    None
}
```

**What's Needed:**
- Store source code in `Parser` struct
- Extract lines around error location (e.g., 3 lines before, error line, 3 lines after)
- Create `CodeSnippet` with line numbers and highlighting

**Implementation Steps:**
1. Add `source: String` field to `Parser` struct
2. Store source in `Parser::new()` and `Parser::new_with_file()`
3. Implement `get_source_snippet()` to extract lines around span
4. Add tests for source snippet extraction

#### 2. Error Recovery - Actual Recovery

**Current Status:**
- Parser stops on first error
- No recovery mechanism to continue parsing

**What's Needed:**
- Panic-mode error recovery (skip tokens until sync point)
- Continue parsing after errors to find multiple issues
- Error recovery strategies:
  - Skip to next statement (`;`)
  - Skip to next declaration (VAR, CONST, TYPE, PROCEDURE, FUNCTION)
  - Skip to next block boundary (BEGIN/END)

**Implementation Steps:**
1. Add `recover_from_error()` method to `Parser`
2. Implement sync points (statement boundaries, declaration boundaries)
3. Modify parsing functions to attempt recovery
4. Collect multiple errors instead of stopping at first
5. Add tests for error recovery scenarios

#### 3. Incremental Parsing - Actual Implementation

**File:** `parser/src/incremental.rs`

**Current Status:**
```rust
// TODO: Implement actual incremental parsing logic
self.needs_full_reparse = true;
```

**What's Needed:**
- Track which AST nodes are affected by source changes
- Re-parse only affected regions
- Merge incremental updates into existing AST
- Handle cascading changes (e.g., changing a type affects all uses)

**Implementation Steps:**
1. Implement change impact analysis (which nodes are affected)
2. Implement incremental re-parsing for affected regions
3. Implement AST merging (combine old and new AST)
4. Handle edge cases (deletions, insertions, multi-line changes)
5. Add comprehensive tests

**Complexity:** High - This is a significant feature requiring:
- AST node tracking (which source spans map to which nodes)
- Dependency tracking (which nodes depend on changed nodes)
- Incremental lexer (re-tokenize only changed regions)
- AST diff/merge algorithm

#### 4. AST Query System - Complete Coverage

**File:** `parser/src/query.rs::get_children()`

**Current Status:**
- Basic node types covered
- Many node types return `Vec::new()` (not fully implemented)

**What's Needed:**
- Complete `get_children()` implementation for all AST node types
- Support for all node types:
  - Unit, Library, UsesClause
  - InterfaceSection, ImplementationSection
  - All statement types (TryStmt, WithStmt, CaseStmt, etc.)
  - All expression types
  - All type nodes
  - All declaration types

**Implementation Steps:**
1. Review all AST node types in `ast/src/lib.rs`
2. Implement `get_children()` for each node type
3. Add tests for each node type
4. Verify query system works for all AST structures

#### 5. Performance Optimization

**Current Status:**
- Benchmarks exist but optimizations not yet applied
- No performance profiling done

**What's Needed:**
1. **Profile the parser** to identify hot paths
2. **Optimize hot paths:**
   - Token lookups (consider caching)
   - Expression parsing (Pratt parser optimization)
   - Type checking (consider memoization)
3. **Memory optimization:**
   - Reduce AST node cloning
   - Use references where possible
   - Consider arena allocation for AST nodes
4. **Run benchmarks** and measure improvements

**Implementation Steps:**
1. Run `cargo bench` to establish baseline
2. Profile with `perf` or `cargo-flamegraph`
3. Identify bottlenecks
4. Apply optimizations incrementally
5. Re-benchmark to verify improvements
6. Document performance characteristics

## Priority Recommendations

### High Priority (Needed for Production)

1. **Error Recovery - Source Snippets** ⭐
   - Quick win, improves developer experience
   - Low complexity, high value
   - Estimated: 2-4 hours

2. **AST Query System - Complete Coverage** ⭐
   - Needed for IDE features (go to definition, refactoring)
   - Medium complexity, high value
   - Estimated: 4-8 hours

3. **Error Recovery - Actual Recovery** ⭐⭐
   - Critical for good error messages
   - Medium complexity, very high value
   - Estimated: 8-16 hours

### Medium Priority (Nice to Have)

4. **Performance Optimization** ⭐⭐
   - Important for large files
   - Medium-high complexity, medium value
   - Estimated: 16-32 hours (iterative)

### Low Priority (Future Enhancement)

5. **Incremental Parsing** ⭐⭐⭐
   - Complex feature, mainly for IDE integration
   - High complexity, medium value (for compiler use)
   - Estimated: 40-80 hours

## Quick Wins

1. **Implement `get_source_snippet()`** - Store source in parser, extract lines
2. **Complete `get_children()` in query.rs** - Add all missing node types
3. **Add basic error recovery** - Skip to next statement on error

## Testing Requirements

For each remaining feature:
- Unit tests for core functionality
- Integration tests for end-to-end scenarios
- Performance tests (for optimization work)
- Edge case tests (for error recovery)

---

**Next Steps:**
1. Start with `get_source_snippet()` (quick win)
2. Complete AST query system coverage
3. Implement basic error recovery
4. Profile and optimize performance
5. Consider incremental parsing for future IDE integration

