unit Game_BSP;

interface

// Binary Space Partitioning (BSP) trees
// Source: algorithms/08_GameAlgorithms.md
// Based on: docs/mikro_docs_archive/Coding/1/BSPTREE.TXT
//
// BSP trees partition space using planes, enabling efficient spatial queries
// and back-to-front rendering

uses
  Game_Types,
  Math_Types,
  Math_Fixed;

// ============================================================================
// BSP Tree Construction
// ============================================================================

// Create an empty BSP tree
function BSPCreate: TBSPTree;

// Create a BSP node
function BSPNodeCreate(
  planeNormal: TVector3;
  planeDistance: Fixed16
): TBSPNode;

// Add a node to the BSP tree
procedure BSPAddNode(var tree: TBSPTree; node: TBSPNode);

// Build BSP tree from polygon list (simplified - uses first polygon as root)
// TODO: Full implementation with optimal plane selection
procedure BSPBuildFromPolygons(var tree: TBSPTree; polygons: Pointer; count: integer);

// ============================================================================
// BSP Tree Queries
// ============================================================================

// Classify point relative to plane
// Returns: -1 = behind, 0 = on plane, +1 = in front
function BSPClassifyPoint(
  const planeNormal: TVector3;
  planeDistance: Fixed16;
  const point: TVector3
): integer;

// Check if bounding box intersects with plane
function BSPBoxIntersectsPlane(
  const boxMin, boxMax: TVector3;
  const planeNormal: TVector3;
  planeDistance: Fixed16
): boolean;

// Query BSP tree for nodes in front of camera
// Returns list of nodes that should be rendered (back-to-front order)
procedure BSPQueryBackToFront(
  const tree: TBSPTree;
  const cameraPos: TVector3;
  var resultNodes: array of TBSPNode;
  var resultCount: integer
);

// Query BSP tree for nodes behind camera (front-to-back order)
procedure BSPQueryFrontToBack(
  const tree: TBSPTree;
  const cameraPos: TVector3;
  var resultNodes: array of TBSPNode;
  var resultCount: integer
);

// ============================================================================
// BSP Tree Traversal
// ============================================================================

// Traverse BSP tree and call callback for each node
type
  TBSPTraverseCallback = procedure(node: TBSPNode; userData: Pointer);

procedure BSPTraverse(
  const tree: TBSPTree;
  callback: TBSPTraverseCallback;
  userData: Pointer
);

// Traverse BSP tree recursively (internal)
procedure BSPTraverseRecursive(
  node: TBSPNode;
  callback: TBSPTraverseCallback;
  userData: Pointer
);

// ============================================================================
// BSP Tree Cleanup
// ============================================================================

// Free BSP tree and all nodes
procedure BSPFree(var tree: TBSPTree);

// Free a BSP node recursively
procedure BSPNodeFree(node: TBSPNode);

implementation

// Helper: Calculate dot product
function DotProduct(const a, b: TVector3): Fixed16;
begin
  Result := Fixed16Mul(a.X, b.X) + Fixed16Mul(a.Y, b.Y) + Fixed16Mul(a.Z, b.Z);
end;

// Create empty BSP tree
function BSPCreate: TBSPTree;
begin
  Result.Root := nil;
  Result.NodeCount := 0;
end;

// Create BSP node
function BSPNodeCreate(
  planeNormal: TVector3;
  planeDistance: Fixed16
): TBSPNode;
begin
  New(Result);
  Result^.FrontNode := nil;
  Result^.BackNode := nil;
  Result^.PlaneNormal := planeNormal;
  Result^.PlaneDistance := planeDistance;
  Result^.BoxMin.X := 0;
  Result^.BoxMin.Y := 0;
  Result^.BoxMin.Z := 0;
  Result^.BoxMax.X := 0;
  Result^.BoxMax.Y := 0;
  Result^.BoxMax.Z := 0;
  Result^.Data := nil;
end;

// Add node to tree (simple - adds as root if tree is empty)
procedure BSPAddNode(var tree: TBSPTree; node: TBSPNode);
begin
  if tree.Root = nil then
  begin
    tree.Root := node;
    tree.NodeCount := 1;
  end
  else
  begin
    // TODO: Insert node based on plane classification
    // For now, just increment count
    tree.NodeCount := tree.NodeCount + 1;
  end;
