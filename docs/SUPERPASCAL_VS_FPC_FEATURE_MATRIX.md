# SuperPascal vs FreePascal Feature Matrix

## Overview

This document maps SuperPascal language features against FreePascal (FPC) to guide:
- **Semantic reference decisions** (where SuperPascal mirrors FPC behavior)
- **Intentional deviations** (where SuperPascal simplifies or extends)
- **Tier 1 vs Tier 2** feature classification
- **Implementation guidance** for the Rust compiler

**Purpose:** Use FPC as an "oracle" for Pascal semantics while maintaining SuperPascal's educational and retro-computing focus.

---

## Feature Classification Legend

| Symbol | Meaning |
|--------|---------|
| ✅ **Tier 1** | Core feature, taught in Tier 1 and available in Tier 2 |
| ✅ **Tier 2** | Advanced feature, available in Tier 2 but not taught until after Tier 1 mastery |
| ❌ **Excluded** | Not supported in SuperPascal (design decision) |
| 🔄 **Modified** | Supported but with intentional changes from FPC |
| 📚 **Reference** | Use FPC behavior as semantic reference |
| 🎯 **SuperPascal** | SuperPascal-specific extension |

**Note:** Tier 2 features exist in the language and compiler, but are not part of the beginner curriculum. Students learn Tier 1 fundamentals first, then progress to Tier 2 advanced features.

---

## 1. Lexical Structure

| Feature | FPC Support | SuperPascal | Notes |
|---------|-------------|-------------|-------|
| Identifiers (alphanumeric + underscore) | ✅ | ✅ **Tier 1** | Standard Pascal identifiers |
| Keywords (case-insensitive) | ✅ | ✅ **Tier 1** | Core Pascal keywords |
| Comments: `{ }`, `(* *)` | ✅ | ✅ **Tier 1** | Standard Pascal comments |
| Comments: `//` | ✅ | ✅ **Tier 2** | Turbo Pascal style |
| String literals: `'...'` | ✅ | ✅ **Tier 1** | Standard Pascal strings |
| Numeric literals (decimal, hex) | ✅ | ✅ **Tier 1** | `$` prefix for hex |
| Compiler directives: `{$...}` | ✅ | ✅ **Tier 2** | Modified subset (see directives) |
| Tilemap DSL: `ROW("...")` | ❌ | 🎯 **Tier 2** | SuperPascal-specific |

**Decision:** Mirror FPC lexical rules for compatibility. Add SuperPascal-specific extensions.

---

## 2. Primitive Types

| Feature | FPC Support | SuperPascal | Notes |
|---------|-------------|-------------|-------|
| `integer` (signed) | ✅ (platform-dependent size) | ✅ **Tier 1** | 16-bit on Z80, 32-bit on m68k |
| `byte` (0..255) | ✅ | ✅ **Tier 1** | Unsigned 8-bit |
| `word` (0..65535) | ✅ | ✅ **Tier 1** | Unsigned 16-bit |
| `boolean` | ✅ | ✅ **Tier 1** | `true`/`false` |
| `char` | ✅ | ✅ **Tier 1** | 8-bit ASCII |
| `real` / `single` / `double` | ✅ | 🔄 **Tier 2** | Optional Float32 only (no double) |
| `shortint`, `smallint`, `longint` | ✅ | ✅ **Tier 2** | Advanced types, not taught in Tier 1 |
| `cardinal` | ✅ | ✅ **Tier 2** | Advanced type, not taught in Tier 1 |
| `string` | ✅ (multiple string types) | 🔄 **Tier 2** | Unified `shortstring` in Tier 1, `ansistring` in Tier 2 |
| `ansistring`, `unicodestring` | ✅ | ✅ **Tier 2** | Advanced string types, not taught in Tier 1 |
| `pointer` (untyped) | ✅ | ✅ **Tier 2** | `pointer` type |
| `nil` | ✅ | ✅ **Tier 2** | Null pointer constant |

**Decision:** Simplify type system for education. Use FPC's type rules as reference for compatibility.

---

## 3. Composite Types

