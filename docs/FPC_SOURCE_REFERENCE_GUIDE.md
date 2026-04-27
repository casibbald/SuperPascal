# FreePascal Source Reference Guide

## Overview

This guide maps SuperPascal compiler implementation phases to relevant FreePascal source files. Use this as a reference when implementing each component.

**Location:** `../freePascal/compiler/`

**Backend Coverage:** FPC has **complete backends** for 3 out of 5 SuperPascal targets:
- ✅ **ZealZ80:** `z80/` backend (complete)
- ✅ **FoenixA2560M:** `m68k/` backend (complete)
- ✅ **Raspberry Pi 5:** `aarch64/` backend (complete)
- ⚠️ **CommanderX16:** No direct backend (study `z80/` and `avr/` for patterns)
- ⚠️ **Foenix65C816:** No direct backend (study `i8086/` for patterns)

See `docs/FPC_BACKEND_COVERAGE.md` for complete backend mapping.

**Additional Reference:** Wirth's "Compiler Construction" (2005) is available in markdown format at:
- `docs/PDFs/compiler construction wirth 2005.md` - **Highly recommended for educational compiler design**

---

## Phase 1: Lexer and Tokens

### Key FPC Files

#### `tokens.pas`
- **Purpose:** Token definitions and keyword tables
- **What to study:**
  - Token type definitions
  - Keyword lookup tables
  - Operator definitions
  - Literal representations

**SuperPascal equivalent:** `tokens` crate

#### `scanner.pas`
- **Purpose:** Lexical analysis (tokenization)
- **What to study:**
  - Single-pass scanning algorithm
  - Comment handling (`{ }`, `(* *)`, `//`)
  - String literal parsing
  - Number parsing (decimal, hex)
  - Directive scanning (`{$...}`)
  - Error recovery

**SuperPascal equivalent:** `lexer` crate

**Key Functions:**
- `scanner()` - Main scanner loop
- `readstring()` - String literal parsing
- `readnumber()` - Number parsing
- `readcomment()` - Comment handling

---

## Phase 2: Parser and AST

### Key FPC Files

#### `parser.pas`
- **Purpose:** Recursive descent parser
- **What to study:**
  - Grammar implementation
  - Expression parsing (operator precedence)
  - Statement parsing
  - Declaration parsing
  - Error recovery strategies

**SuperPascal equivalent:** `parser` crate

**Key Functions:**
- `program_proc()` - Program parsing
- `block()` - Block parsing
- `expr()` - Expression parsing
- `statement()` - Statement parsing

#### `node.pas`
- **Purpose:** AST node definitions
- **What to study:**
  - Node type structure
  - Node allocation
  - Node traversal patterns

**SuperPascal equivalent:** `ast` crate

**Key Types:**
- `tnode` - Base node type
- `tstatementnode` - Statement nodes
- `texprnode` - Expression nodes
- `ttypenode` - Type nodes

#### Expression Parsing Files
- `pexpr.pas` - Expression parsing
- `pstatmnt.pas` - Statement parsing
- `pdecl.pas` - Declaration parsing
- `ptype.pas` - Type parsing

---

## Phase 3: Semantic Analysis

### Key FPC Files

#### `symtable.pas`
- **Purpose:** Symbol table management
- **What to study:**
  - Symbol table structure
  - Scope management
  - Symbol insertion/lookup
  - Name resolution

**SuperPascal equivalent:** `symbols` crate

**Key Functions:**
- `symtablestack_push()` - Enter scope
- `symtablestack_pop()` - Leave scope
- `searchsym()` - Symbol lookup
- `addsym()` - Symbol insertion

#### `symsym.pas`
- **Purpose:** Symbol definitions
- **What to study:**
  - Symbol type structure
  - Symbol kinds (var, const, type, proc, func)
  - Symbol properties

#### `symdef.pas`
- **Purpose:** Symbol definition types
- **What to study:**
  - Variable definitions
  - Type definitions
  - Procedure/function definitions

#### `symtype.pas`
- **Purpose:** Type symbol definitions
- **What to study:**
  - Type representation
  - Type operations

#### `types.pas` (if exists) or `ptype.pas`
- **Purpose:** Type system
- **What to study:**
  - Type representation
  - Type compatibility rules
  - Type size/alignment calculation

**SuperPascal equivalent:** `types` crate

#### `htypechk.pas`
- **Purpose:** High-level type checking
- **What to study:**
  - Type checking algorithms
  - Assignment compatibility
  - Expression type inference

