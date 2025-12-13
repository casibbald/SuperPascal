unit ECS_Physics;

interface

uses
  ECS_Types,
  ECS_World,
  ECS_Query,
  ECS_Component,
  Physics_Bounce,
  Physics_Types,
  Collision,
  Collision_Types,
  Math_Fixed,
  Math_Types,
  Math_Sqrt;  // For Fixed16Sqrt
  // Note: ECS_Types provides IntToFixed16 (delegates to Math_Fixed)

// Physics systems (from Mikro archive algorithms)
procedure MovementSystem(var world: TWorld);
procedure GravitySystem(var world: TWorld);
procedure VelocitySystem(var world: TWorld);
procedure FrictionSystem(var world: TWorld);
procedure ParticleSystem(var world: TWorld);

// Advanced physics systems (new)
procedure BounceSystem(var world: TWorld);      // Handles bounce physics for entities with Material + Shape
procedure ExplosionSystem(var world: TWorld);   // Handles explosions for entities with Explodable = True
procedure CollisionSystem(var world: TWorld);   // Detects collisions using TShape component

implementation

// Movement System (updates position based on velocity)
procedure MovementSystem(var world: TWorld);
var
  query: TQuery;
  entity: TEntity;
  pos: ^TPosition;
  vel: ^TVelocity;
begin
  query := QueryCreate(world, [COMPONENT_POSITION, COMPONENT_VELOCITY]);
  while QueryNext(query, entity) do
  begin
    pos := ComponentGetPosition(world, entity);
    vel := ComponentGetVelocity(world, entity);
    
    if (pos <> nil) and (vel <> nil) then
    begin
      pos^.X := pos^.X + vel^.VX;
      pos^.Y := pos^.Y + vel^.VY;
    end;
  end;
end;

// Gravity System (from GRAVITY.TXT)
procedure GravitySystem(var world: TWorld);
var
  query: TQuery;
  entity: TEntity;
  vel: ^TVelocity;
  physics: ^TPhysicsBody;
begin
  query := QueryCreate(world, [COMPONENT_VELOCITY, COMPONENT_PHYSICS]);
  while QueryNext(query, entity) do
  begin
    vel := ComponentGetVelocity(world, entity);
    physics := ComponentGetPhysics(world, entity);
    
    if (vel <> nil) and (physics <> nil) then
    begin
      // Apply gravity
      vel^.VY := vel^.VY + physics^.Gravity;
    end;
  end;
end;

// Velocity System (updates position based on velocity)
procedure VelocitySystem(var world: TWorld);
var
  query: TQuery;
  entity: TEntity;
  pos: ^TPosition;
  vel: ^TVelocity;
begin
  query := QueryCreate(world, [COMPONENT_POSITION, COMPONENT_VELOCITY]);
  while QueryNext(query, entity) do
  begin
    pos := ComponentGetPosition(world, entity);
    vel := ComponentGetVelocity(world, entity);
    
    if (pos <> nil) and (vel <> nil) then
    begin
      pos^.X := pos^.X + vel^.VX;
      pos^.Y := pos^.Y + vel^.VY;
    end;
  end;
end;

// Friction System
procedure FrictionSystem(var world: TWorld);
var
  query: TQuery;
  entity: TEntity;
  vel: ^TVelocity;
  physics: ^TPhysicsBody;
begin
  query := QueryCreate(world, [COMPONENT_VELOCITY, COMPONENT_PHYSICS]);
  while QueryNext(query, entity) do
  begin
    vel := ComponentGetVelocity(world, entity);
    physics := ComponentGetPhysics(world, entity);
    
    if (vel <> nil) and (physics <> nil) then
    begin
      // Apply friction (fixed-point multiplication)
      vel^.VX := (vel^.VX * physics^.Friction) shr 8;  // Fixed-point multiply
      vel^.VY := (vel^.VY * physics^.Friction) shr 8;
    end;
  end;
end;

// Particle System (from PARTICLE.TXT)
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
    
    if (particle <> nil) and particle^.Active then
    begin
      // Update position
      particle^.X := particle^.X + particle^.VX;
      particle^.Y := particle^.Y + particle^.VY;
      
      // Apply gravity (simplified)
      particle^.VY := particle^.VY + IntToFixed16(1);  // Gravity constant
      
      // Decrease energy
      particle^.Energy := particle^.Energy - 1;
      
      // Check if particle is dead
      if particle^.Energy <= 0 then
      begin
        particle^.Active := False;
        // Optionally destroy entity
        // EntityDestroy(world, entity);
      end;
    end;
  end;
