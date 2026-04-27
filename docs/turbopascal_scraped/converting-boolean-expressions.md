# Converting Boolean Expressions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/converting-boolean-expressions

---

## Table of Contents

- Converting Boolean Expressions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Converting Boolean Expressions
Turbo Pascal uses a very interesting way of dealing with boolean expressions. Each boolean expression contains a conditional opcode to jump if the expression evaluates to
True
and two linked lists of jumps. One list contains jumps to code which executes when the expression evaluates to
True
while the other list contains jumps to code which executes when expression evaluates to
False
. Those lists are very convenient when the code needs to jump to one or another branch like in short-circuit expression evaluation.
Procedure TExpression.SetExpressionToBooleanJump (OpCode: Word);
begin
  Location := elBoolean;
  DataType := [];
  LocationData.JumpIfTrueOpCode := OpCode;
  Value.LastJumpToTrue := 0;
  Value.LastJumpToFalse := 0;
  TypeDefPtr := Ptr (SystemUnitSegment, Boolean_TypeOffset);
end;
This procedure converts a numerical value into boolean expression. It uses OR or CMP with 0 instruction and JNE jump.
Procedure TExpression.ConvertExpressionToBooleanJump;
begin
  If Location <> elBoolean then
    begin
      Calculate;
      Case Location of
        elMemory: Case it32Bit in DataType of
                    True: begin
                            Exclude (DataType, it16Bit);
                            LoadExpressionToRegister (rAX);
                            Inc (Value.Offset, 2);
                            GenerateArithmeticInstructionWith_ACC (OR_Operations);
                            Include (UsedRegisters, urAX);
                          end;
                    else begin
                           Case it16Bit in DataType of
                             True: GenerateInstructionWithExpressionInMemOrReg (
                               ArithmeticOp_16Bit_reg__mod_rm, mod_CMP_rm);
                             else GenerateInstructionWithExpressionInMemOrReg (
                               ArithmeticOperation, mod_CMP_rm);
                           end;
                           GenerateInstruction_Byte ($00);
                         end;
                  end;
        else begin
               If Location <> elRegister then LoadExpressionToRegisters (urDX_AX);
               If it32Bit in DataType then GenerateInstruction_TwoBytes (OR_16Bit, AX_DX) else
                 If it16Bit in DataType then GenerateInstruction_TwoBytes (OR_16Bit, AX_AX) else
                                                    GenerateInstruction_TwoBytes (OR_Operations, AL_AL);
             end;
      end;
      EndIntermediateCodeSubroutine;
      Location := elBoolean;
      DataType := [];
      LocationData.JumpIfTrueOpCode := JNE;
      Value.ShortCircuitJumps := 0;
    end;
end;
This procedure converts boolean expresion which evaluates to
True
to
1
and boolean expresion which evaluates to
False
to
0
. First the jumps are converted into integer value and then normalized.
Procedure TExpression.ConvertToBooleanByte;
begin
  If TypeDefPtr^.BaseType = btBoolean then
    begin
      ConvertBooleanJumpToByteValue;
      Case Location of
        elConstant: If Value.LongInt <> 0 then Value.LongInt := 1;
        else begin
               If DataType = [] then Exit;
               Calculate;
               LoadExpressionToRegisters (urDX_AX);
               If it32Bit in DataType then GenerateInstruction_TwoBytes (OR_16Bit, AX_DX);
               Case it16Bit in DataType of
                 True: GenerateInstruction_Word (NEG_AX);
                 else GenerateInstruction_Word (NEG_AL);
               end;
               GenerateInstruction_Word (SBB_AL_AL);
               GenerateInstruction_Word (NEG_AL);
               EndIntermediateCodeSubroutine;
             end;
      end;
      DataType := [];
    end;
end;
This procedure converts boolean jumps into integer value in
AL
. If there are no short-circuit jumps the jump-if-true opcode is inverted (
xor 1
) to skip incrementing the value in
AL
if the expression evaluates to false.
Procedure TExpression.ConvertBooleanJumpToByteValue;
begin
  If Location = elBoolean then
    begin
      Calculate;
      Case Value.ShortCircuitJumps <> 0 of
        True: begin
                GenerateCodeForNearJump (Value.LastJumpToTrue, LocationData.JumpIfTrueOpCode);
                GenerateLabelAndSetJumpsToIt (Value.LastJumpToFalse);
                GenerateInstruction_TwoBytes (MOV_AL_Immediate, 0);
                GenerateInstruction_TwoBytes (JMP_ShortDirect, 2);
                GenerateLabelAndSetJumpsToIt (Value.LastJumpToTrue);
                GenerateInstruction_TwoBytes (MOV_AL_Immediate, 1);
              end;
        else begin
               GenerateInstruction_TwoBytes (MOV_AL_Immediate, 0);
               GenerateInstruction_TwoBytes (LocationData.JumpIfTrueOpCode xor 1, 1);
               GenerateInstruction_Byte (INC_AX);
             end;
      end;
      EndIntermediateCodeSubroutine;
      Location := elRegister;
      Include (UsedRegisters, urAX);
      LocationData.Register := rAX;
    end;
