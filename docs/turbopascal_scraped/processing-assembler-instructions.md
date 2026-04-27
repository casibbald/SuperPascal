# Processing Assembler Instructions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-assembler-instructions

---

## Table of Contents

- Processing Assembler Instructions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Assembler Instructions
This procedure processes one assembler instruction. It resets data used for instruction processing, reads
next assembler token
, checks for label and processes prefix instructions (if present). Then it checks and
processes assembler directive
(
DB
,
DW
or
DD
) or
assembler instruction parameters
. Finally it generates
intermediate code for assembler instruction
.
Procedure ProcessAssemblerInstruction (Var CharPtr: PChar; LineCharCounter: Byte; AsmFlags: Word; ProcIdDataOffset: Word);

  Procedure ResetAsmStatementData;
  begin
    LineCharacterCounter := LineCharCounter;
    Flags := AsmFlags;
    MaxAsmStackTop := SPtr - $400;
    AsmCharacterPointer := CharPtr;
    AsmSourceErrorPosition := nil;
    AsmIdentifierAdditionalData := nil;
    PrefixOpcode [0] := 0;
    SizeOfAsmIntermediateCodeRecords := 0;
    LastOpCodeBeforeDataValid := False;
  end;

begin
  ResetAsmStatementData;
  BeforeOperands := True;
  GetNextAsmToken;
  AsmSourceErrorPosition := AsmCharacterPointer;
  CheckForAsmLabel;
  ProcessPrefixes;
  BeforeOperands := False;
  If not ProcessAssemblerDirective then
    begin
      If not CheckAndGetNextAsmToken (AsmToken_Semicolon) then
        begin
          ExpectAsmTokenAndGetNext (AsmToken_Instruction);
          ProcessInstructionExpresions;
          GenerateInstructionOpcodes;
        end else GeneratePrefixOpCodes;
    end;
  GenerateAsmStatementIntermediateCode;
  CharPtr := AsmCharacterPointer;
end;
This procedure checks for valid assembler label and creates intermediate code label record.
Procedure CheckForAsmLabel;
Var LabelIdentifierData: PLabelIdentifierData;
    IdToken: TToken;
    LabelIdentifier: Word;
    IntermediateCodeRecord: PIntermediateCodeRecord;
begin
  If GetFirstNonBlankCharacter = ':' then
    begin
      CopyStringToCurrentIdentifier (AsmTokenString);
      If not IsValidAsmIdentifier (nil, IdToken, Pointer (LabelIdentifierData), LabelIdentifier) or
         (IdToken <> Token_LabelIdentifier) then
        AsmError (UnknownIdentifier);
      If Ofs (LabelIdentifierData^) < Ofs (CurrentScopeIdentifierTableAddress^) then AsmError (LabelNotWithinCurrentBlock);
      If LabelIdentifierData^.LabelRecordOffset <> 0 then AsmError (LabelAlreadyDefined);
      IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 3);
      IntermediateCodeRecord^.RecordType := icLabel;
      LabelIdentifierData^.LabelRecordOffset := Ofs (IntermediateCodeRecord^) + 1;
      GetNextAsmToken;
      ExpectAsmTokenAndGetNext (AsmToken_Colon);
    end;
end;
This procedure processes prefixes in instruction (
LOCK
,
REP
,
REPE
,
REPZ
,
REPNE
,
REPNZ
,
SEGES
,
SEGCS
,
SEGSS
,
SEGDS
) and stores its opcodes to
PrefixOpcode
buffer.
Procedure ProcessPrefixes;
Var Counter: Byte;
begin
  While AsmToken = AsmToken_Instruction do
    begin
      If InstructionData^.B0 <> $AE then Exit;
      Counter := 0;
      While (PrefixOpcode [Counter] <> 0) and (Counter < 4) do Inc (Counter);
      PrefixOpcode [Counter] := InstructionData^.Data [0];
      PrefixOpcode [Counter + 1] := 0;
      GetNextAsmToken;
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
Procedure ProcessAssemblerInstruction (Var CharPtr: PChar; LineCharCounter: Byte; AsmFlags: Word; ProcIdDataOffset: Word);

  Procedure ResetAsmStatementData;
  begin
    LineCharacterCounter := LineCharCounter;
    Flags := AsmFlags;
    MaxAsmStackTop := SPtr - $400;
    AsmCharacterPointer := CharPtr;
    AsmSourceErrorPosition := nil;
    AsmIdentifierAdditionalData := nil;
    PrefixOpcode [0] := 0;
    SizeOfAsmIntermediateCodeRecords := 0;
    LastOpCodeBeforeDataValid := False;
  end;

begin
  ResetAsmStatementData;
  BeforeOperands := True;
  GetNextAsmToken;
  AsmSourceErrorPosition := AsmCharacterPointer;
  CheckForAsmLabel;
  ProcessPrefixes;
  BeforeOperands := False;
  If not ProcessAssemblerDirective then
    begin
      If not CheckAndGetNextAsmToken (AsmToken_Semicolon) then
        begin
          ExpectAsmTokenAndGetNext (AsmToken_Instruction);
          ProcessInstructionExpresions;
          GenerateInstructionOpcodes;
        end else GeneratePrefixOpCodes;
    end;
  GenerateAsmStatementIntermediateCode;
  CharPtr := AsmCharacterPointer;