end;

// ============================================================================
// Bounce System
// ============================================================================

// Bounce System: Handles bounce physics for entities with Material + Shape components
// Uses Physics_Bounce functions to handle collisions with surfaces and other objects
procedure BounceSystem(var world: TWorld);
var
  query: TQuery;
  entity: TEntity;
  pos: ^TPosition;
  vel: ^TVelocity;
  material: ^TMaterialProperties;
  shape: ^TShape;
  physicsObj: TPhysicsObject;
  surface: TSurface;
  normalX, normalY: Fixed16;
  screenLeft, screenRight, screenTop, screenBottom: integer;
  radius: integer;
  i: integer;
begin
  // Query entities with Position, Velocity, Material, and Shape components
  query := QueryCreate(world, [COMPONENT_POSITION, COMPONENT_VELOCITY, COMPONENT_MATERIAL, COMPONENT_SHAPE]);
  
  // For simplicity, assume screen bounds (can be made configurable)
  screenLeft := 0;
  screenTop := 0;
  screenRight := 640;  // Default screen width
  screenBottom := 480; // Default screen height
  
  while QueryNext(query, entity) do
  begin
    pos := ComponentGetPosition(world, entity);
    vel := ComponentGetVelocity(world, entity);
    material := ComponentGetMaterial(world, entity);
    shape := ComponentGetShape(world, entity);
    
    if (pos <> nil) and (vel <> nil) and (material <> nil) and (shape <> nil) then
    begin
      // Convert ECS components to TPhysicsObject
      physicsObj.X := pos^.X;
      physicsObj.Y := pos^.Y;
      physicsObj.VX := vel^.VX;
      physicsObj.VY := vel^.VY;
      physicsObj.AX := 0;  // Acceleration not used in bounce
      physicsObj.AY := 0;
      physicsObj.Bouncability := material^.Bouncability;
      physicsObj.Solidness := material^.Solidness;
      physicsObj.Brittleness := material^.Brittleness;
      physicsObj.Fragmentability := material^.Fragmentability;
      physicsObj.ShapeType := shape^.ShapeType;
      physicsObj.CircleRadius := shape^.CircleRadius;
      physicsObj.PolygonArea := shape^.PolygonArea;
      
      // Convert ECS TShape.Polygon to TPolygon2D
      physicsObj.Polygon.Count := shape^.Polygon.Count;
      for i := 0 to shape^.Polygon.Count - 1 do
      begin
        physicsObj.Polygon.Points[i].X := shape^.Polygon.Points[i].X;
        physicsObj.Polygon.Points[i].Y := shape^.Polygon.Points[i].Y;
      end;
      
      // Check for collisions with screen boundaries
      if shape^.ShapeType = stCircle then
      begin
        radius := Fixed16ToInt(shape^.CircleRadius);
        
        // Check left wall
        if Fixed16ToInt(pos^.X) - radius < screenLeft then
        begin
          normalX := FIXED16_ONE;  // Normal pointing right (away from wall)
          normalY := 0;
          surface := CreateSolidSurface;
          BounceOffSurface(physicsObj, surface, normalX, normalY);
        end
        // Check right wall
        else if Fixed16ToInt(pos^.X) + radius > screenRight then
        begin
          normalX := -FIXED16_ONE;  // Normal pointing left (away from wall)
          normalY := 0;
          surface := CreateSolidSurface;
          BounceOffSurface(physicsObj, surface, normalX, normalY);
        end;
        
        // Check top wall
        if Fixed16ToInt(pos^.Y) - radius < screenTop then
        begin
          normalX := 0;
          normalY := FIXED16_ONE;  // Normal pointing down (away from ceiling)
          surface := CreateSolidSurface;
          BounceOffSurface(physicsObj, surface, normalX, normalY);
        end
        // Check bottom wall (floor)
        else if Fixed16ToInt(pos^.Y) + radius > screenBottom then
        begin
          normalX := 0;
          normalY := -FIXED16_ONE;  // Normal pointing up (away from floor)
          surface := CreateSolidSurface;
          BounceOffSurface(physicsObj, surface, normalX, normalY);
        end;
      end
      else
      begin
        // Polygon: Use AABB for boundary checking (simplified)
        // TODO: Full polygon boundary collision detection
        // For now, use circle approximation
        radius := Fixed16ToInt(shape^.CircleRadius);  // Use radius as approximation
        if radius = 0 then radius := 16;  // Default radius
        
        // Check boundaries (same as circle)
        if Fixed16ToInt(pos^.X) - radius < screenLeft then
        begin
          normalX := FIXED16_ONE;
          normalY := 0;
          surface := CreateSolidSurface;
          BounceOffSurface(physicsObj, surface, normalX, normalY);
        end
        else if Fixed16ToInt(pos^.X) + radius > screenRight then
        begin
          normalX := -FIXED16_ONE;
          normalY := 0;
          surface := CreateSolidSurface;
          BounceOffSurface(physicsObj, surface, normalX, normalY);
        end;
        
        if Fixed16ToInt(pos^.Y) - radius < screenTop then
        begin
          normalX := 0;
          normalY := FIXED16_ONE;
          surface := CreateSolidSurface;
          BounceOffSurface(physicsObj, surface, normalX, normalY);
        end
        else if Fixed16ToInt(pos^.Y) + radius > screenBottom then
        begin
          normalX := 0;
          normalY := -FIXED16_ONE;
          surface := CreateSolidSurface;
          BounceOffSurface(physicsObj, surface, normalX, normalY);
        end;
      end;
      
      // Update ECS components with bounced physics object
      vel^.VX := physicsObj.VX;
      vel^.VY := physicsObj.VY;
    end;
  end;
