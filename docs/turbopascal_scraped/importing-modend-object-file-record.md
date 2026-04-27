# Importing MODEND Object File Record - Turbo Pascal Compiler

**URL:** https://turbopascal.org/importing-modend-object-file-record

---

## Table of Contents

- Importing MODEND Object File Record

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Importing MODEND Object File Record
This procedure processes
MODEND
object file record (
$8A
) and updates statement code size symbol table.
Procedure OMF_MODEND; Far;
Var TempPtr: PtrRec;
    StatementCodeSize, NumberOfSourceLines: Word;
begin

{$IFDEF DEBUGOMF}
  Writeln ('        MODEND');
{$ENDIF}

  If SymbolTable [stSourceLineCodeOffsets].UsedSize <> LastBlock_stSourceLineCodeOffsets_Size then
    begin
      TempPtr.Ptr := IncreaseSymbolTable (stSourceLineCodeOffsets, 2);
      StatementCodeSize := SymbolTable [stCode].UsedSize - LastProgramBlockCodeSize - Current_OMF_CodeSize;
      If StatementCodeSize >= $80 then
        begin
          Byte (TempPtr.Ptr^) := Hi (StatementCodeSize) or $80;
          Inc (PChar (TempPtr));
        end;
      Byte (TempPtr.Ptr^) := Lo (StatementCodeSize);
      Inc (PChar (TempPtr));
      SymbolTable [stSourceLineCodeOffsets].UsedSize := TempPtr.Ofs;
      TempPtr.Ofs := LastBlock_stSourceLineCodeOffsets_Size + 8;
      NumberOfSourceLines := Current_OMF_SourceLineNumber - Word (TempPtr.Ptr^) + 1;
      Inc (TempPtr.Ofs, 2);
      Word (TempPtr.Ptr^) := NumberOfSourceLines;
    end;
  Return := True;
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
Procedure OMF_MODEND; Far;
Var TempPtr: PtrRec;
    StatementCodeSize, NumberOfSourceLines: Word;
begin

{$IFDEF DEBUGOMF}
  Writeln ('        MODEND');
{$ENDIF}

  If SymbolTable [stSourceLineCodeOffsets].UsedSize <> LastBlock_stSourceLineCodeOffsets_Size then
    begin
      TempPtr.Ptr := IncreaseSymbolTable (stSourceLineCodeOffsets, 2);
      StatementCodeSize := SymbolTable [stCode].UsedSize - LastProgramBlockCodeSize - Current_OMF_CodeSize;
      If StatementCodeSize >= $80 then
        begin
          Byte (TempPtr.Ptr^) := Hi (StatementCodeSize) or $80;
          Inc (PChar (TempPtr));
        end;
      Byte (TempPtr.Ptr^) := Lo (StatementCodeSize);
      Inc (PChar (TempPtr));
      SymbolTable [stSourceLineCodeOffsets].UsedSize := TempPtr.Ofs;
      TempPtr.Ofs := LastBlock_stSourceLineCodeOffsets_Size + 8;
      NumberOfSourceLines := Current_OMF_SourceLineNumber - Word (TempPtr.Ptr^) + 1;
      Inc (TempPtr.Ofs, 2);
      Word (TempPtr.Ptr^) := NumberOfSourceLines;
    end;
  Return := True;
end;
```

### Code Block 2

```pascal
Procedure OMF_MODEND; Far;
Var TempPtr: PtrRec;
    StatementCodeSize, NumberOfSourceLines: Word;
begin

{$IFDEF DEBUGOMF}
  Writeln ('        MODEND');
{$ENDIF}

  If SymbolTable [stSourceLineCodeOffsets].UsedSize <> LastBlock_stSourceLineCodeOffsets_Size then
    begin
      TempPtr.Ptr := IncreaseSymbolTable (stSourceLineCodeOffsets, 2);
      StatementCodeSize := SymbolTable [stCode].UsedSize - LastProgramBlockCodeSize - Current_OMF_CodeSize;
      If StatementCodeSize >= $80 then
        begin
          Byte (TempPtr.Ptr^) := Hi (StatementCodeSize) or $80;
          Inc (PChar (TempPtr));
        end;
      Byte (TempPtr.Ptr^) := Lo (StatementCodeSize);
      Inc (PChar (TempPtr));
      SymbolTable [stSourceLineCodeOffsets].UsedSize := TempPtr.Ofs;
      TempPtr.Ofs := LastBlock_stSourceLineCodeOffsets_Size + 8;
      NumberOfSourceLines := Current_OMF_SourceLineNumber - Word (TempPtr.Ptr^) + 1;
      Inc (TempPtr.Ofs, 2);
      Word (TempPtr.Ptr^) := NumberOfSourceLines;
    end;
  Return := True;
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
- https://turbopascal.org/importing-modend-object-file-record
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

