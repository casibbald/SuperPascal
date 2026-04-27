# Compilation Progress and Status Display - Turbo Pascal Compiler

**URL:** https://turbopascal.org/compilation-progress-and-status-display

---

## Table of Contents

- Compilation Progress and Status Display

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Compilation Progress and Status Display
During the compilation Turbo Pascal displays current module and source line number (if command line switch for quiet compilation was not used).
Procedure WriteCompilationProgress (LastFile: PFileStructure);
begin
  If QuietCompile then Exit;
  If LastFile = @EndOfFileStructure then Exit;
  WriteModuleNameWithCurrentLineNumber (LastFile);
  Asm
    MOV AH, $0B
    INT $21
    CLD
  end
end;
This procedure checks if there is enough memory for stack and writes compilation progress. It is called before
moving to the next token
and before
processing each statement
.
Procedure CheckStack;
begin
  If (SPtr - $200) < StackLimit then Error (InternalStackOverflow);
  If QuietCompile then Exit;
  If InterruptCounter - StatusReportInterruptCounter < 5 then Exit;
  StatusReportInterruptCounter := InterruptCounter;
  WriteCompilationProgress (CurrentSourceFile);
end;
This procedure wites current module name and source line number. Since the text is displayed on the same line the procedure also overwrites the remaining old text with spaces.
Error reporting
also uses this procedure.
Procedure WriteModuleNameWithCurrentLineNumber (LastFile: PFileStructure);
Var StringDifference: Integer;
    LineNumberStr: String [10];
    N: Byte;
