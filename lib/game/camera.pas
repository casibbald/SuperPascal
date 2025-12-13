unit Game_Camera;

interface

// Camera systems for 3D games
// Source: algorithms/08_GameAlgorithms.md
// Based on: docs/mikro_docs_archive/Coding/1/CAM_MATR.TXT
//
// Camera representation using 3x3 matrix for orientation and 3D vector for position
// Transforms world space coordinates to camera space

uses
  Game_Types,
  Math_Types,
  Math_Fixed,
  Math_Matrix;

// ============================================================================
// Camera Creation and Management
// ============================================================================

// Create a camera at the origin looking forward
function CameraCreate: TCamera;

// Create a camera at specified position
function CameraCreateAt(x, y, z: Fixed16): TCamera;

// Create a camera with position and target
function CameraCreateLookAt(
  posX, posY, posZ: Fixed16;
  targetX, targetY, targetZ: Fixed16;
  upX, upY, upZ: Fixed16
): TCamera;

// ============================================================================
// Camera Updates
// ============================================================================

// Update camera position
procedure CameraSetPosition(var camera: TCamera; x, y, z: Fixed16);

// Update camera orientation (using Euler angles)
procedure CameraSetRotation(var camera: TCamera; pitch, yaw, roll: Fixed16);

// Update camera to look at a target
procedure CameraLookAt(
  var camera: TCamera;
  targetX, targetY, targetZ: Fixed16;
  upX, upY, upZ: Fixed16
);

// ============================================================================
// Camera Transformations
// ============================================================================

// Get camera view matrix (4x4) for rendering
// Transforms world space to camera space
function CameraGetViewMatrix(const camera: TCamera): TMatrix4x4;

// Transform world point to camera space
function CameraTransformPoint(const camera: TCamera; const worldPoint: TVector3): TVector3;

// Check if point is in front of camera (Z > 0 in camera space)
function CameraIsPointVisible(const camera: TCamera; const worldPoint: TVector3): boolean;

implementation

uses
  Math_Trig;

// Helper: Create 3x3 identity matrix
function Matrix3x3Identity: TMatrix3x3;
var
  i, j: integer;
begin
  for i := 0 to 2 do
    for j := 0 to 2 do
      if i = j then
        Result[i, j] := FIXED16_ONE
      else
        Result[i, j] := 0;
end;

// Helper: Multiply 3x3 matrix by vector
function Matrix3x3MulVector(const m: TMatrix3x3; const v: TVector3): TVector3;
begin
  Result.X := Fixed16Add(Fixed16Add(
                Fixed16Mul(m[0, 0], v.X),
                Fixed16Mul(m[0, 1], v.Y)),
                Fixed16Mul(m[0, 2], v.Z));
  Result.Y := Fixed16Add(Fixed16Add(
                Fixed16Mul(m[1, 0], v.X),
                Fixed16Mul(m[1, 1], v.Y)),
                Fixed16Mul(m[1, 2], v.Z));
  Result.Z := Fixed16Add(Fixed16Add(
                Fixed16Mul(m[2, 0], v.X),
                Fixed16Mul(m[2, 1], v.Y)),
                Fixed16Mul(m[2, 2], v.Z));
end;

// Create camera at origin
function CameraCreate: TCamera;
begin
  Result.Matrix := Matrix3x3Identity;
  Result.Position.X := 0;
  Result.Position.Y := 0;
  Result.Position.Z := 0;
  Result.FOV := IntToFixed16(60);  // 60 degrees default
  Result.NearPlane := IntToFixed16(1);
  Result.FarPlane := IntToFixed16(1000);
end;

// Create camera at specified position
function CameraCreateAt(x, y, z: Fixed16): TCamera;
begin
  Result := CameraCreate;
  Result.Position.X := x;
  Result.Position.Y := y;
  Result.Position.Z := z;
end;

// Create camera looking at target (simplified - uses basic orientation)
function CameraCreateLookAt(
  posX, posY, posZ: Fixed16;
  targetX, targetY, targetZ: Fixed16;
  upX, upY, upZ: Fixed16
): TCamera;
var
  forward, right, up: TVector3;
  forwardLen, rightLen, upLen: Fixed16;
