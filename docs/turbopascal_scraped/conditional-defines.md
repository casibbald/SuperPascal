# Conditional Defines - Turbo Pascal Compiler

**URL:** https://turbopascal.org/conditional-defines

---

## Table of Contents

- Conditional Defines

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Conditional Defines
Turbo Pascal contains few procedures and functions that create and check conditional defines.  The first function creates initial conditional defines if certain conditions are met:
MSDOS
,
VER70
,
CPU86
and
CPU87
.
Const DefaultConditionalDefines: Array [1..NumOfDefaultConditionalDefines] of TConditionalDefine = (
        (RequiredEnvironmentFlags: [enfCPU86];               Str: 'MSDOS'),
        (RequiredEnvironmentFlags: [enf04, enf02, enfCPU86]; Str: 'VER70'),
        (RequiredEnvironmentFlags: [enf04, enf02, enfCPU86]; Str: 'CPU86'),
        (RequiredEnvironmentFlags: [enfCPU87];               Str: 'CPU87'));

Procedure CreateConditionalDefines;
Var TempPtr: PChar;
    N: Byte;
begin
  TempPtr := @ConditionalDefines;
  For N := 1 to NumOfDefaultConditionalDefines do
    If InitialEnvironmentFlags * DefaultConditionalDefines [N].RequiredEnvironmentFlags <> [] then
      begin
        Move (DefaultConditionalDefines [N].Str, TempPtr^, SizeOf (DefaultConditionalDefines [N].Str));
        TempPtr := Ptr (Seg (TempPtr^), Ofs (TempPtr^) + SizeOf (DefaultConditionalDefines [N].Str));
      end;
  ConditionalDefinesEnd := PString (TempPtr);
  TempPtr := CommandLineConditionalDefines;
  Repeat
    SkipSeparators (TempPtr);
    If TempPtr^ = #0 then Exit;
    CopyToIdentifier (TempPtr);
    If Identifier [0] = #0 then Error (ErrorInInitialConditionalDdefines);
    AddConditionalDefine;
  until False;
end;
This procedure copies conditional define to the
Identifier
variable.
Procedure CopyToIdentifier (Var CharacterPointer: PChar);
Var Len: Byte;
begin
  Len := 0;
  Repeat
    Case UpCase (CharacterPointer^) of
      '0'..'9': If Len > 0 then
                  begin
                    Inc (Len);
                    Identifier [Len] := CharacterPointer^;
                    Inc (CharacterPointer);
                  end else Break;
      '_', 'A'..'Z': begin
                       Inc (Len);
                       Identifier [Len] := CharacterPointer^;
                       Inc (CharacterPointer);
                     end;
      else Break;
    end;
  until Len = 63;
  Identifier [0] := Char (Len);
end;
This procedure adds conditional define stored in the
Identifier
variable to the list of conditional defines.
Procedure AddConditionalDefine;
begin
  If IdentifierInConditionalDefines then Exit;
  If Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines) +
     Length (Identifier) + 1 > SizeOf (ConditionalDefines) then
       Error (TooManyConditionalSymbols);
  ConditionalDefinesEnd^ := Identifier;
  ConditionalDefinesEnd := Ptr (Seg (ConditionalDefinesEnd^),
                             Ofs (ConditionalDefinesEnd^) + Length (Identifier) + 1);
end;
This function checks if
Identifier
is in the list of conditional defines.
Function IdentifierInConditionalDefines: Boolean;
Var TempPtr: PString;
begin
  TempPtr := @ConditionalDefines;
  IdentifierInConditionalDefines := False;
  While TempPtr <> ConditionalDefinesEnd do
    begin
      If TempPtr^ = Identifier then
        begin
          IdentifierInConditionalDefines := True;
          DummyPtr := PChar (TempPtr);
          Exit;
        end;
      TempPtr := Ptr (Seg (TempPtr^), Ofs (TempPtr^) + Byte (TempPtr^ [0]) + 1);
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
Const DefaultConditionalDefines: Array [1..NumOfDefaultConditionalDefines] of TConditionalDefine = (
        (RequiredEnvironmentFlags: [enfCPU86];               Str: 'MSDOS'),
        (RequiredEnvironmentFlags: [enf04, enf02, enfCPU86]; Str: 'VER70'),
        (RequiredEnvironmentFlags: [enf04, enf02, enfCPU86]; Str: 'CPU86'),
        (RequiredEnvironmentFlags: [enfCPU87];               Str: 'CPU87'));

Procedure CreateConditionalDefines;
Var TempPtr: PChar;
    N: Byte;
begin
  TempPtr := @ConditionalDefines;
  For N := 1 to NumOfDefaultConditionalDefines do
    If InitialEnvironmentFlags * DefaultConditionalDefines [N].RequiredEnvironmentFlags <> [] then
      begin
        Move (DefaultConditionalDefines [N].Str, TempPtr^, SizeOf (DefaultConditionalDefines [N].Str));
        TempPtr := Ptr (Seg (TempPtr^), Ofs (TempPtr^) + SizeOf (DefaultConditionalDefines [N].Str));
      end;
  ConditionalDefinesEnd := PString (TempPtr);
  TempPtr := CommandLineConditionalDefines;
  Repeat
    SkipSeparators (TempPtr);
    If TempPtr^ = #0 then Exit;
    CopyToIdentifier (TempPtr);
    If Identifier [0] = #0 then Error (ErrorInInitialConditionalDdefines);
    AddConditionalDefine;
  until False;
