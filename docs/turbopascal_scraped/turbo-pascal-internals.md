# Compiler Internals - Turbo Pascal Compiler

**URL:** https://turbopascal.org/turbo-pascal-internals

---

## Table of Contents

- Compiler Internals
      - Basic Turbo Pascal Internals
      - Turbo Pascal Internal Structure

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Compiler Internals
Turbo Pascal compiler functions are grouped into several categories/units according to their role in the compiler. This grouping is done only to have a better overview on the individual parts of the compiler. On the other hand, functions from one group usually share common types and variables and therefore it makes sense to place them in separate units.
Basic Turbo Pascal Internals
Unit files in Turbo Pascal (tpu extension) are actually
symbol tables
that are
compacted
and
saved
as individual files. The
System unit
is implicitly used in every program or unit. It contains the boot-strap symbol table and compiler procedures. The definition order of these compiler procedures is important because compiler calls them by id number. To compile
System unit
you need bootstrap symbol table (
SYSTEM.TPS
).
Boot-strap symbol table contains
system types
like
Byte
,
Char
,
Boolean
, port identifiers, memory identifiers, system functions and system procedures.
Turbo Pascal library (extension
tpl
) is simple binary concatenation of one or more units. It is loaded at the compiler start. It should contain at least the system unit. You can create unit with console command copy:
copy /b  system.tpu  unit1.tpu  unit2.tpu  turbo.tpl
Turbo Pascal uses low-level
intermediate code
. Each record can contain target instruction with reference data, intermediate code instruction for subroutines or special meta instruction.
Turbo Pascal relies heavily on the
segment:offset
architecture of the x86 family in the real mode. In many cases this is a limiting factor because many data structures are limited to 64 KB. But on the other hand this comes very convenient when dealing with addresses and offsets.
Turbo Pascal Internal Structure
Compiler
This category contains the main program, common compiler variables, and everything else that does not belong into other categories.
Scanner
Scanner unit contains functions that processes source files and compiler directives and extracts tokens -
basic elements of the language
.
Symbol Tables
Symbol tables are core part of every compiler. Turbo Pascal uses linked lists and hasing to effectively store and retrieve identifiers. Functions in this unit take care for data storing, identifier searching and various symbol table management.
Parser
Parser processes the main program and units, checks syntax, processes stream of tokens and generates intermediate code. This is where the core compiler functions are located.
Expressions
Expression in Turbo Pascal is everything from constant, variable, calculation or just an identifier. Expressions are made up of operators and operands. Most Pascal operators are binary; they take two operands. The rest are unary and take only one operand. This unit contains over 100 functions to process every possible Turbo Pascal expression.
Calculator
This unit is used by the
Expressions unit
and contains functions that process calculations with one or two operands and calculation operation. This unit actually generates the code for addition, subtraction, multiplication, division, shifts, etc.
Statements
This unit contains files that process each Pascal statement:
If
,
While
,
For
,
Repeat
,
Case
,
With
,
GoTo
,
Inline
,
Asm
block, or system procedure.
Assembler
Assembler unit processes assembly instructions in the Asm-end block and generates code for them.
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
System Procedures
This unit contains functions to process system procedures like
Write
,
Writeln
,
Assign
,
Dispose
,
Delete
, etc.
Type Definitions
Type definitions unit defines data structures for basic types and contains few functions to process type definitions.
Object Files
This unit imports and processes object files and generates intermediate code for OMF records.
Code Generator
This unit processes intermediate code and generates executable code and reference records for Linker.
Linker
Linker joins code from all used units, determines addresses of variables, functions and procedures, resolves references and generates executable file.
I/O Utilities
Turbo Pascal contains many functions that read or write files, handle error messages and take care for compiler operation.
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
copy /b  system.tpu  unit1.tpu  unit2.tpu  turbo.tpl
```

### Code Block 2

```pascal
copy /b  system.tpu  unit1.tpu  unit2.tpu  turbo.tpl
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compacting-symbol-tables
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-unit-file
- https://turbopascal.org/expressions
- https://turbopascal.org/intermediate-code
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/system-functions
- https://turbopascal.org/system-procedures
- https://turbopascal.org/system-type-definitions
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

