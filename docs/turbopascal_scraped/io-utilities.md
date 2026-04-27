# I/O Utilities - Turbo Pascal Compiler

**URL:** https://turbopascal.org/io-utilities

---

## Table of Contents

    - I/O Utilities

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
I/O Utilities
Turbo Pascal contains many functions that read or write files, handle error messages and take care for compiler operation.
Loading Library
This procedure loads Turbo Pascal library (a collection of units). It adjust each unit so that it starts on a segment boundary. Actually, each compiled unit is a group of symbol tables.
Finding Files
urbo Pascal needs to access files that may be located in different directories – located where the compiler is run or specified either in the configuration file or at the command line.
File Operations
Turbo Pascal needs to create, read from and write to files. Few simple routins take care for this.
Dealing With Source Files
For each source file Turbo Pascal keeps a record with essential data like file handle, file name, current line number, current position, conditional compilation If level, etc.
Reading Source Lines
Source files are read into buffer from where source lines are extracted. NextLine procedure moves to the next source line. If a line was read it increments counters and returns True, otherwise if there are no more source lines NextLine returns False.
Creating Unit File
CompileUnit calls this procedure to create unit file which is in fact a collection of symbol tables. All segment addresses are cleared and symbol table data is written to the file.
Reading Unit File
This short procedure reads unit from file to symbol tables. If header is not valid it reports error.
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
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-unit-file
- https://turbopascal.org/dealing-with-source-files
- https://turbopascal.org/expressions
- https://turbopascal.org/file-operations
- https://turbopascal.org/finding-files
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/loading-library
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/reading-source-lines
- https://turbopascal.org/reading-unit-file
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

