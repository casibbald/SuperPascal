# System Functions Succ and Pred - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-functions-succ-and-pred

---

## Table of Contents

- System Functions Succ and Pred

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Functions Succ and Pred
This procedure processes system functions
Succ
and
Pred
.
Procedure Func_Succ_Pred; Far;
Type POpCodes = ^TOpCodes;
     TOpCodes = Array [0..10] of Byte;
Const PredOpCodes: TOpCodes = (
        $FE, $C8,         { DEC   AL    }
        $2C, $01,         { SUB   AL, 1 }
        $48,              { DEC   AX    }
        $2D, $01, $00,    { SUB   AX, 1 }
        $83, $DA, $00);   { SBB   DX, 0 }

      SuccOpCodes: TOpCodes = (
        $FE, $C0,         { INC   AL    }
        $04, $01,         { ADD   AL, 1 }
        $40,              { INC   AX    }
        $05, $01, $00,    { ADD   AX, 1 }
        $83, $D2, $00);   { ADC   DX, 0 }

      OpCodes: Array [0..4, 0..1] of Byte = (
        (2, 0),
        (2, 2),
        (1, 4),
        (3, 5),
        (6, 5));

      Increment: Array [0..1] of ShortInt = (- 1, 1);

Var TempInt: LongInt;
    OverFlow: Boolean;
    StepArray: POpCodes;
    N, Index: Byte;
begin
  Case FuncParameter of
    0: StepArray := @PredOpCodes;
    1: StepArray := @SuccOpCodes;
  end;
  Expression^.ExpectOrdinalExpressionInParentheses;
  If Expression^.Location = elConstant then
    begin
      Overflow := False;
      TempInt := Expression^.Value.LongInt + Increment [FuncParameter];
      Asm
        JNO   @1
        INC   Overflow
    @1:
      end;
      Expression^.CheckOrdinalOverflowAndStore (TempInt, Overflow);
      Exit;
    end;
  If Expression^.Location = elPointerToMemory then
    begin
      If not (efSegment in Expression^.LocationData.Flags) then
        begin
          Inc (Expression^.Value.Integer, Increment [FuncParameter]);
          Exit;
        end;
      Expression^.LoadPointerToMemoryTo_DX_AX;
    end;
  Expression^.Calculate;
  Expression^.LoadExpressionToRegisters (urAX);
  Index := 0;
  If it16Bit in Expression^.DataType then
    begin
      Index := 2;
      If it32Bit in Expression^.DataType then Index := 4;
    end;
  If (OverflowChecking in StatementCompilerSwitches) and (itUnsigned in Expression^.DataType) then Inc (Index);
  For N := OpCodes [Index, 1] to OpCodes [Index, 1] + OpCodes [Index, 0] - 1 do GenerateInstruction_Byte (StepArray^ [N]);
  GenerateOverflowCheckingCode (Expression^);
  Expression^.EndIntermediateCodeSubroutine;
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
Procedure Func_Succ_Pred; Far;
Type POpCodes = ^TOpCodes;
     TOpCodes = Array [0..10] of Byte;
Const PredOpCodes: TOpCodes = (
        $FE, $C8,         { DEC   AL    }
        $2C, $01,         { SUB   AL, 1 }
        $48,              { DEC   AX    }
        $2D, $01, $00,    { SUB   AX, 1 }
        $83, $DA, $00);   { SBB   DX, 0 }

      SuccOpCodes: TOpCodes = (
        $FE, $C0,         { INC   AL    }
        $04, $01,         { ADD   AL, 1 }
        $40,              { INC   AX    }
        $05, $01, $00,    { ADD   AX, 1 }
        $83, $D2, $00);   { ADC   DX, 0 }

      OpCodes: Array [0..4, 0..1] of Byte = (
        (2, 0),
        (2, 2),
        (1, 4),
        (3, 5),
        (6, 5));

      Increment: Array [0..1] of ShortInt = (- 1, 1);

Var TempInt: LongInt;
    OverFlow: Boolean;
    StepArray: POpCodes;
    N, Index: Byte;
