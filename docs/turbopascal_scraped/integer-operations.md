# Integer Operations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/integer-operations

---

## Table of Contents

- Integer Operations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Integer Operations
Many operations in Turbo Pascal can be calculated as
integer operations
. This applies to
pointers
,
Boolean values
, enumerated values and
Char
values. Constant integer operations are checked for possible overflow.
Procedure ConstantIntegerOperations;
  Var Overflow: Boolean;
      Result: LongInt;
      BoolResult: LongBool absolute Result;
  begin
    If Operation < Calc_IsEqual then
      begin
        Overflow := False;
        Case Operation of
          Calc_Add:      Result := LeftExpression.Value.Longint + RightExpression.Value.Longint;
          Calc_Subtract: Result := LeftExpression.Value.Longint - RightExpression.Value.Longint;
          Calc_Multiply: Result := LeftExpression.Value.Longint * RightExpression.Value.Longint;
          Calc_DIV: begin
                      If RightExpression.Value.Longint = 0 then Error (DivisionByZero);
                      Result := LeftExpression.Value.Longint div RightExpression.Value.Longint;
                    end;
          Calc_MOD: begin
                      If RightExpression.Value.Longint = 0 then Error (DivisionByZero);
                      Result := LeftExpression.Value.Longint mod RightExpression.Value.Longint;
                    end;
          Calc_SHL: Result := LeftExpression.Value.Longint shl RightExpression.Value.Byte;
          Calc_SHR: Result := LeftExpression.Value.Longint shr RightExpression.Value.Byte;
          Calc_AND: Result := LeftExpression.Value.Longint and RightExpression.Value.Longint;
          Calc_OR:  Result := LeftExpression.Value.Longint  or RightExpression.Value.Longint;
          Calc_XOR: Result := LeftExpression.Value.Longint xor RightExpression.Value.Longint;
        end;
        Asm
          JNO   @1
          INC   Overflow
      @1:
        end;
          If Overflow then Error (OverflowInArithmeticOperation);
          LeftExpression.Value.Longint := Result;
          LowestIntegerType (Result, LeftExpression.DataType);
      end else begin
                 Case Operation of
                   Calc_IsEqual:          BoolResult := LeftExpression.Value.Longint  = RightExpression.Value.Longint;
                   Calc_IsNotEqual:       BoolResult := LeftExpression.Value.Longint <> RightExpression.Value.Longint;
                   Calc_IsGreater:        BoolResult := LeftExpression.Value.Longint  > RightExpression.Value.Longint;
                   Calc_IsLower:          BoolResult := LeftExpression.Value.Longint  < RightExpression.Value.Longint;
                   Calc_IsGreaterOrEqual: BoolResult := LeftExpression.Value.Longint >= RightExpression.Value.Longint;
                   Calc_IsLowerOrEqual:   BoolResult := LeftExpression.Value.Longint <= RightExpression.Value.Longint;
                 end;
                 SetLeftExpressionToConstantBooleanValue (BoolResult);
               end;
  end;
Non-constant integer expressions are prepared a little bit and the operation is performed by general routine for
integer operations
.
Procedure GenerateCodeForIntegerOperations;
  Var CommonDataType: TIntegerTypeSet;
  Label GeneralIntegerOperations;

  begin
    If not ((LeftExpression.Location = elPointerToMemory) and (RightExpression.Location = elConstant)) then
      begin
        If (LeftExpression.Location = elConstant) and (RightExpression.Location = elPointerToMemory) and
             (Operation = Calc_Add) then ExchangeLeftAndRightExpression else GoTo GeneralIntegerOperations;
      end;
    If not (efSegment in LeftExpression.LocationData.Flags) and not (it32Bit in RightExpression.DataType) then
      begin
        Case Operation of
          Calc_Add:      Inc (LeftExpression.Value.Word, RightExpression.Value.Word);
          Calc_Subtract: Dec (LeftExpression.Value.Word, RightExpression.Value.Word);
          else GoTo GeneralIntegerOperations;
        end;
        Exit;
      end;

GeneralIntegerOperations:
    LeftExpression.LoadPointerToMemoryTo_DX_AX;
    RightExpression.LoadPointerToMemoryTo_DX_AX;
    SetLowestCommonIntegerType (LeftExpression.DataType, RightExpression.DataType, CommonDataType);
    If (Operation <= Calc_SHR) and not (it16Bit in CommonDataType) then
      CommonDataType := CommonDataType * [itSigned] + [it16Bit];
    LeftExpression.ExtendInteger (CommonDataType);
    RightExpression.ExtendInteger (CommonDataType);
    IntegerOperations;
  end;
