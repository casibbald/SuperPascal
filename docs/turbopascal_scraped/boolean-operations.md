# Boolean Operations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/boolean-operations

---

## Table of Contents

- Boolean Operations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Boolean Operations
Boolean operations are similar to
integer operations
. Constant
Boolean
operations are performed with boolean expressions
converted to boolean bytes
(0 or 1) and using procedures for
integer calculation
.
Procedure ConstantBooleanOperations;
  begin
    LeftExpression.ConvertToBooleanByte;
    RightExpression.ConvertToBooleanByte;
    ConstantIntegerOperations;
  end;
Generating code for
Boolean
expression is no different from code for integer expressions with the exception of short-circuit evaluations. Boolean operations
AND
and
OR
can be decided after only one operant is calculated. If the first operand in boolean operation
OR
evaluates to True then there is no need to evaluate the second one. Similarly, if the first operand in boolean operation
AND
evaluates to
False
then there is no need to evaluate the second one. This procedure takes care for this. Short-circuit evaluation is a great example of the elegance of Turbo Pascal and
boolean expressions
using jump lists.
Procedure GenerateCodeForBooleanOperations;
  Var LastJumpToTrue, LastJumpToFalse, TempWord: PWord;
      JumpIfTrueOpCode: Byte;
  begin
    If (FullBooleanEval in StatementCompilerSwitches) or not (Operation in [Calc_AND, Calc_OR]) then
      begin
        LeftExpression.ConvertToBooleanByte;
        RightExpression.ConvertToBooleanByte;
        GenerateCodeForIntegerOperations;
        Exit;
      end;
    LeftExpression.ConvertExpressionToBooleanJump;
    RightExpression.ConvertExpressionToBooleanJump;
    LeftExpression.Calculate;
    LastJumpToTrue := @LeftExpression.Value.LastJumpToTrue;
    LastJumpToFalse := @LeftExpression.Value.LastJumpToFalse;
    JumpIfTrueOpCode := LeftExpression.LocationData.JumpIfTrueOpCode;
    If Operation <> Calc_OR then
      begin
        JumpIfTrueOpCode := JumpIfTrueOpCode xor $01;
        TempWord := LastJumpToTrue;
        LastJumpToTrue := LastJumpToFalse;
        LastJumpToFalse := TempWord;
      end;
    GenerateCodeForNearJump (LastJumpToTrue^, JumpIfTrueOpCode);
    GenerateLabelAndSetJumpsToIt (LastJumpToFalse^);
    RightExpression.Calculate;
    LeftExpression.EndIntermediateCodeSubroutine;
    With LeftExpression do UsedRegisters := UsedRegisters + RightExpression.UsedRegisters;
    JoinJumpsOfBothExpressions (LeftExpression.Value.LastJumpToTrue, RightExpression.Value.LastJumpToTrue);
    JoinJumpsOfBothExpressions (LeftExpression.Value.LastJumpToFalse, RightExpression.Value.LastJumpToFalse);
    LeftExpression.LocationData.JumpIfTrueOpCode := RightExpression.LocationData.JumpIfTrueOpCode;
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
  Procedure ConstantBooleanOperations;
  begin
    LeftExpression.ConvertToBooleanByte;
    RightExpression.ConvertToBooleanByte;
    ConstantIntegerOperations;
  end;
