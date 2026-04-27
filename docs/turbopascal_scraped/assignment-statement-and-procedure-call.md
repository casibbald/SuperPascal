# Assignment Statement and Procedure Call - Turbo Pascal Compiler

**URL:** https://turbopascal.org/assignment-statement-and-procedure-call

---

## Table of Contents

- Assignment Statement and Procedure Call

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Assignment Statement and Procedure Call
This procedure processes assignment statements and procedure/function calls. It first checks for variable expression, then if the token is procedure or function it generates code to
call procedure
(or function), for functions it
processes qualifiers
in a loop and stops if it finds assignment token.
For assignments it first calculates
value expression
, it
adjusts type
if necessary,
checks type compatibility
and
range
and generates intermediate code for assignment.
Procedure TStatement.ProcessAssignmentStatementOrProcedureCall;
Var VarExpression, ValueExpression: TExpression;
Label AssignmentFinished;

  Procedure ExpectVariableExpression;
  Var FunctionType: PTypeDefinition;
      FunctionResultOffset: Integer;
      VarFlags: TVarFlagsSet;
  begin
    If (Token = Token_ProcedureIdentifier) and FunctionAsResult (FunctionType, FunctionResultOffset) then
      With VarExpression do
        begin
          TypeDefPtr := FunctionType;
          Value.Word := FunctionResultOffset;
          DataType   := FunctionType^.DataType;
          Location   := elMemory;
          UsedRegisters := [];
          VarFlags := [];
          If Value.Integer >= 0 then VarFlags := [vfVar];
          VarExpression.LoadAddressOfLocalVariable (VarFlags, CurrentIdentifierOffset);
          VarExpression.EndIntermediateCodeSubroutine;
          GetNextToken;
        end else VarExpression.ExpectVariableIdentifier;
  end;

begin
  ExpectVariableExpression;
  With VarExpression do
    begin
      Repeat
        If TypeDefPtr^.BaseType = btFunctionProcedure then
          begin
            Repeat
              If (Location <> elProcedure) and (Token = Token_Assignment) then Break;
              CallProcedure;
              If PProcedureTypeDefinition (TypeDefPtr)^.ResultTypeOffset.TypeOffset = 0 then GoTo AssignmentFinished;
              SetExpressionToFunctionResult;
            until not VarExpression.ProcessQualifiers;
            If not (ExtendedSyntax in ModuleCompilerSwitches) then Break;
            If Token = Token_Assignment then Break;
            RemoveFunctionResultFromStack;
            GoTo AssignmentFinished;
          end;
      until not VarExpression.ProcessQualifiers;
      If (Location <> elMemory) or (efSegment in LocationData.Flags) then Error (InvalidVariableReference);
      Case TypeDefPtr^.BaseType of
        btUntyped,
        btFile,
        btText: Error (IllegalAssignment);
      end;
      ExpectTokenAndGetNext (Token_Assignment);
      ValueExpression.CalculateExpressionWithType (TypeDefPtr);
      ValueExpression.AdjustExpressionToType (TypeDefPtr);
      ValueExpression.CheckTypeCompatibility (TypeDefPtr);
      ValueExpression.CheckRange (TypeDefPtr);
      GenerateAssignmentIntermediateCode (VarExpression, ValueExpression);

AssignmentFinished:

      StatementCode := IntermediateCodeOffset;
    end;
