# LLVM-MOS Backend Path for 6502/65C02/65C816 Targets

## Overview

FreePascal has an **LLVM backend** that can be used as an intermediate step to generate code for 6502/65C02 targets (CommanderX16) and 65C816 targets (Foenix65C816) via **LLVM-MOS**, a production-ready LLVM backend for 6502/65C02/65C816 processors.

**LLVM-MOS:** https://llvm-mos.org/

**Two-Step Path:**
1. **SuperPascal → LLVM IR:** Generate LLVM IR using FPC LLVM backend patterns
2. **LLVM IR → 6502/65C02/65C816:** Use **LLVM-MOS** to generate assembly
   - **CommanderX16:** `-mcpu=mosw65c02` (65C02)
   - **Foenix65C816:** `-mcpu=mosw65816` (65C816)

---

## FPC LLVM Backend

### Location
`../freePascal/compiler/llvm/`

## LLVM-MOS SDK (Local)

### Location
`../mos-6502/`

### Components

#### `llvm-mos/` - LLVM-MOS Compiler
- **Location:** `mos-6502/llvm-mos/`
- **MOS Backend:** `llvm-mos/llvm/lib/Target/MOS/`
- **Purpose:** Complete LLVM-MOS compiler with 6502/65C02 backend
- **Use For:** Understanding LLVM-MOS backend implementation

#### `llvm-mos-sdk/` - Platform SDK
- **Location:** `mos-6502/llvm-mos-sdk/`
- **CommanderX16 Support:** `llvm-mos-sdk/mos-platform/cx16/` ✅
- **Purpose:** Platform-specific libraries, linkers, and examples
- **Use For:** 
  - CommanderX16 platform configuration
  - Linker scripts
  - Runtime libraries
  - Examples

**Key Directories:**
- `mos-platform/cx16/` - **CommanderX16 platform support** (80 files!)
- `examples/cx16/` - CommanderX16 example programs
- `common/` - Common platform libraries

### Key Files

#### `hlcgllvm.pas` - High-Level Code Generator for LLVM
- **Purpose:** Generate LLVM IR from high-level constructs
- **Use For:** Understanding how to generate LLVM IR from SuperPascal AST/IR
- **Key Concepts:**
  - LLVM IR generation
  - Type conversions
  - Function calls
  - Memory operations

#### Other LLVM Backend Files
- LLVM-specific code generation
- LLVM IR representation
- LLVM target selection

---

## LLVM 6502 Backend

### Status
**LLVM-MOS** provides a **complete, production-ready** LLVM backend for 6502/MOS 6502 processors (including 65C02).

**Website:** https://llvm-mos.org/

**Key Features:**
- ✅ **Production-ready** LLVM backend for 6502/65C02/65C816
- ✅ Supports 6502, 65C02, 65C816, and related processors
- ✅ Well-maintained and actively developed
- ✅ Comprehensive 6502/65C02/65C816 instruction support
- ✅ Optimizations and code generation
- ✅ **65C816 Support:** `mosw65816` device/family (WDC 65816/65C816)

### Benefits
- **Mature code generation:** LLVM-MOS handles register allocation, instruction selection, optimization
- **Well-tested:** LLVM-MOS is production-ready and extensively tested
- **Optimization:** LLVM's optimization passes apply automatically
- **Maintenance:** LLVM-MOS team actively maintains the backend
- **6502/65C02 Support:** Complete support for 6502 and 65C02 (perfect for CommanderX16)
- **65C816 Support:** Complete support for 65C816 via `mosw65816` device (perfect for Foenix65C816)
- **Active Development:** Regularly updated with improvements

### Considerations
- **LLVM dependency:** Requires LLVM-MOS as a dependency
- **IR overhead:** Additional compilation step
- **Binary size:** LLVM toolchain is large
- **Note:** LLVM-MOS is production-ready, not experimental

---

## Implementation Strategy

### Option A: Two-Step via LLVM (Recommended for CommanderX16)

#### Step 1: SuperPascal → LLVM IR

**Implementation:**
- Study FPC `llvm/` backend
- Create `backend-llvm` crate (or `backend-commanderx16-llvm`)
- Generate LLVM IR from SuperPascal IR
- Use FPC patterns for LLVM IR generation

**FPC Reference:**
- `llvm/hlcgllvm.pas` - LLVM IR generation patterns
- Study how FPC generates LLVM IR
- Adapt to SuperPascal IR structure

