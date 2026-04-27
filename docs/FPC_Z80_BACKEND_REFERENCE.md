# FreePascal Z80 Backend Reference Guide

## Overview

FreePascal has a **complete, working Z80 backend** that serves as an excellent reference for implementing SuperPascal's `backend-zealz80` crate.

**Location:** `../freePascal/compiler/z80/`

---

## Key Files

### 1. `cpubase.pas` - Base Types and Definitions

**Purpose:** Z80-specific base types, registers, opcodes, conditions

**Key Contents:**
- `TAsmOp` - Z80 instruction opcodes
- Register definitions (A, B, C, D, E, H, L, IX, IY, SP, etc.)
- Register pairs (BC, DE, HL, AF)
- Condition codes (NZ, Z, NC, C, PO, PE, P, M)
- Flags (F_NE, F_E, F_NC, F_C, etc.)
- Volatile registers: `VOLATILE_INTREGISTERS = [RS_A,RS_B,RS_C,RS_D,RS_E,RS_H,RS_L]`

**Use For:**
- Understanding Z80 register set
- Instruction opcodes
- Condition codes for branches
- Register allocation constraints

---

### 2. `cpupara.pas` - Calling Conventions

**Purpose:** Z80 calling conventions and parameter passing

**Key Contents:**
- `tcpuparamanager` class - Parameter management
- `get_volatile_registers_int()` - Volatile registers
- `create_paraloc_info()` - Parameter location creation
- `get_funcretloc()` - Function return value location
- Parameter passing rules
- Stack frame conventions

**Use For:**
- **Defining ZealZ80 ABI** (calling conventions)
- Parameter passing rules
- Return value handling
- Stack frame layout
- Register usage in calls

**Critical Functions:**
```pascal
function create_paraloc_info(p : tabstractprocdef; side: tcallercallee):longint;
function get_funcretloc(p : tabstractprocdef; side: tcallercallee; forcetempdef: tdef): tcgpara;
```

---

### 3. `cgcpu.pas` - Code Generator

**Purpose:** Z80 code generation implementation (2489 lines - comprehensive!)

**Key Contents:**
- `tcgz80` class - Main code generator
- Load operations (`a_load_const_reg`, `a_load_reg_reg`, etc.)
- Arithmetic operations (`a_op_reg_reg`, `a_op_const_reg`)
- Comparison operations (`a_cmp_const_reg_label`, `a_cmp_reg_reg_label`)
- Control flow (`a_jmp_name`, `a_jmp_always`, `a_jmp_flags`)
- Procedure entry/exit (`g_proc_entry`, `g_proc_exit`)
- Procedure calls (`a_call_name`, `a_call_reg`)
- Stack management (`g_stackpointer_alloc`)

**Use For:**
- **Code generation patterns** for Z80
- Register allocation strategies
- Instruction selection
- Stack frame management
- Procedure call/return sequences

**Key Methods:**
```pascal
procedure a_load_const_reg(list : TAsmList; size: tcgsize; a : tcgint; reg : tregister);
procedure a_load_reg_reg(list : TAsmList; fromsize, tosize: tcgsize; reg1,reg2: tregister);
procedure a_op_reg_reg(list: TAsmList; Op: TOpCG; size: TCGSize; src, dst: TRegister);
procedure g_proc_entry(list : TAsmList; localsize : longint; nostackframe:boolean);
procedure g_proc_exit(list : TAsmList; parasize : longint; nostackframe:boolean);
procedure a_call_name(list : TAsmList; const s : string; weak: boolean);
```

---

### 4. `hlcgcpu.pas` - High-Level Code Generator

**Purpose:** High-level code generation for Z80

**Use For:**
- High-level constructs
- Type conversions
- Complex operations

---

### 5. `rgcpu.pas` - Register Allocation

**Purpose:** Register allocation for Z80

**Use For:**
- Register allocation algorithms
- Register spilling
- Register preferences

---

### 6. `z80reg.dat` - Register Definitions

**Purpose:** Z80 register definitions in data format

**Contents:**
- Main registers: B, C, D, E, H, L, A
- Register pairs: BC, DE, HL, AF
- Index registers: IX, IY
- Stack pointer: SP
- Flags: F, CARRYFLAG, ZEROFLAG, etc.

**Use For:**
- Register enumeration
- Register naming
- Register properties

---

### 7. `z80ins.dat` - Instruction Definitions

**Purpose:** Z80 instruction definitions

**Use For:**
- Instruction encoding
- Operand types
- Instruction properties

---

## Calling Conventions (from `cpupara.pas`)

### Volatile Registers

From `cpubase.pas`:
```pascal
VOLATILE_INTREGISTERS = [RS_A,RS_B,RS_C,RS_D,RS_E,RS_H,RS_L];
```

**Meaning:** These registers can be modified by procedure calls and must be saved by the caller if needed.

### Parameter Passing

Study `create_paraloc_info()` in `cpupara.pas` to understand:
- How parameters are passed (registers vs. stack)
- Parameter location assignment
- Stack frame layout

### Return Values

Study `get_funcretloc()` in `cpupara.pas` to understand:
- Where return values are placed
- Register usage for returns
- Stack usage for large returns

---

## Code Generation Patterns (from `cgcpu.pas`)

### 1. Load Constant to Register

```pascal
procedure a_load_const_reg(list : TAsmList; size: tcgsize; a : tcgint; reg : tregister);
```

**Use For:** Loading immediate values into registers

### 2. Move Between Registers

```pascal
procedure a_load_reg_reg(list : TAsmList; fromsize, tosize: tcgsize; reg1,reg2: tregister);
```

**Use For:** Register-to-register moves

### 3. Arithmetic Operations

