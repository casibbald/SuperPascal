# Main Program - Turbo Pascal Compiler

**URL:** https://turbopascal.org/main-program

---

## Table of Contents

- Main Program

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Main Program
This is the main program of the compiler. It initializes some data, reads configuration file,
processes command line parameters
,
loads library
(reports error if it is not found), initializes module data, checks for the main file, resets compilation timer,
compiles the source file
and
writes compilation status
.
If no source file was specified
ProcessCompilerParameters
will display
command line syntax
and halt.
Error handling is done with few routines which save and restore stack position so any compiler error always returns control flow to the main program.
Compilation timer actually measures the time for the
Compile
procedure. This time is
reported
after successful compilation and includes also I/O operations so it is usually shorter on the second run when the source files are already cached.
If command line switch
/F
was used then instead of compilation status the
source file position
is displayed. The
error reported
if the source position is found is actually not an error, only a message ‘
Target address found
‘ is displayed.
GreetingString: PChar = 'Turbo Pascal  Version 7.0  Copyright (c) 1983,92 Borland International';
Program TPC16;

Uses
  Strings,
  Parser,
  Scanner,
  IOUtils,
  ResWords,
  CommVars;

begin
  Writeln (GreetingString);
  CmdPtr := Ptr (PrefixSeg, $80);
  CmdPtr [Byte (CmdPtr [0]) + 1] := #0;
  CmdPtr [0] := ' ';
  CfgPtr := FileBuffer;
  Get_TPL_CFG_Directory;
  Read_TPC_CFG;
  CompilerModeOptions := [cmoCreateExeFile, cmoCompileToDisk];
  LinkerOptions := [];
  CommandLineConditionalDefines := InitialConditionalDefines;
  StartOfSourceFileStructures := @FileStructure;
  ProcessCompilerParameters;
  StrCopy (TempStr, GlobalModuleData^.LibraryName);
  CurrentFileName := TempStr;
  FindFilePath (TempStr, Dir_TPL_CFG or Ext_Original);
  LoadLibrary;
  Case LastError of
    FileNotFound: Writeln ('Warning: ', TempStr, ' not found.');
    NoError:
    else WriteCompilationErrorAndHalt;
  end;
  With GlobalModuleData^ do
    begin
      InitialEnvironmentFlags := EnvironmentFlags;
      InitialCompilerSwitches := CompilerSwitches;
      ModuleStack   := Stack;
      ModuleHeapMin := HeapMin;
      ModuleHeapMax := HeapMax;
    end;
  CurrentFileName := MainFileName;
  FindFilePath (CurrentFileName, Dir_None or Ext_PAS);
  StartCounter := InterruptCounter;
  CompilationInProgress := True;
  Compile;
  CompilationInProgress := False;
  StopCounter := InterruptCounter;
  If LastError &lt;&gt; NoError then WriteCompilationErrorAndHalt;
  If cmoFindError in CompilerModeOptions then
    begin
      FindAndWriteSourceLine;
      WriteCompilationErrorAndHalt;
    end;
  WriteCompilationStatus;
end.
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
GreetingString: PChar = 'Turbo Pascal  Version 7.0  Copyright (c) 1983,92 Borland International';
```

### Code Block 2

```pascal
Program TPC16;

Uses
  Strings,
  Parser,
  Scanner,
  IOUtils,
  ResWords,
  CommVars;

