# Platform Compatibility Guide

**Purpose:** Document platform-specific requirements and considerations for standard library modules

---

## Platform Categories

### No-FPU Platforms
- **ZealZ80** (Zilog Z80 @ 10 MHz)
- **CommanderX16** (WDC 65C02S @ 8 MHz)
- **Foenix65C816** (WDC 65C816S @ 6.29 MHz)
- **FoenixA2560M** (MC68LC060 @ 66 MHz) - No FPU variant

### FPU Platforms
- **Raspberry Pi 5** (ARM Cortex-A76 @ 2.4 GHz) - Has FPU

---

## Module Compatibility Matrix

| Module | ZealZ80 | CommanderX16 | Foenix65C816 | FoenixA2560M | Raspberry Pi 5 |
|--------|---------|---------------|--------------|---------------|-----------------|
| **ECS Library** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Math_Fixed** | ⚠️ | ⚠️ | ✅ | ✅ | ✅ |
| **Math_Trig** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Math_Sqrt** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Math_Matrix** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Math_Vector** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Graphics_Line** | ✅ | ✅ | ✅ | ✅ | ✅ |

**Legend:**
- ✅ **Fully Compatible** - Works without modifications
- ⚠️ **Partial Compatibility** - Works but may need platform-specific optimizations

---

## Platform-Specific Considerations

### 8-Bit Systems (ZealZ80, CommanderX16)

#### Int64 Support
**Issue:** `Math_Fixed` uses `Int64` for multiply/divide operations  
**Impact:** May not be available or efficient on 8-bit systems

**Current Implementation:**
- Uses conditional compilation (`{$IFDEF CPU_8BIT}`)
- Falls back to basic operations (may lose precision)
- **Recommendation:** Provide assembly versions for critical operations

**Affected Functions:**
- `Fixed16Mul` - Uses Int64 for 32-bit multiply
- `Fixed16Div` - Uses Int64 for 32-bit divide

**Workaround:**
- Use smaller fixed-point formats (Q8.8) on 8-bit systems
- Or provide platform-specific assembly implementations

#### Memory Constraints
**Considerations:**
- Trig tables: 512 bytes (SinTable + CosTable)
- Sqrt table: 256 bytes
- Total: ~768 bytes (acceptable for all platforms)

### 16-Bit Systems (Foenix65C816)

#### Int64 Support
**Status:** ✅ **Supported**  
**Note:** 65C816 has 16-bit ALU but can handle 32-bit operations efficiently

**All Math Library Functions:** ✅ Work correctly

### 32-Bit Systems (FoenixA2560M)

#### Int64 Support
**Status:** ✅ **Supported**  
**Note:** MC68LC060 has 32-bit ALU, Int64 operations use software emulation

**All Math Library Functions:** ✅ Work correctly

**Performance:**
- Fixed-point operations: Very fast (native 32-bit multiply)
- Matrix operations: Fast (uses fixed-point)
- Vector operations: Fast

### 64-Bit Systems (Raspberry Pi 5)

#### Int64 Support
**Status:** ✅ **Native Support**  
**Note:** ARM64 has native 64-bit operations

**All Math Library Functions:** ✅ Work correctly (and very fast)

**Optional Optimizations:**
- Can use FPU for table generation (faster initialization)
- Can use hardware sqrt if available
- Can use SIMD for vector operations (future enhancement)

---

## Conditional Compilation

### CPU Type Defines

**Standard Defines:**
- `CPU_8BIT` - 8-bit CPU (Z80, 65C02)
- `CPU_16BIT` - 16-bit CPU (65C816)
- `CPU_32BIT` - 32-bit CPU (m68k)
- `CPU_64BIT` - 64-bit CPU (ARM64)

**Platform-Specific Defines:**
- `HAS_FPU` - Floating-point unit available
- `HAS_INT64` - 64-bit integer support

### Usage Example

```pascal
{$IFDEF CPU_8BIT}
  // 8-bit specific code
  Result := (a * b) shr 16;  // Basic multiply
{$ELSE}
  // 16-bit+ code
  Result := (Int64(a) * Int64(b)) shr 16;  // Extended precision
{$ENDIF}
```

---

## Platform-Specific Optimizations

### For 8-Bit Systems

**Recommended:**
1. Use Q8.8 format instead of Fixed16 (smaller, faster)
2. Provide assembly versions for critical operations
3. Use lookup tables where possible (already done for trig/sqrt)

**Future Work:**
- Assembly implementations for `Fixed16Mul` on Z80
- Assembly implementations for `Fixed16Div` on Z80
- Optimized versions for 65C02

### For 16-Bit Systems

**Status:** ✅ **Optimal**  
**Note:** 65C816 handles 32-bit operations efficiently

### For 32-Bit Systems

**Status:** ✅ **Optimal**  
**Note:** m68k has fast 32-bit multiply, very efficient

### For 64-Bit Systems

**Optional Enhancements:**
- Use FPU for initialization (faster table generation)
- Use hardware sqrt (if available)
- SIMD optimizations (future)

---

## Testing Recommendations

### Platform Testing Matrix

| Module | Test Platform | Critical Tests |
|--------|---------------|----------------|
| **Math_Fixed** | All platforms | Multiply/divide precision, overflow handling |
| **Math_Trig** | All platforms | Table generation, function accuracy |
| **Math_Sqrt** | All platforms | Table initialization, function accuracy |
| **Math_Matrix** | All platforms | Matrix multiplication correctness |
| **Math_Vector** | All platforms | Vector operations, normalization |
| **Graphics_Line** | All platforms | Line drawing correctness, edge cases |

### Test Cases

**Fixed-Point:**
- Basic arithmetic (add, sub, mul, div)
- Conversion (int ↔ fixed)
- Edge cases (zero, overflow, underflow)
- Precision tests

**Trigonometry:**
- Table generation (recursive method)
- Function accuracy (sin, cos, tan)
- Inverse functions (arcsin, arccos, arctan)
- Edge cases (0°, 90°, 180°, 270°)

**Square Root:**
- Table initialization
- Function accuracy
- Edge cases (0, 1, large numbers)

---

## Migration Path

### For New Platforms

1. **Identify CPU Type:**
   - 8-bit, 16-bit, 32-bit, or 64-bit
   - FPU availability
   - Int64 support

2. **Set Defines:**
   - Add platform-specific defines
   - Configure conditional compilation

3. **Test Critical Functions:**
   - Fixed-point multiply/divide
   - Trig table generation
   - Sqrt table initialization

4. **Optimize if Needed:**
   - Provide assembly versions
   - Use platform-specific intrinsics

---

## Summary

### ✅ Fully Compatible Modules

All modules work on all platforms with:
- Proper conditional compilation
- Platform-agnostic algorithms
- Fallback implementations

### ⚠️ Platform-Specific Considerations

**8-Bit Systems:**
- Int64 operations may need assembly versions
- Consider using Q8.8 instead of Fixed16
- All other modules work correctly

**16-Bit+ Systems:**
- All modules work optimally
- No special considerations needed

---

**Last Updated:** 2025-01-XX  
**Status:** All modules compatible with platform-specific optimizations documented

