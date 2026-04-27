# Integer Multiplication - Turbo Pascal Compiler

**URL:** https://turbopascal.org/integer-multiplication

---

## Table of Contents

- Integer Multiplication

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Integer Multiplication
This procedure generates code to multiply integer with integer. Multiplication with power of 2 is translated to shift, other special cases (0, 1, 3, 5, 6) are also treated separately. Result is stored in specified
Register
if possible.
Procedure TExpression.IntegerMultiplicationWithConstant (Register: Byte; Factor: Integer);
Var ShiftCount: ShortInt;
    TempFactor: Word;

  Procedure GeneralIntegerMultiplicationWithConstant;
  begin
    Case Instructions80286 in StatementCompilerSwitches of
      True: If Factor = ShortInt (Factor) then
              begin
                GenerateInstructionWithExpressionInMemOrReg (IMUL_Reg_SignExtendImmediateByte, Register shl 3);
                GenerateInstruction_Byte (Factor);
              end else begin
                         GenerateInstructionWithExpressionInMemOrReg (IMUL_Reg_Opearations, Register shl 3);
                         GenerateInstruction_Word (Factor);
                       end;
      else begin
             Include (Usedregisters, urDX);
             If (Location = elRegister) and (LocationData.Register = rAX) then
               begin
                 GenerateInstruction_Byte (MOV_DX_Immediate);
                 GenerateInstruction_Word (Factor);
                 GenerateInstruction_Word (MUL_DX);
                 Exit;
               end else begin
                          GenerateInstruction_Byte (MOV_AX_Immediate);
                          GenerateInstruction_Word (Factor);
                          GenerateInstructionWithExpressionInMemOrReg (Unary_16Bit_Operations, mod_MUL_rm);
                          Register := rAX;
                        end;
           end;
    end;
    Location := elRegister;
    LocationData.Register := Register;
    Include (Usedregisters, TUsedRegister (Register));
  end;

begin
  If Factor <= 0 then GeneralIntegerMultiplicationWithConstant else
    If Factor > 1 then
      begin
        ShiftCount := 0;
        TempFactor := Factor;
        While (TempFactor and $01) = 0 do
          begin
            Inc (ShiftCount);
            TempFactor := TempFactor shr 1;
          end;
        If TempFactor  = 1 then
          begin
            LoadExpressionToRegisters (TUsedRegister (Register));
            Case ShiftCount of
              1: GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
              else Case Instructions80286 in StatementCompilerSwitches of
                     True: begin
                             GenerateInstruction_TwoBytes (ShiftRotateOperationsByValue_16Bit, SHL_Reg or Register);
                             GenerateInstruction_Byte (ShiftCount);
                           end;
                     else Case ShiftCount of
                            2: begin
                                 GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                                 GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                               end;
                            else begin
                                   Include (UsedRegisters, urCX);
                                   GenerateInstruction_TwoBytes (MOV_8BitReg_Immediate or rCL, ShiftCount);
                                   GenerateInstruction_TwoBytes (ShiftRotateOperationsByCL_16Bit, SHL_Reg or Register);
                                 end;
                          end;
                   end;
            end;
          end else If Factor > 6 then GeneralIntegerMultiplicationWithConstant else
                     begin                          { Factor = 3, 5, 6 }
                       LoadExpressionToRegisters (TUsedRegister (Register));
                       If Factor = 6 then
                         GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                       GenerateInstruction_TwoBytes (MOV_16Bit_Reg_Source, SI__Reg or Register);
                       GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                       If Factor = 5 then
                         GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                       GenerateInstruction_TwoBytes (ADD_16Bit_Dest_Reg, Register or Reg__SI);
                     end;
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
Procedure TExpression.IntegerMultiplicationWithConstant (Register: Byte; Factor: Integer);
Var ShiftCount: ShortInt;
    TempFactor: Word;

  Procedure GeneralIntegerMultiplicationWithConstant;
  begin
    Case Instructions80286 in StatementCompilerSwitches of
      True: If Factor = ShortInt (Factor) then
              begin
                GenerateInstructionWithExpressionInMemOrReg (IMUL_Reg_SignExtendImmediateByte, Register shl 3);
                GenerateInstruction_Byte (Factor);
              end else begin
                         GenerateInstructionWithExpressionInMemOrReg (IMUL_Reg_Opearations, Register shl 3);
                         GenerateInstruction_Word (Factor);
                       end;
      else begin
             Include (Usedregisters, urDX);
             If (Location = elRegister) and (LocationData.Register = rAX) then
               begin
                 GenerateInstruction_Byte (MOV_DX_Immediate);
                 GenerateInstruction_Word (Factor);
                 GenerateInstruction_Word (MUL_DX);
                 Exit;
               end else begin
                          GenerateInstruction_Byte (MOV_AX_Immediate);
                          GenerateInstruction_Word (Factor);
                          GenerateInstructionWithExpressionInMemOrReg (Unary_16Bit_Operations, mod_MUL_rm);
                          Register := rAX;
                        end;
           end;
    end;
    Location := elRegister;
    LocationData.Register := Register;
    Include (Usedregisters, TUsedRegister (Register));
  end;

