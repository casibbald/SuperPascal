# Set Operations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/set-operations

---

## Table of Contents

- Set Operations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Set Operations
Constant sets are located into temprary buffer and expression
Value
holds offset to this buffer. Non-constant sets are always expanded to the
stack frame
. This procedure calculates operation between two constant sets.
Procedure ConstantSetOperations;
  Var LeftSet, RightSet: PSetOfByte;
  begin
    LeftSet  := Ptr (DSeg, LeftExpression.Value.Word);
    RightSet := Ptr (DSeg, RightExpression.Value.Word);
    Case Operation of
      Calc_Add:              LeftSet^ := LeftSet^ + RightSet^;
      Calc_Subtract:         LeftSet^ := LeftSet^ - RightSet^;
      Calc_Multiply:         LeftSet^ := LeftSet^ * RightSet^;
      Calc_IsEqual:          LeftExpression.SetConstantBooleanExpression (LeftSet^  = RightSet^);
      Calc_IsNotEqual:       LeftExpression.SetConstantBooleanExpression (LeftSet^ <> RightSet^);
      Calc_IsLowerOrEqual:   LeftExpression.SetConstantBooleanExpression (LeftSet^ <= RightSet^);
      Calc_IsGreaterOrEqual: LeftExpression.SetConstantBooleanExpression (LeftSet^ >= RightSet^);
    end;
    Case Operation of
      Calc_Add,
      Calc_Subtract,
      Calc_Multiply: LeftExpression.SetDataTypeOfSet;
    end;
  end;
This procedure checks expression for small set and sets its data type accordingly.
Procedure TExpression.SetDataTypeOfSet;
Var SetAddress: PWord;
    First16: Word;
    N: Byte;
    SizeType: TIntegerTypeSet;
    Zero: Boolean;
begin
  SetAddress := Ptr (DSeg, Value.Word);
  First16 := SetAddress^;
  SizeType := [it32Bytes];
  Zero := True;
  For N := 1 to 15 do
    begin
      Inc (SetAddress);
      If SetAddress^ <> 0 then Zero := False;
    end;
  If Zero then
    Case Hi (First16) of
      0: SizeType := [itUnsigned];
      else SizeType := [itUnsigned,it16Bit];
    end;
  DataType := SizeType;
end;
This procedure generates code for operation between sets: union (
+
), intersection (
*
), difference (
-
) and relations (
<=
,
>=
).
Procedure GenerateCodeForSetOperations;
  Var CommonSetType: TIntegerTypeSet;
      TempExpression: TExpression;
  begin
    CommonSetType := LeftExpression.DataType + RightExpression.DataType;
    If not (it32Bytes in CommonSetType) then
      begin
        LeftExpression.LoadConstantSmallSetIntoValue;
        LeftExpression.ExtendInteger (CommonSetType);
        RightExpression.LoadConstantSmallSetIntoValue;
        RightExpression.ExtendInteger (CommonSetType);
        Case Operation of
          Calc_Subtract,
          Calc_IsLowerOrEqual: RightExpression.NOT_Integer;
          Calc_IsGreaterOrEqual: begin
                                   ExchangeLeftAndRightExpression;
                                   RightExpression.NOT_Integer;
                                 end;
        end;
        Case Operation of
          Calc_Add: Operation := Calc_OR;
          Calc_Subtract,
          Calc_Multiply: Operation := Calc_AND;
          Calc_IsLowerOrEqual: begin
                                 Operation := Calc_AND;
                                 IntegerOperations;
                                 LeftExpression.SetExpressionToBooleanJump (JE);
                                 Exit;
                               end;
        end;
        IntegerOperations;
        Exit;
      end;
    Case Operation of
      Calc_IsEqual..Calc_IsLowerOrEqual: LeftExpression.ExpandSetToStackFrameAndPushPointer (32);
      else                               LeftExpression.ExpandSetToStackFrameAndPushPointer (0);
    end;
    RightExpression.ExpandSetToStackFrameAndPushPointer (32);
    If Operation = Calc_IsLowerOrEqual then
      begin
        TempExpression := LeftExpression;
        LeftExpression := RightExpression;
        RightExpression := TempExpression;
      end;
    ConvertToPointerAndPushBothExpressions (LeftExpression, RightExpression);
    CalculateBothExpressions (LeftExpression, RightExpression);
    If Operation = Calc_IsLowerOrEqual then
      begin
        TempExpression  := LeftExpression;
        LeftExpression  := RightExpression;
        RightExpression := TempExpression;
      end;
    Case Operation of
      Calc_Add,
      Calc_Subtract,
      Calc_Multiply: begin
                       Case Operation of
                         Calc_Add:      GenerateInstruction_CALL_FAR (SysProc_ZUnion);
                         Calc_Subtract: GenerateInstruction_CALL_FAR (SysProc_ZDifference);
                         Calc_Multiply: GenerateInstruction_CALL_FAR (SysProc_ZIntersect);
                       end;
                       LeftExpression.EndIntermediateCodeSubroutine;
                       LeftExpression.UsedRegisters := [urBX, urDX, urCX, urAX];
                     end;
      else begin
             Case Operation of
               Calc_IsEqual,
               Calc_IsNotEqual:     GenerateInstruction_CALL_FAR (SysProc_ZEqual);
               Calc_IsGreaterOrEqual,
               Calc_IsLowerOrEqual: GenerateInstruction_CALL_FAR (SysProc_ZRelation);
             end;
             LeftExpression.EndIntermediateCodeSubroutine;
             LeftExpression.UsedRegisters := [urBX, urDX, urCX, urAX];
             Case Operation of
               Calc_IsNotEqual: LeftExpression.SetExpressionToBooleanJump (JNE);
               else             LeftExpression.SetExpressionToBooleanJump (JE);
             end;
          end;
    end;
  end;
This procedure loads small set (16 elements or less) into
Value.Word
from memory at
Value.Pointer
.
Procedure TExpression.LoadConstantSmallSetIntoValue;
begin
  If Location = elConstant then Value.LongInt := Word (Ptr (DSeg, Value.Word)^);
