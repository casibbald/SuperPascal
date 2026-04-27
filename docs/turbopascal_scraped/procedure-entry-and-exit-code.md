# Procedure Entry and Exit Code - Turbo Pascal Compiler

**URL:** https://turbopascal.org/procedure-entry-and-exit-code

---

## Table of Contents

- Procedure Entry and Exit Code

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Procedure Entry and Exit Code
Procedures and functions (including methods) also need initialization and finalization code.
This function generates entry code:
it generates code to
create stack frame
if needed
it generates code to copy all parameters by value
for constructor method it generates code to construct the object
Function GenerateProcedureEntryCode: Word;
Var NegativeSizeOfPassedParameters, StackFrameOffset: Word;
    DS_Changed: Boolean;
    N: Word;
    ProcedureIdentifierData: PProcedureIdentifierData;
    ProcedureParameterData: PProcedureParameterData;
    ParameterTypeDef: PTypeDefinition;
    ParameterTypeDefRec: PtrRec absolute ParameterTypeDef;

  Procedure CheckAndCopyParameterValue (TypeDef: PTypeDefinition; ParameterFlags: TVarFlagsSet);
  Var ValueParameterCopySize, OffsetTo_VMT_Offset: Word;
      ObjectTypeDefinition: PObjectTypeDefinition absolute TypeDef;

    Procedure CopyParameterValue;
    Var DataOffset: Byte;

      Procedure Multiply_AX (Factor: Word);
      Var Expression: TExpression;
      begin
        With Expression do
          begin
            Location := elRegister;
            DataType := itWord;
            UsedRegisters := [urAX];
            LocationData.Register := rAX;
            IntegerMultiplicationWithConstant (rAX, Factor);
          end;
      end;

    begin
      If not DS_Changed then
        begin
          DS_Changed := True;
          GenerateInstruction_Word ($D38C);
          GenerateInstruction_Word ($C38E);
          GenerateInstruction_Word ($DB8C);
          GenerateInstruction_Byte ($FC);
        end;
      If not (vfOpenParameter in ParameterFlags) then
        begin
          GenerateInstructionForStackFrameDisplacement ($8D, $3E, StackFrameOffset);
          GenerateInstructionForStackFrameDisplacement ($C5, $36, NegativeSizeOfPassedParameters);
          Case TypeDef^.BaseType of
            btString: begin
                        GenerateInstruction_Byte ($AC);
                        If TypeDef^.Size <> 0 then
                          begin
                            GenerateInstruction_TwoBytes ($3C, TypeDef^.Size - 1);
                            GenerateInstruction_Word ($0272);
                            GenerateInstruction_TwoBytes ($B0, TypeDef^.Size - 1);
                          end;
                        GenerateInstruction_Word ($91AA);
                        GenerateInstruction_Word ($ED30);
                      end;
            btSet: begin
                     PSetTypeDefinition (TypeDef)^.GetSetSizeAndLowestElementDataOffset (DataOffset);
                     If DataOffset <> 0 then
                       begin
                         GenerateInstruction_Word ($C683);
                         GenerateInstruction_Byte (DataOffset);
                       end;
                     GenerateInstruction_Byte ($B9);
                     GenerateInstruction_Word (TypeDef^.Size);
                   end;
            else begin
                   GenerateInstruction_Byte ($B9);
                   GenerateInstruction_Word (TypeDef^.Size);
                 end;
          end;
          GenerateInstruction_Word ($A4F3);
          Exit;
        end;
      GenerateInstructionForStackFrameDisplacement ($8B, $06, NegativeSizeOfPassedParameters);
      GenerateInstruction_Byte ($40);
      Multiply_AX (TypeDef^.Size);
      GenerateInstruction_Word ($C189);
      If StackChecking in StatementCompilerSwitches then
        begin
          GenerateInstruction_Word ($DB8E);
          GenerateInstruction_CALL_FAR (SysProc_StackCheck);
        end;
      GenerateInstruction_Word ($CC29);
      If TypeDef^.Size = 1 then
        begin
          GenerateInstruction_Word ($E483);
          GenerateInstruction_Byte ($FE);
        end;
      GenerateInstruction_Word ($E789);
      GenerateInstructionForStackFrameDisplacement ($C5, $36, NegativeSizeOfPassedParameters + 2);
      GenerateInstructionForStackFrameDisplacement ($89, $3E, NegativeSizeOfPassedParameters + 2);
      GenerateInstructionForStackFrameDisplacement ($8C, $06, NegativeSizeOfPassedParameters + 4);
      Inc (NumberOfLocalParameters);
      GenerateInstruction_Word ($A4F3);
    end;

  begin
    Dec (NegativeSizeOfPassedParameters, SizeOfPassedParameter (TypeDef, ParameterFlags, ValueParameterCopySize, False));
    Dec (StackFrameOffset, ValueParameterCopySize);
    If not (vfOpenParameter in ParameterFlags) or (ParameterFlags * [vfConst, vfVar] <> []) then
      begin
        If ValueParameterCopySize > 0 then
          begin
            If (ValueParameterCopySize <> 1) and (WordAlignment in ModuleCompilerSwitches) then
              StackFrameOffset := StackFrameOffset and $FFFE;
            CopyParameterValue;
          end;
      end else CopyParameterValue;
    If (ParameterFlags * [vfOpenParameter, vfVar] = []) and (TypeDef^.BaseType = btObject) and
       (ObjectTypeDefinition^.VMT_Size <> 0) then
      begin
        If ValueParameterCopySize <> 0 then
          OffsetTo_VMT_Offset := StackFrameOffset else
            OffsetTo_VMT_Offset := NegativeSizeOfPassedParameters;
        Inc (OffsetTo_VMT_Offset, PObjectTypeDefinition (TypeDef)^.OffsetOf_VMT_Offset);
        GenerateInstructionForStackFrameDisplacement (MOV_RegisterOrMemory_Immediate_16Bit,
                                               rmSS_BP_Displacement,
                                               OffsetTo_VMT_Offset);
      GenerateReference (Seg (TypeDef^), PObjectTypeDefinition (TypeDef)^.VMT_TypedConstantsBlockRecordOffset, 0,
                             [rfDataSegment, rfConstant, rfOffset]);
      end;
  end;

  Procedure ConstructObject (OffsetOf_VMT_Offset: Word);
  begin
    GenerateInstruction_MOV_16BitReg_Immediate (rDI, OffsetOf_VMT_Offset);
    GenerateInstruction_CALL_FAR (SysProc_Construct);
    GenerateCodeForNearJump (LastJumpToProgramBlockExit, $74);
  end;

