# Command Line Syntax Display - Turbo Pascal Compiler

**URL:** https://turbopascal.org/command-line-syntax-display

---

## Table of Contents

- Command Line Syntax Display

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Command Line Syntax Display
When no source file is specified this procedure simply displays Turbo Pascal command line syntax with default switch states.
Procedure WriteSyntax;

  Function Switch (Sw: TCompilerSwitches): Char;
  begin
    If Sw in GlobalModuleData^.CompilerSwitches then Switch := '+'
      else Switch := '-';
  end;

begin
  Writeln ('Syntax: TPC [options] filename [options]');
  Writeln ('  -B = Build all units                -L = Link buffer on disk');
  Writeln ('  -D<syms> = Define conditionals      -M = Make modified units');
  Writeln ('  -E<path> = EXE/TPU directories      -O<path> = Object directories');
  Writeln ('  -F<seg>:<ofs> = Find error          -Q = Quiet compile');
  Writeln ('  -GD = Detailed map file             -T<path> = TPL/CFG directory');
  Writeln ('  -GP = Map file with publics         -U<path> = Unit directories');
  Writeln ('  -GS = Map file with segments        -V = Debug information in EXE');
  Writeln ('  -I<path> = Include directories      -$<dir> = Compiler directive');
  Writeln ('Compiler switches: -$<letter><state>  (defaults are shown below)');
  Writeln ('  A', Switch (WordAlignment),' Word alignment       I',
                  Switch (IOErrorChecking),' I/O error checking   R',
                  Switch (RangeChecking),' Range checking');
  Writeln ('  B', Switch (FullBooleanEval),' Full boolean eval    L',
                  Switch (LocalDebugSymbols),' Local debug symbols  S',
                  Switch (StackChecking),' Stack checking');
  Writeln ('  D', Switch (DebugInformation),' Debug information    N',
                  Switch (Instructions80x87),' 80x87 instructions   T',
                  Switch (TypedPointers),' Typed pointers');
  Writeln ('  E', Switch (Emulation80x87),' 80x87 emulation      O',
                  Switch (OverlaysAllowed),' Overlays allowed     V',
                  Switch (StrictVarStrings),' Strict var-strings');
  Writeln ('  F', Switch (ForceFarCalls),' Force FAR calls      P',
                  Switch (OpenStringParams),' Open string params   X',
                  Switch (ExtendedSyntax),' Extended syntax');
  Writeln ('  G', Switch (Instructions80286),' 80286 instructions   Q',
                  Switch (OverflowChecking),' Overflow checking');
  Writeln ('Memory sizes: -$M<stack>,<heapmin>,<heapmax>  (default: ',
            GlobalModuleData^.Stack, ',',
            Longint (GlobalModuleData^.HeapMin) * 16, ',',
            Longint (GlobalModuleData^.HeapMax) * 16, ')');
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
Procedure WriteSyntax;

  Function Switch (Sw: TCompilerSwitches): Char;
  begin
    If Sw in GlobalModuleData^.CompilerSwitches then Switch := '+'
      else Switch := '-';
  end;

