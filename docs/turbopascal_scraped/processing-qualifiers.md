# Processing Qualifiers - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-qualifiers

---

## Table of Contents

- Processing Qualifiers

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Qualifiers
This procedure processes qualifiers: caret (
^
) to dereference pointers, period as separator for fields and brackets (
[]
) for arrays.
Function TExpression.ProcessQualifiers: Boolean;

  Procedure Qualifier_LeftBracket;
  Var ArrayTypeDefinition: PArrayTypeDefinition absolute TypeDefPtr;
      ArrayIndexTypeDefinition: POrdinalTypeDefinition;
      Index: TExpression;
      IndexLowerLimit, ElementSize: Word;

    Procedure CheckOpenParameterIndexRange;
    Var TempExpression: TExpression;
        LastJumpToRangeError: Word;

      Procedure PositionTempExpressionToOpenParameterHighestIndex;
      begin
        TempExpression := Self;
        TempExpression.PositionToOpenParameterHighestIndex;
      end;

    begin
      With Index do
        begin
          Case Location of
            elConstant: begin
                          If Value.LongRec.WordH <> 0 then Error (ConstantOutOfRange);
                          If RangeChecking in StatementCompilerSwitches then
                            begin
                              PositionTempExpressionToOpenParameterHighestIndex;
                              GenerateArithmeticInstructionWithImmediateValue (Value.Word, $38);
                              GenerateInstruction_Word ($0573);
                              GenerateInstruction_CALL_FAR (SysProc_RangeError);
                              Self.Calculate;
                              Self.EndIntermediateCodeSubroutine;
                            end;
                        end;
            else begin
                   LastJumpToRangeError := 0;
                   If not (it16Bit in DataType) then ExtendInteger ([it16Bit, itUnsigned]);
                   If RangeChecking in StatementCompilerSwitches then
                     begin
                       Calculate;
                       LoadExpressionToRegisters (urAX);
                       If it32Bit in DataType then
                         begin
                           GenerateInstruction_Word ($D209);
                           GenerateCodeForNearJump (LastJumpToRangeError, JNE);
                         end;
                       PositionTempExpressionToOpenParameterHighestIndex;
                       GenerateArithmeticInstructionWith_ACC ($38);
                       GenerateInstruction_Word ($0576);
                       GenerateLabelAndSetJumpsToIt (LastJumpToRangeError);
                       GenerateInstruction_CALL_FAR (SysProc_RangeError);
                       EndIntermediateCodeSubroutine;
                     end;
                 end;
          end;
          DataType := itWord;
        end;
    end;

  begin
    Repeat
      ArrayIndexTypeDefinition := Pointer (TypeDefPtr);
      Case TypeDefPtr^.BaseType of
        btArray,
        btString: begin
                    If Location <> elMemory then Error (InvalidQualifier);
                    ArrayIndexTypeDefinition := PointerFromOffsets (ArrayTypeDefinition^.IndexTypeOffset);
                  end;
        else If (ExtendedSyntax in ModuleCompilerSwitches) and (TypeDefPtr = Ptr (SystemUnitSegment, PChar_TypeOffset)) then
               begin
                 DereferencePointer;
                 ArrayIndexTypeDefinition := Ptr (SystemUnitSegment, Word_TypeOffset);
               end else Error (InvalidQualifier);
      end;
      GetNextToken;
      IndexLowerLimit := ArrayIndexTypeDefinition^.LowerLimit;
      With Index do
        begin
          CalculateExpression;
          ConvertToBooleanByte;
          LoadPointerToMemoryTo_DX_AX;
          CheckTypeCompatibility (ArrayIndexTypeDefinition);
          Case TypeDefPtr^.Size of
            0: CheckOpenParameterIndexRange;
            else begin
                   LowestDataType := [it16Bit];
                   CheckRange (Pointer (ArrayIndexTypeDefinition));
                   LowestDataType := [];
                 end;
          end;
        end;
      TypeDefPtr := PointerFromOffsets (ArrayTypeDefinition^.ElementTypeOffset);
      DataType := TypeDefPtr^.DataType;
      ElementSize := TypeDefPtr^.Size;
      If Index.Location = elConstant then Inc (Value.Offset, (Index.Value.Word - IndexLowerLimit) * ElementSize) else
        begin
          Dec (Value.Word, IndexLowerLimit * ElementSize);
          Case ofsDI in LocationData.Flags of
            True: begin
                    With Index do
                      begin
                        Calculate;
                        IntegerMultiplicationWithConstant (rAX, ElementSize);
                        Save (Self.UsedRegisters);
                        Self.Calculate;
                        PopToRegisters ([urAX]);
                        GenerateInstruction_TwoBytes ($03, $F8 or LocationData.Register);
                      end;
                  end;
            else begin
                   With Index do
                     begin
                       Calculate;
                       IntegerMultiplicationWithConstant (rDI, ElementSize);
                       LoadExpressionToRegisters (urDI);
                     end;
                   Calculate;
                 end;
          end;
          ES_DI_PointerDestroyed;
          EndIntermediateCodeSubroutine;
          UsedRegisters := UsedRegisters + Index.UsedRegisters * [urBX, urDX, urCX, urAX];
          Include (LocationData.Flags, ofsDI);
        end;
    until Token <> Token_Comma;
    ExpectTokenAndGetNext (Token_RightBracket);
  end;

  Procedure Qualifier_Period;
  Var FieldIdentifier: Word;
      FieldIdentifierData: PVariableIdentifierData;
      ProcedureIdentifierData: PProcedureIdentifierData absolute FieldIdentifierData;
      FieldToken: TToken;
      RecordTypeDefinition: PRecordTypeDefinition absolute TypeDefPtr;
  begin
    Case TypeDefPtr^.BaseType of
      btRecord,
      btObject: begin
                  GetNextToken;
                  If Token <> Token_Identifier then Error (FieldIdentifierExpected);
                  If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (RecordTypeDefinition, FieldIdentifierData,
                                                    FieldToken, FieldIdentifier) then Error (FieldIdentifierExpected);
                  Case FieldToken of
                    Token_VariableIdentifier: begin
                                                Inc (Value.Offset, FieldIdentifierData^.W1.Ofs);
                                                TypeDefPtr := PointerFromOffsets (FieldIdentifierData^.UnitTypeOffsets);
                                                DataType := TypeDefPtr^.DataType;
                                              end;
                    else begin
                           TypeDefPtr := @ProcedureIdentifierData^.ProcedureTypeDefinition;
                           Location := elProcedure;
                           DataType := [];
                           Value.W16 := Ofs (CurrentRecordTypeDef^);
                           Value.W18 := Seg (CurrentRecordTypeDef^);
                         end;
                  end;
                  GetNextToken;
                end;
      else Error (InvalidQualifier);
    end;
  end;

  Procedure Qualifier_Caret;
  begin
    GetNextToken;
    If TypeDefPtr^.BaseType <> btPointer then Error (InvalidQualifier);
    TypeDefPtr := PointerFromOffsets (PPointerTypeDefinition (TypeDefPtr)^.PointerBaseTypeOffset);
    DataType := TypeDefPtr^.DataType;
    DereferencePointer;
  end;