begin
  ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
  With ProcedureIdentifierData^ do
    begin
      If pfStackFrame in Flags then CreateStackFrame;
      If (ProcedureTypeDefinition.NumberOfParameters <> 0) and not (pfAssembler in Flags) then
        begin
          NegativeSizeOfPassedParameters := OffsetAfterLastParameter;
          StackFrameOffset := FunctionResultNegativeSize;
          DS_Changed := False;
          ProcedureParameterData := PProcedureParameterData (PChar (ProcedureIdentifierData) + 10 + 14);
          For N := 1 to ProcedureTypeDefinition.NumberOfParameters do
            begin
              ParameterTypeDefRec.Seg := PUnitIdentifierData (Ptr (Seg (ProcedureParameterData^),
                                           ProcedureParameterData^.UnitTypeOffsets.UnitIdentifierData))^.UnitSegment;
              ParameterTypeDefRec.Ofs := ProcedureParameterData^.UnitTypeOffsets.TypeOffset;
              CheckAndCopyParameterValue (ParameterTypeDef, ProcedureParameterData^.VarFlags);
              Inc (ProcedureParameterData);
            end;
          If DS_Changed then GenerateInstruction_Word (MOV_DS_BX);
        end;
      If pfConstructor in Flags then
        ConstructObject (PObjectTypeDefinition (Ptr (Seg (ProcedureIdentifierData^),
                                                          OuterBlockProcedureIdentifier))^.OffsetOf_VMT_Offset);
    end;
  GenerateProcedureEntryCode := EndSubroutine
end;
This function generates exit code for procedure or function:
for constructors: a pointer to object is loaded to
DX:AX
register pair
for destructors:
SysProc_Destruct
is called to deallocate the object from heap
for functions: the result is loaded to registers or FPU stack
for all procedures it also
destroyes stack frame
Function GenerateProcedureExitCode: Word;
Var ProcedureIdentifierData: PProcedureIdentifierData;
    ResultType: PTypeDefinition;
    FOpCode: WordRec;
    ReturnOpCode: Byte;

  Procedure LoadFunctionResultToRegisters (StackFrameOffset: ShortInt; ReturnSize: Word);
  begin
    Case ReturnSize of
      1: GenerateInstruction_Word ($468A);
      2: GenerateInstruction_Word ($468B);
      4: begin
           GenerateInstruction_Word ($468B);
           GenerateInstruction_Byte (StackFrameOffset);
           Inc (StackFrameOffset, 2);
           GenerateInstruction_Word ($568B);
         end;
      else begin
             GenerateInstruction_Word ($468B);
             GenerateInstruction_Byte (StackFrameOffset);
             Inc (StackFrameOffset, 2);
             GenerateInstruction_Word ($5E8B);
             GenerateInstruction_Byte (StackFrameOffset);
             Inc (StackFrameOffset, 2);
             GenerateInstruction_Word ($568B);
           end;
    end;
    GenerateInstruction_Byte (StackFrameOffset);
  end;

