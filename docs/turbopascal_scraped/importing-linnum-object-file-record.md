# Importing LINNUM Object File Record - Turbo Pascal Compiler

**URL:** https://turbopascal.org/importing-linnum-object-file-record

---

## Table of Contents

- Importing LINNUM Object File Record

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Importing LINNUM Object File Record
This procedure processes
LINNUM
object file record (
$94
) and stores statement code sizes to the symbol table.
Procedure OMF_LINNUM; Far;
Var UsedFilesRecord: PUsedFilesBlockRecord;
    TempPtr: Pointer;

  Procedure StoreStatementCodeSizes;
  Var CurrentCodeSize, CurrentSourceLineNumber, StatementCodeSize, SourceLineNumberDifference, Previous_OMF_CodeSize: Word;
  begin
    Repeat
      CurrentSourceLineNumber := Word (Pointer (ObjectFilePtr)^);
      Inc (PChar (ObjectFilePtr), 2);
      CurrentCodeSize := Word (Pointer (ObjectFilePtr)^);
      Inc (PChar (ObjectFilePtr), 2);
      SourceLineNumberDifference := CurrentSourceLineNumber - Current_OMF_SourceLineNumber;
      If SourceLineNumberDifference &gt; 0 then
        begin
          Previous_OMF_CodeSize        := Current_OMF_CodeSize;
          Current_OMF_SourceLineNumber := CurrentSourceLineNumber;
          Current_OMF_CodeSize         := CurrentCodeSize;
          StatementCodeSize := CurrentCodeSize - Previous_OMF_CodeSize;
          TempPtr := IncreaseSymbolTable (stSourceLineCodeOffsets, SourceLineNumberDifference + 1);
          If StatementCodeSize &gt;= $0080 then
            begin
              Byte (TempPtr^) := Hi (StatementCodeSize) or $80;
              Inc (PChar (TempPtr));
            end;
          Byte (TempPtr^) := Lo (StatementCodeSize);
          Inc (PChar (TempPtr));
          FillChar (TempPtr^, SourceLineNumberDifference - 1, 0);
          SymbolTable [stSourceLineCodeOffsets].UsedSize := Ofs (TempPtr) + SourceLineNumberDifference - 1;
        end;
    until Ofs (ObjectFilePtr^) = ObjectRecordChecksumOffset;
  end;

begin

{$IFDEF DEBUGOMF}
  Writeln ('        LINNUM');
{$ENDIF}

  If SourceFileNumber &lt;&gt; 1 then Exit;
  Inc (PChar (ObjectFilePtr), 4);
  If Byte (Pointer (ObjectFilePtr)^) &lt;&gt; CodeSegmentIndex then Exit;;
  Inc (PChar (ObjectFilePtr));
  If SymbolTable [stSourceLineCodeOffsets].UsedSize &lt;&gt; LastBlock_stSourceLineCodeOffsets_Size then
    begin
      StoreStatementCodeSizes;
      Exit;
    end;
  Current_OMF_SourceLineNumber := Word (Pointer (ObjectFilePtr)^);
  Inc (PChar (ObjectFilePtr), 2);
  Current_OMF_CodeSize := Word (Pointer (ObjectFilePtr)^);
  Inc (PChar (ObjectFilePtr), 2);
  UsedFilesRecord := StoreFileNameToFilesBlock (FileBuffer, ufIncludeFile);
  UsedFilesRecord^.L3 := OMF_SourceFileDateTime;
  TempPtr := IncreaseSymbolTable (stSourceLineCodeOffsets, 12);
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^))^) := 0;
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 2)^) := Ofs (UsedFilesRecord^);
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 4)^) := 0;
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 6)^) := Current_OMF_CodeSize;
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 8)^) := Current_OMF_SourceLineNumber;
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 10)^) := 0;
  If Ofs (ObjectFilePtr^) &gt;= ObjectRecordChecksumOffset then Exit;
  StoreStatementCodeSizes;
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
Procedure OMF_LINNUM; Far;
Var UsedFilesRecord: PUsedFilesBlockRecord;
    TempPtr: Pointer;

  Procedure StoreStatementCodeSizes;
  Var CurrentCodeSize, CurrentSourceLineNumber, StatementCodeSize, SourceLineNumberDifference, Previous_OMF_CodeSize: Word;
  begin
    Repeat
      CurrentSourceLineNumber := Word (Pointer (ObjectFilePtr)^);
      Inc (PChar (ObjectFilePtr), 2);
      CurrentCodeSize := Word (Pointer (ObjectFilePtr)^);
      Inc (PChar (ObjectFilePtr), 2);
      SourceLineNumberDifference := CurrentSourceLineNumber - Current_OMF_SourceLineNumber;
      If SourceLineNumberDifference &gt; 0 then
        begin
          Previous_OMF_CodeSize        := Current_OMF_CodeSize;
          Current_OMF_SourceLineNumber := CurrentSourceLineNumber;
          Current_OMF_CodeSize         := CurrentCodeSize;
          StatementCodeSize := CurrentCodeSize - Previous_OMF_CodeSize;
          TempPtr := IncreaseSymbolTable (stSourceLineCodeOffsets, SourceLineNumberDifference + 1);
          If StatementCodeSize &gt;= $0080 then
            begin
              Byte (TempPtr^) := Hi (StatementCodeSize) or $80;
              Inc (PChar (TempPtr));
            end;
          Byte (TempPtr^) := Lo (StatementCodeSize);
          Inc (PChar (TempPtr));
          FillChar (TempPtr^, SourceLineNumberDifference - 1, 0);
          SymbolTable [stSourceLineCodeOffsets].UsedSize := Ofs (TempPtr) + SourceLineNumberDifference - 1;
        end;
    until Ofs (ObjectFilePtr^) = ObjectRecordChecksumOffset;
  end;