end;
This procedure generates assignment intermediate code.
Procedure GenerateAssignmentIntermediateCode (Var VarExpression, ValueExpression: TExpression);

  Procedure CallSystemProcedure (AX, DX: Word);
  begin
    ConvertToPointerAndPushBothExpressions (ValueExpression, VarExpression);
    CalculateBothExpressions (ValueExpression, VarExpression);
    GenerateCodeToPushWord (DX);
    GenerateInstruction_CALL_FAR (AX);
    VarExpression.EndIntermediateCodeSubroutine
  end;

  Procedure GenerateCodeForSimpleAssignment;
  begin
    If ValueExpression.Location <> elConstant then
      begin
        ValueExpression.Calculate;
        ValueExpression.Save (VarExpression.UsedRegisters);
        VarExpression.Calculate;
        ValueExpression.PopToRegisters ([]);
        VarExpression.GenerateInstruction_MOV_Memory_Register (ValueExpression.LocationData.Register);
        VarExpression.EndIntermediateCodeSubroutine;
        Exit;
      end;
    If (ValueExpression.Value.LongInt = 0) and (it16Bit in VarExpression.DataType) then
      begin
        VarExpression.Calculate;
        GenerateInstruction_Word (XOR_AX_AX);
        VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rAX);
        If it32Bit in VarExpression.DataType then
          begin
            Inc (VarExpression.Value.Offset, 2);
            VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rAX);
            Dec (VarExpression.Value.Offset, 2);
          end;
        Include (VarExpression.UsedRegisters, urAX);
        VarExpression.EndIntermediateCodeSubroutine;
        Exit;
      end;
    VarExpression.Calculate;
    VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.Word);
    If it32Bit in VarExpression.DataType then
      begin
        Inc (VarExpression.Value.Offset, 2);
        VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.LongRec.WordH);
        Dec (VarExpression.Value.Offset, 2);
      end;
    VarExpression.EndIntermediateCodeSubroutine;
  end;

  Procedure GenerateCodeForComplexAssignment;
  begin
    ValueExpression.SetDataTypeForSimpleType;
    If VarExpression.SetDataTypeForSimpleType then GenerateCodeForSimpleAssignment
      else CallSystemProcedure (SysProc_BlockMove, VarExpression.TypeDefptr^.Size);
  end;

  Procedure GenerateCodeForObjectAssignment;
  begin
    If PObjectTypeDefinition (VarExpression.TypeDefPtr)^.VMT_Size = 0 then GenerateCodeForComplexAssignment
      else CallSystemProcedure (SysProc_CopyObject, PObjectTypeDefinition (VarExpression.TypeDefPtr)^.OffsetOf_VMT_Offset);
  end;

  Procedure GenerateCodeForSetAssignment;
  Var SetSize, SetElementLowerLimit_div_8: Byte;
  begin
    If it32Bytes in VarExpression.DataType then
      begin
        ValueExpression.ExpandSetToStackFrameAndPushPointer (32);
        ConvertToPointerAndPushBothExpressions (ValueExpression, VarExpression);
        CalculateBothExpressions (ValueExpression, VarExpression);
        SetSize :=
          PSetTypeDefinition (VarExpression.TypeDefPtr)^.GetSetSizeAndLowestElementDataOffset (SetElementLowerLimit_div_8);
        GenerateCodeToPushWord (SetElementLowerLimit_div_8 shl 8 or SetSize);
        GenerateInstruction_CALL_FAR (SysProc_ZStore);
        VarExpression.EndIntermediateCodeSubroutine;
      end else GenerateCodeForSimpleAssignment;
  end;

  Procedure GeneratoCodeForStringAssignment;
  begin
    If (ValueExpression.Location = elConstant) and (PString (Ptr (DSeg, ValueExpression.Value.Offset))^ = '') then
      begin
        VarExpression.Calculate;
        VarExpression.GenerateInstructionWithExpressionInMemOrReg (MOV_RegisterOrMemory_Immediate, 0);
        GenerateInstruction_Byte (0);
      end else begin
                 VarExpression.PushArrayPointerAndHighestIndex;
                 ValueExpression.StoreStringConstantToCodeBlock;
                 ValueExpression.ConvertToPointerAndPush;
                 CalculateBothExpressions (ValueExpression, VarExpression);
                 GenerateInstruction_CALL_FAR (SysProc_SStore);
               end;
    VarExpression.EndIntermediateCodeSubroutine;
  end;

  Procedure GenerateCodeForExtendedAssignment;
  begin
    ValueExpression.LoadExpressionToFPU;
    CalculateBothExpressions (ValueExpression, VarExpression);
    With VarExpression do
      begin
        Case fpExtended in DataType of
          True: VarExpression.GenerateFPUInstructionWithExpressionInMemory ($D9 or Byte (DataType), $38);
          else  VarExpression.GenerateFPUInstructionWithExpressionInMemory ($D9 or Byte (DataType), $18);
        end;
        GenerateInstruction_TwoBytes ($CD, $3D);
        VarExpression.EndIntermediateCodeSubroutine;
      end;
  end;

  Procedure GenerateCodeForRealAssignment;
  begin
    With VarExpression do
      begin
        Case ValueExpression.Location of
          elConstant: begin
                        VarExpression.Calculate;
                        VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.Real1);
                        Inc (Value.Offset, 2);
                        VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.Real2);
                        Inc (Value.Offset, 2);
                        VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.Real3);
                      end;
          else begin
                 ValueExpression.Calculate;
                 ValueExpression.LoadRealExpressionToRegisterSet (rAX_BX_DX);
                 If UsedRegisters <> [] then
                   begin
                     GenerateInstruction_TwoBytes (PUSH_DX, PUSH_BX);
                     GenerateInstruction_Byte (PUSH_AX);
                   end;
                 VarExpression.Calculate;
                 If UsedRegisters <> [] then
                   begin
                     GenerateInstruction_Byte (PUSH_AX);
                     GenerateInstruction_TwoBytes (POP_BX, POP_DX );
                   end;
                 VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rAX);
                 Inc (Value.Offset, 2);
                 VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rBX);
                 Inc (Value.Offset, 2);
                 VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rDX);
               end;
        end;
        Dec (Value.Offset, 4);
        VarExpression.EndIntermediateCodeSubroutine;
      end;
  end;

