# Arithmetic System Functions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/arithmetic-system-functions

---

## Table of Contents

- Arithmetic System Functions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Arithmetic System Functions
This procedure processes system functions
Int
,
Frac
,
Sqrt
,
Sin
,
Cos
,
Ln
,
Exp
and
ArcTan
.
Procedure Func_Trunc_Round; Far;
begin
  Expression^.ExpectIntegerOrFloatingPointExpressionInParentheses;
  With Expression^ do
    begin
      If Location = elConstant then
        begin
          Expression^.ConvertIntegerToExtended;
          Case FuncParameter of
            8: Value.LongInt := Trunc (Value.Extended);
            9: Value.LongInt := Round (Value.Extended);
          end;
          If Value.LongInt = $80000000 then Error (ConstantOutOfRange); { ??? !!! }
          Expression^.CheckOrdinalOverflowAndStore (Value.LongInt, False);
          TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
          Exit;
        end;
      ArithmeticFunctions;
      Location := elRegister;
      DataType := [it32Bit, it16Bit, itSigned];
      LocationData.B9 := 0;
      TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
    end;
end;
This procedure prepares expression and generates a call to compiler routine for system functions
Int
,
Frac
,
Sqrt
,
Sin
,
Cos
,
Ln
,
Exp
,
ArcTan
,
Trunc
and
Round
.
Procedure ArithmeticFunctions;
Const Table: Array [0..9, Boolean] of Word = (
        (SysProc_RInt,    SysProc_FInt),
        (SysProc_RFrac,   SysProc_FFrac),
        (SysProc_RSqrt,   SysProc_FSqrt),
        (SysProc_RSin,    SysProc_FSin),
        (SysProc_RCos,    SysProc_FCos),
        (SysProc_RLn,     SysProc_FLn),
        (SysProc_RExp,    SysProc_FExp),
        (SysProc_RArcTan, SysProc_FArcTan),
        (SysProc_RTrunc,  SysProc_FTrunc),
        (SysProc_RRound,  SysProc_FRound));

begin
  If Instructions80x87 in StatementCompilerSwitches then
    begin
      Expression^.ConvertIntegerToExtended;
      Expression^.ConvertRealToExtended;
      Expression^.LoadExpressionToFPU;
      Expression^.Calculate;
    end else begin
               Expression^.ConvertIntegerToReal;
               Expression^.ConvertExtendedToReal;
               Expression^.Calculate;
               Expression^.LoadRealExpressionToRegisterSet (rAX_BX_DX);
             end;
  GenerateInstruction_CALL_FAR (Table [FuncParameter, Instructions80x87 in StatementCompilerSwitches]);
  Expression^.EndIntermediateCodeSubroutine;
  Expression^.UsedRegisters := [urSP, urDX, urAX];
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
Procedure Func_Trunc_Round; Far;
begin
  Expression^.ExpectIntegerOrFloatingPointExpressionInParentheses;
  With Expression^ do
    begin
      If Location = elConstant then
        begin
          Expression^.ConvertIntegerToExtended;
          Case FuncParameter of
            8: Value.LongInt := Trunc (Value.Extended);
            9: Value.LongInt := Round (Value.Extended);
          end;
          If Value.LongInt = $80000000 then Error (ConstantOutOfRange); { ??? !!! }
          Expression^.CheckOrdinalOverflowAndStore (Value.LongInt, False);
          TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
          Exit;
        end;
      ArithmeticFunctions;
      Location := elRegister;
      DataType := [it32Bit, it16Bit, itSigned];
      LocationData.B9 := 0;
      TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
    end;
end;
```

### Code Block 2

```pascal
Procedure ArithmeticFunctions;
Const Table: Array [0..9, Boolean] of Word = (
        (SysProc_RInt,    SysProc_FInt),
        (SysProc_RFrac,   SysProc_FFrac),
        (SysProc_RSqrt,   SysProc_FSqrt),
        (SysProc_RSin,    SysProc_FSin),
        (SysProc_RCos,    SysProc_FCos),
        (SysProc_RLn,     SysProc_FLn),
        (SysProc_RExp,    SysProc_FExp),
        (SysProc_RArcTan, SysProc_FArcTan),
        (SysProc_RTrunc,  SysProc_FTrunc),
        (SysProc_RRound,  SysProc_FRound));

begin
  If Instructions80x87 in StatementCompilerSwitches then
    begin
      Expression^.ConvertIntegerToExtended;
      Expression^.ConvertRealToExtended;
      Expression^.LoadExpressionToFPU;
      Expression^.Calculate;
    end else begin
               Expression^.ConvertIntegerToReal;
               Expression^.ConvertExtendedToReal;
               Expression^.Calculate;
               Expression^.LoadRealExpressionToRegisterSet (rAX_BX_DX);
             end;
  GenerateInstruction_CALL_FAR (Table [FuncParameter, Instructions80x87 in StatementCompilerSwitches]);
  Expression^.EndIntermediateCodeSubroutine;
  Expression^.UsedRegisters := [urSP, urDX, urAX];
end;
```

### Code Block 3

```pascal
Procedure Func_Trunc_Round; Far;
begin
  Expression^.ExpectIntegerOrFloatingPointExpressionInParentheses;
  With Expression^ do
    begin
      If Location = elConstant then
        begin
          Expression^.ConvertIntegerToExtended;
          Case FuncParameter of
            8: Value.LongInt := Trunc (Value.Extended);
            9: Value.LongInt := Round (Value.Extended);
          end;
          If Value.LongInt = $80000000 then Error (ConstantOutOfRange); { ??? !!! }
          Expression^.CheckOrdinalOverflowAndStore (Value.LongInt, False);
          TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
          Exit;
        end;
      ArithmeticFunctions;
      Location := elRegister;
      DataType := [it32Bit, it16Bit, itSigned];
      LocationData.B9 := 0;
      TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
    end;
end;
```

### Code Block 4

```pascal
Procedure ArithmeticFunctions;
Const Table: Array [0..9, Boolean] of Word = (
        (SysProc_RInt,    SysProc_FInt),
        (SysProc_RFrac,   SysProc_FFrac),
        (SysProc_RSqrt,   SysProc_FSqrt),
        (SysProc_RSin,    SysProc_FSin),
        (SysProc_RCos,    SysProc_FCos),
        (SysProc_RLn,     SysProc_FLn),
        (SysProc_RExp,    SysProc_FExp),
        (SysProc_RArcTan, SysProc_FArcTan),
        (SysProc_RTrunc,  SysProc_FTrunc),
        (SysProc_RRound,  SysProc_FRound));

begin
  If Instructions80x87 in StatementCompilerSwitches then
    begin
      Expression^.ConvertIntegerToExtended;
      Expression^.ConvertRealToExtended;
      Expression^.LoadExpressionToFPU;
      Expression^.Calculate;
    end else begin
               Expression^.ConvertIntegerToReal;
               Expression^.ConvertExtendedToReal;
               Expression^.Calculate;
               Expression^.LoadRealExpressionToRegisterSet (rAX_BX_DX);
             end;
  GenerateInstruction_CALL_FAR (Table [FuncParameter, Instructions80x87 in StatementCompilerSwitches]);
  Expression^.EndIntermediateCodeSubroutine;
  Expression^.UsedRegisters := [urSP, urDX, urAX];
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/arithmetic-system-functions
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

