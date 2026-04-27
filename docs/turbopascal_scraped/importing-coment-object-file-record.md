# Importing COMENT Object File Record - Turbo Pascal Compiler

**URL:** https://turbopascal.org/importing-coment-object-file-record

---

## Table of Contents

- Importing COMENT Object File Record

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Importing COMENT Object File Record
If local debug information is in enabled and comment class equals to
ccSourceFile
(
$E8
) this procedure processes object file
COMENT
record (
$88
). If source file number is 1, file name is copied to
FileBuffer
.
Procedure OMF_COMENT; Far;
Var TempSourceFileNumber: Byte;
begin
  If (DebugInformation in ModuleCompilerSwitches) and (ObjectFilePtr^.Comment.Class = ccSourceFile) then
    begin
      TempSourceFileNumber := ObjectFilePtr^.Comment.SourceFileNumber;
      If TempSourceFileNumber = 0 then
        begin
          Inc (LastSourceFileNumber);
          TempSourceFileNumber := LastSourceFileNumber;
        end;
      If (Ofs (ObjectFilePtr^.Comment.SourceFileName) <> ObjectRecordChecksumOffset) and (TempSourceFileNumber = 1) then
        begin
          StrUpper (StrPCopy (FileBuffer, ObjectFilePtr^.Comment.SourceFileName));
          OMF_SourceFileDateTime := LongInt (Pointer (
            PChar (@ObjectFilePtr^.Comment.SourceFileName) + Length (ObjectFilePtr^.Comment.SourceFileName) + 1)^);
          TempSourceFileNumber := 1;
        end;
      SourceFileNumber := TempSourceFileNumber;

{$IFDEF DEBUGOMF}
      Writeln ('        COMENT, Class: ', HexByte (ObjectFilePtr^.Comment.Class), ', Source file name ',
                                          SourceFileNumber, ': ', PChar (@FileBuffer));
{$ENDIF}

    end;
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
Procedure OMF_COMENT; Far;
Var TempSourceFileNumber: Byte;
begin
  If (DebugInformation in ModuleCompilerSwitches) and (ObjectFilePtr^.Comment.Class = ccSourceFile) then
    begin
      TempSourceFileNumber := ObjectFilePtr^.Comment.SourceFileNumber;
      If TempSourceFileNumber = 0 then
        begin
          Inc (LastSourceFileNumber);
          TempSourceFileNumber := LastSourceFileNumber;
        end;
      If (Ofs (ObjectFilePtr^.Comment.SourceFileName) <> ObjectRecordChecksumOffset) and (TempSourceFileNumber = 1) then
        begin
          StrUpper (StrPCopy (FileBuffer, ObjectFilePtr^.Comment.SourceFileName));
          OMF_SourceFileDateTime := LongInt (Pointer (
            PChar (@ObjectFilePtr^.Comment.SourceFileName) + Length (ObjectFilePtr^.Comment.SourceFileName) + 1)^);
          TempSourceFileNumber := 1;
        end;
      SourceFileNumber := TempSourceFileNumber;

{$IFDEF DEBUGOMF}
      Writeln ('        COMENT, Class: ', HexByte (ObjectFilePtr^.Comment.Class), ', Source file name ',
                                          SourceFileNumber, ': ', PChar (@FileBuffer));
{$ENDIF}

    end;
end;
```

### Code Block 2

```pascal
Procedure OMF_COMENT; Far;
Var TempSourceFileNumber: Byte;
begin
  If (DebugInformation in ModuleCompilerSwitches) and (ObjectFilePtr^.Comment.Class = ccSourceFile) then
    begin
      TempSourceFileNumber := ObjectFilePtr^.Comment.SourceFileNumber;
      If TempSourceFileNumber = 0 then
        begin
          Inc (LastSourceFileNumber);
          TempSourceFileNumber := LastSourceFileNumber;
        end;
      If (Ofs (ObjectFilePtr^.Comment.SourceFileName) <> ObjectRecordChecksumOffset) and (TempSourceFileNumber = 1) then
        begin
          StrUpper (StrPCopy (FileBuffer, ObjectFilePtr^.Comment.SourceFileName));
          OMF_SourceFileDateTime := LongInt (Pointer (
            PChar (@ObjectFilePtr^.Comment.SourceFileName) + Length (ObjectFilePtr^.Comment.SourceFileName) + 1)^);
          TempSourceFileNumber := 1;
        end;
      SourceFileNumber := TempSourceFileNumber;

{$IFDEF DEBUGOMF}
      Writeln ('        COMENT, Class: ', HexByte (ObjectFilePtr^.Comment.Class), ', Source file name ',
                                          SourceFileNumber, ': ', PChar (@FileBuffer));
{$ENDIF}

    end;
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
- https://turbopascal.org/importing-coment-object-file-record
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

