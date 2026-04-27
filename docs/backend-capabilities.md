# Backend Capabilities and Feature Support

SuperPascal supports multiple target platforms, each with different capabilities based on hardware constraints. This document describes which Pascal language features are supported on each backend.

## Overview

SuperPascal uses a **backend capabilities system** that:
- Tracks which language features each backend supports
- Validates code during compilation to ensure compatibility
- Generates clear error messages when unsupported features are used
- Allows students to write code for modern platforms (64-bit ARM) with full Pascal features
- Restricts retro platforms (8-bit, 16-bit) to features they can actually support

## Target Platforms

### Retro Platforms (Limited Features)

#### ZealZ80 (Z80 @ 10 MHz)
**Status:** Primary Platform  
**Memory:** 512 KB SRAM (banked)

**Supported Features:**
- ✅ Core language (types, arrays, records, procedures, control flow)
- ✅ Sets, strings, variant records, enumerated types
- ✅ Pointers, file types
- ✅ Classes, properties, method pointers
- ✅ Nested routines, WITH statement, GOTO/Labels
- ✅ Inline assembly, for..in loops
- ✅ FORWARD/EXTERNAL, ABSOLUTE addressing

**NOT Supported:**
- ❌ Dynamic arrays (no heap management)
- ❌ Procedural types (too complex for 8-bit)
- ❌ Interfaces (too complex)
- ❌ Operator overloading (performance)
- ❌ Generics (too complex)
- ❌ Anonymous functions (too complex)
- ❌ Exception handling (no runtime support)
- ❌ THREADVAR, CONSTREF, OUT, RESOURCESTRING, default params
- ❌ Class methods/properties/variables/helpers/nested classes
- ❌ Reference counting, GC, multithreading, dynamic linking

#### CommanderX16 (65C02 @ 8 MHz)
**Status:** Tier 1 Platform  
**Memory:** 512 KB banked RAM

**Supported Features:** Same as ZealZ80 (8-bit limitations)

#### Foenix65C816 (W65C816S @ 6.29 MHz)
**Status:** Retro 16-bit Platform  
**Memory:** More capable than 8-bit

**Supported Features:**
- ✅ All ZealZ80 features
- ✅ Procedural types
- ✅ Interfaces
- ✅ Operator overloading
- ✅ Exception handling
- ✅ THREADVAR, CONSTREF, OUT, RESOURCESTRING, default params
- ✅ Class methods

**NOT Supported:**
- ❌ Dynamic arrays (limited heap)
- ❌ Generics (too complex)
- ❌ Anonymous functions (too complex)
- ❌ Class properties/variables/helpers/nested classes
- ❌ Reference counting, GC, multithreading, dynamic linking

#### FoenixA2560M (MC68LC060 @ 66 MHz)
**Status:** Retro 32-bit Platform  
**Memory:** 1 GB DDR3

**Supported Features:**
- ✅ All Foenix65C816 features
- ✅ Dynamic arrays
- ✅ Class properties, class variables

**NOT Supported:**
- ❌ Generics (too complex for retro)
- ❌ Anonymous functions (too complex)
- ❌ Class helpers, nested classes
- ❌ Reference counting, GC, multithreading, dynamic linking

#### Intel8051 (Microcontroller)
**Status:** Minimal Platform  
**Memory:** Very limited

**Supported Features:**
- ✅ Core language only (basic types, arrays, records, procedures, control flow)
- ✅ Enumerated types, pointers
- ✅ Inline assembly, ABSOLUTE addressing

**NOT Supported:** Almost everything else (minimal feature set)

### Modern Platform (Full Features)

#### RaspberryPi5 (ARM Cortex-A76 @ 2.4 GHz) - "Full Fat" Backend
**Status:** Full Modern Pascal Support  
**Memory:** 8 GB LPDDR4X

**Supported Features:** ✅ **ALL** Pascal language features including:
- All core and advanced types
- Dynamic arrays
- Generics
- Anonymous functions
- Exception handling
- All class features (methods, properties, variables, helpers, nested classes)
- Reference counting, garbage collection
- Multithreading
- Dynamic linking

