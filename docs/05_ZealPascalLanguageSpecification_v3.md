# 📘 ZealPascal Language Specification — Combined & Modernized (Hybrid OOP + Struct Model)

## Version 0.3 (Draft) — High-Level Philosophy + Tiered Feature Set + Updated OOP/Struct Hybrid Model + Clean Grammar Foundations

This document unifies the **v0.1 High-Level Language Specification**, **v0.2 Grammar Definition**, and our subsequent design updates (notably the **Hybrid OOP + Struct Model**) into a single, clean, modern specification.

It reflects:

* A pedagogically friendly Pascal for schools
* A powerful systems language for Zeal game development
* A hybrid approach: **records for data, classes for orchestration**
* A modern, safe, and approachable language identity

This is the new authoritative reference for continued development.

---

# 0. Language Philosophy (Modernized)

ZealPascal is a structured, statically-typed language designed for the **Zeal 8‑bit Computer**. It builds on the legacy of Pascal but modernizes it with:

* **Records-as-structs** for data modeling (ECS-friendly)
* **Lightweight Object Pascal classes** for orchestration and behavior
* **Modules/units** for clean organization
* **Clear syntax and semantics for teaching**
* **Modern compiler features** (intrinsics, directives, clean strings)
* **ZVB graphics/audio integration**

### Core Principles

1. **Simplicity First** — Easy for children and beginners.
2. **Predictability** — Deterministic runtime, clear memory model.
3. **Performance** — Z80-friendly codegen, SoA arrays, unrolled loops.
4. **Expressiveness** — Classes, units, DSL constructs.
5. **Safety** — No undefined behavior, checked errors in debug mode.
6. **Practicality** — Designed around ZealPascal Game Engine and ZealIDE.

### Hybrid Model Summary

* **Records = data** (used everywhere in ECS, physics, animation, tilemaps)
* **Classes = behavior controllers** (Scenes, Game, UI, scripting-bound objects)

This hybrid model replaces traditional heavy OOP with a **clean, modern design**.

---

# 1. Language Tier Structure

SuperPascal is defined across **three conceptual tiers**, ensuring clarity for teaching and progressive mastery. **All Tier 2 and Tier 3 features exist in the language**, but are not taught until students master Tier 1 fundamentals.

## 1.1 Tier 1 — ISO 7185 Pascal Core (Foundation)

**Taught in:** Beginner curriculum  
**Available in:** All compilers (micro-compiler and full compiler)

Provides:

* Primitive types: `integer`, `boolean`, `char`, `byte`, `word`
* Enumerations and subranges
* Constants and variables
* Arrays and records
* Basic pointers
* Control flow structures
* Simple procedures and functions
* Program block structure
* Basic I/O (`Write`, `WriteLn`, `Read`, `ReadLn`)

This tier is used for early education and console-level programs. Students learn these fundamentals before progressing to advanced features.

## 1.2 Tier 2 — Turbo Pascal Extensions + Advanced Features (Practical Pascal)

**Taught in:** Advanced curriculum (after Tier 1 mastery)  
**Available in:** Full compiler only

Introduces:

* **Units** for modular compilation
* **Short strings** (unified `string` type in Tier 1, `ansistring`/`unicodestring` in Tier 2)
* **Sets**, **packed records**
* Improved file I/O via Zeal OS
* Compiler directives (`{$INLINE}`, `{$UNROLL}`, etc.)
* ZealPascal Tilemap DSL
* **Advanced integer types:** `shortint`, `smallint`, `longint`, `cardinal` (not taught in Tier 1)
* **Advanced string types:** `ansistring`, `unicodestring` (manual memory management)
* **Generics/Templates** (available but not taught until Tier 2)
* **Operator overloading** (available but not taught until Tier 2)
* **Type helpers** (available but not taught until Tier 2)
* **Attributes/Annotations** (available but not taught until Tier 2)

This tier covers everything needed for real-world programs and early games. **All features exist in the compiler**, but are introduced progressively after Tier 1 mastery.

## 1.3 Tier 3 — ZealPascal OOP Subset (Behavior Layer)

**Taught in:** Advanced curriculum (part of Tier 2)  
**Available in:** Full compiler only

Adds:

* Lightweight `class` types
* Single inheritance
* Virtual/override methods
* Constructors and destructors
* Optional RTTI (future)

**Important:** classes orchestrate systems and behavior, but do **not** hold ECS data.

**Note:** Tier 3 is conceptually part of Tier 2 - it's the OOP layer within the advanced features.

---

# 2. Types and Data Model

## 2.1 Primitive Types

