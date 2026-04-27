# CASE Statement - Turbo Pascal Compiler

**URL:** https://turbopascal.org/case-statement

---

## Table of Contents

- CASE Statement

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
CASE Statement
This procedure processes Case statement. It
expects ordinal expression
, processes
Case
constants and constant ranges and generates comparison intermediate code.
Procedure TStatement.Process_CASE_Statement;
Var CaseExpression: TExpression;
    LastJumpToStartOfCaseStatement, LastJumpToNextCaseComparison, LastJumpToEndOf_CASE_Statement: Word;
    IntermediateCodeOffsets: TIntermediateCodeOffsets;
    CaseExpressionOrdinalType: PTypeDefinition;
    Statement: TStatement;

  Procedure ExpectCaseConstantsCompareAndJump;
  Var OpCodeL, OpCodeH: Byte;

    Procedure ExpectCaseConstantAndCompare;
    Var LowestCommonIntegerType: TIntegerTypeSet;
        CaseConstant: TExpression;
    begin
      With CaseConstant do
        begin
          ExpectConstantExpression;
          If TypeDefPtr <> CaseExpressionOrdinalType then Error (ConstantAnd_CASE_TypesDoNotMatch);
          SetLowestCommonIntegerType (CaseExpression.DataType, DataType, LowestCommonIntegerType);
          If LowestCommonIntegerType <> CaseExpression.DataType then Error (CASE_ConstantOutOfRange);
          Case it16Bit in LowestCommonIntegerType of
            True: begin
                    GenerateInstruction_Byte (CMP_AX_Immediate);
                    GenerateInstruction_Word (Value.Word);
                  end;
            else GenerateInstruction_TwoBytes (CMP_AL_Immediate, Value.Byte);
          end;
        end;
    end;

  begin
    MarkSourceLineNumber (GetCurrentProgramBlockSourceLineNumber);
    Repeat
      ExpectCaseConstantAndCompare;
      OpCodeH := JE;
      If CheckAndGetNextToken (Token_PeriodPeriod) then
        begin
          OpCodeL := JL;
          OpCodeH := JLE;
          If itUnsigned in CaseExpression.DataType then
            begin
              OpCodeL := JB;
              OpCodeH := JBE;
            end;
          GenerateCodeForNearJump (LastJumpToNextCaseComparison, OpCodeL);
          ExpectCaseConstantAndCompare;
        end;
      If not CheckAndGetNextToken (Token_Comma) then Break;
      GenerateCodeForNearJump (LastJumpToStartOfCaseStatement, OpCodeH);
      GenerateLabelAndSetJumpsToIt (LastJumpToNextCaseComparison);
    until False;
    GenerateCodeForNearJump (LastJumpToNextCaseComparison, OpCodeH xor $01);
    GenerateLabelAndSetJumpsToIt (LastJumpToStartOfCaseStatement);
    ExpectTokenAndGetNext (Token_Colon);
  end;

