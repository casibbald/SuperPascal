# ECS ↔ Game Engine Synchronization Strategy

**Date:** 2025-01-XX  
**Status:** 📋 Active Strategy  
**Purpose:** Keep ECS and Game Engine components in sync

---

## Overview

This document defines the strategy for keeping the ECS system and future game engine components synchronized. The goal is to maintain a **holistic codebase** where ECS components and game engine systems work seamlessly together without duplication or divergence.

---

## Synchronization Principles

### 1. Single Source of Truth
- **ECS defines component structures** - Game engine uses ECS components
- **Physics library defines physics types** - ECS components match Physics types
- **Math library defines Fixed16** - All systems use the same Fixed16 type

### 2. Direct Compatibility
- **No conversion layers** - Direct field access between systems
- **Type consistency** - Same types across all systems
- **Naming consistency** - Same field names across all systems

### 3. Extension Protocol
- **New components** - Add to ECS first, then use in game engine
- **New types** - Add to shared libraries (Math, Physics, Collision)
- **Breaking changes** - Coordinate across all systems

---

## What Needs to Stay in Sync

### 1. Component Types
- `TPosition` - Position component
- `TVelocity` - Velocity component
- `TAcceleration` - Acceleration component
- `TMaterialProperties` - Material properties
- `TShape` - Shape information
- `TSprite` - Sprite/rendering component
- `TPhysicsBody` - Basic physics properties
- `TParticle` - Particle component

### 2. Type Definitions
- `Fixed16` - Fixed-point type (Q8.8 format)
- `TShapeType` - Shape type enum
- `TEntity` - Entity ID type
- `TComponentID` - Component ID constants

### 3. Component IDs
- Component ID constants must match across systems
- New components must get unique IDs
- Component ID changes require coordination

### 4. Naming Conventions
- Field names (VX, VY, not DX, DY)
- Type names (Fixed16, not Integer)
- Function names (consistent patterns)

---

## Synchronization Checklist

### When Adding a New Component

- [ ] **1. Define in ECS_Types**
  - Add component type definition
  - Assign unique component ID
  - Document purpose and usage

- [ ] **2. Update ECS_World**
  - Add component array to TWorld
  - Initialize in WorldInit
  - Clear in EntityDestroy

- [ ] **3. Update ECS_Component**
  - Add ComponentGet* accessor
  - Add ComponentSet* accessor
  - Initialize in ComponentAdd
  - Clear in ComponentRemove

- [ ] **4. Update ECS_System**
  - Update TWorldWithSystems layout
  - Ensure layout matches TWorld

- [ ] **5. Update mod.pas**
  - Export new component type
  - Export new component ID constant

- [ ] **6. Update Game Engine (if applicable)**
  - Use new component in game engine systems
  - Update game engine documentation
  - Add game engine examples

- [ ] **7. Update Documentation**
  - Update ECS README
  - Update component usage examples
  - Update synchronization guide

---

## Component Type Guidelines

### Standard Component Structure

```pascal
type
  TComponentName = record
    Field1: Type1;  // Use Fixed16 for physics values
    Field2: Type2;  // Use consistent naming
    // ... more fields ...
  end;
```

### Naming Conventions

- **Component Types:** `T` prefix (e.g., `TPosition`, `TVelocity`)
- **Component IDs:** `COMPONENT_` prefix (e.g., `COMPONENT_POSITION`)
- **Field Names:** 
  - Position: `X, Y`
  - Velocity: `VX, VY` (not DX, DY)
  - Acceleration: `AX, AY`
  - Material: `Bouncability, Solidness, Brittleness, Fragmentability`
  - Shape: `ShapeType, CircleRadius, Polygon, PolygonArea`

### Type Guidelines

- **Physics Values:** Always use `Fixed16` (Q8.8 format)
- **Integer Values:** Use `Integer` or `SmallInt` as appropriate
- **Enums:** Use Pascal enums (e.g., `TShapeType`)
- **Arrays:** Use static arrays for retro platforms where possible

---

## Shared Type Dependencies

### Core Types (Math Library)
- `Fixed16` - Q8.8 fixed-point type
- `TVector2`, `TVector3` - Vector types
- `TMatrix2x2`, `TMatrix3x3` - Matrix types

### Physics Types (Physics Library)
- `TPhysicsObject` - Physics object structure
- `TShapeType` - Shape type enum
- `TParticle` - Particle structure
- `TSurface` - Surface/material properties

### Collision Types (Collision Library)
- `TPoint2D` - 2D point
- `TPolygon2D` - Polygon structure
- `TAABB` - Axis-aligned bounding box
- `TCircle` - Circle structure

### ECS Types (ECS Library)
- `TEntity` - Entity ID
- `TComponentID` - Component ID
- `TComponentMask` - Component presence mask
- All component types (`TPosition`, `TVelocity`, etc.)

---

## Game Engine Integration Points

