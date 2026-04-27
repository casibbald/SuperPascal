# Type Definition Data Structures - Turbo Pascal Compiler

**URL:** https://turbopascal.org/type-definition-data-structures

---

## Table of Contents

- Type Definition Data Structures

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Type Definition Data Structures
Here are presented data structures that describe types in Turbo Pascal. All types belong to one of base Turbo Pascal types:
Untyped
,
Array
,
Record
,
Object
,
File
,
Text
,
Function
/
Procedure
,
Set
,
Pointer
,
String
,
Extended
,
Real
,
Integer
,
Boolean
,
Char
,
Enumeration
.
Type TBaseType = (btUntyped, btArray, btRecord, btObject, btFile, btText, btFunctionProcedure, btSet,
                  btPointer, btString, btExtended, btReal, btInteger, btBoolean, btChar, btEnumeration);
Integer types have additional flags that determine integer properties.
TIntegerType = (itSigned, itUnsigned, it16Bit, it32Bit, it32Bytes);

     TIntegerTypeSet = Set of TIntegerType;
Each Turbo Pascal compiler data record that needs to specify a type uses a type definition record with two offsets. One offset defines unit where the type is defined. It specifies the offset in current
main symbol table
where the name of the unit is stored. The other offset specifies offset in the
main symbol table
of the target unit where type identifier is stored. All indentifiers in the symbol table are followed by
some data
which defines their properties. This way a complete reference to the type definition data is provided.
PUnitOffsets = ^TUnitOffsets;
     TUnitOffsets = Record
                      Case Byte of     { UnitIdentifierDataOffset }
                        0: (TypeOffset, UnitIdentifierData: Word);
                        1: (Offset, Segment: Word);
                        2: (UnitAndTypeOffset: LongInt);
                        3: (PreviousPointerToTypeDefinitionOffset, TemporarySymbolTableRecordOffset: Word);
                        4: (TypedConstantOffset, TypedConstantsBlockRecordOffset: Word);
                    end;
This procedure creates pointer to type definition record from type definition offsets.
Function PointerFromOffsets (Var UnitTypeOffsets: TUnitOffsets): Pointer;
begin
  PointerFromOffsets := Ptr (
    PUnitIdentifierData (Ptr (Seg (UnitTypeOffsets), UnitTypeOffsets.UnitIdentifierData))^.UnitSegment,
    UnitTypeOffsets.TypeOffset);
end;
All base types share the same basic fields, however, every base type uses some additional data fields.
ype PTypeDefinition = ^TTypeDefinition;
     TTypeDefinition = Object
                         BaseType: TBaseType;
                         DataType: TIntegerTypeSet;
                         Size: Word;
                         TypeIdentifierOffset: Word;
                         W06_: Word;  { NextMemberOffset }
                       end;

     PArrayTypeDefinition = ^TArrayTypeDefinition;                    { btArray }
     TArrayTypeDefinition = Object (TTypeDefinition)
                              ElementTypeOffset: TUnitOffsets;
                              IndexTypeOffset:   TUnitOffsets;
                              Function  IsZeroBasedCharacterArray: Boolean;
                              Function  IsCharacterArrayCompatibleWithString (Var Len: Byte): Boolean;
                            end;

     PRecordTypeDefinition = ^TRecordTypeDefinition;                  { btRecord }
     TRecordTypeDefinition = Object (TTypeDefinition)
                               FieldsListOffset: Word;
                               W0A:              Word;
                             end;

     PObjectTypeDefinition = ^TObjectTypeDefinition;                  { btObject }
     TObjectTypeDefinition = Object (TRecordTypeDefinition)
                               AncestorTypeOffset                 : TUnitOffsets;
                               VMT_Size                           : Word;
                               VMT_TypedConstantsBlockRecordOffset: Word;
                               OffsetOf_VMT_Offset: Word;
                               W16: Word;
                               W18: Word;
                               W1A: Word;
                               W1C: Word;
                               W1E: Word;
                               W20: Word;
                               W22: Word;
                               W24: Word;
                             end;

     PFileTypeDefinition = ^TFileTypeDefinition;                      { btFile }
     TFileTypeDefinition = Object (TTypeDefinition)
                             BaseFileTypeOffset: TUnitOffsets;
                           end;

     PProcedureTypeDefinition = ^TProcedureTypeDefinition;            { btProcedure }
     TProcedureTypeDefinition = Object (TTypeDefinition)
                                  ResultTypeOffset:   TUnitOffsets;
                                  NumberOfParameters: Word;
                                end;

     PSetTypeDefinition = ^TSetTypeDefinition;                        { btSet }
     TSetTypeDefinition = Object (TTypeDefinition)
                            BaseSetTypeOffset: TUnitOffsets;
                            Function  GetSetSizeAndLowestElementDataOffset (Var DataOffset: Byte): Byte;
                          end;

     PPointerTypeDefinition = ^TPointerTypeDefinition;                { btPointer }
     TPointerTypeDefinition = Object (TTypeDefinition)
                                PointerBaseTypeOffset: TUnitOffsets;
                              end;

     PStringTypeDefinition = PArrayTypeDefinition;                    { btString }
     TStringTypeDefinition = TArrayTypeDefinition;

     POrdinalTypeDefinition = ^TOrdinalTypeDefinition;                { btInteger, btBoolean, btChar, btEnumeration }
     TOrdinalTypeDefinition = Object (TTypeDefinition)
                                LowerLimit:  LongInt;
                                UpperLimit:  LongInt;
                                OrdinalType: TUnitOffsets;
                              end;