end;
This procedure generates code for operator
IN
. If set is constant it generates special code to compare ranges of set elements.
Procedure GenerateCodeForOperator_IN (Var ElementExpression, SetExpression: TExpression);
Var ExpressionSet: PSetOfByte;
    Index, StartIndex, N, FoundJumpOpCode: Byte;
    _8_OrMoreElementRanges: Boolean;

  Function SearchForSetElementRange: Boolean;
  begin
    SearchForSetElementRange := False;
    Repeat
      If Index in ExpressionSet^ then Break;
      Inc (Index);
      If Index = 0 then Exit;
    until False;
    StartIndex := Index;
    Repeat
      If not (Index in ExpressionSet^) then Break;
      Inc (Index);
    until Index = 0;
    SearchForSetElementRange := True;
  end;

begin
  CheckElementAndSetCompatibility (ElementExpression, SetExpression);
  ElementExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Byte_TypeOffset));
  ExpressionSet := PSetOfByte (Ptr (DSeg, SetExpression.Value.Word));
  If SetExpression.Location <> elConstant then
    begin
      OperationWithSet (ElementExpression, SetExpression, $01, TEST_Memory_AL, TEST_Memory_ImmediateByte);
      Exit;
    end;
  If ElementExpression.Location = elConstant then
    begin
      ElementExpression.SetConstantBooleanExpression (ElementExpression.Value.Byte in ExpressionSet^);
      Exit;
    end;
  Index := 0;             { Set is known at compile time }
  If not SearchForSetElementRange then
    begin
      ElementExpression.SetConstantBooleanExpression (False);
      Exit;
    end;
  If Index or StartIndex = 0 then
    begin
      ElementExpression.SetConstantBooleanExpression (True);
      Exit;
    end;
  _8_OrMoreElementRanges := True;
  For N := 1 to 8 do
    If (Index = 0) or (not SearchForSetElementRange) then
      begin
        _8_OrMoreElementRanges := False;
        Break;
      end;
  If _8_OrMoreElementRanges then
    begin
      SetExpression.ExpandSetToStackFrameAndPushPointer (32);
      OperationWithSet (ElementExpression, SetExpression, $01, TEST_Memory_AL, TEST_Memory_ImmediateByte);
      Exit;
    end;
  ElementExpression.Calculate;
  ElementExpression.LoadExpressionToRegisters (urAX);
  With ElementExpression do
    begin
      Location := elBoolean;
      DataType := [];
      Value.ShortCircuitJumps := 0;
    end;
  Index := 0;
  FoundJumpOpCode := 0;
  While SearchForSetElementRange do
    begin
      Repeat
        If FoundJumpOpCode <> 0 then GenerateCodeForNearJump (ElementExpression.Value.LastJumpToTrue, FoundJumpOpCode);
        Dec (Index);
        If Index <> StartIndex then
          begin
            If StartIndex <> 0 then
              begin
                GenerateInstruction_TwoBytes (CMP_AL_Immediate, StartIndex);
                FoundJumpOpCode := JNB;
                If Index = $FF then Break;
                GenerateCodeForNearJump (ElementExpression.Value.LastJumpToFalse, JB);
              end;
            GenerateInstruction_TwoBytes (CMP_AL_Immediate, Index);
            FoundJumpOpCode := JBE;
            Break;
          end;
        GenerateInstruction_TwoBytes (CMP_AL_Immediate, StartIndex);
        FoundJumpOpCode := JE;
      until True;
      Inc (Index);
      If Index = 0 then Break;
    end;
  With ElementExpression do
    begin
      LocationData.JumpIfTrueOpCode := FoundJumpOpCode;
      ElementExpression.EndIntermediateCodeSubroutine;
      TypeDefPtr := Ptr (SystemUnitSegment, Boolean_TypeOffset);
    end;
end;
This procedure checks compatibility between element type and set element type and reports error if they don’t match.
Procedure CheckElementAndSetCompatibility (Var ElementExpression, SetExpression: TExpression);
begin
  If ElementExpression.TypeDefPtr^.BaseType < btInteger then Error (OperandTypesDoNotMatchOperator);
  If SetExpression.TypeDefPtr^.BaseType <> btSet then Error (OperandTypesDoNotMatchOperator);
  If PSetTypeDefinition (SetExpression.TypeDefPtr)^.BaseSetTypeOffset.TypeOffset <> 0 then
    begin
      If PointerFromOffsets (POrdinalTypeDefinition (ElementExpression.TypeDefPtr)^.OrdinalType) <>
    PointerFromOffsets (POrdinalTypeDefinition (
    PointerFromOffsets (PSetTypeDefinition (SetExpression.TypeDefPtr)^.BaseSetTypeOffset))^.OrdinalType) then
           Error (TypeMismatch);
    end;
end;
This procedure generates code with general non-constant set operation in memory.
Procedure OperationWithSet (Var ElementExpression, SetExpression: TExpression; Mask, TestMemoryRegisterOpCode: Byte; TestMemoryImmediateOpCode: Word);
Var SetSize: Byte;
    SetElementLowerLimit_div_8: Byte;
    MaskWord: WordRec;

  Function RegisterSetSizeInBits: Byte;
  begin
    If it16Bit in SetExpression.DataType then RegisterSetSizeInBits := 16 else RegisterSetSizeInBits := 8;
  end;


  Procedure GetSetSizeInMemory;
  begin
    If SetExpression.Location = elMemory then
      SetSize :=
        PSetTypeDefinition (SetExpression.TypeDefPtr)^.GetSetSizeAndLowestElementDataOffset (SetElementLowerLimit_div_8)
        else begin
               PopPointerAndConvertToMemory (SetExpression);
               SetSize := 32;
               SetElementLowerLimit_div_8 := 0;
             end;
  end;

