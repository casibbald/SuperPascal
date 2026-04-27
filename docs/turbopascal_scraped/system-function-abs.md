# System Function Abs - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-function-abs

---

## Table of Contents

- System Function Abs

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Function Abs
This procedure processes system function
Abs
. It simply converts negative numbers to positive.
Procedure Func_Abs; Far;
Var TempInteger: LongInt;
    Overflow: Boolean;
    CommonType: TIntegerTypeSet;
begin
  Expression^.ExpectIntegerOrFloatingPointExpressionInParentheses;
  With Expression^ do
    begin
      If TypeDefPtr^.BaseType = btInteger then
      begin
        If Location = elConstant then
          begin
            If Value.W12 and $8000 <> 0 then
              begin
                Overflow := False;
                TempInteger := 0 - Value.LongInt;
                Asm
                  JNO   @1
                  INC   Overflow
              @1:
                end;
                Expression^.CheckOrdinalOverflowAndStore (TempInteger, Overflow);
              end;
            Exit;
          end;
        SetLowestCommonIntegerType ([itSigned, it16Bit], DataType, CommonType);
        Expression^.ExtendInteger (CommonType);
        Expression^.Calculate;
        Expression^.LoadExpressionToRegisters (urAX);
        If it16Bit in DataType then
          begin
            GenerateInstruction_TwoBytes ($09, $D2);            { OR    DX, DX }
            GenerateInstruction_TwoBytes ($79, $07);            { JNS   $ + 9  }
            GenerateCode_NEG_LongInt;
          end else begin
                     GenerateInstruction_Byte ($99);            { CWD          }
                     GenerateInstruction_TwoBytes ($31, $D0);   { XOR   AX, DX }
                     GenerateInstruction_TwoBytes ($29, $D0);   { SUB   AX, DX }
                   end;
        GenerateOverflowCheckingCode (Expression^);
        Expression^.EndIntermediateCodeSubroutine;
        Exit;
      end;
      If TypeDefPtr^.BaseType = btExtended then
        begin
          If Location = elConstant then
            begin
              Value.As10Bytes [9] := Value.As10Bytes [9] and $7F;
              Exit;
            end;
          Expression^.ConvertRealToExtended;
          Expression^.LoadExpressionToFPU;
          Expression^.Calculate;
          ModifyAndSroreFPUInstructionForFixup ($E1D9);    { FABS }
          Expression^.EndIntermediateCodeSubroutine;
          Exit;
        end;
      If Instructions80x87 in StatementCompilerSwitches then
        begin
          Expression^.ConvertRealToExtended;
          Expression^.LoadExpressionToFPU;
          Expression^.Calculate;
          ModifyAndSroreFPUInstructionForFixup ($E1D9);    { FABS }
          Expression^.EndIntermediateCodeSubroutine;
          Exit;
        end;
      Expression^.Calculate;
      Expression^.LoadRealExpressionToRegisterSet (rAX_BX_DX);
      GenerateInstruction_TwoBytes ($80, $E6);
      GenerateInstruction_Byte ($7F);
      Expression^.EndIntermediateCodeSubroutine;
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
Procedure Func_Abs; Far;
Var TempInteger: LongInt;
    Overflow: Boolean;
    CommonType: TIntegerTypeSet;
