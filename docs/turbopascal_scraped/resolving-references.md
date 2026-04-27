# Resolving References - Turbo Pascal Compiler

**URL:** https://turbopascal.org/resolving-references

---

## Table of Contents

- Resolving References

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Resolving References
Once the
offsets of code blocks
,
variables
and typed constants are known, the compiler can resolve references. This procedure calculates all references (displacements, offsets, near and far pointers) to code or data.
Procedure ResolveReferences (Block: TSymbolTable; DataBlockSegment, ReferencesBlockSegment: Word);
Var CodeConstBlock: PCodeConstBlockRecord;
    CodeConstBlockRec: PtrRec absolute CodeConstBlock;
    ReferenceRecord: PReferencesBlockRecord;
    ReferenceRec: PtrRec absolute ReferenceRecord;
    ReferencedUnitRecord: PReferencedModulesBlockRecord;
    CurrentBlockOffset: Word;

  Procedure ResolveReferencesForUsedBlock;
  Var EndOffset: Word;
      TempUnit: PUnitHeader;
      BlockRecord: Word;
      ReferenceFlags: TReferenceFlagSet;
      ReferencePosition: Word;
      ReferencePointer: PPtrRec;
      ReferencePointerRec: PtrRec absolute ReferencePointer;
      ProcedureRecordPtr: PProceduresBlockRecord;
      Reference: PtrRec;
  begin
    EndOffset := ReferenceRec.Ofs + CodeConstBlock^.ReferencesSize;
    While ReferenceRec.Ofs <> EndOffset do
      begin
        TempUnit := Ptr (CurrentUnitForProcessing, 0);
        ReferencedUnitRecord := Ptr (CurrentUnitForProcessing,
                                     TempUnit^.BlockOffset [stReferencedModules] +
                                     ReferenceRecord^.ReferencedUnitRecord and $0FFF);
        TempUnit := Ptr (ReferencedUnitRecord^.ModuleSegment, 0);
        With ReferenceRecord^ do
          begin
            ReferenceFlags := Flags;
            BlockRecord := ReferencedBlockRecordOffset;
            Reference.Ofs := ReferencedOffset;
            ReferencePosition := PositionOfReference;
          end;
        If rfDataSegment in ReferenceFlags then
          begin
            Case rfConstant in ReferenceFlags of
              True: Inc (BlockRecord, TempUnit^.BlockOffset [stTypedConstantsBlocks]);
              else  Inc (BlockRecord, TempUnit^.BlockOffset [stVariablesBlocks]);
            end;
            Inc (Reference.Ofs, PConstVarBlockRecord (PChar (TempUnit) + BlockRecord)^.Offset);
            Reference.Seg := DataSegment;
          end else begin
                     If rfConstant in ReferenceFlags then
                       begin
                         Inc (BlockRecord, TempUnit^.BlockOffset [stCodeBlocks]);
                         Inc (Reference.Ofs, PProgramCodeBlockRecord (PChar (TempUnit) + BlockRecord)^.Offset);
                         Reference.Seg := TempUnit^.CodeSegment;
                       end else begin
                                  Inc (BlockRecord, TempUnit^.BlockOffset [stProcedures]);
                                  ProcedureRecordPtr := Ptr (Seg (TempUnit^), BlockRecord);
                                  If Lo (ProcedureRecordPtr^.prW2) and $08 <> 0 then
                                    begin
                                      Inc (Reference.Ofs, ProcedureRecordPtr^.SizeOfConstants);
                                      BlockRecord := ProcedureRecordPtr^.ProgramCodeBlockRecordOffset;
                                      Inc (BlockRecord, TempUnit^.BlockOffset [stTypedConstantsBlocks]);
                                      Inc (Reference.Ofs, PWord (PChar (TempUnit) + BlockRecord)^);
                                      Reference.Seg := DataSegment;
                                    end else begin
                                               If (ReferenceFlags * [rfSegment, rfOffset] = []) or
                                                  (UnitPtr^.OverlayedUnitCodeSize = 0) then
                                                 begin
                                                   Inc (Reference.Ofs, ProcedureRecordPtr^.SizeOfConstants);
                                                   BlockRecord := ProcedureRecordPtr^.ProgramCodeBlockRecordOffset;
                                                   Inc (BlockRecord, TempUnit^.BlockOffset [stCodeBlocks]);
                                                   Inc (Reference.Ofs, PWord (PChar (TempUnit) + BlockRecord)^);
                                                   Reference.Seg := TempUnit^.CodeSegment;
                                                 end else begin
                                                            Inc (Reference.Ofs, ProcedureRecordPtr^.OverlayedProcedureOffset);
                                                            Reference.Seg := TempUnit^.CodeSegment;
                                                          end;
                                             end;
                                end;
                   end;
        ReferencePointer := DataBlockPointer.Ptr;
        Inc (ReferencePointerRec.Ofs, ReferencePosition);
        Case rfOffset in ReferenceFlags of
          True: begin
                  ReferencePointer^.Ofs := Reference.Ofs;
                  If rfSegment in ReferenceFlags then ReferencePointer^.Seg := Reference.Seg;
                end;
          else If rfSegment in ReferenceFlags then ReferencePointer^.Word := Reference.Seg
                 else ReferencePointer^.Word := Reference.Ofs - ReferencePosition - UsedBlockOffset - 2;
        end;
        Inc (ReferenceRecord);
      end;
  end;

