# Generating Code - Turbo Pascal Compiler

**URL:** https://turbopascal.org/generating-code

---

## Table of Contents

- Generating Code

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Generating Code
This procedure generates target (x86) code from intermediate code instructions. This is the last phase in
program block compilation
. If debug info is enabled, source line code offsets will be saved into dedicated table, redundant loadings of addresses to ES:DI register pair will be
removed
and near jumps will be
converted to short jumps
where possible. The latest optimization of jump instructions will be performed in a loop since each optimization reduces code size and provides opportunity for further jump optimizations. After the optimizations are finished the code size and instruction offsets are already
calculated
.
Procedure GenerateCode;
Var Block9Record: PStatementCodeSizesBlockRecord;
    ProcedureIdentifierData: PProcedureIdentifierData;
    LocalDebugProcedureIdentifier: Word;
    StatementCodeSize: Word;
    N, Dif: Integer;
    CodePointer: Pointer;
    CodePointerRec: PtrRec absolute CodePointer;
    IntermediateCodePointer: PIntermediateCodeRecord;
    IntermediateCodePointerRec: PtrRec absolute IntermediateCodePointer;
    LabelIdentifier: PIdentifier;
    LabelIdentifierData: PLabelIdentifierData absolute LabelIdentifier;
    LabelIdentifierDataRec: PtrRec absolute LabelIdentifierData;
    ErrorString: PString;
