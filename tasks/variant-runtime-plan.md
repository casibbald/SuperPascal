# Variant Type Runtime Support - Implementation Plan

**Status:** Planning  
**Priority:** Medium (96% compliance achieved, runtime support is a significant undertaking)  
**Estimated Effort:** 8-16 hours (compiler-side) + Runtime library implementation

---

## Overview

Variant type allows dynamic typing - a Variant variable can hold values of any type. This requires runtime support for:
- Type information storage
- Type checking at runtime
- Type conversion
- Memory management

---

## Current Status

✅ **Parser:** Complete - Variant type is recognized  
✅ **Semantic Analysis:** Complete - Variant accepts/assigns any type  
✅ **IR Support:** Complete - Variant is in Type enum  
⚠️ **Code Generation:** Pending - Need to generate runtime calls  
⚠️ **Runtime Library:** Pending - Need runtime functions

---

## Runtime Library Requirements

### Required Runtime Functions

1. **Variant Creation**
   - `Variant_New(type_id, value)` - Create a new Variant with a value
   - `Variant_NewEmpty()` - Create an empty Variant

2. **Variant Assignment**
   - `Variant_Assign(variant_ptr, type_id, value)` - Assign a value to a Variant
   - `Variant_Copy(dest, src)` - Copy one Variant to another

3. **Type Checking**
   - `Variant_GetType(variant_ptr)` - Get the type ID of a Variant value
   - `Variant_IsType(variant_ptr, type_id)` - Check if Variant is a specific type

4. **Type Conversion**
   - `Variant_ToInteger(variant_ptr)` - Convert Variant to integer (with type check)
   - `Variant_ToString(variant_ptr)` - Convert Variant to string (with type check)
   - `Variant_ToBoolean(variant_ptr)` - Convert Variant to boolean (with type check)
   - `Variant_ToChar(variant_ptr)` - Convert Variant to char (with type check)
   - `Variant_ToByte(variant_ptr)` - Convert Variant to byte (with type check)
   - `Variant_ToWord(variant_ptr)` - Convert Variant to word (with type check)

5. **Memory Management**
   - `Variant_Free(variant_ptr)` - Free Variant memory
   - `Variant_Cleanup()` - Cleanup all Variants (for shutdown)

### Variant Structure (Runtime)

```pascal
type
  TVariantType = (
    vtEmpty, vtInteger, vtString, vtBoolean, 
    vtChar, vtByte, vtWord, vtArray, vtRecord, vtPointer
  );
  
  PVariant = ^TVariant;
  TVariant = record
    VarType: TVariantType;
    case TVariantType of
      vtInteger: (IntValue: integer);
      vtString: (StrValue: ^string);
      vtBoolean: (BoolValue: boolean);
      vtChar: (CharValue: char);
      vtByte: (ByteValue: byte);
      vtWord: (WordValue: word);
      vtArray: (ArrayValue: pointer);
      vtRecord: (RecordValue: pointer);
      vtPointer: (PtrValue: pointer);
  end;
```

---

## Compiler-Side Implementation

### 1. Code Generation for Variant Assignments

When generating code for `v := value` where `v` is Variant:

```rust
// Pseudo-code for code generation
match target_type {
    Type::Variant => {
        // Generate call to Variant_Assign
        // 1. Get type ID of source value
        // 2. Call Variant_Assign(variant_ptr, type_id, value)
    }
    _ if source_type == Type::Variant => {
        // Generate call to Variant conversion function
        // 1. Check type at runtime
        // 2. Convert and assign
    }
}
```

### 2. Code Generation for Variant Operations

- **Variant to Variant assignment:** `Variant_Copy(dest, src)`
- **Variant to typed assignment:** Type-specific conversion function
- **Typed to Variant assignment:** `Variant_Assign(variant_ptr, type_id, value)`

### 3. Type ID Mapping

Need to map Pascal types to runtime type IDs:
- `integer` → `vtInteger`
- `string` → `vtString`
- `boolean` → `vtBoolean`
- `char` → `vtChar`
- `byte` → `vtByte`
- `word` → `vtWord`
- etc.

---

## Implementation Steps

### Phase 1: Compiler-Side Preparation (4-8 hours)
1. ✅ Add Variant to Type enum (DONE)
2. ✅ Add Variant semantic analysis (DONE)
3. [ ] Add Variant type ID mapping in code generation
4. [ ] Add Variant assignment code generation
5. [ ] Add Variant conversion code generation
6. [ ] Add tests for code generation

### Phase 2: Runtime Library Implementation (8-16 hours)
1. [ ] Implement Variant structure
2. [ ] Implement Variant creation functions
3. [ ] Implement Variant assignment functions
4. [ ] Implement type checking functions
5. [ ] Implement type conversion functions
6. [ ] Implement memory management
7. [ ] Add runtime tests

### Phase 3: Integration (2-4 hours)
1. [ ] Link runtime library with compiler output
2. [ ] Test end-to-end Variant operations
3. [ ] Performance testing
4. [ ] Documentation

---

## Backend Considerations

### Retro Backends (ZealZ80, CommanderX16, etc.)
- **Memory:** Variant structure adds overhead (type + value)
- **Performance:** Runtime type checking adds overhead
- **Recommendation:** Variant may not be suitable for very constrained platforms
- **Capability Flag:** Add `VariantType` to `LanguageFeature` enum
- **Backend Support:** Only enable for platforms with sufficient memory

### Modern Backends (RaspberryPi5, etc.)
- **Full Support:** All Variant features available
- **Performance:** Acceptable overhead for dynamic typing
- **Recommendation:** Full Variant support

---

## Testing Requirements

1. **Unit Tests:**
   - Variant creation
   - Variant assignment (all types)
   - Type checking
   - Type conversion (all types)
   - Error cases (invalid conversions)

2. **Integration Tests:**
   - Variant in procedures/functions
   - Variant in records
   - Variant in arrays
   - Variant with generics (if applicable)

3. **Performance Tests:**
   - Variant assignment overhead
   - Type checking overhead
   - Memory usage

---

## Notes

- Variant runtime support is a significant undertaking
- Requires both compiler and runtime library work
- May not be suitable for all target platforms
- Consider adding capability flags to disable Variant on constrained platforms
- Runtime library should be platform-specific but interface should be consistent

---

**Last Updated:** 2024-12-19  
**Next Steps:** Decide whether to implement now or defer to runtime library development phase

