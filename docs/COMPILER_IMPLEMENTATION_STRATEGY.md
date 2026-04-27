# SuperPascal Compiler Implementation Strategy

## Overview

This document outlines a practical, incremental strategy for implementing the SuperPascal compiler using FreePascal (FPC) as a semantic reference. The approach prioritizes:

1. **Incremental development** - Build one component at a time
2. **Test-driven development** - Write tests first
3. **FPC as oracle** - Use FPC to validate semantics
4. **Start simple** - Begin with Tier 1 features only
5. **Platform-first** - Target ZealZ80 initially

---

## Phase 0: Setup and Infrastructure

### 1. Create Rust Workspace Structure

```bash
cd ../SuperPascal
mkdir -p crates/compiler-rs
cd crates/compiler-rs
cargo init --name compiler
```

### 2. Set Up Workspace Cargo.toml

Create `crates/compiler-rs/Cargo.toml` with workspace structure:

```toml
[workspace]
members = [
    "tokens",
    "lexer",
    "parser",
    "ast",
    "semantics",
    "types",
    "symbols",
    "ir",
    "errors",
    "diagnostics",
    "backend-zealz80",
    "object-zealz80",
    "runtime-spec",
    "driver",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
```

### 3. Create Test Infrastructure

```bash
mkdir -p tests/compliance
mkdir -p tests/unit
mkdir -p tests/integration
```

### 4. Set Up CI/CD

- GitHub Actions for `cargo test`
- Pre-commit hooks for formatting
- Coverage tracking

---

## Phase 1: Foundation - Tokens and Lexer (Week 1-2)

### Goal
Get basic tokenization working for Tier 1 Pascal subset.

### Step 1.1: Create `tokens` Crate

**Why start here:** Pure data structures, no dependencies, easy to test.

```bash
cd crates/compiler-rs
cargo new --lib tokens
```

**Implementation:**
- Define `TokenKind` enum (all Pascal keywords, operators, literals)
- Define `Token` struct with span information
- Keyword lookup table
- Operator precedence table

**FPC Reference:**
- Study `freePascal/compiler/token.pas` for token definitions
- Study `freePascal/compiler/scanner.pas` for scanning logic

**Wirth Reference:**
- `docs/PDFs/compiler construction wirth 2005.md` - Chapter 7.1 (The scanner)

**Tests:**
- Token recognition for all keywords
- Number parsing (decimal, hex)
- String literal parsing
- Comment handling (`{ }`, `(* *)`)

**Deliverable:** `tokens` crate with 100% test coverage.

---

### Step 1.2: Create `lexer` Crate

**Why next:** Uses tokens, single responsibility, straightforward implementation.

```bash
cargo new --lib lexer
```

**Dependencies:**
- `tokens`

**Implementation:**
- Single-pass scanner
- Token lookahead (1-2 tokens)
- Comment stripping
- Directive tokenization (`{$...}`) - basic support

**FPC Reference:**
- `freePascal/compiler/scanner.pas` - Scanner implementation
- Study how FPC handles comments, directives, string literals

**Wirth Reference:**
- `docs/PDFs/compiler construction wirth 2005.md` - Chapter 7.1 (The scanner)

**Tests:**
- Tokenize simple programs
- Comment handling
- String literals with escapes
- Error cases (unterminated strings, invalid tokens)

**Deliverable:** `lexer` that can tokenize Tier 1 Pascal programs.

---

## Phase 2: Parsing - AST Construction (Week 3-4)

### Step 2.1: Create `ast` Crate

**Why first:** Defines the data structures parser will build.

```bash
cargo new --lib ast
```

**Dependencies:**
- `tokens` (for spans)

**Implementation:**
- AST node types for Tier 1:
  - `Program`, `Block`
  - `VarDecl`, `ConstDecl`, `TypeDecl`
  - `ProcDecl`, `FuncDecl`
  - `IfStmt`, `WhileStmt`, `ForStmt`, `RepeatStmt`, `CaseStmt`
  - `AssignStmt`, `CallExpr`
  - `BinaryExpr`, `UnaryExpr`, `LiteralExpr`
  - `RecordType`, `ArrayType`
