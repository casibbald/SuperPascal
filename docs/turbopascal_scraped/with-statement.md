# WITH Statement - Turbo Pascal Compiler

**URL:** https://turbopascal.org/with-statement

---

## Table of Contents

- WITH Statement

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
WITH Statement
This procedure processes
With
statement. It expects
Record
or
Object
variable and stores its data to a linked list. The calculated address is stored on the
stack frame
for further use. Turbo Pascal generates intermediate code which can be optimized for redundant
ES:DI
address loadings. Therefore, this procedure generates appropriate instructions to mark the address that is loaded into
ES:DI
register pair.
Procedure TStatement.Process_WITH_Statement;
Var VariableReference: TExpression;
    WithStatement: TStatement;
    Saved_LastWithRecord, SaveAddressSubroutine: Word;
    IntermediateCodeOffsets: TIntermediateCodeOffsets;
    VariableReferenceSegment: Byte;
    IntermediateCodeRecord: PIntermediateCodeRecord;
    WithRecord: PWithRecord;
begin
  GetNextToken;
  Saved_LastWithRecord := LastWithRecord;
  IntermediateCodeOffsets.NumberOfStatements := 0;
  Repeat
    VariableReference.ExpectVariableReference;
    Case VariableReference.TypeDefPtr^.BaseType of
      btRecord,
      btObject:
      else Error (RecordOrObjectVariableExpected);
    end;
    Asm
      SUB   SP, TYPE TWithRecord;
    end;
    WithRecord := Ptr (SSeg, SPtr);
    WithRecord^.PreviousWithRecord := LastWithRecord;
    LastWithRecord := Ofs (WithRecord^);
    WithRecord^.RecordTypeDefinition := PRecordTypeDefinition (VariableReference.TypeDefPtr);
    With VariableReference, WithRecord^ do
      If (LocationData.Flags * [segSS, segDS] <> []) and not (ofsDI in LocationData.Flags) then
        begin
          RecordType := 1;
          ExpressionFlags := LocationData.Flags;
          AddressOffset := Value.Offset;
          BlockRecord := Value.BlockRecord;
          AddressSegment := Value.Segment;
        end else begin
                   RecordType := 2;
                   ExpressionFlags := LocationData.Flags * [efSegment] + [ofsDI, segES];
                   AddressOffset := CreateTempStackFrameSpace (4);
                   GenerateInstructionForStackFrameDisplacement (MOV_16Bit, SS_BP_Displacement_DI, AddressOffset);
                   GenerateInstructionForStackFrameDisplacement ($8C, $06, AddressOffset + 2);
                   SaveAddressSubroutine := EndSubroutine;
                   Calculate;
                   VariableReferenceSegment := Load_DI_AndReturnSegment;
                   If VariableReferenceSegment <> Segment_ES then
                     GenerateInstruction_TwoBytes (VariableReferenceSegment or $06, $07);
                   IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 10);
                   BlockRecord := Ofs (IntermediateCodeRecord^);
                   With IntermediateCodeRecord^ do
                     begin
                       RecordType := icSkippedGoSubSaveAddress;
                       SaveAddress := SaveAddressSubroutine;
                     end;
                   Inc (PChar (IntermediateCodeRecord), 3);
                   With IntermediateCodeRecord^ do
                     begin
                       RecordType           := ic_ES_DI_Loaded;
                       DestinationProcedure := CurrentProcedureIdentifier;
                       Displacement         := AddressOffset;
                       SaveAddressCode      := 0;
                     end;
                   AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
                 end;
  until not CheckAndGetNextToken (Token_Comma);
  ExpectTokenAndGetNext (Token_DO);
  WithStatement.ProcessStatement;
  AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, WithStatement.StatementCode);
  LastWithRecord := Saved_LastWithRecord;
  Generate_icGoSub_ForEachSubroutine (IntermediateCodeOffsets);
  StatementCode := EndSubroutine;
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
Procedure TStatement.Process_WITH_Statement;
Var VariableReference: TExpression;
    WithStatement: TStatement;
    Saved_LastWithRecord, SaveAddressSubroutine: Word;
    IntermediateCodeOffsets: TIntermediateCodeOffsets;
    VariableReferenceSegment: Byte;
    IntermediateCodeRecord: PIntermediateCodeRecord;
    WithRecord: PWithRecord;
