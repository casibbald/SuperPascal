# Checking Public Identifiers - Turbo Pascal Compiler

**URL:** https://turbopascal.org/checking-public-identifiers

---

## Table of Contents

- Checking Public Identifiers

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Checking Public Identifiers
Turbo Pascal calculates a checksum of public identifiers for every unit to quickly check if something has been changed.
Procedure CalculateIdentifiersChecksum;
Var UnitCounter: Word;
    UnitHeader: PUnitHeader;
    UnitIdentifier: PIdentifier;
    UnitIdentifierRec: PtrRec absolute UnitIdentifier;
    UnitIdentifierData: PUnitIdentifierData;
    CheckSum, N: Word;
    IdWord: PWord;
begin
  UnitCounter := 0;                                                 { Push And Clear Unit Segments;}
  UnitHeader := Ptr (MainSymbolTable.Segment, 0);
  UnitIdentifier := Ptr (MainSymbolTable.Segment, UnitHeader^.UnitNameIdentifierOffset);
  Repeat
    UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
    Asm
      LES   DI, UnitIdentifierData
      PUSH  WORD PTR ES:[DI]            { UnitIdentifierData^.UnitSegment) }
      PUSH  DI                          { Ofs (UnitIdentifierData^) }
    end;
    UnitIdentifierData^.UnitSegment := 0;
    Inc (UnitCounter);
    UnitIdentifierRec.Ofs := UnitIdentifierData^.NextUnitIdentifier;
  until UnitIdentifierRec.Ofs = 0;

  CheckSum := 0;
  IdWord := Ptr (MainSymbolTable.Segment, UnitHeader^.PublicIdentifiersListOffset);
  For N := 1 to (SymbolTable [stMain].UsedSize - UnitHeader^.PublicIdentifiersListOffset) div 2 do
    begin
      Asm
        MOV   AX, CheckSum
        ROL   AX, 1
        MOV   CheckSum, AX
      end;
      Inc (CheckSum, IdWord^);
      Inc (IdWord);
    end;
  If CheckSum = 0 then CheckSum := $FFFF;
  UnitIdentifier := Ptr (MainSymbolTable.Segment, UnitHeader^.UnitNameIdentifierOffset);
  UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
  UnitIdentifierData^.PublicIdentifiersChecksum := CheckSum;

  Asm
    LES   DI, UnitHeader
    MOV   CX, UnitCounter
@1:
    POP   DI
    POP   WORD PTR ES:[DI]
    LOOP  @1
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
Procedure CalculateIdentifiersChecksum;
Var UnitCounter: Word;
    UnitHeader: PUnitHeader;
    UnitIdentifier: PIdentifier;
    UnitIdentifierRec: PtrRec absolute UnitIdentifier;
    UnitIdentifierData: PUnitIdentifierData;
    CheckSum, N: Word;
    IdWord: PWord;
begin
  UnitCounter := 0;                                                 { Push And Clear Unit Segments;}
  UnitHeader := Ptr (MainSymbolTable.Segment, 0);
  UnitIdentifier := Ptr (MainSymbolTable.Segment, UnitHeader^.UnitNameIdentifierOffset);
  Repeat
    UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
    Asm
      LES   DI, UnitIdentifierData
      PUSH  WORD PTR ES:[DI]            { UnitIdentifierData^.UnitSegment) }
      PUSH  DI                          { Ofs (UnitIdentifierData^) }
    end;
    UnitIdentifierData^.UnitSegment := 0;
    Inc (UnitCounter);
    UnitIdentifierRec.Ofs := UnitIdentifierData^.NextUnitIdentifier;
  until UnitIdentifierRec.Ofs = 0;

  CheckSum := 0;
  IdWord := Ptr (MainSymbolTable.Segment, UnitHeader^.PublicIdentifiersListOffset);
  For N := 1 to (SymbolTable [stMain].UsedSize - UnitHeader^.PublicIdentifiersListOffset) div 2 do
    begin
      Asm
        MOV   AX, CheckSum
        ROL   AX, 1
        MOV   CheckSum, AX
      end;
      Inc (CheckSum, IdWord^);
      Inc (IdWord);
    end;
  If CheckSum = 0 then CheckSum := $FFFF;
  UnitIdentifier := Ptr (MainSymbolTable.Segment, UnitHeader^.UnitNameIdentifierOffset);
  UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
  UnitIdentifierData^.PublicIdentifiersChecksum := CheckSum;

  Asm
    LES   DI, UnitHeader
    MOV   CX, UnitCounter
@1:
    POP   DI
    POP   WORD PTR ES:[DI]
    LOOP  @1
  end;
end;
```

### Code Block 2

```pascal
Procedure CalculateIdentifiersChecksum;
Var UnitCounter: Word;
    UnitHeader: PUnitHeader;
    UnitIdentifier: PIdentifier;
    UnitIdentifierRec: PtrRec absolute UnitIdentifier;
    UnitIdentifierData: PUnitIdentifierData;
    CheckSum, N: Word;
    IdWord: PWord;
begin
  UnitCounter := 0;                                                 { Push And Clear Unit Segments;}
  UnitHeader := Ptr (MainSymbolTable.Segment, 0);
  UnitIdentifier := Ptr (MainSymbolTable.Segment, UnitHeader^.UnitNameIdentifierOffset);
  Repeat
    UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
    Asm
      LES   DI, UnitIdentifierData
      PUSH  WORD PTR ES:[DI]            { UnitIdentifierData^.UnitSegment) }
      PUSH  DI                          { Ofs (UnitIdentifierData^) }
    end;
    UnitIdentifierData^.UnitSegment := 0;
    Inc (UnitCounter);
    UnitIdentifierRec.Ofs := UnitIdentifierData^.NextUnitIdentifier;
  until UnitIdentifierRec.Ofs = 0;

  CheckSum := 0;
  IdWord := Ptr (MainSymbolTable.Segment, UnitHeader^.PublicIdentifiersListOffset);
  For N := 1 to (SymbolTable [stMain].UsedSize - UnitHeader^.PublicIdentifiersListOffset) div 2 do
    begin
      Asm
        MOV   AX, CheckSum
        ROL   AX, 1
        MOV   CheckSum, AX
      end;
      Inc (CheckSum, IdWord^);
      Inc (IdWord);
    end;
  If CheckSum = 0 then CheckSum := $FFFF;
  UnitIdentifier := Ptr (MainSymbolTable.Segment, UnitHeader^.UnitNameIdentifierOffset);
  UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
  UnitIdentifierData^.PublicIdentifiersChecksum := CheckSum;

  Asm
    LES   DI, UnitHeader
    MOV   CX, UnitCounter
@1:
    POP   DI
    POP   WORD PTR ES:[DI]
    LOOP  @1
  end;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/checking-public-identifiers
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
- https://turbopascal.org/type-definitions

