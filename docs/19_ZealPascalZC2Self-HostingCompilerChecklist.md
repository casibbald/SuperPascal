Below is the **full engineering checklist for zc2**, the **first *self-hosting* ZealPascal compiler**.
This compiler runs directly on the **Zeal 8-bit Computer**, compiling real programs in ZealPascal—including large parts of itself.

This is the turning point where ZealPascal becomes a **real language ecosystem**, capable of evolving independently of the host toolchain.

---

# 🧬 **ZealPascal zc2 — Self-Hosting Compiler Checklist**

### *The first Zeal-native compiler, built from zc1, running on real hardware.*

---

# 🎯 **zc2 Requirements**

`zc2` MUST:

* Run natively on the Zeal Computer under ZealOS
* Compile **real ZealPascal programs** (small → medium complexity)
* Be built **from ZealPascal source** compiled by `zc1`
* Emit assembly or `.ZOF` object files
* Use the real ZealPascal runtime and stdlib
* Pass a Zeal-native subset of the compliance suite
* Handle memory constraints (48 KB default program space + paging)

`zc2` does **not** need to compile itself yet.
(The full self-host happens in zc3.)

---

# ================================================================================

# **SECTION 1 — Identify Compiler Subset That Can Run on Zeal**

# ================================================================================

### zc2 cannot include everything from zc1 because of:

* memory constraints
* no virtual memory
* limited stack
* performance limits of Z80

Therefore:

### ✔ Split zc1 into modules:

* [ ] Lexer module
* [ ] Parser module
* [ ] AST node allocator
* [ ] Minimal semantic checker
* [ ] IR1 codegen
* [ ] Z80 backend
* [ ] File writer module
* [ ] Error reporter
* [ ] Driver program interface

### ✔ Classify modules:

* FAST, small → move to Zeal *early*
* LARGE, heavy → stay host-only until zc3

### Required on Zeal now:

* [ ] Lexer
* [ ] Parser
* [ ] Minimal semantic analysis (no OOP, no units initially)
* [ ] IR1 generation
* [ ] Backend for Z80

### Deferred to zc3:

* full OOP semantics
* exceptions
* units and `.ZOU` creation
* optimizations
* hardware intrinsics lowering

---

# ================================================================================

# **SECTION 2 — Implement zc2 Runtime & Memory Model**

# ================================================================================

### zc2 must use the Zeal runtime:

* [ ] Page allocator
* [ ] Basic heap (`GetMem`, `FreeMem`)
* [ ] String runtime (minimal)
* [ ] File I/O (ZealFS)
* [ ] Debug output (`WriteLn`)

### Memory plan:

* [ ] Pages 1–3 = program pages
* [ ] Extra pages allocated for:

    * Lexer tokens
    * AST nodes
    * IR
    * Assembly output

### Memory safety strategies:

* [ ] Arena allocator for AST + tokens
* [ ] Chunked IR storage
* [ ] No recursion in parser (convert to iterative if needed)
* [ ] Minimal symbol table memory footprint

---

# ================================================================================

# **SECTION 3 — Minimal Native Parser for Zeal**

# ================================================================================

zc2 parser must support:

* [ ] Procedure/function declarations
* [ ] `var` declarations
* [ ] Compound statements
* [ ] Assignments
* [ ] If/While/Repeat/For
* [ ] Expressions (full precedence rules)
* [ ] Basic types: integer, boolean, byte

zc2 parser **should NOT support initially**:

* Classes
* Units
* Records
* Sets
* Arrays
* Exceptions
* Tilemap DSL
* Directives

(These return in zc3.)

---

# ================================================================================

# **SECTION 4 — zc2 Semantic Subset**

# ================================================================================

Implement **only essential semantic rules**:

### Required:

* [ ] Type checking for primitives
* [ ] Assignment compatibility
* [ ] Expression typing
* [ ] Procedure/function signature checking
* [ ] Block scoping
* [ ] Return type rules

### Delayed until zc3:

* advanced type inference
* class inheritance checks
* virtual override resolution
* unit import/export semantics
* record layout
* array bounds checking
* set type restrictions

Goal: Keep semantic layer tiny + fast.

