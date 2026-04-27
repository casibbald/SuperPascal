# Physics Library OO Pattern Audit

**Date:** 2025-01-XX  
**Status:** 🔍 Audit Complete - Refactoring Required

---

## Executive Summary

The current physics library uses a **purely procedural** approach with records for data and standalone functions for behavior. This does **not** follow proper OO patterns for inheritance and polymorphism. The library should be refactored to use a class hierarchy with virtual methods for polymorphic behavior.

---

## Current Architecture

### Data Structures (Records)
- ✅ `TParticle` - Record for particle data
- ✅ `TPhysicsObject` - Record for physics object data
- ✅ `TSurface` - Record for surface/material properties
- ✅ `TFragment` - Record for fragment data

### Behavior (Standalone Functions)
- ❌ All behavior implemented as standalone procedures/functions
- ❌ No class hierarchy
- ❌ No inheritance
- ❌ No polymorphism
- ❌ No virtual methods

### Current Pattern
```pascal
// Data
type
  TPhysicsObject = record
    X, Y: Fixed16;
    VX, VY: Fixed16;
    // ... properties ...
  end;

// Behavior (standalone functions)
procedure UpdatePhysics(var obj: TPhysicsObject);
procedure BounceOffSurface(var obj: TPhysicsObject; ...);
function CalculateFragmentCount(const obj: TPhysicsObject; ...): integer;
```

---

## Issues Identified

### 1. No Inheritance Hierarchy

**Problem:** All physics objects are the same `TPhysicsObject` record type. There's no way to:
- Create specialized physics object types (e.g., `TBall`, `TBlock`, `TBubble`)
- Inherit common properties
- Override behavior for specific types

**Impact:** Cannot extend the system with new physics object types without modifying existing code.

### 2. No Polymorphism

**Problem:** All behavior is implemented as standalone functions that check object properties at runtime:
```pascal
if obj.ShapeType = stCircle then
  FragmentCircle(...)
else
  FragmentPolygon(...);
```

**Impact:** 
- Runtime type checking instead of compile-time polymorphism
- Cannot have different behaviors for different object types
- Harder to extend with new shape types

### 3. No Virtual Methods

**Problem:** No way to override behavior in derived classes. All behavior is hardcoded in functions.

**Impact:** Cannot customize behavior for specific physics object types.

### 4. Mixed Concerns

**Problem:** Material properties (Bouncability, Solidness, Brittleness) are mixed with shape properties (ShapeType, CircleRadius, Polygon) in a single record.

**Impact:** 
- Violates Single Responsibility Principle
- Hard to extend with new material types
- Hard to extend with new shape types

---

## Recommended OO Design

### Class Hierarchy

```pascal
// Base class for all physics objects
type
  TPhysicsObjectBase = class
  protected
    FX, FY: Fixed16;           // Position
    FVX, FVY: Fixed16;         // Velocity
    FAX, FAY: Fixed16;         // Acceleration
    
    // Material properties
    FBouncability: Fixed16;
    FSolidness: Fixed16;
    FBrittleness: Fixed16;
    FFragmentability: Fixed16;
    
  public
    // Constructors
    constructor Create(x, y: Fixed16);
    
    // Property accessors
    property X: Fixed16 read FX write FX;
    property Y: Fixed16 read FY write FY;
    property VX: Fixed16 read FVX write FVX;
    property VY: Fixed16 read FVY write FVY;
    property Bouncability: Fixed16 read FBouncability write FBouncability;
    property Solidness: Fixed16 read FSolidness write FSolidness;
    property Brittleness: Fixed16 read FBrittleness write FBrittleness;
    property Fragmentability: Fixed16 read FFragmentability write FFragmentability;
    
    // Virtual methods (can be overridden)
    procedure Update(deltaTime: Fixed16); virtual;
    procedure ApplyGravity; virtual;
    procedure ApplyFriction; virtual;
    function CalculateFragmentCount(collisionVelocity: Fixed16): integer; virtual;
    procedure CreateFragments(fragmentCount: integer; var fragments: TFragmentArray); virtual; abstract;
    
    // Material property queries
    function IsSolid: boolean; virtual;
    function IsBouncy: boolean; virtual;
    function IsBrittle: boolean; virtual;
  end;

// Circle/Bubble physics object
type
  TCirclePhysicsObject = class(TPhysicsObjectBase)
  protected
    FRadius: Fixed16;
  public
    constructor Create(x, y, radius: Fixed16);
    property Radius: Fixed16 read FRadius write FRadius;
    
    // Override virtual methods
    procedure CreateFragments(fragmentCount: integer; var fragments: TFragmentArray); override;
    function CalculateArea: Fixed16; override;
  end;

// Polygon physics object
type
  TPolygonPhysicsObject = class(TPhysicsObjectBase)
  protected
    FPolygon: TPolygon2D;
    FPolygonArea: Fixed16;
  public
    constructor Create(x, y: Fixed16; const polygon: TPolygon2D);
    property Polygon: TPolygon2D read FPolygon write FPolygon;
    property PolygonArea: Fixed16 read FPolygonArea write FPolygonArea;
    
    // Override virtual methods
    procedure CreateFragments(fragmentCount: integer; var fragments: TFragmentArray); override;
    function CalculateArea: Fixed16; override;
  end;
```