| Feature | FPC Support | SuperPascal | Notes |
|---------|-------------|-------------|-------|
| **Arrays** | | | |
| Static arrays: `array[1..10] of integer` | ✅ | ✅ **Tier 1** | Standard Pascal arrays |
| Multi-dimensional arrays | ✅ | ✅ **Tier 2** | 2D/3D arrays |
| Packed arrays | ✅ | 🔄 **Tier 2** | Packed for memory efficiency |
| Dynamic arrays | ✅ | ❌ | Not supported (no GC) |
| Open arrays (procedure params) | ✅ | ❌ | Use pointers or fixed-size |
| **Records** | | | |
| Records (structs) | ✅ | ✅ **Tier 1** | Primary data type |
| Record methods | ✅ | ✅ **Tier 2** | Methods compile to free functions |
| Packed records | ✅ | 🔄 **Tier 2** | Packed for ECS SoA layout |
| Variant records | ✅ | ❌ | Not supported (safety) |
| Record operators | ✅ | ❌ | Not supported |
| **Sets** | | | |
| Sets: `set of char` | ✅ | ✅ **Tier 2** | Bitset representation |
| Set operations: `+`, `-`, `*`, `in` | ✅ | ✅ **Tier 2** | Standard set ops |
| Large sets (beyond 256 elements) | ✅ | ❌ | Max 256 elements (bitset) |
| **Pointers** | | | |
| Typed pointers: `^T` | ✅ | ✅ **Tier 2** | Standard Pascal pointers |
| Pointer arithmetic | ✅ | 🔄 **Tier 2** | Restricted (safety) |
| `@` (address-of) | ✅ | ✅ **Tier 2** | Address operator |
| `^` (dereference) | ✅ | ✅ **Tier 2** | Dereference operator |
| `new` / `dispose` | ✅ | 🔄 **Tier 2** | Simplified (no GC) |

**Decision:** Use FPC's record/array/set semantics as reference. Simplify for educational clarity.

---

## 4. Control Flow

| Feature | FPC Support | SuperPascal | Notes |
|---------|-------------|-------------|-------|
| `if` / `then` / `else` | ✅ | ✅ **Tier 1** | Standard conditional |
| `case` / `of` / `else` | ✅ | ✅ **Tier 1** | Case statements |
| `while` / `do` | ✅ | ✅ **Tier 1** | While loops |
| `repeat` / `until` | ✅ | ✅ **Tier 1** | Repeat loops |
| `for` / `to` / `downto` | ✅ | ✅ **Tier 1** | For loops |
| `break` | ✅ | ✅ **Tier 2** | Loop exit |
| `continue` | ✅ | ✅ **Tier 2** | Loop continue |
| `goto` / labels | ✅ | ❌ | Not supported (safety) |
| `exit` (procedure return) | ✅ | ✅ **Tier 1** | Early return |
| `halt` | ✅ | ✅ **Tier 2** | Program termination |

**Decision:** Mirror FPC control flow semantics. Exclude `goto` for safety.

---

## 5. Procedures and Functions

| Feature | FPC Support | SuperPascal | Notes |
|---------|-------------|-------------|-------|
| Procedure declarations | ✅ | ✅ **Tier 1** | `procedure Name;` |
| Function declarations | ✅ | ✅ **Tier 1** | `function Name: Type;` |
| Value parameters | ✅ | ✅ **Tier 1** | By-value params |
| `var` parameters (by-reference) | ✅ | ✅ **Tier 1** | By-reference params |
| `const` parameters | ✅ | ✅ **Tier 2** | Const params |
| `out` parameters | ✅ | ❌ | Use `var` instead |
| Forward declarations | ✅ | ✅ **Tier 2** | Forward proc/func |
| Nested procedures | ✅ | ❌ | Not supported (simplicity) |
| Procedure/function types | ✅ | ✅ **Tier 2** | Function pointers |
| Overloading | ✅ | ✅ **Tier 2** | Function overloading |
| Default parameters | ✅ | ❌ | Not supported |
| Variadic functions | ✅ | ❌ | Not supported |
| Inline procedures | ✅ | 🔄 **Tier 2** | Via `{$INLINE}` directive |

**Decision:** Use FPC calling conventions as reference. Simplify parameter model.

---

## 6. Units and Modular Programming

| Feature | FPC Support | SuperPascal | Notes |
|---------|-------------|-------------|-------|
| Unit structure: `unit` / `interface` / `implementation` | ✅ | ✅ **Tier 2** | Standard Pascal units |
| `uses` clause | ✅ | ✅ **Tier 2** | Unit imports |
| Unit initialization | ✅ | ✅ **Tier 2** | `initialization` section |
| Unit finalization | ✅ | ❌ | Not supported (simplicity) |
| Circular unit dependencies | ✅ (with restrictions) | ❌ | Not allowed |
| Unit aliases | ✅ | ❌ | Not supported |
| Conditional compilation: `{$IFDEF}` | ✅ | 🔄 **Tier 2** | Platform-specific units |

**Decision:** Mirror FPC unit model. Simplify finalization and circular deps.