begin
  If DebugInformation in ModuleCompilerSwitches then
    With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 3))^ do
      begin
        RecordType := icSourceLineNumber;
        StatementLineNumber := LastProgramBlockSourceLineNumber + 1;
      end;
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 1))^ do RecordType := icEnd;
  RemoveRedundant_ES_DI_AddressLoading;
  Repeat
    CalculateCodeSizeAndOffsets;
  until not ConvertNearJumpsToShort;
  If DebugInformation in ModuleCompilerSwitches then
    begin
      ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
      Block9Record := IncreaseSymbolTable (stSourceLineCodeOffsets,
                        2 * (LastProgramBlockSourceLineNumber - FirstProgramBlocktSourceLineNumber) + 14);
      If (LocalDebugSymbols in ModuleCompilerSwitches) then
        begin
          If not (pfAssembler in ProcedureIdentifierData^.Flags ) or
            (PushedParametersSize or ProgramBlockMaxStackFrameOffset <> 0) then
              LocalDebugProcedureIdentifier := CurrentProcedureIdentifier else LocalDebugProcedureIdentifier := 0;
        end else LocalDebugProcedureIdentifier := 0;
      With Block9Record^ do
        begin
          ProcedureIdentifier := LocalDebugProcedureIdentifier;
          SourceFileRecordOffset := CurrentSourceFile^.FileRecordOffset;
          ProcedureStartLine := ProcedureStartLineNumber;
          StatementStartCodeOffset := SymbolTable [stCode].UsedSize;
          SizeOfConstantsInCode := SymbolTable [stCode].UsedSize - LastProgramBlockCodeSize;
          FirstSourceLineNumber := FirstProgramBlocktSourceLineNumber;
          NumberOfSourceLines := LastProgramBlockSourceLineNumber - FirstProgramBlocktSourceLineNumber + 1;
          SymbolTable [stSourceLineCodeOffsets].UsedSize := Ofs (StatementsCodeSizeData);
        end;
      StatementFirstSourceLineNumber := FirstProgramBlocktSourceLineNumber;
    end;
  SymbolTable [stCodeReferences].UsedSize := Ofs (IncreaseSymbolTable (stCodeReferences, SizeOfCodeReferenceRecords)^);
  CodePointer := IncreaseSymbolTable (stCode, SizeOfGeneratedCode);
  IntermediateCodePointer := Ptr (SymbolTable [stIntermediateCode].Segment, LastIntermediateCodeSubroutine);
  Repeat
    Case IntermediateCodePointer^.RecordType of
      icGoSub: begin
                 Asm
                   MOV    AX, WORD PTR IntermediateCodePointer
                   ADD    AX, 3
                   PUSH   AX
                 end;
                 IntermediateCodePointerRec.Ofs := IntermediateCodePointer^.NewOffset;
               end;
      icReturn: Asm
                  POP    WORD PTR IntermediateCodePointer
                end;
      icByte: begin
              Byte (CodePointer^) := IntermediateCodePointer^.ByteCode;
              Inc (CodePointerRec.Ofs);
              Inc (IntermediateCodePointerRec.Ofs, 2);
            end;
      icWord: begin
              Word (CodePointer^) := IntermediateCodePointer^.WordCode.Word;
              Inc (CodePointerRec.Ofs, 2);
              Inc (IntermediateCodePointerRec.Ofs, 3);
            end;
      icJumpNear: With IntermediateCodePointer^ do
                    begin
                      If JumpOpCode <> JMP_ShortDirect then
                        begin
                          If JumpOpCode >= JMP_ConditionalShortLimit then Error (CodeGenerationError);
                          Byte (CodePointer^) := JumpOpCode xor $01;          { Invert condition }
                          Inc (CodePointerRec.Ofs);
                          Byte (CodePointer^) := 3;                           { Size of next near jump }
                          Inc (CodePointerRec.Ofs);
                        end;
                      Byte (CodePointer^) := JMP_NearDirect;
                      Inc (CodePointerRec.Ofs);
                      Word (CodePointer^) :=
                        PIntermediateCodeRecord (Ptr (SymbolTable [stIntermediateCode].Segment, LabelRecordOffset - 1))^
                        .LabelAddress - CodePointerRec.Ofs - 2;
                      Inc (CodePointerRec.Ofs, 2);
                      Inc (IntermediateCodePointerRec.Ofs, 8);
                    end;
      icJumpShort: With IntermediateCodePointer^ do
                     begin
                       Byte (CodePointer^) := JumpOpCode;
                       Inc (CodePointerRec.Ofs);
                       Byte (CodePointer^) :=
                         PIntermediateCodeRecord (Ptr (SymbolTable [stIntermediateCode].Segment, LabelRecordOffset - 1))^.
                           LabelAddress - CodePointerRec.Ofs - 1;
                       Inc (CodePointerRec.Ofs);
                       Inc (IntermediateCodePointerRec.Ofs, 8);
                     end;
      icLabel,
      icSkippedGoSubSaveAddress: Inc (IntermediateCodePointerRec.Ofs, 3);
      icReference: With IntermediateCodePointer^ do
                     begin
                       With PReferencesBlockRecord (SymbolTable [stCodeReferences].Ptr)^ do
                         begin
                           ReferencedUnitRecord :=        ReferencedUnitRecordOfs;
                           ReferencedBlockRecordOffset := ReferencedBlockRecordOfs;
                           ReferencedOffset :=            ReferencedOfs;
                           PositionOfReference :=         CodePointerRec.Ofs - LastProgramBlockCodeSize;
                         end;
                       Inc (SymbolTable [stCodeReferences].UsedSize, SizeOf (TReferencesBlockRecord));
                       Word (CodePointer^) := 0;
                       Inc (CodePointerRec.Ofs, 2);
                       If Flags * [rfSegment, rfOffset] = [rfSegment, rfOffset] then
                         begin
                           Word (CodePointer^) := 0;
                           Inc (CodePointerRec.Ofs, 2);
                         end;
                       Inc (IntermediateCodePointerRec.Ofs, 7);
                     end;
      icReferenceAsmBlock: With IntermediateCodePointer^ do
                             begin
                               LabelIdentifier := Ptr (SymbolTable [stMain].Segment, LabelIdentifierOfs);
                               LabelIdentifierDataRec.Seg := SymbolTable [stMain].Segment;
                               ErrorString := @LabelIdentifier^.Name.Str;
                               Inc (LabelIdentifierDataRec.Ofs, LabelIdentifier^.Name.Len + 4);
                               If LabelIdentifierData^.LabelRecordOffset = 0 then
                                 IdentifierError (ErrorString^, UndefinedLabelInPrecedingStatementPart);
                               With PReferencesBlockRecord (SymbolTable [stCodeReferences].Ptr)^ do
                                 begin
                                   ReferencedUnitRecord := ReferencedUnitRecordOfs;
                                   ReferencedBlockRecordOffset := SymbolTable [stCodeBlocks].NextRecordOffset;
                                   ReferencedOffset :=
                                     W5_10 + PIntermediateCodeRecord (Ptr (SymbolTable [stIntermediateCode].Segment,
                                     LabelIdentifierData^.LabelRecordOffset - 1))^.LabelAddress -
                                       LastProgramBlockCodeSize;
                                   PositionOfReference := CodePointerRec.Ofs - LastProgramBlockCodeSize;
                                 end;
                               Inc (SymbolTable [stCodeReferences].UsedSize, SizeOf (TReferencesBlockRecord));
                               Word (CodePointer^) := 0;
                               Inc (CodePointerRec.Ofs, 2);
                               If Flags * [rfSegment, rfOffset] = [rfSegment, rfOffset] then
                                 begin
                                   Word (CodePointer^) := 0;
                                   Inc (CodePointerRec.Ofs, 2);
                                 end;
                               Inc (IntermediateCodePointerRec.Ofs, 7);
                             end;
      ic12,
      icSourceLineNumber: With IntermediateCodePointer^ do
                            begin
                              Dif := StatementLineNumber - StatementFirstSourceLineNumber;
                              If Dif > 0 then
                                begin
                                  StatementFirstSourceLineNumber := StatementLineNumber;          { Next statement }
                                  StatementCodeSize := CodePointerRec.Ofs - StatementStartCodeOffset;
                                  StatementStartCodeOffset := CodePointerRec.Ofs;
                                  If StatementCodeSize >= 128 then
                                    begin
                                      Byte (SymbolTable [stSourceLineCodeOffsets].Ptr^) := Hi (StatementCodeSize) or $80;
                                      Inc (SymbolTable [stSourceLineCodeOffsets].UsedSize);
                                    end;
                                  Byte (SymbolTable [stSourceLineCodeOffsets].Ptr^) := Lo (StatementCodeSize);
                                  Inc (SymbolTable [stSourceLineCodeOffsets].UsedSize);
                                  For N := 1 to Dif - 1 do
                                    begin
                                      Byte (SymbolTable [stSourceLineCodeOffsets].Ptr^) := 0;
                                      Inc (SymbolTable [stSourceLineCodeOffsets].UsedSize);
                                    end;
                                end;
                              Inc (IntermediateCodePointerRec.Ofs, 3);
                            end;
      icLoad_ES_DI: begin
                       With IntermediateCodePointer^ do
                         GenerateCode_LES_DI_WithVariableAddress (Displacement, DestinationProcedure);
                       Inc (IntermediateCodePointerRec.Ofs, 7);
                     end;
      ic_ES_DI_Loaded: Inc (IntermediateCodePointerRec.Ofs, 7);
      ic_ES_DI_Destroyed: Inc (IntermediateCodePointerRec.Ofs);
      icEnd: Break;
      {$IFDEF DEBUG}
      else begin
             Writeln;
             Writeln ('Error: ic Error');
             Halt ($FF);
           end;
      {$ENDIF}
    end;
  until False;
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
Procedure GenerateCode;
Var Block9Record: PStatementCodeSizesBlockRecord;
    ProcedureIdentifierData: PProcedureIdentifierData;
    LocalDebugProcedureIdentifier: Word;
    StatementCodeSize: Word;
    N, Dif: Integer;
    CodePointer: Pointer;
    CodePointerRec: PtrRec absolute CodePointer;
    IntermediateCodePointer: PIntermediateCodeRecord;
    IntermediateCodePointerRec: PtrRec absolute IntermediateCodePointer;
    LabelIdentifier: PIdentifier;
    LabelIdentifierData: PLabelIdentifierData absolute LabelIdentifier;
    LabelIdentifierDataRec: PtrRec absolute LabelIdentifierData;
    ErrorString: PString;