This is the main routine for integer operations. It takes care for some special cases where shorter code can be generated, processes both expressions and generates code for all possible integer operations:
+
,
-
,
*
,
DIV
,
MOD
,
SHL
,
SHR
,
OR
,
XOR
and
AND
.
Procedure IntegerOperations;
  Const IntegerOpCodes: Array [Calc_Add..Calc_XOR] of WordRec = (
          (ByteL: mod_ADD_rm; ByteH: INC_AX),
          (ByteL: mod_SUB_rm; ByteH: DEC_AX),
          (ByteL: Unary_16Bit_Operations; ByteH: mod_MUL_rm),
          (ByteL: Unary_16Bit_Operations; ByteH: mod_IDIV_rm),
          (ByteL: Unary_16Bit_Operations; ByteH: mod_IDIV_rm),
          (ByteL: ShiftRotateOperationsBy1_16Bit; ByteH: SHL_AX),
          (ByteL: ShiftRotateOperationsBy1_16Bit; ByteH: SHR_AX),
          (ByteL: mod_AND_rm),
          (ByteL: mod_OR_rm),
          (ByteL: mod_XOR_rm));

        LongIntOpCodes: Array [Calc_Add..Calc_XOR] of WordRec = (
          (ByteL: ADD_Operations; ByteH: ADC_Operations),
          (ByteL: SUB_Operations; ByteH: SBB_Operations),
          (Word:  SysProc_LongMul),
          (Word:  SysProc_LongDiv),
          (Word:  SysProc_LongDiv),
          (Word:  SysProc_LongShl),
          (Word:  SysProc_LongShr),
          (ByteL: AND_Operations; ByteH: 0),
          (ByteL: OR_Operations;  ByteH: 0),
          (ByteL: XOR_Operations; ByteH: 0));
  begin
    RearrangeOperands;
    If Operation >= Calc_IsEqual then
      begin
        If it32Bit in LeftExpression.DataType then
          GenerateCodeForLongIntComparisons else GenerateCodeForIntegerComparisons;
        LeftExpression.EndIntermediateCodeSubroutine;
        LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
        Exit;
      end;
    Case it32Bit in LeftExpression.DataType of
      True: begin
              LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urDX, urAX]);
              Case Operation of
                Calc_Add,
                Calc_Subtract,
                Calc_AND,
                Calc_OR,
                Calc_XOR: begin
                            RightExpression.GenerateArithmeticInstructionWith_ACC (
                              LongIntOpCodes [Operation].ByteL);
                            RightExpression.SwitchBetweenLoWordAndHiWord (2);
                            Case LongIntOpCodes [Operation].ByteH of
                              0: Case RightExpression.Location of
                                   elConstant: begin
                                                 GenerateInstruction_TwoBytes (ArithmeticOperation_16Bit,
                                                                    Operation_Register_DX or LongIntOpCodes [Operation].ByteL);
                                                 GenerateInstruction_Word (RightExpression.Value.Word);
                                               end;
                                   else RightExpression.GenerateInstructionWithExpressionInMemOrReg (
                                                                 LongIntOpCodes [Operation].ByteL or reg__mod_rm_16Bit, regDX);
                                 end;
                              else begin
                                     RightExpression.GenerateArithmeticInstructionWith_DX (
                                       LongIntOpCodes [Operation].ByteH);
                                     GenerateOverflowCheckingCode (RightExpression)
                                   end;
                            end;
                          end;
                Calc_Multiply: begin
                                 RightExpression.LoadExpressionToRegisters (urCX);
                                 Case OverflowChecking in StatementCompilerSwitches of
                                   True: GenerateInstruction_CALL_FAR (SysProc_LongMulOvf);
                                   else GenerateInstruction_CALL_FAR (SysProc_LongMul);
                                 end;
                               end;
                Calc_DIV,
                Calc_SHL,
                Calc_SHR: begin
                            RightExpression.LoadExpressionToRegisters (urCX);
                            GenerateInstruction_CALL_FAR (LongIntOpCodes [Operation].Word);
                          end;
                Calc_MOD: begin
                            RightExpression.LoadExpressionToRegisters (urCX);
                            GenerateInstruction_CALL_FAR (SysProc_LongDiv);
                            GenerateInstruction_TwoBytes (MOV_16Bit, MOV_AX_CX);
                            GenerateInstruction_TwoBytes (MOV_16Bit, MOV_DX_BX);
                          end;
              end;
            end;
      else begin
             If (Operation = Calc_Multiply) and (RightExpression.Location = elConstant) and
                not (OverflowChecking in StatementCompilerSwitches) then
                  begin
                    LeftExpression.Calculate;
                    LeftExpression.IntegerMultiplicationWithConstant (rAX, RightExpression.Value.Word);
                    LeftExpression.EndIntermediateCodeSubroutine;
                    LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
                    Exit;
                  end;
             Case Operation of
               Calc_DIV,
               Calc_MOD: LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urDX, urAX]);
               else  LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urAX]);
             end;
             Case Operation of
               Calc_Add,
               Calc_Subtract: Case OverflowChecking in StatementCompilerSwitches of
                                True: begin
                                        RightExpression.GenerateArithmeticInstructionWith_ACC (
                                                                             IntegerOpCodes [Operation].ByteL);
                                        GenerateOverflowCheckingCode (RightExpression);
                                      end;
                                else If RightExpression.Location = elConstant then
                                       Case RightExpression.Value.Word of
                                         1: GenerateInstruction_Byte (IntegerOpCodes [Operation].ByteH);
                                         2: begin
                                              GenerateInstruction_Byte (IntegerOpCodes [Operation].ByteH);
                                              GenerateInstruction_Byte (IntegerOpCodes [Operation].ByteH);
                                            end;
                                         else RightExpression.GenerateArithmeticInstructionWith_ACC (
                                                                               IntegerOpCodes [Operation].ByteL);
                                       end else RightExpression.GenerateArithmeticInstructionWith_ACC (
                                                                                 IntegerOpCodes [Operation].ByteL);
                              end;
               Calc_Multiply: Case OverflowChecking in StatementCompilerSwitches of
                                True: begin
                                        If itSigned in RightExpression.DataType then
                                          GenerateCodeForDivisionOrMultiplication (IntegerOpCodes [Operation].ByteL,
                                                                                   mod_IMUL_rm)
                                            else GenerateCodeForDivisionOrMultiplication (IntegerOpCodes [Operation].ByteL,
                                                                                          mod_MUL_rm);
                                        GenerateOverflowCheckingCode (RightExpression);
                                      end;
                                else begin
                                       RightExpression.GenerateInstructionWithExpressionInMemOrReg (
                                         IntegerOpCodes [Operation].ByteL, IntegerOpCodes [Operation].ByteH);
                                       Include (RightExpression.UsedRegisters, urDX);
                                     end;
                              end;
               Calc_DIV: begin
                           If RightExpression.Location = elConstant then
                             begin
                               If RightExpression.Value.Word = 0 then Error (DivisionByZero);
                               IntegerConstantDivision (RightExpression.Value.Word);
                             end else GeneralIntegerDivision;
                         end;
               Calc_MOD: begin
                           GeneralIntegerDivision;
                           GenerateInstruction_Byte (XCHG_DX_AX);
                         end;
               Calc_SHL,
               Calc_SHR: If RightExpression.Location = elConstant then
                           begin
                             If RightExpression.Value.Word = 1 then
                               GenerateInstruction_Word (IntegerOpCodes [Operation].Word) else
                                 Case Instructions80286 in StatementCompilerSwitches of
                                   True: begin
                                           GenerateInstruction_TwoBytes (ShiftRotateOperationsByValue_16Bit,
                                                               IntegerOpCodes [Operation].ByteH);
                                           GenerateInstruction_Byte (RightExpression.Value.Byte);
                                         end;
                                   else begin
                                          RightExpression.LoadExpressionToRegisters (urCX);
                                          GenerateInstruction_TwoBytes (ShiftRotateOperationsByCL_16Bit,
                                                                        IntegerOpCodes [Operation].ByteH);
                                   end;
                                 end;
                           end else begin
                                      RightExpression.LoadExpressionToRegisters (urCX);
                                      GenerateInstruction_TwoBytes (ShiftRotateOperationsByCL_16Bit,
                                                                    IntegerOpCodes [Operation].ByteH);
                                    end;
               Calc_AND,
               Calc_OR,
               Calc_XOR:
                 RightExpression.GenerateArithmeticInstructionWith_ACC (IntegerOpCodes [Operation].ByteL);
             end;
           end;
    end;
    LeftExpression.EndIntermediateCodeSubroutine;
    LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
  end;
