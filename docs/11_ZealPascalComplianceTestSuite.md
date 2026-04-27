# 📘 ZealPascal Language Specification v0.6 — Compliance & Test Suite

## Compiler, Runtime, ABI, Standard Library, Engine, and Hardware Integration Tests

This document defines the **official compliance and test suite** for the ZealPascal language, standard library, and runtime ecosystem.

Its purpose is to ensure:

* **Correctness** across all compliant compilers
* **Stability** as features evolve
* **Consistency** across versions of ZealPascal
* **Safety** and **predictable behavior** for educational settings
* **Engine and hardware reliability** (ZVB graphics/audio/input)

This is a foundational QA & verification framework for:

* Compiler developers
* Engine contributors
* Educators verifying classroom builds
* Automated CI systems

---

# 1. Goals of the Compliance & Test Suite

### 🎯 Guarantee Language Correctness

All language constructs must behave as defined in:

* v0.3 Semantic Rules
* v0.4 ABI & Codegen

### 🧪 Verify Standard Library Behavior

Every function in:

* `System`
* `Math`, `MathFP`, `MathTrig`, `MathFloat`
* `Strings`
* `SysUtils_Lite`
* `ZVB_*`
* `Engine_*`

…must behave according to specification.

### 🛡 Ensure Safety for Teaching

* Debug mode checks must activate correctly.
* Error messages must be child-friendly.

### 🎮 Validate Engine Integration

* ECS systems
* Scene transitions
* Rendering and sprite correctness
* Audio timing and mixing

### 🔧 Provide CI-Ready Automation

Tests must run both:

* On real Zeal hardware
* In QEMU-style or custom Zeal emulator

---

# 2. Test Categories Overview

The test suite is divided into the following categories:

```
1. Syntax Tests
2. Semantic Tests
3. ABI & Codegen Tests
4. Standard Library Tests
5. Intrinsics & Hardware Tests
6. ECS & Engine Tests
7. Performance Baselines
8. Error & Diagnostic Tests
9. Educational Profile Tests
```

Each category includes:

* Test description
* Expected result
* Failure conditions

---

# 3. Syntax Tests (Parser Compliance)

Tests that ensure the parser correctly handles all language syntax.

## 3.1 Valid Syntax Cases

* Minimal program
* Units with interface/implementation
* Records
* Classes (hybrid model subset)
* Procedures & functions
* Structured statements (`if`, `for`, `while`, `repeat`, `case`)
* Try/except/finally blocks
* Tilemap DSL constructs
* Intrinsics

## 3.2 Invalid Syntax Cases

* Unterminated blocks
* Missing semicolons
* Wrong parameter lists
* Misplaced modifiers (`virtual`, `override`)
* Bad DSL rows

Each case must produce:

* Clear error message
* Line & column reference

---

# 4. Semantic Tests

Ensures compiler enforces **type rules**, **scoping**, **assignment semantics**, and **OOP hybrid rules**.

## 4.1 Type Compatibility

Test:

* integer ↔ byte conversions
* record nominal typing
* class inheritance rules
* pointer rules
* string assignment rules
* fixed-point conversion semantics

## 4.2 Expression Semantics

* Constant folding
* Overflow detection (debug)
* Comparison operators
* Boolean short-circuiting

## 4.3 Procedure & Function Semantics

* `var` parameters
* Hidden parameters (`Self`, return buffers)
* Correct return semantics for complex types

## 4.4 Exception Semantics

* correct unwinding
* try/except/try/finally behavior
* rethrow semantics

---

# 5. ABI & Codegen Tests

Ensures different compilers produce compatible binaries.

## 5.1 Stack Frame Tests

* Validate SP/IX behavior
* Parameter offsets
* Return value conventions

## 5.2 VTable Tests

* Slot ordering
* Inheritance replacement behavior
* Binary layout stability

## 5.3 Record Layout Tests

* Field order
* Alignment rules
* Size computation

## 5.4 Intrinsic Lowering

Ensure correct codegen for:

* WaitVBlank
* DMA_Copy
* WriteReg
* MapPage
* SpriteSet

---

# 6. Standard Library Tests

Every stdlib unit must have coverage.

## 6.1 `Math` Tests

* abs, sqr, max/min
* integer edge cases

## 6.2 `MathFP` Tests

* Fixed-point add/sub/mul/div
* Range/overflow checks

## 6.3 `MathTrig` Tests

* sin, cos, tan, sec, csc, cot
* arc functions
* LUT correctness
* interpolation error bounds

## 6.4 `MathFloat` Tests

* float operations against known references
* rounding rules

## 6.5 `Strings` Tests

* copy, concat, insert, delete
* overflow behavior
* lexicographic compare

## 6.6 `SysUtils_Lite` Tests

* int conversion
* hex conversion
* RaiseError behavior

---

# 7. Hardware & Intrinsics Test Suite

Tests require real hardware OR an emulator with ZVB emulation.

## 7.1 ZVB Graphics Tests

* mode switching
* tile drawing
* sprite movement
* palette changes
* DMA blits

## 7.2 ZVB Audio Tests

* SFX
* music playback
* volume & mixing
* streaming tests

## 7.3 Input Tests

* key state transitions
* axis mapping
* multi-key behavior

---

# 8. ECS & Engine Behavioral Tests

Tests validate ECS pipeline and high-level engine behavior.

## 8.1 ECS Tests

* Creation/destruction of entities
* Component storage integrity
* System iteration ordering
* Change detection correctness

## 8.2 Scene Tests

* Scene transitions
* Init/Update/Render calls
* Exception-safe scene switching

## 8.3 Physics Tests

* collision detection
* gravity model
* tilemap overlap resolution

## 8.4 Animation Tests

* keyframe interpolation
* frame stepping

## 8.5 Scripting Tests

* script execution
* WAIT/MOVE/ANIM correctness

---

# 9. Performance Baseline Tests

These tests ensure ZealPascal meets minimum performance thresholds.

## 9.1 ECS Throughput

* Minimum entities @ 60 FPS (target: 100+)

## 9.2 Sprite Budget Test

* 64 sprites with movement and animation

## 9.3 Tilemap Redraw

* ≤128 tile changes per frame at 60 FPS

## 9.4 Audio Stability

* No glitches during heavy CPU load

---

# 10. Error Handling & Diagnostics Tests

Ensures compiler and runtime produce helpful messages.

Cases:

* Undefined identifiers
* Type mismatches
* Out-of-range array access (debug)
* Division by zero
* Incorrect DSL syntax
* Failed intrinsic calls

Errors MUST:

* Contain the problem
* Suggest resolution
* Indicate file and line

---

# 11. Educational Profile Test Suite

Tests that the **Beginner Profile** behaves as expected:

## 11.1 Forbidden Features

* `goto`
* pointers
* classes
* manual memory mgmt

Compiler must detect and forbid them.

## 11.2 Allowed Subset Tests

* structured Pascal
* strings
* simple graphics
* integer math

## 11.3 Classroom Safety Tests

* overflow errors must be clear
* infinite loop detection (optional heuristic)
* tilemap DSL must reject malformed definitions

---

# ✔️ ZealPascal Compliance & Test Suite v0.6 Complete

This document defines the **official compliance test plan**:

* Syntax → semantics → ABI → runtime → engine → hardware tests
* Educational subset validation
* Performance thresholds

This ensures ZealPascal is:

* Reliable
* Consistent
* Safe for students
* Robust for game development

Next steps:

* v0.7 Exception Semantics (completed)
* v1.0 Finalization & Release of ZealPascal Toolkit
