# Reading Source Lines - Turbo Pascal Compiler

**URL:** https://turbopascal.org/reading-source-lines

---

## Table of Contents

- Reading Source Lines

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Reading Source Lines
Source files are read into buffer from where source lines are extracted.
NextLine
procedure moves to the next source line. If a line was read it increments counters and returns
True
, otherwise if there are no more source lines
NextLine
returns
False
.
Function NextLine: Boolean;
Var LineLength: Word;
begin
  If CurrentSourceFile^.CurrentLineLength <> 0 then
    begin
      Inc (CurrentSourceFile^.CurrentLineNumber);
      Inc (CurrentSourceFile^.FilePosition, CurrentSourceFile^.CurrentLineLength);
    end;
  LineLength := ReadNextLine;
  CurrentSourceFile^.CurrentLineLength := LineLength;
  If LineLength <> 0 then
    begin
      Inc (ModuleSourceLineCounter);
      Inc (LineCounter);
    end;
  CurrentSourceFile^.LineCounter := ModuleSourceLineCounter;
  NextLine := LineLength <> 0;
end;
ReadNextLine
reads next source line from source file and returns its length. This procedure also checks line length and reports ‘
Line Too Long
‘ error if the line has more than 128 characters which is the size of the source line buffer.
Function ReadNextLine: Word;
Var SrcPtr, DestPtr, EndPtr: PChar;
    CurrentChar: Char;
    LineLengthSpace, CurrentLineLength, BytesRead: Word;
