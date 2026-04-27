# Parser - Turbo Pascal Compiler

**URL:** https://turbopascal.org/parser

---

## Table of Contents

    - Parser

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Parser
Parser processes the main program and units, checks syntax, processes stream of tokens and generates intermediate code. This is where the core compiler functions are located.
Processing Declarations
This procedure processes Pascal declarations: Constants, Types, Variables, Procedures, Functions, and if declarations are not in the Interface part of the unit, also Lables, Constructors and Destructors are processed. Before declarations are processes, statement compiler switches are reset to module compiler switches. This procedure is called from program compilation, unit interface and implementation compilation and …
Processing Constant Declarations
Turbo Pascal constants can be of two types: ordinary constants which are only stored to the symbol table and typed constants which are also embedded into the program. Therefore, typed constant values are stored into a separate symbol table.
Processing Typed Constants
This procedure processes and stores the value of typed constants. For complex types like Arrays, Records and Objects recursion is used to process elements and fields.
Processing Type Declarations
This procedure processes type declarations – identifiers that denote types. Object types have their own processing.
Processing Non-object Types
This procedure processes all types except Objects and creates type definition records. One one hand objects need different and more complex processing and on the other hand they can not be declared anywhere else as in the program, interface or implementation part. This procedure is used also to process types in variable declarations, ordinal types …
Processing Object Types
Turbo Pascal includes many functions and procedures to process object type declaration. The reason is in the complexity of object type. It can contain fields, methods, constructors, destructors, private and public members, static methods, dynamic methods, virtual methods, etc.
Processing Variable Declarations
This procedure expects a list of identifiers followed by a colon and variable type and processes declaration of variables: global, local and absolute.
Processing Procedure Declarations
Turbo Pascal uses few procedures to process declaration of procedures and functions. The same code is also used to process methods, constructors and destructors. The first procedure processes header.
Processing Label Declarations
Processing Label declarations is trivial. They are simply stored into symbol table as identifiers.
Processing Program Block
This is where the main compilation routines are called. This procedure resets many compiler flags and variables, initializes intermediate code, saves source line number, resets compiler switches, and processes statements between begin and end tokes (or asm and end for assembler procedures). After the compilation, module entry and exit code is added. Of course, for …
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
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/non-object-types
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-constant-declarations
- https://turbopascal.org/processing-declarations
- https://turbopascal.org/processing-label-declarations
- https://turbopascal.org/processing-object-types
- https://turbopascal.org/processing-procedure-declarations
- https://turbopascal.org/processing-program-block
- https://turbopascal.org/processing-type-declarations
- https://turbopascal.org/processing-typed-constants
- https://turbopascal.org/processing-variable-declarations
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

