# GOTO Statement - Turbo Pascal Compiler

**URL:** https://turbopascal.org/goto-statement

---

## Table of Contents

- GOTO Statement

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
GOTO Statement
This procedure processes
GoTo
statement. It expects already declared
numeric label
that should be in the in the current block and adds
GoTo
record to a linked list.
Procedure TStatement.Process_GOTO_Statement;
Var JumpCodeRecord: PIntermediateCodeRecord;
begin
  GetNextToken;
  CheckForNumericLabel;
  CheckForDeclaredIdentifier;
  ExpectTokenAndGetNext (Token_LabelIdentifier);
  If CurrentIdentifierOffset < Ofs (CurrentScopeIdentifierTableAddress^) then Error (LabelNotWithinCurrentBlock);
  JumpCodeRecord := IncreaseSymbolTable (stIntermediateCode, 8);
  With JumpCodeRecord^ do
    begin
      RecordType := icJumpNear;
      JumpOpCode := JMP_ShortDirect;
      LabelIdentifierOffset := CurrentIdentifierOffset;
      Next_GOTO_Record := Last_GOTO_Record;
    end;
  Last_GOTO_Record := Ofs (JumpCodeRecord^);
  StatementCode := EndSubroutine;
end;
After the
program block is processed
, labels are added to the
GoTo
records.
Procedure PlaceLabelRecordsTo_GOTO_Records;
Var GOTO_Record: PIntermediateCodeRecord;
    GOTO_RecordOfs: Word absolute GOTO_Record;
    LabelIdentifier: PIdentifier;
    LabelIdentifierOfs: Word absolute LabelIdentifier;
    LabelIdentifierData: PLabelIdentifierData;
    LabelIdentifierDataOfs: Word absolute LabelIdentifierData;
begin
  GOTO_Record := Ptr (SymbolTable [stIntermediateCode].Segment, Last_GOTO_Record);
  LabelIdentifier := Ptr (SymbolTable [stMain].Segment, 0);
  LabelIdentifierData := Ptr (SymbolTable [stMain].Segment, 0);
  While GOTO_RecordOfs <> 0 do
    begin
      LabelIdentifierOfs := GOTO_Record^.LabelIdentifierOffset;
      LabelIdentifierDataOfs := LabelIdentifierOfs + LabelIdentifier^.Name.Len + 4;
      If LabelIdentifierData^.LabelRecordOffset = 0 then
        IdentifierError (LabelIdentifier^.Name.Str, UndefinedLabelInPrecedingStatementPart);
      GOTO_Record^.LabelRecordOffset := LabelIdentifierData^.LabelRecordOffset;
      GOTO_RecordOfs := GOTO_Record^.Next_GOTO_Record;
    end;
end;
This procedure processes labels in program block. It expects
already declared label
, reports error if the label was already placed somewhere and generates intermediate code for the label. Finally, it
processes the next statement
.
Procedure TStatement.ProcessLabelIdentifierStatement;
Var TempLabelIdentifierData: PLabelIdentifierData;
    LabelRecord: PIntermediateCodeRecord;
    Statement: TStatement;
begin
  TempLabelIdentifierData := CurrentLabelIdentifierData;
  If Ofs (CurrentIdentifierDataPtr^) < Ofs (CurrentScopeIdentifierTableAddress^) then Error (LabelNotWithinCurrentBlock);
  If TempLabelIdentifierData^.LabelRecordOffset <> 0 then Error (LabelAlreadyDefined);
  Dec (TempLabelIdentifierData^.LabelRecordOffset);
  GetNextToken;
  ExpectTokenAndGetNext (Token_Colon);
  Statement.ProcessStatement;
  LabelRecord := IncreaseSymbolTable (stIntermediateCode, 3);
  LabelRecord^.RecordType := icLabel;
  TempLabelIdentifierData^.LabelRecordOffset := Ofs (LabelRecord^) + 1;
  StoreCode_icGoSub (Statement.StatementCode);
  StatementCode := EndSubroutine
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
Procedure TStatement.Process_GOTO_Statement;
Var JumpCodeRecord: PIntermediateCodeRecord;
begin
  GetNextToken;
  CheckForNumericLabel;
  CheckForDeclaredIdentifier;
  ExpectTokenAndGetNext (Token_LabelIdentifier);
  If CurrentIdentifierOffset < Ofs (CurrentScopeIdentifierTableAddress^) then Error (LabelNotWithinCurrentBlock);
  JumpCodeRecord := IncreaseSymbolTable (stIntermediateCode, 8);
  With JumpCodeRecord^ do
    begin
      RecordType := icJumpNear;
      JumpOpCode := JMP_ShortDirect;
      LabelIdentifierOffset := CurrentIdentifierOffset;
      Next_GOTO_Record := Last_GOTO_Record;
    end;
  Last_GOTO_Record := Ofs (JumpCodeRecord^);
  StatementCode := EndSubroutine;
end;
```

### Code Block 2

```pascal
Procedure PlaceLabelRecordsTo_GOTO_Records;
Var GOTO_Record: PIntermediateCodeRecord;
    GOTO_RecordOfs: Word absolute GOTO_Record;
    LabelIdentifier: PIdentifier;
    LabelIdentifierOfs: Word absolute LabelIdentifier;
    LabelIdentifierData: PLabelIdentifierData;
    LabelIdentifierDataOfs: Word absolute LabelIdentifierData;
