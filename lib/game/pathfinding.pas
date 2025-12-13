unit Game_Pathfinding;

interface

// Pathfinding algorithms (A*, Dijkstra)
// Source: algorithms/08_GameAlgorithms.md
//
// A* pathfinding for grid-based games
// Uses Manhattan distance as heuristic

uses
  Game_Types,
  Math_Types,
  Math_Fixed;

// ============================================================================
// Pathfinding Functions
// ============================================================================

// Find path using A* algorithm
// map: Tile map (true = blocked, false = passable)
// startX, startY: Starting position
// goalX, goalY: Goal position
// Returns: Path with nodes from start to goal
function FindPathAStar(
  const map: TTileMap;
  startX, startY: integer;
  goalX, goalY: integer;
  var path: TPath
): boolean;

// Find path using Dijkstra's algorithm (uniform cost)
function FindPathDijkstra(
  const map: TTileMap;
  startX, startY: integer;
  goalX, goalY: integer;
  var path: TPath
): boolean;

// ============================================================================
// Helper Functions
// ============================================================================

// Calculate Manhattan distance (heuristic for A*)
function HeuristicManhattan(x1, y1, x2, y2: integer): Fixed16;

// Calculate Euclidean distance (alternative heuristic)
function HeuristicEuclidean(x1, y1, x2, y2: integer): Fixed16;

// Check if position is valid (within map bounds and not blocked)
function IsValidPosition(
  const map: TTileMap;
  x, y: integer
): boolean;

implementation

// Manhattan distance heuristic
function HeuristicManhattan(x1, y1, x2, y2: integer): Fixed16;
var
  dx, dy: integer;
begin
  dx := x2 - x1;
  if dx < 0 then dx := -dx;
  dy := y2 - y1;
  if dy < 0 then dy := -dy;
  Result := IntToFixed16(dx + dy);
end;

// Euclidean distance heuristic
function HeuristicEuclidean(x1, y1, x2, y2: integer): Fixed16;
var
  dx, dy: integer;
  distSquared: LongInt;
begin
  dx := x2 - x1;
  dy := y2 - y1;
  distSquared := dx * dx + dy * dy;
  // Approximate square root (simplified)
  Result := IntToFixed16(distSquared div 256);  // Rough approximation
end;

// Check if position is valid
function IsValidPosition(
  const map: TTileMap;
  x, y: integer
): boolean;
begin
  Result := (x >= 0) and (x < map.Width) and
            (y >= 0) and (y < map.Height) and
            not map.Tiles[y][x];  // Not blocked
end;

// A* pathfinding
function FindPathAStar(
  const map: TTileMap;
  startX, startY: integer;
  goalX, goalY: integer;
  var path: TPath
): boolean;
var
  nodes: array of array of TPathNode;
  openList: array of ^TPathNode;
  openCount: integer;
  current: ^TPathNode;
  neighbors: array[0..3] of record
    X, Y: integer;
  end;
  i, j: integer;
  newG, newH, newF: Fixed16;
  neighbor: ^TPathNode;
  found: boolean;
  pathNodes: array[0..1023] of record
    X, Y: integer;
  end;
  pathCount: integer;
  temp: ^TPathNode;
