# Calculator - Turbo Pascal Compiler

**URL:** https://turbopascal.org/calculator

---

## Table of Contents

    - Calculator

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Calculator
This unit is used by the
Expressions unit
and contains functions that process calculations with one or two operands and calculation operation. This unit actually generates the code for addition, subtraction, multiplication, division, shifts, etc.
Processing Mathematical Operations
Turbo Pascal generates code for all mathemathical operations with two operands by the following procedure. The input of this procedure are left and right expression and the operation.
Character Pointer Operations
This procedure checks if operation is PChar ± Integer or PChar ± PChar. If so, it generates code and returns True otherwise it returns False.
Set Operations
Constant sets are located into temprary buffer and expression Value holds offset to this buffer. Non-constant sets are always expanded to the stack frame. This procedure calculates operation between two constant sets.
String Operations
Constant strings are added in the temporary buffer, while all non-constant string operations are performed in stack frame.
Extended Operations
Constant Extended values are always stored in expression Value, while for non-constant expressions numeric constants are stored in code block and FPU instructions are used for calculation. This procedure calculates operation between two constant Extended values.
Real Operations
For Real operations code is always generated. This procedure generates code for operations on Real values. If 80×87 instructions are enabled, it converts both expressions to Extended and generates FPU code otherwise it generates code to call system function.
Boolean Operations
Boolean operations are similar to integer operations. Constant Boolean operations are performed with boolean expressions converted to boolean bytes (0 or 1) and using procedures for integer calculation.
Pointer Operations
Pointer operations are similar to Integer operations. However, only extended Turbo Pascal syntax allows pointer operations other than equality check.
Integer Operations
Many operations in Turbo Pascal can be calculated as integer operations. This applies to pointers, Boolean values, enumerated values and Char values. Constant integer operations are checked for possible overflow.
LongInt Comparison
This procedure generates code to compare two LongInt values. Again, for some special cases shorter code is generated.
Integer Division
This procedure generates code for integer division with constant. Special code is generated if divisor is power of 2.
Integer Multiplication
This procedure generates code to multiply integer with integer. Multiplication with power of 2 is translated to shift, other special cases (0, 1, 3, 5, 6) are also treated separately. Result is stored in specified Register if possible.
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
- https://turbopascal.org/boolean-operations
- https://turbopascal.org/calculator
- https://turbopascal.org/character-pointer-operations
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/expressions
- https://turbopascal.org/extended-operations
- https://turbopascal.org/integer-division
- https://turbopascal.org/integer-multiplication
- https://turbopascal.org/integer-operations
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/longint-comparison
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/pointer-operations
- https://turbopascal.org/processing-mathematical-operations
- https://turbopascal.org/real-operations
- https://turbopascal.org/scanner
- https://turbopascal.org/set-operations
- https://turbopascal.org/statements
- https://turbopascal.org/string-operations
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

