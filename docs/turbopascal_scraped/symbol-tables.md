# Symbol Tables - Turbo Pascal Compiler

**URL:** https://turbopascal.org/symbol-tables

---

## Table of Contents

    - Symbol Tables

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Symbol Tables
Symbol tables are core part of every compiler. Turbo Pascal uses linked lists and hasing to effectively store and retrieve identifiers. Functions in this unit take care for data storing, identifier searching and various symbol table management.
Searching For Identifiers
This function checks for identifier that should be already declared.
Symbol Tables and Hash Functions
Every compiler stores data in different symbol tables. Symbol table is a special data structure that is designed for easy insertion of new symbol with associated data and quick search through all elements in the table.
Symbol Table Management
Each identifier symbol table must be created before it can be used. Turbo Pascal uses diferent parameters (number of linked lists) for different identifier symbol tables.
Storing Identifiers Into Symbol Table
Storing identifiers into symbol table is simple because of the way how Turbo Pascal organizes symbol tables.
Searching in Symbol Table
This function searches for current identifier in specified symbol table. The speed of this function and the design of symbol tables (hash function) have a significant impact on the speed of the compiler. Hash function splits identifiers into several linked lists so only one list is checked when searching for identifier. However, all the identifiers …
Compacting Symbol Tables
After each module is compiled the symbol tables are compacted to remove unused space.
Checking Public Identifiers
Turbo Pascal calculates a checksum of public identifiers for every unit to quickly check if something has been changed.
Reserved Words
All Turbo Pascal reserved words are stored in a dedicated symbol table.
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
- https://turbopascal.org/checking-public-identifiers
- https://turbopascal.org/code-generator
- https://turbopascal.org/compacting-symbol-tables
- https://turbopascal.org/compiler
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/reserved-words
- https://turbopascal.org/scanner
- https://turbopascal.org/searching-for-identifiers
- https://turbopascal.org/searching-in-symbol-table
- https://turbopascal.org/statements
- https://turbopascal.org/storing-identifiers-into-symbol-table
- https://turbopascal.org/symbol-table-management
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/symbol-tables-and-hash-functions
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

