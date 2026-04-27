# SuperPascal Standard Library Organization

**Base Directory:** `SuperPascal/lib/`

---

## Overview

The `lib/` directory is the foundation for the entire SuperPascal Standard Library. All reusable libraries are organized here using Rust-style module organization with `mod.pas` files as entry points.

---

## Standard Library Structure

### Core Libraries

```
lib/
├── README.md                    # Library organization guide
├── STDLIB_ORGANIZATION.md      # This file
│
├── ecs/                         # Entity-Component-System library
│   ├── mod.pas
│   ├── types.pas
│   ├── world.pas
│   ├── entity.pas
│   ├── component.pas
│   ├── query.pas
│   ├── system.pas
│   ├── physics.pas
│   └── README.md
│
├── math/                        # Mathematical algorithms
│   ├── mod.pas                  # Main math module
│   ├── fixed.pas                # Fixed-point arithmetic
│   ├── matrix.pas               # Matrix operations
│   ├── vector.pas               # Vector math
│   ├── trig.pas                 # Trigonometry
│   ├── sqrt.pas                 # Square root
│   └── README.md
│
├── graphics/                    # Graphics algorithms
│   ├── mod.pas                  # Main graphics module
│   ├── line.pas                 # Line drawing (Bresenham)
│   ├── polygon.pas              # Polygon rendering
│   ├── texture.pas             # Texture mapping
│   ├── tilemap.pas              # Tilemap rendering
│   ├── circle.pas               # Circle drawing
│   └── README.md
│
├── collision/                   # Collision detection
│   ├── mod.pas                  # Main collision module
│   ├── types.pas                # Core collision types
│   ├── aabb.pas                 # AABB collision
│   ├── circle.pas               # Circle collision
│   ├── polygon.pas              # Polygon clipping
│   └── README.md
│
├── testing/                     # Unit testing framework ⭐ CRITICAL
│   ├── mod.pas                  # Main testing module
│   ├── types.pas                # Core testing types
│   ├── assertions.pas           # Assert functions
│   ├── runner.pas               # Test runner
│   ├── fixtures.pas             # Setup/teardown
│   ├── reporting.pas            # Test result reporting
│   ├── TESTING_FRAMEWORK_DESIGN.md  # Design document
│   └── README.md
│
├── physics/                     # Physics simulation
│   ├── mod.pas                  # Main physics module
│   ├── particle.pas             # Particle systems
│   ├── gravity.pas              # Gravity simulation
│   ├── friction.pas             # Friction simulation
│   └── README.md
│
├── sorting/                     # Sorting algorithms
│   ├── mod.pas                  # Main sorting module
│   ├── quicksort.pas            # Quicksort
│   ├── shellsort.pas            # Shellsort
│   ├── mergesort.pas            # Mergesort
│   ├── heapsort.pas             # Heapsort
│   └── README.md
│
├── compression/                 # Data compression
│   ├── mod.pas                  # Main compression module
│   └── README.md
│
├── crypto/                      # Cryptography
│   ├── mod.pas                  # Main crypto module
│   ├── crc.pas                  # CRC checksums
│   └── README.md
│
└── game/                        # Game development utilities
    ├── mod.pas                  # Main game module
    ├── camera.pas               # Camera systems
    ├── bsp.pas                  # BSP trees
    ├── los.pas                  # Line of sight
    ├── pathfinding.pas          # Pathfinding algorithms
    └── README.md
```

---

## Library Organization Principles

### 1. Rust-Style Module Pattern

Each library follows the Rust `mod.rs` pattern:

- **`mod.pas`** - Main entry point (re-exports all functionality)
- **Sub-modules** - Separate `.pas` files for specific functionality
- **Naming:** `LibraryName_SubModule` for unit names

### 2. Separation of Concerns

Each library has a single, clear purpose:
- `math/` - Mathematical operations
- `graphics/` - Graphics algorithms
- `collision/` - Collision detection
- `physics/` - Physics simulation
- etc.