begin

{$IFDEF DEBUGOMF}
  Writeln ('        LINNUM');
{$ENDIF}

  If SourceFileNumber &lt;&gt; 1 then Exit;
  Inc (PChar (ObjectFilePtr), 4);
  If Byte (Pointer (ObjectFilePtr)^) &lt;&gt; CodeSegmentIndex then Exit;;
  Inc (PChar (ObjectFilePtr));
  If SymbolTable [stSourceLineCodeOffsets].UsedSize &lt;&gt; LastBlock_stSourceLineCodeOffsets_Size then
    begin
      StoreStatementCodeSizes;
      Exit;
    end;
  Current_OMF_SourceLineNumber := Word (Pointer (ObjectFilePtr)^);
  Inc (PChar (ObjectFilePtr), 2);
  Current_OMF_CodeSize := Word (Pointer (ObjectFilePtr)^);
  Inc (PChar (ObjectFilePtr), 2);
  UsedFilesRecord := StoreFileNameToFilesBlock (FileBuffer, ufIncludeFile);
  UsedFilesRecord^.L3 := OMF_SourceFileDateTime;
  TempPtr := IncreaseSymbolTable (stSourceLineCodeOffsets, 12);
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^))^) := 0;
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 2)^) := Ofs (UsedFilesRecord^);
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 4)^) := 0;
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 6)^) := Current_OMF_CodeSize;
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 8)^) := Current_OMF_SourceLineNumber;
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 10)^) := 0;
  If Ofs (ObjectFilePtr^) &gt;= ObjectRecordChecksumOffset then Exit;
  StoreStatementCodeSizes;
end;
```

### Code Block 2

```pascal
Procedure OMF_LINNUM; Far;
Var UsedFilesRecord: PUsedFilesBlockRecord;
    TempPtr: Pointer;

  Procedure StoreStatementCodeSizes;
  Var CurrentCodeSize, CurrentSourceLineNumber, StatementCodeSize, SourceLineNumberDifference, Previous_OMF_CodeSize: Word;
  begin
    Repeat
      CurrentSourceLineNumber := Word (Pointer (ObjectFilePtr)^);
      Inc (PChar (ObjectFilePtr), 2);
      CurrentCodeSize := Word (Pointer (ObjectFilePtr)^);
      Inc (PChar (ObjectFilePtr), 2);
      SourceLineNumberDifference := CurrentSourceLineNumber - Current_OMF_SourceLineNumber;
      If SourceLineNumberDifference &gt; 0 then
        begin
          Previous_OMF_CodeSize        := Current_OMF_CodeSize;
          Current_OMF_SourceLineNumber := CurrentSourceLineNumber;
          Current_OMF_CodeSize         := CurrentCodeSize;
          StatementCodeSize := CurrentCodeSize - Previous_OMF_CodeSize;
          TempPtr := IncreaseSymbolTable (stSourceLineCodeOffsets, SourceLineNumberDifference + 1);
          If StatementCodeSize &gt;= $0080 then
            begin
              Byte (TempPtr^) := Hi (StatementCodeSize) or $80;
              Inc (PChar (TempPtr));
            end;
          Byte (TempPtr^) := Lo (StatementCodeSize);
          Inc (PChar (TempPtr));
          FillChar (TempPtr^, SourceLineNumberDifference - 1, 0);
          SymbolTable [stSourceLineCodeOffsets].UsedSize := Ofs (TempPtr) + SourceLineNumberDifference - 1;
        end;
    until Ofs (ObjectFilePtr^) = ObjectRecordChecksumOffset;
  end;

begin

{$IFDEF DEBUGOMF}
  Writeln ('        LINNUM');
{$ENDIF}

  If SourceFileNumber &lt;&gt; 1 then Exit;
  Inc (PChar (ObjectFilePtr), 4);
  If Byte (Pointer (ObjectFilePtr)^) &lt;&gt; CodeSegmentIndex then Exit;;
  Inc (PChar (ObjectFilePtr));
  If SymbolTable [stSourceLineCodeOffsets].UsedSize &lt;&gt; LastBlock_stSourceLineCodeOffsets_Size then
    begin
      StoreStatementCodeSizes;
      Exit;
    end;
  Current_OMF_SourceLineNumber := Word (Pointer (ObjectFilePtr)^);
  Inc (PChar (ObjectFilePtr), 2);
  Current_OMF_CodeSize := Word (Pointer (ObjectFilePtr)^);
  Inc (PChar (ObjectFilePtr), 2);
  UsedFilesRecord := StoreFileNameToFilesBlock (FileBuffer, ufIncludeFile);
  UsedFilesRecord^.L3 := OMF_SourceFileDateTime;
  TempPtr := IncreaseSymbolTable (stSourceLineCodeOffsets, 12);
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^))^) := 0;
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 2)^) := Ofs (UsedFilesRecord^);
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 4)^) := 0;
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 6)^) := Current_OMF_CodeSize;
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 8)^) := Current_OMF_SourceLineNumber;
  Word (Ptr (Seg (TempPtr^), Ofs (TempPtr^) + 10)^) := 0;
  If Ofs (ObjectFilePtr^) &gt;= ObjectRecordChecksumOffset then Exit;
  StoreStatementCodeSizes;
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
- https://turbopascal.org/importing-linnum-object-file-record
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

