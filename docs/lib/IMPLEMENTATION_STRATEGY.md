# Standard Library Implementation Strategy

**Status:** Draft  
**Last Updated:** 2025-01-XX

---

## Overview

This document outlines the implementation strategy for the SuperPascal Standard Library, addressing the question: **Should we implement libraries in Pascal or Assembly?**

---

## Core Principle: Pascal First, Assembly When Needed

### Primary Approach: Pure Pascal

**All libraries should be implemented in SuperPascal first** for the following reasons:

1. **Portability:** Pascal code works across all platforms (Z80, 65C816, 68060, ARM64)
2. **Maintainability:** Easier to read, debug, and modify
3. **Compiler Optimization:** The SuperPascal compiler can optimize Pascal code
4. **Type Safety:** Pascal's type system catches errors at compile time
5. **Rapid Development:** Faster to implement and test

### When to Use Assembly

Assembly should be used **only** for:

1. **Performance-Critical Hot Paths:** Functions called millions of times per frame
2. **Hardware-Specific Operations:** Intrinsics that cannot be expressed in Pascal
3. **Platform-Specific Optimizations:** When compiler cannot generate optimal code
4. **Proven Bottlenecks:** After profiling shows Pascal is too slow

---

## Implementation Tiers

### Tier 1: Pure Pascal (Default)

**All libraries start here.**

- ✅ **Math library:** Pure Pascal (fixed-point, matrix, trig, vector)
- ✅ **Graphics library:** Pure Pascal (Bresenham, polygon, texture)
- ✅ **Collision library:** Pure Pascal (AABB, circle, polygon)
- ✅ **Physics library:** Pure Pascal (gravity, bounce, fragmentation)
- ✅ **Sorting library:** Pure Pascal (quicksort, mergesort, etc.)
- ✅ **Game library:** Pure Pascal (camera, BSP, LOS, pathfinding)
- ✅ **Testing library:** Pure Pascal (test framework)
- ⚠️ **Compression library:** Pure Pascal (LZ77, RLE, Huffman)
- ⚠️ **Crypto library:** Pure Pascal (CRC checksums)

**Rationale:** These algorithms are well-understood and can be optimized by the compiler. Pascal implementations are:
- Portable across all platforms
- Easy to maintain and debug
- Sufficiently fast for most use cases

---

### Tier 2: Pascal with Inline Assembly (Performance Hot Paths)

**Add inline assembly only after profiling identifies bottlenecks.**

#### Candidates for Inline Assembly:

1. **Fixed-Point Multiplication/Division** (`Math_Fixed`)
   - **Why:** Called millions of times per frame in games
   - **Platforms:** Z80, 65C816 (8/16-bit platforms benefit most)
   - **Approach:** Inline assembly for `Fixed16Mul`, `Fixed16Div` on retro platforms

2. **Matrix Multiplication** (`Math_Matrix`)
   - **Why:** Core 3D math operation, called frequently
   - **Platforms:** All (especially 68060 with MOVE16 instruction)
   - **Approach:** Platform-specific optimized versions

3. **Bresenham Line Drawing** (`Graphics_Line`)
   - **Why:** Core graphics primitive, called per-pixel
   - **Platforms:** Z80, 65C816 (8/16-bit platforms)
   - **Approach:** Inline assembly for inner loop

4. **CRC Lookup Table Access** (`Crypto_CRC`)
   - **Why:** Table-driven CRC is already fast, but inner loop could be optimized
   - **Platforms:** All (if profiling shows need)
   - **Approach:** Inline assembly for table lookup + XOR

5. **Sorting Inner Loops** (`Sorting_*`)
   - **Why:** Comparison and swap operations in tight loops
   - **Platforms:** Z80, 65C816 (if sorting large arrays)
   - **Approach:** Inline assembly for swap operations

---

### Tier 3: Platform-Specific Assembly Modules

**Separate assembly files for platform-specific optimizations.**

#### Structure:

```
lib/math/
├── fixed.pas           # Pure Pascal implementation (default)
├── fixed_z80.asm       # Z80-optimized assembly (optional)
├── fixed_65c816.asm    # 65C816-optimized assembly (optional)
├── fixed_68060.asm     # 68060-optimized assembly (optional)
└── fixed_arm64.asm     # ARM64-optimized assembly (optional)
```

#### When to Create:

- **After profiling:** Only if Pascal version is proven too slow
- **Platform-specific instructions:** MOVE16 (68060), SIMD (ARM64)
- **Hardware acceleration:** DSP (Foenix), VERA (Commander X16)

---

## Prioritization Strategy

### Phase 1: Complete All Libraries in Pascal ✅ (Current)

**Goal:** Get all libraries working in pure Pascal first.

**Remaining Libraries:**
1. **Compression** (LZ77, RLE, Huffman) - Medium priority
2. **Crypto** (CRC checksums) - Low priority (if needed)

**Rationale:** 
- Complete the standard library API
- Ensure all algorithms work correctly
- Establish baseline performance

---

### Phase 2: Profile and Optimize (After Compiler Exists)

**Goal:** Identify bottlenecks and optimize hot paths.

**Process:**
1. **Profile:** Run real-world benchmarks on each platform
2. **Identify:** Find functions called >1000 times per frame
3. **Optimize:** Add inline assembly only where needed
4. **Verify:** Ensure optimization actually improves performance

**Tools Needed:**
- Profiler for each target platform
- Benchmark suite
- Performance regression tests

---

### Phase 3: Platform-Specific Optimizations

