# FOR Statement - Turbo Pascal Compiler

**URL:** https://turbopascal.org/for-statement

---

## Table of Contents

- FOR Statement

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
FOR Statement
This procedure processes
For
statement. It expects assignable variable and loop limits. It generates intermediate code to assign lower limit to control variable, code to increment control variable and code to check for upper limit taking into account
Break
and
Continue
jumps.
Procedure TStatement.Process_FOR_Statement;
Type TForStatementData = Record
                           NoLoopComparisonOperation: TCalcOperation;
                           IntegerOpCode,
                           LongIntOpCode_Lo,
                           LongIntOpCode_Hi: Byte;
                         end;

Const ForStatementData_TO: TForStatementData     = (NoLoopComparisonOperation: Calc_IsGreater;
                                                    IntegerOpCode: $00;
                                                    LongIntOpCode_Lo: mod_ADD_rm;
                                                    LongIntOpCode_Hi: mod_ADC_rm);
      ForStatementData_DOWNTO: TForStatementData = (NoLoopComparisonOperation: Calc_IsLower;
                                                    IntegerOpCode: $08;
                                                    LongIntOpCode_Lo: mod_SUB_rm;
                                                    LongIntOpCode_Hi: mod_SBB_rm);

Var SavedLastJumpOutOfBlock, SavedLastJumpToNextBlockIteration: Word;
    JumpToLoopStatement: Word;
    ForStatementData: TForStatementData;
    FOR_Variable, StartValue, EndValue, LoopRepeatComparison, NoLoopComparison, TempStartValue: TExpression;
    LoopStatement: TStatement;

  Procedure Expect_FOR_LoopLimit (Var Expression: TExpression);
  begin
    With Expression do
      begin
        ExpectOrdinalExpression;
        CheckTypeCompatibility (FOR_Variable.TypeDefPtr);
        CheckRange (FOR_Variable.TypeDefPtr);
      end;
  end;

  Procedure Compare_FOR_Variable (Operation: TCalcOperation; Var LeftExpression: TExpression; RightExpression: TExpression);
  begin
    RightExpression.IntermediateCodeOffset := 0;
    CalculateOperation (Operation, LeftExpression, RightExpression);
    With LeftExpression do If Location <> elConstant then
      begin
        Calculate;
        GenerateCodeForNearJump (Value.LastJumpToTrue, LocationData.JumpIfTrueOpCode);
        GenerateLabelAndSetJumpsToIt (Value.LastJumpToFalse);
        EndIntermediateCodeSubroutine
      end;
  end;

  Procedure Next_FOR_VariableValue;
  begin
    With FOR_Variable do
      If it32Bit in DataType then
        begin
          GenerateArithmeticInstructionWithImmediateValue (1, ForStatementData.LongIntOpCode_Lo);
          Inc (FOR_Variable.Value.Offset, 2);
          GenerateArithmeticInstructionWithImmediateValue (0, ForStatementData.LongIntOpCode_Hi);
          Dec (Value.Offset, 2);
        end else GenerateInstruction_8_16_bit ($FE, ForStatementData.IntegerOpCode);
  end;

