# Standard Library Modules

- Status: partially-verified
- Source docs: `docs/09_ZealPascalStandardLibrarySpecification.md`, `lib/` README files
- Code anchors: `lib/`

## Module overview

| Module | Pascal files | Purpose |
|--------|-------------|---------|
| `collision/` | 5 | AABB, circle, polygon collision detection |
| `compression/` | 4 | RLE, LZ77 compression algorithms |
| `crypto/` | 3 | CRC checksums |
| `ecs/` | 9 | Entity Component System: entity, component, world, query, system, physics, types |
| `game/` | 5 | Camera, LOS, pathfinding, BSP, types |
| `graphics/` | 7 | Circle, ellipse, line, polygon, tilemap, texture, types |
| `math/` | 6 | Matrix, vector, trig, fixed-point, sqrt, types |
| `physics/` | 9 | Gravity, friction, bounce, particle, material, object_base, object_circle, object_polygon, ECS integration |
| `sorting/` | 5 | Heapsort, shellsort, mergesort, quicksort, types |
| `testing/` | 5 | Assertions, fixtures, reporting, runner, types |

## Module organization pattern

Each Pascal module follows the `mod.pas` pattern:
- `mod.pas` — module entry point, re-exports other units
- `types.pas` — shared type definitions
- `*.pas` — individual algorithm/implementation files
- `README.md` — module documentation

## ECS Library (lib/ecs/)

The ECS library is the most sophisticated module, with 9 Pascal files and detailed design docs:

### Core types
- `entity.pas` — Entity IDs and entity management
- `component.pas` — Component registry, component storage
- `system.pas` — System registration and execution
- `world.pas` — World (ecs context) management
- `query.pas` — Query system for filtering entities by components

### Integration
- `physics.pas` — ECS + physics integration
- `ecs_integration.pas` — Generic ECS integration utilities
- `sprite_physics_integration.md` — Sprite physics integration design
- `USAGE_EXAMPLE.pas` — Usage examples

### Design docs
- `README.md` — Module overview
- `SPRITE_PHYSICS_INTEGRATION.md` — Sprite + physics integration plan
- `physics.pas` — Physics systems within ECS

## Physics Library (lib/physics/)

Comprehensive physics simulation:
- `gravity.pas` — Gravity model
- `friction.pas` — Friction calculations
- `bounce.pas` / `bounce_oo.pas` — Bounce/collision response (OO and functional styles)
- `material.pas` — Material properties (restitution, friction coefficients)
- `object_base.pas` — Base physics object
- `object_circle.pas` / `object_polygon.pas` — Physics objects by shape
- `particle.pas` — Particle system
- `conversion.pas` — Unit conversions
- `ecs_integration.pas` — Physics + ECS integration

## Graphics Library (lib/graphics/)

Graphics primitives and rendering:
- `circle.pas` — Circle drawing
- `ellipse.pas` — Ellipse drawing
- `line.pas` — Line drawing
- `polygon.pas` — Polygon rendering
- `tilemap.pas` — Tilemap rendering
- `texture.pas` — Texture management
- `types.pas` — Graphics types

## Math Library (lib/math/)

Mathematical functions:
- `vector.pas` — 2D/3D vector operations
- `matrix.pas` — Matrix operations
- `trig.pas` — Trigonometric functions
- `fixed.pas` — Fixed-point arithmetic (Q8.8, Q12.12)
- `sqrt.pas` — Square root
- `types.pas` — Math types

## Testing Library (lib/testing/)

Unit testing framework for Pascal:
- `assertions.pas` — Assert macros/functions
- `fixtures.pas` — Test fixtures
- `reporting.pas` — Test result reporting
- `runner.pas` — Test runner