begin
  ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
  With ProcedureIdentifierData^ do
    begin
      If pfConstructor in Flags then
        begin
          GenerateInstruction_Word ($46C4);
          GenerateInstruction_Byte ($06);
          GenerateInstruction_Word ($C28C);
        end else If pfDestructor in Flags then
          begin
            GenerateInstruction_MOV_16BitReg_Immediate (rDI,
              PObjectTypeDefinition (Ptr (Seg (ProcedureIdentifierData^),
                                          OuterBlockProcedureIdentifier))^.OffsetOf_VMT_Offset);
            GenerateInstruction_CALL_FAR (SysProc_Destruct);
          end;
      If (ProcedureTypeDefinition.ResultTypeOffset.UnitIdentifierData <> 0) and not (pfAssembler in Flags) then
        begin
          ResultType := PointerFromOffsets (ProcedureTypeDefinition.ResultTypeOffset);
          Case ResultType^.BaseType of
            btPointer,
            btReal,
            btInteger..btEnumeration: LoadFunctionResultToRegisters (FunctionResultNegativeSize, ResultType^.Size);
            btExtended: begin
                          FOpCode.Word := $06D9 or Byte (ResultType^.DataType);
                          If fpExtended in ResultType^.DataType then FOpCode.ByteH := $2E;
                          GenerateFPUInstructionForStackFrameDisplacement (FOpCode.Word, FunctionResultNegativeSize);
                          GenerateInstruction_TwoBytes ($CD, $3D);
                        end;
          end;
        end;
      If pfStackFrame in Flags then DestroyStackFrame;
      If pfFar in Flags then ReturnOpCode := $CB else ReturnOpCode := $C3;
      Case PushedParametersSize of
        0: GenerateInstruction_Byte (ReturnOpCode);
        else begin
               GenerateInstruction_Byte (ReturnOpCode - 1);
               GenerateInstruction_Word (PushedParametersSize);
             end;
      end;
    end;
  GenerateProcedureExitCode := EndSubroutine;
end;
This function returns the size of passed parameter and the size of value parameter that needs to be copied to stack frame.
Function SizeOfPassedParameter (Var TypeDef: PTypeDefinition; Var ParameterFlags: TVarFlagsSet;
                                Var ValueParameterCopySize: Word; AssemblerProcedure: Boolean): Word;
Var Size: Word;
    ExitFlag: Boolean;

  Procedure PassPointer;
  begin
    ExitFlag := True;
    If (vfConst in ParameterFlags) or AssemblerProcedure then
      begin
        Include (ParameterFlags, vfVar);
        If TypeDef^.BaseType <> btSet then SizeOfPassedParameter := 4 else
          begin
            TypeDef := PointerFromOffsets (POrdinalTypeDefinition (
                         PointerFromOffsets (PSetTypeDefinition (TypeDef)^.BaseSetTypeOffset))^.OrdinalType);
            SizeOfPassedParameter := 4;
          end;
      end else begin
                 ValueParameterCopySize := Size;
                 SizeOfPassedParameter := 4;
               end;
  end;

