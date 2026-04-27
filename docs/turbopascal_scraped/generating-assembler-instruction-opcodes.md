# Generating Assembler Instruction Opcodes - Turbo Pascal Compiler

**URL:** https://turbopascal.org/generating-assembler-instruction-opcodes

---

## Table of Contents

- Generating Assembler Instruction Opcodes

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Generating Assembler Instruction Opcodes
Assembler instructions (in fact
mnemonics
) are stored in a
symbol table
where each instruction has few data bytes which define which procedure will process it and what combinations of parameters are possible.
Type PInstructionData = ^TInstructionData;
     TInstructionData = Record
                          B0,       { x654321x  = InstructionProc shl 1 }

                          B1: Byte; { 76xx xxxx = Number of parameters
                                    { xxxx x2xx = Instruction 80286 }
                                    { xxxx xx10 = Number of additinal data bytes - 1 }

                          Data: Array [0..7] of Byte;
                        end;
This procedure iterates over all possible parameter combinations for current instruction and if right combination of parameters is found, the associated procedure is called.
Procedure GenerateInstructionOpcodes;
begin
  Repeat
    SizeOfAsmIntermediateCodeRecords := 0;
    LastOpCodeBeforeDataValid := False;
    Direction := 0;
    SegmentOverrideOpCode := 0;
    If InstructionData^.B1 shr 6 = ExpressionCounter then
      begin
        LeftExpression  := @AsmExpression1;
        RightExpression := @AsmExpression2;
        LeftToken := LeftExpression^.Token;
        RightToken := RightExpression^.Token;
        InstructionProcessed := False;
        InstructionProc [(InstructionData^.B0 and $7E) shr 1];
        If InstructionProcessed then Break;
      end;
    If InstructionData^.B0 and $80 = 0 then                 { Last variant }
      begin
        If ExpressionCounter < 2 then
          AsmSourceErrorPosition := ErrorPositionInstructionExpresion1 else
            AsmSourceErrorPosition := ErrorPositionInstructionExpresion2;
        AsmError (InvalidCombinationOfOpcodeAndOperands);
      end;
    Inc (PChar (InstructionData), NumberOfOpCodesBeforeModem + 2);
  until False;
  If (InstructionData^.B1 and $04 <> 0) and (Flags and $03 = 0) then
    begin
      AsmSourceErrorPosition := ErrorPositionInstructionExpresion1;
      AsmError (_286_287_InstructionsAreNotEnabled);
    end;
  GenerateLastOpCodeBeforeData;
<span>end;</span>
This table contains opcode generation procedures for possible instruction types.
Const InstructionProc: Array [0..53] of Procedure = (
 { 00 } Proc_NoParameters,
 { 01 } Proc_ImmediateWord,
 { 02 } Proc_ImmediateSignedByte,
 { 03 } Proc_16BitReg_RegMem,
 { 04 } Proc_DX_ACC,
 { 05 } Proc_ImmediateByte_ACC,
 { 06 } Proc_ACC_DX,
 { 07 } Proc_ACC_ImmediateByte,
 { 08 } Proc_16BitReg_AX,
 { 09 } Proc_AX_16BitReg,
 { 0A } Proc_ImmediateByteWithSpecialFor_3,
 { 0B } Proc_ImmediateByte_ImmediateWord,
 { 0C } Proc_ACC_Immediate,
 { 0D } Proc_Reg_Immediate,
 { 0E } Proc_NearIndirect,
 { 0F } Proc_FarIdentifierReference,
 { 10 } Proc_FarIndirect,
 { 11 } Proc_JumpLabel,
 { 12 } Proc_DisplacementIdentifierReference,
 { 13 } Proc_MemAddress_ACC,
 { 14 } Proc_ACC_MemAddress,
 { 15 } Proc_ImmediateWithSpecialFor_0_OrNoParameter,
 { 16 } Proc_ImmediateWithSpecialFor_0,
 { 17 } Proc_SyntaxError,
 { 18 } Proc_SegmentRegister,
 { 19 } Proc_SegmentRegisterExcept_CS,
 { 1A } Proc_16BitReg,
 { 1B } Proc_DescriptorTable,
 { 1C } Proc_16BitReg_16BitRegMem,
 { 1D } Proc_16BitRegMem_16BitReg,
 { 1E } Proc_Reg_RegMem_Immediate,
 { 1F } Proc_SegmentReg_RegMem,
 { 20 } Proc_RegMem_SegmentReg,
 { 21 } Proc_ByteOrWord_RegMem_Reg,
 { 22 } Proc_ByteOrWord_RegMem_Reg_Or_Reg_RegMem,
 { 23 } Proc_16bit_RegisterOrMemory,
 { 24 } Proc_RegMem_ImmediateSignedByteOrWord,
 { 25 } Proc_RegMem__Immediate,
 { 26 } Proc_UnaryOperations,
 { 27 } Proc_16BitReg_PointerMemoryReference,
 { 28 } Proc_ImmediateWithSpecialFor_1_Or_CL,
 { 29 } Proc_FPU_Instruction,
 { 2A } Proc_AX,
 { 2B } Proc_STi,
 { 2C } Proc_STi_ST,
 { 2D } Proc_ST_STi,
 { 2E } Proc_real32_or_real64,
 { 2F } Proc_real80,
 { 30 } Proc_int16,
 { 31 } Proc_int16_or_int32,
 { 32 } Proc_int64,
 { 33 } Proc_MemoryReferenceSize_14,
 { 34 } Proc_MemoryReferenceSize_94,
 { 35 } Proc_SyntaxError);
These opcode generation procedures actually generate opcodes for assembler instructions.
Procedure Proc_NoParameters; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  InstructionProcessed := True;
end;

Procedure Proc_ImmediateWord; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  SwapLeftAndRightExpression;
  InstructionProcessed := GenerateImmediateWord;
end;

Procedure Proc_ImmediateSignedByte; Far;
begin
  SwapLeftAndRightExpression;
  GenerateOpCodesExceptLastBeforeData;
  If not RightExpressionSignedByteConstant then Exit;
  GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte);
  InstructionProcessed := True;
end;

Procedure Proc_16BitReg_RegMem; Far;
begin
  If InstructionTokens <> Register_MemoryReference then Exit;
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  SwapLeftAndRightExpression;
  If not LeftExpressionRegMem then Exit;
  SwapLeftAndRightExpression;
  SegmentOverrideOpCode := 0;
  GenerateOpCodesExceptLastBeforeData;
  ModemOpcode := ModemOpcode or LeftExpression^.Register shl 3;
  InstructionProcessed := ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
end;

Procedure Proc_ACC_DX; Far;
begin
  With RightExpression^ do
    begin
      If Token <> AsmToken_Register then Exit;
      If RegisterType <> rt16BitRegister then Exit;
      If Register <> rDX then Exit;
    end;
  InstructionProcessed := IsLeftExpression_ACC;
end;

Procedure Proc_DX_ACC; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_ACC_DX;
end;

Procedure Proc_ACC_ImmediateByte; Far;
begin
  If not IsLeftExpression_ACC then Exit;
  InstructionProcessed := GenerateImmediateByte;
end;

Procedure Proc_ImmediateByte_ACC; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_ACC_ImmediateByte;
end;

Procedure Proc_AX_16BitReg; Far;
begin
  If InstructionTokens <> Register_Register then Exit;
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  If LeftExpression^.Register <> rAX then Exit;
  If RightExpression^.RegisterType <> rt16BitRegister then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or RightExpression^.Register;
  InstructionProcessed := True;
end;

Procedure Proc_16BitReg_AX; Far;
begin
  SwapLeftAndRightExpression;
  Proc_AX_16BitReg;
end;

Procedure Proc_ImmediateByteWithSpecialFor_3; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  RightExpression := LeftExpression;
  If not RightExpressionNumericConstantAndSize8Bit then Exit;
  If LeftExpression^.Value.Byte <> $03 then
    begin
      Inc (LastOpCodeBeforeModem);
      GenerateLastOpCodeBeforeDataAnd_acByte (LeftExpression^.Value.Byte);
    end;
  InstructionProcessed := True;
end;