end;
This procedure sets expression to constant boolean value.
Procedure TExpression.SetConstantBooleanExpression (BoolValue: Boolean);
begin
  Location := elConstant;
  DataType := [];
  UsedRegisters := [];
  Value.LongInt := LongInt (BoolValue);
  TypeDefPtr := Ptr (SystemUnitSegment, Boolean_TypeOffset);
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
Procedure TExpression.SetExpressionToBooleanJump (OpCode: Word);
begin
  Location := elBoolean;
  DataType := [];
  LocationData.JumpIfTrueOpCode := OpCode;
  Value.LastJumpToTrue := 0;
  Value.LastJumpToFalse := 0;
  TypeDefPtr := Ptr (SystemUnitSegment, Boolean_TypeOffset);
end;

```

### Code Block 2

```pascal
Procedure TExpression.ConvertExpressionToBooleanJump;
begin
  If Location <> elBoolean then
    begin
      Calculate;
      Case Location of
        elMemory: Case it32Bit in DataType of
                    True: begin
                            Exclude (DataType, it16Bit);
                            LoadExpressionToRegister (rAX);
                            Inc (Value.Offset, 2);
                            GenerateArithmeticInstructionWith_ACC (OR_Operations);
                            Include (UsedRegisters, urAX);
                          end;
                    else begin
                           Case it16Bit in DataType of
                             True: GenerateInstructionWithExpressionInMemOrReg (
                               ArithmeticOp_16Bit_reg__mod_rm, mod_CMP_rm);
                             else GenerateInstructionWithExpressionInMemOrReg (
                               ArithmeticOperation, mod_CMP_rm);
                           end;
                           GenerateInstruction_Byte ($00);
                         end;
                  end;
        else begin
               If Location <> elRegister then LoadExpressionToRegisters (urDX_AX);
               If it32Bit in DataType then GenerateInstruction_TwoBytes (OR_16Bit, AX_DX) else
                 If it16Bit in DataType then GenerateInstruction_TwoBytes (OR_16Bit, AX_AX) else
                                                    GenerateInstruction_TwoBytes (OR_Operations, AL_AL);
             end;
      end;
      EndIntermediateCodeSubroutine;
      Location := elBoolean;
      DataType := [];
      LocationData.JumpIfTrueOpCode := JNE;
      Value.ShortCircuitJumps := 0;
    end;
end;
```

### Code Block 3

```pascal
Procedure TExpression.ConvertToBooleanByte;
begin
  If TypeDefPtr^.BaseType = btBoolean then
    begin
      ConvertBooleanJumpToByteValue;
      Case Location of
        elConstant: If Value.LongInt <> 0 then Value.LongInt := 1;
        else begin
               If DataType = [] then Exit;
               Calculate;
               LoadExpressionToRegisters (urDX_AX);
               If it32Bit in DataType then GenerateInstruction_TwoBytes (OR_16Bit, AX_DX);
               Case it16Bit in DataType of
                 True: GenerateInstruction_Word (NEG_AX);
                 else GenerateInstruction_Word (NEG_AL);
               end;
               GenerateInstruction_Word (SBB_AL_AL);
               GenerateInstruction_Word (NEG_AL);
               EndIntermediateCodeSubroutine;
             end;
      end;
      DataType := [];
    end;
end;
```

### Code Block 4

```pascal
Procedure TExpression.ConvertBooleanJumpToByteValue;
begin
  If Location = elBoolean then
    begin
      Calculate;
      Case Value.ShortCircuitJumps <> 0 of
        True: begin
                GenerateCodeForNearJump (Value.LastJumpToTrue, LocationData.JumpIfTrueOpCode);
                GenerateLabelAndSetJumpsToIt (Value.LastJumpToFalse);
                GenerateInstruction_TwoBytes (MOV_AL_Immediate, 0);
                GenerateInstruction_TwoBytes (JMP_ShortDirect, 2);
                GenerateLabelAndSetJumpsToIt (Value.LastJumpToTrue);
                GenerateInstruction_TwoBytes (MOV_AL_Immediate, 1);
              end;
        else begin
               GenerateInstruction_TwoBytes (MOV_AL_Immediate, 0);
               GenerateInstruction_TwoBytes (LocationData.JumpIfTrueOpCode xor 1, 1);
               GenerateInstruction_Byte (INC_AX);
             end;
      end;
      EndIntermediateCodeSubroutine;
      Location := elRegister;
      Include (UsedRegisters, urAX);
      LocationData.Register := rAX;
    end;
