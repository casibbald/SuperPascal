# Processing Program Block - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-program-block

---

## Table of Contents

- Processing Program Block

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Program Block
This is where the main compilation routines are called. This procedure resets many compiler flags and variables, initializes
intermediate code
, saves source line number, resets compiler switches, and
processes statements between
begin
and
end
tokes (or
asm
and
end
for assembler procedures). After the compilation,
module entry and exit code
is added. Of course, for interrupts
different code
is generated. Finally, the target code is generated. Because of the possibility to use
subroutines in intermediate code
, there is no need to generate linear code as it needs to be at the end. Turbo Compiler
generates code
by cleverly joining smaller pieces into one big code.
Code generator
takes care to generate code in the right order.
Function ProcessProgramBlock: Word;
Var SavedMainSymbolTableUsedSize: Word;
    SavedModuleCompilerSwitches: TCompilerSwitchesSet;
    BlockEntryCode, BlockExitCode, ProgramBlockStartLineNumber: Word;
    ProgramBlockCode: TStatement;
    ProgramBlockEndLineNumber, ResultUnitIdData: Word;
    ProcedureIdentifierData: PProcedureIdentifierData;
    ProcedureIdentifierDataRec: PtrRec absolute ProcedureIdentifierData;
    ResultTypeDef: PTypeDefinition;
    ResultTypeDefRec: PtrRec absolute ResultTypeDef;

begin
  SavedMainSymbolTableUsedSize := SymbolTable [stMain].UsedSize;
  ProgramBlockCompilation := True;
  LastIntermediateCodeSubroutine := 2;
  IncreaseSymbolTable (stIntermediateCode, 2);
  StatementMaxStackFrameOffset := ProgramBlockMaxStackFrameOffset;
  NumberOfLocalParameters := 0;
  LastJumpToProgramBlockExit := 0;
  LastJumpToFail := 0;
  Last_GOTO_Record := 0;
  LastJumpOutOfBlock := $FFFF;
  LastJumpToNextBlockIteration := $FFFF;
  With CurrentSourceFile^ do
    begin
      FirstProgramBlocktSourceLineNumber := CurrentLineNumber;
      LastProgramBlockSourceLineNumber := CurrentLineNumber;
    end;
  ProgramBlockStartLineNumber := GetCurrentProgramBlockSourceLineNumber;
  SavedModuleCompilerSwitches := ModuleCompilerSwitches;
  ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
  If (ProcedureIdentifierDataOffset <> 0) and (pfAssembler in ProcedureIdentifierData^.Flags) then
    ProgramBlockCode.Process_ASM_END_Block else ProgramBlockCode.ProcessStatementsBetweenTokens (Token_BEGIN, Token_END);
  StatementCompilerSwitches := SavedModuleCompilerSwitches;
  ProgramBlockEndLineNumber := GetCurrentProgramBlockSourceLineNumber;
  PlaceLabelRecordsTo_GOTO_Records;
  ProgramBlockMaxStackFrameOffset := ProgramBlockMaxStackFrameOffset and $FFFE;
  If ProcedureIdentifierDataOffset = 0 then
    begin
      BlockEntryCode := GenerateModuleEntryCode;
      BlockExitCode := GenerateModuleExitCode;
    end else
      begin
        ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
        If pfInterrupt in ProcedureIdentifierData^.Flags then
          begin
            Exclude (StatementCompilerSwitches, StackChecking);
            BlockEntryCode := GenerateInterruptEntryCode;
            BlockExitCode := GenerateInterruptExitCode;
          end else
            begin
              If (pfAssembler in ProcedureIdentifierData^.Flags) and
                 (PushedParametersSize or ProgramBlockMaxStackFrameOffset = 0) then
                begin
                  ResultUnitIdData := ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset.UnitIdentifierData;
                  If ResultUnitIdData <> 0 then
                    begin
                      ResultTypeDefRec.Seg :=
                        PUnitIdentifierData (Ptr (Seg (ProcedureIdentifierData^), ResultUnitIdData))^.UnitSegment;
                      ResultTypeDefRec.Ofs := ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset.TypeOffset;
                      If ResultTypeDef^.BaseType = btString then Include (ProcedureIdentifierData^.Flags, pfStackFrame);
                    end;
                end else Include (ProcedureIdentifierData^.Flags, pfStackFrame);
              BlockEntryCode := GenerateProcedureEntryCode;
              BlockExitCode := GenerateProcedureExitCode;
            end;
      end;
  MarkSourceLineNumber (ProgramBlockStartLineNumber);
  StoreCode_icGoSub (BlockEntryCode);
  StoreCode_icGoSub (ProgramBlockCode.StatementCode);
  If LastJumpToFail <> 0 then
    begin
      GenerateCodeForNearJump (LastJumpToProgramBlockExit, JMP_ShortDirect);
      GenerateLabelAndSetJumpsToIt (LastJumpToFail);
      ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
      ProcedureIdentifierDataRec.Ofs := ProcedureIdentifierData^.OuterBlockProcedureIdentifier;
      GenerateInstruction_MOV_16BitReg_Immediate (rDI,
        ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset.UnitIdentifierData);
      GenerateInstruction_CALL_FAR (SysProc_Destruct);
    end;
  GenerateLabelAndSetJumpsToIt (LastJumpToProgramBlockExit);
  MarkSourceLineNumber (ProgramBlockEndLineNumber);
  StoreCode_icGoSub (BlockExitCode);
  ProcessProgramBlock := SymbolTable [stCode].UsedSize - LastProgramBlockCodeSize;
  GenerateCode;
  SymbolTable [stIntermediateCode].UsedSize := 0;
  LastIntermediateCodeSubroutine := 0;
  ProgramBlockCompilation := False;
  SymbolTable [stMain].UsedSize := SavedMainSymbolTableUsedSize;
