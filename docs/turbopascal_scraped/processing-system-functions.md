# Processing System Functions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-system-functions

---

## Table of Contents

- Processing System Functions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing System Functions
All Turbo Pascal system functions are processed by this procedure. Each system function has a processing procedure and an optional parameter. This parameter is used when two or more functions share the same processing procedure.
Type TFunctions = Record
                    Proc: Procedure;
                    FuncParameter: Word;
                  end;

Const Functions: Array [$00..$33] of TFunctions = (
        (Proc: Func_New;                 FuncParameter: 0),                 { New }
        (Proc: Func_TextFunctions;       FuncParameter: SysProc_GetSEoln),  { SeekEoln }
        (Proc: Func_TextFunctions;       FuncParameter: SysProc_GetSEof),   { SeekEof }
        (Proc: Func_TextFunctions;       FuncParameter: SysProc_GetTEoln),  { Eoln }
        (Proc: Func_TextFunctions;       FuncParameter: SysProc_GetTEof),   { Eof }
        (Proc: Func_FilePos_FileSize;    FuncParameter: SysProc_GetFPos),   { FilePos }
        (Proc: Func_FilePos_FileSize;    FuncParameter: SysProc_GetFSize),  { FileSize }
        (Proc: Func_UpCase;              FuncParameter: 0),                 { UpCase }
        (Proc: Func_Abs;                 FuncParameter: 0),                 { Abs }
        (Proc: Func_Sqr;                 FuncParameter: 0),                 { Sqr }
        (Proc: Func_Succ_Pred;           FuncParameter: 1),                 { Succ }
        (Proc: Func_Succ_Pred;           FuncParameter: 0),                 { Pred }
        (Proc: Func_Odd;                 FuncParameter: 0),                 { Odd }
        (Proc: Func_Ord;                 FuncParameter: 0),                 { Ord }
        (Proc: Func_Chr;                 FuncParameter: 0),                 { Chr }
        (Proc: Func_Ptr;                 FuncParameter: 0),                 { Ptr }
        (Proc: Func_Addr_Ofs_Seg;        FuncParameter: 1),                 { Ofs }
        (Proc: Func_Addr_Ofs_Seg;        FuncParameter: 2),                 { Seg }
        (Proc: Func_Registers;           FuncParameter: MOV_AX_CS),         { CSeg }
        (Proc: Func_Registers;           FuncParameter: MOV_AX_DS),         { DSeg }
        (Proc: Func_Registers;           FuncParameter: MOV_AX_SS),         { SSeg }
        (Proc: Func_Registers;           FuncParameter: MOV_AX_SP),         { SPtr }
        (Proc: Func_SizeOf;              FuncParameter: 0),                 { SizeOf }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 2),                 { Sqrt }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 0),                 { Int }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 3),                 { Sin }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 4),                 { Cos }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 7),                 { ArcTan }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 5),                 { Ln }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 6),                 { Exp }
        (Proc: Func_Trunc_Round;         FuncParameter: 8),                 { Trunc }
        (Proc: Func_Trunc_Round;         FuncParameter: 9),                 { Round }
        (Proc: Func_MemAvail_MaxAvail;   FuncParameter: SysProc_GetFreMem), { MemAvail }
        (Proc: Func_MemAvail_MaxAvail;   FuncParameter: SysProc_GetFreMax), { MaxAvail }
        (Proc: Func_Length;              FuncParameter: 0),                 { Length }
        (Proc: Func_Pos;                 FuncParameter: 0),                 { Pos }
        (Proc: Func_Copy;                FuncParameter: 0),                 { Copy }
        (Proc: Func_Concat;              FuncParameter: 0),                 { Concat }
        (Proc: Func_IOResult_ParamCount; FuncParameter: SysProc_GetIORes),  { IOResult }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 1),                 { Frac }
        (Proc: Func_Random;              FuncParameter: 0),                 { Random }
        (Proc: Func_ParamStr;            FuncParameter: 0),                 { ParamStr }
        (Proc: Func_IOResult_ParamCount; FuncParameter: SysProc_GetParCnt), { ParamCount }
        (Proc: Func_Lo;                  FuncParameter: 0),                 { Lo }
        (Proc: Func_Hi;                  FuncParameter: 0),                 { Hi }
        (Proc: Func_Swap;                FuncParameter: 0),                 { Swap }
        (Proc: Func_Pi;                  FuncParameter: 0),                 { Pi }
        (Proc: Func_Addr_Ofs_Seg;        FuncParameter: 0),                 { Addr }
        (Proc: Func_TypeOf;              FuncParameter: 0),                 { TypeOf }
        (Proc: Func_Assigned;            FuncParameter: 0),                 { Assigned }
        (Proc: Func_Low_High;            FuncParameter: 8),                 { Low }
        (Proc: Func_Low_High;            FuncParameter: 12));               { High }
