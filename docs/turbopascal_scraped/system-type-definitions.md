# System Type Definitions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-type-definitions

---

## Table of Contents

- System Type Definitions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Type Definitions
Here are listed all Turbo Pascal elementary types that are defined in the
System
unit (using the boot-strap symbol table
SYSTEM.TPS
).
With NoTypeDefinition do
        begin
          BaseType             := btUntyped;
          DataType             := [];
          Size                 := 0;
          TypeIdentifierOffset := 0;
          W06_       := 1;
        end;

      With FileTypeDefinition do
        begin
          BaseType             := btFile;
          DataType             := [];
          Size                 := $0080;
          TypeIdentifierOffset := 0;
          W06_     := $0019;

          BaseFileTypeOffset.TypeOffset               := 0;
          BaseFileTypeOffset.UnitIdentifierData := 0;
        end;

      With TextTypeDefinition do
        begin
          BaseType             := btText;
          DataType             := [];
          Size                 := $0100;
          TypeIdentifierOffset := Text_TypeNameOffset;
          W06_     := $0018;

          BaseFileTypeOffset.TypeOffset               := Char_TypeOffset;
          BaseFileTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With EmptySetTypeDefinition do
        begin
          BaseType             := btSet;
          DataType             := [it32Bytes];
          Size                 := 32;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          BaseSetTypeOffset.TypeOffset               := 0;
          BaseSetTypeOffset.UnitIdentifierData := 0;
        end;

      With PointerTypeDefinition do
        begin
          BaseType             := btPointer;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := Pointer_TypeNameOffset;
          W06_     := $001A;

          PointerBaseTypeOffset.TypeOffset         := Untyped_TypeOffset;
          PointerBaseTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With StringTypeDefinition do
        begin
          BaseType             := btString;
          DataType             := [];
          Size                 := $0100;
          TypeIdentifierOffset := 0;
          W06_     := $001B;

          ElementTypeOffset.TypeOffset         := Char_TypeOffset;
          ElementTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          IndexTypeOffset.TypeOffset           := Byte_TypeOffset;
          IndexTypeOffset.UnitIdentifierData   := SystemUnitIdentifierDataOffset;
        end;

      With OpenStringTypeDefinition do
        begin
          BaseType             := btString;
          DataType             := [];
          Size                 := 0;
          TypeIdentifierOffset := OpenString_NameOffset;
          W06_     := $001B;

          ElementTypeOffset.TypeOffset         := Char_TypeOffset;
          ElementTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          IndexTypeOffset.TypeOffset           := Byte_TypeOffset;
          IndexTypeOffset.UnitIdentifierData   := SystemUnitIdentifierDataOffset;
        end;

      With SingleTypeDefinition do
        begin
          BaseType             := btExtended;
          DataType             := [];
          Size                 := 4;
          TypeIdentifierOffset := Single_TypeNameOffset;
          W06_     := $000E;
        end;

      With DoubleTypeDefinition do
        begin
          BaseType             := btExtended;
          DataType             := [fpDouble];
          Size                 := 8;
          TypeIdentifierOffset := Double_TypeNameOffset;
          W06_     := $000F;
        end;

      With CompTypeDefinition do
        begin
          BaseType             := btExtended;
          DataType             := fpComp;
          Size                 := 8;
          TypeIdentifierOffset := Comp_TypeNameOffset;
          W06_     := $0015;
        end;

      With ExtendedTypeDefinition do
        begin
          BaseType             := btExtended;
          DataType             := [fpExtended];
          Size                 := 10;
          TypeIdentifierOffset := Extended_TypeNameOffset;
          W06_     := $0010;
        end;

      With RealTypeDefinition do
        begin
          BaseType             := btReal;
          DataType             := [fpDouble];
          Size                 := 6;
          TypeIdentifierOffset := Real_TypeNameOffset;
          W06_     := $0011;
        end;

      With ShortIntTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := [itSigned];
          Size                 := 1;
          TypeIdentifierOffset := ShortInt_TypeNameOffset;
          W06_     := $0002;

          LowerLimit           := Integer ($FFFFFF80);
          UpperLimit           := $0000007F;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With ByteTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := [itUnsigned];
          Size                 := 1;
          TypeIdentifierOffset := Byte_TypeNameOffset;
          W06_     := $0008;

          LowerLimit           := $00000000;
          UpperLimit           := $000000FF;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With IntegerTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := [it16Bit, itSigned];
          Size                 := 2;
          TypeIdentifierOffset := Integer_TypeNameOffset;
          W06_     := $0004;

          LowerLimit           := Integer ($FFFF8000);
          UpperLimit           := $00007FFF;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With WordTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := itWord;
          Size                 := 2;
          TypeIdentifierOffset := Word_TypeNameOffset;
          W06_     := $000A;

          LowerLimit           := $00000000;
          UpperLimit           := $0000FFFF;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With LongIntTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := LongInt_TypeNameOffset;
          W06_     := $0006;

          LowerLimit           := LongInt ($80000000);
          UpperLimit           := $7FFFFFFF;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With SetOfByteTypeDefinition do
        begin
          BaseType             := btSet;
          DataType             := [it32Bytes];
          Size                 := 32;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          BaseSetTypeOffset.TypeOffset         := Byte_TypeOffset;
          BaseSetTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With BooleanTypeDefinition do
        begin
          BaseType             := btBoolean;
          DataType             := [];
          Size                 := 1;
          TypeIdentifierOffset := Boolean_TypeNameOffset;
          W06_     := $0012;

          LowerLimit           := $00000000;
          UpperLimit           := $00000001;
          OrdinalType.TypeOffset         := LongBool_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With ByteBoolTypeDefinition do
        begin
          BaseType             := btBoolean;
          DataType             := [it32Bytes];
          Size                 := 1;
          TypeIdentifierOffset := ByteBool_TypeNameOffset;
          W06_     := $0012;

          LowerLimit           := $00000000;
          UpperLimit           := $00000001;
          OrdinalType.TypeOffset         := LongBool_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With WordBoolTypeDefinition do
        begin
          BaseType             := btBoolean;
          DataType             := [it32Bytes, it16Bit];
          Size                 := 2;
          TypeIdentifierOffset := WordBool_TypeNameOffset;
          W06_     := $0021;

          LowerLimit           := $00000000;
          UpperLimit           := $00000001;
          OrdinalType.TypeOffset         := LongBool_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With LongBoolTypeDefinition do
        begin
          BaseType             := btBoolean;
          DataType             := [it32Bytes, it32Bit, it16Bit];
          Size                 := 4;
          TypeIdentifierOffset := LongBool_TypeNameOffset;
          W06_     := $0022;

          LowerLimit           := $00000000;
          UpperLimit           := $00000001;
          OrdinalType.TypeOffset         := LongBool_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With SetOfBooleanTypeDefinition do
        begin
          BaseType             := btSet;
          DataType             := [it32Bytes];
          Size                 := 32;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          BaseSetTypeOffset.TypeOffset         := Boolean_TypeOffset;
          BaseSetTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With CharTypeDefinition do
        begin
          BaseType             := btChar;
          DataType             := [itUnsigned];
          Size                 := 1;
          TypeIdentifierOffset := Char_TypeNameOffset;
          W06_     := $0013;

          LowerLimit           := $00000000;
          UpperLimit           := $000000FF;
          OrdinalType.TypeOffset         := Char_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With SetOfCharTypeDefinition do
        begin
          BaseType             := btSet;
          DataType             := [it32Bytes];
          Size                 := 32;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          BaseSetTypeOffset.TypeOffset         := Char_TypeOffset;
          BaseSetTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With PCharTypeDefinition do
        begin
          BaseType             := btPointer;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := PChar_TypeNameOffset;
          W06_     := $0023;

          PointerBaseTypeOffset.TypeOffset         := Char_TypeOffset;
          PointerBaseTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With ProcedureTypeDefinition do
        begin
          BaseType             := btFunctionProcedure;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          ResultTypeOffset.TypeOffset         := Pointer_TypeOffset;
          ResultTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          NumberOfParameters                  := 1;
        end;
      With ProcedureParameterData do
        begin
          UnitTypeOffsets.TypeOffset         := Word_TypeOffset;
          UnitTypeOffsets.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          VarFlags                           := LocalStackVariable;
        end;

      With FunctionTypeDefinition do
        begin
          BaseType             := btFunctionProcedure;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := 0;
          W06_     := 0;
          ResultTypeOffset.TypeOffset         := 0;
          ResultTypeOffset.UnitIdentifierData := 0;
          NumberOfParameters                  := 1;
        end;
      With FunctionParameterData do
        begin
          UnitTypeOffsets.TypeOffset         := Pointer_TypeOffset;
          UnitTypeOffsets.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          VarFlags                           := LocalStackVariable;
        end;

      With Function2TypeDefinition do
        begin
          BaseType             := btFunctionProcedure;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          ResultTypeOffset.TypeOffset         := 0;
          ResultTypeOffset.UnitIdentifierData := 0;
          NumberOfParameters                  := 2;
        end;
      With Function2ParameterData1 do
        begin
          UnitTypeOffsets.TypeOffset         := Pointer_TypeOffset;
          UnitTypeOffsets.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          VarFlags                           := LocalStackVariable;
        end;
      With Function2ParameterData2 do
        begin
          UnitTypeOffsets.TypeOffset         := Word_TypeOffset;
          UnitTypeOffsets.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          VarFlags                           := LocalStackVariable;
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
      With NoTypeDefinition do
        begin
          BaseType             := btUntyped;
          DataType             := [];
          Size                 := 0;
          TypeIdentifierOffset := 0;
          W06_       := 1;
        end;

      With FileTypeDefinition do
        begin
          BaseType             := btFile;
          DataType             := [];
          Size                 := $0080;
          TypeIdentifierOffset := 0;
          W06_     := $0019;

          BaseFileTypeOffset.TypeOffset               := 0;
          BaseFileTypeOffset.UnitIdentifierData := 0;
        end;

      With TextTypeDefinition do
        begin
          BaseType             := btText;
          DataType             := [];
          Size                 := $0100;
          TypeIdentifierOffset := Text_TypeNameOffset;
          W06_     := $0018;

          BaseFileTypeOffset.TypeOffset               := Char_TypeOffset;
          BaseFileTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With EmptySetTypeDefinition do
        begin
          BaseType             := btSet;
          DataType             := [it32Bytes];
          Size                 := 32;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          BaseSetTypeOffset.TypeOffset               := 0;
          BaseSetTypeOffset.UnitIdentifierData := 0;
        end;

      With PointerTypeDefinition do
        begin
          BaseType             := btPointer;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := Pointer_TypeNameOffset;
          W06_     := $001A;

          PointerBaseTypeOffset.TypeOffset         := Untyped_TypeOffset;
          PointerBaseTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With StringTypeDefinition do
        begin
          BaseType             := btString;
          DataType             := [];
          Size                 := $0100;
          TypeIdentifierOffset := 0;
          W06_     := $001B;

          ElementTypeOffset.TypeOffset         := Char_TypeOffset;
          ElementTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          IndexTypeOffset.TypeOffset           := Byte_TypeOffset;
          IndexTypeOffset.UnitIdentifierData   := SystemUnitIdentifierDataOffset;
        end;

      With OpenStringTypeDefinition do
        begin
          BaseType             := btString;
          DataType             := [];
          Size                 := 0;
          TypeIdentifierOffset := OpenString_NameOffset;
          W06_     := $001B;

          ElementTypeOffset.TypeOffset         := Char_TypeOffset;
          ElementTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          IndexTypeOffset.TypeOffset           := Byte_TypeOffset;
          IndexTypeOffset.UnitIdentifierData   := SystemUnitIdentifierDataOffset;
        end;

      With SingleTypeDefinition do
        begin
          BaseType             := btExtended;
          DataType             := [];
          Size                 := 4;
          TypeIdentifierOffset := Single_TypeNameOffset;
          W06_     := $000E;
        end;

      With DoubleTypeDefinition do
        begin
          BaseType             := btExtended;
          DataType             := [fpDouble];
          Size                 := 8;
          TypeIdentifierOffset := Double_TypeNameOffset;
          W06_     := $000F;
        end;

      With CompTypeDefinition do
        begin
          BaseType             := btExtended;
          DataType             := fpComp;
          Size                 := 8;
          TypeIdentifierOffset := Comp_TypeNameOffset;
          W06_     := $0015;
        end;

      With ExtendedTypeDefinition do
        begin
          BaseType             := btExtended;
          DataType             := [fpExtended];
          Size                 := 10;
          TypeIdentifierOffset := Extended_TypeNameOffset;
          W06_     := $0010;
        end;

      With RealTypeDefinition do
        begin
          BaseType             := btReal;
          DataType             := [fpDouble];
          Size                 := 6;
          TypeIdentifierOffset := Real_TypeNameOffset;
          W06_     := $0011;
        end;

      With ShortIntTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := [itSigned];
          Size                 := 1;
          TypeIdentifierOffset := ShortInt_TypeNameOffset;
          W06_     := $0002;

          LowerLimit           := Integer ($FFFFFF80);
          UpperLimit           := $0000007F;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With ByteTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := [itUnsigned];
          Size                 := 1;
          TypeIdentifierOffset := Byte_TypeNameOffset;
          W06_     := $0008;

          LowerLimit           := $00000000;
          UpperLimit           := $000000FF;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With IntegerTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := [it16Bit, itSigned];
          Size                 := 2;
          TypeIdentifierOffset := Integer_TypeNameOffset;
          W06_     := $0004;

          LowerLimit           := Integer ($FFFF8000);
          UpperLimit           := $00007FFF;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With WordTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := itWord;
          Size                 := 2;
          TypeIdentifierOffset := Word_TypeNameOffset;
          W06_     := $000A;

          LowerLimit           := $00000000;
          UpperLimit           := $0000FFFF;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With LongIntTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := LongInt_TypeNameOffset;
          W06_     := $0006;

          LowerLimit           := LongInt ($80000000);
          UpperLimit           := $7FFFFFFF;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With SetOfByteTypeDefinition do
        begin
          BaseType             := btSet;
          DataType             := [it32Bytes];
          Size                 := 32;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          BaseSetTypeOffset.TypeOffset         := Byte_TypeOffset;
          BaseSetTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With BooleanTypeDefinition do
        begin
          BaseType             := btBoolean;
          DataType             := [];
          Size                 := 1;
          TypeIdentifierOffset := Boolean_TypeNameOffset;
          W06_     := $0012;

          LowerLimit           := $00000000;
          UpperLimit           := $00000001;
          OrdinalType.TypeOffset         := LongBool_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With ByteBoolTypeDefinition do
        begin
          BaseType             := btBoolean;
          DataType             := [it32Bytes];
          Size                 := 1;
          TypeIdentifierOffset := ByteBool_TypeNameOffset;
          W06_     := $0012;

          LowerLimit           := $00000000;
          UpperLimit           := $00000001;
          OrdinalType.TypeOffset         := LongBool_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With WordBoolTypeDefinition do
        begin
          BaseType             := btBoolean;
          DataType             := [it32Bytes, it16Bit];
          Size                 := 2;
          TypeIdentifierOffset := WordBool_TypeNameOffset;
          W06_     := $0021;

          LowerLimit           := $00000000;
          UpperLimit           := $00000001;
          OrdinalType.TypeOffset         := LongBool_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With LongBoolTypeDefinition do
        begin
          BaseType             := btBoolean;
          DataType             := [it32Bytes, it32Bit, it16Bit];
          Size                 := 4;
          TypeIdentifierOffset := LongBool_TypeNameOffset;
          W06_     := $0022;

          LowerLimit           := $00000000;
          UpperLimit           := $00000001;
          OrdinalType.TypeOffset         := LongBool_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With SetOfBooleanTypeDefinition do
        begin
          BaseType             := btSet;
          DataType             := [it32Bytes];
          Size                 := 32;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          BaseSetTypeOffset.TypeOffset         := Boolean_TypeOffset;
          BaseSetTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With CharTypeDefinition do
        begin
          BaseType             := btChar;
          DataType             := [itUnsigned];
          Size                 := 1;
          TypeIdentifierOffset := Char_TypeNameOffset;
          W06_     := $0013;

          LowerLimit           := $00000000;
          UpperLimit           := $000000FF;
          OrdinalType.TypeOffset         := Char_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With SetOfCharTypeDefinition do
        begin
          BaseType             := btSet;
          DataType             := [it32Bytes];
          Size                 := 32;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          BaseSetTypeOffset.TypeOffset         := Char_TypeOffset;
          BaseSetTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With PCharTypeDefinition do
        begin
          BaseType             := btPointer;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := PChar_TypeNameOffset;
          W06_     := $0023;

          PointerBaseTypeOffset.TypeOffset         := Char_TypeOffset;
          PointerBaseTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With ProcedureTypeDefinition do
        begin
          BaseType             := btFunctionProcedure;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          ResultTypeOffset.TypeOffset         := Pointer_TypeOffset;
          ResultTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          NumberOfParameters                  := 1;
        end;
      With ProcedureParameterData do
        begin
          UnitTypeOffsets.TypeOffset         := Word_TypeOffset;
          UnitTypeOffsets.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          VarFlags                           := LocalStackVariable;
        end;

      With FunctionTypeDefinition do
        begin
          BaseType             := btFunctionProcedure;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := 0;
          W06_     := 0;
          ResultTypeOffset.TypeOffset         := 0;
          ResultTypeOffset.UnitIdentifierData := 0;
          NumberOfParameters                  := 1;
        end;
      With FunctionParameterData do
        begin
          UnitTypeOffsets.TypeOffset         := Pointer_TypeOffset;
          UnitTypeOffsets.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          VarFlags                           := LocalStackVariable;
        end;

      With Function2TypeDefinition do
        begin
          BaseType             := btFunctionProcedure;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          ResultTypeOffset.TypeOffset         := 0;
          ResultTypeOffset.UnitIdentifierData := 0;
          NumberOfParameters                  := 2;
        end;
      With Function2ParameterData1 do
        begin
          UnitTypeOffsets.TypeOffset         := Pointer_TypeOffset;
          UnitTypeOffsets.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          VarFlags                           := LocalStackVariable;
        end;
      With Function2ParameterData2 do
        begin
          UnitTypeOffsets.TypeOffset         := Word_TypeOffset;
          UnitTypeOffsets.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          VarFlags                           := LocalStackVariable;
        end;
