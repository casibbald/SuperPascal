# Generating Instructions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/generating-instructions

---

## Table of Contents

- Generating Instructions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Generating Instructions
Turbo Pascal uses few procedures to generate any x86 instruction needed. In general two
intermediate code records
are used: one for the actual inctruction opcodes and immediate data and one for references.
Reference records are processed by
code generator
which generates spaces where
Linker
will generate remaining parameters of instructions and place references to program code or data.
Procedure GenerateInstruction_Byte (Code: Byte);
begin
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 2))^ do
    begin
      Recordtype := icByte;
      ByteCode := Code;
    end;
end;

Procedure GenerateInstruction_TwoBytes (B0, B1: Byte);
begin
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 3))^ do
    begin
      Recordtype := icWord;
      WordCode.ByteL := B0;
      WordCode.ByteH := B1;
    end;
end;

Procedure GenerateInstruction_Word (Code: Word);
begin
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 3))^ do
    begin
      Recordtype := icWord;
      WordCode.Word := Code;
    end;
end;
If expression is not in data segment, this procedure generates code for segment override.
Procedure TExpression.GenerateSegmentOverride;
begin
  If LocationData.Flags * [ofsBP, segDS] = [] then
    GenerateInstruction_Byte (SegmentOverride or ExpressionSegment);
end;

Procedure TExpression.GenerateInstructionWithSegment (OpCode: Byte);
begin
  GenerateInstruction_Byte (OpCode);
  If efRelocatable in LocationData.Flags then GenerateReference (Value.Segment, Value.BlockRecord, 0, [rfSegment])
    else GenerateInstruction_Word (Value.VarSegment);
end;
If expression is not in data or code segment, this procedure generates immediate offset value, otherwise it generates offset reference.
Procedure TExpression.GenerateOffsetReferenceForExpressionInMemory;
Var Flags: TReferenceFlagSet;
begin
  If LocationData.Flags * [efTypedConstant, segCS, segDS] = [] then GenerateInstruction_Word (Value.Offset) else
    begin
      Case segDS in LocationData.Flags of
        True: If efTypedConstant in LocationData.Flags then
                Flags := [rfDataSegment, rfConstant, rfOffset] else Flags := [rfDataSegment, rfOffset];
        else  If segCS in LocationData.Flags then Flags := [rfConstant, rfOffset] else Flags := [rfOffset];
      end;
      GenerateReference (Value.Segment, Value.BlockRecord, Value.Offset, Flags);
    end;
end;
This procedure generates instruction which references memory location.
Procedure TExpression.GenerateInstructionWithMemoryReference (MainOpCode, MODEM_OpCode: Byte);
begin
  Case ofsBP in LocationData.Flags of
    True: begin
            If ofsDI in LocationData.Flags then MODEM_OpCode := MODEM_OpCode xor rmSS_BP_DI_Displacement
              else MODEM_OpCode := MODEM_OpCode xor rmSS_BP_Displacement;
          end;
    else Case ofsDI in LocationData.Flags of
           True: begin
                   MODEM_OpCode := MODEM_OpCode xor rmDS_DI;
                   If LocationData.Flags * [efTypedConstant, segCS, segDS] <> [] then
                     begin
                       GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode or modMemoryDisp16);
                       GenerateOffsetReferenceForExpressionInMemory;
                       Exit;
                     end;
                   If Value.Word = 0 then
                     begin
                       GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode);
                       Exit;
                     end;
                 end;
           else begin
                   GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode or rmDS_Displacement16);
                   GenerateOffsetReferenceForExpressionInMemory;
                   Exit;
                 end;
         end;
  end;
  If Value.Integer = Value.ShortInt then
    begin
      GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode or modMemoryDisp8);
      GenerateInstruction_Byte (Value.Byte);
    end else begin
               GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode or modMemoryDisp16);
               GenerateOffsetReferenceForExpressionInMemory;
             end;
