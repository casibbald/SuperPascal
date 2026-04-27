Below is the **first high-level draft** of the **ZealPascal Language Specification**, built directly on the **three-tier architecture** we established earlier in the ZealPascal PRD:

* **Tier 1: ISO 7185 Standard Pascal Core**
* **Tier 2: Turbo Pascal Extensions**
* **Tier 3: Object Pascal Subset (ZealPascal OOP)**

This is the **starting point** for our full ZealPascal Language Specification.
We will iterate, refine, and eventually expand this into the *formal specification* used by the compiler and toolchain.

---

# 📘 **ZealPascal Language Specification (High-Level Draft v0.1)**

---

# **0. Language Philosophy**

ZealPascal is a structured, statically-typed programming language for the **Zeal 8-bit Computer**, inspired by:

* ISO Standard Pascal (core correctness and simplicity)
* Turbo Pascal (practical modules, strings, sets, files)
* Object Pascal (class system, virtual methods, instance architecture)

ZealPascal aims to be:

* **Predictable** — deterministic behavior, clear semantics
* **Fast** — optimized for the Z80 architecture using SoA layouts and ECS-friendly codegen
* **Expressive** — supports OOP, modules, animations, scripting, tilemap DSL
* **Productive** — integrated directly with ZealIDE, asset pipelines, game engine APIs

This draft defines the high-level structure of the language.

---

# **1. Tier 1 — ISO 7185 Standard Pascal Core**

This is the **foundation** of ZealPascal.

## **1.1 Basic Types**

* `integer` (16-bit signed)
* `boolean` (`true`, `false`)
* `char` (8-bit)
* `byte` (0..255)
* `word` (0..65535)
* Enumerations
* Subranges (`1..10`)

## **1.2 Composite Types**

* Records
* Arrays
* Sets (bitsets for ordinal types)
* Pointers

## **1.3 Variable & Constant Declarations**

Standard Pascal syntax:

```pascal
var
  x: integer;
  y: array[0..9] of byte;

const
  MaxScore = 1000;
```

## **1.4 Control Flow**

* `if … then … else`
* `while … do`
* `repeat … until`
* `for … to | downto`
* `case`

## **1.5 Procedures & Functions**

```pascal
procedure MovePlayer;
function Distance(x, y: integer): integer;
```

## **1.6 Program Structure**

```pascal
program GameMain;
begin
  ...
end.
```

## **1.7 Memory Model**

* Stack-based call frames using `IX`
* Static globals in `.data` / `.bss`
* No garbage collection; manual memory via `New`/`Dispose`

This tier provides all **baseline Pascal correctness** and is suitable for tools, utilities, and basic gameplay code.

---

# **2. Tier 2 — Turbo Pascal Extensions (ZealPascal Practical Layer)**

This tier makes ZealPascal **usable for real-world development**.

## **2.1 Units (Modular Compilation)**

ZealPascal supports Turbo Pascal–style units:

```pascal
unit Player;
interface
  procedure InitPlayer;
implementation
  procedure InitPlayer;
  begin
  end;
end.
```

### Features:

* `interface` / `implementation` sections
* Type and symbol export tables
* Separate compilation into `.ZOU` unit files

## **2.2 Strings**

ZealPascal supports **short strings**:

* 1-byte length prefix
* Max length = 255

```pascal
var s: string;
s := 'Hello Zeal!';
```

## **2.3 Sets**

Bitset-backed sets:

```pascal
var A: set of 0..7;
A := [1, 2, 3];
```

## **2.4 File I/O**

Mapped to Zeal OS syscalls:

```pascal
assign(f, 'save1.dat');
rewrite(f);
write(f, x);
close(f);
```

## **2.5 Packed Types**

For compact data structures:

```pascal
type TSmall = packed record
```

## **2.6 Compiler Directives**

* `{$INLINE ON}`
* `{$O+}` (optimizations)
* `{$VBLANK_AWARE ON}`
* `{$UNROLL}`

## **2.7 Tilemap DSL (ZealPascal Extension)**

Unique to ZealPascal:

```pascal
const Level1: TTileMap = (
  Width: 16; Height: 16;
  Data: (
    ROW('################'),
    ROW('#..............#'),
    ...
  )
);
```

This tier is essential for ZealPascal’s **graphics, physics, ECS, and engine integration**.

---

# **3. Tier 3 — ZealPascal Object Pascal Subset (OOP Layer)**

This tier introduces **Object Pascal**, adapted for Z80 efficiency.

## **3.1 Class Types**

Basic syntax:

```pascal
type
  TEntity = class
    X, Y: integer;
    procedure Update; virtual;
  end;
```

## **3.2 Inheritance**

Single inheritance supported:

```pascal
type
  TEnemy = class(TEntity)
    procedure Update; override;
  end;
```

## **3.3 Virtual Methods & VTables**

* Vtable pointer stored at offset 0 of each instance
* Virtual dispatch implemented via indirect `jp (hl)`
* Overrides replace inherited vtable slots

## **3.4 Constructors & Destructors**

```pascal
constructor Create(x, y: integer);
destructor Destroy; override;
```

Allocated via ZealPascal heap manager.

## **3.5 Visibility Modifiers**

* `public`
* `private`
* `protected` (optional)

## **3.6 RTTI (Later Tier)**

* Basic type name support
* Optional class metadata for debugging

### Notes:

The OOP system is intentionally compact to remain efficient on Z80 while still powerful enough for:

* ECS integration
* Game entity hierarchies
* Script binding
* UI widgets
* Animation controllers

---

# **4. Cross-Tier Integration**

ZealPascal’s language tiers combine to support:

### **Graphics & ZVB Integration**

* Tilemaps, sprites, animations
* Memory-mapped IO primitives
* Palette and DMA utilities

### **ECS Architecture**

* SoA component arrays
* Static archetypes
* System schedules
* Change detection

### **Game Engine Features**

* Scenes, transitions, scripting, physics, audio

### **Toolchain Integration**

* ZealIDE asset pipelines (tiles, sprites, audio, scenes)
* Unit builder (`.ZOU`)
* Asset packages (`.ZPK`)

---

# **5. Next Steps (for Iteration)**

We can now begin expanding the specification in depth:

### 🧩 Detailed syntax & grammar

* Expand the high-level grammar into a full EBNF document
* Specify operator precedence, parsing rules

### 🔧 Semantic rules

* Type compatibility
* Overload resolution
* Record layout rules
* Class initialization semantics

### 🏗️ ABI & Codegen Specification

* Stack frame layout
* Calling conventions
* Vtable structure
* Exception frame design

### 🧰 Standard library specification

* `System`, `Math`, `Strings`, `SysUtils-Lite`, `ZVB_*` modules

### 🔬 Compliance tests

* Tier-by-tier conformance test suite
* Syntactic and semantic error tests

---

# ✔️ Specification v0.1 Complete

This is our **first high-level definition** of the ZealPascal language: its structure, tiers, capability envelopes, and the philosophy behind it.

When you’re ready, we will begin fleshing out:

* **v0.2: Full grammar**
* **v0.3: Semantics + type system**
* **v0.4: ABI + codegen rules**
* **v0.5: Standard library specification**

Just tell me:

👉 **“Proceed to ZealPascal v0.2 Grammar Specification.”**
