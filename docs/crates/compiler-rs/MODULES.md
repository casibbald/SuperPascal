# 📦 **Module Structure Overview — SuperPascal Rust Compiler (`compiler-rs`)**

The compiler is divided into **14 modules**, each responsible for one stage of the pipeline from source text → tokens → AST → semantics → IR → Z80 machine code → `.ZOF` object file.

```
src/
 ├── lexer/
 ├── tokens/
 ├── parser/
 ├── ast/
 ├── semantics/
 ├── types/
 ├── symbol_table/
 ├── ir/
 │    ├── ir1/
 │    └── ir2/        (optional future)
 ├── codegen/
 │    └── z80/
 ├── object/
 ├── runtime_specs/
 ├── errors/
 ├── diagnostics/
 └── driver/
```

---

# 1. **`lexer/` — Lexical Analysis**

**Purpose:** Convert raw source text → sequence of tokens.

**Key responsibilities:**

* Scan identifiers, literals, operators, keywords
* Support Pascal-style comments (`{ }`, `(* *)`, `//`)
* Handle compiler directives (`{$...}`)
* Tokenize tilemap DSL (`ROW("xxxx")`)

**Outputs:** `Vec<Token>` or token iterator
**Tests:** token patterns, malformed input, directive handling

---

# 2. **`tokens/` — Token Definitions**

**Purpose:** Define all tokens used throughout the compiler.

Includes:

* `TokenKind` enum
* tables for `Keyword → TokenKind`
* operator definitions
* literal storage models

This module is pure data + helpers.

---

# 3. **`parser/` — Syntax Analysis**

**Purpose:** Convert tokens → structured AST.

Supports full SuperPascal grammar:

* program + unit structure
* declarations
* statements
* exceptions
* classes + methods
* expressions with full precedence
* tilemap DSL

**Strategy:** recursive descent + Pratt expressions
**Outputs:** `ast::Program`
**Tests:** grammar conformance, parse trees, error recovery

---

# 4. **`ast/` — Abstract Syntax Tree**

**Purpose:** Define typed AST node structures.

Contains:

* `Program`, `Unit`, `Block`
* `Decl` types (VarDecl, TypeDecl, ProcDecl, etc.)
* `Expr` types (Binary, Unary, Call, Literal, Field, Index…)
* `Stmt` types (Assign, If, For, While, Case, Try, Raise…)

**Key properties:**

* Arena-allocated for performance
* Immutable once built
* Stores source spans for error messages

---

# 5. **`symbol_table/` — Scopes & Symbol Tracking**

**Purpose:** Resolve identifiers to declarations and enforce visibility.

Supports:

* Global scope
* Unit interface scope
* Nested procedure/function scopes
* Class member scope (fields + methods)
* VTable slot binding

**Outputs:** resolved symbol entries for semantics.

---

# 6. **`types/` — Semantic Types**

**Purpose:** Implement full SuperPascal type system.

Types include:

* primitives
* enums, subranges
* arrays
* records
* sets
* pointers
* classes
* procedures/functions

Also responsible for:

* type equality rules
* record + class layout sizes and offsets
* cardinality of sets

---

# 7. **`semantics/` — Semantic Analysis**

**Purpose:** Apply all rules from v0.3 specification.

Tasks:

* Resolve identifiers
* Check type correctness
* Enforce assignment compatibility
* Resolve overloads
* Rule-check classes (inheritance, virtual/override)
* Validate try/except/finally
* Constant folding

**Outputs:**

* typed AST
* annotated expressions
* list of semantic errors

---

# 8. **`errors/` + `diagnostics/` — Error Handling**

**Purpose:** Human-friendly compiler diagnostics.

Implements:

* error types
* spans, highlights, multi-span messages
* hints and suggestions
* IDE-friendly formatting
* “teaching mode” (more verbose)

---

# 9. **`ir/` — Intermediate Representation**

Two layers:

## **`ir/ir1/` (Primary IR)**

**Purpose:** A linear, low-level, architecture-neutral IR.

IR instructions:

* MOV, LOAD, STORE
* ADD, SUB, MUL, DIV
* CMP, JUMP, CJUMP
* CALL, RET
* VTABLE_LOAD
* EXCEPTION_FRAME_PUSH/POP
* RAISE

**Design goals:**

* simple to generate
* representative of Pascal semantics
* easy to lower to Z80

## **`ir/ir2/` (Optional)**

Three-address IR for future optimizations.

---

# 10. **`codegen/z80/` — Z80 Backend**

**Purpose:** Lower IR → Z80 assembly or machine code.

Implements:

* calling convention (ABI v0.4)
* stack frame creation
* parameter passing
* return value handling
* record/class layout rules
* vtable emission
* exception handler block lowering
* peephole optimizer

Outputs:

* `.asm` suitable for sjasmplus
* or `.ZOF` object sections

---

# 11. **`object/` — Object File Writer**

**Purpose:** Write binary sections for the Zeal dev environment.

Supports:

* code section
* data section
* bss
* symbol tables
* relocations
* vtable metadata
* unit metadata (for .ZOU)

Output files:

* `.ZOF` (object file)
* `.ZOU` (unit file)

---

# 12. **`runtime_specs/` — Runtime Contract**

**Purpose:** Define call signatures for runtime helpers used by codegen.

Includes:

* Multiply/divide/mod helpers
* Exception helper routines
* Memory allocation calls
* String operations

This ensures backend and runtime remain ABI-compatible.

---

# 13. **`driver/` — Command-Line Interface**

**Purpose:** Provide the `zc` executable interface.

Commands:

```
zc build main.pas
zc compile file.pas -o file.zof
zc asm file.pas
zc emit-ir file.pas
zc emit-ast file.pas
zc check file.pas
```

Handles:

* orchestrating all compiler stages
* coordinating assembler/linker calls
* output formatting

---

# 14. **`tests/` — Compliance & Regression Suite**

Contains:

* syntax tests
* semantic tests
* ABI tests
* stdlib build tests
* engine build tests
* negative/error message tests
* math/trig precision tests
* performance baselines

All contributions must maintain or expand test coverage.

---

# 🔄 **Module Interactions Overview**

### Core sequence:

```
lexer → parser → ast → semantics → ir → codegen → object_writer → ZOF
```

### Runtime integration:

* Codegen emits calls defined in `runtime_specs`
* Runtime implements ABI-compatible helpers

### Unit system:

* parser builds units
* symbol_table stores exports
* object_writer emits .ZOU metadata

### Engine:

* engine code compiled via zc
* depends on stdlib

---

# 🧭 **Contribution Advice by Module**

### If you are new:

Start with `lexer`, `parser`, or `errors`.

### If you are a compiler engineer:

`semantics`, `ir`, `codegen/z80`.

### If you like performance:

Work inside the peephole optimizer or ECS-aware unrolling strategies.

### If you prefer user experience:

Improve diagnostics and error messages.