Procedure Proc_ImmediateByte_ImmediateWord; Far;
begin
  SwapLeftAndRightExpression;
  GenerateOpCodesExceptLastBeforeData;
  If not GenerateImmediateWord then Exit;
  SwapLeftAndRightExpression;
  InstructionProcessed := GenerateImmediateByte;
end;

Procedure Proc_ACC_Immediate; Far;
begin
  If InstructionTokens <> Register_Constant then Exit;
  If not RightExpression16BitNumericConstant then Exit;
  With LeftExpression^ do
    begin
      If (Register <> rACC) or (RegisterType > rt16BitRegister) then Exit;
      Case RegisterType of
        rt8BitRegister: begin
                          If not RightExpressionNumericConstantAndSize8Bit then Exit;
                          GenerateOpCodesExceptLastBeforeData;
                          InstructionProcessed := GenerateImmediateByte;
                        end;
        rt16BitRegister: begin
                           GenerateOpCodesExceptLastBeforeData;
                           Inc (LastOpCodeBeforeModem);
                           InstructionProcessed := GenerateImmediateWord;
                         end;
      end;
    end;
end;

Procedure Proc_Reg_Immediate; Far;
begin
  With LeftExpression^ do
    begin
      If InstructionTokens <> Register_Constant then Exit;
      If RegisterType > rt16BitRegister then Exit;
      GenerateOpCodesExceptLastBeforeData;
      LastOpCodeBeforeModem := LastOpCodeBeforeModem or Register;
      Case RegisterType of
        rt8BitRegister: InstructionProcessed := GenerateImmediateByte;
        rt16BitRegister: begin
                           LastOpCodeBeforeModem := LastOpCodeBeforeModem or $08;
                           InstructionProcessed := GenerateImmediateWord;
                         end;
      end;
    end;
end;

Procedure Proc_NearIndirect; Far;
begin
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> Ord (rt16BitRegister) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LeftExpressionTypeSize := 0;
  If not GenerateRestOf_RegMem__Register_Instruction then Exit;
  InstructionProcessed := True;
end;

Procedure Proc_FarIdentifierReference; Far;
begin
  GenerateInstructionWithIdentifierReference (3, $FFFE);
end;

Procedure Proc_FarIndirect; Far;
begin
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> $FF then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_JumpLabel; Far;
begin
  With LeftExpression^ do
    begin
      If Token <> AsmToken_MemoryReference then Exit;
      If TypeSize <> $FFFD then Exit;
      If MemoryReferences <> [mrMemoryLocation] then Exit;
      If Value.LongInt <> 0 then Exit;
      GeneratePrefixOpCodes;
      CurrentRelocatableIdentifier.Long := RelocatableIdentifier;
      AddAsmIntermediateCodeRecord (acJumpNear, 0, InstructionData^.Data [0], Byte (AsmToken));
    end;
  InstructionProcessed := True;
end;

Procedure Proc_DisplacementIdentifierReference; Far;
begin
  If LeftExpression^.TypeSize = $FFFD then
    GenerateInstructionWithIdentifierReference (0, $FFFD) else
      GenerateInstructionWithIdentifierReference (0, $FFFF);
end;

Procedure Proc_MemAddress_ACC; Far;
begin
  If InstructionTokens <> MemoryReference_Register then Exit;
  If RightExpression^.Register <> rACC then Exit;
  If not LeftExpressionRegMem then Exit;
  If RegisterInMemoryReference then Exit;
  GenerateOpCodesExceptLastBeforeData;
  If RightExpression^.RegisterType > rt16BitRegister then Exit;
  If Ord (RightExpression^.RegisterType) <> LeftExpressionTypeSize then Exit;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or Ord (RightExpression^.RegisterType);
  Generate16BitDisplacement;
  InstructionProcessed := True;
end;

Procedure Proc_ACC_MemAddress; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_MemAddress_ACC;
end;

Procedure Proc_ImmediateWithSpecialFor_0_OrNoParameter; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  If Flags and $0100 <> 0 then LastOpCodeBeforeModem := LastOpCodeBeforeModem or $08;
  InstructionProcessed := True;
  If ExpressionCounter = 0 then Exit;
  RightExpression := LeftExpression;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or $01;
  If RightExpressionWordConstant_0 then Exit;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem and $FE;
  InstructionProcessed := GenerateImmediateWord;
end;

Procedure Proc_ImmediateWithSpecialFor_0; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  RightExpression := LeftExpression;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or $01;
  InstructionProcessed := True;
  If RightExpressionWordConstant_0 then Exit;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem and $FE;
  InstructionProcessed := GenerateImmediateWord;
end;

Procedure Proc_SyntaxError; Far;
begin
  AsmError (SyntaxError);
end;

Procedure Proc_SegmentRegister; Far;
begin
  InstructionWithSegmentRegister ($FF);
end;

Procedure Proc_SegmentRegisterExcept_CS; Far;
begin
  InstructionWithSegmentRegister (1);
end;

Procedure Proc_16BitReg; Far;
begin
  If LeftToken <> AsmToken_Register then Exit;
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or LeftExpression^.Register;
  InstructionProcessed := True;
end;

Procedure Proc_DescriptorTable; Far;
begin
  If not LeftExpressionRegMem then Exit;
  With LeftExpression^ do If (TypeSize <> 6) and (TypeSize <> 0) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_16BitRegMem_16BitReg; Far;
begin
  If RightToken <> AsmToken_Register then Exit;
  If RightExpression^.RegisterType <> rt16BitRegister then Exit;
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> Ord (rt16BitRegister) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  InstructionProcessed := ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
end;

Procedure Proc_16BitReg_16BitRegMem; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_16BitRegMem_16BitReg
end;

Procedure Proc_Reg_RegMem_Immediate; Far;
Var IsByte: Boolean;
begin
  If LeftToken <> AsmToken_Register then
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  SwapLeftAndRightExpression;
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> Ord (rt16BitRegister) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  RightExpression := @AsmExpression3;
  If not RightExpression16BitNumericConstant then Exit;
  IsByte := RightExpressionSignedByteConstant;
  If IsByte then LastOpCodeBeforeModem := LastOpCodeBeforeModem or $02;
  ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
  If IsByte then GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte) else
    begin
      GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte);
      GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte1);
    end;
  InstructionProcessed := True;
end;

Procedure Proc_RegMem_SegmentReg; Far;
begin
  If RightExpression^.Token <> AsmToken_Register then Exit;
  If RightExpression^.RegisterType <> rtSegmentRegister then Exit;
  If not LeftExpressionRegMem then Exit;
  If (LeftExpressionTypeSize <> 1) and (RightExpression^.TypeSize <> 0) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  InstructionProcessed := ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
end;

Procedure Proc_SegmentReg_RegMem; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  If RightExpression^.Register <> rCX then Proc_RegMem_SegmentReg;
end;

Procedure Proc_ByteOrWord_RegMem_Reg; Far;
begin
  If RightToken <> AsmToken_Register then
    begin
      If LeftToken <> AsmToken_Register then Exit;
      Direction := 2;
      SwapLeftAndRightExpression;
    end;
  If RightExpression^.RegisterType > rt16BitRegister then Exit;
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  If Byte (RightExpression^.RegisterType) = LeftExpressionTypeSize then InstructionProcessed := GenerateRestOfInstruction;
end;

Procedure Proc_ByteOrWord_RegMem_Reg_Or_Reg_RegMem; Far;

  Function ByteOrWord_RegMem_Reg: Boolean;
  begin
    ByteOrWord_RegMem_Reg := False;
    If RightToken <> AsmToken_Register then Exit;
    If RightExpression^.RegisterType > rt16BitRegister then Exit;
    If not LeftExpressionByteOrWordRegMem then Exit;
    GenerateOpCodesExceptLastBeforeData;
    AddRegisterToModemOpcode;
    If Byte (RightExpression^.RegisterType) <> LeftExpressionTypeSize then Exit;
    ByteOrWord_RegMem_Reg := GenerateRestOfInstruction;
  end;

begin
  InstructionProcessed := True;
  If ByteOrWord_RegMem_Reg then Exit;
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  InstructionProcessed := ByteOrWord_RegMem_Reg;
end;

