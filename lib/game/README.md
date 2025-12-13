# SuperPascal Game Library

**Status:** ✅ Complete  
**Priority:** ⭐⭐⭐ MEDIUM  
**Target:** All platforms (8-bit through 64-bit)

---

## Overview

Game development algorithms for camera systems, spatial organization, line of sight, and pathfinding. These algorithms are **generic** and work on **all platforms**.

**Key Features:**
- **Camera Systems:** 3D camera with matrix-based transformations
- **BSP Trees:** Binary Space Partitioning for spatial organization and rendering
- **Line of Sight:** Fast LOS calculation for tile-based games
- **Pathfinding:** A* and Dijkstra algorithms for AI navigation

---

## Module Structure

```
lib/game/
├── mod.pas          # Main entry point
├── types.pas        # Core types (TCamera, TBSPTree, TTileMap, TPath)
├── camera.pas       # Camera systems and transformations
├── bsp.pas          # BSP tree construction and traversal
├── los.pas          # Line of sight algorithms
├── pathfinding.pas  # A* and Dijkstra pathfinding
└── README.md        # This file
```

---

## Quick Start

### Camera System

```pascal
uses Game, Math_Types;

var
  camera: TCamera;
  viewMatrix: TMatrix4x4;
  worldPoint, cameraPoint: TVector3;
begin
  // Create camera at origin
  camera := CameraCreate;
  
  // Set camera position
  CameraSetPosition(camera, IntToFixed16(0), IntToFixed16(0), IntToFixed16(10));
  
  // Set camera rotation (pitch, yaw, roll in degrees)
  CameraSetRotation(camera, IntToFixed16(0), IntToFixed16(0), IntToFixed16(0));
  
  // Get view matrix for rendering
  viewMatrix := CameraGetViewMatrix(camera);
  
  // Transform world point to camera space
  worldPoint.X := IntToFixed16(5);
  worldPoint.Y := IntToFixed16(0);
  worldPoint.Z := IntToFixed16(0);
  cameraPoint := CameraTransformPoint(camera, worldPoint);
  
  // Check if point is visible
  if CameraIsPointVisible(camera, worldPoint) then
    WriteLn('Point is in front of camera');
end.
```

### Line of Sight

```pascal
uses Game;

var
  map: TTileMap;
  visible: boolean;
  losResult: TLOSResult;
begin
  // Initialize tile map (10x10 grid)
  map.Width := 10;
  map.Height := 10;
  SetLength(map.Tiles, 10);
  for i := 0 to 9 do
  begin
    SetLength(map.Tiles[i], 10);
    for j := 0 to 9 do
      map.Tiles[i][j] := False;  // False = passable
  end;
  
  // Add some walls (blocking tiles)
  map.Tiles[5][5] := True;  // Wall at (5, 5)
  
  // Check line of sight from (0, 0) to (9, 9)
  visible := CheckLineOfSight(map, 0, 0, 9, 9);
  
  if visible then
    WriteLn('Clear line of sight')
  else
    WriteLn('Line of sight blocked');
  
  // Get detailed LOS result
  CheckLineOfSightDetailed(map, 0, 0, 9, 9, losResult);
  WriteLn('Distance: ', Fixed16ToInt(losResult.Distance));
  WriteLn('Blocked at: (', losResult.BlockedAt.X, ', ', losResult.BlockedAt.Y, ')');
end.
```

### Pathfinding

```pascal
uses Game;

var
  map: TTileMap;
  path: TPath;
  i: integer;
begin
  // Initialize tile map (same as LOS example)
  // ...
  
  // Find path from (0, 0) to (9, 9) using A*
  if FindPathAStar(map, 0, 0, 9, 9, path) then
  begin
    WriteLn('Path found! Length: ', path.Count);
    for i := 0 to path.Count - 1 do
      WriteLn('Step ', i, ': (', path.Nodes[i].X, ', ', path.Nodes[i].Y, ')');
  end
  else
    WriteLn('No path found');
end.
```

### BSP Trees

```pascal
uses Game, Math_Types;

var
  tree: TBSPTree;
  node: TBSPNode;
  planeNormal: TVector3;
  cameraPos: TVector3;
  resultNodes: array[0..255] of TBSPNode;
  resultCount: integer;
begin
  // Create BSP tree
  tree := BSPCreate;
  
  // Create a node with a plane
  planeNormal.X := IntToFixed16(0);
  planeNormal.Y := IntToFixed16(1);
  planeNormal.Z := IntToFixed16(0);  // Y-axis normal (horizontal plane)
  
  node := BSPNodeCreate(planeNormal, IntToFixed16(0));  // Plane at Y=0
  
  // Add node to tree
  BSPAddNode(tree, node);
  
  // Query tree for back-to-front rendering
  cameraPos.X := IntToFixed16(0);
  cameraPos.Y := IntToFixed16(5);
  cameraPos.Z := IntToFixed16(10);
  
  BSPQueryBackToFront(tree, cameraPos, resultNodes, resultCount);
  
  WriteLn('Nodes to render: ', resultCount);
  
  // Free tree when done
  BSPFree(tree);
end.
```

---

## API Reference

### Camera Functions

| Function | Description |
|----------|-------------|
| `CameraCreate` | Create camera at origin |
| `CameraCreateAt(x, y, z)` | Create camera at specified position |
| `CameraCreateLookAt(pos, target, up)` | Create camera looking at target |
| `CameraSetPosition(camera, x, y, z)` | Set camera position |
| `CameraSetRotation(camera, pitch, yaw, roll)` | Set camera rotation (Euler angles) |
| `CameraLookAt(camera, target, up)` | Make camera look at target |
| `CameraGetViewMatrix(camera)` | Get 4x4 view matrix for rendering |
| `CameraTransformPoint(camera, point)` | Transform world point to camera space |
| `CameraIsPointVisible(camera, point)` | Check if point is in front of camera |

