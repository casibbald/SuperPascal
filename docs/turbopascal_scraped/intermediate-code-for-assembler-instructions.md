# Intermediate Code For Assembler Instructions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/intermediate-code-for-assembler-instructions

---

## Table of Contents

- Intermediate Code For Assembler Instructions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Intermediate Code For Assembler Instructions
This procedure generates
intermediate code
from opcodes and references generated for current assembler instruction.
Procedure GenerateAsmStatementIntermediateCode;
Var AsmCodeBlock: PAsmCodeBlock;

  Procedure Generate_rcByte (OpCode: Byte);
  Var IntermediateCodeRecord: PIntermediateCodeRecord;
  begin
    IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 2);
    With IntermediateCodeRecord^ do
      begin
        RecordType := icByte;
        ByteCode := OpCode;
      end;
  end;

  Procedure Generate_Reference (W2, W4, W6, W8: Word; B1: Byte);
  Var IntermediateCodeRecord: PIntermediateCodeRecord;
      Word1, Word2: Word;
  begin
    IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 7);
    Word1 := W8 or Word (B1) shl 12;
    Word2 := W6;
    If (Word1 and $3000 = 0) and (Word1 and $8000 <> 0) then AsmError (InvalidSymbolReference);
    With IntermediateCodeRecord^ do
      begin
        If (Word1 and $C000 = $4000) and (Word2 <> 0) then RecordType := icReferenceAsmBlock else RecordType := icReference;
        ReferencedUnitRecordOfs := Word1;
        ReferencedBlockRecordOfs := Word2;
        ReferencedOfs := W2;
      end;
  end;

  Procedure Generate_rcJumpNear (W6, W8: Word; OpCode: Byte);
  Var IntermediateCodeRecord: PIntermediateCodeRecord;
  begin
    IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 8);
    If W8 and $C000 <> $4000 then AsmError (InvalidSymbolReference);
    With IntermediateCodeRecord^ do
      begin
        RecordType := icJumpNear;
        JumpCode := OpCode;
        LabelIdentifierOffset := W6;
        Next_GOTO_Record := Last_GOTO_Record;
      end;
    Last_GOTO_Record := Ofs (IntermediateCodeRecord^);
  end;

  Procedure Generate_FPU_Instruction (B1_01, B1_FE, W2: Byte);
  begin
    Case B1_01 of
      0: begin
           If W2 = $9B then GenerateInstruction_TwoBytes ($CD, $3D) else
             If B1_FE = 0 then GenerateInstruction_Byte (W2) else
               GenerateInstruction_TwoBytes (B1_FE, W2);
         end;
      1: begin
           If B1_FE <> 0 then ModifyFPUInstructionForFixup (W2, B1_FE - $26) else ModifyFPUInstructionForFixup (W2, $FF);
           GenerateInstruction_Byte (W2);
         end;
    end;
  end;

begin
  AsmCodeBlock := @AsmIntermediateCodeRecord;
  While SizeOfAsmIntermediateCodeRecords > 0 do
    begin
      Dec (SizeOfAsmIntermediateCodeRecords, SizeOf (TAsmCodeBlock));
      With AsmCodeBlock^ do
        Case Recordtype of
          acByte:      Generate_rcByte (OpCode);
          acReference: Generate_Reference (icW2, icW4, icW6, icW8, icB1);
          acJumpNear:  Generate_rcJumpNear (icW6, icW8, JumpOpCode);
          else         Generate_FPU_Instruction (icB1 and $01, icB1 and $FE, icW2);
        end;
      Inc (AsmCodeBlock);
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
Procedure GenerateAsmStatementIntermediateCode;
Var AsmCodeBlock: PAsmCodeBlock;

  Procedure Generate_rcByte (OpCode: Byte);
  Var IntermediateCodeRecord: PIntermediateCodeRecord;
  begin
    IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 2);
    With IntermediateCodeRecord^ do
      begin
        RecordType := icByte;
        ByteCode := OpCode;
      end;
  end;

  Procedure Generate_Reference (W2, W4, W6, W8: Word; B1: Byte);
  Var IntermediateCodeRecord: PIntermediateCodeRecord;
      Word1, Word2: Word;
  begin
    IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 7);
    Word1 := W8 or Word (B1) shl 12;
    Word2 := W6;
    If (Word1 and $3000 = 0) and (Word1 and $8000 <> 0) then AsmError (InvalidSymbolReference);
    With IntermediateCodeRecord^ do
      begin
        If (Word1 and $C000 = $4000) and (Word2 <> 0) then RecordType := icReferenceAsmBlock else RecordType := icReference;
        ReferencedUnitRecordOfs := Word1;
        ReferencedBlockRecordOfs := Word2;
        ReferencedOfs := W2;
      end;
  end;

  Procedure Generate_rcJumpNear (W6, W8: Word; OpCode: Byte);
  Var IntermediateCodeRecord: PIntermediateCodeRecord;
  begin
    IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 8);
    If W8 and $C000 <> $4000 then AsmError (InvalidSymbolReference);
    With IntermediateCodeRecord^ do
      begin
        RecordType := icJumpNear;
        JumpCode := OpCode;
        LabelIdentifierOffset := W6;
        Next_GOTO_Record := Last_GOTO_Record;
      end;
    Last_GOTO_Record := Ofs (IntermediateCodeRecord^);
  end;

  Procedure Generate_FPU_Instruction (B1_01, B1_FE, W2: Byte);
  begin
    Case B1_01 of
      0: begin
           If W2 = $9B then GenerateInstruction_TwoBytes ($CD, $3D) else
             If B1_FE = 0 then GenerateInstruction_Byte (W2) else
               GenerateInstruction_TwoBytes (B1_FE, W2);
         end;
      1: begin
           If B1_FE <> 0 then ModifyFPUInstructionForFixup (W2, B1_FE - $26) else ModifyFPUInstructionForFixup (W2, $FF);
           GenerateInstruction_Byte (W2);
         end;
    end;
  end;

