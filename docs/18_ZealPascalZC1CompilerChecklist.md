Below is the **complete engineering checklist for `zc1`**, the *full host-based ZealPascal compiler* that implements all **Tier 1, Tier 2, and Tier 3** features EXCEPT:

* self-hosting
* Zeal-native backend
* advanced optimizations (ECS-aware, hardware intrinsics)

`zc1` is the compiler that takes ZealPascal from a “toy bootstrap compiler” to a **real, usable programming language** that can compile:

* the ZealPascal Standard Library
* the ZealPascal Game Engine
* the ZealPascal Curriculum projects
* itself (later, in zc2)

This is the compiler that will be used by teachers, students, game creators, and Zeal IDE.

---

# 📘 **ZealPascal zc1 Compiler Checklist**

### *The complete Tier 1 + Tier 2 + Tier 3 host compiler implementation.*

---

# ✅ 0. Requirements for zc1

`zc1` MUST:

* Build and run on a modern host PC
* Compile all features from:

    * v0.1 Language Spec
    * v0.2 Grammar
    * v0.3 Semantics
    * v0.4 ABI
    * v0.5 StdLib
    * v0.7 Exceptions
* Emit valid `.ZOF` object files
* Produce `.ZOU` unit binaries
* Compile the full Standard Library
* Compile the ZealPascal Engine
* Pass the v0.6 Compliance Test Suite

`zc1` does **not** need to be able to run on Zeal hardware.

---

# ====================================================================================

# **SECTION 1 — FRONTEND SUBSYSTEM COMPLETION**

# ====================================================================================

# ✅ 1.1 Lexer Completion

Implement all tokens from full grammar:

* [ ] Identifiers
* [ ] Integer literals
* [ ] Boolean literals (`true`, `false`)
* [ ] Char literals
* [ ] String literals
* [ ] Operators:

    * +, -, *, /, mod, div
    * shl, shr
    * and, or, xor, not
    * =, <>, <, >, <=, >=
    * :=
    * ^
    * @
* [ ] Keywords:

    * program, unit, interface, implementation
    * const, type, var
    * record, array, set, packed
    * class, constructor, destructor
    * virtual, override
    * try, except, finally, raise
    * begin, end, if, then, else
    * while, repeat, until
    * for, to, downto
    * case, of
* [ ] Symbols:

    * (, ), [, ], ., ,, ;, :
* [ ] Comments `{ }`, `(* *)`, `//` (optional)
* [ ] Directives `{$...}`

### Special:

* [ ] Directive tokenizer
* [ ] DSL tokens for tilemaps: `ROW("...")`

---

# ====================================================================================

# **SECTION 2 — PARSER SUBSYSTEM COMPLETION**

# ====================================================================================

# ✅ 2.1 Implement full grammar from v0.2

Parser must support:

### Program & Unit structure

* [ ] program …
* [ ] unit … interface … implementation
* [ ] initialization/finalization blocks

### Declarations

* [ ] const
* [ ] type
* [ ] var
* [ ] records
* [ ] arrays
* [ ] sets
* [ ] pointer types
* [ ] classes
* [ ] methods (proc/func), constructors, destructors
* [ ] visibility sections
* [ ] virtual/override detection

### Statements

* [ ] assign
* [ ] call
* [ ] compound begin/end
* [ ] if/then/else
* [ ] case/of
* [ ] for/to/downto
* [ ] while/repeat
* [ ] try/except/finally
* [ ] raise
* [ ] with (*should parse but error unless disabled*)
* [ ] exit

### Expressions

* [ ] precedence climbing parser for:

    * calls
    * derefs
    * unary ops
    * binary ops
    * indexing (arr[i])
    * field access (`record.field`)
    * class member access (`obj.method`)

---

# ====================================================================================

# **SECTION 3 — SEMANTIC ANALYSIS COMPLETION**

# ====================================================================================

# 🎯 zc1 MUST implement **all** semantics from v0.3.

# ✅ 3.1 Symbol Tables

* [ ] Global table
* [ ] Unit interface table
* [ ] Per-scope tables
* [ ] Class method table
* [ ] Field table with offsets
* [ ] Built-in symbols

### Checks:

* [ ] Duplicate definitions
* [ ] Circular type definitions
* [ ] Missing symbols
* [ ] Invalid visibility access

---

# ✅ 3.2 Type Checker

Implement:

### Primitive types

* [ ] integer → word widening
* [ ] byte → integer widening
* [ ] boolean logic rules

### Composite types

* Records:

    * [ ] nominal typing
    * [ ] field lookup
* Arrays:

    * [ ] index type validation
    * [ ] element type compatibility
* Sets:

    * [ ] ordinal type restriction
* Pointers:

    * [ ] ^T referencing logic
* Classes:

    * [ ] inheritance chains
    * [ ] vtable construction
    * [ ] override compatibility
    * [ ] constructor rules
    * [ ] destructor rules

### Expressions

* [ ] binary operator type rules
* [ ] unary operator type rules
* [ ] constant folding
* [ ] literal widening

### Callable resolution

