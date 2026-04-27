# Assembler - Turbo Pascal Compiler

**URL:** https://turbopascal.org/assembler

---

## Table of Contents

    - Assembler

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Assembler
Assembler unit processes assembly instructions in the Asm-end block and generates code for them.
Processing Assembler Instructions
This procedure processes one assembler instruction. It resets data used for instruction processing, reads next assembler token, checks for label and processes prefix instructions (if present). Then it checks and processes assembler directive (DB, DW or DD) or assembler instruction parameters. Finally it generates intermediate code for assembler instruction.
Extracting Assembler Tokens
This procedure scans assembler instruction and returns the next assembler token.
Identifiers In Assembler Instructions
This function checks if identifier in instruction is valid and returns its data. Identifier can be a local lable starting with the @ sign or any other declared Turbo Pascal identifier.
Processing Assembler Expressions
Assembler expressions (instruction parameters) are processed similarly to Pascal expressions. The expression is divided into smaller parts according to the operator precedence. For assembler expressions Turbo Pascal uses few data structures and procedures. TAsmExpression contains fields that hold data for each expression (instruction parameter).
Processing Assembler Directives
This function checks and processes assembler directives DB, DW and DD. More then one byte, word or double word can be separated by comma.
Generating Assembler Instruction Opcodes
Assembler instructions (in fact mnemonics) are stored in a symbol table where each instruction has few data bytes which define which procedure will process it and what combinations of parameters are possible.
Intermediate Code For Assembler Instructions
This procedure generates intermediate code from opcodes and references generated for current assembler instruction.
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
- https://turbopascal.org/extracting-assembler-tokens
- https://turbopascal.org/generating-assembler-instruction-opcodes
- https://turbopascal.org/identifiers-in-assembler-instructions
- https://turbopascal.org/intermediate-code-for-assembler-instructions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-assembler-directives
- https://turbopascal.org/processing-assembler-expressions
- https://turbopascal.org/processing-assembler-instructions
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