Procedure Proc_16bit_RegisterOrMemory; Far;
begin
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> Ord (rt16BitRegister) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_RegMem_ImmediateSignedByteOrWord; Far;
begin
  If not RightExpression16BitNumericConstant then Exit;
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  If LeftExpressionTypeSize <> 0 then
    begin
      If not RightExpressionSignedByteConstant then
        begin
          If not GenerateRestOf_RegMem__Register_Instruction then Exit;
          InstructionProcessed := GenerateImmediateWord;
          Exit;
        end;
      LastOpCodeBeforeModem := LastOpCodeBeforeModem or $02;
    end;
  If not RightExpressionNumericConstantAndSize8Bit then Exit;
  If not GenerateRestOf_RegMem__Register_Instruction then Exit;
  InstructionProcessed := GenerateImmediateByte;
end;

Procedure Proc_RegMem__Immediate; Far;
begin
  If RightToken <> AsmToken_Constant then Exit;
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  GenerateRestOf_RegMem__Register_Instruction;
  Case LeftExpressionTypeSize of
    1: InstructionProcessed := GenerateImmediateWord;
    else InstructionProcessed := GenerateImmediateByte;
  end;
end;

Procedure Proc_UnaryOperations; Far;
begin
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  If not GenerateRestOf_RegMem__Register_Instruction then Exit;
  InstructionProcessed := True;
end;

Procedure Proc_16BitReg_PointerMemoryReference; Far;
begin
  If InstructionTokens <> Register_MemoryReference then Exit;
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  SwapLeftAndRightExpression;
  LeftExpressionRegMem;
  If (SavedRightExpressionTypeSize <> 0) and (LeftExpressionTypeSize <> $FF) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  InstructionProcessed := ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
end;

Procedure Proc_ImmediateWithSpecialFor_1_Or_CL; Far;
begin
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  If RightExpressionNumericConstantAndSize8Bit then
    begin
      If RightExpression^.Value.Byte <> 1 then
        begin
          If Flags and $0003 = 0 then AsmError (_286_287_InstructionsAreNotEnabled);
          Dec (LastOpCodeBeforeModem, $10);
          GenerateRestOf_RegMem__Register_Instruction;
          GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte);
          InstructionProcessed := True;
        end else InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
    end else begin
               If RightExpression^.Token <> AsmToken_Register then Exit;
               If RightExpression^.RegisterType <> rt8BitRegister then Exit;
               If RightExpression^.Register <> rCL then Exit;
               Inc (LastOpCodeBeforeModem, 2);
               InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
             end;
end;

Procedure Proc_FPU_Instruction;
begin
  ModifyAndGenerate_FPU_Instruction (0)
end;

Procedure Proc_AX; Far;
begin
  If LeftToken <> AsmToken_Register then Exit;
  With Leftexpression^ do
    begin
      If RegisterType <> rt16BitRegister then Exit;
      If Register <> rAX then Exit;
    end;
  ModifyAndGenerate_FPU_Instruction (0)
end;

Procedure Process_FPU_Instruction__STi;
begin
  If InstructionTokens <> Register_Register then Exit;
  If LeftExpression^.RegisterType <> rtFPURegister then Exit;
  Proc_FPU_Instruction;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or LeftExpression^.Register;
  InstructionProcessed := True;
end;

Procedure Proc_STi; Far;
begin
  RightToken := AsmToken_Register;
  Process_FPU_Instruction__STi;
end;

Procedure Proc_STi_ST; Far;
begin
  With RightExpression^ do If (RegisterType <> rtFPURegister) or (Register <> 0) then Exit;
  Process_FPU_Instruction__STi;
end;

Procedure Proc_ST_STi; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_STi_ST;
end;

Procedure FPU_Instruction_Size1_or_Size2 (Size: Word);
Var RealSizeModifier: Byte;
begin
  If LeftToken <> AsmToken_MemoryReference then Exit;
  If not LeftExpressionRegMem then Exit;
  RealSizeModifier := 0;
  If LeftExpressionTypeSize <> Hi (Size) then
    begin
      If LeftExpressionTypeSize <> Lo (Size) then Exit;
      RealSizeModifier := $04;
    end;
  ModifyAndGenerate_FPU_Instruction (RealSizeModifier);
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_real32_or_real64; Far;
begin
  FPU_Instruction_Size1_or_Size2 ($FFFE);
end;

Procedure Proc_real80; Far;
begin
  FPU_Instruction_Size1_or_Size2 ($FDFD);
end;

Procedure Proc_int16; Far;
begin
  With LeftExpression^ do
    If TypeSize = 0 then TypeSize := 2;
  FPU_Instruction_Size1_or_Size2 ($0101);
end;

Procedure Proc_int16_or_int32; Far;
begin
  FPU_Instruction_Size1_or_Size2 ($FF01);
end;

Procedure Proc_int64; Far;
begin
  FPU_Instruction_Size1_or_Size2 ($FEFE);
end;

Procedure FPU_Instruction__MemoryReferenceSize (Size: Word);
begin
  With LeftExpression^ do If (TypeSize <> Size) and (TypeSize <> 0) then Exit;
  If LeftToken <> AsmToken_MemoryReference then Exit;
  If not LeftExpressionRegMem then Exit;
  Proc_FPU_Instruction;
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_MemoryReferenceSize_14; Far;
begin
  FPU_Instruction__MemoryReferenceSize ($000E);
end;

Procedure Proc_MemoryReferenceSize_94; Far;
begin
  FPU_Instruction__MemoryReferenceSize ($005E);
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
Type PInstructionData = ^TInstructionData;
     TInstructionData = Record
                          B0,       { x654321x  = InstructionProc shl 1 }

                          B1: Byte; { 76xx xxxx = Number of parameters
                                    { xxxx x2xx = Instruction 80286 }
                                    { xxxx xx10 = Number of additinal data bytes - 1 }

                          Data: Array [0..7] of Byte;
                        end;
```

### Code Block 2

```pascal
Procedure GenerateInstructionOpcodes;
begin
  Repeat
    SizeOfAsmIntermediateCodeRecords := 0;
    LastOpCodeBeforeDataValid := False;
    Direction := 0;
    SegmentOverrideOpCode := 0;
    If InstructionData^.B1 shr 6 = ExpressionCounter then
      begin
        LeftExpression  := @AsmExpression1;
        RightExpression := @AsmExpression2;
        LeftToken := LeftExpression^.Token;
        RightToken := RightExpression^.Token;
        InstructionProcessed := False;
        InstructionProc [(InstructionData^.B0 and $7E) shr 1];
        If InstructionProcessed then Break;
      end;
    If InstructionData^.B0 and $80 = 0 then                 { Last variant }
      begin
        If ExpressionCounter < 2 then
          AsmSourceErrorPosition := ErrorPositionInstructionExpresion1 else
            AsmSourceErrorPosition := ErrorPositionInstructionExpresion2;
        AsmError (InvalidCombinationOfOpcodeAndOperands);
      end;
    Inc (PChar (InstructionData), NumberOfOpCodesBeforeModem + 2);
  until False;
  If (InstructionData^.B1 and $04 <> 0) and (Flags and $03 = 0) then
    begin
      AsmSourceErrorPosition := ErrorPositionInstructionExpresion1;
      AsmError (_286_287_InstructionsAreNotEnabled);
    end;
  GenerateLastOpCodeBeforeData;
<span>end;</span>
```

### Code Block 3

```pascal
Const InstructionProc: Array [0..53] of Procedure = (
 { 00 } Proc_NoParameters,
 { 01 } Proc_ImmediateWord,
 { 02 } Proc_ImmediateSignedByte,
 { 03 } Proc_16BitReg_RegMem,
 { 04 } Proc_DX_ACC,
 { 05 } Proc_ImmediateByte_ACC,
 { 06 } Proc_ACC_DX,
 { 07 } Proc_ACC_ImmediateByte,
 { 08 } Proc_16BitReg_AX,
 { 09 } Proc_AX_16BitReg,
 { 0A } Proc_ImmediateByteWithSpecialFor_3,
 { 0B } Proc_ImmediateByte_ImmediateWord,
 { 0C } Proc_ACC_Immediate,
 { 0D } Proc_Reg_Immediate,
 { 0E } Proc_NearIndirect,
 { 0F } Proc_FarIdentifierReference,
 { 10 } Proc_FarIndirect,
 { 11 } Proc_JumpLabel,
 { 12 } Proc_DisplacementIdentifierReference,
 { 13 } Proc_MemAddress_ACC,
 { 14 } Proc_ACC_MemAddress,
 { 15 } Proc_ImmediateWithSpecialFor_0_OrNoParameter,
 { 16 } Proc_ImmediateWithSpecialFor_0,
 { 17 } Proc_SyntaxError,
 { 18 } Proc_SegmentRegister,
 { 19 } Proc_SegmentRegisterExcept_CS,
 { 1A } Proc_16BitReg,
 { 1B } Proc_DescriptorTable,
 { 1C } Proc_16BitReg_16BitRegMem,
 { 1D } Proc_16BitRegMem_16BitReg,
 { 1E } Proc_Reg_RegMem_Immediate,
 { 1F } Proc_SegmentReg_RegMem,
 { 20 } Proc_RegMem_SegmentReg,
 { 21 } Proc_ByteOrWord_RegMem_Reg,
 { 22 } Proc_ByteOrWord_RegMem_Reg_Or_Reg_RegMem,
 { 23 } Proc_16bit_RegisterOrMemory,
 { 24 } Proc_RegMem_ImmediateSignedByteOrWord,
 { 25 } Proc_RegMem__Immediate,
 { 26 } Proc_UnaryOperations,
 { 27 } Proc_16BitReg_PointerMemoryReference,
 { 28 } Proc_ImmediateWithSpecialFor_1_Or_CL,
 { 29 } Proc_FPU_Instruction,
 { 2A } Proc_AX,
 { 2B } Proc_STi,
 { 2C } Proc_STi_ST,
 { 2D } Proc_ST_STi,
 { 2E } Proc_real32_or_real64,
 { 2F } Proc_real80,
 { 30 } Proc_int16,
 { 31 } Proc_int16_or_int32,
 { 32 } Proc_int64,
 { 33 } Proc_MemoryReferenceSize_14,
 { 34 } Proc_MemoryReferenceSize_94,
 { 35 } Proc_SyntaxError);
```

### Code Block 4

```pascal
Procedure Proc_NoParameters; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  InstructionProcessed := True;
end;

Procedure Proc_ImmediateWord; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  SwapLeftAndRightExpression;
  InstructionProcessed := GenerateImmediateWord;
end;

Procedure Proc_ImmediateSignedByte; Far;
begin
  SwapLeftAndRightExpression;
  GenerateOpCodesExceptLastBeforeData;
  If not RightExpressionSignedByteConstant then Exit;
  GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte);
  InstructionProcessed := True;
