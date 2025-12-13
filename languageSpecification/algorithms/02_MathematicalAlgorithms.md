# Mathematical Algorithms

**Part of:** [Algorithms Appendix](../99_Algorithms_Appendix.md)

---

## Overview

Mathematical algorithms for graphics, games, and general computation. These algorithms are **generic** and work on **all platforms**.

**Source Material:** Mikro Documentation Archive

**See Also:**
- Book Chapter: [Chapter 19: Mathematics for Graphics and Games](../../book/19_MathematicsForGraphicsAndGames/README.md)
- Standard Library: [Math Unit](../13_StandardLibrary.md#math-functions)
- Fixed-Point Arithmetic: [Fixed-Point Arithmetic](./01_FixedPointArithmetic.md)

---

## Matrix Math

**Source:** `docs/mikro_docs_archive/Coding/1/OTMMATX.TXT`

### Matrix Representation

A 4x4 transformation matrix represents:
- **X, Y, Z columns:** World space coordinates of local axis vectors (unit vectors)
- **C column:** Translation/origin (always [0, 0, 0, 1])
- **O row:** World space coordinates of object's origin

**Identity Matrix:**
```pascal
type
  TMatrix4x4 = array[0..3, 0..3] of Fixed16;  // Using fixed-point

const
  IdentityMatrix: TMatrix4x4 = (
    (65536, 0,      0,      0),      // X axis: (1, 0, 0, 0)
    (0,      65536, 0,      0),      // Y axis: (0, 1, 0, 0)
    (0,      0,      65536, 0),      // Z axis: (0, 0, 1, 0)
    (0,      0,      0,      65536)  // Origin: (0, 0, 0, 1)
  );
```

### Matrix Multiplication

**Algorithm:** Concatenate two transformation matrices

```pascal
procedure MatrixMultiply(var a: TMatrix4x4; const b: TMatrix4x4);
var
  temp: TMatrix4x4;
  i, j: integer;
begin
  // Transform by columns first, then by rows
  for j := 0 to 3 do
    for i := 0 to 3 do
      temp[i, j] := Fixed16Mul(a[i, 0], b[0, j]) +
                    Fixed16Mul(a[i, 1], b[1, j]) +
                    Fixed16Mul(a[i, 2], b[2, j]) +
                    Fixed16Mul(a[i, 3], b[3, j]);
  
  // Copy result back to matrix a
  a := temp;
end;
```

**Note:** Matrix multiplication is **not commutative**: `[a] * [b] ≠ [b] * [a]`

### Transform Vector by Matrix

**Algorithm:** Project vector onto transformed axis vectors using dot product

```pascal
type
  TVector3 = record
    X, Y, Z: Fixed16;
  end;

function TransformVector(const v: TVector3; const m: TMatrix4x4): TVector3;
begin
  // Transform using dot product with matrix columns
  Result.X := Fixed16Mul(v.X, m[0, 0]) +
              Fixed16Mul(v.Y, m[1, 0]) +
              Fixed16Mul(v.Z, m[2, 0]) +
              m[3, 0];  // Translation
  
  Result.Y := Fixed16Mul(v.X, m[0, 1]) +
              Fixed16Mul(v.Y, m[1, 1]) +
              Fixed16Mul(v.Z, m[2, 1]) +
              m[3, 1];  // Translation
  
  Result.Z := Fixed16Mul(v.X, m[0, 2]) +
              Fixed16Mul(v.Y, m[1, 2]) +
              Fixed16Mul(v.Z, m[2, 2]) +
              m[3, 2];  // Translation
end;
```

### Rotation Matrices

**Rotate about X axis:**
```pascal
function MatrixRotateX(angle: Fixed16): TMatrix4x4;
var
  cosA, sinA: Fixed16;
begin
  cosA := CosFixed(angle);
  sinA := SinFixed(angle);
  
  Result := IdentityMatrix;
  Result[1, 1] := cosA;
  Result[1, 2] := sinA;
  Result[2, 1] := -sinA;
  Result[2, 2] := cosA;
end;
```

**Rotate about Y axis:**
```pascal
function MatrixRotateY(angle: Fixed16): TMatrix4x4;
var
  cosA, sinA: Fixed16;
begin
  cosA := CosFixed(angle);
  sinA := SinFixed(angle);
  
  Result := IdentityMatrix;
  Result[0, 0] := cosA;
  Result[0, 2] := -sinA;
  Result[2, 0] := sinA;
  Result[2, 2] := cosA;
end;
```

**Rotate about Z axis:**
```pascal
function MatrixRotateZ(angle: Fixed16): TMatrix4x4;
var
  cosA, sinA: Fixed16;
begin
  cosA := CosFixed(angle);
  sinA := SinFixed(angle);
  
  Result := IdentityMatrix;
  Result[0, 0] := cosA;
  Result[0, 1] := sinA;
  Result[1, 0] := -sinA;
  Result[1, 1] := cosA;
end;
```

### Translation Matrix

```pascal
function MatrixTranslate(x, y, z: Fixed16): TMatrix4x4;
begin
  Result := IdentityMatrix;
  Result[3, 0] := x;
  Result[3, 1] := y;
  Result[3, 2] := z;
end;
```

### Scale Matrix

```pascal
function MatrixScale(sx, sy, sz: Fixed16): TMatrix4x4;
begin
  Result := IdentityMatrix;
  Result[0, 0] := sx;
  Result[1, 1] := sy;
  Result[2, 2] := sz;
end;
```

### Camera Transformations

**Source:** `docs/mikro_docs_archive/Coding/1/OTMMATX.TXT`

Camera transformations convert object space to eye space (camera space). Each camera has its own transformation matrix.

**Building the Camera Matrix:**

```pascal
// Camera transformation formula:
// [C] = [C] * [CT] * [CX] * [CY] * [CZ]
// where CT = translation, CX/CY/CZ = rotation matrices

procedure BuildCameraMatrix(
  var cmatrix: TMatrix4x4;
  dx, dy, dz: Fixed16;  // Camera translation
  xangle, yangle, zangle: Fixed16  // Camera rotation angles
);
var
  tmatrix, xmatrix, ymatrix, zmatrix: TMatrix4x4;
begin
  // Build translation matrix (may need to negate components)
  tmatrix := MatrixTranslate(dx, dy, dz);
  
  // Build rotation matrices
  xmatrix := MatrixRotateX(xangle);
  ymatrix := MatrixRotateY(yangle);
  zmatrix := MatrixRotateZ(zangle);
  
  // Concatenate: [C] = [C] * [T] * [X] * [Y] * [Z]
  MatrixMultiply(cmatrix, tmatrix);
  MatrixMultiply(cmatrix, zmatrix);
  MatrixMultiply(cmatrix, xmatrix);
  MatrixMultiply(cmatrix, ymatrix);
end;
```

**Transforming Objects to Eye Space:**

```pascal
// Transform object to eye space:
// [E] = [O] * [C]
// where [E] = eyespace matrix, [O] = object space matrix, [C] = camera matrix

procedure TransformToEyeSpace(
  const omatrix: TMatrix4x4;
  const origin: TVector3;  // Object's world space origin
  const cmatrix: TMatrix4x4;
  var ematrix: TMatrix4x4
);
begin
  // Initialize eyespace matrix to identity
  ematrix := IdentityMatrix;
  
  // Copy object space matrix to eyespace
  MatrixMultiply(ematrix, omatrix);
  
  // Add object's world space origin
  ematrix[3, 0] := Fixed16Add(ematrix[3, 0], origin.X);
  ematrix[3, 1] := Fixed16Add(ematrix[3, 1], origin.Y);
  ematrix[3, 2] := Fixed16Add(ematrix[3, 2], origin.Z);
  
  // Transform to camera space
  MatrixMultiply(ematrix, cmatrix);
end;
```

**Note:** Camera translation/rotation components may need to be negated depending on your coordinate system. If the camera moves backward instead of forward, negate the appropriate components.

### Inverse Transformations

**Source:** `docs/mikro_docs_archive/Coding/1/OTMMATX.TXT`

Inverse transformations are used for hidden surface removal and shading in object space without transforming normal vectors. This provides significant speed improvements.

**Why Use Inverse Transformations:**
- Avoid transforming normal vectors (saves time)
- Hide points as well as polygons (can skip ~50% of points)
- Determine visibility/shading by inverse transforming view/light vectors

**Algorithm:** For transformation matrices (determinant = 1), the inverse can be computed by swapping rows and columns. This is much faster than computing the full inverse matrix.

**Inverse Transform Vector:**

```pascal
// Inverse transform a vector (swap row/column indices)
// Used for transforming view/light vectors to object space

function InverseTransformVector(
  const v: TVector3;
  const m: TMatrix4x4
): TVector3;
begin
  // Note: row and column indices are swapped from normal transformation
  // Also note: translation (w term) is omitted - vectors start at origin
  Result.X := Fixed16Mul(v.X, m[0, 0]) +
              Fixed16Mul(v.Y, m[0, 1]) +
              Fixed16Mul(v.Z, m[0, 2]);
  
  Result.Y := Fixed16Mul(v.X, m[1, 0]) +
              Fixed16Mul(v.Y, m[1, 1]) +
              Fixed16Mul(v.Z, m[1, 2]);
  
  Result.Z := Fixed16Mul(v.X, m[2, 0]) +
              Fixed16Mul(v.Y, m[2, 1]) +
              Fixed16Mul(v.Z, m[2, 2]);
end;
```

**Usage Example:**

```pascal
// Inverse transform view vector for backface culling
var
  viewVector: TVector3;
  inverseView: TVector3;
  normal: TVector3;
  dotProduct: Fixed16;
begin
  // View vector in world space (from object to camera)
  viewVector.X := cameraX - objectX;
  viewVector.Y := cameraY - objectY;
  viewVector.Z := cameraZ - objectZ;
  
  // Inverse transform to object space
  inverseView := InverseTransformVector(viewVector, omatrix);
  
  // Dot product with untransformed normal (in object space)
  dotProduct := Vector3Dot(inverseView, normal);
  
  // If dot product > 0, polygon is back-facing
  if dotProduct > 0 then
    // Skip this polygon (backface culling)
end;
```

### Hierarchical Transformations

**Source:** `docs/mikro_docs_archive/Coding/1/OTMMATX.TXT`

Hierarchical transformations allow objects to have parent-child relationships, where child objects move relative to their parents (e.g., arm → forearm → hand → fingers).

**Data Structure:**

```pascal
type
  PObject = ^TObject;
  TObject = record
    // Object data
    OMatrix: TMatrix4x4;  // Object space matrix
    EMatrix: TMatrix4x4;  // Eye space matrix
    Origin: TVector3;     // World space origin
    
    // Hierarchy
    Parent: PObject;
    Children: array of PObject;
    NumChildren: Integer;
  end;
```

**Transformation Formula:**

For a child object in a hierarchy:
```
[E] = [O] * [T] * [X] * [Y] * [Z] * parent.[E]
```

Where:
- `[O]` = child's object space matrix
- `[T]`, `[X]`, `[Y]`, `[Z]` = child's translation and rotation matrices
- `parent.[E]` = parent's eye space matrix (contains parent's object space + camera)

