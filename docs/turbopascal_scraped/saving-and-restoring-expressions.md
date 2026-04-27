# Saving and Restoring Expressions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/saving-and-restoring-expressions

---

## Table of Contents

- Saving and Restoring Expressions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Saving and Restoring Expressions
During expression calculation Turbo Pascal stores the value in registers. However, they are limited and some operations can be performed only with specific registers. Therefore, Turbo Pascal compiler needs some way to save and restore expressions.
Using stack is an elegant way to temprarily save expressions until registers are needed for other calculations. The power of this mechanism lies in the possibility to use
non-linear way of generating code
. This means that the compiler can generate code for some part of the calculation and then according to used registers (which are tracked) proceed with next calculation or save it to the stack to free registers. The first procedure saves the expression and pushes 2 or 4 bytes to stack.
Procedure TExpression.Push;
begin
  Case Location of
    elMemory: begin
                If not (it16Bit in DataType) then
                  begin
                    LoadExpressionToRegisters (urAX);
                    GenerateInstruction_Byte (PUSH_AX);
                  end else begin
                             If it32Bit in DataType then
                               begin
                                 Inc (Value.Offset, 2);
                                 GenerateInstructionWithExpressionInMemOrReg (PUSH_RegMem, PUSH_Mem);
                                 Dec (Value.Offset, 2);
                               end;
                             GenerateInstructionWithExpressionInMemOrReg (PUSH_RegMem, PUSH_Mem);
                           end;
              end;
    elRegister: begin
                  If it32Bit in DataType then GenerateInstruction_Byte (PUSH_DX);
                  GenerateInstruction_Byte (PUSH_AX);
                end;
    elPointerToMemory: begin
                         If not (efSegment in LocationData.Flags) then
                           begin
                             LoadOffsetTo_DI;
                             If not (it32Bit in DataType) then
                               begin
                                 GenerateInstruction_Byte (PUSH_DI);
                                 Location := elStack;
                                 Exit;
                               end;
                           end;
                         If LocationData.Flags * SegmentMask = [] then
                           begin
                             GenerateInstructionWithSegment (MOV_AX_Immediate);
                             GenerateInstruction_Byte (PUSH_AX);
                           end else GenerateInstruction_Byte (PUSH_SegmentRegister or ExpressionSegmentRegister);
                         If it32Bit in DataType then GenerateInstruction_Byte (PUSH_DI);
                       end;
    else begin
           If Instructions80286 in StatementCompilerSwitches then
             begin
               If it32Bit in DataType then GenerateInstruction_PUSH_Immediate (Value.LongRec.WordH);
               If it16Bit in DataType then GenerateInstruction_PUSH_Immediate (Value.LongRec.WordL) else
                 GenerateInstruction_PUSH_Immediate (Value.ShortInt);
             end else begin
                        LoadExpressionToRegisters (urAX);
                        If it32Bit in DataType then GenerateInstruction_Byte (PUSH_DX);
                        GenerateInstruction_Byte (PUSH_AX);
                      end;
         end;
  end;
  Location := elStack;
end;
This procedure calculates the expression and pushes it on the stack.
Procedure TExpression.PushExpression;
begin
  Calculate;
  Push;
  EndIntermediateCodeSubroutine;
end;
This procedure restores pushed expression to registers. It avoids already used registers and pops data to free ones.
Procedure TExpression.PopToRegisters (AlreadyUsedRegisters: TUsedRegistersSet);
Var OpCode, OpCode2: Byte;
    NewUsedRegs: TUsedRegistersSet;
    Register: Byte;