```

### Code Block 2

```pascal
      With NoTypeDefinition do
        begin
          BaseType             := btUntyped;
          DataType             := [];
          Size                 := 0;
          TypeIdentifierOffset := 0;
          W06_       := 1;
        end;

      With FileTypeDefinition do
        begin
          BaseType             := btFile;
          DataType             := [];
          Size                 := $0080;
          TypeIdentifierOffset := 0;
          W06_     := $0019;

          BaseFileTypeOffset.TypeOffset               := 0;
          BaseFileTypeOffset.UnitIdentifierData := 0;
        end;

      With TextTypeDefinition do
        begin
          BaseType             := btText;
          DataType             := [];
          Size                 := $0100;
          TypeIdentifierOffset := Text_TypeNameOffset;
          W06_     := $0018;

          BaseFileTypeOffset.TypeOffset               := Char_TypeOffset;
          BaseFileTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With EmptySetTypeDefinition do
        begin
          BaseType             := btSet;
          DataType             := [it32Bytes];
          Size                 := 32;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          BaseSetTypeOffset.TypeOffset               := 0;
          BaseSetTypeOffset.UnitIdentifierData := 0;
        end;

      With PointerTypeDefinition do
        begin
          BaseType             := btPointer;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := Pointer_TypeNameOffset;
          W06_     := $001A;

          PointerBaseTypeOffset.TypeOffset         := Untyped_TypeOffset;
          PointerBaseTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With StringTypeDefinition do
        begin
          BaseType             := btString;
          DataType             := [];
          Size                 := $0100;
          TypeIdentifierOffset := 0;
          W06_     := $001B;

          ElementTypeOffset.TypeOffset         := Char_TypeOffset;
          ElementTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          IndexTypeOffset.TypeOffset           := Byte_TypeOffset;
          IndexTypeOffset.UnitIdentifierData   := SystemUnitIdentifierDataOffset;
        end;

      With OpenStringTypeDefinition do
        begin
          BaseType             := btString;
          DataType             := [];
          Size                 := 0;
          TypeIdentifierOffset := OpenString_NameOffset;
          W06_     := $001B;

          ElementTypeOffset.TypeOffset         := Char_TypeOffset;
          ElementTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          IndexTypeOffset.TypeOffset           := Byte_TypeOffset;
          IndexTypeOffset.UnitIdentifierData   := SystemUnitIdentifierDataOffset;
        end;

      With SingleTypeDefinition do
        begin
          BaseType             := btExtended;
          DataType             := [];
          Size                 := 4;
          TypeIdentifierOffset := Single_TypeNameOffset;
          W06_     := $000E;
        end;

      With DoubleTypeDefinition do
        begin
          BaseType             := btExtended;
          DataType             := [fpDouble];
          Size                 := 8;
          TypeIdentifierOffset := Double_TypeNameOffset;
          W06_     := $000F;
        end;

      With CompTypeDefinition do
        begin
          BaseType             := btExtended;
          DataType             := fpComp;
          Size                 := 8;
          TypeIdentifierOffset := Comp_TypeNameOffset;
          W06_     := $0015;
        end;

      With ExtendedTypeDefinition do
        begin
          BaseType             := btExtended;
          DataType             := [fpExtended];
          Size                 := 10;
          TypeIdentifierOffset := Extended_TypeNameOffset;
          W06_     := $0010;
        end;

      With RealTypeDefinition do
        begin
          BaseType             := btReal;
          DataType             := [fpDouble];
          Size                 := 6;
          TypeIdentifierOffset := Real_TypeNameOffset;
          W06_     := $0011;
        end;

      With ShortIntTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := [itSigned];
          Size                 := 1;
          TypeIdentifierOffset := ShortInt_TypeNameOffset;
          W06_     := $0002;

          LowerLimit           := Integer ($FFFFFF80);
          UpperLimit           := $0000007F;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With ByteTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := [itUnsigned];
          Size                 := 1;
          TypeIdentifierOffset := Byte_TypeNameOffset;
          W06_     := $0008;

          LowerLimit           := $00000000;
          UpperLimit           := $000000FF;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With IntegerTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := [it16Bit, itSigned];
          Size                 := 2;
          TypeIdentifierOffset := Integer_TypeNameOffset;
          W06_     := $0004;

          LowerLimit           := Integer ($FFFF8000);
          UpperLimit           := $00007FFF;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With WordTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := itWord;
          Size                 := 2;
          TypeIdentifierOffset := Word_TypeNameOffset;
          W06_     := $000A;

          LowerLimit           := $00000000;
          UpperLimit           := $0000FFFF;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With LongIntTypeDefinition do
        begin
          BaseType             := btInteger;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := LongInt_TypeNameOffset;
          W06_     := $0006;

          LowerLimit           := LongInt ($80000000);
          UpperLimit           := $7FFFFFFF;
          OrdinalType.TypeOffset         := LongInt_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With SetOfByteTypeDefinition do
        begin
          BaseType             := btSet;
          DataType             := [it32Bytes];
          Size                 := 32;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          BaseSetTypeOffset.TypeOffset         := Byte_TypeOffset;
          BaseSetTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With BooleanTypeDefinition do
        begin
          BaseType             := btBoolean;
          DataType             := [];
          Size                 := 1;
          TypeIdentifierOffset := Boolean_TypeNameOffset;
          W06_     := $0012;

          LowerLimit           := $00000000;
          UpperLimit           := $00000001;
          OrdinalType.TypeOffset         := LongBool_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With ByteBoolTypeDefinition do
        begin
          BaseType             := btBoolean;
          DataType             := [it32Bytes];
          Size                 := 1;
          TypeIdentifierOffset := ByteBool_TypeNameOffset;
          W06_     := $0012;

          LowerLimit           := $00000000;
          UpperLimit           := $00000001;
          OrdinalType.TypeOffset         := LongBool_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With WordBoolTypeDefinition do
        begin
          BaseType             := btBoolean;
          DataType             := [it32Bytes, it16Bit];
          Size                 := 2;
          TypeIdentifierOffset := WordBool_TypeNameOffset;
          W06_     := $0021;

          LowerLimit           := $00000000;
          UpperLimit           := $00000001;
          OrdinalType.TypeOffset         := LongBool_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With LongBoolTypeDefinition do
        begin
          BaseType             := btBoolean;
          DataType             := [it32Bytes, it32Bit, it16Bit];
          Size                 := 4;
          TypeIdentifierOffset := LongBool_TypeNameOffset;
          W06_     := $0022;

          LowerLimit           := $00000000;
          UpperLimit           := $00000001;
          OrdinalType.TypeOffset         := LongBool_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With SetOfBooleanTypeDefinition do
        begin
          BaseType             := btSet;
          DataType             := [it32Bytes];
          Size                 := 32;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          BaseSetTypeOffset.TypeOffset         := Boolean_TypeOffset;
          BaseSetTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With CharTypeDefinition do
        begin
          BaseType             := btChar;
          DataType             := [itUnsigned];
          Size                 := 1;
          TypeIdentifierOffset := Char_TypeNameOffset;
          W06_     := $0013;

          LowerLimit           := $00000000;
          UpperLimit           := $000000FF;
          OrdinalType.TypeOffset         := Char_TypeOffset;
          OrdinalType.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With SetOfCharTypeDefinition do
        begin
          BaseType             := btSet;
          DataType             := [it32Bytes];
          Size                 := 32;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          BaseSetTypeOffset.TypeOffset         := Char_TypeOffset;
          BaseSetTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With PCharTypeDefinition do
        begin
          BaseType             := btPointer;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := PChar_TypeNameOffset;
          W06_     := $0023;

          PointerBaseTypeOffset.TypeOffset         := Char_TypeOffset;
          PointerBaseTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
        end;

      With ProcedureTypeDefinition do
        begin
          BaseType             := btFunctionProcedure;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          ResultTypeOffset.TypeOffset         := Pointer_TypeOffset;
          ResultTypeOffset.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          NumberOfParameters                  := 1;
        end;
      With ProcedureParameterData do
        begin
          UnitTypeOffsets.TypeOffset         := Word_TypeOffset;
          UnitTypeOffsets.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          VarFlags                           := LocalStackVariable;
        end;

      With FunctionTypeDefinition do
        begin
          BaseType             := btFunctionProcedure;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := 0;
          W06_     := 0;
          ResultTypeOffset.TypeOffset         := 0;
          ResultTypeOffset.UnitIdentifierData := 0;
          NumberOfParameters                  := 1;
        end;
      With FunctionParameterData do
        begin
          UnitTypeOffsets.TypeOffset         := Pointer_TypeOffset;
          UnitTypeOffsets.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          VarFlags                           := LocalStackVariable;
        end;

      With Function2TypeDefinition do
        begin
          BaseType             := btFunctionProcedure;
          DataType             := itLongInt;
          Size                 := 4;
          TypeIdentifierOffset := 0;
          W06_     := 0;

          ResultTypeOffset.TypeOffset         := 0;
          ResultTypeOffset.UnitIdentifierData := 0;
          NumberOfParameters                  := 2;
        end;
      With Function2ParameterData1 do
        begin
          UnitTypeOffsets.TypeOffset         := Pointer_TypeOffset;
          UnitTypeOffsets.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          VarFlags                           := LocalStackVariable;
        end;
      With Function2ParameterData2 do
        begin
          UnitTypeOffsets.TypeOffset         := Word_TypeOffset;
          UnitTypeOffsets.UnitIdentifierData := SystemUnitIdentifierDataOffset;
          VarFlags                           := LocalStackVariable;
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
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/system-functions
- https://turbopascal.org/system-procedures
- https://turbopascal.org/system-type-definitions
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

