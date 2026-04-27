# Calling a Procedure or Function - Turbo Pascal Compiler

**URL:** https://turbopascal.org/calling-a-procedure-or-function

---

## Table of Contents

- Calling a Procedure or Function

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Calling a Procedure or Function
Calls to all procedures, functions and methods are processed from this procedure. It takes care for pushing all parameters and generates instruction to call the procedure.
Procedure TExpression.CallProcedure;
Var ProcedureIdentifierData: PProcedureIdentifierData;
    OffsetOfMethodAddress: Word;
    MainBlockPtr: Pointer;
    ProcedureParameterData: PProcedureParameterData;
    InlineRecord: PInlineRecord absolute ProcedureParameterData;

begin
  If Location = elMemory then
    begin
      PushProcedureParameters;
      Calculate;
      GenerateInstructionWithExpressionInMemOrReg ($FF, $18);
      ES_DI_PointerDestroyed;
      EndIntermediateCodeSubroutine;
      Exit;
    end;
  ProcedureIdentifierData := PProcedureIdentifierData (PChar (TypeDefPtr) - 10);
  With ProcedureIdentifierData^ do
    begin
      If not (mcStatic in DataType) and (W8 <> 0) and not (pfConstructor in Flags) then
          begin
            PushProcedureParameters;
            PushMethodParameters;
            OffsetOfMethodAddress := MethodAddressOffset;
            If not (pfDynamic in ProcedureIdentifierData^.Flags) then
              begin
                GenerateInstructionForStackFrameDisplacement ($FF, $1D, OffsetOfMethodAddress);
                ES_DI_PointerDestroyed;
              end;
            EndIntermediateCodeSubroutine;
            Exit;
          end;
      PushProcedureParameters;
      If pfInterrupt in Flags then Error (CannotCallAnInterruptProcedure);
      If pfInline in Flags then
        begin
          GenerateIntermediateCodeForInlineRecords (InlineRecord, ProceduresRecordOffset);
          EndIntermediateCodeSubroutine;
          Exit;
        end;
      If pfMethod in Flags then PushMethodParameters else
        If OuterBlockProcedureIdentifier <> 0 then
          begin
            If OuterBlockProcedureIdentifier = CurrentProcedureIdentifier then GenerateInstruction_Byte (PUSH_BP) else
              begin
                Load_ES_DI (OuterBlockProcedureIdentifier, 0, 0);
                GenerateInstruction_Byte (PUSH_DI);
              end;
          end;
      If pfFar in Flags then
        begin
          If Seg (TypeDefPtr^) = SymbolTable [stMain].Segment then
            begin
              GenerateInstruction_Byte ($0E);
              GenerateInstruction_Byte ($E8);
              GenerateReference (Seg (TypeDefPtr^), ProceduresRecordOffset, 0, []);
            end else begin
                       GenerateInstruction_Byte ($9A);
                       GenerateReference (Seg (TypeDefPtr^), ProceduresRecordOffset, 0, [rfSegment, rfOffset]);
                     end;
        end else begin
                   GenerateInstruction_Byte ($E8);
                   GenerateReference (Seg (TypeDefPtr^), ProceduresRecordOffset, 0, []);
                 end;
      ES_DI_PointerDestroyed;
      EndIntermediateCodeSubroutine;
    end;
end;
When a procedure is called actual parameters need to bo calculated and pushed on the stack. Depending on the parameter type (called by reference – Var, open parameter or normal parameter) appropriate procedure is called.
Procedure PushProcedureParameters;
 Var ProcedureParameterDataOfs: Word absolute ProcedureParameterData;
      ParameterIntermediateCodeOffsets: TIntermediateCodeOffsets;
      ParameterExpression: Texpression;
      ParameterTypeDef: PTypeDefinition;
      Parameter: Word;
      VarFlags: TVarFlagsSet;

 begin
  ProcedureParameterData := PProcedureParameterData (PChar (TypeDefPtr) + SizeOf (TProcedureTypeDefinition));
    If PProcedureTypeDefinition (TypeDefPtr)^.NumberOfParameters <> 0 then
      begin
        ParameterIntermediateCodeOffsets.NumberOfStatements := 0;
        ExpectTokenAndGetNext (Token_LeftParenthesis);
        For Parameter := 1 to PProcedureTypeDefinition (TypeDefPtr)^.NumberOfParameters do
          begin
            ParameterTypeDef := PointerFromOffsets (ProcedureParameterData^.UnitTypeOffsets);
            VarFlags := ProcedureParameterData^.VarFlags;
            If vfOpenParameter in VarFlags then PushOpenParameter else
              If vfVar in VarFlags then PushPointerToParameter else PushParameter;
            AddCallToIntermediateCodeSubroutine (ParameterIntermediateCodeOffsets, ParameterExpression.IntermediateCodeOffset);
            Inc (ProcedureParameterData);
            If Parameter <> PProcedureTypeDefinition (TypeDefPtr)^.NumberOfParameters then ExpectTokenAndGetNext (Token_Comma);
          end;
        ExpectTokenAndGetNext (Token_RightParenthesis);
        Generate_icGoSub_ForEachSubroutine (ParameterIntermediateCodeOffsets);
      end;
  end;