* [ ] overload resolution (Pascal-style simple)
* [ ] direct + virtual call differentiation
* [ ] static vs instance methods

### Exceptions

* [ ] semantic validation of try/except/finally
* [ ] ensure rethrow paths correct
* [ ] ensure destructors in finally run

---

# ====================================================================================

# **SECTION 4 — IR GENERATION COMPLETION**

# ====================================================================================

# zc1 must generate IR for **every Pascal construct**, including:

# ✅ 4.1 Statements

* [ ] assign (simple + compound lvalues)
* [ ] call (static, virtual)
* [ ] if/else
* [ ] while
* [ ] repeat until
* [ ] for
* [ ] case
* [ ] try/except
* [ ] try/finally
* [ ] raise
* [ ] exit

# ✅ 4.2 Expressions

* [ ] all operators
* [ ] short-circuit boolean logic
* [ ] array indexing
* [ ] pointer deref
* [ ] field access

# ✅ 4.3 Class Support

* [ ] vtable construction
* [ ] method slot indexing
* [ ] implicit `Self` injection
* [ ] constructor lowering
* [ ] destructor calls

# ✅ 4.4 Units

* [ ] IR for interface/implementation separation
* [ ] Symbol exports
* [ ] Initialization sections

---

# ====================================================================================

# **SECTION 5 — BACKEND (Z80) COMPLETION**

# ====================================================================================

zc1 must:

* [ ] Implement full ABI from v0.4
* [ ] Generate **.ZOF**
* [ ] Generate **relocations**
* [ ] Implement name mangling rules
* [ ] Emit separate sections for code/data/bss
* [ ] Emit class vtables in correct form
* [ ] Emit unit metadata

### Optimizations required for zc1:

* [ ] Constant folding
* [ ] Copy elision
* [ ] Peephole elimination of dead MOVs
* [ ] Loop-invariant hoisting (minimal)

---

# ====================================================================================

# **SECTION 6 — RUNTIME & STANDARD LIBRARY BOOTSTRAP**

# ====================================================================================

zc1 must compile the entire standard library from v1.0:

### Core:

* [ ] System
* [ ] Math
* [ ] MathFP
* [ ] MathTrig
* [ ] MathFloat
* [ ] Strings
* [ ] SysUtils_Lite

### Engine:

* [ ] ZVB_Graphics
* [ ] ZVB_Sprites
* [ ] ZVB_Audio
* [ ] ZVB_Input
* [ ] Engine_ECS
* [ ] Engine_Scene
* [ ] Script runtime

### Testing:

* [ ] All stdlib units compile cleanly
* [ ] All exported symbols are linkable
* [ ] All runtime initialization routines work

---

# ====================================================================================

# **SECTION 7 — LINKER INTEGRATION**

# ====================================================================================

`zc1` must integrate with or produce inputs for:

* [ ] Zeal dev environment linker
* [ ] Symbol resolution
* [ ] Relocation patching
* [ ] Unit loading
* [ ] Vtable fixups
* [ ] Exception frame tables (optional for later)

---

# ====================================================================================

# **SECTION 8 — EXCEPTIONS IMPLEMENTATION**

# ====================================================================================

zc1 must fully implement:

* [ ] try/except
* [ ] try/finally
* [ ] raise
* [ ] unwind logic
* [ ] restoring SP and IX
* [ ] handler dispatch
* [ ] runtime RaiseError mappings

---

# ====================================================================================

# **SECTION 9 — COMPLIANCE SUITE PASS REQUIREMENT**

# ====================================================================================

To qualify as zc1:

* [ ] Pass **all syntax tests**
* [ ] Pass **all semantic tests**
* [ ] Pass **ABI conformance**
* [ ] Pass **stdlib validation**
* [ ] Pass **hardware intrinsic lowering tests**
* [ ] Pass **engine integration tests**
* [ ] Hit basic performance metrics

    * simple ECS loops at expected instruction counts
    * correct tilemap and sprite operations

---

# ====================================================================================

# **SECTION 10 — RELEASE CRITERIA FOR zc1**

# ====================================================================================

`zc1` is considered *complete* when:

* ✔ All Tier 1, Tier 2, Tier 3 features compile
* ✔ Standard library compiles without stubs
* ✔ Game engine compiles and runs in emulator
* ✔ Output is ABI-compatible with runtime
* ✔ Exceptions work
* ✔ Units produce `.ZOU`
* ✔ Compiler is stable in ZealIDE
* ✔ Full compliance suite passes
* ✔ Ready to begin *partial self-hosting* (zc2)

---

# 🎉 **Conclusion: zc1 Is the “Real Compiler”**

`zc1` represents the **first full ZealPascal implementation** capable of building:

* real games
* educational programs
* the full standard library
* the Zeal engine
* and the next generation ZealPascal compiler

zc2 (next step) will begin **self-hosting** inside ZealPascal.

---

If you'd like, I can now generate:

### ✔ zc2 Self-Hosting Compiler Checklist

### ✔ A dependency graph diagram

### ✔ A bootstrap timeline (Gantt-style)

### ✔ A GitHub Projects board with columns & tasks ready to paste into GitHub

Just tell me what you'd like.