begin
  Writeln (GreetingString);
  CmdPtr := Ptr (PrefixSeg, $80);
  CmdPtr [Byte (CmdPtr [0]) + 1] := #0;
  CmdPtr [0] := ' ';
  CfgPtr := FileBuffer;
  Get_TPL_CFG_Directory;
  Read_TPC_CFG;
  CompilerModeOptions := [cmoCreateExeFile, cmoCompileToDisk];
  LinkerOptions := [];
  CommandLineConditionalDefines := InitialConditionalDefines;
  StartOfSourceFileStructures := @FileStructure;
  ProcessCompilerParameters;
  StrCopy (TempStr, GlobalModuleData^.LibraryName);
  CurrentFileName := TempStr;
  FindFilePath (TempStr, Dir_TPL_CFG or Ext_Original);
  LoadLibrary;
  Case LastError of
    FileNotFound: Writeln ('Warning: ', TempStr, ' not found.');
    NoError:
    else WriteCompilationErrorAndHalt;
  end;
  With GlobalModuleData^ do
    begin
      InitialEnvironmentFlags := EnvironmentFlags;
      InitialCompilerSwitches := CompilerSwitches;
      ModuleStack   := Stack;
      ModuleHeapMin := HeapMin;
      ModuleHeapMax := HeapMax;
    end;
  CurrentFileName := MainFileName;
  FindFilePath (CurrentFileName, Dir_None or Ext_PAS);
  StartCounter := InterruptCounter;
  CompilationInProgress := True;
  Compile;
  CompilationInProgress := False;
  StopCounter := InterruptCounter;
  If LastError &lt;&gt; NoError then WriteCompilationErrorAndHalt;
  If cmoFindError in CompilerModeOptions then
    begin
      FindAndWriteSourceLine;
      WriteCompilationErrorAndHalt;
    end;
  WriteCompilationStatus;
end.
```

### Code Block 3

```pascal
GreetingString: PChar = 'Turbo Pascal  Version 7.0  Copyright (c) 1983,92 Borland International';
```

### Code Block 4

```pascal
Program TPC16;

Uses
  Strings,
  Parser,
  Scanner,
  IOUtils,
  ResWords,
  CommVars;

begin
  Writeln (GreetingString);
  CmdPtr := Ptr (PrefixSeg, $80);
  CmdPtr [Byte (CmdPtr [0]) + 1] := #0;
  CmdPtr [0] := ' ';
  CfgPtr := FileBuffer;
  Get_TPL_CFG_Directory;
  Read_TPC_CFG;
  CompilerModeOptions := [cmoCreateExeFile, cmoCompileToDisk];
  LinkerOptions := [];
  CommandLineConditionalDefines := InitialConditionalDefines;
  StartOfSourceFileStructures := @FileStructure;
  ProcessCompilerParameters;
  StrCopy (TempStr, GlobalModuleData^.LibraryName);
  CurrentFileName := TempStr;
  FindFilePath (TempStr, Dir_TPL_CFG or Ext_Original);
  LoadLibrary;
  Case LastError of
    FileNotFound: Writeln ('Warning: ', TempStr, ' not found.');
    NoError:
    else WriteCompilationErrorAndHalt;
  end;
  With GlobalModuleData^ do
    begin
      InitialEnvironmentFlags := EnvironmentFlags;
      InitialCompilerSwitches := CompilerSwitches;
      ModuleStack   := Stack;
      ModuleHeapMin := HeapMin;
      ModuleHeapMax := HeapMax;
    end;
  CurrentFileName := MainFileName;
  FindFilePath (CurrentFileName, Dir_None or Ext_PAS);
  StartCounter := InterruptCounter;
  CompilationInProgress := True;
  Compile;
  CompilationInProgress := False;
  StopCounter := InterruptCounter;
  If LastError &lt;&gt; NoError then WriteCompilationErrorAndHalt;
  If cmoFindError in CompilerModeOptions then
    begin
      FindAndWriteSourceLine;
      WriteCompilationErrorAndHalt;
    end;
  WriteCompilationStatus;
end.
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/command-line-syntax-display
- https://turbopascal.org/compilation-process
- https://turbopascal.org/compilation-progress-and-status-display
- https://turbopascal.org/compiler
- https://turbopascal.org/error-reporting
- https://turbopascal.org/expressions
- https://turbopascal.org/finding-source-position-from-code-address
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/loading-library
- https://turbopascal.org/main-program
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-command-line-parameters
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