end;
This procedure takes care for the main program block.
Procedure ProcessMainProgramBlock;
Var MainProgramBlockSizeOfConstants: Word;
begin
  ProgramBlockMaxStackFrameOffset := 0;
  ProcedureStartLineNumber := 1;
  MainProgramBlockSizeOfConstants := ProcessProgramBlock;

{ SymbolTable [stProcedures].Segment might change in ProcessProgramBlock }

  With PProceduresBlockRecord (Ptr (SymbolTable [stProcedures].Segment, 0))^ do
    begin
      SizeOfConstants              := MainProgramBlockSizeOfConstants;
      ProgramCodeBlockRecordOffset := SymbolTable [stCodeBlocks].NextRecordOffset;
    end;
  CreateProgramCodeBlockRecord;
  CreateTypedConstantsBlockRecord;
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
Function ProcessProgramBlock: Word;
Var SavedMainSymbolTableUsedSize: Word;
    SavedModuleCompilerSwitches: TCompilerSwitchesSet;
    BlockEntryCode, BlockExitCode, ProgramBlockStartLineNumber: Word;
    ProgramBlockCode: TStatement;
    ProgramBlockEndLineNumber, ResultUnitIdData: Word;
    ProcedureIdentifierData: PProcedureIdentifierData;
    ProcedureIdentifierDataRec: PtrRec absolute ProcedureIdentifierData;
    ResultTypeDef: PTypeDefinition;
    ResultTypeDefRec: PtrRec absolute ResultTypeDef;