begin
  Case VarExpression.TypeDefPtr^.BaseType of
    btUntyped,
    btArray,
    btRecord,
    btFile,
    btText:        GenerateCodeForComplexAssignment;
    btObject:      GenerateCodeForObjectAssignment;
    btFunctionProcedure,
    btPointer,
    btInteger,
    btBoolean,
    btChar,
    btEnumeration: GenerateCodeForSimpleAssignment;
    btSet:         GenerateCodeForSetAssignment;
    btString:      GeneratoCodeForStringAssignment;
    btExtended:    GenerateCodeForExtendedAssignment;
    btReal:        GenerateCodeForRealAssignment;
  end;
end;
This function checks if function name is used as a variable to store function’s result value.
Function FunctionAsResult (Var TypeDef: PTypeDefinition; Var FuncResultOffset: Integer): Boolean;
Var ProceduresBlockRecord: PProceduresBlockRecord;
    ProcedureIdentifierData: PProcedureIdentifierData absolute CurrentIdentifierDataPtr;
begin
  FunctionAsResult := False;
  If Seg (CurrentIdentifierDataPtr^) <> SymbolTable [stMain].Segment then Exit;
  If ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset.TypeOffset = 0 then Exit;
  ProceduresBlockRecord := Ptr (SymbolTable [stProcedures].Segment, ProcedureIdentifierData^.ProceduresRecordOffset);
  If ProceduresBlockRecord^.ProgramCodeBlockRecordOffset <> $FFFE then Exit;
  TypeDef := PointerFromOffsets (ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset);
  With TypeDef^ do
    begin
      FuncResultOffset := - Size;
      If BaseType = btString then SizeOfPushedParameters (CurrentIdentifierDataPtr, FuncResultOffset);
    end;
  FunctionAsResult := True;
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
Procedure TStatement.ProcessAssignmentStatementOrProcedureCall;
Var VarExpression, ValueExpression: TExpression;
Label AssignmentFinished;

  Procedure ExpectVariableExpression;
  Var FunctionType: PTypeDefinition;
      FunctionResultOffset: Integer;
      VarFlags: TVarFlagsSet;
  begin
    If (Token = Token_ProcedureIdentifier) and FunctionAsResult (FunctionType, FunctionResultOffset) then
      With VarExpression do
        begin
          TypeDefPtr := FunctionType;
          Value.Word := FunctionResultOffset;
          DataType   := FunctionType^.DataType;
          Location   := elMemory;
          UsedRegisters := [];
          VarFlags := [];
          If Value.Integer >= 0 then VarFlags := [vfVar];
          VarExpression.LoadAddressOfLocalVariable (VarFlags, CurrentIdentifierOffset);
          VarExpression.EndIntermediateCodeSubroutine;
          GetNextToken;
        end else VarExpression.ExpectVariableIdentifier;
  end;