begin
  If Location = elStack then
    begin
      Location := elRegister;
      Case it32Bit in DataType of
        True: begin
                OpCode := POP_AX;
                OpCode2 := POP_DX;
                Register := rDX_AX;
                NewUsedRegs := [urDX, urAX];
                If [urDX, urAX] * AlreadyUsedRegisters <> [] then
                  begin
                    OpCode := POP_CX;
                    OpCode2 := POP_BX;
                    Register := rBX_CX;
                    NewUsedRegs := [urBX, urCX];
                  end;
                GenerateInstruction_TwoBytes (OpCode, OpCode2);
              end;
        else begin
               Case urAX in AlreadyUsedRegisters of
                 True: Case urDX in AlreadyUsedRegisters of
                         True: Case urCX in AlreadyUsedRegisters of
                                 True: begin
                                         OpCode := POP_BX;
                                         Register := rBX;
                                         NewUsedRegs := [urBX];
                                       end;
                                 else begin
                                        OpCode := POP_CX;
                                        Register := rCX;
                                        NewUsedRegs := [urCX];
                                      end;
                               end;
                         else begin
                                OpCode := POP_DX;
                                Register := rDX;
                                NewUsedRegs := [urDX];
                              end;
                       end;
                 else begin
                        OpCode := POP_AX;
                        Register := rAX;
                        NewUsedRegs := [urAX];
                      end;
               end;
               GenerateInstruction_Byte (OpCode);
             end;
      end;
      UsedRegisters := UsedRegisters + NewUsedRegs;
      LocationData.Register := Register;
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
Procedure TExpression.Push;
begin
  Case Location of
    elMemory: begin
                If not (it16Bit in DataType) then
                  begin
                    LoadExpressionToRegisters (urAX);
                    GenerateInstruction_Byte (PUSH_AX);
                  end else begin
                             If it32Bit in DataType then
                               begin
                                 Inc (Value.Offset, 2);
                                 GenerateInstructionWithExpressionInMemOrReg (PUSH_RegMem, PUSH_Mem);
                                 Dec (Value.Offset, 2);
                               end;
                             GenerateInstructionWithExpressionInMemOrReg (PUSH_RegMem, PUSH_Mem);
                           end;
              end;
    elRegister: begin
                  If it32Bit in DataType then GenerateInstruction_Byte (PUSH_DX);
                  GenerateInstruction_Byte (PUSH_AX);
                end;
    elPointerToMemory: begin
                         If not (efSegment in LocationData.Flags) then
                           begin
                             LoadOffsetTo_DI;
                             If not (it32Bit in DataType) then
                               begin
                                 GenerateInstruction_Byte (PUSH_DI);
                                 Location := elStack;
                                 Exit;
                               end;
                           end;
                         If LocationData.Flags * SegmentMask = [] then
                           begin
                             GenerateInstructionWithSegment (MOV_AX_Immediate);
                             GenerateInstruction_Byte (PUSH_AX);
                           end else GenerateInstruction_Byte (PUSH_SegmentRegister or ExpressionSegmentRegister);
                         If it32Bit in DataType then GenerateInstruction_Byte (PUSH_DI);
                       end;
    else begin
           If Instructions80286 in StatementCompilerSwitches then
             begin
               If it32Bit in DataType then GenerateInstruction_PUSH_Immediate (Value.LongRec.WordH);
               If it16Bit in DataType then GenerateInstruction_PUSH_Immediate (Value.LongRec.WordL) else
                 GenerateInstruction_PUSH_Immediate (Value.ShortInt);
             end else begin
                        LoadExpressionToRegisters (urAX);
                        If it32Bit in DataType then GenerateInstruction_Byte (PUSH_DX);
                        GenerateInstruction_Byte (PUSH_AX);
                      end;
         end;
  end;
  Location := elStack;
end;
```

### Code Block 2

```pascal
Procedure TExpression.PushExpression;
begin
  Calculate;
  Push;
  EndIntermediateCodeSubroutine;
end;
```

### Code Block 3

```pascal
Procedure TExpression.PopToRegisters (AlreadyUsedRegisters: TUsedRegistersSet);
Var OpCode, OpCode2: Byte;
    NewUsedRegs: TUsedRegistersSet;
    Register: Byte;
begin
  If Location = elStack then
    begin
      Location := elRegister;
      Case it32Bit in DataType of
        True: begin
                OpCode := POP_AX;
                OpCode2 := POP_DX;
                Register := rDX_AX;
                NewUsedRegs := [urDX, urAX];
                If [urDX, urAX] * AlreadyUsedRegisters <> [] then
                  begin
                    OpCode := POP_CX;
                    OpCode2 := POP_BX;
                    Register := rBX_CX;
                    NewUsedRegs := [urBX, urCX];
                  end;
                GenerateInstruction_TwoBytes (OpCode, OpCode2);
              end;
        else begin
               Case urAX in AlreadyUsedRegisters of
                 True: Case urDX in AlreadyUsedRegisters of
                         True: Case urCX in AlreadyUsedRegisters of
                                 True: begin
                                         OpCode := POP_BX;
                                         Register := rBX;
                                         NewUsedRegs := [urBX];
                                       end;
                                 else begin
                                        OpCode := POP_CX;
                                        Register := rCX;
                                        NewUsedRegs := [urCX];
                                      end;
                               end;
                         else begin
                                OpCode := POP_DX;
                                Register := rDX;
                                NewUsedRegs := [urDX];
                              end;
                       end;
                 else begin
                        OpCode := POP_AX;
                        Register := rAX;
                        NewUsedRegs := [urAX];
                      end;
               end;
               GenerateInstruction_Byte (OpCode);
             end;
      end;
      UsedRegisters := UsedRegisters + NewUsedRegs;
      LocationData.Register := Register;
    end;