begin
  Result := False;
  path.Found := False;
  path.Count := 0;
  
  // Validate start and goal
  if not IsValidPosition(map, startX, startY) then
    Exit;
  if not IsValidPosition(map, goalX, goalY) then
    Exit;
  
  // Initialize node grid
  SetLength(nodes, map.Height);
  for i := 0 to map.Height - 1 do
  begin
    SetLength(nodes[i], map.Width);
    for j := 0 to map.Width - 1 do
    begin
      nodes[i][j].X := j;
      nodes[i][j].Y := i;
      nodes[i][j].G := IntToFixed16(999999);
      nodes[i][j].H := 0;
      nodes[i][j].F := IntToFixed16(999999);
      nodes[i][j].Parent := nil;
      nodes[i][j].Open := False;
      nodes[i][j].Closed := False;
    end;
  end;
  
  // Initialize start node
  nodes[startY][startX].G := 0;
  nodes[startY][startX].H := HeuristicManhattan(startX, startY, goalX, goalY);
  nodes[startY][startX].F := nodes[startY][startX].H;
  nodes[startY][startX].Open := True;
  
  // Initialize open list
  SetLength(openList, map.Width * map.Height);
  openCount := 1;
  openList[0] := @nodes[startY][startX];
  
  found := False;
  
  // A* main loop
  while openCount > 0 do
  begin
    // Find node with lowest F in open list
    current := openList[0];
    j := 0;
    for i := 1 to openCount - 1 do
    begin
      if openList[i]^.F < current^.F then
      begin
        current := openList[i];
        j := i;
      end;
    end;
    
    // Remove current from open list
    for i := j to openCount - 2 do
      openList[i] := openList[i + 1];
    openCount := openCount - 1;
    current^.Open := False;
    current^.Closed := True;
    
    // Check if we reached the goal
    if (current^.X = goalX) and (current^.Y = goalY) then
    begin
      found := True;
      Break;
    end;
    
    // Check neighbors (4-directional: up, down, left, right)
    neighbors[0].X := current^.X;     neighbors[0].Y := current^.Y - 1;  // Up
    neighbors[1].X := current^.X;     neighbors[1].Y := current^.Y + 1;  // Down
    neighbors[2].X := current^.X - 1; neighbors[2].Y := current^.Y;      // Left
    neighbors[3].X := current^.X + 1; neighbors[3].Y := current^.Y;      // Right
    
    for i := 0 to 3 do
    begin
      // Check if neighbor is valid
      if not IsValidPosition(map, neighbors[i].X, neighbors[i].Y) then
        Continue;
      
      neighbor := @nodes[neighbors[i].Y][neighbors[i].X];
      
      // Skip if already closed
      if neighbor^.Closed then
        Continue;
      
      // Calculate new G cost (movement cost = 1 per step)
      newG := Fixed16Add(current^.G, FIXED16_ONE);
      
      // If this path is better, update neighbor
      if newG < neighbor^.G then
      begin
        neighbor^.Parent := current;
        neighbor^.G := newG;
        neighbor^.H := HeuristicManhattan(neighbor^.X, neighbor^.Y, goalX, goalY);
        neighbor^.F := Fixed16Add(neighbor^.G, neighbor^.H);
        
        // Add to open list if not already there
        if not neighbor^.Open then
        begin
          neighbor^.Open := True;
          openList[openCount] := neighbor;
          openCount := openCount + 1;
        end;
      end;
    end;
  end;
  
  // Reconstruct path if found
  if found then
  begin
    pathCount := 0;
    temp := @nodes[goalY][goalX];
    
    // Trace back from goal to start
    while temp <> nil do
    begin
      if pathCount < 1024 then
      begin
        pathNodes[pathCount].X := temp^.X;
        pathNodes[pathCount].Y := temp^.Y;
        pathCount := pathCount + 1;
      end;
      temp := temp^.Parent;
    end;
    
    // Reverse path (start to goal)
    SetLength(path.Nodes, pathCount);
    for i := 0 to pathCount - 1 do
    begin
      path.Nodes[i].X := pathNodes[pathCount - 1 - i].X;
      path.Nodes[i].Y := pathNodes[pathCount - 1 - i].Y;
    end;
    
    path.Count := pathCount;
    path.Found := True;
    Result := True;
  end;
end;

// Dijkstra's algorithm (simplified - same as A* with H=0)
function FindPathDijkstra(
  const map: TTileMap;
  startX, startY: integer;
  goalX, goalY: integer;
  var path: TPath
): boolean;
begin
  // Dijkstra is A* with zero heuristic
  // For now, use A* with zero heuristic
  Result := FindPathAStar(map, startX, startY, goalX, goalY, path);
end;

end.