end;

// Build BSP tree from polygons (simplified)
procedure BSPBuildFromPolygons(var tree: TBSPTree; polygons: Pointer; count: integer);
begin
  // TODO: Full implementation
  // 1. Select optimal splitting plane
  // 2. Partition polygons into front/back/on-plane
  // 3. Recursively build subtrees
  // For now, this is a placeholder
  tree := BSPCreate;
end;

// Classify point relative to plane
function BSPClassifyPoint(
  const planeNormal: TVector3;
  planeDistance: Fixed16;
  const point: TVector3
): integer;
var
  distance: Fixed16;
  epsilon: Fixed16;
begin
  // Calculate signed distance from point to plane
  distance := DotProduct(planeNormal, point) - planeDistance;
  
  // Epsilon for "on plane" classification
  epsilon := IntToFixed16(1) div 256;  // Small threshold
  
  if distance < -epsilon then
    Result := -1  // Behind plane
  else if distance > epsilon then
    Result := 1    // In front of plane
  else
    Result := 0;   // On plane
end;

// Check if bounding box intersects plane
function BSPBoxIntersectsPlane(
  const boxMin, boxMax: TVector3;
  const planeNormal: TVector3;
  planeDistance: Fixed16
): boolean;
var
  minDist, maxDist: Fixed16;
  corners: array[0..7] of TVector3;
  i: integer;
  dist: Fixed16;
begin
  // Calculate box corners
  corners[0].X := boxMin.X; corners[0].Y := boxMin.Y; corners[0].Z := boxMin.Z;
  corners[1].X := boxMax.X; corners[1].Y := boxMin.Y; corners[1].Z := boxMin.Z;
  corners[2].X := boxMin.X; corners[2].Y := boxMax.Y; corners[2].Z := boxMin.Z;
  corners[3].X := boxMax.X; corners[3].Y := boxMax.Y; corners[3].Z := boxMin.Z;
  corners[4].X := boxMin.X; corners[4].Y := boxMin.Y; corners[4].Z := boxMax.Z;
  corners[5].X := boxMax.X; corners[5].Y := boxMin.Y; corners[5].Z := boxMax.Z;
  corners[6].X := boxMin.X; corners[6].Y := boxMax.Y; corners[6].Z := boxMax.Z;
  corners[7].X := boxMax.X; corners[7].Y := boxMax.Y; corners[7].Z := boxMax.Z;
  
  // Find min and max distances
  minDist := DotProduct(planeNormal, corners[0]) - planeDistance;
  maxDist := minDist;
  
  for i := 1 to 7 do
  begin
    dist := DotProduct(planeNormal, corners[i]) - planeDistance;
    if dist < minDist then minDist := dist;
    if dist > maxDist then maxDist := dist;
  end;
  
  // Box intersects plane if min and max distances have opposite signs
  Result := (minDist <= 0) and (maxDist >= 0);
end;

// Query BSP tree back-to-front (for rendering)
procedure BSPQueryBackToFront(
  const tree: TBSPTree;
  const cameraPos: TVector3;
  var resultNodes: array of TBSPNode;
  var resultCount: integer
);
var
  stack: array[0..255] of TBSPNode;  // Stack for traversal
  stackTop: integer;
  node: TBSPNode;
  classification: integer;
