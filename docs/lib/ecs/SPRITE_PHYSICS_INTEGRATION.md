# Sprite Physics Integration Guide

**Date:** 2025-01-XX  
**Status:** 📋 Integration Guide  
**Purpose:** How to make sprites bouncy, explodable, and physics-enabled

---

## Overview

Sprites can now interact with the physics system through **component composition**. A sprite entity can have both `TSprite` (rendering) and physics components (`TPosition`, `TVelocity`, `TMaterialProperties`, `TShape`) attached to enable physics behavior.

---

## Making a Sprite Bouncy

### Step 1: Add Required Components

```pascal
var
  entity: TEntity;
  sprite: TSprite;
  pos: TPosition;
  vel: TVelocity;
  material: TMaterialProperties;
  shape: TShape;
begin
  entity := EntityCreate(world);
  
  // Add sprite component (rendering)
  ComponentAdd(world, entity, COMPONENT_SPRITE);
  
  // Add physics components (for bouncing)
  ComponentAdd(world, entity, COMPONENT_POSITION);
  ComponentAdd(world, entity, COMPONENT_VELOCITY);
  ComponentAdd(world, entity, COMPONENT_MATERIAL);
  ComponentAdd(world, entity, COMPONENT_SHAPE);
  
  // Configure sprite
  sprite.SpriteID := 1;
  sprite.TileID := 100;
  sprite.Palette := 0;
  sprite.Visible := True;
  sprite.Explodable := False;  // Not explodable
  ComponentSetSprite(world, entity, sprite);
  
  // Configure position
  pos.X := IntToFixed16(100);
  pos.Y := IntToFixed16(200);
  ComponentSetPosition(world, entity, pos);
  
  // Configure velocity (bouncing ball)
  vel.VX := IntToFixed16(5);
  vel.VY := IntToFixed16(-10);
  ComponentSetVelocity(world, entity, vel);
  
  // Configure material (bouncy ball)
  material.Bouncability := 230;  // 0.9 (very bouncy)
  material.Solidness := 50;      // 0.2 (not solid)
  material.Brittleness := 0;      // 0.0 (not brittle)
  material.Fragmentability := 0;  // 0.0 (no fragments)
  ComponentSetMaterial(world, entity, material);
  
  // Configure shape (circle for ball)
  shape.ShapeType := stCircle;
  shape.CircleRadius := IntToFixed16(25);
  shape.PolygonArea := 0;
  ComponentSetShape(world, entity, shape);
end.
```

### Step 2: Use Physics Systems

```pascal
// Register physics systems
SystemRegister(world, GravitySystem);
SystemRegister(world, FrictionSystem);
SystemRegister(world, MovementSystem);

// Register bounce system (future)
// SystemRegister(world, BounceSystem);

// Game loop
while True do
begin
  WorldUpdate(world);  // Runs all systems
  // Render sprites...
end.
```

---

## Making a Sprite Explodable

### Step 1: Configure Sprite as Explodable

```pascal
var
  entity: TEntity;
  sprite: TSprite;
  pos: TPosition;
  material: TMaterialProperties;
  shape: TShape;
begin
  entity := EntityCreate(world);
  
  // Add components
  ComponentAdd(world, entity, COMPONENT_SPRITE);
  ComponentAdd(world, entity, COMPONENT_POSITION);
  ComponentAdd(world, entity, COMPONENT_MATERIAL);
  ComponentAdd(world, entity, COMPONENT_SHAPE);
  
  // Configure sprite as explodable
  sprite.SpriteID := 2;
  sprite.TileID := 200;
  sprite.Palette := 1;
  sprite.Visible := True;
  sprite.Explodable := True;              // ✅ Can explode
  sprite.ExplosionRadius := IntToFixed16(50);  // 50 pixel radius
  sprite.ExplosionDamage := 100;          // 100 damage
  ComponentSetSprite(world, entity, sprite);
  
  // Configure position
  pos.X := IntToFixed16(300);
  pos.Y := IntToFixed16(400);
  ComponentSetPosition(world, entity, pos);
  
  // Configure material (brittle, explodable)
  material.Bouncability := 0;      // 0.0 (not bouncy)
  material.Solidness := 200;       // 0.78 (solid)
  material.Brittleness := 230;     // 0.9 (very brittle)
  material.Fragmentability := 200; // 0.78 (high fragments)
  ComponentSetMaterial(world, entity, material);
  
  // Configure shape (circle for explosion)
  shape.ShapeType := stCircle;
  shape.CircleRadius := IntToFixed16(30);
  shape.PolygonArea := 0;
  ComponentSetShape(world, entity, shape);
end.
```

### Step 2: Create Explosion System