begin
  ExpectVariableExpression;
  With VarExpression do
    begin
      Repeat
        If TypeDefPtr^.BaseType = btFunctionProcedure then
          begin
            Repeat
              If (Location <> elProcedure) and (Token = Token_Assignment) then Break;
              CallProcedure;
              If PProcedureTypeDefinition (TypeDefPtr)^.ResultTypeOffset.TypeOffset = 0 then GoTo AssignmentFinished;
              SetExpressionToFunctionResult;
            until not VarExpression.ProcessQualifiers;
            If not (ExtendedSyntax in ModuleCompilerSwitches) then Break;
            If Token = Token_Assignment then Break;
            RemoveFunctionResultFromStack;
            GoTo AssignmentFinished;
          end;
      until not VarExpression.ProcessQualifiers;
      If (Location <> elMemory) or (efSegment in LocationData.Flags) then Error (InvalidVariableReference);
      Case TypeDefPtr^.BaseType of
        btUntyped,
        btFile,
        btText: Error (IllegalAssignment);
      end;
      ExpectTokenAndGetNext (Token_Assignment);
      ValueExpression.CalculateExpressionWithType (TypeDefPtr);
      ValueExpression.AdjustExpressionToType (TypeDefPtr);
      ValueExpression.CheckTypeCompatibility (TypeDefPtr);
      ValueExpression.CheckRange (TypeDefPtr);
      GenerateAssignmentIntermediateCode (VarExpression, ValueExpression);

AssignmentFinished:

      StatementCode := IntermediateCodeOffset;
    end;