begin
  TestSetInForElementMask := Mask;
  TestMemoryWithRegisterOpCode := TestMemoryRegisterOpCode;
  TestMemoryWithImmediateOpCode := TestMemoryImmediateOpCode;
  LastJumpToFalse := 0;
  Case ElementExpression.Location of
    elConstant: begin
                  Case SetExpression.Location of
                    elRegister: begin
                                  If ElementExpression.Value.Byte >= RegisterSetSizeInBits then
                                    begin
                                      ElementExpression.SetConstantBooleanExpression (False);
                                      Exit;
                                    end;
                                  SetExpression.Calculate;
                                  Case it16Bit in SetExpression.DataType of
                                    True: begin
                                            GenerateInstruction_Byte (TEST_AX_Immediate);
                                            GenerateInstruction_Word ($0001 shl ElementExpression.Value.Byte);
                                          end;
                                    else begin
                                           GenerateInstruction_Byte (TEST_AL_Immediate);
                                           GenerateInstruction_Byte ($01 shl ElementExpression.Value.Byte)
                                         end;
                                  end;
                                end;
                    else begin
                           GetSetSizeInMemory;
                           If (ElementExpression.Value.Byte shr 3 < SetElementLowerLimit_div_8) or
                              (ElementExpression.Value.Byte shr 3 - SetElementLowerLimit_div_8 >= SetSize) then
                             begin
                               ElementExpression.SetConstantBooleanExpression (False);
                               Exit;
                             end;
                           Inc (SetExpression.Value.Word, ElementExpression.Value.Byte shr 3 - SetElementLowerLimit_div_8);
                           SetExpression.Calculate;
                           SetExpression.GenerateInstructionWithExpressionInMemOrReg (
                             Lo (TestMemoryWithImmediateOpCode), Hi (TestMemoryWithImmediateOpCode));
                           MaskWord.Word := TestSetInForElementMask shl (ElementExpression.Value.Byte and $07);
                           GenerateInstruction_Byte (MaskWord.ByteL or MaskWord.ByteH);
                         end;
                  end;

                end;
    else begin
           Case it32Bytes in SetExpression.DataType of
             True: begin
                     GetSetSizeInMemory;
                     ElementExpression.Calculate;
                     ElementExpression.LoadExpressionToRegisters (urAX);
                     GenerateInstruction_TwoBytes (MOV_AH_Immediate, TestSetInForElementMask);
                     GenerateInstruction_Byte (MOV_DX_Immediate);
                     GenerateInstruction_TwoBytes (SetSize, SetElementLowerLimit_div_8);
                     GenerateInstruction_CALL_FAR (SysProc_ZBitMask);
                     With ElementExpression do
                       begin
                         DataType := [it32Bit, it16Bit];
                         UsedRegisters := [urBX, urDX, urCX, urAX];
                       end;
                   end;
             else begin
                    ElementExpression.Calculate;
                    ElementExpression.LoadExpressionToRegisters (urCX);
                    GenerateInstruction_Word (CMP_CL_Immediate);
                    GenerateInstruction_Byte (RegisterSetSizeInBits);
                    GenerateCodeForNearJump (LastJumpToFalse, JNB);
                    If it16Bit in SetExpression.DataType then
                       begin
                         GenerateInstruction_Byte (MOV_AX_Immediate);
                         GenerateInstruction_Word (Integer (ShortInt (TestSetInForElementMask)));
                         GenerateInstruction_Word (ROL_AX_CL);
                       end else begin
                                  GenerateInstruction_TwoBytes (MOV_AL_Immediate, TestSetInForElementMask);
                                  GenerateInstruction_Word (ROL_AL_CL);
                                end;
                    With ElementExpression do
                      begin
                        DataType := SetExpression.DataType;
                        Include (UsedRegisters, urAX);
                        LocationData.Flags := [];
                      end
                  end;
           end;
           ElementExpression.Save (SetExpression.UsedRegisters);
           SetExpression.Calculate;
           ElementExpression.PopToRegisters ([urAX]);
           With SetExpression do
             If it32Bytes in DataType then
               begin
                 DataType := [itUnsigned];
                 Case ofsDI in LocationData.Flags of
                   True: GenerateInstruction_Word (ADD_DI_Register or (rDX or ElementExpression.LocationData.Register) shl 8);
                   else begin
                          Include (SetExpression.LocationData.Flags, ofsDI);
                          GenerateInstruction_Word (MOV_DI_Register or
                                                    (rDX or ElementExpression.LocationData.Register) shl 8);
                        end;
                 end;
                 ES_DI_PointerDestroyed;
               end;
           SetExpression.GenerateInstruction_8_16_bit (TestMemoryWithRegisterOpCode,
                                                              ElementExpression.LocationData.Register shl 3);
         end;
  end;
  With ElementExpression do
    begin
      ElementExpression.EndIntermediateCodeSubroutine;
      ElementExpression.SetExpressionToBooleanJump (JNE);
      Value.LastJumpToFalse := LastJumpToFalse;
      UsedRegisters := UsedRegisters + SetExpression.UsedRegisters;
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
  Procedure ConstantSetOperations;
  Var LeftSet, RightSet: PSetOfByte;
  begin
    LeftSet  := Ptr (DSeg, LeftExpression.Value.Word);
    RightSet := Ptr (DSeg, RightExpression.Value.Word);
    Case Operation of
      Calc_Add:              LeftSet^ := LeftSet^ + RightSet^;
      Calc_Subtract:         LeftSet^ := LeftSet^ - RightSet^;
      Calc_Multiply:         LeftSet^ := LeftSet^ * RightSet^;
      Calc_IsEqual:          LeftExpression.SetConstantBooleanExpression (LeftSet^  = RightSet^);
      Calc_IsNotEqual:       LeftExpression.SetConstantBooleanExpression (LeftSet^ <> RightSet^);
      Calc_IsLowerOrEqual:   LeftExpression.SetConstantBooleanExpression (LeftSet^ <= RightSet^);
      Calc_IsGreaterOrEqual: LeftExpression.SetConstantBooleanExpression (LeftSet^ >= RightSet^);
    end;
    Case Operation of
      Calc_Add,
      Calc_Subtract,
      Calc_Multiply: LeftExpression.SetDataTypeOfSet;
    end;
  end;
```

### Code Block 2

```pascal
Procedure TExpression.SetDataTypeOfSet;
Var SetAddress: PWord;
    First16: Word;
    N: Byte;
    SizeType: TIntegerTypeSet;
    Zero: Boolean;
begin
  SetAddress := Ptr (DSeg, Value.Word);
  First16 := SetAddress^;
  SizeType := [it32Bytes];
  Zero := True;
  For N := 1 to 15 do
    begin
      Inc (SetAddress);
      If SetAddress^ <> 0 then Zero := False;
    end;
  If Zero then
    Case Hi (First16) of
      0: SizeType := [itUnsigned];
      else SizeType := [itUnsigned,it16Bit];
    end;
  DataType := SizeType;