end;
This procedure generates general instruction with register or memory reference.
Procedure TExpression.GenerateInstructionWithExpressionInMemOrReg (MainOpCode, MODEM_OpCode: Byte);
begin
  Case Location of
    elRegister: GenerateInstruction_TwoBytes (MainOpCode, modRegister or MODEM_OpCode or LocationData.Register)
    else begin
           GenerateSegmentOverride;
           GenerateInstructionWithMemoryReference (MainOpCode, MODEM_OpCode);
         end;
  end;
end;
This procedure generates 8-bit arithmetic instruction with AL or 16-bit arithmetic instruction with
AX
.
Procedure TExpression.GenerateArithmeticInstructionWith_ACC (OpCode: Byte);
begin
  If Location = elConstant then
    begin
      If it16Bit in DataType then
        begin
          OpCode := OpCode or ArithmeticOp_AX_Immediate;
          GenerateInstruction_Byte (OpCode);
          GenerateInstruction_Word (Value.Word);
        end else GenerateInstruction_TwoBytes (OpCode or ArithmeticOp_AL_Immediate, Value.Byte);
    end else GenerateInstruction_8_16_bit (OpCode or reg__mod_rm, rAX);
end;
This procedure generates 16-bit arithmetic instruction with
DX
.
Procedure TExpression.GenerateArithmeticInstructionWith_DX (OpCode: Byte);
begin
  If Location = elConstant then
    GenerateArithmeticInstruction_16BitReg_Immediate (modRegister or OpCode or rDX, Value.Word) else
      GenerateInstructionWithExpressionInMemOrReg (OpCode or reg__mod_rm or OperandMode16Bit, rDX shl 3);
end;
This procedure generates arithmetic instruction with immediate value. If value is in range of
ShortInt
then it generates shorter version of instruction.
Procedure TExpression.GenerateArithmeticInstructionWithImmediateValue (ImmediateValue: Integer; SecondOpCode: Byte);
begin
  If it16Bit in DataType then
    begin
      If ImmediateValue = ShortInt (ImmediateValue) then
        begin
          GenerateInstructionWithExpressionInMemOrReg (ArithmeticOperation or SignExtendImmediateByte or
                                                                                 OperandMode16Bit, SecondOpCode);
          GenerateInstruction_Byte (ImmediateValue);
        end else begin
                   GenerateInstructionWithExpressionInMemOrReg (ArithmeticOperation or OperandMode16Bit, SecondOpCode);
                   GenerateInstruction_Word (ImmediateValue);
                 end;
    end else begin
               GenerateInstructionWithExpressionInMemOrReg (ArithmeticOperation, SecondOpCode);
               GenerateInstruction_Byte (ImmediateValue);
             end;
end;
This procedure generates instruction to move 8, 16 or 32-bit register to memory.
Procedure TExpression.GenerateInstruction_MOV_Memory_Register (Register: Byte);
begin
  GenerateInstruction_16bit_MOV_Memory_Register (Register);
  If it32Bit in DataType then
    begin
      Inc (Value.Offset, 2);
      GenerateInstruction_16bit_MOV_Memory_Register (Register + 2);
      Dec (Value.Offset, 2);
    end;
end;
This procedure generates instruction to move memory to 8, 16 or 32-bit register.
Procedure TExpression.GenerateInstruction_16bit_MOV_Memory_Register (Register: Byte);
begin
  If (Register = rAX) and (LocationData.Flags * [ofsDI, ofsBP] = []) then
    begin
      GenerateSegmentOverride;
      If it16Bit in DataType then GenerateInstruction_Byte (MOV_Mem_AX)
        else GenerateInstruction_Byte (MOV_Mem_AL);
      GenerateOffsetReferenceForExpressionInMemory;
    end else GenerateInstruction_8_16_bit (MOV_Operations, Register shl 3);
