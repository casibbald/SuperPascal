# Error Reporting - Turbo Pascal Compiler

**URL:** https://turbopascal.org/error-reporting

---

## Table of Contents

- Error Reporting

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Error Reporting
Turbo Pascal reports errors with
module name, source line
and error position where the error occured. This is very handy to locate the problem. This information can also be used in some text editors to position the cursor where the compiler reported error.
For some errors also short additional information like identifier name is displayed.
WriteCompilationError
procedure takes care for this.
Procedure WriteCompilationError;
Var N: Integer;
begin
  If Assigned (ErrorSourceFile) then
    begin
      If LastError = CompilationAborted then
        Asm                                           { Move one row up to compensate ^C#13#10 }
          MOV   AH, 3
          XOR   BH, BH
          PUSH  SI
          PUSH  DI
          PUSH  BP
          PUSH  ES
          INT   $10
          DEC   DH
          MOV   AH, 2
          XOR   BH, BH
          INT   $10
          POP   ES
          POP   BP
          POP   DI
          POP   SI
        end;
      WriteModuleNameWithCurrentLineNumber (ErrorSourceFile);
      Write (': ');
    end;
  If LastError <> NoError then Write ('Error ', Ord (LastError), ': ');
  Write (CompilerErrorString [LastError]);
  If Assigned (AdditionalErrorStr) then Write (' (', AdditionalErrorStr, ')');
  Writeln ('.');

  If LastError = CompilationAborted then Halt ($FF);

  If Assigned (ErrorSourceFile) then
    begin
      Writeln (CurrentLine);
      For N := 0 to ErrorSourceFile^.CurrentPosition - 1 - Ofs (CurrentLine^) do
        Case CurrentLine [N] of
          #9: Write (#9);
          else Write (' ');
        end;
      Writeln ('^');
    end;
end;
This procedure reports error and halts the compiler.
begin
  If LastError = CompilationAborted then Halt ($FF);
  WriteCompilationError;
  Halt (1);
end;
There are also few additional error procedures that display additional error information.
Procedure CurrentIdentifierError (Err: TCompilerError);
begin
  AdditionalErrorStr := @Identifier;
  StrPCopy (@Identifier, CurrentIdentifier);
  Error (Err);
end;
This procedure copies string paremeter as additional error information.
Procedure IdentifierError (Const Id: String; Err: TCompilerError);
begin
  AdditionalErrorStr := @Identifier;
  StrPCopy (@Identifier, Id);
  Error (Err);
end;
FileError reports error with file name as additional error string.
Procedure FileError (FileName: Pchar; Err: TCompilerError);
begin
  StrCopy (NameStr, FileName);
  AdditionalErrorStr := NameStr;
  Error (Err);
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
Procedure WriteCompilationError;
Var N: Integer;
begin
  If Assigned (ErrorSourceFile) then
    begin
      If LastError = CompilationAborted then
        Asm                                           { Move one row up to compensate ^C#13#10 }
          MOV   AH, 3
          XOR   BH, BH
          PUSH  SI
          PUSH  DI
          PUSH  BP
          PUSH  ES
          INT   $10
          DEC   DH
          MOV   AH, 2
          XOR   BH, BH
          INT   $10
          POP   ES
          POP   BP
          POP   DI
          POP   SI
        end;
      WriteModuleNameWithCurrentLineNumber (ErrorSourceFile);
      Write (': ');
    end;
  If LastError <> NoError then Write ('Error ', Ord (LastError), ': ');
  Write (CompilerErrorString [LastError]);
  If Assigned (AdditionalErrorStr) then Write (' (', AdditionalErrorStr, ')');
  Writeln ('.');

  If LastError = CompilationAborted then Halt ($FF);

  If Assigned (ErrorSourceFile) then
    begin
      Writeln (CurrentLine);
      For N := 0 to ErrorSourceFile^.CurrentPosition - 1 - Ofs (CurrentLine^) do
        Case CurrentLine [N] of
          #9: Write (#9);
          else Write (' ');
        end;
      Writeln ('^');
    end;
end;
```

### Code Block 2

```pascal
begin
  If LastError = CompilationAborted then Halt ($FF);
  WriteCompilationError;
  Halt (1);
end;
```

### Code Block 3

```pascal
Procedure CurrentIdentifierError (Err: TCompilerError);
begin
  AdditionalErrorStr := @Identifier;
  StrPCopy (@Identifier, CurrentIdentifier);
  Error (Err);
end;
```

### Code Block 4

```pascal
Procedure IdentifierError (Const Id: String; Err: TCompilerError);
begin
  AdditionalErrorStr := @Identifier;
  StrPCopy (@Identifier, Id);
  Error (Err);
end;
```

### Code Block 5

```pascal
Procedure FileError (FileName: Pchar; Err: TCompilerError);
begin
  StrCopy (NameStr, FileName);
  AdditionalErrorStr := NameStr;
  Error (Err);
end;
```

### Code Block 6

```pascal
WriteCompilationError
```

### Code Block 7

```pascal
Procedure WriteCompilationError;
Var N: Integer;
begin
  If Assigned (ErrorSourceFile) then
    begin
      If LastError = CompilationAborted then
        Asm                                           { Move one row up to compensate ^C#13#10 }
          MOV   AH, 3
          XOR   BH, BH
          PUSH  SI
          PUSH  DI
          PUSH  BP
          PUSH  ES
          INT   $10
          DEC   DH
          MOV   AH, 2
          XOR   BH, BH
          INT   $10
          POP   ES
          POP   BP
          POP   DI
          POP   SI
        end;
      WriteModuleNameWithCurrentLineNumber (ErrorSourceFile);
      Write (': ');
    end;
  If LastError <> NoError then Write ('Error ', Ord (LastError), ': ');
  Write (CompilerErrorString [LastError]);
  If Assigned (AdditionalErrorStr) then Write (' (', AdditionalErrorStr, ')');
  Writeln ('.');

  If LastError = CompilationAborted then Halt ($FF);

  If Assigned (ErrorSourceFile) then
    begin
      Writeln (CurrentLine);
      For N := 0 to ErrorSourceFile^.CurrentPosition - 1 - Ofs (CurrentLine^) do
        Case CurrentLine [N] of
          #9: Write (#9);
          else Write (' ');
        end;
      Writeln ('^');
    end;
end;
```

### Code Block 8

```pascal
begin
  If LastError = CompilationAborted then Halt ($FF);
  WriteCompilationError;
  Halt (1);
end;
```

### Code Block 9

```pascal
Procedure CurrentIdentifierError (Err: TCompilerError);
begin
  AdditionalErrorStr := @Identifier;
  StrPCopy (@Identifier, CurrentIdentifier);
  Error (Err);
end;
```

### Code Block 10

```pascal
Procedure IdentifierError (Const Id: String; Err: TCompilerError);
begin
  AdditionalErrorStr := @Identifier;
  StrPCopy (@Identifier, Id);
  Error (Err);
end;
```

### Code Block 11

```pascal
Procedure FileError (FileName: Pchar; Err: TCompilerError);
begin
  StrCopy (NameStr, FileName);
  AdditionalErrorStr := NameStr;
  Error (Err);
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
- https://turbopascal.org/error-reporting
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