begin
  LineLengthSpace := SizeOf (TLine);
  CurrentLineLength := 0;
  SrcPtr := FileBufferPtr;
  DestPtr := CurrentLine;
  Repeat
    EndPtr := FileBufferEnd;
    While SrcPtr <> EndPtr do
      begin
        CurrentChar := SrcPtr^;
        Inc (SrcPtr);
        Inc (CurrentLineLength);
        If CurrentChar in [#0, #10, #13, #26] then
          begin
            Case CurrentChar of
              #0, #13: Continue;
              #26: begin
                     Dec (SrcPtr);
                     Dec (CurrentLineLength);
                   end;
            end;
            DestPtr^ := #0;
            FileBufferPtr := SrcPtr;
            ErrorSourcePosition := CurrentLine;
            ReadNextLine := CurrentLineLength;
            Exit;
          end else
            begin
              DestPtr^ := CurrentChar;
              Inc (DestPtr);
              Dec (LineLengthSpace);
              If LineLengthSpace <> 0 then Continue;
              Dec (DestPtr);
              DestPtr^ := #0;
              FileBufferPtr := SrcPtr;
              ErrorSourcePosition := DestPtr;
              Error (LineTooLong);
            end;
      end;

    BytesRead := 0;
    {$I-}
      BlockRead (SourceFile, FileBuffer, SizeOf (FileBuffer), BytesRead);
    {$I+}
    FileBufferEnd := PChar (@FileBuffer) + BytesRead;
    SrcPtr := @FileBuffer;
  until BytesRead = 0;
  DestPtr^ := #0;
  FileBufferPtr := SrcPtr;
  ErrorSourcePosition := CurrentLine;
  ReadNextLine := CurrentLineLength;
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
Function NextLine: Boolean;
Var LineLength: Word;
begin
  If CurrentSourceFile^.CurrentLineLength <> 0 then
    begin
      Inc (CurrentSourceFile^.CurrentLineNumber);
      Inc (CurrentSourceFile^.FilePosition, CurrentSourceFile^.CurrentLineLength);
    end;
  LineLength := ReadNextLine;
  CurrentSourceFile^.CurrentLineLength := LineLength;
  If LineLength <> 0 then
    begin
      Inc (ModuleSourceLineCounter);
      Inc (LineCounter);
    end;
  CurrentSourceFile^.LineCounter := ModuleSourceLineCounter;
  NextLine := LineLength <> 0;
end;
```

### Code Block 2

```pascal
Function ReadNextLine: Word;
Var SrcPtr, DestPtr, EndPtr: PChar;
    CurrentChar: Char;
    LineLengthSpace, CurrentLineLength, BytesRead: Word;
begin
  LineLengthSpace := SizeOf (TLine);
  CurrentLineLength := 0;
  SrcPtr := FileBufferPtr;
  DestPtr := CurrentLine;
  Repeat
    EndPtr := FileBufferEnd;
    While SrcPtr <> EndPtr do
      begin
        CurrentChar := SrcPtr^;
        Inc (SrcPtr);
        Inc (CurrentLineLength);
        If CurrentChar in [#0, #10, #13, #26] then
          begin
            Case CurrentChar of
              #0, #13: Continue;
              #26: begin
                     Dec (SrcPtr);
                     Dec (CurrentLineLength);
                   end;
            end;
            DestPtr^ := #0;
            FileBufferPtr := SrcPtr;
            ErrorSourcePosition := CurrentLine;
            ReadNextLine := CurrentLineLength;
            Exit;
          end else
            begin
              DestPtr^ := CurrentChar;
              Inc (DestPtr);
              Dec (LineLengthSpace);
              If LineLengthSpace <> 0 then Continue;
              Dec (DestPtr);
              DestPtr^ := #0;
              FileBufferPtr := SrcPtr;
              ErrorSourcePosition := DestPtr;
              Error (LineTooLong);
            end;
      end;

    BytesRead := 0;
    {$I-}
      BlockRead (SourceFile, FileBuffer, SizeOf (FileBuffer), BytesRead);
    {$I+}
    FileBufferEnd := PChar (@FileBuffer) + BytesRead;
    SrcPtr := @FileBuffer;
  until BytesRead = 0;
  DestPtr^ := #0;
  FileBufferPtr := SrcPtr;
  ErrorSourcePosition := CurrentLine;
  ReadNextLine := CurrentLineLength;
end;
```

### Code Block 3

```pascal
Function NextLine: Boolean;
Var LineLength: Word;
begin
  If CurrentSourceFile^.CurrentLineLength <> 0 then
    begin
      Inc (CurrentSourceFile^.CurrentLineNumber);
      Inc (CurrentSourceFile^.FilePosition, CurrentSourceFile^.CurrentLineLength);
    end;
  LineLength := ReadNextLine;
  CurrentSourceFile^.CurrentLineLength := LineLength;
  If LineLength <> 0 then
    begin
      Inc (ModuleSourceLineCounter);
      Inc (LineCounter);
    end;
  CurrentSourceFile^.LineCounter := ModuleSourceLineCounter;
  NextLine := LineLength <> 0;
end;
```

### Code Block 4

```pascal
Function ReadNextLine: Word;
Var SrcPtr, DestPtr, EndPtr: PChar;
    CurrentChar: Char;
    LineLengthSpace, CurrentLineLength, BytesRead: Word;
begin
  LineLengthSpace := SizeOf (TLine);
  CurrentLineLength := 0;
  SrcPtr := FileBufferPtr;
  DestPtr := CurrentLine;
  Repeat
    EndPtr := FileBufferEnd;
    While SrcPtr <> EndPtr do
      begin
        CurrentChar := SrcPtr^;
        Inc (SrcPtr);
        Inc (CurrentLineLength);
        If CurrentChar in [#0, #10, #13, #26] then
          begin
            Case CurrentChar of
              #0, #13: Continue;
              #26: begin
                     Dec (SrcPtr);
                     Dec (CurrentLineLength);
                   end;
            end;
            DestPtr^ := #0;
            FileBufferPtr := SrcPtr;
            ErrorSourcePosition := CurrentLine;
            ReadNextLine := CurrentLineLength;
            Exit;
          end else
            begin
              DestPtr^ := CurrentChar;
              Inc (DestPtr);
              Dec (LineLengthSpace);
              If LineLengthSpace <> 0 then Continue;
              Dec (DestPtr);
              DestPtr^ := #0;
              FileBufferPtr := SrcPtr;
              ErrorSourcePosition := DestPtr;
              Error (LineTooLong);
            end;
      end;

    BytesRead := 0;
    {$I-}
      BlockRead (SourceFile, FileBuffer, SizeOf (FileBuffer), BytesRead);
    {$I+}
    FileBufferEnd := PChar (@FileBuffer) + BytesRead;
    SrcPtr := @FileBuffer;
  until BytesRead = 0;
  DestPtr^ := #0;
  FileBufferPtr := SrcPtr;
  ErrorSourcePosition := CurrentLine;
  ReadNextLine := CurrentLineLength;
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
- https://turbopascal.org/reading-source-lines
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

