# Marking Used Blocks - Turbo Pascal Compiler

**URL:** https://turbopascal.org/marking-used-blocks

---

## Table of Contents

- Marking Used Blocks

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Marking Used Blocks
Turbo Pascal uses “smart” linker which eleminates code and variables that are never referenced. This reduces code size and memory usage. First all symbol table blocks in all modules that reference variables, typed constants and code blocks are marked as unused. Finally, the main program block is marked as referenced since it will be executed when the program starts.
Procedure MarkProcCodeConstVarBlocksAsUnused;
Var ProcedureRecordPtr: PProceduresBlockRecord;
    ProcedureRecordPtrRec: PtrRec absolute ProcedureRecordPtr;
    CodeConstVarBlockRecord: PCodeConstVarBlockRecord absolute ProcedureRecordPtr;
    CodeConstVarBlockRecordOfs: Word absolute CodeConstVarBlockRecord;
begin
  UnitPtrRec.Seg := LastLoadedUsedUnit;
  Repeat
    With UnitPtr^ do
      begin
        NumberOfUnprocessedBlocksForReferences := 0;
        NumberOfSegmentReferencesInProgramCodeBlocks := 0;
        NumberOfSegmentReferencesInTypedConstantsBlocks := 0;
        ProcedureRecordPtr := Ptr (UnitPtrRec.Seg, BlockOffset [stProcedures]);
        While ProcedureRecordPtrRec.Ofs <> BlockOffset [Succ (stProcedures)] do
          begin
            ProcedureRecordPtr^.OverlayedProcedureOffset := 0;
            Inc (ProcedureRecordPtr);
          end;
        CodeConstVarBlockRecord := Ptr (UnitPtrRec.Seg, BlockOffset [stCodeBlocks]);
        While CodeConstVarBlockRecordOfs <> BlockOffset [Succ (stVariablesBlocks)] do
          begin
            CodeConstVarBlockRecord^.Offset := BlockUnused;
            Inc (CodeConstVarBlockRecord);
          end;
        UnitPtrRec.Seg := PreviousUnitSegment;
      end;
  until UnitPtrRec.Seg = 0;
  UnitPtrRec.Seg := SymbolTable [stMain].Segment;
  With UnitPtr^ do                                  { Mark main program code block }
    begin                                           { All references start from here }
      ProcedureRecordPtr := Ptr (UnitPtrRec.Seg, BlockOffset [stProcedures]);
      Inc (PProgramCodeBlockRecord (Ptr (UnitPtrRec.Seg,
           BlockOffset [stCodeBlocks] + ProcedureRecordPtr^.ProgramCodeBlockRecordOffset))^.Offset);
      Inc (NumberOfUnprocessedBlocksForReferences);
    end;
