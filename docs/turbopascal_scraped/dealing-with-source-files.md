# Dealing With Source Files - Turbo Pascal Compiler

**URL:** https://turbopascal.org/dealing-with-source-files

---

## Table of Contents

- Dealing With Source Files

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Dealing With Source Files
For each source file Turbo Pascal keeps a record with essential data like file handle, file name, current line number, current position, conditional compilation
If
level, etc.
This way it can always reurn to the right position in previous source file (when include files are used). After the source file has changed Turbo Pascal displays
compilation progress
showing current module name and source line number.
Type PFileStructure = ^TFileStructure;
     TFileStructure = Record
                       CurrentPosition: Word;      { 00 }
                       CurrentLineNumber: Word;    { 02 }
                       FilePosition: Longint;      { 04 }
                       Handle: Word;               { 08 }
                       CurrentLineLength: Word;    { 0A }
                       FileRecordOffset: Word;     { 0C }
                       LineCounter: Word;          { 0E }
                       IF_Level: Word;             { 10 }
                       Name: Str64Rec;             { 12 13..XX, Len + NameChars + #0, Len = Length (+ NameChars + #0) }
                     end;

Procedure OpenSourceFile (UsedFilesRecord: PUsedFilesBlockRecord; FileName: PChar);
Var NewFileStructure: PFileStructure;
    NameLength: Byte;
begin
  If ProgramBlockCompilation then Error (IncludeFilesAreNotAllowedHere);
  NameLength := StrLen (FileName) + 1;
  NewFileStructure := Ptr (Seg (CurrentSourceFile^), Ofs (CurrentSourceFile^) - NameLength - 19);
  If Ofs (NewFileStructure^) < Ofs (StartOfSourceFileStructures^) then Error (TooManyFiles);
  NewFileStructure^.FileRecordOffset := Ofs (UsedFilesRecord^);
  NewFileStructure^.Name.Len := NameLength;
  StrCopy (@NewFileStructure^.Name.Chr, FileName);

  If FileRec (SourceFile).Handle <> 0 then Close (SourceFile);

  OpenFile (SourceFile, FileName);
  NewFileStructure^.Handle := FileRec (SourceFile).Handle;
  NewFileStructure^.CurrentPosition := Ofs (CurrentLine^);
  ErrorSourcePosition := CurrentLine;
  NewFileStructure^.CurrentLineNumber := 1;
  NewFileStructure^.FilePosition := 0;
  NewFileStructure^.CurrentLineLength := 0;
  NewFileStructure^.IF_Level := 0;

  CurrentLine^ := #0;
  FileBufferPtr := nil;
  FileBufferEnd := nil;
  CurrentSourceFile := NewFileStructure;
  WriteCompilationProgress (CurrentSourceFile);
end;

Procedure ReturnToPreviousSourceFile;
Var SavedLastLinePosition: Word;
begin
  If ProgramBlockCompilation or (CurrentSourceFile = FirstSourceFile) then Error (UnexpectedEndOfFile);
  If CurrentSourceFile^.IF_Level <> 0 then Error (ENDIF_directiveMissing);
  WritelnModuleNameWithCurrentLineNumber (CurrentSourceFile);
  Close (SourceFile);
  Inc (PChar (CurrentSourceFile), CurrentSourceFile^.Name.Len + 19);
  If CurrentSourceFile <> @EndOfFileStructure then
    With CurrentSourceFile^ do
      begin
        SavedLastLinePosition := CurrentPosition;
        CurrentPosition := Ofs (CurrentLine^);
        ErrorSourcePosition := CurrentLine;
        CurrentLine^ := #0;
        FileBufferPtr := nil;
        FileBufferEnd := nil;
        OpenFile (SourceFile, @Name.Chr);
        Handle := FileRec (SourceFile).Handle;
        Seek (SourceFile, FilePosition);
        If ReadNextLine <> CurrentLineLength then Error (UnexpectedEndOfFile);
        CurrentPosition := SavedLastLinePosition;
        ErrorSourcePosition := PChar (Ptr (Seg (ErrorSourcePosition^), SavedLastLinePosition));
      end;
  WriteCompilationProgress (CurrentSourceFile);
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
Type PFileStructure = ^TFileStructure;
     TFileStructure = Record
                       CurrentPosition: Word;      { 00 }
                       CurrentLineNumber: Word;    { 02 }
                       FilePosition: Longint;      { 04 }
                       Handle: Word;               { 08 }
                       CurrentLineLength: Word;    { 0A }
                       FileRecordOffset: Word;     { 0C }
                       LineCounter: Word;          { 0E }
                       IF_Level: Word;             { 10 }
                       Name: Str64Rec;             { 12 13..XX, Len + NameChars + #0, Len = Length (+ NameChars + #0) }
                     end;

Procedure OpenSourceFile (UsedFilesRecord: PUsedFilesBlockRecord; FileName: PChar);
Var NewFileStructure: PFileStructure;
    NameLength: Byte;
begin
  If ProgramBlockCompilation then Error (IncludeFilesAreNotAllowedHere);
  NameLength := StrLen (FileName) + 1;
  NewFileStructure := Ptr (Seg (CurrentSourceFile^), Ofs (CurrentSourceFile^) - NameLength - 19);
  If Ofs (NewFileStructure^) < Ofs (StartOfSourceFileStructures^) then Error (TooManyFiles);
  NewFileStructure^.FileRecordOffset := Ofs (UsedFilesRecord^);
  NewFileStructure^.Name.Len := NameLength;
  StrCopy (@NewFileStructure^.Name.Chr, FileName);

  If FileRec (SourceFile).Handle <> 0 then Close (SourceFile);

  OpenFile (SourceFile, FileName);
  NewFileStructure^.Handle := FileRec (SourceFile).Handle;
  NewFileStructure^.CurrentPosition := Ofs (CurrentLine^);
  ErrorSourcePosition := CurrentLine;
  NewFileStructure^.CurrentLineNumber := 1;
  NewFileStructure^.FilePosition := 0;
  NewFileStructure^.CurrentLineLength := 0;
  NewFileStructure^.IF_Level := 0;

  CurrentLine^ := #0;
  FileBufferPtr := nil;
  FileBufferEnd := nil;
  CurrentSourceFile := NewFileStructure;
  WriteCompilationProgress (CurrentSourceFile);
end;

Procedure ReturnToPreviousSourceFile;
Var SavedLastLinePosition: Word;
begin
  If ProgramBlockCompilation or (CurrentSourceFile = FirstSourceFile) then Error (UnexpectedEndOfFile);
  If CurrentSourceFile^.IF_Level <> 0 then Error (ENDIF_directiveMissing);
  WritelnModuleNameWithCurrentLineNumber (CurrentSourceFile);
  Close (SourceFile);
  Inc (PChar (CurrentSourceFile), CurrentSourceFile^.Name.Len + 19);
  If CurrentSourceFile <> @EndOfFileStructure then
    With CurrentSourceFile^ do
      begin
        SavedLastLinePosition := CurrentPosition;
        CurrentPosition := Ofs (CurrentLine^);
        ErrorSourcePosition := CurrentLine;
        CurrentLine^ := #0;
        FileBufferPtr := nil;
        FileBufferEnd := nil;
        OpenFile (SourceFile, @Name.Chr);
        Handle := FileRec (SourceFile).Handle;
        Seek (SourceFile, FilePosition);
        If ReadNextLine <> CurrentLineLength then Error (UnexpectedEndOfFile);
        CurrentPosition := SavedLastLinePosition;
        ErrorSourcePosition := PChar (Ptr (Seg (ErrorSourcePosition^), SavedLastLinePosition));
      end;
  WriteCompilationProgress (CurrentSourceFile);
end;
```

### Code Block 2

```pascal
Type PFileStructure = ^TFileStructure;
     TFileStructure = Record
                       CurrentPosition: Word;      { 00 }
                       CurrentLineNumber: Word;    { 02 }
                       FilePosition: Longint;      { 04 }
                       Handle: Word;               { 08 }
                       CurrentLineLength: Word;    { 0A }
                       FileRecordOffset: Word;     { 0C }
                       LineCounter: Word;          { 0E }
                       IF_Level: Word;             { 10 }
                       Name: Str64Rec;             { 12 13..XX, Len + NameChars + #0, Len = Length (+ NameChars + #0) }
                     end;

Procedure OpenSourceFile (UsedFilesRecord: PUsedFilesBlockRecord; FileName: PChar);
Var NewFileStructure: PFileStructure;
    NameLength: Byte;
begin
  If ProgramBlockCompilation then Error (IncludeFilesAreNotAllowedHere);
  NameLength := StrLen (FileName) + 1;
  NewFileStructure := Ptr (Seg (CurrentSourceFile^), Ofs (CurrentSourceFile^) - NameLength - 19);
  If Ofs (NewFileStructure^) < Ofs (StartOfSourceFileStructures^) then Error (TooManyFiles);
  NewFileStructure^.FileRecordOffset := Ofs (UsedFilesRecord^);
  NewFileStructure^.Name.Len := NameLength;
  StrCopy (@NewFileStructure^.Name.Chr, FileName);

  If FileRec (SourceFile).Handle <> 0 then Close (SourceFile);

  OpenFile (SourceFile, FileName);
  NewFileStructure^.Handle := FileRec (SourceFile).Handle;
  NewFileStructure^.CurrentPosition := Ofs (CurrentLine^);
  ErrorSourcePosition := CurrentLine;
  NewFileStructure^.CurrentLineNumber := 1;
  NewFileStructure^.FilePosition := 0;
  NewFileStructure^.CurrentLineLength := 0;
  NewFileStructure^.IF_Level := 0;

  CurrentLine^ := #0;
  FileBufferPtr := nil;
  FileBufferEnd := nil;
  CurrentSourceFile := NewFileStructure;
  WriteCompilationProgress (CurrentSourceFile);
end;

Procedure ReturnToPreviousSourceFile;
Var SavedLastLinePosition: Word;
begin
  If ProgramBlockCompilation or (CurrentSourceFile = FirstSourceFile) then Error (UnexpectedEndOfFile);
  If CurrentSourceFile^.IF_Level <> 0 then Error (ENDIF_directiveMissing);
  WritelnModuleNameWithCurrentLineNumber (CurrentSourceFile);
  Close (SourceFile);
  Inc (PChar (CurrentSourceFile), CurrentSourceFile^.Name.Len + 19);
  If CurrentSourceFile <> @EndOfFileStructure then
    With CurrentSourceFile^ do
      begin
        SavedLastLinePosition := CurrentPosition;
        CurrentPosition := Ofs (CurrentLine^);
        ErrorSourcePosition := CurrentLine;
        CurrentLine^ := #0;
        FileBufferPtr := nil;
        FileBufferEnd := nil;
        OpenFile (SourceFile, @Name.Chr);
        Handle := FileRec (SourceFile).Handle;
        Seek (SourceFile, FilePosition);
        If ReadNextLine <> CurrentLineLength then Error (UnexpectedEndOfFile);
        CurrentPosition := SavedLastLinePosition;
        ErrorSourcePosition := PChar (Ptr (Seg (ErrorSourcePosition^), SavedLastLinePosition));
      end;
  WriteCompilationProgress (CurrentSourceFile);
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compilation-progress-and-status-display
- https://turbopascal.org/compiler
- https://turbopascal.org/dealing-with-source-files
- https://turbopascal.org/expressions
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