end;
This procedure generates general instruction with 8 or 16-bit register or memory reference.
Procedure TExpression.GenerateInstruction_8_16_bit (MainOpcode, SecondOpCode: Byte);
begin
  If it16Bit in DataType then MainOpcode := MainOpcode or OperandMode16Bit;
  GenerateInstructionWithExpressionInMemOrReg (MainOpcode, SecondOpCode);
end;
This procedure generates instruction to move immediate value to memory.
Procedure TExpression.GenerateInstruction_MOV_Memory_Immediate (ImmediateValue: Word);
begin
  GenerateInstruction_8_16_bit (MOV_RegisterOrMemory_Immediate, modMemory);
  If it16Bit in DataType then GenerateInstruction_Word (ImmediateValue) else GenerateInstruction_Byte (Lo (ImmediateValue));
end;
This procedure generates instruction for far call. It marks unit for 80×87 opcodes (if they are used) and generates
CALL FAR
instruction for system procedure. Input parameter is index in
Procedures
symbol table in the
system unit
.
Procedure GenerateInstruction_CALL_FAR (SysProc: Word);
begin
  If SysProc and $8000 <> 0 then MarkUnitForInstructions80x87;
  ES_DI_PointerDestroyed;
  GenerateInstruction_Byte (CALL_FAR);
  GenerateReference (SystemUnitSegment, SysProc shl 1, 0, [rfSegment, rfOffset]);
end;
This procedure generates instruction to access the
stack frame
.
Procedure GenerateInstructionForStackFrameDisplacement (MainOpCode, SecondOpCode: Byte; Displacement: Integer);
begin
  If (SecondOpCode and rmMask <> rmSS_BP_Displacement) and (Displacement = 0) then
    GenerateInstruction_TwoBytes (MainOpCode, SecondOpCode) else
      begin
        If Displacement = ShortInt (Displacement) then
          begin
            GenerateInstruction_TwoBytes (MainOpCode, SecondOpCode or modMemoryDisp8);
            GenerateInstruction_Byte (Displacement);
          end else begin
                     GenerateInstruction_TwoBytes (MainOpCode, SecondOpCode or modMemoryDisp16);
                     GenerateInstruction_Word (Displacement);
                   end;
      end;
end;
This procedure generates instruction to push immediate value.
Procedure GenerateInstruction_PUSH_Immediate (PushWord: Integer);
begin
  If PushWord = ShortInt (Lo (PushWord)) then GenerateInstruction_TwoBytes (PUSH_SignedByte, Lo (PushWord))
    else begin
           GenerateInstruction_Byte (PUSH_Word);
           GenerateInstruction_Word (PushWord);
         end;
end;
This procedure generates code for arithmetic instruction with immediate value.
Procedure GenerateArithmeticInstruction_16BitReg_Immediate (SecondOpCode: Byte; Value: Integer);
begin
  If Value = ShortInt (Value) then
    begin
      GenerateInstruction_TwoBytes (ArithmeticOperation or SignExtendImmediateByte or OperandMode16Bit, SecondOpCode);
      GenerateInstruction_Byte (Lo (Value));
    end else begin
               GenerateInstruction_TwoBytes (ArithmeticOperation or OperandMode16Bit, SecondOpCode);
               GenerateInstruction_Word (Value);
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
Procedure GenerateInstruction_Byte (Code: Byte);
begin
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 2))^ do
    begin
      Recordtype := icByte;
      ByteCode := Code;
    end;
end;

Procedure GenerateInstruction_TwoBytes (B0, B1: Byte);
begin
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 3))^ do
    begin
      Recordtype := icWord;
      WordCode.ByteL := B0;
      WordCode.ByteH := B1;
    end;
end;

Procedure GenerateInstruction_Word (Code: Word);
begin
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 3))^ do
    begin
      Recordtype := icWord;
      WordCode.Word := Code;
    end;
end;
```

### Code Block 2

```pascal
Procedure TExpression.GenerateSegmentOverride;
begin
  If LocationData.Flags * [ofsBP, segDS] = [] then
    GenerateInstruction_Byte (SegmentOverride or ExpressionSegment);
