# 📘 ZealPascal Standard Library Specification — Full Rewrite (v1.0 Draft)

## A Modern, Educational, Hardware-Aware Standard Library for the Zeal 8-bit Computer

This document defines the **complete, modernized, and unified Standard Library for ZealPascal**, rewritten from scratch to incorporate:

* The hybrid **OOP + Struct** model
* The needs of **education without internet access**
* Hardware acceleration via ZVB FPGA
* Full mathematics support (integer, fixed-point, floating-point, and trigonometry)
* Game engine integration (graphics, audio, input, ECS)

This is the authoritative high-level API and behavioral contract for all ZealPascal programs.

---

# 0. Design Goals

### 🎯 Educational Clarity

Students must be able to:

* Learn structured programming safely
* Build games and simulations easily
* Perform mathematics with real trig functions

### ⚙️ Z80-Friendly Efficiency

All operations must:

* Run efficiently on 10 MHz Z80
* Leverage memory pages & FPGA peripherals when appropriate

### 🧩 Orthogonality & Coherence

Each unit has a clear purpose.
No redundant APIs. No legacy clutter.

### 🔐 Safety

* Bounds checking in debug builds
* Safe string handling
* No silent overflows

### 🧠 Modernized Pascal

* Unified `string`
* Fixed-point math
* Optional 32-bit floats
* Clean error messages

---

# 1. Core Runtime Unit — `System`

The `System` unit is implicitly included in all programs.
It defines the fundamental language operations and types.

## 1.1 Primitive Types

```
Integer  = 16-bit signed
Byte     = 8-bit unsigned
Word     = 16-bit unsigned
Boolean  = true|false
Char     = 8-bit ASCII
String   = shortstring (max 255 chars)
```

## 1.2 Memory Operations

```
procedure Move(const Source; var Dest; Count: Word);
procedure FillChar(var X; Count: Word; Value: Byte);
procedure GetMem(var P; Size: Word);
procedure FreeMem(var P; Size: Word);
```

## 1.3 Program Control

```
procedure Halt(ExitCode: Integer);
```

## 1.4 Runtime Guarantees

* Debug builds enforce bounds & overflow checks
* Release builds may relax checks for speed

---

# 2. Math Units Suite

ZealPascal provides **three complementary math models**:

1. **Integer math** (fast, always available)
2. **Fixed-point math** (fast trig & physics)
3. **Float math** (teaching correctness over speed)

## 2.1 `Math` — Basic Integer Math

```
function Abs(I: Integer): Integer;
function Sqr(I: Integer): Integer;
function Max(A,B: Integer): Integer;
function Min(A,B: Integer): Integer;
```

---

# 3. Fixed-Point Math — `MathFP`

Used for games, physics, animation.

## 3.1 Types

```
type
  Q8_8   = record value: Integer; end;   // range −128..127.996
  Q12_12 = record value: Integer; end;   // extended precision
```

## 3.2 Constructors & Conversion

```
function ToQ8_8(i: Integer): Q8_8;
function ToInt(x: Q8_8): Integer;
```

## 3.3 Arithmetic

```
function Add(a,b: Q8_8): Q8_8;
function Sub(a,b: Q8_8): Q8_8;
function Mul(a,b: Q8_8): Q8_8;
function Div(a,b: Q8_8): Q8_8;
```

---

# 4. Full Trigonometry for Teaching — `MathTrig`

The complete set of trigonometric functions is available for **Float32** and **fixed-point**.

### 4.1 Float32 Trig (Teaching)

```
function Sin(x: Float32): Float32;
function Cos(x: Float32): Float32;
function Tan(x: Float32): Float32;
function Sec(x: Float32): Float32;   // 1/cos
function Csc(x: Float32): Float32;   // 1/sin
function Cot(x: Float32): Float32;   // 1/tan

function ArcSin(x: Float32): Float32;
function ArcCos(x: Float32): Float32;
function ArcTan(x: Float32): Float32;
```

These may be slow but mathematically correct.

### 4.2 Fixed-Point Trig (Games)

