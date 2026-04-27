# ECS Library

- Status: partially-verified
- Source docs: `languageSpecification/ecs/`, `lib/ecs/`, `docs/BEVY_ECS_REFERENCE.md`
- Code anchors: `lib/ecs/`, `languageSpecification/ecs/`

## Architecture

The ECS library follows a standard Entity-Component-System pattern:

### Core components

| Component | Pascal file | Purpose |
|-----------|------------|---------|
| Entity | `entity.pas` | Entity IDs, entity registry |
| Component | `component.pas` | Component types, registry, storage |
| System | `system.pas` | System base class, registration |
| World | `world.pas` | ECS world context, entity creation |
| Query | `query.pas` | Query system for filtering entities |

### Integration

| Component | Pascal file | Purpose |
|-----------|------------|---------|
| Physics | `physics.pas` | Physics systems |
| ECS Integration | `ecs_integration.pas` | Generic ECS integration |
| Types | `types.pas` | Shared ECS types |

### Documentation

- `README.md` — Module overview
- `USAGE_EXAMPLE.pas` — Usage examples
- `SPRITE_PHYSICS_INTEGRATION.md` — Sprite + physics design
- `SPRITE_PHYSICS_INTEGRATION.md` (languageSpecification) — Extended design

## ECS Design iterations

The `languageSpecification/ecs/` directory documents 6 iterations of ECS design:

| Iteration | Focus | Doc |
|-----------|-------|-----|
| 1 | Core types | `01_Iteration1_CoreTypes.md`, `ECS_Unit_Iteration1.pas` |
| 2 | Entity management | `02_Iteration2_EntityManagement.md`, `ECS_Unit_Iteration2.pas` |
| 3 | Component management | `03_Iteration3_ComponentManagement.md`, `ECS_Unit_Iteration3.pas` |
| 4 | Query system | `04_Iteration4_QuerySystem.md`, `ECS_Unit_Iteration4.pas` |
| 5 | System registration | `05_Iteration5_SystemRegistration.md`, `ECS_Unit_Iteration5.pas` |
| 6 | Algorithm integration | `06_Iteration6_AlgorithmIntegration.md` |
| Final | Complete ECS | `ECS_Unit_Final.pas` |
| README | Overview | `README.md` |

## ECS + Physics + Sprites

The ECS library is designed to integrate with:
1. **Physics** (`lib/physics/`) — collision detection, gravity, bounce
2. **Graphics** (`lib/graphics/`) — sprites, tilemaps, textures

This is documented in:
- `lib/ecs/SPRITE_PHYSICS_INTEGRATION.md`
- `lib/ecs/USAGE_EXAMPLE.pas` — shows integrated usage

## By virtue of Bevy ECS reference

`docs/BEVY_ECS_REFERENCE.md` provides Bevy ECS as a reference design for the SuperPascal ECS library.