**Deliverable:** LLVM IR generation from SuperPascal programs

#### Step 2: LLVM IR → 6502 Assembly

**Implementation:**
- Use LLVM's 6502 backend
- Invoke `llc` (LLVM compiler) with 6502 target
- Generate 6502 assembly
- Assemble to object files

**Process:**
```bash
# Generate LLVM IR from SuperPascal
superpascal-compiler --emit-llvm program.pas -o program.ll

# Compile LLVM IR to 6502 assembly using LLVM-MOS
llc -march=mos6502 program.ll -o program.s

# Assemble to object file (using CommanderX16 assembler)
x16-assembler program.s -o program.o
```

**Note:** LLVM-MOS uses `mos6502` as the target triple. Check LLVM-MOS documentation for exact target specification.

**Deliverable:** 6502 assembly/code from LLVM IR

---

### Option B: Direct Backend (Alternative)

**Implementation:**
- Study `z80/` and `avr/` for 8-bit patterns
- Implement direct `backend-commanderx16` crate
- Generate 6502 assembly directly from SuperPascal IR

**Pros:**
- No LLVM dependency
- Single compilation step
- Full control over code generation

**Cons:**
- More implementation work
- Must implement register allocation, instruction selection
- Must implement optimizations

---

## Comparison: LLVM Path vs. Direct Backend

| Aspect | LLVM Path | Direct Backend |
|--------|-----------|----------------|
| **Implementation Effort** | Medium (LLVM IR gen) | High (full backend) |
| **Code Quality** | High (LLVM optimizations) | Depends on implementation |
| **Dependencies** | LLVM required | None |
| **Compilation Steps** | 2 steps | 1 step |
| **Maintenance** | LLVM team maintains 6502 backend | We maintain backend |
| **Optimization** | LLVM passes | Custom optimizations |
| **Binary Size** | Large (LLVM) | Small |

---

## Recommended Approach

### For CommanderX16 (65C02)

**Recommended:** **Option A (LLVM Path)**

**Rationale:**
1. **Faster to implement:** Only need LLVM IR generation
2. **Better code quality:** LLVM's optimizations and codegen
3. **Less maintenance:** LLVM team maintains 6502 backend
4. **Proven approach:** LLVM is widely used for code generation

**Implementation Steps:**
1. Study FPC `llvm/` backend
2. Implement `backend-llvm` crate (or integrate into `backend-commanderx16`)
3. Generate LLVM IR from SuperPascal IR
4. Use LLVM's 6502 backend for final code generation
5. Integrate LLVM toolchain into build process

### For Foenix65C816 (65C816)

**Recommended:** **Option A (LLVM Path)** ✅

**Rationale:**
1. **LLVM-MOS supports 65C816:** `-mcpu=mosw65816` device/family available
2. **Faster to implement:** Only need LLVM IR generation
3. **Better code quality:** LLVM's optimizations and codegen
4. **Less maintenance:** LLVM team maintains 65C816 backend
5. **24-bit addressing handled:** LLVM-MOS supports 65C816's 24-bit addressing model

**Implementation Steps:**
1. Study FPC `llvm/` backend
2. Implement `backend-llvm` crate (or integrate into `backend-foenix65c816`)
3. Generate LLVM IR from SuperPascal IR
4. Use LLVM-MOS with `-mcpu=mosw65816` for 65C816 code generation
5. **Create custom platform support:** Linker scripts, runtime, etc. (no pre-made Foenix support in SDK)
6. **Reference:** Study `llvm-mos-sdk/mos-platform/cx16/` as template for platform files

**Note:** While LLVM-MOS supports the 65C816 CPU, there's no pre-made Foenix platform support in the SDK. You'll need to create custom platform files (linker scripts, runtime initialization, etc.) similar to how CommanderX16 has platform support.

---

## FPC LLVM Backend Study Guide

### Key Files to Study

1. **`llvm/hlcgllvm.pas`**
   - LLVM IR generation
   - High-level construct translation
   - Type handling

2. **LLVM IR Generation Patterns**
   - How FPC generates LLVM IR
   - Function definitions
   - Variable declarations
   - Control flow

### What to Extract

- **LLVM IR generation patterns**
- **Type conversion to LLVM types**
- **Function call conventions**
- **Memory operations**
- **Control flow translation**

---