This procedure pushes normal parameters. Larger parameters are not pushed directly – a pointer to the actual parameter is pushed to save stack space.
Procedure PushParameter;
   begin
      With ParameterExpression do
        begin
          CalculateExpressionWithType (ParameterTypeDef);
          AdjustExpressionToType      (ParameterTypeDef);
          CheckTypeCompatibility      (ParameterTypeDef);
          CheckRange                  (ParameterTypeDef);
          Case TypeDefPtr^.BaseType of
            btPointer,
            btFunctionProcedure,
            btInteger..btEnumeration: PushExpression;
            btSet: If it32Bytes in DataType then
                     begin
                       If IsExpressionInOverlaidCode then ExpandSetToStackFrameAndPushPointer (0);
                       ConvertToPointerAndPush;
                     end else PushExpression;
            btString: begin
                        StoreStringConstantToCodeBlock;
                        If IsExpressionInOverlaidCode then CopyStringToStackFrame ($0100);
                        ConvertToPointerAndPush;
                      end;
            btExtended: Push_FP_Expression (ParameterTypeDef);
            btReal: PushRealExpression;
            else If SetDataTypeForSimpleType then PushExpression else
                   ConvertToPointerAndPush;
          end;
        end;
    end;
For open parameters also the highest index of the array is pushed.
Procedure PushOpenParameter;
    Var StringLen: Byte; 
        ArrayTypeDefinition: PArrayTypeDefinition absolute TypeDefPtr;
    begin
      With ParameterExpression do
        begin
          If not (vfArray in VarFlags) then
            begin
              ExpectAssignableVariableReferenceExceptProcedureOrFunction;
              If TypeDefPtr^.BaseType <> btString then Error (TypeMismatch);
              PushArrayPointerAndHighestIndex;
              Exit;
            end;
          If vfVar in VarFlags then ExpectAssignableVariableReferenceExceptProcedureOrFunction else
            begin
              CalculateExpression;
              If (ParameterTypeDef = Ptr (SystemUnitSegment, Char_TypeOffset)) and IsConstantString (0, StringLen) then
                begin
                  ConvertToPointerAndPush;
                  Calculate;
                  GenerateCodeToPushWord (StringLen - 1);
                  EndIntermediateCodeSubroutine;
                  Exit;
                end;
            end;
          If Location <> elMemory then Error (InvalidVariableReference);
          If TypeDefPtr = ParameterTypeDef then
            begin
              ConvertToPointerAndPush;
              Calculate;
              GenerateCodeToPushWord (0);
              EndIntermediateCodeSubroutine;
              Exit;
            end;
          If TypeDefPtr^.BaseType <> btArray then Error (TypeMismatch);
          If PointerFromOffsets (ArrayTypeDefinition^.ElementTypeOffset) <> ParameterTypeDef then Error (TypeMismatch);
          PushArrayPointerAndHighestIndex;
        end;
    end;

  Procedure PushPointerToParameter;
   begin
      With ParameterExpression do
        begin
          If vfConst in VarFlags then ExpectVariableReferenceExceptProcedureOrFunction else
                                        ExpectAssignableVariableReferenceExceptProcedureOrFunction;
          Case ParameterTypeDef^.BaseType of
            btUntyped: ConvertToPointerAndPush;
            btString: Case StrictVarStrings in StatementCompilerSwitches of
                        True: If TypeDefPtr = ParameterTypeDef then ConvertToPointerAndPush else
                                Error (TypeMismatch);
                        else If TypeDefPtr^.BaseType = btString then ConvertToPointerAndPush else
                                Error (TypeMismatch);
                      end;
            btObject: If InObjectTypeDomain (PObjectTypeDefinition (TypeDefPtr),
                                PObjectTypeDefinition (ParameterTypeDef)) then
                        ConvertToPointerAndPush else Error (TypeMismatch);
            else If TypeDefPtr = ParameterTypeDef then
                   ConvertToPointerAndPush else Error (TypeMismatch);
          end;
        end
    end;