begin
  GetNextToken;
  Saved_LastWithRecord := LastWithRecord;
  IntermediateCodeOffsets.NumberOfStatements := 0;
  Repeat
    VariableReference.ExpectVariableReference;
    Case VariableReference.TypeDefPtr^.BaseType of
      btRecord,
      btObject:
      else Error (RecordOrObjectVariableExpected);
    end;
    Asm
      SUB   SP, TYPE TWithRecord;
    end;
    WithRecord := Ptr (SSeg, SPtr);
    WithRecord^.PreviousWithRecord := LastWithRecord;
    LastWithRecord := Ofs (WithRecord^);
    WithRecord^.RecordTypeDefinition := PRecordTypeDefinition (VariableReference.TypeDefPtr);
    With VariableReference, WithRecord^ do
      If (LocationData.Flags * [segSS, segDS] <> []) and not (ofsDI in LocationData.Flags) then
        begin
          RecordType := 1;
          ExpressionFlags := LocationData.Flags;
          AddressOffset := Value.Offset;
          BlockRecord := Value.BlockRecord;
          AddressSegment := Value.Segment;
        end else begin
                   RecordType := 2;
                   ExpressionFlags := LocationData.Flags * [efSegment] + [ofsDI, segES];
                   AddressOffset := CreateTempStackFrameSpace (4);
                   GenerateInstructionForStackFrameDisplacement (MOV_16Bit, SS_BP_Displacement_DI, AddressOffset);
                   GenerateInstructionForStackFrameDisplacement ($8C, $06, AddressOffset + 2);
                   SaveAddressSubroutine := EndSubroutine;
                   Calculate;
                   VariableReferenceSegment := Load_DI_AndReturnSegment;
                   If VariableReferenceSegment <> Segment_ES then
                     GenerateInstruction_TwoBytes (VariableReferenceSegment or $06, $07);
                   IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 10);
                   BlockRecord := Ofs (IntermediateCodeRecord^);
                   With IntermediateCodeRecord^ do
                     begin
                       RecordType := icSkippedGoSubSaveAddress;
                       SaveAddress := SaveAddressSubroutine;
                     end;
                   Inc (PChar (IntermediateCodeRecord), 3);
                   With IntermediateCodeRecord^ do
                     begin
                       RecordType           := ic_ES_DI_Loaded;
                       DestinationProcedure := CurrentProcedureIdentifier;
                       Displacement         := AddressOffset;
                       SaveAddressCode      := 0;
                     end;
                   AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
                 end;
  until not CheckAndGetNextToken (Token_Comma);
  ExpectTokenAndGetNext (Token_DO);
  WithStatement.ProcessStatement;
  AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, WithStatement.StatementCode);
  LastWithRecord := Saved_LastWithRecord;
  Generate_icGoSub_ForEachSubroutine (IntermediateCodeOffsets);
  StatementCode := EndSubroutine;
end;
```

### Code Block 2

```pascal
Procedure TStatement.Process_WITH_Statement;
Var VariableReference: TExpression;
    WithStatement: TStatement;
    Saved_LastWithRecord, SaveAddressSubroutine: Word;
    IntermediateCodeOffsets: TIntermediateCodeOffsets;
    VariableReferenceSegment: Byte;
    IntermediateCodeRecord: PIntermediateCodeRecord;
    WithRecord: PWithRecord;
begin
  GetNextToken;
  Saved_LastWithRecord := LastWithRecord;
  IntermediateCodeOffsets.NumberOfStatements := 0;
  Repeat
    VariableReference.ExpectVariableReference;
    Case VariableReference.TypeDefPtr^.BaseType of
      btRecord,
      btObject:
      else Error (RecordOrObjectVariableExpected);
    end;
    Asm
      SUB   SP, TYPE TWithRecord;
    end;
    WithRecord := Ptr (SSeg, SPtr);
    WithRecord^.PreviousWithRecord := LastWithRecord;
    LastWithRecord := Ofs (WithRecord^);
    WithRecord^.RecordTypeDefinition := PRecordTypeDefinition (VariableReference.TypeDefPtr);
    With VariableReference, WithRecord^ do
      If (LocationData.Flags * [segSS, segDS] <> []) and not (ofsDI in LocationData.Flags) then
        begin
          RecordType := 1;
          ExpressionFlags := LocationData.Flags;
          AddressOffset := Value.Offset;
          BlockRecord := Value.BlockRecord;
          AddressSegment := Value.Segment;
        end else begin
                   RecordType := 2;
                   ExpressionFlags := LocationData.Flags * [efSegment] + [ofsDI, segES];
                   AddressOffset := CreateTempStackFrameSpace (4);
                   GenerateInstructionForStackFrameDisplacement (MOV_16Bit, SS_BP_Displacement_DI, AddressOffset);
                   GenerateInstructionForStackFrameDisplacement ($8C, $06, AddressOffset + 2);
                   SaveAddressSubroutine := EndSubroutine;
                   Calculate;
                   VariableReferenceSegment := Load_DI_AndReturnSegment;
                   If VariableReferenceSegment <> Segment_ES then
                     GenerateInstruction_TwoBytes (VariableReferenceSegment or $06, $07);
                   IntermediateCodeRecord := IncreaseSymbolTable (stIntermediateCode, 10);
                   BlockRecord := Ofs (IntermediateCodeRecord^);
                   With IntermediateCodeRecord^ do
                     begin
                       RecordType := icSkippedGoSubSaveAddress;
                       SaveAddress := SaveAddressSubroutine;
                     end;
                   Inc (PChar (IntermediateCodeRecord), 3);
                   With IntermediateCodeRecord^ do
                     begin
                       RecordType           := ic_ES_DI_Loaded;
                       DestinationProcedure := CurrentProcedureIdentifier;
                       Displacement         := AddressOffset;
                       SaveAddressCode      := 0;
                     end;
                   AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
                 end;
  until not CheckAndGetNextToken (Token_Comma);
  ExpectTokenAndGetNext (Token_DO);
  WithStatement.ProcessStatement;
  AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, WithStatement.StatementCode);
  LastWithRecord := Saved_LastWithRecord;
  Generate_icGoSub_ForEachSubroutine (IntermediateCodeOffsets);
  StatementCode := EndSubroutine;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-and-destroying-stack-frame
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
- https://turbopascal.org/with-statement

