# Processing System Procedures - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-system-procedures

---

## Table of Contents

- Processing System Procedures

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing System Procedures
All Turbo Pascal system procedures are processed by this function. Each system procedure has a processing function with optional parameter which is used when two or more procedures share the same processing function. Processing function returns the offset to
intermediate code subroutine
.
Type TSystemProcedure = Record
                          Proc: Function: Word;
                          ProcParameter: Word;
                        end;

Const stProcedures: Array [$00..$2B] of TSystemProcedure = (
        (Proc: Proc_New;                  ProcParameter: 0),
        (Proc: Proc_Write_Writeln;        ProcParameter: SysProc_WriteEnd),
        (Proc: Proc_Write_Writeln;        ProcParameter: SysProc_WriteLine),
        (Proc: Proc_Read_Readln;          ProcParameter: SysProc_ReadEnd),
        (Proc: Proc_Read_Readln;          ProcParameter: SysProc_ReadLine),
        (Proc: Proc_Dispose;              ProcParameter: 0),
        (Proc: Proc_Mark_Release;         ProcParameter: SysProc_MarkPtr),
        (Proc: Proc_Mark_Release;         ProcParameter: SysProc_ReleasePtr),
        (Proc: Proc_Insert;               ProcParameter: 0),
        (Proc: Proc_Delete;               ProcParameter: 0),
        (Proc: Proc_Assign;               ProcParameter: 0),
        (Proc: Proc_Reset_Rewrite;        ProcParameter: SysProc_ResetFile),
        (Proc: Proc_Reset_Rewrite;        ProcParameter: SysProc_RewriteFile),
        (Proc: Proc_Close;                ProcParameter: 0),
        (Proc: Proc_Seek;                 ProcParameter: 0),
        (Proc: Proc_Erase;                ProcParameter: 0),
        (Proc: Proc_Rename;               ProcParameter: 0),
        (Proc: Proc_Halt_RunError;        ProcParameter: SysProc_HaltTurbo),
        (Proc: Proc_BREAK_CONTINUE_Exit;  ProcParameter: Ofs (LastJumpToProgramBlockExit)),
        (Proc: Proc_GetMem;               ProcParameter: 0),
        (Proc: Proc_FreeMem;              ProcParameter: 0),
        (Proc: Proc_Truncate;             ProcParameter: 0),
        (Proc: Proc_FillChar;             ProcParameter: 0),
        (Proc: Proc_Move;                 ProcParameter: 0),
        (Proc: Proc_Randomize;            ProcParameter: 0),
        (Proc: Proc_Str;                  ProcParameter: 0),
        (Proc: Proc_Val;                  ProcParameter: 0),
        (Proc: Proc_GetDir;               ProcParameter: 0),
        (Proc: Proc_ChDir_MkDir_RmDir;    ProcParameter: SysProc_DirSet),
        (Proc: Proc_ChDir_MkDir_RmDir;    ProcParameter: SysProc_DirCreate),
        (Proc: Proc_ChDir_MkDir_RmDir;    ProcParameter: SysProc_DirRemove),
        (Proc: Proc_Append_Flush;         ProcParameter: SysProc_AppendText),
        (Proc: Proc_Append_Flush;         ProcParameter: SysProc_FlushText),
        (Proc: Proc_Inc_Dec;              ProcParameter: $00),
        (Proc: Proc_Inc_Dec;              ProcParameter: $28),
        (Proc: Proc_BlockRead_BlockWrite; ProcParameter: SysProc_ReadBlock),
        (Proc: Proc_BlockRead_BlockWrite; ProcParameter: SysProc_WriteBlock),
        (Proc: Proc_SetTextBuf;           ProcParameter: 0),
        (Proc: Proc_Halt_RunError;        ProcParameter: SysProc_HaltError),
        (Proc: Proc_Fail;                 ProcParameter: 0),
        (Proc: Proc_BREAK_CONTINUE_Exit;  ProcParameter: Ofs (LastJumpOutOfBlock)),
        (Proc: Proc_BREAK_CONTINUE_Exit;  ProcParameter: Ofs (LastJumpToNextBlockIteration)),
        (Proc: Proc_Include_Exclude;      ProcParameter: Include),
        (Proc: Proc_Include_Exclude;      ProcParameter: Exclude));
