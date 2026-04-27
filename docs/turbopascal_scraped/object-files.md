# Object Files - Turbo Pascal Compiler

**URL:** https://turbopascal.org/object-files

---

## Table of Contents

    - Object Files

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Object Files
This unit imports and processes object files and generates intermediate code for OMF records.
Importing Object Files
Turbo Pascal can import and link object files created with any other compiler that can create Relocatable Object Module Format file. OMF file is a collection of object records with the following format:
Importing COMENT Object File Record
If local debug information is in enabled and comment class equals to ccSourceFile ($E8) this procedure processes object file COMENT record ($88). If source file number is 1, file name is copied to FileBuffer.
Importing MODEND Object File Record
This procedure processes MODEND object file record ($8A) and updates statement code size symbol table.
Importing EXTDEF Object File Record
This procedure processes EXTDEF object file record ($8C) and adds record to ExtrnDefinitions array for each EXTRN definition.
Importing PUBDEF Object File Record
This procedure processes PUBDEF object file record ($90) and sets data in ProceduresRecord to the procedure defined in the object file.
Importing LINNUM Object File Record
This procedure processes LINNUM object file record ($94) and stores statement code sizes to the symbol table.
Importing SEGDEF Object File Record
This procedure processes SEGDEF record ($98), finds appropriate Turbo Pascal segment and creates space in the symbol table for that segment.
Importing FIXUPP Object File Record
This procedure processes FIXUPP record ($9C) and creates reference record in the symbol table.
Importing LEDATA and LIDATA Object File Records
This procedure processes LEDATA object file record ($A0) and copies data from object file to symbol table.
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
- https://turbopascal.org/importing-coment-object-file-record
- https://turbopascal.org/importing-extdef-object-file-record
- https://turbopascal.org/importing-fixupp-object-file-record
- https://turbopascal.org/importing-ledata-and-lidata-object-file-records
- https://turbopascal.org/importing-linnum-object-file-record
- https://turbopascal.org/importing-modend-object-file-record
- https://turbopascal.org/importing-object-files
- https://turbopascal.org/importing-pubdef-object-file-record
- https://turbopascal.org/importing-segdef-object-file-record
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
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

