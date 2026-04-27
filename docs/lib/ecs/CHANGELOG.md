# ECS Component Changelog

**Purpose:** Track all component changes for synchronization with game engine

---

## 2025-01-XX - ECS Extension for Physics Compatibility

### Added Components
- **TAcceleration** (ID: 2) - Acceleration component for physics
- **TMaterialProperties** (ID: 3) - Material properties for bounce physics
- **TShape** (ID: 4) - Shape information for collision detection

### Changed Components
- **TPosition** - Standardized to `Fixed16` (was `Integer`)
- **TVelocity** - Standardized to `Fixed16` and renamed fields `DX, DY` → `VX, VY`

### Changed Types
- **Fixed16** - Changed from `Integer` (16.16) to `SmallInt` (Q8.8) to match Physics library

### Component ID Changes
- `COMPONENT_SPRITE`: 2 → 5
- `COMPONENT_PHYSICS`: 3 → 6
- `COMPONENT_PARTICLE`: 4 → 7

### Breaking Changes
- ⚠️ **TVelocity field names:** `DX, DY` → `VX, VY`
- ⚠️ **TPosition type:** `Integer` → `Fixed16`
- ⚠️ **Component IDs:** Some IDs shifted

### Migration Guide
1. Replace `vel.DX` with `vel.VX`
2. Replace `vel.DY` with `vel.VY`
3. Use `IntToFixed16()` for position initialization if needed
4. Update hardcoded component IDs to use named constants

### Files Modified
- `ECS_Types.pas` - Added new components, standardized types
- `ECS_World.pas` - Added component arrays
- `ECS_Component.pas` - Added accessors
- `ECS_System.pas` - Updated world layout
- `ECS_Physics.pas` - Updated to use VX, VY
- `ECS_Entity.pas` - Updated cleanup
- `mod.pas` - Exported new types

---

## Future Changes

### Planned
- Animation component (ID: 8)
- Audio component (ID: 9)
- Input component (ID: 10)

### Under Consideration
- Deprecate `TPhysicsBody` in favor of `TMaterialProperties` + `TAcceleration`
- Add `TTransform` component for hierarchical transforms
- Add `TCamera` component for camera management

---

**Format:** Date - Description  
**Breaking Changes:** Marked with ⚠️  
**Migration Guides:** Provided for breaking changes