**Goal:** Leverage platform-specific features.

**Examples:**
- **68060:** Use MOVE16 instruction for memory copies
- **ARM64:** Use SIMD for vector operations
- **Z80:** Use register pairs for 16-bit operations
- **65C816:** Use native mode for 16-bit operations

---

## Assembly Implementation Guidelines

### When Writing Assembly:

1. **Start with Pascal:** Always have a working Pascal version first
2. **Profile First:** Don't optimize until you've measured
3. **Platform-Specific:** Each platform gets its own assembly file
4. **Fallback:** Always provide Pascal fallback
5. **Document:** Clearly document why assembly is needed
6. **Test:** Verify assembly produces same results as Pascal

### Assembly File Naming:

```
{Platform}_{Module}_{Function}.asm
```

Examples:
- `z80_math_fixed_mul.asm` - Z80 fixed-point multiply
- `68060_graphics_line_bresenham.asm` - 68060 Bresenham line
- `arm64_math_matrix_mul.asm` - ARM64 matrix multiply

### Integration Pattern:

```pascal
unit Math_Fixed;

interface

function Fixed16Mul(a, b: Fixed16): Fixed16;

implementation

{$IFDEF CPU_Z80}
  {$I z80_fixed_mul.inc}  // Include Z80 assembly
{$ELSEIF DEFINED(CPU_65C816)}
  {$I 65c816_fixed_mul.inc}  // Include 65C816 assembly
{$ELSE}
  // Pure Pascal fallback
  function Fixed16Mul(a, b: Fixed16): Fixed16;
  begin
    Result := (LongInt(a) * LongInt(b)) shr 8;
  end;
{$ENDIF}

end.
```

---

## Performance Targets

### Acceptable Performance (Pascal is Fine):

- **Math operations:** < 100 cycles per operation (fixed-point)
- **Graphics primitives:** < 1000 cycles per primitive (line, circle)
- **Collision detection:** < 100 cycles per check (AABB, circle)
- **Sorting:** O(n log n) with reasonable constant factor
- **CRC calculation:** < 10 cycles per byte (table-driven)

### Needs Optimization (Consider Assembly):

- **Math operations:** > 1000 cycles per operation
- **Graphics primitives:** > 10000 cycles per primitive
- **Inner loops:** Called > 10000 times per frame
- **Memory bandwidth:** > 50% of frame time spent in memory operations

---

## Platform-Specific Considerations

### Z80 (8-bit, 10 MHz)

**Characteristics:**
- Limited registers (A, B, C, D, E, H, L)
- 16-bit register pairs (BC, DE, HL)
- No hardware multiply/divide
- Memory-mapped I/O

**Assembly Candidates:**
- Fixed-point multiply (uses 16-bit register pairs)
- Memory copies (LDIR instruction)
- Bit manipulation (BIT, SET, RES instructions)

### 65C816 (16-bit, 6.29 MHz)

**Characteristics:**
- 16-bit native mode
- 8-bit emulation mode
- Indexed addressing modes
- No hardware multiply (but faster than Z80)

**Assembly Candidates:**
- Fixed-point operations (native 16-bit)
- Array indexing (indexed addressing)
- Memory operations (block move instructions)

### 68060 (32-bit, 66 MHz, no FPU)

**Characteristics:**
- 32-bit registers
- MOVE16 instruction (16-byte aligned moves)
- No FPU (MC68LC060)
- Cache-friendly architecture

**Assembly Candidates:**
- Matrix operations (MOVE16 for 4x4 matrices)
- Memory copies (MOVE16 for aligned data)
- Fixed-point operations (32-bit registers)

### ARM64 (64-bit, 2.4 GHz, FPU + SIMD)

**Characteristics:**
- 64-bit registers
- SIMD (NEON) instructions
- Hardware FPU
- Out-of-order execution

**Assembly Candidates:**
- Vector operations (SIMD)
- Matrix operations (SIMD)
- Memory operations (SIMD loads/stores)

---

## Recommended Approach

### For Remaining Libraries:

1. **Compression Library:**
   - ✅ **Start:** Pure Pascal (LZ77, RLE, Huffman)
   - ⚠️ **Optimize:** Only if profiling shows need
   - **Rationale:** Compression is typically I/O bound, not CPU bound

2. **Crypto Library:**
   - ✅ **Start:** Pure Pascal (CRC checksums)
   - ⚠️ **Optimize:** Table-driven CRC is already fast
   - **Rationale:** CRC is O(n) and table-driven, very fast already

### For Existing Libraries:

1. **Profile First:** Run benchmarks on target platforms
2. **Identify Hot Paths:** Find functions called > 1000 times/frame
3. **Optimize Selectively:** Add inline assembly only where needed
4. **Maintain Compatibility:** Always provide Pascal fallback

---

## Conclusion

**Recommendation: Implement all libraries in Pure Pascal first.**

**Rationale:**
- ✅ Faster development
- ✅ Portable across all platforms
- ✅ Easier to maintain and debug
- ✅ Compiler can optimize
- ✅ Sufficient performance for most use cases

**Add Assembly Later:**
- ⚠️ Only after profiling identifies bottlenecks
- ⚠️ Only for proven hot paths
- ⚠️ Only with Pascal fallback
- ⚠️ Only with comprehensive testing

**Exception:** Platform-specific intrinsics (hardware features) may require assembly from the start, but these should be isolated in platform-specific modules.

---

**Last Updated:** 2025-01-XX  
**Status:** Strategy defined, ready for implementation