begin
  ProcessQualifiers := False;
  Case Token of
    Token_LeftBracket: Qualifier_LeftBracket;
    Token_Period:      Qualifier_Period;
    Token_Caret:       Qualifier_Caret;
    else Exit;
  end;
  ProcessQualifiers := True;
end;
This procedure dereferences pointer – it creates a variable to which the pointer is pointing to. This is the reverse operation of
creating a pointer to variable reference
.
Procedure TExpression.DereferencePointer;
begin
  Case Location of
    elConstant: begin
                  Location := elMemory;
                  LocationData.Register := rDX_AX;
                end;
    elPointerToMemory:
    else begin
           Calculate;
           Case Location of
             elMemory: GenerateInstructionWithExpressionInMemOrReg (LES, regDI);
             else begin
                    GenerateInstruction_TwoBytes (MOV_16Bit, DI_AX);
                    GenerateInstruction_TwoBytes (MOV_SegmentReg_RegMem, Operation_Register_DX);
                  end;
           end;
           ES_DI_PointerDestroyed;
           EndIntermediateCodeSubroutine;
           LocationData.Flags := [ofsDI, segES];
           Value.Word := 0;
         end;
  end;
  Location := elMemory;
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
Function TExpression.ProcessQualifiers: Boolean;

  Procedure Qualifier_LeftBracket;
  Var ArrayTypeDefinition: PArrayTypeDefinition absolute TypeDefPtr;
      ArrayIndexTypeDefinition: POrdinalTypeDefinition;
      Index: TExpression;
      IndexLowerLimit, ElementSize: Word;

    Procedure CheckOpenParameterIndexRange;
    Var TempExpression: TExpression;
        LastJumpToRangeError: Word;

      Procedure PositionTempExpressionToOpenParameterHighestIndex;
      begin
        TempExpression := Self;
        TempExpression.PositionToOpenParameterHighestIndex;
      end;

    begin
      With Index do
        begin
          Case Location of
            elConstant: begin
                          If Value.LongRec.WordH <> 0 then Error (ConstantOutOfRange);
                          If RangeChecking in StatementCompilerSwitches then
                            begin
                              PositionTempExpressionToOpenParameterHighestIndex;
                              GenerateArithmeticInstructionWithImmediateValue (Value.Word, $38);
                              GenerateInstruction_Word ($0573);
                              GenerateInstruction_CALL_FAR (SysProc_RangeError);
                              Self.Calculate;
                              Self.EndIntermediateCodeSubroutine;
                            end;
                        end;
            else begin
                   LastJumpToRangeError := 0;
                   If not (it16Bit in DataType) then ExtendInteger ([it16Bit, itUnsigned]);
                   If RangeChecking in StatementCompilerSwitches then
                     begin
                       Calculate;
                       LoadExpressionToRegisters (urAX);
                       If it32Bit in DataType then
                         begin
                           GenerateInstruction_Word ($D209);
                           GenerateCodeForNearJump (LastJumpToRangeError, JNE);
                         end;
                       PositionTempExpressionToOpenParameterHighestIndex;
                       GenerateArithmeticInstructionWith_ACC ($38);
                       GenerateInstruction_Word ($0576);
                       GenerateLabelAndSetJumpsToIt (LastJumpToRangeError);
                       GenerateInstruction_CALL_FAR (SysProc_RangeError);
                       EndIntermediateCodeSubroutine;
                     end;
                 end;
          end;
          DataType := itWord;
        end;
    end;

  begin
    Repeat
      ArrayIndexTypeDefinition := Pointer (TypeDefPtr);
      Case TypeDefPtr^.BaseType of
        btArray,
        btString: begin
                    If Location <> elMemory then Error (InvalidQualifier);
                    ArrayIndexTypeDefinition := PointerFromOffsets (ArrayTypeDefinition^.IndexTypeOffset);
                  end;
        else If (ExtendedSyntax in ModuleCompilerSwitches) and (TypeDefPtr = Ptr (SystemUnitSegment, PChar_TypeOffset)) then
               begin
                 DereferencePointer;
                 ArrayIndexTypeDefinition := Ptr (SystemUnitSegment, Word_TypeOffset);
               end else Error (InvalidQualifier);
      end;
      GetNextToken;
      IndexLowerLimit := ArrayIndexTypeDefinition^.LowerLimit;
      With Index do
        begin
          CalculateExpression;
          ConvertToBooleanByte;
          LoadPointerToMemoryTo_DX_AX;
          CheckTypeCompatibility (ArrayIndexTypeDefinition);
          Case TypeDefPtr^.Size of
            0: CheckOpenParameterIndexRange;
            else begin
                   LowestDataType := [it16Bit];
                   CheckRange (Pointer (ArrayIndexTypeDefinition));
                   LowestDataType := [];
                 end;
          end;
        end;
      TypeDefPtr := PointerFromOffsets (ArrayTypeDefinition^.ElementTypeOffset);
      DataType := TypeDefPtr^.DataType;
      ElementSize := TypeDefPtr^.Size;
      If Index.Location = elConstant then Inc (Value.Offset, (Index.Value.Word - IndexLowerLimit) * ElementSize) else
        begin
          Dec (Value.Word, IndexLowerLimit * ElementSize);
          Case ofsDI in LocationData.Flags of
            True: begin
                    With Index do
                      begin
                        Calculate;
                        IntegerMultiplicationWithConstant (rAX, ElementSize);
                        Save (Self.UsedRegisters);
                        Self.Calculate;
                        PopToRegisters ([urAX]);
                        GenerateInstruction_TwoBytes ($03, $F8 or LocationData.Register);
                      end;
                  end;
            else begin
                   With Index do
                     begin
                       Calculate;
                       IntegerMultiplicationWithConstant (rDI, ElementSize);
                       LoadExpressionToRegisters (urDI);
                     end;
                   Calculate;
                 end;
          end;
          ES_DI_PointerDestroyed;
          EndIntermediateCodeSubroutine;
          UsedRegisters := UsedRegisters + Index.UsedRegisters * [urBX, urDX, urCX, urAX];
          Include (LocationData.Flags, ofsDI);
        end;
    until Token <> Token_Comma;
    ExpectTokenAndGetNext (Token_RightBracket);
  end;

  Procedure Qualifier_Period;
  Var FieldIdentifier: Word;
      FieldIdentifierData: PVariableIdentifierData;
      ProcedureIdentifierData: PProcedureIdentifierData absolute FieldIdentifierData;
      FieldToken: TToken;
      RecordTypeDefinition: PRecordTypeDefinition absolute TypeDefPtr;
  begin
    Case TypeDefPtr^.BaseType of
      btRecord,
      btObject: begin
                  GetNextToken;
                  If Token <> Token_Identifier then Error (FieldIdentifierExpected);
                  If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (RecordTypeDefinition, FieldIdentifierData,
                                                    FieldToken, FieldIdentifier) then Error (FieldIdentifierExpected);
                  Case FieldToken of
                    Token_VariableIdentifier: begin
                                                Inc (Value.Offset, FieldIdentifierData^.W1.Ofs);
                                                TypeDefPtr := PointerFromOffsets (FieldIdentifierData^.UnitTypeOffsets);
                                                DataType := TypeDefPtr^.DataType;
                                              end;
                    else begin
                           TypeDefPtr := @ProcedureIdentifierData^.ProcedureTypeDefinition;
                           Location := elProcedure;
                           DataType := [];
                           Value.W16 := Ofs (CurrentRecordTypeDef^);
                           Value.W18 := Seg (CurrentRecordTypeDef^);
                         end;
                  end;
                  GetNextToken;
                end;
      else Error (InvalidQualifier);
    end;
  end;

  Procedure Qualifier_Caret;
  begin
    GetNextToken;
    If TypeDefPtr^.BaseType <> btPointer then Error (InvalidQualifier);
    TypeDefPtr := PointerFromOffsets (PPointerTypeDefinition (TypeDefPtr)^.PointerBaseTypeOffset);
    DataType := TypeDefPtr^.DataType;
    DereferencePointer;
  end;