begin
  SavedLastJumpOutOfBlock := LastJumpOutOfBlock;
  SavedLastJumpToNextBlockIteration := LastJumpToNextBlockIteration;
  LastJumpOutOfBlock := 0;
  LastJumpToNextBlockIteration := 0;
  GetNextToken;
  JumpToLoopStatement := 0;
  With FOR_Variable do
    begin
      ExpectAssignableVariableReferenceExceptProcedureOrFunction;
      If TypeDefPtr^.BaseType < btInteger then Error (Invalid_FOR_ControlVariable);
      If (LocationData.Flags * [segSS, segDS] = []) or (ofsDI in LocationData.Flags) then Error (Invalid_FOR_ControlVariable);
    end;
  ExpectTokenAndGetNext (Token_Assignment);
  Expect_FOR_LoopLimit (StartValue);
  Case Token of
    Token_TO: ForStatementData := ForStatementData_TO;
    Token_DOWNTO: ForStatementData := ForStatementData_DOWNTO;
    else Error (TO_Or_DOWNTO_Expected);
  end;
  GetNextToken;
  Expect_FOR_LoopLimit (EndValue);
  If not ((StartValue.Location = elConstant) and (EndValue.Location = elConstant)) then
    begin
      With StartValue do
        begin
          Calculate;
          LoadExpressionToRegisters (urDX_AX);
          EndIntermediateCodeSubroutine;
        end;
      With EndValue do If Location <> elConstant then
        begin
          StoreIntegerToStackFrame;
          UsedRegisters := [];
        end;
    end;
  ExpectTokenAndGetNext (Token_DO);
  LoopStatement.ProcessStatement;
  NoLoopComparison := StartValue;
  NoLoopComparison.IntermediateCodeOffset := 0;
  Compare_FOR_Variable (ForStatementData.NoLoopComparisonOperation, NoLoopComparison, EndValue);
  LoopRepeatComparison := FOR_Variable;
  LoopRepeatComparison.IntermediateCodeOffset := 0;
  Compare_FOR_Variable (Calc_IsNotEqual, LoopRepeatComparison, EndValue);
  TempStartValue := StartValue;
  TempStartValue.IntermediateCodeOffset := 0;
  GenerateAssignmentIntermediateCode (FOR_Variable, TempStartValue);
  If NoLoopComparison.Location = elConstant then
    begin
      If NoLoopComparison.Value.Byte = 0 then NoLoopComparison.Value.LastJumpToTrue := 0
        else begin
               LastJumpOutOfBlock := SavedLastJumpOutOfBlock;
               LastJumpToNextBlockIteration := SavedLastJumpToNextBlockIteration;
               StatementCode := 0;
               Exit;
             end;
    end;
  StoreCode_icGoSub (EndValue.IntermediateCodeOffset);
  StoreCode_icGoSub (StartValue.IntermediateCodeOffset);
  StoreCode_icGoSub (NoLoopComparison.IntermediateCodeOffset);
  StoreCode_icGoSub (FOR_Variable.IntermediateCodeOffset);
  GenerateCodeForNearJump (JumpToLoopStatement, JMP_ShortDirect);
  GenerateLabelAndSetJumpsToIt (LoopRepeatComparison.Value.LastJumpToTrue);
  Next_FOR_VariableValue;
  GenerateLabelAndSetJumpsToIt (JumpToLoopStatement);
  StoreCode_icGoSub (LoopStatement.StatementCode);
  GenerateLabelAndSetJumpsToIt (LastJumpToNextBlockIteration);
  StoreCode_icGoSub (LoopRepeatComparison.IntermediateCodeOffset);
  JoinJumpsOfBothExpressions (LastJumpOutOfBlock, NoLoopComparison.Value.LastJumpToTrue);
  GenerateLabelAndSetJumpsToIt (LastJumpOutOfBlock);
  StatementCode := EndSubroutine;
  LastJumpOutOfBlock := SavedLastJumpOutOfBlock;
  LastJumpToNextBlockIteration := SavedLastJumpToNextBlockIteration;
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
Procedure TStatement.Process_FOR_Statement;
Type TForStatementData = Record
                           NoLoopComparisonOperation: TCalcOperation;
                           IntegerOpCode,
                           LongIntOpCode_Lo,
                           LongIntOpCode_Hi: Byte;
                         end;

Const ForStatementData_TO: TForStatementData     = (NoLoopComparisonOperation: Calc_IsGreater;
                                                    IntegerOpCode: $00;
                                                    LongIntOpCode_Lo: mod_ADD_rm;
                                                    LongIntOpCode_Hi: mod_ADC_rm);
      ForStatementData_DOWNTO: TForStatementData = (NoLoopComparisonOperation: Calc_IsLower;
                                                    IntegerOpCode: $08;
                                                    LongIntOpCode_Lo: mod_SUB_rm;
                                                    LongIntOpCode_Hi: mod_SBB_rm);

