# Linking Modules and Creating EXE File - Turbo Pascal Compiler

**URL:** https://turbopascal.org/linking-modules-and-creating-exe-file

---

## Table of Contents

- Linking Modules and Creating EXE File

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Linking Modules and Creating EXE File
This is the last phase of program compilation – linking all modules and creating the executable file for program. Turbo Pascal first
marks all symbol table blocks
for variables, typed constants, procedures and code blocks as unused, then it iterates through all modules and marks used blocks starting from the main program, after that it calculates
code offsets
,
variable offsets
,
resolves references
, and finally, it writes code to either overlay file or to the
executable file
.
Procedure Link;
Var SavedHeapPtr1: Word;
    SavedHeapPtr2: Word;
    UnitCodeSize: Word;
    NumberOfUnprocessedUnitsForReferences: Word;

begin
  UnitPtrRec.Ofs := 0;
  If not (LinkBufferOnDisk in LinkerOptions) then
    begin
      UnitPtrRec.Seg := LastLoadedUsedUnit;
      Repeat
        LoadUnitFileSymbolTables (stCode, stTypedConstantsReferences);
        UnitPtr^.UnitPathAndNameOffset := 0;
        UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
      until UnitPtrRec.Seg = 0;
    end;
  MarkProcCodeConstVarBlocksAsUnused;
  PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.OverlayedUnitCodeSize := 0;
  PUnitHeader (Ptr (SystemUnitSegment, 0))^.OverlayedUnitCodeSize := 0;
  Repeat
    UnitPtrRec.Seg := LastLoadedUsedUnit;
    NumberOfUnprocessedUnitsForReferences := 0;
    Repeat
      If UnitPtr^.NumberOfUnprocessedBlocksForReferences <> 0 then
        begin
          SavedHeapPtr1 := HeapPtrRec.Seg;
          LoadUnitFileSymbolTables (stCodeReferences, stTypedConstantsReferences);
          MarkUsedBlocksInUnit;
          HeapPtrRec.Seg := SavedHeapPtr1;
          Inc (NumberOfUnprocessedUnitsForReferences);
        end;
      UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
    until UnitPtrRec.Seg = 0;
  until NumberOfUnprocessedUnitsForReferences = 0;
  CurrentCodeSegment := 0;
  OverlayHeapSize := 0;
  CurrentRelocationItemOffset := RelocationTableOffset;
  CalculateCodeBlockOffsetsAndTotalCodeSize;
  DataSegment := CurrentCodeSegment;
  CalculateVariableOffsetsAndTotalDataSize;
  CalculateFirstFreeSegment;
  If not (cmoCreateExeFile in CompilerModeOptions) then
    begin
      UnitPtrRec.Seg := LastLoadedUsedUnit;
      HeapPtrRec.Seg := UnitPtr^.SymbolTableSegment [stCode];
      Exit;
    end;
  SavedHeapPtr1 := HeapPtrRec.Seg;
  CreateExeFile;
  If OverlayHeapSize <> 0 then
    begin
      FindFilePath (StrCopy (@Identifier, CurrentFileName), Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_OVR);
      CreateFile (OverlayFile, @Identifier);
      OutputFileHandle := FileRec (OverlayFile).Handle;
      Seek (OverlayFile, 8);
    end;
  OverlayCodeList := 0;
  OverlayHeader.OverlaySize.Long := 8;
  ExeHeaderBuffer.Ofs := RelocationTableOffset;
  UnitPtrRec.Seg := LastLoadedUsedUnit;
  Repeat
    SavedHeapPtr2 := HeapPtrRec.Seg;
    LoadUnitFileSymbolTables (stCode, stTypedConstantsReferences);
    With  UnitPtr^ do
      begin
        UnitCodeSize := CodeSize;
        If UnitCodeSize < OverlayedUnitCodeSize then UnitCodeSize := OverlayedUnitCodeSize;
      end;
    CurrentUnitCodeSegment := HeapPtrRec.Seg;
    Inc (HeapPtrRec.Seg, (UnitCodeSize + $000F) shr 4);
    If HeapPtrRec.Seg > HeapEndRec.Seg then Error (OutOfMemory);
    ResolveReferences (stCodeBlocks, UnitPtr^.SymbolTableSegment [stCode], UnitPtr^.SymbolTableSegment [stCodeReferences]);
    JoinUsedSymbolTables (stCodeBlocks, UnitPtr^.SymbolTableSegment [stCode], CurrentUnitCodeSegment);
    If UnitPtr^.OverlayedUnitCodeSize <> 0 then
      begin
        BlockWrite (OverlayFile, Ptr (CurrentUnitCodeSegment, 0)^, UnitPtr^.OverlayedUnitCodeSize);
        CreateSegmentReferencesTableForOverlayedUnit;
        BlockWrite (OverlayFile, Ptr (CurrentUnitCodeSegment, 0)^, UnitPtr^.NumberOfSegmentReferencesInProgramCodeBlocks * 2);
        CreateProcedureOffsetsTableForOverlayedUnit;
      end else WriteRelocationItems (stCodeBlocks, UnitPtr^.SymbolTableSegment [stCodeReferences], UnitPtr^.CodeSegment);
    FillChar (Ptr (CurrentUnitCodeSegment, UnitPtr^.CodeSize)^, 16 - UnitPtr^.CodeSize and $000F, 0);
    BlockWrite (ExeFile, Ptr (CurrentUnitCodeSegment, 0)^, (UnitPtr^.CodeSize + $000F) and $FFF0);
    ResolveReferences (stTypedConstantsBlocks, UnitPtr^.SymbolTableSegment [stTypedConstants],
                                             UnitPtr^.SymbolTableSegment [stTypedConstantsReferences]);
    JoinUsedSymbolTables (stTypedConstantsBlocks, UnitPtr^.SymbolTableSegment [stTypedConstants], TypedConstantsPointer.Seg);
    WriteRelocationItems (stTypedConstantsBlocks, UnitPtr^.SymbolTableSegment [stTypedConstantsReferences], DataSegment);
    HeapPtrRec.Seg := SavedHeapPtr2;
    UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
  until UnitPtrRec.Seg = 0;
  WriteTypedConstantsAndExeHeader;
  If OverlayHeapSize <> 0 then
    begin
      Seek (OverlayFile, 0);
      Dec (OverlayHeader.OverlaySize.Long, 8);
      BlockWrite (OverlayFile, OverlayHeader, 9);
      Close (OverlayFile);
    end;
  HeapPtrRec.Seg := SavedHeapPtr1;
  UnitPtrRec.Seg := LastLoadedUsedUnit;
  HeapPtrRec.Seg := UnitPtr^.SymbolTableSegment [stCode];
  If DebugInformationInExe in LinkerOptions then
    begin
      Seek (ExeFile, ExeFileSize);
      SavedHeapPtr1 := HeapPtrRec.Seg;
      AddDebugInfoToExeFile;
      HeapPtrRec.Seg := SavedHeapPtr1;
    end;
  ExeFileHandle := 0;
  Close (ExeFile);
  If LinkerOptions * [MapFileWithSegments, MapFileWithPublics] <> [] then
    begin
      SavedHeapPtr1 := HeapPtrRec.Seg;
      CreateMapFile;
      HeapPtrRec.Seg := SavedHeapPtr1;
    end;