Methods need also Self address which is always
loaded to the ES:DI register pair
.
Procedure PushMethodParameters;
  Var ObjectTypeDefinition: PObjectTypeDefinition;
      MethodIdentifier: PIdentifier;
      MethodIdentifierData: PProcedureIdentifierData;
  begin
    ObjectTypeDefinition := Value.ObjectTypeDefinition;
    If pfConstructor in ProcedureIdentifierData^.Flags then
      begin
        If mcStatic in DataType then GenerateInstruction_Word ($C031) else
          begin
            GenerateInstruction_Byte ($B8);
            GenerateReference (Seg (ObjectTypeDefinition^),
                                   ObjectTypeDefinition^.VMT_TypedConstantsBlockRecordOffset, 0,
                                   [rfDataSegment, rfConstant, rfOffset]);
          end;
        GenerateInstruction_Byte ($50);
      end else
        If pfDestructor in ProcedureIdentifierData^.Flags then
          begin
            Case mcDispose in DataType of
              True: GenerateInstruction_Word ($01B0);
              else GenerateInstruction_Word ($C031);
            end;
            GenerateInstruction_Byte ($50);
          end;
    If mcStatic in DataType then
      begin
        If not FindCurrentMethod (MethodIdentifier, MethodIdentifierData) then Error (InvalidProcedureOrFunctionReference);
        If not InObjectTypeDomain (Ptr (Seg (MethodIdentifierData^),
                 MethodIdentifierData^.OuterBlockProcedureIdentifier), ObjectTypeDefinition) then
          Error (InvalidProcedureOrFunctionReference);
        Load_ES_DI (Ofs (MethodIdentifier^), 6, 0);
        LocationData.Flags := [ofsDI, segES];
        Value.Offset := 0;
        GenerateInstruction_Word ($5706);
        Exit;
      end;
    If mcNew in DataType then
      begin
        GenerateInstruction_Word ($C031);
        GenerateInstruction_Word ($5050);
        Exit;
      end;
    Calculate;
    GenerateInstruction_TwoBytes (Load_DI_AndReturnSegment or $06, $57);
  end;

  Function MethodAddressOffset: Word;
  Var AncestorType: PObjectTypeDefinition;
  begin
    LocationData.Flags := LocationData.Flags * [segCS, segSS, segDS] + [segES];
    GenerateSegmentOverride;
    AncestorType := Ptr (Seg (ProcedureIdentifierData^), ProcedureIdentifierData^.OuterBlockProcedureIdentifier);
    GenerateInstructionForStackFrameDisplacement ($8B, $3D, AncestorType^.OffsetOf_VMT_Offset);
    Case pfDynamic in ProcedureIdentifierData^.Flags of
      True: begin
              GenerateInstruction_Byte ($B8);
              GenerateInstruction_Word (ProcedureIdentifierData^.W8);
              GenerateInstruction_CALL_FAR (SysProc_CallMethod);
              UsedRegisters := [urBX, urDX, urCX, urAX];
              MethodAddressOffset := 0;
            end;
      else If RangeChecking in StatementCompilerSwitches then
             begin
               GenerateInstruction_CALL_FAR (SysProc_MethodCheck);
              UsedRegisters := [urBX, urDX, urCX, urAX];
              MethodAddressOffset := ProcedureIdentifierData^.W8;
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
Procedure TExpression.CallProcedure;
Var ProcedureIdentifierData: PProcedureIdentifierData;
    OffsetOfMethodAddress: Word;
    MainBlockPtr: Pointer;
    ProcedureParameterData: PProcedureParameterData;
    InlineRecord: PInlineRecord absolute ProcedureParameterData;