- Use `bumpalo` or similar for arena allocation

**FPC Reference:**
- `freePascal/compiler/nodes.pas` - AST node definitions
- Study how FPC structures its AST

**Tests:**
- AST node construction
- Span preservation
- Immutability

**Deliverable:** Complete AST definition for Tier 1.

---

### Step 2.2: Create `parser` Crate

**Why next:** Builds AST from tokens.

```bash
cargo new --lib parser
```

**Dependencies:**
- `tokens`
- `ast`
- `errors`

**Implementation:**
- Recursive descent parser
- Pratt parser for expressions (operator precedence)
- Error recovery (panic mode)
- Support Tier 1 grammar only

**FPC Reference:**
- `freePascal/compiler/parser.pas` - Parser implementation
- Study FPC's recursive descent approach
- Note: FPC uses hand-written parser, not generated

**Wirth Reference:**
- `docs/PDFs/compiler construction wirth 2005.md` - Chapter 4.1 (Recursive descent), Chapter 7.2 (The parser)

**Grammar Coverage (Tier 1):**
- Program structure
- Variable/constant/type declarations
- Procedures and functions (no nesting)
- Control flow (if, while, for, repeat, case)
- Expressions (arithmetic, relational, boolean)
- Arrays and records (simple)

**Tests:**
- Parse simple programs
- Parse all Tier 1 constructs
- Error recovery
- Invalid syntax handling

**Deliverable:** Parser that builds AST for Tier 1 Pascal programs.

---

## Phase 3: Semantic Analysis (Week 5-7)

### Step 3.1: Create `types` Crate

**Why first:** Type system is foundation for semantic analysis.

```bash
cargo new --lib types
```

**Dependencies:**
- `ast`

**Implementation:**
- Type representation:
  - Primitive types (`integer`, `byte`, `word`, `boolean`, `char`)
  - Array types (with bounds)
  - Record types (with field offsets)
  - Pointer types
- Type operations:
  - Type equality
  - Assignment compatibility
  - Size calculation
  - Alignment calculation

**FPC Reference:**
- `freePascal/compiler/types.pas` - Type system
- `freePascal/compiler/defs.pas` - Type definitions
- Study FPC's type representation and compatibility rules

**Tests:**
- Type equality
- Assignment compatibility
- Size/alignment calculation
- Record field offset calculation

**Deliverable:** Complete type system for Tier 1.

---

### Step 3.2: Create `symbols` Crate

**Why next:** Symbol table needed for semantic analysis.

```bash
cargo new --lib symbols
```

**Dependencies:**
- `ast`
- `types`

**Implementation:**
- Symbol table with scopes
- Symbol kinds (var, const, type, proc, func)
- Name resolution
- Scope management (global only for Tier 1)

**FPC Reference:**
- `freePascal/compiler/symtable.pas` - Symbol table
- `freePascal/compiler/symsym.pas` - Symbol definitions
- Study FPC's symbol table structure

**Wirth Reference:**
- `docs/PDFs/compiler construction wirth 2005.md` - Chapter 8 (Consideration of Context Specified by Declarations)

**Tests:**
- Symbol insertion/lookup
- Scope management
- Name resolution
- Duplicate symbol detection

**Deliverable:** Symbol table for Tier 1 (global scope only).

---

### Step 3.3: Create `semantics` Crate

**Why next:** Ties together types and symbols for semantic checking.

```bash
cargo new --lib semantics
```

**Dependencies:**
- `ast`
- `types`
- `symbols`
- `errors`

**Implementation:**
- Type checking
- Symbol resolution
- Assignment compatibility checking
- Constant folding
- Control flow analysis (basic)

**FPC Reference:**
- `freePascal/compiler/typecheck.pas` - Type checking
- `freePascal/compiler/check.pas` - Semantic checking
- Study FPC's type checking rules

**Semantic Rules (Tier 1):**
- All identifiers must be declared
- Types must match in assignments
- Procedure/function calls must match signatures
- Array bounds checking
- Record field access validation

**Tests:**
- Type checking for all Tier 1 constructs
- Symbol resolution
- Assignment compatibility
- Error detection (type mismatches, undefined symbols)