begin
  If Factor <= 0 then GeneralIntegerMultiplicationWithConstant else
    If Factor > 1 then
      begin
        ShiftCount := 0;
        TempFactor := Factor;
        While (TempFactor and $01) = 0 do
          begin
            Inc (ShiftCount);
            TempFactor := TempFactor shr 1;
          end;
        If TempFactor  = 1 then
          begin
            LoadExpressionToRegisters (TUsedRegister (Register));
            Case ShiftCount of
              1: GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
              else Case Instructions80286 in StatementCompilerSwitches of
                     True: begin
                             GenerateInstruction_TwoBytes (ShiftRotateOperationsByValue_16Bit, SHL_Reg or Register);
                             GenerateInstruction_Byte (ShiftCount);
                           end;
                     else Case ShiftCount of
                            2: begin
                                 GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                                 GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                               end;
                            else begin
                                   Include (UsedRegisters, urCX);
                                   GenerateInstruction_TwoBytes (MOV_8BitReg_Immediate or rCL, ShiftCount);
                                   GenerateInstruction_TwoBytes (ShiftRotateOperationsByCL_16Bit, SHL_Reg or Register);
                                 end;
                          end;
                   end;
            end;
          end else If Factor > 6 then GeneralIntegerMultiplicationWithConstant else
                     begin                          { Factor = 3, 5, 6 }
                       LoadExpressionToRegisters (TUsedRegister (Register));
                       If Factor = 6 then
                         GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                       GenerateInstruction_TwoBytes (MOV_16Bit_Reg_Source, SI__Reg or Register);
                       GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                       If Factor = 5 then
                         GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                       GenerateInstruction_TwoBytes (ADD_16Bit_Dest_Reg, Register or Reg__SI);
                     end;
      end;
end;
```

### Code Block 2

```pascal
Procedure TExpression.IntegerMultiplicationWithConstant (Register: Byte; Factor: Integer);
Var ShiftCount: ShortInt;
    TempFactor: Word;

  Procedure GeneralIntegerMultiplicationWithConstant;
  begin
    Case Instructions80286 in StatementCompilerSwitches of
      True: If Factor = ShortInt (Factor) then
              begin
                GenerateInstructionWithExpressionInMemOrReg (IMUL_Reg_SignExtendImmediateByte, Register shl 3);
                GenerateInstruction_Byte (Factor);
              end else begin
                         GenerateInstructionWithExpressionInMemOrReg (IMUL_Reg_Opearations, Register shl 3);
                         GenerateInstruction_Word (Factor);
                       end;
      else begin
             Include (Usedregisters, urDX);
             If (Location = elRegister) and (LocationData.Register = rAX) then
               begin
                 GenerateInstruction_Byte (MOV_DX_Immediate);
                 GenerateInstruction_Word (Factor);
                 GenerateInstruction_Word (MUL_DX);
                 Exit;
               end else begin
                          GenerateInstruction_Byte (MOV_AX_Immediate);
                          GenerateInstruction_Word (Factor);
                          GenerateInstructionWithExpressionInMemOrReg (Unary_16Bit_Operations, mod_MUL_rm);
                          Register := rAX;
                        end;
           end;
    end;
    Location := elRegister;
    LocationData.Register := Register;
    Include (Usedregisters, TUsedRegister (Register));
  end;

begin
  If Factor <= 0 then GeneralIntegerMultiplicationWithConstant else
    If Factor > 1 then
      begin
        ShiftCount := 0;
        TempFactor := Factor;
        While (TempFactor and $01) = 0 do
          begin
            Inc (ShiftCount);
            TempFactor := TempFactor shr 1;
          end;
        If TempFactor  = 1 then
          begin
            LoadExpressionToRegisters (TUsedRegister (Register));
            Case ShiftCount of
              1: GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
              else Case Instructions80286 in StatementCompilerSwitches of
                     True: begin
                             GenerateInstruction_TwoBytes (ShiftRotateOperationsByValue_16Bit, SHL_Reg or Register);
                             GenerateInstruction_Byte (ShiftCount);
                           end;
                     else Case ShiftCount of
                            2: begin
                                 GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                                 GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                               end;
                            else begin
                                   Include (UsedRegisters, urCX);
                                   GenerateInstruction_TwoBytes (MOV_8BitReg_Immediate or rCL, ShiftCount);
                                   GenerateInstruction_TwoBytes (ShiftRotateOperationsByCL_16Bit, SHL_Reg or Register);
                                 end;
                          end;
                   end;
            end;
          end else If Factor > 6 then GeneralIntegerMultiplicationWithConstant else
                     begin                          { Factor = 3, 5, 6 }
                       LoadExpressionToRegisters (TUsedRegister (Register));
                       If Factor = 6 then
                         GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                       GenerateInstruction_TwoBytes (MOV_16Bit_Reg_Source, SI__Reg or Register);
                       GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                       If Factor = 5 then
                         GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHL_Reg or Register);
                       GenerateInstruction_TwoBytes (ADD_16Bit_Dest_Reg, Register or Reg__SI);
                     end;
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
- https://turbopascal.org/integer-multiplication
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

