# Scanner - Turbo Pascal Compiler

**URL:** https://turbopascal.org/scanner

---

## Table of Contents

    - Scanner

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Scanner
Scanner unit contains functions that processes source files and compiler directives and extracts tokens –
basic elements of the language
.
Processing Source Line Characters
The NextNonCommentCharacter function reads and processes characters from the source line buffer. It returns pointer to the next non-comment character. The null (#0) character means end of line. Turbo Pascal tries to read the next source line. If this fails it tries to return to the previous source file (if $I include file was loaded). …
Processing Comments and Compiler Directives
This procedure processes compiler directives and skips comment. Here also conditional compilation directives (with support for nested directives) are processed.
Conditional Defines
Turbo Pascal contains few procedures and functions that create and check conditional defines.  The first function creates initial conditional defines if certain conditions are met: MSDOS, VER70, CPU86 and CPU87.
Extracting Tokens
Parser operates only with tokens – basic elements of the language. Tokens like identifiers and constants have also additional data structures where actual data is stored.
Processing Integer Constants
These two functions check and process decimal and hexadecimal integer constants. The value is stored in the IntegerNumericConstant variable.
Processing String Constants
This function checks and processes string constants. It processes characters enclosed with single quotes, control characters and characters defined with the ASCII code (#code).
Checking For Special Words
Turbo Pascal uses few special words as directives for special functions: ABSOLUTE, ASSEMBLER, EXTERNAL, FAR, FORWARD, INTERRUPT, NEAR, PRIVATE, PUBLIC, VIRTUAL and System as unit name.
Comparing Identifiers
This function case insensitive compares two identifiers and returns false if they are not equal. Otherwise returns true and Id2 points to byte after its end. This function must be as fast as possible! It is called whenever Turbo Pascal needs to compare two identifiers.
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
- https://turbopascal.org/checking-for-special-words
- https://turbopascal.org/code-generator
- https://turbopascal.org/comparing-identifiers
- https://turbopascal.org/compiler
- https://turbopascal.org/conditional-defines
- https://turbopascal.org/expressions
- https://turbopascal.org/extracting-tokens
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-comments-and-compiler-directives
- https://turbopascal.org/processing-integer-constants
- https://turbopascal.org/processing-source-line-characters
- https://turbopascal.org/processing-string-constants
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