begin
  ExitFlag := False;
  ValueParameterCopySize := 0;
  If vfOpenParameter in ParameterFlags then SizeOfPassedParameter := 6 else
    If vfVar in ParameterFlags then SizeOfPassedParameter := 4 else
      begin
        Size := TypeDef^.Size;
        Case TypeDef^.BaseType of
          btUntyped..btText: Case Size of
                               1, 2, 4:
                               else PassPointer;
                             end;
          btSet: If it32Bytes in TypeDef^.DataType then PassPointer;
          btString: PassPointer;
        end;
        If not ExitFlag then SizeOfPassedParameter := (Size + 1) and $FFFE;
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
Function GenerateProcedureEntryCode: Word;
Var NegativeSizeOfPassedParameters, StackFrameOffset: Word;
    DS_Changed: Boolean;
    N: Word;
    ProcedureIdentifierData: PProcedureIdentifierData;
    ProcedureParameterData: PProcedureParameterData;
    ParameterTypeDef: PTypeDefinition;
    ParameterTypeDefRec: PtrRec absolute ParameterTypeDef;

  Procedure CheckAndCopyParameterValue (TypeDef: PTypeDefinition; ParameterFlags: TVarFlagsSet);
  Var ValueParameterCopySize, OffsetTo_VMT_Offset: Word;
      ObjectTypeDefinition: PObjectTypeDefinition absolute TypeDef;

    Procedure CopyParameterValue;
    Var DataOffset: Byte;

      Procedure Multiply_AX (Factor: Word);
      Var Expression: TExpression;
      begin
        With Expression do
          begin
            Location := elRegister;
            DataType := itWord;
            UsedRegisters := [urAX];
            LocationData.Register := rAX;
            IntegerMultiplicationWithConstant (rAX, Factor);
          end;
      end;

    begin
      If not DS_Changed then
        begin
          DS_Changed := True;
          GenerateInstruction_Word ($D38C);
          GenerateInstruction_Word ($C38E);
          GenerateInstruction_Word ($DB8C);
          GenerateInstruction_Byte ($FC);
        end;
      If not (vfOpenParameter in ParameterFlags) then
        begin
          GenerateInstructionForStackFrameDisplacement ($8D, $3E, StackFrameOffset);
          GenerateInstructionForStackFrameDisplacement ($C5, $36, NegativeSizeOfPassedParameters);
          Case TypeDef^.BaseType of
            btString: begin
                        GenerateInstruction_Byte ($AC);
                        If TypeDef^.Size <> 0 then
                          begin
                            GenerateInstruction_TwoBytes ($3C, TypeDef^.Size - 1);
                            GenerateInstruction_Word ($0272);
                            GenerateInstruction_TwoBytes ($B0, TypeDef^.Size - 1);
                          end;
                        GenerateInstruction_Word ($91AA);
                        GenerateInstruction_Word ($ED30);
                      end;
            btSet: begin
                     PSetTypeDefinition (TypeDef)^.GetSetSizeAndLowestElementDataOffset (DataOffset);
                     If DataOffset <> 0 then
                       begin
                         GenerateInstruction_Word ($C683);
                         GenerateInstruction_Byte (DataOffset);
                       end;
                     GenerateInstruction_Byte ($B9);
                     GenerateInstruction_Word (TypeDef^.Size);
                   end;
            else begin
                   GenerateInstruction_Byte ($B9);
                   GenerateInstruction_Word (TypeDef^.Size);
                 end;
          end;
          GenerateInstruction_Word ($A4F3);
          Exit;
        end;
      GenerateInstructionForStackFrameDisplacement ($8B, $06, NegativeSizeOfPassedParameters);
      GenerateInstruction_Byte ($40);
      Multiply_AX (TypeDef^.Size);
      GenerateInstruction_Word ($C189);
      If StackChecking in StatementCompilerSwitches then
        begin
          GenerateInstruction_Word ($DB8E);
          GenerateInstruction_CALL_FAR (SysProc_StackCheck);
        end;
      GenerateInstruction_Word ($CC29);
      If TypeDef^.Size = 1 then
        begin
          GenerateInstruction_Word ($E483);
          GenerateInstruction_Byte ($FE);
        end;
      GenerateInstruction_Word ($E789);
      GenerateInstructionForStackFrameDisplacement ($C5, $36, NegativeSizeOfPassedParameters + 2);
      GenerateInstructionForStackFrameDisplacement ($89, $3E, NegativeSizeOfPassedParameters + 2);
      GenerateInstructionForStackFrameDisplacement ($8C, $06, NegativeSizeOfPassedParameters + 4);
      Inc (NumberOfLocalParameters);
      GenerateInstruction_Word ($A4F3);
    end;

  begin
    Dec (NegativeSizeOfPassedParameters, SizeOfPassedParameter (TypeDef, ParameterFlags, ValueParameterCopySize, False));
    Dec (StackFrameOffset, ValueParameterCopySize);
    If not (vfOpenParameter in ParameterFlags) or (ParameterFlags * [vfConst, vfVar] <> []) then
      begin
        If ValueParameterCopySize > 0 then
          begin
            If (ValueParameterCopySize <> 1) and (WordAlignment in ModuleCompilerSwitches) then
              StackFrameOffset := StackFrameOffset and $FFFE;
            CopyParameterValue;
          end;
      end else CopyParameterValue;
    If (ParameterFlags * [vfOpenParameter, vfVar] = []) and (TypeDef^.BaseType = btObject) and
       (ObjectTypeDefinition^.VMT_Size <> 0) then
      begin
        If ValueParameterCopySize <> 0 then
          OffsetTo_VMT_Offset := StackFrameOffset else
            OffsetTo_VMT_Offset := NegativeSizeOfPassedParameters;
        Inc (OffsetTo_VMT_Offset, PObjectTypeDefinition (TypeDef)^.OffsetOf_VMT_Offset);
        GenerateInstructionForStackFrameDisplacement (MOV_RegisterOrMemory_Immediate_16Bit,
                                               rmSS_BP_Displacement,
                                               OffsetTo_VMT_Offset);
      GenerateReference (Seg (TypeDef^), PObjectTypeDefinition (TypeDef)^.VMT_TypedConstantsBlockRecordOffset, 0,
                             [rfDataSegment, rfConstant, rfOffset]);
      end;
  end;

  Procedure ConstructObject (OffsetOf_VMT_Offset: Word);
  begin
    GenerateInstruction_MOV_16BitReg_Immediate (rDI, OffsetOf_VMT_Offset);
    GenerateInstruction_CALL_FAR (SysProc_Construct);
    GenerateCodeForNearJump (LastJumpToProgramBlockExit, $74);
  end;

