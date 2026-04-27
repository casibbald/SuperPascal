# SuperPascal Library Organization

**Location:** `SuperPascal/lib/`

---

## Overview

All SuperPascal libraries are organized in the `lib/` directory using **Rust-style module organization** with `mod.pas` files as module entry points (equivalent to Rust's `mod.rs`).

---

## Rust-Style Module Pattern

### Concept

In Rust, modules are organized with `mod.rs` files that act as the module entry point. SuperPascal uses the equivalent pattern with `mod.pas` files.

**Rust Pattern:**
```
lib/ecs/
├── mod.rs          # Module entry point
├── types.rs        # Sub-module
└── world.rs        # Sub-module
```

**SuperPascal Pattern:**
```
lib/ecs/
├── mod.pas         # Module entry point (unit ECS)
├── types.pas       # Sub-module (unit ECS_Types)
└── world.pas       # Sub-module (unit ECS_World)
```

### Key Principles

1. **`mod.pas` is the main entry point** - Users import the library via `uses LibraryName;`
2. **Sub-modules are separate units** - Each sub-module is a separate `.pas` file
3. **`mod.pas` re-exports functionality** - Types and constants are re-exported for convenience
4. **Functions available through imported units** - Functions are called directly from their units

---

## Library Structure

### Directory Layout

```
lib/
├── ecs/                    # ECS Library
│   ├── mod.pas            # Main module (unit ECS)
│   ├── types.pas           # Types and constants (unit ECS_Types)
│   ├── world.pas           # World management (unit ECS_World)
│   ├── entity.pas          # Entity management (unit ECS_Entity)
│   ├── component.pas       # Component management (unit ECS_Component)
│   ├── query.pas            # Query system (unit ECS_Query)
│   ├── system.pas           # System management (unit ECS_System)
│   ├── physics.pas          # Physics systems (unit ECS_Physics)
│   └── README.md            # Library documentation
└── README.md                # Library organization guide
```

### Module Naming Convention

**Unit Names:**
- **Main module:** `LibraryName` (e.g., `ECS`)
- **Sub-modules:** `LibraryName_SubModule` (e.g., `ECS_Types`, `ECS_World`)

**File Names:**
- **Main module:** `mod.pas`
- **Sub-modules:** `submodule.pas` (e.g., `types.pas`, `world.pas`)

---

## ECS Library Example

### Module Structure

```
lib/ecs/
├── mod.pas          → unit ECS; (re-exports everything)
├── types.pas        → unit ECS_Types; (core types)
├── world.pas        → unit ECS_World; (world management)
├── entity.pas       → unit ECS_Entity; (entity management)
├── component.pas     → unit ECS_Component; (component management)
├── query.pas        → unit ECS_Query; (query system)
├── system.pas       → unit ECS_System; (system management)
└── physics.pas      → unit ECS_Physics; (physics systems)
```

### Usage

**Simple Usage (Import Everything):**
```pascal
program GameDemo;
uses ECS;  // Imports mod.pas, which re-exports types/constants

var
  world: TWorld;  // Type available directly
  player: TEntity;
begin
  WorldInit(world);  // Function available through ECS_World
  player := EntityCreate(world);  // Function available through ECS_Entity
  // ...
end.
```

**Advanced Usage (Import Specific Modules):**
```pascal
program GameDemo;
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

## Creating a New Library

### Step 1: Create Library Directory

```bash
mkdir -p lib/new_library
```

### Step 2: Create Sub-Modules

Create sub-modules as needed:
- `lib/new_library/types.pas` - Types and constants
- `lib/new_library/core.pas` - Core functionality
- etc.

### Step 3: Create mod.pas

Create `lib/new_library/mod.pas`:

```pascal
unit NewLibrary;

interface

// Import sub-modules
uses
  NewLibrary_Types,
  NewLibrary_Core;

// Re-export types for convenience
type
  TMyType = NewLibrary_Types.TMyType;

// Re-export constants
const
  MY_CONSTANT = NewLibrary_Types.MY_CONSTANT;

// Functions are available through imported units:
// - NewLibrary_Core: MyFunction, MyProcedure
// Users call: MyFunction(...)

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

## Module Dependencies

### ECS Library Dependencies

```
mod.pas (ECS)
  ├── ECS_Types (no dependencies)
  ├── ECS_World
  │     ├── ECS_Types
  │     └── ECS_System
  ├── ECS_Entity
  │     ├── ECS_Types
  │     └── ECS_World
  ├── ECS_Component
  │     ├── ECS_Types
  │     ├── ECS_World
  │     └── ECS_Entity
  ├── ECS_Query
  │     ├── ECS_Types
  │     └── ECS_World
  ├── ECS_System
  │     ├── ECS_Types
  │     └── ECS_World
  └── ECS_Physics
        ├── ECS_Types
        ├── ECS_World
        ├── ECS_Query
        └── ECS_Component
```

---

## Benefits

1. **Rust-Style:** Familiar to developers coming from Rust
2. **Modular:** Easy to import only what you need
3. **Maintainable:** Clear separation of concerns
4. **Scalable:** Easy to add new modules
5. **Testable:** Test modules independently
6. **Reusable:** Libraries can be used across projects

---

## Migration from Old Structure

### Old Structure (languageSpecification/ecs/)

```
languageSpecification/ecs/
├── ECS_Unit_Final.pas    # Monolithic unit
└── ECS_Unit_Iteration*.pas
```

### New Structure (lib/ecs/)

```
lib/ecs/
├── mod.pas               # Main entry point
├── types.pas             # Types module
├── world.pas             # World module
├── entity.pas            # Entity module
├── component.pas         # Component module
├── query.pas             # Query module
├── system.pas            # System module
└── physics.pas           # Physics module
```

**Migration Complete:** ✅ ECS library moved to `lib/ecs/` with modular structure

---

## Library Standards

All libraries should:

1. ✅ **Follow mod.pas pattern** - Main entry point re-exports functionality
2. ✅ **Document dependencies** - Clear `uses` clauses
3. ✅ **Provide README** - Document structure and usage
4. ✅ **Use consistent naming** - `LibraryName_SubModule` for units
5. ✅ **Export clean API** - Clear public interface
6. ✅ **Separate concerns** - Each module has a single responsibility

---

## Current Libraries

### ECS Library

**Location:** `lib/ecs/`  
**Status:** ✅ Complete  
**Modules:** 7 modules (mod, types, world, entity, component, query, system, physics)  
**See:** [ECS Library README](./lib/ecs/README.md)

---

## Future Libraries

Libraries to be added:
- `lib/math/` - Mathematical algorithms (fixed-point, matrix, trigonometry)
- `lib/graphics/` - Graphics algorithms (polygon rendering, texture mapping)
- `lib/collision/` - Collision detection algorithms
- `lib/sorting/` - Sorting algorithms
- `lib/compression/` - Compression algorithms
- etc.

---

**Last Updated:** 2025-01-XX  
**Status:** ECS library migrated to lib/ with Rust-style module organization