end;
```

### Code Block 2

```pascal
Procedure GenerateAssignmentIntermediateCode (Var VarExpression, ValueExpression: TExpression);

  Procedure CallSystemProcedure (AX, DX: Word);
  begin
    ConvertToPointerAndPushBothExpressions (ValueExpression, VarExpression);
    CalculateBothExpressions (ValueExpression, VarExpression);
    GenerateCodeToPushWord (DX);
    GenerateInstruction_CALL_FAR (AX);
    VarExpression.EndIntermediateCodeSubroutine
  end;

  Procedure GenerateCodeForSimpleAssignment;
  begin
    If ValueExpression.Location <> elConstant then
      begin
        ValueExpression.Calculate;
        ValueExpression.Save (VarExpression.UsedRegisters);
        VarExpression.Calculate;
        ValueExpression.PopToRegisters ([]);
        VarExpression.GenerateInstruction_MOV_Memory_Register (ValueExpression.LocationData.Register);
        VarExpression.EndIntermediateCodeSubroutine;
        Exit;
      end;
    If (ValueExpression.Value.LongInt = 0) and (it16Bit in VarExpression.DataType) then
      begin
        VarExpression.Calculate;
        GenerateInstruction_Word (XOR_AX_AX);
        VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rAX);
        If it32Bit in VarExpression.DataType then
          begin
            Inc (VarExpression.Value.Offset, 2);
            VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rAX);
            Dec (VarExpression.Value.Offset, 2);
          end;
        Include (VarExpression.UsedRegisters, urAX);
        VarExpression.EndIntermediateCodeSubroutine;
        Exit;
      end;
    VarExpression.Calculate;
    VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.Word);
    If it32Bit in VarExpression.DataType then
      begin
        Inc (VarExpression.Value.Offset, 2);
        VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.LongRec.WordH);
        Dec (VarExpression.Value.Offset, 2);
      end;
    VarExpression.EndIntermediateCodeSubroutine;
  end;

  Procedure GenerateCodeForComplexAssignment;
  begin
    ValueExpression.SetDataTypeForSimpleType;
    If VarExpression.SetDataTypeForSimpleType then GenerateCodeForSimpleAssignment
      else CallSystemProcedure (SysProc_BlockMove, VarExpression.TypeDefptr^.Size);
  end;

  Procedure GenerateCodeForObjectAssignment;
  begin
    If PObjectTypeDefinition (VarExpression.TypeDefPtr)^.VMT_Size = 0 then GenerateCodeForComplexAssignment
      else CallSystemProcedure (SysProc_CopyObject, PObjectTypeDefinition (VarExpression.TypeDefPtr)^.OffsetOf_VMT_Offset);
  end;

  Procedure GenerateCodeForSetAssignment;
  Var SetSize, SetElementLowerLimit_div_8: Byte;
  begin
    If it32Bytes in VarExpression.DataType then
      begin
        ValueExpression.ExpandSetToStackFrameAndPushPointer (32);
        ConvertToPointerAndPushBothExpressions (ValueExpression, VarExpression);
        CalculateBothExpressions (ValueExpression, VarExpression);
        SetSize :=
          PSetTypeDefinition (VarExpression.TypeDefPtr)^.GetSetSizeAndLowestElementDataOffset (SetElementLowerLimit_div_8);
        GenerateCodeToPushWord (SetElementLowerLimit_div_8 shl 8 or SetSize);
        GenerateInstruction_CALL_FAR (SysProc_ZStore);
        VarExpression.EndIntermediateCodeSubroutine;
      end else GenerateCodeForSimpleAssignment;
  end;

  Procedure GeneratoCodeForStringAssignment;
  begin
    If (ValueExpression.Location = elConstant) and (PString (Ptr (DSeg, ValueExpression.Value.Offset))^ = '') then
      begin
        VarExpression.Calculate;
        VarExpression.GenerateInstructionWithExpressionInMemOrReg (MOV_RegisterOrMemory_Immediate, 0);
        GenerateInstruction_Byte (0);
      end else begin
                 VarExpression.PushArrayPointerAndHighestIndex;
                 ValueExpression.StoreStringConstantToCodeBlock;
                 ValueExpression.ConvertToPointerAndPush;
                 CalculateBothExpressions (ValueExpression, VarExpression);
                 GenerateInstruction_CALL_FAR (SysProc_SStore);
               end;
    VarExpression.EndIntermediateCodeSubroutine;
  end;

  Procedure GenerateCodeForExtendedAssignment;
  begin
    ValueExpression.LoadExpressionToFPU;
    CalculateBothExpressions (ValueExpression, VarExpression);
    With VarExpression do
      begin
        Case fpExtended in DataType of
          True: VarExpression.GenerateFPUInstructionWithExpressionInMemory ($D9 or Byte (DataType), $38);
          else  VarExpression.GenerateFPUInstructionWithExpressionInMemory ($D9 or Byte (DataType), $18);
        end;
        GenerateInstruction_TwoBytes ($CD, $3D);
        VarExpression.EndIntermediateCodeSubroutine;
      end;
  end;

  Procedure GenerateCodeForRealAssignment;
  begin
    With VarExpression do
      begin
        Case ValueExpression.Location of
          elConstant: begin
                        VarExpression.Calculate;
                        VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.Real1);
                        Inc (Value.Offset, 2);
                        VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.Real2);
                        Inc (Value.Offset, 2);
                        VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.Real3);
                      end;
          else begin
                 ValueExpression.Calculate;
                 ValueExpression.LoadRealExpressionToRegisterSet (rAX_BX_DX);
                 If UsedRegisters <> [] then
                   begin
                     GenerateInstruction_TwoBytes (PUSH_DX, PUSH_BX);
                     GenerateInstruction_Byte (PUSH_AX);
                   end;
                 VarExpression.Calculate;
                 If UsedRegisters <> [] then
                   begin
                     GenerateInstruction_Byte (PUSH_AX);
                     GenerateInstruction_TwoBytes (POP_BX, POP_DX );
                   end;
                 VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rAX);
                 Inc (Value.Offset, 2);
                 VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rBX);
                 Inc (Value.Offset, 2);
                 VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rDX);
               end;
        end;
        Dec (Value.Offset, 4);
        VarExpression.EndIntermediateCodeSubroutine;
      end;
  end;

begin
  Case VarExpression.TypeDefPtr^.BaseType of
    btUntyped,
    btArray,
    btRecord,
    btFile,
    btText:        GenerateCodeForComplexAssignment;
    btObject:      GenerateCodeForObjectAssignment;
    btFunctionProcedure,
    btPointer,
    btInteger,
    btBoolean,
    btChar,
    btEnumeration: GenerateCodeForSimpleAssignment;
    btSet:         GenerateCodeForSetAssignment;
    btString:      GeneratoCodeForStringAssignment;
    btExtended:    GenerateCodeForExtendedAssignment;
    btReal:        GenerateCodeForRealAssignment;
  end;
