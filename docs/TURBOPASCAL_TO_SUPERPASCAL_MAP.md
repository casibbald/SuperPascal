# Turbo Pascal → SuperPascal Implementation Mapping

**Purpose:** Direct mapping of Turbo Pascal compiler techniques to SuperPascal implementation needs

---

## 1. Symbol Table Implementation

### Turbo Pascal Approach
- Hash tables with linked lists (chaining)
- Hash function: `CurrentIdentifierHash := -Length + sum(Byte[i] & $DF) << 1`
- Mask parameter defines number of buckets (2^N)
- Only search one bucket (linked list)

### SuperPascal Implementation
**Location:** `crates/compiler-rs/symbols/src/lib.rs`

**Current Status:** ✅ Basic symbol table implemented
**Needs:**
- ⏳ Hash table implementation (currently using HashMap)
- ⏳ Custom hash function for identifiers
- ⏳ Symbol table compacting after each module
- ⏳ Public identifier checksums for unit dependencies

**Action Items:**
1. Implement hash table with chaining
2. Add hash function similar to Turbo Pascal
3. Implement compacting after module compilation
4. Add checksum calculation for public identifiers

---

## 2. Identifier Comparison

### Turbo Pascal Approach
- Case-insensitive comparison
- Assembly-optimized (inline assembly)
- Early exit on mismatch
- Updates pointer if match

### SuperPascal Implementation
**Location:** `crates/compiler-rs/lexer/src/lib.rs`

**Current Status:** ✅ Token comparison exists
**Needs:**
- ⏳ Fast case-insensitive identifier comparison
- ⏳ Optimize for hot path (called frequently)

**Action Items:**
1. Implement fast case-insensitive comparison
2. Consider caching uppercase conversion
3. Early exit optimization

---

## 3. Jump Optimization

### Turbo Pascal Approach
- Convert near jumps to short jumps (if displacement fits)
- Iterative: loop until no more optimizations
- Each optimization creates new opportunities

### SuperPascal Implementation
**Location:** `crates/compiler-rs/ir/src/lib.rs` (future optimization pass)

**Current Status:** ⏳ Not yet implemented
**Needs:**
- ⏳ Jump size optimization pass
- ⏳ Platform-specific optimizations:
  - Z80: JP → JR
  - 65C02: JMP → BRA
  - 65C816: 16-bit → 8-bit relative

**Action Items:**
1. Add jump optimization pass to IR
2. Implement platform-specific jump size checks
3. Iterative optimization until convergence

---

## 4. Constant Folding

### Turbo Pascal Approach
- Evaluate constant expressions at compile time
- IF: Only generate then/else branch if constant
- WHILE: Generate no code if false constant

### SuperPascal Implementation
**Location:** `crates/compiler-rs/semantics/src/lib.rs` (future enhancement)

**Current Status:** ⏳ Not yet implemented
**Needs:**
- ⏳ Constant expression evaluation
- ⏳ Dead code elimination for constant branches

**Action Items:**
1. Add constant folding to semantic analysis
2. Evaluate expressions during type checking
3. Eliminate dead code for constant branches

---

## 5. Expression Evaluation

### Turbo Pascal Approach
- Incremental evaluation during parsing
- Type conversion on-the-fly
- Register allocation for expressions
- Save/restore expression state

### SuperPascal Implementation
**Location:** `crates/compiler-rs/parser/src/lib.rs`, `crates/compiler-rs/semantics/src/lib.rs`

**Current Status:** ✅ Basic expression parsing
**Needs:**
- ⏳ Incremental type conversion
- ⏳ Register allocation during IR generation
- ⏳ Expression state save/restore

**Action Items:**
1. Implement type conversion during expression parsing
2. Add register allocation to IR builder
3. Implement expression state management

---

## 6. Single-Pass Compilation

### Turbo Pascal Approach
- Process source in single pass
- Generate code as you parse
- No full AST storage
- Reduces memory requirements

### SuperPascal Implementation
**Location:** Compiler driver (future)

**Current Status:** ⏳ Multi-pass (AST → Semantics → IR)
**Needs:**
- ⏳ Single-pass option for Tier 1 platforms
- ⏳ Incremental IR generation during parsing
- ⏳ Memory-efficient compilation

**Action Items:**
1. Design single-pass compilation mode
2. Implement incremental IR generation
3. Add compiler flag for single-pass mode

---

## 7. Error Handling

### Turbo Pascal Approach
- SetErrorAddress saves stack position
- Report: module, line, column
- Editor integration support

### SuperPascal Implementation
**Location:** `crates/compiler-rs/errors/src/lib.rs`

**Current Status:** ✅ Enhanced error system implemented
**Needs:**
- ✅ Already has module, line, column
- ✅ Already supports editor integration
- ⏳ Lightweight error context (for memory efficiency)

**Action Items:**
1. Optimize error context storage
2. Ensure editor integration works
3. Add lightweight mode for Tier 1 platforms

---

## 8. System Functions

### Turbo Pascal Approach
- Shared processing procedures
- Parameter-based dispatch
- Register-based returns (DX:AX for Boolean)

### SuperPascal Implementation
**Location:** Runtime library (future)

**Current Status:** ⏳ Not yet implemented
**Needs:**
- ⏳ System function implementation
- ⏳ Shared processing procedures
- ⏳ Register-based returns per target ABI

