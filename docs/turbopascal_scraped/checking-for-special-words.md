# Checking For Special Words - Turbo Pascal Compiler

**URL:** https://turbopascal.org/checking-for-special-words

---

## Table of Contents

- Checking For Special Words

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Checking For Special Words
Turbo Pascal uses few special words as directives for special functions:
ABSOLUTE
,
ASSEMBLER
,
EXTERNAL
,
FAR
,
FORWARD
,
INTERRUPT
,
NEAR
,
PRIVATE
,
PUBLIC
,
VIRTUAL
and
System
as unit name.
Type TSpecialWord = Record
                      Text: String [9];
                      Hash: Byte;
                    end;

  _ABSOLUTE: TSpecialWord = (Text: 'ABSOLUTE'; Hash: $AE);
  _ASSEMBLER: TSpecialWord = (Text: 'ASSEMBLER'; Hash: $2A);
  _EXTERNAL: TSpecialWord = (Text: 'EXTERNAL'; Hash: $B6);
  _FAR: TSpecialWord = (Text: 'FAR'; Hash: $AC);
  _FORWARD: TSpecialWord = (Text: 'FORWARD'; Hash: $1C);
  _INTERRUPT: TSpecialWord = (Text: 'INTERRUPT'; Hash: $88);
  _NEAR: TSpecialWord = (Text: 'NEAR'; Hash: $44);
  _PRIVATE: TSpecialWord = (Text: 'PRIVATE'; Hash: $28);
  _PUBLIC: TSpecialWord = (Text: 'PUBLIC'; Hash: $72);
  _VIRTUAL: TSpecialWord = (Text: 'VIRTUAL'; Hash: $40);
  _System: TSpecialWord = (Text: 'System'; Hash: $BE);
This function checks if special directive is present.
Procedure CheckIfDirecive (Directive: TSpecialWord; NewToken: TToken);
begin
  If CompareIdentifierToWord (Directive) then Token := NewToken;
end;
This function actually compares current identifier with each of special words. To speed up the search the
hash value
is compared first, then
IdentifiersEqual
is called.
Function CompareIdentifierToWord (SpecialWord: TSpecialWord): Boolean;
Var TempPtr: PString;
begin
  CompareIdentifierToWord := False;
  If Token <> Token_Identifier then Exit;
  If CurrentIdentifierHash <> SpecialWord.Hash then Exit;
  TempPtr := @SpecialWord.Text;
  If not IdentifiersEqual (@CurrentIdentifier, TempPtr) then Exit;
  CompareIdentifierToWord := True;
end;
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
Type TSpecialWord = Record
                      Text: String [9];
                      Hash: Byte;
                    end;

  _ABSOLUTE: TSpecialWord = (Text: 'ABSOLUTE'; Hash: $AE);
  _ASSEMBLER: TSpecialWord = (Text: 'ASSEMBLER'; Hash: $2A);
  _EXTERNAL: TSpecialWord = (Text: 'EXTERNAL'; Hash: $B6);
  _FAR: TSpecialWord = (Text: 'FAR'; Hash: $AC);
  _FORWARD: TSpecialWord = (Text: 'FORWARD'; Hash: $1C);
  _INTERRUPT: TSpecialWord = (Text: 'INTERRUPT'; Hash: $88);
  _NEAR: TSpecialWord = (Text: 'NEAR'; Hash: $44);
  _PRIVATE: TSpecialWord = (Text: 'PRIVATE'; Hash: $28);
  _PUBLIC: TSpecialWord = (Text: 'PUBLIC'; Hash: $72);
  _VIRTUAL: TSpecialWord = (Text: 'VIRTUAL'; Hash: $40);
  _System: TSpecialWord = (Text: 'System'; Hash: $BE);
```

### Code Block 2

```pascal
Procedure CheckIfDirecive (Directive: TSpecialWord; NewToken: TToken);
begin
  If CompareIdentifierToWord (Directive) then Token := NewToken;
end;
```

### Code Block 3

```pascal
Function CompareIdentifierToWord (SpecialWord: TSpecialWord): Boolean;
Var TempPtr: PString;
begin
  CompareIdentifierToWord := False;
  If Token <> Token_Identifier then Exit;
  If CurrentIdentifierHash <> SpecialWord.Hash then Exit;
  TempPtr := @SpecialWord.Text;
  If not IdentifiersEqual (@CurrentIdentifier, TempPtr) then Exit;
  CompareIdentifierToWord := True;
end;
```

### Code Block 4

```pascal
Type TSpecialWord = Record
                      Text: String [9];
                      Hash: Byte;
                    end;

  _ABSOLUTE: TSpecialWord = (Text: 'ABSOLUTE'; Hash: $AE);
  _ASSEMBLER: TSpecialWord = (Text: 'ASSEMBLER'; Hash: $2A);
  _EXTERNAL: TSpecialWord = (Text: 'EXTERNAL'; Hash: $B6);
  _FAR: TSpecialWord = (Text: 'FAR'; Hash: $AC);
  _FORWARD: TSpecialWord = (Text: 'FORWARD'; Hash: $1C);
  _INTERRUPT: TSpecialWord = (Text: 'INTERRUPT'; Hash: $88);
  _NEAR: TSpecialWord = (Text: 'NEAR'; Hash: $44);
  _PRIVATE: TSpecialWord = (Text: 'PRIVATE'; Hash: $28);
  _PUBLIC: TSpecialWord = (Text: 'PUBLIC'; Hash: $72);
  _VIRTUAL: TSpecialWord = (Text: 'VIRTUAL'; Hash: $40);
  _System: TSpecialWord = (Text: 'System'; Hash: $BE);
```

### Code Block 5

```pascal
Procedure CheckIfDirecive (Directive: TSpecialWord; NewToken: TToken);
begin
  If CompareIdentifierToWord (Directive) then Token := NewToken;
end;
```

### Code Block 6

```pascal
Function CompareIdentifierToWord (SpecialWord: TSpecialWord): Boolean;
Var TempPtr: PString;
begin
  CompareIdentifierToWord := False;
  If Token <> Token_Identifier then Exit;
  If CurrentIdentifierHash <> SpecialWord.Hash then Exit;
  TempPtr := @SpecialWord.Text;
  If not IdentifiersEqual (@CurrentIdentifier, TempPtr) then Exit;
  CompareIdentifierToWord := True;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/checking-for-special-words
- https://turbopascal.org/code-generator
- https://turbopascal.org/comparing-identifiers
- https://turbopascal.org/compiler
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
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

