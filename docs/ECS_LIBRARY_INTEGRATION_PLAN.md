# ECS Library Integration Plan

## Overview

This document outlines the plan to integrate the SuperPascal ECS Library into all game-related algorithms from the Mikro archive, ensuring all algorithms use ECS patterns for consistency and reusability.

---

## Integration Strategy

### Phase 1: Core ECS Library (Week 1)

**Goal:** Complete the core ECS library specification and implementation

**Tasks:**
1. ✅ Finalize ECS library API (see `09_ECS_Library.md`)
2. [ ] Implement core `ECS` unit
3. [ ] Create basic component types (Position, Velocity, Sprite)
4. [ ] Implement entity management
5. [ ] Implement component management
6. [ ] Implement query system
7. [ ] Implement system registration and execution

**Deliverables:**
- Complete `ECS` unit implementation
- Basic example demonstrating ECS usage
- Documentation for library API

---

### Phase 2: Algorithm Integration (Week 2-3)

**Goal:** Refactor all game algorithms to use ECS patterns

#### 2.1 Physics Algorithms (algorithms/06_PhysicsSimulation.md)

**Current Status:** ✅ Algorithms documented, but not ECS-integrated

**Integration Tasks:**
- [ ] Create `TPhysicsBody` component
- [ ] Create `TParticle` component
- [ ] Refactor `GravitySystem` as ECS system
- [ ] Refactor `ParticleSystem` as ECS system
- [ ] Refactor `VelocitySystem` as ECS system
- [ ] Refactor `FrictionSystem` as ECS system
- [ ] Update algorithm examples to use ECS

**Example Integration:**
```pascal
// Before (standalone):
procedure UpdateParticle(var particle: TParticle);

// After (ECS-integrated):
procedure ParticleSystem(var world: TWorld);
var
  query: TQuery;
  entity: TEntity;
  particle: ^TParticle;
begin
  query := QueryCreate(world, [COMPONENT_PARTICLE]);
  while QueryNext(query, entity) do
  begin
    particle := ComponentGetParticle(world, entity);
    UpdateParticle(particle^);
  end;
end;
```

#### 2.2 Collision Detection (algorithms/05_CollisionDetection.md)

**Current Status:** ✅ Algorithms documented, but not ECS-integrated

**Integration Tasks:**
- [ ] Create `TColliderAABB` component
- [ ] Create `TColliderCircle` component
- [ ] Refactor `CollisionSystem` as ECS system
- [ ] Update AABB collision to use ECS queries
- [ ] Update circle collision to use ECS queries
- [ ] Update polygon clipping to use ECS queries

**Example Integration:**
```pascal
// Before (standalone):
function CheckAABBCollision(const a, b: TAABB): Boolean;

// After (ECS-integrated):
procedure CollisionSystem(var world: TWorld);
var
  query1, query2: TQuery;
  entity1, entity2: TEntity;
  collider1, collider2: ^TColliderAABB;
begin
  query1 := QueryCreate(world, [COMPONENT_POSITION, COMPONENT_COLLIDER]);
  while QueryNext(query1, entity1) do
  begin
    collider1 := ComponentGetCollider(world, entity1);
    // ... collision checking
  end;
end;
```

#### 2.3 Graphics Algorithms (algorithms/04_GraphicsAlgorithms.md)

**Current Status:** ✅ Algorithms documented, but not ECS-integrated

**Integration Tasks:**
- [ ] Create `TTransform` component (position, rotation, scale)
- [ ] Create `TPolygon` component
- [ ] Create `TSprite` component (already defined)
- [ ] Refactor `RenderSystem` as ECS system
- [ ] Refactor polygon rendering to use ECS
- [ ] Refactor texture mapping to use ECS
- [ ] Refactor tilemap rendering to use ECS

