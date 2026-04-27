# LLVM-MOS SDK Reference Guide

## Overview

The LLVM-MOS SDK provides **complete platform support** for CommanderX16, including libraries, linker scripts, and examples. This is a **huge advantage** for implementing the CommanderX16 backend.

**Location:** `../mos-6502/`

**Website:** https://llvm-mos.org/

---

## SDK Structure

### 1. LLVM-MOS Compiler (`llvm-mos/`)

**Location:** `mos-6502/llvm-mos/`

**Purpose:** Complete LLVM-MOS compiler with 6502/65C02 backend

**Key Components:**
- `llvm/lib/Target/MOS/` - MOS backend implementation
- `clang/` - Clang frontend with MOS support
- `lld/` - Linker with MOS support

**Use For:**
- Understanding LLVM-MOS backend implementation
- MOS-specific code generation patterns
- Register allocation strategies
- Calling conventions

---

### 2. LLVM-MOS SDK (`llvm-mos-sdk/`)

**Location:** `mos-6502/llvm-mos-sdk/`

**Purpose:** Platform libraries, tools, and examples

**Key Components:**
- `mos-platform/` - Platform-specific support
- `examples/` - Example programs
- `common/` - Common libraries
- `utils/` - Utility tools

---

## CommanderX16 Platform Support

### Location
`mos-6502/llvm-mos-sdk/mos-platform/cx16/`

### Status
✅ **Complete platform support** - 80+ files!

### Key Files

#### Platform Configuration

**`clang.cfg`** - Compiler configuration
```
-mlto-zp=90
-D__CX16__
-mcpu=mosw65c02
```

**Key Settings:**
- `-mcpu=mosw65c02` - Target CPU (WDC 65C02)
- `-mlto-zp=90` - Zero-page allocation
- `-D__CX16__` - CommanderX16 platform define

#### Linker Scripts

**`link.ld`** - Main linker script
- Memory layout
- Section definitions
- Entry point
- Memory regions

**`imag-regs.ld`** - Imaginary registers layout
- Zero-page register allocation
- Register placement

#### KERNAL Function Wrappers

**KERNAL API Functions** (all `cx16_k_*.s` files):
- `cx16_k_console_*.s` - Console functions
- `cx16_k_graph_*.s` - Graphics functions
- `cx16_k_fb_*.s` - Framebuffer functions
- `cx16_k_mouse_*.s` - Mouse functions
- `cx16_k_joystick_*.s` - Joystick functions
- `cx16_k_keymap_*.s` - Keyboard functions
- `cx16_k_memory_*.s` - Memory functions
- `cx16_k_screen_*.s` - Screen functions
- `cx16_k_sprite_*.s` - Sprite functions
- `cx16_k_clock_*.s` - Clock functions
- `cx16_k_i2c_*.s` - I2C functions
- And many more...

**Use For:**
- Understanding KERNAL API
- Function calling conventions
- Parameter passing
- Return value handling

#### Headers and Definitions

**`cx16.h`** - CommanderX16 header file
- Platform constants
- Type definitions
- Function declarations

**`cx16.inc`** - Assembly includes
- Register definitions
- Memory map constants
- Assembly macros

#### Runtime Support

**`kernal.S`** - KERNAL initialization
- Startup code
- System initialization

**`basic-header.S`** - BASIC header
- Program header
- BASIC compatibility

**`char-conv.c`** - Character conversion
- Character encoding utilities

**`charset.h`** - Character set definitions

#### Utility Functions

**`vpeek.s`, `vpoke.s`** - VERA memory access
- VERA register access
- Memory-mapped I/O

**`waitvsync.s`** - VSYNC wait
- Vertical sync synchronization

**`videomode.s`** - Video mode setup
- VERA configuration

**`vera_layer_enable.s`, `vera_sprites_enable.s`** - VERA features
- Layer and sprite control

---

## Examples

### Location
`mos-6502/llvm-mos-sdk/examples/cx16/`

### Example Programs

1. **`k_cbm_file_test.c`** - File I/O test
2. **`k_console_test.c`** - Console functions test
3. **`k_datetime_test.c`** - Date/time functions test
4. **`k_graph_test.c`** - Graphics functions test
5. **`k_graph_line_test.c`** - Line drawing test
6. **`k_joystick_get_test.cc`** - Joystick test (C++)
7. **`plasma.cc`** - Plasma effect demo (C++)