end;
```

### Code Block 5

```pascal
Procedure TExpression.SetConstantBooleanExpression (BoolValue: Boolean);
begin
  Location := elConstant;
  DataType := [];
  UsedRegisters := [];
  Value.LongInt := LongInt (BoolValue);
  TypeDefPtr := Ptr (SystemUnitSegment, Boolean_TypeOffset);
end;
```

### Code Block 6

```pascal
Procedure TExpression.SetExpressionToBooleanJump (OpCode: Word);
begin
  Location := elBoolean;
  DataType := [];
  LocationData.JumpIfTrueOpCode := OpCode;
  Value.LastJumpToTrue := 0;
  Value.LastJumpToFalse := 0;
  TypeDefPtr := Ptr (SystemUnitSegment, Boolean_TypeOffset);
end;

```

### Code Block 7

```pascal
Procedure TExpression.ConvertExpressionToBooleanJump;
begin
  If Location <> elBoolean then
    begin
      Calculate;
      Case Location of
        elMemory: Case it32Bit in DataType of
                    True: begin
                            Exclude (DataType, it16Bit);
                            LoadExpressionToRegister (rAX);
                            Inc (Value.Offset, 2);
                            GenerateArithmeticInstructionWith_ACC (OR_Operations);
                            Include (UsedRegisters, urAX);
                          end;
                    else begin
                           Case it16Bit in DataType of
                             True: GenerateInstructionWithExpressionInMemOrReg (
                               ArithmeticOp_16Bit_reg__mod_rm, mod_CMP_rm);
                             else GenerateInstructionWithExpressionInMemOrReg (
                               ArithmeticOperation, mod_CMP_rm);
                           end;
                           GenerateInstruction_Byte ($00);
                         end;
                  end;
        else begin
               If Location <> elRegister then LoadExpressionToRegisters (urDX_AX);
               If it32Bit in DataType then GenerateInstruction_TwoBytes (OR_16Bit, AX_DX) else
                 If it16Bit in DataType then GenerateInstruction_TwoBytes (OR_16Bit, AX_AX) else
                                                    GenerateInstruction_TwoBytes (OR_Operations, AL_AL);
             end;
      end;
      EndIntermediateCodeSubroutine;
      Location := elBoolean;
      DataType := [];
      LocationData.JumpIfTrueOpCode := JNE;
      Value.ShortCircuitJumps := 0;
    end;
end;
```

### Code Block 8

```pascal
Procedure TExpression.ConvertToBooleanByte;
begin
  If TypeDefPtr^.BaseType = btBoolean then
    begin
      ConvertBooleanJumpToByteValue;
      Case Location of
        elConstant: If Value.LongInt <> 0 then Value.LongInt := 1;
        else begin
               If DataType = [] then Exit;
               Calculate;
               LoadExpressionToRegisters (urDX_AX);
               If it32Bit in DataType then GenerateInstruction_TwoBytes (OR_16Bit, AX_DX);
               Case it16Bit in DataType of
                 True: GenerateInstruction_Word (NEG_AX);
                 else GenerateInstruction_Word (NEG_AL);
               end;
               GenerateInstruction_Word (SBB_AL_AL);
               GenerateInstruction_Word (NEG_AL);
               EndIntermediateCodeSubroutine;
             end;
      end;
      DataType := [];
    end;
end;
```

### Code Block 9

```pascal
Procedure TExpression.ConvertBooleanJumpToByteValue;
begin
  If Location = elBoolean then
    begin
      Calculate;
      Case Value.ShortCircuitJumps <> 0 of
        True: begin
                GenerateCodeForNearJump (Value.LastJumpToTrue, LocationData.JumpIfTrueOpCode);
                GenerateLabelAndSetJumpsToIt (Value.LastJumpToFalse);
                GenerateInstruction_TwoBytes (MOV_AL_Immediate, 0);
                GenerateInstruction_TwoBytes (JMP_ShortDirect, 2);
                GenerateLabelAndSetJumpsToIt (Value.LastJumpToTrue);
                GenerateInstruction_TwoBytes (MOV_AL_Immediate, 1);
              end;
        else begin
               GenerateInstruction_TwoBytes (MOV_AL_Immediate, 0);
               GenerateInstruction_TwoBytes (LocationData.JumpIfTrueOpCode xor 1, 1);
               GenerateInstruction_Byte (INC_AX);
             end;
      end;
      EndIntermediateCodeSubroutine;
      Location := elRegister;
      Include (UsedRegisters, urAX);
      LocationData.Register := rAX;
    end;
end;
```

### Code Block 10

```pascal
Procedure TExpression.SetConstantBooleanExpression (BoolValue: Boolean);
begin
  Location := elConstant;
  DataType := [];
  UsedRegisters := [];
  Value.LongInt := LongInt (BoolValue);
  TypeDefPtr := Ptr (SystemUnitSegment, Boolean_TypeOffset);
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/converting-boolean-expressions
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

