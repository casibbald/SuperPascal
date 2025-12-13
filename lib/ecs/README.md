# ECS Library - Rust-Style Module Organization

**Location:** `SuperPascal/lib/ecs/`

---

## Module Structure

This library follows Rust-style module organization with `mod.pas` as the main entry point (equivalent to Rust's `mod.rs`).

### Module Files

```
lib/ecs/
├── mod.pas          # Main module entry point (re-exports all functionality)
├── types.pas        # Core types and constants
├── world.pas        # World management
├── entity.pas       # Entity management
├── component.pas    # Component management
├── query.pas        # Query system
├── system.pas       # System registration and execution
├── physics.pas      # Physics systems (from Mikro archive)
└── README.md        # This file
```

---

## Module Organization

### `mod.pas` - Main Entry Point

The main module file that re-exports all ECS functionality:

```pascal
uses ECS;  // Import everything through mod.pas
```

**Purpose:**
- Single import point for all ECS functionality
- Re-exports types, constants, and functions from sub-modules
- Provides clean public API

### `types.pas` - Core Types

**Exports:**
- `TEntity`, `TComponentMask`, `TComponentID`
- `TPosition`, `TVelocity`, `TSprite`, `TPhysicsBody`, `TParticle`
- `Fixed16` type
- All constants (ENTITY_NULL, MAX_ENTITIES, COMPONENT_*, etc.)
- Fixed-point helper functions

### `world.pas` - World Management

**Exports:**
- `TWorld` record structure
- `WorldInit`, `WorldCleanup`, `WorldUpdate` procedures

**Dependencies:**
- Uses `ECS_Types` for types
- Uses `ECS_System` for system initialization

### `entity.pas` - Entity Management

**Exports:**
- `EntityCreate`, `EntityDestroy`, `EntityIsValid`

**Dependencies:**
- Uses `ECS_Types` for types
- Uses `ECS_World` for world structure

### `component.pas` - Component Management

**Exports:**
- `ComponentAdd`, `ComponentRemove`, `ComponentHas`
- `ComponentGet*` and `ComponentSet*` accessors for all component types

**Dependencies:**
- Uses `ECS_Types` for types
- Uses `ECS_World` for world structure
- Uses `ECS_Entity` for entity validation

### `query.pas` - Query System

**Exports:**
- `TQuery` record
- `QueryCreate`, `QueryNext`, `QueryReset`

**Dependencies:**
- Uses `ECS_Types` for types
- Uses `ECS_World` for world structure

### `system.pas` - System Management

**Exports:**
- `TSystem` type
- `SystemRegister`, `SystemRunAll`, `SystemGetCount`
- `WorldInitSystems` (internal)

**Dependencies:**
- Uses `ECS_Types` for types
- Uses `ECS_World` for world structure

### `physics.pas` - Physics Systems

**Exports:**
- `MovementSystem`, `GravitySystem`, `VelocitySystem`, `FrictionSystem`, `ParticleSystem`

**Dependencies:**
- Uses `ECS_Types` for types
- Uses `ECS_World` for world structure
- Uses `ECS_Query` for query system
- Uses `ECS_Component` for component access

---

## Usage

### Simple Usage (Import Everything)

```pascal
program GameDemo;
uses ECS;  // Imports mod.pas, which re-exports everything

var
  world: TWorld;
  player: TEntity;
begin
  WorldInit(world);
  player := EntityCreate(world);
  // ... use ECS ...
end.
```

### Advanced Usage (Import Specific Modules)

```pascal
program GameDemo;
uses
  ECS_Types,    // Only types
  ECS_World,    // World management
  ECS_Entity,   // Entity management
  ECS_Component; // Component management

var
  world: TWorld;
  player: TEntity;
begin
  WorldInit(world);
  player := EntityCreate(world);
  // ... use specific modules ...
end.
```

---

## Module Dependencies

```
mod.pas
  ├── ECS_Types
  ├── ECS_World
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

## Benefits of Modular Organization

1. **Separation of Concerns:** Each module has a single responsibility
2. **Reusability:** Import only what you need
3. **Maintainability:** Easy to find and modify specific functionality
4. **Testability:** Test modules independently
5. **Rust-Style:** Familiar to developers coming from Rust

---

## Adding New Modules

To add a new module (e.g., `ECS_Collision`):

1. **Create module file:** `lib/ecs/collision.pas`
2. **Define unit:** `unit ECS_Collision;`
3. **Add dependencies:** `uses ECS_Types, ECS_World, ...;`
4. **Export functions:** Add to interface section
5. **Re-export in mod.pas:** Add to `mod.pas` interface section

---

## Physics Systems

### Basic Physics Systems

- **MovementSystem:** Updates position based on velocity
- **GravitySystem:** Applies gravity to entities with `TPhysicsBody` component
- **VelocitySystem:** Updates position based on velocity (alias for MovementSystem)
- **FrictionSystem:** Applies friction to entities with `TPhysicsBody` component
- **ParticleSystem:** Updates particle positions and lifetimes

### Advanced Physics Systems

- **BounceSystem:** Handles bounce physics for entities with `TMaterialProperties` and `TShape` components
  - Bounces objects off screen boundaries (walls, floor, ceiling)
  - Uses material properties (bouncability, solidness) to calculate bounce response
  - Supports both circle and polygon shapes
  
- **ExplosionSystem:** Handles explosions for entities with `TSprite.Explodable = True`
  - Applies explosion force to nearby entities within `ExplosionRadius`
  - Uses inverse square law for force calculation
  - Can trigger particle effects (TODO: particle creation)
  
- **CollisionSystem:** Detects collisions between entities using `TShape` components
  - Circle-Circle collision detection
  - Polygon collision detection (simplified, uses circle approximation)
  - Triggers bounce physics when collisions are detected
  - Supports fragmentation for brittle objects (TODO: fragment entity creation)

### Usage Example

```pascal
uses ECS;

var
  world: TWorld;
begin
  WorldInit(world);
  
  // Register physics systems
  SystemRegister(world, GravitySystem);
  SystemRegister(world, FrictionSystem);
  SystemRegister(world, MovementSystem);
  SystemRegister(world, BounceSystem);      // NEW: Bounce physics
  SystemRegister(world, ExplosionSystem);   // NEW: Explosion handling
  SystemRegister(world, CollisionSystem);  // NEW: Collision detection
  
  // Game loop
  while True do
  begin
    WorldUpdate(world);  // Runs all registered systems
    // Render sprites...
  end;
  
  WorldCleanup(world);
end.
```

## Sprite Physics Integration

Sprites can now be **bouncy** and **explodable** through component composition:

- **Bouncy Sprites:** Add `TMaterialProperties` component with `Bouncability > 128`
- **Explodable Sprites:** Set `Sprite.Explodable = True` with `ExplosionRadius` and `ExplosionDamage`
- **Physics-Enabled:** Combine `TSprite` + `TPosition` + `TVelocity` + `TMaterialProperties` + `TShape`

**See:** [Sprite Physics Integration Guide](../docs/lib/ecs/SPRITE_PHYSICS_INTEGRATION.md)

---

## Synchronization with Game Engine

The ECS system is designed to work seamlessly with the game engine and physics library. See:
- **[SYNC_STRATEGY.md](../docs/lib/ecs/SYNC_STRATEGY.md)** - Synchronization strategy and guidelines
- **[COMPONENT_REGISTRY.md](../docs/lib/ecs/COMPONENT_REGISTRY.md)** - Component registry and relationships

### Key Principles
- **Single Source of Truth:** ECS defines component structures
- **Direct Compatibility:** No conversion layers needed
- **Type Consistency:** Same types across all systems
- **Naming Consistency:** Same field names across all systems

---

**Last Updated:** 2025-01-XX  
**Status:** Complete modular ECS library