end;
```

### Code Block 2

```pascal
Procedure CheckForAsmLabel;
Var LabelIdentifierData: PLabelIdentifierData;
    IdToken: TToken;
    LabelIdentifier: Word;
    IntermediateCodeRecord: PIntermediateCodeRecord;
begin
  If GetFirstNonBlankCharacter = ':' then
    begin
      CopyStringToCurrentIdentifier (AsmTokenString);
      If not IsValidAsmIdentifier (nil, IdToken, Pointer (LabelIdentifierData), LabelIdentifier) or
         (IdToken <> Token_LabelIdentifier) then
        AsmError (UnknownIdentifier);
      If Ofs (LabelIdentifierData^) < Ofs (CurrentScopeIdentifierTableAddress^) then AsmError (LabelNotWithinCurrentBlock);
      If LabelIdentifierData^.LabelRecordOffset <> 0 then AsmError (LabelAlreadyDefined);
      IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 3);
      IntermediateCodeRecord^.RecordType := icLabel;
      LabelIdentifierData^.LabelRecordOffset := Ofs (IntermediateCodeRecord^) + 1;
      GetNextAsmToken;
      ExpectAsmTokenAndGetNext (AsmToken_Colon);
    end;
end;
```

### Code Block 3

```pascal
Procedure ProcessPrefixes;
Var Counter: Byte;
begin
  While AsmToken = AsmToken_Instruction do
    begin
      If InstructionData^.B0 <> $AE then Exit;
      Counter := 0;
      While (PrefixOpcode [Counter] <> 0) and (Counter < 4) do Inc (Counter);
      PrefixOpcode [Counter] := InstructionData^.Data [0];
      PrefixOpcode [Counter + 1] := 0;
      GetNextAsmToken;
    end;
end;
```

### Code Block 4

```pascal
Procedure ProcessAssemblerInstruction (Var CharPtr: PChar; LineCharCounter: Byte; AsmFlags: Word; ProcIdDataOffset: Word);

  Procedure ResetAsmStatementData;
  begin
    LineCharacterCounter := LineCharCounter;
    Flags := AsmFlags;
    MaxAsmStackTop := SPtr - $400;
    AsmCharacterPointer := CharPtr;
    AsmSourceErrorPosition := nil;
    AsmIdentifierAdditionalData := nil;
    PrefixOpcode [0] := 0;
    SizeOfAsmIntermediateCodeRecords := 0;
    LastOpCodeBeforeDataValid := False;
  end;

begin
  ResetAsmStatementData;
  BeforeOperands := True;
  GetNextAsmToken;
  AsmSourceErrorPosition := AsmCharacterPointer;
  CheckForAsmLabel;
  ProcessPrefixes;
  BeforeOperands := False;
  If not ProcessAssemblerDirective then
    begin
      If not CheckAndGetNextAsmToken (AsmToken_Semicolon) then
        begin
          ExpectAsmTokenAndGetNext (AsmToken_Instruction);
          ProcessInstructionExpresions;
          GenerateInstructionOpcodes;
        end else GeneratePrefixOpCodes;
    end;
  GenerateAsmStatementIntermediateCode;
  CharPtr := AsmCharacterPointer;
end;
```

### Code Block 5

```pascal
Procedure CheckForAsmLabel;
Var LabelIdentifierData: PLabelIdentifierData;
    IdToken: TToken;
    LabelIdentifier: Word;
    IntermediateCodeRecord: PIntermediateCodeRecord;
begin
  If GetFirstNonBlankCharacter = ':' then
    begin
      CopyStringToCurrentIdentifier (AsmTokenString);
      If not IsValidAsmIdentifier (nil, IdToken, Pointer (LabelIdentifierData), LabelIdentifier) or
         (IdToken <> Token_LabelIdentifier) then
        AsmError (UnknownIdentifier);
      If Ofs (LabelIdentifierData^) < Ofs (CurrentScopeIdentifierTableAddress^) then AsmError (LabelNotWithinCurrentBlock);
      If LabelIdentifierData^.LabelRecordOffset <> 0 then AsmError (LabelAlreadyDefined);
      IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 3);
      IntermediateCodeRecord^.RecordType := icLabel;
      LabelIdentifierData^.LabelRecordOffset := Ofs (IntermediateCodeRecord^) + 1;
      GetNextAsmToken;
      ExpectAsmTokenAndGetNext (AsmToken_Colon);
    end;
end;
```

### Code Block 6

```pascal
Procedure ProcessPrefixes;
Var Counter: Byte;
begin
  While AsmToken = AsmToken_Instruction do
    begin
      If InstructionData^.B0 <> $AE then Exit;
      Counter := 0;
      While (PrefixOpcode [Counter] <> 0) and (Counter < 4) do Inc (Counter);
      PrefixOpcode [Counter] := InstructionData^.Data [0];
      PrefixOpcode [Counter + 1] := 0;
      GetNextAsmToken;
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
- https://turbopascal.org/extracting-assembler-tokens
- https://turbopascal.org/intermediate-code-for-assembler-instructions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-assembler-directives
- https://turbopascal.org/processing-assembler-expressions
- https://turbopascal.org/processing-assembler-instructions
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