begin
  SavedMainSymbolTableUsedSize := SymbolTable [stMain].UsedSize;
  ProgramBlockCompilation := True;
  LastIntermediateCodeSubroutine := 2;
  IncreaseSymbolTable (stIntermediateCode, 2);
  StatementMaxStackFrameOffset := ProgramBlockMaxStackFrameOffset;
  NumberOfLocalParameters := 0;
  LastJumpToProgramBlockExit := 0;
  LastJumpToFail := 0;
  Last_GOTO_Record := 0;
  LastJumpOutOfBlock := $FFFF;
  LastJumpToNextBlockIteration := $FFFF;
  With CurrentSourceFile^ do
    begin
      FirstProgramBlocktSourceLineNumber := CurrentLineNumber;
      LastProgramBlockSourceLineNumber := CurrentLineNumber;
    end;
  ProgramBlockStartLineNumber := GetCurrentProgramBlockSourceLineNumber;
  SavedModuleCompilerSwitches := ModuleCompilerSwitches;
  ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
  If (ProcedureIdentifierDataOffset <> 0) and (pfAssembler in ProcedureIdentifierData^.Flags) then
    ProgramBlockCode.Process_ASM_END_Block else ProgramBlockCode.ProcessStatementsBetweenTokens (Token_BEGIN, Token_END);
  StatementCompilerSwitches := SavedModuleCompilerSwitches;
  ProgramBlockEndLineNumber := GetCurrentProgramBlockSourceLineNumber;
  PlaceLabelRecordsTo_GOTO_Records;
  ProgramBlockMaxStackFrameOffset := ProgramBlockMaxStackFrameOffset and $FFFE;
  If ProcedureIdentifierDataOffset = 0 then
    begin
      BlockEntryCode := GenerateModuleEntryCode;
      BlockExitCode := GenerateModuleExitCode;
    end else
      begin
        ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
        If pfInterrupt in ProcedureIdentifierData^.Flags then
          begin
            Exclude (StatementCompilerSwitches, StackChecking);
            BlockEntryCode := GenerateInterruptEntryCode;
            BlockExitCode := GenerateInterruptExitCode;
          end else
            begin
              If (pfAssembler in ProcedureIdentifierData^.Flags) and
                 (PushedParametersSize or ProgramBlockMaxStackFrameOffset = 0) then
                begin
                  ResultUnitIdData := ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset.UnitIdentifierData;
                  If ResultUnitIdData <> 0 then
                    begin
                      ResultTypeDefRec.Seg :=
                        PUnitIdentifierData (Ptr (Seg (ProcedureIdentifierData^), ResultUnitIdData))^.UnitSegment;
                      ResultTypeDefRec.Ofs := ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset.TypeOffset;
                      If ResultTypeDef^.BaseType = btString then Include (ProcedureIdentifierData^.Flags, pfStackFrame);
                    end;
                end else Include (ProcedureIdentifierData^.Flags, pfStackFrame);
              BlockEntryCode := GenerateProcedureEntryCode;
              BlockExitCode := GenerateProcedureExitCode;
            end;
      end;
  MarkSourceLineNumber (ProgramBlockStartLineNumber);
  StoreCode_icGoSub (BlockEntryCode);
  StoreCode_icGoSub (ProgramBlockCode.StatementCode);
  If LastJumpToFail <> 0 then
    begin
      GenerateCodeForNearJump (LastJumpToProgramBlockExit, JMP_ShortDirect);
      GenerateLabelAndSetJumpsToIt (LastJumpToFail);
      ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
      ProcedureIdentifierDataRec.Ofs := ProcedureIdentifierData^.OuterBlockProcedureIdentifier;
      GenerateInstruction_MOV_16BitReg_Immediate (rDI,
        ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset.UnitIdentifierData);
      GenerateInstruction_CALL_FAR (SysProc_Destruct);
    end;
  GenerateLabelAndSetJumpsToIt (LastJumpToProgramBlockExit);
  MarkSourceLineNumber (ProgramBlockEndLineNumber);
  StoreCode_icGoSub (BlockExitCode);
  ProcessProgramBlock := SymbolTable [stCode].UsedSize - LastProgramBlockCodeSize;
  GenerateCode;
  SymbolTable [stIntermediateCode].UsedSize := 0;
  LastIntermediateCodeSubroutine := 0;
  ProgramBlockCompilation := False;
  SymbolTable [stMain].UsedSize := SavedMainSymbolTableUsedSize;
end;
```

### Code Block 2

```pascal
Procedure ProcessMainProgramBlock;
Var MainProgramBlockSizeOfConstants: Word;
begin
  ProgramBlockMaxStackFrameOffset := 0;
  ProcedureStartLineNumber := 1;
  MainProgramBlockSizeOfConstants := ProcessProgramBlock;

{ SymbolTable [stProcedures].Segment might change in ProcessProgramBlock }

  With PProceduresBlockRecord (Ptr (SymbolTable [stProcedures].Segment, 0))^ do
    begin
      SizeOfConstants              := MainProgramBlockSizeOfConstants;
      ProgramCodeBlockRecordOffset := SymbolTable [stCodeBlocks].NextRecordOffset;
    end;
  CreateProgramCodeBlockRecord;
  CreateTypedConstantsBlockRecord;
end;
```

### Code Block 3

```pascal
Function ProcessProgramBlock: Word;
Var SavedMainSymbolTableUsedSize: Word;
    SavedModuleCompilerSwitches: TCompilerSwitchesSet;
    BlockEntryCode, BlockExitCode, ProgramBlockStartLineNumber: Word;
    ProgramBlockCode: TStatement;
    ProgramBlockEndLineNumber, ResultUnitIdData: Word;
    ProcedureIdentifierData: PProcedureIdentifierData;
    ProcedureIdentifierDataRec: PtrRec absolute ProcedureIdentifierData;
    ResultTypeDef: PTypeDefinition;
    ResultTypeDefRec: PtrRec absolute ResultTypeDef;

