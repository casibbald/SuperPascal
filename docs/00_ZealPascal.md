# 📘 Product Requirements Document (PRD)

## Self-Hosted Pascal Compiler for Zeal 8-bit OS

### Version 2.0 — Extended Complete Pascal Vision

---

# 1. Product Summary

The Zeal 8-bit Computer (Z80 @ 10 MHz, 512 KB SRAM, 16 KB paging system) is uniquely powerful among 8-bit architectures. Its most important architectural advantage is its **FPGA-based video subsystem**, which completely offloads all rendering, sprite handling, tilemaps, timing, raster effects, and screen-refresh logic away from the Z80 CPU.

This makes Zeal fundamentally different from classic Z80 machines (ZX Spectrum, Amstrad CPC, MSX, TRS‑80, CP/M systems). On those systems, the CPU is heavily constrained by video‑related memory contention, cycle stealing, and VRAM timing penalties. On Zeal, the Z80 becomes a **pure computation engine**, with its full 10 MHz available for program logic, compiler workloads, or advanced Pascal features.

This architectural separation enables a complete Pascal ecosystem far beyond historical 8‑bit limitations, including:

* Standard Pascal (ISO 7185)
* Turbo Pascal units
* A full Object Pascal class model
* Dynamic linking and loadable units
* Floating point support
* Multi-pass optimizing compiler
* Modern IDE and debugging environment
* Optional exception handling and RTTI
* Potential cooperative multitasking

This PRD defines all phases necessary for a full Pascal development environment, growing from a strict Pascal core into a complete Object Pascal workstation uniquely enabled by Zeal’s architecture.

---

# 2. Goals & Scope Goals & Scope

## 2.1 Primary Goal

Deliver a **complete Pascal compiler**, self-hosted on Zeal, supporting Object Pascal, modular compilation, dynamic linking, and optimizations.

## 2.2 Secondary Goal

Provide a Turbo-Pascal-style **Integrated Development Environment (IDE)** running natively on Zeal.

---

# 3. Language Features (Mandatory Across Phases)

## PHASE 1 — MVP Compiler

* Integer, Boolean, Char
* Arrays, Records
* Procedures & Functions
* Control structures: If/While/For/Repeat/Case
* File I/O
* Basic runtime library
* Z80 code generation
* Single-file programs

## PHASE 2 — Full Standard Pascal (ISO 7185)

* Sets, Subranges, Packed Records
* Nested procedures
* Full type checking
* Constant folding
* Optional AST

## PHASE 3 — Turbo Pascal–style Units

* Interface & Implementation sections
* Initialization/finalization
* Binary interface files
* Symbol export/import
* Incremental recompilation

## PHASE 4 — Object Pascal

* Classes and inheritance
* Virtual methods & vtables
* Constructors / destructors
* Properties
* Object reference model
* Optional RTTI

## PHASE 5 — Dynamic Linking / Overlays

* Shared runtime libraries
* Dynamically loaded units
* Overlay manager
* Relocatable code
* Symbol resolution at load time

## PHASE 6 — Floating Point

* Software IEEE-754 32-bit
* Float operators and temporaries
* FP runtime library

## PHASE 7 — Optimizing Backend

* Constant propagation
* Dead code elimination
* Strength reduction
* Loop invariant motion (if feasible)
* Z80 peephole optimizations

## PHASE 8 — Integrated Development Environment (IDE)

* Syntax highlighting
* F9 Compile
* Error navigation
* Project explorer

## PHASE 9 — Debugger

* Breakpoints
* Step Into / Step Over
* Watch variables
* Stack frame viewer
* Symbol-aware disassembler

---

# 4. High-Level Roadmap

1. **Phase 1** → MVP Standard Pascal
2. **Phase 2** → Full ISO Pascal
3. **Phase 3** → Units
4. **Phase 4** → Object Pascal
5. **Phase 5** → Dynamic Linking
6. **Phase 6** → Floating Point
7. **Phase 7** → Optimizing Backend
8. **Phase 8** → IDE
9. **Phase 9** → Debugger

---

# 5. System Architecture

## 5.1 Compiler Modules

* Lexer (paged buffers)
* Parser (LL(1) or recursive descent)
* Symbol Table (paged)
* Code Generator (Z80)
* Output Builder
* Runtime Library
* Page Manager
* Syscall Layer

## 5.2 Memory Model

### Default Program Pages

| Page | Range     | Purpose        |
| ---- | --------- | -------------- |
| 0    | 0000–3FFF | Kernel         |
| 1    | 4000–7FFF | Compiler main  |
| 2    | 8000–BFFF | Parser/codegen |
| 3    | C000–FFFF | Stack          |

### Allocated Pages

* Lexer: 4–8 pages
* Symbols: 4–12 pages
* AST nodes: 4–20 pages
* Codegen: 4–8 pages
* Output: 2 pages

---

# 6. Syscall ABI Integration

All syscalls use `RST 08h` with registers for parameters.

### Required Syscalls:

* File: OPEN, READ, WRITE, CLOSE, CREATE, DELETE
* Memory: ALLOC_PAGE, MAP_PAGE, FREE_PAGE
* Console: WRITE_CHAR, WRITE_BUFFER, CLEAR_SCREEN
* Control: EXIT

---

# 7. Bootstrapping Strategy

### Phase A — PC Bootstrap

* Minimal compiler cross-compiled on PC
* Output used to produce first native Zeal compiler

### Phase B — Native Build

* Compiler compiles itself on Zeal
* Validates full self-hosting

### Phase C — Expansion

* Units, OOP, linker, dynamic loader
* IDE + Debugger

---

# 8. Development Workflow