end;
This procedure writes relocation items (pointers to segment references) for all used blocks in a unit.
Procedure WriteRelocationItems (Block: TSymbolTable; Seg1, Seg2: Word);
Var CodeConstVarBlockRecord: PCodeConstVarBlockRecord;
    CodeConstVarBlockRecordPtrRec: PtrRec absolute CodeConstVarBlockRecord;
    ReferencesBlockRecord: PReferencesBlockRecord;
    ReferencesBlockRecordPtrRec: PtrRec absolute ReferencesBlockRecord;

  Procedure WriteRelocationItemsForBlock;
  Var EndOffset: Word;
  begin
    EndOffset := ReferencesBlockRecordPtrRec.Ofs + CodeConstVarBlockRecord^.ReferencesSize;
    While ReferencesBlockRecordPtrRec.Ofs <> EndOffset do
      begin
        If rfSegment in ReferencesBlockRecord^.Flags then
          begin
            With RelocationTable^ do
              begin
                Offset := ReferencesBlockRecord^.PositionOfReference + CodeConstVarBlockRecord^.Offset;
                If rfOffset in ReferencesBlockRecord^.Flags then Inc (Offset, 2);
                Segment := Seg2;
              end;
            Inc (RelocationTable);
          end;
        Inc (ReferencesBlockRecord);
      end;
  end;

begin
  ReferenceRecordsSegment := Seg1;
  CodeConstVarBlockRecord := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [Block]);
  ReferencesBlockRecord := Ptr (ReferenceRecordsSegment, 0);
  While CodeConstVarBlockRecordPtrRec.Ofs <> UnitPtr^.BlockOffset [Succ (Block)] do
    begin
      If CodeConstVarBlockRecord^.Offset <> $FFFF then WriteRelocationItemsForBlock else
        Inc (ReferencesBlockRecordPtrRec.Ofs, CodeConstVarBlockRecord^.ReferencesSize);
      Inc (CodeConstVarBlockRecord);
    end;
end;
This procedure creates table of offsets to each segment reference in a unit.
Procedure CreateSegmentReferencesTableForOverlayedUnit;
Var ProgramCodeBlock: PCodeConstVarBlockRecord;
    ProgramCodeBlockPtrRec: PtrRec absolute ProgramCodeBlock;
    ReferencesBlockRecord: PReferencesBlockRecord;
    ReferencesBlockRecordPtrRec: PtrRec absolute ReferencesBlockRecord;
    SegmentReferencesTable: PWord;

  Procedure CreateSegmentReferencesTableForOneCodeBlock;
  Var EndOffset, OffsetOfReference: Word;
  begin
    EndOffset := ReferencesBlockRecordPtrRec.Ofs + ProgramCodeBlock^.ReferencesSize;
    While ReferencesBlockRecordPtrRec.Ofs <> EndOffset do
      begin
        If rfSegment in ReferencesBlockRecord^.Flags then
          begin
            OffsetOfReference := ReferencesBlockRecord^.PositionOfReference + ProgramCodeBlock^.Offset;
            If rfOffset in ReferencesBlockRecord^.Flags then Inc (OffsetOfReference, 2);
            SegmentReferencesTable^ := OffsetOfReference;
            Inc (SegmentReferencesTable);
          end;
        Inc (ReferencesBlockRecordPtrRec.Ofs, 8);
      end;
  end;