end;
```

### Code Block 3

```pascal
  Procedure GenerateCodeForSetOperations;
  Var CommonSetType: TIntegerTypeSet;
      TempExpression: TExpression;
  begin
    CommonSetType := LeftExpression.DataType + RightExpression.DataType;
    If not (it32Bytes in CommonSetType) then
      begin
        LeftExpression.LoadConstantSmallSetIntoValue;
        LeftExpression.ExtendInteger (CommonSetType);
        RightExpression.LoadConstantSmallSetIntoValue;
        RightExpression.ExtendInteger (CommonSetType);
        Case Operation of
          Calc_Subtract,
          Calc_IsLowerOrEqual: RightExpression.NOT_Integer;
          Calc_IsGreaterOrEqual: begin
                                   ExchangeLeftAndRightExpression;
                                   RightExpression.NOT_Integer;
                                 end;
        end;
        Case Operation of
          Calc_Add: Operation := Calc_OR;
          Calc_Subtract,
          Calc_Multiply: Operation := Calc_AND;
          Calc_IsLowerOrEqual: begin
                                 Operation := Calc_AND;
                                 IntegerOperations;
                                 LeftExpression.SetExpressionToBooleanJump (JE);
                                 Exit;
                               end;
        end;
        IntegerOperations;
        Exit;
      end;
    Case Operation of
      Calc_IsEqual..Calc_IsLowerOrEqual: LeftExpression.ExpandSetToStackFrameAndPushPointer (32);
      else                               LeftExpression.ExpandSetToStackFrameAndPushPointer (0);
    end;
    RightExpression.ExpandSetToStackFrameAndPushPointer (32);
    If Operation = Calc_IsLowerOrEqual then
      begin
        TempExpression := LeftExpression;
        LeftExpression := RightExpression;
        RightExpression := TempExpression;
      end;
    ConvertToPointerAndPushBothExpressions (LeftExpression, RightExpression);
    CalculateBothExpressions (LeftExpression, RightExpression);
    If Operation = Calc_IsLowerOrEqual then
      begin
        TempExpression  := LeftExpression;
        LeftExpression  := RightExpression;
        RightExpression := TempExpression;
      end;
    Case Operation of
      Calc_Add,
      Calc_Subtract,
      Calc_Multiply: begin
                       Case Operation of
                         Calc_Add:      GenerateInstruction_CALL_FAR (SysProc_ZUnion);
                         Calc_Subtract: GenerateInstruction_CALL_FAR (SysProc_ZDifference);
                         Calc_Multiply: GenerateInstruction_CALL_FAR (SysProc_ZIntersect);
                       end;
                       LeftExpression.EndIntermediateCodeSubroutine;
                       LeftExpression.UsedRegisters := [urBX, urDX, urCX, urAX];
                     end;
      else begin
             Case Operation of
               Calc_IsEqual,
               Calc_IsNotEqual:     GenerateInstruction_CALL_FAR (SysProc_ZEqual);
               Calc_IsGreaterOrEqual,
               Calc_IsLowerOrEqual: GenerateInstruction_CALL_FAR (SysProc_ZRelation);
             end;
             LeftExpression.EndIntermediateCodeSubroutine;
             LeftExpression.UsedRegisters := [urBX, urDX, urCX, urAX];
             Case Operation of
               Calc_IsNotEqual: LeftExpression.SetExpressionToBooleanJump (JNE);
               else             LeftExpression.SetExpressionToBooleanJump (JE);
             end;
          end;
    end;
  end;
```

### Code Block 4

```pascal
Procedure TExpression.LoadConstantSmallSetIntoValue;
begin
  If Location = elConstant then Value.LongInt := Word (Ptr (DSeg, Value.Word)^);
end;
```

### Code Block 5

```pascal
Procedure GenerateCodeForOperator_IN (Var ElementExpression, SetExpression: TExpression);
Var ExpressionSet: PSetOfByte;
    Index, StartIndex, N, FoundJumpOpCode: Byte;
    _8_OrMoreElementRanges: Boolean;

  Function SearchForSetElementRange: Boolean;
  begin
    SearchForSetElementRange := False;
    Repeat
      If Index in ExpressionSet^ then Break;
      Inc (Index);
      If Index = 0 then Exit;
    until False;
    StartIndex := Index;
    Repeat
      If not (Index in ExpressionSet^) then Break;
      Inc (Index);
    until Index = 0;
    SearchForSetElementRange := True;
  end;

begin
  CheckElementAndSetCompatibility (ElementExpression, SetExpression);
  ElementExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Byte_TypeOffset));
  ExpressionSet := PSetOfByte (Ptr (DSeg, SetExpression.Value.Word));
  If SetExpression.Location <> elConstant then
    begin
      OperationWithSet (ElementExpression, SetExpression, $01, TEST_Memory_AL, TEST_Memory_ImmediateByte);
      Exit;
    end;
  If ElementExpression.Location = elConstant then
    begin
      ElementExpression.SetConstantBooleanExpression (ElementExpression.Value.Byte in ExpressionSet^);
      Exit;
    end;
  Index := 0;             { Set is known at compile time }
  If not SearchForSetElementRange then
    begin
      ElementExpression.SetConstantBooleanExpression (False);
      Exit;
    end;
  If Index or StartIndex = 0 then
    begin
      ElementExpression.SetConstantBooleanExpression (True);
      Exit;
    end;
  _8_OrMoreElementRanges := True;
  For N := 1 to 8 do
    If (Index = 0) or (not SearchForSetElementRange) then
      begin
        _8_OrMoreElementRanges := False;
        Break;
      end;
  If _8_OrMoreElementRanges then
    begin
      SetExpression.ExpandSetToStackFrameAndPushPointer (32);
      OperationWithSet (ElementExpression, SetExpression, $01, TEST_Memory_AL, TEST_Memory_ImmediateByte);
      Exit;
    end;
  ElementExpression.Calculate;
  ElementExpression.LoadExpressionToRegisters (urAX);
  With ElementExpression do
    begin
      Location := elBoolean;
      DataType := [];
      Value.ShortCircuitJumps := 0;
    end;
  Index := 0;
  FoundJumpOpCode := 0;
  While SearchForSetElementRange do
    begin
      Repeat
        If FoundJumpOpCode <> 0 then GenerateCodeForNearJump (ElementExpression.Value.LastJumpToTrue, FoundJumpOpCode);
        Dec (Index);
        If Index <> StartIndex then
          begin
            If StartIndex <> 0 then
              begin
                GenerateInstruction_TwoBytes (CMP_AL_Immediate, StartIndex);
                FoundJumpOpCode := JNB;
                If Index = $FF then Break;
                GenerateCodeForNearJump (ElementExpression.Value.LastJumpToFalse, JB);
              end;
            GenerateInstruction_TwoBytes (CMP_AL_Immediate, Index);
            FoundJumpOpCode := JBE;
            Break;
          end;
        GenerateInstruction_TwoBytes (CMP_AL_Immediate, StartIndex);
        FoundJumpOpCode := JE;
      until True;
      Inc (Index);
      If Index = 0 then Break;
    end;
  With ElementExpression do
    begin
      LocationData.JumpIfTrueOpCode := FoundJumpOpCode;
      ElementExpression.EndIntermediateCodeSubroutine;
      TypeDefPtr := Ptr (SystemUnitSegment, Boolean_TypeOffset);
    end;
