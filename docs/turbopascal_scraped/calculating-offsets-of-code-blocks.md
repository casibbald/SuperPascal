# Calculating Offsets of Code Blocks - Turbo Pascal Compiler

**URL:** https://turbopascal.org/calculating-offsets-of-code-blocks

---

## Table of Contents

- Calculating Offsets of Code Blocks

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Calculating Offsets of Code Blocks
This procedure calculates offset of each program code block and total code size.
Procedure CalculateCodeBlockOffsetsAndTotalCodeSize;
Var ProgramCodeBlock: PProgramCodeBlockRecord;
    ProcedureRecordPtr: PProceduresBlockRecord absolute ProgramCodeBlock;
    BlockPtr: PtrRec absolute ProgramCodeBlock;
    UnitCodeSize, NumberOfSegmentReferencesInUnitProgramCode: Word;
    ExeHeaderSize: LongRec;
begin
  UnitPtrRec.Seg := LastLoadedUsedUnit;
  Repeat
    UnitPtr^.CodeSegment := CurrentCodeSegment;
    ProgramCodeBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [stCodeBlocks]);
    UnitCodeSize := 0;
    While BlockPtr.Ofs <> UnitPtr^.BlockOffset [Succ (stCodeBlocks)] do
      begin
        If ProgramCodeBlock^.Offset <> BlockUnused then
          begin
            ProgramCodeBlock^.Offset := UnitCodeSize;
            Inc (UnitCodeSize, ProgramCodeBlock^.CodeSize);
          end;
        Inc (ProgramCodeBlock);
      end;
    NumberOfSegmentReferencesInUnitProgramCode := UnitPtr^.NumberOfSegmentReferencesInProgramCodeBlocks;
    If UnitPtr^.OverlayedUnitCodeSize <> 0 then
      begin
        UnitPtr^.OverlayedUnitCodeSize := UnitCodeSize;
        UnitCodeSize := (UnitCodeSize + $000F) shr 4;
        Inc (UnitCodeSize, (NumberOfSegmentReferencesInUnitProgramCode + $0007) shr 3);
        If UnitCodeSize > OverlayHeapSize then OverlayHeapSize := UnitCodeSize;
        UnitCodeSize := $20;
        BlockPtr.Ofs := UnitPtr^.BlockOffset [stProcedures];
        While BlockPtr.Ofs <> UnitPtr^.BlockOffset [Succ (stProcedures)] do
          begin
            If ProcedureRecordPtr^.OverlayedProcedureOffset <> 0 then
              begin
                ProcedureRecordPtr^.OverlayedProcedureOffset := UnitCodeSize;
                Inc (UnitCodeSize, 5);
              end;
            Inc (ProcedureRecordPtr);
          end;
        NumberOfSegmentReferencesInUnitProgramCode := 0;
      end;
    UnitPtr^.CodeSize := UnitCodeSize;
    Inc (CurrentCodeSegment, (UnitCodeSize + $000F) shr 4);
    ExeHeaderSize.Long := (NumberOfSegmentReferencesInUnitProgramCode +
                           UnitPtr^.NumberOfSegmentReferencesInTypedConstantsBlocks) * 4 + CurrentRelocationItemOffset;
    If (ExeHeaderSize.WordH <> 0) or (ExeHeaderSize.WordL > $FFF0) then Error (TooManyRelocationItems);
    CurrentRelocationItemOffset := ExeHeaderSize.WordL;
    UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
  until UnitPtrRec.Seg = 0;
  CodeBytes := 16 * LongInt (CurrentCodeSegment);
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
Procedure CalculateCodeBlockOffsetsAndTotalCodeSize;
Var ProgramCodeBlock: PProgramCodeBlockRecord;
    ProcedureRecordPtr: PProceduresBlockRecord absolute ProgramCodeBlock;
    BlockPtr: PtrRec absolute ProgramCodeBlock;
    UnitCodeSize, NumberOfSegmentReferencesInUnitProgramCode: Word;
    ExeHeaderSize: LongRec;
