unit Compression_RLE;

interface

uses
  Compression_Types,
  Math_Types,
  Math_Fixed;

// Run-Length Encoding (RLE) compression
// Simple compression algorithm that replaces sequences of repeated bytes
// with a count and the byte value.

// Compress data using RLE
// Parameters:
//   Input: Pointer to input data
//   InputSize: Size of input data in bytes
//   Output: Pointer to output buffer (must be pre-allocated, at least InputSize bytes)
//   OutputSize: On input, maximum output size; on output, actual compressed size
//   Params: RLE compression parameters (optional, uses defaults if nil)
// Returns: True if compression successful, False otherwise
function RLECompress(
  Input: PByte;
  InputSize: LongInt;
  var Output: PByte;
  var OutputSize: LongInt;
  Params: PRLEParams
): Boolean;

// Decompress RLE-compressed data
// Parameters:
//   Input: Pointer to compressed data
//   InputSize: Size of compressed data in bytes
//   Output: Pointer to output buffer (must be pre-allocated, at least OriginalSize bytes)
//   OutputSize: On input, maximum output size; on output, actual decompressed size
// Returns: True if decompression successful, False otherwise
function RLEDecompress(
  Input: PByte;
  InputSize: LongInt;
  var Output: PByte;
  var OutputSize: LongInt
): Boolean;

// Calculate compression ratio for RLE
function RLECalculateRatio(OriginalSize, CompressedSize: LongInt): Fixed16;

implementation

function RLECompress(
  Input: PByte;
  InputSize: LongInt;
  var Output: PByte;
  var OutputSize: LongInt;
  Params: PRLEParams
): Boolean;
var
  i, j: LongInt;
  currentByte: Byte;
  runLength: Byte;
  maxRun, minRun: Byte;
  outPos: LongInt;
  inputPos: LongInt;
begin
  Result := False;
  
  if (Input = nil) or (Output = nil) or (InputSize <= 0) or (OutputSize <= 0) then
    Exit;
  
  // Set default parameters if not provided
  if Params = nil then
  begin
    maxRun := RLE_DEFAULT_MAX_RUN;
    minRun := RLE_DEFAULT_MIN_RUN;
  end
  else
  begin
    maxRun := Params^.MaxRunLength;
    minRun := Params^.MinRunLength;
  end;
  
  outPos := 0;
  inputPos := 0;
  
  while inputPos < InputSize do
  begin
    currentByte := PByte(Input + inputPos)^;
    runLength := 1;
    
    // Count consecutive identical bytes
    while (inputPos + runLength < InputSize) and
          (PByte(Input + inputPos + runLength)^ = currentByte) and
          (runLength < maxRun) do
    begin
      Inc(runLength);
    end;
    
    // Check if we have enough space in output
    if outPos + 2 > OutputSize then
      Exit;  // Output buffer too small
    
    // If run length is >= minRun, encode as run
    if runLength >= minRun then
    begin
      // Write run marker: 0x80 | (runLength - minRun)
      // This allows runs of 3-255 bytes (with minRun=3)
      PByte(Output + outPos)^ := $80 or (runLength - minRun);
      Inc(outPos);
      PByte(Output + outPos)^ := currentByte;
      Inc(outPos);
      Inc(inputPos, runLength);
    end
    else
    begin
      // Write literal byte: 0x00-0x7F
      // For single bytes, we use 0x00-0x7F range
      // For sequences of 2 bytes, we use 0x01-0x7F with special encoding
      if runLength = 1 then
      begin
        // Single literal byte
        if currentByte < $80 then
        begin
          PByte(Output + outPos)^ := currentByte;
          Inc(outPos);
        end
        else
        begin
          // High byte, use escape sequence
          if outPos + 2 > OutputSize then
            Exit;
          PByte(Output + outPos)^ := $7F;  // Escape marker
          Inc(outPos);
          PByte(Output + outPos)^ := currentByte;
          Inc(outPos);
        end;
        Inc(inputPos);
      end
      else
      begin
        // Multiple literals (2 bytes)
        // Encode as: 0x01-0x7F (count-1), followed by bytes
        if outPos + runLength + 1 > OutputSize then
          Exit;
        PByte(Output + outPos)^ := runLength - 1;  // Count (1-127)
        Inc(outPos);
        for j := 0 to runLength - 1 do
        begin
          PByte(Output + outPos)^ := PByte(Input + inputPos + j)^;
          Inc(outPos);
        end;
        Inc(inputPos, runLength);
      end;
    end;
  end;
  
  OutputSize := outPos;
  Result := True;
end;

function RLEDecompress(
  Input: PByte;
  InputSize: LongInt;
  var Output: PByte;
  var OutputSize: LongInt
): Boolean;
var
  i, j: LongInt;
  inputPos: LongInt;
  outPos: LongInt;
  currentByte: Byte;
  runLength: Byte;
  count: Byte;
begin
  Result := False;
  
  if (Input = nil) or (Output = nil) or (InputSize <= 0) or (OutputSize <= 0) then
    Exit;
  
  inputPos := 0;
  outPos := 0;
  
  while inputPos < InputSize do
  begin
    currentByte := PByte(Input + inputPos)^;
    Inc(inputPos);
    
    // Check if this is a run marker (bit 7 set)
    if (currentByte and $80) <> 0 then
    begin
      // Run encoding: 0x80 | (runLength - 3)
      runLength := (currentByte and $7F) + RLE_DEFAULT_MIN_RUN;
      
      if inputPos >= InputSize then
        Exit;  // Missing byte value
      
      currentByte := PByte(Input + inputPos)^;
      Inc(inputPos);
      
      // Expand run
      if outPos + runLength > OutputSize then
        Exit;  // Output buffer too small
      
      for j := 0 to runLength - 1 do
      begin
        PByte(Output + outPos)^ := currentByte;
        Inc(outPos);
      end;
    end
    else if currentByte = $7F then
    begin
      // Escape sequence for high byte
      if inputPos >= InputSize then
        Exit;  // Missing byte value
      
      currentByte := PByte(Input + inputPos)^;
      Inc(inputPos);
      
      if outPos >= OutputSize then
        Exit;  // Output buffer too small
      
      PByte(Output + outPos)^ := currentByte;
      Inc(outPos);
    end
    else if currentByte > 0 then
    begin
      // Literal sequence: count (1-127), followed by bytes
      count := currentByte + 1;  // Actual count (1-128)
      
      if inputPos + count > InputSize then
        Exit;  // Not enough data
      
      if outPos + count > OutputSize then
        Exit;  // Output buffer too small
      
      for j := 0 to count - 1 do
      begin
        PByte(Output + outPos)^ := PByte(Input + inputPos + j)^;
        Inc(outPos);
      end;
      Inc(inputPos, count);
    end
    else
    begin
      // Single literal byte (0x00)
      if outPos >= OutputSize then
        Exit;  // Output buffer too small
      
      PByte(Output + outPos)^ := 0;
      Inc(outPos);
    end;
  end;
  
  OutputSize := outPos;
  Result := True;
end;

function RLECalculateRatio(OriginalSize, CompressedSize: LongInt): Fixed16;
begin
  if OriginalSize = 0 then
    Result := FIXED16_ONE
  else
    Result := Fixed16Div(IntToFixed16(CompressedSize), IntToFixed16(OriginalSize));
end;

end.