## LLVM-MOS Backend

### LLVM-MOS Project

**Website:** https://llvm-mos.org/

**Local SDK:** `../mos-6502/`

**Status:** ✅ **Production-ready** LLVM backend for 6502/65C02/65C816

**Key Information:**
- **Target:** `mos6502` (6502, 65C02, 65C816, and related processors)
- **Status:** Stable, actively maintained
- **Support:** Complete 6502/65C02/65C816 instruction set
- **Optimization:** Full LLVM optimization passes
- **Documentation:** Comprehensive documentation available
- **CommanderX16 Support:** ✅ **Complete platform support in SDK** (`llvm-mos-sdk/mos-platform/cx16/`)
- **Foenix65C816 Support:** ✅ **CPU support available** (`-mcpu=mosw65816`), **custom platform support needed**

### LLVM-MOS Features

- ✅ Complete 6502/65C02/65C816 instruction support
- ✅ Register allocation
- ✅ Code optimization
- ✅ Calling conventions
- ✅ Stack frame management
- ✅ Memory model support
- ✅ **65C816 Support:** `mosw65816` device/family (WDC 65816/65C816)

### Integration

**Local SDK Available:**
- **Location:** `../mos-6502/`
- **Compiler:** `llvm-mos/` - Build or use pre-built binaries
- **SDK:** `llvm-mos-sdk/` - Platform libraries and tools
- **CommanderX16:** `llvm-mos-sdk/mos-platform/cx16/` - Complete platform support

**Installation:**
- Use local SDK at `mos-6502/llvm-mos/`
- Build LLVM-MOS compiler if needed
- Ensure `llc` (LLVM compiler) supports `mos6502` target
- Verify with: `llc --version` (should list mos6502)

**Target Specification:**
- Use `mos6502` as target triple
- **CommanderX16 platform:** Use `cx16` platform configuration (`-mcpu=mosw65c02`)
  - Check `llvm-mos-sdk/mos-platform/cx16/clang.cfg` for platform settings
- **Foenix65C816 platform:** Use `-mcpu=mosw65816` for 65C816 code generation
  - **Note:** No pre-made Foenix platform support in SDK - create custom platform files
  - **Reference:** Study `llvm-mos-sdk/mos-platform/cx16/` as template for platform support

---

## Integration with SuperPascal

### Compiler Pipeline

```
SuperPascal Source
    ↓
[Lexer] → [Parser] → [Semantics] → [IR]
    ↓
[LLVM IR Generator] ← (Study FPC llvm/ backend)
    ↓
LLVM IR (.ll file)
    ↓
[LLVM 6502 Backend] ← (Use LLVM toolchain)
    ↓
6502 Assembly (.s file)
    ↓
[Assembler] ← (6502 assembler)
    ↓
Object File (.o file)
```

### Crate Structure

**Option 1: Separate LLVM Backend Crate**
```
backend-llvm/          # LLVM IR generation
backend-commanderx16/  # Wrapper that uses LLVM backend
```

**Option 2: Integrated LLVM Backend**
```
backend-commanderx16/  # Contains LLVM IR generation + LLVM invocation
```

---

## Advantages of LLVM Path

### 1. Faster Implementation
- Only need to implement LLVM IR generation
- LLVM handles code generation, optimization, register allocation

### 2. Better Code Quality
- LLVM's optimization passes
- Mature register allocation
- Instruction selection

### 3. Less Maintenance
- LLVM team maintains 6502 backend
- Bug fixes and improvements come from LLVM

### 4. Future-Proof
- If LLVM adds more 6502 features, we benefit automatically
- Can target other architectures via LLVM

---

## Disadvantages of LLVM Path

### 1. LLVM Dependency
- Requires LLVM to be installed
- Large binary size
- Additional dependency

### 2. Two-Step Compilation
- More complex build process
- Additional compilation step
- Slower compilation (though LLVM is fast)

### 3. Less Control
- Can't customize code generation as easily
- Must work within LLVM's constraints

---

## Decision Matrix

### Use LLVM Path If:
- ✅ **LLVM-MOS is production-ready** (✅ Confirmed!)
- ✅ Code quality is priority
- ✅ Faster implementation is desired
- ✅ LLVM dependency is acceptable
- ✅ **CommanderX16 (65C02) is fully supported** (✅ Confirmed!)