This function processes system procedures. The parameter is saved to allow recursion.
Function SystemProcedure (ProcNumber: Word): Word;
Var SavedParameter: Word;
begin
  SavedParameter := ProcParameter;
  ProcParameter := stProcedures [ProcNumber].ProcParameter;
  SystemProcedure := stProcedures [ProcNumber].Proc;

  If ProcParameter = $FFFF then Halt (1);

  ProcParameter := SavedParameter;
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
Type TSystemProcedure = Record
                          Proc: Function: Word;
                          ProcParameter: Word;
                        end;

Const stProcedures: Array [$00..$2B] of TSystemProcedure = (
        (Proc: Proc_New;                  ProcParameter: 0),
        (Proc: Proc_Write_Writeln;        ProcParameter: SysProc_WriteEnd),
        (Proc: Proc_Write_Writeln;        ProcParameter: SysProc_WriteLine),
        (Proc: Proc_Read_Readln;          ProcParameter: SysProc_ReadEnd),
        (Proc: Proc_Read_Readln;          ProcParameter: SysProc_ReadLine),
        (Proc: Proc_Dispose;              ProcParameter: 0),
        (Proc: Proc_Mark_Release;         ProcParameter: SysProc_MarkPtr),
        (Proc: Proc_Mark_Release;         ProcParameter: SysProc_ReleasePtr),
        (Proc: Proc_Insert;               ProcParameter: 0),
        (Proc: Proc_Delete;               ProcParameter: 0),
        (Proc: Proc_Assign;               ProcParameter: 0),
        (Proc: Proc_Reset_Rewrite;        ProcParameter: SysProc_ResetFile),
        (Proc: Proc_Reset_Rewrite;        ProcParameter: SysProc_RewriteFile),
        (Proc: Proc_Close;                ProcParameter: 0),
        (Proc: Proc_Seek;                 ProcParameter: 0),
        (Proc: Proc_Erase;                ProcParameter: 0),
        (Proc: Proc_Rename;               ProcParameter: 0),
        (Proc: Proc_Halt_RunError;        ProcParameter: SysProc_HaltTurbo),
        (Proc: Proc_BREAK_CONTINUE_Exit;  ProcParameter: Ofs (LastJumpToProgramBlockExit)),
        (Proc: Proc_GetMem;               ProcParameter: 0),
        (Proc: Proc_FreeMem;              ProcParameter: 0),
        (Proc: Proc_Truncate;             ProcParameter: 0),
        (Proc: Proc_FillChar;             ProcParameter: 0),
        (Proc: Proc_Move;                 ProcParameter: 0),
        (Proc: Proc_Randomize;            ProcParameter: 0),
        (Proc: Proc_Str;                  ProcParameter: 0),
        (Proc: Proc_Val;                  ProcParameter: 0),
        (Proc: Proc_GetDir;               ProcParameter: 0),
        (Proc: Proc_ChDir_MkDir_RmDir;    ProcParameter: SysProc_DirSet),
        (Proc: Proc_ChDir_MkDir_RmDir;    ProcParameter: SysProc_DirCreate),
        (Proc: Proc_ChDir_MkDir_RmDir;    ProcParameter: SysProc_DirRemove),
        (Proc: Proc_Append_Flush;         ProcParameter: SysProc_AppendText),
        (Proc: Proc_Append_Flush;         ProcParameter: SysProc_FlushText),
        (Proc: Proc_Inc_Dec;              ProcParameter: $00),
        (Proc: Proc_Inc_Dec;              ProcParameter: $28),
        (Proc: Proc_BlockRead_BlockWrite; ProcParameter: SysProc_ReadBlock),
        (Proc: Proc_BlockRead_BlockWrite; ProcParameter: SysProc_WriteBlock),
        (Proc: Proc_SetTextBuf;           ProcParameter: 0),
        (Proc: Proc_Halt_RunError;        ProcParameter: SysProc_HaltError),
        (Proc: Proc_Fail;                 ProcParameter: 0),
        (Proc: Proc_BREAK_CONTINUE_Exit;  ProcParameter: Ofs (LastJumpOutOfBlock)),
        (Proc: Proc_BREAK_CONTINUE_Exit;  ProcParameter: Ofs (LastJumpToNextBlockIteration)),
        (Proc: Proc_Include_Exclude;      ProcParameter: Include),
        (Proc: Proc_Include_Exclude;      ProcParameter: Exclude));
```

### Code Block 2

```pascal
Function SystemProcedure (ProcNumber: Word): Word;
Var SavedParameter: Word;
begin
  SavedParameter := ProcParameter;
  ProcParameter := stProcedures [ProcNumber].ProcParameter;
  SystemProcedure := stProcedures [ProcNumber].Proc;

  If ProcParameter = $FFFF then Halt (1);

  ProcParameter := SavedParameter;