begin
  SavedMainSymbolTableUsedSize := SymbolTable [stMain].UsedSize;
  ProgramBlockCompilation := True;
  LastIntermediateCodeSubroutine := 2;
  IncreaseSymbolTable (stIntermediateCode, 2);
  StatementMaxStackFrameOffset := ProgramBlockMaxStackFrameOffset;
  NumberOfLocalParameters := 0;
  LastJumpToProgramBlockExit := 0;
  LastJumpToFail := 0;
  Last_GOTO_Record := 0;
  LastJumpOutOfBlock := $FFFF;
  LastJumpToNextBlockIteration := $FFFF;
  With CurrentSourceFile^ do
    begin
      FirstProgramBlocktSourceLineNumber := CurrentLineNumber;
      LastProgramBlockSourceLineNumber := CurrentLineNumber;
    end;
  ProgramBlockStartLineNumber := GetCurrentProgramBlockSourceLineNumber;
  SavedModuleCompilerSwitches := ModuleCompilerSwitches;
  ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
  If (ProcedureIdentifierDataOffset <> 0) and (pfAssembler in ProcedureIdentifierData^.Flags) then
    ProgramBlockCode.Process_ASM_END_Block else ProgramBlockCode.ProcessStatementsBetweenTokens (Token_BEGIN, Token_END);
  StatementCompilerSwitches := SavedModuleCompilerSwitches;
  ProgramBlockEndLineNumber := GetCurrentProgramBlockSourceLineNumber;
  PlaceLabelRecordsTo_GOTO_Records;
  ProgramBlockMaxStackFrameOffset := ProgramBlockMaxStackFrameOffset and $FFFE;
  If ProcedureIdentifierDataOffset = 0 then
    begin
      BlockEntryCode := GenerateModuleEntryCode;
      BlockExitCode := GenerateModuleExitCode;
    end else
      begin
        ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
        If pfInterrupt in ProcedureIdentifierData^.Flags then
          begin
            Exclude (StatementCompilerSwitches, StackChecking);
            BlockEntryCode := GenerateInterruptEntryCode;
            BlockExitCode := GenerateInterruptExitCode;
          end else
            begin
              If (pfAssembler in ProcedureIdentifierData^.Flags) and
                 (PushedParametersSize or ProgramBlockMaxStackFrameOffset = 0) then
                begin
                  ResultUnitIdData := ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset.UnitIdentifierData;
                  If ResultUnitIdData <> 0 then
                    begin
                      ResultTypeDefRec.Seg :=
                        PUnitIdentifierData (Ptr (Seg (ProcedureIdentifierData^), ResultUnitIdData))^.UnitSegment;
                      ResultTypeDefRec.Ofs := ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset.TypeOffset;
                      If ResultTypeDef^.BaseType = btString then Include (ProcedureIdentifierData^.Flags, pfStackFrame);
                    end;
                end else Include (ProcedureIdentifierData^.Flags, pfStackFrame);
              BlockEntryCode := GenerateProcedureEntryCode;
              BlockExitCode := GenerateProcedureExitCode;
            end;
      end;
  MarkSourceLineNumber (ProgramBlockStartLineNumber);
  StoreCode_icGoSub (BlockEntryCode);
  StoreCode_icGoSub (ProgramBlockCode.StatementCode);
  If LastJumpToFail <> 0 then
    begin
      GenerateCodeForNearJump (LastJumpToProgramBlockExit, JMP_ShortDirect);
      GenerateLabelAndSetJumpsToIt (LastJumpToFail);
      ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
      ProcedureIdentifierDataRec.Ofs := ProcedureIdentifierData^.OuterBlockProcedureIdentifier;
      GenerateInstruction_MOV_16BitReg_Immediate (rDI,
        ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset.UnitIdentifierData);
      GenerateInstruction_CALL_FAR (SysProc_Destruct);
    end;
  GenerateLabelAndSetJumpsToIt (LastJumpToProgramBlockExit);
  MarkSourceLineNumber (ProgramBlockEndLineNumber);
  StoreCode_icGoSub (BlockExitCode);
  ProcessProgramBlock := SymbolTable [stCode].UsedSize - LastProgramBlockCodeSize;
  GenerateCode;
  SymbolTable [stIntermediateCode].UsedSize := 0;
  LastIntermediateCodeSubroutine := 0;
  ProgramBlockCompilation := False;
  SymbolTable [stMain].UsedSize := SavedMainSymbolTableUsedSize;
end;
```

### Code Block 4

```pascal
Procedure ProcessMainProgramBlock;
Var MainProgramBlockSizeOfConstants: Word;
begin
  ProgramBlockMaxStackFrameOffset := 0;
  ProcedureStartLineNumber := 1;
  MainProgramBlockSizeOfConstants := ProcessProgramBlock;

{ SymbolTable [stProcedures].Segment might change in ProcessProgramBlock }

  With PProceduresBlockRecord (Ptr (SymbolTable [stProcedures].Segment, 0))^ do
    begin
      SizeOfConstants              := MainProgramBlockSizeOfConstants;
      ProgramCodeBlockRecordOffset := SymbolTable [stCodeBlocks].NextRecordOffset;
    end;
  CreateProgramCodeBlockRecord;
  CreateTypedConstantsBlockRecord;
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
- https://turbopascal.org/intermediate-code
- https://turbopascal.org/interrupt-entry-and-exit-code
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/module-entry-and-exit-code
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-program-block
- https://turbopascal.org/processing-statements
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