Var SavedLastJumpOutOfBlock, SavedLastJumpToNextBlockIteration: Word;
    JumpToLoopStatement: Word;
    ForStatementData: TForStatementData;
    FOR_Variable, StartValue, EndValue, LoopRepeatComparison, NoLoopComparison, TempStartValue: TExpression;
    LoopStatement: TStatement;

  Procedure Expect_FOR_LoopLimit (Var Expression: TExpression);
  begin
    With Expression do
      begin
        ExpectOrdinalExpression;
        CheckTypeCompatibility (FOR_Variable.TypeDefPtr);
        CheckRange (FOR_Variable.TypeDefPtr);
      end;
  end;

  Procedure Compare_FOR_Variable (Operation: TCalcOperation; Var LeftExpression: TExpression; RightExpression: TExpression);
  begin
    RightExpression.IntermediateCodeOffset := 0;
    CalculateOperation (Operation, LeftExpression, RightExpression);
    With LeftExpression do If Location <> elConstant then
      begin
        Calculate;
        GenerateCodeForNearJump (Value.LastJumpToTrue, LocationData.JumpIfTrueOpCode);
        GenerateLabelAndSetJumpsToIt (Value.LastJumpToFalse);
        EndIntermediateCodeSubroutine
      end;
  end;

  Procedure Next_FOR_VariableValue;
  begin
    With FOR_Variable do
      If it32Bit in DataType then
        begin
          GenerateArithmeticInstructionWithImmediateValue (1, ForStatementData.LongIntOpCode_Lo);
          Inc (FOR_Variable.Value.Offset, 2);
          GenerateArithmeticInstructionWithImmediateValue (0, ForStatementData.LongIntOpCode_Hi);
          Dec (Value.Offset, 2);
        end else GenerateInstruction_8_16_bit ($FE, ForStatementData.IntegerOpCode);
  end;

begin
  SavedLastJumpOutOfBlock := LastJumpOutOfBlock;
  SavedLastJumpToNextBlockIteration := LastJumpToNextBlockIteration;
  LastJumpOutOfBlock := 0;
  LastJumpToNextBlockIteration := 0;
  GetNextToken;
  JumpToLoopStatement := 0;
  With FOR_Variable do
    begin
      ExpectAssignableVariableReferenceExceptProcedureOrFunction;
      If TypeDefPtr^.BaseType < btInteger then Error (Invalid_FOR_ControlVariable);
      If (LocationData.Flags * [segSS, segDS] = []) or (ofsDI in LocationData.Flags) then Error (Invalid_FOR_ControlVariable);
    end;
  ExpectTokenAndGetNext (Token_Assignment);
  Expect_FOR_LoopLimit (StartValue);
  Case Token of
    Token_TO: ForStatementData := ForStatementData_TO;
    Token_DOWNTO: ForStatementData := ForStatementData_DOWNTO;
    else Error (TO_Or_DOWNTO_Expected);
  end;
  GetNextToken;
  Expect_FOR_LoopLimit (EndValue);
  If not ((StartValue.Location = elConstant) and (EndValue.Location = elConstant)) then
    begin
      With StartValue do
        begin
          Calculate;
          LoadExpressionToRegisters (urDX_AX);
          EndIntermediateCodeSubroutine;
        end;
      With EndValue do If Location <> elConstant then
        begin
          StoreIntegerToStackFrame;
          UsedRegisters := [];
        end;
    end;
  ExpectTokenAndGetNext (Token_DO);
  LoopStatement.ProcessStatement;
  NoLoopComparison := StartValue;
  NoLoopComparison.IntermediateCodeOffset := 0;
  Compare_FOR_Variable (ForStatementData.NoLoopComparisonOperation, NoLoopComparison, EndValue);
  LoopRepeatComparison := FOR_Variable;
  LoopRepeatComparison.IntermediateCodeOffset := 0;
  Compare_FOR_Variable (Calc_IsNotEqual, LoopRepeatComparison, EndValue);
  TempStartValue := StartValue;
  TempStartValue.IntermediateCodeOffset := 0;
  GenerateAssignmentIntermediateCode (FOR_Variable, TempStartValue);
  If NoLoopComparison.Location = elConstant then
    begin
      If NoLoopComparison.Value.Byte = 0 then NoLoopComparison.Value.LastJumpToTrue := 0
        else begin
               LastJumpOutOfBlock := SavedLastJumpOutOfBlock;
               LastJumpToNextBlockIteration := SavedLastJumpToNextBlockIteration;
               StatementCode := 0;
               Exit;
             end;
    end;
  StoreCode_icGoSub (EndValue.IntermediateCodeOffset);
  StoreCode_icGoSub (StartValue.IntermediateCodeOffset);
  StoreCode_icGoSub (NoLoopComparison.IntermediateCodeOffset);
  StoreCode_icGoSub (FOR_Variable.IntermediateCodeOffset);
  GenerateCodeForNearJump (JumpToLoopStatement, JMP_ShortDirect);
  GenerateLabelAndSetJumpsToIt (LoopRepeatComparison.Value.LastJumpToTrue);
  Next_FOR_VariableValue;
  GenerateLabelAndSetJumpsToIt (JumpToLoopStatement);
  StoreCode_icGoSub (LoopStatement.StatementCode);
  GenerateLabelAndSetJumpsToIt (LastJumpToNextBlockIteration);
  StoreCode_icGoSub (LoopRepeatComparison.IntermediateCodeOffset);
  JoinJumpsOfBothExpressions (LastJumpOutOfBlock, NoLoopComparison.Value.LastJumpToTrue);
  GenerateLabelAndSetJumpsToIt (LastJumpOutOfBlock);
  StatementCode := EndSubroutine;
  LastJumpOutOfBlock := SavedLastJumpOutOfBlock;
  LastJumpToNextBlockIteration := SavedLastJumpToNextBlockIteration;