### 3. Platform-Agnostic Core

Core libraries work on **all platforms**:
- Generic algorithms
- Platform-agnostic APIs
- Platform-specific optimizations in separate modules

### 4. ECS Integration

All game-related algorithms integrate with the ECS library:
- Physics systems use ECS
- Graphics systems use ECS
- Collision systems use ECS

---

## Library Development Roadmap

### Phase 1: Core Libraries (Current)

**Status:** ✅ Complete
- `ecs/` - Entity-Component-System library

### Phase 2: Mathematical Libraries

**Priority:** High  
**Source:** Mikro archive algorithms

**Libraries to Create:**
1. `math/fixed.pas` - Fixed-point arithmetic (from `FIXEDPOI.TXT`)
2. `math/matrix.pas` - Matrix operations (from `OTMMATX.TXT`)
3. `math/trig.pas` - Trigonometry (from `SIN.TXT`, `SINCOSC.TXT`)
4. `math/sqrt.pas` - Square root (from `SQROOT.TXT`, `SQRT.TXT`)
5. `math/vector.pas` - Vector math

**Integration:**
- All algorithms from `algorithms/01_FixedPointArithmetic.md`
- All algorithms from `algorithms/02_MathematicalAlgorithms.md`

### Phase 3: Graphics Libraries

**Priority:** High  
**Source:** Mikro archive algorithms

**Libraries to Create:**
1. `graphics/line.pas` - Line drawing (from `BRESENHA.TXT`)
2. `graphics/polygon.pas` - Polygon rendering (from `OTMPOLY.TXT`)
3. `graphics/texture.pas` - Texture mapping (from `TEXTURE.TXT`)
4. `graphics/tilemap.pas` - Tilemap rendering (from `TILEMAP.TXT`)
5. `graphics/circle.pas` - Circle drawing

**Integration:**
- All algorithms from `algorithms/04_GraphicsAlgorithms.md`
- All algorithms from `algorithms/07_AdvancedGraphics.md`

### Phase 4: Collision & Physics Libraries

**Priority:** High  
**Source:** Mikro archive algorithms

**Libraries to Create:**
1. `collision/aabb.pas` - AABB collision (from `CLIP.TXT`)
2. `collision/polygon.pas` - Polygon clipping (from `POLYCLIP.TXT`, `PLYGCLIP.TXT`)
3. `physics/particle.pas` - Particle systems (from `PARTICLE.TXT`)
4. `physics/gravity.pas` - Gravity simulation (from `GRAVITY.TXT`)

**Integration:**
- All algorithms from `algorithms/05_CollisionDetection.md`
- All algorithms from `algorithms/06_PhysicsSimulation.md`

### Phase 4.5: Testing Framework

**Priority:** ⭐⭐⭐⭐⭐ CRITICAL  
**Source:** Modern testing principles

**Rationale:** Unit testing is essential for modern software development. While not common in retro computing era, we can design a comprehensive testing framework for SuperPascal.

**Library to Create:**
1. `testing/` - Unit testing framework
   - `types.pas` - Core testing types (TestCase, TestSuite, TestResult)
   - `assertions.pas` - Assert functions (True, False, Equal, etc.)
   - `runner.pas` - Test runner and execution
   - `fixtures.pas` - Setup/teardown support
   - `reporting.pas` - Test result reporting

**See:** `lib/testing/TESTING_FRAMEWORK_DESIGN.md` for complete design

### Phase 5: Utility Libraries

**Priority:** Medium  
**Source:** Mikro archive algorithms

**Libraries to Create:**
1. `sorting/` - Sorting algorithms (from `SORT_ALG.TXT`)
2. `compression/` - Data compression (from `COMPRFAQ.TXT`)
3. `crypto/crc.pas` - CRC checksums (from `CRC.TXT`)

**Integration:**
- All algorithms from `algorithms/03_SortingAlgorithms.md`
- All algorithms from `algorithms/09_UtilityAlgorithms.md`