end;
```

### Code Block 4

```pascal
Procedure TExpression.Push;
begin
  Case Location of
    elMemory: begin
                If not (it16Bit in DataType) then
                  begin
                    LoadExpressionToRegisters (urAX);
                    GenerateInstruction_Byte (PUSH_AX);
                  end else begin
                             If it32Bit in DataType then
                               begin
                                 Inc (Value.Offset, 2);
                                 GenerateInstructionWithExpressionInMemOrReg (PUSH_RegMem, PUSH_Mem);
                                 Dec (Value.Offset, 2);
                               end;
                             GenerateInstructionWithExpressionInMemOrReg (PUSH_RegMem, PUSH_Mem);
                           end;
              end;
    elRegister: begin
                  If it32Bit in DataType then GenerateInstruction_Byte (PUSH_DX);
                  GenerateInstruction_Byte (PUSH_AX);
                end;
    elPointerToMemory: begin
                         If not (efSegment in LocationData.Flags) then
                           begin
                             LoadOffsetTo_DI;
                             If not (it32Bit in DataType) then
                               begin
                                 GenerateInstruction_Byte (PUSH_DI);
                                 Location := elStack;
                                 Exit;
                               end;
                           end;
                         If LocationData.Flags * SegmentMask = [] then
                           begin
                             GenerateInstructionWithSegment (MOV_AX_Immediate);
                             GenerateInstruction_Byte (PUSH_AX);
                           end else GenerateInstruction_Byte (PUSH_SegmentRegister or ExpressionSegmentRegister);
                         If it32Bit in DataType then GenerateInstruction_Byte (PUSH_DI);
                       end;
    else begin
           If Instructions80286 in StatementCompilerSwitches then
             begin
               If it32Bit in DataType then GenerateInstruction_PUSH_Immediate (Value.LongRec.WordH);
               If it16Bit in DataType then GenerateInstruction_PUSH_Immediate (Value.LongRec.WordL) else
                 GenerateInstruction_PUSH_Immediate (Value.ShortInt);
             end else begin
                        LoadExpressionToRegisters (urAX);
                        If it32Bit in DataType then GenerateInstruction_Byte (PUSH_DX);
                        GenerateInstruction_Byte (PUSH_AX);
                      end;
         end;
  end;
  Location := elStack;
end;
```

### Code Block 5

```pascal
Procedure TExpression.PushExpression;
begin
  Calculate;
  Push;
  EndIntermediateCodeSubroutine;
end;
```

### Code Block 6

```pascal
Procedure TExpression.PopToRegisters (AlreadyUsedRegisters: TUsedRegistersSet);
Var OpCode, OpCode2: Byte;
    NewUsedRegs: TUsedRegistersSet;
    Register: Byte;
begin
  If Location = elStack then
    begin
      Location := elRegister;
      Case it32Bit in DataType of
        True: begin
                OpCode := POP_AX;
                OpCode2 := POP_DX;
                Register := rDX_AX;
                NewUsedRegs := [urDX, urAX];
                If [urDX, urAX] * AlreadyUsedRegisters <> [] then
                  begin
                    OpCode := POP_CX;
                    OpCode2 := POP_BX;
                    Register := rBX_CX;
                    NewUsedRegs := [urBX, urCX];
                  end;
                GenerateInstruction_TwoBytes (OpCode, OpCode2);
              end;
        else begin
               Case urAX in AlreadyUsedRegisters of
                 True: Case urDX in AlreadyUsedRegisters of
                         True: Case urCX in AlreadyUsedRegisters of
                                 True: begin
                                         OpCode := POP_BX;
                                         Register := rBX;
                                         NewUsedRegs := [urBX];
                                       end;
                                 else begin
                                        OpCode := POP_CX;
                                        Register := rCX;
                                        NewUsedRegs := [urCX];
                                      end;
                               end;
                         else begin
                                OpCode := POP_DX;
                                Register := rDX;
                                NewUsedRegs := [urDX];
                              end;
                       end;
                 else begin
                        OpCode := POP_AX;
                        Register := rAX;
                        NewUsedRegs := [urAX];
                      end;
               end;
               GenerateInstruction_Byte (OpCode);
             end;
      end;
      UsedRegisters := UsedRegisters + NewUsedRegs;
      LocationData.Register := Register;
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
- https://turbopascal.org/expressions
- https://turbopascal.org/intermediate-code
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/saving-and-restoring-expressions
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