end;

Procedure TExpression.GenerateInstructionWithSegment (OpCode: Byte);
begin
  GenerateInstruction_Byte (OpCode);
  If efRelocatable in LocationData.Flags then GenerateReference (Value.Segment, Value.BlockRecord, 0, [rfSegment])
    else GenerateInstruction_Word (Value.VarSegment);
end;
```

### Code Block 3

```pascal
Procedure TExpression.GenerateOffsetReferenceForExpressionInMemory;
Var Flags: TReferenceFlagSet;
begin
  If LocationData.Flags * [efTypedConstant, segCS, segDS] = [] then GenerateInstruction_Word (Value.Offset) else
    begin
      Case segDS in LocationData.Flags of
        True: If efTypedConstant in LocationData.Flags then
                Flags := [rfDataSegment, rfConstant, rfOffset] else Flags := [rfDataSegment, rfOffset];
        else  If segCS in LocationData.Flags then Flags := [rfConstant, rfOffset] else Flags := [rfOffset];
      end;
      GenerateReference (Value.Segment, Value.BlockRecord, Value.Offset, Flags);
    end;
end;
```

### Code Block 4

```pascal
Procedure TExpression.GenerateInstructionWithMemoryReference (MainOpCode, MODEM_OpCode: Byte);
begin
  Case ofsBP in LocationData.Flags of
    True: begin
            If ofsDI in LocationData.Flags then MODEM_OpCode := MODEM_OpCode xor rmSS_BP_DI_Displacement
              else MODEM_OpCode := MODEM_OpCode xor rmSS_BP_Displacement;
          end;
    else Case ofsDI in LocationData.Flags of
           True: begin
                   MODEM_OpCode := MODEM_OpCode xor rmDS_DI;
                   If LocationData.Flags * [efTypedConstant, segCS, segDS] <> [] then
                     begin
                       GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode or modMemoryDisp16);
                       GenerateOffsetReferenceForExpressionInMemory;
                       Exit;
                     end;
                   If Value.Word = 0 then
                     begin
                       GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode);
                       Exit;
                     end;
                 end;
           else begin
                   GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode or rmDS_Displacement16);
                   GenerateOffsetReferenceForExpressionInMemory;
                   Exit;
                 end;
         end;
  end;
  If Value.Integer = Value.ShortInt then
    begin
      GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode or modMemoryDisp8);
      GenerateInstruction_Byte (Value.Byte);
    end else begin
               GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode or modMemoryDisp16);
               GenerateOffsetReferenceForExpressionInMemory;
             end;
end;
```

### Code Block 5

```pascal
Procedure TExpression.GenerateInstructionWithExpressionInMemOrReg (MainOpCode, MODEM_OpCode: Byte);
begin
  Case Location of
    elRegister: GenerateInstruction_TwoBytes (MainOpCode, modRegister or MODEM_OpCode or LocationData.Register)
    else begin
           GenerateSegmentOverride;
           GenerateInstructionWithMemoryReference (MainOpCode, MODEM_OpCode);
         end;
  end;
end;
```

### Code Block 6

```pascal
Procedure TExpression.GenerateArithmeticInstructionWith_ACC (OpCode: Byte);
begin
  If Location = elConstant then
    begin
      If it16Bit in DataType then
        begin
          OpCode := OpCode or ArithmeticOp_AX_Immediate;
          GenerateInstruction_Byte (OpCode);
          GenerateInstruction_Word (Value.Word);
        end else GenerateInstruction_TwoBytes (OpCode or ArithmeticOp_AL_Immediate, Value.Byte);
    end else GenerateInstruction_8_16_bit (OpCode or reg__mod_rm, rAX);