1. Open file in editor (Zeal IDE / Zepto-based environment).
2. Press F9 (or configured key) to compile.
3. Compiler loads and parses source.
4. Symbol resolution and code generation.
5. Binary emitted as `.bin` (and optionally `.map`, `.lst`).
6. Run via `run program.bin`.
7. Errors show line/column and integrate with the editor.

## 8.1 IDE Integration (Zepto)

The existing Zeal IDE (Zepto) will be used as the foundation for the Pascal development experience:

* Fork Zepto and add a **Pascal toolchain profile** (compiler, assembler/linker if used, post-processing tools).
* Add **build configurations** per project: single-file, multi-unit, library, test harness.
* Integrate **F9 / Build** and **Run** commands that invoke the Zeal Pascal compiler with appropriate flags.
* Implement **compiler output parsing** so Zepto can:

    * Highlight errors and warnings in the editor.
    * Jump to line/column on error selection.
    * Show a build output pane.
* Add **project templates** for:

    * "Console Pascal Program"
    * "Multi-unit Pascal Project"
    * "Object Pascal Application"
* Extend Zepto’s configuration UI to manage:

    * Include paths for units.
    * Library search paths.
    * Optimization levels and debug symbols.

Zepto becomes the primary UI surface for all Pascal development on Zeal, providing a familiar IDE workflow while the compiler, linker, and runtime evolve underneath.

---

# 9. Future Extensions

* RTTI
* Exception handling
* Generic types (investigate feasibility)
* Cross-compilation backends (optional)

---

# 10. Summary

This PRD outlines a multi-phase plan to deliver a **fully self-hosted, Object-Pascal-capable development environment** for the Zeal 8-bit Computer. Leveraging 512 KB of paged SRAM and an FPGA-offloaded video subsystem, this system surpasses historical Z80-language implementations and becomes a modern, powerful 8-bit development workstation.

---

# 11. Language Specification (Zeal Pascal)

This section defines the core language specification for Zeal Pascal. It is structured in tiers:

* Tier 1: ISO Standard Pascal (core semantics)
* Tier 2: Turbo Pascal extensions (units, strings, sets, packed records)
* Tier 3: Object Pascal subset (classes, inheritance, virtual methods)

## 11.1 Lexical Structure

* **Character set**: ASCII subset, with extended characters reserved for future use.
* **Identifiers**: start with a letter, followed by letters, digits, or `_`. Case-insensitive.
* **Keywords** (non-exhaustive): `program`, `unit`, `interface`, `implementation`, `uses`, `begin`, `end`, `var`, `const`, `type`, `record`, `array`, `set`, `of`, `procedure`, `function`, `if`, `then`, `else`, `while`, `do`, `for`, `to`, `downto`, `repeat`, `until`, `case`, `class`, `constructor`, `destructor`, `virtual`, `override`, `try`, `except`, `finally`, `raise`.
* **Literals**:

    * Integer literals: decimal; optional hexadecimal with `$` prefix.
    * Char literals: `'c'` or `#nn`.
    * String literals: `'...'`, with `''` representing a single quote inside.
* **Comments**: `{ ... }`, `(* ... *)`. Nested comments are not required in Phase 1, but may be supported later.
* **Operators**: `+ - * / div mod and or not shl shr = <> < <= > >= := @ ^`.

## 11.2 Grammar (EBNF Overview)

High-level EBNF (abbreviated, can be expanded later into a full grammar):

```ebnf
program      = "program" ident [ "(" ident { "," ident } ")" ] ";" block "." ;
unit         = "unit" ident ";" interface-part implementation-part [ init-part ] "." ;

block        = label-part const-part type-part var-part proc-func-part statement-part ;

label-part   = [ "label" label { "," label } ";" ] ;
const-part   = [ "const" const-def { const-def } ] ;
const-def    = ident "=" const-expr ";" ;

type-part    = [ "type" type-def { type-def } ] ;
type-def     = ident "=" type-spec ";" ;

var-part     = [ "var" var-def { var-def } ] ;
var-def      = ident { "," ident } ":" type-spec ";" ;

proc-func-part = { proc-def | func-def | class-def } ;

proc-def     = "procedure" ident [ formal-params ] ";" block ";" ;
func-def     = "function" ident [ formal-params ] ":" type-spec ";" block ";" ;

formal-params = "(" param-group { ";" param-group } ")" ;
param-group  = [ "var" ] ident { "," ident } ":" type-spec ;

statement-part = "begin" statement { ";" statement } "end" ;

statement    = simple-stmt | structured-stmt | empty-stmt ;

simple-stmt  = assign-stmt | call-stmt | goto-stmt | io-stmt ;
structured-stmt = if-stmt | while-stmt | for-stmt | repeat-stmt | case-stmt | try-stmt ;

expr         = simple-expr [ rel-op simple-expr ] ;
simple-expr  = [ "+" | "-" ] term { add-op term } ;
term         = factor { mul-op factor } ;

factor       = ident | literal | "(" expr ")" | "not" factor | set-ctor ;
```

Object Pascal constructs extend this grammar with `class` types and method declarations.

## 11.3 Type System Rules

### 11.3.1 Base Types

* `integer`: signed 16-bit, range typically -32768..32767.
* `boolean`: `false` or `true`; codegen as 0 or 1 in a byte.
* `char`: 8-bit character.
* `string`: implemented as `shortstring` by default, i.e. a length-prefixed sequence of up to 255 chars stored as `array [0..255] of char`, where index 0 stores the length.

### 11.3.2 Composite Types

* **Subranges**: `type small = 1..10;` constrained integer subsets.
* **Enumerations**: `type color = (red, green, blue);` mapped to small integers.
* **Arrays**: `array[lo..hi] of T`; bounds known at compile time.
* **Records**: product types, with named fields, optionally `packed`.
* **Sets**: sets of small ordinal types; internally bitsets.
* **Pointers**: `^T` referencing a type `T` allocated in the heap.
* **Classes**: reference types with associated vtables (see Object Pascal subset).