begin
  Result.Position.X := posX;
  Result.Position.Y := posY;
  Result.Position.Z := posZ;
  Result.FOV := IntToFixed16(60);
  Result.NearPlane := IntToFixed16(1);
  Result.FarPlane := IntToFixed16(1000);
  
  // Calculate forward vector (from position to target)
  forward.X := Fixed16Sub(targetX, posX);
  forward.Y := Fixed16Sub(targetY, posY);
  forward.Z := Fixed16Sub(targetZ, posZ);
  
  // Normalize forward vector (simplified - use fixed-point approximation)
  forwardLen := Fixed16Add(Fixed16Add(
                  Fixed16Mul(forward.X, forward.X),
                  Fixed16Mul(forward.Y, forward.Y)),
                  Fixed16Mul(forward.Z, forward.Z));
  // TODO: Normalize properly with square root
  
  // Calculate right vector (cross product of forward and up)
  // Simplified: assume up is (0, 1, 0) if not provided
  if (upX = 0) and (upY = 0) and (upZ = 0) then
  begin
    up.X := 0;
    up.Y := FIXED16_ONE;
    up.Z := 0;
  end
  else
  begin
    up.X := upX;
    up.Y := upY;
    up.Z := upZ;
  end;
  
  // Cross product: right = forward × up
  right.X := Fixed16Mul(forward.Y, up.Z) - Fixed16Mul(forward.Z, up.Y);
  right.Y := Fixed16Mul(forward.Z, up.X) - Fixed16Mul(forward.X, up.Z);
  right.Z := Fixed16Mul(forward.X, up.Y) - Fixed16Mul(forward.Y, up.X);
  
  // Build camera matrix from right, up, forward vectors
  // Matrix columns represent camera's local X, Y, Z axes
  Result.Matrix[0, 0] := right.X;
  Result.Matrix[0, 1] := right.Y;
  Result.Matrix[0, 2] := right.Z;
  
  Result.Matrix[1, 0] := up.X;
  Result.Matrix[1, 1] := up.Y;
  Result.Matrix[1, 2] := up.Z;
  
  Result.Matrix[2, 0] := forward.X;
  Result.Matrix[2, 1] := forward.Y;
  Result.Matrix[2, 2] := forward.Z;
end;

// Set camera position
procedure CameraSetPosition(var camera: TCamera; x, y, z: Fixed16);
begin
  camera.Position.X := x;
  camera.Position.Y := y;
  camera.Position.Z := z;
end;

// Set camera rotation using Euler angles (simplified)
procedure CameraSetRotation(var camera: TCamera; pitch, yaw, roll: Fixed16);
var
  cosP, sinP, cosY, sinY, cosR, sinR: Fixed16;
  pitchInt, yawInt, rollInt: SmallInt;
begin
  // Convert angles to SmallInt for trig lookup (-180 to +180 range)
  pitchInt := Fixed16ToInt(pitch) mod 360;
  if pitchInt > 180 then pitchInt := pitchInt - 360;
  if pitchInt < -180 then pitchInt := pitchInt + 360;
  
  yawInt := Fixed16ToInt(yaw) mod 360;
  if yawInt > 180 then yawInt := yawInt - 360;
  if yawInt < -180 then yawInt := yawInt + 360;
  
  rollInt := Fixed16ToInt(roll) mod 360;
  if rollInt > 180 then rollInt := rollInt - 360;
  if rollInt < -180 then rollInt := rollInt + 360;
  
  // Get sin/cos values (using signed trig functions)
  cosP := CosSigned(pitchInt);
  sinP := SinSigned(pitchInt);
  cosY := CosSigned(yawInt);
  sinY := SinSigned(yawInt);
  cosR := CosSigned(rollInt);
  sinR := SinSigned(rollInt);
  
  // Build rotation matrix (ZYX order: roll, pitch, yaw)
  // This is a simplified Euler angle rotation
  camera.Matrix[0, 0] := Fixed16Mul(cosY, cosP);
  camera.Matrix[0, 1] := Fixed16Mul(cosY, sinP);
  camera.Matrix[0, 2] := Fixed16Neg(sinY);
  
  camera.Matrix[1, 0] := Fixed16Sub(Fixed16Mul(sinR, sinY), Fixed16Mul(Fixed16Mul(cosR, sinP), cosY));
  camera.Matrix[1, 1] := Fixed16Add(Fixed16Mul(cosR, cosP), Fixed16Mul(Fixed16Mul(sinR, sinY), sinP));
  camera.Matrix[1, 2] := Fixed16Mul(cosR, sinY);
  
  camera.Matrix[2, 0] := Fixed16Add(Fixed16Mul(cosR, Fixed16Mul(sinY, cosP)), Fixed16Mul(sinR, sinP));
  camera.Matrix[2, 1] := Fixed16Sub(Fixed16Mul(cosR, Fixed16Mul(sinY, sinP)), Fixed16Mul(sinR, cosP));
  camera.Matrix[2, 2] := Fixed16Mul(cosR, cosY);