begin
  resultCount := 0;
  stackTop := 0;
  
  if tree.Root = nil then
    Exit;
  
  // Push root onto stack
  stack[stackTop] := tree.Root;
  stackTop := stackTop + 1;
  
  while stackTop > 0 do
  begin
    // Pop node from stack
    stackTop := stackTop - 1;
    node := stack[stackTop];
    
    if node = nil then
      Continue;
    
    // Classify camera position relative to node's plane
    classification := BSPClassifyPoint(
      node^.PlaneNormal,
      node^.PlaneDistance,
      cameraPos
    );
    
    if classification >= 0 then
    begin
      // Camera in front or on plane: render back first, then front
      if node^.BackNode <> nil then
      begin
        stack[stackTop] := node^.BackNode;
        stackTop := stackTop + 1;
      end;
      
      // Add current node to result
      if resultCount < Length(resultNodes) then
      begin
        resultNodes[resultCount] := node;
        resultCount := resultCount + 1;
      end;
      
      if node^.FrontNode <> nil then
      begin
        stack[stackTop] := node^.FrontNode;
        stackTop := stackTop + 1;
      end;
    end
    else
    begin
      // Camera behind plane: render front first, then back
      if node^.FrontNode <> nil then
      begin
        stack[stackTop] := node^.FrontNode;
        stackTop := stackTop + 1;
      end;
      
      // Add current node to result
      if resultCount < Length(resultNodes) then
      begin
        resultNodes[resultCount] := node;
        resultCount := resultCount + 1;
      end;
      
      if node^.BackNode <> nil then
      begin
        stack[stackTop] := node^.BackNode;
        stackTop := stackTop + 1;
      end;
    end;
  end;
end;

// Query BSP tree front-to-back (for culling)
procedure BSPQueryFrontToBack(
  const tree: TBSPTree;
  const cameraPos: TVector3;
  var resultNodes: array of TBSPNode;
  var resultCount: integer
);
var
  stack: array[0..255] of TBSPNode;
  stackTop: integer;
  node: TBSPNode;
  classification: integer;
begin
  resultCount := 0;
  stackTop := 0;
  
  if tree.Root = nil then
    Exit;
  
  stack[stackTop] := tree.Root;
  stackTop := stackTop + 1;
  
  while stackTop > 0 do
  begin
    stackTop := stackTop - 1;
    node := stack[stackTop];
    
    if node = nil then
      Continue;
    
    classification := BSPClassifyPoint(
      node^.PlaneNormal,
      node^.PlaneDistance,
      cameraPos
    );
    
    if classification >= 0 then
    begin
      // Camera in front: process front first
      if node^.FrontNode <> nil then
      begin
        stack[stackTop] := node^.FrontNode;
        stackTop := stackTop + 1;
      end;
      
      if resultCount < Length(resultNodes) then
      begin
        resultNodes[resultCount] := node;
        resultCount := resultCount + 1;
      end;
      
      if node^.BackNode <> nil then
      begin
        stack[stackTop] := node^.BackNode;
        stackTop := stackTop + 1;
      end;
    end
    else
    begin
      // Camera behind: process back first
      if node^.BackNode <> nil then
      begin
        stack[stackTop] := node^.BackNode;
        stackTop := stackTop + 1;
      end;
      
      if resultCount < Length(resultNodes) then
      begin
        resultNodes[resultCount] := node;
        resultCount := resultCount + 1;
      end;
      
      if node^.FrontNode <> nil then
      begin
        stack[stackTop] := node^.FrontNode;
        stackTop := stackTop + 1;
      end;
    end;
  end;
end;

// Traverse BSP tree
procedure BSPTraverse(
  const tree: TBSPTree;
  callback: TBSPTraverseCallback;
  userData: Pointer
);
begin
  if tree.Root <> nil then
    BSPTraverseRecursive(tree.Root, callback, userData);
end;

// Traverse recursively
procedure BSPTraverseRecursive(
  node: TBSPNode;
  callback: TBSPTraverseCallback;
  userData: Pointer
);
begin
  if node = nil then
    Exit;
  
  // Call callback for current node
  callback(node, userData);
  
  // Traverse children
  if node^.FrontNode <> nil then
    BSPTraverseRecursive(node^.FrontNode, callback, userData);
  
  if node^.BackNode <> nil then
    BSPTraverseRecursive(node^.BackNode, callback, userData);
end;

// Free BSP tree
procedure BSPFree(var tree: TBSPTree);
begin
  if tree.Root <> nil then
  begin
    BSPNodeFree(tree.Root);
    tree.Root := nil;
  end;
  tree.NodeCount := 0;
end;

// Free BSP node recursively
procedure BSPNodeFree(node: TBSPNode);
begin
  if node = nil then
    Exit;
  
  if node^.FrontNode <> nil then
    BSPNodeFree(node^.FrontNode);
  
  if node^.BackNode <> nil then
    BSPNodeFree(node^.BackNode);
  
  Dispose(node);
end;

end.