---

## 7. Object-Oriented Programming

| Feature | FPC Support | SuperPascal | Notes |
|---------|-------------|-------------|-------|
| **Classes** | | | |
| `class` type declarations | ✅ | ✅ **Tier 3** | Lightweight classes |
| Single inheritance | ✅ | ✅ **Tier 3** | One base class |
| Multiple inheritance | ❌ | ❌ | Not supported |
| Interfaces | ✅ | ❌ | Not supported (simplicity) |
| Constructors | ✅ | ✅ **Tier 3** | `constructor Create;` |
| Destructors | ✅ | ✅ **Tier 3** | `destructor Destroy;` |
| Virtual methods | ✅ | ✅ **Tier 3** | `virtual` keyword |
| Override methods | ✅ | ✅ **Tier 3** | `override` keyword |
| Abstract methods | ✅ | ❌ | Not supported |
| Class methods | ✅ | ❌ | Not supported |
| Properties | ✅ | ❌ | Not supported (use getters/setters) |
| RTTI (Runtime Type Info) | ✅ | 🔄 **Tier 3** | Optional, minimal |
| **Records with Methods** | | | |
| Record methods | ✅ | ✅ **Tier 2** | Methods compile to free functions |
| Record operators | ✅ | ❌ | Not supported |

**Decision:** Use FPC's class model as reference. Simplify to single inheritance, no interfaces. Records are primary data type.

---

## 8. Exceptions

| Feature | FPC Support | SuperPascal | Notes |
|---------|-------------|-------------|-------|
| `try` / `except` | ✅ | ✅ **Tier 2** | Exception handling |
| `try` / `finally` | ✅ | ✅ **Tier 2** | Cleanup blocks |
| `raise` | ✅ | ✅ **Tier 2** | Raise exceptions |
| Exception types | ✅ | 🔄 **Tier 2** | `EError` base class only |
| Exception stack unwinding | ✅ | ✅ **Tier 2** | Frame chain via `IY` |
| `on E: Exception do` | ✅ | 🔄 **Tier 2** | Simplified exception matching |

**Decision:** Use FPC exception model as reference. Simplify exception types.

---

## 9. Advanced Features

| Feature | FPC Support | SuperPascal | Notes |
|---------|-------------|-------------|-------|
| Generics / Templates | ✅ | ✅ **Tier 2** | Advanced feature, not taught in Tier 1 |
| Operator overloading | ✅ | ✅ **Tier 2** | Advanced feature, not taught in Tier 1 |
| Type helpers | ✅ | ✅ **Tier 2** | Advanced feature, not taught in Tier 1 |
| Managed types | ✅ | ❌ | Not supported (no GC) |
| Attributes / Annotations | ✅ | ✅ **Tier 2** | Advanced feature, not taught in Tier 1 |
| Macros | ✅ | ❌ | Not supported (safety) |
| Assembly blocks | ✅ | 🔄 **Tier 2** | Via intrinsics only |
| Inline assembly | ✅ | ❌ | Not supported (safety) |

**Decision:** Advanced features available in Tier 2, but not taught until students master Tier 1 fundamentals.

---

## 10. Compiler Directives

| Feature | FPC Support | SuperPascal | Notes |
|---------|-------------|-------------|-------|
| `{$MODE}` | ✅ | ❌ | Single mode only |
| `{$IFDEF}` / `{$IFNDEF}` | ✅ | 🔄 **Tier 2** | Platform conditionals |
| `{$INCLUDE}` | ✅ | ✅ **Tier 2** | File inclusion |
| `{$INLINE}` | ✅ | 🔄 **Tier 2** | Inline procedure hint |
| `{$UNROLL}` | ❌ | 🎯 **Tier 2** | SuperPascal-specific (loop unrolling) |
| `{$OPTIMIZE}` | ✅ | 🔄 **Tier 2** | Optimization hints |
| `{$RANGECHECKS}` | ✅ | 🔄 **Tier 2** | Debug mode only |
| `{$OVERFLOWCHECKS}` | ✅ | 🔄 **Tier 2** | Debug mode only |
| `{$I}` (include) | ✅ | ✅ **Tier 2** | Short form include |
| `{$DEFINE}` / `{$UNDEF}` | ✅ | 🔄 **Tier 2** | Conditional compilation |

**Decision:** Subset of FPC directives. Add SuperPascal-specific optimization hints.

---

## 11. Standard Library

