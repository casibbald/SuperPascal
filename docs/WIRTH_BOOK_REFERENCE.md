# Wirth's Compiler Construction Book - Quick Reference

## Overview

Niklaus Wirth's "Compiler Construction" (2005) is available in markdown format and serves as a **primary reference** for SuperPascal compiler implementation, especially for Tier 1 micro-compiler design.

**Location:** `docs/PDFs/compiler construction wirth 2005.md`

**Why This Book:**
- Emphasizes simplicity and educational value
- One-pass compilation techniques (perfect for Z80 micro-compiler)
- Complete working compiler example (Oberon-0)
- Clear, understandable explanations
- Practical, not theoretical

---

## Chapter Mapping to SuperPascal Implementation

### Phase 1: Tokens and Lexer

#### Chapter 7.1: The Scanner
- **Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL**
- **Use For:**
  - Implementing `tokens` crate
  - Implementing `lexer` crate
  - Token recognition algorithms
  - Comment handling
  - Number/string literal parsing
- **Key Concepts:**
  - Single-pass scanning
  - Token lookahead
  - Error recovery

---

### Phase 2: Parser and AST

#### Chapter 4.1: The Method of Recursive Descent
- **Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL**
- **Use For:**
  - Implementing `parser` crate
  - Recursive descent parsing
  - Grammar implementation
- **Key Concepts:**
  - Recursive descent algorithm
  - Expression parsing
  - Error recovery

#### Chapter 7.2: The Parser
- **Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL**
- **Use For:**
  - Complete parser implementation
  - Oberon-0 parser as reference
  - AST construction
- **Key Concepts:**
  - Parser structure
  - Statement parsing
  - Declaration parsing

#### Chapter 5: Attributed Grammars and Semantics
- **Relevance:** ⭐⭐⭐⭐ **VERY USEFUL**
- **Use For:**
  - Understanding semantic analysis
  - Type rules
  - Translation rules
- **Key Concepts:**
  - Attribute grammars
  - Type checking
  - Code generation preparation

---

### Phase 3: Semantic Analysis

#### Chapter 8: Consideration of Context Specified by Declarations
- **Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL**
- **Use For:**
  - Implementing `symbols` crate
  - Symbol table design
  - Declaration handling
  - Type representation
- **Key Concepts:**
  - Symbol table structure
  - Declaration processing
  - Type entries
  - Data representation

---

### Phase 4: Code Generation

#### Chapter 9: A RISC Architecture as Target
- **Relevance:** ⭐⭐⭐⭐ **USEFUL**
- **Use For:**
  - Understanding target architecture design
  - Instruction set design
  - Register usage
- **Note:** Wirth uses RISC, but concepts apply to Z80/m68k

#### Chapter 10: Expressions and Assignments
- **Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL**
- **Use For:**
  - Implementing `ir` crate
  - Code generation for expressions
  - Stack-based code generation
  - Register allocation basics
- **Key Concepts:**
  - Stack principle
  - Delayed code generation
  - Indexed variables
  - Record fields

#### Chapter 11: Conditional and Repeated Statements and Boolean Expressions
- **Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL**
- **Use For:**
  - Code generation for control flow
  - If/while/for statements
  - Boolean expressions
  - Jump generation
- **Key Concepts:**
  - Conditional statements
  - Loops
  - Boolean operations
  - Jump optimization

#### Chapter 12: Procedures and the Concept of Locality
- **Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL**
- **Use For:**
  - Implementing procedure/function codegen
  - Stack frame management
  - Parameter passing
  - Local variable addressing
- **Key Concepts:**
  - Run-time organization
  - Stack frames
  - Parameter passing
  - Procedure calls
  - Function returns

---

### Advanced Topics

#### Chapter 13: Elementary Data Types
- **Relevance:** ⭐⭐⭐ **USEFUL**
- **Use For:**
  - Additional data types (REAL, SET)
  - Type compatibility
- **Note:** Tier 2 features

#### Chapter 14: Open Arrays, Pointers and Procedure Types
- **Relevance:** ⭐⭐⭐ **USEFUL**
- **Use For:**
  - Advanced types (Tier 2/3)
  - Dynamic data structures
  - Pointers
- **Note:** Tier 2/3 features

#### Chapter 15: Modules and Separate Compilation
- **Relevance:** ⭐⭐⭐⭐ **VERY USEFUL**
- **Use For:**
  - Unit/module system (Tier 2)
  - Separate compilation
  - Symbol files
  - Interface checking
- **Note:** Tier 2 feature

#### Chapter 16: Code Optimizations
- **Relevance:** ⭐⭐⭐ **USEFUL**
- **Use For:**
  - Basic optimizations
  - Register allocation
  - Frontend/backend structure
- **Note:** Advanced topic, implement later

---

## Appendix References