end;
```

### Code Block 3

```pascal
Function FunctionAsResult (Var TypeDef: PTypeDefinition; Var FuncResultOffset: Integer): Boolean;
Var ProceduresBlockRecord: PProceduresBlockRecord;
    ProcedureIdentifierData: PProcedureIdentifierData absolute CurrentIdentifierDataPtr;
begin
  FunctionAsResult := False;
  If Seg (CurrentIdentifierDataPtr^) <> SymbolTable [stMain].Segment then Exit;
  If ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset.TypeOffset = 0 then Exit;
  ProceduresBlockRecord := Ptr (SymbolTable [stProcedures].Segment, ProcedureIdentifierData^.ProceduresRecordOffset);
  If ProceduresBlockRecord^.ProgramCodeBlockRecordOffset <> $FFFE then Exit;
  TypeDef := PointerFromOffsets (ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset);
  With TypeDef^ do
    begin
      FuncResultOffset := - Size;
      If BaseType = btString then SizeOfPushedParameters (CurrentIdentifierDataPtr, FuncResultOffset);
    end;
  FunctionAsResult := True;
end;
```

### Code Block 4

```pascal
Procedure TStatement.ProcessAssignmentStatementOrProcedureCall;
Var VarExpression, ValueExpression: TExpression;
Label AssignmentFinished;

  Procedure ExpectVariableExpression;
  Var FunctionType: PTypeDefinition;
      FunctionResultOffset: Integer;
      VarFlags: TVarFlagsSet;
  begin
    If (Token = Token_ProcedureIdentifier) and FunctionAsResult (FunctionType, FunctionResultOffset) then
      With VarExpression do
        begin
          TypeDefPtr := FunctionType;
          Value.Word := FunctionResultOffset;
          DataType   := FunctionType^.DataType;
          Location   := elMemory;
          UsedRegisters := [];
          VarFlags := [];
          If Value.Integer >= 0 then VarFlags := [vfVar];
          VarExpression.LoadAddressOfLocalVariable (VarFlags, CurrentIdentifierOffset);
          VarExpression.EndIntermediateCodeSubroutine;
          GetNextToken;
        end else VarExpression.ExpectVariableIdentifier;
  end;

begin
  ExpectVariableExpression;
  With VarExpression do
    begin
      Repeat
        If TypeDefPtr^.BaseType = btFunctionProcedure then
          begin
            Repeat
              If (Location <> elProcedure) and (Token = Token_Assignment) then Break;
              CallProcedure;
              If PProcedureTypeDefinition (TypeDefPtr)^.ResultTypeOffset.TypeOffset = 0 then GoTo AssignmentFinished;
              SetExpressionToFunctionResult;
            until not VarExpression.ProcessQualifiers;
            If not (ExtendedSyntax in ModuleCompilerSwitches) then Break;
            If Token = Token_Assignment then Break;
            RemoveFunctionResultFromStack;
            GoTo AssignmentFinished;
          end;
      until not VarExpression.ProcessQualifiers;
      If (Location <> elMemory) or (efSegment in LocationData.Flags) then Error (InvalidVariableReference);
      Case TypeDefPtr^.BaseType of
        btUntyped,
        btFile,
        btText: Error (IllegalAssignment);
      end;
      ExpectTokenAndGetNext (Token_Assignment);
      ValueExpression.CalculateExpressionWithType (TypeDefPtr);
      ValueExpression.AdjustExpressionToType (TypeDefPtr);
      ValueExpression.CheckTypeCompatibility (TypeDefPtr);
      ValueExpression.CheckRange (TypeDefPtr);
      GenerateAssignmentIntermediateCode (VarExpression, ValueExpression);

AssignmentFinished:

      StatementCode := IntermediateCodeOffset;
    end;