**Deliverable:** Semantic analyzer for Tier 1 Pascal.

---

## Phase 4: IR and Code Generation (Week 8-12)

**Reference:** See `TURBO51_ANALYSIS.md` for optimization strategies from Turbo51 compiler.

### Step 4.1: Create `ir` Crate

**Why first:** IR is platform-agnostic, needed before codegen.

```bash
cargo new --lib ir
```

**Dependencies:**
- `ast`
- `types`

**Implementation:**
- IR1 (linear IR):
  - Basic blocks
  - IR instructions (MOV, ADD, SUB, MUL, DIV, CMP, JUMP, CJUMP, CALL, RET)
  - IR values (immediate, register, memory, temp)
- IR builder
- Basic optimizations (constant folding, dead code elimination)
  - **Reference Turbo51:** See `TURBO51_ANALYSIS.md` for 13 optimization techniques
  - **Optimization Roadmap:** See `OPTIMIZATION_ROADMAP.md` for implementation plan

**FPC Reference:**
- `freePascal/compiler/ncgutil.pas` - Code generation utilities
- Study FPC's intermediate representation (if any)
- Note: FPC may generate code directly, but we want IR for multi-platform

**Wirth Reference:**
- `docs/PDFs/compiler construction wirth 2005.md` - Chapter 10 (Expressions and Assignments), Chapter 11 (Conditional and Repeated Statements), Chapter 12 (Procedures)

**Tests:**
- IR generation for simple programs
- Basic block construction
- Control flow representation
- Constant folding

**Deliverable:** IR generator for Tier 1 Pascal.

---

### Step 4.2: Create `runtime-spec` Crate

**Why next:** Defines runtime contracts for codegen.

```bash
cargo new --lib runtime-spec
```

**Dependencies:** None (specification only)

**Implementation:**
- Runtime helper function signatures
- ABI contracts
- Platform-specific runtime specs (Z80 initially)

**FPC Reference:**
- `freePascal/rtl/` - Runtime library
- Study FPC's runtime helper functions

**Deliverable:** Runtime specification for ZealZ80.

---

### Step 4.3: Create `backend-zealz80` Crate

**Why next:** First codegen backend for primary platform.

```bash
cargo new --lib backend-zealz80
```

**Dependencies:**
- `ir`
- `types`
- `runtime-spec`

**Implementation:**
- IR → Z80 assembly codegen
- Register allocation (simple: HL, DE, BC for temporaries)
- Stack frame management (IX as frame pointer)
- Parameter passing (stack-based)
- Return value handling (HL for 16-bit, A for 8-bit)
- Peephole optimizer

**FPC Reference:**
- `freePascal/compiler/z80/` - **Z80-specific codegen (PRIMARY REFERENCE)**
  - `z80/cpubase.pas` - Z80 registers, opcodes, base types
  - `z80/cpupara.pas` - Z80 calling conventions and parameter passing
  - `z80/cgcpu.pas` - Z80 code generator implementation
  - `z80/z80reg.dat` - Z80 register definitions
  - `z80/hlcgcpu.pas` - High-level codegen for Z80
  - `z80/rgcpu.pas` - Register allocation for Z80
- Study FPC's Z80 code generation approach
- Extract Z80 calling conventions from `cpupara.pas`
- Study register usage patterns from `cgcpu.pas`

**Z80-Specific:**
- Use ZealZ80 ABI specification (`platforms/ZealZ80/ABI.md`)
- **FPC Z80 backend** (`freePascal/compiler/z80/`) as reference for:
  - Register allocation strategies
  - Calling conventions (from `cpupara.pas`)
  - Code generation patterns (from `cgcpu.pas`)
  - Stack frame management
- IX frame pointer convention (verify against FPC)
- IY exception frame (for future exceptions)
- Stack growth (downward)

**Tests:**
- Codegen for simple expressions
- Codegen for control flow
- Codegen for procedures/functions
- ABI compliance
- Stack frame correctness

**Deliverable:** Z80 code generator for Tier 1 Pascal.

---

### Step 4.4: Create `object-zealz80` Crate

