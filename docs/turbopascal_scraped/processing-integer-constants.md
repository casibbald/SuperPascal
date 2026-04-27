# Processing Integer Constants - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-integer-constants

---

## Table of Contents

- Processing Integer Constants

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Integer Constants
These two functions check and process decimal and hexadecimal integer constants. The value is stored in the
IntegerNumericConstant
variable.
Function IsIntegerNumericConstant (Var CharacterPointer: PChar): Boolean;
begin
  If CharacterPointer [0] = '$' then
    begin
      Inc (CharacterPointer);
      IsIntegerNumericConstant := IsHexConstant (CharacterPointer);
      Exit;
    end;
  IntegerNumericConstant := 0;
  IsIntegerNumericConstant := False;
  While CharacterPointer [0] in DecChars do
    begin
      If IntegerNumericConstantRec.WordH and $F000 <> 0 then
        begin
          IsIntegerNumericConstant := False;
          Exit;
        end;
      IntegerNumericConstant := IntegerNumericConstant * 10 + Ord (CharacterPointer [0]) - Ord ('0');;
      If IntegerNumericConstantRec.WordH and $8000 <> 0 then    { BUG in TPC ?                 }
        begin                                                   { -2147483648 causes error !!! }
          IsIntegerNumericConstant := False;
          Exit;
        end;
      Inc (CharacterPointer);
      IsIntegerNumericConstant := True;
    end;
end;

Function IsHexConstant (Var HexCharPointer: PChar): Boolean;
Var Digit: Byte;
    Chr: Char;
begin
  IsHexConstant := False;
  IntegerNumericConstant := 0;
  Chr := UpCase (HexCharPointer [0]);
  While Chr in HexChars do
    begin
      Digit := Byte (Chr) - Ord ('0');
      If Digit > 9 then Digit := Digit - (Ord ('A') - Ord ('0') - 10);
      If IntegerNumericConstantRec.WordH and $F000 <> 0 then
        begin
          IsHexConstant := False;
          Exit;
        end;
      IntegerNumericConstant := IntegerNumericConstant shl 4;
      IntegerNumericConstantRec.WordL := IntegerNumericConstantRec.WordL or Digit;
      Inc (HexCharPointer);
      Chr := UpCase (HexCharPointer [0]);
      IsHexConstant := True;
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
Function IsIntegerNumericConstant (Var CharacterPointer: PChar): Boolean;
begin
  If CharacterPointer [0] = '$' then
    begin
      Inc (CharacterPointer);
      IsIntegerNumericConstant := IsHexConstant (CharacterPointer);
      Exit;
    end;
  IntegerNumericConstant := 0;
  IsIntegerNumericConstant := False;
  While CharacterPointer [0] in DecChars do
    begin
      If IntegerNumericConstantRec.WordH and $F000 <> 0 then
        begin
          IsIntegerNumericConstant := False;
          Exit;
        end;
      IntegerNumericConstant := IntegerNumericConstant * 10 + Ord (CharacterPointer [0]) - Ord ('0');;
      If IntegerNumericConstantRec.WordH and $8000 <> 0 then    { BUG in TPC ?                 }
        begin                                                   { -2147483648 causes error !!! }
          IsIntegerNumericConstant := False;
          Exit;
        end;
      Inc (CharacterPointer);
      IsIntegerNumericConstant := True;
    end;
end;

Function IsHexConstant (Var HexCharPointer: PChar): Boolean;
Var Digit: Byte;
    Chr: Char;
begin
  IsHexConstant := False;
  IntegerNumericConstant := 0;
  Chr := UpCase (HexCharPointer [0]);
  While Chr in HexChars do
    begin
      Digit := Byte (Chr) - Ord ('0');
      If Digit > 9 then Digit := Digit - (Ord ('A') - Ord ('0') - 10);
      If IntegerNumericConstantRec.WordH and $F000 <> 0 then
        begin
          IsHexConstant := False;
          Exit;
        end;
      IntegerNumericConstant := IntegerNumericConstant shl 4;
      IntegerNumericConstantRec.WordL := IntegerNumericConstantRec.WordL or Digit;
      Inc (HexCharPointer);
      Chr := UpCase (HexCharPointer [0]);
      IsHexConstant := True;
    end;
end;
```

### Code Block 2

```pascal
IntegerNumericConstant
```

### Code Block 3

```pascal
Function IsIntegerNumericConstant (Var CharacterPointer: PChar): Boolean;
begin
  If CharacterPointer [0] = '$' then
    begin
      Inc (CharacterPointer);
      IsIntegerNumericConstant := IsHexConstant (CharacterPointer);
      Exit;
    end;
  IntegerNumericConstant := 0;
  IsIntegerNumericConstant := False;
  While CharacterPointer [0] in DecChars do
    begin
      If IntegerNumericConstantRec.WordH and $F000 <> 0 then
        begin
          IsIntegerNumericConstant := False;
          Exit;
        end;
      IntegerNumericConstant := IntegerNumericConstant * 10 + Ord (CharacterPointer [0]) - Ord ('0');;
      If IntegerNumericConstantRec.WordH and $8000 <> 0 then    { BUG in TPC ?                 }
        begin                                                   { -2147483648 causes error !!! }
          IsIntegerNumericConstant := False;
          Exit;
        end;
      Inc (CharacterPointer);
      IsIntegerNumericConstant := True;
    end;
end;

Function IsHexConstant (Var HexCharPointer: PChar): Boolean;
Var Digit: Byte;
    Chr: Char;
begin
  IsHexConstant := False;
  IntegerNumericConstant := 0;
  Chr := UpCase (HexCharPointer [0]);
  While Chr in HexChars do
    begin
      Digit := Byte (Chr) - Ord ('0');
      If Digit > 9 then Digit := Digit - (Ord ('A') - Ord ('0') - 10);
      If IntegerNumericConstantRec.WordH and $F000 <> 0 then
        begin
          IsHexConstant := False;
          Exit;
        end;
      IntegerNumericConstant := IntegerNumericConstant shl 4;
      IntegerNumericConstantRec.WordL := IntegerNumericConstantRec.WordL or Digit;
      Inc (HexCharPointer);
      Chr := UpCase (HexCharPointer [0]);
      IsHexConstant := True;
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
- https://turbopascal.org/processing-integer-constants
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