begin
  If Location = elMemory then
    begin
      PushProcedureParameters;
      Calculate;
      GenerateInstructionWithExpressionInMemOrReg ($FF, $18);
      ES_DI_PointerDestroyed;
      EndIntermediateCodeSubroutine;
      Exit;
    end;
  ProcedureIdentifierData := PProcedureIdentifierData (PChar (TypeDefPtr) - 10);
  With ProcedureIdentifierData^ do
    begin
      If not (mcStatic in DataType) and (W8 <> 0) and not (pfConstructor in Flags) then
          begin
            PushProcedureParameters;
            PushMethodParameters;
            OffsetOfMethodAddress := MethodAddressOffset;
            If not (pfDynamic in ProcedureIdentifierData^.Flags) then
              begin
                GenerateInstructionForStackFrameDisplacement ($FF, $1D, OffsetOfMethodAddress);
                ES_DI_PointerDestroyed;
              end;
            EndIntermediateCodeSubroutine;
            Exit;
          end;
      PushProcedureParameters;
      If pfInterrupt in Flags then Error (CannotCallAnInterruptProcedure);
      If pfInline in Flags then
        begin
          GenerateIntermediateCodeForInlineRecords (InlineRecord, ProceduresRecordOffset);
          EndIntermediateCodeSubroutine;
          Exit;
        end;
      If pfMethod in Flags then PushMethodParameters else
        If OuterBlockProcedureIdentifier <> 0 then
          begin
            If OuterBlockProcedureIdentifier = CurrentProcedureIdentifier then GenerateInstruction_Byte (PUSH_BP) else
              begin
                Load_ES_DI (OuterBlockProcedureIdentifier, 0, 0);
                GenerateInstruction_Byte (PUSH_DI);
              end;
          end;
      If pfFar in Flags then
        begin
          If Seg (TypeDefPtr^) = SymbolTable [stMain].Segment then
            begin
              GenerateInstruction_Byte ($0E);
              GenerateInstruction_Byte ($E8);
              GenerateReference (Seg (TypeDefPtr^), ProceduresRecordOffset, 0, []);
            end else begin
                       GenerateInstruction_Byte ($9A);
                       GenerateReference (Seg (TypeDefPtr^), ProceduresRecordOffset, 0, [rfSegment, rfOffset]);
                     end;
        end else begin
                   GenerateInstruction_Byte ($E8);
                   GenerateReference (Seg (TypeDefPtr^), ProceduresRecordOffset, 0, []);
                 end;
      ES_DI_PointerDestroyed;
      EndIntermediateCodeSubroutine;
    end;
end;
```

### Code Block 2

```pascal
 Procedure PushProcedureParameters;
 Var ProcedureParameterDataOfs: Word absolute ProcedureParameterData;
      ParameterIntermediateCodeOffsets: TIntermediateCodeOffsets;
      ParameterExpression: Texpression;
      ParameterTypeDef: PTypeDefinition;
      Parameter: Word;
      VarFlags: TVarFlagsSet;

 begin
  ProcedureParameterData := PProcedureParameterData (PChar (TypeDefPtr) + SizeOf (TProcedureTypeDefinition));
    If PProcedureTypeDefinition (TypeDefPtr)^.NumberOfParameters <> 0 then
      begin
        ParameterIntermediateCodeOffsets.NumberOfStatements := 0;
        ExpectTokenAndGetNext (Token_LeftParenthesis);
        For Parameter := 1 to PProcedureTypeDefinition (TypeDefPtr)^.NumberOfParameters do
          begin
            ParameterTypeDef := PointerFromOffsets (ProcedureParameterData^.UnitTypeOffsets);
            VarFlags := ProcedureParameterData^.VarFlags;
            If vfOpenParameter in VarFlags then PushOpenParameter else
              If vfVar in VarFlags then PushPointerToParameter else PushParameter;
            AddCallToIntermediateCodeSubroutine (ParameterIntermediateCodeOffsets, ParameterExpression.IntermediateCodeOffset);
            Inc (ProcedureParameterData);
            If Parameter <> PProcedureTypeDefinition (TypeDefPtr)^.NumberOfParameters then ExpectTokenAndGetNext (Token_Comma);
          end;
        ExpectTokenAndGetNext (Token_RightParenthesis);
        Generate_icGoSub_ForEachSubroutine (ParameterIntermediateCodeOffsets);
      end;
  end;
```

### Code Block 3

```pascal
  Procedure PushParameter;
   begin
      With ParameterExpression do
        begin
          CalculateExpressionWithType (ParameterTypeDef);
          AdjustExpressionToType      (ParameterTypeDef);
          CheckTypeCompatibility      (ParameterTypeDef);
          CheckRange                  (ParameterTypeDef);
          Case TypeDefPtr^.BaseType of
            btPointer,
            btFunctionProcedure,
            btInteger..btEnumeration: PushExpression;
            btSet: If it32Bytes in DataType then
                     begin
                       If IsExpressionInOverlaidCode then ExpandSetToStackFrameAndPushPointer (0);
                       ConvertToPointerAndPush;
                     end else PushExpression;
            btString: begin
                        StoreStringConstantToCodeBlock;
                        If IsExpressionInOverlaidCode then CopyStringToStackFrame ($0100);
                        ConvertToPointerAndPush;
                      end;
            btExtended: Push_FP_Expression (ParameterTypeDef);
            btReal: PushRealExpression;
            else If SetDataTypeForSimpleType then PushExpression else
                   ConvertToPointerAndPush;
          end;
        end;
    end;
