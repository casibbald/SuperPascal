# Module Entry and Exit Code - Turbo Pascal Compiler

**URL:** https://turbopascal.org/module-entry-and-exit-code

---

## Table of Contents

- Module Entry and Exit Code

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Module Entry and Exit Code
Each module (program or unit) needs some initialization and finalization: entry and exit code.
For executable programs this function generates the following:
a call to initialize program environment
if at least one module was compiled with
N+
it generates a call to initialize FPU emulator: if program was compiled with
E+
it generates a call to initialize software emulator if no FPU if found, otherwise it generates a call to initialize dummy emulator to use FPU instructions.
it generates a call to initialization code for each used unit
Finally for programs and units this procedure creates
stack frame
.
Function GenerateModuleEntryCode: Word;
Var UnitHeader: PUnitHeader;
    UnitHeaderRec: PtrRec absolute UnitHeader;
    NumberOfUnitsWithInitialization, N, Segment: Word;
begin
  If SourceType < stUnit then
    begin
      GenerateInstruction_CALL_FAR (SysProc_InitTurbo);
      UnitHeader := Ptr (LastLoadedUsedUnit, 0);
      Repeat
        If ufInstructions80x87 in UnitHeader^.Flags then
          begin
            If Emulation80x87 in ModuleCompilerSwitches then GenerateInstruction_CALL_FAR (SysProc_InitEM86)
              else GenerateInstruction_CALL_FAR (SysProc_InitEM87);
            Break;
          end;
        UnitHeaderRec.Seg := UnitHeader^.PreviousUnitSegment;
      until UnitHeaderRec.Seg = 0;
      UnitHeader := Ptr (LastLoadedUsedUnit, 0);
      NumberOfUnitsWithInitialization := 0;
      While UnitHeader^.PreviousUnitSegment <> 0 do
        begin
          UnitHeaderRec.Seg := UnitHeader^.PreviousUnitSegment;
          If PProceduresBlockRecord (Ptr (UnitHeaderRec.Seg,
                                          UnitHeader^.BlockOffset [stProcedures]))^.ProgramCodeBlockRecordOffset <> $FFFF then
            begin
              Inc (NumberOfUnitsWithInitialization);
              Asm
                LES     DI, UnitHeader
                PUSH    ES
              end;
            end;
        end;
      For N := 1 to NumberOfUnitsWithInitialization do
        begin
          GenerateInstruction_Byte (CALL_FAR);
          Asm
            POP    Segment
          end;
          UnitHeaderRec.Seg := Segment;
          GenerateReference (Segment, 0, 0, [rfSegment, rfOffset]);
        end;
    end;
  CreateStackFrame;
  GenerateModuleEntryCode := EndSubroutine;
end;
This function generates code to destroy stack frame and then if unit is compiled it generates return instruction from initialization code (
RETF
) otherwise it generates code to halt the program.
Function GenerateModuleExitCode: Word;
begin
  DestroyStackFrame;
  If SourceType >= stUnit then GenerateInstruction_Byte (RETF) else
    begin
      GenerateInstruction_Word (XOR_AX_AX);
      GenerateInstruction_CALL_FAR (SysProc_HaltTurbo);
    end;
  GenerateModuleExitCode := EndSubroutine;
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
Function GenerateModuleEntryCode: Word;
Var UnitHeader: PUnitHeader;
    UnitHeaderRec: PtrRec absolute UnitHeader;
    NumberOfUnitsWithInitialization, N, Segment: Word;