begin
  If DebugInformation in ModuleCompilerSwitches then
    With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 3))^ do
      begin
        RecordType := icSourceLineNumber;
        StatementLineNumber := LastProgramBlockSourceLineNumber + 1;
      end;
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 1))^ do RecordType := icEnd;
  RemoveRedundant_ES_DI_AddressLoading;
  Repeat
    CalculateCodeSizeAndOffsets;
  until not ConvertNearJumpsToShort;
  If DebugInformation in ModuleCompilerSwitches then
    begin
      ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
      Block9Record := IncreaseSymbolTable (stSourceLineCodeOffsets,
                        2 * (LastProgramBlockSourceLineNumber - FirstProgramBlocktSourceLineNumber) + 14);
      If (LocalDebugSymbols in ModuleCompilerSwitches) then
        begin
          If not (pfAssembler in ProcedureIdentifierData^.Flags ) or
            (PushedParametersSize or ProgramBlockMaxStackFrameOffset <> 0) then
              LocalDebugProcedureIdentifier := CurrentProcedureIdentifier else LocalDebugProcedureIdentifier := 0;
        end else LocalDebugProcedureIdentifier := 0;
      With Block9Record^ do
        begin
          ProcedureIdentifier := LocalDebugProcedureIdentifier;
          SourceFileRecordOffset := CurrentSourceFile^.FileRecordOffset;
          ProcedureStartLine := ProcedureStartLineNumber;
          StatementStartCodeOffset := SymbolTable [stCode].UsedSize;
          SizeOfConstantsInCode := SymbolTable [stCode].UsedSize - LastProgramBlockCodeSize;
          FirstSourceLineNumber := FirstProgramBlocktSourceLineNumber;
          NumberOfSourceLines := LastProgramBlockSourceLineNumber - FirstProgramBlocktSourceLineNumber + 1;
          SymbolTable [stSourceLineCodeOffsets].UsedSize := Ofs (StatementsCodeSizeData);
        end;
      StatementFirstSourceLineNumber := FirstProgramBlocktSourceLineNumber;
    end;
  SymbolTable [stCodeReferences].UsedSize := Ofs (IncreaseSymbolTable (stCodeReferences, SizeOfCodeReferenceRecords)^);
  CodePointer := IncreaseSymbolTable (stCode, SizeOfGeneratedCode);
  IntermediateCodePointer := Ptr (SymbolTable [stIntermediateCode].Segment, LastIntermediateCodeSubroutine);
  Repeat
    Case IntermediateCodePointer^.RecordType of
      icGoSub: begin
                 Asm
                   MOV    AX, WORD PTR IntermediateCodePointer
                   ADD    AX, 3
                   PUSH   AX
                 end;
                 IntermediateCodePointerRec.Ofs := IntermediateCodePointer^.NewOffset;
               end;
      icReturn: Asm
                  POP    WORD PTR IntermediateCodePointer
                end;
      icByte: begin
              Byte (CodePointer^) := IntermediateCodePointer^.ByteCode;
              Inc (CodePointerRec.Ofs);
              Inc (IntermediateCodePointerRec.Ofs, 2);
            end;
      icWord: begin
              Word (CodePointer^) := IntermediateCodePointer^.WordCode.Word;
              Inc (CodePointerRec.Ofs, 2);
              Inc (IntermediateCodePointerRec.Ofs, 3);
            end;
      icJumpNear: With IntermediateCodePointer^ do
                    begin
                      If JumpOpCode <> JMP_ShortDirect then
                        begin
                          If JumpOpCode >= JMP_ConditionalShortLimit then Error (CodeGenerationError);
                          Byte (CodePointer^) := JumpOpCode xor $01;          { Invert condition }
                          Inc (CodePointerRec.Ofs);
                          Byte (CodePointer^) := 3;                           { Size of next near jump }
                          Inc (CodePointerRec.Ofs);
                        end;
                      Byte (CodePointer^) := JMP_NearDirect;
                      Inc (CodePointerRec.Ofs);
                      Word (CodePointer^) :=
                        PIntermediateCodeRecord (Ptr (SymbolTable [stIntermediateCode].Segment, LabelRecordOffset - 1))^
                        .LabelAddress - CodePointerRec.Ofs - 2;
                      Inc (CodePointerRec.Ofs, 2);
                      Inc (IntermediateCodePointerRec.Ofs, 8);
                    end;
      icJumpShort: With IntermediateCodePointer^ do
                     begin
                       Byte (CodePointer^) := JumpOpCode;
                       Inc (CodePointerRec.Ofs);
                       Byte (CodePointer^) :=
                         PIntermediateCodeRecord (Ptr (SymbolTable [stIntermediateCode].Segment, LabelRecordOffset - 1))^.
                           LabelAddress - CodePointerRec.Ofs - 1;
                       Inc (CodePointerRec.Ofs);
                       Inc (IntermediateCodePointerRec.Ofs, 8);
                     end;
      icLabel,
      icSkippedGoSubSaveAddress: Inc (IntermediateCodePointerRec.Ofs, 3);
      icReference: With IntermediateCodePointer^ do
                     begin
                       With PReferencesBlockRecord (SymbolTable [stCodeReferences].Ptr)^ do
                         begin
                           ReferencedUnitRecord :=        ReferencedUnitRecordOfs;
                           ReferencedBlockRecordOffset := ReferencedBlockRecordOfs;
                           ReferencedOffset :=            ReferencedOfs;
                           PositionOfReference :=         CodePointerRec.Ofs - LastProgramBlockCodeSize;
                         end;
                       Inc (SymbolTable [stCodeReferences].UsedSize, SizeOf (TReferencesBlockRecord));
                       Word (CodePointer^) := 0;
                       Inc (CodePointerRec.Ofs, 2);
                       If Flags * [rfSegment, rfOffset] = [rfSegment, rfOffset] then
                         begin
                           Word (CodePointer^) := 0;
                           Inc (CodePointerRec.Ofs, 2);
                         end;
                       Inc (IntermediateCodePointerRec.Ofs, 7);
                     end;
      icReferenceAsmBlock: With IntermediateCodePointer^ do
                             begin
                               LabelIdentifier := Ptr (SymbolTable [stMain].Segment, LabelIdentifierOfs);
                               LabelIdentifierDataRec.Seg := SymbolTable [stMain].Segment;
                               ErrorString := @LabelIdentifier^.Name.Str;
                               Inc (LabelIdentifierDataRec.Ofs, LabelIdentifier^.Name.Len + 4);
                               If LabelIdentifierData^.LabelRecordOffset = 0 then
                                 IdentifierError (ErrorString^, UndefinedLabelInPrecedingStatementPart);
                               With PReferencesBlockRecord (SymbolTable [stCodeReferences].Ptr)^ do
                                 begin
                                   ReferencedUnitRecord := ReferencedUnitRecordOfs;
                                   ReferencedBlockRecordOffset := SymbolTable [stCodeBlocks].NextRecordOffset;
                                   ReferencedOffset :=
                                     W5_10 + PIntermediateCodeRecord (Ptr (SymbolTable [stIntermediateCode].Segment,
                                     LabelIdentifierData^.LabelRecordOffset - 1))^.LabelAddress -
                                       LastProgramBlockCodeSize;
                                   PositionOfReference := CodePointerRec.Ofs - LastProgramBlockCodeSize;
                                 end;
                               Inc (SymbolTable [stCodeReferences].UsedSize, SizeOf (TReferencesBlockRecord));
                               Word (CodePointer^) := 0;
                               Inc (CodePointerRec.Ofs, 2);
                               If Flags * [rfSegment, rfOffset] = [rfSegment, rfOffset] then
                                 begin
                                   Word (CodePointer^) := 0;
                                   Inc (CodePointerRec.Ofs, 2);
                                 end;
                               Inc (IntermediateCodePointerRec.Ofs, 7);
                             end;
      ic12,
      icSourceLineNumber: With IntermediateCodePointer^ do
                            begin
                              Dif := StatementLineNumber - StatementFirstSourceLineNumber;
                              If Dif > 0 then
                                begin
                                  StatementFirstSourceLineNumber := StatementLineNumber;          { Next statement }
                                  StatementCodeSize := CodePointerRec.Ofs - StatementStartCodeOffset;
                                  StatementStartCodeOffset := CodePointerRec.Ofs;
                                  If StatementCodeSize >= 128 then
                                    begin
                                      Byte (SymbolTable [stSourceLineCodeOffsets].Ptr^) := Hi (StatementCodeSize) or $80;
                                      Inc (SymbolTable [stSourceLineCodeOffsets].UsedSize);
                                    end;
                                  Byte (SymbolTable [stSourceLineCodeOffsets].Ptr^) := Lo (StatementCodeSize);
                                  Inc (SymbolTable [stSourceLineCodeOffsets].UsedSize);
                                  For N := 1 to Dif - 1 do
                                    begin
                                      Byte (SymbolTable [stSourceLineCodeOffsets].Ptr^) := 0;
                                      Inc (SymbolTable [stSourceLineCodeOffsets].UsedSize);
                                    end;
                                end;
                              Inc (IntermediateCodePointerRec.Ofs, 3);
                            end;
      icLoad_ES_DI: begin
                       With IntermediateCodePointer^ do
                         GenerateCode_LES_DI_WithVariableAddress (Displacement, DestinationProcedure);
                       Inc (IntermediateCodePointerRec.Ofs, 7);
                     end;
      ic_ES_DI_Loaded: Inc (IntermediateCodePointerRec.Ofs, 7);
      ic_ES_DI_Destroyed: Inc (IntermediateCodePointerRec.Ofs);
      icEnd: Break;
      {$IFDEF DEBUG}
      else begin
             Writeln;
             Writeln ('Error: ic Error');
             Halt ($FF);
           end;
      {$ENDIF}
    end;
  until False;