begin
  Writeln ('Syntax: TPC [options] filename [options]');
  Writeln ('  -B = Build all units                -L = Link buffer on disk');
  Writeln ('  -D<syms> = Define conditionals      -M = Make modified units');
  Writeln ('  -E<path> = EXE/TPU directories      -O<path> = Object directories');
  Writeln ('  -F<seg>:<ofs> = Find error          -Q = Quiet compile');
  Writeln ('  -GD = Detailed map file             -T<path> = TPL/CFG directory');
  Writeln ('  -GP = Map file with publics         -U<path> = Unit directories');
  Writeln ('  -GS = Map file with segments        -V = Debug information in EXE');
  Writeln ('  -I<path> = Include directories      -$<dir> = Compiler directive');
  Writeln ('Compiler switches: -$<letter><state>  (defaults are shown below)');
  Writeln ('  A', Switch (WordAlignment),' Word alignment       I',
                  Switch (IOErrorChecking),' I/O error checking   R',
                  Switch (RangeChecking),' Range checking');
  Writeln ('  B', Switch (FullBooleanEval),' Full boolean eval    L',
                  Switch (LocalDebugSymbols),' Local debug symbols  S',
                  Switch (StackChecking),' Stack checking');
  Writeln ('  D', Switch (DebugInformation),' Debug information    N',
                  Switch (Instructions80x87),' 80x87 instructions   T',
                  Switch (TypedPointers),' Typed pointers');
  Writeln ('  E', Switch (Emulation80x87),' 80x87 emulation      O',
                  Switch (OverlaysAllowed),' Overlays allowed     V',
                  Switch (StrictVarStrings),' Strict var-strings');
  Writeln ('  F', Switch (ForceFarCalls),' Force FAR calls      P',
                  Switch (OpenStringParams),' Open string params   X',
                  Switch (ExtendedSyntax),' Extended syntax');
  Writeln ('  G', Switch (Instructions80286),' 80286 instructions   Q',
                  Switch (OverflowChecking),' Overflow checking');
  Writeln ('Memory sizes: -$M<stack>,<heapmin>,<heapmax>  (default: ',
            GlobalModuleData^.Stack, ',',
            Longint (GlobalModuleData^.HeapMin) * 16, ',',
            Longint (GlobalModuleData^.HeapMax) * 16, ')');
end;
```

### Code Block 2

```pascal
Procedure WriteSyntax;

  Function Switch (Sw: TCompilerSwitches): Char;
  begin
    If Sw in GlobalModuleData^.CompilerSwitches then Switch := '+'
      else Switch := '-';
  end;

begin
  Writeln ('Syntax: TPC [options] filename [options]');
  Writeln ('  -B = Build all units                -L = Link buffer on disk');
  Writeln ('  -D<syms> = Define conditionals      -M = Make modified units');
  Writeln ('  -E<path> = EXE/TPU directories      -O<path> = Object directories');
  Writeln ('  -F<seg>:<ofs> = Find error          -Q = Quiet compile');
  Writeln ('  -GD = Detailed map file             -T<path> = TPL/CFG directory');
  Writeln ('  -GP = Map file with publics         -U<path> = Unit directories');
  Writeln ('  -GS = Map file with segments        -V = Debug information in EXE');
  Writeln ('  -I<path> = Include directories      -$<dir> = Compiler directive');
  Writeln ('Compiler switches: -$<letter><state>  (defaults are shown below)');
  Writeln ('  A', Switch (WordAlignment),' Word alignment       I',
                  Switch (IOErrorChecking),' I/O error checking   R',
                  Switch (RangeChecking),' Range checking');
  Writeln ('  B', Switch (FullBooleanEval),' Full boolean eval    L',
                  Switch (LocalDebugSymbols),' Local debug symbols  S',
                  Switch (StackChecking),' Stack checking');
  Writeln ('  D', Switch (DebugInformation),' Debug information    N',
                  Switch (Instructions80x87),' 80x87 instructions   T',
                  Switch (TypedPointers),' Typed pointers');
  Writeln ('  E', Switch (Emulation80x87),' 80x87 emulation      O',
                  Switch (OverlaysAllowed),' Overlays allowed     V',
                  Switch (StrictVarStrings),' Strict var-strings');
  Writeln ('  F', Switch (ForceFarCalls),' Force FAR calls      P',
                  Switch (OpenStringParams),' Open string params   X',
                  Switch (ExtendedSyntax),' Extended syntax');
  Writeln ('  G', Switch (Instructions80286),' 80286 instructions   Q',
                  Switch (OverflowChecking),' Overflow checking');
  Writeln ('Memory sizes: -$M<stack>,<heapmin>,<heapmax>  (default: ',
            GlobalModuleData^.Stack, ',',
            Longint (GlobalModuleData^.HeapMin) * 16, ',',
            Longint (GlobalModuleData^.HeapMax) * 16, ')');
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/command-line-syntax-display
- https://turbopascal.org/compiler
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