end;
This procedure marks all used (referenced) variable, typed constant and code blocks in a unit. Since this process may create blocks with new references this procedure is called repeatedly from
main linking procedure
until there are no more new references.
Procedure MarkUsedBlocksInUnit;

  Function ProcessUnprocessedBlocksForReferences (Block: TSymbolTable; Segment: Word): Word;
  Var CodeOrConstBlock: PCodeConstBlockRecord;
      CodeOrConstBlockRec: PtrRec absolute CodeOrConstBlock;
      Reference: PReferencesBlockRecord;
      ReferenceRec: PtrRec absolute Reference;
      SegmentReferencesInBlock: Word;

    Procedure ProcessBlockForReferences;
    Var EndOffset: Word;
        TempUnitPtr: PUnitHeader;
        TempUnitPtrRec: PtrRec absolute TempUnitPtr;
        ProcedureRecord: PProceduresBlockRecord;
        ProgramCodeBlockRecord: PProgramCodeBlockRecord absolute ProcedureRecord;
        TypedConstantsBlockRecord: PTypedConstantsBlockRecord absolute ProcedureRecord;
        VariablesBlock: PVariablesBlockRecord absolute ProcedureRecord;
        BlockRecordOfs: Word absolute ProcedureRecord;
        Flags: TReferenceFlagSet;
        ReferencedUnitRecord: PReferencedModulesBlockRecord;
        ReferencedUnitRecordRec: PtrRec absolute ReferencedUnitRecord;

    Label NextReference;

    begin
      EndOffset := ReferenceRec.Ofs + CodeOrConstBlock^.ReferencesSize;
      TempUnitPtrRec.Ofs := 0;
      While ReferenceRec.Ofs <> EndOffset do
        begin
          Flags := Reference^.Flags;
          ReferencedUnitRecord := PReferencedModulesBlockRecord (Ptr (CurrentUnitForProcessing,
            PUnitHeader (Ptr (CurrentUnitForProcessing, 0))^.BlockOffset [stReferencedModules] +
                          Reference^.ReferencedUnitRecord and $0FFF));
          TempUnitPtrRec.Seg := ReferencedUnitRecord^.ModuleSegment;
          ProgramCodeBlockRecord := Ptr (TempUnitPtrRec.Seg, Reference^.ReferencedBlockRecordOffset);
          If rfSegment in Flags then Inc (SegmentReferencesInBlock);
          Case rfDataSegment in Flags of
            True: If rfOffset in Flags then
                    Case rfConstant in Flags of
                      True: begin
                              Inc (BlockRecordOfs, TempUnitPtr^.BlockOffset [stTypedConstantsBlocks]);
                              If TypedConstantsBlockRecord^.Offset = BlockUnused then
                                begin
                                  Inc (TypedConstantsBlockRecord^.Offset); { BlockUsedButUnprocessed }
                                  Inc (TempUnitPtr^.NumberOfUnprocessedBlocksForReferences);
                                end
                            end;
                      False: begin
                               Inc (BlockRecordOfs, TempUnitPtr^.BlockOffset [stVariablesBlocks]);
                               VariablesBlock^.Offset := BlockUsedAndProcessed;
                             end;
                    end;
            False: begin
                     If not (rfConstant in Flags) then
                       begin
                         Inc (BlockRecordOfs, TempUnitPtr^.BlockOffset [stProcedures]);
                         If ProcedureRecord^.prW2 and $08 <> 0 then
                           begin
                             BlockRecordOfs := TempUnitPtr^.BlockOffset [stTypedConstantsBlocks] +
                                               ProcedureRecord^.ProgramCodeBlockRecordOffset;
                             If TypedConstantsBlockRecord^.Offset = BlockUnused then
                               begin
                                 Inc (TypedConstantsBlockRecord^.Offset); { BlockUsedButUnprocessed }
                                 Inc (TempUnitPtr^.NumberOfUnprocessedBlocksForReferences);
                               end;
                             Goto NextReference;
                           end;
                         If Flags * [rfSegment, rfOffset] <> [] then ProcedureRecord^.OverlayedProcedureOffset := 1;
                         BlockRecordOfs := ProcedureRecord^.ProgramCodeBlockRecordOffset;
                       end;
                     Inc (BlockRecordOfs, TempUnitPtr^.BlockOffset [stCodeBlocks]);
                     If ProgramCodeBlockRecord^.Offset = BlockUnused then
                       begin
                         Inc (ProgramCodeBlockRecord^.Offset); { BlockUsedButUnprocessed }
                         Inc (TempUnitPtr^.NumberOfUnprocessedBlocksForReferences);
                       end
                   end;
          end;

NextReference:

          Inc (ReferenceRec.Ofs, SizeOf (TReferencesBlockRecord));
        end;
    end;


  begin
    ReferenceRecordsSegment := Segment;
    Reference := Ptr (ReferenceRecordsSegment, 0);
    CodeOrConstBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [Block]);
    SegmentReferencesInBlock := 0;
    While CodeOrConstBlockRec.Ofs <> UnitPtr^.BlockOffset [Succ (Block)] do
      begin
        If CodeOrConstBlock^.Offset = BlockUsedButUnprocessed then
          begin
            Inc (CodeOrConstBlock^.Offset); { BlockUsedAndProcessed }
            Dec (UnitPtr^.NumberOfUnprocessedBlocksForReferences);
            ProcessBlockForReferences;
          end else Inc (ReferenceRec.Ofs, CodeOrConstBlock^.ReferencesSize);
        Inc (CodeOrConstBlock);
      end;
    ProcessUnprocessedBlocksForReferences := SegmentReferencesInBlock;
  end;