begin
  ReferenceRecordsSegment := UnitPtr^.SymbolTableSegment [stCodeReferences];
  ReferencesBlockRecord := Ptr (ReferenceRecordsSegment, 0);
  SegmentReferencesTable := Ptr (CurrentUnitCodeSegment, 0);
  ProgramCodeBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [stCodeBlocks]);
  While ProgramCodeBlockPtrRec.Ofs <> UnitPtr^.BlockOffset [Succ (stCodeBlocks)] do
    begin
      If ProgramCodeBlock^.Offset <> $FFFF then
        CreateSegmentReferencesTableForOneCodeBlock else
          Inc (ReferencesBlockRecordPtrRec.Ofs, ProgramCodeBlock^.ReferencesSize);
      Inc (ProgramCodeBlock);
    end;
end;
This procedure creates table of offsets to procedures in an overlayed unit.
Procedure CreateProcedureOffsetsTableForOverlayedUnit;
Var TempPtr: PWord;
    ProceduresBlockRecord: PProceduresBlockRecord;
    ProceduresBlockRecordPtrRec: PtrRec absolute ProceduresBlockRecord;
    N: Byte;
begin
  TempPtr := Ptr (CurrentUnitCodeSegment, 0);
  TempPtr^ := $3FCD;
  Inc (TempPtr);
  TempPtr^ := 0;
  Inc (TempPtr);
  TempPtr^ := OverlayHeader.OverlaySize.WordL;
  Inc (TempPtr);
  TempPtr^ := OverlayHeader.OverlaySize.WordH;
  Inc (TempPtr);
  TempPtr^ := UnitPtr^.OverlayedUnitCodeSize;
  Inc (TempPtr);
  TempPtr^ := UnitPtr^.NumberOfSegmentReferencesInProgramCodeBlocks * 2;
  Inc (TempPtr);
  TempPtr^ := (UnitPtr^.CodeSize - $20) div 5;
  Inc (TempPtr);
  TempPtr^ := OverlayCodeList;
  Inc (TempPtr);
  For N := 1 to 8 do
    begin
      TempPtr^ := 0;
      Inc (TempPtr);
    end;
  ProceduresBlockRecord := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [stProcedures]);
  Repeat
    If ProceduresBlockRecord^.OverlayedProcedureOffset <> 0 then
      begin
        TempPtr^ := $3FCD;
        Inc (TempPtr);
        TempPtr^ := ProceduresBlockRecord^.SizeOfConstants + PProgramCodeBlockRecord (Ptr (UnitPtrRec.Seg,
                       UnitPtr^.BlockOffset [stCodeBlocks] + ProceduresBlockRecord^.ProgramCodeBlockRecordOffset))^.Offset;
        Inc (TempPtr);
        Byte (TempPtr^) := 0;
        Inc (PChar (TempPtr));
      end;
    Inc (ProceduresBlockRecord);
  until ProceduresBlockRecordPtrRec.Ofs = UnitPtr^.BlockOffset [Succ (stProcedures)];
  Inc (OverlayHeader.OverlaySize.Long, UnitPtr^.OverlayedUnitCodeSize);
  Inc (OverlayHeader.OverlaySize.Long, UnitPtr^.NumberOfSegmentReferencesInProgramCodeBlocks * 2);
  OverlayCodeList := UnitPtr^.CodeSegment;
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
Procedure Link;
Var SavedHeapPtr1: Word;
    SavedHeapPtr2: Word;
    UnitCodeSize: Word;
    NumberOfUnprocessedUnitsForReferences: Word;