begin
  CurrentUnitForProcessing := UnitPtrRec.Seg;
  ReferenceRecordsSegment := ReferencesBlockSegment;
  DataBlockPointer.Seg := DataBlockSegment;
  CodeConstBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [Block]);
  CurrentBlockOffset := 0;
  ReferenceRecord := Ptr (ReferenceRecordsSegment, 0);
  While CodeConstBlockRec.Ofs <> UnitPtr^.BlockOffset [Succ (Block)] do
    begin
      If CodeConstBlock^.Offset <> BlockUnused then
        begin
          UsedBlockOffset := CodeConstBlock^.Offset;
          DataBlockPointer.Ofs := CurrentBlockOffset;
          ResolveReferencesForUsedBlock;
        end else Inc (ReferenceRec.Ofs, CodeConstBlock^.ReferencesSize);
      Inc (CurrentBlockOffset, CodeConstBlock^.Size);
      Inc (CodeConstBlock);
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
Procedure ResolveReferences (Block: TSymbolTable; DataBlockSegment, ReferencesBlockSegment: Word);
Var CodeConstBlock: PCodeConstBlockRecord;
    CodeConstBlockRec: PtrRec absolute CodeConstBlock;
    ReferenceRecord: PReferencesBlockRecord;
    ReferenceRec: PtrRec absolute ReferenceRecord;
    ReferencedUnitRecord: PReferencedModulesBlockRecord;
    CurrentBlockOffset: Word;

  Procedure ResolveReferencesForUsedBlock;
  Var EndOffset: Word;
      TempUnit: PUnitHeader;
      BlockRecord: Word;
      ReferenceFlags: TReferenceFlagSet;
      ReferencePosition: Word;
      ReferencePointer: PPtrRec;
      ReferencePointerRec: PtrRec absolute ReferencePointer;
      ProcedureRecordPtr: PProceduresBlockRecord;
      Reference: PtrRec;
  begin
    EndOffset := ReferenceRec.Ofs + CodeConstBlock^.ReferencesSize;
    While ReferenceRec.Ofs <> EndOffset do
      begin
        TempUnit := Ptr (CurrentUnitForProcessing, 0);
        ReferencedUnitRecord := Ptr (CurrentUnitForProcessing,
                                     TempUnit^.BlockOffset [stReferencedModules] +
                                     ReferenceRecord^.ReferencedUnitRecord and $0FFF);
        TempUnit := Ptr (ReferencedUnitRecord^.ModuleSegment, 0);
        With ReferenceRecord^ do
          begin
            ReferenceFlags := Flags;
            BlockRecord := ReferencedBlockRecordOffset;
            Reference.Ofs := ReferencedOffset;
            ReferencePosition := PositionOfReference;
          end;
        If rfDataSegment in ReferenceFlags then
          begin
            Case rfConstant in ReferenceFlags of
              True: Inc (BlockRecord, TempUnit^.BlockOffset [stTypedConstantsBlocks]);
              else  Inc (BlockRecord, TempUnit^.BlockOffset [stVariablesBlocks]);
            end;
            Inc (Reference.Ofs, PConstVarBlockRecord (PChar (TempUnit) + BlockRecord)^.Offset);
            Reference.Seg := DataSegment;
          end else begin
                     If rfConstant in ReferenceFlags then
                       begin
                         Inc (BlockRecord, TempUnit^.BlockOffset [stCodeBlocks]);
                         Inc (Reference.Ofs, PProgramCodeBlockRecord (PChar (TempUnit) + BlockRecord)^.Offset);
                         Reference.Seg := TempUnit^.CodeSegment;
                       end else begin
                                  Inc (BlockRecord, TempUnit^.BlockOffset [stProcedures]);
                                  ProcedureRecordPtr := Ptr (Seg (TempUnit^), BlockRecord);
                                  If Lo (ProcedureRecordPtr^.prW2) and $08 <> 0 then
                                    begin
                                      Inc (Reference.Ofs, ProcedureRecordPtr^.SizeOfConstants);
                                      BlockRecord := ProcedureRecordPtr^.ProgramCodeBlockRecordOffset;
                                      Inc (BlockRecord, TempUnit^.BlockOffset [stTypedConstantsBlocks]);
                                      Inc (Reference.Ofs, PWord (PChar (TempUnit) + BlockRecord)^);
                                      Reference.Seg := DataSegment;
                                    end else begin
                                               If (ReferenceFlags * [rfSegment, rfOffset] = []) or
                                                  (UnitPtr^.OverlayedUnitCodeSize = 0) then
                                                 begin
                                                   Inc (Reference.Ofs, ProcedureRecordPtr^.SizeOfConstants);
                                                   BlockRecord := ProcedureRecordPtr^.ProgramCodeBlockRecordOffset;
                                                   Inc (BlockRecord, TempUnit^.BlockOffset [stCodeBlocks]);
                                                   Inc (Reference.Ofs, PWord (PChar (TempUnit) + BlockRecord)^);
                                                   Reference.Seg := TempUnit^.CodeSegment;
                                                 end else begin
                                                            Inc (Reference.Ofs, ProcedureRecordPtr^.OverlayedProcedureOffset);
                                                            Reference.Seg := TempUnit^.CodeSegment;
                                                          end;
                                             end;
                                end;
                   end;
        ReferencePointer := DataBlockPointer.Ptr;
        Inc (ReferencePointerRec.Ofs, ReferencePosition);
        Case rfOffset in ReferenceFlags of
          True: begin
                  ReferencePointer^.Ofs := Reference.Ofs;
                  If rfSegment in ReferenceFlags then ReferencePointer^.Seg := Reference.Seg;
                end;
          else If rfSegment in ReferenceFlags then ReferencePointer^.Word := Reference.Seg
                 else ReferencePointer^.Word := Reference.Ofs - ReferencePosition - UsedBlockOffset - 2;
        end;
        Inc (ReferenceRecord);
      end;
  end;