**SuperPascal equivalent:** `semantics` crate

**Key Functions:**
- `compatible_types()` - Type compatibility checking
- `is_assignable()` - Assignment compatibility
- `check_assignment()` - Assignment validation

---

## Phase 4: Code Generation

### Key FPC Files

#### `cgbase.pas`
- **Purpose:** Code generation base classes
- **What to study:**
  - Code generation interface
  - IR representation (if any)
  - Code generation patterns

**SuperPascal equivalent:** `ir` crate (conceptually)

#### `cgobj.pas`
- **Purpose:** Object code generation
- **What to study:**
  - Code generation algorithms
  - Register allocation patterns
  - Stack frame management

#### Platform-Specific Codegen

**For Z80 (reference, FPC has z80 backend):**
- `z80/` directory - Z80-specific codegen
- Study register usage
- Study calling conventions

**For m68k (reference for FoenixA2560M):**
- `m68k/` directory - m68k-specific codegen
- **Key files:**
  - `m68k/cpu.pas` - CPU-specific codegen
  - `m68k/cgbase.pas` - m68k codegen base
  - `m68k/cgutils.pas` - m68k utilities
- **What to study:**
  - Calling conventions
  - Register usage (D0-D7, A0-A7)
  - Stack frame layout
  - Parameter passing

**SuperPascal equivalent:** `backend-zealz80`, `backend-foenix68k`

#### `ncgutil.pas`
- **Purpose:** Node code generation utilities
- **What to study:**
  - Code generation helpers
  - Expression codegen
  - Statement codegen

---

## Phase 5: Object Files

### Key FPC Files

#### `ogbase.pas`
- **Purpose:** Object file generation base
- **What to study:**
  - Object file structure
  - Section management
  - Symbol table generation

#### `ogelf.pas`
- **Purpose:** ELF object file generation
- **What to study:**
  - ELF format structure
  - Symbol table format
  - Relocation entries

**Note:** SuperPascal uses custom formats (.ZOF, .FOF, .AOF), but FPC's object file generation patterns are still useful.

**SuperPascal equivalent:** `object-zealz80` crate

---

## Phase 6: Runtime

### Key FPC Files

#### `rtl/` directory
- **Purpose:** Runtime library
- **What to study:**
  - Runtime helper functions
  - System calls
  - Memory management
  - String operations

**SuperPascal equivalent:** `runtime-spec` crate

**Key Files:**
- `rtl/system.pas` - System unit
- `rtl/sysutils.pas` - System utilities
- `rtl/strings.pas` - String operations

---

## Phase 7: Driver

### Key FPC Files

#### `compiler.pas`
- **Purpose:** Main compiler driver
- **What to study:**
  - Compiler pipeline orchestration
  - Phase management
  - Error handling
  - File I/O

**SuperPascal equivalent:** `driver` crate

#### `pp.pas`
- **Purpose:** Pascal compiler main program
- **What to study:**
  - Command-line interface
  - Option parsing
  - Compiler invocation

---

## Platform-Specific References

### aarch64 Backend (Raspberry Pi 5)

**FPC Files:**
- `aarch64/` directory - **COMPLETE BACKEND AVAILABLE - PRIMARY REFERENCE**
- `aarch64/cpubase.pas` - ARM64 registers, opcodes
- `aarch64/cpupara.pas` - ARM64 calling conventions (AAPCS64)
- `aarch64/cgcpu.pas` - ARM64 code generator
- `aarch64/hlcgcpu.pas` - High-level codegen

**What to extract:**
- AAPCS64 calling conventions (from `cpupara.pas`)
- ARM64 register usage
- Stack frame layout
- Parameter passing rules
- Return value handling

**SuperPascal equivalent:** `backend-raspberrypi5` crate

**Note:** FPC has a **complete, working ARM64 backend**. Use this as the **primary reference** for Raspberry Pi 5.

---

### Z80 Backend (ZealZ80)

**FPC Files:**
- `z80/` directory - **COMPLETE Z80 BACKEND AVAILABLE**
- **Key Files:**
  - `z80/cpubase.pas` - Z80 registers, opcodes, base types, conditions
  - `z80/cpupara.pas` - Z80 calling conventions and parameter passing
  - `z80/cgcpu.pas` - Z80 code generator (2489 lines - comprehensive!)
  - `z80/hlcgcpu.pas` - High-level codegen for Z80
  - `z80/rgcpu.pas` - Register allocation for Z80
  - `z80/z80reg.dat` - Register definitions
  - `z80/z80ins.dat` - Instruction definitions