```
function SinQ(x: Q8_8): Q8_8;
function CosQ(x: Q8_8): Q8_8;
function TanQ(x: Q8_8): Q8_8;
function SecQ(x: Q8_8): Q8_8;
function CscQ(x: Q8_8): Q8_8;
function CotQ(x: Q8_8): Q8_8;
```

Implementation:

* LUT (1024 entries)
* Linear interpolation
* Z80-fast

---

# 5. Floating-Point (Optional) — `MathFloat`

For scientific computing and teaching real-world math.

```
type Float32 = record value: LongInt; end;
```

Operations:

* add, sub, mul, div
* trig via polynomial approximation or LUT

Floating-point is optional but recommended for educational bundles.

---

# 6. FPGA-Accelerated Math — `MathHW`

Optional future expansion leveraging the ZVB FPGA.

## 6.1 Hardware Intrinsics

```
function HW_Sin(x: Q12_12): Q12_12; intrinsic;
function HW_Cos(x: Q12_12): Q12_12; intrinsic;
function HW_Tan(x: Q12_12): Q12_12; intrinsic;
```

## 6.2 Vector & Matrix Operations

```
procedure HW_VectorNormalize(var X,Y: Q12_12); intrinsic;
function HW_MatrixMul(A,B: PMatrix3x3): PMatrix3x3; intrinsic;
```

This makes Zeal a world-class math teaching machine.

---

# 7. Strings Unit — `Strings`

Unified short-string model.

## 7.1 Basic Operations

```
function Length(const S: string): Byte;
function Copy(const S: string; Index, Count: Byte): string;
procedure Delete(var S: string; Index, Count: Byte);
procedure Insert(const SubStr: string; var S: string; Index: Byte);
function Concat(const A,B: string): string;
```

String overflow must raise clear errors in debug builds.

---

# 8. Utility Unit — `SysUtils_Lite`

Simplified utility unit.

## 8.1 Conversions

```
function IntToStr(Value: Integer): string;
function StrToInt(const S: string): Integer;
function ByteToHex(B: Byte): string;
```

## 8.2 Errors

```
type EError = class
  Code: Integer;
  Msg: string;
end;

procedure RaiseError(Code: Integer; const Msg: string);
```

---

# 9. Hardware Units (ZVB)

## 9.1 `ZVB_Graphics`

```
procedure SetMode(Mode: Byte);
procedure ClearScreen(Color: Byte);
procedure DrawTile(X,Y: Word; TileID: Word);
```

## 9.2 `ZVB_Sprites`

```
procedure SpriteSet(ID: Byte; X,Y: Word; Tile, Pal: Byte);
procedure SpriteShow(ID: Byte);
procedure SpriteHide(ID: Byte);
```

## 9.3 `ZVB_Audio`

```
procedure PlaySFX(ID: Byte);
procedure PlayMusic(ID: Byte);
procedure StopMusic;
```

## 9.4 `ZVB_Input`

```
function KeyDown(Code: Byte): Boolean;
function Axis(Name: string): Integer;
```

---

# 10. Engine Units

## 10.1 ECS

```
function CreateEntity: TEntityID;
procedure DestroyEntity(E: TEntityID);
```

## 10.2 Scenes

```
type TScene = class
  procedure Init; virtual;
  procedure Update; virtual;
  procedure Render; virtual;
end;

procedure SetScene(Scene: TScene);
```

## 10.3 UI

Future expansion for teaching UI programming.

---

# 11. Educational Profile (Beginner Mode)

Beginner projects have access to:

* `System` (limited)
* `Math`
* `Strings` (basic)
* `ZVB_Graphics` (simple drawing)
* `ZVB_Input`

Hidden in beginner mode:

* Pointers
* GetMem/FreeMem
* SysUtils_Lite error system
* Floating point APIs
* ECS units

Unlockable per curriculum stage.

---

# ✔️ ZealPascal Standard Library Specification — Rewrite Complete

This rewritten spec defines:

* Complete math system (integer, fixed-point, floating-point)
* Full 6-function trig (sin, cos, tan, sec, csc, cot)
* Optional inverse trig
* Optional FPGA-accelerated math
* Unified strings
* Clean, educational runtime
* Full hardware wrappers
* Engine and ECS integration

This is the new canonical standard library baseline for the ZealPascal ecosystem.