### 11.3.3 Type Equivalence

* **Named types**: type-identity based; two types are equivalent only if they share the same name (aliasing allowed via `type alias = existing;`).
* **Array and record types**: structural equivalence is not assumed; name equivalence is used to avoid ambiguity.
* **Subranges**: compatible with their base type where ranges fit.
* **Enumerations**: each enumeration is a distinct type; ordinal compatibility only where explicitly allowed.

### 11.3.4 Assignment and Compatibility

* Assignment requires type compatibility:

    * Same base type, or type alias.
    * Subranges must fit within destination range.
    * Enumerations must match exactly.
    * Pointers assignable if types are identical or via explicit casts.
    * `nil` is assignable to any pointer or class reference.
* Implicit numeric promotions are not performed (no automatic integer-to-float in Phase 1–2).

## 11.4 Function and Procedure Calling Conventions

### 11.4.1 Z80 Stack and Register Model

* Arguments are passed on the stack, **right-to-left**, Pascal-style.
* Return address is pushed by the CALL instruction.
* A frame pointer register (typically `IX`) is used to access parameters and locals.
* Callee is responsible for restoring `IX` and local stack frame.
* Callee cleans up the stack (Pascal convention), unless otherwise specified for special runtimes.

### 11.4.2 Value and Var Parameters

* By default, parameters are passed **by value** (copied on stack).
* `var` parameters are passed **by reference** (as addresses on stack) and must be l-values at call sites.

### 11.4.3 Return Values

* `integer`: returned in `HL`.
* `boolean` and `char`: returned in `L`; `H` undefined.
* `pointer` and `class` references: returned in `HL`.
* Larger structures: returned via a hidden first parameter (pointer to caller-allocated space).

### 11.4.4 Methods

* Instance methods receive a hidden `Self` parameter (class reference) as the **first** argument on stack.
* Virtual methods look up the appropriate method entry in the vtable using the `Self` reference.

## 11.5 Object Pascal Subset Definition

The supported Object Pascal subset includes:

* `class` types with single inheritance:

    * `type TChild = class(TParent) ... end;`
* Fields (data members) inside classes.
* Methods: regular, virtual, and overridden:

    * `procedure Foo; virtual;`
    * `procedure Foo; override;`
* Constructors and destructors:

    * `constructor Create(...);`
    * `destructor Destroy; override;`
* Visibility sections: `public`, `private`; `protected` optional later.
* No interfaces, generics, or advanced Delphi-only features in early phases.

### 11.5.1 Class Layout

* Each instance contains:

    * Pointer to vtable at offset 0.
    * Data fields laid out sequentially.
* Vtable contains:

    * Pointer to RTTI or class metadata (optional, later phase).
    * Pointers to virtual methods in declaration order.

## 11.6 Exception Model

Exceptions are a **later phase** feature but must be designed into the language:

* Syntax: `try ... except ... end;`, `try ... finally ... end;`, `raise;`.
* Implementation model: **setjmp/longjmp-style stack unwinding**:

    * Each `try` block pushes an exception frame descriptor onto an internal stack.
    * `raise` walks the frames, unwinding stack and transferring control.
* Zero-cost exceptions (like modern C++/LLVM models) are **not required** and are likely impractical on Z80.
* The compiler maintains metadata for exception frames in paged memory.

Phase ordering:

* Phases 1–3: no exceptions.
* Phase 4+: syntax parsed and compiled, but only simple runtime behavior.
* Later phase: full unwinding, nested try/except, and try/finally semantics.

---

# 12. Implementation Roadmap

This roadmap refines the earlier phase list into concrete implementation steps, acceptance criteria, and risk analysis.

## 12.1 Phase Overview

1. **Phase 1 – MVP Compiler**

    * Single-file programs.
    * Integer/boolean/char types, arrays, records.
    * Basic runtime and syscalls.
    * Simple code generation, minimal optimization.

2. **Phase 2 – Full Standard Pascal**

    * Subranges, sets, enumerations, packed records.
    * More robust type checking and constant folding.

3. **Phase 3 – Units and Separate Compilation**

    * Turbo Pascal-style units with interface/implementation.
    * Symbol export/import and unit binaries.

4. **Phase 4 – Object Pascal Subset**

    * Classes, inheritance, virtual methods, constructors/destructors.

5. **Phase 5 – Dynamic Linking and Overlays**

    * Loadable libraries, relocatable unit files.

6. **Phase 6 – Floating Point Support**

    * Software-based IEEE-754 32-bit reals and math library.

7. **Phase 7 – Optimizing Backend**

    * Multi-pass optimizer and improved codegen quality.

8. **Phase 8 – IDE Integration and Tooling**

    * Zepto-based Pascal IDE with project system.

9. **Phase 9 – Debugger and Advanced Tooling**

    * Breakpoints, stepping, watches.

## 12.2 Per-Phase Acceptance Criteria (Examples)

### Phase 1

* Compiler runs on Zeal and compiles itself from source (or from a stable subset).
* Can compile and run a non-trivial single-file Pascal program (~500 lines).
* Emits valid binaries that use Zeal syscalls.

### Phase 2

* Full ISO-style Pascal constructs supported in at least 10 representative test programs.
* Type errors reported with line/column accuracy.
* Constant folding works for arithmetic and simple boolean expressions.

### Phase 3

* Units compile to separate binary/interface artifacts.
* Programs can `uses` multiple units and successfully link.
* Incremental rebuild works: changing one unit does not require recompiling all units.

### Phase 4

