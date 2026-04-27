# Processing Mem and Port Arrays - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-mem-and-port-arrays

---

## Table of Contents

- Processing Mem and Port Arrays

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Mem and Port Arrays
Turbo Pascal Provides two ‘virtual’ arrays that provide acces to memory (
Mem
) and I/O ports (
Port
). To access memory we need segment and offset. Both parameters are processed and calculated. The segment is put into the
ES
register while offset is in
DI
.
Procedure TExpression.Process_Mem;
Var SegmentExpression, OffsetExpression: TExpression;
begin
  GetNextToken;
  TypeDefPtr := PointerFromOffsets (PUnitOffsets (CurrentIdentifierDataPtr)^);
  DataType := TypeDefPtr^.DataType;
  ExpectTokenAndGetNext (Token_LeftBracket);
  SegmentExpression.ExpectIntegerExpression;
  SegmentExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_Colon);
  OffsetExpression.ExpectIntegerExpression;
  OffsetExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_RightBracket);
  SegmentExpression.Calculate;
  SegmentExpression.LoadExpressionToRegisters (urAX);
  GenerateInstruction_Byte (PUSH_AX);
  OffsetExpression.Calculate;
  OffsetExpression.LoadExpressionToRegisters (urDI);
  GenerateInstruction_Byte (POP_ES);
  ES_DI_PointerDestroyed;
  EndIntermediateCodeSubroutine;
  UsedRegisters := (SegmentExpression.UsedRegisters + OffsetExpression.UsedRegisters) * [urBX, urDX, urCX, urAX];
  LocationData.Flags := [ofsDI, segES];
  Value.Word := 0;
  Value.W12 := 0;
  Value.W14 := 0;
  Location := elMemory;
end;
I/O ports can be accessed for
reading and writing
. This procedure processes
Port
expression for reading. There are two versions of this function: 8-bit
Port
and 16-bit
PortW
. Both are defined in the bootstrap system table where
PortW
has identifier data set to 1.
Procedure Process_Port;
  Var IOCode: Byte;
  begin
    GetNextToken;
    IOCode := $E4 + Byte (CurrentIdentifierDataPtr^);        { $E4:  IN   AL, XX }
                                                             { $E5:  IN   AX, XX }
    ExpectTokenAndGetNext (Token_LeftBracket);
    ExpectIntegerExpression;
    CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
    ExpectTokenAndGetNext (Token_RightBracket);
    If (Location = elConstant) and (Value.As10Bytes [1] = 0) then
      GenerateInstruction_TwoBytes (IOCode, Value.Byte)
        else begin
               Calculate;
               LoadExpressionToRegisters (urDX);
               GenerateInstruction_Byte (IOCode or $08);     { $EC:  IN   AL, DX }
                                                             { $ED:  IN   AX, DX }
             end;
    EndIntermediateCodeSubroutine;
    DataType := [itUnsigned];
    If IOCode and $01 <> 0 then DataType := [itUnsigned, it16Bit];
    Location := elRegister;
    Include (UsedRegisters, urAX);
    LocationData.Register := rAX;
  end;
Search
Search
Home
Internals
Download
Compiler
Scanner
Symbol Tables
Parser
Expressions
Calculator
Statements
Assembler
System Functions
System Procedures
Type Definitions
Object Files
Code Generator
Linker
I/O Utilities
About
assembler
block
code
const
declarations
entry
exit
expressions
file
function
identifiers
instructions
integer
intermediate code
label
mathematical operations
new
offset
omf
pointer
procedure
program
read
source
statements
string
symbol table
symbol tables
type
unit

---

## Code Examples

### Code Block 1

```pascal
Procedure TExpression.Process_Mem;
Var SegmentExpression, OffsetExpression: TExpression;
begin
  GetNextToken;
  TypeDefPtr := PointerFromOffsets (PUnitOffsets (CurrentIdentifierDataPtr)^);
  DataType := TypeDefPtr^.DataType;
  ExpectTokenAndGetNext (Token_LeftBracket);
  SegmentExpression.ExpectIntegerExpression;
  SegmentExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_Colon);
  OffsetExpression.ExpectIntegerExpression;
  OffsetExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_RightBracket);
  SegmentExpression.Calculate;
  SegmentExpression.LoadExpressionToRegisters (urAX);
  GenerateInstruction_Byte (PUSH_AX);
  OffsetExpression.Calculate;
  OffsetExpression.LoadExpressionToRegisters (urDI);
  GenerateInstruction_Byte (POP_ES);
  ES_DI_PointerDestroyed;
  EndIntermediateCodeSubroutine;
  UsedRegisters := (SegmentExpression.UsedRegisters + OffsetExpression.UsedRegisters) * [urBX, urDX, urCX, urAX];
  LocationData.Flags := [ofsDI, segES];
  Value.Word := 0;
  Value.W12 := 0;
  Value.W14 := 0;
  Location := elMemory;
end;
```

### Code Block 2

