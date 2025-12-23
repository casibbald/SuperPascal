# SuperPascal Language Specification — Language Extensions

## Superset Features Beyond Standard Pascal

**Version:** 1.0  
**Part of:** SuperPascal Language Specification

---

## 1. Overview

SuperPascal is designed as a **superset** of standard Pascal (ISO 7185), meaning it accepts all valid Pascal programs while adding modern extensions. This document describes the extensions that go beyond standard Pascal syntax and semantics.

**Extensions covered:**
1. **C-Style Struct Syntax** — Alternative record syntax
2. **Array Slicing** — Python-style slice syntax (`arr[start:end]`)
3. **String Interpolation** — Embed expressions in strings
4. **Fixed-Point Types** — Q8.8, Q12.12 for game development
5. **Tuples** — Lightweight grouping of multiple values
6. **Heterogeneous Lists** — Python-like lists with mixed types
7. **Python-like `in` Operator** — Membership testing for arrays, lists, strings, tuples
8. **Python-style Set Operations** — `|`, `&`, `^` operators and set methods
9. **C-Style Pointer Arithmetic** — Full pointer arithmetic with type-aware scaling
10. **Rust-Style Module System** — Directory-based modules with qualified names
11. **Namespace Support** — Explicit namespaces, `using` statements, and aliases

---

## 2. C-Style Struct Syntax

### 2.1 Overview

SuperPascal allows **C-style struct syntax** as an alternative to Pascal's `record` syntax. This makes SuperPascal more familiar to C/C++ programmers and provides syntactic flexibility.

### 2.2 Syntax Comparison

#### Pascal-Style (Standard)

```pascal
type
  TVec2 = record
    X, Y: integer;
  end;
  
  TPlayer = record
    Name: string;
    Position: TVec2;
    Score: integer;
  end;
```

#### C-Style (Extension)

```pascal
type
  TVec2 = struct {
    integer X, Y;
  };
  
  TPlayer = struct {
    string Name;
    TVec2 Position;
    integer Score;
  };
```

### 2.3 Key Differences

| Feature | Pascal-Style | C-Style |
|---------|--------------|---------|
| Keyword | `record` | `struct` |
| Delimiters | `record ... end` | `{ ... }` |
| Field syntax | `name: type;` | `type name;` |
| Semicolon | Optional after `end` | Required after `}` |
| Anonymous | Not supported | Supported |

### 2.4 Anonymous Structs

C-style syntax supports anonymous structs (structs without a type name):

```pascal
var point = struct {
  integer X, Y;
};

point.X := 10;
point.Y := 20;
```

**Use cases:**
- One-off data structures
- Local data organization
- Temporary grouping

**Limitations:**
- Cannot be used in type declarations
- Cannot be passed as parameters (must use type alias)
- Cannot be returned from functions (must use type alias)

### 2.5 Field Declaration Syntax

**Pascal-style** (name first, type second):
```pascal
X, Y: integer;
Name: string;
```

**C-style** (type first, name second):
```pascal
integer X, Y;
string Name;
```

**Multiple fields of same type:**
- Pascal: `X, Y, Z: integer;`
- C-style: `integer X, Y, Z;`

### 2.6 Semantic Equivalence

Both syntaxes are **semantically identical**:

- Same memory layout
- Same type compatibility rules (nominal typing)
- Same field access syntax (`record.field`)
- Same method support
- Same assignment semantics

**Example:**
```pascal
type
  R1 = record X, Y: integer; end;
  R2 = struct { integer X, Y; };
  
var r1: R1;
var r2: R2;

r1.X := 10;  // Same syntax
r2.X := 10;  // Same syntax

// But R1 and R2 are different types (nominal typing)
// r1 := r2;  // ERROR: incompatible types
```

### 2.7 Mixing Syntax

You can freely mix both syntaxes in the same program:

```pascal
type
  TVec2 = record
    X, Y: integer;
  end;
  
  TPoint = struct {
    integer X, Y;
  };
  
  TRect = record
    TopLeft: TPoint;      // C-style struct in Pascal record
    BottomRight: TVec2;   // Pascal record in Pascal record
  end;
```

### 2.8 Methods in C-Style Structs

C-style structs support methods just like Pascal records:

```pascal
type
  TVec2 = struct {
    integer X, Y;
    procedure Add(const B: TVec2);
  };
  
implementation
  procedure TVec2.Add(const B: TVec2);
  begin
    X := X + B.X;
    Y := Y + B.Y;
  end;
```

---

## 3. Initialization Syntax Extensions

### 3.1 C-Style Initialization

SuperPascal supports C-style struct initialization:

**Pascal-style:**
```pascal
var v: TVec2;
v.X := 10;
v.Y := 20;
```

**C-style initialization:**
```pascal
var v: TVec2 = {10, 20};           // Positional
var v2: TVec2 = {X: 10, Y: 20};   // Named fields
```

**Rules:**
- Positional initialization: values in field declaration order
- Named initialization: explicit field names
- Mixing allowed: `{X: 10, 20}` (named then positional)
- Partial initialization: uninitialized fields are zero

**Examples:**
```pascal
type
  TPlayer = struct {
    string Name;
    integer Score;
    boolean Active;
  };

var p1: TPlayer = {'Player1', 100, true};
var p2: TPlayer = {Name: 'Player2', Score: 200, Active: true};
var p3: TPlayer = {Name: 'Player3'};  // Score and Active are 0/false
```

### 3.2 Array Initialization

C-style array initialization:

**Pascal-style:**
```pascal
var arr: array[0..2] of integer;
arr[0] := 1;
arr[1] := 2;
arr[2] := 3;
```

**C-style:**
```pascal
var arr: array[0..2] of integer = {1, 2, 3};
```

---

## 4. Type Inference Extensions

### 4.1 Auto Type for Variables

SuperPascal supports `auto` keyword for type inference (future feature):

```pascal
var x: auto := 42;        // Inferred as integer
var s: auto := 'Hello';   // Inferred as string
var v: auto := {10, 20};  // Inferred as struct type
```

**Status**: Planned for future version, not in v1.0.

### 4.2 Function Return Type Inference

Future feature: infer function return types from `Result` assignments:

```pascal
function Add(a, b: integer): auto;  // Future: inferred as integer
begin
  Result := a + b;
end;
```

**Status**: Planned for future version, not in v1.0.

---

## 5. Operator Overloading (Tier 2)

SuperPascal supports operator overloading in Tier 2 (available but not taught until after Tier 1 mastery):

```pascal
operator + (a, b: TVec2): TVec2;
begin
  Result.X := a.X + b.X;
  Result.Y := a.Y + b.Y;
end;

var v1, v2, v3: TVec2;
v3 := v1 + v2;  // Uses overloaded operator
```

**Status**: Available in Tier 2, not taught in Tier 1 curriculum.
**Educational Note**: Students learn predictable operator behavior in Tier 1, then learn operator overloading as an advanced feature in Tier 2.

---

## 6. Anonymous Functions

**Status:** ✅ **FULLY IMPLEMENTED** in v1.0

SuperPascal supports anonymous functions and procedures with closure capture:

```pascal
var
  Square: function(x: integer): integer;
begin
  Square := function(x: integer): integer begin
    Result := x * x;
  end;
  
  WriteLn(Square(5));  // 25
end.
```

**Features:**
- ✅ Anonymous function syntax: `function(params): return_type begin ... end`
- ✅ Anonymous procedure syntax: `procedure(params) begin ... end`
- ✅ Closure capture (automatic detection of outer-scope variables)
- ✅ Runtime support for closures
- ✅ Full semantic analysis