This is the **"Full Fat"** backend that supports complete modern Object Pascal as described in the [Modern Object Pascal Introduction](https://castle-engine.io/modern_pascal).

## Feature Compatibility Matrix

| Feature | ZealZ80 | CommanderX16 | Foenix65C816 | FoenixA2560M | Intel8051 | RaspberryPi5 |
|---------|---------|--------------|--------------|--------------|-----------|--------------|
| **Core Features** |
| Basic Types | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Arrays | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Records | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Procedures | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Control Flow | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Advanced Types** |
| Dynamic Arrays | ❌ | ❌ | ❌ | ✅ | ❌ | ✅ |
| Sets | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Strings | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Variant Records | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Enumerated Types | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Pointers | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Procedural Types | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ |
| File Types | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| **Object Pascal** |
| Classes | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Interfaces | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ |
| Properties | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Operator Overloading | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ |
| Method Pointers | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| **Advanced Features** |
| Generics | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Anonymous Functions | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Nested Routines | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Exception Handling | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ |
| WITH Statement | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| GOTO/Labels | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| Inline Assembly | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| For..In Loops | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| **Advanced Declarations** |
| THREADVAR | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ |
| CONSTREF | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ |
| OUT Parameters | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ |
| RESOURCESTRING | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ |
| ABSOLUTE | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Default Params | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ |
| FORWARD/EXTERNAL | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| **Class Features** |
| Class Methods | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ |
| Class Properties | ❌ | ❌ | ❌ | ✅ | ❌ | ✅ |
| Class Variables | ❌ | ❌ | ❌ | ✅ | ❌ | ✅ |
| Class Helpers | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Nested Classes | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Runtime Features** |
| Reference Counting | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Garbage Collection | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Multithreading | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Dynamic Linking | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |

## Usage

### Compiler API

```rust
use runtime_spec::TargetPlatform;
use driver::Compiler;

// Create compiler for retro platform (ZealZ80)
let mut compiler = Compiler::new_with_target(TargetPlatform::ZealZ80);
compiler.compile_file("program.pas", None)?;

// Create compiler for modern platform (RaspberryPi5 - Full Fat)
let mut compiler = Compiler::new_with_target(TargetPlatform::RaspberryPi5);
compiler.compile_file("program.pas", None)?;

// Disable feature checking (for development/testing)
let mut compiler = Compiler::new_without_feature_check(TargetPlatform::ZealZ80);
```

### Error Messages

When code uses an unsupported feature, the compiler generates clear error messages:

```
Error: Feature 'Exception Handling' is not supported on ZealZ80 backend
  --> program.pas:15:5
   |
15 |   try
   |   ^^^
```

### Feature Detection

The feature checker automatically detects:
- Type usage (dynamic arrays, sets, strings, etc.)
- Statement usage (try/except, with, goto, asm, for..in)
- Declaration usage (threadvar, constref, out, resourcestring, absolute, default params)
- Class features (methods, properties, variables, helpers, nested classes)
- Advanced features (generics, anonymous functions, etc.)

## Design Rationale

### Why Different Feature Sets?

1. **Hardware Constraints**: 8-bit and 16-bit platforms have limited memory and processing power
2. **Runtime Requirements**: Some features (exception handling, GC) require runtime support
3. **Educational Value**: Students learn what features are practical on different platforms
4. **Real-World Compatibility**: Modern platforms (64-bit ARM) can support full Pascal

### Why "Full Fat" Backend?

Students building for:
- Local 64-bit ARM Raspberry Pi
- Modern desktop computers (x86-64, ARM64)
- Cloud platforms

...should have access to **complete modern Pascal** including:
- Generics for type-safe collections
- Anonymous functions for functional patterns
- Full exception handling
- Multithreading support
- Dynamic linking

This allows SuperPascal to serve both:
- **Educational retro computing** (limited features, learn constraints)
- **Modern software development** (full features, real-world applications)

## Implementation

The feature checking system:
- Runs after semantic analysis
- Traverses the AST to detect feature usage
- Compares against backend capabilities
- Generates diagnostics with clear error messages
- Can be disabled for development/testing

## Future Enhancements

- **Warning Mode**: Option to warn instead of error for unsupported features
- **Feature Flags**: Allow enabling experimental features
- **Backend-Specific Optimizations**: Use feature knowledge for code generation
- **Feature Documentation**: Auto-generate feature support docs per platform

---

**See Also:**
- [Parser Modern Pascal Comparison](./parser-modern-pascal-comparison.md)
- [Runtime Specification](../crates/compiler-rs/runtime-spec/src/lib.rs)
- [Backend Capabilities](../crates/compiler-rs/runtime-spec/src/capabilities.rs)