end;

Procedure Proc_16BitReg_RegMem; Far;
begin
  If InstructionTokens <> Register_MemoryReference then Exit;
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  SwapLeftAndRightExpression;
  If not LeftExpressionRegMem then Exit;
  SwapLeftAndRightExpression;
  SegmentOverrideOpCode := 0;
  GenerateOpCodesExceptLastBeforeData;
  ModemOpcode := ModemOpcode or LeftExpression^.Register shl 3;
  InstructionProcessed := ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
end;

Procedure Proc_ACC_DX; Far;
begin
  With RightExpression^ do
    begin
      If Token <> AsmToken_Register then Exit;
      If RegisterType <> rt16BitRegister then Exit;
      If Register <> rDX then Exit;
    end;
  InstructionProcessed := IsLeftExpression_ACC;
end;

Procedure Proc_DX_ACC; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_ACC_DX;
end;

Procedure Proc_ACC_ImmediateByte; Far;
begin
  If not IsLeftExpression_ACC then Exit;
  InstructionProcessed := GenerateImmediateByte;
end;

Procedure Proc_ImmediateByte_ACC; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_ACC_ImmediateByte;
end;

Procedure Proc_AX_16BitReg; Far;
begin
  If InstructionTokens <> Register_Register then Exit;
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  If LeftExpression^.Register <> rAX then Exit;
  If RightExpression^.RegisterType <> rt16BitRegister then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or RightExpression^.Register;
  InstructionProcessed := True;
end;

Procedure Proc_16BitReg_AX; Far;
begin
  SwapLeftAndRightExpression;
  Proc_AX_16BitReg;
end;

Procedure Proc_ImmediateByteWithSpecialFor_3; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  RightExpression := LeftExpression;
  If not RightExpressionNumericConstantAndSize8Bit then Exit;
  If LeftExpression^.Value.Byte <> $03 then
    begin
      Inc (LastOpCodeBeforeModem);
      GenerateLastOpCodeBeforeDataAnd_acByte (LeftExpression^.Value.Byte);
    end;
  InstructionProcessed := True;
end;

Procedure Proc_ImmediateByte_ImmediateWord; Far;
begin
  SwapLeftAndRightExpression;
  GenerateOpCodesExceptLastBeforeData;
  If not GenerateImmediateWord then Exit;
  SwapLeftAndRightExpression;
  InstructionProcessed := GenerateImmediateByte;
end;

Procedure Proc_ACC_Immediate; Far;
begin
  If InstructionTokens <> Register_Constant then Exit;
  If not RightExpression16BitNumericConstant then Exit;
  With LeftExpression^ do
    begin
      If (Register <> rACC) or (RegisterType > rt16BitRegister) then Exit;
      Case RegisterType of
        rt8BitRegister: begin
                          If not RightExpressionNumericConstantAndSize8Bit then Exit;
                          GenerateOpCodesExceptLastBeforeData;
                          InstructionProcessed := GenerateImmediateByte;
                        end;
        rt16BitRegister: begin
                           GenerateOpCodesExceptLastBeforeData;
                           Inc (LastOpCodeBeforeModem);
                           InstructionProcessed := GenerateImmediateWord;
                         end;
      end;
    end;
end;

Procedure Proc_Reg_Immediate; Far;
begin
  With LeftExpression^ do
    begin
      If InstructionTokens <> Register_Constant then Exit;
      If RegisterType > rt16BitRegister then Exit;
      GenerateOpCodesExceptLastBeforeData;
      LastOpCodeBeforeModem := LastOpCodeBeforeModem or Register;
      Case RegisterType of
        rt8BitRegister: InstructionProcessed := GenerateImmediateByte;
        rt16BitRegister: begin
                           LastOpCodeBeforeModem := LastOpCodeBeforeModem or $08;
                           InstructionProcessed := GenerateImmediateWord;
                         end;
      end;
    end;
end;

Procedure Proc_NearIndirect; Far;
begin
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> Ord (rt16BitRegister) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LeftExpressionTypeSize := 0;
  If not GenerateRestOf_RegMem__Register_Instruction then Exit;
  InstructionProcessed := True;
end;

Procedure Proc_FarIdentifierReference; Far;
begin
  GenerateInstructionWithIdentifierReference (3, $FFFE);
end;

Procedure Proc_FarIndirect; Far;
begin
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> $FF then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_JumpLabel; Far;
begin
  With LeftExpression^ do
    begin
      If Token <> AsmToken_MemoryReference then Exit;
      If TypeSize <> $FFFD then Exit;
      If MemoryReferences <> [mrMemoryLocation] then Exit;
      If Value.LongInt <> 0 then Exit;
      GeneratePrefixOpCodes;
      CurrentRelocatableIdentifier.Long := RelocatableIdentifier;
      AddAsmIntermediateCodeRecord (acJumpNear, 0, InstructionData^.Data [0], Byte (AsmToken));
    end;
  InstructionProcessed := True;
end;

Procedure Proc_DisplacementIdentifierReference; Far;
begin
  If LeftExpression^.TypeSize = $FFFD then
    GenerateInstructionWithIdentifierReference (0, $FFFD) else
      GenerateInstructionWithIdentifierReference (0, $FFFF);
end;

