# Compiler - Turbo Pascal Compiler

**URL:** https://turbopascal.org/compiler

---

## Table of Contents

    - Compiler

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Compiler
This category contains the main program, common compiler variables, and everything else that does not belong into other categories.
Main Program
This is the main program of the compiler. It initializes some data, reads configuration file, processes command line parameters, loads library (reports error if it is not found), initializes module data, checks for the main file, resets compilation timer, compiles the source file and writes compilation status.
Processing Command Line Parameters
This procedure processes command line parameters and sets compiler flags and data accordingly.
Error Handling
Compiler should have the possibility to report error from any function. Turbo Pascal solves this problem by defining procedure SetErrorAddress which saves the stack position and return address.
Error Reporting
Turbo Pascal reports errors with module name, source line and error position where the error occured. This is very handy to locate the problem. This information can also be used in some text editors to position the cursor where the compiler reported error.
Compilation Progress and Status Display
During the compilation Turbo Pascal displays current module and source line number (if command line switch for quiet compilation was not used).
Finding Source Position From Code Address
Turbo Pascal has a command line switch /Fseg:ofs which can find module name and source line number compiled at this address. The program is normally compiled but instead of compilation status the source position is displayed.
Command Line Syntax Display
When no source file is specified this procedure simply displays Turbo Pascal command line syntax with default switch states.
The Compilation Process
This is where the actual compilation starts. Turbo Pascal first clears and prepares some variables and data structures and then compiles the module (unit or program). If program was compiled then compiler checks for indirect references and calls a procedure to link the code and data and to create the executable file.
Processing Used Units
Every Turbo Pascal module (program or unit) can use other units. Processing them is one of the trickiest parts of Turbo Pascal compiler.
Loading Used Units
Each used unit is checked if it is already loaded. If it is not it needs to be found and checked if recompilation is needed
System Unit
This is the Turbo Pascal System unit. This unit implements Turbo Pascal’s runtime library. When compiling this unit, the compiler automatically includes a boot-strap symbol table (SYSTEM.TPS) which declares all built-in types, procedures, and functions.
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
- https://turbopascal.org/command-line-syntax-display
- https://turbopascal.org/compilation-process
- https://turbopascal.org/compilation-progress-and-status-display
- https://turbopascal.org/compiler
- https://turbopascal.org/error-handling
- https://turbopascal.org/error-reporting
- https://turbopascal.org/expressions
- https://turbopascal.org/finding-source-position-from-code-address
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/loading-used-units
- https://turbopascal.org/main-program
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-command-line-parameters
- https://turbopascal.org/processing-used-units
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/system-functions
- https://turbopascal.org/system-procedures
- https://turbopascal.org/system-unit
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

