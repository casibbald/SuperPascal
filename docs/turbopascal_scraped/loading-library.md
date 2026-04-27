# Loading Library - Turbo Pascal Compiler

**URL:** https://turbopascal.org/loading-library

---

## Table of Contents

- Loading Library

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Loading Library
This procedure loads Turbo Pascal library (a collection of units). It adjust each unit so that it starts on a segment boundary. Actually, each compiled unit is a group of symbol tables.
Procedure LoadLibrary;
Var DummyVar: Pointer;

  Procedure LoadLibraryProc;
  Var LibraryFile: File;
      LibPtr, UnitPtr: PUnitHeader;

    Procedure CalculateUnitSegments;
    Var SymbolTable: TSymbolTable;

      Procedure AdjustUnitSeg (Size: Word);
      begin
        UnitPtr := Ptr (Seg (UnitPtr^) + (Size + $000F) shr 4, 0);
      end;

    begin
      AdjustUnitSeg (LibPtr^.JoinedBlockSize);
      For SymbolTable := st11 to stTypedConstantsReferences do
        begin
          LibPtr^.SymbolTableSegment [SymbolTable] := Seg (UnitPtr^);
          AdjustUnitSeg (LibPtr^.BlockSize [SymbolTable]);
        end;
    end;

  begin
    LastLibraryUnitSegment := 0;
    If CurrentFileName [0] = #0 then Exit;
    OpenFile (LibraryFile, CurrentFileName);
    LibPtr := ReadFile (LibraryFile);
    Close (LibraryFile);
    Repeat
      If LibPtr^.Signature <> UnitSignature then FileError (CurrentFileName, UnitFileFormatError);
      UnitPtr := LibPtr;
      CalculateUnitSegments;
      Include (LibPtr^.Flags, ufUnitFlag_8000);
      LibPtr^.PreviousLibraryUnitSegment := LastLibraryUnitSegment;
      LastLibraryUnitSegment := Seg (LibPtr^);
      LibPtr := UnitPtr;
    until LibPtr = HeapPtr;
  end;

begin
  SetErrorAddress (@LoadLibraryError);
  LoadLibraryProc;
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
Procedure LoadLibrary;
Var DummyVar: Pointer;

  Procedure LoadLibraryProc;
  Var LibraryFile: File;
      LibPtr, UnitPtr: PUnitHeader;

    Procedure CalculateUnitSegments;
    Var SymbolTable: TSymbolTable;

      Procedure AdjustUnitSeg (Size: Word);
      begin
        UnitPtr := Ptr (Seg (UnitPtr^) + (Size + $000F) shr 4, 0);
      end;

    begin
      AdjustUnitSeg (LibPtr^.JoinedBlockSize);
      For SymbolTable := st11 to stTypedConstantsReferences do
        begin
          LibPtr^.SymbolTableSegment [SymbolTable] := Seg (UnitPtr^);
          AdjustUnitSeg (LibPtr^.BlockSize [SymbolTable]);
        end;
    end;

  begin
    LastLibraryUnitSegment := 0;
    If CurrentFileName [0] = #0 then Exit;
    OpenFile (LibraryFile, CurrentFileName);
    LibPtr := ReadFile (LibraryFile);
    Close (LibraryFile);
    Repeat
      If LibPtr^.Signature <> UnitSignature then FileError (CurrentFileName, UnitFileFormatError);
      UnitPtr := LibPtr;
      CalculateUnitSegments;
      Include (LibPtr^.Flags, ufUnitFlag_8000);
      LibPtr^.PreviousLibraryUnitSegment := LastLibraryUnitSegment;
      LastLibraryUnitSegment := Seg (LibPtr^);
      LibPtr := UnitPtr;
    until LibPtr = HeapPtr;
  end;

begin
  SetErrorAddress (@LoadLibraryError);
  LoadLibraryProc;
end;
```

### Code Block 2

```pascal
Procedure LoadLibrary;
Var DummyVar: Pointer;

  Procedure LoadLibraryProc;
  Var LibraryFile: File;
      LibPtr, UnitPtr: PUnitHeader;

    Procedure CalculateUnitSegments;
    Var SymbolTable: TSymbolTable;

      Procedure AdjustUnitSeg (Size: Word);
      begin
        UnitPtr := Ptr (Seg (UnitPtr^) + (Size + $000F) shr 4, 0);
      end;

    begin
      AdjustUnitSeg (LibPtr^.JoinedBlockSize);
      For SymbolTable := st11 to stTypedConstantsReferences do
        begin
          LibPtr^.SymbolTableSegment [SymbolTable] := Seg (UnitPtr^);
          AdjustUnitSeg (LibPtr^.BlockSize [SymbolTable]);
        end;
    end;

  begin
    LastLibraryUnitSegment := 0;
    If CurrentFileName [0] = #0 then Exit;
    OpenFile (LibraryFile, CurrentFileName);
    LibPtr := ReadFile (LibraryFile);
    Close (LibraryFile);
    Repeat
      If LibPtr^.Signature <> UnitSignature then FileError (CurrentFileName, UnitFileFormatError);
      UnitPtr := LibPtr;
      CalculateUnitSegments;
      Include (LibPtr^.Flags, ufUnitFlag_8000);
      LibPtr^.PreviousLibraryUnitSegment := LastLibraryUnitSegment;
      LastLibraryUnitSegment := Seg (LibPtr^);
      LibPtr := UnitPtr;
    until LibPtr = HeapPtr;
  end;

begin
  SetErrorAddress (@LoadLibraryError);
  LoadLibraryProc;
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
- https://turbopascal.org/loading-library
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