begin
  UnitPtrRec.Ofs := 0;
  If not (LinkBufferOnDisk in LinkerOptions) then
    begin
      UnitPtrRec.Seg := LastLoadedUsedUnit;
      Repeat
        LoadUnitFileSymbolTables (stCode, stTypedConstantsReferences);
        UnitPtr^.UnitPathAndNameOffset := 0;
        UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
      until UnitPtrRec.Seg = 0;
    end;
  MarkProcCodeConstVarBlocksAsUnused;
  PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.OverlayedUnitCodeSize := 0;
  PUnitHeader (Ptr (SystemUnitSegment, 0))^.OverlayedUnitCodeSize := 0;
  Repeat
    UnitPtrRec.Seg := LastLoadedUsedUnit;
    NumberOfUnprocessedUnitsForReferences := 0;
    Repeat
      If UnitPtr^.NumberOfUnprocessedBlocksForReferences <> 0 then
        begin
          SavedHeapPtr1 := HeapPtrRec.Seg;
          LoadUnitFileSymbolTables (stCodeReferences, stTypedConstantsReferences);
          MarkUsedBlocksInUnit;
          HeapPtrRec.Seg := SavedHeapPtr1;
          Inc (NumberOfUnprocessedUnitsForReferences);
        end;
      UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
    until UnitPtrRec.Seg = 0;
  until NumberOfUnprocessedUnitsForReferences = 0;
  CurrentCodeSegment := 0;
  OverlayHeapSize := 0;
  CurrentRelocationItemOffset := RelocationTableOffset;
  CalculateCodeBlockOffsetsAndTotalCodeSize;
  DataSegment := CurrentCodeSegment;
  CalculateVariableOffsetsAndTotalDataSize;
  CalculateFirstFreeSegment;
  If not (cmoCreateExeFile in CompilerModeOptions) then
    begin
      UnitPtrRec.Seg := LastLoadedUsedUnit;
      HeapPtrRec.Seg := UnitPtr^.SymbolTableSegment [stCode];
      Exit;
    end;
  SavedHeapPtr1 := HeapPtrRec.Seg;
  CreateExeFile;
  If OverlayHeapSize <> 0 then
    begin
      FindFilePath (StrCopy (@Identifier, CurrentFileName), Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_OVR);
      CreateFile (OverlayFile, @Identifier);
      OutputFileHandle := FileRec (OverlayFile).Handle;
      Seek (OverlayFile, 8);
    end;
  OverlayCodeList := 0;
  OverlayHeader.OverlaySize.Long := 8;
  ExeHeaderBuffer.Ofs := RelocationTableOffset;
  UnitPtrRec.Seg := LastLoadedUsedUnit;
  Repeat
    SavedHeapPtr2 := HeapPtrRec.Seg;
    LoadUnitFileSymbolTables (stCode, stTypedConstantsReferences);
    With  UnitPtr^ do
      begin
        UnitCodeSize := CodeSize;
        If UnitCodeSize < OverlayedUnitCodeSize then UnitCodeSize := OverlayedUnitCodeSize;
      end;
    CurrentUnitCodeSegment := HeapPtrRec.Seg;
    Inc (HeapPtrRec.Seg, (UnitCodeSize + $000F) shr 4);
    If HeapPtrRec.Seg > HeapEndRec.Seg then Error (OutOfMemory);
    ResolveReferences (stCodeBlocks, UnitPtr^.SymbolTableSegment [stCode], UnitPtr^.SymbolTableSegment [stCodeReferences]);
    JoinUsedSymbolTables (stCodeBlocks, UnitPtr^.SymbolTableSegment [stCode], CurrentUnitCodeSegment);
    If UnitPtr^.OverlayedUnitCodeSize <> 0 then
      begin
        BlockWrite (OverlayFile, Ptr (CurrentUnitCodeSegment, 0)^, UnitPtr^.OverlayedUnitCodeSize);
        CreateSegmentReferencesTableForOverlayedUnit;
        BlockWrite (OverlayFile, Ptr (CurrentUnitCodeSegment, 0)^, UnitPtr^.NumberOfSegmentReferencesInProgramCodeBlocks * 2);
        CreateProcedureOffsetsTableForOverlayedUnit;
      end else WriteRelocationItems (stCodeBlocks, UnitPtr^.SymbolTableSegment [stCodeReferences], UnitPtr^.CodeSegment);
    FillChar (Ptr (CurrentUnitCodeSegment, UnitPtr^.CodeSize)^, 16 - UnitPtr^.CodeSize and $000F, 0);
    BlockWrite (ExeFile, Ptr (CurrentUnitCodeSegment, 0)^, (UnitPtr^.CodeSize + $000F) and $FFF0);
    ResolveReferences (stTypedConstantsBlocks, UnitPtr^.SymbolTableSegment [stTypedConstants],
                                             UnitPtr^.SymbolTableSegment [stTypedConstantsReferences]);
    JoinUsedSymbolTables (stTypedConstantsBlocks, UnitPtr^.SymbolTableSegment [stTypedConstants], TypedConstantsPointer.Seg);
    WriteRelocationItems (stTypedConstantsBlocks, UnitPtr^.SymbolTableSegment [stTypedConstantsReferences], DataSegment);
    HeapPtrRec.Seg := SavedHeapPtr2;
    UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
  until UnitPtrRec.Seg = 0;
  WriteTypedConstantsAndExeHeader;
  If OverlayHeapSize <> 0 then
    begin
      Seek (OverlayFile, 0);
      Dec (OverlayHeader.OverlaySize.Long, 8);
      BlockWrite (OverlayFile, OverlayHeader, 9);
      Close (OverlayFile);
    end;
  HeapPtrRec.Seg := SavedHeapPtr1;
  UnitPtrRec.Seg := LastLoadedUsedUnit;
  HeapPtrRec.Seg := UnitPtr^.SymbolTableSegment [stCode];
  If DebugInformationInExe in LinkerOptions then
    begin
      Seek (ExeFile, ExeFileSize);
      SavedHeapPtr1 := HeapPtrRec.Seg;
      AddDebugInfoToExeFile;
      HeapPtrRec.Seg := SavedHeapPtr1;
    end;
  ExeFileHandle := 0;
  Close (ExeFile);
  If LinkerOptions * [MapFileWithSegments, MapFileWithPublics] <> [] then
    begin
      SavedHeapPtr1 := HeapPtrRec.Seg;
      CreateMapFile;
      HeapPtrRec.Seg := SavedHeapPtr1;
    end;