begin
  CurrentUnitForProcessing := UnitPtrRec.Seg;
  ReferenceRecordsSegment := ReferencesBlockSegment;
  DataBlockPointer.Seg := DataBlockSegment;
  CodeConstBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [Block]);
  CurrentBlockOffset := 0;
  ReferenceRecord := Ptr (ReferenceRecordsSegment, 0);
  While CodeConstBlockRec.Ofs <> UnitPtr^.BlockOffset [Succ (Block)] do
    begin
      If CodeConstBlock^.Offset <> BlockUnused then
        begin
          UsedBlockOffset := CodeConstBlock^.Offset;
          DataBlockPointer.Ofs := CurrentBlockOffset;
          ResolveReferencesForUsedBlock;
        end else Inc (ReferenceRec.Ofs, CodeConstBlock^.ReferencesSize);
      Inc (CurrentBlockOffset, CodeConstBlock^.Size);
      Inc (CodeConstBlock);
    end;
end;
```

### Code Block 2

```pascal
Procedure ResolveReferences (Block: TSymbolTable; DataBlockSegment, ReferencesBlockSegment: Word);
Var CodeConstBlock: PCodeConstBlockRecord;
    CodeConstBlockRec: PtrRec absolute CodeConstBlock;
    ReferenceRecord: PReferencesBlockRecord;
    ReferenceRec: PtrRec absolute ReferenceRecord;
    ReferencedUnitRecord: PReferencedModulesBlockRecord;
    CurrentBlockOffset: Word;

  Procedure ResolveReferencesForUsedBlock;
  Var EndOffset: Word;
      TempUnit: PUnitHeader;
      BlockRecord: Word;
      ReferenceFlags: TReferenceFlagSet;
      ReferencePosition: Word;
      ReferencePointer: PPtrRec;
      ReferencePointerRec: PtrRec absolute ReferencePointer;
      ProcedureRecordPtr: PProceduresBlockRecord;
      Reference: PtrRec;
  begin
    EndOffset := ReferenceRec.Ofs + CodeConstBlock^.ReferencesSize;
    While ReferenceRec.Ofs <> EndOffset do
      begin
        TempUnit := Ptr (CurrentUnitForProcessing, 0);
        ReferencedUnitRecord := Ptr (CurrentUnitForProcessing,
                                     TempUnit^.BlockOffset [stReferencedModules] +
                                     ReferenceRecord^.ReferencedUnitRecord and $0FFF);
        TempUnit := Ptr (ReferencedUnitRecord^.ModuleSegment, 0);
        With ReferenceRecord^ do
          begin
            ReferenceFlags := Flags;
            BlockRecord := ReferencedBlockRecordOffset;
            Reference.Ofs := ReferencedOffset;
            ReferencePosition := PositionOfReference;
          end;
        If rfDataSegment in ReferenceFlags then
          begin
            Case rfConstant in ReferenceFlags of
              True: Inc (BlockRecord, TempUnit^.BlockOffset [stTypedConstantsBlocks]);
              else  Inc (BlockRecord, TempUnit^.BlockOffset [stVariablesBlocks]);
            end;
            Inc (Reference.Ofs, PConstVarBlockRecord (PChar (TempUnit) + BlockRecord)^.Offset);
            Reference.Seg := DataSegment;
          end else begin
                     If rfConstant in ReferenceFlags then
                       begin
                         Inc (BlockRecord, TempUnit^.BlockOffset [stCodeBlocks]);
                         Inc (Reference.Ofs, PProgramCodeBlockRecord (PChar (TempUnit) + BlockRecord)^.Offset);
                         Reference.Seg := TempUnit^.CodeSegment;
                       end else begin
                                  Inc (BlockRecord, TempUnit^.BlockOffset [stProcedures]);
                                  ProcedureRecordPtr := Ptr (Seg (TempUnit^), BlockRecord);
                                  If Lo (ProcedureRecordPtr^.prW2) and $08 <> 0 then
                                    begin
                                      Inc (Reference.Ofs, ProcedureRecordPtr^.SizeOfConstants);
                                      BlockRecord := ProcedureRecordPtr^.ProgramCodeBlockRecordOffset;
                                      Inc (BlockRecord, TempUnit^.BlockOffset [stTypedConstantsBlocks]);
                                      Inc (Reference.Ofs, PWord (PChar (TempUnit) + BlockRecord)^);
                                      Reference.Seg := DataSegment;
                                    end else begin
                                               If (ReferenceFlags * [rfSegment, rfOffset] = []) or
                                                  (UnitPtr^.OverlayedUnitCodeSize = 0) then
                                                 begin
                                                   Inc (Reference.Ofs, ProcedureRecordPtr^.SizeOfConstants);
                                                   BlockRecord := ProcedureRecordPtr^.ProgramCodeBlockRecordOffset;
                                                   Inc (BlockRecord, TempUnit^.BlockOffset [stCodeBlocks]);
                                                   Inc (Reference.Ofs, PWord (PChar (TempUnit) + BlockRecord)^);
                                                   Reference.Seg := TempUnit^.CodeSegment;
                                                 end else begin
                                                            Inc (Reference.Ofs, ProcedureRecordPtr^.OverlayedProcedureOffset);
                                                            Reference.Seg := TempUnit^.CodeSegment;
                                                          end;
                                             end;
                                end;
                   end;
        ReferencePointer := DataBlockPointer.Ptr;
        Inc (ReferencePointerRec.Ofs, ReferencePosition);
        Case rfOffset in ReferenceFlags of
          True: begin
                  ReferencePointer^.Ofs := Reference.Ofs;
                  If rfSegment in ReferenceFlags then ReferencePointer^.Seg := Reference.Seg;
                end;
          else If rfSegment in ReferenceFlags then ReferencePointer^.Word := Reference.Seg
                 else ReferencePointer^.Word := Reference.Ofs - ReferencePosition - UsedBlockOffset - 2;
        end;
        Inc (ReferenceRecord);
      end;
  end;

begin
  CurrentUnitForProcessing := UnitPtrRec.Seg;
  ReferenceRecordsSegment := ReferencesBlockSegment;
  DataBlockPointer.Seg := DataBlockSegment;
  CodeConstBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [Block]);
  CurrentBlockOffset := 0;
  ReferenceRecord := Ptr (ReferenceRecordsSegment, 0);
  While CodeConstBlockRec.Ofs <> UnitPtr^.BlockOffset [Succ (Block)] do
    begin
      If CodeConstBlock^.Offset <> BlockUnused then
        begin
          UsedBlockOffset := CodeConstBlock^.Offset;
          DataBlockPointer.Ofs := CurrentBlockOffset;
          ResolveReferencesForUsedBlock;
        end else Inc (ReferenceRec.Ofs, CodeConstBlock^.ReferencesSize);
      Inc (CurrentBlockOffset, CodeConstBlock^.Size);
      Inc (CodeConstBlock);
    end;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculating-offsets-of-code-blocks
- https://turbopascal.org/calculating-offsets-of-variables
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
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