begin
  GetNextToken;
  LastJumpToStartOfCaseStatement := 0;
  LastJumpToNextCaseComparison := 0;
  LastJumpToEndOf_CASE_Statement := 0;
  IntermediateCodeOffsets.NumberOfStatements := 0;
  With CaseExpression do
    begin
      ExpectOrdinalExpression;
      CaseExpressionOrdinalType := PointerFromOffsets (POrdinalTypeDefinition (TypeDefPtr)^.OrdinalType);
      If it32Bit in DataType then CheckOrdinalRange (Ptr (SystemUnitSegment, Integer_TypeOffset));
      Calculate;
      LoadExpressionToRegisters (urAX);
    end;
  AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
  ExpectTokenAndGetNext (Token_OF);
  Repeat
    ExpectCaseConstantsCompareAndJump;
    AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
    Statement.ProcessStatement;
    AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, Statement.StatementCode);
    If not CheckAndGetNextToken (Token_Semicolon) then Break;
    Case Token of
      Token_END,
      Token_ELSE: Break;
    end;
    GenerateCodeForNearJump (LastJumpToEndOf_CASE_Statement, JMP_ShortDirect);
    GenerateLabelAndSetJumpsToIt (LastJumpToNextCaseComparison);
  until False;
  If CheckAndGetNextToken (Token_ELSE) then
    begin
      GenerateCodeForNearJump (LastJumpToEndOf_CASE_Statement, JMP_ShortDirect);
      GenerateLabelAndSetJumpsToIt (LastJumpToNextCaseComparison);
      AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
      Repeat
        Statement.ProcessStatement;
        AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, Statement.StatementCode);
        If Token = Token_END then Break;
        ExpectTokenAndGetNext (Token_Semicolon);
      until False;
    end;
  ExpectTokenAndGetNext (Token_END);
  Generate_icGoSub_ForEachSubroutine (IntermediateCodeOffsets);
  JoinJumpsOfBothExpressions (LastJumpToEndOf_CASE_Statement, LastJumpToNextCaseComparison);
  GenerateLabelAndSetJumpsToIt (LastJumpToEndOf_CASE_Statement);
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
Procedure TStatement.Process_CASE_Statement;
Var CaseExpression: TExpression;
    LastJumpToStartOfCaseStatement, LastJumpToNextCaseComparison, LastJumpToEndOf_CASE_Statement: Word;
    IntermediateCodeOffsets: TIntermediateCodeOffsets;
    CaseExpressionOrdinalType: PTypeDefinition;
    Statement: TStatement;

  Procedure ExpectCaseConstantsCompareAndJump;
  Var OpCodeL, OpCodeH: Byte;

    Procedure ExpectCaseConstantAndCompare;
    Var LowestCommonIntegerType: TIntegerTypeSet;
        CaseConstant: TExpression;
    begin
      With CaseConstant do
        begin
          ExpectConstantExpression;
          If TypeDefPtr <> CaseExpressionOrdinalType then Error (ConstantAnd_CASE_TypesDoNotMatch);
          SetLowestCommonIntegerType (CaseExpression.DataType, DataType, LowestCommonIntegerType);
          If LowestCommonIntegerType <> CaseExpression.DataType then Error (CASE_ConstantOutOfRange);
          Case it16Bit in LowestCommonIntegerType of
            True: begin
                    GenerateInstruction_Byte (CMP_AX_Immediate);
                    GenerateInstruction_Word (Value.Word);
                  end;
            else GenerateInstruction_TwoBytes (CMP_AL_Immediate, Value.Byte);
          end;
        end;
    end;

  begin
    MarkSourceLineNumber (GetCurrentProgramBlockSourceLineNumber);
    Repeat
      ExpectCaseConstantAndCompare;
      OpCodeH := JE;
      If CheckAndGetNextToken (Token_PeriodPeriod) then
        begin
          OpCodeL := JL;
          OpCodeH := JLE;
          If itUnsigned in CaseExpression.DataType then
            begin
              OpCodeL := JB;
              OpCodeH := JBE;
            end;
          GenerateCodeForNearJump (LastJumpToNextCaseComparison, OpCodeL);
          ExpectCaseConstantAndCompare;
        end;
      If not CheckAndGetNextToken (Token_Comma) then Break;
      GenerateCodeForNearJump (LastJumpToStartOfCaseStatement, OpCodeH);
      GenerateLabelAndSetJumpsToIt (LastJumpToNextCaseComparison);
    until False;
    GenerateCodeForNearJump (LastJumpToNextCaseComparison, OpCodeH xor $01);
    GenerateLabelAndSetJumpsToIt (LastJumpToStartOfCaseStatement);
    ExpectTokenAndGetNext (Token_Colon);
  end;

begin
  GetNextToken;
  LastJumpToStartOfCaseStatement := 0;
  LastJumpToNextCaseComparison := 0;
  LastJumpToEndOf_CASE_Statement := 0;
  IntermediateCodeOffsets.NumberOfStatements := 0;
  With CaseExpression do
    begin
      ExpectOrdinalExpression;
      CaseExpressionOrdinalType := PointerFromOffsets (POrdinalTypeDefinition (TypeDefPtr)^.OrdinalType);
      If it32Bit in DataType then CheckOrdinalRange (Ptr (SystemUnitSegment, Integer_TypeOffset));
      Calculate;
      LoadExpressionToRegisters (urAX);
    end;
  AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
  ExpectTokenAndGetNext (Token_OF);
  Repeat
    ExpectCaseConstantsCompareAndJump;
    AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
    Statement.ProcessStatement;
    AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, Statement.StatementCode);
    If not CheckAndGetNextToken (Token_Semicolon) then Break;
    Case Token of
      Token_END,
      Token_ELSE: Break;
    end;
    GenerateCodeForNearJump (LastJumpToEndOf_CASE_Statement, JMP_ShortDirect);
    GenerateLabelAndSetJumpsToIt (LastJumpToNextCaseComparison);
  until False;
  If CheckAndGetNextToken (Token_ELSE) then
    begin
      GenerateCodeForNearJump (LastJumpToEndOf_CASE_Statement, JMP_ShortDirect);
      GenerateLabelAndSetJumpsToIt (LastJumpToNextCaseComparison);
      AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
      Repeat
        Statement.ProcessStatement;
        AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, Statement.StatementCode);
        If Token = Token_END then Break;
        ExpectTokenAndGetNext (Token_Semicolon);
      until False;
    end;
  ExpectTokenAndGetNext (Token_END);
  Generate_icGoSub_ForEachSubroutine (IntermediateCodeOffsets);
  JoinJumpsOfBothExpressions (LastJumpToEndOf_CASE_Statement, LastJumpToNextCaseComparison);
  GenerateLabelAndSetJumpsToIt (LastJumpToEndOf_CASE_Statement);
  StatementCode := EndSubroutine;