This procedure creates type definition record in the main symbol table and sets its data.
Function CreateTypeDefinition (Size, TypeSize: Word; IType: TIntegerTypeSet; BType: TBaseType): Pointer;
Var TypeDef: PTypeDefinition;
begin
  TypeDef := IncreaseSymbolTable (stMain, Size);
  CreateTypeDefinition := TypeDef;
  With TypeDef^ do
    begin
      BaseType := BType;
      DataType := IType;
      Size := TypeSize;
      TypeIdentifierOffset := 0;
      W06_ := 0;
    end;
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
Type TBaseType = (btUntyped, btArray, btRecord, btObject, btFile, btText, btFunctionProcedure, btSet,
                  btPointer, btString, btExtended, btReal, btInteger, btBoolean, btChar, btEnumeration);
```

### Code Block 2

```pascal
     TIntegerType = (itSigned, itUnsigned, it16Bit, it32Bit, it32Bytes);

     TIntegerTypeSet = Set of TIntegerType;
```

### Code Block 3

```pascal
     PUnitOffsets = ^TUnitOffsets;
     TUnitOffsets = Record
                      Case Byte of     { UnitIdentifierDataOffset }
                        0: (TypeOffset, UnitIdentifierData: Word);
                        1: (Offset, Segment: Word);
                        2: (UnitAndTypeOffset: LongInt);
                        3: (PreviousPointerToTypeDefinitionOffset, TemporarySymbolTableRecordOffset: Word);
                        4: (TypedConstantOffset, TypedConstantsBlockRecordOffset: Word);
                    end;
```

### Code Block 4

```pascal
Function PointerFromOffsets (Var UnitTypeOffsets: TUnitOffsets): Pointer;
begin
  PointerFromOffsets := Ptr (
    PUnitIdentifierData (Ptr (Seg (UnitTypeOffsets), UnitTypeOffsets.UnitIdentifierData))^.UnitSegment,
    UnitTypeOffsets.TypeOffset);