### Tier 1 Types (Taught First)

* `integer` = signed 16-bit (-32768..32767)
* `byte` = unsigned 8-bit (0..255)
* `word` = unsigned 16-bit (0..65535)
* `boolean` = `true`/`false`
* `char` = 8-bit ASCII
* `string` = shortstring (1‑byte length + 0-255 chars)

### Tier 2 Types (Available, Taught Later)

* `shortint` = signed 8-bit (-128..127) - Advanced integer type
* `smallint` = signed 16-bit (same as `integer`, explicit size) - Advanced integer type
* `longint` = signed 32-bit (-2147483648..2147483647) - Advanced integer type
* `cardinal` = unsigned 32-bit (0..4294967295) - Advanced integer type
* `ansistring` = dynamic string with manual memory management - Advanced string type
* `unicodestring` = Unicode string with manual memory management - Advanced string type

**Educational Note:** Tier 1 students learn core types (`byte`, `integer`, `word`, `string`). Tier 2 students learn when and why to use advanced types (`shortint`, `longint`, `ansistring`, etc.).

## 2.2 Composite Types

* `record` (struct-like, no inheritance)
* `array`
* `set` (bitset)
* `^T` pointers
* `class` types

### Records

Records are the backbone of the ECS and all data modeling:

```pascal
type
  TVec2 = record
    X, Y: integer;
    procedure Add(const B: TVec2);
  end;
```

Methods compile as standalone functions.

### Classes

Used mainly for orchestration:

```pascal
type
  TEntity = class
    ID: word;
    procedure Update; virtual;
  end;
```

---

# 3. Program & Unit Structure

Programs:

```pascal
program Main;
begin
end.
```

Units:

```pascal
unit Player;
interface
  procedure InitPlayer;
implementation
  procedure InitPlayer;
  begin
  end;
end.
```

Units compile to `.ZOU` files.

---

# 4. Grammar Overview (Full EBNF in Appendix)

Key grammar groups:

* Program & unit declarations
* Types, variables, constants
* Procedures & functions
* Classes (restricted subset)
* Statements & expressions
* Intrinsics (`WaitVBlank`, `SpriteSet`, etc.)
* Tilemap DSL constructs
* Compiler directives

See **Appendix A** for the full EBNF (the v0.2 grammar merged here).

---

# 5. Memory Model

ZealPascal uses:

* Stack frames built around `IX`
* Static global memory
* Heap allocation for class instances
* ECS data stored in page-based arenas
* Record methods use by-value semantics
* Class methods use reference semantics

---

# 6. Hybrid OOP + Struct Model (Finalized Specification)

## 6.1 Records Own Data

* ECS components are record arrays
* No polymorphism
* Fast SoA iteration
* Perfect for Z80 optimizations

## 6.2 Classes Own Behavior

* Reference types with inheritance
* Lightweight, minimal vtables
* Used for Scenes, Widgets, Controllers, etc.

## 6.3 Interoperation

Classes reference ECS IDs:

```pascal
TPlayerController = class
  ID: TEntityID;
  procedure Update; override;
end;
```

## 6.4 Educational Simplicity

Beginner subset hides:

* Classes
* Pointers
* Directives

Kids start with structured programming → ECS → OOP orchestration

---

# 7. ZealPascal Extensions

## 7.1 Tilemap DSL

Allows clean tilemap definitions.

## 7.2 Intrinsics

Hardware-level operations mapped to simple Pascal calls.

## 7.3 ZealPascal-specific Directives

* `{$INLINE}`
* `{$UNROLL}`
* `{$VBLANK_AWARE}`

---

# 8. Cross-Tier Integration

The language supports and powers:

* ZVB graphics API
* Audio system
* Physics engine
* ECS architecture
* Scripting
* File system
* Scene editor
* Tooling pipelines (ZealIDE)

---

# 9. Appendix A — Full Grammar (Merged v0.2 EBNF)

(Insert full grammar block here — identical to v0.2 document, but formatted for publication. In the next iteration, we can refine and expand precedence tables.)

---

# 10. Appendix B — Future Specification Layers

* v0.3 Semantic Rules
* v0.4 ABI & Codegen
* v0.5 Standard Library
* v0.6 Compliance Suite
* v1.0 Official Language Standard

---

# ✔️ ZealPascal Language Specification (Combined v0.3 Draft) Complete

This unified document now reflects:

* Tiered Pascal evolution
* Hybrid OOP + Struct model
* Cleaned, modernized syntax
* Full grammar coverage
* Game engine integration
* Educational friendliness

Ready for iteration into the **v0.3 Semantic Rules Specification**.