end;

// Make camera look at target
procedure CameraLookAt(
  var camera: TCamera;
  targetX, targetY, targetZ: Fixed16;
  upX, upY, upZ: Fixed16
);
var
  newCamera: TCamera;
begin
  newCamera := CameraCreateLookAt(
    camera.Position.X, camera.Position.Y, camera.Position.Z,
    targetX, targetY, targetZ,
    upX, upY, upZ
  );
  camera.Matrix := newCamera.Matrix;
end;

// Get 4x4 view matrix for rendering
function CameraGetViewMatrix(const camera: TCamera): TMatrix4x4;
var
  invPos: TVector3;
  invMatrix: TMatrix3x3;
  i, j: integer;
begin
  // Create inverse translation (move world so camera is at origin)
  invPos.X := -camera.Position.X;
  invPos.Y := -camera.Position.Y;
  invPos.Z := -camera.Position.Z;
  
  // Create inverse rotation matrix (transpose of 3x3 matrix)
  for i := 0 to 2 do
    for j := 0 to 2 do
      invMatrix[i, j] := camera.Matrix[j, i];  // Transpose
  
  // Build 4x4 view matrix
  Result := MatrixIdentity;
  
  // Copy 3x3 rotation part
  for i := 0 to 2 do
    for j := 0 to 2 do
      Result[i, j] := invMatrix[i, j];
  
  // Apply inverse translation (transform position by inverse rotation, then negate)
  Result[3, 0] := Fixed16Add(Fixed16Add(
                    Fixed16Mul(invMatrix[0, 0], invPos.X),
                    Fixed16Mul(invMatrix[0, 1], invPos.Y)),
                    Fixed16Mul(invMatrix[0, 2], invPos.Z));
  Result[3, 1] := Fixed16Add(Fixed16Add(
                    Fixed16Mul(invMatrix[1, 0], invPos.X),
                    Fixed16Mul(invMatrix[1, 1], invPos.Y)),
                    Fixed16Mul(invMatrix[1, 2], invPos.Z));
  Result[3, 2] := Fixed16Add(Fixed16Add(
                    Fixed16Mul(invMatrix[2, 0], invPos.X),
                    Fixed16Mul(invMatrix[2, 1], invPos.Y)),
                    Fixed16Mul(invMatrix[2, 2], invPos.Z));
end;

// Transform world point to camera space
function CameraTransformPoint(const camera: TCamera; const worldPoint: TVector3): TVector3;
var
  translated: TVector3;
begin
  // Translate: subtract camera position
  translated.X := Fixed16Sub(worldPoint.X, camera.Position.X);
  translated.Y := Fixed16Sub(worldPoint.Y, camera.Position.Y);
  translated.Z := Fixed16Sub(worldPoint.Z, camera.Position.Z);
  
  // Rotate: multiply by camera matrix
  Result := Matrix3x3MulVector(camera.Matrix, translated);
end;

// Check if point is visible (in front of camera)
function CameraIsPointVisible(const camera: TCamera; const worldPoint: TVector3): boolean;
var
  cameraPoint: TVector3;
begin
  cameraPoint := CameraTransformPoint(camera, worldPoint);
  // Point is visible if Z > 0 (in front of camera)
  Result := cameraPoint.Z > 0;
end;

end.

