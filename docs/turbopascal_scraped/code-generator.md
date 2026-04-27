# Code Generator - Turbo Pascal Compiler

**URL:** https://turbopascal.org/code-generator

---

## Table of Contents

    - Code Generator

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Code Generator
This unit processes intermediate code and generates executable code and reference records for Linker.
Intermediate Code
Most compilers use some kind of intermediate code which is a program representation somewhere betwen the high-level source code and executable code for target processor.
Generating Instructions
Turbo Pascal uses few procedures to generate any x86 instruction needed. In general two intermediate code records are used: one for the actual inctruction opcodes and immediate data and one for references.
Creating and Destroying Stack Frame
Programs created with Turbo Pascal use stack frame. This is part of the processor’s stack space which can be accessed using a dedicated base pointer (BP) register.
Module Entry and Exit Code
Each module (program or unit) needs some initialization and finalization: entry and exit code.
Interrupt Entry and Exit Code
Interrupt procedures also need some initialization and finalization. Turbo Pascal generates interrupt entry code that saves registers, creates stack frame and sets DS register to data segment.
Procedure Entry and Exit Code
Procedures and functions (including methods) also need initialization and finalization code.
Jumps and Labels
Turbo Pascal generates jumps in an interesting way. Since most jumps will jump to the same place, Turbo Pascal keeps them in a linked list.
Creating References
Whenever Turbo Pascal needs to reference anything that doesn’t have fixed location, i.e. its address is not known yet like variables, typed constants or procedures, the compiler generates intermediate code reference record which contains all information to later resolve the reference.
ES:DI Address Loading
Turbo Pascal tries to optimize loading of addresses to ES:DI register pair. For this purpose it uses few intermediate code instructions that track addresses that need to be loaded. The first intermediate code instruction will load local variable address to ES:DI registers.
Optimizing Jumps
Turbo Pascal tries to convert all near jumps to short. This is only possible if jump displacement can be stored in a byte. This procedure is called just before the target code is generated. Since all changes reduce code size and therefore provide new opportunities for optimizations, the function is called in a loop until …
Calculating Code Size and Offsets
Before the target code is generated Turbo Pascal needs to know code size and offsets of instructions. This procedure processes intermediate code instructions and calculates the size of generated code, offsets of near jump instructions, offsets of labels and size of reference records.
Generating Code
This procedure generates target (x86) code from intermediate code instructions. This is the last phase in program block compilation. If debug info is enabled, source line code offsets will be saved into dedicated table, redundant loadings of addresses to ES:DI register pair will be removed and near jumps will be converted to short jumps where …
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
- https://turbopascal.org/calculating-code-size-and-offsets
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-and-destroying-stack-frame
- https://turbopascal.org/creating-references
- https://turbopascal.org/es-di-address-loading
- https://turbopascal.org/expressions
- https://turbopascal.org/generating-code
- https://turbopascal.org/generating-instructions
- https://turbopascal.org/intermediate-code
- https://turbopascal.org/interrupt-entry-and-exit-code
- https://turbopascal.org/io-utilities
- https://turbopascal.org/jumps-and-labels
- https://turbopascal.org/linker
- https://turbopascal.org/module-entry-and-exit-code
- https://turbopascal.org/object-files
- https://turbopascal.org/optimizing-jumps
- https://turbopascal.org/parser
- https://turbopascal.org/procedure-entry-and-exit-code
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