```

### Code Block 4

```pascal
   Procedure PushOpenParameter;
    Var StringLen: Byte; 
        ArrayTypeDefinition: PArrayTypeDefinition absolute TypeDefPtr;
    begin
      With ParameterExpression do
        begin
          If not (vfArray in VarFlags) then
            begin
              ExpectAssignableVariableReferenceExceptProcedureOrFunction;
              If TypeDefPtr^.BaseType <> btString then Error (TypeMismatch);
              PushArrayPointerAndHighestIndex;
              Exit;
            end;
          If vfVar in VarFlags then ExpectAssignableVariableReferenceExceptProcedureOrFunction else
            begin
              CalculateExpression;
              If (ParameterTypeDef = Ptr (SystemUnitSegment, Char_TypeOffset)) and IsConstantString (0, StringLen) then
                begin
                  ConvertToPointerAndPush;
                  Calculate;
                  GenerateCodeToPushWord (StringLen - 1);
                  EndIntermediateCodeSubroutine;
                  Exit;
                end;
            end;
          If Location <> elMemory then Error (InvalidVariableReference);
          If TypeDefPtr = ParameterTypeDef then
            begin
              ConvertToPointerAndPush;
              Calculate;
              GenerateCodeToPushWord (0);
              EndIntermediateCodeSubroutine;
              Exit;
            end;
          If TypeDefPtr^.BaseType <> btArray then Error (TypeMismatch);
          If PointerFromOffsets (ArrayTypeDefinition^.ElementTypeOffset) <> ParameterTypeDef then Error (TypeMismatch);
          PushArrayPointerAndHighestIndex;
        end;
    end;

  Procedure PushPointerToParameter;
   begin
      With ParameterExpression do
        begin
          If vfConst in VarFlags then ExpectVariableReferenceExceptProcedureOrFunction else
                                        ExpectAssignableVariableReferenceExceptProcedureOrFunction;
          Case ParameterTypeDef^.BaseType of
            btUntyped: ConvertToPointerAndPush;
            btString: Case StrictVarStrings in StatementCompilerSwitches of
                        True: If TypeDefPtr = ParameterTypeDef then ConvertToPointerAndPush else
                                Error (TypeMismatch);
                        else If TypeDefPtr^.BaseType = btString then ConvertToPointerAndPush else
                                Error (TypeMismatch);
                      end;
            btObject: If InObjectTypeDomain (PObjectTypeDefinition (TypeDefPtr),
                                PObjectTypeDefinition (ParameterTypeDef)) then
                        ConvertToPointerAndPush else Error (TypeMismatch);
            else If TypeDefPtr = ParameterTypeDef then
                   ConvertToPointerAndPush else Error (TypeMismatch);
          end;
        end
    end;
```

### Code Block 5

```pascal
  Procedure PushMethodParameters;
  Var ObjectTypeDefinition: PObjectTypeDefinition;
      MethodIdentifier: PIdentifier;
      MethodIdentifierData: PProcedureIdentifierData;
  begin
    ObjectTypeDefinition := Value.ObjectTypeDefinition;
    If pfConstructor in ProcedureIdentifierData^.Flags then
      begin
        If mcStatic in DataType then GenerateInstruction_Word ($C031) else
          begin
            GenerateInstruction_Byte ($B8);
            GenerateReference (Seg (ObjectTypeDefinition^),
                                   ObjectTypeDefinition^.VMT_TypedConstantsBlockRecordOffset, 0,
                                   [rfDataSegment, rfConstant, rfOffset]);
          end;
        GenerateInstruction_Byte ($50);
      end else
        If pfDestructor in ProcedureIdentifierData^.Flags then
          begin
            Case mcDispose in DataType of
              True: GenerateInstruction_Word ($01B0);
              else GenerateInstruction_Word ($C031);
            end;
            GenerateInstruction_Byte ($50);
          end;
    If mcStatic in DataType then
      begin
        If not FindCurrentMethod (MethodIdentifier, MethodIdentifierData) then Error (InvalidProcedureOrFunctionReference);
        If not InObjectTypeDomain (Ptr (Seg (MethodIdentifierData^),
                 MethodIdentifierData^.OuterBlockProcedureIdentifier), ObjectTypeDefinition) then
          Error (InvalidProcedureOrFunctionReference);
        Load_ES_DI (Ofs (MethodIdentifier^), 6, 0);
        LocationData.Flags := [ofsDI, segES];
        Value.Offset := 0;
        GenerateInstruction_Word ($5706);
        Exit;
      end;
    If mcNew in DataType then
      begin
        GenerateInstruction_Word ($C031);
        GenerateInstruction_Word ($5050);
        Exit;
      end;
    Calculate;
    GenerateInstruction_TwoBytes (Load_DI_AndReturnSegment or $06, $57);
  end;

  Function MethodAddressOffset: Word;
  Var AncestorType: PObjectTypeDefinition;
  begin
    LocationData.Flags := LocationData.Flags * [segCS, segSS, segDS] + [segES];
    GenerateSegmentOverride;
    AncestorType := Ptr (Seg (ProcedureIdentifierData^), ProcedureIdentifierData^.OuterBlockProcedureIdentifier);
    GenerateInstructionForStackFrameDisplacement ($8B, $3D, AncestorType^.OffsetOf_VMT_Offset);
    Case pfDynamic in ProcedureIdentifierData^.Flags of
      True: begin
              GenerateInstruction_Byte ($B8);
              GenerateInstruction_Word (ProcedureIdentifierData^.W8);
              GenerateInstruction_CALL_FAR (SysProc_CallMethod);
              UsedRegisters := [urBX, urDX, urCX, urAX];
              MethodAddressOffset := 0;
            end;
      else If RangeChecking in StatementCompilerSwitches then
             begin
               GenerateInstruction_CALL_FAR (SysProc_MethodCheck);
              UsedRegisters := [urBX, urDX, urCX, urAX];
              MethodAddressOffset := ProcedureIdentifierData^.W8;
             end;
    end;
  end;