**Key Insight:** The parent's eye space matrix already contains the camera transformation, so we can use it directly instead of computing the full hierarchy chain.

**Recursive Transformation:**

```pascal
procedure TransformObject(
  obj: PObject;
  const cmatrix: TMatrix4x4  // Camera matrix (only for root objects)
);
var
  tmatrix, xmatrix, ymatrix, zmatrix: TMatrix4x4;
  i: Integer;
begin
  // Build transformation matrices for this object
  tmatrix := MatrixTranslate(obj.Translation.X, obj.Translation.Y, obj.Translation.Z);
  xmatrix := MatrixRotateX(obj.Rotation.X);
  ymatrix := MatrixRotateY(obj.Rotation.Y);
  zmatrix := MatrixRotateZ(obj.Rotation.Z);
  
  // Apply transformations to object space matrix
  MatrixMultiply(obj.OMatrix, tmatrix);
  MatrixMultiply(obj.OMatrix, zmatrix);
  MatrixMultiply(obj.OMatrix, xmatrix);
  MatrixMultiply(obj.OMatrix, ymatrix);
  
  // Initialize eye space matrix
  obj.EMatrix := IdentityMatrix;
  MatrixMultiply(obj.EMatrix, obj.OMatrix);
  
  // Add object origin
  obj.EMatrix[3, 0] := Fixed16Add(obj.EMatrix[3, 0], obj.Origin.X);
  obj.EMatrix[3, 1] := Fixed16Add(obj.EMatrix[3, 1], obj.Origin.Y);
  obj.EMatrix[3, 2] := Fixed16Add(obj.EMatrix[3, 2], obj.Origin.Z);
  
  // Apply camera or parent's eye space matrix
  if obj.Parent = nil then
    MatrixMultiply(obj.EMatrix, cmatrix)  // Root object: use camera
  else
    MatrixMultiply(obj.EMatrix, obj.Parent.EMatrix);  // Child: use parent's eye space
  
  // Recursively transform children
  for i := 0 to obj.NumChildren - 1 do
    TransformObject(obj.Children[i], cmatrix);  // Pass cmatrix (not used for children)
end;
```