---

# ================================================================================

# **SECTION 5 — IR Generation on Zeal**

# ================================================================================

zc2 IR must be **linear IR1 only**.

Required IR instructions:

* MOV
* LOAD
* STORE
* ADD
* SUB
* MUL (call helper)
* DIV
* CMP
* JMP / conditional JMP
* CALL
* RET

Optional:

* logical ops handled inline

IR must be chunked into page-backed arenas.

---

# ================================================================================

# **SECTION 6 — Native Z80 Backend for zc2**

# ================================================================================

### Required:

* [ ] Generate .asm
* [ ] Follow ABI (stack frames, IX, return values)
* [ ] Emit labels, jumps, arithmetic
* [ ] Write `.ASM` to file on ZealFS
* [ ] Shell out to external assembler via ZealOS command

### Optional:

* [ ] Direct binary emission

### Must NOT include yet:

* Vtables
* Class lowering
* Runtime intrinsics
* ECS optimizations

zc2 is intentionally simple.

---

# ================================================================================

# **SECTION 7 — Driver / CLI (on Zeal)**

# ================================================================================

Provide minimal CLI:

```
zc2 input.pas -o output.asm
zc2 input.pas -c (compile only)
zc2 --tokens (debug)
zc2 --ast (debug)
zc2 --ir (debug)
zc2 --help
```

---

# ================================================================================

# **SECTION 8 — Build System Integration**

# ================================================================================

zc2 must integrate with the Zeal dev environment:

* [ ] Compilation → `.ASM`
* [ ] Assembly → `.BIN`
* [ ] Link → runnable Zeal app
* [ ] Write results to Zeal filesystem

Test sequence:

1. Write simple Pascal program
2. Run zc2 on Zeal
3. Assemble using ZealOS tooling
4. Run program
5. Confirm correct behavior

---

# ================================================================================

# **SECTION 9 — Test Suite for zc2**

# ================================================================================

zc2 must pass a **native subset** of v0.6 tests:

### Syntax tests:

* [ ] arithmetic
* [ ] conditionals
* [ ] loops
* [ ] procedure/function definitions
* [ ] nested blocks

### Semantic tests:

* [ ] basic type errors
* [ ] undefined vars
* [ ] incorrect call signatures

### Runtime tests:

* [ ] correct stack behavior
* [ ] correct returns
* [ ] simple recursion
* [ ] integer math
* [ ] while/for loops produce correct output

### Negative tests:

* [ ] missing semicolon
* [ ] mismatched begin/end
* [ ] illegal token

---

# ================================================================================

# **SECTION 10 — Final Acceptance Criteria for zc2**

# ================================================================================

zc2 is considered *complete* when:

* ✔ Runs natively on ZealOS
* ✔ Can compile **at least 20 representative programs**
* ✔ Produces correct `.ASM` and `.BIN` output
* ✔ Passes native compliance subset
* ✔ Stable enough for daily classroom use
* ✔ Produces identical or near-identical output to zc1 for the supported subset
* ✔ Approaches but does not yet achieve full self-hosting

After zc2:

### zc3 = **Full Self-Hosting Compiler**

* Compiles itself on Zeal
* Supports full language (OOP, units, exceptions, arrays, records…)
* Includes optimizations
* Produces `.ZOF`, `.ZOU`, `.ZPK`
* Can build the entire standard library and engine natively

---

# 🎉 Summary

This checklist defines everything required to move from:

🌱 **zc0 (minimal host compiler)**
⬇
🌳 **zc1 (complete host compiler)**
⬇
🚀 **zc2 (first Zeal-native self-hosting seed compiler)**

zc2 is where ZealPascal stops being an idea and becomes a **native language on its own hardware**.

---

If you'd like next steps, I can also generate:

### ✔ **zc3 Full Self-Hosting Checklist**

### ✔ **Dependency graph diagram (zc0 → zc1 → zc2 → zc3)**

### ✔ **Complete Gantt timeline for the entire compiler bootstrap**

### ✔ **Developer Contribution Guide (CONTRIBUTING.md)**

### ✔ **Compiler module API documentation**

Tell me what you want.