end;
```

### Code Block 6

```pascal
Procedure CheckElementAndSetCompatibility (Var ElementExpression, SetExpression: TExpression);
begin
  If ElementExpression.TypeDefPtr^.BaseType < btInteger then Error (OperandTypesDoNotMatchOperator);
  If SetExpression.TypeDefPtr^.BaseType <> btSet then Error (OperandTypesDoNotMatchOperator);
  If PSetTypeDefinition (SetExpression.TypeDefPtr)^.BaseSetTypeOffset.TypeOffset <> 0 then
    begin
      If PointerFromOffsets (POrdinalTypeDefinition (ElementExpression.TypeDefPtr)^.OrdinalType) <>
    PointerFromOffsets (POrdinalTypeDefinition (
    PointerFromOffsets (PSetTypeDefinition (SetExpression.TypeDefPtr)^.BaseSetTypeOffset))^.OrdinalType) then
           Error (TypeMismatch);
    end;
end;
```

### Code Block 7

```pascal
Procedure OperationWithSet (Var ElementExpression, SetExpression: TExpression; Mask, TestMemoryRegisterOpCode: Byte; TestMemoryImmediateOpCode: Word);
Var SetSize: Byte;
    SetElementLowerLimit_div_8: Byte;
    MaskWord: WordRec;

  Function RegisterSetSizeInBits: Byte;
  begin
    If it16Bit in SetExpression.DataType then RegisterSetSizeInBits := 16 else RegisterSetSizeInBits := 8;
  end;


  Procedure GetSetSizeInMemory;
  begin
    If SetExpression.Location = elMemory then
      SetSize :=
        PSetTypeDefinition (SetExpression.TypeDefPtr)^.GetSetSizeAndLowestElementDataOffset (SetElementLowerLimit_div_8)
        else begin
               PopPointerAndConvertToMemory (SetExpression);
               SetSize := 32;
               SetElementLowerLimit_div_8 := 0;
             end;
  end;

begin
  TestSetInForElementMask := Mask;
  TestMemoryWithRegisterOpCode := TestMemoryRegisterOpCode;
  TestMemoryWithImmediateOpCode := TestMemoryImmediateOpCode;
  LastJumpToFalse := 0;
  Case ElementExpression.Location of
    elConstant: begin
                  Case SetExpression.Location of
                    elRegister: begin
                                  If ElementExpression.Value.Byte >= RegisterSetSizeInBits then
                                    begin
                                      ElementExpression.SetConstantBooleanExpression (False);
                                      Exit;
                                    end;
                                  SetExpression.Calculate;
                                  Case it16Bit in SetExpression.DataType of
                                    True: begin
                                            GenerateInstruction_Byte (TEST_AX_Immediate);
                                            GenerateInstruction_Word ($0001 shl ElementExpression.Value.Byte);
                                          end;
                                    else begin
                                           GenerateInstruction_Byte (TEST_AL_Immediate);
                                           GenerateInstruction_Byte ($01 shl ElementExpression.Value.Byte)
                                         end;
                                  end;
                                end;
                    else begin
                           GetSetSizeInMemory;
                           If (ElementExpression.Value.Byte shr 3 < SetElementLowerLimit_div_8) or
                              (ElementExpression.Value.Byte shr 3 - SetElementLowerLimit_div_8 >= SetSize) then
                             begin
                               ElementExpression.SetConstantBooleanExpression (False);
                               Exit;
                             end;
                           Inc (SetExpression.Value.Word, ElementExpression.Value.Byte shr 3 - SetElementLowerLimit_div_8);
                           SetExpression.Calculate;
                           SetExpression.GenerateInstructionWithExpressionInMemOrReg (
                             Lo (TestMemoryWithImmediateOpCode), Hi (TestMemoryWithImmediateOpCode));
                           MaskWord.Word := TestSetInForElementMask shl (ElementExpression.Value.Byte and $07);
                           GenerateInstruction_Byte (MaskWord.ByteL or MaskWord.ByteH);
                         end;
                  end;

                end;
    else begin
           Case it32Bytes in SetExpression.DataType of
             True: begin
                     GetSetSizeInMemory;
                     ElementExpression.Calculate;
                     ElementExpression.LoadExpressionToRegisters (urAX);
                     GenerateInstruction_TwoBytes (MOV_AH_Immediate, TestSetInForElementMask);
                     GenerateInstruction_Byte (MOV_DX_Immediate);
                     GenerateInstruction_TwoBytes (SetSize, SetElementLowerLimit_div_8);
                     GenerateInstruction_CALL_FAR (SysProc_ZBitMask);
                     With ElementExpression do
                       begin
                         DataType := [it32Bit, it16Bit];
                         UsedRegisters := [urBX, urDX, urCX, urAX];
                       end;
                   end;
             else begin
                    ElementExpression.Calculate;
                    ElementExpression.LoadExpressionToRegisters (urCX);
                    GenerateInstruction_Word (CMP_CL_Immediate);
                    GenerateInstruction_Byte (RegisterSetSizeInBits);
                    GenerateCodeForNearJump (LastJumpToFalse, JNB);
                    If it16Bit in SetExpression.DataType then
                       begin
                         GenerateInstruction_Byte (MOV_AX_Immediate);
                         GenerateInstruction_Word (Integer (ShortInt (TestSetInForElementMask)));
                         GenerateInstruction_Word (ROL_AX_CL);
                       end else begin
                                  GenerateInstruction_TwoBytes (MOV_AL_Immediate, TestSetInForElementMask);
                                  GenerateInstruction_Word (ROL_AL_CL);
                                end;
                    With ElementExpression do
                      begin
                        DataType := SetExpression.DataType;
                        Include (UsedRegisters, urAX);
                        LocationData.Flags := [];
                      end
                  end;
           end;
           ElementExpression.Save (SetExpression.UsedRegisters);
           SetExpression.Calculate;
           ElementExpression.PopToRegisters ([urAX]);
           With SetExpression do
             If it32Bytes in DataType then
               begin
                 DataType := [itUnsigned];
                 Case ofsDI in LocationData.Flags of
                   True: GenerateInstruction_Word (ADD_DI_Register or (rDX or ElementExpression.LocationData.Register) shl 8);
                   else begin
                          Include (SetExpression.LocationData.Flags, ofsDI);
                          GenerateInstruction_Word (MOV_DI_Register or
                                                    (rDX or ElementExpression.LocationData.Register) shl 8);
                        end;
                 end;
                 ES_DI_PointerDestroyed;
               end;
           SetExpression.GenerateInstruction_8_16_bit (TestMemoryWithRegisterOpCode,
                                                              ElementExpression.LocationData.Register shl 3);
         end;
  end;
  With ElementExpression do
    begin
      ElementExpression.EndIntermediateCodeSubroutine;
      ElementExpression.SetExpressionToBooleanJump (JNE);
      Value.LastJumpToFalse := LastJumpToFalse;
      UsedRegisters := UsedRegisters + SetExpression.UsedRegisters;
    end;