begin
  ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
  With ProcedureIdentifierData^ do
    begin
      If pfStackFrame in Flags then CreateStackFrame;
      If (ProcedureTypeDefinition.NumberOfParameters <> 0) and not (pfAssembler in Flags) then
        begin
          NegativeSizeOfPassedParameters := OffsetAfterLastParameter;
          StackFrameOffset := FunctionResultNegativeSize;
          DS_Changed := False;
          ProcedureParameterData := PProcedureParameterData (PChar (ProcedureIdentifierData) + 10 + 14);
          For N := 1 to ProcedureTypeDefinition.NumberOfParameters do
            begin
              ParameterTypeDefRec.Seg := PUnitIdentifierData (Ptr (Seg (ProcedureParameterData^),
                                           ProcedureParameterData^.UnitTypeOffsets.UnitIdentifierData))^.UnitSegment;
              ParameterTypeDefRec.Ofs := ProcedureParameterData^.UnitTypeOffsets.TypeOffset;
              CheckAndCopyParameterValue (ParameterTypeDef, ProcedureParameterData^.VarFlags);
              Inc (ProcedureParameterData);
            end;
          If DS_Changed then GenerateInstruction_Word (MOV_DS_BX);
        end;
      If pfConstructor in Flags then
        ConstructObject (PObjectTypeDefinition (Ptr (Seg (ProcedureIdentifierData^),
                                                          OuterBlockProcedureIdentifier))^.OffsetOf_VMT_Offset);
    end;
  GenerateProcedureEntryCode := EndSubroutine
end;
```

### Code Block 2

```pascal
Function GenerateProcedureExitCode: Word;
Var ProcedureIdentifierData: PProcedureIdentifierData;
    ResultType: PTypeDefinition;
    FOpCode: WordRec;
    ReturnOpCode: Byte;

  Procedure LoadFunctionResultToRegisters (StackFrameOffset: ShortInt; ReturnSize: Word);
  begin
    Case ReturnSize of
      1: GenerateInstruction_Word ($468A);
      2: GenerateInstruction_Word ($468B);
      4: begin
           GenerateInstruction_Word ($468B);
           GenerateInstruction_Byte (StackFrameOffset);
           Inc (StackFrameOffset, 2);
           GenerateInstruction_Word ($568B);
         end;
      else begin
             GenerateInstruction_Word ($468B);
             GenerateInstruction_Byte (StackFrameOffset);
             Inc (StackFrameOffset, 2);
             GenerateInstruction_Word ($5E8B);
             GenerateInstruction_Byte (StackFrameOffset);
             Inc (StackFrameOffset, 2);
             GenerateInstruction_Word ($568B);
           end;
    end;
    GenerateInstruction_Byte (StackFrameOffset);
  end;

begin
  ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
  With ProcedureIdentifierData^ do
    begin
      If pfConstructor in Flags then
        begin
          GenerateInstruction_Word ($46C4);
          GenerateInstruction_Byte ($06);
          GenerateInstruction_Word ($C28C);
        end else If pfDestructor in Flags then
          begin
            GenerateInstruction_MOV_16BitReg_Immediate (rDI,
              PObjectTypeDefinition (Ptr (Seg (ProcedureIdentifierData^),
                                          OuterBlockProcedureIdentifier))^.OffsetOf_VMT_Offset);
            GenerateInstruction_CALL_FAR (SysProc_Destruct);
          end;
      If (ProcedureTypeDefinition.ResultTypeOffset.UnitIdentifierData <> 0) and not (pfAssembler in Flags) then
        begin
          ResultType := PointerFromOffsets (ProcedureTypeDefinition.ResultTypeOffset);
          Case ResultType^.BaseType of
            btPointer,
            btReal,
            btInteger..btEnumeration: LoadFunctionResultToRegisters (FunctionResultNegativeSize, ResultType^.Size);
            btExtended: begin
                          FOpCode.Word := $06D9 or Byte (ResultType^.DataType);
                          If fpExtended in ResultType^.DataType then FOpCode.ByteH := $2E;
                          GenerateFPUInstructionForStackFrameDisplacement (FOpCode.Word, FunctionResultNegativeSize);
                          GenerateInstruction_TwoBytes ($CD, $3D);
                        end;
          end;
        end;
      If pfStackFrame in Flags then DestroyStackFrame;
      If pfFar in Flags then ReturnOpCode := $CB else ReturnOpCode := $C3;
      Case PushedParametersSize of
        0: GenerateInstruction_Byte (ReturnOpCode);
        else begin
               GenerateInstruction_Byte (ReturnOpCode - 1);
               GenerateInstruction_Word (PushedParametersSize);
             end;
      end;
    end;
  GenerateProcedureExitCode := EndSubroutine;