### 1. Rendering System
- Uses: `TPosition`, `TSprite`, `TShape`
- Reads: Position, sprite data, shape for collision
- Writes: Sprite visibility, animation state

### 2. Physics System
- Uses: `TPosition`, `TVelocity`, `TAcceleration`, `TMaterialProperties`, `TShape`
- Reads: All physics components
- Writes: Position, velocity, acceleration updates

### 3. Collision System
- Uses: `TPosition`, `TShape`, `TVelocity`
- Reads: Position, shape for collision detection
- Writes: Collision responses (velocity updates)

### 4. Particle System
- Uses: `TParticle`
- Reads: Particle data
- Writes: Particle updates (position, velocity, energy)

### 5. Input System
- Uses: `TVelocity`, `TAcceleration`
- Reads: Input state
- Writes: Velocity/acceleration changes

---

## Version Control Strategy

### Component Versioning
- **Major Changes:** Breaking changes to component structure
- **Minor Changes:** New fields added (backward compatible)
- **Patch Changes:** Bug fixes, documentation updates

### Change Coordination
1. **Proposal:** Document proposed changes
2. **Review:** Review impact on all systems
3. **Implementation:** Implement in ECS first
4. **Testing:** Test with game engine systems
5. **Documentation:** Update all documentation
6. **Migration:** Provide migration guide if breaking

---

## Testing Strategy

### Unit Tests
- Component creation and initialization
- Component accessors (Get/Set)
- Component removal and cleanup
- Type compatibility checks

### Integration Tests
- ECS ↔ Game Engine component access
- ECS ↔ Physics library compatibility
- Component synchronization across systems
- System execution order

### Regression Tests
- Existing game engine code continues to work
- Component ID changes don't break systems
- Type changes don't cause compilation errors

---

## Documentation Requirements

### Component Documentation
- **Purpose:** What the component represents
- **Fields:** Field descriptions and types
- **Usage:** How to use the component
- **Examples:** Code examples
- **Related:** Related components and systems

### System Documentation
- **Dependencies:** Which components are used
- **Reads:** Which component fields are read
- **Writes:** Which component fields are written
- **Order:** Execution order requirements

### Change Documentation
- **Breaking Changes:** Document all breaking changes
- **Migration Guides:** Provide migration steps
- **Deprecation:** Mark deprecated components/fields

---

## Automation Opportunities

### Future Enhancements
1. **Code Generation:** Generate component accessors from definitions
2. **Type Checking:** Automated type compatibility checks
3. **Documentation Generation:** Auto-generate component docs
4. **Synchronization Checks:** Automated sync verification

---

## Current Sync Status

### ✅ Synchronized
- `TPosition` - ECS ↔ Physics ↔ Game Engine
- `TVelocity` - ECS ↔ Physics ↔ Game Engine
- `TAcceleration` - ECS ↔ Physics ↔ Game Engine
- `TMaterialProperties` - ECS ↔ Physics ↔ Game Engine
- `TShape` - ECS ↔ Physics ↔ Game Engine
- `TParticle` - ECS ↔ Physics ↔ Game Engine
- `Fixed16` - All systems use Q8.8 format
- `TShapeType` - Shared enum across systems

### ⚠️ Needs Attention
- `TPhysicsBody` - ECS-specific, may need game engine integration
- `TSprite` - ECS-specific, game engine rendering integration

### 📋 Future Additions
- Animation components
- Audio components
- Input components
- Network components
- AI components

---

## Quick Reference

### Adding a New Component

1. **Define Type** (`ECS_Types.pas`):
```pascal
type
  TNewComponent = record
    Field1: Fixed16;
    Field2: Integer;
  end;

const
  COMPONENT_NEW = 8;  // Next available ID
```

2. **Add to World** (`ECS_World.pas`):
```pascal
type
  TWorld = record
    // ... existing ...
    NewComponents: array[0..MAX_ENTITIES-1] of TNewComponent;
  end;
```

3. **Add Accessors** (`ECS_Component.pas`):
```pascal
function ComponentGetNew(var world: TWorld; entity: TEntity): ^TNewComponent;
procedure ComponentSetNew(var world: TWorld; entity: TEntity; const comp: TNewComponent);
```

4. **Initialize** (`ComponentAdd`):
```pascal
COMPONENT_NEW:
begin
  world.NewComponents[entity].Field1 := 0;
  world.NewComponents[entity].Field2 := 0;
end;
```

5. **Export** (`mod.pas`):
```pascal
type
  TNewComponent = ECS_Types.TNewComponent;

const
  COMPONENT_NEW = ECS_Types.COMPONENT_NEW;
```

---

## Maintenance Schedule

### Weekly
- Review component usage in game engine
- Check for type mismatches
- Verify naming consistency

### Monthly
- Review synchronization status
- Update documentation
- Check for deprecated components

### Per Release
- Full synchronization audit
- Breaking change documentation
- Migration guide updates

---

**Last Updated:** 2025-01-XX  
**Status:** 📋 Active Strategy  
**Next Review:** Monthly

