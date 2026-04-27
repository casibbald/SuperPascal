# Compacting Symbol Tables - Turbo Pascal Compiler

**URL:** https://turbopascal.org/compacting-symbol-tables

---

## Table of Contents

- Compacting Symbol Tables

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Compacting Symbol Tables
After each module is
compiled
the symbol tables are compacted to remove unused space.
Procedure JoinSymbolTablesAndCreateUnit;
Var ReferencedUnitRecord: PReferencedModulesBlockRecord;
    SymbolsSize: LongRec;
    Block: TSymbolTable;
    UnitHeader: PUnitHeader;
    CurrentPointer: PChar;
  Function AdjustCurrentPointerToNextParagraph: Word;
  begin
    AdjustCurrentPointerToNextParagraph := Seg (CurrentPointer^);
    FillChar (CurrentPointer^, (16 - Ofs (CurrentPointer^)) and $000F, 0);
    Inc (CurrentPointer, (16 - Ofs (CurrentPointer^)) and $000F);
    CurrentPointer := Ptr (Seg (CurrentPointer^) + Ofs (CurrentPointer^) shr 4, 0);
  end;

begin
  ReferencedUnitRecord := Ptr (SymbolTable [stReferencedModules].Segment, 0);
  While ReferencedUnitRecord <> SymbolTable [stReferencedModules].Ptr do
    begin
      ReferencedUnitRecord^.ModuleSegment := 0;
      Inc (PChar (ReferencedUnitRecord), ReferencedUnitRecord^.UnitName.Len + 5);
    end;
  SymbolsSize.Long := 0;
  For Block := stMain to stSourceLineCodeOffsets do
    Inc (SymbolsSize.Long, SymbolTable [Block].UsedSize);
  If SymbolsSize.WordH <> 0 then Error (TooManySymbols);
  If SymbolsSize.WordL > $FFF0 then Error (TooManySymbols);
  UnitHeader := Ptr (SymbolTable [stMain].Segment, 0);
  CurrentPointer := SymbolTable [stMain].Ptr;
  For Block := stProcedures to stSourceLineCodeOffsets do
    begin
      UnitHeader^.BlockOffset [Block] := Ofs (CurrentPointer^);
      MoveBackward (Ptr (SymbolTable [Block].Segment, 0)^, CurrentPointer^, SymbolTable [Block].UsedSize);
      Inc (CurrentPointer, SymbolTable [Block].UsedSize);
    end;
  SymbolTable [stMain].UsedSize := Ofs (CurrentPointer^);
  UnitHeader^.JoinedBlockSize := Ofs (CurrentPointer^);
  For Block := st11 to stTypedConstantsReferences do
    UnitHeader^.BlockSize [Block] := SymbolTable [Block].UsedSize;
  UnitHeader^.SizeOfVariablesInDataSegment := SizeOfVariablesInDataSegment;
  AdjustCurrentPointerToNextParagraph;
  For Block := st11 to stTypedConstantsReferences do
    begin
      MoveBackward (Ptr (SymbolTable [Block].Segment, 0)^, CurrentPointer^, SymbolTable [Block].UsedSize);
      Inc (CurrentPointer, SymbolTable [Block].UsedSize);
      SymbolTable [Block].Segment := AdjustCurrentPointerToNextParagraph;
    end;
  HeapPtr := CurrentPointer;
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
Procedure JoinSymbolTablesAndCreateUnit;
Var ReferencedUnitRecord: PReferencedModulesBlockRecord;
    SymbolsSize: LongRec;
    Block: TSymbolTable;
    UnitHeader: PUnitHeader;
    CurrentPointer: PChar;
  Function AdjustCurrentPointerToNextParagraph: Word;
  begin
    AdjustCurrentPointerToNextParagraph := Seg (CurrentPointer^);
    FillChar (CurrentPointer^, (16 - Ofs (CurrentPointer^)) and $000F, 0);
    Inc (CurrentPointer, (16 - Ofs (CurrentPointer^)) and $000F);
    CurrentPointer := Ptr (Seg (CurrentPointer^) + Ofs (CurrentPointer^) shr 4, 0);
  end;