**Rendering Hierarchy:**

```pascal
procedure RenderObject(obj: PObject);
var
  i: Integer;
begin
  // Render this object using obj.EMatrix
  // ... rendering code ...
  
  // Recursively render children
  for i := 0 to obj.NumChildren - 1 do
    RenderObject(obj.Children[i]);
end;
```

**Note:** Only root objects (objects with no parent) should be in the main object list. Children are rendered automatically when their parents are rendered.

### Matrix Precision and Normalization

**Source:** `docs/mikro_docs_archive/Coding/1/OTMMATX.TXT`

Matrix transformations can lose precision over time, causing axis vectors to:
1. Change magnitude (no longer unit vectors)
2. Lose perpendicularity (no longer orthogonal)

**Precision Considerations:**

- **16.16 Fixed-Point:** Loses precision after ~10² transformations
- **32-bit Float:** Shows no noticeable loss after ~10⁵ transformations
- **64-bit Double:** Very high precision, minimal loss

**Matrix Normalization (Dot Product Method):**

```pascal
// Correct perpendicularity using dot products
// Based on: dot product of perpendicular vectors = 0

procedure NormalizeMatrixDotProduct(var m: TMatrix4x4);
var
  x, y, z: TVector3;
  dotXY, dotXZ, dotYZ: Fixed16;
begin
  // Extract axis vectors
  x.X := m[0, 0]; x.Y := m[1, 0]; x.Z := m[2, 0];
  y.X := m[0, 1]; y.Y := m[1, 1]; y.Z := m[2, 1];
  z.X := m[0, 2]; z.Y := m[1, 2]; z.Z := m[2, 2];
  
  // Correct Y axis (make perpendicular to X)
  dotXY := Vector3Dot(x, y);
  y.X := Fixed16Sub(y.X, Fixed16Mul(dotXY, x.X));
  y.Y := Fixed16Sub(y.Y, Fixed16Mul(dotXY, x.Y));
  y.Z := Fixed16Sub(y.Z, Fixed16Mul(dotXY, x.Z));
  
  // Correct Z axis (make perpendicular to X and Y)
  dotXZ := Vector3Dot(x, z);
  dotYZ := Vector3Dot(y, z);
  z.X := Fixed16Sub(z.X, Fixed16Mul(dotXZ, x.X));
  z.Y := Fixed16Sub(z.Y, Fixed16Mul(dotXZ, x.Y));
  z.Z := Fixed16Sub(z.Z, Fixed16Mul(dotXZ, x.Z));
  z.X := Fixed16Sub(z.X, Fixed16Mul(dotYZ, y.X));
  z.Y := Fixed16Sub(z.Y, Fixed16Mul(dotYZ, y.Y));
  z.Z := Fixed16Sub(z.Z, Fixed16Mul(dotYZ, y.Z));
  
  // Normalize all vectors to unit length
  x := Vector3Normalize(x);
  y := Vector3Normalize(y);
  z := Vector3Normalize(z);
  
  // Store back in matrix
  m[0, 0] := x.X; m[1, 0] := x.Y; m[2, 0] := x.Z;
  m[0, 1] := y.X; m[1, 1] := y.Y; m[2, 1] := y.Z;
  m[0, 2] := z.X; m[1, 2] := z.Y; m[2, 2] := z.Z;
end;
```

