# Processing Unary Sign Operators - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-unary-sign-operators

---

## Table of Contents

- Processing Unary Sign Operators

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Unary Sign Operators
This procedure processes unary plus or minus before a factor. Plus is skipped while minus changes the sign if expression is integer, extended or real.
Procedure ProcessUnaryMinus_Plus;
  Var LastToken: TToken;
      Overflow: Boolean;
      TempInteger: LongInt;
      TempIntegerTypeSet: TIntegerTypeSet;
  begin
    LastToken := Token;
    CheckPlusMinusAndGetNextToken;
    ProcessFactor;
    If LastToken <> Token_Minus then Exit;
    Case TypeDefPtr^.BaseType of
      btInteger: begin
              If Location = elConstant then
                begin
                  Overflow := False;
                  TempInteger := 0 - Value.LongInt;
                  Asm
                    JNO   @1
                    INC   Overflow
                @1:
                  end;
                  CheckOrdinalOverflowAndStore (TempInteger, Overflow);
                  Exit;
                end;
              SetLowestCommonIntegerType ([itSigned, it16Bit], DataType, TempIntegerTypeSet);
              ExtendInteger (TempIntegerTypeSet);
              Calculate;
              LoadExpressionToRegisters (urAX);
              If not (it32Bit in DataType) then GenerateInstruction_TwoBytes ($F7, $D8) { NEG   AX  }
                else GenerateCode_NEG_LongInt;
              GenerateOverflowCheckingCode (Self);
              EndIntermediateCodeSubroutine;
            end;
      btExtended: begin
                    If Location <> elConstant then
                      begin
                        ConvertRealToExtended;
                        LoadExpressionToFPU;
                        Calculate;
                        ModifyAndSroreFPUInstructionForFixup ($E0D9);         { FCHS }
                        EndIntermediateCodeSubroutine;
                      end else Value.Extended := - Value.Extended;
                  end;
      btReal: begin
              If Instructions80x87 in StatementCompilerSwitches then
                begin
                  ConvertRealToExtended;
                  LoadExpressionToFPU;
                  Calculate;
                  ModifyAndSroreFPUInstructionForFixup ($E0D9);         { FCHS }
                  EndIntermediateCodeSubroutine;
                  Exit;
                end;
              Calculate;
              LoadRealExpressionToRegisterSet (rAX_BX_DX);
              GenerateInstruction_TwoBytes ($08, $C0);              { OR     AL, AL }
              GenerateInstruction_TwoBytes ($74, $03);              { JE     $ + 5  }
              GenerateInstruction_TwoBytes ($80, $F6);              { XOR    DH, 80 }
              GenerateInstruction_Byte ($80);
              EndIntermediateCodeSubroutine;
            end;
      else Error (OperandTypesDoNotMatchOperator);
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
  Procedure ProcessUnaryMinus_Plus;
  Var LastToken: TToken;
      Overflow: Boolean;
      TempInteger: LongInt;
      TempIntegerTypeSet: TIntegerTypeSet;
  begin
    LastToken := Token;
    CheckPlusMinusAndGetNextToken;
    ProcessFactor;
    If LastToken <> Token_Minus then Exit;
    Case TypeDefPtr^.BaseType of
      btInteger: begin
              If Location = elConstant then
                begin
                  Overflow := False;
                  TempInteger := 0 - Value.LongInt;
                  Asm
                    JNO   @1
                    INC   Overflow
                @1:
                  end;
                  CheckOrdinalOverflowAndStore (TempInteger, Overflow);
                  Exit;
                end;
              SetLowestCommonIntegerType ([itSigned, it16Bit], DataType, TempIntegerTypeSet);
              ExtendInteger (TempIntegerTypeSet);
              Calculate;
              LoadExpressionToRegisters (urAX);
              If not (it32Bit in DataType) then GenerateInstruction_TwoBytes ($F7, $D8) { NEG   AX  }
                else GenerateCode_NEG_LongInt;
              GenerateOverflowCheckingCode (Self);
              EndIntermediateCodeSubroutine;
            end;
      btExtended: begin
                    If Location <> elConstant then
                      begin
                        ConvertRealToExtended;
                        LoadExpressionToFPU;
                        Calculate;
                        ModifyAndSroreFPUInstructionForFixup ($E0D9);         { FCHS }
                        EndIntermediateCodeSubroutine;
                      end else Value.Extended := - Value.Extended;
                  end;
      btReal: begin
              If Instructions80x87 in StatementCompilerSwitches then
                begin
                  ConvertRealToExtended;
                  LoadExpressionToFPU;
                  Calculate;
                  ModifyAndSroreFPUInstructionForFixup ($E0D9);         { FCHS }
                  EndIntermediateCodeSubroutine;
                  Exit;
                end;
              Calculate;
              LoadRealExpressionToRegisterSet (rAX_BX_DX);
              GenerateInstruction_TwoBytes ($08, $C0);              { OR     AL, AL }
              GenerateInstruction_TwoBytes ($74, $03);              { JE     $ + 5  }
              GenerateInstruction_TwoBytes ($80, $F6);              { XOR    DH, 80 }
              GenerateInstruction_Byte ($80);
              EndIntermediateCodeSubroutine;
            end;
      else Error (OperandTypesDoNotMatchOperator);
    end;
  end;