end;
```

### Code Block 5

```pascal
ype PTypeDefinition = ^TTypeDefinition;
     TTypeDefinition = Object
                         BaseType: TBaseType;
                         DataType: TIntegerTypeSet;
                         Size: Word;
                         TypeIdentifierOffset: Word;
                         W06_: Word;  { NextMemberOffset }
                       end;

     PArrayTypeDefinition = ^TArrayTypeDefinition;                    { btArray }
     TArrayTypeDefinition = Object (TTypeDefinition)
                              ElementTypeOffset: TUnitOffsets;
                              IndexTypeOffset:   TUnitOffsets;
                              Function  IsZeroBasedCharacterArray: Boolean;
                              Function  IsCharacterArrayCompatibleWithString (Var Len: Byte): Boolean;
                            end;

     PRecordTypeDefinition = ^TRecordTypeDefinition;                  { btRecord }
     TRecordTypeDefinition = Object (TTypeDefinition)
                               FieldsListOffset: Word;
                               W0A:              Word;
                             end;

     PObjectTypeDefinition = ^TObjectTypeDefinition;                  { btObject }
     TObjectTypeDefinition = Object (TRecordTypeDefinition)
                               AncestorTypeOffset                 : TUnitOffsets;
                               VMT_Size                           : Word;
                               VMT_TypedConstantsBlockRecordOffset: Word;
                               OffsetOf_VMT_Offset: Word;
                               W16: Word;
                               W18: Word;
                               W1A: Word;
                               W1C: Word;
                               W1E: Word;
                               W20: Word;
                               W22: Word;
                               W24: Word;
                             end;

     PFileTypeDefinition = ^TFileTypeDefinition;                      { btFile }
     TFileTypeDefinition = Object (TTypeDefinition)
                             BaseFileTypeOffset: TUnitOffsets;
                           end;

     PProcedureTypeDefinition = ^TProcedureTypeDefinition;            { btProcedure }
     TProcedureTypeDefinition = Object (TTypeDefinition)
                                  ResultTypeOffset:   TUnitOffsets;
                                  NumberOfParameters: Word;
                                end;

     PSetTypeDefinition = ^TSetTypeDefinition;                        { btSet }
     TSetTypeDefinition = Object (TTypeDefinition)
                            BaseSetTypeOffset: TUnitOffsets;
                            Function  GetSetSizeAndLowestElementDataOffset (Var DataOffset: Byte): Byte;
                          end;

     PPointerTypeDefinition = ^TPointerTypeDefinition;                { btPointer }
     TPointerTypeDefinition = Object (TTypeDefinition)
                                PointerBaseTypeOffset: TUnitOffsets;
                              end;

     PStringTypeDefinition = PArrayTypeDefinition;                    { btString }
     TStringTypeDefinition = TArrayTypeDefinition;

     POrdinalTypeDefinition = ^TOrdinalTypeDefinition;                { btInteger, btBoolean, btChar, btEnumeration }
     TOrdinalTypeDefinition = Object (TTypeDefinition)
                                LowerLimit:  LongInt;
                                UpperLimit:  LongInt;
                                OrdinalType: TUnitOffsets;
                              end;
```

### Code Block 6

```pascal
Function CreateTypeDefinition (Size, TypeSize: Word; IType: TIntegerTypeSet; BType: TBaseType): Pointer;
Var TypeDef: PTypeDefinition;
begin
  TypeDef := IncreaseSymbolTable (stMain, Size);
  CreateTypeDefinition := TypeDef;
  With TypeDef^ do
    begin
      BaseType := BType;
      DataType := IType;
      Size := TypeSize;
      TypeIdentifierOffset := 0;
      W06_ := 0;
    end;
end;
```

### Code Block 7

```pascal
Type TBaseType = (btUntyped, btArray, btRecord, btObject, btFile, btText, btFunctionProcedure, btSet,
                  btPointer, btString, btExtended, btReal, btInteger, btBoolean, btChar, btEnumeration);
```

### Code Block 8

```pascal
     TIntegerType = (itSigned, itUnsigned, it16Bit, it32Bit, it32Bytes);

     TIntegerTypeSet = Set of TIntegerType;
```

### Code Block 9

```pascal
     PUnitOffsets = ^TUnitOffsets;
     TUnitOffsets = Record
                      Case Byte of     { UnitIdentifierDataOffset }
                        0: (TypeOffset, UnitIdentifierData: Word);
                        1: (Offset, Segment: Word);
                        2: (UnitAndTypeOffset: LongInt);
                        3: (PreviousPointerToTypeDefinitionOffset, TemporarySymbolTableRecordOffset: Word);
                        4: (TypedConstantOffset, TypedConstantsBlockRecordOffset: Word);
                    end;