In some cases more efficient code can be generated if expressions are in certain registers. This procedure exchanges left and right expressions if such case is identified.
Procedure RearrangeOperands;
    begin
      Case Operation of
        Calc_IsEqual..Calc_Divide,
        Calc_Add,
        Calc_Multiply,
        Calc_AND,
        Calc_OR,
        Calc_XOR:
        else Exit;
      end;
    If RightExpression.Location = elConstant then Exit else
      If LeftExpression.Location = elConstant then ExchangeLeftAndRightExpression else
        If RightExpression.UsedRegisters = [] then Exit else
          If LeftExpression.UsedRegisters = [] then ExchangeLeftAndRightExpression else
            If RightExpression.UsedRegisters = [urBX, urDX, urCX, urAX] then Exit else
              If LeftExpression.UsedRegisters = [urBX, urDX, urCX, urAX] then ExchangeLeftAndRightExpression;
    end;
This procedure exchanges left and right expressions and inverts operation if necessary.
Procedure ExchangeLeftAndRightExpression;
  Var TempExpression: TExpression;
  begin
    TempExpression := LeftExpression;
    LeftExpression := RightExpression;
    RightExpression := TempExpression;
    If Operation in [Calc_IsGreater, Calc_IsLower, Calc_IsGreaterOrEqual, Calc_IsLowerOrEqual] then
      Operation := TCalcOperation (Ord (Operation) xor 1);
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
  Procedure ConstantIntegerOperations;
  Var Overflow: Boolean;
      Result: LongInt;
      BoolResult: LongBool absolute Result;
  begin
    If Operation < Calc_IsEqual then
      begin
        Overflow := False;
        Case Operation of
          Calc_Add:      Result := LeftExpression.Value.Longint + RightExpression.Value.Longint;
          Calc_Subtract: Result := LeftExpression.Value.Longint - RightExpression.Value.Longint;
          Calc_Multiply: Result := LeftExpression.Value.Longint * RightExpression.Value.Longint;
          Calc_DIV: begin
                      If RightExpression.Value.Longint = 0 then Error (DivisionByZero);
                      Result := LeftExpression.Value.Longint div RightExpression.Value.Longint;
                    end;
          Calc_MOD: begin
                      If RightExpression.Value.Longint = 0 then Error (DivisionByZero);
                      Result := LeftExpression.Value.Longint mod RightExpression.Value.Longint;
                    end;
          Calc_SHL: Result := LeftExpression.Value.Longint shl RightExpression.Value.Byte;
          Calc_SHR: Result := LeftExpression.Value.Longint shr RightExpression.Value.Byte;
          Calc_AND: Result := LeftExpression.Value.Longint and RightExpression.Value.Longint;
          Calc_OR:  Result := LeftExpression.Value.Longint  or RightExpression.Value.Longint;
          Calc_XOR: Result := LeftExpression.Value.Longint xor RightExpression.Value.Longint;
        end;
        Asm
          JNO   @1
          INC   Overflow
      @1:
        end;
          If Overflow then Error (OverflowInArithmeticOperation);
          LeftExpression.Value.Longint := Result;
          LowestIntegerType (Result, LeftExpression.DataType);
      end else begin
                 Case Operation of
                   Calc_IsEqual:          BoolResult := LeftExpression.Value.Longint  = RightExpression.Value.Longint;
                   Calc_IsNotEqual:       BoolResult := LeftExpression.Value.Longint <> RightExpression.Value.Longint;
                   Calc_IsGreater:        BoolResult := LeftExpression.Value.Longint  > RightExpression.Value.Longint;
                   Calc_IsLower:          BoolResult := LeftExpression.Value.Longint  < RightExpression.Value.Longint;
                   Calc_IsGreaterOrEqual: BoolResult := LeftExpression.Value.Longint >= RightExpression.Value.Longint;
                   Calc_IsLowerOrEqual:   BoolResult := LeftExpression.Value.Longint <= RightExpression.Value.Longint;
                 end;
                 SetLeftExpressionToConstantBooleanValue (BoolResult);
               end;
  end;