end;
```

### Code Block 2

```pascal
Procedure GenerateCode;
Var Block9Record: PStatementCodeSizesBlockRecord;
    ProcedureIdentifierData: PProcedureIdentifierData;
    LocalDebugProcedureIdentifier: Word;
    StatementCodeSize: Word;
    N, Dif: Integer;
    CodePointer: Pointer;
    CodePointerRec: PtrRec absolute CodePointer;
    IntermediateCodePointer: PIntermediateCodeRecord;
    IntermediateCodePointerRec: PtrRec absolute IntermediateCodePointer;
    LabelIdentifier: PIdentifier;
    LabelIdentifierData: PLabelIdentifierData absolute LabelIdentifier;
    LabelIdentifierDataRec: PtrRec absolute LabelIdentifierData;
    ErrorString: PString;
begin
  If DebugInformation in ModuleCompilerSwitches then
    With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 3))^ do
      begin
        RecordType := icSourceLineNumber;
        StatementLineNumber := LastProgramBlockSourceLineNumber + 1;
      end;
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 1))^ do RecordType := icEnd;
  RemoveRedundant_ES_DI_AddressLoading;
  Repeat
    CalculateCodeSizeAndOffsets;
  until not ConvertNearJumpsToShort;
  If DebugInformation in ModuleCompilerSwitches then
    begin
      ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
      Block9Record := IncreaseSymbolTable (stSourceLineCodeOffsets,
                        2 * (LastProgramBlockSourceLineNumber - FirstProgramBlocktSourceLineNumber) + 14);
      If (LocalDebugSymbols in ModuleCompilerSwitches) then
        begin
          If not (pfAssembler in ProcedureIdentifierData^.Flags ) or
            (PushedParametersSize or ProgramBlockMaxStackFrameOffset <> 0) then
              LocalDebugProcedureIdentifier := CurrentProcedureIdentifier else LocalDebugProcedureIdentifier := 0;
        end else LocalDebugProcedureIdentifier := 0;
      With Block9Record^ do
        begin
          ProcedureIdentifier := LocalDebugProcedureIdentifier;
          SourceFileRecordOffset := CurrentSourceFile^.FileRecordOffset;
          ProcedureStartLine := ProcedureStartLineNumber;
          StatementStartCodeOffset := SymbolTable [stCode].UsedSize;
          SizeOfConstantsInCode := SymbolTable [stCode].UsedSize - LastProgramBlockCodeSize;
          FirstSourceLineNumber := FirstProgramBlocktSourceLineNumber;
          NumberOfSourceLines := LastProgramBlockSourceLineNumber - FirstProgramBlocktSourceLineNumber + 1;
          SymbolTable [stSourceLineCodeOffsets].UsedSize := Ofs (StatementsCodeSizeData);
        end;
      StatementFirstSourceLineNumber := FirstProgramBlocktSourceLineNumber;
    end;
  SymbolTable [stCodeReferences].UsedSize := Ofs (IncreaseSymbolTable (stCodeReferences, SizeOfCodeReferenceRecords)^);
  CodePointer := IncreaseSymbolTable (stCode, SizeOfGeneratedCode);
  IntermediateCodePointer := Ptr (SymbolTable [stIntermediateCode].Segment, LastIntermediateCodeSubroutine);
  Repeat
    Case IntermediateCodePointer^.RecordType of
      icGoSub: begin
                 Asm
                   MOV    AX, WORD PTR IntermediateCodePointer
                   ADD    AX, 3
                   PUSH   AX
                 end;
                 IntermediateCodePointerRec.Ofs := IntermediateCodePointer^.NewOffset;
               end;
      icReturn: Asm
                  POP    WORD PTR IntermediateCodePointer
                end;
      icByte: begin
              Byte (CodePointer^) := IntermediateCodePointer^.ByteCode;
              Inc (CodePointerRec.Ofs);
              Inc (IntermediateCodePointerRec.Ofs, 2);
            end;
      icWord: begin
              Word (CodePointer^) := IntermediateCodePointer^.WordCode.Word;
              Inc (CodePointerRec.Ofs, 2);
              Inc (IntermediateCodePointerRec.Ofs, 3);
            end;
      icJumpNear: With IntermediateCodePointer^ do
                    begin
                      If JumpOpCode <> JMP_ShortDirect then
                        begin
                          If JumpOpCode >= JMP_ConditionalShortLimit then Error (CodeGenerationError);
                          Byte (CodePointer^) := JumpOpCode xor $01;          { Invert condition }
                          Inc (CodePointerRec.Ofs);
                          Byte (CodePointer^) := 3;                           { Size of next near jump }
                          Inc (CodePointerRec.Ofs);
                        end;
                      Byte (CodePointer^) := JMP_NearDirect;
                      Inc (CodePointerRec.Ofs);
                      Word (CodePointer^) :=
                        PIntermediateCodeRecord (Ptr (SymbolTable [stIntermediateCode].Segment, LabelRecordOffset - 1))^
                        .LabelAddress - CodePointerRec.Ofs - 2;
                      Inc (CodePointerRec.Ofs, 2);
                      Inc (IntermediateCodePointerRec.Ofs, 8);
                    end;
      icJumpShort: With IntermediateCodePointer^ do
                     begin
                       Byte (CodePointer^) := JumpOpCode;
                       Inc (CodePointerRec.Ofs);
                       Byte (CodePointer^) :=
                         PIntermediateCodeRecord (Ptr (SymbolTable [stIntermediateCode].Segment, LabelRecordOffset - 1))^.
                           LabelAddress - CodePointerRec.Ofs - 1;
                       Inc (CodePointerRec.Ofs);
                       Inc (IntermediateCodePointerRec.Ofs, 8);
                     end;
      icLabel,
      icSkippedGoSubSaveAddress: Inc (IntermediateCodePointerRec.Ofs, 3);
      icReference: With IntermediateCodePointer^ do
                     begin
                       With PReferencesBlockRecord (SymbolTable [stCodeReferences].Ptr)^ do
                         begin
                           ReferencedUnitRecord :=        ReferencedUnitRecordOfs;
                           ReferencedBlockRecordOffset := ReferencedBlockRecordOfs;
                           ReferencedOffset :=            ReferencedOfs;
                           PositionOfReference :=         CodePointerRec.Ofs - LastProgramBlockCodeSize;
                         end;
                       Inc (SymbolTable [stCodeReferences].UsedSize, SizeOf (TReferencesBlockRecord));
                       Word (CodePointer^) := 0;
                       Inc (CodePointerRec.Ofs, 2);
                       If Flags * [rfSegment, rfOffset] = [rfSegment, rfOffset] then
                         begin
                           Word (CodePointer^) := 0;
                           Inc (CodePointerRec.Ofs, 2);
                         end;
                       Inc (IntermediateCodePointerRec.Ofs, 7);
                     end;
      icReferenceAsmBlock: With IntermediateCodePointer^ do
                             begin
                               LabelIdentifier := Ptr (SymbolTable [stMain].Segment, LabelIdentifierOfs);
                               LabelIdentifierDataRec.Seg := SymbolTable [stMain].Segment;
                               ErrorString := @LabelIdentifier^.Name.Str;
                               Inc (LabelIdentifierDataRec.Ofs, LabelIdentifier^.Name.Len + 4);
                               If LabelIdentifierData^.LabelRecordOffset = 0 then
                                 IdentifierError (ErrorString^, UndefinedLabelInPrecedingStatementPart);
                               With PReferencesBlockRecord (SymbolTable [stCodeReferences].Ptr)^ do
                                 begin
                                   ReferencedUnitRecord := ReferencedUnitRecordOfs;
                                   ReferencedBlockRecordOffset := SymbolTable [stCodeBlocks].NextRecordOffset;
                                   ReferencedOffset :=
                                     W5_10 + PIntermediateCodeRecord (Ptr (SymbolTable [stIntermediateCode].Segment,
                                     LabelIdentifierData^.LabelRecordOffset - 1))^.LabelAddress -
                                       LastProgramBlockCodeSize;
                                   PositionOfReference := CodePointerRec.Ofs - LastProgramBlockCodeSize;
                                 end;
                               Inc (SymbolTable [stCodeReferences].UsedSize, SizeOf (TReferencesBlockRecord));
                               Word (CodePointer^) := 0;
                               Inc (CodePointerRec.Ofs, 2);
                               If Flags * [rfSegment, rfOffset] = [rfSegment, rfOffset] then
                                 begin
                                   Word (CodePointer^) := 0;
                                   Inc (CodePointerRec.Ofs, 2);
                                 end;
                               Inc (IntermediateCodePointerRec.Ofs, 7);
                             end;
      ic12,
      icSourceLineNumber: With IntermediateCodePointer^ do
                            begin
                              Dif := StatementLineNumber - StatementFirstSourceLineNumber;
                              If Dif > 0 then
                                begin
                                  StatementFirstSourceLineNumber := StatementLineNumber;          { Next statement }
                                  StatementCodeSize := CodePointerRec.Ofs - StatementStartCodeOffset;
                                  StatementStartCodeOffset := CodePointerRec.Ofs;
                                  If StatementCodeSize >= 128 then
                                    begin
                                      Byte (SymbolTable [stSourceLineCodeOffsets].Ptr^) := Hi (StatementCodeSize) or $80;
                                      Inc (SymbolTable [stSourceLineCodeOffsets].UsedSize);
                                    end;
                                  Byte (SymbolTable [stSourceLineCodeOffsets].Ptr^) := Lo (StatementCodeSize);
                                  Inc (SymbolTable [stSourceLineCodeOffsets].UsedSize);
                                  For N := 1 to Dif - 1 do
                                    begin
                                      Byte (SymbolTable [stSourceLineCodeOffsets].Ptr^) := 0;
                                      Inc (SymbolTable [stSourceLineCodeOffsets].UsedSize);
                                    end;
                                end;
                              Inc (IntermediateCodePointerRec.Ofs, 3);
                            end;
      icLoad_ES_DI: begin
                       With IntermediateCodePointer^ do
                         GenerateCode_LES_DI_WithVariableAddress (Displacement, DestinationProcedure);
                       Inc (IntermediateCodePointerRec.Ofs, 7);
                     end;
      ic_ES_DI_Loaded: Inc (IntermediateCodePointerRec.Ofs, 7);
      ic_ES_DI_Destroyed: Inc (IntermediateCodePointerRec.Ofs);
      icEnd: Break;
      {$IFDEF DEBUG}
      else begin
             Writeln;
             Writeln ('Error: ic Error');
             Halt ($FF);
           end;
      {$ENDIF}
    end;
  until False;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculating-code-size-and-offsets
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/es-di-address-loading
- https://turbopascal.org/expressions
- https://turbopascal.org/generating-code
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/optimizing-jumps
- https://turbopascal.org/parser
- https://turbopascal.org/processing-program-block
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