* Class inheritance and virtual methods work with vtable dispatch.
* At least one sample program demonstrates polymorphism (e.g. `TShape` with `Draw` methods).
* Constructors/destructors correctly allocate and free objects from the heap.

### Phase 5

* Dynamic loading of a unit at runtime.
* Relocation and symbol resolution verified for at least one shared library.

### Phase 6

* Floating point arithmetic passes a suite of deterministic tests.
* Basic math routines (sin, cos, sqrt, etc.) implemented.

### Phase 7

* Benchmark programs show measurable speedups compared to unoptimized builds.

### Phase 8

* Zepto integration: F9 compile, F10 run (example bindings).
* Errors navigate directly to source locations.

### Phase 9

* Debugger can set breakpoints, step, and inspect local variables.

## 12.3 Risk Analysis & Feasibility

### Technical Risks

* **Memory Pressure**: Compiler data structures may exceed comfortable limits. Mitigation: use paged arenas and compact representations.
* **Complexity of Object Pascal**: Class system and vtables may increase code size. Mitigation: staged roll-out and strict subset.
* **Dynamic Linking**: Relocation and symbol resolution on Z80 is non-trivial. Mitigation: keep formats simple, design with Zeal paging in mind.
* **Performance**: Multi-pass optimization on Z80 could be slow. Mitigation: focus on critical-path optimizations and consider optional optimization levels.

### Project Risks

* **Scope Creep**: Temptation to add every Delphi feature. Mitigation: enforce the defined subset.
* **Tooling Maintenance**: Keeping Zepto integration and compiler in sync. Mitigation: shared configuration and versioning.

## 12.4 Zeal OS Syscall Interaction (Conceptual Diagram)

Compiler and generated programs interact with Zeal OS using a syscall layer:

* **Compiler runtime**:

    * Opens source files via file syscalls.
    * Allocates pages for symbol tables, AST, and code buffers.
    * Writes output binaries and auxiliary files.

* **Generated programs**:

    * Use the Pascal runtime library, which wraps syscalls for I/O, memory, and process control.

Conceptual call sequence when compiling:

1. Zepto invokes compiler binary with arguments.
2. Compiler initializes and allocates base pages.
3. Compiler uses file syscalls to read source.
4. During compilation, compiler requests additional pages for internal structures.
5. On successful compilation, compiler writes `.bin` and metadata.
6. Zepto reads compiler status and updates the UI.

## 12.5 Memory Model Expansions (Paged Heaps and Arenas)

### Compiler Memory Model

* **Arenas per subsystem**:

    * Lexer buffers
    * Symbol tables
    * AST nodes (optional)
    * Intermediate representation for optimizer
* Each arena manages a list of 16 KB pages.
* Allocations are bump-pointer based within each page.
* No fine-grained free in early phases; full arena reset per compilation.

### Runtime Memory Model

* Heap implemented as one or more page ranges dedicated to dynamic allocation.
* Allocation strategies:

    * Simple first-fit or segregated free lists.
    * Optional separate pools for small objects and large blocks.
* Class instances and dynamic data structures (e.g. strings, dynamic arrays) allocated from the heap.

This paged model exploits Zeal’s 512 KB SRAM to provide generous memory for both compiler and user programs while retaining simple allocation patterns that are feasible on Z80.

---

# 13. Future Extensions

* RTTI for classes and type introspection.
* Exception handling with richer metadata and stack traces.
* Generic types, if feasible within memory and performance constraints.
* Cross-compilation backends (e.g., generating code for other 8-bit targets).
* Background or cooperative multiprogramming support inside the Pascal runtime.

---

# 14. Object and Unit File Formats (.ZOF, .ZOU)

To support separate compilation, object-oriented features, and dynamic linking, Zeal Pascal introduces two binary formats:

## 14.1 .ZOF — Zeal Object File (Unlinked)

A `.ZOF` file is produced by compiling a single Pascal source file (program, unit, or class definition). It contains:

### 14.1.1 Header

* Magic bytes: `ZOF`
* Format version
* Target architecture (Z80)
* Flags: unit/object/program, OOP features used
* Section counts (code, data, bss, relocation entries, symbol tables)

### 14.1.2 Sections

* **CODE** — machine code, relocatable
* **DATA** — initialized data
* **BSS** — zero-initialized data definitions
* **RTTI** (optional, depending on OOP phase)
* **VTable Definitions** (for classes)

### 14.1.3 Symbol Table

Symbol entries include:

* Name
* Kind: function, procedure, variable, type, constant, method, field
* Section + offset
* Flags: exported, imported, local, virtual
* Size (for arrays/records)
* Scope (unit-level, global, class-level)

### 14.1.4 Relocation Table

Each entry defines:

* Location to patch
* Type: absolute, relative, vtable reference, unit reference
* Target symbol

### 14.1.5 Dependency Table

List of units referenced by this compilation unit.
Allows incremental build & dynamic loading.

---

## 14.2 .ZOU — Zeal Unit File (Linked Interface)

A `.ZOU` file is generated after compiling a unit.
It represents a **stable binary interface**, similar to Turbo Pascal `.TPU` files.

Contents:

* Exported symbol table
* Compressed metadata
* Layout of types and classes
* Vtable entries for classes
* Public constants
* CRC/hash to detect mismatches between unit versions
* Minimal relocation info (usually resolved at unit creation)

Units can be **statically linked** or **dynamically loaded** during execution.

---

# 15. VTable Layout Specification (Binary Level)

Object Pascal support requires a stable vtable structure.

## 15.1 Class Instance Memory Layout

```
Offset  Size  Description
0       2     Pointer to vtable
2       n     Instance fields in declared order
```

## 15.2 VTable Memory Layout

Each class has its own vtable allocated in a read-only or shared page.