begin
  CurrentUnitForProcessing := UnitPtrRec.Seg;
  Repeat
    With UnitPtr^ do
      begin
        Inc (NumberOfSegmentReferencesInProgramCodeBlocks,
          ProcessUnprocessedBlocksForReferences (stCodeBlocks, SymbolTableSegment [stCodeReferences]));
        Inc (NumberOfSegmentReferencesInTypedConstantsBlocks,
          ProcessUnprocessedBlocksForReferences (stTypedConstantsBlocks, SymbolTableSegment [stTypedConstantsReferences]));
      end;
  until UnitPtr^.NumberOfUnprocessedBlocksForReferences = 0;
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
Procedure MarkProcCodeConstVarBlocksAsUnused;
Var ProcedureRecordPtr: PProceduresBlockRecord;
    ProcedureRecordPtrRec: PtrRec absolute ProcedureRecordPtr;
    CodeConstVarBlockRecord: PCodeConstVarBlockRecord absolute ProcedureRecordPtr;
    CodeConstVarBlockRecordOfs: Word absolute CodeConstVarBlockRecord;
begin
  UnitPtrRec.Seg := LastLoadedUsedUnit;
  Repeat
    With UnitPtr^ do
      begin
        NumberOfUnprocessedBlocksForReferences := 0;
        NumberOfSegmentReferencesInProgramCodeBlocks := 0;
        NumberOfSegmentReferencesInTypedConstantsBlocks := 0;
        ProcedureRecordPtr := Ptr (UnitPtrRec.Seg, BlockOffset [stProcedures]);
        While ProcedureRecordPtrRec.Ofs <> BlockOffset [Succ (stProcedures)] do
          begin
            ProcedureRecordPtr^.OverlayedProcedureOffset := 0;
            Inc (ProcedureRecordPtr);
          end;
        CodeConstVarBlockRecord := Ptr (UnitPtrRec.Seg, BlockOffset [stCodeBlocks]);
        While CodeConstVarBlockRecordOfs <> BlockOffset [Succ (stVariablesBlocks)] do
          begin
            CodeConstVarBlockRecord^.Offset := BlockUnused;
            Inc (CodeConstVarBlockRecord);
          end;
        UnitPtrRec.Seg := PreviousUnitSegment;
      end;
  until UnitPtrRec.Seg = 0;
  UnitPtrRec.Seg := SymbolTable [stMain].Segment;
  With UnitPtr^ do                                  { Mark main program code block }
    begin                                           { All references start from here }
      ProcedureRecordPtr := Ptr (UnitPtrRec.Seg, BlockOffset [stProcedures]);
      Inc (PProgramCodeBlockRecord (Ptr (UnitPtrRec.Seg,
           BlockOffset [stCodeBlocks] + ProcedureRecordPtr^.ProgramCodeBlockRecordOffset))^.Offset);
      Inc (NumberOfUnprocessedBlocksForReferences);
    end;