### Phase 6: Game Development Libraries

**Priority:** Medium  
**Source:** Mikro archive algorithms

**Libraries to Create:**
1. `game/camera.pas` - Camera systems (from `CAM_MATR.TXT`, `3D_CAM.TXT`)
2. `game/bsp.pas` - BSP trees (from `BSPTREE.TXT`)
3. `game/los.pas` - Line of sight (from `LOS.TXT`)
4. `game/pathfinding.pas` - Pathfinding algorithms

**Integration:**
- All algorithms from `algorithms/08_GameAlgorithms.md`

---

## Module Template

### Standard Library Module Structure

Each library follows this pattern:

```pascal
unit LibraryName_SubModule;

interface

uses
  LibraryName_Types,  // If types module exists
  ECS;                // If ECS integration needed

// Types
type
  TMyType = record
    // ...
  end;

// Functions
procedure MyFunction(...);
function MyFunction2(...): ReturnType;

implementation

// Implementation

end.
```

### Main Module (mod.pas) Template

```pascal
unit LibraryName;

interface

// Import sub-modules
uses
  LibraryName_Types,
  LibraryName_SubModule1,
  LibraryName_SubModule2;

// Re-export types
type
  TMyType = LibraryName_Types.TMyType;

// Re-export constants
const
  MY_CONSTANT = LibraryName_Types.MY_CONSTANT;

// Functions available through imported units
// Users call: MyFunction(...)

implementation

end.
```

---

## Integration with Algorithms Appendix

### Mapping Algorithms to Libraries

| Algorithm File | Library | Module |
|----------------|---------|--------|
| `01_FixedPointArithmetic.md` | `math/` | `fixed.pas` |
| `02_MathematicalAlgorithms.md` | `math/` | `matrix.pas`, `trig.pas`, `sqrt.pas`, `vector.pas` |
| `03_SortingAlgorithms.md` | `sorting/` | `quicksort.pas`, `shellsort.pas`, etc. |
| `04_GraphicsAlgorithms.md` | `graphics/` | `line.pas`, `polygon.pas`, `texture.pas`, `tilemap.pas` |
| `05_CollisionDetection.md` | `collision/` | `aabb.pas`, `polygon.pas` |
| `06_PhysicsSimulation.md` | `physics/` | `particle.pas`, `gravity.pas` |
| `07_AdvancedGraphics.md` | `graphics/` | Advanced modules |
| `08_GameAlgorithms.md` | `game/` | `camera.pas`, `bsp.pas`, `los.pas` |
| `09_UtilityAlgorithms.md` | `compression/`, `crypto/` | Various modules |

---

## ECS Integration Strategy

### All Game Libraries Use ECS

**Principle:** All game-related algorithms are implemented as ECS systems.

**Example:**
```pascal
// lib/physics/gravity.pas
unit Physics_Gravity;

interface

uses
  ECS;

// Gravity system (ECS-integrated)
procedure GravitySystem(var world: TWorld);

implementation

uses
  ECS_Query,
  ECS_Component;

procedure GravitySystem(var world: TWorld);
var
  query: TQuery;
  entity: TEntity;
  vel: ^TVelocity;
  physics: ^TPhysicsBody;
begin
  query := QueryCreate(world, [COMPONENT_VELOCITY, COMPONENT_PHYSICS]);
  while QueryNext(query, entity) do
  begin
    vel := ComponentGetVelocity(world, entity);
    physics := ComponentGetPhysics(world, entity);
    if (vel <> nil) and (physics <> nil) then
      vel^.DY := vel^.DY + physics^.Gravity;
  end;
end;

end.
```

---

## Standard Library Usage

### Importing Standard Library