Procedure Proc_MemAddress_ACC; Far;
begin
  If InstructionTokens <> MemoryReference_Register then Exit;
  If RightExpression^.Register <> rACC then Exit;
  If not LeftExpressionRegMem then Exit;
  If RegisterInMemoryReference then Exit;
  GenerateOpCodesExceptLastBeforeData;
  If RightExpression^.RegisterType > rt16BitRegister then Exit;
  If Ord (RightExpression^.RegisterType) <> LeftExpressionTypeSize then Exit;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or Ord (RightExpression^.RegisterType);
  Generate16BitDisplacement;
  InstructionProcessed := True;
end;

Procedure Proc_ACC_MemAddress; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_MemAddress_ACC;
end;

Procedure Proc_ImmediateWithSpecialFor_0_OrNoParameter; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  If Flags and $0100 <> 0 then LastOpCodeBeforeModem := LastOpCodeBeforeModem or $08;
  InstructionProcessed := True;
  If ExpressionCounter = 0 then Exit;
  RightExpression := LeftExpression;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or $01;
  If RightExpressionWordConstant_0 then Exit;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem and $FE;
  InstructionProcessed := GenerateImmediateWord;
end;

Procedure Proc_ImmediateWithSpecialFor_0; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  RightExpression := LeftExpression;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or $01;
  InstructionProcessed := True;
  If RightExpressionWordConstant_0 then Exit;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem and $FE;
  InstructionProcessed := GenerateImmediateWord;
end;

Procedure Proc_SyntaxError; Far;
begin
  AsmError (SyntaxError);
end;

Procedure Proc_SegmentRegister; Far;
begin
  InstructionWithSegmentRegister ($FF);
end;

Procedure Proc_SegmentRegisterExcept_CS; Far;
begin
  InstructionWithSegmentRegister (1);
end;

Procedure Proc_16BitReg; Far;
begin
  If LeftToken <> AsmToken_Register then Exit;
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or LeftExpression^.Register;
  InstructionProcessed := True;
end;

Procedure Proc_DescriptorTable; Far;
begin
  If not LeftExpressionRegMem then Exit;
  With LeftExpression^ do If (TypeSize <> 6) and (TypeSize <> 0) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_16BitRegMem_16BitReg; Far;
begin
  If RightToken <> AsmToken_Register then Exit;
  If RightExpression^.RegisterType <> rt16BitRegister then Exit;
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> Ord (rt16BitRegister) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  InstructionProcessed := ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
end;

Procedure Proc_16BitReg_16BitRegMem; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_16BitRegMem_16BitReg
end;

Procedure Proc_Reg_RegMem_Immediate; Far;
Var IsByte: Boolean;
begin
  If LeftToken <> AsmToken_Register then
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  SwapLeftAndRightExpression;
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> Ord (rt16BitRegister) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  RightExpression := @AsmExpression3;
  If not RightExpression16BitNumericConstant then Exit;
  IsByte := RightExpressionSignedByteConstant;
  If IsByte then LastOpCodeBeforeModem := LastOpCodeBeforeModem or $02;
  ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
  If IsByte then GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte) else
    begin
      GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte);
      GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte1);
    end;
  InstructionProcessed := True;
end;

Procedure Proc_RegMem_SegmentReg; Far;
begin
  If RightExpression^.Token <> AsmToken_Register then Exit;
  If RightExpression^.RegisterType <> rtSegmentRegister then Exit;
  If not LeftExpressionRegMem then Exit;
  If (LeftExpressionTypeSize <> 1) and (RightExpression^.TypeSize <> 0) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  InstructionProcessed := ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
end;

Procedure Proc_SegmentReg_RegMem; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  If RightExpression^.Register <> rCX then Proc_RegMem_SegmentReg;
end;

Procedure Proc_ByteOrWord_RegMem_Reg; Far;
begin
  If RightToken <> AsmToken_Register then
    begin
      If LeftToken <> AsmToken_Register then Exit;
      Direction := 2;
      SwapLeftAndRightExpression;
    end;
  If RightExpression^.RegisterType > rt16BitRegister then Exit;
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  If Byte (RightExpression^.RegisterType) = LeftExpressionTypeSize then InstructionProcessed := GenerateRestOfInstruction;
end;

Procedure Proc_ByteOrWord_RegMem_Reg_Or_Reg_RegMem; Far;

  Function ByteOrWord_RegMem_Reg: Boolean;
  begin
    ByteOrWord_RegMem_Reg := False;
    If RightToken <> AsmToken_Register then Exit;
    If RightExpression^.RegisterType > rt16BitRegister then Exit;
    If not LeftExpressionByteOrWordRegMem then Exit;
    GenerateOpCodesExceptLastBeforeData;
    AddRegisterToModemOpcode;
    If Byte (RightExpression^.RegisterType) <> LeftExpressionTypeSize then Exit;
    ByteOrWord_RegMem_Reg := GenerateRestOfInstruction;
  end;

begin
  InstructionProcessed := True;
  If ByteOrWord_RegMem_Reg then Exit;
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  InstructionProcessed := ByteOrWord_RegMem_Reg;
end;

Procedure Proc_16bit_RegisterOrMemory; Far;
begin
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> Ord (rt16BitRegister) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_RegMem_ImmediateSignedByteOrWord; Far;
begin
  If not RightExpression16BitNumericConstant then Exit;
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  If LeftExpressionTypeSize <> 0 then
    begin
      If not RightExpressionSignedByteConstant then
        begin
          If not GenerateRestOf_RegMem__Register_Instruction then Exit;
          InstructionProcessed := GenerateImmediateWord;
          Exit;
        end;
      LastOpCodeBeforeModem := LastOpCodeBeforeModem or $02;
    end;
  If not RightExpressionNumericConstantAndSize8Bit then Exit;
  If not GenerateRestOf_RegMem__Register_Instruction then Exit;
  InstructionProcessed := GenerateImmediateByte;
end;

Procedure Proc_RegMem__Immediate; Far;
begin
  If RightToken <> AsmToken_Constant then Exit;
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  GenerateRestOf_RegMem__Register_Instruction;
  Case LeftExpressionTypeSize of
    1: InstructionProcessed := GenerateImmediateWord;
    else InstructionProcessed := GenerateImmediateByte;
  end;
end;

Procedure Proc_UnaryOperations; Far;
begin
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  If not GenerateRestOf_RegMem__Register_Instruction then Exit;
  InstructionProcessed := True;
end;

Procedure Proc_16BitReg_PointerMemoryReference; Far;
begin
  If InstructionTokens <> Register_MemoryReference then Exit;
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  SwapLeftAndRightExpression;
  LeftExpressionRegMem;
  If (SavedRightExpressionTypeSize <> 0) and (LeftExpressionTypeSize <> $FF) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  InstructionProcessed := ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
end;

Procedure Proc_ImmediateWithSpecialFor_1_Or_CL; Far;
begin
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  If RightExpressionNumericConstantAndSize8Bit then
    begin
      If RightExpression^.Value.Byte <> 1 then
        begin
          If Flags and $0003 = 0 then AsmError (_286_287_InstructionsAreNotEnabled);
          Dec (LastOpCodeBeforeModem, $10);
          GenerateRestOf_RegMem__Register_Instruction;
          GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte);
          InstructionProcessed := True;
        end else InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
    end else begin
               If RightExpression^.Token <> AsmToken_Register then Exit;
               If RightExpression^.RegisterType <> rt8BitRegister then Exit;
               If RightExpression^.Register <> rCL then Exit;
               Inc (LastOpCodeBeforeModem, 2);
               InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
             end;
end;

Procedure Proc_FPU_Instruction;
begin
  ModifyAndGenerate_FPU_Instruction (0)
end;

Procedure Proc_AX; Far;
begin
  If LeftToken <> AsmToken_Register then Exit;
  With Leftexpression^ do
    begin
      If RegisterType <> rt16BitRegister then Exit;
      If Register <> rAX then Exit;
    end;
  ModifyAndGenerate_FPU_Instruction (0)
end;

Procedure Process_FPU_Instruction__STi;
begin
  If InstructionTokens <> Register_Register then Exit;
  If LeftExpression^.RegisterType <> rtFPURegister then Exit;
  Proc_FPU_Instruction;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or LeftExpression^.Register;
  InstructionProcessed := True;
end;