```

### Code Block 2

```pascal
  Procedure ProcessUnaryMinus_Plus;
  Var LastToken: TToken;
      Overflow: Boolean;
      TempInteger: LongInt;
      TempIntegerTypeSet: TIntegerTypeSet;
  begin
    LastToken := Token;
    CheckPlusMinusAndGetNextToken;
    ProcessFactor;
    If LastToken <> Token_Minus then Exit;
    Case TypeDefPtr^.BaseType of
      btInteger: begin
              If Location = elConstant then
                begin
                  Overflow := False;
                  TempInteger := 0 - Value.LongInt;
                  Asm
                    JNO   @1
                    INC   Overflow
                @1:
                  end;
                  CheckOrdinalOverflowAndStore (TempInteger, Overflow);
                  Exit;
                end;
              SetLowestCommonIntegerType ([itSigned, it16Bit], DataType, TempIntegerTypeSet);
              ExtendInteger (TempIntegerTypeSet);
              Calculate;
              LoadExpressionToRegisters (urAX);
              If not (it32Bit in DataType) then GenerateInstruction_TwoBytes ($F7, $D8) { NEG   AX  }
                else GenerateCode_NEG_LongInt;
              GenerateOverflowCheckingCode (Self);
              EndIntermediateCodeSubroutine;
            end;
      btExtended: begin
                    If Location <> elConstant then
                      begin
                        ConvertRealToExtended;
                        LoadExpressionToFPU;
                        Calculate;
                        ModifyAndSroreFPUInstructionForFixup ($E0D9);         { FCHS }
                        EndIntermediateCodeSubroutine;
                      end else Value.Extended := - Value.Extended;
                  end;
      btReal: begin
              If Instructions80x87 in StatementCompilerSwitches then
                begin
                  ConvertRealToExtended;
                  LoadExpressionToFPU;
                  Calculate;
                  ModifyAndSroreFPUInstructionForFixup ($E0D9);         { FCHS }
                  EndIntermediateCodeSubroutine;
                  Exit;
                end;
              Calculate;
              LoadRealExpressionToRegisterSet (rAX_BX_DX);
              GenerateInstruction_TwoBytes ($08, $C0);              { OR     AL, AL }
              GenerateInstruction_TwoBytes ($74, $03);              { JE     $ + 5  }
              GenerateInstruction_TwoBytes ($80, $F6);              { XOR    DH, 80 }
              GenerateInstruction_Byte ($80);
              EndIntermediateCodeSubroutine;
            end;
      else Error (OperandTypesDoNotMatchOperator);
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
- https://turbopascal.org/processing-unary-sign-operators
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

