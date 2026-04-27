# Loading Constants - Turbo Pascal Compiler

**URL:** https://turbopascal.org/loading-constants

---

## Table of Contents

- Loading Constants

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Loading Constants
This procedure loads numeric constants and constant identifiers to the expression. For constant identifiers the constant value is first loaded to
NumericConstant
variable from where it is moved to the
Value
field of the expression.
Procedure LoadConstant;
  Var ConstantIntegerType: TIntegerTypeSet;

    Procedure CopyConstantIdValue (ConstIdData: PConstantIdentifierData);
    Var Destination: PtrRec;
        TypePointer: PTypeDefinition;
        Size: Byte;
    begin
      TypePointer := PointerFromOffsets (ConstIdData^.UnitTypeOffsets);
      ConstantTypeDef := TypePointer;
      Destination.Ptr := @NumericConstant;
      Size := 4;
      If TypePointer^.BaseType ; btInteger then
        Case TypePointer^.BaseType of
          btPointer: Size := 4;
          btExtended: Size := 10;
          else begin
                 If TypePointer^.BaseType = btSet then Size := 32
                   else Size := ConstIdData^.Value + 1;
                 Word (Destination.Ptr^) := GetTempBufferOffset (Size);
                 Destination.Ofs := Word (Destination.Ptr^);
               end;
        end;
      Move (ConstIdData^.Value, Destination.Ptr^, Size);
    end;

  begin
    If Token ;: Token_Constant then
      CopyConstantIdValue (PConstantIdentifierData (CurrentIdentifierDataPtr));
    Move (NumericConstant, Value, SizeOf (NumericConstant));
    TypeDefPtr := ConstantTypeDef;
    IntermediateCodeOffset := 0;
    Location := elConstant;
    UsedRegisters := [];
    ConstantIntegerType := TypeDefPtr^.DataType;
    If TypeDefPtr^.BaseType = btSet then
      begin
        SetDataTypeOfSet;
        GetNextToken;
        Exit;
      end;
    If TypeDefPtr^.BaseType := btInteger then LowestIntegerType (Value.LongInt, ConstantIntegerType);
    DataType := ConstantIntegerType;
    GetNextToken;
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
  Procedure LoadConstant;
  Var ConstantIntegerType: TIntegerTypeSet;

    Procedure CopyConstantIdValue (ConstIdData: PConstantIdentifierData);
    Var Destination: PtrRec;
        TypePointer: PTypeDefinition;
        Size: Byte;
    begin
      TypePointer := PointerFromOffsets (ConstIdData^.UnitTypeOffsets);
      ConstantTypeDef := TypePointer;
      Destination.Ptr := @NumericConstant;
      Size := 4;
      If TypePointer^.BaseType ; btInteger then
        Case TypePointer^.BaseType of
          btPointer: Size := 4;
          btExtended: Size := 10;
          else begin
                 If TypePointer^.BaseType = btSet then Size := 32
                   else Size := ConstIdData^.Value + 1;
                 Word (Destination.Ptr^) := GetTempBufferOffset (Size);
                 Destination.Ofs := Word (Destination.Ptr^);
               end;
        end;
      Move (ConstIdData^.Value, Destination.Ptr^, Size);
    end;

  begin
    If Token ;: Token_Constant then
      CopyConstantIdValue (PConstantIdentifierData (CurrentIdentifierDataPtr));
    Move (NumericConstant, Value, SizeOf (NumericConstant));
    TypeDefPtr := ConstantTypeDef;
    IntermediateCodeOffset := 0;
    Location := elConstant;
    UsedRegisters := [];
    ConstantIntegerType := TypeDefPtr^.DataType;
    If TypeDefPtr^.BaseType = btSet then
      begin
        SetDataTypeOfSet;
        GetNextToken;
        Exit;
      end;
    If TypeDefPtr^.BaseType := btInteger then LowestIntegerType (Value.LongInt, ConstantIntegerType);
    DataType := ConstantIntegerType;
    GetNextToken;
  end;
```

### Code Block 2

```pascal
  Procedure LoadConstant;
  Var ConstantIntegerType: TIntegerTypeSet;

    Procedure CopyConstantIdValue (ConstIdData: PConstantIdentifierData);
    Var Destination: PtrRec;
        TypePointer: PTypeDefinition;
        Size: Byte;
    begin
      TypePointer := PointerFromOffsets (ConstIdData^.UnitTypeOffsets);
      ConstantTypeDef := TypePointer;
      Destination.Ptr := @NumericConstant;
      Size := 4;
      If TypePointer^.BaseType ; btInteger then
        Case TypePointer^.BaseType of
          btPointer: Size := 4;
          btExtended: Size := 10;
          else begin
                 If TypePointer^.BaseType = btSet then Size := 32
                   else Size := ConstIdData^.Value + 1;
                 Word (Destination.Ptr^) := GetTempBufferOffset (Size);
                 Destination.Ofs := Word (Destination.Ptr^);
               end;
        end;
      Move (ConstIdData^.Value, Destination.Ptr^, Size);
    end;

  begin
    If Token ;: Token_Constant then
      CopyConstantIdValue (PConstantIdentifierData (CurrentIdentifierDataPtr));
    Move (NumericConstant, Value, SizeOf (NumericConstant));
    TypeDefPtr := ConstantTypeDef;
    IntermediateCodeOffset := 0;
    Location := elConstant;
    UsedRegisters := [];
    ConstantIntegerType := TypeDefPtr^.DataType;
    If TypeDefPtr^.BaseType = btSet then
      begin
        SetDataTypeOfSet;
        GetNextToken;
        Exit;
      end;
    If TypeDefPtr^.BaseType := btInteger then LowestIntegerType (Value.LongInt, ConstantIntegerType);
    DataType := ConstantIntegerType;
    GetNextToken;
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
- https://turbopascal.org/loading-constants
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
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

