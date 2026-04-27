# File Operations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/file-operations

---

## Table of Contents

- File Operations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
File Operations
Turbo Pascal needs to create, read from and write to files. Few simple routins take care for this.
Procedure OpenFile (Var FileToOpen: File; FileName: PChar);
Var Error: Integer;
begin
  Assign (FileToOpen, FileName);
{$I-}
  Repeat
    Reset (FileToOpen, 1);
    Error := IOResult;
    Case Error of
      0: Exit;
      2: FileError (FileName, FileNotFound);
      4: If not CloseOneSourceFile then FileError (FileName, TooManyOpenFiles);
      5: FileError (FileName, FileAccessDenied);
      else FileError (FileName, InvalidFilename);
    end;
  until False;
{$I+}
end;

Procedure CreateFile (Var FileToCreate: File; FileName: PChar);
Var Error: Integer;
begin
  Assign (FileToCreate, FileName);
{$I-}
  Repeat
    Rewrite (FileToCreate, 1);
    Error := IOResult;
    Case Error of
      0: Exit;
      2: FileError (FileName, FileNotFound);
      4: If not CloseOneSourceFile then FileError (FileName, TooManyOpenFiles);
      5: FileError (FileName, FileAccessDenied);
      else FileError (FileName, InvalidFilename);
    end;
  until False;
{$I+}
end;

Procedure DeleteFile (FileName: PChar);
Var TempFile: File;
begin
  Assign (TempFile, FileName);
  {$I-}
    Close (TempFile);
    Erase (TempFile);
  {$I+}
end;
If there are too many files open Turbo Pascal tries to close them one by one and repeat requested file operation before reporting an error.
Function CloseOneSourceFile: Boolean;
Var CurrentFileStructPtr, NextFileStructPtr: PFileStructure;
    TempHandle: Word;
begin
  CloseOneSourceFile := False;
  If CurrentSourceFile = @EndOfFileStructure then Exit;
  NextFileStructPtr := Ptr (Seg (FileStructure), CurrentSourceFile^.Name.Len + Ofs (CurrentSourceFile^.Name.Chr));
  If Ofs (NextFileStructPtr^) = Ofs (EndOfFileStructure) then Exit;
  Repeat
    CurrentFileStructPtr := NextFileStructPtr;
    NextFileStructPtr := Ptr (Seg (FileStructure), CurrentFileStructPtr^.Name.Len + Ofs (CurrentFileStructPtr^.Name.Chr));
    If Ofs (NextFileStructPtr^) = Ofs (EndOfFileStructure) then Break;
  until NextFileStructPtr^.Handle = 0;
  TempHandle := CurrentFileStructPtr^.Handle;
  CurrentFileStructPtr^.Handle := 0;
  If TempHandle <> 0 then
    Asm
      MOV  BX, TempHandle
      MOV  AH, $3E
      INT  $21
      CLD
    end;
  CloseOneSourceFile := True;
end;
Turbo Pascal always reads all files as binary ones. It processes them on the lowest level.
Function ReadFile (Var FileToRead: File): Pointer;
Var FSize: Longint;
    BytesRead: Word;
begin
  FSize := FileSize (FileToRead) - FilePos (FileToRead);
  If FSize > 16 * LongInt (HeapEndRec.Seg - HeapPtrRec.Seg) then Error (OutOfMemory);
  ReadFile := HeapPtr;
  Repeat
    BlockRead (FileToRead, HeapPtr^, $FFF0, BytesRead);
    FSize := FSize - BytesRead;
    HeapPtrRec.Seg := HeapPtrRec.Seg + (BytesRead + $000F) shr 4;
  until FSize = 0;

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
Procedure OpenFile (Var FileToOpen: File; FileName: PChar);
Var Error: Integer;
begin
  Assign (FileToOpen, FileName);
{$I-}
  Repeat
    Reset (FileToOpen, 1);
    Error := IOResult;
    Case Error of
      0: Exit;
      2: FileError (FileName, FileNotFound);
      4: If not CloseOneSourceFile then FileError (FileName, TooManyOpenFiles);
      5: FileError (FileName, FileAccessDenied);
      else FileError (FileName, InvalidFilename);
    end;
  until False;
{$I+}
end;

Procedure CreateFile (Var FileToCreate: File; FileName: PChar);
Var Error: Integer;
begin
  Assign (FileToCreate, FileName);
{$I-}
  Repeat
    Rewrite (FileToCreate, 1);
    Error := IOResult;
    Case Error of
      0: Exit;
      2: FileError (FileName, FileNotFound);
      4: If not CloseOneSourceFile then FileError (FileName, TooManyOpenFiles);
      5: FileError (FileName, FileAccessDenied);
      else FileError (FileName, InvalidFilename);
    end;
  until False;
{$I+}
end;

Procedure DeleteFile (FileName: PChar);
Var TempFile: File;
begin
  Assign (TempFile, FileName);
  {$I-}
    Close (TempFile);
    Erase (TempFile);
  {$I+}
end;
```

### Code Block 2

```pascal
Function CloseOneSourceFile: Boolean;
Var CurrentFileStructPtr, NextFileStructPtr: PFileStructure;
    TempHandle: Word;