```
Offset  Size  Description
0       2     Pointer to parent vtable (nil if no parent)
2       2     Pointer to class RTTI block (optional phase)
4       2     Method pointer #0
6       2     Method pointer #1
...     ...   One entry per virtual method
```

### Vtable Requirements:

* Methods appear in declaration order.
* Overridden methods replace inherited slots.
* Non-virtual methods are not included in vtable.
* Constructors/destructors are normal methods unless marked virtual.

---

# 16. Minimal Standard Library Specification

The Zeal Pascal RTL must remain compact, with modular growth by phase.

## 16.1 SYSTEM Unit (Always Available)

Provides:

* Basic types and casts
* Memory management (`New`, `Dispose`, `GetMem`, `FreeMem`)
* Runtime helpers (bounds checking, nil checking, arithmetic helpers)
* Page allocation wrapper for Zeal OS syscalls
* Exit/halt logic
* Fundamental constants (`MaxInt`, `MinInt`)

## 16.2 SYSUTILS-Lite Unit (Phase 3+)

Includes:

* Simple exception base types
* String manipulation helpers
* Conversion routines (`IntToStr`, `StrToInt`)
* Basic file utilities
* Optional high-level error reporting

## 16.3 MATH Unit (Phase 6)

Provides:

* Add, subtract, multiply, divide for floating-point
* `Sqrt`, `Sin`, `Cos`, `Ln`, `Exp` (software-emulated)
* Fixed-point helpers for performance-sensitive code

## 16.4 IO Unit

Thin wrappers around Zeal OS syscalls:

* `ReadLn`, `WriteLn`
* `Assign`, `Reset`, `Rewrite`
* `Read`, `Write`
* Buffered file I/O for large datasets

## 16.5 STRINGS Unit

* `Concat`, `Copy`, `Delete`, `Insert`
* Lexicographic comparison
* Length-prefixed string operations

---

# 17. Debug Information Format

To support debugging, breakpoints, stepping, and IDE integration, Zeal Pascal introduces a compact debug-info format.

## 17.1 .ZDBG Debug File (Optional Sidecar)

Generated alongside `.bin` for programs compiled with debug flags.

### 17.1.1 Header

* Magic: `ZDBG`
* Version
* CRC of program binary

### 17.1.2 Symbol Map

For each symbol:

* Name
* Address (absolute or relocatable)
* Type: function, variable, parameter, field
* Scope and lifetime

### 17.1.3 Line Mapping Table

Maps instruction addresses to source lines:

```
AddrLow AddrHigh → LineNumber
```

Enables IDE breakpoints and single-stepping.

### 17.1.4 Stack Frame Metadata

For each function:

* Local variable offsets
* Parameter offsets
* Frame size
* Whether function uses `IX` or frame pointer

### 17.1.5 Class & RTTI Debug Info (Phase 7+)

Includes:

* Class name
* Field layout
* Method indices
* Vtable mapping

---

# 18. Compiler Architecture Specification

This section defines the full internal architecture of the Zeal Pascal compiler. It is designed for multi-phase expansion, supporting both a fast single-pass mode for MVP and a full multi-pass pipeline for optimization and Object Pascal features.

The compiler is divided into **seven major subsystems**:

1. **Frontend (Lexer + Parser)**
2. **Semantic Analyzer (Type System + Symbol Resolver)**
3. **Intermediate Representation Layer (IR)**
4. **Code Generation Backend (Z80 Assembly/Direct Machine Code)**
5. **Optimizer Pipeline (Peephole + Optional Multi-Pass)**
6. **Linker / Unit Merger**
7. **Runtime Integration (Syscalls, Vtables, Memory)**

Each subsystem is isolated via clean interfaces and makes use of Zeal’s paged memory model to allow arbitrarily large programs.

---

## 18.1 High-Level Dataflow Diagram

```
      +-----------+       +------------+        +--------------+
Source|   Lexer   |       |   Parser   |        |   Semantic   |
 File →| Tokenizer |  →   |  AST/IR1   |  →     |   Analyzer   |
      +-----------+       +------------+        +--------------+
                                                 | Type Check |
                                                 | Symbol Res |
                                                 +------------+
                                                         ↓
                                             +----------------------+
                                             |     IR Builder       |
                                             |  (Linear IR/Quad IR) |
                                             +----------------------+
                                                         ↓
                                             +----------------------+
                                             |   Code Generator     |
                                             | (Z80, vtables, ABI)  |
                                             +----------------------+
                                                         ↓
                                             +----------------------+
                                             |     Optimizer        |
                                             |  Peephole / IR Pass  |
                                             +----------------------+
                                                         ↓
                                             +----------------------+
                                             | Linker / Unit Loader |
                                             +----------------------+
                                                         ↓
                                             Final Binary (.bin/.zou)
```

---

# 18.2 Lexer Architecture

The lexer is a **deterministic finite automaton (DFA)** producing tokens:

* Identifiers
* Keywords
* Literals (char, string, integer)
* Operators and delimiters

### Implementation details

* Works on **paged text buffers** allocated via Zeal OS.
* Uses **rope-like structures** to handle large source files spanning multiple pages.
* Tracks:

    * Line and column for error reporting
    * File offsets for debug mapping

### Token emission

Tokens are compact (<=8 bytes) and stored in token pages with bump-pointer allocation.

---

# 18.3 Parser Architecture

The parser is a **recursive descent LL(1)** parser supporting:

* ISO Pascal grammar
* Turbo Pascal structural extensions
* Object Pascal class grammar

### Modes

* **Single-pass mode**: For Phase 1–2, directly emits IR during parse.
* **AST mode**: For OOP and optimization phases, builds structured AST nodes stored across multiple pages.

### Error Recovery

The parser supports minimal panic-mode recovery:

* Skip to sync tokens (`;`, `end`, `until`, `.`)
* Emit diagnostic entries to assist IDE

---

# 18.4 Semantic Analyzer

The semantic analyzer runs after or during parsing.
Responsibilities:

* Type checking (with strong ISO Pascal rules)
* Constant evaluation
* Operator overload resolution (integer only in early phases)
* Symbol table management
* Visibility and scope resolution (units, classes, interfaces)
* Layout computation (arrays, records, class fields)
* Method resolution and vtable slot assignment

### Symbol Table

Implemented as a **paged hash table**, storing:

* Types
* Variables
* Constants
* Procedures/functions
* Methods and fields
* Unit exports

Scope stack is kept in a compact frame structure.

---

# 18.5 Intermediate Representation (IR)

IR is required for optimization and OOP.
Two layers exist:

### IR1 — Parse-Time Linear IR

* Produced directly by parser
* Contains pseudo-instructions (assign, call, jump, load, store)
* Suitable for simple peephole optimization

### IR2 — Quadruple or Three-Address Form

* Needed for object model
* Needed for more advanced optimizations (constant propagation, DCE)
* Can spill across many pages

Examples of IR2:

```
t1 := a + b
if t1 > 10 goto L1
call WriteLn(t1)
```

Both IRs map cleanly to Z80 codegen.

---

# 18.6 Code Generator (Z80 Backend)

The backend translates IR into Z80 machine code.
It handles:

* Register allocation (simple or linear scan)
* Stack frame management (`IX`-based)
* Parameter passing (Pascal calling convention)
* Class layout & vtable entries
* Constructor/destructor prologues
* Virtual dispatch code
* Heap operations for object creation
* Syscall wrappers

### Vtable dispatch example

IR:

```
callvirt Self.Draw()
```

Z80 sequence:

```
ld hl,(ix+SelfOffset)   ; load instance
ld de,(hl)              ; vtable pointer
ld hl,(de+MethodIndex)  ; method pointer
push args...
jp (hl)                 ; tail call
```

---

# 18.7 Optimizer Pipeline

Optimizer is modular and supports:

### Peephole Optimizations (Phase 1+)

* Remove redundant loads/stores
* Replace unnecessary pushes/pops
* Strength reduction
* Folding obvious patterns (`inc/dec`)

### Multi-Pass IR Optimizations (Phase 6–7)

* Constant propagation
* Constant folding
* Dead code elimination
* Common subexpression elimination (optional)
* Algebraic simplifications

Optimization passes run in paged memory safely.

---

# 18.8 Linker / Unit Merger

The linker combines `.ZOF` and `.ZOU` files into a final `.bin` executable.

Responsibilities:

* Symbol resolution
* Relocation patching
* Vtable finalization
* Inheritance resolution
* Layout of combined code/data sections
* Optional dynamic linking

### Linker Output Formats

* **Flat binary** for Zeal OS
* **Extended binary** with relocation tables for dynamic overlays

---

# 18.9 Runtime Integration

The compiler depends on the runtime for:

* Memory allocation via pages
* Heap manager
* String operations
* Vtable lookup
* Exception frame installation (future phase)
* Syscall interfaces for I/O

This runtime is modular and versioned so programs compiled with older compilers remain compatible.

---

# 19. Application Binary Interface (ABI) Specification

This section defines the Zeal Pascal ABI: calling conventions, stack frames, name mangling, object layout, and exception frame structure. It ensures compatibility between separately compiled units, dynamic libraries, and future compiler versions.

## 19.1 Register Usage Policy

* **AF**: temporary arithmetic, flags
* **BC**: scratch / block copy operations
* **DE**: scratch / data pointer / function temporaries
* **HL**: primary return register; pointer and integer return values
* **IX**: frame pointer for Pascal functions
* **IY**: reserved for runtime system use (exception frames or thread-local state)

## 19.2 Calling Convention

### 19.2.1 Parameter Passing

* All parameters are passed **on the stack**, right-to-left.
* `var` parameters are **addresses** pushed instead of values.
* Hidden parameters precede user parameters:

    * First hidden parameter: destination buffer pointer (for large returns)
    * Second hidden parameter: `Self` pointer for methods

### 19.2.2 Return Values

* **Integer, Boolean, Char, Pointer, Class Reference**: returned in `HL`.
* **Sets, Records, Arrays**: returned via hidden first parameter.

### 19.2.3 Stack Frame Layout

```
| caller params       |
| return address      |
| old IX              | ← IX on entry
| local variables     |
| temps/spill slots   |
| ------------------- |
IX+0  old IX
IX+2  return addr
IX+4  first param
...
```

### 19.2.4 Prologue/Epilogue

```
; prologue
push ix
ld ix,0
add ix,sp
sub sp,<localsize>

; epilogue
ld sp,ix
pop ix
ret
```

---

## 19.3 Name Mangling Rules

Used for units, overloaded methods (future), class methods.

```
<unit>_<symbol>
<class>_<method>
<class>_<method>_V<n>   ; virtual index
```

Names stored in symbol tables are unmangled; mangled names exist only in object/unit files.

---

## 19.4 Exception Frame Layout

Exceptions use a **setjmp/longjmp-like model**.

### Frame Structure

```
Offset Size  Meaning
0      2     Previous exception frame pointer
2      2     Handler address
4      2     SP snapshot
6      2     IX snapshot
8      1     Active flag
```

Frames are stored in a runtime-managed linked list.

### Raising an Exception

1. Runtime looks up nearest frame
2. Restores SP, IX
3. Jumps to handler
4. Handler uses `raise` to continue unwinding if needed

---

# 20. Compiler Data Structures Specification

These structures define how the compiler stores tokens, symbols, types, AST nodes, IR, and metadata.