end;

// ============================================================================
// Explosion System
// ============================================================================

// Explosion System: Handles explosions for entities with Explodable = True
// Creates particle effects and applies damage/force to nearby entities
procedure ExplosionSystem(var world: TWorld);
var
  query: TQuery;
  entity: TEntity;
  sprite: ^TSprite;
  pos: ^TPosition;
  explosionPos: ^TPosition;
  explosionSprite: ^TSprite;
  explosionRadius: Fixed16;
  explosionDamage: integer;
  distance: Fixed16;
  dx, dy: Fixed16;
  force: Fixed16;
  affectedEntity: TEntity;
  affectedVel: ^TVelocity;
  affectedPos: ^TPosition;
  i: integer;
  allEntities: TQuery;
begin
  // Query entities with Sprite component that are explodable
  query := QueryCreate(world, [COMPONENT_SPRITE]);
  
  while QueryNext(query, entity) do
  begin
    sprite := ComponentGetSprite(world, entity);
    
    // Check if sprite is explodable and should explode
    // Note: In a real game, you'd check collision or trigger conditions
    // For now, this is a placeholder that checks if Explodable is set
    // You would typically trigger explosions based on collision or game logic
    
    if (sprite <> nil) and sprite^.Explodable then
    begin
      // Get explosion properties
      explosionRadius := sprite^.ExplosionRadius;
      explosionDamage := sprite^.ExplosionDamage;
      
      // Get explosion position
      pos := ComponentGetPosition(world, entity);
      if pos = nil then
        Continue;  // Need position for explosion
      
      // Apply explosion force to nearby entities
      // Query all entities with Position and Velocity (can be affected by explosion)
      allEntities := QueryCreate(world, [COMPONENT_POSITION, COMPONENT_VELOCITY]);
      
      while QueryNext(allEntities, affectedEntity) do
      begin
        // Don't affect the exploding entity itself
        if affectedEntity = entity then
          Continue;
        
        affectedPos := ComponentGetPosition(world, affectedEntity);
        affectedVel := ComponentGetVelocity(world, affectedEntity);
        
        if (affectedPos <> nil) and (affectedVel <> nil) then
        begin
          // Calculate distance from explosion center
          dx := Fixed16Sub(affectedPos^.X, pos^.X);
          dy := Fixed16Sub(affectedPos^.Y, pos^.Y);
          distance := Fixed16Mul(dx, dx) + Fixed16Mul(dy, dy);  // Distance squared
          
          // Check if within explosion radius
          if distance <= Fixed16Mul(explosionRadius, explosionRadius) then
          begin
            // Calculate force (inverse square law, simplified)
            // Force = (1 - distance/radius) * damage
            if explosionRadius > 0 then
            begin
              force := Fixed16Div(distance, Fixed16Mul(explosionRadius, explosionRadius));
              force := Fixed16Sub(FIXED16_ONE, force);
              force := Fixed16Mul(force, IntToFixed16(explosionDamage));
              
              // Apply force as velocity (normalized direction)
              // Normalize direction
              if distance > 0 then
              begin
                // Simplified: use dx, dy directly (not normalized, but works)
                affectedVel^.VX := Fixed16Add(affectedVel^.VX, Fixed16Mul(dx, force));
                affectedVel^.VY := Fixed16Add(affectedVel^.VY, Fixed16Mul(dy, force));
              end;
            end;
          end;
        end;
      end;
      
      // TODO: Create particle effects for explosion
      // TODO: Apply damage to entities with health component
      // TODO: Remove or mark exploding entity as destroyed
    end;
  end;
