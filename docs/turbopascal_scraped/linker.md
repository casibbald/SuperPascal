# Linker - Turbo Pascal Compiler

**URL:** https://turbopascal.org/linker

---

## Table of Contents

    - Linker

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Linker
Linker joins code from all used units, determines addresses of variables, functions and procedures, resolves references and generates executable file.
Linking Modules and Creating EXE File
This is the last phase of program compilation – linking all modules and creating the executable file for program. Turbo Pascal first marks all symbol table blocks for variables, typed constants, procedures and code blocks as unused, then it iterates through all modules and marks used blocks starting from the main program, after that it …
Marking Used Blocks
Turbo Pascal uses “smart” linker which eleminates code and variables that are never referenced. This reduces code size and memory usage. First all symbol table blocks in all modules that reference variables, typed constants and code blocks are marked as unused. Finally, the main program block is marked as referenced since it will be executed …
Calculating Offsets of Code Blocks
This procedure calculates offset of each program code block and total code size.
Calculating Offsets of Variables
This procedure calculates offset in data segment for each block of typed constants and variables and checks total data size.
Resolving References
Once the offsets of code blocks, variables and typed constants are known, the compiler can resolve references. This procedure calculates all references (displacements, offsets, near and far pointers) to code or data.
Creating Executable File
This procedure creates space in heap for exe header and typed constants, creates exe file and positions it to the start of the code.
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

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculating-offsets-of-code-blocks
- https://turbopascal.org/calculating-offsets-of-variables
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-executable-file
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/linking-modules-and-creating-exe-file
- https://turbopascal.org/marking-used-blocks
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/resolving-references
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

