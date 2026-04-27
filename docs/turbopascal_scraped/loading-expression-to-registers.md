# Loading Expression To Registers - Turbo Pascal Compiler

**URL:** https://turbopascal.org/loading-expression-to-registers

---

## Table of Contents

- Loading Expression To Registers

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Loading Expression To Registers
Loading an expression into registers is a very common task in Turbo Pascal compiler. These two functions take care for this. They generate instructions to load the expression regardless where it is located.
Procedure TExpression.LoadExpressionToRegisters (Register: TUsedRegister);
begin
  LoadExpressionToRegister (Ord (Register));
  If it32Bit in DataType then
    begin
      SwitchBetweenLoWordAndHiWord (2);
      LoadExpressionToRegister (Ord (Register) + 2);
      Include (UsedRegisters, Succ (Succ(Register)));
    end ;
  Include (UsedRegisters, Register);
  Location := elRegister;
  LocationData.Register := Ord (Register);
end;

Procedure TExpression.LoadExpressionToRegister (Register: Byte);
begin
  Case Location of
    elMemory: If (Register = rACC) and (LocationData.Flags * [ofsDI, ofsBP] = []) then
                begin
                  GenerateSegmentOverride;
                  If not (it16Bit in DataType) then GenerateInstruction_Byte (MOV_AL_Mem)
                    else GenerateInstruction_Byte (MOV_AX_Mem);
                  GenerateOffsetReferenceForExpressionInMemory;
                end else GenerateInstruction_8_16_bit (MOV_AX_RegMem, Register shl 3);
    elRegister: If LocationData.Register <> Register then
                  GenerateInstruction_8_16_bit (MOV_AX_RegMem, Register shl 3);
    elPointerToMemory: begin
                         If efSegment in LocationData.Flags then
                           begin
                             If LocationData.Flags * SegmentMask <> [] then
                               GenerateInstruction_TwoBytes (
                                 MOV_RegMem_ES, modRegister or Register or ExpressionSegmentRegister) else
                                   GenerateInstructionWithSegment (MOV_16BitReg_Immediate or Register);
                           end else
                             If ofsBP in LocationData.Flags then
                               GenerateInstructionWithExpressionInMemOrReg (LEA, Register shl 3) else
                                 If ofsDI in LocationData.Flags then
                                   begin
                                     If (LocationData.Flags * [efRelocatable, segCS, segDS] <> []) or (Value.Offset <> 0) then
                                       GenerateInstructionWithExpressionInMemOrReg (LEA, Register shl 3) else
                                         GenerateInstruction_TwoBytes (MOV_16Bit_Mem_Reg,
                                                                       modRegister or rDI shl 3 or Register);
                                   end else
                                     If LocationData.Flags * [efRelocatable, segCS, segDS] = [] then
                                       GenerateInstruction_MOV_16BitReg_Immediate (Register, Value.Offset) else
                                         begin
                                           GenerateInstruction_Byte (MOV_16BitReg_Immediate or Register);
                                           GenerateOffsetReferenceForExpressionInMemory;
                                         end;
                       end;
    else If not (it16Bit in DataType) then GenerateInstruction_TwoBytes (MOV_8BitReg_Immediate or Register, Value.ShortInt)
           else GenerateInstruction_MOV_16BitReg_Immediate (Register, Value.Word);
  end;
end;
This procedure loads
Real
expression to register set. There are two sets of registers for real expressions:
AX BX DX
and
CX SI DI
.
Type TRealRegistersSet = (rAX_BX_DX, rCX_SI_DI);

Procedure TExpression.LoadRealExpressionToRegisterSet (RealRegistersSet: TRealRegistersSet);
Const RealRegisters: Array [TRealRegistersSet, 1..3] of Byte = (
        (rAX, rBX, rDX),
        (rCX, rSI, rDI));

  Procedure LoadRealExpressionToRegister (Reg: Byte);
  begin
    LoadExpressionToRegister (RealRegisters [RealRegistersSet, Reg]);
    Case Location of
      elMemory: Inc (Value.Offset, 2);
      else begin
             Value.Word := Value.W12;
             Value.W12 := Value.W14;
           end;
    end;
  end;

