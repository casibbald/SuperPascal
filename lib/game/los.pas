unit Game_LOS;

interface

// Line of Sight algorithms
// Source: algorithms/08_GameAlgorithms.md
// Based on: docs/mikro_docs_archive/Coding/2/LOS.TXT
//
// Fast line-of-sight calculation for tile-based games (rogue-like)
// Uses Bresenham's line algorithm to check for blocking tiles

uses
  Game_Types,
  Math_Types;

// Check line of sight between two points in a tile map
// Returns true if there's a clear path (no blocking tiles)
function CheckLineOfSight(
  const map: TTileMap;
  x1, y1, x2, y2: integer
): boolean;

// Check line of sight and return detailed result
function CheckLineOfSightDetailed(
  const map: TTileMap;
  x1, y1, x2, y2: integer;
  var result: TLOSResult
): boolean;

// Calculate visibility field from a point (for lighting/shading)
// Marks visible tiles within radius
procedure CalculateVisibilityField(
  const map: TTileMap;
  centerX, centerY: integer;
  radius: integer;
  var visible: array of array of boolean
);

implementation

// Check line of sight using Bresenham's line algorithm
// Traces line from (x1,y1) to (x2,y2) and checks for blocking tiles
function CheckLineOfSight(
  const map: TTileMap;
  x1, y1, x2, y2: integer
): boolean;
var
  deltaX, deltaY: integer;
  xChange, yChange: integer;
  error: integer;
  x, y: integer;
  temp: integer;
begin
  // Validate coordinates
  if (x1 < 0) or (x1 >= map.Width) or (y1 < 0) or (y1 >= map.Height) then
  begin
    Result := False;
    Exit;
  end;
  
  if (x2 < 0) or (x2 >= map.Width) or (y2 < 0) or (y2 >= map.Height) then
  begin
    Result := False;
    Exit;
  end;
  
  // Same point - always visible
  if (x1 = x2) and (y1 = y2) then
  begin
    Result := True;
    Exit;
  end;
  
  x := x1;
  y := y1;
  
  deltaX := x2 - x1;
  deltaY := y2 - y1;
  
  // Determine direction
  if deltaX < 0 then
  begin
    xChange := -1;
    deltaX := -deltaX;
  end
  else
    xChange := 1;
  
  if deltaY < 0 then
  begin
    yChange := -1;
    deltaY := -deltaY;
  end
  else
    yChange := 1;
  
  error := 0;
  
  // Trace line using Bresenham's algorithm
  if deltaX > deltaY then
  begin
    // X is major axis
    while x <> x2 do
    begin
      // Check if current tile is blocking
      if map.Tiles[y][x] then  // true = solid/blocking
      begin
        Result := False;
        Exit;
      end;
      
      error := error + deltaY;
      if error >= deltaX then
      begin
        y := y + yChange;
        error := error - deltaX;
      end;
      x := x + xChange;
    end;
  end
  else
  begin
    // Y is major axis
    while y <> y2 do
    begin
      // Check if current tile is blocking
      if map.Tiles[y][x] then  // true = solid/blocking
      begin
        Result := False;
        Exit;
      end;
      
      error := error + deltaX;
      if error >= deltaY then
      begin
        x := x + xChange;
        error := error - deltaY;
      end;
      y := y + yChange;
    end;
  end;
  
  // Check destination tile (don't block on destination)
  // Line of sight is clear
  Result := True;
end;

// Check line of sight with detailed result
function CheckLineOfSightDetailed(
  const map: TTileMap;
  x1, y1, x2, y2: integer;
  var result: TLOSResult
): boolean;
var
  deltaX, deltaY: integer;
  xChange, yChange: integer;
  error: integer;
  x, y: integer;
  distance: Fixed16;
begin
  // Initialize result
  result.Visible := True;
  result.Distance := 0;
  result.BlockedAt.X := -1;
  result.BlockedAt.Y := -1;
  
  // Validate coordinates
  if (x1 < 0) or (x1 >= map.Width) or (y1 < 0) or (y1 >= map.Height) then
  begin
    result.Visible := False;
    Result := False;
    Exit;
  end;
  
  if (x2 < 0) or (x2 >= map.Width) or (y2 < 0) or (y2 >= map.Height) then
  begin
    result.Visible := False;
    Result := False;
    Exit;
  end;
  
  // Same point - always visible
  if (x1 = x2) and (y1 = y2) then
  begin
    result.Visible := True;
    result.Distance := 0;
    Result := True;
    Exit;
  end;
  
  x := x1;
  y := y1;
  
  deltaX := x2 - x1;
  deltaY := y2 - y1;
  
  // Calculate distance (for result)
  distance := 0;
  
  // Determine direction
  if deltaX < 0 then
  begin
    xChange := -1;
    deltaX := -deltaX;
  end
  else
    xChange := 1;
  
  if deltaY < 0 then
  begin
    yChange := -1;
    deltaY := -deltaY;
  end
  else
    yChange := 1;
  
  error := 0;
  
  // Trace line using Bresenham's algorithm
  if deltaX > deltaY then
  begin
    // X is major axis
    while x <> x2 do
    begin
      // Check if current tile is blocking
      if map.Tiles[y][x] then  // true = solid/blocking
      begin
        result.Visible := False;
        result.BlockedAt.X := x;
        result.BlockedAt.Y := y;
        Result := False;
        Exit;
      end;
      
      distance := distance + 1;  // Increment distance
      
      error := error + deltaY;
      if error >= deltaX then
      begin
        y := y + yChange;
        error := error - deltaX;
      end;
      x := x + xChange;
    end;
  end
  else
  begin
    // Y is major axis
    while y <> y2 do
    begin
      // Check if current tile is blocking
      if map.Tiles[y][x] then  // true = solid/blocking
      begin
        result.Visible := False;
        result.BlockedAt.X := x;
        result.BlockedAt.Y := y;
        Result := False;
        Exit;
      end;
      
      distance := distance + 1;  // Increment distance
      
      error := error + deltaX;
      if error >= deltaY then
      begin
        x := x + xChange;
        error := error - deltaY;
      end;
      y := y + yChange;
    end;
  end;
  
  result.Distance := distance;
  result.Visible := True;
  Result := True;
end;

// Calculate visibility field from a point (simplified version)
// For full implementation, see LOS.TXT shadow table algorithm
procedure CalculateVisibilityField(
  const map: TTileMap;
  centerX, centerY: integer;
  radius: integer;
  var visible: array of array of boolean
);
var
  x, y: integer;
  dx, dy: integer;
  distanceSquared: integer;
  radiusSquared: integer;
begin
  radiusSquared := radius * radius;
  
  // Mark all tiles within radius as potentially visible
  for y := 0 to map.Height - 1 do
    for x := 0 to map.Width - 1 do
    begin
      dx := x - centerX;
      dy := y - centerY;
      distanceSquared := dx * dx + dy * dy;
      
      if distanceSquared <= radiusSquared then
      begin
        // Check line of sight to this tile
        visible[y][x] := CheckLineOfSight(map, centerX, centerY, x, y);
      end
      else
        visible[y][x] := False;
    end;
end;

end.