Procedure Proc_STi; Far;
begin
  RightToken := AsmToken_Register;
  Process_FPU_Instruction__STi;
end;

Procedure Proc_STi_ST; Far;
begin
  With RightExpression^ do If (RegisterType <> rtFPURegister) or (Register <> 0) then Exit;
  Process_FPU_Instruction__STi;
end;

Procedure Proc_ST_STi; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_STi_ST;
end;

Procedure FPU_Instruction_Size1_or_Size2 (Size: Word);
Var RealSizeModifier: Byte;
begin
  If LeftToken <> AsmToken_MemoryReference then Exit;
  If not LeftExpressionRegMem then Exit;
  RealSizeModifier := 0;
  If LeftExpressionTypeSize <> Hi (Size) then
    begin
      If LeftExpressionTypeSize <> Lo (Size) then Exit;
      RealSizeModifier := $04;
    end;
  ModifyAndGenerate_FPU_Instruction (RealSizeModifier);
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_real32_or_real64; Far;
begin
  FPU_Instruction_Size1_or_Size2 ($FFFE);
end;

Procedure Proc_real80; Far;
begin
  FPU_Instruction_Size1_or_Size2 ($FDFD);
end;

Procedure Proc_int16; Far;
begin
  With LeftExpression^ do
    If TypeSize = 0 then TypeSize := 2;
  FPU_Instruction_Size1_or_Size2 ($0101);
end;

Procedure Proc_int16_or_int32; Far;
begin
  FPU_Instruction_Size1_or_Size2 ($FF01);
end;

Procedure Proc_int64; Far;
begin
  FPU_Instruction_Size1_or_Size2 ($FEFE);
end;

Procedure FPU_Instruction__MemoryReferenceSize (Size: Word);
begin
  With LeftExpression^ do If (TypeSize <> Size) and (TypeSize <> 0) then Exit;
  If LeftToken <> AsmToken_MemoryReference then Exit;
  If not LeftExpressionRegMem then Exit;
  Proc_FPU_Instruction;
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_MemoryReferenceSize_14; Far;
begin
  FPU_Instruction__MemoryReferenceSize ($000E);
end;

Procedure Proc_MemoryReferenceSize_94; Far;
begin
  FPU_Instruction__MemoryReferenceSize ($005E);
end;
```

### Code Block 5

```pascal
Type PInstructionData = ^TInstructionData;
     TInstructionData = Record
                          B0,       { x654321x  = InstructionProc shl 1 }

                          B1: Byte; { 76xx xxxx = Number of parameters
                                    { xxxx x2xx = Instruction 80286 }
                                    { xxxx xx10 = Number of additinal data bytes - 1 }

                          Data: Array [0..7] of Byte;
                        end;
```

### Code Block 6

```pascal
Procedure GenerateInstructionOpcodes;
begin
  Repeat
    SizeOfAsmIntermediateCodeRecords := 0;
    LastOpCodeBeforeDataValid := False;
    Direction := 0;
    SegmentOverrideOpCode := 0;
    If InstructionData^.B1 shr 6 = ExpressionCounter then
      begin
        LeftExpression  := @AsmExpression1;
        RightExpression := @AsmExpression2;
        LeftToken := LeftExpression^.Token;
        RightToken := RightExpression^.Token;
        InstructionProcessed := False;
        InstructionProc [(InstructionData^.B0 and $7E) shr 1];
        If InstructionProcessed then Break;
      end;
    If InstructionData^.B0 and $80 = 0 then                 { Last variant }
      begin
        If ExpressionCounter < 2 then
          AsmSourceErrorPosition := ErrorPositionInstructionExpresion1 else
            AsmSourceErrorPosition := ErrorPositionInstructionExpresion2;
        AsmError (InvalidCombinationOfOpcodeAndOperands);
      end;
    Inc (PChar (InstructionData), NumberOfOpCodesBeforeModem + 2);
  until False;
  If (InstructionData^.B1 and $04 <> 0) and (Flags and $03 = 0) then
    begin
      AsmSourceErrorPosition := ErrorPositionInstructionExpresion1;
      AsmError (_286_287_InstructionsAreNotEnabled);
    end;
  GenerateLastOpCodeBeforeData;
<span>end;</span>
```

### Code Block 7

```pascal
Const InstructionProc: Array [0..53] of Procedure = (
 { 00 } Proc_NoParameters,
 { 01 } Proc_ImmediateWord,
 { 02 } Proc_ImmediateSignedByte,
 { 03 } Proc_16BitReg_RegMem,
 { 04 } Proc_DX_ACC,
 { 05 } Proc_ImmediateByte_ACC,
 { 06 } Proc_ACC_DX,
 { 07 } Proc_ACC_ImmediateByte,
 { 08 } Proc_16BitReg_AX,
 { 09 } Proc_AX_16BitReg,
 { 0A } Proc_ImmediateByteWithSpecialFor_3,
 { 0B } Proc_ImmediateByte_ImmediateWord,
 { 0C } Proc_ACC_Immediate,
 { 0D } Proc_Reg_Immediate,
 { 0E } Proc_NearIndirect,
 { 0F } Proc_FarIdentifierReference,
 { 10 } Proc_FarIndirect,
 { 11 } Proc_JumpLabel,
 { 12 } Proc_DisplacementIdentifierReference,
 { 13 } Proc_MemAddress_ACC,
 { 14 } Proc_ACC_MemAddress,
 { 15 } Proc_ImmediateWithSpecialFor_0_OrNoParameter,
 { 16 } Proc_ImmediateWithSpecialFor_0,
 { 17 } Proc_SyntaxError,
 { 18 } Proc_SegmentRegister,
 { 19 } Proc_SegmentRegisterExcept_CS,
 { 1A } Proc_16BitReg,
 { 1B } Proc_DescriptorTable,
 { 1C } Proc_16BitReg_16BitRegMem,
 { 1D } Proc_16BitRegMem_16BitReg,
 { 1E } Proc_Reg_RegMem_Immediate,
 { 1F } Proc_SegmentReg_RegMem,
 { 20 } Proc_RegMem_SegmentReg,
 { 21 } Proc_ByteOrWord_RegMem_Reg,
 { 22 } Proc_ByteOrWord_RegMem_Reg_Or_Reg_RegMem,
 { 23 } Proc_16bit_RegisterOrMemory,
 { 24 } Proc_RegMem_ImmediateSignedByteOrWord,
 { 25 } Proc_RegMem__Immediate,
 { 26 } Proc_UnaryOperations,
 { 27 } Proc_16BitReg_PointerMemoryReference,
 { 28 } Proc_ImmediateWithSpecialFor_1_Or_CL,
 { 29 } Proc_FPU_Instruction,
 { 2A } Proc_AX,
 { 2B } Proc_STi,
 { 2C } Proc_STi_ST,
 { 2D } Proc_ST_STi,
 { 2E } Proc_real32_or_real64,
 { 2F } Proc_real80,
 { 30 } Proc_int16,
 { 31 } Proc_int16_or_int32,
 { 32 } Proc_int64,
 { 33 } Proc_MemoryReferenceSize_14,
 { 34 } Proc_MemoryReferenceSize_94,
 { 35 } Proc_SyntaxError);
```

### Code Block 8

```pascal
Procedure Proc_NoParameters; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  InstructionProcessed := True;
end;

Procedure Proc_ImmediateWord; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  SwapLeftAndRightExpression;
  InstructionProcessed := GenerateImmediateWord;
end;

Procedure Proc_ImmediateSignedByte; Far;
begin
  SwapLeftAndRightExpression;
  GenerateOpCodesExceptLastBeforeData;
  If not RightExpressionSignedByteConstant then Exit;
  GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte);
  InstructionProcessed := True;
end;

Procedure Proc_16BitReg_RegMem; Far;
begin
  If InstructionTokens <> Register_MemoryReference then Exit;
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  SwapLeftAndRightExpression;
  If not LeftExpressionRegMem then Exit;
  SwapLeftAndRightExpression;
  SegmentOverrideOpCode := 0;
  GenerateOpCodesExceptLastBeforeData;
  ModemOpcode := ModemOpcode or LeftExpression^.Register shl 3;
  InstructionProcessed := ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
end;