```

### Code Block 2

```pascal
  Procedure GenerateCodeForBooleanOperations;
  Var LastJumpToTrue, LastJumpToFalse, TempWord: PWord;
      JumpIfTrueOpCode: Byte;
  begin
    If (FullBooleanEval in StatementCompilerSwitches) or not (Operation in [Calc_AND, Calc_OR]) then
      begin
        LeftExpression.ConvertToBooleanByte;
        RightExpression.ConvertToBooleanByte;
        GenerateCodeForIntegerOperations;
        Exit;
      end;
    LeftExpression.ConvertExpressionToBooleanJump;
    RightExpression.ConvertExpressionToBooleanJump;
    LeftExpression.Calculate;
    LastJumpToTrue := @LeftExpression.Value.LastJumpToTrue;
    LastJumpToFalse := @LeftExpression.Value.LastJumpToFalse;
    JumpIfTrueOpCode := LeftExpression.LocationData.JumpIfTrueOpCode;
    If Operation <> Calc_OR then
      begin
        JumpIfTrueOpCode := JumpIfTrueOpCode xor $01;
        TempWord := LastJumpToTrue;
        LastJumpToTrue := LastJumpToFalse;
        LastJumpToFalse := TempWord;
      end;
    GenerateCodeForNearJump (LastJumpToTrue^, JumpIfTrueOpCode);
    GenerateLabelAndSetJumpsToIt (LastJumpToFalse^);
    RightExpression.Calculate;
    LeftExpression.EndIntermediateCodeSubroutine;
    With LeftExpression do UsedRegisters := UsedRegisters + RightExpression.UsedRegisters;
    JoinJumpsOfBothExpressions (LeftExpression.Value.LastJumpToTrue, RightExpression.Value.LastJumpToTrue);
    JoinJumpsOfBothExpressions (LeftExpression.Value.LastJumpToFalse, RightExpression.Value.LastJumpToFalse);
    LeftExpression.LocationData.JumpIfTrueOpCode := RightExpression.LocationData.JumpIfTrueOpCode;
  end;
```

### Code Block 3

```pascal
  Procedure ConstantBooleanOperations;
  begin
    LeftExpression.ConvertToBooleanByte;
    RightExpression.ConvertToBooleanByte;
    ConstantIntegerOperations;
  end;
```

### Code Block 4

```pascal
  Procedure GenerateCodeForBooleanOperations;
  Var LastJumpToTrue, LastJumpToFalse, TempWord: PWord;
      JumpIfTrueOpCode: Byte;
  begin
    If (FullBooleanEval in StatementCompilerSwitches) or not (Operation in [Calc_AND, Calc_OR]) then
      begin
        LeftExpression.ConvertToBooleanByte;
        RightExpression.ConvertToBooleanByte;
        GenerateCodeForIntegerOperations;
        Exit;
      end;
    LeftExpression.ConvertExpressionToBooleanJump;
    RightExpression.ConvertExpressionToBooleanJump;
    LeftExpression.Calculate;
    LastJumpToTrue := @LeftExpression.Value.LastJumpToTrue;
    LastJumpToFalse := @LeftExpression.Value.LastJumpToFalse;
    JumpIfTrueOpCode := LeftExpression.LocationData.JumpIfTrueOpCode;
    If Operation <> Calc_OR then
      begin
        JumpIfTrueOpCode := JumpIfTrueOpCode xor $01;
        TempWord := LastJumpToTrue;
        LastJumpToTrue := LastJumpToFalse;
        LastJumpToFalse := TempWord;
      end;
    GenerateCodeForNearJump (LastJumpToTrue^, JumpIfTrueOpCode);
    GenerateLabelAndSetJumpsToIt (LastJumpToFalse^);
    RightExpression.Calculate;
    LeftExpression.EndIntermediateCodeSubroutine;
    With LeftExpression do UsedRegisters := UsedRegisters + RightExpression.UsedRegisters;
    JoinJumpsOfBothExpressions (LeftExpression.Value.LastJumpToTrue, RightExpression.Value.LastJumpToTrue);
    JoinJumpsOfBothExpressions (LeftExpression.Value.LastJumpToFalse, RightExpression.Value.LastJumpToFalse);
    LeftExpression.LocationData.JumpIfTrueOpCode := RightExpression.LocationData.JumpIfTrueOpCode;
  end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/boolean-operations
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/converting-boolean-expressions
- https://turbopascal.org/expressions
- https://turbopascal.org/integer-operations
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