begin
  ProcessQualifiers := False;
  Case Token of
    Token_LeftBracket: Qualifier_LeftBracket;
    Token_Period:      Qualifier_Period;
    Token_Caret:       Qualifier_Caret;
    else Exit;
  end;
  ProcessQualifiers := True;
end;
```

### Code Block 2

```pascal
Procedure TExpression.DereferencePointer;
begin
  Case Location of
    elConstant: begin
                  Location := elMemory;
                  LocationData.Register := rDX_AX;
                end;
    elPointerToMemory:
    else begin
           Calculate;
           Case Location of
             elMemory: GenerateInstructionWithExpressionInMemOrReg (LES, regDI);
             else begin
                    GenerateInstruction_TwoBytes (MOV_16Bit, DI_AX);
                    GenerateInstruction_TwoBytes (MOV_SegmentReg_RegMem, Operation_Register_DX);
                  end;
           end;
           ES_DI_PointerDestroyed;
           EndIntermediateCodeSubroutine;
           LocationData.Flags := [ofsDI, segES];
           Value.Word := 0;
         end;
  end;
  Location := elMemory;
end;
```

### Code Block 3

```pascal
Function TExpression.ProcessQualifiers: Boolean;

  Procedure Qualifier_LeftBracket;
  Var ArrayTypeDefinition: PArrayTypeDefinition absolute TypeDefPtr;
      ArrayIndexTypeDefinition: POrdinalTypeDefinition;
      Index: TExpression;
      IndexLowerLimit, ElementSize: Word;

    Procedure CheckOpenParameterIndexRange;
    Var TempExpression: TExpression;
        LastJumpToRangeError: Word;

      Procedure PositionTempExpressionToOpenParameterHighestIndex;
      begin
        TempExpression := Self;
        TempExpression.PositionToOpenParameterHighestIndex;
      end;

    begin
      With Index do
        begin
          Case Location of
            elConstant: begin
                          If Value.LongRec.WordH <> 0 then Error (ConstantOutOfRange);
                          If RangeChecking in StatementCompilerSwitches then
                            begin
                              PositionTempExpressionToOpenParameterHighestIndex;
                              GenerateArithmeticInstructionWithImmediateValue (Value.Word, $38);
                              GenerateInstruction_Word ($0573);
                              GenerateInstruction_CALL_FAR (SysProc_RangeError);
                              Self.Calculate;
                              Self.EndIntermediateCodeSubroutine;
                            end;
                        end;
            else begin
                   LastJumpToRangeError := 0;
                   If not (it16Bit in DataType) then ExtendInteger ([it16Bit, itUnsigned]);
                   If RangeChecking in StatementCompilerSwitches then
                     begin
                       Calculate;
                       LoadExpressionToRegisters (urAX);
                       If it32Bit in DataType then
                         begin
                           GenerateInstruction_Word ($D209);
                           GenerateCodeForNearJump (LastJumpToRangeError, JNE);
                         end;
                       PositionTempExpressionToOpenParameterHighestIndex;
                       GenerateArithmeticInstructionWith_ACC ($38);
                       GenerateInstruction_Word ($0576);
                       GenerateLabelAndSetJumpsToIt (LastJumpToRangeError);
                       GenerateInstruction_CALL_FAR (SysProc_RangeError);
                       EndIntermediateCodeSubroutine;
                     end;
                 end;
          end;
          DataType := itWord;
        end;
    end;

  begin
    Repeat
      ArrayIndexTypeDefinition := Pointer (TypeDefPtr);
      Case TypeDefPtr^.BaseType of
        btArray,
        btString: begin
                    If Location <> elMemory then Error (InvalidQualifier);
                    ArrayIndexTypeDefinition := PointerFromOffsets (ArrayTypeDefinition^.IndexTypeOffset);
                  end;
        else If (ExtendedSyntax in ModuleCompilerSwitches) and (TypeDefPtr = Ptr (SystemUnitSegment, PChar_TypeOffset)) then
               begin
                 DereferencePointer;
                 ArrayIndexTypeDefinition := Ptr (SystemUnitSegment, Word_TypeOffset);
               end else Error (InvalidQualifier);
      end;
      GetNextToken;
      IndexLowerLimit := ArrayIndexTypeDefinition^.LowerLimit;
      With Index do
        begin
          CalculateExpression;
          ConvertToBooleanByte;
          LoadPointerToMemoryTo_DX_AX;
          CheckTypeCompatibility (ArrayIndexTypeDefinition);
          Case TypeDefPtr^.Size of
            0: CheckOpenParameterIndexRange;
            else begin
                   LowestDataType := [it16Bit];
                   CheckRange (Pointer (ArrayIndexTypeDefinition));
                   LowestDataType := [];
                 end;
          end;
        end;
      TypeDefPtr := PointerFromOffsets (ArrayTypeDefinition^.ElementTypeOffset);
      DataType := TypeDefPtr^.DataType;
      ElementSize := TypeDefPtr^.Size;
      If Index.Location = elConstant then Inc (Value.Offset, (Index.Value.Word - IndexLowerLimit) * ElementSize) else
        begin
          Dec (Value.Word, IndexLowerLimit * ElementSize);
          Case ofsDI in LocationData.Flags of
            True: begin
                    With Index do
                      begin
                        Calculate;
                        IntegerMultiplicationWithConstant (rAX, ElementSize);
                        Save (Self.UsedRegisters);
                        Self.Calculate;
                        PopToRegisters ([urAX]);
                        GenerateInstruction_TwoBytes ($03, $F8 or LocationData.Register);
                      end;
                  end;
            else begin
                   With Index do
                     begin
                       Calculate;
                       IntegerMultiplicationWithConstant (rDI, ElementSize);
                       LoadExpressionToRegisters (urDI);
                     end;
                   Calculate;
                 end;
          end;
          ES_DI_PointerDestroyed;
          EndIntermediateCodeSubroutine;
          UsedRegisters := UsedRegisters + Index.UsedRegisters * [urBX, urDX, urCX, urAX];
          Include (LocationData.Flags, ofsDI);
        end;
    until Token <> Token_Comma;
    ExpectTokenAndGetNext (Token_RightBracket);
  end;

  Procedure Qualifier_Period;
  Var FieldIdentifier: Word;
      FieldIdentifierData: PVariableIdentifierData;
      ProcedureIdentifierData: PProcedureIdentifierData absolute FieldIdentifierData;
      FieldToken: TToken;
      RecordTypeDefinition: PRecordTypeDefinition absolute TypeDefPtr;
  begin
    Case TypeDefPtr^.BaseType of
      btRecord,
      btObject: begin
                  GetNextToken;
                  If Token <> Token_Identifier then Error (FieldIdentifierExpected);
                  If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (RecordTypeDefinition, FieldIdentifierData,
                                                    FieldToken, FieldIdentifier) then Error (FieldIdentifierExpected);
                  Case FieldToken of
                    Token_VariableIdentifier: begin
                                                Inc (Value.Offset, FieldIdentifierData^.W1.Ofs);
                                                TypeDefPtr := PointerFromOffsets (FieldIdentifierData^.UnitTypeOffsets);
                                                DataType := TypeDefPtr^.DataType;
                                              end;
                    else begin
                           TypeDefPtr := @ProcedureIdentifierData^.ProcedureTypeDefinition;
                           Location := elProcedure;
                           DataType := [];
                           Value.W16 := Ofs (CurrentRecordTypeDef^);
                           Value.W18 := Seg (CurrentRecordTypeDef^);
                         end;
                  end;
                  GetNextToken;
                end;
      else Error (InvalidQualifier);
    end;
  end;

  Procedure Qualifier_Caret;
  begin
    GetNextToken;
    If TypeDefPtr^.BaseType <> btPointer then Error (InvalidQualifier);
    TypeDefPtr := PointerFromOffsets (PPointerTypeDefinition (TypeDefPtr)^.PointerBaseTypeOffset);
    DataType := TypeDefPtr^.DataType;
    DereferencePointer;
  end;

begin
  ProcessQualifiers := False;
  Case Token of
    Token_LeftBracket: Qualifier_LeftBracket;
    Token_Period:      Qualifier_Period;
    Token_Caret:       Qualifier_Caret;
    else Exit;
  end;
  ProcessQualifiers := True;
end;
```

### Code Block 4

```pascal
Procedure TExpression.DereferencePointer;
begin
  Case Location of
    elConstant: begin
                  Location := elMemory;
                  LocationData.Register := rDX_AX;
                end;
    elPointerToMemory:
    else begin
           Calculate;
           Case Location of
             elMemory: GenerateInstructionWithExpressionInMemOrReg (LES, regDI);
             else begin
                    GenerateInstruction_TwoBytes (MOV_16Bit, DI_AX);
                    GenerateInstruction_TwoBytes (MOV_SegmentReg_RegMem, Operation_Register_DX);
                  end;
           end;
           ES_DI_PointerDestroyed;
           EndIntermediateCodeSubroutine;
           LocationData.Flags := [ofsDI, segES];
           Value.Word := 0;
         end;
  end;
  Location := elMemory;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-pointer-to-variable-reference
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