**Action Items:**
1. Design system function architecture
2. Implement shared processing procedures
3. Use target-specific return registers

---

## 9. Code Generation

### Turbo Pascal Approach
- Procedure-based instruction generation
- Two records: opcodes + references
- Deferred address resolution

### SuperPascal Implementation
**Location:** `crates/compiler-rs/ir/src/lib.rs` (IR), Backend crates (future)

**Current Status:** ✅ IR implemented
**Needs:**
- ⏳ Backend code generation
- ⏳ Reference resolution during linking
- ⏳ Platform-specific instruction generation

**Action Items:**
1. Implement backend code generation
2. Add reference resolution to linker
3. Platform-specific instruction selection

---

## 10. Symbol Table Compacting

### Turbo Pascal Approach
- After each module, compact symbol tables
- Remove unused space
- Critical for memory efficiency

### SuperPascal Implementation
**Location:** `crates/compiler-rs/symbols/src/lib.rs`

**Current Status:** ⏳ Not yet implemented
**Needs:**
- ⏳ Compacting after module compilation
- ⏳ Defragmentation if needed
- ⏳ Reference updates

**Action Items:**
1. Implement symbol table compacting
2. Add defragmentation if needed
3. Update all symbol references

---

## 11. Unit Processing

### Turbo Pascal Approach
- Check if unit already loaded
- Checksum for recompilation detection
- Load in dependency order

### SuperPascal Implementation
**Location:** Compiler driver (future)

**Current Status:** ⏳ Not yet implemented
**Needs:**
- ⏳ Unit dependency tracking
- ⏳ Checksum calculation
- ⏳ Dependency order loading

**Action Items:**
1. Design unit system
2. Implement dependency tracking
3. Add checksum calculation

---

## 12. Register Optimization

### Turbo Pascal Approach
- Remove redundant register loadings
- ES:DI optimization
- Track register usage

### SuperPascal Implementation
**Location:** IR optimization passes (future)

**Current Status:** ⏳ Not yet implemented
**Needs:**
- ⏳ Register usage tracking
- ⏳ Redundant loading removal
- ⏳ Platform-specific optimizations

**Action Items:**
1. Implement register usage tracking
2. Add redundant loading removal
3. Platform-specific register optimization

---

## Implementation Priority Matrix

### High Priority (Core Functionality)
| Turbo Pascal Technique | SuperPascal Status | Priority |
|------------------------|-------------------|----------|
| Hash table for symbols | ⏳ Needs implementation | **HIGH** |
| Fast identifier comparison | ⏳ Needs optimization | **HIGH** |
| Constant folding | ⏳ Needs implementation | **HIGH** |
| Expression evaluation | ✅ Basic, needs enhancement | **MEDIUM** |
| Error handling | ✅ Implemented | **LOW** |

### Medium Priority (Optimization)
| Turbo Pascal Technique | SuperPascal Status | Priority |
|------------------------|-------------------|----------|
| Jump optimization | ⏳ Needs implementation | **MEDIUM** |
| Register optimization | ⏳ Needs implementation | **MEDIUM** |
| Symbol table compacting | ⏳ Needs implementation | **MEDIUM** |
| Code size calculation | ⏳ Needs implementation | **MEDIUM** |

### Low Priority (Advanced)
| Turbo Pascal Technique | SuperPascal Status | Priority |
|------------------------|-------------------|----------|
| Single-pass compilation | ⏳ Needs design | **LOW** |
| Unit processing | ⏳ Needs design | **LOW** |
| System functions | ⏳ Needs design | **LOW** |
| Assembler integration | ⏳ Needs design | **LOW** |

---

## Quick Reference: File Locations

### SuperPascal Compiler Crates
- **Tokens:** `crates/compiler-rs/tokens/src/lib.rs`
- **Lexer:** `crates/compiler-rs/lexer/src/lib.rs`
- **Parser:** `crates/compiler-rs/parser/src/lib.rs`
- **AST:** `crates/compiler-rs/ast/src/lib.rs`
- **Semantics:** `crates/compiler-rs/semantics/src/lib.rs`
- **Symbols:** `crates/compiler-rs/symbols/src/lib.rs`
- **Types:** `crates/compiler-rs/types/src/lib.rs`
- **IR:** `crates/compiler-rs/ir/src/lib.rs`
- **Errors:** `crates/compiler-rs/errors/src/lib.rs`

### Turbo Pascal Reference Pages
- **Symbol Tables:** `docs/turbopascal_scraped/symbol-tables-and-hash-functions.md`
- **Identifier Comparison:** `docs/turbopascal_scraped/comparing-identifiers.md`
- **Code Generator:** `docs/turbopascal_scraped/code-generator.md`
- **Expressions:** `docs/turbopascal_scraped/expressions.md`
- **System Functions:** `docs/turbopascal_scraped/system-functions.md`

---

## Next Steps

1. ✅ **Extract all knowledge** - Complete
2. ✅ **Create algorithm reference** - Complete
3. ✅ **Create implementation mapping** - Complete
4. ⏳ **Implement hash tables** - Next priority
5. ⏳ **Implement constant folding** - High priority
6. ⏳ **Implement jump optimization** - Medium priority
7. ⏳ **Implement symbol table compacting** - Medium priority

---

**Version:** 1.0  
**Date:** 2024-12-14  
**Status:** Implementation Mapping Complete