Procedure Proc_ACC_DX; Far;
begin
  With RightExpression^ do
    begin
      If Token <> AsmToken_Register then Exit;
      If RegisterType <> rt16BitRegister then Exit;
      If Register <> rDX then Exit;
    end;
  InstructionProcessed := IsLeftExpression_ACC;
end;

Procedure Proc_DX_ACC; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_ACC_DX;
end;

Procedure Proc_ACC_ImmediateByte; Far;
begin
  If not IsLeftExpression_ACC then Exit;
  InstructionProcessed := GenerateImmediateByte;
end;

Procedure Proc_ImmediateByte_ACC; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_ACC_ImmediateByte;
end;

Procedure Proc_AX_16BitReg; Far;
begin
  If InstructionTokens <> Register_Register then Exit;
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  If LeftExpression^.Register <> rAX then Exit;
  If RightExpression^.RegisterType <> rt16BitRegister then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or RightExpression^.Register;
  InstructionProcessed := True;
end;

Procedure Proc_16BitReg_AX; Far;
begin
  SwapLeftAndRightExpression;
  Proc_AX_16BitReg;
end;

Procedure Proc_ImmediateByteWithSpecialFor_3; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  RightExpression := LeftExpression;
  If not RightExpressionNumericConstantAndSize8Bit then Exit;
  If LeftExpression^.Value.Byte <> $03 then
    begin
      Inc (LastOpCodeBeforeModem);
      GenerateLastOpCodeBeforeDataAnd_acByte (LeftExpression^.Value.Byte);
    end;
  InstructionProcessed := True;
end;

Procedure Proc_ImmediateByte_ImmediateWord; Far;
begin
  SwapLeftAndRightExpression;
  GenerateOpCodesExceptLastBeforeData;
  If not GenerateImmediateWord then Exit;
  SwapLeftAndRightExpression;
  InstructionProcessed := GenerateImmediateByte;
end;

Procedure Proc_ACC_Immediate; Far;
begin
  If InstructionTokens <> Register_Constant then Exit;
  If not RightExpression16BitNumericConstant then Exit;
  With LeftExpression^ do
    begin
      If (Register <> rACC) or (RegisterType > rt16BitRegister) then Exit;
      Case RegisterType of
        rt8BitRegister: begin
                          If not RightExpressionNumericConstantAndSize8Bit then Exit;
                          GenerateOpCodesExceptLastBeforeData;
                          InstructionProcessed := GenerateImmediateByte;
                        end;
        rt16BitRegister: begin
                           GenerateOpCodesExceptLastBeforeData;
                           Inc (LastOpCodeBeforeModem);
                           InstructionProcessed := GenerateImmediateWord;
                         end;
      end;
    end;
end;

Procedure Proc_Reg_Immediate; Far;
begin
  With LeftExpression^ do
    begin
      If InstructionTokens <> Register_Constant then Exit;
      If RegisterType > rt16BitRegister then Exit;
      GenerateOpCodesExceptLastBeforeData;
      LastOpCodeBeforeModem := LastOpCodeBeforeModem or Register;
      Case RegisterType of
        rt8BitRegister: InstructionProcessed := GenerateImmediateByte;
        rt16BitRegister: begin
                           LastOpCodeBeforeModem := LastOpCodeBeforeModem or $08;
                           InstructionProcessed := GenerateImmediateWord;
                         end;
      end;
    end;
end;

Procedure Proc_NearIndirect; Far;
begin
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> Ord (rt16BitRegister) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LeftExpressionTypeSize := 0;
  If not GenerateRestOf_RegMem__Register_Instruction then Exit;
  InstructionProcessed := True;
end;

Procedure Proc_FarIdentifierReference; Far;
begin
  GenerateInstructionWithIdentifierReference (3, $FFFE);
end;

Procedure Proc_FarIndirect; Far;
begin
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> $FF then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_JumpLabel; Far;
begin
  With LeftExpression^ do
    begin
      If Token <> AsmToken_MemoryReference then Exit;
      If TypeSize <> $FFFD then Exit;
      If MemoryReferences <> [mrMemoryLocation] then Exit;
      If Value.LongInt <> 0 then Exit;
      GeneratePrefixOpCodes;
      CurrentRelocatableIdentifier.Long := RelocatableIdentifier;
      AddAsmIntermediateCodeRecord (acJumpNear, 0, InstructionData^.Data [0], Byte (AsmToken));
    end;
  InstructionProcessed := True;
end;

Procedure Proc_DisplacementIdentifierReference; Far;
begin
  If LeftExpression^.TypeSize = $FFFD then
    GenerateInstructionWithIdentifierReference (0, $FFFD) else
      GenerateInstructionWithIdentifierReference (0, $FFFF);
end;

Procedure Proc_MemAddress_ACC; Far;
begin
  If InstructionTokens <> MemoryReference_Register then Exit;
  If RightExpression^.Register <> rACC then Exit;
  If not LeftExpressionRegMem then Exit;
  If RegisterInMemoryReference then Exit;
  GenerateOpCodesExceptLastBeforeData;
  If RightExpression^.RegisterType > rt16BitRegister then Exit;
  If Ord (RightExpression^.RegisterType) <> LeftExpressionTypeSize then Exit;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or Ord (RightExpression^.RegisterType);
  Generate16BitDisplacement;
  InstructionProcessed := True;
end;

Procedure Proc_ACC_MemAddress; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_MemAddress_ACC;
end;

Procedure Proc_ImmediateWithSpecialFor_0_OrNoParameter; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  If Flags and $0100 <> 0 then LastOpCodeBeforeModem := LastOpCodeBeforeModem or $08;
  InstructionProcessed := True;
  If ExpressionCounter = 0 then Exit;
  RightExpression := LeftExpression;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or $01;
  If RightExpressionWordConstant_0 then Exit;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem and $FE;
  InstructionProcessed := GenerateImmediateWord;
end;

Procedure Proc_ImmediateWithSpecialFor_0; Far;
begin
  GenerateOpCodesExceptLastBeforeData;
  RightExpression := LeftExpression;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or $01;
  InstructionProcessed := True;
  If RightExpressionWordConstant_0 then Exit;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem and $FE;
  InstructionProcessed := GenerateImmediateWord;
end;

Procedure Proc_SyntaxError; Far;
begin
  AsmError (SyntaxError);
end;

Procedure Proc_SegmentRegister; Far;
begin
  InstructionWithSegmentRegister ($FF);
end;

Procedure Proc_SegmentRegisterExcept_CS; Far;
begin
  InstructionWithSegmentRegister (1);
end;

Procedure Proc_16BitReg; Far;
begin
  If LeftToken <> AsmToken_Register then Exit;
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or LeftExpression^.Register;
  InstructionProcessed := True;
end;

Procedure Proc_DescriptorTable; Far;
begin
  If not LeftExpressionRegMem then Exit;
  With LeftExpression^ do If (TypeSize <> 6) and (TypeSize <> 0) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_16BitRegMem_16BitReg; Far;
begin
  If RightToken <> AsmToken_Register then Exit;
  If RightExpression^.RegisterType <> rt16BitRegister then Exit;
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> Ord (rt16BitRegister) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  InstructionProcessed := ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
end;

Procedure Proc_16BitReg_16BitRegMem; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_16BitRegMem_16BitReg
end;

Procedure Proc_Reg_RegMem_Immediate; Far;
Var IsByte: Boolean;
begin
  If LeftToken <> AsmToken_Register then
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  SwapLeftAndRightExpression;
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> Ord (rt16BitRegister) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  RightExpression := @AsmExpression3;
  If not RightExpression16BitNumericConstant then Exit;
  IsByte := RightExpressionSignedByteConstant;
  If IsByte then LastOpCodeBeforeModem := LastOpCodeBeforeModem or $02;
  ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
  If IsByte then GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte) else
    begin
      GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte);
      GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte1);
    end;
  InstructionProcessed := True;
end;

Procedure Proc_RegMem_SegmentReg; Far;
begin
  If RightExpression^.Token <> AsmToken_Register then Exit;
  If RightExpression^.RegisterType <> rtSegmentRegister then Exit;
  If not LeftExpressionRegMem then Exit;
  If (LeftExpressionTypeSize <> 1) and (RightExpression^.TypeSize <> 0) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  InstructionProcessed := ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