end;
```

### Code Block 2

```pascal
Procedure TStatement.Process_FOR_Statement;
Type TForStatementData = Record
                           NoLoopComparisonOperation: TCalcOperation;
                           IntegerOpCode,
                           LongIntOpCode_Lo,
                           LongIntOpCode_Hi: Byte;
                         end;

Const ForStatementData_TO: TForStatementData     = (NoLoopComparisonOperation: Calc_IsGreater;
                                                    IntegerOpCode: $00;
                                                    LongIntOpCode_Lo: mod_ADD_rm;
                                                    LongIntOpCode_Hi: mod_ADC_rm);
      ForStatementData_DOWNTO: TForStatementData = (NoLoopComparisonOperation: Calc_IsLower;
                                                    IntegerOpCode: $08;
                                                    LongIntOpCode_Lo: mod_SUB_rm;
                                                    LongIntOpCode_Hi: mod_SBB_rm);

Var SavedLastJumpOutOfBlock, SavedLastJumpToNextBlockIteration: Word;
    JumpToLoopStatement: Word;
    ForStatementData: TForStatementData;
    FOR_Variable, StartValue, EndValue, LoopRepeatComparison, NoLoopComparison, TempStartValue: TExpression;
    LoopStatement: TStatement;

  Procedure Expect_FOR_LoopLimit (Var Expression: TExpression);
  begin
    With Expression do
      begin
        ExpectOrdinalExpression;
        CheckTypeCompatibility (FOR_Variable.TypeDefPtr);
        CheckRange (FOR_Variable.TypeDefPtr);
      end;
  end;

  Procedure Compare_FOR_Variable (Operation: TCalcOperation; Var LeftExpression: TExpression; RightExpression: TExpression);
  begin
    RightExpression.IntermediateCodeOffset := 0;
    CalculateOperation (Operation, LeftExpression, RightExpression);
    With LeftExpression do If Location <> elConstant then
      begin
        Calculate;
        GenerateCodeForNearJump (Value.LastJumpToTrue, LocationData.JumpIfTrueOpCode);
        GenerateLabelAndSetJumpsToIt (Value.LastJumpToFalse);
        EndIntermediateCodeSubroutine
      end;
  end;

  Procedure Next_FOR_VariableValue;
  begin
    With FOR_Variable do
      If it32Bit in DataType then
        begin
          GenerateArithmeticInstructionWithImmediateValue (1, ForStatementData.LongIntOpCode_Lo);
          Inc (FOR_Variable.Value.Offset, 2);
          GenerateArithmeticInstructionWithImmediateValue (0, ForStatementData.LongIntOpCode_Hi);
          Dec (Value.Offset, 2);
        end else GenerateInstruction_8_16_bit ($FE, ForStatementData.IntegerOpCode);
  end;