begin
  Expression^.ExpectIntegerOrFloatingPointExpressionInParentheses;
  With Expression^ do
    begin
      If TypeDefPtr^.BaseType = btInteger then
      begin
        If Location = elConstant then
          begin
            If Value.W12 and $8000 <> 0 then
              begin
                Overflow := False;
                TempInteger := 0 - Value.LongInt;
                Asm
                  JNO   @1
                  INC   Overflow
              @1:
                end;
                Expression^.CheckOrdinalOverflowAndStore (TempInteger, Overflow);
              end;
            Exit;
          end;
        SetLowestCommonIntegerType ([itSigned, it16Bit], DataType, CommonType);
        Expression^.ExtendInteger (CommonType);
        Expression^.Calculate;
        Expression^.LoadExpressionToRegisters (urAX);
        If it16Bit in DataType then
          begin
            GenerateInstruction_TwoBytes ($09, $D2);            { OR    DX, DX }
            GenerateInstruction_TwoBytes ($79, $07);            { JNS   $ + 9  }
            GenerateCode_NEG_LongInt;
          end else begin
                     GenerateInstruction_Byte ($99);            { CWD          }
                     GenerateInstruction_TwoBytes ($31, $D0);   { XOR   AX, DX }
                     GenerateInstruction_TwoBytes ($29, $D0);   { SUB   AX, DX }
                   end;
        GenerateOverflowCheckingCode (Expression^);
        Expression^.EndIntermediateCodeSubroutine;
        Exit;
      end;
      If TypeDefPtr^.BaseType = btExtended then
        begin
          If Location = elConstant then
            begin
              Value.As10Bytes [9] := Value.As10Bytes [9] and $7F;
              Exit;
            end;
          Expression^.ConvertRealToExtended;
          Expression^.LoadExpressionToFPU;
          Expression^.Calculate;
          ModifyAndSroreFPUInstructionForFixup ($E1D9);    { FABS }
          Expression^.EndIntermediateCodeSubroutine;
          Exit;
        end;
      If Instructions80x87 in StatementCompilerSwitches then
        begin
          Expression^.ConvertRealToExtended;
          Expression^.LoadExpressionToFPU;
          Expression^.Calculate;
          ModifyAndSroreFPUInstructionForFixup ($E1D9);    { FABS }
          Expression^.EndIntermediateCodeSubroutine;
          Exit;
        end;
      Expression^.Calculate;
      Expression^.LoadRealExpressionToRegisterSet (rAX_BX_DX);
      GenerateInstruction_TwoBytes ($80, $E6);
      GenerateInstruction_Byte ($7F);
      Expression^.EndIntermediateCodeSubroutine;
    end;
end;
```

### Code Block 2

```pascal
Procedure Func_Abs; Far;
Var TempInteger: LongInt;
    Overflow: Boolean;
    CommonType: TIntegerTypeSet;
begin
  Expression^.ExpectIntegerOrFloatingPointExpressionInParentheses;
  With Expression^ do
    begin
      If TypeDefPtr^.BaseType = btInteger then
      begin
        If Location = elConstant then
          begin
            If Value.W12 and $8000 <> 0 then
              begin
                Overflow := False;
                TempInteger := 0 - Value.LongInt;
                Asm
                  JNO   @1
                  INC   Overflow
              @1:
                end;
                Expression^.CheckOrdinalOverflowAndStore (TempInteger, Overflow);
              end;
            Exit;
          end;
        SetLowestCommonIntegerType ([itSigned, it16Bit], DataType, CommonType);
        Expression^.ExtendInteger (CommonType);
        Expression^.Calculate;
        Expression^.LoadExpressionToRegisters (urAX);
        If it16Bit in DataType then
          begin
            GenerateInstruction_TwoBytes ($09, $D2);            { OR    DX, DX }
            GenerateInstruction_TwoBytes ($79, $07);            { JNS   $ + 9  }
            GenerateCode_NEG_LongInt;
          end else begin
                     GenerateInstruction_Byte ($99);            { CWD          }
                     GenerateInstruction_TwoBytes ($31, $D0);   { XOR   AX, DX }
                     GenerateInstruction_TwoBytes ($29, $D0);   { SUB   AX, DX }
                   end;
        GenerateOverflowCheckingCode (Expression^);
        Expression^.EndIntermediateCodeSubroutine;
        Exit;
      end;
      If TypeDefPtr^.BaseType = btExtended then
        begin
          If Location = elConstant then
            begin
              Value.As10Bytes [9] := Value.As10Bytes [9] and $7F;
              Exit;
            end;
          Expression^.ConvertRealToExtended;
          Expression^.LoadExpressionToFPU;
          Expression^.Calculate;
          ModifyAndSroreFPUInstructionForFixup ($E1D9);    { FABS }
          Expression^.EndIntermediateCodeSubroutine;
          Exit;
        end;
      If Instructions80x87 in StatementCompilerSwitches then
        begin
          Expression^.ConvertRealToExtended;
          Expression^.LoadExpressionToFPU;
          Expression^.Calculate;
          ModifyAndSroreFPUInstructionForFixup ($E1D9);    { FABS }
          Expression^.EndIntermediateCodeSubroutine;
          Exit;
        end;
      Expression^.Calculate;
      Expression^.LoadRealExpressionToRegisterSet (rAX_BX_DX);
      GenerateInstruction_TwoBytes ($80, $E6);
      GenerateInstruction_Byte ($7F);
      Expression^.EndIntermediateCodeSubroutine;
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
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/system-function-abs
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