### Line of Sight Functions

| Function | Description |
|----------|-------------|
| `CheckLineOfSight(map, x1, y1, x2, y2)` | Check if clear path exists (boolean) |
| `CheckLineOfSightDetailed(map, x1, y1, x2, y2, result)` | Get detailed LOS result |
| `CalculateVisibilityField(map, centerX, centerY, radius, visible)` | Calculate visibility field |

### Pathfinding Functions

| Function | Description |
|----------|-------------|
| `FindPathAStar(map, startX, startY, goalX, goalY, path)` | Find path using A* algorithm |
| `FindPathDijkstra(map, startX, startY, goalX, goalY, path)` | Find path using Dijkstra's algorithm |
| `HeuristicManhattan(x1, y1, x2, y2)` | Calculate Manhattan distance |
| `HeuristicEuclidean(x1, y1, x2, y2)` | Calculate Euclidean distance |
| `IsValidPosition(map, x, y)` | Check if position is valid |

### BSP Tree Functions

| Function | Description |
|----------|-------------|
| `BSPCreate` | Create empty BSP tree |
| `BSPNodeCreate(planeNormal, planeDistance)` | Create BSP node |
| `BSPAddNode(tree, node)` | Add node to tree |
| `BSPBuildFromPolygons(tree, polygons, count)` | Build tree from polygon list |
| `BSPClassifyPoint(planeNormal, planeDistance, point)` | Classify point relative to plane |
| `BSPBoxIntersectsPlane(boxMin, boxMax, planeNormal, planeDistance)` | Check box-plane intersection |
| `BSPQueryBackToFront(tree, cameraPos, resultNodes, resultCount)` | Query for back-to-front rendering |
| `BSPQueryFrontToBack(tree, cameraPos, resultNodes, resultCount)` | Query for front-to-back culling |
| `BSPTraverse(tree, callback, userData)` | Traverse tree with callback |
| `BSPFree(tree)` | Free tree and all nodes |

---

## Algorithm Details

### Camera System

The camera system uses a **3x3 matrix for orientation** and a **3D vector for position**, following the approach described in the Mikro archive (`CAM_MATR.TXT`). This representation allows efficient transformation from world space to camera space:

```
C = M * (W - P)
```

Where:
- `C` = Camera space coordinates
- `M` = Camera matrix (3x3 orientation)
- `W` = World space coordinates
- `P` = Camera position

### Line of Sight

Uses **Bresenham's line algorithm** to trace a line from start to goal, checking for blocking tiles. This is efficient for tile-based games (rogue-likes) and provides O(n) performance where n is the line length.

For advanced lighting/shading, see the Mikro archive `LOS.TXT` which describes a shadow table algorithm that can calculate visibility fields in constant time.

### Pathfinding

**A* algorithm** uses:
- **G cost:** Actual cost from start to current node
- **H cost:** Heuristic estimate from current node to goal (Manhattan distance)
- **F cost:** Total cost (G + H)

The algorithm maintains an open list of nodes to explore and a closed list of nodes already processed. It guarantees finding the shortest path if the heuristic is admissible (never overestimates).

**Dijkstra's algorithm** is A* with H=0 (uniform cost search).

### BSP Trees

**Binary Space Partitioning** divides space using planes, creating a binary tree structure. Each node represents a plane that splits space into "front" and "back" half-spaces.

**Uses:**
- **Back-to-front rendering:** Render polygons in correct order without Z-buffer
- **Spatial culling:** Quickly discard entire subtrees
- **Collision detection:** Efficient spatial queries

**Construction:** Select optimal splitting planes to minimize polygon splits and balance the tree.

---

## Platform Considerations

### Memory Usage

- **Camera:** ~64 bytes per camera (3x3 matrix + position + FOV)
- **BSP Tree:** Variable (depends on scene complexity)
- **Pathfinding:** O(n²) memory for node grid (n = map size)
- **LOS:** O(1) per query, O(n²) for visibility field

### Performance

- **Camera transformations:** O(1) per point
- **LOS:** O(n) where n = line length
- **A* pathfinding:** O(n log n) where n = number of nodes
- **BSP traversal:** O(log n) where n = number of nodes

### Fixed-Point Math

All algorithms use **Q8.8 fixed-point** format (`Fixed16`) for consistency across platforms. This ensures:
- No floating-point unit required
- Consistent behavior on all platforms
- Acceptable precision for game applications

---

## Source Material

- **Camera Systems:** `docs/mikro_docs_archive/Coding/1/CAM_MATR.TXT`
- **BSP Trees:** `docs/mikro_docs_archive/Coding/1/BSPTREE.TXT`, `Coding/2/BSP_extracted/BSP/`
- **Line of Sight:** `docs/mikro_docs_archive/Coding/2/LOS.TXT`
- **Pathfinding:** Standard A* algorithm (well-documented)

---

## Future Enhancements

- [ ] Full BSP tree construction from polygon lists
- [ ] Optimal plane selection algorithms
- [ ] Shadow table algorithm for LOS (from `LOS.TXT`)
- [ ] Hierarchical pathfinding (HPA*)
- [ ] Jump point search for uniform-cost grids
- [ ] Camera frustum culling
- [ ] View frustum calculations

---

**Last Updated:** 2025-01-XX  
**Status:** Complete - Ready for use

