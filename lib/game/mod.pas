unit Game;

interface

// Game development library
// Source: algorithms/08_GameAlgorithms.md
//
// Provides camera systems, BSP trees, line of sight, and pathfinding
// for game development

uses
  Game_Types,
  Game_Camera,
  Game_BSP,
  Game_LOS,
  Game_Pathfinding;

// Re-export types
type
  TCamera = Game_Types.TCamera;
  TBSPNode = Game_Types.TBSPNode;
  TBSPTree = Game_Types.TBSPTree;
  TTileMap = Game_Types.TTileMap;
  TLOSResult = Game_Types.TLOSResult;
  TPathNode = Game_Types.TPathNode;
  TPath = Game_Types.TPath;

// Re-export camera functions
function CameraCreate: TCamera;
function CameraCreateAt(x, y, z: Fixed16): TCamera;
function CameraCreateLookAt(
  posX, posY, posZ: Fixed16;
  targetX, targetY, targetZ: Fixed16;
  upX, upY, upZ: Fixed16
): TCamera;
procedure CameraSetPosition(var camera: TCamera; x, y, z: Fixed16);
procedure CameraSetRotation(var camera: TCamera; pitch, yaw, roll: Fixed16);
procedure CameraLookAt(
  var camera: TCamera;
  targetX, targetY, targetZ: Fixed16;
  upX, upY, upZ: Fixed16
);
function CameraGetViewMatrix(const camera: TCamera): TMatrix4x4;
function CameraTransformPoint(const camera: TCamera; const worldPoint: TVector3): TVector3;
function CameraIsPointVisible(const camera: TCamera; const worldPoint: TVector3): boolean;

// Re-export BSP functions
function BSPCreate: TBSPTree;
function BSPNodeCreate(planeNormal: TVector3; planeDistance: Fixed16): TBSPNode;
procedure BSPAddNode(var tree: TBSPTree; node: TBSPNode);
procedure BSPBuildFromPolygons(var tree: TBSPTree; polygons: Pointer; count: integer);
function BSPClassifyPoint(
  const planeNormal: TVector3;
  planeDistance: Fixed16;
  const point: TVector3
): integer;
function BSPBoxIntersectsPlane(
  const boxMin, boxMax: TVector3;
  const planeNormal: TVector3;
  planeDistance: Fixed16
): boolean;
procedure BSPQueryBackToFront(
  const tree: TBSPTree;
  const cameraPos: TVector3;
  var resultNodes: array of TBSPNode;
  var resultCount: integer
);
procedure BSPQueryFrontToBack(
  const tree: TBSPTree;
  const cameraPos: TVector3;
  var resultNodes: array of TBSPNode;
  var resultCount: integer
);
procedure BSPTraverse(
  const tree: TBSPTree;
  callback: TBSPTraverseCallback;
  userData: Pointer
);
procedure BSPFree(var tree: TBSPTree);

// Re-export LOS functions
function CheckLineOfSight(
  const map: TTileMap;
  x1, y1, x2, y2: integer
): boolean;
function CheckLineOfSightDetailed(
  const map: TTileMap;
  x1, y1, x2, y2: integer;
  var result: TLOSResult
): boolean;
procedure CalculateVisibilityField(
  const map: TTileMap;
  centerX, centerY: integer;
  radius: integer;
  var visible: array of array of boolean
);

// Re-export pathfinding functions
function FindPathAStar(
  const map: TTileMap;
  startX, startY: integer;
  goalX, goalY: integer;
  var path: TPath
): boolean;
function FindPathDijkstra(
  const map: TTileMap;
  startX, startY: integer;
  goalX, goalY: integer;
  var path: TPath
): boolean;
function HeuristicManhattan(x1, y1, x2, y2: integer): Fixed16;
function HeuristicEuclidean(x1, y1, x2, y2: integer): Fixed16;
function IsValidPosition(const map: TTileMap; x, y: integer): boolean;

implementation

// All implementations are in their respective units
// This module just re-exports them for convenience

function CameraCreate: TCamera;
begin
  Result := Game_Camera.CameraCreate;
end;

function CameraCreateAt(x, y, z: Fixed16): TCamera;
begin
  Result := Game_Camera.CameraCreateAt(x, y, z);
end;

function CameraCreateLookAt(
  posX, posY, posZ: Fixed16;
  targetX, targetY, targetZ: Fixed16;
  upX, upY, upZ: Fixed16
): TCamera;
begin
  Result := Game_Camera.CameraCreateLookAt(posX, posY, posZ, targetX, targetY, targetZ, upX, upY, upZ);
end;

procedure CameraSetPosition(var camera: TCamera; x, y, z: Fixed16);
begin
  Game_Camera.CameraSetPosition(camera, x, y, z);
end;

procedure CameraSetRotation(var camera: TCamera; pitch, yaw, roll: Fixed16);
begin
  Game_Camera.CameraSetRotation(camera, pitch, yaw, roll);