**Why next:** Generates linkable object files.

```bash
cargo new --lib object-zealz80
```

**Dependencies:**
- `backend-zealz80`

**Implementation:**
- .ZOF object file format
- Symbol table generation
- Relocation entries
- Section management (code, data, bss)

**Tests:**
- Object file generation
- Symbol table correctness
- Relocation correctness

**Deliverable:** Object file writer for ZealZ80.

---

## Phase 5: Driver and Integration (Week 13-14)

### Step 5.1: Create `errors` and `diagnostics` Crates

**Why now:** Needed for user-facing compiler.

```bash
cargo new --lib errors
cargo new --lib diagnostics
```

**Implementation:**
- Error types and codes
- Source span formatting
- Multi-line error messages
- Suggestions ("did you mean...?")
- IDE-friendly output (JSON, LSP)

**Tests:**
- Error message formatting
- Span highlighting
- Suggestion generation

**Deliverable:** Error handling and diagnostics.

---

### Step 5.2: Create `driver` Crate

**Why last:** Orchestrates all components.

```bash
cargo new --bin driver
```

**Dependencies:** All other crates

**Implementation:**
- CLI interface (`spc` command)
- Compiler pipeline orchestration
- File I/O
- Integration with assembler/linker

**CLI Commands:**
```bash
spc build program.pas          # Full compilation
spc compile file.pas -o file   # Compile to object
spc asm file.pas               # Emit assembly
spc emit-ir file.pas           # Emit IR
spc emit-ast file.pas          # Emit AST
spc check file.pas             # Type check only
```

**Tests:**
- End-to-end compilation
- CLI argument parsing
- Error handling

**Deliverable:** Working compiler driver.

---

## Using FreePascal as Reference

### 1. Semantic Oracle

**When to consult FPC:**
- Type compatibility rules
- Expression evaluation order
- Record/array layout
- Calling conventions (for m68k reference)
- Standard library behavior

**How to use:**
1. Write a test program in Pascal
2. Compile with FPC to see what it does
3. Implement same behavior in SuperPascal
4. Verify SuperPascal matches FPC behavior

### 2. Code Organization

**Study FPC structure:**
- `freePascal/compiler/` - Compiler source
- `freePascal/rtl/` - Runtime library
- `freePascal/packages/` - Standard library

**Adapt for Rust:**
- FPC uses Pascal, we use Rust
- FPC is monolithic, we use modular crates
- FPC targets many platforms, we target specific ones

### 3. Test Suite Inspiration

**Extract from FPC:**
- `freePascal/tests/` - FPC test suite
- Adapt FPC tests for SuperPascal
- Use FPC tests as compatibility baseline

### 4. ABI Reference (for m68k)

**For FoenixA2560M:**
- Study `freePascal/compiler/cpu/m68k/`
- Extract calling conventions
- Extract register usage
- Extract stack frame layout

---

## Implementation Principles

### 1. Test-Driven Development (TDD)

**For each component:**
1. Write tests first
2. Implement minimal code to pass tests
3. Refactor
4. Repeat

**Test coverage target:** 65% minimum, 80% preferred.

### 2. Incremental Development

**Build one crate at a time:**
- Complete one crate before moving to next
- Each crate should be independently testable
- Use integration tests to verify pipeline

### 3. FPC as Semantic Reference

**When in doubt:**
- Check FPC behavior
- Match FPC semantics (unless intentionally different)
- Use FPC test cases as inspiration

### 4. Platform-First Approach

**Start with ZealZ80:**
- Primary platform
- Complete ABI specification
- Most documentation
- Simplest architecture (8-bit)

**Add other platforms later:**
- CommanderX16 (✅ **LLVM-MOS path available** - FPC `llvm/` → LLVM-MOS → 6502/65C02, complete SDK support)
- Foenix65C816 (✅ **LLVM-MOS path available** - FPC `llvm/` → LLVM-MOS → 65C816, custom platform support needed)
- FoenixA2560M (32-bit, ✅ **FPC m68k backend available**)
- RaspberryPi5 (64-bit ARM, ✅ **FPC aarch64 backend available**)

---

## First Milestone: "Hello Zeal"