end;
```

### Code Block 3

```pascal
Function SizeOfPassedParameter (Var TypeDef: PTypeDefinition; Var ParameterFlags: TVarFlagsSet;
                                Var ValueParameterCopySize: Word; AssemblerProcedure: Boolean): Word;
Var Size: Word;
    ExitFlag: Boolean;

  Procedure PassPointer;
  begin
    ExitFlag := True;
    If (vfConst in ParameterFlags) or AssemblerProcedure then
      begin
        Include (ParameterFlags, vfVar);
        If TypeDef^.BaseType <> btSet then SizeOfPassedParameter := 4 else
          begin
            TypeDef := PointerFromOffsets (POrdinalTypeDefinition (
                         PointerFromOffsets (PSetTypeDefinition (TypeDef)^.BaseSetTypeOffset))^.OrdinalType);
            SizeOfPassedParameter := 4;
          end;
      end else begin
                 ValueParameterCopySize := Size;
                 SizeOfPassedParameter := 4;
               end;
  end;

begin
  ExitFlag := False;
  ValueParameterCopySize := 0;
  If vfOpenParameter in ParameterFlags then SizeOfPassedParameter := 6 else
    If vfVar in ParameterFlags then SizeOfPassedParameter := 4 else
      begin
        Size := TypeDef^.Size;
        Case TypeDef^.BaseType of
          btUntyped..btText: Case Size of
                               1, 2, 4:
                               else PassPointer;
                             end;
          btSet: If it32Bytes in TypeDef^.DataType then PassPointer;
          btString: PassPointer;
        end;
        If not ExitFlag then SizeOfPassedParameter := (Size + 1) and $FFFE;
      end;
