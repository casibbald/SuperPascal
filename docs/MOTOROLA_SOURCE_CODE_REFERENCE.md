# Motorola Source Code Reference

## Overview

This document catalogs the Motorola 680x0 source code available in `../Motorolla/` and its relevance for SuperPascal compiler development, particularly for the FoenixA2560M (MC68LC060) backend.

**Last Updated:** 2025-01-XX

---

## Directory Structure

```
Motorolla/
├── amiga_mc68060sp/          ⭐⭐⭐⭐⭐ CRITICAL - Official Motorola MC68060 Software Package
├── defender/                 ⭐⭐⭐⭐ VERY USEFUL - Complete game showing real-world m68k patterns
├── hatari/                   ⭐⭐⭐ USEFUL - Atari ST emulator (m68k CPU emulation)
├── aranym/                   ⭐⭐⭐ USEFUL - Atari emulator (m68k CPU emulation)
├── atari_public/             ⭐⭐⭐⭐ VERY USEFUL - Public domain Atari code (C + Assembly)
├── Amiga/                    ⭐⭐⭐ USEFUL - Amiga demos and code
├── amiga-stuff/              ⭐⭐⭐ USEFUL - Amiga utilities
└── Projects/                 ⭐⭐ INTERESTING - Hardware projects
```

---

## 1. MC68060 Software Package ⭐⭐⭐⭐⭐

**Location:** `../Motorolla/amiga_mc68060sp/`

**Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL for FoenixA2560M**

### Overview

Official Motorola MC68060 Software Package (M68060SP) - **This is the official Motorola software support package!**

### Key Components

#### 68060 Library (`68060_lib/`)

**Assembly Files:**
- `mc60_startup.asm` - **Startup code and initialization**
- `mc60_dispatcher.asm` - **Exception dispatcher** (shows calling conventions!)
- `mc60_misc.asm` - Miscellaneous utilities
- `mc60_patches.asm` - CPU patches
- `mc60_tagstub.asm` - Tag stub code

**C Files:**
- `mc60_init.c` - Initialization code
- `mc60_mmu.c` - MMU management

**FPU Emulation (`fpuemu/`):**
- `fpsp_base.asm` - FPU emulation base
- `fpsp_init.asm` - FPU initialization
- `fpsp_excepts.asm` - FPU exception handlers
- `fpsp_subs.asm` - FPU subroutines
- `fpsp_trans.asm` - FPU translation routines
- **Note:** MC68LC060 has no FPU, but these show exception handling patterns

**Integer Unimplemented (`ispemu/`):**
- `isp.asm` - Integer unimplemented instruction handler
- **Relevant for MC68LC060** - Shows exception handling

### Key Features

1. **Exception Handling:**
   - Shows how to handle MC68060 exceptions
   - Exception stack frame layout
   - Exception dispatcher patterns

2. **Calling Conventions:**
   - Assembly code shows register usage
   - Stack frame management (LINK/UNLK patterns)
   - Parameter passing conventions

3. **Startup Code:**
   - System initialization
   - MMU setup
   - Cache configuration

4. **Optimization:**
   - 68060-specific optimizations
   - Cache management
   - Pipeline considerations

### Relevance for SuperPascal

**For FoenixA2560M ABI:**
- ✅ **Exception handling patterns** - Shows how to set up exception handlers
- ✅ **Stack frame layout** - Real examples of LINK/UNLK usage
- ✅ **Register usage** - Shows register conventions in practice
- ✅ **Startup code** - System initialization patterns
- ✅ **MMU management** - MC68LC060 has MMU, this shows how to use it

**For Code Generation:**
- ✅ **Instruction patterns** - Shows optimal instruction sequences
- ✅ **Exception vectors** - How to set up exception vectors
- ✅ **System integration** - How to integrate with operating system

### Key Files to Study

1. **`mc60_startup.asm`** - Startup and initialization
2. **`mc60_dispatcher.asm`** - Exception dispatcher (calling conventions!)
3. **`mc60_misc.asm`** - Utility functions
4. **`isp.asm`** - Integer exception handler (relevant for MC68LC060)

---