end;
```

### Code Block 7

```pascal
Procedure TExpression.GenerateArithmeticInstructionWith_DX (OpCode: Byte);
begin
  If Location = elConstant then
    GenerateArithmeticInstruction_16BitReg_Immediate (modRegister or OpCode or rDX, Value.Word) else
      GenerateInstructionWithExpressionInMemOrReg (OpCode or reg__mod_rm or OperandMode16Bit, rDX shl 3);
end;
```

### Code Block 8

```pascal
Procedure TExpression.GenerateArithmeticInstructionWithImmediateValue (ImmediateValue: Integer; SecondOpCode: Byte);
begin
  If it16Bit in DataType then
    begin
      If ImmediateValue = ShortInt (ImmediateValue) then
        begin
          GenerateInstructionWithExpressionInMemOrReg (ArithmeticOperation or SignExtendImmediateByte or
                                                                                 OperandMode16Bit, SecondOpCode);
          GenerateInstruction_Byte (ImmediateValue);
        end else begin
                   GenerateInstructionWithExpressionInMemOrReg (ArithmeticOperation or OperandMode16Bit, SecondOpCode);
                   GenerateInstruction_Word (ImmediateValue);
                 end;
    end else begin
               GenerateInstructionWithExpressionInMemOrReg (ArithmeticOperation, SecondOpCode);
               GenerateInstruction_Byte (ImmediateValue);
             end;
end;
```

### Code Block 9

```pascal
Procedure TExpression.GenerateInstruction_MOV_Memory_Register (Register: Byte);
begin
  GenerateInstruction_16bit_MOV_Memory_Register (Register);
  If it32Bit in DataType then
    begin
      Inc (Value.Offset, 2);
      GenerateInstruction_16bit_MOV_Memory_Register (Register + 2);
      Dec (Value.Offset, 2);
    end;
end;
```

### Code Block 10

```pascal
Procedure TExpression.GenerateInstruction_16bit_MOV_Memory_Register (Register: Byte);
begin
  If (Register = rAX) and (LocationData.Flags * [ofsDI, ofsBP] = []) then
    begin
      GenerateSegmentOverride;
      If it16Bit in DataType then GenerateInstruction_Byte (MOV_Mem_AX)
        else GenerateInstruction_Byte (MOV_Mem_AL);
      GenerateOffsetReferenceForExpressionInMemory;
    end else GenerateInstruction_8_16_bit (MOV_Operations, Register shl 3);
end;
```

### Code Block 11

```pascal
Procedure TExpression.GenerateInstruction_8_16_bit (MainOpcode, SecondOpCode: Byte);
begin
  If it16Bit in DataType then MainOpcode := MainOpcode or OperandMode16Bit;
  GenerateInstructionWithExpressionInMemOrReg (MainOpcode, SecondOpCode);
end;

```

### Code Block 12

```pascal
Procedure TExpression.GenerateInstruction_MOV_Memory_Immediate (ImmediateValue: Word);
begin
  GenerateInstruction_8_16_bit (MOV_RegisterOrMemory_Immediate, modMemory);
  If it16Bit in DataType then GenerateInstruction_Word (ImmediateValue) else GenerateInstruction_Byte (Lo (ImmediateValue));
end;
```

### Code Block 13

```pascal
Procedure GenerateInstruction_CALL_FAR (SysProc: Word);
begin
  If SysProc and $8000 <> 0 then MarkUnitForInstructions80x87;
  ES_DI_PointerDestroyed;
  GenerateInstruction_Byte (CALL_FAR);
  GenerateReference (SystemUnitSegment, SysProc shl 1, 0, [rfSegment, rfOffset]);
end;
```

### Code Block 14

```pascal
Procedure GenerateInstructionForStackFrameDisplacement (MainOpCode, SecondOpCode: Byte; Displacement: Integer);
begin
  If (SecondOpCode and rmMask <> rmSS_BP_Displacement) and (Displacement = 0) then
    GenerateInstruction_TwoBytes (MainOpCode, SecondOpCode) else
      begin
        If Displacement = ShortInt (Displacement) then
          begin
            GenerateInstruction_TwoBytes (MainOpCode, SecondOpCode or modMemoryDisp8);
            GenerateInstruction_Byte (Displacement);
          end else begin
                     GenerateInstruction_TwoBytes (MainOpCode, SecondOpCode or modMemoryDisp16);
                     GenerateInstruction_Word (Displacement);
                   end;
      end;
