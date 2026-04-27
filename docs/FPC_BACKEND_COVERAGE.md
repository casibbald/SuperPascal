# FreePascal Backend Coverage for SuperPascal Targets

## Overview

FreePascal has **extensive backend coverage** for SuperPascal target architectures. This document maps FPC backends to SuperPascal platforms and identifies what's available.

**Location:** `../freePascal/compiler/`

**Special Note:** FPC has an **LLVM backend** (`llvm/`) which enables a two-step path to 6502/65C02 targets via **LLVM-MOS** (production-ready 6502/65C02 backend).

**LLVM-MOS SDK:** `../mos-6502/`
- **CommanderX16 Platform:** `llvm-mos-sdk/mos-platform/cx16/` ✅ **Complete support (80+ files)!**
- **See:** `docs/LLVM_MOS_SDK_REFERENCE.md` for complete SDK reference

---

## Direct Backend Coverage

### ✅ ZealZ80 (Z80)
- **FPC Backend:** `z80/`
- **Status:** ✅ **COMPLETE BACKEND AVAILABLE**
- **Key Files:**
  - `z80/cpubase.pas` - Z80 registers, opcodes
  - `z80/cpupara.pas` - Z80 calling conventions
  - `z80/cgcpu.pas` - Z80 code generator (2489 lines)
  - `z80/hlcgcpu.pas` - High-level codegen
  - `z80/rgcpu.pas` - Register allocation
- **Use As:** **PRIMARY REFERENCE** for `backend-zealz80` crate

---

### ✅ FoenixA2560M (MC68060)
- **FPC Backend:** `m68k/`
- **Status:** ✅ **COMPLETE BACKEND AVAILABLE**
- **Key Files:**
  - `m68k/cpu.pas` - m68k CPU-specific codegen
  - `m68k/cgbase.pas` - m68k codegen base
  - `m68k/cgutils.pas` - m68k utilities
  - `m68k/cpupara.pas` - m68k calling conventions
- **Use As:** **PRIMARY REFERENCE** for `backend-foenix68k` crate

---

### ✅ Raspberry Pi 5 (ARM64)
- **FPC Backend:** `aarch64/`
- **Status:** ✅ **COMPLETE BACKEND AVAILABLE**
- **Key Files:**
  - `aarch64/cpubase.pas` - ARM64 registers, opcodes
  - `aarch64/cpupara.pas` - ARM64 calling conventions (AAPCS64)
  - `aarch64/cgcpu.pas` - ARM64 code generator
  - `aarch64/hlcgcpu.pas` - High-level codegen
- **Use As:** **PRIMARY REFERENCE** for `backend-raspberrypi5` crate
- **Note:** ARM64 Procedure Call Standard (AAPCS64) is well-documented in FPC

---

## Partial/Indirect Backend Coverage

### ⚠️ CommanderX16 (WDC 65C02)
- **FPC Backend:** None (6502/65C02 not directly supported)
- **Status:** ⚠️ **NO DIRECT BACKEND** (but LLVM path available!)
- **Two-Step Path via LLVM-MOS:**
  1. **SuperPascal → LLVM IR:** Use FPC `llvm/` backend patterns
  2. **LLVM IR → 6502:** Use **LLVM-MOS** (production-ready 6502/65C02 backend)
  3. **References:**
     - **Website:** https://llvm-mos.org/
     - **Local SDK:** `../mos-6502/`
     - **CommanderX16 Platform:** `mos-6502/llvm-mos-sdk/mos-platform/cx16/` ✅
- **Potential References:**
  - `llvm/` - **LLVM IR generation** (use for step 1)
  - `i8086/` - 16-bit x86 (some 8-bit patterns)
  - `z80/` - 8-bit architecture patterns (different CPU, but similar constraints)
  - `avr/` - 8-bit microcontroller (some patterns)
- **Use As:** 
  - **Option A (Recommended):** Two-step via LLVM-MOS (SuperPascal → LLVM IR → 6502 via LLVM-MOS)
  - **Option B:** Study 8-bit patterns from `z80/` and `avr/`, implement custom backend