begin
  CloseOneSourceFile := False;
  If CurrentSourceFile = @EndOfFileStructure then Exit;
  NextFileStructPtr := Ptr (Seg (FileStructure), CurrentSourceFile^.Name.Len + Ofs (CurrentSourceFile^.Name.Chr));
  If Ofs (NextFileStructPtr^) = Ofs (EndOfFileStructure) then Exit;
  Repeat
    CurrentFileStructPtr := NextFileStructPtr;
    NextFileStructPtr := Ptr (Seg (FileStructure), CurrentFileStructPtr^.Name.Len + Ofs (CurrentFileStructPtr^.Name.Chr));
    If Ofs (NextFileStructPtr^) = Ofs (EndOfFileStructure) then Break;
  until NextFileStructPtr^.Handle = 0;
  TempHandle := CurrentFileStructPtr^.Handle;
  CurrentFileStructPtr^.Handle := 0;
  If TempHandle <> 0 then
    Asm
      MOV  BX, TempHandle
      MOV  AH, $3E
      INT  $21
      CLD
    end;
  CloseOneSourceFile := True;
end;
```

### Code Block 3

```pascal
Function ReadFile (Var FileToRead: File): Pointer;
Var FSize: Longint;
    BytesRead: Word;
begin
  FSize := FileSize (FileToRead) - FilePos (FileToRead);
  If FSize > 16 * LongInt (HeapEndRec.Seg - HeapPtrRec.Seg) then Error (OutOfMemory);
  ReadFile := HeapPtr;
  Repeat
    BlockRead (FileToRead, HeapPtr^, $FFF0, BytesRead);
    FSize := FSize - BytesRead;
    HeapPtrRec.Seg := HeapPtrRec.Seg + (BytesRead + $000F) shr 4;
  until FSize = 0;

end;
```

### Code Block 4

```pascal
Procedure OpenFile (Var FileToOpen: File; FileName: PChar);
Var Error: Integer;
begin
  Assign (FileToOpen, FileName);
{$I-}
  Repeat
    Reset (FileToOpen, 1);
    Error := IOResult;
    Case Error of
      0: Exit;
      2: FileError (FileName, FileNotFound);
      4: If not CloseOneSourceFile then FileError (FileName, TooManyOpenFiles);
      5: FileError (FileName, FileAccessDenied);
      else FileError (FileName, InvalidFilename);
    end;
  until False;
{$I+}
end;

Procedure CreateFile (Var FileToCreate: File; FileName: PChar);
Var Error: Integer;
begin
  Assign (FileToCreate, FileName);
{$I-}
  Repeat
    Rewrite (FileToCreate, 1);
    Error := IOResult;
    Case Error of
      0: Exit;
      2: FileError (FileName, FileNotFound);
      4: If not CloseOneSourceFile then FileError (FileName, TooManyOpenFiles);
      5: FileError (FileName, FileAccessDenied);
      else FileError (FileName, InvalidFilename);
    end;
  until False;
{$I+}
end;

Procedure DeleteFile (FileName: PChar);
Var TempFile: File;
begin
  Assign (TempFile, FileName);
  {$I-}
    Close (TempFile);
    Erase (TempFile);
  {$I+}
end;
```

### Code Block 5

```pascal
Function CloseOneSourceFile: Boolean;
Var CurrentFileStructPtr, NextFileStructPtr: PFileStructure;
    TempHandle: Word;
begin
  CloseOneSourceFile := False;
  If CurrentSourceFile = @EndOfFileStructure then Exit;
  NextFileStructPtr := Ptr (Seg (FileStructure), CurrentSourceFile^.Name.Len + Ofs (CurrentSourceFile^.Name.Chr));
  If Ofs (NextFileStructPtr^) = Ofs (EndOfFileStructure) then Exit;
  Repeat
    CurrentFileStructPtr := NextFileStructPtr;
    NextFileStructPtr := Ptr (Seg (FileStructure), CurrentFileStructPtr^.Name.Len + Ofs (CurrentFileStructPtr^.Name.Chr));
    If Ofs (NextFileStructPtr^) = Ofs (EndOfFileStructure) then Break;
  until NextFileStructPtr^.Handle = 0;
  TempHandle := CurrentFileStructPtr^.Handle;
  CurrentFileStructPtr^.Handle := 0;
  If TempHandle <> 0 then
    Asm
      MOV  BX, TempHandle
      MOV  AH, $3E
      INT  $21
      CLD
    end;
  CloseOneSourceFile := True;
end;
```

### Code Block 6

```pascal
Function ReadFile (Var FileToRead: File): Pointer;
Var FSize: Longint;
    BytesRead: Word;
begin
  FSize := FileSize (FileToRead) - FilePos (FileToRead);
  If FSize > 16 * LongInt (HeapEndRec.Seg - HeapPtrRec.Seg) then Error (OutOfMemory);
  ReadFile := HeapPtr;
  Repeat
    BlockRead (FileToRead, HeapPtr^, $FFF0, BytesRead);
    FSize := FSize - BytesRead;
    HeapPtrRec.Seg := HeapPtrRec.Seg + (BytesRead + $000F) shr 4;
  until FSize = 0;

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
- https://turbopascal.org/file-operations
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

