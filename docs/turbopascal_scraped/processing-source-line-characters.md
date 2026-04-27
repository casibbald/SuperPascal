# Processing Source Line Characters - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-source-line-characters

---

## Table of Contents

- Processing Source Line Characters

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Source Line Characters
The
NextNonCommentCharacter
function reads and processes characters from the source line buffer. It returns pointer to the next non-comment character. The null (
#0
) character means end of line. Turbo Pascal tries to
read the next source line
. If this fails it tries to
return to the previous source file
(if
$I
include file was loaded). Whitespace characters (
#1
to space) are skipped. If comment is detected (‘
{
‘ or ‘
(*
‘ characters) it is
processed
together with compiler directives. In all other cases the function returns pointer to the source line character. This fuction is called by the function that
extracts tokens
.
Function NextNonCommentCharacter: PChar;
Var TempPtr: PChar;
    C: Char;
    TwoChars: TTwoChars;
begin
  TempPtr := Ptr (Seg (CurrentLine^), CurrentSourceFile^.CurrentPosition);
  Repeat
    C := TempPtr^;
    Inc (TempPtr);
    If C = #0 then begin
                     If NextLine then
                       begin
                         TempPtr := CurrentLine;
                         Continue;
                       end;
                     CurrentSourceFile^.CurrentPosition := Ofs (TempPtr^);
                     CreateSourceFilesRecord;
                     ReturnToPreviousSourceFile;
                     TempPtr := Ptr (Seg (CurrentLine^), CurrentSourceFile^.CurrentPosition);
                     Continue;
                   end;
    If C In [#1..' '] then Continue
      else begin
             TwoChars := PTwoChars (TempPtr - 1)^;
             If TwoChars.First = '{' then
               begin
                 ProcessComment (TempPtr, TwoChars.First);
                 Continue;
               end;
             If TwoChars.Both = '(*' then
               begin
                 Inc (TempPtr);
                 ProcessComment (TempPtr, TwoChars.Second);
                 Continue;
               end;
             NextNonCommentCharacter := TempPtr - 1;
             Exit;
           end;
  until False;
  NextNonCommentCharacter := TempPtr - 1;
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
Function NextNonCommentCharacter: PChar;
Var TempPtr: PChar;
    C: Char;
    TwoChars: TTwoChars;
begin
  TempPtr := Ptr (Seg (CurrentLine^), CurrentSourceFile^.CurrentPosition);
  Repeat
    C := TempPtr^;
    Inc (TempPtr);
    If C = #0 then begin
                     If NextLine then
                       begin
                         TempPtr := CurrentLine;
                         Continue;
                       end;
                     CurrentSourceFile^.CurrentPosition := Ofs (TempPtr^);
                     CreateSourceFilesRecord;
                     ReturnToPreviousSourceFile;
                     TempPtr := Ptr (Seg (CurrentLine^), CurrentSourceFile^.CurrentPosition);
                     Continue;
                   end;
    If C In [#1..' '] then Continue
      else begin
             TwoChars := PTwoChars (TempPtr - 1)^;
             If TwoChars.First = '{' then
               begin
                 ProcessComment (TempPtr, TwoChars.First);
                 Continue;
               end;
             If TwoChars.Both = '(*' then
               begin
                 Inc (TempPtr);
                 ProcessComment (TempPtr, TwoChars.Second);
                 Continue;
               end;
             NextNonCommentCharacter := TempPtr - 1;
             Exit;
           end;
  until False;
  NextNonCommentCharacter := TempPtr - 1;
end;
```

### Code Block 2

```pascal
NextNonCommentCharacter
```

### Code Block 3

```pascal
Function NextNonCommentCharacter: PChar;
Var TempPtr: PChar;
    C: Char;
    TwoChars: TTwoChars;
begin
  TempPtr := Ptr (Seg (CurrentLine^), CurrentSourceFile^.CurrentPosition);
  Repeat
    C := TempPtr^;
    Inc (TempPtr);
    If C = #0 then begin
                     If NextLine then
                       begin
                         TempPtr := CurrentLine;
                         Continue;
                       end;
                     CurrentSourceFile^.CurrentPosition := Ofs (TempPtr^);
                     CreateSourceFilesRecord;
                     ReturnToPreviousSourceFile;
                     TempPtr := Ptr (Seg (CurrentLine^), CurrentSourceFile^.CurrentPosition);
                     Continue;
                   end;
    If C In [#1..' '] then Continue
      else begin
             TwoChars := PTwoChars (TempPtr - 1)^;
             If TwoChars.First = '{' then
               begin
                 ProcessComment (TempPtr, TwoChars.First);
                 Continue;
               end;
             If TwoChars.Both = '(*' then
               begin
                 Inc (TempPtr);
                 ProcessComment (TempPtr, TwoChars.Second);
                 Continue;
               end;
             NextNonCommentCharacter := TempPtr - 1;
             Exit;
           end;
  until False;
  NextNonCommentCharacter := TempPtr - 1;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/dealing-with-source-files
- https://turbopascal.org/expressions
- https://turbopascal.org/extracting-tokens
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-comments-and-compiler-directives
- https://turbopascal.org/processing-source-line-characters
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