**Example Integration:**
```pascal
// Before (standalone):
procedure RenderPolygon(const vertices: array of TPoint);

// After (ECS-integrated):
procedure PolygonRenderSystem(var world: TWorld);
var
  query: TQuery;
  entity: TEntity;
  transform: ^TTransform;
  polygon: ^TPolygon;
begin
  query := QueryCreate(world, [COMPONENT_TRANSFORM, COMPONENT_POLYGON]);
  while QueryNext(query, entity) do
  begin
    transform := ComponentGetTransform(world, entity);
    polygon := ComponentGetPolygon(world, entity);
    RenderPolygon(transform^, polygon^);
  end;
end;
```

#### 2.4 Advanced Graphics (algorithms/07_AdvancedGraphics.md)

**Current Status:** ⚠️ Structure created, content pending

**Integration Tasks:**
- [ ] Create `T3DTransform` component
- [ ] Create `TMaterial` component
- [ ] Create `TLight` component
- [ ] Refactor 3D shading systems as ECS systems
- [ ] Refactor shadow systems as ECS systems
- [ ] Refactor special effects as ECS systems

#### 2.5 Game Algorithms (algorithms/08_GameAlgorithms.md)

**Current Status:** ⚠️ Structure created, content pending

**Integration Tasks:**
- [ ] Create `TCamera` component
- [ ] Create `TAI` component
- [ ] Create `TGameState` component
- [ ] Refactor camera systems as ECS systems
- [ ] Refactor AI systems as ECS systems
- [ ] Refactor game state management as ECS systems

---

### Phase 3: Book Integration (Week 4)

**Goal:** Update book chapters to use ECS patterns

**Tasks:**
- [ ] Update Chapter 16 (Entity Component System) with ECS library examples
- [ ] Update Chapter 18 (Physics and Movement) to use ECS systems
- [ ] Update Chapter 20 (Graphics Programming) to use ECS systems
- [ ] Update Chapter 21 (Game Development) to use ECS systems
- [ ] Add ECS examples to all relevant chapters

---

### Phase 4: Platform Extensions (Week 5)

**Goal:** Create platform-specific ECS extensions

**Tasks:**
- [ ] Create `ECS_ZealZ80` extension unit
- [ ] Create `ECS_CommanderX16` extension unit
- [ ] Create `ECS_Foenix65C816` extension unit
- [ ] Create `ECS_FoenixA2560M` extension unit
- [ ] Create `ECS_RaspberryPi5` extension unit

**Extension Pattern:**
```pascal
unit ECS_Platform;

interface
uses ECS;

// Platform-specific components
type
  TSpritePlatform = record
    // Platform-specific sprite data
  end;

// Platform-specific systems
procedure RenderSystemPlatform(var world: TWorld);

implementation
  // Platform-specific implementation
end.
```

---

## Algorithm Refactoring Template

### Before (Standalone Algorithm)

```pascal
// algorithms/XX_Algorithm.md
procedure AlgorithmName(var data: TData);
begin
  // Algorithm implementation
end;

// Usage
var
  data: TData;
begin
  AlgorithmName(data);
end;
```

### After (ECS-Integrated Algorithm)

```pascal
// algorithms/XX_Algorithm.md

// Component definition
type
  TAlgorithmComponent = record
    // Algorithm-specific data
  end;

// System implementation
procedure AlgorithmSystem(var world: TWorld);
var
  query: TQuery;
  entity: TEntity;
  component: ^TAlgorithmComponent;
begin
  query := QueryCreate(world, [COMPONENT_ALGORITHM]);
  while QueryNext(query, entity) do
  begin
    component := ComponentGetAlgorithm(world, entity);
    // Algorithm implementation using component data
  end;
end;

// Usage
var
  world: TWorld;
  entity: TEntity;
begin
  WorldInit(world);
  entity := EntityCreate(world);
  ComponentAdd(world, entity, COMPONENT_ALGORITHM);
  SystemRegister(world, AlgorithmSystem);
  WorldUpdate(world);
end;
```

---

## Component Library

### Core Components (ECS unit)

- `TPosition` - 2D position
- `TVelocity` - 2D velocity
- `TSprite` - Basic sprite data

### Physics Components (ECS_Physics unit)

- `TPhysicsBody` - Physics properties (mass, friction, gravity)
- `TParticle` - Particle system data
- `TForce` - Applied forces