end;

// ============================================================================
// Collision System
// ============================================================================

// Collision System: Detects collisions between entities using TShape component
// Uses Collision_* functions to detect collisions and trigger bounce physics
procedure CollisionSystem(var world: TWorld);
var
  query1, query2: TQuery;
  entity1, entity2: TEntity;
  pos1, pos2: ^TPosition;
  vel1, vel2: ^TVelocity;
  material1, material2: ^TMaterialProperties;
  shape1, shape2: ^TShape;
  physicsObj1, physicsObj2: TPhysicsObject;
  fragments1, fragments2: TFragmentArray;
  normalX, normalY: Fixed16;
  dx, dy: Fixed16;
  distance: Fixed16;
  circle1, circle2: TCircle;
  collisionDetected: boolean;
  i: integer;
begin
  // Query entities with Position, Velocity, Material, and Shape components
  query1 := QueryCreate(world, [COMPONENT_POSITION, COMPONENT_VELOCITY, COMPONENT_MATERIAL, COMPONENT_SHAPE]);
  
  while QueryNext(query1, entity1) do
  begin
    pos1 := ComponentGetPosition(world, entity1);
    vel1 := ComponentGetVelocity(world, entity1);
    material1 := ComponentGetMaterial(world, entity1);
    shape1 := ComponentGetShape(world, entity1);
    
    if (pos1 = nil) or (vel1 = nil) or (material1 = nil) or (shape1 = nil) then
      Continue;
    
    // Convert entity1 to TPhysicsObject
    physicsObj1.X := pos1^.X;
    physicsObj1.Y := pos1^.Y;
    physicsObj1.VX := vel1^.VX;
    physicsObj1.VY := vel1^.VY;
    physicsObj1.AX := 0;
    physicsObj1.AY := 0;
    physicsObj1.Bouncability := material1^.Bouncability;
    physicsObj1.Solidness := material1^.Solidness;
    physicsObj1.Brittleness := material1^.Brittleness;
    physicsObj1.Fragmentability := material1^.Fragmentability;
    physicsObj1.ShapeType := shape1^.ShapeType;
    physicsObj1.CircleRadius := shape1^.CircleRadius;
    physicsObj1.PolygonArea := shape1^.PolygonArea;
    physicsObj1.Polygon.Count := shape1^.Polygon.Count;
    for i := 0 to shape1^.Polygon.Count - 1 do
    begin
      physicsObj1.Polygon.Points[i].X := shape1^.Polygon.Points[i].X;
      physicsObj1.Polygon.Points[i].Y := shape1^.Polygon.Points[i].Y;
    end;
    
    // Check collision with other entities
    query2 := QueryCreate(world, [COMPONENT_POSITION, COMPONENT_VELOCITY, COMPONENT_MATERIAL, COMPONENT_SHAPE]);
    
    while QueryNext(query2, entity2) do
    begin
      // Don't check collision with self
      if entity1 = entity2 then
        Continue;
      
      pos2 := ComponentGetPosition(world, entity2);
      vel2 := ComponentGetVelocity(world, entity2);
      material2 := ComponentGetMaterial(world, entity2);
      shape2 := ComponentGetShape(world, entity2);
      
      if (pos2 = nil) or (vel2 = nil) or (material2 = nil) or (shape2 = nil) then
        Continue;
      
      collisionDetected := False;
      normalX := 0;
      normalY := 0;
      
      // Detect collision based on shape types
      if (shape1^.ShapeType = stCircle) and (shape2^.ShapeType = stCircle) then
      begin
        // Circle-Circle collision
        circle1.X := Fixed16ToInt(pos1^.X);
        circle1.Y := Fixed16ToInt(pos1^.Y);
        circle1.Radius := Fixed16ToInt(shape1^.CircleRadius);
        
        circle2.X := Fixed16ToInt(pos2^.X);
        circle2.Y := Fixed16ToInt(pos2^.Y);
        circle2.Radius := Fixed16ToInt(shape2^.CircleRadius);
        
        collisionDetected := CircleCollision(circle1, circle2);
        
        if collisionDetected then
        begin
          // Calculate collision normal (direction from entity1 to entity2)
          dx := Fixed16Sub(pos2^.X, pos1^.X);
          dy := Fixed16Sub(pos2^.Y, pos1^.Y);
          distance := Fixed16Mul(dx, dx) + Fixed16Mul(dy, dy);
          
          if distance > 0 then
          begin
            // Normalize normal vector
            distance := Fixed16Sqrt(distance);  // Note: Fixed16Sqrt might not exist, use approximation
            if distance > 0 then
            begin
              normalX := Fixed16Div(dx, distance);
              normalY := Fixed16Div(dy, distance);
            end;
          end;
        end;
      end
      else if (shape1^.ShapeType = stPolygon) or (shape2^.ShapeType = stPolygon) then
      begin
        // Polygon collision (simplified - use AABB for now)
        // TODO: Full polygon-polygon collision detection
        // For now, use circle approximation
        circle1.X := Fixed16ToInt(pos1^.X);
        circle1.Y := Fixed16ToInt(pos1^.Y);
        circle1.Radius := Fixed16ToInt(shape1^.CircleRadius);
        if circle1.Radius = 0 then circle1.Radius := 16;
        
        circle2.X := Fixed16ToInt(pos2^.X);
        circle2.Y := Fixed16ToInt(pos2^.Y);
        circle2.Radius := Fixed16ToInt(shape2^.CircleRadius);
        if circle2.Radius = 0 then circle2.Radius := 16;
        
        collisionDetected := CircleCollision(circle1, circle2);
        
        if collisionDetected then
        begin
          dx := Fixed16Sub(pos2^.X, pos1^.X);
          dy := Fixed16Sub(pos2^.Y, pos1^.Y);
          distance := Fixed16Mul(dx, dx) + Fixed16Mul(dy, dy);
          
          if distance > 0 then
          begin
            distance := Fixed16Sqrt(distance);
            if distance > 0 then
            begin
              normalX := Fixed16Div(dx, distance);
              normalY := Fixed16Div(dy, distance);
            end;
          end;
        end;
      end;
      
      // If collision detected, apply bounce physics
      if collisionDetected then
      begin
        // Convert entity2 to TPhysicsObject
        physicsObj2.X := pos2^.X;
        physicsObj2.Y := pos2^.Y;
        physicsObj2.VX := vel2^.VX;
        physicsObj2.VY := vel2^.VY;
        physicsObj2.AX := 0;
        physicsObj2.AY := 0;
        physicsObj2.Bouncability := material2^.Bouncability;
        physicsObj2.Solidness := material2^.Solidness;
        physicsObj2.Brittleness := material2^.Brittleness;
        physicsObj2.Fragmentability := material2^.Fragmentability;
        physicsObj2.ShapeType := shape2^.ShapeType;
        physicsObj2.CircleRadius := shape2^.CircleRadius;
        physicsObj2.PolygonArea := shape2^.PolygonArea;
        physicsObj2.Polygon.Count := shape2^.Polygon.Count;
        for i := 0 to shape2^.Polygon.Count - 1 do
        begin
          physicsObj2.Polygon.Points[i].X := shape2^.Polygon.Points[i].X;
          physicsObj2.Polygon.Points[i].Y := shape2^.Polygon.Points[i].Y;
        end;
        
        // Apply bounce physics
        BounceObjects(physicsObj1, physicsObj2, normalX, normalY, fragments1, fragments2);
        
        // Update ECS components with bounced physics objects
        vel1^.VX := physicsObj1.VX;
        vel1^.VY := physicsObj1.VY;
        vel2^.VX := physicsObj2.VX;
        vel2^.VY := physicsObj2.VY;
        
        // TODO: Handle fragments (create new entities for fragments)
        // For now, fragments are ignored
      end;
    end;
  end;
end;

end.

