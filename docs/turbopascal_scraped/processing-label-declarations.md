# Processing Label Declarations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-label-declarations

---

## Table of Contents

- Processing Label Declarations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Label Declarations
Processing Label declarations is trivial. They are simply
stored into symbol table
as identifiers.
Procedure ProcessLabelDeclarations;
Var Id: PIdentifier;
    AddData: Pointer;
begin
  GetNextToken;
  Repeat
    CheckForNumericLabel;
    Id := ExpectAndStoreIdentifier (2, AddData);
    Id^.Token := Token_LabelIdentifier;
  until not CheckAndGetNextToken (Token_Comma);
  ExpectTokenAndGetNext (TOKEN_Semicolon);
end;
This procedure reads numeric label, checks limits and converts it to string identifier with
hash value
.
Procedure CheckForNumericLabel;
Var N: Byte;
begin
  If Token <> Token_Constant then Exit;
  If ConstantTypeDefRec.Ofs <> LongInt_TypeOffset then Exit;
  If NumericConstant.LongRec.WordH <> 0 then Exit;
  If NumericConstant.LongRec.WordL > 9999 then Exit;
  Str (NumericConstant.LongRec.WordL: 4, CurrentIdentifier);
  ShortInt (CurrentIdentifierHash) :=  - 4;
  For N := 1 to 4 do
    begin
      If CurrentIdentifier [N] = ' ' then CurrentIdentifier [N] := '0';
      Inc (CurrentIdentifierHash, Byte (CurrentIdentifier [N]) and $DF);
    end;
  CurrentIdentifierHash := CurrentIdentifierHash shl 1;
  Token := Token_Identifier;
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
Procedure ProcessLabelDeclarations;
Var Id: PIdentifier;
    AddData: Pointer;
begin
  GetNextToken;
  Repeat
    CheckForNumericLabel;
    Id := ExpectAndStoreIdentifier (2, AddData);
    Id^.Token := Token_LabelIdentifier;
  until not CheckAndGetNextToken (Token_Comma);
  ExpectTokenAndGetNext (TOKEN_Semicolon);
end;
```

### Code Block 2

```pascal
Procedure CheckForNumericLabel;
Var N: Byte;
begin
  If Token <> Token_Constant then Exit;
  If ConstantTypeDefRec.Ofs <> LongInt_TypeOffset then Exit;
  If NumericConstant.LongRec.WordH <> 0 then Exit;
  If NumericConstant.LongRec.WordL > 9999 then Exit;
  Str (NumericConstant.LongRec.WordL: 4, CurrentIdentifier);
  ShortInt (CurrentIdentifierHash) :=  - 4;
  For N := 1 to 4 do
    begin
      If CurrentIdentifier [N] = ' ' then CurrentIdentifier [N] := '0';
      Inc (CurrentIdentifierHash, Byte (CurrentIdentifier [N]) and $DF);
    end;
  CurrentIdentifierHash := CurrentIdentifierHash shl 1;
  Token := Token_Identifier;
end;
```

### Code Block 3

```pascal
Procedure ProcessLabelDeclarations;
Var Id: PIdentifier;
    AddData: Pointer;
begin
  GetNextToken;
  Repeat
    CheckForNumericLabel;
    Id := ExpectAndStoreIdentifier (2, AddData);
    Id^.Token := Token_LabelIdentifier;
  until not CheckAndGetNextToken (Token_Comma);
  ExpectTokenAndGetNext (TOKEN_Semicolon);
end;
```

### Code Block 4

```pascal
Procedure CheckForNumericLabel;
Var N: Byte;
begin
  If Token <> Token_Constant then Exit;
  If ConstantTypeDefRec.Ofs <> LongInt_TypeOffset then Exit;
  If NumericConstant.LongRec.WordH <> 0 then Exit;
  If NumericConstant.LongRec.WordL > 9999 then Exit;
  Str (NumericConstant.LongRec.WordL: 4, CurrentIdentifier);
  ShortInt (CurrentIdentifierHash) :=  - 4;
  For N := 1 to 4 do
    begin
      If CurrentIdentifier [N] = ' ' then CurrentIdentifier [N] := '0';
      Inc (CurrentIdentifierHash, Byte (CurrentIdentifier [N]) and $DF);
    end;
  CurrentIdentifierHash := CurrentIdentifierHash shl 1;
  Token := Token_Identifier;
end;
```

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
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-label-declarations
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/storing-identifiers-into-symbol-table
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

