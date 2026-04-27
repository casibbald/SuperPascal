# External Resources Reference

## Overview

This document catalogs external resources (websites, archives, documentation) that are useful for SuperPascal compiler and platform development.

**Last Updated:** 2025-01-XX

---

## Game Engine Resources

### Bevy ECS Game Engine

**Local Path:** `../ECS/bevy/`

**Description:** Complete Bevy ECS game engine source code (1,613 Rust source files). Modern, data-driven game engine built in Rust using an ECS (Entity Component System) architecture.

**Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL for SuperPascal Game Engine Design**

#### Key Characteristics
- **ECS Architecture:** Entity-Component-System pattern
- **Data-Driven:** Systems operate on components, not objects
- **Performance:** Optimized for cache-friendly data access
- **Modular:** Composed of many crates
- **Production-Tested:** Real-world game engine

#### Relevance for SuperPascal

**For Game Engine Design:**
- Complete ECS reference implementation
- Shows modern ECS patterns (Entity-Component-System architecture)
- Shows archetype-based storage (performance optimization)
- Shows query system patterns
- Shows system scheduling patterns
- **Note:** This is modern design (not retro prior art), but provides excellent patterns for retro-adapted ECS

**Key Files to Study:**
- `crates/bevy_ecs/src/world/mod.rs` - World management
- `crates/bevy_ecs/src/query/mod.rs` - Query system
- `crates/bevy_ecs/src/system/mod.rs` - System definition
- `crates/bevy_ecs/src/archetype/mod.rs` - Archetype storage
- `crates/bevy_ecs/src/entity/mod.rs` - Entity management
- `crates/bevy_app/src/lib.rs` - Application framework

**Action Items:**
1. Study Bevy ECS core implementation
2. Design SuperPascal ECS API (adapt Bevy patterns to Pascal)
3. Adapt for retro constraints (memory efficiency, simplicity)
4. Integrate with platform-specific hardware (VICKY, VERA, etc.)

**See Also:**
- `docs/BEVY_ECS_REFERENCE.md` - Detailed Bevy ECS reference guide

---

## Motorola 680x0 Resources

### Atari Demoscene Documentation Archive

