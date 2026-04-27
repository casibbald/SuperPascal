# 🧩 **Contributing to SuperPascal**

Thank you for your interest in contributing to **SuperPascal**, a modern educational Pascal superset compiler and toolchain supporting multiple 8-bit platforms.

SuperPascal is a large, multi-layered project involving:

* A **Rust-based compiler** targeting multiple retro computing platforms (Z80, WDC65C816, MC68LC060)
* A **runtime & standard library**
* A **2D game engine**
* An **IDE for teachers and students**
* A full **36-week curriculum** and **educational materials**

This guide explains how contributions work, what parts of the system you can help with, and how to begin.

---

# 📘 1. Project Structure

```
/compiler-rs       → Rust compiler source
/runtime           → SuperPascal runtime (Z80)
/stdlib            → Standard library (Pascal)
/engine            → ECS & game engine (Pascal)
/docs              → Specifications, curriculum, references
/tests             → Compliance suite
/examples          → Demo programs, games, math visualizers
```

---

# 🧠 2. Understanding the Architecture

Before contributing, we strongly recommend reading:

* **Language Specification (v0.1–v0.7)**
* **Compiler Architecture Plan (v0.8)**
* **ABI & Codegen Specification (v0.4)**
* **Standard Library Specification (v1.0)**
* **Compliance & Test Suite (v0.6)**
* **SuperPascal Teacher Guide + Curriculum**

These form the foundation of the compiler, runtime, stdlib, and engine.

---

# 🦀 3. Compiler Development (Rust)

The production compiler is implemented in Rust.
Tasks in this area include:

### 🧩 Frontend

* Lexer/tokenizer
* Parser (recursive descent or Pratt)
* AST structure
* Error recovery & diagnostic messages

### 🧮 Semantic Analyzer

* Type checker
* Scope & symbol table logic
* Class + record semantics
* Unit/interface export/import
* Exception semantics

### ⚙️ IR & Codegen

* IR1 (linear IR) construction
* Z80 backend (ABI-compliant)
* Peephole optimizer
* Code emission & object file generation

### 🧪 Testing

* Expand test suite
* Fuzz parser & lexer
* Add negative tests

---

# 📚 4. Standard Library Development (Pascal)

Contributions are welcome in:

### Core libraries

* `System`
* `Math`
* `MathFP`, `MathTrig`
* `Strings`
* `SysUtils_Lite`

### Hardware libraries

* Graphics (ZVB)
* Audio
* Input
* Tilemap utilities

### Engine libraries

* ECS system
* Scene system
* Animator
* UI widgets
* Physics helpers

You may contribute:

* new units
* bug fixes
* performance improvements
* documentation

---

# 🎮 5. Game Engine Contributions

The SuperPascal engine includes:

* ECS (Entity Component System)
* Rendering pipeline
* Audio
* Input handling
* Scene system
* Script runtime

Ways to help:

* implementing new pipeline features
* improving ECS performance
* adding UI widgets
* expanding animation tools
* creating example projects

---

# ✏️ 6. Documentation & Curriculum

We welcome contributions to:

* Language reference
* Game engine API docs
* Student exercises and worksheets
* Teacher guides
* Example code for classroom use
* Slide decks & illustrations

Strong writing and teaching skills are valued as much as coding skills.

---

# 🧪 7. Compliance Test Suite

SuperPascal includes a full compliance suite (v0.6).
You can help by:

* Adding new tests
* Improving coverage
* Writing performance benchmarks
* Creating stress tests for Z80 output
* Adding math/trig accuracy cases

All compiler changes **must pass** the compliance suite.

---

# 🛠 8. Development Workflow

### 1. Fork the repository

### 2. Create a feature branch

Use descriptive names:

```
feature/parser-improvements
bugfix/z80-subtraction
stdlib/add-math-cot-function
doc/add-unit-tests-section
```

### 3. Run the full test suite

```
cargo test
cargo fmt
cargo clippy
```

### 4. Use Zeal dev environment to assemble & run sample binaries

Validate correctness in emulator.

### 5. Submit a Pull Request

PRs should include:

* Description of change
* Which spec sections it implements
* Relevant tests
* Any docs updated

### 6. Code Review

Compiler or engine maintainers will review.
We favor clarity, correctness, and maintainability.

---

# ✔️ 9. Coding Standards

### Rust (compiler)

* Use `clippy`
* Prefer clarity over cleverness
* Use enums for AST variants
* Use lifetimes sparingly; prefer owned clones where simpler
* Avoid unsafe code unless absolutely necessary

### Pascal (stdlib, engine)

* Follow SuperPascal formatting conventions
* Keep units focused and small
* Prefer records (structs) for data; classes for behavior
* Minimize global state

---

# 📦 10. Branching & Release Model

### `main`

Stable, tested, documented.
Always passes CI.

### `dev`

Active development, feature integration.

### `feature/*`

Short-lived branches for contributions.

### Releases

* **v1.0**: Educational toolkit release
* Subsequent versions follow semver with spec updates

---

# 📬 11. Communication & Discussion

* GitHub Issues → feature requests, bug reports
* GitHub Discussions → design questions, proposals
* Chat/Discord (coming later)
* Formal proposals for large features are welcome

---

# 🤝 12. Contributor Community Goals

We aim to build:

* A welcoming community
* A sustainable ecosystem
* A reliable educational platform
* A technically interesting compiler project

All contributors are expected to follow a future **CODE_OF_CONDUCT.md** (pending finalization).

---

# 🎉 Thanks for Contributing!

SuperPascal is an ambitious educational + technical project.
Your contributions will directly improve:

* Computer science education
* Retro programming culture
* Math and game development teaching
* The Zeal hardware ecosystem

We are excited to build this together.