```

### Code Block 10

```pascal
Function PointerFromOffsets (Var UnitTypeOffsets: TUnitOffsets): Pointer;
begin
  PointerFromOffsets := Ptr (
    PUnitIdentifierData (Ptr (Seg (UnitTypeOffsets), UnitTypeOffsets.UnitIdentifierData))^.UnitSegment,
    UnitTypeOffsets.TypeOffset);
end;
```

### Code Block 11

```pascal
ype PTypeDefinition = ^TTypeDefinition;
     TTypeDefinition = Object
                         BaseType: TBaseType;
                         DataType: TIntegerTypeSet;
                         Size: Word;
                         TypeIdentifierOffset: Word;
                         W06_: Word;  { NextMemberOffset }
                       end;

     PArrayTypeDefinition = ^TArrayTypeDefinition;                    { btArray }
     TArrayTypeDefinition = Object (TTypeDefinition)
                              ElementTypeOffset: TUnitOffsets;
                              IndexTypeOffset:   TUnitOffsets;
                              Function  IsZeroBasedCharacterArray: Boolean;
                              Function  IsCharacterArrayCompatibleWithString (Var Len: Byte): Boolean;
                            end;

     PRecordTypeDefinition = ^TRecordTypeDefinition;                  { btRecord }
     TRecordTypeDefinition = Object (TTypeDefinition)
                               FieldsListOffset: Word;
                               W0A:              Word;
                             end;

     PObjectTypeDefinition = ^TObjectTypeDefinition;                  { btObject }
     TObjectTypeDefinition = Object (TRecordTypeDefinition)
                               AncestorTypeOffset                 : TUnitOffsets;
                               VMT_Size                           : Word;
                               VMT_TypedConstantsBlockRecordOffset: Word;
                               OffsetOf_VMT_Offset: Word;
                               W16: Word;
                               W18: Word;
                               W1A: Word;
                               W1C: Word;
                               W1E: Word;
                               W20: Word;
                               W22: Word;
                               W24: Word;
                             end;

     PFileTypeDefinition = ^TFileTypeDefinition;                      { btFile }
     TFileTypeDefinition = Object (TTypeDefinition)
                             BaseFileTypeOffset: TUnitOffsets;
                           end;

     PProcedureTypeDefinition = ^TProcedureTypeDefinition;            { btProcedure }
     TProcedureTypeDefinition = Object (TTypeDefinition)
                                  ResultTypeOffset:   TUnitOffsets;
                                  NumberOfParameters: Word;
                                end;

     PSetTypeDefinition = ^TSetTypeDefinition;                        { btSet }
     TSetTypeDefinition = Object (TTypeDefinition)
                            BaseSetTypeOffset: TUnitOffsets;
                            Function  GetSetSizeAndLowestElementDataOffset (Var DataOffset: Byte): Byte;
                          end;

     PPointerTypeDefinition = ^TPointerTypeDefinition;                { btPointer }
     TPointerTypeDefinition = Object (TTypeDefinition)
                                PointerBaseTypeOffset: TUnitOffsets;
                              end;

     PStringTypeDefinition = PArrayTypeDefinition;                    { btString }
     TStringTypeDefinition = TArrayTypeDefinition;

     POrdinalTypeDefinition = ^TOrdinalTypeDefinition;                { btInteger, btBoolean, btChar, btEnumeration }
     TOrdinalTypeDefinition = Object (TTypeDefinition)
                                LowerLimit:  LongInt;
                                UpperLimit:  LongInt;
                                OrdinalType: TUnitOffsets;
                              end;
```

### Code Block 12

```pascal
Function CreateTypeDefinition (Size, TypeSize: Word; IType: TIntegerTypeSet; BType: TBaseType): Pointer;
Var TypeDef: PTypeDefinition;
begin
  TypeDef := IncreaseSymbolTable (stMain, Size);
  CreateTypeDefinition := TypeDef;
  With TypeDef^ do
    begin
      BaseType := BType;
      DataType := IType;
      Size := TypeSize;
      TypeIdentifierOffset := 0;
      W06_ := 0;
    end;
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
- https://turbopascal.org/storing-identifiers-into-symbol-table
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/symbol-tables-and-hash-functions
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
- https://turbopascal.org/type-definition-data-structures
- https://turbopascal.org/type-definitions