```

### Code Block 2

```pascal
  Procedure GenerateCodeForIntegerOperations;
  Var CommonDataType: TIntegerTypeSet;
  Label GeneralIntegerOperations;

  begin
    If not ((LeftExpression.Location = elPointerToMemory) and (RightExpression.Location = elConstant)) then
      begin
        If (LeftExpression.Location = elConstant) and (RightExpression.Location = elPointerToMemory) and
             (Operation = Calc_Add) then ExchangeLeftAndRightExpression else GoTo GeneralIntegerOperations;
      end;
    If not (efSegment in LeftExpression.LocationData.Flags) and not (it32Bit in RightExpression.DataType) then
      begin
        Case Operation of
          Calc_Add:      Inc (LeftExpression.Value.Word, RightExpression.Value.Word);
          Calc_Subtract: Dec (LeftExpression.Value.Word, RightExpression.Value.Word);
          else GoTo GeneralIntegerOperations;
        end;
        Exit;
      end;

GeneralIntegerOperations:
    LeftExpression.LoadPointerToMemoryTo_DX_AX;
    RightExpression.LoadPointerToMemoryTo_DX_AX;
    SetLowestCommonIntegerType (LeftExpression.DataType, RightExpression.DataType, CommonDataType);
    If (Operation <= Calc_SHR) and not (it16Bit in CommonDataType) then
      CommonDataType := CommonDataType * [itSigned] + [it16Bit];
    LeftExpression.ExtendInteger (CommonDataType);
    RightExpression.ExtendInteger (CommonDataType);
    IntegerOperations;
  end;