### Appendix A: Syntax
- **Oberon-0 Grammar** - Reference for Tier 1 grammar
- **Oberon Grammar** - Full language reference

### Appendix C: The Oberon-0 Compiler
- **C.1: The Scanner** - Complete scanner implementation
- **C.2: The Parser** - Complete parser implementation
- **C.3: The Code Generator** - Complete code generator

**Note:** These appendices contain **complete working code** that can serve as a reference implementation.

---

## How to Use This Book

### 1. For Tier 1 Micro-Compiler (Z80)

**Primary Reference:**
- Chapters 7.1, 7.2 (Scanner and Parser)
- Chapter 8 (Symbol Table)
- Chapters 10, 11, 12 (Code Generation)
- Appendix C (Complete Implementation)

**Approach:**
- Study Wirth's one-pass approach
- Adapt Oberon-0 compiler to Pascal subset
- Simplify for Z80 architecture

### 2. For Tier 2 Full Compiler (Rust)

**Primary Reference:**
- Same chapters as Tier 1
- Chapter 5 (Attributed Grammars)
- Chapter 15 (Modules)
- Use as semantic reference alongside FPC

**Approach:**
- Use Wirth for algorithm understanding
- Use FPC for Pascal-specific semantics
- Implement in Rust with modern patterns

### 3. For Learning and Understanding

**Reading Order:**
1. Chapter 1 (Introduction) - Overview
2. Chapter 4.1 (Recursive Descent) - Parsing method
3. Chapter 7 (Parser for Oberon-0) - Complete example
4. Chapter 8 (Declarations) - Symbol table
5. Chapters 10-12 (Code Generation) - Codegen techniques

---

## Key Differences: Wirth vs. FPC

### When to Use Wirth:
- **Algorithm design** - How to structure a compiler
- **Educational approach** - Simple, understandable methods
- **One-pass compilation** - For Tier 1 micro-compiler
- **Code generation basics** - Fundamental techniques

### When to Use FPC:
- **Pascal semantics** - Language-specific rules
- **Type system** - Pascal type compatibility
- **Standard library** - Built-in procedures/functions
- **Platform-specific** - m68k calling conventions

### Combined Approach:
- **Use Wirth for structure** - How to build a compiler
- **Use FPC for semantics** - What Pascal means
- **Adapt to Rust** - Modern implementation patterns

---

## Quick Lookup Guide

### Need to implement... → Read Chapter...

| Task | Chapter(s) |
|------|-----------|
| Token recognition | 7.1 |
| Lexical analysis | 7.1 |
| Recursive descent parser | 4.1, 7.2 |
| Symbol table | 8 |
| Type checking | 5, 8 |
| Expression codegen | 10 |
| Control flow codegen | 11 |
| Procedure codegen | 12 |
| Stack frames | 12 |
| Error handling | 7.3 |
| Complete compiler | Appendix C |

---

## Integration with Implementation Strategy

### Phase 1: Tokens and Lexer
- **Read:** Chapter 7.1 (The Scanner)
- **Study:** Scanner implementation in Appendix C.1
- **Apply:** One-pass scanning to `lexer` crate

### Phase 2: Parser
- **Read:** Chapter 4.1 (Recursive Descent), Chapter 7.2 (The Parser)
- **Study:** Parser implementation in Appendix C.2
- **Apply:** Recursive descent to `parser` crate

### Phase 3: Semantics
- **Read:** Chapter 5 (Attributed Grammars), Chapter 8 (Declarations)
- **Study:** Symbol table design
- **Apply:** To `symbols` and `semantics` crates

### Phase 4: Code Generation
- **Read:** Chapters 10, 11, 12 (Code Generation)
- **Study:** Code generator in Appendix C.3
- **Apply:** Codegen techniques to `ir` and backend crates

---

## Advantages of Markdown Version

1. **Searchable** - Easy to find specific topics
2. **Copy-paste** - Can extract code examples
3. **Version control** - Can track changes
4. **Quick reference** - Fast lookups
5. **Code-friendly** - Can reference in documentation

---

## Example: Using Wirth's Scanner

Wirth's scanner (Chapter 7.1) provides a complete, working example that can be adapted:

```pascal
PROCEDURE GetSym;
BEGIN
  WHILE (ch <= ' ') DO GetCh;
  ...
END GetSym;
```

**Adaptation for SuperPascal:**
- Convert Pascal code to Rust
- Adapt to SuperPascal token set
- Maintain same algorithm structure
- Add error handling

---

## Summary

**Wirth's book is essential for:**
- ✅ Understanding compiler structure
- ✅ Learning one-pass compilation
- ✅ Educational compiler design
- ✅ Complete working examples

**Use alongside:**
- FPC source for Pascal semantics
- Platform manuals for target architectures
- Language specifications for grammar

---

**Version:** 1.0  
**Date:** 2024  
**Status:** Quick Reference Guide