**Simple Usage:**
```pascal
program GameDemo;
uses
  ECS,        // ECS library
  Math,       // Math library (mod.pas)
  Graphics,   // Graphics library (mod.pas)
  Physics;    // Physics library (mod.pas)

var
  world: TWorld;
begin
  // Use ECS
  WorldInit(world);
  
  // Use math
  var result: Fixed16;
  result := Fixed16Mul(IntToFixed16(100), IntToFixed16(200));
  
  // Use graphics
  DrawLineBresenham(0, 0, 100, 100);
  
  // Use physics
  GravitySystem(world);
end.
```

**Advanced Usage (Import Specific Modules):**
```pascal
program GameDemo;
uses
  ECS_Types,
  ECS_World,
  Math_Fixed,      // Only fixed-point math
  Graphics_Line;   // Only line drawing

// Use specific modules
```

---

## Library Development Guidelines

### 1. Module Naming

- **Main module:** `mod.pas` → `unit LibraryName;`
- **Sub-modules:** `submodule.pas` → `unit LibraryName_SubModule;`
- **File names:** Lowercase with underscores (e.g., `fixed_point.pas`)

### 2. Dependencies

- **Core libraries** (math, graphics) should have minimal dependencies
- **Game libraries** (physics, collision) should use ECS
- **Utility libraries** (sorting, compression) should be standalone

### 3. Platform Considerations

- **Generic first:** Core algorithms work on all platforms
- **Platform-specific:** Optimizations in separate modules
- **Fixed-point:** Use for no-FPU platforms
- **Floating-point:** Optional for FPU platforms

### 4. Documentation

Each library should have:
- `README.md` - Library documentation
- `mod.pas` - Well-commented main module
- `USAGE_EXAMPLE.pas` - Usage examples

---

## Migration Plan

### From Algorithms Appendix to Libraries

**Current State:**
- Algorithms documented in `languageSpecification/algorithms/*.md`
- ECS library in `lib/ecs/`

**Target State:**
- All algorithms implemented as libraries in `lib/`
- Algorithms appendix references library modules
- Book chapters use library examples

**Migration Steps:**
1. ✅ ECS library migrated to `lib/ecs/`
2. [ ] Math library created from algorithms
3. [ ] Graphics library created from algorithms
4. [ ] Collision library created from algorithms
5. [ ] Physics library created from algorithms
6. [ ] Sorting library created from algorithms
7. [ ] Game library created from algorithms
8. [ ] Utility libraries created from algorithms

---

## Success Metrics

### Completeness
- ✅ ECS library: 100% complete
- ⚠️ Math library: 0% (algorithms documented, library pending)
- ⚠️ Graphics library: 0% (algorithms documented, library pending)
- ⚠️ Collision library: 0% (algorithms documented, library pending)
- ⚠️ Physics library: 0% (algorithms documented, library pending)
- ⚠️ Sorting library: 0% (algorithms documented, library pending)
- ⚠️ Game library: 0% (algorithms documented, library pending)
- ⚠️ Utility libraries: 0% (algorithms documented, libraries pending)

### Target
- **100% of algorithms** implemented as libraries
- **100% of libraries** follow mod.pas pattern
- **100% of libraries** have documentation
- **100% of libraries** integrate with ECS (where applicable)

---

## Next Steps

1. **Create Math Library** - Start with fixed-point, matrix, trig
2. **Create Graphics Library** - Start with line, polygon, texture
3. **Create Collision Library** - Start with AABB, polygon clipping
4. **Create Physics Library** - Start with particle, gravity
5. **Create Sorting Library** - Implement all sorting algorithms
6. **Create Game Library** - Start with camera, BSP, LOS
7. **Create Utility Libraries** - Compression, crypto, etc.

**Priority Order:**
1. Math (foundation for everything)
2. Graphics (essential for games)
3. Collision (ECS integration) ✅ Complete
4. Testing (critical for quality) ⭐ CRITICAL
5. Physics (ECS integration)
6. Sorting (utility)
7. Game (advanced)
8. Utilities (specialized)

---

**Last Updated:** 2025-01-XX  
**Status:** ECS library complete, stdlib foundation established