```

### Code Block 3

```pascal
  Procedure IntegerOperations;
  Const IntegerOpCodes: Array [Calc_Add..Calc_XOR] of WordRec = (
          (ByteL: mod_ADD_rm; ByteH: INC_AX),
          (ByteL: mod_SUB_rm; ByteH: DEC_AX),
          (ByteL: Unary_16Bit_Operations; ByteH: mod_MUL_rm),
          (ByteL: Unary_16Bit_Operations; ByteH: mod_IDIV_rm),
          (ByteL: Unary_16Bit_Operations; ByteH: mod_IDIV_rm),
          (ByteL: ShiftRotateOperationsBy1_16Bit; ByteH: SHL_AX),
          (ByteL: ShiftRotateOperationsBy1_16Bit; ByteH: SHR_AX),
          (ByteL: mod_AND_rm),
          (ByteL: mod_OR_rm),
          (ByteL: mod_XOR_rm));

        LongIntOpCodes: Array [Calc_Add..Calc_XOR] of WordRec = (
          (ByteL: ADD_Operations; ByteH: ADC_Operations),
          (ByteL: SUB_Operations; ByteH: SBB_Operations),
          (Word:  SysProc_LongMul),
          (Word:  SysProc_LongDiv),
          (Word:  SysProc_LongDiv),
          (Word:  SysProc_LongShl),
          (Word:  SysProc_LongShr),
          (ByteL: AND_Operations; ByteH: 0),
          (ByteL: OR_Operations;  ByteH: 0),
          (ByteL: XOR_Operations; ByteH: 0));
  begin
    RearrangeOperands;
    If Operation >= Calc_IsEqual then
      begin
        If it32Bit in LeftExpression.DataType then
          GenerateCodeForLongIntComparisons else GenerateCodeForIntegerComparisons;
        LeftExpression.EndIntermediateCodeSubroutine;
        LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
        Exit;
      end;
    Case it32Bit in LeftExpression.DataType of
      True: begin
              LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urDX, urAX]);
              Case Operation of
                Calc_Add,
                Calc_Subtract,
                Calc_AND,
                Calc_OR,
                Calc_XOR: begin
                            RightExpression.GenerateArithmeticInstructionWith_ACC (
                              LongIntOpCodes [Operation].ByteL);
                            RightExpression.SwitchBetweenLoWordAndHiWord (2);
                            Case LongIntOpCodes [Operation].ByteH of
                              0: Case RightExpression.Location of
                                   elConstant: begin
                                                 GenerateInstruction_TwoBytes (ArithmeticOperation_16Bit,
                                                                    Operation_Register_DX or LongIntOpCodes [Operation].ByteL);
                                                 GenerateInstruction_Word (RightExpression.Value.Word);
                                               end;
                                   else RightExpression.GenerateInstructionWithExpressionInMemOrReg (
                                                                 LongIntOpCodes [Operation].ByteL or reg__mod_rm_16Bit, regDX);
                                 end;
                              else begin
                                     RightExpression.GenerateArithmeticInstructionWith_DX (
                                       LongIntOpCodes [Operation].ByteH);
                                     GenerateOverflowCheckingCode (RightExpression)
                                   end;
                            end;
                          end;
                Calc_Multiply: begin
                                 RightExpression.LoadExpressionToRegisters (urCX);
                                 Case OverflowChecking in StatementCompilerSwitches of
                                   True: GenerateInstruction_CALL_FAR (SysProc_LongMulOvf);
                                   else GenerateInstruction_CALL_FAR (SysProc_LongMul);
                                 end;
                               end;
                Calc_DIV,
                Calc_SHL,
                Calc_SHR: begin
                            RightExpression.LoadExpressionToRegisters (urCX);
                            GenerateInstruction_CALL_FAR (LongIntOpCodes [Operation].Word);
                          end;
                Calc_MOD: begin
                            RightExpression.LoadExpressionToRegisters (urCX);
                            GenerateInstruction_CALL_FAR (SysProc_LongDiv);
                            GenerateInstruction_TwoBytes (MOV_16Bit, MOV_AX_CX);
                            GenerateInstruction_TwoBytes (MOV_16Bit, MOV_DX_BX);
                          end;
              end;
            end;
      else begin
             If (Operation = Calc_Multiply) and (RightExpression.Location = elConstant) and
                not (OverflowChecking in StatementCompilerSwitches) then
                  begin
                    LeftExpression.Calculate;
                    LeftExpression.IntegerMultiplicationWithConstant (rAX, RightExpression.Value.Word);
                    LeftExpression.EndIntermediateCodeSubroutine;
                    LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
                    Exit;
                  end;
             Case Operation of
               Calc_DIV,
               Calc_MOD: LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urDX, urAX]);
               else  LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urAX]);
             end;
             Case Operation of
               Calc_Add,
               Calc_Subtract: Case OverflowChecking in StatementCompilerSwitches of
                                True: begin
                                        RightExpression.GenerateArithmeticInstructionWith_ACC (
                                                                             IntegerOpCodes [Operation].ByteL);
                                        GenerateOverflowCheckingCode (RightExpression);
                                      end;
                                else If RightExpression.Location = elConstant then
                                       Case RightExpression.Value.Word of
                                         1: GenerateInstruction_Byte (IntegerOpCodes [Operation].ByteH);
                                         2: begin
                                              GenerateInstruction_Byte (IntegerOpCodes [Operation].ByteH);
                                              GenerateInstruction_Byte (IntegerOpCodes [Operation].ByteH);
                                            end;
                                         else RightExpression.GenerateArithmeticInstructionWith_ACC (
                                                                               IntegerOpCodes [Operation].ByteL);
                                       end else RightExpression.GenerateArithmeticInstructionWith_ACC (
                                                                                 IntegerOpCodes [Operation].ByteL);
                              end;
               Calc_Multiply: Case OverflowChecking in StatementCompilerSwitches of
                                True: begin
                                        If itSigned in RightExpression.DataType then
                                          GenerateCodeForDivisionOrMultiplication (IntegerOpCodes [Operation].ByteL,
                                                                                   mod_IMUL_rm)
                                            else GenerateCodeForDivisionOrMultiplication (IntegerOpCodes [Operation].ByteL,
                                                                                          mod_MUL_rm);
                                        GenerateOverflowCheckingCode (RightExpression);
                                      end;
                                else begin
                                       RightExpression.GenerateInstructionWithExpressionInMemOrReg (
                                         IntegerOpCodes [Operation].ByteL, IntegerOpCodes [Operation].ByteH);
                                       Include (RightExpression.UsedRegisters, urDX);
                                     end;
                              end;
               Calc_DIV: begin
                           If RightExpression.Location = elConstant then
                             begin
                               If RightExpression.Value.Word = 0 then Error (DivisionByZero);
                               IntegerConstantDivision (RightExpression.Value.Word);
                             end else GeneralIntegerDivision;
                         end;
               Calc_MOD: begin
                           GeneralIntegerDivision;
                           GenerateInstruction_Byte (XCHG_DX_AX);
                         end;
               Calc_SHL,
               Calc_SHR: If RightExpression.Location = elConstant then
                           begin
                             If RightExpression.Value.Word = 1 then
                               GenerateInstruction_Word (IntegerOpCodes [Operation].Word) else
                                 Case Instructions80286 in StatementCompilerSwitches of
                                   True: begin
                                           GenerateInstruction_TwoBytes (ShiftRotateOperationsByValue_16Bit,
                                                               IntegerOpCodes [Operation].ByteH);
                                           GenerateInstruction_Byte (RightExpression.Value.Byte);
                                         end;
                                   else begin
                                          RightExpression.LoadExpressionToRegisters (urCX);
                                          GenerateInstruction_TwoBytes (ShiftRotateOperationsByCL_16Bit,
                                                                        IntegerOpCodes [Operation].ByteH);
                                   end;
                                 end;
                           end else begin
                                      RightExpression.LoadExpressionToRegisters (urCX);
                                      GenerateInstruction_TwoBytes (ShiftRotateOperationsByCL_16Bit,
                                                                    IntegerOpCodes [Operation].ByteH);
                                    end;
               Calc_AND,
               Calc_OR,
               Calc_XOR:
                 RightExpression.GenerateArithmeticInstructionWith_ACC (IntegerOpCodes [Operation].ByteL);
             end;
           end;
    end;
    LeftExpression.EndIntermediateCodeSubroutine;
    LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
  end;