### Material Property Classes

```pascal
// Base material class
type
  TMaterial = class
  protected
    FBouncability: Fixed16;
    FSolidness: Fixed16;
    FBrittleness: Fixed16;
    FFragmentability: Fixed16;
  public
    constructor Create(bouncability, solidness, brittleness, fragmentability: Fixed16);
    property Bouncability: Fixed16 read FBouncability;
    property Solidness: Fixed16 read FSolidness;
    property Brittleness: Fixed16 read FBrittleness;
    property Fragmentability: Fixed16 read FFragmentability;
  end;

// Specific material types
type
  TRockMaterial = class(TMaterial)
  public
    constructor Create;
  end;

  TGlassMaterial = class(TMaterial)
  public
    constructor Create;
  end;

  TBallMaterial = class(TMaterial)
  public
    constructor Create;
  end;
```

### Composition Pattern

```pascal
// Physics object with material composition
type
  TPhysicsObjectBase = class
  protected
    FMaterial: TMaterial;  // Composition instead of inheritance
  public
    constructor Create(material: TMaterial);
    property Material: TMaterial read FMaterial;
    
    // Delegate to material
    function IsSolid: boolean;
    function IsBouncy: boolean;
    function IsBrittle: boolean;
  end;
```

---

## Refactoring Plan

### Phase 1: Create Base Class Hierarchy
1. Create `TPhysicsObjectBase` abstract class
2. Move common properties to base class
3. Define virtual methods for polymorphic behavior

### Phase 2: Create Shape-Specific Classes
1. Create `TCirclePhysicsObject` class
2. Create `TPolygonPhysicsObject` class
3. Override virtual methods for shape-specific behavior

### Phase 3: Create Material Classes
1. Create `TMaterial` base class
2. Create specific material types (Rock, Glass, Ball, etc.)
3. Use composition to attach materials to physics objects

### Phase 4: Refactor Behavior Functions
1. Convert standalone functions to class methods
2. Use polymorphism instead of runtime type checking
3. Update all call sites

### Phase 5: Maintain Backward Compatibility
1. Keep record types for ECS compatibility
2. Provide conversion functions between records and classes
3. Update documentation

---

## Benefits of OO Refactoring

### 1. Extensibility
- Easy to add new physics object types by inheriting from base class
- Easy to add new material types
- Easy to add new shape types

### 2. Polymorphism
- Compile-time type safety
- No runtime type checking needed
- Virtual methods allow behavior customization

### 3. Code Organization
- Related properties and behavior grouped together
- Clear inheritance hierarchy
- Separation of concerns (shape vs. material)

### 4. Maintainability
- Changes to base class automatically propagate to derived classes
- Override only what needs to be different
- Clear contract via virtual methods

---

## Migration Strategy

### Option 1: Gradual Migration
- Keep existing procedural API
- Add new OO API alongside
- Migrate code gradually
- Deprecate procedural API over time

### Option 2: Complete Refactor
- Refactor all at once
- Break backward compatibility
- Update all call sites
- Cleaner final result

### Option 3: Hybrid Approach (Recommended)
- Keep records for ECS compatibility
- Add classes for OO usage
- Provide conversion utilities
- Support both paradigms

---

## Example: Before vs. After

### Before (Procedural)
```pascal
var
  obj: TPhysicsObject;
begin
  obj.ShapeType := stCircle;
  obj.CircleRadius := IntToFixed16(50);
  obj.Bouncability := 230;
  
  // Runtime type checking
  if obj.ShapeType = stCircle then
    FragmentCircle(obj, 5, fragments)
  else
    FragmentPolygon(obj, 5, fragments);
end.
```

### After (OO)
```pascal
var
  obj: TCirclePhysicsObject;
  material: TBallMaterial;
begin
  material := TBallMaterial.Create;
  obj := TCirclePhysicsObject.Create(100, 100, 50, material);
  
  // Polymorphic call - no runtime type checking needed
  obj.CreateFragments(5, fragments);
end.
```

---

## Recommendations

1. **Implement class hierarchy** for physics objects
2. **Use virtual methods** for polymorphic behavior
3. **Separate concerns** - shape classes vs. material classes
4. **Use composition** for materials (not inheritance)
5. **Maintain backward compatibility** with record types for ECS
6. **Provide conversion utilities** between records and classes

---

**Status:** 🔍 Audit Complete  
**Priority:** ⭐⭐⭐ HIGH  
**Effort:** Medium (2-3 days refactoring)

