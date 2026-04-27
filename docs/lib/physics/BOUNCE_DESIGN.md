# Bounce Physics Design

**Status:** 🎨 Design Document  
**Library:** Physics Library

---

## Overview

Bounce physics system that handles collisions between objects with different material properties:
- **Bouncability**: How bouncy an object is (0 = no bounce, 1 = perfect bounce)
- **Solidness**: How solid/rigid an object is (0 = soft, 1 = solid)
- **Absorbability**: How much energy a surface absorbs (0 = no absorption, 1 = full absorption)

---

## Material Properties

### Object Properties

```pascal
type
  TPhysicsObject = record
    X, Y: Fixed16;           // Position
    VX, VY: Fixed16;         // Velocity
    AX, AY: Fixed16;         // Acceleration
    Bouncability: Fixed16;   // 0-256 (0.0-1.0) - how bouncy
    Solidness: Fixed16;      // 0-256 (0.0-1.0) - how solid/rigid
    Brittleness: Fixed16;    // 0-256 (0.0-1.0) - how brittle (prone to breaking)
    Fragmentability: Fixed16; // 0-256 (0.0-1.0) - how many fragments when breaking
  end;
```

### Surface Properties

```pascal
type
  TSurface = record
    Solidness: Fixed16;      // 0-256 (0.0-1.0) - how solid (wall/floor/ceiling)
    Absorbability: Fixed16;  // 0-256 (0.0-1.0) - how much energy it absorbs
  end;
```

---

## Collision Scenarios

### 1. Two Solid Objects → "Smash"

**Behavior:** Both objects lose velocity, no bounce (inelastic collision)

**Example:**
- Two rocks hitting each other
- Two solid blocks colliding

**Calculation:**
- Both objects lose most/all velocity
- Effective bounce = 0 (or very low)

### 1b. Two Brittle Objects → "Fragment"

**Behavior:** Both objects break apart into fragments

**Example:**
- Two glass objects colliding
- Two brittle objects shattering

**Calculation:**
- Both objects fragment into multiple pieces
- Fragment count = based on Fragmentability and collision velocity
- Fragments inherit velocity from parent object

### 2. Bouncy Object + Solid Surface → Bounce Off

**Behavior:** Bouncy object bounces, solid surface doesn't move

**Example:**
- Ball hitting wall/floor/ceiling
- Bouncy ball on concrete

**Calculation:**
- Effective bounce = `object.Bouncability × (1 - surface.Absorbability)`
- Object bounces, surface unaffected

### 2b. Bouncy Object + Brittle Object → Bounce Away