## 2. Defender Game ⭐⭐⭐⭐

**Location:** `../Motorolla/defender/`

**Relevance:** ⭐⭐⭐⭐ **VERY USEFUL - Real-world m68k programming**

### Overview

Complete port of the Defender arcade game for Atari ST/STE/Falcon030. Shows real-world m68k programming patterns.

### Key Components

**Assembly Files:**
- `start.S` - **Startup code** - Shows system initialization
- `gemdos.S` - GEMDOS system calls
- `plot.S` - Graphics plotting routines
- `grplot.S` - Graphics routines
- `raster.S` - Raster interrupt handling
- `irq.c` - Interrupt handling (C code)
- `thread.S` - Thread management

**C Files:**
- `main.c` - Main game loop
- `exec.c` - Execution management
- `screen.c` - Screen management
- `sound.c` - Sound handling
- Many game logic files

### Key Features

1. **Real-World Patterns:**
   - Complete game showing m68k programming in practice
   - Mix of C and assembly
   - System integration (GEMDOS, interrupts)

2. **Calling Conventions:**
   - Shows how C and assembly interact
   - Parameter passing between C and assembly
   - Stack frame usage

3. **System Integration:**
   - GEMDOS system calls
   - Interrupt handling
   - Memory management

### Relevance for SuperPascal

**For ABI:**
- ✅ **Real calling conventions** - Shows how functions are actually called
- ✅ **C/Assembly interop** - Shows parameter passing between languages
- ✅ **Stack usage** - Real-world stack frame patterns

**For Code Generation:**
- ✅ **Instruction sequences** - Real code showing instruction usage
- ✅ **Optimization patterns** - How code is optimized in practice
- ✅ **System calls** - How to generate system call code

### Key Files to Study

1. **`start.S`** - Startup code showing system initialization
2. **`gemdos.S`** - System call interface
3. **`main.c`** - Main program structure
4. **`irq.c`** - Interrupt handling patterns

---

## 3. Hatari Emulator ⭐⭐⭐

**Location:** `../Motorolla/hatari/`

**Relevance:** ⭐⭐⭐ **USEFUL - m68k CPU emulation**

### Overview

Hatari is an Atari ST/STE/TT/Falcon emulator. The source code contains m68k CPU emulation.

### Key Components

**CPU Emulation:**
- `src/cpu/` - CPU emulation code
- `src/m68000.c` - 68000 CPU emulation
- `src/m68020.c` - 68020 CPU emulation
- `src/m68030.c` - 68030 CPU emulation
- `src/m68040.c` - 68040 CPU emulation
- `src/m68060.c` - **68060 CPU emulation!**

**Tests:**
- `tests/cpu/` - CPU instruction tests
- `tests/cycles/` - Cycle-accurate tests

### Relevance for SuperPascal

**For Code Generation:**
- ✅ **Instruction semantics** - Shows how instructions work
- ✅ **Cycle counting** - Instruction timing
- ✅ **Exception handling** - How exceptions are processed

**For Testing:**
- ✅ **Test cases** - Can use Hatari tests to validate code generation
- ✅ **Instruction coverage** - Ensures all instructions are handled

### Key Files to Study

1. **`src/m68060.c`** - 68060 CPU emulation (if available)
2. **`tests/cpu/`** - CPU instruction tests

---

## 4. ARAnyM Emulator ⭐⭐⭐

**Location:** `../Motorolla/aranym/`

**Relevance:** ⭐⭐⭐ **USEFUL - Atari emulator with m68k support**

### Overview

ARAnyM (Atari Running on Any Machine) - Atari emulator with m68k CPU support.

### Key Components

**CPU Emulation:**
- `atari/` - Atari-specific code
- `src/` - Emulator source code

### Relevance for SuperPascal

Similar to Hatari - useful for understanding m68k instruction semantics and testing.

---

## 5. Atari Public Domain Code ⭐⭐⭐⭐

**Location:** `../Motorolla/atari_public/`

**Relevance:** ⭐⭐⭐⭐ **VERY USEFUL - Public domain examples**

### Overview

Collection of public domain Atari ST/TT/Falcon code in C and Assembly.

### Key Projects