```pascal
procedure ExplosionSystem(var world: TWorld);
var
  query: TQuery;
  entity: TEntity;
  sprite: ^TSprite;
  pos: ^TPosition;
  material: ^TMaterialProperties;
  shape: ^TShape;
  i: integer;
  otherEntity: TEntity;
  otherPos: ^TPosition;
  otherSprite: ^TSprite;
  distance: Fixed16;
  dx, dy: Fixed16;
begin
  // Find all explodable sprites
  query := QueryCreate(world, [COMPONENT_SPRITE, COMPONENT_POSITION, COMPONENT_MATERIAL, COMPONENT_SHAPE]);
  
  while QueryNext(query, entity) do
  begin
    sprite := ComponentGetSprite(world, entity);
    pos := ComponentGetPosition(world, entity);
    material := ComponentGetMaterial(world, entity);
    shape := ComponentGetShape(world, entity);
    
    if (sprite <> nil) and (pos <> nil) and (material <> nil) and (shape <> nil) then
    begin
      // Check if sprite should explode (example: collision with high velocity)
      // This is a placeholder - actual explosion trigger logic goes here
      
      if sprite^.Explodable then
      begin
        // Create explosion effect
        // 1. Create particles at explosion location
        // 2. Damage nearby entities
        // 3. Apply physics forces to nearby objects
        
        // Example: Damage nearby entities
        for i := 0 to world.EntityCount - 1 do
        begin
          if world.EntityValid[i] and (i <> entity) then
          begin
            otherPos := ComponentGetPosition(world, i);
            otherSprite := ComponentGetSprite(world, i);
            
            if (otherPos <> nil) and (otherSprite <> nil) then
            begin
              // Calculate distance
              dx := otherPos^.X - pos^.X;
              dy := otherPos^.Y - pos^.Y;
              distance := Fixed16Sqrt(Fixed16Mul(dx, dx) + Fixed16Mul(dy, dy));
              
              // Check if within explosion radius
              if distance <= sprite^.ExplosionRadius then
              begin
                // Apply damage (example - actual damage system would be separate)
                // DamageEntity(world, i, sprite^.ExplosionDamage);
                
                // Apply explosion force (if entity has velocity component)
                var vel: ^TVelocity;
                vel := ComponentGetVelocity(world, i);
                if vel <> nil then
                begin
                  // Push away from explosion center
                  var force: Fixed16;
                  force := Fixed16Div(sprite^.ExplosionDamage, distance);
                  vel^.VX := vel^.VX + Fixed16Mul(force, Fixed16Div(dx, distance));
                  vel^.VY := vel^.VY + Fixed16Mul(force, Fixed16Div(dy, distance));
                end;
              end;
            end;
          end;
        end;
        
        // Destroy or hide exploded sprite
        sprite^.Visible := False;
        // Or: EntityDestroy(world, entity);
      end;
    end;
  end;
end;
```

---

## Complete Example: Bouncy Explodable Sprite

```pascal
program BouncyExplodableSprite;

uses ECS, Physics, Math;

var
  world: TWorld;
  entity: TEntity;
  sprite: TSprite;
  pos: TPosition;
  vel: TVelocity;
  material: TMaterialProperties;
  shape: TShape;
begin
  WorldInit(world);
  entity := EntityCreate(world);
  
  // Add all components
  ComponentAdd(world, entity, COMPONENT_SPRITE);
  ComponentAdd(world, entity, COMPONENT_POSITION);
  ComponentAdd(world, entity, COMPONENT_VELOCITY);
  ComponentAdd(world, entity, COMPONENT_ACCELERATION);
  ComponentAdd(world, entity, COMPONENT_MATERIAL);
  ComponentAdd(world, entity, COMPONENT_SHAPE);
  
  // Configure sprite (bouncy AND explodable)
  sprite.SpriteID := 1;
  sprite.TileID := 100;
  sprite.Palette := 0;
  sprite.Visible := True;
  sprite.Explodable := True;              // Can explode
  sprite.ExplosionRadius := IntToFixed16(75);
  sprite.ExplosionDamage := 150;
  ComponentSetSprite(world, entity, sprite);
  
  // Configure position
  pos.X := IntToFixed16(100);
  pos.Y := IntToFixed16(100);
  ComponentSetPosition(world, entity, pos);
  
  // Configure velocity
  vel.VX := IntToFixed16(10);
  vel.VY := IntToFixed16(-15);
  ComponentSetVelocity(world, entity, vel);
  
  // Configure material (bouncy but brittle)
  material.Bouncability := 200;    // 0.78 (bouncy)
  material.Solidness := 100;       // 0.39 (not very solid)
  material.Brittleness := 180;    // 0.7 (brittle - will break on impact)
  material.Fragmentability := 150; // 0.59 (moderate fragments)
  ComponentSetMaterial(world, entity, material);
  
  // Configure shape (circle)
  shape.ShapeType := stCircle;
  shape.CircleRadius := IntToFixed16(30);
  shape.PolygonArea := 0;
  ComponentSetShape(world, entity, shape);
  
  // Register systems
  SystemRegister(world, GravitySystem);
  SystemRegister(world, FrictionSystem);
  SystemRegister(world, MovementSystem);
  // SystemRegister(world, BounceSystem);  // Future
  // SystemRegister(world, ExplosionSystem);  // Future
  
  // Game loop
  while True do
  begin
    WorldUpdate(world);
    // Check for collisions and trigger explosions
    // Render sprites...
  end;
  
  WorldCleanup(world);
end.
```

