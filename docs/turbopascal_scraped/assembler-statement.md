# Assembler Statement - Turbo Pascal Compiler

**URL:** https://turbopascal.org/assembler-statement

---

## Table of Contents

- Assembler Statement

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Assembler Statement
This procedure processes
Asm
–
end
blocks of assembler instructions. It creates a temporary
symbol table
for assembler identifiers, indirectly checks for end token and processes assembler instructions.
Procedure TStatement.Process_ASM_END_Block;
Var Saved_MainSymbolTableUsedSize: Word;
    CharPtr: PChar;
    AsmFlags: Word;
    Var List: PIdentifierList;
begin
  If Token <> Token_ASM then Error (ASM_Expected);
  TemporaryAssemblerStatementIdentifierTable := SymbolTable [stMain].UsedSize;
  CreateSymbolTable (4);
  Saved_MainSymbolTableUsedSize := SymbolTable [stMain].UsedSize;
  Repeat
    CheckStack;
    CharPtr := NextNonCommentCharacter;
    ErrorSourcePosition := CharPtr;
    If CharPtr [0] = ';' then
      begin
        CurrentSourceFile^.CurrentPosition := Ofs (CharPtr^) + 1;
        Continue;
      end;
    If (UpCase (CharPtr [0]) = 'E') and (UpCase (CharPtr [1]) = 'N') and (UpCase (CharPtr [2]) = 'D') then
      Case UpCase (CharPtr [3]) of
        '0'..'9',
        'A'..'Z', '_':
        else Break;
      end;
    MarkSourceLineNumber (GetCurrentProgramBlockSourceLineNumber);
    AsmFlags := $0000;
    If Instructions80286 in ModuleCompilerSwitches then AsmFlags := $0001;
    If (ProcedureIdentifierDataOffset <> 0) and
      (pfFar in  PProcedureIdentifierData (Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset))^.Flags)
        then AsmFlags := AsmFlags or $0100;
    ProcessAssemblerInstruction (CharPtr, 128, AsmFlags, ProcedureIdentifierDataOffset);
    CurrentSourceFile^.CurrentPosition := Ofs (CharPtr^);
  until False;
  CurrentSourceFile^.CurrentPosition := Ofs (CharPtr^) + 3;
  GetNextToken;
  If Saved_MainSymbolTableUsedSize = SymbolTable [stMain].UsedSize then SymbolTable [stMain].UsedSize :=
    TemporaryAssemblerStatementIdentifierTable;
  ES_DI_PointerDestroyed;
  StatementCode := EndSubroutine;
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
Procedure TStatement.Process_ASM_END_Block;
Var Saved_MainSymbolTableUsedSize: Word;
    CharPtr: PChar;
    AsmFlags: Word;
    Var List: PIdentifierList;
begin
  If Token <> Token_ASM then Error (ASM_Expected);
  TemporaryAssemblerStatementIdentifierTable := SymbolTable [stMain].UsedSize;
  CreateSymbolTable (4);
  Saved_MainSymbolTableUsedSize := SymbolTable [stMain].UsedSize;
  Repeat
    CheckStack;
    CharPtr := NextNonCommentCharacter;
    ErrorSourcePosition := CharPtr;
    If CharPtr [0] = ';' then
      begin
        CurrentSourceFile^.CurrentPosition := Ofs (CharPtr^) + 1;
        Continue;
      end;
    If (UpCase (CharPtr [0]) = 'E') and (UpCase (CharPtr [1]) = 'N') and (UpCase (CharPtr [2]) = 'D') then
      Case UpCase (CharPtr [3]) of
        '0'..'9',
        'A'..'Z', '_':
        else Break;
      end;
    MarkSourceLineNumber (GetCurrentProgramBlockSourceLineNumber);
    AsmFlags := $0000;
    If Instructions80286 in ModuleCompilerSwitches then AsmFlags := $0001;
    If (ProcedureIdentifierDataOffset <> 0) and
      (pfFar in  PProcedureIdentifierData (Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset))^.Flags)
        then AsmFlags := AsmFlags or $0100;
    ProcessAssemblerInstruction (CharPtr, 128, AsmFlags, ProcedureIdentifierDataOffset);
    CurrentSourceFile^.CurrentPosition := Ofs (CharPtr^);
  until False;
  CurrentSourceFile^.CurrentPosition := Ofs (CharPtr^) + 3;
  GetNextToken;
  If Saved_MainSymbolTableUsedSize = SymbolTable [stMain].UsedSize then SymbolTable [stMain].UsedSize :=
    TemporaryAssemblerStatementIdentifierTable;
  ES_DI_PointerDestroyed;
  StatementCode := EndSubroutine;
end;
```

### Code Block 2

```pascal
Procedure TStatement.Process_ASM_END_Block;
Var Saved_MainSymbolTableUsedSize: Word;
    CharPtr: PChar;
    AsmFlags: Word;
    Var List: PIdentifierList;
begin
  If Token <> Token_ASM then Error (ASM_Expected);
  TemporaryAssemblerStatementIdentifierTable := SymbolTable [stMain].UsedSize;
  CreateSymbolTable (4);
  Saved_MainSymbolTableUsedSize := SymbolTable [stMain].UsedSize;
  Repeat
    CheckStack;
    CharPtr := NextNonCommentCharacter;
    ErrorSourcePosition := CharPtr;
    If CharPtr [0] = ';' then
      begin
        CurrentSourceFile^.CurrentPosition := Ofs (CharPtr^) + 1;
        Continue;
      end;
    If (UpCase (CharPtr [0]) = 'E') and (UpCase (CharPtr [1]) = 'N') and (UpCase (CharPtr [2]) = 'D') then
      Case UpCase (CharPtr [3]) of
        '0'..'9',
        'A'..'Z', '_':
        else Break;
      end;
    MarkSourceLineNumber (GetCurrentProgramBlockSourceLineNumber);
    AsmFlags := $0000;
    If Instructions80286 in ModuleCompilerSwitches then AsmFlags := $0001;
    If (ProcedureIdentifierDataOffset <> 0) and
      (pfFar in  PProcedureIdentifierData (Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset))^.Flags)
        then AsmFlags := AsmFlags or $0100;
    ProcessAssemblerInstruction (CharPtr, 128, AsmFlags, ProcedureIdentifierDataOffset);
    CurrentSourceFile^.CurrentPosition := Ofs (CharPtr^);
  until False;
  CurrentSourceFile^.CurrentPosition := Ofs (CharPtr^) + 3;
  GetNextToken;
  If Saved_MainSymbolTableUsedSize = SymbolTable [stMain].UsedSize then SymbolTable [stMain].UsedSize :=
    TemporaryAssemblerStatementIdentifierTable;
  ES_DI_PointerDestroyed;
  StatementCode := EndSubroutine;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/assembler-statement
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/symbol-table-management
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