- **Note:** 65C02 is 6502-compatible. **LLVM-MOS (https://llvm-mos.org/) is production-ready** and fully supports 65C02!

---

### ✅ Foenix65C816 (WDC 65C816)
- **FPC Backend:** None (65C816 not directly supported)
- **Status:** ✅ **LLVM-MOS path available!**
- **Two-Step Path via LLVM-MOS:**
  1. **SuperPascal → LLVM IR:** Use FPC `llvm/` backend patterns
  2. **LLVM IR → 65C816:** Use **LLVM-MOS** with `-mcpu=mosw65816` (production-ready 65C816 support!)
  3. **References:**
     - **Website:** https://llvm-mos.org/
     - **Local SDK:** `../mos-6502/`
     - **CPU Support:** LLVM-MOS supports `mosw65816` device (WDC 65816/65C816)
- **Platform Support:**
  - **Note:** No pre-made Foenix platform support in LLVM-MOS SDK
  - **Action Required:** Create custom platform files (linker scripts, runtime, etc.)
  - **Reference:** Study `mos-6502/llvm-mos-sdk/mos-platform/cx16/` as template
- **Use As:** 
  - **Option A (Recommended):** Two-step via LLVM-MOS (SuperPascal → LLVM IR → 65C816 via LLVM-MOS)
  - **Option B:** Study 16-bit patterns from `i8086/`, implement custom backend
- **Note:** 65C816 has 24-bit addressing (banked), which LLVM-MOS handles via `mosw65816` CPU target

---

## Available FPC Backends (Not Our Targets)

These backends exist in FPC but are not directly relevant to SuperPascal targets:

- `arm/` - ARM 32-bit (not our target)
- `i386/` - x86 32-bit (not our target)
- `x86_64/` - x86 64-bit (not our target)
- `riscv/`, `riscv32/`, `riscv64/` - RISC-V (not our targets)
- `powerpc/`, `powerpc64/` - PowerPC (not our targets)
- `sparc/`, `sparc64/` - SPARC (not our targets)
- `avr/` - AVR microcontrollers (not our target, but useful for 8-bit patterns)
- `xtensa/` - Xtensa (not our target)
- `wasm32/` - WebAssembly (not our target)
- `jvm/` - Java Virtual Machine (not our target)
- `llvm/` - LLVM IR (not our target)

**Note:** These can still be useful as **reference patterns** for:
- Code generation techniques
- Register allocation strategies
- Calling convention patterns
- Stack frame management

---

## Backend Coverage Summary

| SuperPascal Target | FPC Backend | Status | Coverage |
|-------------------|-------------|--------|----------|
| **ZealZ80** | `z80/` | ✅ Complete | 100% - Primary reference |
| **FoenixA2560M** | `m68k/` | ✅ Complete | 100% - Primary reference |
| **Raspberry Pi 5** | `aarch64/` | ✅ Complete | 100% - Primary reference |
| **CommanderX16** | `llvm/` → LLVM-MOS 6502 | ✅ **LLVM Path** | **LLVM IR gen** - Two-step via LLVM-MOS |
| **Foenix65C816** | `llvm/` → LLVM-MOS 65C816 | ✅ **LLVM Path** | **LLVM IR gen** - Two-step via LLVM-MOS |

**Overall Coverage:** 
- **3 direct backends** (60%)
- **2 LLVM paths** (CommanderX16 and Foenix65C816 via LLVM-MOS)
- **0 custom backends** needed! 🎉

**Total:** 5 out of 5 targets (100%) have FPC support or LLVM path! ✅

---

## How to Use FPC Backends

### For Targets with Direct Backend Support

#### 1. ZealZ80 (`z80/`)
- **Study:** `z80/cpupara.pas` for calling conventions
- **Study:** `z80/cgcpu.pas` for code generation patterns
- **Study:** `z80/cpubase.pas` for register definitions
- **Extract:** ABI rules, register usage, stack frames
- **Compare:** With ZealZ80 ABI specification
- **Implement:** `backend-zealz80` crate in Rust

#### 2. FoenixA2560M (`m68k/`)
- **Study:** `m68k/cpupara.pas` for calling conventions
- **Study:** `m68k/cpu.pas` for code generation
- **Study:** `m68k/cgbase.pas` for base codegen
- **Extract:** m68k calling conventions, register usage
- **Compare:** With FoenixA2560M ABI specification
- **Implement:** `backend-foenix68k` crate in Rust

#### 3. Raspberry Pi 5 (`aarch64/`)
- **Study:** `aarch64/cpupara.pas` for AAPCS64 calling conventions
- **Study:** `aarch64/cgcpu.pas` for ARM64 code generation
- **Study:** `aarch64/cpubase.pas` for ARM64 registers
- **Extract:** AAPCS64 compliance, register usage
- **Compare:** With ARM64 Procedure Call Standard
- **Implement:** `backend-raspberrypi5` crate in Rust

### For Targets Without Direct Backend Support

#### 1. CommanderX16 (65C02) - **LLVM Path Available!**
- **Option A (Recommended):** Two-step via LLVM
  - **Study:** `llvm/` backend for LLVM IR generation
  - **Implement:** `backend-llvm` crate (or integrate into `backend-commanderx16`)
  - **Use:** LLVM's 6502 backend for final code generation
  - **See:** `docs/LLVM_BACKEND_PATH.md` for detailed strategy
- **Option B:** Direct backend
  - **Study:** `z80/` for 8-bit architecture patterns
  - **Study:** `avr/` for 8-bit microcontroller patterns
  - **Reference:** 6502/65C02 documentation directly
  - **Implement:** Custom `backend-commanderx16` crate
- **Note:** 65C02 is 6502-compatible. LLVM path is recommended if LLVM 6502 backend is available.

#### 2. Foenix65C816 (65C816)
- **Study:** `i8086/` for 16-bit architecture patterns
- **Study:** `i386/` for some patterns (but different architecture)
- **Reference:** 65C816 documentation directly
- **Implement:** Custom `backend-foenix816` crate
- **Note:** 65C816 has unique 24-bit addressing (banked)

---

## Common Backend File Patterns

All FPC backends follow similar structure:

### Core Files (Present in All Backends)

1. **`cpubase.pas`** - Base types, registers, opcodes
   - Register definitions
   - Instruction opcodes
   - Condition codes
   - Flags

2. **`cpupara.pas`** - Calling conventions
   - Parameter passing rules
   - Return value handling
   - Volatile/non-volatile registers
   - Stack frame conventions

3. **`cgcpu.pas`** - Code generator
   - Load operations
   - Arithmetic operations
   - Control flow
   - Procedure entry/exit

4. **`hlcgcpu.pas`** - High-level code generator
   - High-level constructs
   - Type conversions
   - Complex operations

5. **`rgcpu.pas`** - Register allocation
   - Register allocation algorithms
   - Register spilling
   - Register preferences

### Optional Files (Some Backends)

- **`cpupi.pas`** - Platform-specific information
- **`symcpu.pas`** - CPU-specific symbol handling
- **`tgcpu.pas`** - Type generation
- **`aoptcpu.pas`** - CPU-specific optimizations

---

## Backend Implementation Strategy

### Phase 1: Backends with FPC Support (Easier)

1. **ZealZ80** (`backend-zealz80`)
   - ✅ Use FPC `z80/` as primary reference
   - Study calling conventions
   - Adapt codegen patterns
   - Implement in Rust

2. **FoenixA2560M** (`backend-foenix68k`)
   - ✅ Use FPC `m68k/` as primary reference
   - Study m68k calling conventions
   - Adapt codegen patterns
   - Implement in Rust

3. **Raspberry Pi 5** (`backend-raspberrypi5`)
   - ✅ Use FPC `aarch64/` as primary reference
   - Study AAPCS64 calling conventions
   - Adapt ARM64 codegen patterns
   - Implement in Rust

### Phase 2: Backends with LLVM Path or Custom Implementation

4. **CommanderX16** (`backend-commanderx16`)
   - **Option A (Recommended):** LLVM-MOS Path
     - ✅ Study `llvm/` backend for LLVM IR generation
     - ✅ Implement LLVM IR generation from SuperPascal IR
     - ✅ Use **LLVM-MOS** (https://llvm-mos.org/) for 6502/65C02 code generation
     - ✅ **Local SDK available:** `mos-6502/llvm-mos-sdk/`
     - ✅ **CommanderX16 platform support:** `mos-6502/llvm-mos-sdk/mos-platform/cx16/`
     - ✅ Faster implementation, better code quality
     - ✅ **Production-ready backend** (not experimental!)
   - **Option B:** Direct Backend
     - ⚠️ Study `z80/` and `avr/` for 8-bit patterns
     - Study 6502/65C02 documentation
     - Implement custom backend
     - Reference CommanderX16 ABI specification

5. **Foenix65C816** (`backend-foenix816`)
   - ⚠️ Study `i8086/` for 16-bit patterns
   - Study 65C816 documentation
   - Implement custom backend
   - Handle unique 24-bit addressing
   - Reference Foenix65C816 ABI specification

---

## Key Insights

### 1. Excellent Coverage for Primary Targets

- **3 out of 5 targets** have complete FPC backend support
- **All Tier 1 platforms** (ZealZ80) have FPC support
- **Primary Tier 2 platform** (Raspberry Pi 5) has FPC support

### 2. FPC Backends Are Comprehensive

- Each backend has **thousands of lines** of code
- Complete calling convention implementations
- Register allocation
- Stack frame management
- Code generation patterns

### 3. Use as Primary Reference

- **Don't copy** FPC code directly
- **Study** FPC patterns and algorithms
- **Adapt** to Rust and SuperPascal needs
- **Compare** with platform-specific ABI specifications

### 4. LLVM Path for CommanderX16

- **CommanderX16** can use **LLVM-MOS path** (two-step: SuperPascal → LLVM IR → 6502 via LLVM-MOS)
- Study FPC `llvm/` backend for LLVM IR generation
- Use **LLVM-MOS** (https://llvm-mos.org/) for 6502/65C02 code generation
- **Local SDK:** `../mos-6502/`
- **CommanderX16 Platform:** `mos-6502/llvm-mos-sdk/mos-platform/cx16/` (complete support!)
- **LLVM-MOS is production-ready** and fully supports 65C02
- Faster implementation, better code quality
- See `docs/LLVM_BACKEND_PATH.md` for details

### 5. Custom Backend Still Needed

- **Foenix65C816** needs custom backend (no LLVM support for 65C816)
- Can learn from similar FPC backends (`i8086/`)
- Use platform documentation as primary source

---

## Quick Reference: Backend Files by Target

| Target | Primary FPC Files |
|--------|-------------------|
| **ZealZ80** | `z80/cpupara.pas`, `z80/cgcpu.pas`, `z80/cpubase.pas` |
| **FoenixA2560M** | `m68k/cpupara.pas`, `m68k/cpu.pas`, `m68k/cgbase.pas` |
| **Raspberry Pi 5** | `aarch64/cpupara.pas`, `aarch64/cgcpu.pas`, `aarch64/cpubase.pas` |
| **CommanderX16** | `llvm/` (**LLVM IR gen**), **LLVM-MOS** (https://llvm-mos.org/), **Local SDK:** `mos-6502/llvm-mos-sdk/mos-platform/cx16/` |
| **Foenix65C816** | `i8086/` (patterns), 65C816 docs |

---

## Summary

**FPC Backend Coverage:**
- ✅ **3 complete direct backends** for SuperPascal targets
- ✅ **1 LLVM path** (CommanderX16 via LLVM)
- ⚠️ **1 custom backend** needed (Foenix65C816)
- 📚 **Extensive reference material** for all targets

**Recommendation:**
- Start with backends that have FPC support (easier)
- Use FPC as primary reference for those targets
- **Use LLVM path for CommanderX16** (faster, better code quality)
- Study FPC patterns for custom backends
- Implement all backends in Rust with SuperPascal ABI compliance

**See Also:**
- `docs/LLVM_BACKEND_PATH.md` - Detailed LLVM path strategy for CommanderX16

---

**Version:** 1.0  
**Date:** 2024  
**Status:** FPC Backend Coverage Analysis

