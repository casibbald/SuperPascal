# 📘 ZealPascal Language Specification v0.3 — Semantic Rules

## Formal Definition of Types, Scopes, Compatibility, Dispatch, Evaluation & Error Semantics

This document defines the **semantic layer** of ZealPascal—how the language *means*, not just how it *parses*. It builds directly on:

* v0.1 (Language Philosophy)
* v0.2 (Grammar Specification)
* Combined Spec (Hybrid OOP + Struct Model)

This forms the foundation for:

* Compiler type checking
* Record & class layout
* ECS-safe design
* Code generation
* Error reporting
* Standard library contracts

This is **one of the core documents the compiler will directly implement**.

---

# 1. Type System

The ZealPascal type system is **nominal** for classes and records, and **structural** for primitive and composite types.

## 1.1 Primitive Type Semantics

* `integer` is a 16-bit signed value (−32768..32767).
* `byte` is 8-bit unsigned (0..255).
* `word` is 16-bit unsigned (0..65535).
* `boolean` is 1 byte (`0` = false, non-zero = true).
* `char` is 1 byte ASCII.
* `string` is short-string with 1-byte length + data.

Primitive values are copied **by value**, stored inline.

## 1.2 Enumeration Types

Enumerations define **distinct nominal types**.
Two enumerations with identical elements are **not interchangeable**.

Example:

```pascal
type Color = (Red, Green, Blue);
type Mood = (Red, Green, Blue); // NOT compatible with Color
```

## 1.3 Subrange Types

Subranges must be subsets of their base ordinal type.
Semantics:

* Assignment must check the runtime value fits inside the subrange (optional in release mode).

## 1.4 Record Types

Records are **value types** with no inheritance.

Rules:

* Two record types are **compatible only if they share the same name** (nominal typing).
* Assignments copy all fields.
* Methods defined on records compile into free functions.
* No constructors or destructors on records.

This supports ECS’s value-oriented design.

## 1.5 Set Types

Sets are fixed-size bitsets over small ordinal ranges.
Operations allowed:

* Union, intersection, difference
* Membership (`x in SetA`)
* Range of indexes must be ≤ 255

## 1.6 Array Types

Arrays have:

* Compile-time fixed bounds
* Structural equivalence: arrays of same bounds and element types are compatible

## 1.7 Pointer Types

Pointers may only point to:

* Records
* Classes
* Primitive data

Pointer arithmetic is **not allowed**.

---

# 2. Class Semantics (Hybrid Model)

ZealPascal uses a **lightweight Object Pascal** subset.

## 2.1 Nominal Typing

Two classes are compatible only if:

* They are identical types, or
* One is a descendant of the other (assignment to base allowed).

## 2.2 Memory Layout

A class instance has layout:

```
Offset 0: VTable pointer (2 bytes)
Offset 2+: Fields in declared order
```

Fields obey same layout rules as records.

## 2.3 Constructors & Destructors

* Must have the name `Create` / `Destroy`.
* Constructors return a **reference** to the instance.
* Destructors must be invoked explicitly unless part of engine lifecycle.

## 2.4 Method Dispatch

### Non-virtual methods

* Resolved at compile time (direct call).

### Virtual methods

* Resolved through vtable lookup.
* Overridden methods replace inherited vtable entries.

Dispatch rule:

```
callvirt O.Foo
→ load vtable from O
→ jump to method slot for Foo
```

## 2.5 Reference Semantics

Class variables hold **references**, not copies.
Assignment copies references.

## 2.6 Interoperability With ECS

Semantics enforce:

* Class must not hold complex game state internally
* Class may hold an ECS entity ID or configuration data
* All mutable gameplay data lives in ECS component arrays

This prevents fragmentation and scattered memory access patterns.

---

# 3. Expression Semantics

## 3.1 Constant Expressions

Evaluated at compile time if:

* All operands are literals or compile-time constants
* Operators are pure

Used for:

* Array bounds
* Case labels
* Packed set definitions

## 3.2 Integer Arithmetic

Overflow behavior:

* **Debug mode:** overflow raises compile-time or runtime error
* **Release mode:** overflow wraps around (Z80-compatible)

## 3.3 Boolean Logic

`and`, `or`, `not` operate on boolean values only.
Short-circuiting is required.

## 3.4 String Semantics

Strings are shortstrings:

* Assignments copy full value
* Comparisons are lexicographical
* Concatenation allocates new string on heap

Memory safety:

* Overflow into >255 chars is compile-time error if detectable; otherwise runtime error.

---

# 4. Statement Semantics

## 4.1 Assignment

Assignment rules:

* LHS must be assignable
* Types must be compatible (nominal for structured types)
* Subrange values must fit target

## 4.2 Procedure & Function Calls

Argument evaluation is left-to-right.

Parameter passing:

* Value parameters: copy
* `var` parameters: pass reference

## 4.3 Control Flow

`for` loop semantics:

* Index variable is read-only within loop body
* Final value is predictable
* Direction (`to` / `downto`) determines stepping behavior

## 4.4 With Statement

**Removed** in ZealPascal (too ambiguous for teaching).

## 4.5 Exception Handling (Future Tier)

Static semantics ensure try/except/finally blocks are well formed.

---

# 5. Unit & Scope Semantics

## 5.1 Visibility Rules

* Identifiers in `interface` are exported
* Identifiers in `implementation` are private

## 5.2 Name Resolution Order

1. Local scope
2. Parameter scope
3. Unit-level scope
4. Imported unit scopes
5. Global scope (program)

Shadowing allowed only if unambiguous.

## 5.3 Initialization & Finalization

Optional unit-level:

```pascal
begin
  ... init code ...
end;
```

Runs at program startup.

---

# 6. Intrinsic Semantics

Intrinsics represent direct hardware or engine operations.

Examples:

* `WaitVBlank`: synchronizes with display
* `WriteReg`: writes ZVB register
* `DMA_Copy`: bulk memory transfer
* `MapPage`: MMU page mapping
* `SpriteSet`: sprite binding
* `FadeMusic`: adjusts audio levels

Rules:

* Intrinsics must resolve to known compiler opcodes
* Must have fixed argument types
* Cannot be redefined

---

# 7. Tilemap DSL Semantics

Tilemap constructors must:

* Have compile-time known dimensions
* Convert `ROW('###..')` textual rows into tile indices
* Raise error if inconsistent row lengths

Tile legend may be defined globally or in a unit.

Generated tilemaps are constant arrays.

---

# 8. Semantic Errors

Compiler must report errors with clarity:

* Type mismatch
* Unknown identifier
* Invalid assignment target
* Out-of-range index
* Ambiguous overload
* Illegal recursion in constant expressions
* Misaligned class inheritance
* Excessively deep nesting (teaching safeguard)

Errors must explain *cause* and *fix*.

Example:

> Error: "5 is not in subrange 1..4. Consider adjusting the value or widening the range."

---

# 9. Implementation Notes for Compiler Writers

* ECS-style record methods should inline aggressively
* Vtable entries resolved once at compile time
* Unit imports produce small symbol tables
* Debug builds require runtime bounds checking
* Release builds may remove checks for performance
* Global variables should be placed sequentially for fast addressing

---

# ✔️ ZealPascal Semantic Rules Specification v0.3 Complete

This document now defines:

* Type semantics
* Scope & visibility rules
* Expression and statement semantics
* Class mechanics in the hybrid model
* Tilemap DSL behaviors
* Intrinsic constraints
* Error reporting semantics

Next milestones:

* **v0.4 ABI & Codegen Rules**
* **v0.5 Standard Library Specification**
* **v0.6 Compliance & Test Suite**