end;
```

### Code Block 2

```pascal
Procedure WriteRelocationItems (Block: TSymbolTable; Seg1, Seg2: Word);
Var CodeConstVarBlockRecord: PCodeConstVarBlockRecord;
    CodeConstVarBlockRecordPtrRec: PtrRec absolute CodeConstVarBlockRecord;
    ReferencesBlockRecord: PReferencesBlockRecord;
    ReferencesBlockRecordPtrRec: PtrRec absolute ReferencesBlockRecord;

  Procedure WriteRelocationItemsForBlock;
  Var EndOffset: Word;
  begin
    EndOffset := ReferencesBlockRecordPtrRec.Ofs + CodeConstVarBlockRecord^.ReferencesSize;
    While ReferencesBlockRecordPtrRec.Ofs <> EndOffset do
      begin
        If rfSegment in ReferencesBlockRecord^.Flags then
          begin
            With RelocationTable^ do
              begin
                Offset := ReferencesBlockRecord^.PositionOfReference + CodeConstVarBlockRecord^.Offset;
                If rfOffset in ReferencesBlockRecord^.Flags then Inc (Offset, 2);
                Segment := Seg2;
              end;
            Inc (RelocationTable);
          end;
        Inc (ReferencesBlockRecord);
      end;
  end;

begin
  ReferenceRecordsSegment := Seg1;
  CodeConstVarBlockRecord := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [Block]);
  ReferencesBlockRecord := Ptr (ReferenceRecordsSegment, 0);
  While CodeConstVarBlockRecordPtrRec.Ofs <> UnitPtr^.BlockOffset [Succ (Block)] do
    begin
      If CodeConstVarBlockRecord^.Offset <> $FFFF then WriteRelocationItemsForBlock else
        Inc (ReferencesBlockRecordPtrRec.Ofs, CodeConstVarBlockRecord^.ReferencesSize);
      Inc (CodeConstVarBlockRecord);
    end;
end;
```

### Code Block 3

```pascal
Procedure CreateSegmentReferencesTableForOverlayedUnit;
Var ProgramCodeBlock: PCodeConstVarBlockRecord;
    ProgramCodeBlockPtrRec: PtrRec absolute ProgramCodeBlock;
    ReferencesBlockRecord: PReferencesBlockRecord;
    ReferencesBlockRecordPtrRec: PtrRec absolute ReferencesBlockRecord;
    SegmentReferencesTable: PWord;

  Procedure CreateSegmentReferencesTableForOneCodeBlock;
  Var EndOffset, OffsetOfReference: Word;
  begin
    EndOffset := ReferencesBlockRecordPtrRec.Ofs + ProgramCodeBlock^.ReferencesSize;
    While ReferencesBlockRecordPtrRec.Ofs <> EndOffset do
      begin
        If rfSegment in ReferencesBlockRecord^.Flags then
          begin
            OffsetOfReference := ReferencesBlockRecord^.PositionOfReference + ProgramCodeBlock^.Offset;
            If rfOffset in ReferencesBlockRecord^.Flags then Inc (OffsetOfReference, 2);
            SegmentReferencesTable^ := OffsetOfReference;
            Inc (SegmentReferencesTable);
          end;
        Inc (ReferencesBlockRecordPtrRec.Ofs, 8);
      end;
  end;

begin
  ReferenceRecordsSegment := UnitPtr^.SymbolTableSegment [stCodeReferences];
  ReferencesBlockRecord := Ptr (ReferenceRecordsSegment, 0);
  SegmentReferencesTable := Ptr (CurrentUnitCodeSegment, 0);
  ProgramCodeBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [stCodeBlocks]);
  While ProgramCodeBlockPtrRec.Ofs <> UnitPtr^.BlockOffset [Succ (stCodeBlocks)] do
    begin
      If ProgramCodeBlock^.Offset <> $FFFF then
        CreateSegmentReferencesTableForOneCodeBlock else
          Inc (ReferencesBlockRecordPtrRec.Ofs, ProgramCodeBlock^.ReferencesSize);
      Inc (ProgramCodeBlock);
    end;
end;
```

### Code Block 4

```pascal
Procedure CreateProcedureOffsetsTableForOverlayedUnit;
Var TempPtr: PWord;
    ProceduresBlockRecord: PProceduresBlockRecord;
    ProceduresBlockRecordPtrRec: PtrRec absolute ProceduresBlockRecord;
    N: Byte;
