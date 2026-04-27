# Fragmentation System Design

**Status:** 🎨 Design Document  
**Library:** Physics Library

---

## Overview

Enhanced fragmentation system that produces polygon objects from fragmented physics objects. The system ensures area/surface conservation - fragments sum to the original object's area.

**Key Concepts:**
- **Fragmentability**: Determines how many polygon fragments are produced
- **Area Conservation**: Total area of fragments = original object area
- **Shape Types**: Circles (bubbles) and Polygons
- **Fragment Calculation**: Complex algorithm based on Fragmentability, velocity, and Brittleness

---

## Object Shape Types

### Shape Definition

```pascal
type
  TShapeType = (stCircle, stPolygon);
  
  TPhysicsObject = record
    // ... position, velocity, acceleration, material properties ...
    ShapeType: TShapeType;  // Circle or Polygon
    
    // Circle properties (if ShapeType = stCircle)
    CircleRadius: Fixed16;  // Radius of circle
    
    // Polygon properties (if ShapeType = stPolygon)
    Polygon: TPolygon2D;    // Polygon vertices
    PolygonArea: Fixed16;   // Cached polygon area
  end;
```

---

## Fragment Types

### Enhanced Fragment Structure

```pascal
type
  TFragment = record
    X, Y: Fixed16;      // Position (center for circle, centroid for polygon)
    VX, VY: Fixed16;    // Velocity
    Angle: integer;     // Fragment angle (degrees)
    
    // Shape data
    ShapeType: TShapeType;
    
    // Circle fragment (if ShapeType = stCircle)
    CircleRadius: Fixed16;  // Radius of circular fragment
    CircleArea: Fixed16;    // Area of circle (π * r²)
    
    // Polygon fragment (if ShapeType = stPolygon)
    Polygon: TPolygon2D;    // Polygon vertices
    PolygonArea: Fixed16;    // Area of polygon
  end;

  TFragmentArray = array of TFragment;
```

---

## Fragmentation Algorithms

### 1. Circular Fragmentation (Bubbles)

**Principle:** Fragment a circle into smaller circles whose total area equals the original.

**Algorithm:**
1. Calculate fragment count based on Fragmentability
2. Calculate total area of original circle: `Area = π * r²`
3. Distribute area among fragments (weighted by Fragmentability)
4. Calculate radius for each fragment: `r_fragment = sqrt(Area_fragment / π)`
5. Position fragments around original center
6. Assign velocities based on fragment position

**Example:**
```pascal
Original circle: radius = 50, area = π * 50² = 7854
Fragment count: 5
Fragment areas: [1571, 1571, 1571, 1571, 1571] (equal distribution)
Fragment radii: [22.4, 22.4, 22.4, 22.4, 22.4]
Total area: 7854 ✓ (conserved)
```

### 2. Polygon Fragmentation

**Principle:** Fragment a polygon into smaller polygons whose total area equals the original.

**Algorithm:**
1. Calculate fragment count based on Fragmentability
2. Calculate total area of original polygon (using shoelace formula)
3. Use Voronoi-like partitioning or recursive subdivision
4. Create fragments as sub-polygons
5. Ensure area conservation: `Σ(Area_fragments) = Area_original`
6. Assign velocities based on fragment centroid

**Subdivision Methods:**

**Method A: Voronoi Partitioning**
- Generate random points inside polygon (one per fragment)
- Create Voronoi diagram
- Clip Voronoi cells to polygon boundary
- Each cell becomes a fragment

**Method B: Recursive Subdivision**
- Divide polygon along longest edge or random edge
- Recursively subdivide until fragment count reached
- Ensure area conservation at each step

**Method C: Grid-Based (Simpler)**
- Overlay grid on polygon bounding box
- Assign grid cells to fragments
- Merge cells into fragments
- Clip to polygon boundary

---

## Fragment Count Calculation

### Enhanced Calculation

The fragment count is now more sophisticated, considering:
- **Fragmentability**: Base fragment count (2-16)
- **Collision Velocity**: Higher velocity = more fragments
- **Brittleness**: Higher brittleness = more fragments
- **Shape Complexity**: Polygons may fragment differently than circles

