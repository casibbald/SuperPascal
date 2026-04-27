# Extended Operations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/extended-operations

---

## Table of Contents

- Extended Operations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Extended Operations
Constant
Extended
values are always stored in expression
Value
, while for non-constant expressions numeric constants are stored in code block and FPU instructions are used for calculation. This procedure calculates operation between two constant
Extended
values.
Procedure ConstantExtendedOperations;
  Var BooleanValue: LongBool;
  begin
    Case Operation of
      Calc_Add,
      Calc_Subtract,
      Calc_Multiply,
      Calc_Divide: begin
                     Case Operation of
                       Calc_Add:      LeftExpression.Value.Extended :=
                                        LeftExpression.Value.Extended + RightExpression.Value.Extended;
                       Calc_Subtract: LeftExpression.Value.Extended :=
                                        LeftExpression.Value.Extended - RightExpression.Value.Extended;
                       Calc_Multiply: LeftExpression.Value.Extended :=
                                        LeftExpression.Value.Extended * RightExpression.Value.Extended;
                       Calc_Divide:   LeftExpression.Value.Extended :=
                                        LeftExpression.Value.Extended / RightExpression.Value.Extended;
                     end;
                     If LeftExpression.Value.W18 and $7FFF = $7FFF then Error (InvalidFloatingPointOperation);
                   end;
      else begin
             Case Operation of
               Calc_IsEqual:          BooleanValue := LeftExpression.Value.Extended  = RightExpression.Value.Extended;
               Calc_IsNotEqual:       BooleanValue := LeftExpression.Value.Extended <> RightExpression.Value.Extended;
               Calc_IsGreater:        BooleanValue := LeftExpression.Value.Extended  > RightExpression.Value.Extended;
               Calc_IsLower:          BooleanValue := LeftExpression.Value.Extended  < RightExpression.Value.Extended;
               Calc_IsGreaterOrEqual: BooleanValue := LeftExpression.Value.Extended >= RightExpression.Value.Extended;
               Calc_IsLowerOrEqual:   BooleanValue := LeftExpression.Value.Extended <= RightExpression.Value.Extended;
             end;
             SetLeftExpressionToConstantBooleanValue (BooleanValue);
           end;
    end;
  end;
This procedure generates code for operations on Extended expressions. FPU instructions are used.
Procedure GenerateCodeForExtendedOperations;
  Var OpCode: Word;
      StackFrameOffset: Word;

    Procedure ExchangeExpressionsAndInvertOperation;
    begin
      ExchangeLeftAndRightExpression;
      If OpCode and $2000 <> 0 then OpCode := OpCode xor $0800;
    end;

  begin
    LeftExpression.ConvertToLowestFPTypeAndStoreToCodeBlock;
    RightExpression.ConvertToLowestFPTypeAndStoreToCodeBlock;
    Case Operation of
      Calc_Add:      OpCode := FADDP;
      Calc_Subtract: OpCode := FSUBRP;
      Calc_Multiply: OpCode := FMULP;
      Calc_Divide:   OpCode := FDIVRP;
      Calc_IsEqual..Calc_IsLowerOrEqual: OpCode := FCOMPP;
    end;
    If LeftExpression.Location <> elStackFrame then
      begin
        If RightExpression.Location = elStackFrame then ExchangeExpressionsAndInvertOperation else
          If (fpExtended in LeftExpression.DataType) and not (fpExtended in RightExpression.DataType) then
            ExchangeExpressionsAndInvertOperation;
      end;
    If fpExtended in RightExpression.DataType then
      begin
        ExchangeExpressionsAndInvertOperation;
        LeftExpression.LoadExpressionToFPU;
        RightExpression.LoadExpressionToFPU;
        CalculateBothExpressions (RightExpression, LeftExpression);
        ModifyAndSroreFPUInstructionForFixup (OpCode);
      end else begin
                 LeftExpression.LoadExpressionToFPU;
                 CalculateBothExpressions (LeftExpression, RightExpression);
                 RightExpression.GenerateFPUInstructionWithExpressionInMemory (
                                                  ESC or Byte (RightExpression.DataType), Hi (OpCode) xor $C1);
               end;

    Case Operation of
      Calc_Add,
      Calc_Subtract,
      Calc_Multiply,
      Calc_Divide: begin
                     LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
                     LeftExpression.EndIntermediateCodeSubroutine;
                     Exit;
                   end;
    end;
    StackFrameOffset := CreateTempStackFrameSpace (2);
    GenerateFPUInstructionForStackFrameDisplacement ($3EDD, StackFrameOffset);
    GenerateInstruction_TwoBytes ($CD, $3D);
    GenerateInstructionForStackFrameDisplacement ($8A, $26, StackFrameOffset + 1);
    GenerateInstruction_Byte ($9E);
    LeftExpression.EndIntermediateCodeSubroutine;
    LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters + [urAX];
    LeftExpression.SetExpressionToBooleanJump (UnsignedConditionalJumpOpCode [Operation]);
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
  Procedure ConstantExtendedOperations;
  Var BooleanValue: LongBool;
  begin
    Case Operation of
      Calc_Add,
      Calc_Subtract,
      Calc_Multiply,
      Calc_Divide: begin
                     Case Operation of
                       Calc_Add:      LeftExpression.Value.Extended :=
                                        LeftExpression.Value.Extended + RightExpression.Value.Extended;
                       Calc_Subtract: LeftExpression.Value.Extended :=
                                        LeftExpression.Value.Extended - RightExpression.Value.Extended;
                       Calc_Multiply: LeftExpression.Value.Extended :=
                                        LeftExpression.Value.Extended * RightExpression.Value.Extended;
                       Calc_Divide:   LeftExpression.Value.Extended :=
                                        LeftExpression.Value.Extended / RightExpression.Value.Extended;
                     end;
                     If LeftExpression.Value.W18 and $7FFF = $7FFF then Error (InvalidFloatingPointOperation);
                   end;
      else begin
             Case Operation of
               Calc_IsEqual:          BooleanValue := LeftExpression.Value.Extended  = RightExpression.Value.Extended;
               Calc_IsNotEqual:       BooleanValue := LeftExpression.Value.Extended <> RightExpression.Value.Extended;
               Calc_IsGreater:        BooleanValue := LeftExpression.Value.Extended  > RightExpression.Value.Extended;
               Calc_IsLower:          BooleanValue := LeftExpression.Value.Extended  < RightExpression.Value.Extended;
               Calc_IsGreaterOrEqual: BooleanValue := LeftExpression.Value.Extended >= RightExpression.Value.Extended;
               Calc_IsLowerOrEqual:   BooleanValue := LeftExpression.Value.Extended <= RightExpression.Value.Extended;
             end;
             SetLeftExpressionToConstantBooleanValue (BooleanValue);
           end;
    end;
  end;