## 20.1 Token Structure (8 bytes)

```
0: token kind (byte)
1: flags
2-3: source line
4-5: source column
6-7: value index (string pool, number table, symbol index)
```

Tokens are stored in contiguous paged arenas.

---

## 20.2 Symbol Table Entry

```
struct Symbol {
  name_index: word
  kind: byte   ; var, const, type, proc, func, field, method
  type_index: word
  section: byte ; code, data, bss
  offset: word ; offset inside section
  flags: byte  ; exported, imported, virtual
  scope_level: byte
}
```

Stored in paged hash table: each bucket covers a range of name hashes.

---

## 20.3 Type Descriptor

```
struct TypeDesc {
  kind: byte ; integer, boolean, char, array, record, class, pointer, set
  size: word
  aux: word ; element type, parent type, etc.
  flags: byte ; packed, has_vtable
}
```

Class types have extended descriptors referencing vtables and method tables.

---

## 20.4 AST Nodes

Used in multi-pass phases.

```
struct ASTNode {
  kind: byte
  type_index: word
  left: word
  right: word
  data: word ; literal index, symbol index, etc.
}
```

Nodes live in paged arenas allocated per compilation unit.

---

## 20.5 IR (Intermediate Representation)

Two forms:

### IR1 (Linear IR)

```
OPCODE | arg1 | arg2 | arg3
```

### IR2 (Quadruples)

```
(t) = a OP b
if t RELOP x goto L
call f, n_params
```

IR instructions are packed for sequential scans and peephole passes.

---

## 20.6 Unit Metadata Tables

Stored inside `.ZOU` files.

* Export table
* Import table
* Type table
* Class table
* Vtable definitions
* Initialization/finalization procedure addresses

---

# 21. Debugger PRD

The Zeal Pascal debugger provides breakpoints, single-step, watch variables, and IDE integration.

## 21.1 Debugger Architecture

Debugger consists of:

1. **Debug Monitor** (runs inside Zeal OS)
2. **Debug Info Loader** (reads `.ZDBG` files)
3. **Execution Controller** (patches instructions with traps)
4. **IDE Interface Layer** (Zepto integration)

---

## 21.2 Breakpoints

Breakpoints implemented via **trap opcode injection**:

* Replace instruction at address with `RST 30h` (reserved breakpoint vector)
* On trap:

    * Debug monitor halts CPU
    * Registers captured
    * Stack displayed
    * IDE notified

Breakpoints stored in a table:

```
addr | original_byte
```

---

## 21.3 Single-Stepping

Implemented via:

* Temporary breakpoint at the next instruction
* Step-over uses call-depth tracking

Debug monitor maintains:

* Last executed address
* Depth counter for call/ret tracking

---

## 21.4 Variable Inspection

Using `.ZDBG` metadata:

* Function frame layout read from debug info
* Locals resolved via IX offsets
* Watch expressions evaluate symbol values by computing effective addresses

---

## 21.5 Memory & Register Viewing

The debugger can:

* Dump memory pages
* Show vtable contents
* Show instance layouts
* Display floating-point registers (software registers) in Phase 6+
* Inspect heap blocks

---

## 21.6 IDE Integration

Zepto receives:

* Breakpoint events
* Error messages
* Step events
* Variable snapshots

Debugger UI features:

* Source listing with arrow indicating current instruction
* Breakpoint markers
* Watch panel
* Call stack panel
* CPU state panel

---

# 22. Runtime Architecture Specification

The Zeal Pascal Runtime (ZPR) provides memory management, object support, string operations, page allocation, exception handling infrastructure, and system-call wrappers. It is designed to be compact yet scalable, making full use of Zeal’s 16 KB page model and 512 KB SRAM.

---

## 22.1 Memory Architecture Overview

The runtime divides memory into structured regions:

### **22.1.1 Static Regions**

* **Code pages:** loaded program code, mapped contiguously.
* **Data/BSS pages:** global variables, constants.
* **Runtime pages:** permanent pages used by ZPR.

### **22.1.2 Dynamic Regions**

Allocated at runtime:

* **Heap pages** — for objects, strings, arrays.
* **Exception frame pages** — stored in IY-managed lists.
* **Scratch pages** — used temporarily for calculations or large buffers.

### **22.1.3 Page Allocator**

ZPR uses a thin wrapper over Zeal OS syscalls:

* `AllocPage()`
* `FreePage()`
* `MapPage()`

Pages are grouped into **arenas** for:

* Heap
* Strings
* Classes
* Temporary workspaces (compiler phases)

---

## 22.2 Heap Manager

The heap manager supports two allocation strategies:

### **22.2.1 Small Block Heap (SBH)**

* For objects < 256 bytes
* Maintains free lists per size class
* Minimizes fragmentation

### **22.2.2 Large Block Allocator (LBA)**

* For arrays, records, strings > 256 bytes
* Allocates whole blocks within a page
* Can span multiple pages using block chaining

### **22.2.3 Allocation Algorithm**

```
Alloc(size):
  if size < 256 -> SBH
  else -> LBA
```

### **22.2.4 Deallocation**

SBH: return block to free list.
LBA: return one or more pages to arena.

---

## 22.3 Runtime Data Structures

### **22.3.1 Class Instances**

```
+0: pointer to vtable
+2..n: fields
```

### **22.3.2 Strings (ShortString model)**

```
byte length
char data[length]
padding
```

### **22.3.3 Dynamic Arrays** (Phase 5+)

```
pointer data
word length
word capacity
```

---

## 22.4 Exception Runtime

### **22.4.1 Exception Frame Stack (IY)**

```
IY -> latest frame
Prev -> older frames
```

### **22.4.2 Raising Exceptions**

* Runtime unwinds through frames
* Restores SP, IX
* Jumps to handler address