```pascal
  Procedure Process_Port;
  Var IOCode: Byte;
  begin
    GetNextToken;
    IOCode := $E4 + Byte (CurrentIdentifierDataPtr^);        { $E4:  IN   AL, XX }
                                                             { $E5:  IN   AX, XX }
    ExpectTokenAndGetNext (Token_LeftBracket);
    ExpectIntegerExpression;
    CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
    ExpectTokenAndGetNext (Token_RightBracket);
    If (Location = elConstant) and (Value.As10Bytes [1] = 0) then
      GenerateInstruction_TwoBytes (IOCode, Value.Byte)
        else begin
               Calculate;
               LoadExpressionToRegisters (urDX);
               GenerateInstruction_Byte (IOCode or $08);     { $EC:  IN   AL, DX }
                                                             { $ED:  IN   AX, DX }
             end;
    EndIntermediateCodeSubroutine;
    DataType := [itUnsigned];
    If IOCode and $01 <> 0 then DataType := [itUnsigned, it16Bit];
    Location := elRegister;
    Include (UsedRegisters, urAX);
    LocationData.Register := rAX;
  end;
```

### Code Block 3

```pascal
Procedure TExpression.Process_Mem;
Var SegmentExpression, OffsetExpression: TExpression;
begin
  GetNextToken;
  TypeDefPtr := PointerFromOffsets (PUnitOffsets (CurrentIdentifierDataPtr)^);
  DataType := TypeDefPtr^.DataType;
  ExpectTokenAndGetNext (Token_LeftBracket);
  SegmentExpression.ExpectIntegerExpression;
  SegmentExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_Colon);
  OffsetExpression.ExpectIntegerExpression;
  OffsetExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_RightBracket);
  SegmentExpression.Calculate;
  SegmentExpression.LoadExpressionToRegisters (urAX);
  GenerateInstruction_Byte (PUSH_AX);
  OffsetExpression.Calculate;
  OffsetExpression.LoadExpressionToRegisters (urDI);
  GenerateInstruction_Byte (POP_ES);
  ES_DI_PointerDestroyed;
  EndIntermediateCodeSubroutine;
  UsedRegisters := (SegmentExpression.UsedRegisters + OffsetExpression.UsedRegisters) * [urBX, urDX, urCX, urAX];
  LocationData.Flags := [ofsDI, segES];
  Value.Word := 0;
  Value.W12 := 0;
  Value.W14 := 0;
  Location := elMemory;
end;
```

### Code Block 4

```pascal
  Procedure Process_Port;
  Var IOCode: Byte;
  begin
    GetNextToken;
    IOCode := $E4 + Byte (CurrentIdentifierDataPtr^);        { $E4:  IN   AL, XX }
                                                             { $E5:  IN   AX, XX }
    ExpectTokenAndGetNext (Token_LeftBracket);
    ExpectIntegerExpression;
    CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
    ExpectTokenAndGetNext (Token_RightBracket);
    If (Location = elConstant) and (Value.As10Bytes [1] = 0) then
      GenerateInstruction_TwoBytes (IOCode, Value.Byte)
        else begin
               Calculate;
               LoadExpressionToRegisters (urDX);
               GenerateInstruction_Byte (IOCode or $08);     { $EC:  IN   AL, DX }
                                                             { $ED:  IN   AX, DX }
             end;
    EndIntermediateCodeSubroutine;
    DataType := [itUnsigned];
    If IOCode and $01 <> 0 then DataType := [itUnsigned, it16Bit];
    Location := elRegister;
    Include (UsedRegisters, urAX);
    LocationData.Register := rAX;
  end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/port-statement
- https://turbopascal.org/processing-mem-and-port-arrays
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/system-functions
- https://turbopascal.org/system-procedures
- https://turbopascal.org/tag/assembler
- https://turbopascal.org/tag/block
- https://turbopascal.org/tag/code
- https://turbopascal.org/tag/const
- https://turbopascal.org/tag/declarations
- https://turbopascal.org/tag/entry
- https://turbopascal.org/tag/exit
- https://turbopascal.org/tag/expressions
- https://turbopascal.org/tag/file
- https://turbopascal.org/tag/function
- https://turbopascal.org/tag/identifiers
- https://turbopascal.org/tag/instructions
- https://turbopascal.org/tag/integer
- https://turbopascal.org/tag/intermediate-code
- https://turbopascal.org/tag/label
- https://turbopascal.org/tag/mathematical-operations
- https://turbopascal.org/tag/new
- https://turbopascal.org/tag/offset
- https://turbopascal.org/tag/omf
- https://turbopascal.org/tag/pointer
- https://turbopascal.org/tag/procedure
- https://turbopascal.org/tag/program
- https://turbopascal.org/tag/read
- https://turbopascal.org/tag/source
- https://turbopascal.org/tag/statements
- https://turbopascal.org/tag/string
- https://turbopascal.org/tag/symbol-table
- https://turbopascal.org/tag/symbol-tables
- https://turbopascal.org/tag/type
- https://turbopascal.org/tag/unit
- https://turbopascal.org/turbo-pascal-download
- https://turbopascal.org/turbo-pascal-internals
- https://turbopascal.org/type-definitions