**URL:** [https://mikro.naprvyraz.sk/docs/](https://mikro.naprvyraz.sk/docs/)  
**Local Archive:** `SuperPascal/docs/mikro_docs_archive/`

**Description:** Comprehensive archive of Atari ST/Falcon demoscene coding documentation, including extensive Motorola 680x0 processor resources, game development algorithms, and optimization guides.

**Relevance:** ⭐⭐⭐⭐ **VERY USEFUL for FoenixA2560M (MC68LC060)**

#### Key Sections for SuperPascal

##### 68030 Stuff
- **`MOTOROLA.TXT`** - Motorola 68030 documentation
  - **Author:** Zynx/Inferiors
  - **Comment:** "a MUST for Falcon coders!"
  - **Relevance:** Processor architecture details, register usage, instruction set

- **`PIPELINE.TXT`** - Pipelining on 030
  - **Author:** Evil/DHS
  - **Relevance:** Understanding 68030 pipeline behavior (applicable to 68060)

##### Optimizing 680x0 Code
- **`68OPT.TXT`** - Optimizing 680x0 Applications
  - **Author:** Tomas Evensen
  - **Comment:** "various optimizing methods for 68000/030/040/060"
  - **Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL** - Optimization techniques directly applicable to MC68LC060

- **`OPTIMIZE.TXT`** - Nice optimizing tricks
  - **Relevance:** General optimization patterns

- **`OPTSRC.TXT`** - Optimizing your source code
  - **Author:** Stefan Posthuma
  - **Comment:** "dirty optimizing on pure 68000"
  - **Relevance:** Low-level optimization techniques

- **`MOVE16V2.ZIP`** - Move16 instruction
  - **Comment:** "Illustrate using of 040's Move16"
  - **Relevance:** Cache optimization techniques (68040/060 have similar features)

##### 68881/2 FPU Stuff
- **`68881_1.TXT`** - Cooperating with the Coprocessor
  - **Author:** Jim Reekes
  - **Relevance:** FPU programming patterns (note: MC68LC060 has no FPU, but useful for understanding coprocessor architecture)

- **`68881_2.TXT`** - Speedy the Math Coprocessor
  - **Author:** Rich Collyer
  - **Relevance:** FPU optimization techniques

- **`881_ASM.TXT`** - FPU assembler programming
  - **Author:** Erik H. Bakke
  - **Relevance:** Assembly-level FPU programming

- **`MACRO881.ZIP`** - MC68881 as a peripheral
  - **Author:** Motorola
  - **Relevance:** Official Motorola documentation

#### Other Relevant Sections

##### ST/Falcon Memory Maps
- **`FALC1.TXT`, `FALC2.TXT`, `FALC3.TXT`** - Atari Falcon Hardware Registers
  - **Author:** Chris/Aura & Aeon/Aura
  - **Relevance:** Hardware register documentation (may contain processor-specific details)

- **`FALVIDEO.TXT`** - The Authoritative Guide To The Falcon Video Hardware
  - **Author:** Aura & Animal Mine
  - **Comment:** "the coolest VIDEL description on the world!"
  - **Relevance:** Video hardware (not directly relevant to Foenix, but shows hardware programming patterns)

##### Coding Documentation
- Various 3D graphics, math, and algorithm tutorials
- **Relevance:** General programming techniques applicable to game engine development

#### Usage Notes

1. **Processor Compatibility:** While this archive focuses on Atari ST/Falcon (68030/040), the optimization techniques and processor knowledge are directly applicable to FoenixA2560M (MC68LC060).

2. **Optimization Focus:** The `68OPT.TXT` document is particularly valuable as it covers optimization techniques for the entire 680x0 family, including the 68060.

3. **Code Generation:** These resources can inform:
   - Register allocation strategies
   - Instruction selection patterns
   - Pipeline optimization
   - Cache optimization

4. **ABI Reference:** While not explicitly about calling conventions, the optimization guides may show register usage patterns that inform ABI design.

#### Action Items

1. ✅ Document this resource in `MISSING_AREAS_AND_SDK_NEEDS.md`
2. 🔄 Download/archive key documents (`68OPT.TXT`, `MOTOROLA.TXT`, `PIPELINE.TXT`)
3. 🔄 Extract optimization patterns for code generator
4. 🔄 Cross-reference with MC68060UM manual

---

## LLVM-MOS Resources

### LLVM-MOS Project

**URL:** [https://llvm-mos.org/](https://llvm-mos.org/)

**Description:** Production-ready LLVM backend for 6502/65C02/65C816 targets.

**Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL for CommanderX16 and Foenix65C816**

**Status:** ✅ Already documented in `LLVM_BACKEND_PATH.md` and `LLVM_MOS_SDK_REFERENCE.md`

**Local SDK Location:** `../mos-6502/`

---

## FreePascal Resources

### FreePascal Source Code

**Local Location:** `../freePascal/`

**Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL** - Primary reference for Pascal semantics and backend implementations

**Status:** ✅ Already documented in:
- `FPC_INTEGRATION_SUMMARY.md`
- `FPC_SOURCE_REFERENCE_GUIDE.md`
- `FPC_BACKEND_COVERAGE.md`
- `FPC_Z80_BACKEND_REFERENCE.md`

---

## Commander X16 Resources

### Commander X16 Documentation

**Local Location:** `../CommandX16/`

**Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL for CommanderX16 platform**

**Status:** ✅ Already documented in `platforms/CommanderX16/README.md`

---

## Foenix Resources

### Foenix SDKs and Documentation

**Local Location:** `../Foenix/`

**Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL for Foenix65C816 and FoenixA2560M**

**Status:** ✅ Already documented in `MISSING_AREAS_AND_SDK_NEEDS.md`

---

## Motorola Source Code Resources

### Motorola 680x0 Source Code Collection

**Local Location:** `../Motorolla/`

**Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL for FoenixA2560M (MC68LC060)**

**Status:** ✅ Documented in `MOTOROLA_SOURCE_CODE_REFERENCE.md`

**Key Components:**
- **MC68060 Software Package** - Official Motorola code
- **Defender Game** - Real-world m68k programming
- **Atari Public Code** - Public domain examples
- **Hatari/ARAnyM** - Emulator code

See `MOTOROLA_SOURCE_CODE_REFERENCE.md` for detailed analysis.

---

## Raspberry Pi Resources

### Spectrum +4 Project (Bare-Metal ARM64)

**Local Location:** `../RaspberryPi/spectrum4/`

**Relevance:** ⭐⭐⭐⭐ **VERY USEFUL for Raspberry Pi 5**

**Status:** ✅ Already documented in `platforms/RaspberryPi5/README.md`

---

## Compiler Construction Resources

### Wirth's Compiler Construction Books

**Local Location:** `SuperPascal/docs/PDFs/`

**Relevance:** ⭐⭐⭐⭐⭐ **CRITICAL**

**Status:** ✅ Already documented in:
- `WIRTH_BOOK_REFERENCE.md`
- `MARKDOWN_ANALYSIS.md`
- `PDF_ANALYSIS.md`

---

## Summary

### Resources by Platform

| Platform | Primary Resources | Status |
|----------|------------------|--------|
| **ZealZ80** | Zeal SDK, FPC Z80 backend | ✅ Complete |
| **CommanderX16** | X16 SDK, LLVM-MOS | ✅ Complete |
| **Foenix65C816** | Foenix SDK, LLVM-MOS | ✅ Complete |
| **FoenixA2560M** | MC68060UM, FPC m68k backend, **Atari Archive** | ✅ **Enhanced** |
| **Raspberry Pi 5** | Spectrum4, FPC aarch64 backend | ✅ Complete |

### New Resources Added

1. ✅ **Atari Demoscene Documentation Archive** - Motorola 680x0 optimization resources
   - Added to `MISSING_AREAS_AND_SDK_NEEDS.md`
   - Added to `MARKDOWN_ANALYSIS.md`
   - Created `EXTERNAL_RESOURCES.md` (this document)

---

**Next Steps:**
1. 🔄 Download key documents from Atari archive (`68OPT.TXT`, `MOTOROLA.TXT`)
2. 🔄 Extract optimization patterns for m68k code generator
3. 🔄 Cross-reference optimization techniques with MC68060UM manual