This procedure processes system functions. The parameter is saved to allow recursion calls.
Procedure ProcessFunction (Func: Byte; Var FunctionExpression: TExpression);
Var SavedExpression: PExpression;
    SavedParameter: Word;
begin
  SavedParameter := FuncParameter;
  FuncParameter := Functions [Func].FuncParameter;
  SavedExpression := Expression;
  Expression := @FunctionExpression;
  Functions [Func].Proc;
  Expression := SavedExpression;
  FuncParameter := SavedParameter;
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
Type TFunctions = Record
                    Proc: Procedure;
                    FuncParameter: Word;
                  end;

Const Functions: Array [$00..$33] of TFunctions = (
        (Proc: Func_New;                 FuncParameter: 0),                 { New }
        (Proc: Func_TextFunctions;       FuncParameter: SysProc_GetSEoln),  { SeekEoln }
        (Proc: Func_TextFunctions;       FuncParameter: SysProc_GetSEof),   { SeekEof }
        (Proc: Func_TextFunctions;       FuncParameter: SysProc_GetTEoln),  { Eoln }
        (Proc: Func_TextFunctions;       FuncParameter: SysProc_GetTEof),   { Eof }
        (Proc: Func_FilePos_FileSize;    FuncParameter: SysProc_GetFPos),   { FilePos }
        (Proc: Func_FilePos_FileSize;    FuncParameter: SysProc_GetFSize),  { FileSize }
        (Proc: Func_UpCase;              FuncParameter: 0),                 { UpCase }
        (Proc: Func_Abs;                 FuncParameter: 0),                 { Abs }
        (Proc: Func_Sqr;                 FuncParameter: 0),                 { Sqr }
        (Proc: Func_Succ_Pred;           FuncParameter: 1),                 { Succ }
        (Proc: Func_Succ_Pred;           FuncParameter: 0),                 { Pred }
        (Proc: Func_Odd;                 FuncParameter: 0),                 { Odd }
        (Proc: Func_Ord;                 FuncParameter: 0),                 { Ord }
        (Proc: Func_Chr;                 FuncParameter: 0),                 { Chr }
        (Proc: Func_Ptr;                 FuncParameter: 0),                 { Ptr }
        (Proc: Func_Addr_Ofs_Seg;        FuncParameter: 1),                 { Ofs }
        (Proc: Func_Addr_Ofs_Seg;        FuncParameter: 2),                 { Seg }
        (Proc: Func_Registers;           FuncParameter: MOV_AX_CS),         { CSeg }
        (Proc: Func_Registers;           FuncParameter: MOV_AX_DS),         { DSeg }
        (Proc: Func_Registers;           FuncParameter: MOV_AX_SS),         { SSeg }
        (Proc: Func_Registers;           FuncParameter: MOV_AX_SP),         { SPtr }
        (Proc: Func_SizeOf;              FuncParameter: 0),                 { SizeOf }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 2),                 { Sqrt }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 0),                 { Int }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 3),                 { Sin }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 4),                 { Cos }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 7),                 { ArcTan }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 5),                 { Ln }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 6),                 { Exp }
        (Proc: Func_Trunc_Round;         FuncParameter: 8),                 { Trunc }
        (Proc: Func_Trunc_Round;         FuncParameter: 9),                 { Round }
        (Proc: Func_MemAvail_MaxAvail;   FuncParameter: SysProc_GetFreMem), { MemAvail }
        (Proc: Func_MemAvail_MaxAvail;   FuncParameter: SysProc_GetFreMax), { MaxAvail }
        (Proc: Func_Length;              FuncParameter: 0),                 { Length }
        (Proc: Func_Pos;                 FuncParameter: 0),                 { Pos }
        (Proc: Func_Copy;                FuncParameter: 0),                 { Copy }
        (Proc: Func_Concat;              FuncParameter: 0),                 { Concat }
        (Proc: Func_IOResult_ParamCount; FuncParameter: SysProc_GetIORes),  { IOResult }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 1),                 { Frac }
        (Proc: Func_Random;              FuncParameter: 0),                 { Random }
        (Proc: Func_ParamStr;            FuncParameter: 0),                 { ParamStr }
        (Proc: Func_IOResult_ParamCount; FuncParameter: SysProc_GetParCnt), { ParamCount }
        (Proc: Func_Lo;                  FuncParameter: 0),                 { Lo }
        (Proc: Func_Hi;                  FuncParameter: 0),                 { Hi }
        (Proc: Func_Swap;                FuncParameter: 0),                 { Swap }
        (Proc: Func_Pi;                  FuncParameter: 0),                 { Pi }
        (Proc: Func_Addr_Ofs_Seg;        FuncParameter: 0),                 { Addr }
        (Proc: Func_TypeOf;              FuncParameter: 0),                 { TypeOf }
        (Proc: Func_Assigned;            FuncParameter: 0),                 { Assigned }
        (Proc: Func_Low_High;            FuncParameter: 8),                 { Low }
        (Proc: Func_Low_High;            FuncParameter: 12));               { High }