end;
```

### Code Block 3

```pascal
Type TSystemProcedure = Record
                          Proc: Function: Word;
                          ProcParameter: Word;
                        end;

Const stProcedures: Array [$00..$2B] of TSystemProcedure = (
        (Proc: Proc_New;                  ProcParameter: 0),
        (Proc: Proc_Write_Writeln;        ProcParameter: SysProc_WriteEnd),
        (Proc: Proc_Write_Writeln;        ProcParameter: SysProc_WriteLine),
        (Proc: Proc_Read_Readln;          ProcParameter: SysProc_ReadEnd),
        (Proc: Proc_Read_Readln;          ProcParameter: SysProc_ReadLine),
        (Proc: Proc_Dispose;              ProcParameter: 0),
        (Proc: Proc_Mark_Release;         ProcParameter: SysProc_MarkPtr),
        (Proc: Proc_Mark_Release;         ProcParameter: SysProc_ReleasePtr),
        (Proc: Proc_Insert;               ProcParameter: 0),
        (Proc: Proc_Delete;               ProcParameter: 0),
        (Proc: Proc_Assign;               ProcParameter: 0),
        (Proc: Proc_Reset_Rewrite;        ProcParameter: SysProc_ResetFile),
        (Proc: Proc_Reset_Rewrite;        ProcParameter: SysProc_RewriteFile),
        (Proc: Proc_Close;                ProcParameter: 0),
        (Proc: Proc_Seek;                 ProcParameter: 0),
        (Proc: Proc_Erase;                ProcParameter: 0),
        (Proc: Proc_Rename;               ProcParameter: 0),
        (Proc: Proc_Halt_RunError;        ProcParameter: SysProc_HaltTurbo),
        (Proc: Proc_BREAK_CONTINUE_Exit;  ProcParameter: Ofs (LastJumpToProgramBlockExit)),
        (Proc: Proc_GetMem;               ProcParameter: 0),
        (Proc: Proc_FreeMem;              ProcParameter: 0),
        (Proc: Proc_Truncate;             ProcParameter: 0),
        (Proc: Proc_FillChar;             ProcParameter: 0),
        (Proc: Proc_Move;                 ProcParameter: 0),
        (Proc: Proc_Randomize;            ProcParameter: 0),
        (Proc: Proc_Str;                  ProcParameter: 0),
        (Proc: Proc_Val;                  ProcParameter: 0),
        (Proc: Proc_GetDir;               ProcParameter: 0),
        (Proc: Proc_ChDir_MkDir_RmDir;    ProcParameter: SysProc_DirSet),
        (Proc: Proc_ChDir_MkDir_RmDir;    ProcParameter: SysProc_DirCreate),
        (Proc: Proc_ChDir_MkDir_RmDir;    ProcParameter: SysProc_DirRemove),
        (Proc: Proc_Append_Flush;         ProcParameter: SysProc_AppendText),
        (Proc: Proc_Append_Flush;         ProcParameter: SysProc_FlushText),
        (Proc: Proc_Inc_Dec;              ProcParameter: $00),
        (Proc: Proc_Inc_Dec;              ProcParameter: $28),
        (Proc: Proc_BlockRead_BlockWrite; ProcParameter: SysProc_ReadBlock),
        (Proc: Proc_BlockRead_BlockWrite; ProcParameter: SysProc_WriteBlock),
        (Proc: Proc_SetTextBuf;           ProcParameter: 0),
        (Proc: Proc_Halt_RunError;        ProcParameter: SysProc_HaltError),
        (Proc: Proc_Fail;                 ProcParameter: 0),
        (Proc: Proc_BREAK_CONTINUE_Exit;  ProcParameter: Ofs (LastJumpOutOfBlock)),
        (Proc: Proc_BREAK_CONTINUE_Exit;  ProcParameter: Ofs (LastJumpToNextBlockIteration)),
        (Proc: Proc_Include_Exclude;      ProcParameter: Include),
        (Proc: Proc_Include_Exclude;      ProcParameter: Exclude));
```

### Code Block 4

```pascal
Function SystemProcedure (ProcNumber: Word): Word;
Var SavedParameter: Word;
begin
  SavedParameter := ProcParameter;
  ProcParameter := stProcedures [ProcNumber].ProcParameter;
  SystemProcedure := stProcedures [ProcNumber].Proc;

  If ProcParameter = $FFFF then Halt (1);

  ProcParameter := SavedParameter;
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
- https://turbopascal.org/intermediate-code
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-system-procedures
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

