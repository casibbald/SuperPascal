# **SuperPascal**

### A Modern, Safe, Educational Pascal Superset for Retro Computing Platforms (8-bit, 16-bit, 32-bit)

**Compiler • Toolchain • Standard Library • Game Engine • Educational Platform • Multi-Platform**

---

## 🚀 Overview

**SuperPascal** is a modern Pascal superset compiler and educational toolchain supporting multiple retro computing platforms, including the **Zeal 8-bit Computer** (Z80 @ 10 MHz), **Foenix F256x systems** (WDC65C816S @ 6.29 MHz, 16-bit microprocessor with 24-bit addressing), and **Foenix A2560M** (MC68LC060 @ 66 MHz, 32-bit system).

It revives classic Pascal’s clarity while introducing:

* A **hybrid OOP + Struct** model suitable for both teaching and systems programming
* **ECS-friendly data layout** for high-performance game development
* **Hardware-accelerated math**, fixed-point arithmetic, and a full trigonometry suite
* **Modern compiler architecture** with well-defined IR, ABI, runtime, and object model
* A **full game engine** + ZealIDE for offline learning environments
* A complete **36-week curriculum** for schools and universities

SuperPascal is simultaneously:

* A **research platform** for compiler design
* A **lab environment** for building 8-bit games
* A **STEM education stack** for classrooms without internet access

---

## 🎯 Project Status

**Language Specifications:** v0.1–v0.7 complete
**Compiler Architecture:** v0.8 complete
**Curriculum / Teacher Guide:** v1.0 drafts complete
**Standard Library:** v1.0 draft complete
**Compliance Suite:** v0.6 complete
**Exceptions Model:** fully specified

**Compiler implementation**:
🟡 *Spec-complete, implementation pending*

This repository hosts specifications, architecture documents, and future reference implementations.

---

## 🧩 Key Features of SuperPascal

### ✔ Clean, Modern Pascal

* Deterministic semantics
* Beginner-friendly
* Single, unified string type
* Clear error messages & debug support

### ✔ Hybrid OOP + Struct Model

* **Records (structs)**: primary data carrier, optimized for SoA storage
* **Classes**: lightweight behavior containers with small vtables
* Ideal for ECS-driven game logic

### ✔ Game Development Built-In

* Tilemaps, sprites, animation
* Physics engine (AABB, gravity, velocity integration)
* Audio system (SFX, music, streaming, mixers)
* Scene system and scripting language
* Profiling, debugging, and error reporting

### ✔ Advanced Math Stack

* Fixed-point (`Q8.8`, `Q12.12`)
* Full trig suite: **sin, cos, tan, sec, csc, cot, arcsin, arccos, arctan**
* Optional Float32 for scientific computing
* Optional FPGA-accelerated trig and vector math

### ✔ Designed for Education

* 3 × 12-week teaching curriculum
* Teacher Guide v1.0
* Marketing + outreach materials for STEM adoption
* Offline-first deployment

---

## 🛠 Architecture Overview

### **Compiler Phases**

1. **Lexer** → token stream
2. **Parser** → AST (recursive descent)
3. **Semantic Analyzer** → symbol tables, type inference, hybrid rules
4. **IR Generation**

    * IR1: linear, low-level Pascal
    * IR2: optional three-address form
5. **Z80 Codegen**
6. **Optimizer** (peephole, constant folding, loop unrolling)
7. **Linker** → `.ZOF`, `.ZOU`, `.ZPK` bundles

### **ABI**

* Pascal calling convention
* `IX` frame pointer
* `IY` exception frame chain
* Class layout: vtable at offset 0
* Record methods lowered to free functions
* Large returns via hidden pointer parameter

### **Runtime**

* Page-aware heap manager
* No GC / predictable memory usage
* Debug checks (bounds, overflow)
* Intrinsics for hardware operations

---

## 📦 Repository Structure (Recommended)

```
/docs
  /specs
    LanguageSpec_v0.x.md
    Grammar_v0.2.md
    Semantics_v0.3.md
    ABI_Codegen_v0.4.md
    StdLib_v1.0.md
    Exceptions_v0.7.md
    ComplianceSuite_v0.6.md
    CompilerArchitecture_v0.8.md
  /curriculum
  /teacher_guide
  /marketing

/compiler
  /lexer
  /parser
  /ast
  /semantics
  /ir
  /codegen
  /linker
  /runtime

/stdlib
/engine
/ide
/examples
```

---

## 🔧 Build Targets

**Target 1 — Host Compiler (Rust or C++):**

* Fast iteration
* CI integration
* Produces Zeal binaries

**Target 2 — Self-Hosting Compiler (SuperPascal → Zeal):**

* Long-term milestone
* Runs directly on Zeal hardware

---

## 📚 Documentation & Standards

The following documents define the official SuperPascal language and toolchain:

* **Language Spec v0.1–v0.3**
* **Grammar Spec v0.2**
* **Semantic Rules v0.3**
* **ABI + Codegen Spec v0.4**
* **StdLib Specification v1.0**
* **Exceptions Model v0.7**
* **Compliance & Test Suite v0.6**
* **Compiler Architecture Plan v0.8**
* **Game Engine PRD**
* **Graphics & Audio PRDs**

These documents provide a full blueprint for implementing or validating a SuperPascal compiler.

---

## 🧪 Compliance & Testing

A complete **Compliance & Test Suite** (v0.6) ensures:

* Parser conformance
* Semantic consistency
* ABI stability
* Stdlib correctness
* Hardware integration correctness (ZVB graphics/audio/input)
* ECS and game engine behavior
* Performance baselines (60 FPS target)

This suite is required for any independent or alternate compiler implementation.

---

## 🤝 Contributing

We welcome contributions from:

* Compiler engineers
* Language designers
* University researchers
* FPGA & hardware engineers
* Game engine developers
* Educators and curriculum designers

### Areas needing contributions:

* Lexer/parser implementation
* Semantic analyzer infrastructure
* IR + codegen backend
* FPGA math co-processor exploration
* IDE integration tooling
* Stdlib implementation
* Test suite automation

**📖 See [CONTRIBUTING.md](CONTRIBUTING.md) for:**
- Development setup instructions
- The FreePascal "jerryrig" (why we maintain a patched FPC fork)
- Building and using the patched compiler
- Testing and submission guidelines

Open a PR or start a discussion thread to get involved.

---

## 📜 License

To be determined. Suggestion:

* MIT for compiler + stdlib
* Creative Commons for curriculum + teaching materials

---

## 🧭 Roadmap

* ✔ v0.1–v0.8 specifications complete
* ☐ Compiler implementation (host-first)
* ☐ Stdlib + runtime implementation
* ☐ ZealIDE integration
* ☐ v1.0 Toolkit Release
* ☐ Pilot School Deployment
* ☐ Full educational expansion (workbooks, translations)

---

## 📬 Contact & Community

**Discord / Matrix / Mailing list** — coming soon
**Issues & discussions** on GitHub are open for proposals

