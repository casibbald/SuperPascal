Below is a **precisely scoped, engineering-ready checklist** for building the **minimal bootstrap compiler** (`zc0`) — the very first working ZealPascal compiler used to initiate the full self-hosting chain.

This list is designed so a small team (or one determined engineer) can implement it **without getting lost** in future features.
Nothing here depends on OOP, exceptions, units, ECS, runtime, or Zeal-specific hardware.

This is the “grain of sand” from which the entire language grows.

---

# 📘 **ZealPascal Bootstrap Compiler Checklist (zc0)**

### *The smallest Pascal compiler capable of compiling real programs and bootstrapping future phases.*

---

# ✅ 0. Constraints for the First Compiler

This compiler MUST:

* Run on the **host system** (Linux/macOS/Windows).
* Support only a **small Pascal subset**.
* Emit **Z80 assembly** or **.bin** via assembler.
* Have **no standard library**, only built-in primitives.
* Produce **valid stack-using functions**.
* Not implement classes, records, arrays, units, exceptions, or strings.

zc0 must be tiny, reliable, and testable.

---

# ✅ 1. Bootstrap Language Subset — MUST Implement

### **Types (only):**

* `integer` (16-bit)
* `boolean`
* `byte` (optional)

### **Statements:**

* assignment
* `if … then`
* `if … then … else`
* `while … do`
* `begin … end` blocks
* function/procedure calls

### **Expressions:**

* +, -, *, /, mod
* =, <>, <, >, <=, >=
* AND, OR, NOT

### **Declarations:**

* `var`
* `procedure`
* `function`

### **Program structure:**

```
program X;
var ...
begin
   ...
end.
```

**Everything else is forbidden at this stage.**

---

# ✅ 2. Frontend Checklist

## **Lexing**

* [ ] Identifiers
* [ ] Integers
* [ ] Boolean literals (`true`, `false`)
* [ ] Operators (`+ - * / mod`, `< > <= >= = <>`)
* [ ] Keywords (`program`, `var`, `begin`, `end`, `if`, `else`, `while`, `procedure`, `function`)
* [ ] Comments `{ ... }`
* [ ] Whitespace and newlines
* [ ] Error: unknown token

## **Parser (Recursive Descent)**

* [ ] Parse program header
* [ ] Parse variable declarations
* [ ] Parse procedure/function headers
* [ ] Parse compound statements
* [ ] Parse assignments
* [ ] Parse boolean expressions
* [ ] Parse relational expressions
* [ ] Parse arithmetic expressions
* [ ] Parse function calls (no args OR simple args only)
* [ ] Error recovery for missing semicolons
* [ ] Error messages with line/col

## **AST Nodes** (minimal)

* [ ] Program
* [ ] Block
* [ ] VarDecl
* [ ] ProcDecl / FuncDecl
* [ ] AssignStmt
* [ ] IfStmt
* [ ] WhileStmt
* [ ] BinOp
* [ ] UnaryOp
* [ ] Literal
* [ ] VarRef
* [ ] Call

---

# ✅ 3. Semantic Analyzer Checklist

## **Symbol Table**

* [ ] One global scope
* [ ] One local scope per procedure/function
* [ ] Entries: name → (kind, type, address offset)
* [ ] Detect redefinitions
* [ ] Detect unknown identifiers

## **Type Checking**

* [ ] Integer arithmetic
* [ ] Boolean operators (`and`, `or`, `not`)
* [ ] Comparison operators produce boolean
* [ ] Assignment type compatibility
* [ ] Return type matches function signature
* [ ] Call argument checks (count & type)

## **Constant Folding (optional early win)**

* [ ] Evaluate constant subexpressions at compile-time
* [ ] Replace literal arithmetic
* [ ] Replace boolean operations where possible

---

# ✅ 4. IR (Intermediate Representation) Checklist

Use a **very simple linear IR**:

```
MOV dest, src
ADD dest, src
SUB dest, src
MUL dest, src
DIV dest, src
CMP a, b
JMP label
JZ label
JNZ label
CALL label
RET
STORE var, src
LOAD dest, var
```

Checklist:

* [ ] Generate IR from AST
* [ ] Allocate temporary registers/stack slots
* [ ] Number labels
* [ ] Track variable offsets

---

# ✅ 5. Z80 Backend Checklist

## **ABI Implementation**

zc0 must implement the v0.4 ABI subset:

### **Stack frame**

* [ ] Prologue: push IX; move SP → IX; allocate locals
* [ ] Epilogue: restore SP; pop IX; ret

### **Parameter passing**

* [ ] Push arguments right→left
* [ ] Callee reads parameters at +offset from IX
* [ ] Return value in HL

### **Local variables**

* [ ] Stored at negative offsets from IX

## **Instruction Lowering**

* [ ] MOV → LD
* [ ] ADD → ADD HL, … / ADD A, …
* [ ] SUB → SUB / SBC
* [ ] CMP → CP
* [ ] MUL/DIV → call runtime helpers
* [ ] JZ/JNZ → JP Z / JP NZ

## **Assembly Output**

zc0 should output `.asm` files that assemble cleanly with:

* sjasmplus
  or
* z88dk-z80asm

---

# ✅ 6. Minimal Runtime Checklist (zc0 runtime)

zc0 programs require a **tiny runtime stub**, not the full ZealPascal runtime.

### Required:

* [ ] Startup entry point → run user's main procedure
* [ ] Multiply helper
* [ ] Divide helper
* [ ] Mod helper
* [ ] Halt or return to ZealOS

### Not included yet:

* Strings
* Exceptions
* Heap
* File I/O
* Graphics/audio/input

---

# ✅ 7. Testing Checklist

## **Smoke Tests**

* [ ] `begin end.` compiles
* [ ] Variable assignment test
* [ ] Arithmetic test
* [ ] If/else test
* [ ] While loop test
* [ ] Simple function call
* [ ] Factorial
* [ ] Fibonacci

## **Negative Tests**

* [ ] Using undefined variable
* [ ] Type mismatch assignment
* [ ] Missing semicolon
* [ ] Unterminated block

## **Runtime Tests**

Run on Zeal emulator:

* [ ] Print debug integers
* [ ] Return codes
* [ ] Function chaining

---

# ✅ 8. Acceptance Criteria for zc0

zc0 is READY when:

* ✔ Can compile a multi-function integer program
* ✔ Generates correct stack frames
* ✔ Respects calling convention
* ✔ Produces valid Z80 assembly
* ✔ Output runs correctly on Zeal emulator
* ✔ All smoke tests pass
* ✔ All negative tests emit correct errors

Once zc0 is stable, we proceed to **zc1**, which adds:

* full Tier-1 Pascal
* records
* arrays
* units
* standard library
* classes
* OOP
* exceptions

zc0 is the **hardest leap**.
Once it exists → the rest is incremental.

---

If you'd like, I can now generate:

### ✔ A **checklist for zc1**, the full host compiler

### ✔ A **checklist for making the compiler self-hosting on Zeal**

### ✔ A **Gantt chart timeline** for the entire bootstrap

### ✔ A **GitHub Project board** with tasks and milestones

Just tell me what you want.