begin
  If LastStringLength <> 0 then Write (#13);
  Str (LastFile^.CurrentLineNumber, LineNumberStr);
  Write (PChar (@LastFile^.Name.Chr), '(', LastFile^.CurrentLineNumber, ')');
  StringDifference := LastStringLength - (Length (LineNumberStr) + LastFile^.Name.Len + 2);
  LastStringLength := Length (LineNumberStr) + LastFile^.Name.Len + 2;
  If StringDifference <= 0 then Exit;
  For N := 1 to StringDifference do Write (' ');
  For N := 1 to StringDifference do Write (#8);
end;
At the end of successful compilation Turbo Pascal reports the time taken to compile and the size of the compiled program.
Procedure WriteCompilationStatus;
Const TimeConstant = 1 / 1.82064673;
Var   Time: Longint;
begin
  Write (LineCounter, ' lines, ');
  Time := Round (TimeConstant * (StopCounter - StartCounter));
  If Time <> 0 then Write (Time div 10, '.', Time mod 10, ' seconds, ');
  Writeln (CodeBytes, ' bytes code, ', DataBytes, ' bytes data.');
  Asm
    MOV AH, $0B
    INT $21
    CLD
  end
end;
This procedure is called when Turbo Pascal finishes compiling a module.
Procedure WritelnModuleNameWithCurrentLineNumber (LastFile: Pointer);
begin
  If QuietCompile then Exit;
  WriteModuleNameWithCurrentLineNumber (LastFile);
  Writeln;
  Asm
    MOV AH, $0B
    INT $21
    CLD
  end
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
Procedure WriteCompilationProgress (LastFile: PFileStructure);
begin
  If QuietCompile then Exit;
  If LastFile = @EndOfFileStructure then Exit;
  WriteModuleNameWithCurrentLineNumber (LastFile);
  Asm
    MOV AH, $0B
    INT $21
    CLD
  end
end;
```

### Code Block 2

```pascal
Procedure CheckStack;
begin
  If (SPtr - $200) < StackLimit then Error (InternalStackOverflow);
  If QuietCompile then Exit;
  If InterruptCounter - StatusReportInterruptCounter < 5 then Exit;
  StatusReportInterruptCounter := InterruptCounter;
  WriteCompilationProgress (CurrentSourceFile);
end;
```

### Code Block 3

```pascal
Procedure WriteModuleNameWithCurrentLineNumber (LastFile: PFileStructure);
Var StringDifference: Integer;
    LineNumberStr: String [10];
    N: Byte;
begin
  If LastStringLength <> 0 then Write (#13);
  Str (LastFile^.CurrentLineNumber, LineNumberStr);
  Write (PChar (@LastFile^.Name.Chr), '(', LastFile^.CurrentLineNumber, ')');
  StringDifference := LastStringLength - (Length (LineNumberStr) + LastFile^.Name.Len + 2);
  LastStringLength := Length (LineNumberStr) + LastFile^.Name.Len + 2;
  If StringDifference <= 0 then Exit;
  For N := 1 to StringDifference do Write (' ');
  For N := 1 to StringDifference do Write (#8);
end;
```

### Code Block 4

```pascal
Procedure WriteCompilationStatus;
Const TimeConstant = 1 / 1.82064673;
Var   Time: Longint;
begin
  Write (LineCounter, ' lines, ');
  Time := Round (TimeConstant * (StopCounter - StartCounter));
  If Time <> 0 then Write (Time div 10, '.', Time mod 10, ' seconds, ');
  Writeln (CodeBytes, ' bytes code, ', DataBytes, ' bytes data.');
  Asm
    MOV AH, $0B
    INT $21
    CLD
  end
end;
```

### Code Block 5

```pascal
Procedure WritelnModuleNameWithCurrentLineNumber (LastFile: Pointer);
begin
  If QuietCompile then Exit;
  WriteModuleNameWithCurrentLineNumber (LastFile);
  Writeln;
  Asm
    MOV AH, $0B
    INT $21
    CLD
  end
end;
```

### Code Block 6

```pascal
Procedure WriteCompilationProgress (LastFile: PFileStructure);
begin
  If QuietCompile then Exit;
  If LastFile = @EndOfFileStructure then Exit;
  WriteModuleNameWithCurrentLineNumber (LastFile);
  Asm
    MOV AH, $0B
    INT $21
    CLD
  end
end;
```

### Code Block 7

```pascal
Procedure CheckStack;
begin
  If (SPtr - $200) < StackLimit then Error (InternalStackOverflow);
  If QuietCompile then Exit;
  If InterruptCounter - StatusReportInterruptCounter < 5 then Exit;
  StatusReportInterruptCounter := InterruptCounter;
  WriteCompilationProgress (CurrentSourceFile);
end;
```

### Code Block 8

```pascal
Procedure WriteModuleNameWithCurrentLineNumber (LastFile: PFileStructure);
Var StringDifference: Integer;
    LineNumberStr: String [10];
    N: Byte;
begin
  If LastStringLength <> 0 then Write (#13);
  Str (LastFile^.CurrentLineNumber, LineNumberStr);
  Write (PChar (@LastFile^.Name.Chr), '(', LastFile^.CurrentLineNumber, ')');
  StringDifference := LastStringLength - (Length (LineNumberStr) + LastFile^.Name.Len + 2);
  LastStringLength := Length (LineNumberStr) + LastFile^.Name.Len + 2;
  If StringDifference <= 0 then Exit;
  For N := 1 to StringDifference do Write (' ');
  For N := 1 to StringDifference do Write (#8);
end;
```

### Code Block 9

```pascal
Procedure WriteCompilationStatus;
Const TimeConstant = 1 / 1.82064673;
Var   Time: Longint;
begin
  Write (LineCounter, ' lines, ');
  Time := Round (TimeConstant * (StopCounter - StartCounter));
  If Time <> 0 then Write (Time div 10, '.', Time mod 10, ' seconds, ');
  Writeln (CodeBytes, ' bytes code, ', DataBytes, ' bytes data.');
  Asm
    MOV AH, $0B
    INT $21
    CLD
  end
end;
```

### Code Block 10

```pascal
Procedure WritelnModuleNameWithCurrentLineNumber (LastFile: Pointer);
begin
  If QuietCompile then Exit;
  WriteModuleNameWithCurrentLineNumber (LastFile);
  Writeln;
  Asm
    MOV AH, $0B
    INT $21
    CLD
  end
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
- https://turbopascal.org/extracting-tokens
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-statements
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

