# System Functions Lo, Hi, Swap and Pi - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-functions-lo-hi-swap-and-pi

---

## Table of Contents

- System Functions Lo, Hi, Swap and Pi

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Functions Lo, Hi, Swap and Pi
These procedures process system functions
Lo
,
Hi
,
Swap
and
Pi
.
Procedure Func_Lo; Far;
begin
  Expression^.ExpectIntegerExpressionInParentheses;
  With Expression^ do If Location = elMemory then Include (LocationData.Flags, efSegment);
  Expression^.ExtendInteger ([itUnsigned]);
end;

Procedure Func_Hi; Far;
begin
  Expression^.ExpectIntegerExpressionInParentheses;
  If Expression^.Location = elConstant then
    begin
      Expression^.CheckOrdinalOverflowAndStore (Hi (Expression^.Value.Word), False);
      Exit;
    end;
  With Expression^ do If Location = elMemory then Include (LocationData.Flags, efSegment);
  Expression^.ExtendInteger ([itUnsigned, it16Bit]);
  If Expression^.Location = elMemory then
    begin
      Inc (Expression^.Value.Offset);
      Expression^.DataType := [itUnsigned];
      Exit;
    end;
  Expression^.Calculate;
  Expression^.LoadExpressionToRegisters (urAX);
  GenerateInstruction_TwoBytes ($88, $E0);    { MOV   AL, AH }
  Expression^.EndIntermediateCodeSubroutine;;
  Expression^.DataType := [itUnsigned];
end;

Procedure Func_Swap; Far;
begin
  Expression^.ExpectIntegerExpressionInParentheses;
  If Expression^.Location = elConstant then
    begin
      Expression^.CheckOrdinalOverflowAndStore (Swap (Expression^.Value.Word), False);
      Exit;
    end;
  Expression^.ExtendInteger ([itUnsigned, it16Bit]);
  Expression^.Calculate;
  Expression^.LoadExpressionToRegisters (urAX);
  GenerateInstruction_TwoBytes ($86, $C4);    { XCHG  AH, AL }
  Expression^.EndIntermediateCodeSubroutine;
end;

Procedure Func_Pi; Far;
begin
  With Expression^ do
    begin
      Value.Extended := Pi;
      IntermediateCodeOffset := 0;
      Location := elConstant;
      DataType := [itUnsigned];
      UsedRegisters := [];
      TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
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
Procedure Func_Lo; Far;
begin
  Expression^.ExpectIntegerExpressionInParentheses;
  With Expression^ do If Location = elMemory then Include (LocationData.Flags, efSegment);
  Expression^.ExtendInteger ([itUnsigned]);
end;

Procedure Func_Hi; Far;
begin
  Expression^.ExpectIntegerExpressionInParentheses;
  If Expression^.Location = elConstant then
    begin
      Expression^.CheckOrdinalOverflowAndStore (Hi (Expression^.Value.Word), False);
      Exit;
    end;
  With Expression^ do If Location = elMemory then Include (LocationData.Flags, efSegment);
  Expression^.ExtendInteger ([itUnsigned, it16Bit]);
  If Expression^.Location = elMemory then
    begin
      Inc (Expression^.Value.Offset);
      Expression^.DataType := [itUnsigned];
      Exit;
    end;
  Expression^.Calculate;
  Expression^.LoadExpressionToRegisters (urAX);
  GenerateInstruction_TwoBytes ($88, $E0);    { MOV   AL, AH }
  Expression^.EndIntermediateCodeSubroutine;;
  Expression^.DataType := [itUnsigned];
end;

Procedure Func_Swap; Far;
begin
  Expression^.ExpectIntegerExpressionInParentheses;
  If Expression^.Location = elConstant then
    begin
      Expression^.CheckOrdinalOverflowAndStore (Swap (Expression^.Value.Word), False);
      Exit;
    end;
  Expression^.ExtendInteger ([itUnsigned, it16Bit]);
  Expression^.Calculate;
  Expression^.LoadExpressionToRegisters (urAX);
  GenerateInstruction_TwoBytes ($86, $C4);    { XCHG  AH, AL }
  Expression^.EndIntermediateCodeSubroutine;
end;

Procedure Func_Pi; Far;
begin
  With Expression^ do
    begin
      Value.Extended := Pi;
      IntermediateCodeOffset := 0;
      Location := elConstant;
      DataType := [itUnsigned];
      UsedRegisters := [];
      TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
    end;
end;
```

### Code Block 2

```pascal
Procedure Func_Lo; Far;
begin
  Expression^.ExpectIntegerExpressionInParentheses;
  With Expression^ do If Location = elMemory then Include (LocationData.Flags, efSegment);
  Expression^.ExtendInteger ([itUnsigned]);
end;

Procedure Func_Hi; Far;
begin
  Expression^.ExpectIntegerExpressionInParentheses;
  If Expression^.Location = elConstant then
    begin
      Expression^.CheckOrdinalOverflowAndStore (Hi (Expression^.Value.Word), False);
      Exit;
    end;
  With Expression^ do If Location = elMemory then Include (LocationData.Flags, efSegment);
  Expression^.ExtendInteger ([itUnsigned, it16Bit]);
  If Expression^.Location = elMemory then
    begin
      Inc (Expression^.Value.Offset);
      Expression^.DataType := [itUnsigned];
      Exit;
    end;
  Expression^.Calculate;
  Expression^.LoadExpressionToRegisters (urAX);
  GenerateInstruction_TwoBytes ($88, $E0);    { MOV   AL, AH }
  Expression^.EndIntermediateCodeSubroutine;;
  Expression^.DataType := [itUnsigned];
end;

Procedure Func_Swap; Far;
begin
  Expression^.ExpectIntegerExpressionInParentheses;
  If Expression^.Location = elConstant then
    begin
      Expression^.CheckOrdinalOverflowAndStore (Swap (Expression^.Value.Word), False);
      Exit;
    end;
  Expression^.ExtendInteger ([itUnsigned, it16Bit]);
  Expression^.Calculate;
  Expression^.LoadExpressionToRegisters (urAX);
  GenerateInstruction_TwoBytes ($86, $C4);    { XCHG  AH, AL }
  Expression^.EndIntermediateCodeSubroutine;
end;

Procedure Func_Pi; Far;
begin
  With Expression^ do
    begin
      Value.Extended := Pi;
      IntermediateCodeOffset := 0;
      Location := elConstant;
      DataType := [itUnsigned];
      UsedRegisters := [];
      TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
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
- https://turbopascal.org/system-functions
- https://turbopascal.org/system-functions-lo-hi-swap-and-pi
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

