# System Procedures Halt, RunError, Fail and Randomize - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-procedures-halt-runerror-fail-and-randomize

---

## Table of Contents

- System Procedures Halt, RunError, Fail and Randomize

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Procedures Halt, RunError, Fail and Randomize
This function processes system procedures
Halt
and
RunError
.
Function Proc_Halt_RunError: Word; Far;
Var ExitCode: TExpression;
begin
  If CheckAndGetNextToken (Token_LeftParenthesis) then
    begin
      ExitCode.ExpectIntegerExpression;
      ExitCode.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
      ExpectTokenAndGetNext (Token_RightParenthesis);
      ExitCode.Calculate;
      ExitCode.LoadExpressionToRegisters (urAX);
    end else GenerateInstruction_Word (XOR_AX_AX);
  GenerateInstruction_CALL_FAR (ProcParameter);
  Proc_Halt_RunError := EndSubroutine;
end;
This function processes system procedure
Fail
.
Function Proc_Fail: Word; Far;
begin
  If pfConstructor in PProcedureIdentifierData (Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset))^.Flags then
    begin
      GenerateCodeForNearJump (LastJumpToFail, JMP_ShortDirect);
      Proc_Fail := EndSubroutine;
    end else Error (FailOnlyAllowedWithinConstructors);
end;
This function processes system procedure
Randomize
. It simply calls an appropriate compiler routine.
Function Proc_Randomize: Word; Far;
begin
  GenerateInstruction_CALL_FAR (SysProc_InitRand);
  Proc_Randomize := EndSubRoutine;
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
Function Proc_Halt_RunError: Word; Far;
Var ExitCode: TExpression;
begin
  If CheckAndGetNextToken (Token_LeftParenthesis) then
    begin
      ExitCode.ExpectIntegerExpression;
      ExitCode.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
      ExpectTokenAndGetNext (Token_RightParenthesis);
      ExitCode.Calculate;
      ExitCode.LoadExpressionToRegisters (urAX);
    end else GenerateInstruction_Word (XOR_AX_AX);
  GenerateInstruction_CALL_FAR (ProcParameter);
  Proc_Halt_RunError := EndSubroutine;
end;
```

### Code Block 2

```pascal
Function Proc_Fail: Word; Far;
begin
  If pfConstructor in PProcedureIdentifierData (Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset))^.Flags then
    begin
      GenerateCodeForNearJump (LastJumpToFail, JMP_ShortDirect);
      Proc_Fail := EndSubroutine;
    end else Error (FailOnlyAllowedWithinConstructors);
end;
```

### Code Block 3

```pascal
Function Proc_Randomize: Word; Far;
begin
  GenerateInstruction_CALL_FAR (SysProc_InitRand);
  Proc_Randomize := EndSubRoutine;
end;
```

### Code Block 4

```pascal
Function Proc_Halt_RunError: Word; Far;
Var ExitCode: TExpression;
begin
  If CheckAndGetNextToken (Token_LeftParenthesis) then
    begin
      ExitCode.ExpectIntegerExpression;
      ExitCode.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
      ExpectTokenAndGetNext (Token_RightParenthesis);
      ExitCode.Calculate;
      ExitCode.LoadExpressionToRegisters (urAX);
    end else GenerateInstruction_Word (XOR_AX_AX);
  GenerateInstruction_CALL_FAR (ProcParameter);
  Proc_Halt_RunError := EndSubroutine;
end;
```

### Code Block 5

```pascal
Function Proc_Fail: Word; Far;
begin
  If pfConstructor in PProcedureIdentifierData (Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset))^.Flags then
    begin
      GenerateCodeForNearJump (LastJumpToFail, JMP_ShortDirect);
      Proc_Fail := EndSubroutine;
    end else Error (FailOnlyAllowedWithinConstructors);
end;
```

### Code Block 6

```pascal
Function Proc_Randomize: Word; Far;
begin
  GenerateInstruction_CALL_FAR (SysProc_InitRand);
  Proc_Randomize := EndSubRoutine;
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
- https://turbopascal.org/system-procedures
- https://turbopascal.org/system-procedures-halt-runerror-fail-and-randomize
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