### Render Components (ECS_Render unit)

- `TTransform` - Transform (position, rotation, scale)
- `TPolygon` - Polygon data
- `TMaterial` - Material properties
- `TLight` - Light source data

### Game Components (ECS_Game unit)

- `TCamera` - Camera data
- `TAI` - AI state
- `THealth` - Health data
- `TInventory` - Inventory data

---

## System Library

### Core Systems

- `MovementSystem` - Updates position based on velocity
- `RenderSystem` - Renders entities with sprite components

### Physics Systems

- `GravitySystem` - Applies gravity
- `VelocitySystem` - Updates velocity
- `FrictionSystem` - Applies friction
- `ParticleSystem` - Updates particles

### Collision Systems

- `CollisionSystem` - Checks collisions
- `CollisionResolveSystem` - Resolves collisions

### Render Systems

- `PolygonRenderSystem` - Renders polygons
- `TextureRenderSystem` - Renders textures
- `TilemapRenderSystem` - Renders tilemaps

### Game Systems

- `CameraSystem` - Updates camera
- `AISystem` - Updates AI
- `GameStateSystem` - Manages game state

---

## Example: Complete ECS Game

```pascal
program ECSGame;
uses ECS, ECS_Physics, ECS_Render;

var
  world: TWorld;
  player: TEntity;
  enemy: TEntity;
  pos: TPosition;
  vel: TVelocity;
  sprite: TSprite;
  physics: TPhysicsBody;
begin
  // Initialize world
  WorldInit(world);
  
  // Register systems
  SystemRegister(world, MovementSystem);
  SystemRegister(world, GravitySystem);
  SystemRegister(world, CollisionSystem);
  SystemRegister(world, RenderSystem);
  
  // Create player
  player := EntityCreate(world);
  ComponentAdd(world, player, COMPONENT_POSITION);
  ComponentAdd(world, player, COMPONENT_VELOCITY);
  ComponentAdd(world, player, COMPONENT_SPRITE);
  ComponentAdd(world, player, COMPONENT_PHYSICS);
  
  pos.X := 100;
  pos.Y := 100;
  ComponentSetPosition(world, player, pos);
  
  vel.DX := 0;
  vel.DY := 0;
  ComponentSetVelocity(world, player, vel);
  
  sprite.SpriteID := 0;
  sprite.TileID := 1;
  sprite.Visible := True;
  ComponentSetSprite(world, player, sprite);
  
  physics.Gravity := 1;
  physics.Friction := $E0;
  ComponentSetPhysics(world, player, physics);
  
  // Create enemy (similar pattern)
  enemy := EntityCreate(world);
  // ... add components ...
  
  // Game loop
  while True do
  begin
    WorldUpdate(world);
    WaitVBlank;
  end;
  
  WorldCleanup(world);
end.
```

---

## Benefits of ECS Integration

### 1. Consistency

All game algorithms use the same ECS patterns, making code easier to understand and maintain.

### 2. Reusability

Algorithms become reusable systems that can be combined in different ways.

### 3. Performance

ECS patterns (SoA storage, queries) are optimized for performance.

### 4. Extensibility

Easy to add new components and systems without modifying existing code.

### 5. Educational Value

Students learn modern game development patterns (ECS is used in Unity, Unreal, Bevy, etc.).

---

## Migration Checklist

For each algorithm file:

- [ ] Identify algorithm data structures
- [ ] Convert to ECS components
- [ ] Refactor algorithm as ECS system
- [ ] Update examples to use ECS
- [ ] Add ECS usage documentation
- [ ] Test with ECS world
- [ ] Update cross-references

---

## Success Metrics

- ✅ **100% of physics algorithms** use ECS
- ✅ **100% of collision algorithms** use ECS
- ✅ **100% of graphics algorithms** use ECS
- ✅ **100% of game algorithms** use ECS
- ✅ **All book chapters** updated with ECS examples
- ✅ **All platforms** have ECS extensions

---

**Last Updated:** 2025-01-XX  
**Status:** Integration plan created, ready for implementation