**Matrix Normalization (Cross Product Method):**

```pascal
// Correct perpendicularity using cross products
// Based on: cross product yields perpendicular vector

procedure NormalizeMatrixCrossProduct(var m: TMatrix4x4);
var
  x, y, z: TVector3;
begin
  // Extract X and Y axes
  x.X := m[0, 0]; x.Y := m[1, 0]; x.Z := m[2, 0];
  y.X := m[0, 1]; y.Y := m[1, 1]; y.Z := m[2, 1];
  
  // Normalize X and Y
  x := Vector3Normalize(x);
  y := Vector3Normalize(y);
  
  // Generate Z from cross product: Z = X × Y
  z := Vector3Cross(x, y);
  z := Vector3Normalize(z);
  
  // Regenerate Y from cross product: Y = Z × X
  y := Vector3Cross(z, x);
  y := Vector3Normalize(y);
  
  // Store back in matrix
  m[0, 0] := x.X; m[1, 0] := x.Y; m[2, 0] := x.Z;
  m[0, 1] := y.X; m[1, 1] := y.Y; m[2, 1] := y.Z;
  m[0, 2] := z.X; m[1, 2] := z.Y; m[2, 2] := z.Z;
end;
```

**When to Normalize:**
- After many transformations (every 100-1000 transformations)
- Before critical operations (rendering, collision detection)
- In hierarchical systems (after each level of hierarchy)

