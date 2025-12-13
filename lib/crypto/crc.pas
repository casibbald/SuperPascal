unit Crypto_CRC;

interface

uses
  Crypto_Types;

// Cyclic Redundancy Check (CRC) checksum calculation
// Table-driven implementation for fast CRC calculation

// Initialize CRC lookup table
// Parameters:
//   Params: CRC parameters (polynomial, width, etc.)
//   Table: Output lookup table (256 entries)
procedure CRCInitTable(const Params: TCRCParams; var Table: TCRCTable);

// Initialize CRC context
// Parameters:
//   Context: CRC context to initialize
//   Params: CRC parameters
procedure CRCInit(var Context: TCRCContext; const Params: TCRCParams);

// Update CRC with a single byte
// Parameters:
//   Context: CRC context
//   Data: Byte to process
procedure CRCUpdateByte(var Context: TCRCContext; Data: Byte);

// Update CRC with a block of data
// Parameters:
//   Context: CRC context
//   Data: Pointer to data block
//   Size: Size of data block in bytes
procedure CRCUpdateBlock(var Context: TCRCContext; Data: PByte; Size: LongInt);

// Finalize CRC calculation
// Parameters:
//   Context: CRC context
// Returns: Final CRC value
function CRCFinalize(var Context: TCRCContext): LongWord;

// Calculate CRC-16 for a data block (convenience function)
// Parameters:
//   Data: Pointer to data block
//   Size: Size of data block in bytes
//   Params: CRC-16 parameters (nil for standard CRC-16)
// Returns: CRC-16 checksum
function CalculateCRC16(Data: PByte; Size: LongInt; Params: PCRCParams): Word;

// Calculate CRC-32 for a data block (convenience function)
// Parameters:
//   Data: Pointer to data block
//   Size: Size of data block in bytes
// Returns: CRC-32 checksum
function CalculateCRC32(Data: PByte; Size: LongInt): LongWord;

implementation

// Reflect a value (bit reversal)
function ReflectValue(Value: LongWord; Width: Byte): LongWord;
var
  i: Byte;
  Reflected: LongWord;
begin
  Reflected := 0;
  for i := 0 to Width - 1 do
  begin
    if (Value and (1 shl i)) <> 0 then
      Reflected := Reflected or (1 shl (Width - 1 - i));
  end;
  ReflectValue := Reflected;
end;

// Get CRC mask based on width
function GetCRCMask(Width: Byte): LongWord;
begin
  if Width = 16 then
    GetCRCMask := $FFFF
  else if Width = 32 then
    GetCRCMask := $FFFFFFFF
  else
    GetCRCMask := (1 shl Width) - 1;
end;

procedure CRCInitTable(const Params: TCRCParams; var Table: TCRCTable);
var
  i, j: Byte;
  crc: LongWord;
  mask: LongWord;
  poly: LongWord;
begin
  mask := GetCRCMask(Params.Width);
  poly := Params.Poly;
  
  for i := 0 to 255 do
  begin
    if Params.RefIn then
      crc := ReflectValue(i, 8)
    else
      crc := i;
    
    if Params.Width > 8 then
      crc := crc shl (Params.Width - 8)
    else
      crc := crc and mask;
    
    for j := 0 to 7 do
    begin
      if (crc and (1 shl (Params.Width - 1))) <> 0 then
        crc := ((crc shl 1) xor poly) and mask
      else
        crc := (crc shl 1) and mask;
    end;
    
    if Params.RefIn and (Params.Width > 8) then
      crc := ReflectValue(crc, Params.Width);
    
    Table[i] := crc;
  end;
end;

procedure CRCInit(var Context: TCRCContext; const Params: TCRCParams);
begin
  Context.Params := Params;
  CRCInitTable(Params, Context.Table);
  Context.Value := Params.Init;
end;

procedure CRCUpdateByte(var Context: TCRCContext; Data: Byte);
var
  index: Byte;
  mask: LongWord;
begin
  mask := GetCRCMask(Context.Params.Width);
  
  if Context.Params.RefIn then
    index := (Context.Value xor Data) and $FF
  else
    index := ((Context.Value shr (Context.Params.Width - 8)) xor Data) and $FF;
  
  if Context.Params.RefIn then
    Context.Value := ((Context.Value shr 8) xor Context.Table[index]) and mask
  else
    Context.Value := ((Context.Value shl 8) xor Context.Table[index]) and mask;
end;

procedure CRCUpdateBlock(var Context: TCRCContext; Data: PByte; Size: LongInt);
var
  i: LongInt;
begin
  for i := 0 to Size - 1 do
    CRCUpdateByte(Context, PByte(Data + i)^);
end;

function CRCFinalize(var Context: TCRCContext): LongWord;
var
  mask: LongWord;
begin
  mask := GetCRCMask(Context.Params.Width);
  
  if Context.Params.RefOut then
    Context.Value := ReflectValue(Context.Value, Context.Params.Width);
  
  Context.Value := (Context.Value xor Context.Params.XorOut) and mask;
  CRCFinalize := Context.Value;
end;

function CalculateCRC16(Data: PByte; Size: LongInt; Params: PCRCParams): Word;
var
  context: TCRCContext;
  paramsToUse: TCRCParams;
begin
  if Params = nil then
    paramsToUse := CRC16_STANDARD
  else
    paramsToUse := Params^;
  
  CRCInit(context, paramsToUse);
  CRCUpdateBlock(context, Data, Size);
  CalculateCRC16 := Word(CRCFinalize(context));
end;

function CalculateCRC32(Data: PByte; Size: LongInt): LongWord;
var
  context: TCRCContext;
begin
  CRCInit(context, CRC32_STANDARD);
  CRCUpdateBlock(context, Data, Size);
  CalculateCRC32 := CRCFinalize(context);
end;

end.

