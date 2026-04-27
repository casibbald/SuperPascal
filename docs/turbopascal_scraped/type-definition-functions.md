# Type Definition Functions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/type-definition-functions

---

## Table of Contents

- Type Definition Functions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Type Definition Functions
Array and set definitions use few functons for type checking.
This function checks if type is zero-based character array:
Base type must be
Array
Element type must be
System.Char
Size must be non zero
Index base type must be
btInteger
Index lower limit must be zero
Function TArrayTypeDefinition.IsZeroBasedCharacterArray: Boolean;
begin
  IsZeroBasedCharacterArray := False;
  If BaseType <> btArray then Exit;
  If ElementTypeOffset.TypeOffset <> Char_TypeOffset then Exit;
  If PUnitIdentifierData (Ptr (Seg (Self), ElementTypeOffset.UnitIdentifierData))^.UnitSegment <>
       SystemUnitSegment then Exit;
  If Size < 1 then Exit;
  With POrdinalTypeDefinition (PointerFromOffsets (IndexTypeOffset))^ do
    begin
      If BaseType <> btInteger then Exit;
      If LowerLimit <> 0 then Exit;
    end;
  IsZeroBasedCharacterArray := True
end;
This function checks if type is character array compatible with string:
Base type must be
Array
Element base type must be
Char
Size must be between 1 and 255
Variable
Len
returns the actual array size.
Function TArrayTypeDefinition.IsCharacterArrayCompatibleWithString (Var Len: Byte): Boolean;
begin
  IsCharacterArrayCompatibleWithString := False;
  If BaseType <> btArray then Exit;
  If PTypeDefinition (PointerFromOffsets (ElementTypeOffset))^.BaseType <> btChar then Exit;
  If Size < 1 then Exit;
  If Size > 255 then Exit;
  Len := Size;
  IsCharacterArrayCompatibleWithString := True;
end;
This function returns set size and offset of the lowest element in set.
Function TSetTypeDefinition.GetSetSizeAndLowestElementDataOffset (Var DataOffset: Byte): Byte;
begin
  GetSetSizeAndLowestElementDataOffset := Size;
  DataOffset := POrdinalTypeDefinition (PointerFromOffsets (BaseSetTypeOffset))^.LowerLimit shr 3;
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
Function TArrayTypeDefinition.IsZeroBasedCharacterArray: Boolean;
begin
  IsZeroBasedCharacterArray := False;
  If BaseType <> btArray then Exit;
  If ElementTypeOffset.TypeOffset <> Char_TypeOffset then Exit;
  If PUnitIdentifierData (Ptr (Seg (Self), ElementTypeOffset.UnitIdentifierData))^.UnitSegment <>
       SystemUnitSegment then Exit;
  If Size < 1 then Exit;
  With POrdinalTypeDefinition (PointerFromOffsets (IndexTypeOffset))^ do
    begin
      If BaseType <> btInteger then Exit;
      If LowerLimit <> 0 then Exit;
    end;
  IsZeroBasedCharacterArray := True
end;
```

### Code Block 2

```pascal
Function TArrayTypeDefinition.IsCharacterArrayCompatibleWithString (Var Len: Byte): Boolean;
begin
  IsCharacterArrayCompatibleWithString := False;
  If BaseType <> btArray then Exit;
  If PTypeDefinition (PointerFromOffsets (ElementTypeOffset))^.BaseType <> btChar then Exit;
  If Size < 1 then Exit;
  If Size > 255 then Exit;
  Len := Size;
  IsCharacterArrayCompatibleWithString := True;
end;
```

### Code Block 3

```pascal
Function TSetTypeDefinition.GetSetSizeAndLowestElementDataOffset (Var DataOffset: Byte): Byte;
begin
  GetSetSizeAndLowestElementDataOffset := Size;
  DataOffset := POrdinalTypeDefinition (PointerFromOffsets (BaseSetTypeOffset))^.LowerLimit shr 3;
end;
```

### Code Block 4

```pascal
Function TArrayTypeDefinition.IsZeroBasedCharacterArray: Boolean;
begin
  IsZeroBasedCharacterArray := False;
  If BaseType <> btArray then Exit;
  If ElementTypeOffset.TypeOffset <> Char_TypeOffset then Exit;
  If PUnitIdentifierData (Ptr (Seg (Self), ElementTypeOffset.UnitIdentifierData))^.UnitSegment <>
       SystemUnitSegment then Exit;
  If Size < 1 then Exit;
  With POrdinalTypeDefinition (PointerFromOffsets (IndexTypeOffset))^ do
    begin
      If BaseType <> btInteger then Exit;
      If LowerLimit <> 0 then Exit;
    end;
  IsZeroBasedCharacterArray := True
end;
```

### Code Block 5

```pascal
Function TArrayTypeDefinition.IsCharacterArrayCompatibleWithString (Var Len: Byte): Boolean;
begin
  IsCharacterArrayCompatibleWithString := False;
  If BaseType <> btArray then Exit;
  If PTypeDefinition (PointerFromOffsets (ElementTypeOffset))^.BaseType <> btChar then Exit;
  If Size < 1 then Exit;
  If Size > 255 then Exit;
  Len := Size;
  IsCharacterArrayCompatibleWithString := True;
end;
```

### Code Block 6

```pascal
Function TSetTypeDefinition.GetSetSizeAndLowestElementDataOffset (Var DataOffset: Byte): Byte;
begin
  GetSetSizeAndLowestElementDataOffset := Size;
  DataOffset := POrdinalTypeDefinition (PointerFromOffsets (BaseSetTypeOffset))^.LowerLimit shr 3;
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
- https://turbopascal.org/type-definition-functions
- https://turbopascal.org/type-definitions