begin
  TempPtr := Ptr (CurrentUnitCodeSegment, 0);
  TempPtr^ := $3FCD;
  Inc (TempPtr);
  TempPtr^ := 0;
  Inc (TempPtr);
  TempPtr^ := OverlayHeader.OverlaySize.WordL;
  Inc (TempPtr);
  TempPtr^ := OverlayHeader.OverlaySize.WordH;
  Inc (TempPtr);
  TempPtr^ := UnitPtr^.OverlayedUnitCodeSize;
  Inc (TempPtr);
  TempPtr^ := UnitPtr^.NumberOfSegmentReferencesInProgramCodeBlocks * 2;
  Inc (TempPtr);
  TempPtr^ := (UnitPtr^.CodeSize - $20) div 5;
  Inc (TempPtr);
  TempPtr^ := OverlayCodeList;
  Inc (TempPtr);
  For N := 1 to 8 do
    begin
      TempPtr^ := 0;
      Inc (TempPtr);
    end;
  ProceduresBlockRecord := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [stProcedures]);
  Repeat
    If ProceduresBlockRecord^.OverlayedProcedureOffset <> 0 then
      begin
        TempPtr^ := $3FCD;
        Inc (TempPtr);
        TempPtr^ := ProceduresBlockRecord^.SizeOfConstants + PProgramCodeBlockRecord (Ptr (UnitPtrRec.Seg,
                       UnitPtr^.BlockOffset [stCodeBlocks] + ProceduresBlockRecord^.ProgramCodeBlockRecordOffset))^.Offset;
        Inc (TempPtr);
        Byte (TempPtr^) := 0;
        Inc (PChar (TempPtr));
      end;
    Inc (ProceduresBlockRecord);
  until ProceduresBlockRecordPtrRec.Ofs = UnitPtr^.BlockOffset [Succ (stProcedures)];
  Inc (OverlayHeader.OverlaySize.Long, UnitPtr^.OverlayedUnitCodeSize);
  Inc (OverlayHeader.OverlaySize.Long, UnitPtr^.NumberOfSegmentReferencesInProgramCodeBlocks * 2);
  OverlayCodeList := UnitPtr^.CodeSegment;
end;
```

### Code Block 5

```pascal
Procedure Link;
Var SavedHeapPtr1: Word;
    SavedHeapPtr2: Word;
    UnitCodeSize: Word;
    NumberOfUnprocessedUnitsForReferences: Word;