end;
```

### Code Block 5

```pascal
Procedure GenerateAssignmentIntermediateCode (Var VarExpression, ValueExpression: TExpression);

  Procedure CallSystemProcedure (AX, DX: Word);
  begin
    ConvertToPointerAndPushBothExpressions (ValueExpression, VarExpression);
    CalculateBothExpressions (ValueExpression, VarExpression);
    GenerateCodeToPushWord (DX);
    GenerateInstruction_CALL_FAR (AX);
    VarExpression.EndIntermediateCodeSubroutine
  end;

  Procedure GenerateCodeForSimpleAssignment;
  begin
    If ValueExpression.Location <> elConstant then
      begin
        ValueExpression.Calculate;
        ValueExpression.Save (VarExpression.UsedRegisters);
        VarExpression.Calculate;
        ValueExpression.PopToRegisters ([]);
        VarExpression.GenerateInstruction_MOV_Memory_Register (ValueExpression.LocationData.Register);
        VarExpression.EndIntermediateCodeSubroutine;
        Exit;
      end;
    If (ValueExpression.Value.LongInt = 0) and (it16Bit in VarExpression.DataType) then
      begin
        VarExpression.Calculate;
        GenerateInstruction_Word (XOR_AX_AX);
        VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rAX);
        If it32Bit in VarExpression.DataType then
          begin
            Inc (VarExpression.Value.Offset, 2);
            VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rAX);
            Dec (VarExpression.Value.Offset, 2);
          end;
        Include (VarExpression.UsedRegisters, urAX);
        VarExpression.EndIntermediateCodeSubroutine;
        Exit;
      end;
    VarExpression.Calculate;
    VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.Word);
    If it32Bit in VarExpression.DataType then
      begin
        Inc (VarExpression.Value.Offset, 2);
        VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.LongRec.WordH);
        Dec (VarExpression.Value.Offset, 2);
      end;
    VarExpression.EndIntermediateCodeSubroutine;
  end;

  Procedure GenerateCodeForComplexAssignment;
  begin
    ValueExpression.SetDataTypeForSimpleType;
    If VarExpression.SetDataTypeForSimpleType then GenerateCodeForSimpleAssignment
      else CallSystemProcedure (SysProc_BlockMove, VarExpression.TypeDefptr^.Size);
  end;

  Procedure GenerateCodeForObjectAssignment;
  begin
    If PObjectTypeDefinition (VarExpression.TypeDefPtr)^.VMT_Size = 0 then GenerateCodeForComplexAssignment
      else CallSystemProcedure (SysProc_CopyObject, PObjectTypeDefinition (VarExpression.TypeDefPtr)^.OffsetOf_VMT_Offset);
  end;

  Procedure GenerateCodeForSetAssignment;
  Var SetSize, SetElementLowerLimit_div_8: Byte;
  begin
    If it32Bytes in VarExpression.DataType then
      begin
        ValueExpression.ExpandSetToStackFrameAndPushPointer (32);
        ConvertToPointerAndPushBothExpressions (ValueExpression, VarExpression);
        CalculateBothExpressions (ValueExpression, VarExpression);
        SetSize :=
          PSetTypeDefinition (VarExpression.TypeDefPtr)^.GetSetSizeAndLowestElementDataOffset (SetElementLowerLimit_div_8);
        GenerateCodeToPushWord (SetElementLowerLimit_div_8 shl 8 or SetSize);
        GenerateInstruction_CALL_FAR (SysProc_ZStore);
        VarExpression.EndIntermediateCodeSubroutine;
      end else GenerateCodeForSimpleAssignment;
  end;

  Procedure GeneratoCodeForStringAssignment;
  begin
    If (ValueExpression.Location = elConstant) and (PString (Ptr (DSeg, ValueExpression.Value.Offset))^ = '') then
      begin
        VarExpression.Calculate;
        VarExpression.GenerateInstructionWithExpressionInMemOrReg (MOV_RegisterOrMemory_Immediate, 0);
        GenerateInstruction_Byte (0);
      end else begin
                 VarExpression.PushArrayPointerAndHighestIndex;
                 ValueExpression.StoreStringConstantToCodeBlock;
                 ValueExpression.ConvertToPointerAndPush;
                 CalculateBothExpressions (ValueExpression, VarExpression);
                 GenerateInstruction_CALL_FAR (SysProc_SStore);
               end;
    VarExpression.EndIntermediateCodeSubroutine;
  end;

  Procedure GenerateCodeForExtendedAssignment;
  begin
    ValueExpression.LoadExpressionToFPU;
    CalculateBothExpressions (ValueExpression, VarExpression);
    With VarExpression do
      begin
        Case fpExtended in DataType of
          True: VarExpression.GenerateFPUInstructionWithExpressionInMemory ($D9 or Byte (DataType), $38);
          else  VarExpression.GenerateFPUInstructionWithExpressionInMemory ($D9 or Byte (DataType), $18);
        end;
        GenerateInstruction_TwoBytes ($CD, $3D);
        VarExpression.EndIntermediateCodeSubroutine;
      end;
  end;

  Procedure GenerateCodeForRealAssignment;
  begin
    With VarExpression do
      begin
        Case ValueExpression.Location of
          elConstant: begin
                        VarExpression.Calculate;
                        VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.Real1);
                        Inc (Value.Offset, 2);
                        VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.Real2);
                        Inc (Value.Offset, 2);
                        VarExpression.GenerateInstruction_MOV_Memory_Immediate (ValueExpression.Value.Real3);
                      end;
          else begin
                 ValueExpression.Calculate;
                 ValueExpression.LoadRealExpressionToRegisterSet (rAX_BX_DX);
                 If UsedRegisters <> [] then
                   begin
                     GenerateInstruction_TwoBytes (PUSH_DX, PUSH_BX);
                     GenerateInstruction_Byte (PUSH_AX);
                   end;
                 VarExpression.Calculate;
                 If UsedRegisters <> [] then
                   begin
                     GenerateInstruction_Byte (PUSH_AX);
                     GenerateInstruction_TwoBytes (POP_BX, POP_DX );
                   end;
                 VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rAX);
                 Inc (Value.Offset, 2);
                 VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rBX);
                 Inc (Value.Offset, 2);
                 VarExpression.GenerateInstruction_16bit_MOV_Memory_Register (rDX);
               end;
        end;
        Dec (Value.Offset, 4);
        VarExpression.EndIntermediateCodeSubroutine;
      end;
  end;

