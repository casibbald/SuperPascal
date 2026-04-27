# Real Operations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/real-operations

---

## Table of Contents

- Real Operations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Real Operations
For
Real
operations code is always generated. This procedure generates code for operations on
Real
values. If 80×87 instructions are enabled, it converts both expressions to
Extended
and generates FPU code otherwise it generates code to call
system function
.
Procedure GenerateCodeForRealOperations;
  begin
    If Instructions80x87 in StatementCompilerSwitches then
      begin
        LeftExpression.ConvertRealToExtended;
        RightExpression.ConvertRealToExtended;
        GenerateCodeForExtendedOperations;
        Exit;
      end;
    Case Operation of
      Calc_IsEqual..Calc_IsLowerOrEqual,
      Calc_Add,
      Calc_Multiply: If (RightExpression.UsedRegisters <> []) and (LeftExpression.UsedRegisters = []) then
                       ExchangeLeftAndRightExpression;
    end;
    Case RightExpression.UsedRegisters = [] of
      True: begin
              LeftExpression.Calculate;
              LeftExpression.LoadRealExpressionToRegisterSet (rAX_BX_DX);
              RightExpression.Calculate;
              RightExpression.LoadRealExpressionToRegisterSet (rCX_SI_DI);
            end;
      else begin
             RightExpression.Calculate;
             RightExpression.LoadRealExpressionToRegisterSet (rAX_BX_DX);
             GenerateInstruction_TwoBytes (PUSH_DX, PUSH_BX);
             GenerateInstruction_Byte (PUSH_AX);
             LeftExpression.Calculate;
             LeftExpression.LoadRealExpressionToRegisterSet (rAX_BX_DX);
             GenerateInstruction_Byte (POP_CX);
             GenerateInstruction_TwoBytes (POP_SI, POP_DI);
           end;
    end;
    Case Operation of
      Calc_Add:      GenerateInstruction_CALL_FAR (SysProc_RAdd);
      Calc_Subtract: GenerateInstruction_CALL_FAR (SysProc_RSub);
      Calc_Multiply: GenerateInstruction_CALL_FAR (SysProc_RMul);
      Calc_Divide:   GenerateInstruction_CALL_FAR (SysProc_RDiv);
      else begin
             GenerateInstruction_CALL_FAR (SysProc_RCmp);
             LeftExpression.EndIntermediateCodeSubroutine;
             LeftExpression.SetExpressionToBooleanJump (UnsignedConditionalJumpOpCode [Operation]);
             Exit;
           end;
    end;
    LeftExpression.EndIntermediateCodeSubroutine;
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
  Procedure GenerateCodeForRealOperations;
  begin
    If Instructions80x87 in StatementCompilerSwitches then
      begin
        LeftExpression.ConvertRealToExtended;
        RightExpression.ConvertRealToExtended;
        GenerateCodeForExtendedOperations;
        Exit;
      end;
    Case Operation of
      Calc_IsEqual..Calc_IsLowerOrEqual,
      Calc_Add,
      Calc_Multiply: If (RightExpression.UsedRegisters <> []) and (LeftExpression.UsedRegisters = []) then
                       ExchangeLeftAndRightExpression;
    end;
    Case RightExpression.UsedRegisters = [] of
      True: begin
              LeftExpression.Calculate;
              LeftExpression.LoadRealExpressionToRegisterSet (rAX_BX_DX);
              RightExpression.Calculate;
              RightExpression.LoadRealExpressionToRegisterSet (rCX_SI_DI);
            end;
      else begin
             RightExpression.Calculate;
             RightExpression.LoadRealExpressionToRegisterSet (rAX_BX_DX);
             GenerateInstruction_TwoBytes (PUSH_DX, PUSH_BX);
             GenerateInstruction_Byte (PUSH_AX);
             LeftExpression.Calculate;
             LeftExpression.LoadRealExpressionToRegisterSet (rAX_BX_DX);
             GenerateInstruction_Byte (POP_CX);
             GenerateInstruction_TwoBytes (POP_SI, POP_DI);
           end;
    end;
    Case Operation of
      Calc_Add:      GenerateInstruction_CALL_FAR (SysProc_RAdd);
      Calc_Subtract: GenerateInstruction_CALL_FAR (SysProc_RSub);
      Calc_Multiply: GenerateInstruction_CALL_FAR (SysProc_RMul);
      Calc_Divide:   GenerateInstruction_CALL_FAR (SysProc_RDiv);
      else begin
             GenerateInstruction_CALL_FAR (SysProc_RCmp);
             LeftExpression.EndIntermediateCodeSubroutine;
             LeftExpression.SetExpressionToBooleanJump (UnsignedConditionalJumpOpCode [Operation]);
             Exit;
           end;
    end;
    LeftExpression.EndIntermediateCodeSubroutine;
  end;
```

### Code Block 2

```pascal
  Procedure GenerateCodeForRealOperations;
  begin
    If Instructions80x87 in StatementCompilerSwitches then
      begin
        LeftExpression.ConvertRealToExtended;
        RightExpression.ConvertRealToExtended;
        GenerateCodeForExtendedOperations;
        Exit;
      end;
    Case Operation of
      Calc_IsEqual..Calc_IsLowerOrEqual,
      Calc_Add,
      Calc_Multiply: If (RightExpression.UsedRegisters <> []) and (LeftExpression.UsedRegisters = []) then
                       ExchangeLeftAndRightExpression;
    end;
    Case RightExpression.UsedRegisters = [] of
      True: begin
              LeftExpression.Calculate;
              LeftExpression.LoadRealExpressionToRegisterSet (rAX_BX_DX);
              RightExpression.Calculate;
              RightExpression.LoadRealExpressionToRegisterSet (rCX_SI_DI);
            end;
      else begin
             RightExpression.Calculate;
             RightExpression.LoadRealExpressionToRegisterSet (rAX_BX_DX);
             GenerateInstruction_TwoBytes (PUSH_DX, PUSH_BX);
             GenerateInstruction_Byte (PUSH_AX);
             LeftExpression.Calculate;
             LeftExpression.LoadRealExpressionToRegisterSet (rAX_BX_DX);
             GenerateInstruction_Byte (POP_CX);
             GenerateInstruction_TwoBytes (POP_SI, POP_DI);
           end;
    end;
    Case Operation of
      Calc_Add:      GenerateInstruction_CALL_FAR (SysProc_RAdd);
      Calc_Subtract: GenerateInstruction_CALL_FAR (SysProc_RSub);
      Calc_Multiply: GenerateInstruction_CALL_FAR (SysProc_RMul);
      Calc_Divide:   GenerateInstruction_CALL_FAR (SysProc_RDiv);
      else begin
             GenerateInstruction_CALL_FAR (SysProc_RCmp);
             LeftExpression.EndIntermediateCodeSubroutine;
             LeftExpression.SetExpressionToBooleanJump (UnsignedConditionalJumpOpCode [Operation]);
             Exit;
           end;
    end;
    LeftExpression.EndIntermediateCodeSubroutine;
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
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/real-operations
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

