# 📘 ZealPascal Language Specification v0.4 — ABI & Code Generation Rules

## Stack Frames, Calling Conventions, Object Layout, VTables, and Z80-Oriented Codegen

This document defines the **Application Binary Interface (ABI)** and **code generation strategy** for ZealPascal on the Zeal 8-bit Computer.

It is the contract between:

* The **language semantics** (v0.3)
* The **compiler backend**
* The **runtime**
* The **ZealPascal Game Engine**

The ABI ensures that:

* Code compiled today remains compatible with future runtimes
* Units can link together safely
* Classes, records, and intrinsics behave predictably
* Tools (debugger, profiler, engine) can inspect and reason about binaries

---

# 1. Target Architecture

* CPU: Zilog Z80 (10 MHz)
* Endianness: Little-endian
* Address space: 16-bit logical addresses (banked via MMU)
* Calling convention: Pascal-style, callee cleans up stack

---

# 2. Register Usage Convention

General rules:

* `AF` : Condition codes + accumulator (temporary)
* `BC` : Temporary / block operations
* `DE` : Temporary / address math
* `HL` : Primary data pointer / return register
* `IX` : Frame pointer for stack frames
* `IY` : Reserved for runtime (exception frames / TLS)

Scratch registers:

* `AF`, `BC`, `DE`, `HL` may be clobbered by calls unless otherwise documented.

---

# 3. Stack Frame Layout & Calling Convention

ZealPascal uses a **stack-based calling convention** with `IX` as the frame pointer.

## 3.1 Stack Growth

* Stack grows **downwards** (toward smaller addresses).

## 3.2 Function Prologue/Epilogue

Prologue (pseudo-assembly):

```asm
; On entry: return address already pushed by CALL
push ix
ld ix, 0
add ix, sp
; Reserve space for locals
ld hl, -LocalSize
add hl, sp
ld sp, hl
```

Epilogue:

```asm
ld sp, ix
pop ix
ret
```

## 3.3 Frame Layout

At function entry (after prologue):

```
           +--------------------+  high addresses
           | caller locals      |
           +--------------------+
return →   | return address     |
ix ->      +--------------------+
           | old ix             |
           +--------------------+
           | local variables    |
           | ...                |
           +--------------------+
sp ->      | (top of local area)|  low addresses
```

Parameters are located **above** the return address (positive offsets from `ix`).

---

# 4. Parameter Passing

## 4.1 Value Parameters

* Passed on stack from **right to left**.
* Scalars (integer, byte, word, boolean, char) use 2 bytes (promoted to word) for alignment simplicity.

Example:

```pascal
procedure Foo(a, b: integer; c: byte);
```

Stack (top to bottom):

```
... caller locals
c (word)
b (word)
a (word)
return address
old ix
locals...
```

## 4.2 `var` Parameters

* Passed as **addresses** (2-byte pointers) on the stack.
* Callee loads address and dereferences.

## 4.3 Hidden Parameters

Hidden parameters are inserted **before user parameters** (closest to return address):

* Return buffer pointer (for large return types: records, arrays, sets)
* `Self` pointer (for class/record methods)

Stack order (top to bottom):

```
... caller locals
user param N
...
user param 1
Self (if any)
Return buffer ptr (if any)
return address
old ix
locals...
```

---

# 5. Return Values

## 5.1 Scalars

* `integer`, `word`, `byte`, `boolean`, `char`, pointers, and class references are returned in `HL`.

## 5.2 Large Types

* Records, arrays, and sets are returned via a **caller-allocated buffer**.
* Caller passes hidden first parameter: `ReturnPtr: ^T`.
* Callee writes result into `ReturnPtr^` and returns.

---

# 6. Record Layout & Codegen

## 6.1 Memory Layout

Records are laid out as:

* Fields in declared order
* No hidden paddings (except where needed for alignment simplicity)

Example:

```pascal
type
  TVec2 = record
    X, Y: integer;
  end;
```

Layout:

```
Offset 0: X (2 bytes)
Offset 2: Y (2 bytes)
Size:    4 bytes
```

## 6.2 Record Methods

Methods on records compile to free functions:

```pascal
procedure TVec2.Add(const B: TVec2);
```

becomes internally:

```pascal
procedure TVec2_Add(var self: TVec2; const B: TVec2);
```

Codegen treats `self` as a `var` parameter located on the stack.