begin
  UnitPtrRec.Ofs := 0;
  If not (LinkBufferOnDisk in LinkerOptions) then
    begin
      UnitPtrRec.Seg := LastLoadedUsedUnit;
      Repeat
        LoadUnitFileSymbolTables (stCode, stTypedConstantsReferences);
        UnitPtr^.UnitPathAndNameOffset := 0;
        UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
      until UnitPtrRec.Seg = 0;
    end;
  MarkProcCodeConstVarBlocksAsUnused;
  PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.OverlayedUnitCodeSize := 0;
  PUnitHeader (Ptr (SystemUnitSegment, 0))^.OverlayedUnitCodeSize := 0;
  Repeat
    UnitPtrRec.Seg := LastLoadedUsedUnit;
    NumberOfUnprocessedUnitsForReferences := 0;
    Repeat
      If UnitPtr^.NumberOfUnprocessedBlocksForReferences <> 0 then
        begin
          SavedHeapPtr1 := HeapPtrRec.Seg;
          LoadUnitFileSymbolTables (stCodeReferences, stTypedConstantsReferences);
          MarkUsedBlocksInUnit;
          HeapPtrRec.Seg := SavedHeapPtr1;
          Inc (NumberOfUnprocessedUnitsForReferences);
        end;
      UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
    until UnitPtrRec.Seg = 0;
  until NumberOfUnprocessedUnitsForReferences = 0;
  CurrentCodeSegment := 0;
  OverlayHeapSize := 0;
  CurrentRelocationItemOffset := RelocationTableOffset;
  CalculateCodeBlockOffsetsAndTotalCodeSize;
  DataSegment := CurrentCodeSegment;
  CalculateVariableOffsetsAndTotalDataSize;
  CalculateFirstFreeSegment;
  If not (cmoCreateExeFile in CompilerModeOptions) then
    begin
      UnitPtrRec.Seg := LastLoadedUsedUnit;
      HeapPtrRec.Seg := UnitPtr^.SymbolTableSegment [stCode];
      Exit;
    end;
  SavedHeapPtr1 := HeapPtrRec.Seg;
  CreateExeFile;
  If OverlayHeapSize <> 0 then
    begin
      FindFilePath (StrCopy (@Identifier, CurrentFileName), Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_OVR);
      CreateFile (OverlayFile, @Identifier);
      OutputFileHandle := FileRec (OverlayFile).Handle;
      Seek (OverlayFile, 8);
    end;
  OverlayCodeList := 0;
  OverlayHeader.OverlaySize.Long := 8;
  ExeHeaderBuffer.Ofs := RelocationTableOffset;
  UnitPtrRec.Seg := LastLoadedUsedUnit;
  Repeat
    SavedHeapPtr2 := HeapPtrRec.Seg;
    LoadUnitFileSymbolTables (stCode, stTypedConstantsReferences);
    With  UnitPtr^ do
      begin
        UnitCodeSize := CodeSize;
        If UnitCodeSize < OverlayedUnitCodeSize then UnitCodeSize := OverlayedUnitCodeSize;
      end;
    CurrentUnitCodeSegment := HeapPtrRec.Seg;
    Inc (HeapPtrRec.Seg, (UnitCodeSize + $000F) shr 4);
    If HeapPtrRec.Seg > HeapEndRec.Seg then Error (OutOfMemory);
    ResolveReferences (stCodeBlocks, UnitPtr^.SymbolTableSegment [stCode], UnitPtr^.SymbolTableSegment [stCodeReferences]);
    JoinUsedSymbolTables (stCodeBlocks, UnitPtr^.SymbolTableSegment [stCode], CurrentUnitCodeSegment);
    If UnitPtr^.OverlayedUnitCodeSize <> 0 then
      begin
        BlockWrite (OverlayFile, Ptr (CurrentUnitCodeSegment, 0)^, UnitPtr^.OverlayedUnitCodeSize);
        CreateSegmentReferencesTableForOverlayedUnit;
        BlockWrite (OverlayFile, Ptr (CurrentUnitCodeSegment, 0)^, UnitPtr^.NumberOfSegmentReferencesInProgramCodeBlocks * 2);
        CreateProcedureOffsetsTableForOverlayedUnit;
      end else WriteRelocationItems (stCodeBlocks, UnitPtr^.SymbolTableSegment [stCodeReferences], UnitPtr^.CodeSegment);
    FillChar (Ptr (CurrentUnitCodeSegment, UnitPtr^.CodeSize)^, 16 - UnitPtr^.CodeSize and $000F, 0);
    BlockWrite (ExeFile, Ptr (CurrentUnitCodeSegment, 0)^, (UnitPtr^.CodeSize + $000F) and $FFF0);
    ResolveReferences (stTypedConstantsBlocks, UnitPtr^.SymbolTableSegment [stTypedConstants],
                                             UnitPtr^.SymbolTableSegment [stTypedConstantsReferences]);
    JoinUsedSymbolTables (stTypedConstantsBlocks, UnitPtr^.SymbolTableSegment [stTypedConstants], TypedConstantsPointer.Seg);
    WriteRelocationItems (stTypedConstantsBlocks, UnitPtr^.SymbolTableSegment [stTypedConstantsReferences], DataSegment);
    HeapPtrRec.Seg := SavedHeapPtr2;
    UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
  until UnitPtrRec.Seg = 0;
  WriteTypedConstantsAndExeHeader;
  If OverlayHeapSize <> 0 then
    begin
      Seek (OverlayFile, 0);
      Dec (OverlayHeader.OverlaySize.Long, 8);
      BlockWrite (OverlayFile, OverlayHeader, 9);
      Close (OverlayFile);
    end;
  HeapPtrRec.Seg := SavedHeapPtr1;
  UnitPtrRec.Seg := LastLoadedUsedUnit;
  HeapPtrRec.Seg := UnitPtr^.SymbolTableSegment [stCode];
  If DebugInformationInExe in LinkerOptions then
    begin
      Seek (ExeFile, ExeFileSize);
      SavedHeapPtr1 := HeapPtrRec.Seg;
      AddDebugInfoToExeFile;
      HeapPtrRec.Seg := SavedHeapPtr1;
    end;
  ExeFileHandle := 0;
  Close (ExeFile);
  If LinkerOptions * [MapFileWithSegments, MapFileWithPublics] <> [] then
    begin
      SavedHeapPtr1 := HeapPtrRec.Seg;
      CreateMapFile;
      HeapPtrRec.Seg := SavedHeapPtr1;
    end;
end;
```

### Code Block 6

```pascal
Procedure WriteRelocationItems (Block: TSymbolTable; Seg1, Seg2: Word);
Var CodeConstVarBlockRecord: PCodeConstVarBlockRecord;
    CodeConstVarBlockRecordPtrRec: PtrRec absolute CodeConstVarBlockRecord;
    ReferencesBlockRecord: PReferencesBlockRecord;
    ReferencesBlockRecordPtrRec: PtrRec absolute ReferencesBlockRecord;

  Procedure WriteRelocationItemsForBlock;
  Var EndOffset: Word;
  begin
    EndOffset := ReferencesBlockRecordPtrRec.Ofs + CodeConstVarBlockRecord^.ReferencesSize;
    While ReferencesBlockRecordPtrRec.Ofs <> EndOffset do
      begin
        If rfSegment in ReferencesBlockRecord^.Flags then
          begin
            With RelocationTable^ do
              begin
                Offset := ReferencesBlockRecord^.PositionOfReference + CodeConstVarBlockRecord^.Offset;
                If rfOffset in ReferencesBlockRecord^.Flags then Inc (Offset, 2);
                Segment := Seg2;
              end;
            Inc (RelocationTable);
          end;
        Inc (ReferencesBlockRecord);
      end;
  end;

begin
  ReferenceRecordsSegment := Seg1;
  CodeConstVarBlockRecord := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [Block]);
  ReferencesBlockRecord := Ptr (ReferenceRecordsSegment, 0);
  While CodeConstVarBlockRecordPtrRec.Ofs <> UnitPtr^.BlockOffset [Succ (Block)] do
    begin
      If CodeConstVarBlockRecord^.Offset <> $FFFF then WriteRelocationItemsForBlock else
        Inc (ReferencesBlockRecordPtrRec.Ofs, CodeConstVarBlockRecord^.ReferencesSize);
      Inc (CodeConstVarBlockRecord);
    end;
