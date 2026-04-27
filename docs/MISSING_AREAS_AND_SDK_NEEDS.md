# Missing Areas and SDK/Information Needs

## Overview

This document identifies areas where SuperPascal needs additional SDKs, documentation, or specifications to complete platform support.

**Last Updated:** 2025-01-XX

---

## Summary by Platform

| Platform | ABI | Intrinsics | Runtime | Stdlib | Game Engine | Object Format | SDK Status |
|----------|-----|------------|---------|--------|-------------|---------------|------------|
| **ZealZ80** | ✅ Complete | ✅ Complete | ✅ Specified | ✅ Specified | ✅ Complete | ✅ Specified | ✅ Have SDKs |
| **CommanderX16** | ✅ **Have Source** | ✅ **Have Docs** | ✅ **Have Source** | ✅ **Have Docs** | ✅ **Have ECS** | ✅ **Have Source** | ✅ **Complete SDK** |
| **Foenix65C816** | ✅ **Have Source** | ✅ **Have Source** | ✅ **Have Source** | ✅ **Have Source** | ✅ **Have Docs** | ✅ **Have Source** | ✅ **Complete SDK** |
| **FoenixA2560M** | ✅ **Have Manual** | ✅ **Have Source** | ✅ **Have Source** | ✅ **Have Source** | ✅ **Have ECS** | ✅ **Have Source** | ✅ **Complete SDK** |
| **Raspberry Pi 5** | ✅ **Have Source** | ✅ **Have Source** | ✅ **Have Source** | ✅ **Have Source** | N/A | ✅ **Have Source** | ✅ **Bare-Metal Examples** |

---

## 1. ABI Specifications (Calling Conventions)

### ✅ CommanderX16 (65C02)
**Status:** ✅ **Complete SDK Available** - Can extract from source  
**Needs:**
- 6502/65C02 calling conventions
- Stack frame layout (6502 has limited registers)
- Parameter passing rules (stack-based)
- Return value conventions (A register, A+X for 16-bit)
- Register usage (A, X, Y, SP)
- Memory banking conventions