---

## Trigonometry

**Source:** `docs/mikro_docs_archive/Coding/2/SIN.TXT`, `SINCOSC.TXT`

### Lookup Tables

**Pre-compute sin/cos tables for fast access:**

```pascal
var
  SinTable: array[0..255] of Fixed16;
  CosTable: array[0..255] of Fixed16;

procedure GenerateTrigTables;
var
  i: integer;
  angle: Real;
begin
  for i := 0 to 255 do
  begin
    angle := (i * 2.0 * PI) / 256.0;  // Convert to radians
    SinTable[i] := Trunc(Sin(angle) * 32767.0);
    CosTable[i] := Trunc(Cos(angle) * 32767.0);
  end;
end;

function SinFixed(angle: integer): Fixed16;  // angle: 0-255
begin
  Result := SinTable[angle and $FF];
end;

function CosFixed(angle: integer): Fixed16;
begin
  Result := CosTable[angle and $FF];
end;
```

**Space Optimization:** Use only sin table, compute cos as `sin(angle + 64)` (64 = 90° in 256° circle)

### Recursive Sin/Cos Generation

**Source:** `docs/mikro_docs_archive/Coding/2/SIN.TXT`

**Algorithm:** Generate sin/cos using recursive formula (no floating-point needed)

```pascal
procedure GenerateSinTableRecursive(var table: array[0..1023] of Fixed16);
var
  i: integer;
  cos2PiN: Fixed16;  // cos(2π/N) where N = 1024
begin
  // First two values: cos(0) = 1, cos(2π/1024)
  table[0] := 16777216;  // 2^24 (scaling factor)
  table[1] := 16776900;  // 2^24 * cos(2π/1024) (pre-computed)
  
  cos2PiN := table[1];
  
  // Recursive formula: cos(k) = 2*cos(2π/N)*cos(k-1) - cos(k-2)
  for i := 2 to 1023 do
  begin
    table[i] := Fixed16Mul(Fixed16Mul(2, cos2PiN), table[i - 1]) - table[i - 2];
    // Shift right to maintain precision
    table[i] := table[i] shr 23;
  end;
end;
```