begin
  If Location <> elRegister then
    begin
      LoadRealExpressionToRegister (1);
      LoadRealExpressionToRegister (2);
      LoadRealExpressionToRegister (3);
      ES_DI_PointerDestroyed;
      Location := elRegister;
    end;
  UsedRegisters := [urBX, urDX, urCX, urAX];
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
Procedure TExpression.LoadExpressionToRegisters (Register: TUsedRegister);
begin
  LoadExpressionToRegister (Ord (Register));
  If it32Bit in DataType then
    begin
      SwitchBetweenLoWordAndHiWord (2);
      LoadExpressionToRegister (Ord (Register) + 2);
      Include (UsedRegisters, Succ (Succ(Register)));
    end ;
  Include (UsedRegisters, Register);
  Location := elRegister;
  LocationData.Register := Ord (Register);
end;

Procedure TExpression.LoadExpressionToRegister (Register: Byte);
begin
  Case Location of
    elMemory: If (Register = rACC) and (LocationData.Flags * [ofsDI, ofsBP] = []) then
                begin
                  GenerateSegmentOverride;
                  If not (it16Bit in DataType) then GenerateInstruction_Byte (MOV_AL_Mem)
                    else GenerateInstruction_Byte (MOV_AX_Mem);
                  GenerateOffsetReferenceForExpressionInMemory;
                end else GenerateInstruction_8_16_bit (MOV_AX_RegMem, Register shl 3);
    elRegister: If LocationData.Register <> Register then
                  GenerateInstruction_8_16_bit (MOV_AX_RegMem, Register shl 3);
    elPointerToMemory: begin
                         If efSegment in LocationData.Flags then
                           begin
                             If LocationData.Flags * SegmentMask <> [] then
                               GenerateInstruction_TwoBytes (
                                 MOV_RegMem_ES, modRegister or Register or ExpressionSegmentRegister) else
                                   GenerateInstructionWithSegment (MOV_16BitReg_Immediate or Register);
                           end else
                             If ofsBP in LocationData.Flags then
                               GenerateInstructionWithExpressionInMemOrReg (LEA, Register shl 3) else
                                 If ofsDI in LocationData.Flags then
                                   begin
                                     If (LocationData.Flags * [efRelocatable, segCS, segDS] <> []) or (Value.Offset <> 0) then
                                       GenerateInstructionWithExpressionInMemOrReg (LEA, Register shl 3) else
                                         GenerateInstruction_TwoBytes (MOV_16Bit_Mem_Reg,
                                                                       modRegister or rDI shl 3 or Register);
                                   end else
                                     If LocationData.Flags * [efRelocatable, segCS, segDS] = [] then
                                       GenerateInstruction_MOV_16BitReg_Immediate (Register, Value.Offset) else
                                         begin
                                           GenerateInstruction_Byte (MOV_16BitReg_Immediate or Register);
                                           GenerateOffsetReferenceForExpressionInMemory;
                                         end;
                       end;
    else If not (it16Bit in DataType) then GenerateInstruction_TwoBytes (MOV_8BitReg_Immediate or Register, Value.ShortInt)
           else GenerateInstruction_MOV_16BitReg_Immediate (Register, Value.Word);
  end;
end;
```

### Code Block 2

```pascal
Type TRealRegistersSet = (rAX_BX_DX, rCX_SI_DI);

Procedure TExpression.LoadRealExpressionToRegisterSet (RealRegistersSet: TRealRegistersSet);
Const RealRegisters: Array [TRealRegistersSet, 1..3] of Byte = (
        (rAX, rBX, rDX),
        (rCX, rSI, rDI));

  Procedure LoadRealExpressionToRegister (Reg: Byte);
  begin
    LoadExpressionToRegister (RealRegisters [RealRegistersSet, Reg]);
    Case Location of
      elMemory: Inc (Value.Offset, 2);
      else begin
             Value.Word := Value.W12;
             Value.W12 := Value.W14;
           end;
    end;
  end;

begin
  If Location <> elRegister then
    begin
      LoadRealExpressionToRegister (1);
      LoadRealExpressionToRegister (2);
      LoadRealExpressionToRegister (3);
      ES_DI_PointerDestroyed;
      Location := elRegister;
    end;
  UsedRegisters := [urBX, urDX, urCX, urAX];