end;
```

### Code Block 8

```pascal
  Procedure ConstantSetOperations;
  Var LeftSet, RightSet: PSetOfByte;
  begin
    LeftSet  := Ptr (DSeg, LeftExpression.Value.Word);
    RightSet := Ptr (DSeg, RightExpression.Value.Word);
    Case Operation of
      Calc_Add:              LeftSet^ := LeftSet^ + RightSet^;
      Calc_Subtract:         LeftSet^ := LeftSet^ - RightSet^;
      Calc_Multiply:         LeftSet^ := LeftSet^ * RightSet^;
      Calc_IsEqual:          LeftExpression.SetConstantBooleanExpression (LeftSet^  = RightSet^);
      Calc_IsNotEqual:       LeftExpression.SetConstantBooleanExpression (LeftSet^ <> RightSet^);
      Calc_IsLowerOrEqual:   LeftExpression.SetConstantBooleanExpression (LeftSet^ <= RightSet^);
      Calc_IsGreaterOrEqual: LeftExpression.SetConstantBooleanExpression (LeftSet^ >= RightSet^);
    end;
    Case Operation of
      Calc_Add,
      Calc_Subtract,
      Calc_Multiply: LeftExpression.SetDataTypeOfSet;
    end;
  end;
```

### Code Block 9

```pascal
Procedure TExpression.SetDataTypeOfSet;
Var SetAddress: PWord;
    First16: Word;
    N: Byte;
    SizeType: TIntegerTypeSet;
    Zero: Boolean;
begin
  SetAddress := Ptr (DSeg, Value.Word);
  First16 := SetAddress^;
  SizeType := [it32Bytes];
  Zero := True;
  For N := 1 to 15 do
    begin
      Inc (SetAddress);
      If SetAddress^ <> 0 then Zero := False;
    end;
  If Zero then
    Case Hi (First16) of
      0: SizeType := [itUnsigned];
      else SizeType := [itUnsigned,it16Bit];
    end;
  DataType := SizeType;
end;
```

### Code Block 10

```pascal
  Procedure GenerateCodeForSetOperations;
  Var CommonSetType: TIntegerTypeSet;
      TempExpression: TExpression;
  begin
    CommonSetType := LeftExpression.DataType + RightExpression.DataType;
    If not (it32Bytes in CommonSetType) then
      begin
        LeftExpression.LoadConstantSmallSetIntoValue;
        LeftExpression.ExtendInteger (CommonSetType);
        RightExpression.LoadConstantSmallSetIntoValue;
        RightExpression.ExtendInteger (CommonSetType);
        Case Operation of
          Calc_Subtract,
          Calc_IsLowerOrEqual: RightExpression.NOT_Integer;
          Calc_IsGreaterOrEqual: begin
                                   ExchangeLeftAndRightExpression;
                                   RightExpression.NOT_Integer;
                                 end;
        end;
        Case Operation of
          Calc_Add: Operation := Calc_OR;
          Calc_Subtract,
          Calc_Multiply: Operation := Calc_AND;
          Calc_IsLowerOrEqual: begin
                                 Operation := Calc_AND;
                                 IntegerOperations;
                                 LeftExpression.SetExpressionToBooleanJump (JE);
                                 Exit;
                               end;
        end;
        IntegerOperations;
        Exit;
      end;
    Case Operation of
      Calc_IsEqual..Calc_IsLowerOrEqual: LeftExpression.ExpandSetToStackFrameAndPushPointer (32);
      else                               LeftExpression.ExpandSetToStackFrameAndPushPointer (0);
    end;
    RightExpression.ExpandSetToStackFrameAndPushPointer (32);
    If Operation = Calc_IsLowerOrEqual then
      begin
        TempExpression := LeftExpression;
        LeftExpression := RightExpression;
        RightExpression := TempExpression;
      end;
    ConvertToPointerAndPushBothExpressions (LeftExpression, RightExpression);
    CalculateBothExpressions (LeftExpression, RightExpression);
    If Operation = Calc_IsLowerOrEqual then
      begin
        TempExpression  := LeftExpression;
        LeftExpression  := RightExpression;
        RightExpression := TempExpression;
      end;
    Case Operation of
      Calc_Add,
      Calc_Subtract,
      Calc_Multiply: begin
                       Case Operation of
                         Calc_Add:      GenerateInstruction_CALL_FAR (SysProc_ZUnion);
                         Calc_Subtract: GenerateInstruction_CALL_FAR (SysProc_ZDifference);
                         Calc_Multiply: GenerateInstruction_CALL_FAR (SysProc_ZIntersect);
                       end;
                       LeftExpression.EndIntermediateCodeSubroutine;
                       LeftExpression.UsedRegisters := [urBX, urDX, urCX, urAX];
                     end;
      else begin
             Case Operation of
               Calc_IsEqual,
               Calc_IsNotEqual:     GenerateInstruction_CALL_FAR (SysProc_ZEqual);
               Calc_IsGreaterOrEqual,
               Calc_IsLowerOrEqual: GenerateInstruction_CALL_FAR (SysProc_ZRelation);
             end;
             LeftExpression.EndIntermediateCodeSubroutine;
             LeftExpression.UsedRegisters := [urBX, urDX, urCX, urAX];
             Case Operation of
               Calc_IsNotEqual: LeftExpression.SetExpressionToBooleanJump (JNE);
               else             LeftExpression.SetExpressionToBooleanJump (JE);
             end;
          end;
    end;
  end;