### Goal
Compile and run a simple "Hello, Zeal!" program on ZealZ80.

### Program
```pascal
program HelloZeal;
uses System;

begin
  WriteLn('Hello, Zeal!');
end.
```

### Requirements
- ✅ Lexer tokenizes program
- ✅ Parser builds AST
- ✅ Semantic analyzer validates
- ✅ IR generator creates IR
- ✅ Z80 backend generates assembly
- ✅ Object writer creates .ZOF
- ✅ Program runs on Zeal emulator

### Timeline
- **Week 1-2:** Tokens + Lexer
- **Week 3-4:** Parser + AST
- **Week 5-7:** Semantics
- **Week 8-10:** IR + Codegen
- **Week 11-12:** Object writer
- **Week 13-14:** Driver + Integration

**Total:** ~14 weeks for first working compiler.

---

## Next Steps After "Hello Zeal"

### Phase 6: Expand Tier 1 Features
- Arrays and records
- More control flow
- Procedures with parameters
- Functions with return values

### Phase 7: Tier 2 Features
- Units
- Strings
- Sets
- Pointers
- More standard library

### Phase 8: Additional Platforms
- **FoenixA2560M backend** (✅ FPC `m68k/` backend available - easier)
- **RaspberryPi5 backend** (✅ FPC `aarch64/` backend available - easier)
- **CommanderX16 backend** (✅ **LLVM-MOS path available** - FPC `llvm/` → LLVM-MOS → 6502/65C02)
  - Study FPC `llvm/` backend for LLVM IR generation
  - Use LLVM-MOS SDK (`mos-6502/llvm-mos-sdk/mos-platform/cx16/`) - complete platform support!
  - See `docs/LLVM_BACKEND_PATH.md` and `docs/LLVM_MOS_SDK_REFERENCE.md`
- **Foenix65C816 backend** (✅ **LLVM-MOS path available** - FPC `llvm/` → LLVM-MOS → 65C816)
  - Study FPC `llvm/` backend for LLVM IR generation
  - Use LLVM-MOS with `-mcpu=mosw65816` for 65C816 code generation
  - **Note:** No pre-made Foenix platform support in SDK - create custom platform files (linker scripts, runtime, etc.)
  - See `docs/LLVM_BACKEND_PATH.md` for LLVM path details

---

## Tools and Resources

### Development Tools
- **Rust:** Latest stable (2024 edition)
- **Testing:** `cargo test` with coverage
- **Linting:** `clippy`
- **Formatting:** `rustfmt`
- **Documentation:** `cargo doc`

### Reference Materials
- **FPC Source:** `../freePascal/`
- **Language Specs:** `languageSpecification/`
- **Platform Specs:** `platforms/`
- **ABI Specs:** `platforms/ZealZ80/ABI.md`

### Testing Resources
- **FPC Test Suite:** `freePascal/tests/`
- **Compliance Suite:** `docs/11_ZealPascalComplianceTestSuite.md`
- **Zeal Emulator:** For testing generated code

---

## Success Criteria

### Phase 1-2 (Lexer/Parser)
- ✅ Tokenize all Tier 1 keywords
- ✅ Parse all Tier 1 grammar constructs
- ✅ Handle errors gracefully

### Phase 3 (Semantics)
- ✅ Type check all Tier 1 programs
- ✅ Detect all semantic errors
- ✅ Match FPC semantics (where applicable)

### Phase 4 (Codegen)
- ✅ Generate correct Z80 assembly
- ✅ ABI compliance
- ✅ Stack frame correctness

### Phase 5 (Integration)
- ✅ End-to-end compilation works
- ✅ Generated code runs on Zeal
- ✅ Error messages are helpful

---

## Getting Started Checklist

- [ ] Create Rust workspace structure
- [ ] Set up `tokens` crate
- [ ] Write first test (token recognition)
- [ ] Implement token definitions
- [ ] Study FPC `token.pas` for reference
- [ ] Implement keyword lookup
- [ ] Test tokenization of simple program
- [ ] Move to lexer implementation

---

**Version:** 1.0  
**Date:** 2024  
**Status:** Implementation Guide

