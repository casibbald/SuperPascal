# Creating and Destroying Stack Frame - Turbo Pascal Compiler

**URL:** https://turbopascal.org/creating-and-destroying-stack-frame

---

## Table of Contents

- Creating and Destroying Stack Frame

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Creating and Destroying Stack Frame
Programs created with Turbo Pascal use stack frame. This is part of the processor’s stack space which can be accessed using a dedicated base pointer (
BP
) register.
PUSH    BP
MOV     BP, SP
SUB     SP, StackFrameSize
...
...
...
MOV     SP, BP
POP     BP
RET
Stack frame is usually used for local variables. This is very convenient because
BP
can acces all pushed parameters (using positive offset) and local variables in stack frame (using negative offset). Two procedures generate code to create and destroy stack frame.
Procedure CreateStackFrame;
begin
  If (Instructions80286 in StatementCompilerSwitches) and (ProgramBlockMaxStackFrameOffset &lt;&gt; 0) and
    not (StackChecking in StatementCompilerSwitches) then
      begin
        GenerateInstruction_Byte (ENTER);
        GenerateInstruction_Word (- ProgramBlockMaxStackFrameOffset);
        GenerateInstruction_Byte (0);
      end else begin
                 GenerateInstruction_Byte (PUSH_BP);
                 GenerateInstruction_Word (MOV_BP_SP);
                 If StackChecking in StatementCompilerSwitches then
                  begin
                    GenerateInstruction_MOV_16BitReg_Immediate (rAX, - ProgramBlockMaxStackFrameOffset);
                    GenerateInstruction_CALL_FAR (SysProc_StackCheck);
                  end;
                 If ProgramBlockMaxStackFrameOffset &lt;&gt; 0 then
                   GenerateArithmeticInstruction_16BitReg_Immediate (SUB_SP, - ProgramBlockMaxStackFrameOffset);
               end;
end;

Procedure DestroyStackFrame;
begin
  If Instructions80286 in StatementCompilerSwitches then GenerateInstruction_Byte (LEAVE)
    else begin
           If ProgramBlockMaxStackFrameOffset or NumberOfLocalParameters &lt;&gt; 0 then GenerateInstruction_Word (MOV_SP_BP);
           GenerateInstruction_Byte (POP_BP)
         end;
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
PUSH    BP
MOV     BP, SP
SUB     SP, StackFrameSize
...
...
...
MOV     SP, BP
POP     BP
RET
```

### Code Block 2

```pascal
Procedure CreateStackFrame;
begin
  If (Instructions80286 in StatementCompilerSwitches) and (ProgramBlockMaxStackFrameOffset &lt;&gt; 0) and
    not (StackChecking in StatementCompilerSwitches) then
      begin
        GenerateInstruction_Byte (ENTER);
        GenerateInstruction_Word (- ProgramBlockMaxStackFrameOffset);
        GenerateInstruction_Byte (0);
      end else begin
                 GenerateInstruction_Byte (PUSH_BP);
                 GenerateInstruction_Word (MOV_BP_SP);
                 If StackChecking in StatementCompilerSwitches then
                  begin
                    GenerateInstruction_MOV_16BitReg_Immediate (rAX, - ProgramBlockMaxStackFrameOffset);
                    GenerateInstruction_CALL_FAR (SysProc_StackCheck);
                  end;
                 If ProgramBlockMaxStackFrameOffset &lt;&gt; 0 then
                   GenerateArithmeticInstruction_16BitReg_Immediate (SUB_SP, - ProgramBlockMaxStackFrameOffset);
               end;
end;

Procedure DestroyStackFrame;
begin
  If Instructions80286 in StatementCompilerSwitches then GenerateInstruction_Byte (LEAVE)
    else begin
           If ProgramBlockMaxStackFrameOffset or NumberOfLocalParameters &lt;&gt; 0 then GenerateInstruction_Word (MOV_SP_BP);
           GenerateInstruction_Byte (POP_BP)
         end;
end;
```

### Code Block 3

```pascal
PUSH    BP
MOV     BP, SP
SUB     SP, StackFrameSize
...
...
...
MOV     SP, BP
POP     BP
RET
```

### Code Block 4

```pascal
Procedure CreateStackFrame;
begin
  If (Instructions80286 in StatementCompilerSwitches) and (ProgramBlockMaxStackFrameOffset &lt;&gt; 0) and
    not (StackChecking in StatementCompilerSwitches) then
      begin
        GenerateInstruction_Byte (ENTER);
        GenerateInstruction_Word (- ProgramBlockMaxStackFrameOffset);
        GenerateInstruction_Byte (0);
      end else begin
                 GenerateInstruction_Byte (PUSH_BP);
                 GenerateInstruction_Word (MOV_BP_SP);
                 If StackChecking in StatementCompilerSwitches then
                  begin
                    GenerateInstruction_MOV_16BitReg_Immediate (rAX, - ProgramBlockMaxStackFrameOffset);
                    GenerateInstruction_CALL_FAR (SysProc_StackCheck);
                  end;
                 If ProgramBlockMaxStackFrameOffset &lt;&gt; 0 then
                   GenerateArithmeticInstruction_16BitReg_Immediate (SUB_SP, - ProgramBlockMaxStackFrameOffset);
               end;
end;

Procedure DestroyStackFrame;
begin
  If Instructions80286 in StatementCompilerSwitches then GenerateInstruction_Byte (LEAVE)
    else begin
           If ProgramBlockMaxStackFrameOffset or NumberOfLocalParameters &lt;&gt; 0 then GenerateInstruction_Word (MOV_SP_BP);
           GenerateInstruction_Byte (POP_BP)
         end;
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