end;
```

### Code Block 15

```pascal
Procedure GenerateInstruction_PUSH_Immediate (PushWord: Integer);
begin
  If PushWord = ShortInt (Lo (PushWord)) then GenerateInstruction_TwoBytes (PUSH_SignedByte, Lo (PushWord))
    else begin
           GenerateInstruction_Byte (PUSH_Word);
           GenerateInstruction_Word (PushWord);
         end;
end;
```

### Code Block 16

```pascal
Procedure GenerateArithmeticInstruction_16BitReg_Immediate (SecondOpCode: Byte; Value: Integer);
begin
  If Value = ShortInt (Value) then
    begin
      GenerateInstruction_TwoBytes (ArithmeticOperation or SignExtendImmediateByte or OperandMode16Bit, SecondOpCode);
      GenerateInstruction_Byte (Lo (Value));
    end else begin
               GenerateInstruction_TwoBytes (ArithmeticOperation or OperandMode16Bit, SecondOpCode);
               GenerateInstruction_Word (Value);
             end;

end;
```

### Code Block 17

```pascal
Procedure GenerateInstruction_Byte (Code: Byte);
begin
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 2))^ do
    begin
      Recordtype := icByte;
      ByteCode := Code;
    end;
end;

Procedure GenerateInstruction_TwoBytes (B0, B1: Byte);
begin
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 3))^ do
    begin
      Recordtype := icWord;
      WordCode.ByteL := B0;
      WordCode.ByteH := B1;
    end;
end;

Procedure GenerateInstruction_Word (Code: Word);
begin
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 3))^ do
    begin
      Recordtype := icWord;
      WordCode.Word := Code;
    end;
end;
```

### Code Block 18

```pascal
Procedure TExpression.GenerateSegmentOverride;
begin
  If LocationData.Flags * [ofsBP, segDS] = [] then
    GenerateInstruction_Byte (SegmentOverride or ExpressionSegment);
end;

Procedure TExpression.GenerateInstructionWithSegment (OpCode: Byte);
begin
  GenerateInstruction_Byte (OpCode);
  If efRelocatable in LocationData.Flags then GenerateReference (Value.Segment, Value.BlockRecord, 0, [rfSegment])
    else GenerateInstruction_Word (Value.VarSegment);
end;
```

### Code Block 19

```pascal
Procedure TExpression.GenerateOffsetReferenceForExpressionInMemory;
Var Flags: TReferenceFlagSet;
begin
  If LocationData.Flags * [efTypedConstant, segCS, segDS] = [] then GenerateInstruction_Word (Value.Offset) else
    begin
      Case segDS in LocationData.Flags of
        True: If efTypedConstant in LocationData.Flags then
                Flags := [rfDataSegment, rfConstant, rfOffset] else Flags := [rfDataSegment, rfOffset];
        else  If segCS in LocationData.Flags then Flags := [rfConstant, rfOffset] else Flags := [rfOffset];
      end;
      GenerateReference (Value.Segment, Value.BlockRecord, Value.Offset, Flags);
    end;