```

### Code Block 2

```pascal
  Procedure GenerateCodeForExtendedOperations;
  Var OpCode: Word;
      StackFrameOffset: Word;

    Procedure ExchangeExpressionsAndInvertOperation;
    begin
      ExchangeLeftAndRightExpression;
      If OpCode and $2000 <> 0 then OpCode := OpCode xor $0800;
    end;

  begin
    LeftExpression.ConvertToLowestFPTypeAndStoreToCodeBlock;
    RightExpression.ConvertToLowestFPTypeAndStoreToCodeBlock;
    Case Operation of
      Calc_Add:      OpCode := FADDP;
      Calc_Subtract: OpCode := FSUBRP;
      Calc_Multiply: OpCode := FMULP;
      Calc_Divide:   OpCode := FDIVRP;
      Calc_IsEqual..Calc_IsLowerOrEqual: OpCode := FCOMPP;
    end;
    If LeftExpression.Location <> elStackFrame then
      begin
        If RightExpression.Location = elStackFrame then ExchangeExpressionsAndInvertOperation else
          If (fpExtended in LeftExpression.DataType) and not (fpExtended in RightExpression.DataType) then
            ExchangeExpressionsAndInvertOperation;
      end;
    If fpExtended in RightExpression.DataType then
      begin
        ExchangeExpressionsAndInvertOperation;
        LeftExpression.LoadExpressionToFPU;
        RightExpression.LoadExpressionToFPU;
        CalculateBothExpressions (RightExpression, LeftExpression);
        ModifyAndSroreFPUInstructionForFixup (OpCode);
      end else begin
                 LeftExpression.LoadExpressionToFPU;
                 CalculateBothExpressions (LeftExpression, RightExpression);
                 RightExpression.GenerateFPUInstructionWithExpressionInMemory (
                                                  ESC or Byte (RightExpression.DataType), Hi (OpCode) xor $C1);
               end;

    Case Operation of
      Calc_Add,
      Calc_Subtract,
      Calc_Multiply,
      Calc_Divide: begin
                     LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
                     LeftExpression.EndIntermediateCodeSubroutine;
                     Exit;
                   end;
    end;
    StackFrameOffset := CreateTempStackFrameSpace (2);
    GenerateFPUInstructionForStackFrameDisplacement ($3EDD, StackFrameOffset);
    GenerateInstruction_TwoBytes ($CD, $3D);
    GenerateInstructionForStackFrameDisplacement ($8A, $26, StackFrameOffset + 1);
    GenerateInstruction_Byte ($9E);
    LeftExpression.EndIntermediateCodeSubroutine;
    LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters + [urAX];
    LeftExpression.SetExpressionToBooleanJump (UnsignedConditionalJumpOpCode [Operation]);
  end;