**SDK/Info Sources:**
- ✅ **Have:** Complete CommanderX16 SDK (`../CommandX16/`)
  - **x16-docs/**: Complete documentation including:
    - KERNAL API (`X16 Reference - 05 - KERNAL.md`) - **Calling conventions documented!**
    - Memory Map (`X16 Reference - 08 - Memory Map.md`)
    - 65C02 Processor reference (`X16 Reference - Appendix C - 65C02 Processor.md`)
    - VERA Programmer's Reference (`X16 Reference - 09 - VERA Programmer's Reference.md`)
    - Sound Programming (`X16 Reference - 11 - Sound Programming.md`) - YM2151, PSG APIs
  - **x16-rom/**: Complete ROM source code including:
    - **kernal/**: KERNAL source code - **Can extract calling conventions from assembly!**
    - **audio/**: Audio routines source
    - **graphics/**: Graphics routines source
    - **basic/**: BASIC interpreter source
  - **x16-emulator/**: Emulator source code
- ✅ **Have:** LLVM-MOS SDK (`mos-6502/llvm-mos-sdk/mos-platform/cx16/`) - has linker scripts, examples

**Action Items:**
1. ✅ Extract calling conventions from KERNAL source (`x16-rom/kernal/`)
2. ✅ Review KERNAL API documentation for calling conventions (`x16-docs/X16 Reference - 05 - KERNAL.md`)
3. ✅ Study LLVM-MOS SDK `cx16/` platform files for calling patterns
4. Define SuperPascal-specific 65C02 ABI based on KERNAL conventions
5. Document in `platforms/CommanderX16/ABI.md`

---

### ✅ Foenix65C816 (65C816)
**Status:** ✅ **Complete SDK Available** - Can extract from source  
**Needs:**
- 65C816 calling conventions (native mode vs. emulation mode)
- Stack frame layout (24-bit addressing)
- Parameter passing rules (16-bit registers in native mode)
- Return value conventions
- Register usage (A, X, Y, SP, D, PBR, DBR)
- Memory banking conventions (24-bit addressing)

**SDK/Info Sources:**
- ✅ **Have:** Complete Foenix SDK (`../Foenix/`)
  - **BASIC816/** - BASIC interpreter for 65C816 with **calling convention macros!**
    - `src/macros.s` - **ENTER/LEAVE macros showing stack frame layout!**
    - `src/basic816.s` - Complete BASIC implementation
    - Shows 65C816 calling conventions in practice
  - **f256-microkernel/** - MicroKernel with hardware definitions
    - `f256/TinyVicky_Def.asm` - VICKY register definitions
    - `f256/interrupt_def.asm` - Interrupt controller definitions
    - `hardware/` - Hardware driver implementations
  - **FoenixIDE/Manuals/** - PDF documentation
    - `65816 Programmanual.pdf` - 65C816 programming manual
    - `65816 Quick Reference.pdf` - Quick reference
    - `C256_Foenix_Memory_Map_RevA.pdf` - Memory map
    - `c256-foenix-devguide.pdf` - Developer guide
  - **f256-superbasic/** - Another BASIC implementation (reference)
  - **F256-FileManager/** - File manager application (API examples)
- ✅ **Have:** LLVM-MOS supports `mosw65816` CPU

**Action Items:**
1. ✅ Extract calling conventions from BASIC816 `macros.s` (ENTER/LEAVE macros)
2. ✅ Study Foenix MicroKernel assembly for calling patterns
3. ✅ Review FoenixIDE manuals for 65C816 details
4. Review LLVM-MOS 65C816 code generation patterns
5. Define SuperPascal-specific 65C816 ABI (native mode) based on BASIC816 patterns
6. Document in `platforms/Foenix65C816/ABI.md`

---

### ✅ FoenixA2560M (MC68060)
**Status:** ✅ **Complete MC68060 Manual + Optimization Resources Available** - Can extract from source  
**Needs:**
- m68k calling conventions (Motorola 68K ABI)
- Stack frame layout
- Parameter passing rules (register-based)
- Return value conventions
- Register usage (D0-D7, A0-A7)
- Big-endian data layout

**SDK/Info Sources:**
- ✅ **Have:** **Complete Motorola MC68060 User's Manual** (Three versions available):
  - **`MC68LC060.md`** (7,477 lines) - ⭐⭐⭐⭐⭐ **MOST RELEVANT** - Specifically for MC68LC060 (FoenixA2560M chip!)
  - **`MC68060UM.md`** (7,664 lines) - Better formatted version with HTML tables
  - **`Mototolla MC68060UM.md`** (7,826 lines) - Original version
  - **Section 3:** Integer Unit Register Description
    - Data Registers (D7-D0) - 32-bit data registers
    - Address Registers (A6-A0) - 32-bit address registers
    - **A7:** User Stack Pointer (SP)
    - **A6:** Commonly used as Frame Pointer (FP)
    - Program Counter (PC)
    - Condition Code Register (CCR)
  - **Section 8:** Exception Processing
    - Exception stack frame format
    - Stack frame layout patterns
  - **Section 10:** Instruction Timings
    - LINK/UNLK instructions (stack frame management)
    - JSR/BSR/RTS instructions (subroutine calls)
  - Complete instruction set reference
  - Architecture details (big-endian, 32-bit)
- ✅ **Have:** **Motorola MC68060 Source Code** (`../Motorolla/`)
  - **`amiga_mc68060sp/`** - ⭐⭐⭐⭐⭐ **CRITICAL** - **Official Motorola MC68060 Software Package!**
    - `68060_lib/mc60_startup.asm` - **Startup code and initialization**
    - `68060_lib/mc60_dispatcher.asm` - **Exception dispatcher** (shows calling conventions!)
    - `68060_lib/mc60_misc.asm` - Utility functions
    - `68060_lib/ispemu/isp.asm` - Integer exception handler (relevant for MC68LC060)
    - Shows real exception handling, stack frames, calling conventions
  - **`defender/`** - ⭐⭐⭐⭐ **VERY USEFUL** - Complete game showing real-world m68k programming
    - `start.S` - Startup code showing system initialization
    - `gemdos.S` - System call interface
    - Mix of C and assembly showing calling conventions
  - **`atari_public/`** - ⭐⭐⭐⭐ **VERY USEFUL** - Public domain Atari code (C + Assembly)
    - `OUTSIDE/SRC/OUTSIDE.S` - Virtual memory manager
    - `SCSI2PI/` - SCSI driver (C code)
    - Shows real-world system programming patterns
  - **`hatari/`** and **`aranym/`** - ⭐⭐⭐ **USEFUL** - Atari emulators with m68k CPU emulation
    - Can study instruction semantics and test code generation
- ✅ **Have:** **Atari Demoscene Documentation Archive** ([https://mikro.naprvyraz.sk/docs/](https://mikro.naprvyraz.sk/docs/))
  - **68030 Stuff:**
    - `MOTOROLA.TXT` - Motorola 68030 documentation (MUST for Falcon coders)
    - `PIPELINE.TXT` - Pipelining on 030 (Evil/DHS)
  - **Optimizing 680x0 Code:**
    - `68OPT.TXT` - **Optimizing 680x0 Applications (Tomas Evensen)** - ⭐⭐⭐⭐⭐ **CRITICAL** - Various optimizing methods for 68000/030/040/060
    - `OPTIMIZE.TXT` - Nice optimizing tricks
    - `OPTSRC.TXT` - Optimizing your source code (Stefan Posthuma) - Dirty optimizing on pure 68000
    - `MOVE16V2.ZIP` - Move16 instruction (Illustrate using of 040's Move16)
  - **68881/2 FPU Stuff:**
    - `68881_1.TXT` - Cooperating with the Coprocessor (Jim Reekes)
    - `68881_2.TXT` - Speedy the Math Coprocessor (Rich Collyer)
    - `881_ASM.TXT` - FPU assembler programming (Erik H. Bakke)
    - `MACRO881.ZIP` - MC68881 as a peripheral (Motorola)
  - **Note:** This archive focuses on Atari ST/Falcon demoscene coding, but contains valuable Motorola 680x0 optimization and programming techniques applicable to FoenixA2560M
- ✅ **Have:** FPC `m68k/` backend (`freePascal/compiler/m68k/`) - **PRIMARY REFERENCE**
- ✅ **Have:** FPC m68k calling conventions in `m68k/cpupara.pas`
- 🔍 **Need:** System V ABI for m68k (if applicable)

**Action Items:**
1. ✅ Extract register usage from MC68060UM Section 3
2. ✅ Extract stack frame patterns from MC68060UM Section 8
3. ✅ Study LINK/UNLK instructions for stack frame management
4. ✅ Study JSR/BSR/RTS for calling conventions
5. 🔄 Download/archive key documents from Atari archive (`68OPT.TXT`, `MOTOROLA.TXT`, `PIPELINE.TXT`)
6. 🔄 Extract optimization patterns from `68OPT.TXT` for code generator
7. Extract calling conventions from FPC `m68k/cpupara.pas`
8. Review FPC m68k code generation patterns
9. Define SuperPascal-specific m68k ABI (may mirror FPC)
10. Document in `platforms/FoenixA2560M/ABI.md`

---

### ✅ Raspberry Pi 5 (ARM64)
**Status:** ✅ **Bare-Metal Examples Available** - Can extract from source  
**Needs:**
- ARM64 Procedure Call Standard (AAPCS64)
- Stack frame layout
- Parameter passing rules (X0-X7 registers)
- Return value conventions (X0-X7, V0-V7 for SIMD)
- Register usage (X0-X30, V0-V31, SP, FP, LR)
- Stack alignment (16-byte aligned)

**SDK/Info Sources:**
- ✅ **Have:** Complete bare-metal ARM64 examples (`../RaspberryPi/spectrum4/`)
  - **kernel/** - Complete ARM64 kernel implementation
    - `entry.s` - **Exception vectors and kernel entry/exit macros showing register usage!**
    - `kernel.s` - Kernel initialization and ARM64 register setup
    - `macros.s` - ARM64 assembly macros
    - `uart.s` - UART driver (I/O example)
    - `timer.s` - Timer driver (interrupt example)
    - `irq.s` - IRQ handling
    - `mmu.s` - MMU setup
    - `spectrum4.ld` - Linker script example
  - Shows ARM64 calling conventions in practice
  - Shows exception handling patterns
  - Shows register save/restore patterns
- ✅ **Have:** FPC `aarch64/` backend (`freePascal/compiler/aarch64/`) - **PRIMARY REFERENCE**
- ✅ **Have:** ARM64 AAPCS64 standard (publicly available)
- ✅ **Have:** Standard Linux ABI documentation

**Action Items:**
1. ✅ Extract calling conventions from Spectrum4 `entry.s` (kernel_entry/kernel_exit macros)
2. ✅ Extract register usage patterns from kernel source
3. Extract calling conventions from FPC `aarch64/cpupara.pas`
4. Reference ARM64 AAPCS64 standard
5. Define SuperPascal-specific ARM64 ABI (may mirror FPC/Linux)
6. Document in `platforms/RaspberryPi5/ABI.md`

---

## 2. Intrinsics Specifications

### ✅ CommanderX16
**Status:** ✅ **Complete Documentation Available**  
**Needs:**
- VERA graphics intrinsics (sprite control, tilemaps, layers, scrolling)
- YM2151 audio intrinsics (FM synthesis, channels)
- VERA PSG intrinsics (16 channels, waveforms)
- VERA PCM intrinsics (audio playback)
- Memory banking intrinsics (bank switching)
- KERNAL function wrappers (console I/O, file I/O)

**SDK/Info Sources:**
- ✅ **Have:** Complete CommanderX16 SDK (`../CommandX16/`)
  - **x16-docs/X16 Reference - 09 - VERA Programmer's Reference.md**: Complete VERA register map and API
  - **x16-docs/X16 Reference - 11 - Sound Programming.md**: Complete YM2151 and PSG API documentation
  - **x16-docs/X16 Reference - 05 - KERNAL.md**: Complete KERNAL API with calling conventions
  - **x16-docs/X16 Reference - 08 - Memory Map.md**: Memory banking details
  - **x16-rom/kernal/**: KERNAL source code for reference
  - **x16-rom/graphics/**: Graphics routines source
  - **x16-rom/audio/**: Audio routines source
- ✅ **Have:** LLVM-MOS SDK (`mos-6502/llvm-mos-sdk/mos-platform/cx16/`)
  - KERNAL function wrappers (80+ files!)
  - Platform headers (`cx16.h`)

**Action Items:**
1. ✅ Review CommanderX16 VERA documentation (`x16-docs/X16 Reference - 09 - VERA Programmer's Reference.md`)
2. ✅ Review Sound Programming documentation (`x16-docs/X16 Reference - 11 - Sound Programming.md`)
3. ✅ Review LLVM-MOS SDK KERNAL wrappers
4. Define SuperPascal intrinsics API based on documented APIs
5. Document in `platforms/CommanderX16/intrinsics/`

---

### ✅ Foenix65C816
**Status:** ✅ **Complete SDK Available** - Can extract from source  
**Needs:**
- Complete VICKY intrinsics implementation (currently planned)
- Audio intrinsics (PSG, SID, OPL3, CODEC, etc.)
- I/O intrinsics (keyboard, mouse, joystick, serial, SD card)
- Memory intrinsics (MMU, banking)
- Game engine intrinsics

**SDK/Info Sources:**
- ✅ **Have:** Complete Foenix SDK (`../Foenix/`)
  - **f256-microkernel/** - MicroKernel with hardware definitions
    - `f256/TinyVicky_Def.asm` - **Complete VICKY register definitions**
    - `f256/audio.asm` - Audio chip definitions
    - `f256/interrupt_def.asm` - **Complete interrupt controller definitions**
    - `f256/keyboard.asm` - Keyboard handling
    - `f256/serial.asm` - Serial port handling
    - `f256/iec.asm` - IEC bus handling
    - `hardware/` - Hardware driver implementations
  - **BASIC816/** - BASIC implementation showing hardware usage
  - **f256-superbasic/** - Another BASIC implementation (reference)
  - **F256-FileManager/** - File manager showing I/O patterns
  - **FoenixIDE/Manuals/** - PDF documentation
    - `c256-foenix-devguide.pdf` - Developer guide
    - `C256_Foenix_Memory_Map_RevA.pdf` - Memory map
- ✅ **Have:** Foenix Hardware Wiki

**Action Items:**
1. ✅ Extract VICKY intrinsics from `TinyVicky_Def.asm`
2. ✅ Extract audio intrinsics from `audio.asm`
3. ✅ Extract interrupt intrinsics from `interrupt_def.asm`
4. ✅ Extract I/O intrinsics from MicroKernel drivers
5. Define SuperPascal intrinsics API based on MicroKernel definitions
6. Document in `platforms/Foenix65C816/intrinsics/`

---

### ✅ FoenixA2560M
**Status:** ✅ **System-Level Source Available** - Can extract patterns from Motorola code  
**Needs:**
- VICKY III Mark II intrinsics
- Audio intrinsics (OPM, OPN2, PSG, OPL3, SID, SAM2695, VS1053B)
- PCIe intrinsics
- USB intrinsics
- Ethernet/WiFi intrinsics
- Memory intrinsics (DDR3, Flash)
- System call intrinsics (if applicable)
- Interrupt intrinsics

**SDK/Info Sources:**
- ✅ **Have:** **Motorola MC68060 Source Code** (`../Motorolla/`)
  - **`amiga_mc68060sp/68060_lib/mc60_dispatcher.asm`** - Exception dispatcher showing interrupt handling patterns
  - **`defender/gemdos.S`** - System call interface showing parameter passing
  - **`defender/irq.c`** - Interrupt handling patterns
  - **`atari_public/`** - System programming examples
  - Shows system-level intrinsics patterns (interrupts, system calls, exceptions)
- ✅ **Have:** **Mikro Documentation Archive** (`SuperPascal/docs/mikro_docs_archive/`)
  - **`Optimize/68OPT.TXT`** - ⭐⭐⭐⭐⭐ **CRITICAL** - **Optimizing 680x0 Applications** (directly applicable to MC68LC060)
  - **`030/MOTOROLA.TXT`** - ⭐⭐⭐⭐⭐ **CRITICAL** - Motorola 68030 documentation (instruction set reference)
  - **`Optimize/OPTSRC.TXT`** - ⭐⭐⭐⭐ **VERY USEFUL** - Source code optimization with assembly examples
  - **`Optimize/MOVE16V2.ZIP`** - ⭐⭐⭐ **USEFUL** - Cache optimization (MOVE16 instruction)
  - Provides backend optimization knowledge for m68k code generator
- 🔍 **Need:** Foenix A2560M hardware documentation (VICKY III, audio chips, etc.)
- 🔍 **Need:** VICKY III Mark II register map
- 🔍 **Need:** Audio chip documentation
- 🔍 **Need:** PCIe/USB/Ethernet chip documentation

**Action Items:**
1. ✅ Extract system call patterns from `defender/gemdos.S`
2. ✅ Extract interrupt handling from `defender/irq.c` and `mc60_dispatcher.asm`
3. ✅ Extract exception handling from MC68060 Software Package
4. ✅ Study m68k optimization from `68OPT.TXT` (critical for code generator)
5. ✅ Study instruction set from `030/MOTOROLA.TXT`
6. Locate Foenix A2560M hardware documentation
7. Define VICKY III intrinsics (need Foenix hardware docs)
8. Define audio intrinsics (need Foenix hardware docs)
9. Define I/O intrinsics (need Foenix hardware docs)
10. Document in `platforms/FoenixA2560M/intrinsics/`

---

### ✅ Raspberry Pi 5
**Status:** ✅ **Bare-Metal Examples Available** - Can extract from source  
**Needs:**
- GPIO intrinsics (40-pin GPIO header)
- I2C intrinsics
- SPI intrinsics
- UART intrinsics
- PWM intrinsics
- VideoCore VII intrinsics (if needed)

**SDK/Info Sources:**
- ✅ **Have:** Complete bare-metal ARM64 examples (`../RaspberryPi/spectrum4/`)
  - **kernel/uart.s** - UART driver implementation
  - **kernel/timer.s** - Timer driver implementation
  - **kernel/irq.s** - IRQ handling implementation
  - Shows bare-metal I/O patterns
- ✅ **Have:** Standard Linux system calls (GPIO, I2C, SPI via `/dev/` or syscalls)
- ✅ **Have:** Raspberry Pi documentation
- 🔍 **Need:** Raspberry Pi 5 GPIO documentation
- 🔍 **Need:** VideoCore VII documentation (if needed)

**Action Items:**
1. ✅ Extract UART intrinsics from `uart.s`
2. ✅ Extract timer/interrupt intrinsics from `timer.s` and `irq.s`
3. Define GPIO/I2C/SPI intrinsics (Linux-based or bare-metal)
4. Document in `platforms/RaspberryPi5/intrinsics/`

---

## 3. Runtime Specifications

### ❌ CommanderX16
**Status:** Not yet specified  
**Needs:**
- KERNAL integration (system calls)
- Memory management (heap, stack)
- Exception handling (if applicable)
- Startup code (crt0)
- Library initialization

**SDK/Info Sources:**
- ✅ **Have:** LLVM-MOS SDK (`llvm-mos-sdk/mos-platform/cx16/`)
  - Linker scripts (`link.ld`)
  - Runtime initialization examples
- ✅ **Have:** CommanderX16 KERNAL documentation
- 🔍 **Need:** KERNAL API reference

**Action Items:**
1. Study LLVM-MOS SDK runtime patterns
2. Define SuperPascal runtime for CommanderX16
3. Document in `platforms/CommanderX16/runtime/`

---

### ❌ Foenix65C816
**Status:** Not yet specified  
**Needs:**
- MicroKernel integration (if applicable)
- Memory management (heap, stack, MMU)
- Exception handling
- Startup code
- Library initialization

**SDK/Info Sources:**
- ✅ **Have:** Foenix MicroKernel (`Foenix/f256-microkernel/`)
- 🔍 **Need:** MicroKernel API documentation

**Action Items:**
1. Study Foenix MicroKernel for runtime patterns
2. Define SuperPascal runtime for Foenix65C816
3. Document in `platforms/Foenix65C816/runtime/`

---

### ✅ FoenixA2560M
**Status:** ✅ **Startup & Exception Code Available** - Can extract from Motorola source  
**Needs:**
- OS integration (if applicable)
- Memory management (heap, stack, DDR3)
- Exception handling
- Startup code
- Library initialization

**SDK/Info Sources:**
- ✅ **Have:** **Motorola MC68060 Source Code** (`../Motorolla/`)
  - **`amiga_mc68060sp/68060_lib/mc60_startup.asm`** - ⭐⭐⭐⭐⭐ **CRITICAL** - **Startup code and initialization!**
  - **`amiga_mc68060sp/68060_lib/mc60_dispatcher.asm`** - ⭐⭐⭐⭐⭐ **CRITICAL** - **Exception dispatcher showing exception handling!**
  - **`amiga_mc68060sp/68060_lib/ispemu/isp.asm`** - Integer exception handler
  - **`defender/start.S`** - Startup code showing system initialization
  - **`defender/xcpt.S`** - Exception handling code
  - Shows real startup sequences, exception handling, memory management patterns
- ✅ **Have:** MC68060UM manual - Exception processing section (Section 8)
- 🔍 **Need:** Foenix A2560M OS/SDK documentation (if applicable)

**Action Items:**
1. ✅ Extract startup code from `mc60_startup.asm` and `defender/start.S`
2. ✅ Extract exception handling from `mc60_dispatcher.asm` and `defender/xcpt.S`
3. ✅ Study memory management patterns from Motorola source
4. Cross-reference with MC68060UM Section 8 (Exception Processing)
5. Define SuperPascal runtime for FoenixA2560M
6. Document in `platforms/FoenixA2560M/runtime/`

---

### ❌ Raspberry Pi 5
**Status:** Not yet specified  
**Needs:**
- Linux system call integration
- Memory management (heap, stack)
- Exception handling (Linux signals)
- Startup code (standard Linux crt0)
- Library initialization

**SDK/Info Sources:**
- ✅ **Have:** Standard Linux ABI documentation
- ✅ **Have:** FPC aarch64 runtime patterns

**Action Items:**
1. Define SuperPascal runtime for Linux/ARM64
2. Document in `platforms/RaspberryPi5/runtime/`

---

## 4. Standard Library Specifications

### ❌ CommanderX16
**Status:** Not yet specified  
**Needs:**
- System unit (basic I/O, memory)
- Graphics unit (VERA wrapper)
- Audio unit (YM2151, PSG, PCM wrapper)
- File I/O unit (SD card, IEC)
- Input unit (keyboard, joystick)

**SDK/Info Sources:**
- ✅ **Have:** CommanderX16 KERNAL API
- ✅ **Have:** LLVM-MOS SDK platform libraries

**Action Items:**
1. Define standard library units
2. Document in `platforms/CommanderX16/stdlib/`

---

### ❌ Foenix65C816
**Status:** Not yet specified  
**Needs:**
- System unit
- Graphics unit (VICKY wrapper)
- Audio unit (PSG, SID, OPL3 wrapper)
- File I/O unit (SD card)
- Input unit (keyboard, mouse, joystick)

**SDK/Info Sources:**
- ✅ **Have:** Foenix MicroKernel

**Action Items:**
1. Define standard library units
2. Document in `platforms/Foenix65C816/stdlib/`

---

### ✅ FoenixA2560M
**Status:** ✅ **System Call Wrappers + Algorithm Reference Available** - Can extract patterns from Motorola code + algorithm docs  
**Needs:**
- System unit
- Graphics unit (VICKY III wrapper)
- Audio unit (multiple chips)
- File I/O unit
- Network unit (Ethernet, WiFi)
- Math unit (fixed-point arithmetic - **CRITICAL for no-FPU system!**)

**SDK/Info Sources:**
- ✅ **Have:** **Motorola MC68060 Source Code** (`../Motorolla/`)
  - **`defender/gemdos.S`** - ⭐⭐⭐⭐⭐ **CRITICAL** - **System call wrappers showing parameter passing!**
    - `gemdos_cconout` - Console output
    - `gemdos_fcreate`, `gemdos_fopen`, `gemdos_fclose` - File operations
    - `gemdos_fread`, `gemdos_fwrite` - File I/O
    - Shows real system call interface patterns
  - **`atari_public/SCSI2PI/*.C`** - System programming examples
  - Shows how to wrap system calls, parameter passing, error handling
- ✅ **Have:** **Mikro Documentation Archive** (`SuperPascal/docs/mikro_docs_archive/`)
  - **`Coding/2/FIXEDPOI.TXT`** - ⭐⭐⭐⭐⭐ **CRITICAL** - **Fixed-point arithmetic guide (no FPU on MC68LC060!)**
  - **`Coding/1/SORT_ALG.TXT`** - ⭐⭐⭐ **USEFUL** - Sorting algorithms (includes Pascal examples)
  - **`Coding/1/CRC.TXT`** - ⭐⭐⭐ **USEFUL** - CRC algorithm reference
  - **`Coding/1/SQROOT.TXT`**, **`SQRT.ZIP`** - ⭐⭐⭐ **USEFUL** - Square root algorithms
  - **`Coding/2/SIN.TXT`**, **`SINCOSC.TXT`** - ⭐⭐⭐⭐ **VERY USEFUL** - Sine/cosine algorithms
  - Algorithm documentation for standard library implementation
- 🔍 **Need:** Foenix A2560M SDK/documentation (for VICKY, audio, etc.)

**Action Items:**
1. ✅ Extract system call wrapper patterns from `defender/gemdos.S`
2. ✅ Study parameter passing conventions
3. ✅ Study error handling patterns
4. ✅ Study fixed-point arithmetic from `FIXEDPOI.TXT` (critical for no-FPU)
5. ✅ Study math algorithms (sqrt, sin, cos) for stdlib implementation
6. Locate Foenix A2560M SDK/documentation
7. Define standard library units based on patterns + Foenix hardware + algorithm docs
8. Document in `platforms/FoenixA2560M/stdlib/`

---

### ❌ Raspberry Pi 5
**Status:** Not yet specified  
**Needs:**
- System unit (Linux system calls)
- GPIO unit
- I2C/SPI/UART units
- File I/O unit (standard Linux)
- Network unit (standard Linux)

**SDK/Info Sources:**
- ✅ **Have:** Standard Linux libraries

**Action Items:**
1. Define standard library units
2. Document in `platforms/RaspberryPi5/stdlib/`

---

## 5. Game Engine Specifications

### ✅ CommanderX16
**Status:** ✅ **Complete ECS Reference Available** - Can extract design patterns from Bevy  
**Needs:**
- Entity-Component-System (ECS) design
- Sprite management (VERA hardware sprites)
- Tilemap management (VERA tilemap layers)
- Collision detection
- Audio integration (YM2151, PSG)

**SDK/Info Sources:**
- ✅ **Have:** **Bevy ECS Game Engine** (`../ECS/bevy/`)
  - ⭐⭐⭐⭐⭐ **CRITICAL** - **Complete modern ECS implementation!**
  - **`crates/bevy_ecs/`** - Core ECS implementation
    - Shows Entity-Component-System architecture
    - Shows archetype-based storage patterns
    - Shows query system patterns
    - Shows system scheduling patterns
  - **1,613 Rust source files** - Comprehensive implementation
  - **Note:** Modern design (not retro prior art), but provides excellent patterns for retro-adapted ECS
- ✅ **Have:** CommanderX16 VERA documentation (for sprite/tilemap hardware)
- 🔍 **Need:** Adapt Bevy patterns to 65C02 constraints (memory, performance)

**Action Items:**
1. ✅ Study Bevy ECS core implementation (`bevy_ecs/`)
2. Design SuperPascal ECS API (adapt Bevy patterns to Pascal)
3. Adapt for 65C02 constraints (memory efficiency, 8-bit addressing)
4. Integrate with VERA (CommanderX16 hardware)
5. Document in `platforms/CommanderX16/gameEngine/`

---

### ✅ Foenix65C816
**Status:** ✅ **Complete ECS Reference Available** - Can extract design patterns from Bevy  
**Needs:**
- ECS design
- Sprite management (VICKY sprites)
- Tilemap management (VICKY tilemaps)
- Collision detection
- Audio integration

**SDK/Info Sources:**
- ✅ **Have:** **Bevy ECS Game Engine** (`../ECS/bevy/`)
  - ⭐⭐⭐⭐⭐ **CRITICAL** - **Complete modern ECS implementation!**
  - **`crates/bevy_ecs/`** - Core ECS implementation
    - Shows Entity-Component-System architecture
    - Shows archetype-based storage patterns
    - Shows query system patterns
    - Shows system scheduling patterns
  - **1,613 Rust source files** - Comprehensive implementation
  - **Note:** Modern design (not retro prior art), but provides excellent patterns for retro-adapted ECS
- ✅ **Have:** Foenix VICKY documentation
- 🔍 **Need:** Adapt Bevy patterns to 65C816 constraints (memory, performance)

**Action Items:**
1. ✅ Study Bevy ECS core implementation (`bevy_ecs/`)
2. Design SuperPascal ECS API (adapt Bevy patterns to Pascal)
3. Adapt for 65C816 constraints (memory efficiency, 24-bit addressing)
4. Integrate with VICKY (Foenix hardware)
5. Document in `platforms/Foenix65C816/gameEngine/`

---

### ✅ FoenixA2560M
**Status:** ✅ **Complete ECS + Comprehensive Algorithm Reference Available** - Can extract design patterns from Bevy + algorithm docs  
**Needs:**
- ECS design
- Sprite management (VICKY III)
- Tilemap management
- Collision detection
- Audio integration
- Rendering algorithms (polygons, textures, shading)
- Math algorithms (matrices, fixed-point, trigonometry)

**SDK/Info Sources:**
- ✅ **Have:** **Bevy ECS Game Engine** (`../ECS/bevy/`)
  - ⭐⭐⭐⭐⭐ **CRITICAL** - **Complete modern ECS implementation!**
  - **`crates/bevy_ecs/`** - Core ECS implementation
    - `world/` - World management, entity storage
    - `entity/` - Entity ID management
    - `component/` - Component storage and access
    - `system/` - System definition and execution
    - `query/` - Query system for selecting entities
    - `archetype/` - Archetype-based storage (performance optimization)
    - `schedule/` - System scheduling and execution order
  - **1,613 Rust source files** - Comprehensive implementation
  - Shows modern ECS patterns (Entity-Component-System architecture)
  - **Note:** This is modern design (not retro prior art), but provides excellent patterns for retro-adapted ECS
- ✅ **Have:** **Mikro Documentation Archive** (`SuperPascal/docs/mikro_docs_archive/`)
  - ⭐⭐⭐⭐⭐ **CRITICAL** - **Comprehensive game development algorithm documentation!**
  - **`Coding/1/OTMPOLY.TXT`** - ⭐⭐⭐⭐⭐ **MUST READ** - Complete polygon rendering guide
  - **`Coding/1/CAM_MATR.TXT`** - ⭐⭐⭐⭐⭐ **MUST READ** - Camera mathematics
  - **`Coding/1/OTMMATX.TXT`** - ⭐⭐⭐⭐⭐ **MUST READ** - Matrix transformation tutorial
  - **`Coding/2/FIXEDPOI.TXT`** - ⭐⭐⭐⭐⭐ **MUST READ** - Fixed-point arithmetic (no FPU on MC68LC060!)
  - **`Coding/1/TEXTURE.TXT`** - ⭐⭐⭐⭐⭐ **MUST READ** - Texture mapping algorithms
  - **`Coding/2/TILEMAP.TXT`** - ⭐⭐⭐⭐⭐ **MUST READ** - Tilemap algorithms
  - **`Coding/2/CLIP.TXT`** - ⭐⭐⭐⭐ **VERY USEFUL** - Polygon clipping (collision detection)
  - **`Coding/1/BSPTREE.TXT`** - ⭐⭐⭐⭐ **VERY USEFUL** - BSP trees (spatial algorithms)
  - **`Coding/1/3DSHADE.TXT`** - ⭐⭐⭐⭐ **VERY USEFUL** - 3D shading algorithms
  - **`Coding/1/OTMPHONG.TXT`** - ⭐⭐⭐⭐ **VERY USEFUL** - Phong shading
  - **`Coding/3/PARTICLE.TXT`** - ⭐⭐⭐⭐ **VERY USEFUL** - Particle systems
  - **`Coding/2/GRAVITY.TXT`** - ⭐⭐⭐ **USEFUL** - Gravity simulation
  - **210+ text files** with algorithm documentation
- ✅ **Have:** **Motorola MC68060 Source Code** (`../Motorolla/`)
  - **`defender/`** - ⭐⭐⭐ **USEFUL** - Complete game showing:
    - Game loop patterns (`main.c`)
    - Object management (`obj.c`)
    - Collision detection (`collide.c`)
    - Graphics rendering (`screen.c`, `plot.S`)
    - Audio integration (`sound.c`, `sound_dma.c`)
    - **Note:** This is a game, not a game engine framework, but shows patterns
- 🔍 **Need:** Foenix A2560M SDK/documentation (for VICKY III specifics)

**Action Items:**
1. ✅ Study Bevy ECS core implementation (`bevy_ecs/`)
2. ✅ Study game patterns from `defender/` (game loop, object management, collision)
3. ✅ Study polygon rendering from `OTMPOLY.TXT`
4. ✅ Study fixed-point math from `FIXEDPOI.TXT` (critical for no-FPU)
5. ✅ Study texture mapping from `TEXTURE.TXT`
6. ✅ Study tilemap algorithms from `TILEMAP.TXT`
7. Design SuperPascal ECS API (adapt Bevy patterns to Pascal)
8. Adapt for retro constraints (memory efficiency, simplicity)
9. Integrate with VICKY III (Foenix hardware)
10. Document in `platforms/FoenixA2560M/gameEngine/`

---

## 6. Object File Formats

### ❌ CommanderX16 (.X16)
**Status:** Not yet specified  
**Needs:**
- Object file format specification
- Symbol table format
- Relocation format
- Section definitions
- Linker script format

**SDK/Info Sources:**
- ✅ **Have:** LLVM-MOS SDK (`llvm-mos-sdk/mos-platform/cx16/link.ld`)
- 🔍 **Need:** CommanderX16 object format documentation (if any)

**Action Items:**
1. Define `.X16` object format
2. Create linker specification
3. Document format

---

### ❌ Foenix65C816 (.FOF)
**Status:** Not yet specified  
**Needs:**
- Object file format specification
- Symbol table format
- Relocation format (24-bit addressing)
- Section definitions
- Linker script format

**SDK/Info Sources:**
- 🔍 **Need:** Foenix object format documentation (if any)
- 🔍 **Need:** Study LLVM-MOS 65C816 object format

**Action Items:**
1. Define `.FOF` object format
2. Create linker specification
3. Document format

---

### ✅ FoenixA2560M (.AOF)
**Status:** ✅ **Linker Script Example Available** - Can extract patterns from Motorola code  
**Needs:**
- Object file format specification
- Symbol table format
- Relocation format (32-bit addressing)
- Section definitions
- Linker script format

**SDK/Info Sources:**
- ✅ **Have:** **Motorola MC68060 Source Code** (`../Motorolla/`)
  - **`defender/defender.lds`** - ⭐⭐⭐⭐ **VERY USEFUL** - **Linker script example!**
    - Shows section definitions (.text, .data, .bss)
    - Shows memory layout
    - Shows stack allocation (40KB)
    - Shows alignment requirements (4-byte alignment)
    - Shows symbol definitions (`_data`, `_edata`, `_bss`, `_ebss`, `_end`)
  - **`amiga_mc68060sp/68060_lib/*.ld`** - Library linker scripts
  - Shows real linker script patterns for m68k
- ✅ **Have:** FPC m68k object format (can reference)
- ✅ **Have:** Standard m68k object formats (ELF, a.out, COFF) - Can use standard formats
- 🔍 **Need:** Foenix-specific object format requirements (if any)

**Action Items:**
1. ✅ Extract linker script patterns from `defender/defender.lds`
2. ✅ Study section layout from Motorola examples
3. Choose standard object format (ELF recommended for m68k)
4. Define `.AOF` object format (or use standard ELF)
5. Create linker specification
6. Document format

---

### ❌ Raspberry Pi 5 (ELF64)
**Status:** Not yet specified  
**Needs:**
- ELF64 format specification (standard, but needs documentation)
- Linker script format
- Section definitions

**SDK/Info Sources:**
- ✅ **Have:** Standard ELF64 specification
- ✅ **Have:** FPC aarch64 object format

**Action Items:**
1. Document ELF64 usage for SuperPascal
2. Create linker specification
3. Document format

---

## 7. Linker Specifications

### ❌ CommanderX16
**Needs:**
- Linker script format
- Memory map definition
- Section placement
- Bank switching support

**SDK/Info Sources:**
- ✅ **Have:** LLVM-MOS SDK (`llvm-mos-sdk/mos-platform/cx16/link.ld`)
- ✅ **Have:** `imag-regs.ld` (zero-page allocation)

**Action Items:**
1. Study LLVM-MOS linker scripts
2. Define SuperPascal linker specification
3. Document in `platforms/CommanderX16/`

---

### ❌ Foenix65C816
**Needs:**
- Linker script format
- Memory map definition (24-bit addressing)
- Section placement
- MMU support

**SDK/Info Sources:**
- 🔍 **Need:** Foenix linker documentation
- 🔍 **Need:** Study LLVM-MOS 65C816 linker patterns

**Action Items:**
1. Define SuperPascal linker specification
2. Document in `platforms/Foenix65C816/`

---

### ✅ FoenixA2560M
**Status:** ✅ **Linker Script Example Available** - Can extract patterns from Motorola code  
**Needs:**
- Linker script format
- Memory map definition (32-bit addressing)
- Section placement
- DDR3/Flash mapping

**SDK/Info Sources:**
- ✅ **Have:** **Motorola MC68060 Source Code** (`../Motorolla/`)
  - **`defender/defender.lds`** - ⭐⭐⭐⭐ **VERY USEFUL** - **Complete linker script example!**
    - Shows `.text`, `.data`, `.bss` section definitions
    - Shows memory layout (stack allocation: 40KB)
    - Shows alignment requirements (4-byte alignment)
    - Shows symbol definitions (`_data`, `_edata`, `_bss`, `_ebss`, `_end`)
  - **`amiga_mc68060sp/68060_lib/*.ld`** - Library linker scripts
  - Shows real linker script patterns for m68k
- ✅ **Have:** MC68060UM manual - Memory management section
- 🔍 **Need:** Foenix-specific memory map (if different from standard)

**Action Items:**
1. ✅ Extract linker script patterns from `defender/defender.lds`
2. ✅ Study section layout and memory allocation
3. ✅ Study symbol definitions
4. Cross-reference with MC68060UM memory management
5. Define SuperPascal linker specification
6. Document in `platforms/FoenixA2560M/`

---

### ❌ Raspberry Pi 5
**Needs:**
- Linker script format (standard Linux)
- Memory map definition
- Section placement

**SDK/Info Sources:**
- ✅ **Have:** Standard Linux linker scripts

**Action Items:**
1. Define SuperPascal linker specification
2. Document in `platforms/RaspberryPi5/`

---

## 8. Priority Areas for SDK/Information Gathering

### High Priority (Needed for Implementation)

1. **CommanderX16 ABI** - Need 65C02 calling conventions
   - ✅ **COMPLETE:** Have complete SDK with KERNAL source code and documentation
   - ✅ Have: LLVM-MOS SDK, CommanderX16 docs
   - ✅ Have: KERNAL source code (`x16-rom/kernal/`) - can extract calling conventions
   - ✅ Have: KERNAL API documentation (`x16-docs/X16 Reference - 05 - KERNAL.md`) - calling conventions documented
   - **Action:** Extract calling conventions from KERNAL source and documentation

2. **Foenix65C816 ABI** - Need 65C816 calling conventions
   - ✅ **COMPLETE:** Have complete SDK with BASIC816 calling convention macros!
   - ✅ Have: BASIC816 `macros.s` with ENTER/LEAVE macros showing stack frame layout
   - ✅ Have: Foenix MicroKernel assembly code
   - ✅ Have: FoenixIDE manuals (65816 Programmanual.pdf)
   - ✅ Have: LLVM-MOS 65C816 support
   - **Action:** Extract calling conventions from BASIC816 macros and MicroKernel

3. **FoenixA2560M ABI** - Need m68k calling conventions
   - ✅ Have: FPC m68k backend
   - 🔍 Need: Extract from FPC source

4. **Raspberry Pi 5 ABI** - Need ARM64 AAPCS64
   - ✅ **COMPLETE:** Have bare-metal ARM64 examples with calling convention patterns!
   - ✅ Have: Spectrum4 `entry.s` with `kernel_entry`/`kernel_exit` macros showing register usage
   - ✅ Have: FPC aarch64 backend, ARM64 AAPCS64 standard
   - **Action:** Extract calling conventions from Spectrum4 macros and FPC source

### Medium Priority (Needed for Feature Completion)

5. **CommanderX16 Intrinsics** - VERA, YM2151
   - ✅ **COMPLETE:** Have complete documentation and source code
   - ✅ Have: Complete VERA documentation (`x16-docs/X16 Reference - 09 - VERA Programmer's Reference.md`)
   - ✅ Have: Complete audio documentation (`x16-docs/X16 Reference - 11 - Sound Programming.md`)
   - ✅ Have: Graphics source code (`x16-rom/graphics/`)
   - ✅ Have: Audio source code (`x16-rom/audio/`)
   - ✅ Have: LLVM-MOS SDK KERNAL wrappers
   - **Action:** Design SuperPascal intrinsics API based on documented APIs

6. **Foenix65C816 Intrinsics** - Complete VICKY, audio
   - ✅ **COMPLETE:** Have complete MicroKernel with all hardware definitions
   - ✅ Have: Complete VICKY definitions (`TinyVicky_Def.asm`)
   - ✅ Have: Complete interrupt definitions (`interrupt_def.asm`)
   - ✅ Have: Audio definitions (`audio.asm`)
   - ✅ Have: Hardware driver implementations
   - **Action:** Extract intrinsics from MicroKernel definitions

7. **FoenixA2560M SDK** - Complete platform documentation
   - 🔍 Need: Locate Foenix A2560M SDK/documentation

### Lower Priority (Can be done later)

8. **Runtime Specifications** - All platforms
9. **Standard Library** - All platforms
10. **Game Engine** - All platforms (except ZealZ80 ✅)
11. **Object File Formats** - All platforms

---

## Summary: What You Can Help Find

### Immediate Needs (SDKs/Documentation)

1. **Foenix A2560M SDK/Documentation**
   - Hardware register maps
   - VICKY III Mark II documentation
   - Audio chip documentation
   - OS/SDK documentation (if any)

2. **65C02/65C816 Calling Convention Documentation**
   - WDC datasheets (may have calling convention info)
   - Other 6502/65C816 compiler documentation

3. **Foenix Platform Linker Documentation**
   - Linker script examples
   - Memory map definitions

### Reference Materials (Nice to Have)

4. **Motorola 68K Programmer's Reference**
   - For FoenixA2560M ABI reference

5. **Raspberry Pi 5 GPIO/I2C/SPI Documentation**
   - For intrinsics implementation

6. **Game Engine Design Patterns**
   - For retro computing platforms

---

**Next Steps:**
1. Prioritize ABI specifications (needed for compiler implementation)
2. Gather missing SDKs/documentation
3. Extract information from existing sources (FPC, LLVM-MOS, MicroKernel)
4. Document findings in platform-specific directories

