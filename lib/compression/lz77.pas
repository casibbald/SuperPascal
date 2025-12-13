unit Compression_LZ77;

interface

uses
  Compression_Types,
  Math_Types,
  Math_Fixed;

// LZ77 compression algorithm (LZSS variant)
// Sliding window compression that replaces repeated strings with references
// to previous occurrences in a sliding window.

// Compress data using LZ77
// Parameters:
//   Input: Pointer to input data
//   InputSize: Size of input data in bytes
//   Output: Pointer to output buffer (must be pre-allocated, at least InputSize bytes)
//   OutputSize: On input, maximum output size; on output, actual compressed size
//   Params: LZ77 compression parameters (optional, uses defaults if nil)
// Returns: True if compression successful, False otherwise
function LZ77Compress(
  Input: PByte;
  InputSize: LongInt;
  var Output: PByte;
  var OutputSize: LongInt;
  Params: PLZ77Params
): Boolean;

// Decompress LZ77-compressed data
// Parameters:
//   Input: Pointer to compressed data
//   InputSize: Size of compressed data in bytes
//   Output: Pointer to output buffer (must be pre-allocated, at least OriginalSize bytes)
//   OutputSize: On input, maximum output size; on output, actual decompressed size
// Returns: True if decompression successful, False otherwise
function LZ77Decompress(
  Input: PByte;
  InputSize: LongInt;
  var Output: PByte;
  var OutputSize: LongInt
): Boolean;

// Calculate compression ratio for LZ77
function LZ77CalculateRatio(OriginalSize, CompressedSize: LongInt): Fixed16;

implementation

function LZ77Compress(
  Input: PByte;
  InputSize: LongInt;
  var Output: PByte;
  var OutputSize: LongInt;
  Params: PLZ77Params
): Boolean;
var
  windowSize, lookAheadSize, minMatch: Word;
  window: PByte;
  windowPos: LongInt;
  inputPos: LongInt;
  outPos: LongInt;
  tagPos: LongInt;
  tagByte: Byte;
  tagBit: Byte;
  i, j: LongInt;
  bestMatchPos: LongInt;
  bestMatchLen: Word;
  matchPos: LongInt;
  matchLen: Word;
  found: Boolean;
begin
  Result := False;
  
  if (Input = nil) or (Output = nil) or (InputSize <= 0) or (OutputSize <= 0) then
    Exit;
  
  // Set default parameters if not provided
  if Params = nil then
  begin
    windowSize := LZ77_DEFAULT_WINDOW;
    lookAheadSize := LZ77_DEFAULT_LOOKAHEAD;
    minMatch := LZ77_DEFAULT_MIN_MATCH;
  end
  else
  begin
    windowSize := Params^.WindowSize;
    lookAheadSize := Params^.LookAheadSize;
    minMatch := Params^.MinMatchLength;
  end;
  
  // Allocate sliding window
  GetMem(window, windowSize);
  if window = nil then
    Exit;
  
  // Initialize window with zeros
  for i := 0 to windowSize - 1 do
    PByte(window + i)^ := 0;
  
  inputPos := 0;
  outPos := 0;
  windowPos := 0;
  tagPos := 0;
  tagByte := 0;
  tagBit := 0;
  
  // Write file header (simplified - just original size for now)
  if outPos + 4 > OutputSize then
  begin
    FreeMem(window);
    Exit;
  end;
  PLongInt(Output + outPos)^ := InputSize;
  Inc(outPos, 4);
  
  while inputPos < InputSize do
  begin
    // Check if we need a new tag byte
    if tagBit = 0 then
    begin
      tagPos := outPos;
      if outPos >= OutputSize then
      begin
        FreeMem(window);
        Exit;
      end;
      PByte(Output + outPos)^ := 0;
      Inc(outPos);
      tagByte := 0;
      tagBit := 1;
    end;
    
    // Search for match in sliding window
    bestMatchPos := 0;
    bestMatchLen := 0;
    
    // Simple linear search (can be optimized with hash table)
    for i := 0 to windowSize - 1 do
    begin
      matchPos := (windowPos - i + windowSize) mod windowSize;
      matchLen := 0;
      found := True;
      
      // Try to match as many bytes as possible
      while (matchLen < lookAheadSize) and
            (inputPos + matchLen < InputSize) and
            (matchLen < windowSize) and
            found do
      begin
        j := (matchPos + matchLen) mod windowSize;
        if PByte(window + j)^ = PByte(Input + inputPos + matchLen)^ then
          Inc(matchLen)
        else
          found := False;
      end;
      
      if matchLen > bestMatchLen then
      begin
        bestMatchLen := matchLen;
        bestMatchPos := matchPos;
        if bestMatchLen >= lookAheadSize then
          Break;  // Found maximum match
      end;
    end;
    
    // If we found a good match, encode it
    if bestMatchLen >= minMatch then
    begin
      // Set tag bit to 1 (encoded string)
      tagByte := tagByte or tagBit;
      
      // Write match: 2 bytes
      // Byte 1: upper nibble = bits 11-8 of position, lower nibble = length - 2
      // Byte 2: bits 7-0 of position
      if outPos + 2 > OutputSize then
      begin
        FreeMem(window);
        Exit;
      end;
      
      PByte(Output + outPos)^ := ((bestMatchPos shr 8) and $0F) or ((bestMatchLen - 2) shl 4);
      Inc(outPos);
      PByte(Output + outPos)^ := bestMatchPos and $FF;
      Inc(outPos);
      
      // Update window
      for j := 0 to bestMatchLen - 1 do
      begin
        PByte(window + windowPos)^ := PByte(Input + inputPos + j)^;
        windowPos := (windowPos + 1) mod windowSize;
      end;
      
      Inc(inputPos, bestMatchLen);
    end
    else
    begin
      // Literal byte
      // Tag bit is already 0
      
      if outPos >= OutputSize then
      begin
        FreeMem(window);
        Exit;
      end;
      
      PByte(Output + outPos)^ := PByte(Input + inputPos)^;
      Inc(outPos);
      
      // Update window
      PByte(window + windowPos)^ := PByte(Input + inputPos)^;
      windowPos := (windowPos + 1) mod windowSize;
      
      Inc(inputPos);
    end;
    
    // Update tag byte
    PByte(Output + tagPos)^ := tagByte;
    
    // Move to next tag bit
    tagBit := tagBit shl 1;
    if tagBit = 0 then
      tagBit := 1;  // Reset to bit 0 for next tag byte
  end;
  
  // Write final tag byte if needed
  if tagBit <> 1 then
    PByte(Output + tagPos)^ := tagByte;
  
  OutputSize := outPos;
  FreeMem(window);
  Result := True;
