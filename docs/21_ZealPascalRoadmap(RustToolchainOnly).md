Here’s a **fresh, clean roadmap** that assumes:

* ✅ The **production compiler is written in Rust**, running on modern machines
* ✅ ZealPascal is **not** self-hosted on Zeal hardware
* ✅ Zeal students and teachers always use a **host-built cross-compiler** to generate Zeal binaries
* ✅ A future mini-compiler in Pascal (for university-level compiler courses) is optional and separate

No zc2/zc3, no compiler running on the Zeal machine.
Just a **solid, Rust-based cross-toolchain** that targets Zeal.

---

# 🚀 ZealPascal Roadmap (Rust Toolchain Only)

## *From zero → robust cross-compiler → full educational + game dev platform*

---

## PHASE 0 — Foundations & Repo Layout

### Goals

* Decide on language feature set (already done in specs v0.1–v0.7)
* Scaffold repos and directories
* Integrate with **zeal-dev-environment**

### Tasks

* [ ] Create root repo: `zealpascal`
* [ ] Add subdirectories:

    * `/compiler-rs` (Rust compiler)
    * `/runtime`
    * `/stdlib`
    * `/engine`
    * `/examples`
    * `/docs`
    * `/tests`
* [ ] Integrate zeal-dev-environment:

    * Add `Makefile` / scripts for:

        * compile .pas → Z80 asm
        * assemble → .bin
        * inject into Zeal image
        * run via emulator
* [ ] Set up CI (GitHub Actions or similar) for:

    * `cargo test`
    * simple Zeal build pipeline smoke test

**Deliverable:** Empty but wired-up project structure that can build and run a dummy Zeal app via Rust tooling.

---

## PHASE 1 — Minimal Rust-Based Compiler (MVP)

### Goals

* Get **any** ZealPascal subset compiled → Z80 code → running on Zeal
* This is the Rust version of the old “zc0”

### Language Subset

* `integer`, `boolean`
* `var` declarations
* `if/else`
* `while`
* `begin/end`
* procedures/functions without nested or forward declarations
* no classes, no records, no arrays, no units

### Compiler Features

* [ ] Lexer: identifiers, ints, basic operators, keywords
* [ ] Parser: program structure, basic statements, expressions
* [ ] Symbol table: globals + simple locals
* [ ] Codegen: Z80 asm, stack-based, ABI subset
* [ ] Minimal runtime stub:

    * entry point
    * math helpers: MUL/DIV/MOD
    * exit / halt

### Testing

* [ ] Hello world using debug output
* [ ] Basic arithmetic
* [ ] Simple conditionals and loops
* [ ] Factorial/fibonacci in integers

**Deliverable:** `zc` MVP that can compile trivial programs and run them on Zeal.

---

## PHASE 2 — Full Core Language (Tier 1)

### Goals

* Implement full **core Pascal** as per spec
* Solidify the type system, flow, and ABI

### Features

* [ ] Whole Tier-1 feature set:

    * full integer/boolean expressions
    * `for`, `repeat/until`
    * `case`
* [ ] External procedure/function declarations
* [ ] Proper scoping and shadowing
* [ ] Improved diagnostics (with source locations)

### Compiler Internals

* [ ] Complete recursive descent parser (or Pratt parser) for expressions
* [ ] Semantic pass:

    * types
    * flows
    * returns
* [ ] Linear IR1 and Z80 backend integrated

### Testing

* [ ] Full Tier-1 compliance tests
* [ ] Core math and control flow
* [ ] Simple library routines

**Deliverable:** `zc` can compile non-trivial core Pascal programs into reliable Zeal binaries.

---

## PHASE 3 — Types & Data Structures (Tier 2)

### Goals

* Add **records, arrays, sets, pointers, and units**
* Enable building structured systems & libraries

### Language Features

* [ ] Records (structs) with nominal typing
* [ ] Static arrays
* [ ] Sets (over small ordinal types)
* [ ] Pointers (`^T`), with controlled use for teaching
* [ ] Units:

    * `unit X; interface … implementation … end.`
    * Symbol export/import
    * Initialization sections

### Compiler & Backend

* [ ] Type layout for records and arrays
* [ ] Symbol visibility + unit linking model
* [ ] Generate `.ZOF` + `.ZOU` artifacts
* [ ] Sectioned assembly (code, data, bss)

### Standard Library Bootstrap

* [ ] Implement `System` in Pascal
* [ ] Implement `Math` (integer ops, basics)
* [ ] Implement `Strings` for unified short-string type

**Deliverable:** `zc` can build the core standard library and unit-based programs.

---

## PHASE 4 — OOP Layer (Tier 3, Hybrid Model)

### Goals

* Add **lightweight classes** used for orchestration, not data
* Support hybrid ECS + OOP style

### Features

* [ ] `class` definitions
* [ ] Single inheritance
* [ ] `virtual` / `override` semantics
* [ ] Constructors / destructors
* [ ] Method calls with implicit `Self`
* [ ] Integration with runtime (allocation)

### Compiler Internals

* [ ] Vtable generation
* [ ] Method slot resolution
* [ ] Virtual call lowering to vtable lookups
* [ ] Enforce “classes hold behavior, records hold data” style (static checks where possible)