end;
```

### Code Block 2

```pascal
Procedure MarkUsedBlocksInUnit;

  Function ProcessUnprocessedBlocksForReferences (Block: TSymbolTable; Segment: Word): Word;
  Var CodeOrConstBlock: PCodeConstBlockRecord;
      CodeOrConstBlockRec: PtrRec absolute CodeOrConstBlock;
      Reference: PReferencesBlockRecord;
      ReferenceRec: PtrRec absolute Reference;
      SegmentReferencesInBlock: Word;

    Procedure ProcessBlockForReferences;
    Var EndOffset: Word;
        TempUnitPtr: PUnitHeader;
        TempUnitPtrRec: PtrRec absolute TempUnitPtr;
        ProcedureRecord: PProceduresBlockRecord;
        ProgramCodeBlockRecord: PProgramCodeBlockRecord absolute ProcedureRecord;
        TypedConstantsBlockRecord: PTypedConstantsBlockRecord absolute ProcedureRecord;
        VariablesBlock: PVariablesBlockRecord absolute ProcedureRecord;
        BlockRecordOfs: Word absolute ProcedureRecord;
        Flags: TReferenceFlagSet;
        ReferencedUnitRecord: PReferencedModulesBlockRecord;
        ReferencedUnitRecordRec: PtrRec absolute ReferencedUnitRecord;

    Label NextReference;

    begin
      EndOffset := ReferenceRec.Ofs + CodeOrConstBlock^.ReferencesSize;
      TempUnitPtrRec.Ofs := 0;
      While ReferenceRec.Ofs <> EndOffset do
        begin
          Flags := Reference^.Flags;
          ReferencedUnitRecord := PReferencedModulesBlockRecord (Ptr (CurrentUnitForProcessing,
            PUnitHeader (Ptr (CurrentUnitForProcessing, 0))^.BlockOffset [stReferencedModules] +
                          Reference^.ReferencedUnitRecord and $0FFF));
          TempUnitPtrRec.Seg := ReferencedUnitRecord^.ModuleSegment;
          ProgramCodeBlockRecord := Ptr (TempUnitPtrRec.Seg, Reference^.ReferencedBlockRecordOffset);
          If rfSegment in Flags then Inc (SegmentReferencesInBlock);
          Case rfDataSegment in Flags of
            True: If rfOffset in Flags then
                    Case rfConstant in Flags of
                      True: begin
                              Inc (BlockRecordOfs, TempUnitPtr^.BlockOffset [stTypedConstantsBlocks]);
                              If TypedConstantsBlockRecord^.Offset = BlockUnused then
                                begin
                                  Inc (TypedConstantsBlockRecord^.Offset); { BlockUsedButUnprocessed }
                                  Inc (TempUnitPtr^.NumberOfUnprocessedBlocksForReferences);
                                end
                            end;
                      False: begin
                               Inc (BlockRecordOfs, TempUnitPtr^.BlockOffset [stVariablesBlocks]);
                               VariablesBlock^.Offset := BlockUsedAndProcessed;
                             end;
                    end;
            False: begin
                     If not (rfConstant in Flags) then
                       begin
                         Inc (BlockRecordOfs, TempUnitPtr^.BlockOffset [stProcedures]);
                         If ProcedureRecord^.prW2 and $08 <> 0 then
                           begin
                             BlockRecordOfs := TempUnitPtr^.BlockOffset [stTypedConstantsBlocks] +
                                               ProcedureRecord^.ProgramCodeBlockRecordOffset;
                             If TypedConstantsBlockRecord^.Offset = BlockUnused then
                               begin
                                 Inc (TypedConstantsBlockRecord^.Offset); { BlockUsedButUnprocessed }
                                 Inc (TempUnitPtr^.NumberOfUnprocessedBlocksForReferences);
                               end;
                             Goto NextReference;
                           end;
                         If Flags * [rfSegment, rfOffset] <> [] then ProcedureRecord^.OverlayedProcedureOffset := 1;
                         BlockRecordOfs := ProcedureRecord^.ProgramCodeBlockRecordOffset;
                       end;
                     Inc (BlockRecordOfs, TempUnitPtr^.BlockOffset [stCodeBlocks]);
                     If ProgramCodeBlockRecord^.Offset = BlockUnused then
                       begin
                         Inc (ProgramCodeBlockRecord^.Offset); { BlockUsedButUnprocessed }
                         Inc (TempUnitPtr^.NumberOfUnprocessedBlocksForReferences);
                       end
                   end;
          end;

NextReference:

          Inc (ReferenceRec.Ofs, SizeOf (TReferencesBlockRecord));
        end;
    end;


  begin
    ReferenceRecordsSegment := Segment;
    Reference := Ptr (ReferenceRecordsSegment, 0);
    CodeOrConstBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [Block]);
    SegmentReferencesInBlock := 0;
    While CodeOrConstBlockRec.Ofs <> UnitPtr^.BlockOffset [Succ (Block)] do
      begin
        If CodeOrConstBlock^.Offset = BlockUsedButUnprocessed then
          begin
            Inc (CodeOrConstBlock^.Offset); { BlockUsedAndProcessed }
            Dec (UnitPtr^.NumberOfUnprocessedBlocksForReferences);
            ProcessBlockForReferences;
          end else Inc (ReferenceRec.Ofs, CodeOrConstBlock^.ReferencesSize);
        Inc (CodeOrConstBlock);
      end;
    ProcessUnprocessedBlocksForReferences := SegmentReferencesInBlock;
  end;

begin
  CurrentUnitForProcessing := UnitPtrRec.Seg;
  Repeat
    With UnitPtr^ do
      begin
        Inc (NumberOfSegmentReferencesInProgramCodeBlocks,
          ProcessUnprocessedBlocksForReferences (stCodeBlocks, SymbolTableSegment [stCodeReferences]));
        Inc (NumberOfSegmentReferencesInTypedConstantsBlocks,
          ProcessUnprocessedBlocksForReferences (stTypedConstantsBlocks, SymbolTableSegment [stTypedConstantsReferences]));
      end;
  until UnitPtr^.NumberOfUnprocessedBlocksForReferences = 0;