```

### Code Block 4

```pascal
    Procedure RearrangeOperands;
    begin
      Case Operation of
        Calc_IsEqual..Calc_Divide,
        Calc_Add,
        Calc_Multiply,
        Calc_AND,
        Calc_OR,
        Calc_XOR:
        else Exit;
      end;
    If RightExpression.Location = elConstant then Exit else
      If LeftExpression.Location = elConstant then ExchangeLeftAndRightExpression else
        If RightExpression.UsedRegisters = [] then Exit else
          If LeftExpression.UsedRegisters = [] then ExchangeLeftAndRightExpression else
            If RightExpression.UsedRegisters = [urBX, urDX, urCX, urAX] then Exit else
              If LeftExpression.UsedRegisters = [urBX, urDX, urCX, urAX] then ExchangeLeftAndRightExpression;
    end;
```

### Code Block 5

```pascal
  Procedure ExchangeLeftAndRightExpression;
  Var TempExpression: TExpression;
  begin
    TempExpression := LeftExpression;
    LeftExpression := RightExpression;
    RightExpression := TempExpression;
    If Operation in [Calc_IsGreater, Calc_IsLower, Calc_IsGreaterOrEqual, Calc_IsLowerOrEqual] then
      Operation := TCalcOperation (Ord (Operation) xor 1);
  end;
```

### Code Block 6

```pascal
  Procedure ConstantIntegerOperations;
  Var Overflow: Boolean;
      Result: LongInt;
      BoolResult: LongBool absolute Result;
  begin
    If Operation < Calc_IsEqual then
      begin
        Overflow := False;
        Case Operation of
          Calc_Add:      Result := LeftExpression.Value.Longint + RightExpression.Value.Longint;
          Calc_Subtract: Result := LeftExpression.Value.Longint - RightExpression.Value.Longint;
          Calc_Multiply: Result := LeftExpression.Value.Longint * RightExpression.Value.Longint;
          Calc_DIV: begin
                      If RightExpression.Value.Longint = 0 then Error (DivisionByZero);
                      Result := LeftExpression.Value.Longint div RightExpression.Value.Longint;
                    end;
          Calc_MOD: begin
                      If RightExpression.Value.Longint = 0 then Error (DivisionByZero);
                      Result := LeftExpression.Value.Longint mod RightExpression.Value.Longint;
                    end;
          Calc_SHL: Result := LeftExpression.Value.Longint shl RightExpression.Value.Byte;
          Calc_SHR: Result := LeftExpression.Value.Longint shr RightExpression.Value.Byte;
          Calc_AND: Result := LeftExpression.Value.Longint and RightExpression.Value.Longint;
          Calc_OR:  Result := LeftExpression.Value.Longint  or RightExpression.Value.Longint;
          Calc_XOR: Result := LeftExpression.Value.Longint xor RightExpression.Value.Longint;
        end;
        Asm
          JNO   @1
          INC   Overflow
      @1:
        end;
          If Overflow then Error (OverflowInArithmeticOperation);
          LeftExpression.Value.Longint := Result;
          LowestIntegerType (Result, LeftExpression.DataType);
      end else begin
                 Case Operation of
                   Calc_IsEqual:          BoolResult := LeftExpression.Value.Longint  = RightExpression.Value.Longint;
                   Calc_IsNotEqual:       BoolResult := LeftExpression.Value.Longint <> RightExpression.Value.Longint;
                   Calc_IsGreater:        BoolResult := LeftExpression.Value.Longint  > RightExpression.Value.Longint;
                   Calc_IsLower:          BoolResult := LeftExpression.Value.Longint  < RightExpression.Value.Longint;
                   Calc_IsGreaterOrEqual: BoolResult := LeftExpression.Value.Longint >= RightExpression.Value.Longint;
                   Calc_IsLowerOrEqual:   BoolResult := LeftExpression.Value.Longint <= RightExpression.Value.Longint;
                 end;
                 SetLeftExpressionToConstantBooleanValue (BoolResult);
               end;
  end;
```

### Code Block 7

```pascal
  Procedure GenerateCodeForIntegerOperations;
  Var CommonDataType: TIntegerTypeSet;
  Label GeneralIntegerOperations;

  begin
    If not ((LeftExpression.Location = elPointerToMemory) and (RightExpression.Location = elConstant)) then
      begin
        If (LeftExpression.Location = elConstant) and (RightExpression.Location = elPointerToMemory) and
             (Operation = Calc_Add) then ExchangeLeftAndRightExpression else GoTo GeneralIntegerOperations;
      end;
    If not (efSegment in LeftExpression.LocationData.Flags) and not (it32Bit in RightExpression.DataType) then
      begin
        Case Operation of
          Calc_Add:      Inc (LeftExpression.Value.Word, RightExpression.Value.Word);
          Calc_Subtract: Dec (LeftExpression.Value.Word, RightExpression.Value.Word);
          else GoTo GeneralIntegerOperations;
        end;
        Exit;
      end;