---

# 7. Class Layout & VTables

## 7.1 Instance Layout

Each class instance in memory:

```
Offset 0: VTablePtr (2 bytes)
Offset 2: Field1
Offset n: FieldN
```

## 7.2 VTable Layout

VTables are static tables in read-only memory:

```
Offset 0: ParentVTablePtr (2 bytes) or 0
Offset 2: RTTIPtr (2 bytes) or 0
Offset 4: MethodSlot0 (2 bytes)
Offset 6: MethodSlot1 (2 bytes)
...
```

## 7.3 Virtual Method Dispatch

Given:

```pascal
O: TObject;
O.Foo;
```

Generated Z80 sequence (conceptual):

```asm
ld hl, (O)         ; HL = pointer to object
ld de, (hl)        ; DE = vtable
ld hl, (de+Slot)   ; HL = method address
jp (hl)            ; jump to method
```

The compiler maps each virtual method to a **fixed slot index** during compilation.

---

# 8. Name Mangling

## 8.1 Global Symbols

To avoid collisions, ZealPascal applies a simple mangling scheme:

* `UnitName_SymbolName`

Examples:

* `Math_Sin`
* `Player_Init`

## 8.2 Methods & Records

* Record methods: `Type_Method`
* Class methods: `Class_Method`
* Overloaded/virtual methods may receive suffixes (e.g., `_I`, `_B` for integer, byte, etc.)

Mangled names are visible in object files and used by the linker.

---

# 9. Codegen Strategy (Z80-Oriented)

## 9.1 General Principles

* Prefer short instruction sequences
* Avoid unnecessary stack operations
* Hoist constant computations
* Use `HL` as primary data pointer/accumulator
* Use `IX` for frame pointer only (no arithmetic on IX)

## 9.2 Arithmetic

* 16-bit arithmetic via standard Z80 patterns (`add hl,`, `adc hl,`)
* Multiplication/division via library routines (unless small constants)

## 9.3 Branching

* Use conditional jumps with minimal flag recalculation
* Short-circuit boolean expressions (standard Pascal and/or rules)

---

# 10. ECS & Loop Optimizations

ZealPascal compiler recognizes ECS-style patterns:

```pascal
for i := 0 to Count-1 do
  PositionX[i] := PositionX[i] + VelocityX[i];
```

Optimizations:

* Unroll loops with small constant bounds
* Use indexed addressing (`(ix+offset)` or `(hl+index)`) where possible
* Group operations touching the same arrays

---

# 11. Intrinsic Codegen

Each intrinsic maps to a fixed code pattern.

### Example: `WaitVBlank`

* Emits a call to a runtime routine that blocks until VBlank.

### Example: `MapPage(slot, page)`

* Loads slot/page into specific registers
* Uses `OUT` or memory-mapped writes to MMU registers

### Example: `DMA_Copy(src, dest, size)`

* Fills a command block in memory
* Triggers ZVB DMA engine

The compiler forbids redefinition of intrinsics and checks argument types statically.

---

# 12. Exception & Error ABI (Future Tier)

For try/except/finally (when implemented), the ABI will:

* Reserve a small area referenced by `IY` for the current exception frame
* Use setjmp/longjmp-like semantics
* Maintain a chain of exception frames for nested try blocks

Details will be finalized in v0.7 Exception Semantics.

---

# 13. Binary File Formats (Overview)

This ABI underpins the following formats (detailed elsewhere):

* `.ZOF` — Zeal Object File (relocatable)
* `.ZOU` — Zeal Unit File (linked interface)
* `.ZPK` — Zeal Package (asset bundle)
* `.ZDBG` — Debug info

Each relies on:

* Stable symbol names
* Stable vtable layout
* Stable calling convention

---

# ✔️ ZealPascal ABI & Codegen Specification v0.4 Complete

This document now defines:

* Register usage
* Stack frame structure
* Calling convention and parameter passing
* Scalar & aggregate return strategies
* Record and class layout
* Vtable structure and dispatch
* Name mangling
* Z80-oriented code generation patterns
* ECS loop optimizations
* Intrinsic lowering

With v0.4 complete, the backend has a stable contract to generate correct and efficient machine code for ZealPascal.

Next steps:

* v0.5 Standard Library Specification
* v0.6 Compliance & Test Suite
* v0.7 Exceptions & Advanced Features