end;
```

### Code Block 3

```pascal
Procedure MarkProcCodeConstVarBlocksAsUnused;
Var ProcedureRecordPtr: PProceduresBlockRecord;
    ProcedureRecordPtrRec: PtrRec absolute ProcedureRecordPtr;
    CodeConstVarBlockRecord: PCodeConstVarBlockRecord absolute ProcedureRecordPtr;
    CodeConstVarBlockRecordOfs: Word absolute CodeConstVarBlockRecord;
begin
  UnitPtrRec.Seg := LastLoadedUsedUnit;
  Repeat
    With UnitPtr^ do
      begin
        NumberOfUnprocessedBlocksForReferences := 0;
        NumberOfSegmentReferencesInProgramCodeBlocks := 0;
        NumberOfSegmentReferencesInTypedConstantsBlocks := 0;
        ProcedureRecordPtr := Ptr (UnitPtrRec.Seg, BlockOffset [stProcedures]);
        While ProcedureRecordPtrRec.Ofs <> BlockOffset [Succ (stProcedures)] do
          begin
            ProcedureRecordPtr^.OverlayedProcedureOffset := 0;
            Inc (ProcedureRecordPtr);
          end;
        CodeConstVarBlockRecord := Ptr (UnitPtrRec.Seg, BlockOffset [stCodeBlocks]);
        While CodeConstVarBlockRecordOfs <> BlockOffset [Succ (stVariablesBlocks)] do
          begin
            CodeConstVarBlockRecord^.Offset := BlockUnused;
            Inc (CodeConstVarBlockRecord);
          end;
        UnitPtrRec.Seg := PreviousUnitSegment;
      end;
  until UnitPtrRec.Seg = 0;
  UnitPtrRec.Seg := SymbolTable [stMain].Segment;
  With UnitPtr^ do                                  { Mark main program code block }
    begin                                           { All references start from here }
      ProcedureRecordPtr := Ptr (UnitPtrRec.Seg, BlockOffset [stProcedures]);
      Inc (PProgramCodeBlockRecord (Ptr (UnitPtrRec.Seg,
           BlockOffset [stCodeBlocks] + ProcedureRecordPtr^.ProgramCodeBlockRecordOffset))^.Offset);
      Inc (NumberOfUnprocessedBlocksForReferences);
    end;
