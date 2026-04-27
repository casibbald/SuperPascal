# 📘 ZealPascal v0.8 — Compiler Architecture Implementation Plan

## From Spec to Running Compiler on the Zeal 8-bit Computer

This document translates the ZealPascal language specs (v0.1–v0.7) into a **practical, staged implementation plan** for the compiler and supporting tools.

It covers:

* Overall compiler architecture
* Internal representations (tokens, AST, IR)
* Pass structure and modularization
* Integration with runtime, linker, and IDE
* Implementation phases and milestones

The target is a **self-hosting ZealPascal compiler** running on Zeal hardware, with initial development occurring on a modern host (cross-compilation) then migrated to Zeal.

---

# 1. High-Level Architecture

The ZealPascal compiler is structured into the following layers:

1. **Frontend**

    * Lexer
    * Parser
    * AST builder

2. **Semantic Analyzer**

    * Symbol tables
    * Type checker
    * Const evaluation

3. **IR Generator**

    * Linear IR (IR1)
    * Three-address IR (IR2) for certain constructs

4. **Optimizer** (Initial: minimal, then incremental)

    * Constant folding
    * Local peephole optimizations

5. **Backend**

    * Z80 code generator
    * ABI & calling convention adherence
    * Emitting `.ZOF` object files

6. **Linker Integration**

    * External tool or integrated pass
    * Produces `.bin` and `.ZOU`

7. **Tooling Hooks**

    * Debug info emission (`.ZDBG`)
    * IDE integration (error format, location mapping)

---

# 2. Internal Representations

## 2.1 Tokens

Token fields:

* Kind (keyword, ident, literal, operator)
* Lexeme index (string pool)
* Line/column

Token stream feeds the parser and supports error recovery.

## 2.2 AST (Abstract Syntax Tree)

AST nodes represent:

* Declarations (types, vars, consts, routines, classes)
* Statements (assign, call, loop, conditionals, try blocks)
* Expressions (binary/unary operations, literals, identifiers)

Nodes stored in paged arenas for large programs.

## 2.3 IR1 (Linear IR)

A low-level representation close to Pascal but more regular:

* Basic blocks
* Simple operations: `MOV`, `ADD`, `CALL`, `JMP`, `CJMP`

## 2.4 IR2 (Optional Three-Address Form)

Used for:

* Complex optimization
* Advanced features (e.g., future generics)

Initially, IR2 can be omitted or limited to hot paths.

---

# 3. Compiler Passes

## 3.1 Lexing

* Implemented as a DFA over input characters
* Handles comments, directives, and literals

## 3.2 Parsing

* Recursive descent parser based on v0.2 grammar
* Builds AST with minimal backtracking
* Performs basic syntax error recovery

## 3.3 Semantic Analysis

* Populates symbol tables
* Resolves types and overloads
* Enforces hybrid OOP/struct rules
* Validates Tilemap DSL
* Computes constant expressions

## 3.4 IR Generation

* Converts AST to IR1
* Applies constant folding and simplifications

## 3.5 Code Generation

* Lowers IR1 to Z80 assembly or machine code
* Applies calling convention and ABI rules

## 3.6 Optimization (Phase 2+)

* Peephole optimizer over generated code
* Loop unrolling for ECS patterns
* VBlank-aware scheduling hints

---

# 4. Module & File Layout (Host Implementation)

On host (e.g., Rust or C++ implementation):

* `lexer/`
* `parser/`
* `ast/`
* `semantics/`
* `ir/`
* `codegen_z80/`
* `linker/`
* `driver/` (CLI front end)

Each module compiled as a library or part of a monolith initially, later split if needed.

---

# 5. Implementation Phases

## Phase 1 — Minimal Core Compiler (Host-Only)

Scope:

* Subset of Tier 1 (integers, booleans, arithmetic, simple procs)
* Single-file compilation
* No classes, no units, no exceptions

Deliverables:

* `zc` binary (ZealPascal Compiler) on host
* Can compile tiny programs to Z80 assembly

## Phase 2 — Full Tier 1 & Basic ABI

Scope:

* Full core Pascal
* Arrays, records, sets
* Calling convention adherence
* `.ZOF` object file output

Deliverables:

* Object files linkable with a simple linker
* Pass key v0.6 core tests for Tier 1

## Phase 3 — Units and Standard Library Integration

Scope:

* Support `unit` syntax and separate compilation
* Integrate `System`, `Math`, `Strings`, `SysUtils_Lite`
* Produce `.ZOU` unit files

Deliverables:

* ZealPascal stdlib builds successfully
* User programs link against stdlib units

## Phase 4 — OOP & Hybrid Model

Scope:

* Implement classes with single inheritance
* Generate vtables and adhere to v0.4 layout
* Enforce ECS data rules in semantics layer

Deliverables:

* Engine core (Scenes, ECS wrappers) compiles and runs

## Phase 5 — Exceptions & Advanced Features

Scope:

* Implement try/except/finally
* RaiseError integration
* Exception unwinding via ABI spec

Deliverables:

* Exception tests from v0.7 pass

## Phase 6 — Fixed-Point & Trig Libraries

Scope:

* Implement `MathFP`, `MathTrig`, `MathFloat`
* Integrate LUTs and codegen intrinsics

Deliverables:

* Math & trig tests pass

## Phase 7 — Self-Hosting Path

Scope:

* Write a subset of compiler in ZealPascal
* Compile that subset with host compiler
* Run on Zeal hardware

Deliverables:

* Zeal-hosted compiler able to compile simple ZealPascal programs

---

# 6. Tooling Integration

## 6.1 CLI Driver

`zc` front-end:

* `zc file.pas` → compile & link
* Options for:

    * debug vs release
    * target (host vs Zeal)
    * output formats (`.ZOF`, `.ZOU`, `.bin`)

## 6.2 ZealIDE Integration

Compiler must:

* Emit errors as `file:line:col: message`
* Provide debug info for `.ZDBG`

ZealIDE:

* Calls `zc` with project config
* Parses compiler output
* Highlights errors

---

# 7. Runtime & Linker Cooperation

The compiler and linker must agree on:

* Name mangling
* Section layout (code, data, bss)
* Vtable locations
* Init/finalization routines per unit

The linker may be:

* An external tool
* Or a simple integrated tool for educational bundles

---

# 8. Testing & CI Strategy

Each phase ties into v0.6 Compliance Suite.

CI jobs:

* Build host compiler
* Run unit tests for each module
* Run conformance tests on emulator
* Optionally, run on real hardware via harness

---

# 9. Performance Considerations

Key hotspots to monitor:

* ECS loops
* Math & trig routines
* VBlank timing
* Exception overhead (when enabled)

Compiler feedback (warnings) may help users avoid slow patterns.

---

# ✔️ ZealPascal Compiler Architecture Implementation Plan v0.8 — Draft Complete

This plan maps the **language & runtime specifications** to a concrete implementation strategy:

* Stepwise compiler build-out
* Clear module boundaries
* Host-first, then self-hosting
* Full integration with the engine and IDE

It is now ready to guide actual development of the ZealPascal compiler and associated tools.