- **What to Extract:**
  - Calling conventions (from `cpupara.pas`)
  - Register usage patterns (from `cgcpu.pas`)
  - Stack frame layout
  - Parameter passing rules
  - Return value handling

**Note:** FPC has a **complete, working Z80 backend**. Use this as the **primary reference** alongside ZealZ80 ABI specification.

### m68k Backend (FoenixA2560M)

**FPC Files:**
- `m68k/` directory - **COMPLETE BACKEND AVAILABLE - PRIMARY REFERENCE**
- `m68k/cpu.pas` - CPU-specific codegen
- `m68k/cgbase.pas` - m68k codegen base
- `m68k/cgutils.pas` - m68k utilities
- `m68k/cpupara.pas` - m68k calling conventions
- `m68k/cpubase.pas` - m68k registers and opcodes

**What to extract:**
- Calling conventions (from `cpupara.pas`)
- Register usage (D0-D7, A0-A7)
- Stack frame layout
- Parameter passing rules
- Return value handling

**SuperPascal equivalent:** `backend-foenix68k` crate

**Note:** FPC has a **complete, working m68k backend**. Use this as the **primary reference** for FoenixA2560M.

---

## How to Use FPC Source

### 1. Read, Don't Copy

**Do:**
- Study FPC's algorithms and data structures
- Understand FPC's design decisions
- Adapt FPC patterns to Rust idioms

