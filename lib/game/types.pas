unit Game_Types;

interface

// Core types for game algorithms
// Source: algorithms/08_GameAlgorithms.md

uses
  Math_Types;

// ============================================================================
// Camera Types
// ============================================================================

// 3D Camera representation
// Uses 3x3 matrix for orientation and 3D vector for position
type
  TCamera = record
    // Camera matrix (3x3) - orientation
    Matrix: TMatrix3x3;
    // Camera position in world space
    Position: TVector3;
    // Field of view (in degrees, converted to fixed-point)
    FOV: Fixed16;
    // Near and far clipping planes
    NearPlane: Fixed16;
    FarPlane: Fixed16;
  end;

// ============================================================================
// BSP Tree Types
// ============================================================================

// BSP Node for binary space partitioning
type
  TBSPNode = ^TBSPNodeRec;
  
  TBSPNodeRec = record
    // Child nodes
    FrontNode: TBSPNode;
    BackNode: TBSPNode;
    // Partitioning plane (normal + distance)
    PlaneNormal: TVector3;
    PlaneDistance: Fixed16;
    // Bounding box (for culling)
    BoxMin: TVector3;
    BoxMax: TVector3;
    // Node data (polygon list, etc.)
    Data: Pointer;  // Generic pointer to node data
  end;

// BSP Tree structure
type
  TBSPTree = record
    Root: TBSPNode;
    NodeCount: integer;
  end;

// ============================================================================
// Line of Sight Types
// ============================================================================

// Tile-based map for LOS calculations
type
  TTileMap = record
    Width: integer;
    Height: integer;
    Tiles: array of array of boolean;  // true = solid/blocking, false = passable
  end;

// LOS result
type
  TLOSResult = record
    Visible: boolean;
    Distance: Fixed16;
    BlockedAt: record
      X, Y: integer;
    end;
  end;

// ============================================================================
// Pathfinding Types
// ============================================================================

// Node in pathfinding graph
type
  TPathNode = record
    X, Y: integer;        // Grid position
    G: Fixed16;           // Cost from start
    H: Fixed16;           // Heuristic (estimated cost to goal)
    F: Fixed16;           // Total cost (G + H)
    Parent: ^TPathNode;   // Parent node for path reconstruction
    Open: boolean;        // In open set
    Closed: boolean;      // In closed set
  end;

// Pathfinding result
type
  TPath = record
    Nodes: array of record
      X, Y: integer;
    end;
    Count: integer;
    Found: boolean;
  end;

implementation

end.