end;
```

### Code Block 4

```pascal
Function GenerateProcedureEntryCode: Word;
Var NegativeSizeOfPassedParameters, StackFrameOffset: Word;
    DS_Changed: Boolean;
    N: Word;
    ProcedureIdentifierData: PProcedureIdentifierData;
    ProcedureParameterData: PProcedureParameterData;
    ParameterTypeDef: PTypeDefinition;
    ParameterTypeDefRec: PtrRec absolute ParameterTypeDef;

  Procedure CheckAndCopyParameterValue (TypeDef: PTypeDefinition; ParameterFlags: TVarFlagsSet);
  Var ValueParameterCopySize, OffsetTo_VMT_Offset: Word;
      ObjectTypeDefinition: PObjectTypeDefinition absolute TypeDef;

    Procedure CopyParameterValue;
    Var DataOffset: Byte;

      Procedure Multiply_AX (Factor: Word);
      Var Expression: TExpression;
      begin
        With Expression do
          begin
            Location := elRegister;
            DataType := itWord;
            UsedRegisters := [urAX];
            LocationData.Register := rAX;
            IntegerMultiplicationWithConstant (rAX, Factor);
          end;
      end;

    begin
      If not DS_Changed then
        begin
          DS_Changed := True;
          GenerateInstruction_Word ($D38C);
          GenerateInstruction_Word ($C38E);
          GenerateInstruction_Word ($DB8C);
          GenerateInstruction_Byte ($FC);
        end;
      If not (vfOpenParameter in ParameterFlags) then
        begin
          GenerateInstructionForStackFrameDisplacement ($8D, $3E, StackFrameOffset);
          GenerateInstructionForStackFrameDisplacement ($C5, $36, NegativeSizeOfPassedParameters);
          Case TypeDef^.BaseType of
            btString: begin
                        GenerateInstruction_Byte ($AC);
                        If TypeDef^.Size <> 0 then
                          begin
                            GenerateInstruction_TwoBytes ($3C, TypeDef^.Size - 1);
                            GenerateInstruction_Word ($0272);
                            GenerateInstruction_TwoBytes ($B0, TypeDef^.Size - 1);
                          end;
                        GenerateInstruction_Word ($91AA);
                        GenerateInstruction_Word ($ED30);
                      end;
            btSet: begin
                     PSetTypeDefinition (TypeDef)^.GetSetSizeAndLowestElementDataOffset (DataOffset);
                     If DataOffset <> 0 then
                       begin
                         GenerateInstruction_Word ($C683);
                         GenerateInstruction_Byte (DataOffset);
                       end;
                     GenerateInstruction_Byte ($B9);
                     GenerateInstruction_Word (TypeDef^.Size);
                   end;
            else begin
                   GenerateInstruction_Byte ($B9);
                   GenerateInstruction_Word (TypeDef^.Size);
                 end;
          end;
          GenerateInstruction_Word ($A4F3);
          Exit;
        end;
      GenerateInstructionForStackFrameDisplacement ($8B, $06, NegativeSizeOfPassedParameters);
      GenerateInstruction_Byte ($40);
      Multiply_AX (TypeDef^.Size);
      GenerateInstruction_Word ($C189);
      If StackChecking in StatementCompilerSwitches then
        begin
          GenerateInstruction_Word ($DB8E);
          GenerateInstruction_CALL_FAR (SysProc_StackCheck);
        end;
      GenerateInstruction_Word ($CC29);
      If TypeDef^.Size = 1 then
        begin
          GenerateInstruction_Word ($E483);
          GenerateInstruction_Byte ($FE);
        end;
      GenerateInstruction_Word ($E789);
      GenerateInstructionForStackFrameDisplacement ($C5, $36, NegativeSizeOfPassedParameters + 2);
      GenerateInstructionForStackFrameDisplacement ($89, $3E, NegativeSizeOfPassedParameters + 2);
      GenerateInstructionForStackFrameDisplacement ($8C, $06, NegativeSizeOfPassedParameters + 4);
      Inc (NumberOfLocalParameters);
      GenerateInstruction_Word ($A4F3);
    end;

  begin
    Dec (NegativeSizeOfPassedParameters, SizeOfPassedParameter (TypeDef, ParameterFlags, ValueParameterCopySize, False));
    Dec (StackFrameOffset, ValueParameterCopySize);
    If not (vfOpenParameter in ParameterFlags) or (ParameterFlags * [vfConst, vfVar] <> []) then
      begin
        If ValueParameterCopySize > 0 then
          begin
            If (ValueParameterCopySize <> 1) and (WordAlignment in ModuleCompilerSwitches) then
              StackFrameOffset := StackFrameOffset and $FFFE;
            CopyParameterValue;
          end;
      end else CopyParameterValue;
    If (ParameterFlags * [vfOpenParameter, vfVar] = []) and (TypeDef^.BaseType = btObject) and
       (ObjectTypeDefinition^.VMT_Size <> 0) then
      begin
        If ValueParameterCopySize <> 0 then
          OffsetTo_VMT_Offset := StackFrameOffset else
            OffsetTo_VMT_Offset := NegativeSizeOfPassedParameters;
        Inc (OffsetTo_VMT_Offset, PObjectTypeDefinition (TypeDef)^.OffsetOf_VMT_Offset);
        GenerateInstructionForStackFrameDisplacement (MOV_RegisterOrMemory_Immediate_16Bit,
                                               rmSS_BP_Displacement,
                                               OffsetTo_VMT_Offset);
      GenerateReference (Seg (TypeDef^), PObjectTypeDefinition (TypeDef)^.VMT_TypedConstantsBlockRecordOffset, 0,
                             [rfDataSegment, rfConstant, rfOffset]);
      end;
  end;

  Procedure ConstructObject (OffsetOf_VMT_Offset: Word);
  begin
    GenerateInstruction_MOV_16BitReg_Immediate (rDI, OffsetOf_VMT_Offset);
    GenerateInstruction_CALL_FAR (SysProc_Construct);
    GenerateCodeForNearJump (LastJumpToProgramBlockExit, $74);
  end;

begin
  ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
  With ProcedureIdentifierData^ do
    begin
      If pfStackFrame in Flags then CreateStackFrame;
      If (ProcedureTypeDefinition.NumberOfParameters <> 0) and not (pfAssembler in Flags) then
        begin
          NegativeSizeOfPassedParameters := OffsetAfterLastParameter;
          StackFrameOffset := FunctionResultNegativeSize;
          DS_Changed := False;
          ProcedureParameterData := PProcedureParameterData (PChar (ProcedureIdentifierData) + 10 + 14);
          For N := 1 to ProcedureTypeDefinition.NumberOfParameters do
            begin
              ParameterTypeDefRec.Seg := PUnitIdentifierData (Ptr (Seg (ProcedureParameterData^),
                                           ProcedureParameterData^.UnitTypeOffsets.UnitIdentifierData))^.UnitSegment;
              ParameterTypeDefRec.Ofs := ProcedureParameterData^.UnitTypeOffsets.TypeOffset;
              CheckAndCopyParameterValue (ParameterTypeDef, ProcedureParameterData^.VarFlags);
              Inc (ProcedureParameterData);
            end;
          If DS_Changed then GenerateInstruction_Word (MOV_DS_BX);
        end;
      If pfConstructor in Flags then
        ConstructObject (PObjectTypeDefinition (Ptr (Seg (ProcedureIdentifierData^),
                                                          OuterBlockProcedureIdentifier))^.OffsetOf_VMT_Offset);
    end;
  GenerateProcedureEntryCode := EndSubroutine