begin
  UnitPtrRec.Seg := LastLoadedUsedUnit;
  Repeat
    UnitPtr^.CodeSegment := CurrentCodeSegment;
    ProgramCodeBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [stCodeBlocks]);
    UnitCodeSize := 0;
    While BlockPtr.Ofs <> UnitPtr^.BlockOffset [Succ (stCodeBlocks)] do
      begin
        If ProgramCodeBlock^.Offset <> BlockUnused then
          begin
            ProgramCodeBlock^.Offset := UnitCodeSize;
            Inc (UnitCodeSize, ProgramCodeBlock^.CodeSize);
          end;
        Inc (ProgramCodeBlock);
      end;
    NumberOfSegmentReferencesInUnitProgramCode := UnitPtr^.NumberOfSegmentReferencesInProgramCodeBlocks;
    If UnitPtr^.OverlayedUnitCodeSize <> 0 then
      begin
        UnitPtr^.OverlayedUnitCodeSize := UnitCodeSize;
        UnitCodeSize := (UnitCodeSize + $000F) shr 4;
        Inc (UnitCodeSize, (NumberOfSegmentReferencesInUnitProgramCode + $0007) shr 3);
        If UnitCodeSize > OverlayHeapSize then OverlayHeapSize := UnitCodeSize;
        UnitCodeSize := $20;
        BlockPtr.Ofs := UnitPtr^.BlockOffset [stProcedures];
        While BlockPtr.Ofs <> UnitPtr^.BlockOffset [Succ (stProcedures)] do
          begin
            If ProcedureRecordPtr^.OverlayedProcedureOffset <> 0 then
              begin
                ProcedureRecordPtr^.OverlayedProcedureOffset := UnitCodeSize;
                Inc (UnitCodeSize, 5);
              end;
            Inc (ProcedureRecordPtr);
          end;
        NumberOfSegmentReferencesInUnitProgramCode := 0;
      end;
    UnitPtr^.CodeSize := UnitCodeSize;
    Inc (CurrentCodeSegment, (UnitCodeSize + $000F) shr 4);
    ExeHeaderSize.Long := (NumberOfSegmentReferencesInUnitProgramCode +
                           UnitPtr^.NumberOfSegmentReferencesInTypedConstantsBlocks) * 4 + CurrentRelocationItemOffset;
    If (ExeHeaderSize.WordH <> 0) or (ExeHeaderSize.WordL > $FFF0) then Error (TooManyRelocationItems);
    CurrentRelocationItemOffset := ExeHeaderSize.WordL;
    UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
  until UnitPtrRec.Seg = 0;
  CodeBytes := 16 * LongInt (CurrentCodeSegment);
end;
```

### Code Block 2

```pascal
Procedure CalculateCodeBlockOffsetsAndTotalCodeSize;
Var ProgramCodeBlock: PProgramCodeBlockRecord;
    ProcedureRecordPtr: PProceduresBlockRecord absolute ProgramCodeBlock;
    BlockPtr: PtrRec absolute ProgramCodeBlock;
    UnitCodeSize, NumberOfSegmentReferencesInUnitProgramCode: Word;
    ExeHeaderSize: LongRec;
begin
  UnitPtrRec.Seg := LastLoadedUsedUnit;
  Repeat
    UnitPtr^.CodeSegment := CurrentCodeSegment;
    ProgramCodeBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [stCodeBlocks]);
    UnitCodeSize := 0;
    While BlockPtr.Ofs <> UnitPtr^.BlockOffset [Succ (stCodeBlocks)] do
      begin
        If ProgramCodeBlock^.Offset <> BlockUnused then
          begin
            ProgramCodeBlock^.Offset := UnitCodeSize;
            Inc (UnitCodeSize, ProgramCodeBlock^.CodeSize);
          end;
        Inc (ProgramCodeBlock);
      end;
    NumberOfSegmentReferencesInUnitProgramCode := UnitPtr^.NumberOfSegmentReferencesInProgramCodeBlocks;
    If UnitPtr^.OverlayedUnitCodeSize <> 0 then
      begin
        UnitPtr^.OverlayedUnitCodeSize := UnitCodeSize;
        UnitCodeSize := (UnitCodeSize + $000F) shr 4;
        Inc (UnitCodeSize, (NumberOfSegmentReferencesInUnitProgramCode + $0007) shr 3);
        If UnitCodeSize > OverlayHeapSize then OverlayHeapSize := UnitCodeSize;
        UnitCodeSize := $20;
        BlockPtr.Ofs := UnitPtr^.BlockOffset [stProcedures];
        While BlockPtr.Ofs <> UnitPtr^.BlockOffset [Succ (stProcedures)] do
          begin
            If ProcedureRecordPtr^.OverlayedProcedureOffset <> 0 then
              begin
                ProcedureRecordPtr^.OverlayedProcedureOffset := UnitCodeSize;
                Inc (UnitCodeSize, 5);
              end;
            Inc (ProcedureRecordPtr);
          end;
        NumberOfSegmentReferencesInUnitProgramCode := 0;
      end;
    UnitPtr^.CodeSize := UnitCodeSize;
    Inc (CurrentCodeSegment, (UnitCodeSize + $000F) shr 4);
    ExeHeaderSize.Long := (NumberOfSegmentReferencesInUnitProgramCode +
                           UnitPtr^.NumberOfSegmentReferencesInTypedConstantsBlocks) * 4 + CurrentRelocationItemOffset;
    If (ExeHeaderSize.WordH <> 0) or (ExeHeaderSize.WordL > $FFF0) then Error (TooManyRelocationItems);
    CurrentRelocationItemOffset := ExeHeaderSize.WordL;
    UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
  until UnitPtrRec.Seg = 0;
  CodeBytes := 16 * LongInt (CurrentCodeSegment);
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculating-offsets-of-code-blocks
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
- https://turbopascal.org/type-definitions

