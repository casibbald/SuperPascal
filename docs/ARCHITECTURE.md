# 📘 **SuperPascal Compiler (`compiler-rs`)**

## *Module-Level Overview of the Rust-Based SuperPascal Compiler*

The SuperPascal compiler (`zc`) is implemented in Rust and outputs **Z80 assembly or .ZOF object files** that run on the **Zeal 8-bit Computer**.

The compiler is structured into clean, testable modules, following a pipeline from **source text → tokens → AST → IR → Z80 code → object files**.

This guide explains the responsibility of each module, their inputs/outputs, and how they fit together.

---

# 🧱 **High-Level Architecture Diagram**

```
┌────────┐    ┌─────────┐    ┌────────────┐    ┌─────────┐    ┌────────────┐
│  lexer │──▶ │ parser  │──▶ │ semantics  │──▶ │   IR     │──▶ │ codegen-z80 │
└────────┘    └─────────┘    └────────────┘    │ builder │    └────────────┘
                                                   │
                                                   ▼
                                         ┌────────────────┐
                                         │  object_writer │
                                         └────────────────┘
                                                   │
                                                   ▼
                                              final .ZOF
```

---

# 📦 **Crate Structure**

Inside `crates/compiler-rs`:

```
src/
  lexer/
  parser/
  ast/
  tokens/
  semantics/
  types/
  symbol_table/
  ir/
    ir1/
    ir2/         # optional future
  codegen/
    z80/
  runtime_specs/
  object/
  errors/
  diagnostics/
  driver/
tests/
```

---

# 🧩 **Module-by-Module Overview**

Below is the canonical responsibility breakdown.

---

# **1. `lexer/`**

**Responsibility:** Convert raw source text → token stream.

### Features

* Deterministic, single-pass scanner
* Full token set (keywords, identifiers, literals, operators, directives)
* Tilemap DSL tokenization (`ROW("…")`)
* Produces `Token { kind, lexeme_span, line, col }`
* Zero allocation (optional string pool)

### Outputs

`Vec<Token>` or streaming iterator

---

# **2. `tokens/`**

**Responsibility:**
Define token types, keyword tables, operator enums, literal representations.

### Contents

* `TokenKind` enum
* `Keyword` map
* `Operator` enum
* `DirectiveToken` structures

---

# **3. `parser/`**

**Responsibility:** Produce a **typed AST** according to v0.2 grammar.

### Design:

* Recursive descent (or Pratt for expressions)
* Well-defined grammar alignment
* Error recovery via panic-mode or synchronization tokens
* AST node factories inside `ast/`

### Handles:

* Program structure
* Units (interface/implementation)
* Declarations: const, type, var
* Records, arrays, sets
* Classes (Tier 3)
* Statements (if/for/while/repeat/case)
* Exceptions (try/except/finally)
* Expressions with precedence

### Output:

`ast::Program`

---

# **4. `ast/` (Abstract Syntax Tree)**

**Responsibility:** Define all AST node types and structures.

### Includes:

* `Program`, `Unit`, `Block`
* `Decls`: VarDecl, TypeDecl, ConstDecl
* `Types`: RecordType, ArrayType, ClassType
* `Expr`: Binary, Unary, Literal, Call, Field, Index
* `Stmt`: Assign, If, For, While, Case, Try, Raise

### Characteristics:

* Mostly POD structs + enums
* No semantic meaning yet
* Arena-allocated or boxed nodes
* Preserves source spans for diagnostics

---

# **5. `symbol_table/`**

**Responsibility:** Track bindings, scopes, visibility, symbol kinds.

### Features:

* Nested scopes (stack model)
* Unit-level interface/implementation separation
* Class member tables
* VTable symbol tracking
* Type aliases
* Overloadable function signatures

### Outputs:

Resolved symbol references for semantic layer.

---

# **6. `types/`**

**Responsibility:** Define SuperPascal’s **semantic** type model.

### Types handled:

* Primitive types
* Enums/subranges
* Records
* Arrays (static)
* Sets
* Pointers
* Class references
* Function/procedure types