end;
```

### Code Block 20

```pascal
Procedure TExpression.GenerateInstructionWithMemoryReference (MainOpCode, MODEM_OpCode: Byte);
begin
  Case ofsBP in LocationData.Flags of
    True: begin
            If ofsDI in LocationData.Flags then MODEM_OpCode := MODEM_OpCode xor rmSS_BP_DI_Displacement
              else MODEM_OpCode := MODEM_OpCode xor rmSS_BP_Displacement;
          end;
    else Case ofsDI in LocationData.Flags of
           True: begin
                   MODEM_OpCode := MODEM_OpCode xor rmDS_DI;
                   If LocationData.Flags * [efTypedConstant, segCS, segDS] <> [] then
                     begin
                       GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode or modMemoryDisp16);
                       GenerateOffsetReferenceForExpressionInMemory;
                       Exit;
                     end;
                   If Value.Word = 0 then
                     begin
                       GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode);
                       Exit;
                     end;
                 end;
           else begin
                   GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode or rmDS_Displacement16);
                   GenerateOffsetReferenceForExpressionInMemory;
                   Exit;
                 end;
         end;
  end;
  If Value.Integer = Value.ShortInt then
    begin
      GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode or modMemoryDisp8);
      GenerateInstruction_Byte (Value.Byte);
    end else begin
               GenerateInstruction_TwoBytes (MainOpCode, MODEM_OpCode or modMemoryDisp16);
               GenerateOffsetReferenceForExpressionInMemory;
             end;
end;
```

### Code Block 21

```pascal
Procedure TExpression.GenerateInstructionWithExpressionInMemOrReg (MainOpCode, MODEM_OpCode: Byte);
begin
  Case Location of
    elRegister: GenerateInstruction_TwoBytes (MainOpCode, modRegister or MODEM_OpCode or LocationData.Register)
    else begin
           GenerateSegmentOverride;
           GenerateInstructionWithMemoryReference (MainOpCode, MODEM_OpCode);
         end;
  end;
end;
```

### Code Block 22

```pascal
Procedure TExpression.GenerateArithmeticInstructionWith_ACC (OpCode: Byte);
begin
  If Location = elConstant then
    begin
      If it16Bit in DataType then
        begin
          OpCode := OpCode or ArithmeticOp_AX_Immediate;
          GenerateInstruction_Byte (OpCode);
          GenerateInstruction_Word (Value.Word);
        end else GenerateInstruction_TwoBytes (OpCode or ArithmeticOp_AL_Immediate, Value.Byte);
    end else GenerateInstruction_8_16_bit (OpCode or reg__mod_rm, rAX);
end;
```

### Code Block 23

```pascal
Procedure TExpression.GenerateArithmeticInstructionWith_DX (OpCode: Byte);
begin
  If Location = elConstant then
    GenerateArithmeticInstruction_16BitReg_Immediate (modRegister or OpCode or rDX, Value.Word) else
      GenerateInstructionWithExpressionInMemOrReg (OpCode or reg__mod_rm or OperandMode16Bit, rDX shl 3);
end;
```

### Code Block 24

```pascal
Procedure TExpression.GenerateArithmeticInstructionWithImmediateValue (ImmediateValue: Integer; SecondOpCode: Byte);
begin
  If it16Bit in DataType then
    begin
      If ImmediateValue = ShortInt (ImmediateValue) then
        begin
          GenerateInstructionWithExpressionInMemOrReg (ArithmeticOperation or SignExtendImmediateByte or
                                                                                 OperandMode16Bit, SecondOpCode);
          GenerateInstruction_Byte (ImmediateValue);
        end else begin
                   GenerateInstructionWithExpressionInMemOrReg (ArithmeticOperation or OperandMode16Bit, SecondOpCode);
                   GenerateInstruction_Word (ImmediateValue);
                 end;
    end else begin
               GenerateInstructionWithExpressionInMemOrReg (ArithmeticOperation, SecondOpCode);
               GenerateInstruction_Byte (ImmediateValue);
             end;
end;
```

### Code Block 25

```pascal
Procedure TExpression.GenerateInstruction_MOV_Memory_Register (Register: Byte);
begin
  GenerateInstruction_16bit_MOV_Memory_Register (Register);
  If it32Bit in DataType then
    begin
      Inc (Value.Offset, 2);
      GenerateInstruction_16bit_MOV_Memory_Register (Register + 2);
      Dec (Value.Offset, 2);
    end;