---

## 22.5 Syscall Layer

Thin wrappers around `RST 08h` provide:

* File I/O
* Console I/O
* Page allocation
* Process exit

---

# 23. Dynamic Loader and Overlay Manager Specification

Dynamic loading allows Zeal Pascal to execute programs and libraries larger than 48 KB.

---

## 23.1 Loader Overview

Supports loading of:

* `.ZOU` unit files
* `.ZOF` relocatable object files
* Shared runtime libraries

The loader maps pages and performs relocations.

---

## 23.2 Load Process

```
LoadUnit(name):
  read header
  allocate pages
  map code/data sections
  apply relocations
  register exports
  run initialization code
```

### **23.2.1 Relocation Types**

* Absolute address patch
* PC-relative
* Vtable-relative
* Import symbol resolution

---

## 23.3 Overlay System

Large applications may load pages on demand.

### **23.3.1 Overlay Table**

```
OverlayID → file offset, page count, relocation table
```

### **23.3.2 Loading Algorithm**

```
If overlay not loaded:
  free LRU overlay pages
  allocate pages
  load overlay
  fix relocations
```

---

# 24. Full Standard Library Blueprint

The standard library is divided into mandatory and optional units.

---

## 24.1 SYSTEM (Core)

* Type conversions
* Math primitives
* Heap management
* Page allocator wrappers
* Exit/Halt
* Pointer utilities

---

## 24.2 SYSUTILS-Lite

* Error codes
* Exception base types
* Conversions (`StrToInt`, `IntToStr`)
* File helpers

---

## 24.3 STRINGS

* `Copy`, `Insert`, `Delete`, `Concat`
* Comparisons
* Search functions

---

## 24.4 MATH

* Integer operations
* FP operations (Phase 6)
* Trig and logarithmic functions

---

## 24.5 IO

* High-level file I/O
* Buffered readers/writers

---

## 24.6 CLASSES (Phase 4+)

* Base class `TObject`
* Streaming helpers
* Metadata for RTTI (Phase 7)

---

# 25. Full Zeal Pascal Grammar (Expanded EBNF)

### **25.1 Program Structure**

```
program     = "program" ident ";" block "." ;
unit        = "unit" ident ";" interface-part implementation-part init-part? "." ;
```

### **25.2 Blocks**

```
block       = label-part? const-part? type-part? var-part? routine-part? statement-part ;
routine-part= { procedure | function | class-def } ;
```

### **25.3 Statements**

```
statement       = simple-stmt | structured-stmt ;
simple-stmt     = assign | call | goto | empty ;
structured-stmt = if | while | for | repeat | case | try ;
```

### **25.4 Expressions**

```
expr        = simple-expr ( rel-op simple-expr )? ;
simple-expr = ("+" | "-")? term { add-op term } ;
term        = factor { mul-op factor } ;
factor      = ident | number | string | "(" expr ")" | "not" factor | set-ctor ;
```

### **25.5 Type Definitions**

```
type-spec   = simple-type | array-type | record-type | class-type ;
array-type  = "array" "[" range "]" "of" type-spec ;
record-type = "record" field-list "end" ;
class-type  = "class" ("(" ident ")")? class-body "end" ;
```

### **25.6 Class Bodies**

```
class-body = { visibility-section } ;
visibility-section = vis-keyword ":" { field | method } ;
vis-keyword = "public" | "private" | "protected" ;
```

### **25.7 Methods**

```
method = (procedure | function) ( "virtual" | "override" )? ";" ;
```

### **25.8 Try/Except**

```
try = "try" statement-part ( "except" statement-part )? ( "finally" statement-part )? "end" ;
```

(Early phases ignore exception semantics.)

---

# 26. Final Summary

With the runtime architecture, dynamic loader specification, full standard library blueprint, and expanded grammar now integrated, the Zeal Pascal PRD is a **complete, end-to-end specification** for building the most advanced Pascal ecosystem ever created for an 8-bit system.

This foundational document now defines:

* Execution and memory model
* Compiler and toolchain architecture
* Object system
* ABI and file formats
* Debugger and IDE integration
* Standard library and runtime
* Full language grammar

With ABI, compiler data structures, and the debugger PRD added, the Zeal Pascal specification now defines **every major component** required for implementation:

* Full language spec
* Full compiler architecture
* Object & unit formats
* Runtime model
* Debugger and IDE interface

This establishes Zeal Pascal as a comprehensive, modern, 8-bit development ecosystem grounded in rigorous specification and designed for long-term extensibility.

With the compiler architecture now fully defined — including frontend, IR, backend, optimizer, linker, and runtime layers — Zeal Pascal has a complete technical blueprint for implementation. The architecture is specifically tuned to exploit Zeal’s FPGA-offloaded video subsystem and expansive paged memory, enabling multi-pass compilation, object orientation, dynamic linking, and a modern IDE ecosystem on an 8-bit platform.

With object/unit binary formats, stable vtable structure, standard library specification, and debug info format defined, Zeal Pascal now has the full blueprint required to build:

* A complete multi-pass compiler
* A class-based runtime
* A linker and dynamic loader
* An IDE-aware debugger

This elevates Zeal Pascal from a simple language implementation to a **complete 8-bit development ecosystem**, uniquely enabled by Zeal’s FPGA offload and 512 KB paged architecture.

Zeal Pascal is envisioned as a tiered, standards-informed Pascal implementation that starts from a rigorous ISO Pascal core and grows into a full-featured Object Pascal environment. By leveraging Zeal’s unique combination of a Z80 CPU, FPGA-driven video, and 512 KB of paged SRAM, this project aims to deliver the most capable 8-bit Pascal workstation ever built—self-hosted, developer-friendly, and extensible for years of experimentation and growth.