---

## Component Combinations

### Bouncy Sprite (No Explosion)
- `TSprite` + `TPosition` + `TVelocity` + `TMaterialProperties` + `TShape`
- `Material.Bouncability` > 128 (0.5)
- `Sprite.Explodable = False`

### Explodable Sprite (No Bounce)
- `TSprite` + `TPosition` + `TMaterialProperties` + `TShape`
- `Sprite.Explodable = True`
- `Material.Bouncability` < 128 (not bouncy)

### Bouncy AND Explodable Sprite
- `TSprite` + `TPosition` + `TVelocity` + `TMaterialProperties` + `TShape`
- `Material.Bouncability` > 128 (bouncy)
- `Sprite.Explodable = True`
- Bounces until it hits something hard enough, then explodes

### Static Sprite (No Physics)
- `TSprite` only
- No physics components
- Just rendering

---

## Physics System Integration

### Systems That Work with Sprites

1. **GravitySystem** - Applies gravity to sprites with `TVelocity` component
2. **FrictionSystem** - Applies friction to sprites with `TVelocity` component
3. **MovementSystem** - Updates position based on velocity
4. **BounceSystem** (future) - Handles bounce physics for sprites with `TMaterialProperties`
5. **ExplosionSystem** (future) - Handles explosions for sprites with `Explodable = True`
6. **CollisionSystem** (future) - Detects collisions using `TShape` component

### Query Patterns

```pascal
// Find all bouncy sprites
query := QueryCreate(world, [COMPONENT_SPRITE, COMPONENT_MATERIAL]);
while QueryNext(query, entity) do
begin
  sprite := ComponentGetSprite(world, entity);
  material := ComponentGetMaterial(world, entity);
  if (sprite <> nil) and (material <> nil) and (material^.Bouncability > 128) then
  begin
    // Handle bouncy sprite
  end;
end;

// Find all explodable sprites
query := QueryCreate(world, [COMPONENT_SPRITE]);
while QueryNext(query, entity) do
begin
  sprite := ComponentGetSprite(world, entity);
  if (sprite <> nil) and sprite^.Explodable then
  begin
    // Handle explodable sprite
  end;
end;

// Find all physics-enabled sprites
query := QueryCreate(world, [COMPONENT_SPRITE, COMPONENT_POSITION, COMPONENT_VELOCITY]);
while QueryNext(query, entity) do
begin
  // Sprite has physics
end;
```

---

## Best Practices

### 1. Component Composition
- **Don't** add physics properties directly to `TSprite`
- **Do** add physics components (`TPosition`, `TVelocity`, `TMaterialProperties`, `TShape`)
- **Do** use `Sprite.Explodable` for explosion-specific properties

### 2. Performance
- Only add physics components to sprites that need them
- Static sprites (background, UI) should only have `TSprite`
- Use queries efficiently (query only needed components)

### 3. Explosion Handling
- Check `Sprite.Explodable` before processing explosions
- Use `Sprite.ExplosionRadius` for damage/force calculations
- Use `Sprite.ExplosionDamage` for damage values
- Consider creating particle effects on explosion

### 4. Bounce Physics
- Use `TMaterialProperties.Bouncability` for bounce coefficient
- Use `TMaterialProperties.Solidness` for collision response
- Use `TMaterialProperties.Brittleness` for break-on-impact
- Combine with `TShape` for collision detection

---

## Future Enhancements

### Planned Features
1. **ExplosionSystem** - Automatic explosion handling
2. **BounceSystem** - Automatic bounce physics
3. **CollisionSystem** - Sprite-to-sprite collision detection
4. **DamageSystem** - Health and damage tracking
5. **ParticleSystem Integration** - Explosion particles

### Component Extensions
- `THealth` component for damage tracking
- `TExplosionEffect` component for explosion visuals
- `TBounceSound` component for audio feedback

---

**Last Updated:** 2025-01-XX  
**Status:** 📋 Integration Guide  
**See Also:** [Component Registry](../docs/lib/ecs/COMPONENT_REGISTRY.md)