### Also includes:

* Layout information (field offsets, size)
* Ordinal conversions
* Array bounds

---

# **7. `semantics/`**

**Responsibility:** Enforce all rules from v0.3 Semantic Specification.

### Features:

* Type checking
* Constant folding
* Control flow analysis of try/finally    \n- Method resolution (`virtual`/`override`)
* Constructor/destructor correctness
* Unit-level symbol visibility
* Assignment compatibility
* Class inheritance rules
* Exception unwinding semantics verification

### Outputs:

* Annotated AST
* Typed expressions
* Semantic error list

---

# **8. `errors/` + `diagnostics/`**

**Responsibility:** High-quality error messages.

### Includes:

* Error codes
* Source span formatting
* Suggestion messages (“did you mean…?”)
* Multi-span diagnostics
* Coloring and IDE-friendly outputs

Critical for education.

---

# **9. `ir/`**

### **`ir1/` — Linear IR (Primary IR)**

**Responsibility:** Translate AST → IR1 (SSA-like where possible).

IR instructions include:

* MOV, LOAD, STORE
* ADD, SUB, MUL, DIV
* CMP, JUMP, CJUMP
* CALL, RET
* VTableLookup (lowered later)
* ExceptionFramePush/Pop
* Raise

### Goals:

* Lower-level than AST
* Not architecture-specific
* Structured into basic blocks

### Optional Future:

**`ir2/` (Three-address)** for advanced optimizations.

---

# **10. `codegen/z80/`**

**Responsibility:** Lower IR1 → Z80 machine code or assembly.

### Must support:

* ABI v0.4 (stack frames, IX, return rules)
* Parameter stack layout
* Record layout rules
* Class instance layout rules
* Vtable generation
* Exception handler table emission

### Optimizations:

* Peephole
* Strength reduction
* Loop invariant motion
* ECS-aware loop unrolling (Phase 7)

### Output:

* `.asm` (sjasmplus)
* Or `.ZOF` object format (binary sections + relocations)

---

# **11. `object/`**

**Responsibility:** Emit fully linkable binaries for the Zeal dev environment.

Supports:

* Code section
* Data section
* BSS section
* Symbol table
* Relocation entries
* Vtable metadata
* Unit initialization list

Outputs:

* `.ZOF` object files
* `.ZOU` unit binaries
* Human-readable `objdump` for debugging

---

# **12. `runtime_specs/`**

**Responsibility:** Describe runtime helper calls needed by codegen.

Includes:

* MUL/DIV/MOD helpers
* Exception helper routines
* Memory allocation calls
* String operations

Also ensures:

* ABI compatibility
* Stable function signatures

---

# **13. `driver/`**

**Responsibility:** The command-line frontend for the compiler.

Supports:

```
zc build program.pas
zc compile file.pas -o file.zof
zc asm file.pas
zc unit file.pas
zc emit-ir file.pas
zc emit-ast file.pas
zc check file.pas
zc help
```

Integrates with:

* Zeal dev environment
* Assembler
* Linker
* Emulator (optional)

---

# **14. `tests/`**

The full compliance suite:

* Syntax tests
* Semantic tests
* ABI tests
* Stdlib compilation tests
* Engine integration tests
* Negative tests (expected errors)
* Performance baselines

Each compiler change must pass all relevant tests.

---

# 🧠 How the Modules Fit Together

High-level flow:

```
Lexer → Parser → AST → Semantics → IR1 → Z80 Codegen → Object Writer
```

And with units:

```
Unit Parser → Symbol Export → .ZOU Writer
```

And with exceptions:

```
Semantics checks → IR Exception Ops → Z80 Unwind Helpers
```

And with engine:

```
Stdlib+Engine Pascal → zc → .ZOF → Linker → Zeal binary
```

---

# 🧩 Contribution Tips

* Start with **lexer**, **parser**, or **errors** if new to compiler development
* Work on **stdlib** or **engine** if new to SuperPascal
* Follow specs closely (v0.1–v0.8)
* Always write tests for new features
* Ask for help when unsure—compiler design is complex

---