**Behavior:** Both objects bounce away from each other (brittle doesn't break)

**Example:**
- Bouncy ball hitting brittle object
- Bouncy object colliding with glass

**Calculation:**
- Both objects bounce (brittle object doesn't fragment)
- Effective bounce = `min(obj1.Bouncability, obj2.Bouncability)`
- Both bounce away from each other

### 3. Two Bouncy Objects → Both Bounce Away

**Behavior:** Both objects bounce away from each other

**Example:**
- Two balls colliding
- Two bouncy objects hitting each other

**Calculation:**
- Effective bounce = `min(obj1.Bouncability, obj2.Bouncability)`
- Both objects bounce with opposite velocities

### 4. Bouncy Object + Absorbable Surface → Energy Loss

**Behavior:** Object bounces but loses energy due to absorption

**Example:**
- Ball hitting sponge
- Bouncy object on soft material

**Calculation:**
- Effective bounce = `object.Bouncability × (1 - surface.Absorbability)`
- Reduced bounce due to energy absorption

---

## Bounce Calculation

### Object-to-Surface Collision

```pascal
effectiveBounce = object.Bouncability × (1 - surface.Absorbability)
newVelocity = oldVelocity × effectiveBounce × (-1)  // Reflect along normal
```

### Object-to-Object Collision

**Case 1: Both Solid (Solidness > threshold, Brittleness < threshold)**
```pascal
// "Smash" - both lose velocity
obj1.VX := obj1.VX * 0.1;  // Lose 90% velocity
obj1.VY := obj1.VY * 0.1;
obj2.VX := obj2.VX * 0.1;
obj2.VY := obj2.VY * 0.1;
```

**Case 1b: Both Brittle (Brittleness > threshold)**
```pascal
// "Fragment" - both break apart
fragments1 := CalculateFragmentCount(obj1, collisionVelocity);
fragments2 := CalculateFragmentCount(obj2, collisionVelocity);
// Create fragments with inherited velocity
CreateFragments(obj1, fragments1);
CreateFragments(obj2, fragments2);
```

**Case 2: Both Bouncy (Bouncability > threshold)**
```pascal
effectiveBounce = min(obj1.Bouncability, obj2.Bouncability)
// Both bounce away from each other
```

**Case 2b: Bouncy + Brittle**
```pascal
// Both bounce away (brittle doesn't break)
effectiveBounce = min(obj1.Bouncability, obj2.Bouncability)
// Both bounce away from each other
```

**Case 3: Mixed (one solid, one bouncy)**
```pascal
// Bouncy one bounces, solid one loses velocity
if obj1.Bouncability > obj2.Bouncability then
  // obj1 bounces, obj2 loses velocity
else
  // obj2 bounces, obj1 loses velocity
```

## Fragment Calculation

### Fragment Count

The number of fragments created depends on:
- **Fragmentability** of the object (0-1)
- **Collision velocity** (higher velocity = more fragments)
- **Brittleness** (higher brittleness = more fragments)

```pascal
function CalculateFragmentCount(
  const obj: TPhysicsObject;
  collisionVelocity: Fixed16
): integer;
var
  baseFragments: integer;
  velocityFactor: Fixed16;
  fragmentFactor: Fixed16;
begin
  // Base fragments from Fragmentability (2-16 fragments)
  // Fragmentability 0.0 = 2 fragments, 1.0 = 16 fragments
  baseFragments := 2 + Fixed16ToInt(Fixed16Mul(obj.Fragmentability, IntToFixed16(14)));
  
  // Velocity factor (higher velocity = more fragments)
  // Normalize velocity to 0-1 range, then scale
  velocityFactor := Fixed16Div(collisionVelocity, IntToFixed16(100));  // Assuming max velocity ~100
  if velocityFactor > FIXED16_ONE then
    velocityFactor := FIXED16_ONE;
  
  // Brittleness factor (higher brittleness = more fragments)
  fragmentFactor := Fixed16Mul(obj.Brittleness, velocityFactor);
  
  // Final fragment count
  Result := baseFragments + Fixed16ToInt(Fixed16Mul(fragmentFactor, IntToFixed16(8)));
  
  // Clamp to reasonable range (2-32 fragments)
  if Result < 2 then Result := 2;
  if Result > 32 then Result := 32;
end;
```

### Fragment Creation

```pascal
type
  TFragment = record
    X, Y: Fixed16;      // Position
    VX, VY: Fixed16;    // Velocity
    Size: Fixed16;      // Fragment size
    Angle: integer;     // Fragment angle
  end;

  TFragmentArray = array of TFragment;

procedure CreateFragments(
  const obj: TPhysicsObject;
  fragmentCount: integer;
  var fragments: TFragmentArray
);
var
  i: integer;
  angle: integer;
  speed: Fixed16;
  fragmentSize: Fixed16;
begin
  SetLength(fragments, fragmentCount);
  
  // Calculate fragment size (smaller fragments for more pieces)
  fragmentSize := Fixed16Div(IntToFixed16(100), IntToFixed16(fragmentCount));
  
  for i := 0 to fragmentCount - 1 do
  begin
    // Random angle (0-359 degrees)
    angle := Random(360) - 180;
    
    // Fragment velocity based on parent velocity + random spread
    speed := Fixed16Mul(
      Fixed16Abs(obj.VX) + Fixed16Abs(obj.VY),
      IntToFixed16(Random(50) + 50)  // 50-100% of parent velocity
    ) shr 8;  // Divide by 256 for Q8.8
    
    fragments[i].X := obj.X;
    fragments[i].Y := obj.Y;
    fragments[i].VX := Fixed16Mul(speed, CosSigned(angle));
    fragments[i].VY := Fixed16Mul(speed, SinSigned(angle));
    fragments[i].Size := fragmentSize;
    fragments[i].Angle := angle;
  end;
end;
```

---

## Implementation

### Functions

```pascal
// Bounce object off surface
procedure BounceOffSurface(
  var obj: TPhysicsObject;
  const surface: TSurface;
  normalX, normalY: Fixed16  // Collision normal
);

// Bounce two objects off each other
procedure BounceObjects(
  var obj1, obj2: TPhysicsObject;
  normalX, normalY: Fixed16;  // Collision normal (from obj1 to obj2)
  var fragments1, fragments2: TFragmentArray  // Output fragments if brittle
);

// Calculate effective bounce coefficient
function CalculateEffectiveBounce(
  const obj: TPhysicsObject;
  const surface: TSurface
): Fixed16;

// Calculate effective bounce for two objects
function CalculateEffectiveBounceObjects(
  const obj1, obj2: TPhysicsObject
): Fixed16;

// Calculate fragment count based on Fragmentability and collision velocity
function CalculateFragmentCount(
  const obj: TPhysicsObject;
  collisionVelocity: Fixed16
): integer;

// Create fragments from object
procedure CreateFragments(
  const obj: TPhysicsObject;
  fragmentCount: integer;
  var fragments: TFragmentArray
);

// Check if object is brittle
function IsBrittle(const obj: TPhysicsObject): boolean;
```

---

## Constants

```pascal
// Thresholds
const
  SOLIDNESS_THRESHOLD: Fixed16 = 200;  // 0.78 - above this = "solid"
  BOUNCABILITY_THRESHOLD: Fixed16 = 128;  // 0.5 - above this = "bouncy"
  BRITTLENESS_THRESHOLD: Fixed16 = 128;  // 0.5 - above this = "brittle"
  
// Fragment limits
const
  MIN_FRAGMENTS: integer = 2;   // Minimum fragments
  MAX_FRAGMENTS: integer = 32;  // Maximum fragments
  BASE_FRAGMENTS: integer = 2;  // Base fragment count
  MAX_FRAGMENT_VELOCITY: Fixed16 = 25600;  // 100.0 in Q8.8 - max velocity for fragment calculation
  
// Default materials
const
  MATERIAL_SOLID: TSurface = (Solidness: 256; Absorbability: 0);      // Wall/floor
  MATERIAL_SPONGE: TSurface = (Solidness: 64; Absorbability: 200);    // Sponge
  MATERIAL_BOUNCY: TSurface = (Solidness: 128; Absorbability: 0);      // Bouncy surface
```

---

## Usage Examples

### Ball Bouncing Off Wall

```pascal
var
  ball: TPhysicsObject;
  wall: TSurface;
begin
  // Bouncy ball
  ball.Bouncability := 230;  // 0.9
  ball.Solidness := 50;     // 0.2 (not solid)
  
  // Solid wall
  wall.Solidness := 256;     // 1.0 (fully solid)
  wall.Absorbability := 0;   // No absorption
  
  // Bounce off wall (normal pointing away from wall)
  BounceOffSurface(ball, wall, FIXED16_ONE, 0);  // Normal: (1, 0) = right
end.
```

### Two Balls Colliding

```pascal
var
  ball1, ball2: TPhysicsObject;
begin
  // Both bouncy
  ball1.Bouncability := 230;  // 0.9
  ball2.Bouncability := 200;  // 0.78
  
  // Collision normal (from ball1 to ball2)
  BounceObjects(ball1, ball2, normalX, normalY);
end.
```

### Ball Hitting Sponge

```pascal
var
  ball: TPhysicsObject;
  sponge: TSurface;
begin
  // Bouncy ball
  ball.Bouncability := 230;  // 0.9
  
  // Absorbable sponge
  sponge.Solidness := 64;     // 0.25 (soft)
  sponge.Absorbability := 200; // 0.78 (high absorption)
  
  // Bounce with energy loss
  BounceOffSurface(ball, sponge, 0, FIXED16_ONE);  // Normal: (0, 1) = down
end.
```

### Two Solid Objects "Smashing"

```pascal
var
  rock1, rock2: TPhysicsObject;
  fragments1, fragments2: TFragmentArray;
begin
  // Both solid
  rock1.Solidness := 256;  // 1.0
  rock2.Solidness := 256;  // 1.0
  rock1.Bouncability := 0;  // No bounce
  rock2.Bouncability := 0;  // No bounce
  rock1.Brittleness := 0;  // Not brittle
  rock2.Brittleness := 0;  // Not brittle
  
  // "Smash" - both lose velocity
  BounceObjects(rock1, rock2, normalX, normalY, fragments1, fragments2);
end.
```

### Two Brittle Objects Fragmenting

```pascal
var
  glass1, glass2: TPhysicsObject;
  fragments1, fragments2: TFragmentArray;
  collisionVelocity: Fixed16;
begin
  // Both brittle
  glass1.Brittleness := 230;  // 0.9 (very brittle)
  glass2.Brittleness := 200;  // 0.78 (brittle)
  glass1.Fragmentability := 180;  // 0.7 (moderate fragments)
  glass2.Fragmentability := 150;  // 0.59 (fewer fragments)
  
  // Calculate collision velocity
  collisionVelocity := Fixed16Abs(glass1.VX - glass2.VX) + Fixed16Abs(glass1.VY - glass2.VY);
  
  // Fragment - both break apart
  BounceObjects(glass1, glass2, normalX, normalY, fragments1, fragments2);
  
  // fragments1 and fragments2 now contain the broken pieces
end.
```

### Bouncy + Brittle Bouncing

```pascal
var
  ball: TPhysicsObject;
  glass: TPhysicsObject;
  fragments1, fragments2: TFragmentArray;
begin
  // Bouncy ball
  ball.Bouncability := 230;  // 0.9
  ball.Brittleness := 0;     // Not brittle
  
  // Brittle glass
  glass.Brittleness := 230;   // 0.9
  glass.Bouncability := 50;  // 0.2 (low bounce)
  
  // Both bounce away (glass doesn't break)
  BounceObjects(ball, glass, normalX, normalY, fragments1, fragments2);
end.
```

---

**Last Updated:** 2025-01-XX  
**Status:** 🎨 Design Document - Ready for implementation

