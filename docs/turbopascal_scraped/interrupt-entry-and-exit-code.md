# Interrupt Entry and Exit Code - Turbo Pascal Compiler

**URL:** https://turbopascal.org/interrupt-entry-and-exit-code

---

## Table of Contents

- Interrupt Entry and Exit Code

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Interrupt Entry and Exit Code
Interrupt procedures also need some initialization and finalization. Turbo Pascal generates interrupt entry code that saves registers,
creates stack frame
and sets
DS
register to data segment.
Function GenerateInterruptEntryCode: Word;
begin
  GenerateInstruction_TwoBytes (PUSH_AX, PUSH_BX);
  GenerateInstruction_TwoBytes (PUSH_CX, PUSH_DX);
  GenerateInstruction_TwoBytes (PUSH_SI, PUSH_DI);
  GenerateInstruction_TwoBytes (PUSH_DS, PUSH_ES);
  CreateStackFrame;
  GenerateInstruction_Byte (MOV_AX_Immediate);
  GenerateReference (SymbolTable [stMain].Segment, 0, 0, [rfDataSegment, rfSegment]);
  GenerateInstruction_Word (MOV_DS_AX);
  GenerateInterruptEntryCode := EndSubroutine
end;
Interrupt exit code reverses the proces: it
destroys stack frame
, restores registers and returns from the interrupt.
Function GenerateInterruptExitCode: Word;
begin
  DestroyStackFrame;
  GenerateInstruction_TwoBytes (POP_ES, POP_DS);
  GenerateInstruction_TwoBytes (POP_DI, POP_SI);
  GenerateInstruction_TwoBytes (POP_DX, POP_CX);
  GenerateInstruction_TwoBytes (POP_BX, POP_AX);
  GenerateInstruction_Byte (IRET);
  GenerateInterruptExitCode := EndSubroutine;
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
Function GenerateInterruptEntryCode: Word;
begin
  GenerateInstruction_TwoBytes (PUSH_AX, PUSH_BX);
  GenerateInstruction_TwoBytes (PUSH_CX, PUSH_DX);
  GenerateInstruction_TwoBytes (PUSH_SI, PUSH_DI);
  GenerateInstruction_TwoBytes (PUSH_DS, PUSH_ES);
  CreateStackFrame;
  GenerateInstruction_Byte (MOV_AX_Immediate);
  GenerateReference (SymbolTable [stMain].Segment, 0, 0, [rfDataSegment, rfSegment]);
  GenerateInstruction_Word (MOV_DS_AX);
  GenerateInterruptEntryCode := EndSubroutine
end;
```

### Code Block 2

```pascal
Function GenerateInterruptExitCode: Word;
begin
  DestroyStackFrame;
  GenerateInstruction_TwoBytes (POP_ES, POP_DS);
  GenerateInstruction_TwoBytes (POP_DI, POP_SI);
  GenerateInstruction_TwoBytes (POP_DX, POP_CX);
  GenerateInstruction_TwoBytes (POP_BX, POP_AX);
  GenerateInstruction_Byte (IRET);
  GenerateInterruptExitCode := EndSubroutine;
end;
```

### Code Block 3

```pascal
Function GenerateInterruptEntryCode: Word;
begin
  GenerateInstruction_TwoBytes (PUSH_AX, PUSH_BX);
  GenerateInstruction_TwoBytes (PUSH_CX, PUSH_DX);
  GenerateInstruction_TwoBytes (PUSH_SI, PUSH_DI);
  GenerateInstruction_TwoBytes (PUSH_DS, PUSH_ES);
  CreateStackFrame;
  GenerateInstruction_Byte (MOV_AX_Immediate);
  GenerateReference (SymbolTable [stMain].Segment, 0, 0, [rfDataSegment, rfSegment]);
  GenerateInstruction_Word (MOV_DS_AX);
  GenerateInterruptEntryCode := EndSubroutine
end;
```

### Code Block 4

```pascal
Function GenerateInterruptExitCode: Word;
begin
  DestroyStackFrame;
  GenerateInstruction_TwoBytes (POP_ES, POP_DS);
  GenerateInstruction_TwoBytes (POP_DI, POP_SI);
  GenerateInstruction_TwoBytes (POP_DX, POP_CX);
  GenerateInstruction_TwoBytes (POP_BX, POP_AX);
  GenerateInstruction_Byte (IRET);
  GenerateInterruptExitCode := EndSubroutine;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-and-destroying-stack-frame
- https://turbopascal.org/expressions
- https://turbopascal.org/interrupt-entry-and-exit-code
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