```pascal
procedure a_op_reg_reg(list: TAsmList; Op: TOpCG; size: TCGSize; src, dst: TRegister);
procedure a_op_const_reg(list : TAsmList; Op: TOpCG; size: TCGSize; a: tcgint; reg: TRegister);
```

**Use For:** ADD, SUB, MUL, DIV, etc.

### 4. Comparisons

```pascal
procedure a_cmp_const_reg_label(list : TAsmList; size: tcgsize; cmp_op: topcmp; a: tcgint; reg: tregister; l: tasmlabel);
procedure a_cmp_reg_reg_label(list : TAsmList; size: tcgsize; cmp_op: topcmp; reg1,reg2: tregister; l: tasmlabel);
```

**Use For:** Conditional branches

### 5. Procedure Entry/Exit

```pascal
procedure g_proc_entry(list : TAsmList; localsize : longint; nostackframe:boolean);
procedure g_proc_exit(list : TAsmList; parasize : longint; nostackframe:boolean);
```

**Use For:** Stack frame setup/teardown

### 6. Procedure Calls

```pascal
procedure a_call_name(list : TAsmList; const s : string; weak: boolean);
procedure a_call_reg(list : TAsmList; reg: tregister);
```

**Use For:** Procedure/function calls

---

## Register Usage Patterns

### From `cpubase.pas`:

**8-bit Registers:**
- A, B, C, D, E, H, L

**16-bit Register Pairs:**
- BC, DE, HL, AF

**Index Registers:**
- IX, IY

**Stack Pointer:**
- SP

**Volatile (caller-save):**
- A, B, C, D, E, H, L

**Non-volatile (callee-save):**
- IX, IY (typically)

---

## Stack Frame Management

Study `g_proc_entry()` and `g_proc_exit()` in `cgcpu.pas` to understand:
- Stack frame setup
- Local variable allocation
- Parameter access
- Frame pointer usage (IX?)

---

## How to Use FPC Z80 Backend

### 1. Study Calling Conventions

**Read:** `cpupara.pas`
- Understand parameter passing
- Understand return value handling
- Extract ABI rules

**Apply to:** ZealZ80 ABI specification

### 2. Study Code Generation

**Read:** `cgcpu.pas`
- Study code generation patterns
- Understand instruction selection
- Learn register allocation hints

**Apply to:** `backend-zealz80` crate implementation

### 3. Study Register Usage

**Read:** `cpubase.pas`, `z80reg.dat`
- Understand register set
- Learn volatile/non-volatile rules
- Understand register pairs

**Apply to:** Register allocation in `backend-zealz80`

### 4. Adapt to Rust

**Do:**
- Study FPC's algorithms
- Understand FPC's patterns
- Adapt to Rust idioms

**Don't:**
- Copy Pascal code directly
- Assume FPC is perfect
- Ignore ZealZ80-specific requirements

---

## Integration with SuperPascal

### For `backend-zealz80` Crate

1. **Study FPC Z80 backend:**
   - Read `cpupara.pas` for calling conventions
   - Read `cgcpu.pas` for codegen patterns
   - Read `cpubase.pas` for register definitions

2. **Extract ABI Rules:**
   - Parameter passing (from `cpupara.pas`)
   - Return values (from `cpupara.pas`)
   - Stack frames (from `cgcpu.pas`)
   - Register usage (from `cpubase.pas`)

3. **Compare with ZealZ80 ABI:**
   - Verify against `platforms/ZealZ80/ABI.md`
   - Note any differences
   - Document intentional deviations

4. **Implement in Rust:**
   - Adapt FPC patterns to Rust
   - Use modern Rust idioms
   - Maintain SuperPascal ABI compatibility

---

## Key Insights

### 1. FPC Has Complete Z80 Backend

- **2489 lines** of codegen in `cgcpu.pas`
- Complete calling convention implementation
- Register allocation
- Stack frame management

### 2. Use as Primary Reference

- **Primary reference** for Z80 codegen
- Study patterns, not copy code
- Adapt to Rust and SuperPascal needs

### 3. Complement with ZealZ80 ABI

- FPC provides general Z80 patterns
- ZealZ80 ABI provides platform-specific rules
- Combine both for complete picture

---

## Quick Reference

| Need to understand... | Read file... |
|----------------------|--------------|
| Calling conventions | `cpupara.pas` |
| Code generation | `cgcpu.pas` |
| Registers | `cpubase.pas`, `z80reg.dat` |
| Instructions | `z80ins.dat`, `cpubase.pas` |
| Register allocation | `rgcpu.pas` |
| High-level codegen | `hlcgcpu.pas` |

---

## Example: Extracting Calling Convention

From `cpupara.pas`, study `create_paraloc_info()`:

1. **Parameter Location Assignment:**
   - First parameters in registers?
   - Remaining on stack?
   - Stack growth direction?

2. **Return Value Location:**
   - From `get_funcretloc()`:
   - 8-bit in A?
   - 16-bit in HL?
   - Larger on stack?

3. **Volatile Registers:**
   - From `get_volatile_registers_int()`:
   - Which registers can be clobbered?

**Apply to:** ZealZ80 ABI specification

---

## Summary

**FPC Z80 Backend is:**
- ✅ Complete and working
- ✅ Comprehensive (2489 lines of codegen)
- ✅ Well-structured
- ✅ Excellent reference

**Use it for:**
- Calling convention reference
- Code generation patterns
- Register usage
- Stack frame management

**Combine with:**
- ZealZ80 ABI specification
- Wirth's compiler construction book
- SuperPascal language spec

---

**Version:** 1.0  
**Date:** 2024  
**Status:** FPC Z80 Backend Reference Guide