begin
  ReferencedUnitRecord := Ptr (SymbolTable [stReferencedModules].Segment, 0);
  While ReferencedUnitRecord <> SymbolTable [stReferencedModules].Ptr do
    begin
      ReferencedUnitRecord^.ModuleSegment := 0;
      Inc (PChar (ReferencedUnitRecord), ReferencedUnitRecord^.UnitName.Len + 5);
    end;
  SymbolsSize.Long := 0;
  For Block := stMain to stSourceLineCodeOffsets do
    Inc (SymbolsSize.Long, SymbolTable [Block].UsedSize);
  If SymbolsSize.WordH <> 0 then Error (TooManySymbols);
  If SymbolsSize.WordL > $FFF0 then Error (TooManySymbols);
  UnitHeader := Ptr (SymbolTable [stMain].Segment, 0);
  CurrentPointer := SymbolTable [stMain].Ptr;
  For Block := stProcedures to stSourceLineCodeOffsets do
    begin
      UnitHeader^.BlockOffset [Block] := Ofs (CurrentPointer^);
      MoveBackward (Ptr (SymbolTable [Block].Segment, 0)^, CurrentPointer^, SymbolTable [Block].UsedSize);
      Inc (CurrentPointer, SymbolTable [Block].UsedSize);
    end;
  SymbolTable [stMain].UsedSize := Ofs (CurrentPointer^);
  UnitHeader^.JoinedBlockSize := Ofs (CurrentPointer^);
  For Block := st11 to stTypedConstantsReferences do
    UnitHeader^.BlockSize [Block] := SymbolTable [Block].UsedSize;
  UnitHeader^.SizeOfVariablesInDataSegment := SizeOfVariablesInDataSegment;
  AdjustCurrentPointerToNextParagraph;
  For Block := st11 to stTypedConstantsReferences do
    begin
      MoveBackward (Ptr (SymbolTable [Block].Segment, 0)^, CurrentPointer^, SymbolTable [Block].UsedSize);
      Inc (CurrentPointer, SymbolTable [Block].UsedSize);
      SymbolTable [Block].Segment := AdjustCurrentPointerToNextParagraph;
    end;
  HeapPtr := CurrentPointer;
end;
```

### Code Block 2

```pascal
Procedure JoinSymbolTablesAndCreateUnit;
Var ReferencedUnitRecord: PReferencedModulesBlockRecord;
    SymbolsSize: LongRec;
    Block: TSymbolTable;
    UnitHeader: PUnitHeader;
    CurrentPointer: PChar;
  Function AdjustCurrentPointerToNextParagraph: Word;
  begin
    AdjustCurrentPointerToNextParagraph := Seg (CurrentPointer^);
    FillChar (CurrentPointer^, (16 - Ofs (CurrentPointer^)) and $000F, 0);
    Inc (CurrentPointer, (16 - Ofs (CurrentPointer^)) and $000F);
    CurrentPointer := Ptr (Seg (CurrentPointer^) + Ofs (CurrentPointer^) shr 4, 0);
  end;

begin
  ReferencedUnitRecord := Ptr (SymbolTable [stReferencedModules].Segment, 0);
  While ReferencedUnitRecord <> SymbolTable [stReferencedModules].Ptr do
    begin
      ReferencedUnitRecord^.ModuleSegment := 0;
      Inc (PChar (ReferencedUnitRecord), ReferencedUnitRecord^.UnitName.Len + 5);
    end;
  SymbolsSize.Long := 0;
  For Block := stMain to stSourceLineCodeOffsets do
    Inc (SymbolsSize.Long, SymbolTable [Block].UsedSize);
  If SymbolsSize.WordH <> 0 then Error (TooManySymbols);
  If SymbolsSize.WordL > $FFF0 then Error (TooManySymbols);
  UnitHeader := Ptr (SymbolTable [stMain].Segment, 0);
  CurrentPointer := SymbolTable [stMain].Ptr;
  For Block := stProcedures to stSourceLineCodeOffsets do
    begin
      UnitHeader^.BlockOffset [Block] := Ofs (CurrentPointer^);
      MoveBackward (Ptr (SymbolTable [Block].Segment, 0)^, CurrentPointer^, SymbolTable [Block].UsedSize);
      Inc (CurrentPointer, SymbolTable [Block].UsedSize);
    end;
  SymbolTable [stMain].UsedSize := Ofs (CurrentPointer^);
  UnitHeader^.JoinedBlockSize := Ofs (CurrentPointer^);
  For Block := st11 to stTypedConstantsReferences do
    UnitHeader^.BlockSize [Block] := SymbolTable [Block].UsedSize;
  UnitHeader^.SizeOfVariablesInDataSegment := SizeOfVariablesInDataSegment;
  AdjustCurrentPointerToNextParagraph;
  For Block := st11 to stTypedConstantsReferences do
    begin
      MoveBackward (Ptr (SymbolTable [Block].Segment, 0)^, CurrentPointer^, SymbolTable [Block].UsedSize);
      Inc (CurrentPointer, SymbolTable [Block].UsedSize);
      SymbolTable [Block].Segment := AdjustCurrentPointerToNextParagraph;
    end;
  HeapPtr := CurrentPointer;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compacting-symbol-tables
- https://turbopascal.org/compilation-process
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