end;
```

### Code Block 4

```pascal
Procedure MarkUsedBlocksInUnit;

  Function ProcessUnprocessedBlocksForReferences (Block: TSymbolTable; Segment: Word): Word;
  Var CodeOrConstBlock: PCodeConstBlockRecord;
      CodeOrConstBlockRec: PtrRec absolute CodeOrConstBlock;
      Reference: PReferencesBlockRecord;
      ReferenceRec: PtrRec absolute Reference;
      SegmentReferencesInBlock: Word;

    Procedure ProcessBlockForReferences;
    Var EndOffset: Word;
        TempUnitPtr: PUnitHeader;
        TempUnitPtrRec: PtrRec absolute TempUnitPtr;
        ProcedureRecord: PProceduresBlockRecord;
        ProgramCodeBlockRecord: PProgramCodeBlockRecord absolute ProcedureRecord;
        TypedConstantsBlockRecord: PTypedConstantsBlockRecord absolute ProcedureRecord;
        VariablesBlock: PVariablesBlockRecord absolute ProcedureRecord;
        BlockRecordOfs: Word absolute ProcedureRecord;
        Flags: TReferenceFlagSet;
        ReferencedUnitRecord: PReferencedModulesBlockRecord;
        ReferencedUnitRecordRec: PtrRec absolute ReferencedUnitRecord;

    Label NextReference;

    begin
      EndOffset := ReferenceRec.Ofs + CodeOrConstBlock^.ReferencesSize;
      TempUnitPtrRec.Ofs := 0;
      While ReferenceRec.Ofs <> EndOffset do
        begin
          Flags := Reference^.Flags;
          ReferencedUnitRecord := PReferencedModulesBlockRecord (Ptr (CurrentUnitForProcessing,
            PUnitHeader (Ptr (CurrentUnitForProcessing, 0))^.BlockOffset [stReferencedModules] +
                          Reference^.ReferencedUnitRecord and $0FFF));
          TempUnitPtrRec.Seg := ReferencedUnitRecord^.ModuleSegment;
          ProgramCodeBlockRecord := Ptr (TempUnitPtrRec.Seg, Reference^.ReferencedBlockRecordOffset);
          If rfSegment in Flags then Inc (SegmentReferencesInBlock);
          Case rfDataSegment in Flags of
            True: If rfOffset in Flags then
                    Case rfConstant in Flags of
                      True: begin
                              Inc (BlockRecordOfs, TempUnitPtr^.BlockOffset [stTypedConstantsBlocks]);
                              If TypedConstantsBlockRecord^.Offset = BlockUnused then
                                begin
                                  Inc (TypedConstantsBlockRecord^.Offset); { BlockUsedButUnprocessed }
                                  Inc (TempUnitPtr^.NumberOfUnprocessedBlocksForReferences);
                                end
                            end;
                      False: begin
                               Inc (BlockRecordOfs, TempUnitPtr^.BlockOffset [stVariablesBlocks]);
                               VariablesBlock^.Offset := BlockUsedAndProcessed;
                             end;
                    end;
            False: begin
                     If not (rfConstant in Flags) then
                       begin
                         Inc (BlockRecordOfs, TempUnitPtr^.BlockOffset [stProcedures]);
                         If ProcedureRecord^.prW2 and $08 <> 0 then
                           begin
                             BlockRecordOfs := TempUnitPtr^.BlockOffset [stTypedConstantsBlocks] +
                                               ProcedureRecord^.ProgramCodeBlockRecordOffset;
                             If TypedConstantsBlockRecord^.Offset = BlockUnused then
                               begin
                                 Inc (TypedConstantsBlockRecord^.Offset); { BlockUsedButUnprocessed }
                                 Inc (TempUnitPtr^.NumberOfUnprocessedBlocksForReferences);
                               end;
                             Goto NextReference;
                           end;
                         If Flags * [rfSegment, rfOffset] <> [] then ProcedureRecord^.OverlayedProcedureOffset := 1;
                         BlockRecordOfs := ProcedureRecord^.ProgramCodeBlockRecordOffset;
                       end;
                     Inc (BlockRecordOfs, TempUnitPtr^.BlockOffset [stCodeBlocks]);
                     If ProgramCodeBlockRecord^.Offset = BlockUnused then
                       begin
                         Inc (ProgramCodeBlockRecord^.Offset); { BlockUsedButUnprocessed }
                         Inc (TempUnitPtr^.NumberOfUnprocessedBlocksForReferences);
                       end
                   end;
          end;

NextReference:

          Inc (ReferenceRec.Ofs, SizeOf (TReferencesBlockRecord));
        end;
    end;


  begin
    ReferenceRecordsSegment := Segment;
    Reference := Ptr (ReferenceRecordsSegment, 0);
    CodeOrConstBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [Block]);
    SegmentReferencesInBlock := 0;
    While CodeOrConstBlockRec.Ofs <> UnitPtr^.BlockOffset [Succ (Block)] do
      begin
        If CodeOrConstBlock^.Offset = BlockUsedButUnprocessed then
          begin
            Inc (CodeOrConstBlock^.Offset); { BlockUsedAndProcessed }
            Dec (UnitPtr^.NumberOfUnprocessedBlocksForReferences);
            ProcessBlockForReferences;
          end else Inc (ReferenceRec.Ofs, CodeOrConstBlock^.ReferencesSize);
        Inc (CodeOrConstBlock);
      end;
    ProcessUnprocessedBlocksForReferences := SegmentReferencesInBlock;
  end;

begin
  CurrentUnitForProcessing := UnitPtrRec.Seg;
  Repeat
    With UnitPtr^ do
      begin
        Inc (NumberOfSegmentReferencesInProgramCodeBlocks,
          ProcessUnprocessedBlocksForReferences (stCodeBlocks, SymbolTableSegment [stCodeReferences]));
        Inc (NumberOfSegmentReferencesInTypedConstantsBlocks,
          ProcessUnprocessedBlocksForReferences (stTypedConstantsBlocks, SymbolTableSegment [stTypedConstantsReferences]));
      end;
  until UnitPtr^.NumberOfUnprocessedBlocksForReferences = 0;
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
- https://turbopascal.org/linking-modules-and-creating-exe-file
- https://turbopascal.org/marking-used-blocks
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