GeneralIntegerOperations:
    LeftExpression.LoadPointerToMemoryTo_DX_AX;
    RightExpression.LoadPointerToMemoryTo_DX_AX;
    SetLowestCommonIntegerType (LeftExpression.DataType, RightExpression.DataType, CommonDataType);
    If (Operation <= Calc_SHR) and not (it16Bit in CommonDataType) then
      CommonDataType := CommonDataType * [itSigned] + [it16Bit];
    LeftExpression.ExtendInteger (CommonDataType);
    RightExpression.ExtendInteger (CommonDataType);
    IntegerOperations;
  end;
```

### Code Block 8

```pascal
  Procedure IntegerOperations;
  Const IntegerOpCodes: Array [Calc_Add..Calc_XOR] of WordRec = (
          (ByteL: mod_ADD_rm; ByteH: INC_AX),
          (ByteL: mod_SUB_rm; ByteH: DEC_AX),
          (ByteL: Unary_16Bit_Operations; ByteH: mod_MUL_rm),
          (ByteL: Unary_16Bit_Operations; ByteH: mod_IDIV_rm),
          (ByteL: Unary_16Bit_Operations; ByteH: mod_IDIV_rm),
          (ByteL: ShiftRotateOperationsBy1_16Bit; ByteH: SHL_AX),
          (ByteL: ShiftRotateOperationsBy1_16Bit; ByteH: SHR_AX),
          (ByteL: mod_AND_rm),
          (ByteL: mod_OR_rm),
          (ByteL: mod_XOR_rm));

        LongIntOpCodes: Array [Calc_Add..Calc_XOR] of WordRec = (
          (ByteL: ADD_Operations; ByteH: ADC_Operations),
          (ByteL: SUB_Operations; ByteH: SBB_Operations),
          (Word:  SysProc_LongMul),
          (Word:  SysProc_LongDiv),
          (Word:  SysProc_LongDiv),
          (Word:  SysProc_LongShl),
          (Word:  SysProc_LongShr),
          (ByteL: AND_Operations; ByteH: 0),
          (ByteL: OR_Operations;  ByteH: 0),
          (ByteL: XOR_Operations; ByteH: 0));
  begin
    RearrangeOperands;
    If Operation >= Calc_IsEqual then
      begin
        If it32Bit in LeftExpression.DataType then
          GenerateCodeForLongIntComparisons else GenerateCodeForIntegerComparisons;
        LeftExpression.EndIntermediateCodeSubroutine;
        LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
        Exit;
      end;
    Case it32Bit in LeftExpression.DataType of
      True: begin
              LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urDX, urAX]);
              Case Operation of
                Calc_Add,
                Calc_Subtract,
                Calc_AND,
                Calc_OR,
                Calc_XOR: begin
                            RightExpression.GenerateArithmeticInstructionWith_ACC (
                              LongIntOpCodes [Operation].ByteL);
                            RightExpression.SwitchBetweenLoWordAndHiWord (2);
                            Case LongIntOpCodes [Operation].ByteH of
                              0: Case RightExpression.Location of
                                   elConstant: begin
                                                 GenerateInstruction_TwoBytes (ArithmeticOperation_16Bit,
                                                                    Operation_Register_DX or LongIntOpCodes [Operation].ByteL);
                                                 GenerateInstruction_Word (RightExpression.Value.Word);
                                               end;
                                   else RightExpression.GenerateInstructionWithExpressionInMemOrReg (
                                                                 LongIntOpCodes [Operation].ByteL or reg__mod_rm_16Bit, regDX);
                                 end;
                              else begin
                                     RightExpression.GenerateArithmeticInstructionWith_DX (
                                       LongIntOpCodes [Operation].ByteH);
                                     GenerateOverflowCheckingCode (RightExpression)
                                   end;
                            end;
                          end;
                Calc_Multiply: begin
                                 RightExpression.LoadExpressionToRegisters (urCX);
                                 Case OverflowChecking in StatementCompilerSwitches of
                                   True: GenerateInstruction_CALL_FAR (SysProc_LongMulOvf);
                                   else GenerateInstruction_CALL_FAR (SysProc_LongMul);
                                 end;
                               end;
                Calc_DIV,
                Calc_SHL,
                Calc_SHR: begin
                            RightExpression.LoadExpressionToRegisters (urCX);
                            GenerateInstruction_CALL_FAR (LongIntOpCodes [Operation].Word);
                          end;
                Calc_MOD: begin
                            RightExpression.LoadExpressionToRegisters (urCX);
                            GenerateInstruction_CALL_FAR (SysProc_LongDiv);
                            GenerateInstruction_TwoBytes (MOV_16Bit, MOV_AX_CX);
                            GenerateInstruction_TwoBytes (MOV_16Bit, MOV_DX_BX);
                          end;
              end;
            end;
      else begin
             If (Operation = Calc_Multiply) and (RightExpression.Location = elConstant) and
                not (OverflowChecking in StatementCompilerSwitches) then
                  begin
                    LeftExpression.Calculate;
                    LeftExpression.IntegerMultiplicationWithConstant (rAX, RightExpression.Value.Word);
                    LeftExpression.EndIntermediateCodeSubroutine;
                    LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
                    Exit;
                  end;
             Case Operation of
               Calc_DIV,
               Calc_MOD: LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urDX, urAX]);
               else  LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urAX]);
             end;
             Case Operation of
               Calc_Add,
               Calc_Subtract: Case OverflowChecking in StatementCompilerSwitches of
                                True: begin
                                        RightExpression.GenerateArithmeticInstructionWith_ACC (
                                                                             IntegerOpCodes [Operation].ByteL);
                                        GenerateOverflowCheckingCode (RightExpression);
                                      end;
                                else If RightExpression.Location = elConstant then
                                       Case RightExpression.Value.Word of
                                         1: GenerateInstruction_Byte (IntegerOpCodes [Operation].ByteH);
                                         2: begin
                                              GenerateInstruction_Byte (IntegerOpCodes [Operation].ByteH);
                                              GenerateInstruction_Byte (IntegerOpCodes [Operation].ByteH);
                                            end;
                                         else RightExpression.GenerateArithmeticInstructionWith_ACC (
                                                                               IntegerOpCodes [Operation].ByteL);
                                       end else RightExpression.GenerateArithmeticInstructionWith_ACC (
                                                                                 IntegerOpCodes [Operation].ByteL);
                              end;
               Calc_Multiply: Case OverflowChecking in StatementCompilerSwitches of
                                True: begin
                                        If itSigned in RightExpression.DataType then
                                          GenerateCodeForDivisionOrMultiplication (IntegerOpCodes [Operation].ByteL,
                                                                                   mod_IMUL_rm)
                                            else GenerateCodeForDivisionOrMultiplication (IntegerOpCodes [Operation].ByteL,
                                                                                          mod_MUL_rm);
                                        GenerateOverflowCheckingCode (RightExpression);
                                      end;
                                else begin
                                       RightExpression.GenerateInstructionWithExpressionInMemOrReg (
                                         IntegerOpCodes [Operation].ByteL, IntegerOpCodes [Operation].ByteH);
                                       Include (RightExpression.UsedRegisters, urDX);
                                     end;
                              end;
               Calc_DIV: begin
                           If RightExpression.Location = elConstant then
                             begin
                               If RightExpression.Value.Word = 0 then Error (DivisionByZero);
                               IntegerConstantDivision (RightExpression.Value.Word);
                             end else GeneralIntegerDivision;
                         end;
               Calc_MOD: begin
                           GeneralIntegerDivision;
                           GenerateInstruction_Byte (XCHG_DX_AX);
                         end;
               Calc_SHL,
               Calc_SHR: If RightExpression.Location = elConstant then
                           begin
                             If RightExpression.Value.Word = 1 then
                               GenerateInstruction_Word (IntegerOpCodes [Operation].Word) else
                                 Case Instructions80286 in StatementCompilerSwitches of
                                   True: begin
                                           GenerateInstruction_TwoBytes (ShiftRotateOperationsByValue_16Bit,
                                                               IntegerOpCodes [Operation].ByteH);
                                           GenerateInstruction_Byte (RightExpression.Value.Byte);
                                         end;
                                   else begin
                                          RightExpression.LoadExpressionToRegisters (urCX);
                                          GenerateInstruction_TwoBytes (ShiftRotateOperationsByCL_16Bit,
                                                                        IntegerOpCodes [Operation].ByteH);
                                   end;
                                 end;
                           end else begin
                                      RightExpression.LoadExpressionToRegisters (urCX);
                                      GenerateInstruction_TwoBytes (ShiftRotateOperationsByCL_16Bit,
                                                                    IntegerOpCodes [Operation].ByteH);
                                    end;
               Calc_AND,
               Calc_OR,
               Calc_XOR:
                 RightExpression.GenerateArithmeticInstructionWith_ACC (IntegerOpCodes [Operation].ByteL);
             end;
           end;
    end;
    LeftExpression.EndIntermediateCodeSubroutine;
    LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
  end;