begin
  Case FuncParameter of
    0: StepArray := @PredOpCodes;
    1: StepArray := @SuccOpCodes;
  end;
  Expression^.ExpectOrdinalExpressionInParentheses;
  If Expression^.Location = elConstant then
    begin
      Overflow := False;
      TempInt := Expression^.Value.LongInt + Increment [FuncParameter];
      Asm
        JNO   @1
        INC   Overflow
    @1:
      end;
      Expression^.CheckOrdinalOverflowAndStore (TempInt, Overflow);
      Exit;
    end;
  If Expression^.Location = elPointerToMemory then
    begin
      If not (efSegment in Expression^.LocationData.Flags) then
        begin
          Inc (Expression^.Value.Integer, Increment [FuncParameter]);
          Exit;
        end;
      Expression^.LoadPointerToMemoryTo_DX_AX;
    end;
  Expression^.Calculate;
  Expression^.LoadExpressionToRegisters (urAX);
  Index := 0;
  If it16Bit in Expression^.DataType then
    begin
      Index := 2;
      If it32Bit in Expression^.DataType then Index := 4;
    end;
  If (OverflowChecking in StatementCompilerSwitches) and (itUnsigned in Expression^.DataType) then Inc (Index);
  For N := OpCodes [Index, 1] to OpCodes [Index, 1] + OpCodes [Index, 0] - 1 do GenerateInstruction_Byte (StepArray^ [N]);
  GenerateOverflowCheckingCode (Expression^);
  Expression^.EndIntermediateCodeSubroutine;
end;
```

### Code Block 2

```pascal
Procedure Func_Succ_Pred; Far;
Type POpCodes = ^TOpCodes;
     TOpCodes = Array [0..10] of Byte;
Const PredOpCodes: TOpCodes = (
        $FE, $C8,         { DEC   AL    }
        $2C, $01,         { SUB   AL, 1 }
        $48,              { DEC   AX    }
        $2D, $01, $00,    { SUB   AX, 1 }
        $83, $DA, $00);   { SBB   DX, 0 }

      SuccOpCodes: TOpCodes = (
        $FE, $C0,         { INC   AL    }
        $04, $01,         { ADD   AL, 1 }
        $40,              { INC   AX    }
        $05, $01, $00,    { ADD   AX, 1 }
        $83, $D2, $00);   { ADC   DX, 0 }

      OpCodes: Array [0..4, 0..1] of Byte = (
        (2, 0),
        (2, 2),
        (1, 4),
        (3, 5),
        (6, 5));

      Increment: Array [0..1] of ShortInt = (- 1, 1);

Var TempInt: LongInt;
    OverFlow: Boolean;
    StepArray: POpCodes;
    N, Index: Byte;
begin
  Case FuncParameter of
    0: StepArray := @PredOpCodes;
    1: StepArray := @SuccOpCodes;
  end;
  Expression^.ExpectOrdinalExpressionInParentheses;
  If Expression^.Location = elConstant then
    begin
      Overflow := False;
      TempInt := Expression^.Value.LongInt + Increment [FuncParameter];
      Asm
        JNO   @1
        INC   Overflow
    @1:
      end;
      Expression^.CheckOrdinalOverflowAndStore (TempInt, Overflow);
      Exit;
    end;
  If Expression^.Location = elPointerToMemory then
    begin
      If not (efSegment in Expression^.LocationData.Flags) then
        begin
          Inc (Expression^.Value.Integer, Increment [FuncParameter]);
          Exit;
        end;
      Expression^.LoadPointerToMemoryTo_DX_AX;
    end;
  Expression^.Calculate;
  Expression^.LoadExpressionToRegisters (urAX);
  Index := 0;
  If it16Bit in Expression^.DataType then
    begin
      Index := 2;
      If it32Bit in Expression^.DataType then Index := 4;
    end;
  If (OverflowChecking in StatementCompilerSwitches) and (itUnsigned in Expression^.DataType) then Inc (Index);
  For N := OpCodes [Index, 1] to OpCodes [Index, 1] + OpCodes [Index, 0] - 1 do GenerateInstruction_Byte (StepArray^ [N]);
  GenerateOverflowCheckingCode (Expression^);
  Expression^.EndIntermediateCodeSubroutine;
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
- https://turbopascal.org/system-functions
- https://turbopascal.org/system-functions-succ-and-pred
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

