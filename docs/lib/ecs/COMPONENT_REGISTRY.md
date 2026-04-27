# ECS Component Registry

**Date:** 2025-01-XX  
**Status:** 📋 Active Registry  
**Purpose:** Central registry of all ECS components and their relationships

---

## Component Index

| ID | Component | Type | Status | Game Engine | Physics | Notes |
|----|-----------|------|--------|------------|---------|-------|
| 0 | Position | `TPosition` | ✅ Active | ✅ Used | ✅ Compatible | X, Y: Fixed16 |
| 1 | Velocity | `TVelocity` | ✅ Active | ✅ Used | ✅ Compatible | VX, VY: Fixed16 |
| 2 | Acceleration | `TAcceleration` | ✅ Active | ✅ Used | ✅ Compatible | AX, AY: Fixed16 |
| 3 | Material | `TMaterialProperties` | ✅ Active | ✅ Used | ✅ Compatible | Bounce physics |
| 4 | Shape | `TShape` | ✅ Active | ✅ Used | ✅ Compatible | Collision detection |
| 5 | Sprite | `TSprite` | ✅ Active | ✅ Used | ✅ Compatible | Rendering + Physics |
| 6 | Physics | `TPhysicsBody` | ✅ Active | ✅ Used | ⚠️ Partial | Basic physics |
| 7 | Particle | `TParticle` | ✅ Active | ✅ Used | ✅ Compatible | Particle system |

---

## Component Details

### COMPONENT_POSITION (0)

**Type:** `TPosition`

**Fields:**
```pascal
type
  TPosition = record
    X, Y: Fixed16;  // Position in Q8.8 fixed-point
  end;
```

**Usage:**
- Game Engine: Rendering, collision detection
- Physics: Position tracking, integration
- Systems: MovementSystem, VelocitySystem

**Dependencies:**
- `Fixed16` from Math library

**Status:** ✅ Synchronized

---

### COMPONENT_VELOCITY (1)

**Type:** `TVelocity`

**Fields:**
```pascal
type
  TVelocity = record
    VX, VY: Fixed16;  // Velocity in Q8.8 fixed-point
  end;
```

**Usage:**
- Game Engine: Movement, physics integration
- Physics: Velocity tracking, bounce physics
- Systems: MovementSystem, VelocitySystem, GravitySystem, FrictionSystem

**Dependencies:**
- `Fixed16` from Math library

**Status:** ✅ Synchronized

---

### COMPONENT_ACCELERATION (2)

**Type:** `TAcceleration`

**Fields:**
```pascal
type
  TAcceleration = record
    AX, AY: Fixed16;  // Acceleration in Q8.8 fixed-point
  end;
```

**Usage:**
- Game Engine: Physics integration
- Physics: Acceleration tracking, force application
- Systems: Physics systems (future)

**Dependencies:**
- `Fixed16` from Math library

**Status:** ✅ Synchronized

---

### COMPONENT_MATERIAL (3)

**Type:** `TMaterialProperties`

**Fields:**
```pascal
type
  TMaterialProperties = record
    Bouncability: Fixed16;   // 0-256 (0.0-1.0)
    Solidness: Fixed16;      // 0-256 (0.0-1.0)
    Brittleness: Fixed16;    // 0-256 (0.0-1.0)
    Fragmentability: Fixed16; // 0-256 (0.0-1.0)
  end;
```

**Usage:**
- Game Engine: Bounce physics, fragmentation
- Physics: Bounce calculations, fragment generation
- Systems: BounceSystem (future), FragmentationSystem (future)

**Dependencies:**
- `Fixed16` from Math library
- Physics library bounce functions

**Status:** ✅ Synchronized

---

### COMPONENT_SHAPE (4)

**Type:** `TShape`

**Fields:**
```pascal
type
  TShape = record
    ShapeType: TShapeType;   // Circle or Polygon
    CircleRadius: Fixed16;   // For circles
    Polygon: record
      Points: array[0..31] of record
        X, Y: Integer;
      end;
      Count: Byte;
    end;
    PolygonArea: Fixed16;    // Cached area
  end;
```

**Usage:**
- Game Engine: Collision detection, rendering
- Physics: Collision detection, fragmentation
- Systems: CollisionSystem (future), RenderingSystem

**Dependencies:**
- `TShapeType` from Physics library
- `Fixed16` from Math library
- Collision library (for polygon operations)

**Status:** ✅ Synchronized

---

### COMPONENT_SPRITE (5)

**Type:** `TSprite`

**Fields:**
```pascal
type
  TSprite = record
    SpriteID: Byte;
    TileID: Word;
    Palette: Byte;
    Visible: Boolean;
    // Physics integration properties
    Explodable: Boolean;     // Can this sprite explode?
    ExplosionRadius: Fixed16; // Explosion radius if explodable
    ExplosionDamage: Integer; // Damage dealt on explosion
  end;
```

