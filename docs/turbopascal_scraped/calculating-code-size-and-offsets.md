# Calculating Code Size and Offsets - Turbo Pascal Compiler

**URL:** https://turbopascal.org/calculating-code-size-and-offsets

---

## Table of Contents

- Calculating Code Size and Offsets

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Calculating Code Size and Offsets
Before the target code is generated Turbo Pascal needs to know code size and offsets of instructions. This procedure processes
intermediate code instructions
and calculates the size of generated code, offsets of near jump instructions, offsets of labels and size of reference records.
Procedure CalculateCodeSizeAndOffsets;
  Var IntermediateCodePointer: PIntermediateCodeRecord;
      IntermediateCodePointerRec: PtrRec absolute IntermediateCodePointer;
      CodeOffset, AdditionalReferencesSize, LastNearJumpRecord: Word;
      OpCode: Byte;

    Procedure CalculateLoadAddressCodeSize (DestinationDisplacement: Integer; DestinationProc: Word);
    Var ProcedureIdentifier: PIdentifier;
        ProcedureIdentifierRec: PtrRec absolute ProcedureIdentifier;
        ProcedureIdentifierData: PProcedureIdentifierData absolute ProcedureIdentifier;
        ProcedureIdentifierDataRec: PtrRec absolute ProcedureIdentifier;

    begin
      If DestinationProc <> CurrentProcedureIdentifier then
        begin
          ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
          Inc (CodeOffset, 3);
          Repeat
            ProcedureIdentifierRec.Ofs := ProcedureIdentifierData^.OuterBlockProcedureIdentifier;
            If ProcedureIdentifierRec.Ofs = DestinationProc then Break;
            Inc (ProcedureIdentifierRec.Ofs, ProcedureIdentifier^.Name.Len + 4);
            Inc (CodeOffset, 4);
          until False;
          If DestinationDisplacement = 0 then Exit;
          Inc (CodeOffset);
        end;
      Inc (CodeOffset, 3);
      If DestinationDisplacement <> ShortInt (DestinationDisplacement) then Inc (CodeOffset);
    end;

  begin
    AdditionalReferencesSize := 0;
    LastNearJumpRecord := 0;
    IntermediateCodePointer := Ptr (SymbolTable [stIntermediateCode].Segment, LastIntermediateCodeSubroutine);
    CodeOffset := SymbolTable [stCode].UsedSize;
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
                  Inc (CodeOffset);
                  Inc (IntermediateCodePointerRec.Ofs, 2);
                end;
        icWord: begin
                  Inc (CodeOffset, 2);
                  Inc (IntermediateCodePointerRec.Ofs, 3);
                end;
        icJumpNear: With IntermediateCodePointer^ do
                      begin
                        JumpCodeOffset := CodeOffset;
                        OpCode := JumpOpCode;
                        Previous_icJumpNear := LastNearJumpRecord;
                        LastNearJumpRecord := IntermediateCodePointerRec.Ofs;
                        Inc (IntermediateCodePointerRec.Ofs, 8);
                        Inc (CodeOffset, 3);                                            { JMP_Near size }
                        If OpCode < JMP_ConditionalShortLimit then Inc (CodeOffset, 2); { In case conditional jump will }
                      end;                                                              { become two jumps, short and near }
        icJumpShort: begin
                       Inc (IntermediateCodePointerRec.Ofs, 8);
                       Inc (CodeOffset, 2);
                     end;
        icLabel: begin
                   IntermediateCodePointer^.LabelAddress := CodeOffset;
                   Inc (IntermediateCodePointerRec.Ofs, 3);
                 end;
        icReference,
        icReferenceAsmBlock: begin
                               Inc (AdditionalReferencesSize, 8);
                               Inc (CodeOffset, 2);
                               If IntermediateCodePointer^.Flags * [rfSegment, rfOffset] = [rfSegment, rfOffset] then
                                 Inc (CodeOffset, 2);
                               Inc (IntermediateCodePointerRec.Ofs, 7);
                             end;
        ic12,
        icSourceLineNumber,
        icSkippedGoSubSaveAddress: Inc (IntermediateCodePointerRec.Ofs, 3);
        icLoad_ES_DI: With IntermediateCodePointer^ do
                         begin
                           CalculateLoadAddressCodeSize (Displacement, DestinationProcedure);
                           Inc (IntermediateCodePointerRec.Ofs, 7);
                         end;
        ic_ES_DI_Loaded: Inc (IntermediateCodePointerRec.Ofs, 7);
        ic_ES_DI_Destroyed: Inc (IntermediateCodePointerRec.Ofs);
        icEnd: Break;
      end;
    until False;
    SizeOfCodeReferenceRecords := AdditionalReferencesSize;
    Last_icJumpNear := LastNearJumpRecord;
    SizeOfGeneratedCode := CodeOffset - SymbolTable [stCode].UsedSize;
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
  Procedure CalculateCodeSizeAndOffsets;
  Var IntermediateCodePointer: PIntermediateCodeRecord;
      IntermediateCodePointerRec: PtrRec absolute IntermediateCodePointer;
      CodeOffset, AdditionalReferencesSize, LastNearJumpRecord: Word;
      OpCode: Byte;

    Procedure CalculateLoadAddressCodeSize (DestinationDisplacement: Integer; DestinationProc: Word);
    Var ProcedureIdentifier: PIdentifier;
        ProcedureIdentifierRec: PtrRec absolute ProcedureIdentifier;
        ProcedureIdentifierData: PProcedureIdentifierData absolute ProcedureIdentifier;
        ProcedureIdentifierDataRec: PtrRec absolute ProcedureIdentifier;

    begin
      If DestinationProc <> CurrentProcedureIdentifier then
        begin
          ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
          Inc (CodeOffset, 3);
          Repeat
            ProcedureIdentifierRec.Ofs := ProcedureIdentifierData^.OuterBlockProcedureIdentifier;
            If ProcedureIdentifierRec.Ofs = DestinationProc then Break;
            Inc (ProcedureIdentifierRec.Ofs, ProcedureIdentifier^.Name.Len + 4);
            Inc (CodeOffset, 4);
          until False;
          If DestinationDisplacement = 0 then Exit;
          Inc (CodeOffset);
        end;
      Inc (CodeOffset, 3);
      If DestinationDisplacement <> ShortInt (DestinationDisplacement) then Inc (CodeOffset);
    end;

  begin
    AdditionalReferencesSize := 0;
    LastNearJumpRecord := 0;
    IntermediateCodePointer := Ptr (SymbolTable [stIntermediateCode].Segment, LastIntermediateCodeSubroutine);
    CodeOffset := SymbolTable [stCode].UsedSize;
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
                  Inc (CodeOffset);
                  Inc (IntermediateCodePointerRec.Ofs, 2);
                end;
        icWord: begin
                  Inc (CodeOffset, 2);
                  Inc (IntermediateCodePointerRec.Ofs, 3);
                end;
        icJumpNear: With IntermediateCodePointer^ do
                      begin
                        JumpCodeOffset := CodeOffset;
                        OpCode := JumpOpCode;
                        Previous_icJumpNear := LastNearJumpRecord;
                        LastNearJumpRecord := IntermediateCodePointerRec.Ofs;
                        Inc (IntermediateCodePointerRec.Ofs, 8);
                        Inc (CodeOffset, 3);                                            { JMP_Near size }
                        If OpCode < JMP_ConditionalShortLimit then Inc (CodeOffset, 2); { In case conditional jump will }
                      end;                                                              { become two jumps, short and near }
        icJumpShort: begin
                       Inc (IntermediateCodePointerRec.Ofs, 8);
                       Inc (CodeOffset, 2);
                     end;
        icLabel: begin
                   IntermediateCodePointer^.LabelAddress := CodeOffset;
                   Inc (IntermediateCodePointerRec.Ofs, 3);
                 end;
        icReference,
        icReferenceAsmBlock: begin
                               Inc (AdditionalReferencesSize, 8);
                               Inc (CodeOffset, 2);
                               If IntermediateCodePointer^.Flags * [rfSegment, rfOffset] = [rfSegment, rfOffset] then
                                 Inc (CodeOffset, 2);
                               Inc (IntermediateCodePointerRec.Ofs, 7);
                             end;
        ic12,
        icSourceLineNumber,
        icSkippedGoSubSaveAddress: Inc (IntermediateCodePointerRec.Ofs, 3);
        icLoad_ES_DI: With IntermediateCodePointer^ do
                         begin
                           CalculateLoadAddressCodeSize (Displacement, DestinationProcedure);
                           Inc (IntermediateCodePointerRec.Ofs, 7);
                         end;
        ic_ES_DI_Loaded: Inc (IntermediateCodePointerRec.Ofs, 7);
        ic_ES_DI_Destroyed: Inc (IntermediateCodePointerRec.Ofs);
        icEnd: Break;
      end;
    until False;
    SizeOfCodeReferenceRecords := AdditionalReferencesSize;
    Last_icJumpNear := LastNearJumpRecord;
    SizeOfGeneratedCode := CodeOffset - SymbolTable [stCode].UsedSize;
  end;