| Feature | FPC Support | SuperPascal | Notes |
|---------|-------------|-------------|-------|
| `System` unit | ✅ | ✅ **Tier 2** | Core runtime functions |
| `Write` / `Writeln` | ✅ | ✅ **Tier 1** | Console output |
| `Read` / `Readln` | ✅ | ✅ **Tier 1** | Console input |
| `Str` / `Val` | ✅ | ✅ **Tier 2** | String conversion |
| `Length` / `Copy` / `Pos` | ✅ | ✅ **Tier 2** | String operations |
| `Math` unit | ✅ | ✅ **Tier 2** | Math functions |
| `MathFP` unit | ❌ | 🎯 **Tier 2** | SuperPascal fixed-point math |
| `MathTrig` unit | ❌ | 🎯 **Tier 2** | SuperPascal trig suite |
| File I/O | ✅ | 🔄 **Tier 2** | ZealOS file system |
| `SysUtils` | ✅ | 🔄 **Tier 2** | Simplified utilities |

**Decision:** Mirror FPC standard library where appropriate. Add SuperPascal-specific math units.

---

## 12. Platform-Specific Features

| Feature | FPC Support | SuperPascal | Notes |
|---------|-------------|-------------|-------|
| **Intrinsics** | | | |
| Hardware intrinsics | ✅ (limited) | 🎯 **Tier 2** | ZVB graphics/audio intrinsics |
| `WaitVBlank` | ❌ | 🎯 **Tier 2** | Zeal-specific |
| `SpriteSet` | ❌ | 🎯 **Tier 2** | Zeal-specific |
| Direct memory access | ✅ | 🔄 **Tier 2** | Via intrinsics only |
| I/O port access | ✅ | 🔄 **Tier 2** | Via intrinsics only |
| **ABI** | | | |
| Calling conventions | ✅ (multiple) | 🔄 **Tier 2** | Platform-specific ABI |
| Register usage | ✅ (platform-specific) | 🔄 **Tier 2** | Z80: `IX` frame, `IY` exception |
| Stack layout | ✅ (platform-specific) | 🔄 **Tier 2** | Platform-specific |

**Decision:** Use FPC's m68k ABI as reference for A2560M. Define Z80/65C816 ABIs independently.

---

## 13. Memory Management

| Feature | FPC Support | SuperPascal | Notes |
|---------|-------------|-------------|-------|
| Stack allocation | ✅ | ✅ **Tier 1** | Automatic |
| Static allocation | ✅ | ✅ **Tier 1** | Global variables |
| Heap allocation (`new` / `dispose`) | ✅ | 🔄 **Tier 2** | Simplified (no GC) |
| Garbage collection | ✅ (optional) | ❌ | Not supported |
| Memory pools | ✅ | 🔄 **Tier 2** | ECS page-based arenas |
| Reference counting | ✅ | ❌ | Not supported |

**Decision:** Explicit memory management. No GC for predictable performance.

---

## Implementation Strategy

### Using FPC as Reference

1. **Semantic Oracle:** For ambiguous Pascal semantics, check FPC behavior first.
2. **ABI Reference:** Use FPC's m68k backend as reference for A2560M calling conventions.
3. **Test Compatibility:** Use FPC test suite as inspiration for SuperPascal compliance tests.
4. **Code Organization:** Study FPC's frontend/backend separation for Rust crate layout.

### Educational Progression

1. **Tier 1 Type System:** Simplified for beginners (`byte`, `integer`, `word`, `string` only).
2. **Tier 2 Type System:** Full FPC-compatible types available (`shortint`, `smallint`, `longint`, `cardinal`, `ansistring`, etc.).
3. **Tier 2 Advanced Features:** Generics, operator overloading, type helpers available but not taught until Tier 1 mastery.
4. **No GC:** Explicit memory management for retro platforms (all tiers).
5. **Hybrid OOP Model:** Records for data, classes for orchestration.
6. **Platform-Specific Intrinsics:** ZVB graphics/audio integration.

### Tier Classification

- **Tier 1 (Beginner):** Core Pascal subset taught in curriculum. Features: basic types, control flow, procedures, arrays, records.
- **Tier 2 (Advanced):** Complete SuperPascal feature set. All FPC-compatible features available, taught after Tier 1 mastery.
- **Tier 3 (OOP layer):** Lightweight classes for game engine (part of Tier 2).

---

## Next Steps

1. **Extract FPC m68k ABI details** for A2560M backend reference.
2. **Create compatibility test suite** based on FPC tests.
3. **Document semantic differences** in language specification.
4. **Implement Rust compiler** using FPC as semantic reference.

---

**Version:** 1.0  
**Date:** 2024  
**Status:** Reference Document

