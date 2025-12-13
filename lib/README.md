# SuperPascal Standard Library

**Location:** `SuperPascal/lib/`

---

## Overview

This directory contains all reusable libraries for SuperPascal, organized using Rust-style module organization with `mod.pas` files as module entry points (equivalent to Rust's `mod.rs`).

---

## Library Organization

### Module Structure Pattern

Each library follows this structure:

```
lib/library_name/
├── mod.pas          # Main module entry point (re-exports all functionality)
├── submodule1.pas   # Sub-module 1
├── submodule2.pas   # Sub-module 2
└── README.md        # Library documentation
```

### Rust-Style Module Pattern

**Key Concepts:**
- `mod.pas` is the main entry point (like Rust's `mod.rs`)
- Sub-modules are separate `.pas` files
- `mod.pas` re-exports types, constants, and functions from sub-modules
- Users can import the whole library via `uses LibraryName;` (imports `mod.pas`)
- Users can import specific modules via `uses LibraryName_SubModule;`

---

## Available Libraries

### ECS Library ✅

**Location:** `lib/ecs/`  
**Status:** ✅ Complete  
**Modules:** 8 modules (mod, types, world, entity, component, query, system, physics)

**Main Module:** `mod.pas`

**Sub-Modules:**
- `types.pas` - Core types and constants
- `world.pas` - World management
- `entity.pas` - Entity management
- `component.pas` - Component management
- `query.pas` - Query system
- `system.pas` - System registration
- `physics.pas` - Physics systems

**Usage:**
```pascal
uses ECS;  // Import everything through mod.pas
```

**See:** [ECS Library README](./ecs/README.md)

---

### Math Library ✅

**Location:** `lib/math/`  
**Status:** ✅ Complete  
**Modules:** 7 modules (mod, types, fixed, matrix, trig, sqrt, vector)

**Main Module:** `mod.pas`

**Sub-Modules:**
- `types.pas` - Core math types (vectors, matrices)
- `fixed.pas` - Fixed-point arithmetic operations
- `matrix.pas` - Matrix math (multiplication, transformations)
- `trig.pas` - Trigonometric functions (lookup tables)
- `sqrt.pas` - Square root algorithms
- `vector.pas` - Vector math (2D/3D operations)

**Usage:**
```pascal
uses Math; // Import everything through mod.pas
```

**See:** [Math Library README](./math/README.md)

---

### Graphics Library ✅

**Location:** `lib/graphics/`  
**Status:** ✅ Complete  
**Modules:** 7 modules (mod, types, line, polygon, circle, texture, tilemap)

**Main Module:** `mod.pas`

**Sub-Modules:**
- `types.pas` - Core graphics types
- `line.pas` - Line drawing (Bresenham)
- `polygon.pas` - Polygon rendering (scan conversion)
- `circle.pas` - Circle drawing (midpoint algorithm)
- `ellipse.pas` - Ellipse drawing (midpoint algorithm)
- `texture.pas` - Texture mapping (affine, tiled)
- `tilemap.pas` - Tilemap rendering (camera-based)

**Usage:**
```pascal
uses Graphics;  // Import everything through mod.pas
```

**See:** [Graphics Library README](./graphics/README.md)

---

### Collision Library ✅

**Location:** `lib/collision/`  
**Status:** ✅ Complete  
**Modules:** 4 modules (mod, types, aabb, circle, polygon)

**Main Module:** `mod.pas`

**Sub-Modules:**
- `types.pas` - Core collision types (AABB, Circle, Polygon)
- `aabb.pas` - AABB collision detection (2D & 3D)
- `circle.pas` - Circle collision detection
- `polygon.pas` - Polygon collision & clipping

**Usage:**
```pascal
uses Collision;  // Import everything through mod.pas
```

**See:** [Collision Library README](./collision/README.md)

---

### Planned Libraries

**See:** [Standard Library Organization](./STDLIB_ORGANIZATION.md) and [Standard Library Roadmap](./STDLIB_ROADMAP.md) for complete plans.

**Completed Libraries:**
- `ecs/` - Entity-Component-System library ✅ (8 modules)
- `math/` - Mathematical algorithms ✅ (7 modules)
- `graphics/` - Graphics algorithms ✅ (8 modules)
- `collision/` - Collision detection ✅ (4 modules)
- `physics/` - Physics simulation ✅ (13 modules)
- `testing/` - Unit testing framework ✅ (6 modules) ⭐ CRITICAL
- `sorting/` - Sorting algorithms ✅ (7 modules)
- `game/` - Game development algorithms ✅ (5 modules)
- `compression/` - Data compression ✅ (4 modules)
- `crypto/` - Cryptography (CRC checksums) ✅ (3 modules)

---

## Adding New Libraries

### Step 1: Create Library Directory

```bash
mkdir -p lib/new_library
```

### Step 2: Create Module Files

Create sub-modules as needed:
- `lib/new_library/types.pas` - Types and constants
- `lib/new_library/core.pas` - Core functionality
- etc.

### Step 3: Create mod.pas

Create `lib/new_library/mod.pas`:

```pascal
unit NewLibrary;

interface

uses
  NewLibrary_Types,
  NewLibrary_Core;

// Re-export types
type
  TMyType = NewLibrary_Types.TMyType;

// Re-export constants
const
  MY_CONSTANT = NewLibrary_Types.MY_CONSTANT;

// Functions are available through imported units
// Users call NewLibrary_Core.MyFunction

implementation

end.
```

### Step 4: Create README

Create `lib/new_library/README.md` documenting:
- Purpose of the library
- Module structure
- Usage examples
- Dependencies

---

## Module Naming Convention

### Unit Names

- **Main module:** `LibraryName` (e.g., `ECS`)
- **Sub-modules:** `LibraryName_SubModule` (e.g., `ECS_Types`, `ECS_World`)

### File Names

- **Main module:** `mod.pas`
- **Sub-modules:** `submodule.pas` (e.g., `types.pas`, `world.pas`)

### Example

```
lib/ecs/
├── mod.pas          → unit ECS;
├── types.pas        → unit ECS_Types;
├── world.pas        → unit ECS_World;
└── entity.pas       → unit ECS_Entity;
```

---

## Benefits of This Organization

1. **Rust-Style:** Familiar to developers coming from Rust
2. **Modular:** Easy to import only what you need
3. **Maintainable:** Clear separation of concerns
4. **Scalable:** Easy to add new modules
5. **Testable:** Test modules independently

---

## Usage Patterns

### Pattern 1: Import Everything

```pascal
uses ECS;  // Imports mod.pas, which re-exports everything

var
  world: TWorld;  // Type available directly
begin
  WorldInit(world);  // Function available through ECS_World
end.
```

### Pattern 2: Import Specific Modules

```pascal
uses
  ECS_Types,    // Only types
  ECS_World,    // World management
  ECS_Entity;   // Entity management

var
  world: ECS_World.TWorld;
begin
  ECS_World.WorldInit(world);
  // ...
end.
```

---

## Library Standards

All libraries should:

1. **Follow mod.pas pattern** - Main entry point re-exports functionality
2. **Document dependencies** - Clear `uses` clauses
3. **Provide README** - Document structure and usage
4. **Use consistent naming** - `LibraryName_SubModule` for units
5. **Export clean API** - Clear public interface

---

**Last Updated:** 2025-01-XX  
**Status:** All planned libraries complete! ECS, Math, Graphics, Collision, Physics, Testing, Sorting, Game, Compression, and Crypto libraries (65 modules total)