**Usage:**
- Game Engine: Rendering, sprite management, explosions
- Physics: ✅ Compatible (sprites can have Position, Velocity, Material, Shape components)
- Systems: RenderingSystem, ExplosionSystem (future)

**Physics Integration:**
- Sprites can have physics components attached:
  - `TPosition` - For physics-based movement
  - `TVelocity` - For physics-based velocity
  - `TAcceleration` - For physics-based acceleration
  - `TMaterialProperties` - For bounce physics (bouncy sprites)
  - `TShape` - For collision detection

**Dependencies:**
- `Fixed16` from Math library (for ExplosionRadius)

**Status:** ✅ Active (Physics-compatible via component composition)

---

### COMPONENT_PHYSICS (6)

**Type:** `TPhysicsBody`

**Fields:**
```pascal
type
  TPhysicsBody = record
    Mass: Integer;
    Friction: Integer;    // Fixed-point
    Gravity: Integer;
    OnGround: Boolean;
  end;
```

**Usage:**
- Game Engine: Basic physics properties
- Physics: ⚠️ Partial compatibility (different structure)
- Systems: GravitySystem, FrictionSystem

**Dependencies:**
- None (ECS-specific, legacy)

**Status:** ⚠️ Partial (ECS-specific, may be deprecated in favor of Material/Acceleration)

---

### COMPONENT_PARTICLE (7)

**Type:** `TParticle`

**Fields:**
```pascal
type
  TParticle = record
    X, Y: Fixed16;
    VX, VY: Fixed16;
    Energy: Integer;
    Color: Byte;
    Size: Byte;
    Active: Boolean;
  end;
```

**Usage:**
- Game Engine: Particle effects, explosions
- Physics: Particle system utilities
- Systems: ParticleSystem

**Dependencies:**
- `Fixed16` from Math library
- Physics library particle functions

**Status:** ✅ Synchronized

---

## Component Relationships

### Physics Components Group
- `TPosition` - Position tracking
- `TVelocity` - Velocity tracking
- `TAcceleration` - Acceleration tracking
- `TMaterialProperties` - Material properties
- `TShape` - Shape information

**Used Together:** Physics simulation, collision detection, bounce physics

### Rendering Components Group
- `TPosition` - Position for rendering
- `TSprite` - Sprite data
- `TShape` - Shape for rendering

**Used Together:** Rendering system, sprite management

### Particle Components Group
- `TParticle` - Particle data

**Used Alone:** Particle system

---

## Component ID Allocation

### Reserved IDs (0-7)
- 0-4: Core physics components
- 5: Rendering component
- 6: Legacy physics component
- 7: Particle component

### Available IDs (8-31)
- 8-15: Reserved for future core components
- 16-23: Reserved for game engine components
- 24-31: Reserved for platform-specific components

### ID Assignment Rules
1. Core components get lowest IDs (0-7)
2. Game engine components get next range (8-15)
3. Platform-specific components get highest range (24-31)
4. Never reuse IDs
5. Document all ID assignments

---

## Synchronization Status

### ✅ Fully Synchronized
- `TPosition` ↔ Physics ↔ Game Engine
- `TVelocity` ↔ Physics ↔ Game Engine
- `TAcceleration` ↔ Physics ↔ Game Engine
- `TMaterialProperties` ↔ Physics ↔ Game Engine
- `TShape` ↔ Physics ↔ Game Engine
- `TParticle` ↔ Physics ↔ Game Engine

### ⚠️ Partial Synchronization
- `TPhysicsBody` - ECS-specific, different from `TPhysicsObject`

### ✅ Physics-Compatible via Composition
- `TSprite` - Can have physics components attached (Position, Velocity, Material, Shape)
  - Sprites can be bouncy (via Material component)
  - Sprites can explode (via Explodable property)
  - Sprites can have collision (via Shape component)

---

## Future Components

### Planned Components
- **Animation** (ID: 8) - Animation state and frames
- **Audio** (ID: 9) - Audio source and properties
- **Input** (ID: 10) - Input state and bindings
- **AI** (ID: 11) - AI state and behavior
- **Network** (ID: 12) - Network synchronization
- **Camera** (ID: 13) - Camera properties
- **Light** (ID: 14) - Lighting properties
- **Transform** (ID: 15) - Transformation matrix

### Component Proposal Process
1. Document component purpose
2. Define component structure
3. Assign component ID
4. Implement in ECS
5. Integrate with game engine
6. Update documentation

---

## Maintenance

### Regular Updates
- **Weekly:** Review component usage
- **Monthly:** Update synchronization status
- **Per Release:** Full component audit

### Change Log
- Track all component changes
- Document breaking changes
- Provide migration guides

---

**Last Updated:** 2025-01-XX  
**Status:** 📋 Active Registry  
**Next Review:** Monthly