end;

Procedure Proc_SegmentReg_RegMem; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  If RightExpression^.Register <> rCX then Proc_RegMem_SegmentReg;
end;

Procedure Proc_ByteOrWord_RegMem_Reg; Far;
begin
  If RightToken <> AsmToken_Register then
    begin
      If LeftToken <> AsmToken_Register then Exit;
      Direction := 2;
      SwapLeftAndRightExpression;
    end;
  If RightExpression^.RegisterType > rt16BitRegister then Exit;
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  If Byte (RightExpression^.RegisterType) = LeftExpressionTypeSize then InstructionProcessed := GenerateRestOfInstruction;
end;

Procedure Proc_ByteOrWord_RegMem_Reg_Or_Reg_RegMem; Far;

  Function ByteOrWord_RegMem_Reg: Boolean;
  begin
    ByteOrWord_RegMem_Reg := False;
    If RightToken <> AsmToken_Register then Exit;
    If RightExpression^.RegisterType > rt16BitRegister then Exit;
    If not LeftExpressionByteOrWordRegMem then Exit;
    GenerateOpCodesExceptLastBeforeData;
    AddRegisterToModemOpcode;
    If Byte (RightExpression^.RegisterType) <> LeftExpressionTypeSize then Exit;
    ByteOrWord_RegMem_Reg := GenerateRestOfInstruction;
  end;

begin
  InstructionProcessed := True;
  If ByteOrWord_RegMem_Reg then Exit;
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  InstructionProcessed := ByteOrWord_RegMem_Reg;
end;

Procedure Proc_16bit_RegisterOrMemory; Far;
begin
  If not LeftExpressionRegMem then Exit;
  If LeftExpressionTypeSize <> Ord (rt16BitRegister) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_RegMem_ImmediateSignedByteOrWord; Far;
begin
  If not RightExpression16BitNumericConstant then Exit;
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  If LeftExpressionTypeSize <> 0 then
    begin
      If not RightExpressionSignedByteConstant then
        begin
          If not GenerateRestOf_RegMem__Register_Instruction then Exit;
          InstructionProcessed := GenerateImmediateWord;
          Exit;
        end;
      LastOpCodeBeforeModem := LastOpCodeBeforeModem or $02;
    end;
  If not RightExpressionNumericConstantAndSize8Bit then Exit;
  If not GenerateRestOf_RegMem__Register_Instruction then Exit;
  InstructionProcessed := GenerateImmediateByte;
end;

Procedure Proc_RegMem__Immediate; Far;
begin
  If RightToken <> AsmToken_Constant then Exit;
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  GenerateRestOf_RegMem__Register_Instruction;
  Case LeftExpressionTypeSize of
    1: InstructionProcessed := GenerateImmediateWord;
    else InstructionProcessed := GenerateImmediateByte;
  end;
end;

Procedure Proc_UnaryOperations; Far;
begin
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  If not GenerateRestOf_RegMem__Register_Instruction then Exit;
  InstructionProcessed := True;
end;

Procedure Proc_16BitReg_PointerMemoryReference; Far;
begin
  If InstructionTokens <> Register_MemoryReference then Exit;
  If LeftExpression^.RegisterType <> rt16BitRegister then Exit;
  SwapLeftAndRightExpression;
  LeftExpressionRegMem;
  If (SavedRightExpressionTypeSize <> 0) and (LeftExpressionTypeSize <> $FF) then Exit;
  GenerateOpCodesExceptLastBeforeData;
  AddRegisterToModemOpcode;
  InstructionProcessed := ClearLeftExprTypeSizeAndGenerateRestOfInstruction;
end;

Procedure Proc_ImmediateWithSpecialFor_1_Or_CL; Far;
begin
  If not LeftExpressionByteOrWordRegMem then Exit;
  GenerateOpCodesExceptLastBeforeData;
  If RightExpressionNumericConstantAndSize8Bit then
    begin
      If RightExpression^.Value.Byte <> 1 then
        begin
          If Flags and $0003 = 0 then AsmError (_286_287_InstructionsAreNotEnabled);
          Dec (LastOpCodeBeforeModem, $10);
          GenerateRestOf_RegMem__Register_Instruction;
          GenerateLastOpCodeBeforeDataAnd_acByte (RightExpression^.Value.Byte);
          InstructionProcessed := True;
        end else InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
    end else begin
               If RightExpression^.Token <> AsmToken_Register then Exit;
               If RightExpression^.RegisterType <> rt8BitRegister then Exit;
               If RightExpression^.Register <> rCL then Exit;
               Inc (LastOpCodeBeforeModem, 2);
               InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
             end;
end;

Procedure Proc_FPU_Instruction;
begin
  ModifyAndGenerate_FPU_Instruction (0)
end;

Procedure Proc_AX; Far;
begin
  If LeftToken <> AsmToken_Register then Exit;
  With Leftexpression^ do
    begin
      If RegisterType <> rt16BitRegister then Exit;
      If Register <> rAX then Exit;
    end;
  ModifyAndGenerate_FPU_Instruction (0)
end;

Procedure Process_FPU_Instruction__STi;
begin
  If InstructionTokens <> Register_Register then Exit;
  If LeftExpression^.RegisterType <> rtFPURegister then Exit;
  Proc_FPU_Instruction;
  LastOpCodeBeforeModem := LastOpCodeBeforeModem or LeftExpression^.Register;
  InstructionProcessed := True;
end;

Procedure Proc_STi; Far;
begin
  RightToken := AsmToken_Register;
  Process_FPU_Instruction__STi;
end;

Procedure Proc_STi_ST; Far;
begin
  With RightExpression^ do If (RegisterType <> rtFPURegister) or (Register <> 0) then Exit;
  Process_FPU_Instruction__STi;
end;

Procedure Proc_ST_STi; Far;
begin
  SwapLeftAndRightExpression;
  InstructionTokens := Swap (InstructionTokens);
  Proc_STi_ST;
end;

Procedure FPU_Instruction_Size1_or_Size2 (Size: Word);
Var RealSizeModifier: Byte;
begin
  If LeftToken <> AsmToken_MemoryReference then Exit;
  If not LeftExpressionRegMem then Exit;
  RealSizeModifier := 0;
  If LeftExpressionTypeSize <> Hi (Size) then
    begin
      If LeftExpressionTypeSize <> Lo (Size) then Exit;
      RealSizeModifier := $04;
    end;
  ModifyAndGenerate_FPU_Instruction (RealSizeModifier);
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_real32_or_real64; Far;
begin
  FPU_Instruction_Size1_or_Size2 ($FFFE);
end;

Procedure Proc_real80; Far;
begin
  FPU_Instruction_Size1_or_Size2 ($FDFD);
end;

Procedure Proc_int16; Far;
begin
  With LeftExpression^ do
    If TypeSize = 0 then TypeSize := 2;
  FPU_Instruction_Size1_or_Size2 ($0101);
end;

Procedure Proc_int16_or_int32; Far;
begin
  FPU_Instruction_Size1_or_Size2 ($FF01);
end;

Procedure Proc_int64; Far;
begin
  FPU_Instruction_Size1_or_Size2 ($FEFE);
end;

Procedure FPU_Instruction__MemoryReferenceSize (Size: Word);
begin
  With LeftExpression^ do If (TypeSize <> Size) and (TypeSize <> 0) then Exit;
  If LeftToken <> AsmToken_MemoryReference then Exit;
  If not LeftExpressionRegMem then Exit;
  Proc_FPU_Instruction;
  LeftExpressionTypeSize := 0;
  InstructionProcessed := GenerateRestOf_RegMem__Register_Instruction;
end;

Procedure Proc_MemoryReferenceSize_14; Far;
begin
  FPU_Instruction__MemoryReferenceSize ($000E);
end;

Procedure Proc_MemoryReferenceSize_94; Far;
begin
  FPU_Instruction__MemoryReferenceSize ($005E);
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
- https://turbopascal.org/generating-assembler-instruction-opcodes
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/symbol-tables-and-hash-functions
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