begin
  If SourceType < stUnit then
    begin
      GenerateInstruction_CALL_FAR (SysProc_InitTurbo);
      UnitHeader := Ptr (LastLoadedUsedUnit, 0);
      Repeat
        If ufInstructions80x87 in UnitHeader^.Flags then
          begin
            If Emulation80x87 in ModuleCompilerSwitches then GenerateInstruction_CALL_FAR (SysProc_InitEM86)
              else GenerateInstruction_CALL_FAR (SysProc_InitEM87);
            Break;
          end;
        UnitHeaderRec.Seg := UnitHeader^.PreviousUnitSegment;
      until UnitHeaderRec.Seg = 0;
      UnitHeader := Ptr (LastLoadedUsedUnit, 0);
      NumberOfUnitsWithInitialization := 0;
      While UnitHeader^.PreviousUnitSegment <> 0 do
        begin
          UnitHeaderRec.Seg := UnitHeader^.PreviousUnitSegment;
          If PProceduresBlockRecord (Ptr (UnitHeaderRec.Seg,
                                          UnitHeader^.BlockOffset [stProcedures]))^.ProgramCodeBlockRecordOffset <> $FFFF then
            begin
              Inc (NumberOfUnitsWithInitialization);
              Asm
                LES     DI, UnitHeader
                PUSH    ES
              end;
            end;
        end;
      For N := 1 to NumberOfUnitsWithInitialization do
        begin
          GenerateInstruction_Byte (CALL_FAR);
          Asm
            POP    Segment
          end;
          UnitHeaderRec.Seg := Segment;
          GenerateReference (Segment, 0, 0, [rfSegment, rfOffset]);
        end;
    end;
  CreateStackFrame;
  GenerateModuleEntryCode := EndSubroutine;
end;
```

### Code Block 2

```pascal
Function GenerateModuleExitCode: Word;
begin
  DestroyStackFrame;
  If SourceType >= stUnit then GenerateInstruction_Byte (RETF) else
    begin
      GenerateInstruction_Word (XOR_AX_AX);
      GenerateInstruction_CALL_FAR (SysProc_HaltTurbo);
    end;
  GenerateModuleExitCode := EndSubroutine;
end;
```

### Code Block 3

```pascal
Function GenerateModuleEntryCode: Word;
Var UnitHeader: PUnitHeader;
    UnitHeaderRec: PtrRec absolute UnitHeader;
    NumberOfUnitsWithInitialization, N, Segment: Word;
begin
  If SourceType < stUnit then
    begin
      GenerateInstruction_CALL_FAR (SysProc_InitTurbo);
      UnitHeader := Ptr (LastLoadedUsedUnit, 0);
      Repeat
        If ufInstructions80x87 in UnitHeader^.Flags then
          begin
            If Emulation80x87 in ModuleCompilerSwitches then GenerateInstruction_CALL_FAR (SysProc_InitEM86)
              else GenerateInstruction_CALL_FAR (SysProc_InitEM87);
            Break;
          end;
        UnitHeaderRec.Seg := UnitHeader^.PreviousUnitSegment;
      until UnitHeaderRec.Seg = 0;
      UnitHeader := Ptr (LastLoadedUsedUnit, 0);
      NumberOfUnitsWithInitialization := 0;
      While UnitHeader^.PreviousUnitSegment <> 0 do
        begin
          UnitHeaderRec.Seg := UnitHeader^.PreviousUnitSegment;
          If PProceduresBlockRecord (Ptr (UnitHeaderRec.Seg,
                                          UnitHeader^.BlockOffset [stProcedures]))^.ProgramCodeBlockRecordOffset <> $FFFF then
            begin
              Inc (NumberOfUnitsWithInitialization);
              Asm
                LES     DI, UnitHeader
                PUSH    ES
              end;
            end;
        end;
      For N := 1 to NumberOfUnitsWithInitialization do
        begin
          GenerateInstruction_Byte (CALL_FAR);
          Asm
            POP    Segment
          end;
          UnitHeaderRec.Seg := Segment;
          GenerateReference (Segment, 0, 0, [rfSegment, rfOffset]);
        end;
    end;
  CreateStackFrame;
  GenerateModuleEntryCode := EndSubroutine;
end;
```

### Code Block 4

```pascal
Function GenerateModuleExitCode: Word;
begin
  DestroyStackFrame;
  If SourceType >= stUnit then GenerateInstruction_Byte (RETF) else
    begin
      GenerateInstruction_Word (XOR_AX_AX);
      GenerateInstruction_CALL_FAR (SysProc_HaltTurbo);
    end;
  GenerateModuleExitCode := EndSubroutine;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-and-destroying-stack-frame
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/module-entry-and-exit-code
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