```

### Code Block 9

```pascal
    Procedure RearrangeOperands;
    begin
      Case Operation of
        Calc_IsEqual..Calc_Divide,
        Calc_Add,
        Calc_Multiply,
        Calc_AND,
        Calc_OR,
        Calc_XOR:
        else Exit;
      end;
    If RightExpression.Location = elConstant then Exit else
      If LeftExpression.Location = elConstant then ExchangeLeftAndRightExpression else
        If RightExpression.UsedRegisters = [] then Exit else
          If LeftExpression.UsedRegisters = [] then ExchangeLeftAndRightExpression else
            If RightExpression.UsedRegisters = [urBX, urDX, urCX, urAX] then Exit else
              If LeftExpression.UsedRegisters = [urBX, urDX, urCX, urAX] then ExchangeLeftAndRightExpression;
    end;
```

### Code Block 10

```pascal
  Procedure ExchangeLeftAndRightExpression;
  Var TempExpression: TExpression;
  begin
    TempExpression := LeftExpression;
    LeftExpression := RightExpression;
    RightExpression := TempExpression;
    If Operation in [Calc_IsGreater, Calc_IsLower, Calc_IsGreaterOrEqual, Calc_IsLowerOrEqual] then
      Operation := TCalcOperation (Ord (Operation) xor 1);
  end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/boolean-operations
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/expressions
- https://turbopascal.org/integer-operations
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/pointer-operations
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