begin
  Case VarExpression.TypeDefPtr^.BaseType of
    btUntyped,
    btArray,
    btRecord,
    btFile,
    btText:        GenerateCodeForComplexAssignment;
    btObject:      GenerateCodeForObjectAssignment;
    btFunctionProcedure,
    btPointer,
    btInteger,
    btBoolean,
    btChar,
    btEnumeration: GenerateCodeForSimpleAssignment;
    btSet:         GenerateCodeForSetAssignment;
    btString:      GeneratoCodeForStringAssignment;
    btExtended:    GenerateCodeForExtendedAssignment;
    btReal:        GenerateCodeForRealAssignment;
  end;
end;
```

### Code Block 6

```pascal
Function FunctionAsResult (Var TypeDef: PTypeDefinition; Var FuncResultOffset: Integer): Boolean;
Var ProceduresBlockRecord: PProceduresBlockRecord;
    ProcedureIdentifierData: PProcedureIdentifierData absolute CurrentIdentifierDataPtr;
begin
  FunctionAsResult := False;
  If Seg (CurrentIdentifierDataPtr^) <> SymbolTable [stMain].Segment then Exit;
  If ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset.TypeOffset = 0 then Exit;
  ProceduresBlockRecord := Ptr (SymbolTable [stProcedures].Segment, ProcedureIdentifierData^.ProceduresRecordOffset);
  If ProceduresBlockRecord^.ProgramCodeBlockRecordOffset <> $FFFE then Exit;
  TypeDef := PointerFromOffsets (ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset);
  With TypeDef^ do
    begin
      FuncResultOffset := - Size;
      If BaseType = btString then SizeOfPushedParameters (CurrentIdentifierDataPtr, FuncResultOffset);
    end;
  FunctionAsResult := True;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/adjusting-expression-to-specified-type
- https://turbopascal.org/assembler
- https://turbopascal.org/assignment-statement-and-procedure-call
- https://turbopascal.org/calculator
- https://turbopascal.org/calling-a-procedure-or-function
- https://turbopascal.org/checking-ranges
- https://turbopascal.org/checking-type-compatibility
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/expecting-different-expressions
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-qualifiers
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