end;
```

### Code Block 7

```pascal
Procedure CreateSegmentReferencesTableForOverlayedUnit;
Var ProgramCodeBlock: PCodeConstVarBlockRecord;
    ProgramCodeBlockPtrRec: PtrRec absolute ProgramCodeBlock;
    ReferencesBlockRecord: PReferencesBlockRecord;
    ReferencesBlockRecordPtrRec: PtrRec absolute ReferencesBlockRecord;
    SegmentReferencesTable: PWord;

  Procedure CreateSegmentReferencesTableForOneCodeBlock;
  Var EndOffset, OffsetOfReference: Word;
  begin
    EndOffset := ReferencesBlockRecordPtrRec.Ofs + ProgramCodeBlock^.ReferencesSize;
    While ReferencesBlockRecordPtrRec.Ofs <> EndOffset do
      begin
        If rfSegment in ReferencesBlockRecord^.Flags then
          begin
            OffsetOfReference := ReferencesBlockRecord^.PositionOfReference + ProgramCodeBlock^.Offset;
            If rfOffset in ReferencesBlockRecord^.Flags then Inc (OffsetOfReference, 2);
            SegmentReferencesTable^ := OffsetOfReference;
            Inc (SegmentReferencesTable);
          end;
        Inc (ReferencesBlockRecordPtrRec.Ofs, 8);
      end;
  end;

begin
  ReferenceRecordsSegment := UnitPtr^.SymbolTableSegment [stCodeReferences];
  ReferencesBlockRecord := Ptr (ReferenceRecordsSegment, 0);
  SegmentReferencesTable := Ptr (CurrentUnitCodeSegment, 0);
  ProgramCodeBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [stCodeBlocks]);
  While ProgramCodeBlockPtrRec.Ofs <> UnitPtr^.BlockOffset [Succ (stCodeBlocks)] do
    begin
      If ProgramCodeBlock^.Offset <> $FFFF then
        CreateSegmentReferencesTableForOneCodeBlock else
          Inc (ReferencesBlockRecordPtrRec.Ofs, ProgramCodeBlock^.ReferencesSize);
      Inc (ProgramCodeBlock);
    end;
end;
```

### Code Block 8

```pascal
Procedure CreateProcedureOffsetsTableForOverlayedUnit;
Var TempPtr: PWord;
    ProceduresBlockRecord: PProceduresBlockRecord;
    ProceduresBlockRecordPtrRec: PtrRec absolute ProceduresBlockRecord;
    N: Byte;
begin
  TempPtr := Ptr (CurrentUnitCodeSegment, 0);
  TempPtr^ := $3FCD;
  Inc (TempPtr);
  TempPtr^ := 0;
  Inc (TempPtr);
  TempPtr^ := OverlayHeader.OverlaySize.WordL;
  Inc (TempPtr);
  TempPtr^ := OverlayHeader.OverlaySize.WordH;
  Inc (TempPtr);
  TempPtr^ := UnitPtr^.OverlayedUnitCodeSize;
  Inc (TempPtr);
  TempPtr^ := UnitPtr^.NumberOfSegmentReferencesInProgramCodeBlocks * 2;
  Inc (TempPtr);
  TempPtr^ := (UnitPtr^.CodeSize - $20) div 5;
  Inc (TempPtr);
  TempPtr^ := OverlayCodeList;
  Inc (TempPtr);
  For N := 1 to 8 do
    begin
      TempPtr^ := 0;
      Inc (TempPtr);
    end;
  ProceduresBlockRecord := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [stProcedures]);
  Repeat
    If ProceduresBlockRecord^.OverlayedProcedureOffset <> 0 then
      begin
        TempPtr^ := $3FCD;
        Inc (TempPtr);
        TempPtr^ := ProceduresBlockRecord^.SizeOfConstants + PProgramCodeBlockRecord (Ptr (UnitPtrRec.Seg,
                       UnitPtr^.BlockOffset [stCodeBlocks] + ProceduresBlockRecord^.ProgramCodeBlockRecordOffset))^.Offset;
        Inc (TempPtr);
        Byte (TempPtr^) := 0;
        Inc (PChar (TempPtr));
      end;
    Inc (ProceduresBlockRecord);
  until ProceduresBlockRecordPtrRec.Ofs = UnitPtr^.BlockOffset [Succ (stProcedures)];
  Inc (OverlayHeader.OverlaySize.Long, UnitPtr^.OverlayedUnitCodeSize);
  Inc (OverlayHeader.OverlaySize.Long, UnitPtr^.NumberOfSegmentReferencesInProgramCodeBlocks * 2);
  OverlayCodeList := UnitPtr^.CodeSegment;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculating-code-size-and-offsets
- https://turbopascal.org/calculating-offsets-of-variables
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-executable-file
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/linking-modules-and-creating-exe-file
- https://turbopascal.org/marking-used-blocks
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/resolving-references
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