begin
  SavedLastJumpOutOfBlock := LastJumpOutOfBlock;
  SavedLastJumpToNextBlockIteration := LastJumpToNextBlockIteration;
  LastJumpOutOfBlock := 0;
  LastJumpToNextBlockIteration := 0;
  GetNextToken;
  JumpToLoopStatement := 0;
  With FOR_Variable do
    begin
      ExpectAssignableVariableReferenceExceptProcedureOrFunction;
      If TypeDefPtr^.BaseType < btInteger then Error (Invalid_FOR_ControlVariable);
      If (LocationData.Flags * [segSS, segDS] = []) or (ofsDI in LocationData.Flags) then Error (Invalid_FOR_ControlVariable);
    end;
  ExpectTokenAndGetNext (Token_Assignment);
  Expect_FOR_LoopLimit (StartValue);
  Case Token of
    Token_TO: ForStatementData := ForStatementData_TO;
    Token_DOWNTO: ForStatementData := ForStatementData_DOWNTO;
    else Error (TO_Or_DOWNTO_Expected);
  end;
  GetNextToken;
  Expect_FOR_LoopLimit (EndValue);
  If not ((StartValue.Location = elConstant) and (EndValue.Location = elConstant)) then
    begin
      With StartValue do
        begin
          Calculate;
          LoadExpressionToRegisters (urDX_AX);
          EndIntermediateCodeSubroutine;
        end;
      With EndValue do If Location <> elConstant then
        begin
          StoreIntegerToStackFrame;
          UsedRegisters := [];
        end;
    end;
  ExpectTokenAndGetNext (Token_DO);
  LoopStatement.ProcessStatement;
  NoLoopComparison := StartValue;
  NoLoopComparison.IntermediateCodeOffset := 0;
  Compare_FOR_Variable (ForStatementData.NoLoopComparisonOperation, NoLoopComparison, EndValue);
  LoopRepeatComparison := FOR_Variable;
  LoopRepeatComparison.IntermediateCodeOffset := 0;
  Compare_FOR_Variable (Calc_IsNotEqual, LoopRepeatComparison, EndValue);
  TempStartValue := StartValue;
  TempStartValue.IntermediateCodeOffset := 0;
  GenerateAssignmentIntermediateCode (FOR_Variable, TempStartValue);
  If NoLoopComparison.Location = elConstant then
    begin
      If NoLoopComparison.Value.Byte = 0 then NoLoopComparison.Value.LastJumpToTrue := 0
        else begin
               LastJumpOutOfBlock := SavedLastJumpOutOfBlock;
               LastJumpToNextBlockIteration := SavedLastJumpToNextBlockIteration;
               StatementCode := 0;
               Exit;
             end;
    end;
  StoreCode_icGoSub (EndValue.IntermediateCodeOffset);
  StoreCode_icGoSub (StartValue.IntermediateCodeOffset);
  StoreCode_icGoSub (NoLoopComparison.IntermediateCodeOffset);
  StoreCode_icGoSub (FOR_Variable.IntermediateCodeOffset);
  GenerateCodeForNearJump (JumpToLoopStatement, JMP_ShortDirect);
  GenerateLabelAndSetJumpsToIt (LoopRepeatComparison.Value.LastJumpToTrue);
  Next_FOR_VariableValue;
  GenerateLabelAndSetJumpsToIt (JumpToLoopStatement);
  StoreCode_icGoSub (LoopStatement.StatementCode);
  GenerateLabelAndSetJumpsToIt (LastJumpToNextBlockIteration);
  StoreCode_icGoSub (LoopRepeatComparison.IntermediateCodeOffset);
  JoinJumpsOfBothExpressions (LastJumpOutOfBlock, NoLoopComparison.Value.LastJumpToTrue);
  GenerateLabelAndSetJumpsToIt (LastJumpOutOfBlock);
  StatementCode := EndSubroutine;
  LastJumpOutOfBlock := SavedLastJumpOutOfBlock;
  LastJumpToNextBlockIteration := SavedLastJumpToNextBlockIteration;
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
- https://turbopascal.org/for-statement
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