```pascal
function CalculateFragmentCount(
  const obj: TPhysicsObject;
  collisionVelocity: Fixed16
): integer;
var
  baseFragments: integer;
  velocityFactor: Fixed16;
  fragmentFactor: Fixed16;
  shapeFactor: integer;
begin
  // Base fragments from Fragmentability (2-16 fragments)
  baseFragments := 2 + Fixed16ToInt(Fixed16Mul(obj.Fragmentability, IntToFixed16(14)));
  
  // Velocity factor (higher velocity = more fragments)
  velocityFactor := NormalizeVelocity(collisionVelocity);
  
  // Brittleness factor (higher brittleness = more fragments)
  fragmentFactor := Fixed16Mul(obj.Brittleness, velocityFactor);
  
  // Shape complexity factor
  // Circles fragment more uniformly, polygons may fragment more
  if obj.ShapeType = stCircle then
    shapeFactor := 0  // No adjustment for circles
  else
    shapeFactor := Fixed16ToInt(Fixed16Mul(fragmentFactor, IntToFixed16(2)));  // Polygons fragment more
  
  // Final fragment count
  Result := baseFragments + Fixed16ToInt(Fixed16Mul(fragmentFactor, IntToFixed16(8))) + shapeFactor;
  
  // Clamp to reasonable range
  if Result < PHYSICS_MIN_FRAGMENTS then Result := PHYSICS_MIN_FRAGMENTS;
  if Result > PHYSICS_MAX_FRAGMENTS then Result := PHYSICS_MAX_FRAGMENTS;
end;
```

---

## Area Conservation

### Circle Area Conservation

```pascal
function FragmentCircle(
  const obj: TPhysicsObject;
  fragmentCount: integer;
  var fragments: TFragmentArray
): boolean;
var
  originalArea: Fixed16;
  fragmentArea: Fixed16;
  fragmentRadius: Fixed16;
  angleStep: Fixed16;
  i: integer;
  angle: integer;
begin
  // Calculate original area: π * r²
  originalArea := Fixed16Mul(
    FIXED16_PI,
    Fixed16Mul(obj.CircleRadius, obj.CircleRadius)
  );
  
  // Area per fragment (equal distribution)
  fragmentArea := Fixed16Div(originalArea, IntToFixed16(fragmentCount));
  
  // Calculate fragment radius: r = sqrt(Area / π)
  fragmentRadius := Fixed16Sqrt(Fixed16Div(fragmentArea, FIXED16_PI));
  
  // Angle step for positioning
  angleStep := Fixed16Div(FIXED16_TWO_PI, IntToFixed16(fragmentCount));
  
  SetLength(fragments, fragmentCount);
  
  for i := 0 to fragmentCount - 1 do
  begin
    // Position fragments in circle around original center
    angle := Fixed16ToInt(Fixed16Mul(IntToFixed16(i), angleStep));
    
    fragments[i].X := obj.X + Fixed16Mul(
      Fixed16Add(obj.CircleRadius, fragmentRadius),
      CosSigned(angle)
    );
    fragments[i].Y := obj.Y + Fixed16Mul(
      Fixed16Add(obj.CircleRadius, fragmentRadius),
      SinSigned(angle)
    );
    
    fragments[i].ShapeType := stCircle;
    fragments[i].CircleRadius := fragmentRadius;
    fragments[i].CircleArea := fragmentArea;
    
    // Velocity based on position
    fragments[i].VX := obj.VX + Fixed16Mul(IntToFixed16(10), CosSigned(angle));
    fragments[i].VY := obj.VY + Fixed16Mul(IntToFixed16(10), SinSigned(angle));
  end;
  
  Result := True;
end;
```

### Polygon Area Conservation