```

### Code Block 2

```pascal
Procedure ProcessFunction (Func: Byte; Var FunctionExpression: TExpression);
Var SavedExpression: PExpression;
    SavedParameter: Word;
begin
  SavedParameter := FuncParameter;
  FuncParameter := Functions [Func].FuncParameter;
  SavedExpression := Expression;
  Expression := @FunctionExpression;
  Functions [Func].Proc;
  Expression := SavedExpression;
  FuncParameter := SavedParameter;
end;
```

### Code Block 3

```pascal
Type TFunctions = Record
                    Proc: Procedure;
                    FuncParameter: Word;
                  end;

Const Functions: Array [$00..$33] of TFunctions = (
        (Proc: Func_New;                 FuncParameter: 0),                 { New }
        (Proc: Func_TextFunctions;       FuncParameter: SysProc_GetSEoln),  { SeekEoln }
        (Proc: Func_TextFunctions;       FuncParameter: SysProc_GetSEof),   { SeekEof }
        (Proc: Func_TextFunctions;       FuncParameter: SysProc_GetTEoln),  { Eoln }
        (Proc: Func_TextFunctions;       FuncParameter: SysProc_GetTEof),   { Eof }
        (Proc: Func_FilePos_FileSize;    FuncParameter: SysProc_GetFPos),   { FilePos }
        (Proc: Func_FilePos_FileSize;    FuncParameter: SysProc_GetFSize),  { FileSize }
        (Proc: Func_UpCase;              FuncParameter: 0),                 { UpCase }
        (Proc: Func_Abs;                 FuncParameter: 0),                 { Abs }
        (Proc: Func_Sqr;                 FuncParameter: 0),                 { Sqr }
        (Proc: Func_Succ_Pred;           FuncParameter: 1),                 { Succ }
        (Proc: Func_Succ_Pred;           FuncParameter: 0),                 { Pred }
        (Proc: Func_Odd;                 FuncParameter: 0),                 { Odd }
        (Proc: Func_Ord;                 FuncParameter: 0),                 { Ord }
        (Proc: Func_Chr;                 FuncParameter: 0),                 { Chr }
        (Proc: Func_Ptr;                 FuncParameter: 0),                 { Ptr }
        (Proc: Func_Addr_Ofs_Seg;        FuncParameter: 1),                 { Ofs }
        (Proc: Func_Addr_Ofs_Seg;        FuncParameter: 2),                 { Seg }
        (Proc: Func_Registers;           FuncParameter: MOV_AX_CS),         { CSeg }
        (Proc: Func_Registers;           FuncParameter: MOV_AX_DS),         { DSeg }
        (Proc: Func_Registers;           FuncParameter: MOV_AX_SS),         { SSeg }
        (Proc: Func_Registers;           FuncParameter: MOV_AX_SP),         { SPtr }
        (Proc: Func_SizeOf;              FuncParameter: 0),                 { SizeOf }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 2),                 { Sqrt }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 0),                 { Int }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 3),                 { Sin }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 4),                 { Cos }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 7),                 { ArcTan }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 5),                 { Ln }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 6),                 { Exp }
        (Proc: Func_Trunc_Round;         FuncParameter: 8),                 { Trunc }
        (Proc: Func_Trunc_Round;         FuncParameter: 9),                 { Round }
        (Proc: Func_MemAvail_MaxAvail;   FuncParameter: SysProc_GetFreMem), { MemAvail }
        (Proc: Func_MemAvail_MaxAvail;   FuncParameter: SysProc_GetFreMax), { MaxAvail }
        (Proc: Func_Length;              FuncParameter: 0),                 { Length }
        (Proc: Func_Pos;                 FuncParameter: 0),                 { Pos }
        (Proc: Func_Copy;                FuncParameter: 0),                 { Copy }
        (Proc: Func_Concat;              FuncParameter: 0),                 { Concat }
        (Proc: Func_IOResult_ParamCount; FuncParameter: SysProc_GetIORes),  { IOResult }
        (Proc: Func_ArithmeticFunctions; FuncParameter: 1),                 { Frac }
        (Proc: Func_Random;              FuncParameter: 0),                 { Random }
        (Proc: Func_ParamStr;            FuncParameter: 0),                 { ParamStr }
        (Proc: Func_IOResult_ParamCount; FuncParameter: SysProc_GetParCnt), { ParamCount }
        (Proc: Func_Lo;                  FuncParameter: 0),                 { Lo }
        (Proc: Func_Hi;                  FuncParameter: 0),                 { Hi }
        (Proc: Func_Swap;                FuncParameter: 0),                 { Swap }
        (Proc: Func_Pi;                  FuncParameter: 0),                 { Pi }
        (Proc: Func_Addr_Ofs_Seg;        FuncParameter: 0),                 { Addr }
        (Proc: Func_TypeOf;              FuncParameter: 0),                 { TypeOf }
        (Proc: Func_Assigned;            FuncParameter: 0),                 { Assigned }
        (Proc: Func_Low_High;            FuncParameter: 8),                 { Low }
        (Proc: Func_Low_High;            FuncParameter: 12));               { High }
```

### Code Block 4

```pascal
Procedure ProcessFunction (Func: Byte; Var FunctionExpression: TExpression);
Var SavedExpression: PExpression;
    SavedParameter: Word;
begin
  SavedParameter := FuncParameter;
  FuncParameter := Functions [Func].FuncParameter;
  SavedExpression := Expression;
  Expression := @FunctionExpression;
  Functions [Func].Proc;
  Expression := SavedExpression;
  FuncParameter := SavedParameter;
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
- https://turbopascal.org/processing-system-functions
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