```

### Code Block 11

```pascal
Procedure TExpression.LoadConstantSmallSetIntoValue;
begin
  If Location = elConstant then Value.LongInt := Word (Ptr (DSeg, Value.Word)^);
end;
```

### Code Block 12

```pascal
Procedure GenerateCodeForOperator_IN (Var ElementExpression, SetExpression: TExpression);
Var ExpressionSet: PSetOfByte;
    Index, StartIndex, N, FoundJumpOpCode: Byte;
    _8_OrMoreElementRanges: Boolean;

  Function SearchForSetElementRange: Boolean;
  begin
    SearchForSetElementRange := False;
    Repeat
      If Index in ExpressionSet^ then Break;
      Inc (Index);
      If Index = 0 then Exit;
    until False;
    StartIndex := Index;
    Repeat
      If not (Index in ExpressionSet^) then Break;
      Inc (Index);
    until Index = 0;
    SearchForSetElementRange := True;
  end;

begin
  CheckElementAndSetCompatibility (ElementExpression, SetExpression);
  ElementExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Byte_TypeOffset));
  ExpressionSet := PSetOfByte (Ptr (DSeg, SetExpression.Value.Word));
  If SetExpression.Location <> elConstant then
    begin
      OperationWithSet (ElementExpression, SetExpression, $01, TEST_Memory_AL, TEST_Memory_ImmediateByte);
      Exit;
    end;
  If ElementExpression.Location = elConstant then
    begin
      ElementExpression.SetConstantBooleanExpression (ElementExpression.Value.Byte in ExpressionSet^);
      Exit;
    end;
  Index := 0;             { Set is known at compile time }
  If not SearchForSetElementRange then
    begin
      ElementExpression.SetConstantBooleanExpression (False);
      Exit;
    end;
  If Index or StartIndex = 0 then
    begin
      ElementExpression.SetConstantBooleanExpression (True);
      Exit;
    end;
  _8_OrMoreElementRanges := True;
  For N := 1 to 8 do
    If (Index = 0) or (not SearchForSetElementRange) then
      begin
        _8_OrMoreElementRanges := False;
        Break;
      end;
  If _8_OrMoreElementRanges then
    begin
      SetExpression.ExpandSetToStackFrameAndPushPointer (32);
      OperationWithSet (ElementExpression, SetExpression, $01, TEST_Memory_AL, TEST_Memory_ImmediateByte);
      Exit;
    end;
  ElementExpression.Calculate;
  ElementExpression.LoadExpressionToRegisters (urAX);
  With ElementExpression do
    begin
      Location := elBoolean;
      DataType := [];
      Value.ShortCircuitJumps := 0;
    end;
  Index := 0;
  FoundJumpOpCode := 0;
  While SearchForSetElementRange do
    begin
      Repeat
        If FoundJumpOpCode <> 0 then GenerateCodeForNearJump (ElementExpression.Value.LastJumpToTrue, FoundJumpOpCode);
        Dec (Index);
        If Index <> StartIndex then
          begin
            If StartIndex <> 0 then
              begin
                GenerateInstruction_TwoBytes (CMP_AL_Immediate, StartIndex);
                FoundJumpOpCode := JNB;
                If Index = $FF then Break;
                GenerateCodeForNearJump (ElementExpression.Value.LastJumpToFalse, JB);
              end;
            GenerateInstruction_TwoBytes (CMP_AL_Immediate, Index);
            FoundJumpOpCode := JBE;
            Break;
          end;
        GenerateInstruction_TwoBytes (CMP_AL_Immediate, StartIndex);
        FoundJumpOpCode := JE;
      until True;
      Inc (Index);
      If Index = 0 then Break;
    end;
  With ElementExpression do
    begin
      LocationData.JumpIfTrueOpCode := FoundJumpOpCode;
      ElementExpression.EndIntermediateCodeSubroutine;
      TypeDefPtr := Ptr (SystemUnitSegment, Boolean_TypeOffset);
    end;
end;
```

### Code Block 13

```pascal
Procedure CheckElementAndSetCompatibility (Var ElementExpression, SetExpression: TExpression);
begin
  If ElementExpression.TypeDefPtr^.BaseType < btInteger then Error (OperandTypesDoNotMatchOperator);
  If SetExpression.TypeDefPtr^.BaseType <> btSet then Error (OperandTypesDoNotMatchOperator);
  If PSetTypeDefinition (SetExpression.TypeDefPtr)^.BaseSetTypeOffset.TypeOffset <> 0 then
    begin
      If PointerFromOffsets (POrdinalTypeDefinition (ElementExpression.TypeDefPtr)^.OrdinalType) <>
    PointerFromOffsets (POrdinalTypeDefinition (
    PointerFromOffsets (PSetTypeDefinition (SetExpression.TypeDefPtr)^.BaseSetTypeOffset))^.OrdinalType) then
           Error (TypeMismatch);
    end;