**Formula:**
- `cos(k) = 2*cos(2π/N)*cos(k-1) - cos(k-2)`
- `sin(k) = 2*cos(2π/N)*sin(k-1) - sin(k-2)`

---

## Square Root

**Source:** `docs/mikro_docs_archive/Coding/1/SQROOT.TXT`

### Integer Square Root (Fast Approximation)

**Algorithm:** Binary search for highest bit, then lookup table

**Performance:** 16-27 cycles (much faster than FPU on systems without hardware sqrt)

```pascal
var
  SqrtTable: array[0..255] of byte;

procedure SetupSqrtTable;
var
  i: integer;
begin
  for i := 0 to 255 do
    SqrtTable[i] := Trunc(256.0 * Sqrt(i / 256.0));
end;

function IntegerSqrt(n: LongInt): integer;
var
  bitPos: integer;
  shifted: LongInt;
begin
  if n = 0 then
  begin
    Result := 0;
    exit;
  end;
  
  // Find highest bit position using binary search
  if n >= $10000000 then
    bitPos := 30  // Bit 30-31
  else if n >= $1000000 then
    bitPos := 26  // Bit 26-29
  else if n >= $100000 then
    bitPos := 20  // Bit 20-25
  else if n >= $10000 then
    bitPos := 16  // Bit 16-19
  else if n >= $1000 then
    bitPos := 12  // Bit 12-15
  else if n >= $100 then
    bitPos := 8   // Bit 8-11
  else if n >= $10 then
    bitPos := 4   // Bit 4-7
  else
    bitPos := 0;  // Bit 0-3
  
  // Shift to get value in 0..255 range
  shifted := n shr (bitPos - 8);
  
  // Lookup in table
  Result := SqrtTable[shifted and $FF];
  
  // Shift result back
  Result := Result shl ((bitPos div 2) - 4);
end;
```

**Accuracy:** Error < 0.75% for most numbers, improves for larger numbers

**Fixed-Point Square Root:**
```pascal
function FixedSqrt(x: Fixed16): Fixed16;
begin
  // Square root of fixed-point: sqrt(x * 2^16) = sqrt(x) * 2^8
  Result := IntegerSqrt(x) shl 8;
end;
```

---

## Vector Math

### Vector Types

```pascal
type
  TVector2 = record
    X, Y: Fixed16;
  end;
  
  TVector3 = record
    X, Y, Z: Fixed16;
  end;
```

### Vector Addition

```pascal
function Vector2Add(const a, b: TVector2): TVector2;
begin
  Result.X := a.X + b.X;
  Result.Y := a.Y + b.Y;
end;

function Vector3Add(const a, b: TVector3): TVector3;
begin
  Result.X := a.X + b.X;
  Result.Y := a.Y + b.Y;
  Result.Z := a.Z + b.Z;
end;
```

### Vector Subtraction

```pascal
function Vector2Sub(const a, b: TVector2): TVector2;
begin
  Result.X := a.X - b.X;
  Result.Y := a.Y - b.Y;
end;

function Vector3Sub(const a, b: TVector3): TVector3;
begin
  Result.X := a.X - b.X;
  Result.Y := a.Y - b.Y;
  Result.Z := a.Z - b.Z;
end;
```