begin
  GOTO_Record := Ptr (SymbolTable [stIntermediateCode].Segment, Last_GOTO_Record);
  LabelIdentifier := Ptr (SymbolTable [stMain].Segment, 0);
  LabelIdentifierData := Ptr (SymbolTable [stMain].Segment, 0);
  While GOTO_RecordOfs <> 0 do
    begin
      LabelIdentifierOfs := GOTO_Record^.LabelIdentifierOffset;
      LabelIdentifierDataOfs := LabelIdentifierOfs + LabelIdentifier^.Name.Len + 4;
      If LabelIdentifierData^.LabelRecordOffset = 0 then
        IdentifierError (LabelIdentifier^.Name.Str, UndefinedLabelInPrecedingStatementPart);
      GOTO_Record^.LabelRecordOffset := LabelIdentifierData^.LabelRecordOffset;
      GOTO_RecordOfs := GOTO_Record^.Next_GOTO_Record;
    end;
end;
```

### Code Block 3

```pascal
Procedure TStatement.ProcessLabelIdentifierStatement;
Var TempLabelIdentifierData: PLabelIdentifierData;
    LabelRecord: PIntermediateCodeRecord;
    Statement: TStatement;
begin
  TempLabelIdentifierData := CurrentLabelIdentifierData;
  If Ofs (CurrentIdentifierDataPtr^) < Ofs (CurrentScopeIdentifierTableAddress^) then Error (LabelNotWithinCurrentBlock);
  If TempLabelIdentifierData^.LabelRecordOffset <> 0 then Error (LabelAlreadyDefined);
  Dec (TempLabelIdentifierData^.LabelRecordOffset);
  GetNextToken;
  ExpectTokenAndGetNext (Token_Colon);
  Statement.ProcessStatement;
  LabelRecord := IncreaseSymbolTable (stIntermediateCode, 3);
  LabelRecord^.RecordType := icLabel;
  TempLabelIdentifierData^.LabelRecordOffset := Ofs (LabelRecord^) + 1;
  StoreCode_icGoSub (Statement.StatementCode);
  StatementCode := EndSubroutine
end;
```

### Code Block 4

```pascal
Procedure TStatement.Process_GOTO_Statement;
Var JumpCodeRecord: PIntermediateCodeRecord;
begin
  GetNextToken;
  CheckForNumericLabel;
  CheckForDeclaredIdentifier;
  ExpectTokenAndGetNext (Token_LabelIdentifier);
  If CurrentIdentifierOffset < Ofs (CurrentScopeIdentifierTableAddress^) then Error (LabelNotWithinCurrentBlock);
  JumpCodeRecord := IncreaseSymbolTable (stIntermediateCode, 8);
  With JumpCodeRecord^ do
    begin
      RecordType := icJumpNear;
      JumpOpCode := JMP_ShortDirect;
      LabelIdentifierOffset := CurrentIdentifierOffset;
      Next_GOTO_Record := Last_GOTO_Record;
    end;
  Last_GOTO_Record := Ofs (JumpCodeRecord^);
  StatementCode := EndSubroutine;
end;
```

### Code Block 5

```pascal
Procedure PlaceLabelRecordsTo_GOTO_Records;
Var GOTO_Record: PIntermediateCodeRecord;
    GOTO_RecordOfs: Word absolute GOTO_Record;
    LabelIdentifier: PIdentifier;
    LabelIdentifierOfs: Word absolute LabelIdentifier;
    LabelIdentifierData: PLabelIdentifierData;
    LabelIdentifierDataOfs: Word absolute LabelIdentifierData;
begin
  GOTO_Record := Ptr (SymbolTable [stIntermediateCode].Segment, Last_GOTO_Record);
  LabelIdentifier := Ptr (SymbolTable [stMain].Segment, 0);
  LabelIdentifierData := Ptr (SymbolTable [stMain].Segment, 0);
  While GOTO_RecordOfs <> 0 do
    begin
      LabelIdentifierOfs := GOTO_Record^.LabelIdentifierOffset;
      LabelIdentifierDataOfs := LabelIdentifierOfs + LabelIdentifier^.Name.Len + 4;
      If LabelIdentifierData^.LabelRecordOffset = 0 then
        IdentifierError (LabelIdentifier^.Name.Str, UndefinedLabelInPrecedingStatementPart);
      GOTO_Record^.LabelRecordOffset := LabelIdentifierData^.LabelRecordOffset;
      GOTO_RecordOfs := GOTO_Record^.Next_GOTO_Record;
    end;
end;
```

### Code Block 6

```pascal
Procedure TStatement.ProcessLabelIdentifierStatement;
Var TempLabelIdentifierData: PLabelIdentifierData;
    LabelRecord: PIntermediateCodeRecord;
    Statement: TStatement;
begin
  TempLabelIdentifierData := CurrentLabelIdentifierData;
  If Ofs (CurrentIdentifierDataPtr^) < Ofs (CurrentScopeIdentifierTableAddress^) then Error (LabelNotWithinCurrentBlock);
  If TempLabelIdentifierData^.LabelRecordOffset <> 0 then Error (LabelAlreadyDefined);
  Dec (TempLabelIdentifierData^.LabelRecordOffset);
  GetNextToken;
  ExpectTokenAndGetNext (Token_Colon);
  Statement.ProcessStatement;
  LabelRecord := IncreaseSymbolTable (stIntermediateCode, 3);
  LabelRecord^.RecordType := icLabel;
  TempLabelIdentifierData^.LabelRecordOffset := Ofs (LabelRecord^) + 1;
  StoreCode_icGoSub (Statement.StatementCode);
  StatementCode := EndSubroutine
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
- https://turbopascal.org/goto-statement
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-label-declarations
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

