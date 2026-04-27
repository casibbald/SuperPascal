# System Functions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-functions/page/1

---

## Table of Contents

    - System Functions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Functions
This is another unit that is used by the
Expressions
unit which processes system functions like
Abs
,
UpCase
,
Sqr
,
Succ
,
Pred
, etc.
Processing System Functions
All Turbo Pascal system functions are processed by this procedure. Each system function has a processing procedure and an optional parameter. This parameter is used when two or more functions share the same processing procedure.
System Function New
This procedure processes system function New. It uses a common procedure for New as function and procedure.
System Functions for Text Files
This procedure processes system functions SeekEoln, SeekEof, Eoln for text files and Eof for all files.
System Functions FilePos and FileSize
This procedure processes system functions FilePos and FileSize. It expects file variable as parameter and generates a call to appropriate compiler function passed as parameter to this procedure.
System Function UpCase
This procedure processes system function UpCase.
System Function Abs
This procedure processes system function Abs. It simply converts negative numbers to positive.
System Function Sqr
This procedure processes system function Sqr. It generates code to multiply the expression with itself.
System Functions Succ and Pred
This procedure processes system functions Succ and Pred.
System Function Odd
This procedure processes system function Odd.
System Functions Ord and Chr
This procedure processes system function Ord. It simply converts expression type to LongInt.
System Functions Ptr, Addr, Ofs and Seg
These procedures process system function Ptr, Addr, Seg and Ofs.
System Functions for CPU Registers
This procedure processes system functions CSeg, DSeg, SSeg and SPtr. It generates code to load CPU register to AX and sets expression to LongInt type.
System Function SizeOf
This procedure processes system function SizeOf.
Arithmetic System Functions
This procedure processes system functions Int, Frac, Sqrt, Sin, Cos, Ln, Exp and ArcTan.
System Functions MemAvail and MaxAvail
This procedure processes system functions MemAvail and MaxAvail. It generates a call to compiler routine that returns value according to the heap status and sets the expression type to LongInt.
String System Functions
These procedures process system functions Pos, Length, Copy and Concat.
I/O System Functions
These procedures process system functions IOResult, ParamStr and ParamCount.
System Function Random
This procedure processes system function Random.
System Functions Lo, Hi, Swap and Pi
These procedures process system functions Lo, Hi, Swap and Pi.
System Function TypeOf
This procedure processes system function TypeOf.
1
2
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
- https://turbopascal.org/arithmetic-system-functions
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/expressions
- https://turbopascal.org/i-o-system-functions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-system-functions
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/string-system-functions
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/system-function-abs
- https://turbopascal.org/system-function-new
- https://turbopascal.org/system-function-odd
- https://turbopascal.org/system-function-random
- https://turbopascal.org/system-function-sizeof
- https://turbopascal.org/system-function-sqr
- https://turbopascal.org/system-function-typeof
- https://turbopascal.org/system-function-upcase
- https://turbopascal.org/system-functions
- https://turbopascal.org/system-functions-filepos-and-filesize
- https://turbopascal.org/system-functions-for-cpu-registers
- https://turbopascal.org/system-functions-for-text-files
- https://turbopascal.org/system-functions-lo-hi-swap-and-pi
- https://turbopascal.org/system-functions-memavail-and-maxavail
- https://turbopascal.org/system-functions-ord-and-chr
- https://turbopascal.org/system-functions-ptr-addr-ofs-and-seg
- https://turbopascal.org/system-functions-succ-and-pred
- https://turbopascal.org/system-functions/page/1
- https://turbopascal.org/system-functions/page/2
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