end;
```

### Code Block 5

```pascal
Function GenerateProcedureExitCode: Word;
Var ProcedureIdentifierData: PProcedureIdentifierData;
    ResultType: PTypeDefinition;
    FOpCode: WordRec;
    ReturnOpCode: Byte;

  Procedure LoadFunctionResultToRegisters (StackFrameOffset: ShortInt; ReturnSize: Word);
  begin
    Case ReturnSize of
      1: GenerateInstruction_Word ($468A);
      2: GenerateInstruction_Word ($468B);
      4: begin
           GenerateInstruction_Word ($468B);
           GenerateInstruction_Byte (StackFrameOffset);
           Inc (StackFrameOffset, 2);
           GenerateInstruction_Word ($568B);
         end;
      else begin
             GenerateInstruction_Word ($468B);
             GenerateInstruction_Byte (StackFrameOffset);
             Inc (StackFrameOffset, 2);
             GenerateInstruction_Word ($5E8B);
             GenerateInstruction_Byte (StackFrameOffset);
             Inc (StackFrameOffset, 2);
             GenerateInstruction_Word ($568B);
           end;
    end;
    GenerateInstruction_Byte (StackFrameOffset);
  end;

begin
  ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
  With ProcedureIdentifierData^ do
    begin
      If pfConstructor in Flags then
        begin
          GenerateInstruction_Word ($46C4);
          GenerateInstruction_Byte ($06);
          GenerateInstruction_Word ($C28C);
        end else If pfDestructor in Flags then
          begin
            GenerateInstruction_MOV_16BitReg_Immediate (rDI,
              PObjectTypeDefinition (Ptr (Seg (ProcedureIdentifierData^),
                                          OuterBlockProcedureIdentifier))^.OffsetOf_VMT_Offset);
            GenerateInstruction_CALL_FAR (SysProc_Destruct);
          end;
      If (ProcedureTypeDefinition.ResultTypeOffset.UnitIdentifierData <> 0) and not (pfAssembler in Flags) then
        begin
          ResultType := PointerFromOffsets (ProcedureTypeDefinition.ResultTypeOffset);
          Case ResultType^.BaseType of
            btPointer,
            btReal,
            btInteger..btEnumeration: LoadFunctionResultToRegisters (FunctionResultNegativeSize, ResultType^.Size);
            btExtended: begin
                          FOpCode.Word := $06D9 or Byte (ResultType^.DataType);
                          If fpExtended in ResultType^.DataType then FOpCode.ByteH := $2E;
                          GenerateFPUInstructionForStackFrameDisplacement (FOpCode.Word, FunctionResultNegativeSize);
                          GenerateInstruction_TwoBytes ($CD, $3D);
                        end;
          end;
        end;
      If pfStackFrame in Flags then DestroyStackFrame;
      If pfFar in Flags then ReturnOpCode := $CB else ReturnOpCode := $C3;
      Case PushedParametersSize of
        0: GenerateInstruction_Byte (ReturnOpCode);
        else begin
               GenerateInstruction_Byte (ReturnOpCode - 1);
               GenerateInstruction_Word (PushedParametersSize);
             end;
      end;
    end;
  GenerateProcedureExitCode := EndSubroutine;
end;
```

### Code Block 6

```pascal
Function SizeOfPassedParameter (Var TypeDef: PTypeDefinition; Var ParameterFlags: TVarFlagsSet;
                                Var ValueParameterCopySize: Word; AssemblerProcedure: Boolean): Word;
Var Size: Word;
    ExitFlag: Boolean;

  Procedure PassPointer;
  begin
    ExitFlag := True;
    If (vfConst in ParameterFlags) or AssemblerProcedure then
      begin
        Include (ParameterFlags, vfVar);
        If TypeDef^.BaseType <> btSet then SizeOfPassedParameter := 4 else
          begin
            TypeDef := PointerFromOffsets (POrdinalTypeDefinition (
                         PointerFromOffsets (PSetTypeDefinition (TypeDef)^.BaseSetTypeOffset))^.OrdinalType);
            SizeOfPassedParameter := 4;
          end;
      end else begin
                 ValueParameterCopySize := Size;
                 SizeOfPassedParameter := 4;
               end;
  end;

begin
  ExitFlag := False;
  ValueParameterCopySize := 0;
  If vfOpenParameter in ParameterFlags then SizeOfPassedParameter := 6 else
    If vfVar in ParameterFlags then SizeOfPassedParameter := 4 else
      begin
        Size := TypeDef^.Size;
        Case TypeDef^.BaseType of
          btUntyped..btText: Case Size of
                               1, 2, 4:
                               else PassPointer;
                             end;
          btSet: If it32Bytes in TypeDef^.DataType then PassPointer;
          btString: PassPointer;
        end;
        If not ExitFlag then SizeOfPassedParameter := (Size + 1) and $FFFE;
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
- https://turbopascal.org/procedure-entry-and-exit-code
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