```pascal
function FragmentPolygon(
  const obj: TPhysicsObject;
  fragmentCount: integer;
  var fragments: TFragmentArray
): boolean;
var
  originalArea: Fixed16;
  fragmentArea: Fixed16;
  i: integer;
  subPolygon: TPolygon2D;
begin
  // Use cached area or calculate
  if obj.PolygonArea > 0 then
    originalArea := obj.PolygonArea
  else
    originalArea := CalculatePolygonArea(obj.Polygon);
  
  // Area per fragment (equal distribution)
  fragmentArea := Fixed16Div(originalArea, IntToFixed16(fragmentCount));
  
  SetLength(fragments, fragmentCount);
  
  // Use recursive subdivision or Voronoi partitioning
  // For simplicity, use grid-based method
  FragmentPolygonGridBased(obj.Polygon, fragmentCount, fragmentArea, fragments);
  
  Result := True;
end;
```

---

## Implementation Functions

```pascal
// Fragment object into polygon fragments
procedure CreateFragments(
  const obj: TPhysicsObject;
  fragmentCount: integer;
  var fragments: TFragmentArray
);

// Fragment circle into smaller circles
function FragmentCircle(
  const obj: TPhysicsObject;
  fragmentCount: integer;
  var fragments: TFragmentArray
): boolean;

// Fragment polygon into smaller polygons
function FragmentPolygon(
  const obj: TPhysicsObject;
  fragmentCount: integer;
  var fragments: TFragmentArray
): boolean;

// Calculate polygon area (shoelace formula)
function CalculatePolygonArea(const polygon: TPolygon2D): Fixed16;

// Verify area conservation
function VerifyAreaConservation(
  const original: TPhysicsObject;
  const fragments: TFragmentArray
): boolean;
```

---

## Usage Examples

### Fragmenting a Bubble (Circle)

```pascal
var
  bubble: TPhysicsObject;
  fragments: TFragmentArray;
begin
  // Create bubble
  bubble.ShapeType := stCircle;
  bubble.CircleRadius := IntToFixed16(50);
  bubble.Fragmentability := 180;  // 0.7
  bubble.Brittleness := 200;     // 0.78
  
  // Fragment
  CreateFragments(bubble, 5, fragments);
  
  // fragments now contains 5 circular fragments
  // Total area of fragments = original bubble area
end.
```

### Fragmenting a Polygon

```pascal
var
  polygon: TPhysicsObject;
  fragments: TFragmentArray;
begin
  // Create polygon
  polygon.ShapeType := stPolygon;
  polygon.Polygon := CreatePolygon(...);
  polygon.PolygonArea := CalculatePolygonArea(polygon.Polygon);
  polygon.Fragmentability := 200;  // 0.78
  polygon.Brittleness := 230;      // 0.9
  
  // Fragment
  CreateFragments(polygon, 8, fragments);
  
  // fragments now contains 8 polygon fragments
  // Total area of fragments = original polygon area
end.
```

---

## Area Conservation Verification

```pascal
function VerifyAreaConservation(
  const original: TPhysicsObject;
  const fragments: TFragmentArray
): boolean;
var
  originalArea: Fixed16;
  totalFragmentArea: Fixed16;
  i: integer;
  tolerance: Fixed16;
begin
  // Calculate original area
  if original.ShapeType = stCircle then
    originalArea := Fixed16Mul(
      FIXED16_PI,
      Fixed16Mul(original.CircleRadius, original.CircleRadius)
    )
  else
    originalArea := original.PolygonArea;
  
  // Sum fragment areas
  totalFragmentArea := 0;
  for i := 0 to Length(fragments) - 1 do
  begin
    if fragments[i].ShapeType = stCircle then
      totalFragmentArea := Fixed16Add(totalFragmentArea, fragments[i].CircleArea)
    else
      totalFragmentArea := Fixed16Add(totalFragmentArea, fragments[i].PolygonArea);
  end;
  
  // Check within tolerance (1% error allowed)
  tolerance := Fixed16Div(originalArea, IntToFixed16(100));
  Result := Fixed16Abs(Fixed16Sub(totalFragmentArea, originalArea)) <= tolerance;
end;
```

---

**Last Updated:** 2025-01-XX  
**Status:** 🎨 Design Document - Ready for implementation

