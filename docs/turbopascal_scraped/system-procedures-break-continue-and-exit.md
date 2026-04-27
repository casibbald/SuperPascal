# System Procedures Break, Continue and Exit - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-procedures-break-continue-and-exit

---

## Table of Contents

- System Procedures Break, Continue and Exit

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Procedures Break, Continue and Exit
This short function takes care for three system procedures for loop control:
Break
,
Continue
and
Exit
.
The parameter contains offset to one of
jump linked list
variables: jumps to program block exit (
Exit
), jumps out of block (
Break
) or jumps to next block iteration (
Continue
). All loop control
statements
and
program blocks
create labels to these points so these system procedures simply add one jump to the list.
Function Proc_BREAK_CONTINUE_Exit: Word; Far;
begin
  If PWord (Ptr (DSeg, ProcParameter))^ = $FFFF then Error (NoEnclosing_FOR_WHILE_or_REPEAT_statement);
  GenerateCodeForNearJump (PWord (Ptr (DSeg, ProcParameter))^, JMP_ShortDirect);
  Proc_BREAK_CONTINUE_Exit := EndSubroutine;
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
Function Proc_BREAK_CONTINUE_Exit: Word; Far;
begin
  If PWord (Ptr (DSeg, ProcParameter))^ = $FFFF then Error (NoEnclosing_FOR_WHILE_or_REPEAT_statement);
  GenerateCodeForNearJump (PWord (Ptr (DSeg, ProcParameter))^, JMP_ShortDirect);
  Proc_BREAK_CONTINUE_Exit := EndSubroutine;
end;
```

### Code Block 2

```pascal
Function Proc_BREAK_CONTINUE_Exit: Word; Far;
begin
  If PWord (Ptr (DSeg, ProcParameter))^ = $FFFF then Error (NoEnclosing_FOR_WHILE_or_REPEAT_statement);
  GenerateCodeForNearJump (PWord (Ptr (DSeg, ProcParameter))^, JMP_ShortDirect);
  Proc_BREAK_CONTINUE_Exit := EndSubroutine;
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
- https://turbopascal.org/jumps-and-labels
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-statement-blocks
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/system-functions
- https://turbopascal.org/system-procedures
- https://turbopascal.org/system-procedures-break-continue-and-exit
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