### Engine Integration

* [ ] Scene classes
* [ ] AudioManager, UI widgets
* [ ] ECS controller/manager classes

**Deliverable:** `zc` can compile the higher-level engine API and class-based utilities.

---

## PHASE 5 — Exceptions & Error Handling

### Goals

* Provide clean `try/except/finally/raise` support
* Make teaching structured error handling possible

### Features

* [ ] `try … except … end`
* [ ] `try … finally … end`
* [ ] `raise`
* [ ] Integration with `SysUtils_Lite.EError`

### Implementation

* [ ] Exception frame chain modeling
* [ ] Rewriting try/except into appropriate calls and jumps
* [ ] Unwind logic in runtime
* [ ] ZealPascal-aware stack unwinding on Z80

### Testing

* [ ] Nested try/except
* [ ] try/finally correctness
* [ ] Rethrow scenarios

**Deliverable:** Structured error handling works for all hosted programs compiled via Rust compiler.

---

## PHASE 6 — Full Standard Library & Engine Implementation

### Goals

* Implement entire **StdLib v1.0 Rewrite** and **Game Engine** in ZealPascal
* All compiled by Rust compiler

### StdLib Units

* [ ] System
* [ ] Math, MathFP, MathTrig, MathFloat
* [ ] Strings
* [ ] SysUtils_Lite

### Hardware Units

* [ ] ZVB_Graphics
* [ ] ZVB_Sprites
* [ ] ZVB_Audio
* [ ] ZVB_Input

### Engine Units

* [ ] Engine_ECS
* [ ] Engine_Scene
* [ ] Script runtime
* [ ] Physics helpers
* [ ] UI widgets

### Integration

* [ ] Build scripts to compile stdlib + engine into distributable packages
* [ ] Automated tests that run sample games + math demos in emulator

**Deliverable:** The full ZealPascal game engine and standard library are built by the Rust compiler and runnable on Zeal.

---

## PHASE 7 — Optimization & ECS-Aware Codegen

### Goals

* Make code **fast enough** for real games on Z80
* Respect Zeal’s performance budgets

### Optimizations

* [ ] Peephole passes (useless moves, simple arithmetic combos)
* [ ] Constant folding & propagation
* [ ] Improved loop handling (strength reduction)
* [ ] ECS-aware unrolling for certain patterns (Position/Velocity loops)
* [ ] Smarter calling conventions for leaf functions (optional)

### Performance Targets

* [ ] 100–200 entities at 60 FPS in typical ECS loops
* [ ] Smooth scrolling tilemaps
* [ ] Audio without glitches

**Deliverable:** `zc` produces performant code suitable for teaching AND real-world 8-bit game dev.

---

## PHASE 8 — ZealIDE Integration & Developer Experience

### Goals

* Make `zc` the invisible engine behind a **smooth IDE experience**
* Provide documentation, error messages, and project templates

### Features

* [ ] ZealIDE integration:

    * compiler invocation
    * file association
    * click-to-error navigation
* [ ] Project templates for:

    * “Hello ZealPascal”
    * ECS samples
    * Platformer / shmup skeletons
    * Math visualizers

### Error Reporting

* [ ] Human-readable messages
* [ ] Short explanations and hints
* [ ] Possibly a “pedagogical mode” (more verbose errors)

**Deliverable:** ZealPascal is pleasant and approachable to use for normal humans.

---

## PHASE 9 — Educational Packaging & Toolkit Release

### Goals

* Turn ZealPascal into a **product** schools can adopt.

### Components

* [ ] Rust-based `zc` cross-compiler binaries for major OSes
* [ ] Prebuilt stdlib + engine binaries for Zeal
* [ ] SD card image for Zeal with all tools + demos
* [ ] Documentation bundle:

    * Language Reference
    * Teacher Guide
    * Curriculum
    * Student Book (ZealPascal text)
* [ ] Installer / simple instructions for host OS

**Deliverable:** “ZealPascal for Schools v1.0” toolkit.

---

## PHASE 10 — Optional: Mini-Compiler in Pascal (University Level)

**Not for production. Not self-hosting. Teaching-only.**

* [ ] Small, pure Pascal subset compiler written in ZealPascal
* [ ] Targets a simplified assembly / IR
* [ ] Used in advanced CS courses to teach compilers
* [ ] Compiled by Rust `zc` and run on Zeal or host for pedagogy

**Deliverable:** A CS-education-focused compiler project, separate from the production toolchain.

---

## Summary

We’ve now replaced the complex zc0/zc1/zc2/zc3 self-hosting story with a **single, clear path**:

* One production compiler in **Rust**
* Cross-compiling ZealPascal → Zeal
* Rich engine + math + curriculum
* Optionally, a *separate* educational Pascal compiler later

This matches your goals:

* ✅ Practical
* ✅ Maintainable
* ✅ School-friendly
* ✅ Compiler-designer-approved

If you want, I can now:

* Generate a **ROADMAP.md** file ready to drop in the repo
* Draft a **Developer FAQ** explaining “Why Rust; why not self-hosted”
* Define **milestone issues** for GitHub (Phase 1–5)
* Propose a **module layout for the Rust compiler** (crates, structs, traits)