begin
  AsmCodeBlock := @AsmIntermediateCodeRecord;
  While SizeOfAsmIntermediateCodeRecords > 0 do
    begin
      Dec (SizeOfAsmIntermediateCodeRecords, SizeOf (TAsmCodeBlock));
      With AsmCodeBlock^ do
        Case Recordtype of
          acByte:      Generate_rcByte (OpCode);
          acReference: Generate_Reference (icW2, icW4, icW6, icW8, icB1);
          acJumpNear:  Generate_rcJumpNear (icW6, icW8, JumpOpCode);
          else         Generate_FPU_Instruction (icB1 and $01, icB1 and $FE, icW2);
        end;
      Inc (AsmCodeBlock);
    end;
end;
```

### Code Block 2

```pascal
Procedure GenerateAsmStatementIntermediateCode;
Var AsmCodeBlock: PAsmCodeBlock;

  Procedure Generate_rcByte (OpCode: Byte);
  Var IntermediateCodeRecord: PIntermediateCodeRecord;
  begin
    IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 2);
    With IntermediateCodeRecord^ do
      begin
        RecordType := icByte;
        ByteCode := OpCode;
      end;
  end;

  Procedure Generate_Reference (W2, W4, W6, W8: Word; B1: Byte);
  Var IntermediateCodeRecord: PIntermediateCodeRecord;
      Word1, Word2: Word;
  begin
    IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 7);
    Word1 := W8 or Word (B1) shl 12;
    Word2 := W6;
    If (Word1 and $3000 = 0) and (Word1 and $8000 <> 0) then AsmError (InvalidSymbolReference);
    With IntermediateCodeRecord^ do
      begin
        If (Word1 and $C000 = $4000) and (Word2 <> 0) then RecordType := icReferenceAsmBlock else RecordType := icReference;
        ReferencedUnitRecordOfs := Word1;
        ReferencedBlockRecordOfs := Word2;
        ReferencedOfs := W2;
      end;
  end;

  Procedure Generate_rcJumpNear (W6, W8: Word; OpCode: Byte);
  Var IntermediateCodeRecord: PIntermediateCodeRecord;
  begin
    IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 8);
    If W8 and $C000 <> $4000 then AsmError (InvalidSymbolReference);
    With IntermediateCodeRecord^ do
      begin
        RecordType := icJumpNear;
        JumpCode := OpCode;
        LabelIdentifierOffset := W6;
        Next_GOTO_Record := Last_GOTO_Record;
      end;
    Last_GOTO_Record := Ofs (IntermediateCodeRecord^);
  end;

  Procedure Generate_FPU_Instruction (B1_01, B1_FE, W2: Byte);
  begin
    Case B1_01 of
      0: begin
           If W2 = $9B then GenerateInstruction_TwoBytes ($CD, $3D) else
             If B1_FE = 0 then GenerateInstruction_Byte (W2) else
               GenerateInstruction_TwoBytes (B1_FE, W2);
         end;
      1: begin
           If B1_FE <> 0 then ModifyFPUInstructionForFixup (W2, B1_FE - $26) else ModifyFPUInstructionForFixup (W2, $FF);
           GenerateInstruction_Byte (W2);
         end;
    end;
  end;

begin
  AsmCodeBlock := @AsmIntermediateCodeRecord;
  While SizeOfAsmIntermediateCodeRecords > 0 do
    begin
      Dec (SizeOfAsmIntermediateCodeRecords, SizeOf (TAsmCodeBlock));
      With AsmCodeBlock^ do
        Case Recordtype of
          acByte:      Generate_rcByte (OpCode);
          acReference: Generate_Reference (icW2, icW4, icW6, icW8, icB1);
          acJumpNear:  Generate_rcJumpNear (icW6, icW8, JumpOpCode);
          else         Generate_FPU_Instruction (icB1 and $01, icB1 and $FE, icW2);
        end;
      Inc (AsmCodeBlock);
    end;
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
- https://turbopascal.org/intermediate-code-for-assembler-instructions
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