end;
```

### Code Block 2

```pascal
Procedure CopyToIdentifier (Var CharacterPointer: PChar);
Var Len: Byte;
begin
  Len := 0;
  Repeat
    Case UpCase (CharacterPointer^) of
      '0'..'9': If Len > 0 then
                  begin
                    Inc (Len);
                    Identifier [Len] := CharacterPointer^;
                    Inc (CharacterPointer);
                  end else Break;
      '_', 'A'..'Z': begin
                       Inc (Len);
                       Identifier [Len] := CharacterPointer^;
                       Inc (CharacterPointer);
                     end;
      else Break;
    end;
  until Len = 63;
  Identifier [0] := Char (Len);
end;
```

### Code Block 3

```pascal
Procedure AddConditionalDefine;
begin
  If IdentifierInConditionalDefines then Exit;
  If Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines) +
     Length (Identifier) + 1 > SizeOf (ConditionalDefines) then
       Error (TooManyConditionalSymbols);
  ConditionalDefinesEnd^ := Identifier;
  ConditionalDefinesEnd := Ptr (Seg (ConditionalDefinesEnd^),
                             Ofs (ConditionalDefinesEnd^) + Length (Identifier) + 1);
end;
```

### Code Block 4

```pascal
Function IdentifierInConditionalDefines: Boolean;
Var TempPtr: PString;
begin
  TempPtr := @ConditionalDefines;
  IdentifierInConditionalDefines := False;
  While TempPtr <> ConditionalDefinesEnd do
    begin
      If TempPtr^ = Identifier then
        begin
          IdentifierInConditionalDefines := True;
          DummyPtr := PChar (TempPtr);
          Exit;
        end;
      TempPtr := Ptr (Seg (TempPtr^), Ofs (TempPtr^) + Byte (TempPtr^ [0]) + 1);
    end;
end;
```

### Code Block 5

```pascal
Const DefaultConditionalDefines: Array [1..NumOfDefaultConditionalDefines] of TConditionalDefine = (
        (RequiredEnvironmentFlags: [enfCPU86];               Str: 'MSDOS'),
        (RequiredEnvironmentFlags: [enf04, enf02, enfCPU86]; Str: 'VER70'),
        (RequiredEnvironmentFlags: [enf04, enf02, enfCPU86]; Str: 'CPU86'),
        (RequiredEnvironmentFlags: [enfCPU87];               Str: 'CPU87'));

Procedure CreateConditionalDefines;
Var TempPtr: PChar;
    N: Byte;
begin
  TempPtr := @ConditionalDefines;
  For N := 1 to NumOfDefaultConditionalDefines do
    If InitialEnvironmentFlags * DefaultConditionalDefines [N].RequiredEnvironmentFlags <> [] then
      begin
        Move (DefaultConditionalDefines [N].Str, TempPtr^, SizeOf (DefaultConditionalDefines [N].Str));
        TempPtr := Ptr (Seg (TempPtr^), Ofs (TempPtr^) + SizeOf (DefaultConditionalDefines [N].Str));
      end;
  ConditionalDefinesEnd := PString (TempPtr);
  TempPtr := CommandLineConditionalDefines;
  Repeat
    SkipSeparators (TempPtr);
    If TempPtr^ = #0 then Exit;
    CopyToIdentifier (TempPtr);
    If Identifier [0] = #0 then Error (ErrorInInitialConditionalDdefines);
    AddConditionalDefine;
  until False;
end;
```

### Code Block 6

```pascal
Procedure CopyToIdentifier (Var CharacterPointer: PChar);
Var Len: Byte;
begin
  Len := 0;
  Repeat
    Case UpCase (CharacterPointer^) of
      '0'..'9': If Len > 0 then
                  begin
                    Inc (Len);
                    Identifier [Len] := CharacterPointer^;
                    Inc (CharacterPointer);
                  end else Break;
      '_', 'A'..'Z': begin
                       Inc (Len);
                       Identifier [Len] := CharacterPointer^;
                       Inc (CharacterPointer);
                     end;
      else Break;
    end;
  until Len = 63;
  Identifier [0] := Char (Len);
end;
```

### Code Block 7

```pascal
Procedure AddConditionalDefine;
begin
  If IdentifierInConditionalDefines then Exit;
  If Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines) +
     Length (Identifier) + 1 > SizeOf (ConditionalDefines) then
       Error (TooManyConditionalSymbols);
  ConditionalDefinesEnd^ := Identifier;
  ConditionalDefinesEnd := Ptr (Seg (ConditionalDefinesEnd^),
                             Ofs (ConditionalDefinesEnd^) + Length (Identifier) + 1);
end;
```

### Code Block 8

```pascal
Function IdentifierInConditionalDefines: Boolean;
Var TempPtr: PString;
begin
  TempPtr := @ConditionalDefines;
  IdentifierInConditionalDefines := False;
  While TempPtr <> ConditionalDefinesEnd do
    begin
      If TempPtr^ = Identifier then
        begin
          IdentifierInConditionalDefines := True;
          DummyPtr := PChar (TempPtr);
          Exit;
        end;
      TempPtr := Ptr (Seg (TempPtr^), Ofs (TempPtr^) + Byte (TempPtr^ [0]) + 1);
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
- https://turbopascal.org/conditional-defines
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
- https://turbopascal.org/type-definitions