### Scalar Multiplication

```pascal
function Vector2Scale(const v: TVector2; s: Fixed16): TVector2;
begin
  Result.X := Fixed16Mul(v.X, s);
  Result.Y := Fixed16Mul(v.Y, s);
end;

function Vector3Scale(const v: TVector3; s: Fixed16): TVector3;
begin
  Result.X := Fixed16Mul(v.X, s);
  Result.Y := Fixed16Mul(v.Y, s);
  Result.Z := Fixed16Mul(v.Z, s);
end;
```

### Dot Product

```pascal
function Vector2Dot(const a, b: TVector2): Fixed16;
begin
  Result := Fixed16Mul(a.X, b.X) + Fixed16Mul(a.Y, b.Y);
end;

function Vector3Dot(const a, b: TVector3): Fixed16;
begin
  Result := Fixed16Mul(a.X, b.X) +
            Fixed16Mul(a.Y, b.Y) +
            Fixed16Mul(a.Z, b.Z);
end;
```

### Cross Product (3D only)

```pascal
function Vector3Cross(const a, b: TVector3): TVector3;
begin
  Result.X := Fixed16Mul(a.Y, b.Z) - Fixed16Mul(a.Z, b.Y);
  Result.Y := Fixed16Mul(a.Z, b.X) - Fixed16Mul(a.X, b.Z);
  Result.Z := Fixed16Mul(a.X, b.Y) - Fixed16Mul(a.Y, b.X);
end;
```

### Vector Length (Magnitude)

```pascal
function Vector2Length(const v: TVector2): Fixed16;
begin
  Result := FixedSqrt(Fixed16Mul(v.X, v.X) + Fixed16Mul(v.Y, v.Y));
end;

function Vector3Length(const v: TVector3): Fixed16;
begin
  Result := FixedSqrt(Fixed16Mul(v.X, v.X) +
                      Fixed16Mul(v.Y, v.Y) +
                      Fixed16Mul(v.Z, v.Z));
end;
```

### Vector Normalization

```pascal
function Vector2Normalize(const v: TVector2): TVector2;
var
  len: Fixed16;
begin
  len := Vector2Length(v);
  if len > 0 then
  begin
    Result.X := Fixed16Div(v.X, len);
    Result.Y := Fixed16Div(v.Y, len);
  end
  else
  begin
    Result.X := 0;
    Result.Y := 0;
  end;
end;

function Vector3Normalize(const v: TVector3): TVector3;
var
  len: Fixed16;
begin
  len := Vector3Length(v);
  if len > 0 then
  begin
    Result.X := Fixed16Div(v.X, len);
    Result.Y := Fixed16Div(v.Y, len);
    Result.Z := Fixed16Div(v.Z, len);
  end
  else
  begin
    Result.X := 0;
    Result.Y := 0;
    Result.Z := 0;
  end;
end;
```

---

## Performance Notes

**Matrix Operations:**
- Matrix multiplication: O(64) fixed-point multiplies + adds
- Vector transformation: O(12) fixed-point multiplies + adds
- Consider platform-specific optimizations (e.g., m68k has fast 64-bit multiply)

**Trigonometry:**
- Lookup tables: O(1) access time
- Table generation: O(n) one-time cost
- Space: 256 entries × 2 bytes = 512 bytes (very small)

**Square Root:**
- Integer sqrt: 16-27 cycles (much faster than FPU)
- Fixed-point sqrt: Similar performance
- Accuracy: Good enough for most game applications

**Vector Operations:**
- Addition/Subtraction: O(1) - very fast
- Dot product: O(n) where n = dimensions
- Cross product: O(3) - 3D only
- Normalization: O(n) + square root

---

**Previous:** [Fixed-Point Arithmetic](./01_FixedPointArithmetic.md)  
**Next:** [Sorting Algorithms](./03_SortingAlgorithms.md)  
**Last Updated:** 2025-01-XX