```

### Code Block 6

```pascal
Procedure TExpression.CallProcedure;
Var ProcedureIdentifierData: PProcedureIdentifierData;
    OffsetOfMethodAddress: Word;
    MainBlockPtr: Pointer;
    ProcedureParameterData: PProcedureParameterData;
    InlineRecord: PInlineRecord absolute ProcedureParameterData;

begin
  If Location = elMemory then
    begin
      PushProcedureParameters;
      Calculate;
      GenerateInstructionWithExpressionInMemOrReg ($FF, $18);
      ES_DI_PointerDestroyed;
      EndIntermediateCodeSubroutine;
      Exit;
    end;
  ProcedureIdentifierData := PProcedureIdentifierData (PChar (TypeDefPtr) - 10);
  With ProcedureIdentifierData^ do
    begin
      If not (mcStatic in DataType) and (W8 <> 0) and not (pfConstructor in Flags) then
          begin
            PushProcedureParameters;
            PushMethodParameters;
            OffsetOfMethodAddress := MethodAddressOffset;
            If not (pfDynamic in ProcedureIdentifierData^.Flags) then
              begin
                GenerateInstructionForStackFrameDisplacement ($FF, $1D, OffsetOfMethodAddress);
                ES_DI_PointerDestroyed;
              end;
            EndIntermediateCodeSubroutine;
            Exit;
          end;
      PushProcedureParameters;
      If pfInterrupt in Flags then Error (CannotCallAnInterruptProcedure);
      If pfInline in Flags then
        begin
          GenerateIntermediateCodeForInlineRecords (InlineRecord, ProceduresRecordOffset);
          EndIntermediateCodeSubroutine;
          Exit;
        end;
      If pfMethod in Flags then PushMethodParameters else
        If OuterBlockProcedureIdentifier <> 0 then
          begin
            If OuterBlockProcedureIdentifier = CurrentProcedureIdentifier then GenerateInstruction_Byte (PUSH_BP) else
              begin
                Load_ES_DI (OuterBlockProcedureIdentifier, 0, 0);
                GenerateInstruction_Byte (PUSH_DI);
              end;
          end;
      If pfFar in Flags then
        begin
          If Seg (TypeDefPtr^) = SymbolTable [stMain].Segment then
            begin
              GenerateInstruction_Byte ($0E);
              GenerateInstruction_Byte ($E8);
              GenerateReference (Seg (TypeDefPtr^), ProceduresRecordOffset, 0, []);
            end else begin
                       GenerateInstruction_Byte ($9A);
                       GenerateReference (Seg (TypeDefPtr^), ProceduresRecordOffset, 0, [rfSegment, rfOffset]);
                     end;
        end else begin
                   GenerateInstruction_Byte ($E8);
                   GenerateReference (Seg (TypeDefPtr^), ProceduresRecordOffset, 0, []);
                 end;
      ES_DI_PointerDestroyed;
      EndIntermediateCodeSubroutine;
    end;
