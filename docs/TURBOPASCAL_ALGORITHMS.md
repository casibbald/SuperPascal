# Turbo Pascal Compiler - Algorithm Reference

**Source:** [turbopascal.org](https://turbopascal.org/) - Extracted algorithms and patterns  
**Date:** 2024-12-14

This document provides detailed algorithm references extracted from Turbo Pascal compiler internals.

---

## 1. Symbol Table Hash Function Algorithm

**Location:** `symbol-tables-and-hash-functions.md`

### Algorithm Description
- Hash function splits identifiers into several linked lists
- Only one list is checked when searching for identifier
- Speed of search function and hash function design significantly impact compiler speed

### Implementation Pattern
```pascal
// Hash function design:
// 1. Calculate hash value from identifier
// 2. Use hash to select linked list (bucket)
// 3. Search only in selected list
// 4. O(1) average case, O(n) worst case
```

### SuperPascal Implementation
- Use hash table with chaining (linked lists)
- Fast hash function (FNV-1a or similar)
- Bucket selection via modulo
- Search only in selected bucket

---

## 2. Identifier Comparison Algorithm

**Location:** `comparing-identifiers.md`

### Algorithm Description
- Case-insensitive comparison of two identifiers
- Returns false if not equal, true if equal
- Id2 points to byte after its end if equal
- **Must be as fast as possible** - Called frequently

### Implementation Pattern
```pascal
// Fast case-insensitive comparison:
// 1. Compare character by character
// 2. Convert to uppercase for comparison
// 3. Early exit on first mismatch
// 4. Update pointer if match
```

### SuperPascal Implementation
- Use byte-by-byte comparison with case conversion
- Early exit optimization
- Cache uppercase conversion if needed
- Critical path - optimize heavily

---

## 3. Jump Optimization Algorithm

**Location:** `optimizing-jumps.md`, `code-generator.md`

### Algorithm Description
- Convert all near jumps to short jumps
- Only possible if jump displacement fits in byte
- Called just before target code generation
- **Iterative:** Loop until no more optimizations possible
- Each optimization reduces code size, creating new opportunities

### Implementation Pattern
```pascal
// Jump optimization algorithm:
// 1. Calculate all label offsets
// 2. For each jump:
//    a. Calculate displacement
//    b. If displacement fits in byte:
//       - Convert near jump to short jump
// 3. Recalculate offsets (may enable more optimizations)
// 4. Repeat until no changes
```

### SuperPascal Implementation
- **Z80:** Convert JP (absolute) to JR (relative) when possible
- **65C02:** Convert JMP (absolute) to BRA (relative) when possible
- **65C816:** Convert 16-bit relative to 8-bit relative when possible
- Iterative optimization until convergence

---

## 4. Symbol Table Compacting Algorithm

**Location:** `compacting-symbol-tables.md`

### Algorithm Description
- After each module is compiled, symbol tables are compacted
- Removes unused space from symbol tables
- Critical for memory efficiency on resource-constrained systems

### Implementation Pattern
```pascal
// Symbol table compacting:
// 1. After module compilation completes
// 2. Scan symbol table for unused entries
// 3. Remove unused space
// 4. Defragment if needed
// 5. Update all references
```

### SuperPascal Implementation
- Implement after each module compilation
- Mark unused entries
- Remove and defragment
- Update symbol references
- Essential for Tier 1 platforms

---

## 5. Constant Folding Algorithm

**Location:** `if-statement.md`, `while-statement.md`

### Algorithm Description
- Evaluate constant expressions at compile time
- If condition is constant:
  - IF: Only generate then or else branch
  - WHILE: If false, generate no code
- Reduces code size and execution time

### Implementation Pattern
```pascal
// Constant folding:
// 1. Evaluate expression during parsing
// 2. If result is constant:
//    a. IF: Generate only true/false branch
//    b. WHILE: Generate code only if true
//    c. FOR: Calculate loop bounds
// 3. Eliminate dead code
```

### SuperPascal Implementation
- Evaluate expressions during semantic analysis
- Track constant values
- Generate code only for non-constant branches
- Eliminate dead code

---

## 6. Expression Evaluation Algorithm

**Location:** `processing-expressions.md`, `calculator.md`

### Algorithm Description
- Incremental evaluation during parsing
- Type conversion on-the-fly
- Register allocation for expressions
- Save/restore expression state for nested expressions

### Implementation Pattern
```pascal
// Expression evaluation:
// 1. Parse expression incrementally
// 2. Convert types as needed
// 3. Allocate registers for intermediate results
// 4. Save/restore state for nested expressions
// 5. Generate code during evaluation
```

### SuperPascal Implementation
- Process expressions during parsing
- Implement type conversion rules
- Allocate registers per target ABI
- Save/restore expression state
- Generate IR incrementally

---

## 7. Register Optimization Algorithm

**Location:** `code-generator.md`, `with-statement.md`

### Algorithm Description
- Remove redundant register loadings
- ES:DI optimization for address loading
- Track register usage across basic blocks
- Optimize register allocation

### Implementation Pattern
```pascal
// Register optimization:
// 1. Track register contents across blocks
// 2. Identify redundant loadings
// 3. Remove redundant instructions
// 4. Optimize address loading (ES:DI)
// 5. Recalculate if needed
```

### SuperPascal Implementation
- Track register usage per target
- Remove redundant loadings
- Optimize address loading per target ABI
- Platform-specific optimizations

---

## 8. Reference Resolution Algorithm

**Location:** `creating-references.md`, `linker.md`

### Algorithm Description
- Create reference records for unresolved symbols
- Defer address resolution until linking
- Store all information needed for resolution
- Resolve during linking phase

### Implementation Pattern
```pascal
// Reference resolution:
// 1. During code generation:
//    a. Create reference record for unresolved symbol
//    b. Store symbol name, type, location
// 2. During linking:
//    a. Resolve all references
//    b. Update code with addresses
//    c. Handle relocation if needed
```

### SuperPascal Implementation
- Create reference records in IR
- Store symbol information
- Resolve during linking
- Update code with addresses

---

## 9. Unit Dependency Algorithm

**Location:** `processing-used-units.md`, `loading-used-units.md`

### Algorithm Description
- Check if unit is already loaded
- Check if unit needs recompilation
- Calculate checksum of public identifiers
- Load units in dependency order

### Implementation Pattern
```pascal
// Unit dependency:
// 1. For each used unit:
//    a. Check if already loaded
//    b. If not, find unit file
//    c. Check if recompilation needed (checksum)
//    d. Load unit
// 2. Process in dependency order
```

### SuperPascal Implementation
- Track loaded units
- Calculate checksums for change detection
- Load units in dependency order
- Handle circular dependencies

---

## 10. Type Conversion Algorithm

**Location:** `converting-number-types.md`, `checking-type-compatibility.md`

### Algorithm Description
- Check type compatibility
- Convert types as needed
- Handle type promotion
- Generate conversion code

### Implementation Pattern
```pascal
// Type conversion:
// 1. Check type compatibility
// 2. If compatible:
//    a. Determine conversion needed
//    b. Generate conversion code
//    c. Update expression type
// 3. Handle type promotion
```

### SuperPascal Implementation
- Implement type compatibility rules
- Generate type conversion code
- Handle type promotion
- Update expression types

---

## 11. String Processing Algorithm

**Location:** `processing-and-converting-strings.md`, `string-operations.md`

### Algorithm Description
- Process string constants
- Convert strings as needed
- Handle string operations
- Optimize string handling

### Implementation Pattern
```pascal
// String processing:
// 1. Process string constants
// 2. Handle string operations (concatenation, etc.)
// 3. Convert strings as needed
// 4. Optimize string handling
```

### SuperPascal Implementation
- Process string literals
- Implement string operations
- Handle string conversions
- Optimize for target platform

---

## 12. Set Operations Algorithm

**Location:** `set-operations.md`, `processing-set-constants.md`

### Algorithm Description
- Process set constants
- Handle set operations (union, intersection, etc.)
- Optimize set representation
- Generate set operation code

### Implementation Pattern
```pascal
// Set operations:
// 1. Process set constants
// 2. Handle set operations
// 3. Optimize set representation (bit sets)
// 4. Generate set operation code
```

### SuperPascal Implementation
- Implement set data structure
- Handle set operations
- Optimize set representation
- Generate efficient set code

---

## 13. Code Size Calculation Algorithm

**Location:** `calculating-code-size-and-offsets.md`

### Algorithm Description
- Calculate code size before generation
- Calculate offsets for jumps and labels
- Calculate size of reference records
- Update offsets iteratively

### Implementation Pattern
```pascal
// Code size calculation:
// 1. Process intermediate code instructions
// 2. Calculate size of each instruction
// 3. Calculate offsets for jumps
// 4. Calculate offsets for labels
// 5. Update iteratively if needed
```

### SuperPascal Implementation
- Calculate code size per target
- Calculate jump offsets
- Calculate label offsets
- Update iteratively for optimization

---

## 14. Error Handling Algorithm

**Location:** `error-handling.md`, `error-reporting.md`

### Algorithm Description
- Save stack position and return address
- Report error with context
- Support editor integration
- Continue compilation if possible

### Implementation Pattern
```pascal
// Error handling:
// 1. Save stack position (SetErrorAddress)
// 2. Save return address
// 3. Report error (module, line, column)
// 4. Continue compilation if possible
// 5. Support editor integration
```

### SuperPascal Implementation
- Implement lightweight error context
- Report errors with location
- Support editor integration
- Continue compilation for multiple errors

---

## 15. Assembler Processing Algorithm

**Location:** `processing-assembler-instructions.md`, `extracting-assembler-tokens.md`

### Algorithm Description
- Create temporary symbol table
- Extract assembler tokens
- Process assembler expressions
- Generate assembler opcodes

### Implementation Pattern
```pascal
// Assembler processing:
// 1. Create temporary symbol table
// 2. Extract assembler tokens
// 3. Process assembler expressions
// 4. Process assembler directives
// 5. Generate target opcodes
```

### SuperPascal Implementation
- Support inline assembly
- Create temporary symbol tables
- Process assembler syntax
- Generate target-specific opcodes

---

## Algorithm Complexity Analysis

### Time Complexity
- **Hash table lookup:** O(1) average, O(n) worst
- **Identifier comparison:** O(n) where n is identifier length
- **Jump optimization:** O(m) where m is number of jumps (iterative)
- **Symbol table compacting:** O(n) where n is number of symbols
- **Constant folding:** O(1) per expression
- **Expression evaluation:** O(n) where n is expression size

### Space Complexity
- **Hash table:** O(n) where n is number of symbols
- **Linked lists:** O(m) where m is number of forward references
- **Intermediate code:** O(p) where p is program size
- **Symbol tables:** O(n) where n is number of symbols

---

## Implementation Priorities

### High Priority (Core Functionality)
1. Hash table for symbols
2. Identifier comparison (fast path)
3. Constant folding
4. Expression evaluation
5. Type conversion

### Medium Priority (Optimization)
1. Jump optimization
2. Register optimization
3. Symbol table compacting
4. Code size calculation

### Low Priority (Advanced)
1. Set operations
2. String optimization
3. Assembler processing
4. Unit dependency tracking

---

**Version:** 1.0  
**Date:** 2024-12-14  
**Status:** Algorithm Reference Complete