**Use For:**
- Understanding how to use LLVM-MOS with CommanderX16
- Learning KERNAL API usage
- Testing compilation pipeline
- Reference implementations

---

## How to Use LLVM-MOS SDK for CommanderX16

### 1. Study Platform Configuration

**Read:** `mos-6502/llvm-mos-sdk/mos-platform/cx16/clang.cfg`
- Understand compiler flags
- Learn target CPU specification
- Understand zero-page allocation

### 2. Study Linker Scripts

**Read:** `mos-6502/llvm-mos-sdk/mos-platform/cx16/link.ld`
- Understand memory layout
- Learn section definitions
- Understand entry point

**Read:** `mos-6502/llvm-mos-sdk/mos-platform/cx16/imag-regs.ld`
- Understand zero-page register allocation
- Learn register placement

### 3. Study KERNAL Wrappers

**Read:** `mos-6502/llvm-mos-sdk/mos-platform/cx16/cx16_k_*.s` files
- Understand KERNAL API
- Learn calling conventions
- Understand parameter passing
- Learn return value handling

### 4. Study Examples

**Read:** `mos-6502/llvm-mos-sdk/examples/cx16/*.c` and `*.cc` files
- Understand how to use LLVM-MOS
- Learn compilation process
- Understand runtime integration

### 5. Study Headers

**Read:** `mos-6502/llvm-mos-sdk/mos-platform/cx16/cx16.h`
- Understand platform constants
- Learn type definitions
- Understand API structure

---

## Integration with SuperPascal

### For `backend-commanderx16` Implementation

#### Step 1: LLVM IR Generation
- Study FPC `llvm/` backend for LLVM IR generation patterns
- Implement LLVM IR generation from SuperPascal IR
- Use LLVM-MOS SDK as reference for target configuration

#### Step 2: LLVM-MOS Compilation
- Use LLVM-MOS compiler to generate 6502 assembly
- Use CommanderX16 platform configuration (`clang.cfg`)
- Use CommanderX16 linker script (`link.ld`)

#### Step 3: Runtime Integration
- Study KERNAL wrappers for runtime functions
- Integrate with SuperPascal standard library
- Use CommanderX16 headers and definitions

---

## Key Insights

### 1. Complete Platform Support

- ✅ **80+ files** of CommanderX16 platform support
- ✅ **All KERNAL functions** wrapped
- ✅ **Linker scripts** ready to use
- ✅ **Examples** for reference
- ✅ **Headers** for API definitions

### 2. Production-Ready

- LLVM-MOS SDK is actively maintained
- CommanderX16 is a **first-class supported platform**
- Examples demonstrate real-world usage
- Well-documented and tested

### 3. Easy Integration

- Platform configuration is ready (`clang.cfg`)
- Linker scripts are ready (`link.ld`)
- Runtime functions are wrapped
- Examples show how to use everything

---

## Compilation Pipeline

### Using LLVM-MOS SDK

```bash
# 1. Generate LLVM IR from SuperPascal
superpascal-compiler --emit-llvm program.pas -o program.ll

# 2. Compile to 6502 assembly using LLVM-MOS
mos-cx16-clang -c program.ll -o program.o

# 3. Link using LLVM-MOS linker
mos-cx16-ld -T mos-6502/llvm-mos-sdk/mos-platform/cx16/link.ld \
  program.o -o program.prg
```

### Platform-Specific Compiler

LLVM-MOS SDK provides platform-specific compiler wrappers:
- `mos-cx16-clang` - CommanderX16 C compiler
- `mos-cx16-clang++` - CommanderX16 C++ compiler
- `mos-cx16-ld` - CommanderX16 linker

---

## CommanderX16 KERNAL API

### Available Functions (from SDK)

The SDK provides wrappers for **all** CommanderX16 KERNAL functions:

**Console:**
- `cx16_k_console_init`
- `cx16_k_console_put_char`
- `cx16_k_console_get_char`
- `cx16_k_console_put_image`
- `cx16_k_console_set_paging_message`

**Graphics:**
- `cx16_k_graph_init`
- `cx16_k_graph_clear`
- `cx16_k_graph_draw_line`
- `cx16_k_graph_draw_rect`
- `cx16_k_graph_draw_oval`
- `cx16_k_graph_draw_image`
- `cx16_k_graph_put_char`
- `cx16_k_graph_move_rect`
- `cx16_k_graph_set_colors`
- `cx16_k_graph_set_font`
- `cx16_k_graph_set_window`