**See:** [Modern Pascal Features - Anonymous Functions](./16_ModernPascalFeatures.md#2-anonymous-functions) for complete documentation.

---

## 7. Nullable Types (Future)

Future support for nullable value types:

```pascal
var x: integer?;  // Nullable integer
x := 42;
x := nil;  // Can be nil
if x <> nil then
  WriteLn(x^);  // Unwrap nullable
```

**Status**: Planned for future version, not in v1.0.

---

## 8. Extension Summary

### Implemented in v1.0

✅ **C-style struct syntax** (`struct` keyword, `{ }` braces)  
✅ **C-style field declarations** (`type name;` syntax)  
✅ **C-style initialization** (`{ ... }` initializers)  
✅ **Anonymous structs** (C-style only)  
✅ **Mixed syntax** (can mix `record` and `struct`)

### Available in Tier 2 (Not Taught in Tier 1)

✅ **Operator overloading** - Available in Tier 2, taught after Tier 1 mastery  
✅ **Generic types** (templates) - Available in Tier 2, taught after Tier 1 mastery  
✅ **Type helpers** (class helpers, record helpers) - Available in Tier 2, taught after Tier 1 mastery  
✅ **Anonymous functions** - Available in Tier 2, taught after Tier 1 mastery  
✅ **Dynamic arrays** - Available in Tier 2, taught after Tier 1 mastery  
✅ **Class methods** - Available in Tier 2, taught after Tier 1 mastery  
✅ **Class properties/variables** - Available in Tier 2, taught after Tier 1 mastery  
✅ **Attributes/Annotations** - Available in Tier 2, taught after Tier 1 mastery

### Fully Implemented Modern Pascal Features

✅ **All 50 Modern Pascal features** - See [Modern Pascal Features](./16_ModernPascalFeatures.md) for complete documentation

### Planned for Future Versions

☐ **Type inference** (`auto` keyword)  
☐ **Lambda expression syntax** (alternative to anonymous functions)  
☐ **Nullable types**

---

## 9. Compatibility Notes

### 9.1 Standard Pascal Compatibility

All standard Pascal programs are valid SuperPascal programs. The extensions are **additive only** - they don't break existing Pascal syntax.

### 9.2 Migration Path

Programs can be migrated incrementally:

1. Start with pure Pascal syntax
2. Gradually adopt C-style structs where preferred
3. Mix both styles as needed
4. No breaking changes required

### 9.3 Educational Considerations

For teaching:
- **Beginner tier**: Use Pascal-style only (`record`)
- **Intermediate tier**: Introduce C-style as alternative
- **Advanced tier**: Use both styles appropriately

---

## 10. Implementation Notes

### 10.1 Parser Handling

The parser must:
1. Recognize both `record` and `struct` keywords
2. Handle both `record ... end` and `{ ... }` delimiters
3. Parse both field declaration styles
4. Generate identical AST nodes for both syntaxes

### 10.2 Code Generation

Both syntaxes generate identical code:
- Same memory layout
- Same field access patterns
- Same method dispatch
- No runtime difference

### 10.3 Error Messages

Error messages should be syntax-agnostic:
```
Error: Field 'X' not found in type TVec2
```
(Works for both `record` and `struct` syntax)

---

## 11. Examples

### 11.1 ECS Components with C-Style

```pascal
type
  TPosition = struct {
    integer X, Y;
  };
  
  TVelocity = struct {
    integer VX, VY;
  };
  
  TSprite = struct {
    byte SpriteID;
    word TileID;
    byte Palette;
    boolean Visible;
  };
```

### 11.2 Mixed Syntax Example

```pascal
type
  // Pascal-style for main types
  TEntity = record
    ID: word;
    Position: TVec2;
  end;
  
  // C-style for components
  TVec2 = struct {
    integer X, Y;
  };
  
  // C-style initialization
  var player: TEntity = {
    ID: 1,
    Position: {X: 100, Y: 50}
  };
```

### 11.3 Anonymous Struct Usage

```pascal
// Temporary data grouping
var config = struct {
  integer Width, Height;
  boolean Fullscreen;
};

config.Width := 320;
config.Height := 240;
config.Fullscreen := false;
```

---

## 12. Array Slicing

### 12.1 Overview

**Array slicing** provides Python-style syntax for extracting subarrays. This is a SuperPascal extension that makes array manipulation more convenient and expressive.

### 12.2 Syntax

```
arr[start:end]        // Slice from start to end (end exclusive)
arr[start:end:step]  // Slice with step
arr[:end]            // From beginning to end
arr[start:]          // From start to end
arr[:]               // Copy entire array
arr[-n:]             // Last n elements
arr[:-n]             // All but last n
arr[::-1]            // Reverse array
```

### 12.3 Examples

```pascal
var arr: array[0..9] of integer;
var slice: array[0..9] of integer;
var i: integer;
begin
  // Initialize
  for i := 0 to 9 do
    arr[i] := i * 10;  // [0, 10, 20, 30, 40, 50, 60, 70, 80, 90]
  
  // Basic slicing
  slice := arr[1:5];   // [10, 20, 30, 40]
  slice := arr[:5];    // [0, 10, 20, 30, 40]
  slice := arr[3:];    // [30, 40, 50, 60, 70, 80, 90]
  slice := arr[:];     // [0, 10, 20, 30, 40, 50, 60, 70, 80, 90]
  
  // Step slicing
  slice := arr[0:10:2]; // [0, 20, 40, 60, 80]
  
  // Negative indices
  slice := arr[-3:];    // [70, 80, 90]
  slice := arr[:-2];    // [0, 10, 20, 30, 40, 50, 60, 70]
  
  // Reverse
  slice := arr[::-1];   // [90, 80, 70, 60, 50, 40, 30, 20, 10, 0]
end.
```

### 12.4 Slice Assignment

**Assign to slice (modifies original array):**
```pascal
var arr: array[0..9] of integer;
begin
  for i := 0 to 9 do
    arr[i] := i;
  
  // Replace slice
  arr[2:5] := [20, 30, 40];  // arr[2..4] = [20, 30, 40]
  // Result: [0, 1, 20, 30, 40, 5, 6, 7, 8, 9]
end.
```

### 12.5 Semantics

**Behavior:**
- **End exclusive** — `arr[1:5]` includes indices 1, 2, 3, 4 (not 5)
- **Negative indices** — Count from end: `-1` = last element, `-2` = second-to-last
- **Bounds clamping** — Out-of-bounds indices are clamped to valid range
- **Step direction** — Positive step = forward, negative step = reverse
- **Returns new array** — Slicing creates a new array (doesn't modify original)

**Performance:**
- **O(n)** — Linear time where n is slice length
- **Memory** — Allocates new array for slice result
- **Optimization** — Compiler optimizes when possible

### 12.6 Compatibility

**Standard Pascal alternative:**
```pascal
// SuperPascal slice syntax
slice := arr[1:5];

// Standard Pascal equivalent
Slice(arr, count, 1, 4, slice, sliceCount);
```

**Both syntaxes are supported** — Use whichever you prefer.

---

## 13. Tuples (SuperPascal Extension)

SuperPascal introduces **tuples** for lightweight grouping of multiple values without needing a full record type.

### 13.1 Syntax

**Tuple type declaration:**
```pascal
type TupleType = (Type1, Type2, Type3, ...);
```

**Tuple literal:**
```pascal
(value1, value2, value3, ...)
```

**Named tuple fields (optional):**
```pascal
(Field1: value1, Field2: value2, ...)
```

### 13.2 Behavior

- **Ordered collection** — Values are ordered and accessed by position
- **Type-safe** — Compiler checks tuple types
- **Lightweight** — No field names required (unlike records)
- **Multiple return values** — Functions can return tuples
- **Destructuring** — Can destructure tuples into variables

### 13.3 Examples

**Basic tuple:**
```pascal
var point: (integer, integer);
begin
  point := (10, 20);
  WriteLn('X: ', point[0], ', Y: ', point[1]);
end.
```

**Named tuple fields:**
```pascal
var point: (integer, integer);
begin
  point := (X: 10, Y: 20);  // Named fields (optional)
end.
```

**Multiple return values:**
```pascal
function Divide(a, b: integer): (boolean, integer);
begin
  if b <> 0 then
    Divide := (true, a div b)
  else
    Divide := (false, 0);
end;

var result: (boolean, integer);
var success: boolean;
var value: integer;
begin
  result := Divide(10, 2);
  success := result[0];
  value := result[1];
end.
```

**Destructuring assignment:**
```pascal
var point: (integer, integer);
var x, y: integer;
begin
  point := (10, 20);
  (x, y) := point;  // Destructure tuple
  // x = 10, y = 20
end.
```

**Tuple comparison:**
```pascal
var p1, p2: (integer, integer);
begin
  p1 := (10, 20);
  p2 := (10, 20);
  
  if p1 = p2 then
    WriteLn('Equal');
  
  p2 := (10, 21);
  if p1 < p2 then  // Compares element by element
    WriteLn('p1 < p2');
end.
```

### 13.4 Tuple vs Record

**Use tuples when:**
- Simple grouping of a few values
- Temporary data (don't need named type)
- Multiple return values from function
- Lightweight (no methods needed)

**Use records when:**
- Need named fields for clarity
- Need methods attached
- Complex structure with many fields
- Reusable type used in multiple places

### 13.5 Type Safety

Tuples are type-safe:
- Compiler checks tuple element types
- Tuple types must match exactly for assignment
- Different tuple types are incompatible

```pascal
var p1: (integer, integer);
var p2: (integer, integer);
var p3: (integer, integer, integer);  // Different type!

begin
  p1 := (10, 20);
  p2 := p1;      // OK: same type
  // p3 := p1;   // ERROR: incompatible types
end.
```

---

## 14. C-Style Pointer Arithmetic

### 14.1 Overview

SuperPascal extends standard Pascal with **full C-style pointer arithmetic**, allowing type-aware pointer manipulation for systems programming and low-level memory access.

**Standard Pascal**: Pointers exist but arithmetic is not allowed.  
**SuperPascal**: Full C-style pointer arithmetic with automatic type-aware scaling.

### 14.2 Pointer Arithmetic Operations

**Supported Operations:**
```pascal
var p, q: ^integer;
var i: integer;

p := @array[0];        // Address-of operator
p := p + 1;            // Increment by sizeof(integer) = 2 bytes
p := p - 1;            // Decrement by sizeof(integer) = 2 bytes
p := p + 5;            // Add integer to pointer (scaled)
q := p - 3;            // Subtract integer from pointer (scaled)
i := q - p;            // Pointer difference (returns integer, in elements)
```

### 14.3 Type-Aware Scaling

Pointer arithmetic is **automatically scaled** by the size of the pointed-to type:

```pascal
var
  pInt: ^integer;      // sizeof(integer) = 2 bytes
  pByte: ^byte;        // sizeof(byte) = 1 byte
  pVec: ^TVec2;        // sizeof(TVec2) = 4 bytes (example)

pInt := pInt + 1;      // Advances by 2 bytes (1 * sizeof(integer))
pByte := pByte + 1;    // Advances by 1 byte (1 * sizeof(byte))
pVec := pVec + 1;      // Advances by 4 bytes (1 * sizeof(TVec2))
```

**Why type-aware scaling:**
- Makes pointer arithmetic intuitive (elements, not bytes)
- Matches C/C++ behavior
- Prevents common off-by-size errors
- Educational: students learn about type sizes

### 14.4 Pointer Difference

Subtracting two pointers returns the **element difference**, not byte difference:

```pascal
var
  arr: array[0..9] of integer;
  p, q: ^integer;
  diff: integer;

p := @arr[0];          // Points to arr[0]
q := @arr[5];          // Points to arr[5]
diff := q - p;         // diff = 5 (elements), not 10 (bytes)
```

### 14.5 Safety Considerations

**No Automatic Bounds Checking:**
- Pointer arithmetic can access invalid memory
- Dereferencing invalid pointers causes undefined behavior
- No runtime validation of pointer bounds
- Students learn by debugging crashes

**Common Mistakes:**
```pascal
var
  arr: array[0..9] of integer;
  p: ^integer;

p := @arr[0];
p := p + 20;           // Points beyond array end
p^ := 42;              // Undefined behavior (no bounds check)
```

**Best Practices:**
- Use array indexing when possible (has bounds checking in debug mode)
- Use pointer arithmetic only when necessary (performance, hardware access)
- Validate bounds manually before dereferencing
- Test thoroughly in debug mode before release

### 14.6 Comparison with Peek/Poke

**Pointer Arithmetic:**
- Type-safe (points to specific type)
- Type-aware scaling (automatic)
- Can be used with any type
- No bounds checking

**Peek/Poke:**
- Raw memory access (byte/word level)
- Manual address calculation
- No type safety
- No bounds checking

**When to use each:**
- **Pointer arithmetic**: Traversing typed data structures, arrays of records
- **Peek/Poke**: Hardware registers, memory-mapped I/O, raw memory manipulation

### 14.7 Examples

**Traversing an Array:**
```pascal
var
  arr: array[0..9] of integer;
  p: ^integer;
  i: integer;

p := @arr[0];
for i := 0 to 9 do
begin
  p^ := i * 2;
  p := p + 1;          // Advance to next element
end;
```

**Pointer Difference:**
```pascal
var
  arr: array[0..99] of integer;
  start, end: ^integer;
  count: integer;

start := @arr[10];
end := @arr[50];
count := end - start;   // count = 40 (elements)
```

**Type-Aware Scaling:**
```pascal
type TVec2 = record
  X, Y: integer;
end;

var
  vecs: array[0..9] of TVec2;
  p: ^TVec2;

p := @vecs[0];
p := p + 5;            // Points to vecs[5] (advances by 5 * 4 = 20 bytes)
p^.X := 10;
```

---

## 15. Summary

SuperPascal extends standard Pascal with:

1. **C-style struct syntax**: Alternative to `record` with `struct` keyword
2. **C-style initialization**: `{ ... }` initializers
3. **Anonymous structs**: Structs without type names
4. **Array slicing**: Python-style slice syntax (`arr[start:end]`)
5. **Tuples**: Lightweight grouping of multiple values
6. **Heterogeneous lists**: Python-like lists with mixed types
7. **Python-like `in` operator**: Membership testing for arrays, lists, strings, tuples
8. **Python-style set operations**: `|`, `&`, `^` operators and set methods
9. **C-style pointer arithmetic**: Full pointer arithmetic with type-aware scaling
10. **Rust-style module system**: Directory-based modules with qualified names
11. **Namespace support**: Explicit `namespace` declarations, `using` statements, and aliases
12. **Mixed syntax**: Can use both styles in same program

These extensions make SuperPascal more accessible to C/C++, Python, and Rust programmers while maintaining full Pascal compatibility. Most extensions are **syntactic sugar** - they generate identical code to Pascal equivalents, except for heterogeneous lists which require runtime type information.

**See also:** [14_RustStyleModules.md](./14_RustStyleModules.md) for complete Rust-style module system and namespace specification.

---

**End of Language Extensions Specification**