end;
```

### Code Block 7

```pascal
 Procedure PushProcedureParameters;
 Var ProcedureParameterDataOfs: Word absolute ProcedureParameterData;
      ParameterIntermediateCodeOffsets: TIntermediateCodeOffsets;
      ParameterExpression: Texpression;
      ParameterTypeDef: PTypeDefinition;
      Parameter: Word;
      VarFlags: TVarFlagsSet;

 begin
  ProcedureParameterData := PProcedureParameterData (PChar (TypeDefPtr) + SizeOf (TProcedureTypeDefinition));
    If PProcedureTypeDefinition (TypeDefPtr)^.NumberOfParameters <> 0 then
      begin
        ParameterIntermediateCodeOffsets.NumberOfStatements := 0;
        ExpectTokenAndGetNext (Token_LeftParenthesis);
        For Parameter := 1 to PProcedureTypeDefinition (TypeDefPtr)^.NumberOfParameters do
          begin
            ParameterTypeDef := PointerFromOffsets (ProcedureParameterData^.UnitTypeOffsets);
            VarFlags := ProcedureParameterData^.VarFlags;
            If vfOpenParameter in VarFlags then PushOpenParameter else
              If vfVar in VarFlags then PushPointerToParameter else PushParameter;
            AddCallToIntermediateCodeSubroutine (ParameterIntermediateCodeOffsets, ParameterExpression.IntermediateCodeOffset);
            Inc (ProcedureParameterData);
            If Parameter <> PProcedureTypeDefinition (TypeDefPtr)^.NumberOfParameters then ExpectTokenAndGetNext (Token_Comma);
          end;
        ExpectTokenAndGetNext (Token_RightParenthesis);
        Generate_icGoSub_ForEachSubroutine (ParameterIntermediateCodeOffsets);
      end;
  end;
```

### Code Block 8

```pascal
  Procedure PushParameter;
   begin
      With ParameterExpression do
        begin
          CalculateExpressionWithType (ParameterTypeDef);
          AdjustExpressionToType      (ParameterTypeDef);
          CheckTypeCompatibility      (ParameterTypeDef);
          CheckRange                  (ParameterTypeDef);
          Case TypeDefPtr^.BaseType of
            btPointer,
            btFunctionProcedure,
            btInteger..btEnumeration: PushExpression;
            btSet: If it32Bytes in DataType then
                     begin
                       If IsExpressionInOverlaidCode then ExpandSetToStackFrameAndPushPointer (0);
                       ConvertToPointerAndPush;
                     end else PushExpression;
            btString: begin
                        StoreStringConstantToCodeBlock;
                        If IsExpressionInOverlaidCode then CopyStringToStackFrame ($0100);
                        ConvertToPointerAndPush;
                      end;
            btExtended: Push_FP_Expression (ParameterTypeDef);
            btReal: PushRealExpression;
            else If SetDataTypeForSimpleType then PushExpression else
                   ConvertToPointerAndPush;
          end;
        end;
    end;
```

### Code Block 9

```pascal
   Procedure PushOpenParameter;
    Var StringLen: Byte; 
        ArrayTypeDefinition: PArrayTypeDefinition absolute TypeDefPtr;
    begin
      With ParameterExpression do
        begin
          If not (vfArray in VarFlags) then
            begin
              ExpectAssignableVariableReferenceExceptProcedureOrFunction;
              If TypeDefPtr^.BaseType <> btString then Error (TypeMismatch);
              PushArrayPointerAndHighestIndex;
              Exit;
            end;
          If vfVar in VarFlags then ExpectAssignableVariableReferenceExceptProcedureOrFunction else
            begin
              CalculateExpression;
              If (ParameterTypeDef = Ptr (SystemUnitSegment, Char_TypeOffset)) and IsConstantString (0, StringLen) then
                begin
                  ConvertToPointerAndPush;
                  Calculate;
                  GenerateCodeToPushWord (StringLen - 1);
                  EndIntermediateCodeSubroutine;
                  Exit;
                end;
            end;
          If Location <> elMemory then Error (InvalidVariableReference);
          If TypeDefPtr = ParameterTypeDef then
            begin
              ConvertToPointerAndPush;
              Calculate;
              GenerateCodeToPushWord (0);
              EndIntermediateCodeSubroutine;
              Exit;
            end;
          If TypeDefPtr^.BaseType <> btArray then Error (TypeMismatch);
          If PointerFromOffsets (ArrayTypeDefinition^.ElementTypeOffset) <> ParameterTypeDef then Error (TypeMismatch);
          PushArrayPointerAndHighestIndex;
        end;
    end;

  Procedure PushPointerToParameter;
   begin
      With ParameterExpression do
        begin
          If vfConst in VarFlags then ExpectVariableReferenceExceptProcedureOrFunction else
                                        ExpectAssignableVariableReferenceExceptProcedureOrFunction;
          Case ParameterTypeDef^.BaseType of
            btUntyped: ConvertToPointerAndPush;
            btString: Case StrictVarStrings in StatementCompilerSwitches of
                        True: If TypeDefPtr = ParameterTypeDef then ConvertToPointerAndPush else
                                Error (TypeMismatch);
                        else If TypeDefPtr^.BaseType = btString then ConvertToPointerAndPush else
                                Error (TypeMismatch);
                      end;
            btObject: If InObjectTypeDomain (PObjectTypeDefinition (TypeDefPtr),
                                PObjectTypeDefinition (ParameterTypeDef)) then
                        ConvertToPointerAndPush else Error (TypeMismatch);
            else If TypeDefPtr = ParameterTypeDef then
                   ConvertToPointerAndPush else Error (TypeMismatch);
          end;
        end
    end;