**Don't:**
- Copy FPC code directly (it's Pascal, we need Rust)
- Blindly follow FPC (we have different goals)
- Assume FPC is perfect (it has legacy code)

### 2. Focus on Semantics, Not Implementation

**Study:**
- How FPC handles type compatibility
- How FPC evaluates expressions
- How FPC manages scopes
- How FPC generates code

**Adapt:**
- Use Rust idioms (enums, traits, ownership)
- Use modern patterns (Result types, error handling)
- Use our crate structure (modular, testable)

### 3. Test Against FPC

**Strategy:**
1. Write test program in Pascal
2. Compile with FPC
3. Observe FPC's behavior
4. Implement same behavior in SuperPascal
5. Verify SuperPascal matches FPC

**Example:**
```pascal
// Test program
var x: integer;
begin
  x := 10 + 20;
end.
```

1. Compile with FPC: `fpc test.pas`
2. Study FPC's generated code
3. Implement SuperPascal to match FPC's semantics
4. Verify SuperPascal generates equivalent code

---

## FPC File Mapping by SuperPascal Crate

| SuperPascal Crate | Primary FPC Files | Secondary FPC Files |
|-------------------|-------------------|---------------------|
| `tokens` | `tokens.pas` | `scanner.pas` (token definitions) |
| `lexer` | `scanner.pas` | `ccharset.pas`, `cclasses.pas` |
| `ast` | `node.pas` | `pexpr.pas`, `pstatmnt.pas`, `pdecl.pas` |
| `parser` | `parser.pas` | `pexpr.pas`, `pstatmnt.pas`, `pdecl.pas`, `ptype.pas` |
| `types` | `ptype.pas`, `symtype.pas` | `defcmp.pas`, `defutil.pas` |
| `symbols` | `symtable.pas`, `symsym.pas` | `symdef.pas`, `symutil.pas` |
| `semantics` | `htypechk.pas` | `pexpr.pas` (type checking), `pstatmnt.pas` |
| `ir` | `cgbase.pas`, `ncgutil.pas` | `cgobj.pas` (concepts) |
| `backend-zealz80` | `z80/` (**PRIMARY REFERENCE**) | ZealZ80 ABI spec, `z80/cpubase.pas`, `z80/cpupara.pas`, `z80/cgcpu.pas` |
| `backend-foenix68k` | `m68k/` (**PRIMARY REFERENCE**) | `m68k/cpu.pas`, `m68k/cgbase.pas`, `m68k/cpupara.pas` |
| `backend-raspberrypi5` | `aarch64/` (**PRIMARY REFERENCE**) | `aarch64/cpupara.pas`, `aarch64/cgcpu.pas`, `aarch64/cpubase.pas` |
| `backend-commanderx16` | `z80/` (patterns), `avr/` (patterns) | 6502/65C02 docs (no direct FPC backend) |
| `backend-foenix816` | `i8086/` (patterns) | 65C816 docs (no direct FPC backend) |
| `object-zealz80` | `ogbase.pas` | Custom .ZOF format |
| `runtime-spec` | `rtl/system.pas` | `rtl/sysutils.pas` |

---

## Quick Reference: FPC Directory Structure

```
freePascal/compiler/
├── tokens.pas              # Token definitions
├── scanner.pas             # Lexical analysis
├── parser.pas              # Syntax analysis
├── node.pas                # AST node definitions
├── pexpr.pas               # Expression parsing
├── pstatmnt.pas            # Statement parsing
├── pdecl.pas               # Declaration parsing
├── ptype.pas               # Type parsing
├── symtable.pas            # Symbol table
├── symsym.pas              # Symbol definitions
├── symdef.pas              # Symbol definition types
├── symtype.pas             # Type symbols
├── htypechk.pas            # Type checking
├── cgbase.pas              # Code generation base
├── cgobj.pas               # Object code generation
├── ncgutil.pas             # Codegen utilities
├── ogbase.pas              # Object file base
├── ogelf.pas               # ELF object files
├── compiler.pas            # Compiler driver
├── pp.pas                  # Main program
├── m68k/                   # m68k backend (for A2560M)
│   ├── cpu.pas
│   ├── cgbase.pas
│   └── cgutils.pas
└── z80/                    # Z80 backend (if available)
```

---

## Implementation Workflow

### Step 1: Study FPC File
1. Read relevant FPC source file
2. Understand the algorithm/data structure
3. Note key design decisions
4. Identify what to adapt vs. what to do differently

### Step 2: Design Rust Equivalent
1. Design Rust data structures (enums, structs, traits)
2. Plan API (public functions, error handling)
3. Consider Rust idioms (ownership, borrowing, Result types)

### Step 3: Write Tests First (TDD)
1. Write tests for expected behavior
2. Use FPC behavior as oracle
3. Ensure tests cover edge cases

### Step 4: Implement
1. Implement minimal code to pass tests
2. Use FPC as semantic reference
3. Adapt to Rust idioms

### Step 5: Validate
1. Compare behavior with FPC
2. Run compliance tests
3. Verify correctness

---

## Example: Implementing Token Recognition

### FPC Reference (`tokens.pas`, `scanner.pas`)

**Study:**
- How FPC defines token types
- How FPC implements keyword lookup
- How FPC handles token scanning

### SuperPascal Implementation

```rust
// tokens/src/lib.rs
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Program, Var, Const, Type, Procedure, Function,
    Begin, End, If, Then, Else, While, For, Repeat, Until,
    // ... (from FPC tokens.pas as reference)
}

pub struct KeywordTable;
impl KeywordTable {
    pub fn lookup(ident: &str) -> Option<TokenKind> {
        // Implement keyword lookup (study FPC's approach)
    }
}
```

**Test:**
```rust
#[test]
fn test_keyword_lookup() {
    assert_eq!(KeywordTable::lookup("program"), Some(TokenKind::Program));
    assert_eq!(KeywordTable::lookup("var"), Some(TokenKind::Var));
    // ... (verify against FPC behavior)
}
```

---

## Key Insights from FPC

### 1. Parser Design
- **FPC uses:** Recursive descent (hand-written)
- **SuperPascal:** Same approach (recursive descent)
- **Why:** Full control, good error messages, no generator dependency

### 2. Type System
- **FPC uses:** Nominal typing with structural compatibility
- **SuperPascal:** Same approach
- **Why:** Matches Pascal semantics, FPC compatibility

### 3. Code Generation
- **FPC uses:** Direct code generation (no IR for most backends)
- **SuperPascal:** IR-based (for multi-platform support)
- **Why:** We need platform-agnostic IR for multiple targets

### 4. Symbol Table
- **FPC uses:** Stack-based scopes
- **SuperPascal:** Same approach
- **Why:** Standard, efficient, matches Pascal scoping rules

---

## Next Steps

1. **Start with `tokens`:**
   - Study `freePascal/compiler/tokens.pas`
   - Implement token definitions
   - Write tests

2. **Move to `lexer`:**
   - Study `freePascal/compiler/scanner.pas`
   - Implement scanner
   - Test tokenization

3. **Continue incrementally:**
   - One crate at a time
   - Test each crate thoroughly
   - Use FPC as semantic oracle

---

**Version:** 1.0  
**Date:** 2024  
**Status:** Reference Guide