```

### Code Block 2

```pascal
  Procedure CalculateCodeSizeAndOffsets;
  Var IntermediateCodePointer: PIntermediateCodeRecord;
      IntermediateCodePointerRec: PtrRec absolute IntermediateCodePointer;
      CodeOffset, AdditionalReferencesSize, LastNearJumpRecord: Word;
      OpCode: Byte;

    Procedure CalculateLoadAddressCodeSize (DestinationDisplacement: Integer; DestinationProc: Word);
    Var ProcedureIdentifier: PIdentifier;
        ProcedureIdentifierRec: PtrRec absolute ProcedureIdentifier;
        ProcedureIdentifierData: PProcedureIdentifierData absolute ProcedureIdentifier;
        ProcedureIdentifierDataRec: PtrRec absolute ProcedureIdentifier;

    begin
      If DestinationProc <> CurrentProcedureIdentifier then
        begin
          ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
          Inc (CodeOffset, 3);
          Repeat
            ProcedureIdentifierRec.Ofs := ProcedureIdentifierData^.OuterBlockProcedureIdentifier;
            If ProcedureIdentifierRec.Ofs = DestinationProc then Break;
            Inc (ProcedureIdentifierRec.Ofs, ProcedureIdentifier^.Name.Len + 4);
            Inc (CodeOffset, 4);
          until False;
          If DestinationDisplacement = 0 then Exit;
          Inc (CodeOffset);
        end;
      Inc (CodeOffset, 3);
      If DestinationDisplacement <> ShortInt (DestinationDisplacement) then Inc (CodeOffset);
    end;

  begin
    AdditionalReferencesSize := 0;
    LastNearJumpRecord := 0;
    IntermediateCodePointer := Ptr (SymbolTable [stIntermediateCode].Segment, LastIntermediateCodeSubroutine);
    CodeOffset := SymbolTable [stCode].UsedSize;
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
                  Inc (CodeOffset);
                  Inc (IntermediateCodePointerRec.Ofs, 2);
                end;
        icWord: begin
                  Inc (CodeOffset, 2);
                  Inc (IntermediateCodePointerRec.Ofs, 3);
                end;
        icJumpNear: With IntermediateCodePointer^ do
                      begin
                        JumpCodeOffset := CodeOffset;
                        OpCode := JumpOpCode;
                        Previous_icJumpNear := LastNearJumpRecord;
                        LastNearJumpRecord := IntermediateCodePointerRec.Ofs;
                        Inc (IntermediateCodePointerRec.Ofs, 8);
                        Inc (CodeOffset, 3);                                            { JMP_Near size }
                        If OpCode < JMP_ConditionalShortLimit then Inc (CodeOffset, 2); { In case conditional jump will }
                      end;                                                              { become two jumps, short and near }
        icJumpShort: begin
                       Inc (IntermediateCodePointerRec.Ofs, 8);
                       Inc (CodeOffset, 2);
                     end;
        icLabel: begin
                   IntermediateCodePointer^.LabelAddress := CodeOffset;
                   Inc (IntermediateCodePointerRec.Ofs, 3);
                 end;
        icReference,
        icReferenceAsmBlock: begin
                               Inc (AdditionalReferencesSize, 8);
                               Inc (CodeOffset, 2);
                               If IntermediateCodePointer^.Flags * [rfSegment, rfOffset] = [rfSegment, rfOffset] then
                                 Inc (CodeOffset, 2);
                               Inc (IntermediateCodePointerRec.Ofs, 7);
                             end;
        ic12,
        icSourceLineNumber,
        icSkippedGoSubSaveAddress: Inc (IntermediateCodePointerRec.Ofs, 3);
        icLoad_ES_DI: With IntermediateCodePointer^ do
                         begin
                           CalculateLoadAddressCodeSize (Displacement, DestinationProcedure);
                           Inc (IntermediateCodePointerRec.Ofs, 7);
                         end;
        ic_ES_DI_Loaded: Inc (IntermediateCodePointerRec.Ofs, 7);
        ic_ES_DI_Destroyed: Inc (IntermediateCodePointerRec.Ofs);
        icEnd: Break;
      end;
    until False;
    SizeOfCodeReferenceRecords := AdditionalReferencesSize;
    Last_icJumpNear := LastNearJumpRecord;
    SizeOfGeneratedCode := CodeOffset - SymbolTable [stCode].UsedSize;
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
- https://turbopascal.org/expressions
- https://turbopascal.org/intermediate-code
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