end;

procedure CameraLookAt(
  var camera: TCamera;
  targetX, targetY, targetZ: Fixed16;
  upX, upY, upZ: Fixed16
);
begin
  Game_Camera.CameraLookAt(camera, targetX, targetY, targetZ, upX, upY, upZ);
end;

function CameraGetViewMatrix(const camera: TCamera): TMatrix4x4;
begin
  Result := Game_Camera.CameraGetViewMatrix(camera);
end;

function CameraTransformPoint(const camera: TCamera; const worldPoint: TVector3): TVector3;
begin
  Result := Game_Camera.CameraTransformPoint(camera, worldPoint);
end;

function CameraIsPointVisible(const camera: TCamera; const worldPoint: TVector3): boolean;
begin
  Result := Game_Camera.CameraIsPointVisible(camera, worldPoint);
end;

function BSPCreate: TBSPTree;
begin
  Result := Game_BSP.BSPCreate;
end;

function BSPNodeCreate(planeNormal: TVector3; planeDistance: Fixed16): TBSPNode;
begin
  Result := Game_BSP.BSPNodeCreate(planeNormal, planeDistance);
end;

procedure BSPAddNode(var tree: TBSPTree; node: TBSPNode);
begin
  Game_BSP.BSPAddNode(tree, node);
end;

procedure BSPBuildFromPolygons(var tree: TBSPTree; polygons: Pointer; count: integer);
begin
  Game_BSP.BSPBuildFromPolygons(tree, polygons, count);
end;

function BSPClassifyPoint(
  const planeNormal: TVector3;
  planeDistance: Fixed16;
  const point: TVector3
): integer;
begin
  Result := Game_BSP.BSPClassifyPoint(planeNormal, planeDistance, point);
end;

function BSPBoxIntersectsPlane(
  const boxMin, boxMax: TVector3;
  const planeNormal: TVector3;
  planeDistance: Fixed16
): boolean;
begin
  Result := Game_BSP.BSPBoxIntersectsPlane(boxMin, boxMax, planeNormal, planeDistance);
end;

procedure BSPQueryBackToFront(
  const tree: TBSPTree;
  const cameraPos: TVector3;
  var resultNodes: array of TBSPNode;
  var resultCount: integer
);
begin
  Game_BSP.BSPQueryBackToFront(tree, cameraPos, resultNodes, resultCount);
end;

procedure BSPQueryFrontToBack(
  const tree: TBSPTree;
  const cameraPos: TVector3;
  var resultNodes: array of TBSPNode;
  var resultCount: integer
);
begin
  Game_BSP.BSPQueryFrontToBack(tree, cameraPos, resultNodes, resultCount);
end;

procedure BSPTraverse(
  const tree: TBSPTree;
  callback: TBSPTraverseCallback;
  userData: Pointer
);
begin
  Game_BSP.BSPTraverse(tree, callback, userData);
end;

procedure BSPFree(var tree: TBSPTree);
begin
  Game_BSP.BSPFree(tree);
end;

function CheckLineOfSight(
  const map: TTileMap;
  x1, y1, x2, y2: integer
): boolean;
begin
  Result := Game_LOS.CheckLineOfSight(map, x1, y1, x2, y2);
end;

function CheckLineOfSightDetailed(
  const map: TTileMap;
  x1, y1, x2, y2: integer;
  var result: TLOSResult
): boolean;
begin
  Result := Game_LOS.CheckLineOfSightDetailed(map, x1, y1, x2, y2, result);
end;

procedure CalculateVisibilityField(
  const map: TTileMap;
  centerX, centerY: integer;
  radius: integer;
  var visible: array of array of boolean
);
begin
  Game_LOS.CalculateVisibilityField(map, centerX, centerY, radius, visible);
end;

function FindPathAStar(
  const map: TTileMap;
  startX, startY: integer;
  goalX, goalY: integer;
  var path: TPath
): boolean;
begin
  Result := Game_Pathfinding.FindPathAStar(map, startX, startY, goalX, goalY, path);
end;

function FindPathDijkstra(
  const map: TTileMap;
  startX, startY: integer;
  goalX, goalY: integer;
  var path: TPath
): boolean;
begin
  Result := Game_Pathfinding.FindPathDijkstra(map, startX, startY, goalX, goalY, path);
end;

function HeuristicManhattan(x1, y1, x2, y2: integer): Fixed16;
begin
  Result := Game_Pathfinding.HeuristicManhattan(x1, y1, x2, y2);
end;

function HeuristicEuclidean(x1, y1, x2, y2: integer): Fixed16;
begin
  Result := Game_Pathfinding.HeuristicEuclidean(x1, y1, x2, y2);
end;

function IsValidPosition(const map: TTileMap; x, y: integer): boolean;
begin
  Result := Game_Pathfinding.IsValidPosition(map, x, y);
end;

end.