### Use Direct Backend If:
- ✅ Full control is needed
- ✅ No external dependencies desired
- ✅ Single-step compilation required
- ✅ Custom optimizations needed

---

## Implementation Checklist

### Phase 1: Research
- [x] ✅ **LLVM-MOS is production-ready** (https://llvm-mos.org/)
- [ ] Study FPC `llvm/` backend
- [ ] Understand LLVM IR format
- [ ] Install and test LLVM-MOS with sample code
- [ ] Review LLVM-MOS documentation and examples

### Phase 2: LLVM IR Generation
- [ ] Create `backend-llvm` crate (or integrate)
- [ ] Implement LLVM IR generation from SuperPascal IR
- [ ] Test LLVM IR generation
- [ ] Verify LLVM IR correctness

### Phase 3: Integration
- [ ] Integrate LLVM toolchain invocation
- [ ] Test end-to-end compilation
- [ ] Verify generated 6502 code
- [ ] Optimize LLVM IR generation

### Phase 4: Polish
- [ ] Error handling
- [ ] Documentation
- [ ] Performance optimization
- [ ] Testing

---

## Example: LLVM IR Generation

### SuperPascal Code
```pascal
program Hello;
begin
  WriteLn('Hello, World!');
end.
```

### LLVM IR (Generated)
```llvm
; ModuleID = 'hello'
source_filename = "hello.pas"
target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
target triple = "6502-unknown-unknown"

define void @main() {
entry:
  call void @WriteLn(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.str, i32 0, i32 0))
  ret void
}

@.str = private unnamed_addr constant [13 x i8] c"Hello, World!\00"
declare void @WriteLn(i8*)
```

### 6502 Assembly (Generated by LLVM)
```asm
; 6502 assembly generated by LLVM
main:
  ; ... 6502 code ...
```

---

## References

### FPC LLVM Backend
- `freePascal/compiler/llvm/` - FPC LLVM backend source
- Study `hlcgllvm.pas` for LLVM IR generation patterns

### LLVM-MOS SDK (Local) - CommanderX16 Support
- **Location:** `../mos-6502/`
- **CommanderX16 Platform:** `llvm-mos-sdk/mos-platform/cx16/` ✅ **80+ files!**
- **Key Files:**
  - `clang.cfg` - Compiler configuration (`-mcpu=mosw65c02`, `-D__CX16__`)
  - `link.ld` - Linker script (memory layout, entry point)
  - `imag-regs.ld` - Zero-page register allocation
  - `cx16_k_*.s` - **All KERNAL function wrappers** (console, graphics, input, etc.)
  - `cx16.h` - Platform header (constants, types, API)
  - `examples/cx16/` - Example programs
- **See:** `docs/LLVM_MOS_SDK_REFERENCE.md` for complete SDK reference

### LLVM-MOS Documentation
- **LLVM-MOS Website:** https://llvm-mos.org/
- **Local SDK:** `../mos-6502/`
- **Local SDK README:** `mos-6502/llvm-mos-sdk/README.md`
- **CommanderX16 Platform:** `mos-6502/llvm-mos-sdk/mos-platform/cx16/` ✅ **Complete support!**
- **See Also:** `docs/LLVM_MOS_SDK_REFERENCE.md` - Complete SDK reference guide
- LLVM-MOS Documentation and Examples
- LLVM Language Reference Manual
- LLVM-MOS GitHub Repository (for source code and issues)

### LLVM Tools
- `llc` - LLVM compiler (IR → assembly)
- `llvm-as` - LLVM assembler
- `llvm-dis` - LLVM disassembler

---

## Summary

**LLVM Path for CommanderX16:**
- ✅ **Faster implementation** (only LLVM IR generation needed)
- ✅ **Better code quality** (LLVM optimizations)
- ✅ **Less maintenance** (LLVM team maintains backend)
- ⚠️ **LLVM dependency** (must be acceptable)
- ⚠️ **Two-step compilation** (more complex build)

**Recommendation:** ✅ **Use LLVM path for CommanderX16** - LLVM-MOS is production-ready and fully supports 65C02!

**Bonus:** LLVM-MOS SDK has **complete CommanderX16 platform support** at `mos-6502/llvm-mos-sdk/mos-platform/cx16/` with 80+ files including KERNAL wrappers, linker scripts, and examples!

---

**Version:** 1.0  
**Date:** 2024  
**Status:** LLVM Backend Path Analysis