end;
```

### Code Block 26

```pascal
Procedure TExpression.GenerateInstruction_16bit_MOV_Memory_Register (Register: Byte);
begin
  If (Register = rAX) and (LocationData.Flags * [ofsDI, ofsBP] = []) then
    begin
      GenerateSegmentOverride;
      If it16Bit in DataType then GenerateInstruction_Byte (MOV_Mem_AX)
        else GenerateInstruction_Byte (MOV_Mem_AL);
      GenerateOffsetReferenceForExpressionInMemory;
    end else GenerateInstruction_8_16_bit (MOV_Operations, Register shl 3);
end;
```

### Code Block 27

```pascal
Procedure TExpression.GenerateInstruction_8_16_bit (MainOpcode, SecondOpCode: Byte);
begin
  If it16Bit in DataType then MainOpcode := MainOpcode or OperandMode16Bit;
  GenerateInstructionWithExpressionInMemOrReg (MainOpcode, SecondOpCode);
end;

```

### Code Block 28

```pascal
Procedure TExpression.GenerateInstruction_MOV_Memory_Immediate (ImmediateValue: Word);
begin
  GenerateInstruction_8_16_bit (MOV_RegisterOrMemory_Immediate, modMemory);
  If it16Bit in DataType then GenerateInstruction_Word (ImmediateValue) else GenerateInstruction_Byte (Lo (ImmediateValue));
end;
```

### Code Block 29

```pascal
Procedure GenerateInstruction_CALL_FAR (SysProc: Word);
begin
  If SysProc and $8000 <> 0 then MarkUnitForInstructions80x87;
  ES_DI_PointerDestroyed;
  GenerateInstruction_Byte (CALL_FAR);
  GenerateReference (SystemUnitSegment, SysProc shl 1, 0, [rfSegment, rfOffset]);
end;
```

### Code Block 30

```pascal
Procedure GenerateInstructionForStackFrameDisplacement (MainOpCode, SecondOpCode: Byte; Displacement: Integer);
begin
  If (SecondOpCode and rmMask <> rmSS_BP_Displacement) and (Displacement = 0) then
    GenerateInstruction_TwoBytes (MainOpCode, SecondOpCode) else
      begin
        If Displacement = ShortInt (Displacement) then
          begin
            GenerateInstruction_TwoBytes (MainOpCode, SecondOpCode or modMemoryDisp8);
            GenerateInstruction_Byte (Displacement);
          end else begin
                     GenerateInstruction_TwoBytes (MainOpCode, SecondOpCode or modMemoryDisp16);
                     GenerateInstruction_Word (Displacement);
                   end;
      end;
end;
```

### Code Block 31

```pascal
Procedure GenerateInstruction_PUSH_Immediate (PushWord: Integer);
begin
  If PushWord = ShortInt (Lo (PushWord)) then GenerateInstruction_TwoBytes (PUSH_SignedByte, Lo (PushWord))
    else begin
           GenerateInstruction_Byte (PUSH_Word);
           GenerateInstruction_Word (PushWord);
         end;
end;
```

### Code Block 32

```pascal
Procedure GenerateArithmeticInstruction_16BitReg_Immediate (SecondOpCode: Byte; Value: Integer);
begin
  If Value = ShortInt (Value) then
    begin
      GenerateInstruction_TwoBytes (ArithmeticOperation or SignExtendImmediateByte or OperandMode16Bit, SecondOpCode);
      GenerateInstruction_Byte (Lo (Value));
    end else begin
               GenerateInstruction_TwoBytes (ArithmeticOperation or OperandMode16Bit, SecondOpCode);
               GenerateInstruction_Word (Value);
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
- https://turbopascal.org/generating-code
- https://turbopascal.org/generating-instructions
- https://turbopascal.org/intermediate-code
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/linking-modules-and-creating-exe-file
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/system-functions
- https://turbopascal.org/system-procedures
- https://turbopascal.org/system-unit
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