**Framebuffer:**
- `cx16_k_fb_init`
- `cx16_k_fb_get_pixel`
- `cx16_k_fb_set_pixel`
- `cx16_k_fb_fill_pixels`
- `cx16_k_fb_move_pixels`
- `cx16_k_fb_set_palette`
- And more...

**Input:**
- `cx16_k_joystick_get`
- `cx16_k_joystick_scan`
- `cx16_k_mouse_get`
- `cx16_k_mouse_scan`
- `cx16_k_kbdbuf_peek`
- `cx16_k_kbdbuf_put`

**Memory:**
- `cx16_k_memory_copy`
- `cx16_k_memory_fill`
- `cx16_k_memory_crc`
- `cx16_k_memory_decompress`

**System:**
- `cx16_k_clock_get_date_time`
- `cx16_k_clock_set_date_time`
- `cx16_k_screen_mode_get`
- `cx16_k_screen_mode_set`
- `cx16_k_enter_basic`
- `cx16_k_monitor`

**And many more...**

**Use For:**
- Understanding CommanderX16 API
- Implementing SuperPascal standard library
- Runtime function integration

---

## Memory Layout (from `link.ld`)

Study the linker script to understand:
- Memory regions
- Section placement
- Zero-page usage
- Stack location
- Heap location

---

## Zero-Page Allocation

From `clang.cfg`: `-mlto-zp=90`

This allocates 90 bytes of zero-page for imaginary registers.

**Study:** `imag-regs.ld` for register placement

---

## Target Specification

### CPU
- **Target:** `mosw65c02` (WDC 65C02)
- **Specified in:** `clang.cfg` as `-mcpu=mosw65c02`

### Platform
- **Platform:** `cx16` (CommanderX16)
- **Define:** `__CX16__`

---

## Integration Checklist

### For SuperPascal CommanderX16 Backend

- [ ] Study `clang.cfg` for compiler configuration
- [ ] Study `link.ld` for memory layout
- [ ] Study `imag-regs.ld` for register allocation
- [ ] Study KERNAL wrappers for API understanding
- [ ] Study examples for compilation process
- [ ] Study `cx16.h` for API definitions
- [ ] Implement LLVM IR generation (using FPC patterns)
- [ ] Integrate LLVM-MOS compiler invocation
- [ ] Use CommanderX16 platform configuration
- [ ] Integrate with SuperPascal standard library

---

## Advantages of Using LLVM-MOS SDK

### 1. Complete Platform Support
- ✅ All KERNAL functions wrapped
- ✅ Linker scripts ready
- ✅ Headers and definitions
- ✅ Examples for reference

### 2. Production-Ready
- ✅ Actively maintained
- ✅ Well-tested
- ✅ Well-documented

### 3. Easy Integration
- ✅ Platform configuration ready
- ✅ Compiler flags defined
- ✅ Memory layout specified
- ✅ Runtime functions available

### 4. Reference Implementation
- ✅ Examples show best practices
- ✅ KERNAL wrappers show calling conventions
- ✅ Linker scripts show memory layout

---

## Quick Reference

| Need to understand... | Read file... |
|----------------------|--------------|
| Compiler configuration | `clang.cfg` |
| Memory layout | `link.ld` |
| Register allocation | `imag-regs.ld` |
| KERNAL API | `cx16_k_*.s` files |
| Platform constants | `cx16.h` |
| Examples | `examples/cx16/` |
| LLVM-MOS backend | `llvm-mos/llvm/lib/Target/MOS/` |

---

## Summary

**LLVM-MOS SDK provides:**
- ✅ **Complete CommanderX16 platform support**
- ✅ **80+ files** of platform-specific code
- ✅ **All KERNAL functions** wrapped
- ✅ **Linker scripts** ready to use
- ✅ **Examples** for reference
- ✅ **Production-ready** implementation

**For SuperPascal:**
- Use LLVM-MOS SDK as **primary reference** for CommanderX16 backend
- Study platform configuration and linker scripts
- Use KERNAL wrappers as reference for runtime integration
- Follow examples for compilation pipeline

**This makes CommanderX16 backend implementation much easier!**

---

**Version:** 1.0  
**Date:** 2024  
**Status:** LLVM-MOS SDK Reference Guide