end;
```

### Code Block 2

```pascal
Procedure TStatement.Process_CASE_Statement;
Var CaseExpression: TExpression;
    LastJumpToStartOfCaseStatement, LastJumpToNextCaseComparison, LastJumpToEndOf_CASE_Statement: Word;
    IntermediateCodeOffsets: TIntermediateCodeOffsets;
    CaseExpressionOrdinalType: PTypeDefinition;
    Statement: TStatement;

  Procedure ExpectCaseConstantsCompareAndJump;
  Var OpCodeL, OpCodeH: Byte;

    Procedure ExpectCaseConstantAndCompare;
    Var LowestCommonIntegerType: TIntegerTypeSet;
        CaseConstant: TExpression;
    begin
      With CaseConstant do
        begin
          ExpectConstantExpression;
          If TypeDefPtr <> CaseExpressionOrdinalType then Error (ConstantAnd_CASE_TypesDoNotMatch);
          SetLowestCommonIntegerType (CaseExpression.DataType, DataType, LowestCommonIntegerType);
          If LowestCommonIntegerType <> CaseExpression.DataType then Error (CASE_ConstantOutOfRange);
          Case it16Bit in LowestCommonIntegerType of
            True: begin
                    GenerateInstruction_Byte (CMP_AX_Immediate);
                    GenerateInstruction_Word (Value.Word);
                  end;
            else GenerateInstruction_TwoBytes (CMP_AL_Immediate, Value.Byte);
          end;
        end;
    end;

  begin
    MarkSourceLineNumber (GetCurrentProgramBlockSourceLineNumber);
    Repeat
      ExpectCaseConstantAndCompare;
      OpCodeH := JE;
      If CheckAndGetNextToken (Token_PeriodPeriod) then
        begin
          OpCodeL := JL;
          OpCodeH := JLE;
          If itUnsigned in CaseExpression.DataType then
            begin
              OpCodeL := JB;
              OpCodeH := JBE;
            end;
          GenerateCodeForNearJump (LastJumpToNextCaseComparison, OpCodeL);
          ExpectCaseConstantAndCompare;
        end;
      If not CheckAndGetNextToken (Token_Comma) then Break;
      GenerateCodeForNearJump (LastJumpToStartOfCaseStatement, OpCodeH);
      GenerateLabelAndSetJumpsToIt (LastJumpToNextCaseComparison);
    until False;
    GenerateCodeForNearJump (LastJumpToNextCaseComparison, OpCodeH xor $01);
    GenerateLabelAndSetJumpsToIt (LastJumpToStartOfCaseStatement);
    ExpectTokenAndGetNext (Token_Colon);
  end;

begin
  GetNextToken;
  LastJumpToStartOfCaseStatement := 0;
  LastJumpToNextCaseComparison := 0;
  LastJumpToEndOf_CASE_Statement := 0;
  IntermediateCodeOffsets.NumberOfStatements := 0;
  With CaseExpression do
    begin
      ExpectOrdinalExpression;
      CaseExpressionOrdinalType := PointerFromOffsets (POrdinalTypeDefinition (TypeDefPtr)^.OrdinalType);
      If it32Bit in DataType then CheckOrdinalRange (Ptr (SystemUnitSegment, Integer_TypeOffset));
      Calculate;
      LoadExpressionToRegisters (urAX);
    end;
  AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
  ExpectTokenAndGetNext (Token_OF);
  Repeat
    ExpectCaseConstantsCompareAndJump;
    AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
    Statement.ProcessStatement;
    AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, Statement.StatementCode);
    If not CheckAndGetNextToken (Token_Semicolon) then Break;
    Case Token of
      Token_END,
      Token_ELSE: Break;
    end;
    GenerateCodeForNearJump (LastJumpToEndOf_CASE_Statement, JMP_ShortDirect);
    GenerateLabelAndSetJumpsToIt (LastJumpToNextCaseComparison);
  until False;
  If CheckAndGetNextToken (Token_ELSE) then
    begin
      GenerateCodeForNearJump (LastJumpToEndOf_CASE_Statement, JMP_ShortDirect);
      GenerateLabelAndSetJumpsToIt (LastJumpToNextCaseComparison);
      AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
      Repeat
        Statement.ProcessStatement;
        AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, Statement.StatementCode);
        If Token = Token_END then Break;
        ExpectTokenAndGetNext (Token_Semicolon);
      until False;
    end;
  ExpectTokenAndGetNext (Token_END);
  Generate_icGoSub_ForEachSubroutine (IntermediateCodeOffsets);
  JoinJumpsOfBothExpressions (LastJumpToEndOf_CASE_Statement, LastJumpToNextCaseComparison);
  GenerateLabelAndSetJumpsToIt (LastJumpToEndOf_CASE_Statement);
  StatementCode := EndSubroutine;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/case-statement
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/expecting-different-expressions
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