end;
```

### Code Block 3

```pascal
Procedure TExpression.LoadExpressionToRegisters (Register: TUsedRegister);
begin
  LoadExpressionToRegister (Ord (Register));
  If it32Bit in DataType then
    begin
      SwitchBetweenLoWordAndHiWord (2);
      LoadExpressionToRegister (Ord (Register) + 2);
      Include (UsedRegisters, Succ (Succ(Register)));
    end ;
  Include (UsedRegisters, Register);
  Location := elRegister;
  LocationData.Register := Ord (Register);
end;

Procedure TExpression.LoadExpressionToRegister (Register: Byte);
begin
  Case Location of
    elMemory: If (Register = rACC) and (LocationData.Flags * [ofsDI, ofsBP] = []) then
                begin
                  GenerateSegmentOverride;
                  If not (it16Bit in DataType) then GenerateInstruction_Byte (MOV_AL_Mem)
                    else GenerateInstruction_Byte (MOV_AX_Mem);
                  GenerateOffsetReferenceForExpressionInMemory;
                end else GenerateInstruction_8_16_bit (MOV_AX_RegMem, Register shl 3);
    elRegister: If LocationData.Register <> Register then
                  GenerateInstruction_8_16_bit (MOV_AX_RegMem, Register shl 3);
    elPointerToMemory: begin
                         If efSegment in LocationData.Flags then
                           begin
                             If LocationData.Flags * SegmentMask <> [] then
                               GenerateInstruction_TwoBytes (
                                 MOV_RegMem_ES, modRegister or Register or ExpressionSegmentRegister) else
                                   GenerateInstructionWithSegment (MOV_16BitReg_Immediate or Register);
                           end else
                             If ofsBP in LocationData.Flags then
                               GenerateInstructionWithExpressionInMemOrReg (LEA, Register shl 3) else
                                 If ofsDI in LocationData.Flags then
                                   begin
                                     If (LocationData.Flags * [efRelocatable, segCS, segDS] <> []) or (Value.Offset <> 0) then
                                       GenerateInstructionWithExpressionInMemOrReg (LEA, Register shl 3) else
                                         GenerateInstruction_TwoBytes (MOV_16Bit_Mem_Reg,
                                                                       modRegister or rDI shl 3 or Register);
                                   end else
                                     If LocationData.Flags * [efRelocatable, segCS, segDS] = [] then
                                       GenerateInstruction_MOV_16BitReg_Immediate (Register, Value.Offset) else
                                         begin
                                           GenerateInstruction_Byte (MOV_16BitReg_Immediate or Register);
                                           GenerateOffsetReferenceForExpressionInMemory;
                                         end;
                       end;
    else If not (it16Bit in DataType) then GenerateInstruction_TwoBytes (MOV_8BitReg_Immediate or Register, Value.ShortInt)
           else GenerateInstruction_MOV_16BitReg_Immediate (Register, Value.Word);
  end;
end;
```

### Code Block 4

```pascal
Type TRealRegistersSet = (rAX_BX_DX, rCX_SI_DI);

Procedure TExpression.LoadRealExpressionToRegisterSet (RealRegistersSet: TRealRegistersSet);
Const RealRegisters: Array [TRealRegistersSet, 1..3] of Byte = (
        (rAX, rBX, rDX),
        (rCX, rSI, rDI));

  Procedure LoadRealExpressionToRegister (Reg: Byte);
  begin
    LoadExpressionToRegister (RealRegisters [RealRegistersSet, Reg]);
    Case Location of
      elMemory: Inc (Value.Offset, 2);
      else begin
             Value.Word := Value.W12;
             Value.W12 := Value.W14;
           end;
    end;
  end;

begin
  If Location <> elRegister then
    begin
      LoadRealExpressionToRegister (1);
      LoadRealExpressionToRegister (2);
      LoadRealExpressionToRegister (3);
      ES_DI_PointerDestroyed;
      Location := elRegister;
    end;
  UsedRegisters := [urBX, urDX, urCX, urAX];
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
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/loading-expression-to-registers
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