end;

function LZ77Decompress(
  Input: PByte;
  InputSize: LongInt;
  var Output: PByte;
  var OutputSize: LongInt
): Boolean;
var
  originalSize: LongInt;
  window: PByte;
  windowSize: Word;
  windowPos: LongInt;
  inputPos: LongInt;
  outPos: LongInt;
  tagByte: Byte;
  tagBit: Byte;
  matchPos: Word;
  matchLen: Byte;
  i: LongInt;
  offset: Word;
begin
  Result := False;
  
  if (Input = nil) or (Output = nil) or (InputSize < 4) or (OutputSize <= 0) then
    Exit;
  
  // Read original size
  originalSize := PLongInt(Input)^;
  if originalSize > OutputSize then
    Exit;  // Output buffer too small
  
  inputPos := 4;
  outPos := 0;
  windowSize := LZ77_DEFAULT_WINDOW;
  
  // Allocate sliding window
  GetMem(window, windowSize);
  if window = nil then
    Exit;
  
  // Initialize window with zeros
  for i := 0 to windowSize - 1 do
    PByte(window + i)^ := 0;
  
  windowPos := 0;
  tagByte := 0;
  tagBit := 0;
  
  while (inputPos < InputSize) and (outPos < originalSize) do
  begin
    // Read tag byte if needed
    if tagBit = 0 then
    begin
      if inputPos >= InputSize then
      begin
        FreeMem(window);
        Exit;
      end;
      tagByte := PByte(Input + inputPos)^;
      Inc(inputPos);
      tagBit := 1;
    end;
    
    // Check tag bit
    if (tagByte and tagBit) <> 0 then
    begin
      // Encoded string
      if inputPos + 2 > InputSize then
      begin
        FreeMem(window);
        Exit;
      end;
      
      // Read match: 2 bytes
      matchLen := ((PByte(Input + inputPos)^ shr 4) and $0F) + 2;
      offset := ((PByte(Input + inputPos)^ and $0F) shl 8) or PByte(Input + inputPos + 1)^;
      Inc(inputPos, 2);
      
      matchPos := (windowPos - offset + windowSize) mod windowSize;
      
      // Copy match from window
      for i := 0 to matchLen - 1 do
      begin
        if outPos >= originalSize then
        begin
          FreeMem(window);
          Exit;
        end;
        
        PByte(Output + outPos)^ := PByte(window + matchPos)^;
        
        // Update window
        PByte(window + windowPos)^ := PByte(window + matchPos)^;
        windowPos := (windowPos + 1) mod windowSize;
        matchPos := (matchPos + 1) mod windowSize;
        
        Inc(outPos);
      end;
    end
    else
    begin
      // Literal byte
      if inputPos >= InputSize then
      begin
        FreeMem(window);
        Exit;
      end;
      
      if outPos >= originalSize then
      begin
        FreeMem(window);
        Exit;
      end;
      
      PByte(Output + outPos)^ := PByte(Input + inputPos)^;
      
      // Update window
      PByte(window + windowPos)^ := PByte(Input + inputPos)^;
      windowPos := (windowPos + 1) mod windowSize;
      
      Inc(inputPos);
      Inc(outPos);
    end;
    
    // Move to next tag bit
    tagBit := tagBit shl 1;
    if tagBit = 0 then
      tagBit := 1;  // Reset to bit 0 for next tag byte
  end;
  
  OutputSize := outPos;
  FreeMem(window);
  Result := (outPos = originalSize);
end;

function LZ77CalculateRatio(OriginalSize, CompressedSize: LongInt): Fixed16;
begin
  if OriginalSize = 0 then
    Result := FIXED16_ONE
  else
    Result := Fixed16Div(IntToFixed16(CompressedSize), IntToFixed16(OriginalSize));
end;

end.