```

### Code Block 3

```pascal
  Procedure ConstantExtendedOperations;
  Var BooleanValue: LongBool;
  begin
    Case Operation of
      Calc_Add,
      Calc_Subtract,
      Calc_Multiply,
      Calc_Divide: begin
                     Case Operation of
                       Calc_Add:      LeftExpression.Value.Extended :=
                                        LeftExpression.Value.Extended + RightExpression.Value.Extended;
                       Calc_Subtract: LeftExpression.Value.Extended :=
                                        LeftExpression.Value.Extended - RightExpression.Value.Extended;
                       Calc_Multiply: LeftExpression.Value.Extended :=
                                        LeftExpression.Value.Extended * RightExpression.Value.Extended;
                       Calc_Divide:   LeftExpression.Value.Extended :=
                                        LeftExpression.Value.Extended / RightExpression.Value.Extended;
                     end;
                     If LeftExpression.Value.W18 and $7FFF = $7FFF then Error (InvalidFloatingPointOperation);
                   end;
      else begin
             Case Operation of
               Calc_IsEqual:          BooleanValue := LeftExpression.Value.Extended  = RightExpression.Value.Extended;
               Calc_IsNotEqual:       BooleanValue := LeftExpression.Value.Extended <> RightExpression.Value.Extended;
               Calc_IsGreater:        BooleanValue := LeftExpression.Value.Extended  > RightExpression.Value.Extended;
               Calc_IsLower:          BooleanValue := LeftExpression.Value.Extended  < RightExpression.Value.Extended;
               Calc_IsGreaterOrEqual: BooleanValue := LeftExpression.Value.Extended >= RightExpression.Value.Extended;
               Calc_IsLowerOrEqual:   BooleanValue := LeftExpression.Value.Extended <= RightExpression.Value.Extended;
             end;
             SetLeftExpressionToConstantBooleanValue (BooleanValue);
           end;
    end;
  end;
```

### Code Block 4

```pascal
  Procedure GenerateCodeForExtendedOperations;
  Var OpCode: Word;
      StackFrameOffset: Word;

    Procedure ExchangeExpressionsAndInvertOperation;
    begin
      ExchangeLeftAndRightExpression;
      If OpCode and $2000 <> 0 then OpCode := OpCode xor $0800;
    end;

  begin
    LeftExpression.ConvertToLowestFPTypeAndStoreToCodeBlock;
    RightExpression.ConvertToLowestFPTypeAndStoreToCodeBlock;
    Case Operation of
      Calc_Add:      OpCode := FADDP;
      Calc_Subtract: OpCode := FSUBRP;
      Calc_Multiply: OpCode := FMULP;
      Calc_Divide:   OpCode := FDIVRP;
      Calc_IsEqual..Calc_IsLowerOrEqual: OpCode := FCOMPP;
    end;
    If LeftExpression.Location <> elStackFrame then
      begin
        If RightExpression.Location = elStackFrame then ExchangeExpressionsAndInvertOperation else
          If (fpExtended in LeftExpression.DataType) and not (fpExtended in RightExpression.DataType) then
            ExchangeExpressionsAndInvertOperation;
      end;
    If fpExtended in RightExpression.DataType then
      begin
        ExchangeExpressionsAndInvertOperation;
        LeftExpression.LoadExpressionToFPU;
        RightExpression.LoadExpressionToFPU;
        CalculateBothExpressions (RightExpression, LeftExpression);
        ModifyAndSroreFPUInstructionForFixup (OpCode);
      end else begin
                 LeftExpression.LoadExpressionToFPU;
                 CalculateBothExpressions (LeftExpression, RightExpression);
                 RightExpression.GenerateFPUInstructionWithExpressionInMemory (
                                                  ESC or Byte (RightExpression.DataType), Hi (OpCode) xor $C1);
               end;

    Case Operation of
      Calc_Add,
      Calc_Subtract,
      Calc_Multiply,
      Calc_Divide: begin
                     LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
                     LeftExpression.EndIntermediateCodeSubroutine;
                     Exit;
                   end;
    end;
    StackFrameOffset := CreateTempStackFrameSpace (2);
    GenerateFPUInstructionForStackFrameDisplacement ($3EDD, StackFrameOffset);
    GenerateInstruction_TwoBytes ($CD, $3D);
    GenerateInstructionForStackFrameDisplacement ($8A, $26, StackFrameOffset + 1);
    GenerateInstruction_Byte ($9E);
    LeftExpression.EndIntermediateCodeSubroutine;
    LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters + [urAX];
    LeftExpression.SetExpressionToBooleanJump (UnsignedConditionalJumpOpCode [Operation]);
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
- https://turbopascal.org/extended-operations
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