- **OUTSIDE** - Virtual memory manager (C + Assembly)
- **SCSI2PI** - SCSI driver (C code)
- **XHDI_MON** - XHDI monitor (C + Assembly)
- **MODULES** - HDDRIVER modules (C + Assembly)

### Relevance for SuperPascal

**For ABI:**
- ✅ **C/Assembly interop** - Shows calling conventions
- ✅ **System programming** - Real system-level code
- ✅ **Stack frames** - Real-world stack usage

**For Code Generation:**
- ✅ **Code examples** - Real code to study
- ✅ **Optimization** - How code is optimized

### Key Files to Study

1. **`OUTSIDE/SRC/OUTSIDE.S`** - Virtual memory manager (Assembly)
2. **`SCSI2PI/*.C`** - SCSI driver (C code)
3. **`XHDI_MON/*.S`** - XHDI monitor (Assembly)

---

## 6. Amiga Code ⭐⭐⭐

**Location:** `../Motorolla/Amiga/` and `amiga-stuff/`

**Relevance:** ⭐⭐⭐ **USEFUL - Amiga m68k programming**

### Overview

Amiga demos and utilities. Amiga uses m68k processors (68000, 68020, 68030, 68040, 68060).

### Key Components

- **AsmLib** - Assembly library examples
- **Demos** - Various demos showing m68k programming
- **Utilities** - Amiga utilities

### Relevance for SuperPascal

**For Code Generation:**
- ✅ **Optimization techniques** - Amiga demoscene optimization
- ✅ **Instruction patterns** - Real-world instruction usage
- ✅ **Graphics programming** - Hardware interaction patterns

---

## Summary: Most Valuable Resources

### For FoenixA2560M ABI Development

1. **⭐ MC68060 Software Package** (`amiga_mc68060sp/`)
   - Official Motorola code
   - Exception handling
   - Startup code
   - Calling conventions

2. **⭐ Defender Game** (`defender/`)
   - Real-world programming
   - C/Assembly interop
   - System integration

3. **⭐ Atari Public Code** (`atari_public/`)
   - Public domain examples
   - System programming
   - C/Assembly patterns

### For Code Generation

1. **MC68060 Software Package** - Official instruction patterns
2. **Defender Game** - Real code sequences
3. **Hatari/ARAnyM** - Instruction semantics
4. **Atari Public Code** - Code examples

### For Testing

1. **Hatari Tests** (`hatari/tests/`) - CPU instruction tests
2. **Defender** - Real application to test against

---

## Action Items

### Immediate (High Priority)

1. ✅ **Study MC68060 Software Package**
   - Extract calling conventions from `mc60_dispatcher.asm`
   - Study stack frame patterns from `mc60_startup.asm`
   - Review exception handling from `isp.asm`

2. ✅ **Study Defender Game**
   - Extract calling conventions from `start.S` and `gemdos.S`
   - Study C/Assembly interop patterns
   - Review stack frame usage

3. ✅ **Study Atari Public Code**
   - Extract calling conventions from `OUTSIDE.S`
   - Study system programming patterns

### Medium Priority

4. 🔄 **Cross-reference with MC68060UM manual**
   - Compare manual specifications with actual code
   - Validate calling conventions

5. 🔄 **Extract optimization patterns**
   - Study instruction sequences
   - Identify optimization opportunities

### Low Priority

6. 🔄 **Use Hatari tests for validation**
   - Validate code generation against test cases
   - Ensure instruction correctness

---

## Integration with Existing Documentation

### Already Referenced

- ✅ MC68060UM manual - Processor specification
- ✅ FPC m68k backend - Compiler reference
- ✅ Atari archive - Optimization resources

### New Resources Added

- ✅ **MC68060 Software Package** - Official Motorola code
- ✅ **Defender Game** - Real-world examples
- ✅ **Atari Public Code** - Public domain examples
- ✅ **Hatari/ARAnyM** - Emulator code

---

**Next Steps:**
1. Extract calling conventions from MC68060 Software Package
2. Study stack frame patterns from Defender game
3. Cross-reference with MC68060UM manual
4. Create FoenixA2560M ABI specification