```

### Code Block 10

```pascal
  Procedure PushMethodParameters;
  Var ObjectTypeDefinition: PObjectTypeDefinition;
      MethodIdentifier: PIdentifier;
      MethodIdentifierData: PProcedureIdentifierData;
  begin
    ObjectTypeDefinition := Value.ObjectTypeDefinition;
    If pfConstructor in ProcedureIdentifierData^.Flags then
      begin
        If mcStatic in DataType then GenerateInstruction_Word ($C031) else
          begin
            GenerateInstruction_Byte ($B8);
            GenerateReference (Seg (ObjectTypeDefinition^),
                                   ObjectTypeDefinition^.VMT_TypedConstantsBlockRecordOffset, 0,
                                   [rfDataSegment, rfConstant, rfOffset]);
          end;
        GenerateInstruction_Byte ($50);
      end else
        If pfDestructor in ProcedureIdentifierData^.Flags then
          begin
            Case mcDispose in DataType of
              True: GenerateInstruction_Word ($01B0);
              else GenerateInstruction_Word ($C031);
            end;
            GenerateInstruction_Byte ($50);
          end;
    If mcStatic in DataType then
      begin
        If not FindCurrentMethod (MethodIdentifier, MethodIdentifierData) then Error (InvalidProcedureOrFunctionReference);
        If not InObjectTypeDomain (Ptr (Seg (MethodIdentifierData^),
                 MethodIdentifierData^.OuterBlockProcedureIdentifier), ObjectTypeDefinition) then
          Error (InvalidProcedureOrFunctionReference);
        Load_ES_DI (Ofs (MethodIdentifier^), 6, 0);
        LocationData.Flags := [ofsDI, segES];
        Value.Offset := 0;
        GenerateInstruction_Word ($5706);
        Exit;
      end;
    If mcNew in DataType then
      begin
        GenerateInstruction_Word ($C031);
        GenerateInstruction_Word ($5050);
        Exit;
      end;
    Calculate;
    GenerateInstruction_TwoBytes (Load_DI_AndReturnSegment or $06, $57);
  end;

  Function MethodAddressOffset: Word;
  Var AncestorType: PObjectTypeDefinition;
  begin
    LocationData.Flags := LocationData.Flags * [segCS, segSS, segDS] + [segES];
    GenerateSegmentOverride;
    AncestorType := Ptr (Seg (ProcedureIdentifierData^), ProcedureIdentifierData^.OuterBlockProcedureIdentifier);
    GenerateInstructionForStackFrameDisplacement ($8B, $3D, AncestorType^.OffsetOf_VMT_Offset);
    Case pfDynamic in ProcedureIdentifierData^.Flags of
      True: begin
              GenerateInstruction_Byte ($B8);
              GenerateInstruction_Word (ProcedureIdentifierData^.W8);
              GenerateInstruction_CALL_FAR (SysProc_CallMethod);
              UsedRegisters := [urBX, urDX, urCX, urAX];
              MethodAddressOffset := 0;
            end;
      else If RangeChecking in StatementCompilerSwitches then
             begin
               GenerateInstruction_CALL_FAR (SysProc_MethodCheck);
              UsedRegisters := [urBX, urDX, urCX, urAX];
              MethodAddressOffset := ProcedureIdentifierData^.W8;
             end;
    end;
  end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/calling-a-procedure-or-function
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/es-di-address-loading
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