end;
```

### Code Block 14

```pascal
Procedure OperationWithSet (Var ElementExpression, SetExpression: TExpression; Mask, TestMemoryRegisterOpCode: Byte; TestMemoryImmediateOpCode: Word);
Var SetSize: Byte;
    SetElementLowerLimit_div_8: Byte;
    MaskWord: WordRec;

  Function RegisterSetSizeInBits: Byte;
  begin
    If it16Bit in SetExpression.DataType then RegisterSetSizeInBits := 16 else RegisterSetSizeInBits := 8;
  end;


  Procedure GetSetSizeInMemory;
  begin
    If SetExpression.Location = elMemory then
      SetSize :=
        PSetTypeDefinition (SetExpression.TypeDefPtr)^.GetSetSizeAndLowestElementDataOffset (SetElementLowerLimit_div_8)
        else begin
               PopPointerAndConvertToMemory (SetExpression);
               SetSize := 32;
               SetElementLowerLimit_div_8 := 0;
             end;
  end;

begin
  TestSetInForElementMask := Mask;
  TestMemoryWithRegisterOpCode := TestMemoryRegisterOpCode;
  TestMemoryWithImmediateOpCode := TestMemoryImmediateOpCode;
  LastJumpToFalse := 0;
  Case ElementExpression.Location of
    elConstant: begin
                  Case SetExpression.Location of
                    elRegister: begin
                                  If ElementExpression.Value.Byte >= RegisterSetSizeInBits then
                                    begin
                                      ElementExpression.SetConstantBooleanExpression (False);
                                      Exit;
                                    end;
                                  SetExpression.Calculate;
                                  Case it16Bit in SetExpression.DataType of
                                    True: begin
                                            GenerateInstruction_Byte (TEST_AX_Immediate);
                                            GenerateInstruction_Word ($0001 shl ElementExpression.Value.Byte);
                                          end;
                                    else begin
                                           GenerateInstruction_Byte (TEST_AL_Immediate);
                                           GenerateInstruction_Byte ($01 shl ElementExpression.Value.Byte)
                                         end;
                                  end;
                                end;
                    else begin
                           GetSetSizeInMemory;
                           If (ElementExpression.Value.Byte shr 3 < SetElementLowerLimit_div_8) or
                              (ElementExpression.Value.Byte shr 3 - SetElementLowerLimit_div_8 >= SetSize) then
                             begin
                               ElementExpression.SetConstantBooleanExpression (False);
                               Exit;
                             end;
                           Inc (SetExpression.Value.Word, ElementExpression.Value.Byte shr 3 - SetElementLowerLimit_div_8);
                           SetExpression.Calculate;
                           SetExpression.GenerateInstructionWithExpressionInMemOrReg (
                             Lo (TestMemoryWithImmediateOpCode), Hi (TestMemoryWithImmediateOpCode));
                           MaskWord.Word := TestSetInForElementMask shl (ElementExpression.Value.Byte and $07);
                           GenerateInstruction_Byte (MaskWord.ByteL or MaskWord.ByteH);
                         end;
                  end;

                end;
    else begin
           Case it32Bytes in SetExpression.DataType of
             True: begin
                     GetSetSizeInMemory;
                     ElementExpression.Calculate;
                     ElementExpression.LoadExpressionToRegisters (urAX);
                     GenerateInstruction_TwoBytes (MOV_AH_Immediate, TestSetInForElementMask);
                     GenerateInstruction_Byte (MOV_DX_Immediate);
                     GenerateInstruction_TwoBytes (SetSize, SetElementLowerLimit_div_8);
                     GenerateInstruction_CALL_FAR (SysProc_ZBitMask);
                     With ElementExpression do
                       begin
                         DataType := [it32Bit, it16Bit];
                         UsedRegisters := [urBX, urDX, urCX, urAX];
                       end;
                   end;
             else begin
                    ElementExpression.Calculate;
                    ElementExpression.LoadExpressionToRegisters (urCX);
                    GenerateInstruction_Word (CMP_CL_Immediate);
                    GenerateInstruction_Byte (RegisterSetSizeInBits);
                    GenerateCodeForNearJump (LastJumpToFalse, JNB);
                    If it16Bit in SetExpression.DataType then
                       begin
                         GenerateInstruction_Byte (MOV_AX_Immediate);
                         GenerateInstruction_Word (Integer (ShortInt (TestSetInForElementMask)));
                         GenerateInstruction_Word (ROL_AX_CL);
                       end else begin
                                  GenerateInstruction_TwoBytes (MOV_AL_Immediate, TestSetInForElementMask);
                                  GenerateInstruction_Word (ROL_AL_CL);
                                end;
                    With ElementExpression do
                      begin
                        DataType := SetExpression.DataType;
                        Include (UsedRegisters, urAX);
                        LocationData.Flags := [];
                      end
                  end;
           end;
           ElementExpression.Save (SetExpression.UsedRegisters);
           SetExpression.Calculate;
           ElementExpression.PopToRegisters ([urAX]);
           With SetExpression do
             If it32Bytes in DataType then
               begin
                 DataType := [itUnsigned];
                 Case ofsDI in LocationData.Flags of
                   True: GenerateInstruction_Word (ADD_DI_Register or (rDX or ElementExpression.LocationData.Register) shl 8);
                   else begin
                          Include (SetExpression.LocationData.Flags, ofsDI);
                          GenerateInstruction_Word (MOV_DI_Register or
                                                    (rDX or ElementExpression.LocationData.Register) shl 8);
                        end;
                 end;
                 ES_DI_PointerDestroyed;
               end;
           SetExpression.GenerateInstruction_8_16_bit (TestMemoryWithRegisterOpCode,
                                                              ElementExpression.LocationData.Register shl 3);
         end;
  end;
  With ElementExpression do
    begin
      ElementExpression.EndIntermediateCodeSubroutine;
      ElementExpression.SetExpressionToBooleanJump (JNE);
      Value.LastJumpToFalse := LastJumpToFalse;
      UsedRegisters := UsedRegisters + SetExpression.UsedRegisters;
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
- https://turbopascal.org/set-operations
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

