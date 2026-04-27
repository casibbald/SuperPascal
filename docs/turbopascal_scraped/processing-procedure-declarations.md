# Processing Procedure Declarations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-procedure-declarations

---

## Table of Contents

- Processing Procedure Declarations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Procedure Declarations
Turbo Pascal uses few procedures to process declaration of procedures and functions. The same code is also used to process methods, constructors and destructors. The first procedure processes header.
Procedure ProcessProcedureDeclaration;
Var IdentifierDataPtr: Pointer;
    ProcedureIdentifierDataPtr: PProcedureIdentifierData absolute IdentifierDataPtr;
    InlineCodeSize: Word;
    TypePointer: PTypeDefinition;
    NewIdentifier: PIdentifier;
    IdentifierOffset: Word absolute NewIdentifier;
    IdentifierToken: TToken;
    Saved_TemporarySymbolTablePosition: Word;
    ProcTypeDef: PTypeDefinition;
    Flags: TProcedureFlagsSet;

Label SetProcedureFlags;

begin
  SavedToken := Token;
  ProcedureStartLineNumber := CurrentSourceFile^.LineCounter;
  GetNextToken;
  ExpectIdentifier;
  If CurrentIdentifierDeclaredInCurrentScope (IdentifierOffset, IdentifierDataPtr, IdentifierToken) then
    begin
      If SourceType = stUnitInterface then Error (DuplicateIdentifier);
      If IdentifierToken = Token_ProcedureIdentifier then
        begin
          If pfMethod in ProcedureIdentifierDataPtr^.Flags then Error (DuplicateIdentifier);
          ProcessAlreadyDeclaredProcedure;
          Exit;
        end;
      If IdentifierToken = Token_TypeIdentifier then
        begin
          TypePointer := PointerFromOffsets (PTypeIdentifierData (ProcedureIdentifierDataPtr)^.UnitTypeOffsets);
          If TypePointer^.BaseType = btObject then
            begin
              GetNextToken;
              ExpectTokenAndGetNext (TOKEN_Period);
              ExpectIdentifier;
              If not IsIdentifierInSymbolTable (Ptr (Seg (TypePointer^),
                                                PObjectTypeDefinition (TypePointer)^.FieldsListOffset),
                                                IdentifierToken,
                                                IdentifierDataPtr,
                                                IdentifierOffset) then
                Error (MethodIdentifierExpected);
              If IdentifierToken <> Token_ProcedureIdentifier then Error (MethodIdentifierExpected);
              ProcessAlreadyDeclaredProcedure;
              Exit;
            end;
        end;
      Case SavedToken of
        Token_CONSTRUCTOR,
        Token_DESTRUCTOR: Error (ObjectTypeExpected);
        else Error (DuplicateIdentifier);
      end;
    end;
  Case SavedToken of
    Token_CONSTRUCTOR,
    Token_DESTRUCTOR: Error (ObjectTypeExpected);
  end;
  ProcedureIdentifierDataPtr := StoreCurrentIdentifierToSymbolTable (CurrentScopeIdentifierTableAddress, 10, NewIdentifier);
  NewIdentifier^.Token := Token_ProcedureIdentifier;
  GetNextToken;
  Saved_TemporarySymbolTablePosition := SymbolTable [stTemporary].NextRecordOffset;
  ProcTypeDef := ProcessProcedureHeader (SavedToken);
  ExpectTokenAndGetNext (TOKEN_Semicolon);
  If CheckAndGetNextToken (TOKEN_INLINE) then
    begin
      Process_INLINE (InlineCodeSize);
      Include (ProcedureIdentifierDataPtr^.Flags, pfInline);
      ProcedureIdentifierDataPtr^.ProceduresRecordOffset := InlineCodeSize;
      ExpectTokenAndGetNext (TOKEN_Semicolon);
      Exit;
    end;
  ProcedureIdentifierDataPtr^.LocalIdentifiersList := Saved_TemporarySymbolTablePosition;
  CreateProcedureRecord (NewIdentifier, ProcedureIdentifierDataPtr);
  ProcedureIdentifierDataPtr^.OuterBlockProcedureIdentifier := CurrentProcedureIdentifier;
  If CurrentProcedureIdentifier = 0 then
    begin
      Flags := [pfInterrupt];
      If CompareIdentifierToDireciveAndSkipSemicolon (_INTERRUPT) then GoTo SetProcedureFlags;
    end;
  Flags := [pfFar];
  If CompareIdentifierToDireciveAndSkipSemicolon (_FAR) then GoTo SetProcedureFlags;
  If SourceType <> stUnitInterface then
    begin
      Flags := [];
      If CompareIdentifierToDireciveAndSkipSemicolon (_NEAR) then GoTo SetProcedureFlags;
      If ForceFarCalls in StatementCompilerSwitches then Flags := [pfFar];
    end;
SetProcedureFlags:
  ProcedureIdentifierDataPtr^.Flags := ProcedureIdentifierDataPtr^.Flags + Flags;
  If SourceType = stUnitInterface then Exit;
  If CompareIdentifierToDireciveAndSkipSemicolon (_FORWARD) then Exit;
  ProcessProcedureDeclaractionsAndProgramBlock;
end;
This procedure
processes declarations
and
program block
.
Procedure ProcessProcedureDeclaractionsAndProgramBlock;
  Var Saved_PushedParametersSize, Saved_OffsetAfterLastParameter, Saved_FunctionResultNegativeSize, Saved_MaxStackFrameOffset,
      SavedProcedureIdentifierDataOffset: Word;
      SavedCurrentProcedureIdentifier, SavedProceduresNextRecordOffset: Word;
      Saved_ProcedureStartLineNumber: Word;
  begin
    With ProcedureIdentifierDataPtr^ do
      begin
        If (CurrentProcedureIdentifier = 0) and CompareIdentifierToDirectiveAndGetNextToken (_EXTERNAL) then
          begin
            Include (Flags, pfExternal);
            LocalIdentifiersList := 0;
            ExpectTokenAndGetNext (TOKEN_Semicolon);
            Exit;
          end;
        If CompareIdentifierToDireciveAndSkipSemicolon (_ASSEMBLER) then Include (Flags, pfAssembler);
        Saved_PushedParametersSize       := PushedParametersSize;
        Saved_OffsetAfterLastParameter   := OffsetAfterLastParameter;
        Saved_FunctionResultNegativeSize := FunctionResultNegativeSize;
        Saved_MaxStackFrameOffset        := ProgramBlockMaxStackFrameOffset;
        SavedProcedureIdentifierDataOffset := ProcedureIdentifierDataOffset;
        SavedCurrentProcedureIdentifier := CurrentProcedureIdentifier;
        SavedProceduresNextRecordOffset := SymbolTable [stProcedures].UsedSize;
        CurrentProcedureIdentifier := IdentifierOffset;
        ProcedureIdentifierDataOffset := Ofs (ProcedureIdentifierDataPtr^);
        TemporaryStoredParameters := LocalIdentifiersList;
        LocalIdentifiersList := SymbolTable [stMain].UsedSize;
        With PProceduresBlockRecord (Ptr (SymbolTable [stProcedures].Segment, ProceduresRecordOffset))^ do
          ProgramCodeBlockRecordOffset := $FFFE;
        CreateSymbolTable (4);
        CreateParametersAsLocalVariables;
        Saved_ProcedureStartLineNumber := ProcedureStartLineNumber;
        ProcessDeclarations;
        ProcedureStartLineNumber := Saved_ProcedureStartLineNumber;
        PProceduresBlockRecord (Ptr (SymbolTable [stProcedures].Segment,
                                ProceduresRecordOffset))^.SizeOfConstants := ProcessProgramBlock;

      { SymbolTable [stProcedures].Segment might change }

        PProceduresBlockRecord (Ptr (SymbolTable [stProcedures].Segment,
                                ProceduresRecordOffset))^.ProgramCodeBlockRecordOffset := SymbolTable [stCodeBlocks].UsedSize;

        CreateProgramCodeBlockRecord;
        CreateTypedConstantsBlockRecord;
        CheckForUndefined_FORWARD_Or_EXTERNAL (Ptr (SymbolTable [stProcedures].Segment, SavedProceduresNextRecordOffset));
        PushedParametersSize            := Saved_PushedParametersSize;
        OffsetAfterLastParameter        := Saved_OffsetAfterLastParameter;
        FunctionResultNegativeSize      := Saved_FunctionResultNegativeSize;
        ProgramBlockMaxStackFrameOffset := Saved_MaxStackFrameOffset;
        ProcedureIdentifierDataOffset := SavedProcedureIdentifierDataOffset;
        CurrentProcedureIdentifier := SavedCurrentProcedureIdentifier;
        If not (LocalDebugSymbols in ModuleCompilerSwitches) then
          begin
            SymbolTable [stMain].UsedSize := LocalIdentifiersList;
            LocalIdentifiersList := 0;
          end;
        ExpectTokenAndGetNext (TOKEN_Semicolon);
      end;
  end;
This procedure creates parameters as local variables. For methods also the implicit parameter
Self
is added.
Procedure CreateParametersAsLocalVariables;
    Var ProcedureIdentifierData: PProcedureIdentifierData;
        ProcedureParameterData: PProcedureParameterData absolute ProcedureIdentifierData;
        ProcedureParameterDataOfs: Word absolute ProcedureParameterData;
        AssemblerProcedure: Boolean;
        CurrentParameterOffset: Integer;
        Saved_TemporaryStoredParameters, Parameter, NumberOfParameters: Word;
        SelfIdentifier: PIdentifier;
        SelfIdentifierData: PVariableIdentifierData;
    begin
      ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
      AssemblerProcedure := pfAssembler in ProcedureIdentifierData^.Flags;
      PushedParametersSize := SizeOfPushedParameters (ProcedureIdentifierData, OffsetAfterLastParameter);
      CurrentParameterOffset := OffsetAfterLastParameter;
      FunctionResultNegativeSize := FunctionResultStackFrameSize;
      ProgramBlockMaxStackFrameOffset := FunctionResultNegativeSize;
      Saved_TemporaryStoredParameters := TemporaryStoredParameters;
      NumberOfParameters := ProcedureIdentifierData^.ProcedureTypeDefinition.NumberOfParameters;
      Inc (ProcedureParameterDataOfs, 24);
      For Parameter := 1 to NumberOfParameters do
        begin
          CreateParameterAsLocalVariable;
          Inc (ProcedureParameterData);
        end;
      ProcedureParameterDataOfs := ProcedureIdentifierDataOffset;
      If pfMethod in ProcedureIdentifierData^.Flags then
        begin
          VariableData_Flags := [vfVar, vf1];
          RecordTypeDefinitionOffset.TypeOffset := $0006;
          RecordTypeDefinitionOffset.UnitIdentifierData := CurrentProcedureIdentifier;
          VariableData_NextMemberOffset := 0;
          GetTypeAndUnitIdentifierOffsets (Ptr (Seg (ProcedureIdentifierData^),
                                          ProcedureIdentifierData^.OuterBlockProcedureIdentifier), CurrentVarUnitTypeOffsets);

          CopyStringToCurrentIdentifier ('Self');
          SelfIdentifierData := StoreNewIdentifierToSymbolTable (11, SelfIdentifier);
          SelfIdentifier^.Token := Token_VariableIdentifier;
          Move (VariableData_Flags, SelfIdentifierData^, 11);
        end;
      If TemporaryStoredParameters = SymbolTable [stTemporary].UsedSize then
        SymbolTable [stTemporary].UsedSize := Saved_TemporaryStoredParameters;
    end;

      Function FunctionResultStackFrameSize: Integer;
      Var ResultSize: Integer;
      begin
        ResultSize := 0;
        With ProcedureIdentifierData^ do
          If (ProcedureTypeDefinition.ResultTypeOffset.UnitIdentifierData <> 0) and not (pfAssembler in Flags) then
            With PTypeDefinition (PointerFromOffsets (ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset))^ do
              If BaseType <> btString then Dec (ResultSize, Size);
        FunctionResultStackFrameSize := ResultSize;
      end;

      Procedure CreateParameterAsLocalVariable;
      Var ParameterTypeDef: PTypeDefinition;
          ArrayTypeDefinition: PArrayTypeDefinition;
          ProcedureParameterVarFlags: TVarFlagsSet;
          ValueParameterCopySize, StackFrameSizeOfPassedParameter: Word;
          ParameterIdentifier: PIdentifier;
          ParameterIdentifierData: PVariableIdentifierData;
          Offset: Integer;
      begin
        ParameterTypeDef := PointerFromOffsets (ProcedureParameterData^.UnitTypeOffsets);
        ProcedureParameterVarFlags := ProcedureParameterData^.VarFlags;
        If vfArray in ProcedureParameterVarFlags then
          begin
            ArrayTypeDefinition := CreateTypeDefinition (16, 0, [], btArray);
            With ArrayTypeDefinition^ do
              begin
                GetTypeAndUnitIdentifierOffsets (ParameterTypeDef, ElementTypeOffset);
                GetTypeAndUnitIdentifierOffsets (Ptr (SystemUnitSegment, Word_TypeOffset), IndexTypeOffset);
              end;
            ParameterTypeDef := ArrayTypeDefinition;
          end;
        StackFrameSizeOfPassedParameter :=
          SizeOfPassedParameter (ParameterTypeDef, ProcedureParameterVarFlags, ValueParameterCopySize, AssemblerProcedure);
        Include (ProcedureParameterVarFlags, vfArray);
        VariableData_Flags := ProcedureParameterVarFlags;
        GetTypeAndUnitIdentifierOffsets (ParameterTypeDef, CurrentVarUnitTypeOffsets);
        Dec (CurrentParameterOffset, StackFrameSizeOfPassedParameter);
        If ValueParameterCopySize <> 0 then
          begin
            Offset := ProgramBlockMaxStackFrameOffset - ValueParameterCopySize;
            If (WordAlignment in ModuleCompilerSwitches) and (ValueParameterCopySize <> 1) then Offset := Offset and $FFFE;
            ProgramBlockMaxStackFrameOffset := Offset;
          end else begin
                     Offset := CurrentParameterOffset;
                     If vfOpenParameter in VariableData_Flags then Inc (Offset, 2);
                   end;
        RecordTypeDefinitionOffset.TypeOffset := Offset;
        RecordTypeDefinitionOffset.UnitIdentifierData := CurrentProcedureIdentifier;
        VariableData_NextMemberOffset := 0;
        CopyStringFromTemporaryBlockToCurrentIdentifier (TemporaryStoredParameters);
        Inc (TemporaryStoredParameters, Length (CurrentIdentifier) + 3);
        ParameterIdentifierData := StoreNewIdentifierToSymbolTable (11, ParameterIdentifier);
        ParameterIdentifier^.Token := Token_VariableIdentifier;
        Move (VariableData_Flags, ParameterIdentifierData^, 11);
      end;
Procedures can be declared in the
Interface
part of the unit, in the
Object
declaration or with the
Forward
directive. Such cases are handled with this procedure. Of course, header must match previous declaration.
Procedure ProcessAlreadyDeclaredProcedure;
  Var SavedTempSymbolTableCurrentPointerOfs: Word;
      ProcTypeDefAndParametersSize: Word;
      N, NewTempSymbolTableCurrentPointerOfs: Word;
      ExpectedToken: TToken;
      DIPtr, SIPtr: PChar;
      ProceduresRecord: PProceduresBlockRecord;
  begin
    ProceduresRecord := Ptr (SymbolTable [stProcedures].Segment, ProcedureIdentifierDataPtr^.ProceduresRecordOffset);
    If ProceduresRecord^.ProgramCodeBlockRecordOffset <> $FFFF then Error (DuplicateIdentifier);
    GetNextToken;
    If Word (Ptr (Seg (ProcedureIdentifierDataPtr^), Ofs (ProcedureIdentifierDataPtr^) + 18)^) = 0 then
      begin
        If pfConstructor in ProcedureIdentifierDataPtr^.Flags then ExpectedToken := Token_CONSTRUCTOR else
          If pfDestructor in ProcedureIdentifierDataPtr^.Flags then ExpectedToken := Token_DESTRUCTOR else
            ExpectedToken := Token_PROCEDURE;
      end else ExpectedToken := Token_FUNCTION;
    If SavedToken <> ExpectedToken then Error (HeaderDoesNotMatchPreviousDefinition);
    If (Token = Token_LeftParenthesis) or (Token = Token_Colon) then
      begin
        SavedTempSymbolTableCurrentPointerOfs := SymbolTable [stTemporary].NextRecordOffset;
        ProcTypeDef := ProcessProcedureHeader (SavedToken);
        ProcTypeDefAndParametersSize := SymbolTable [stMain].UsedSize - Ofs (ProcTypeDef^);
        SymbolTable [stMain].UsedSize := Ofs (ProcTypeDef^);
        ProcTypeDef^.Size := ProcedureIdentifierDataPtr^.ProcedureTypeDefinition.Size;
        ProcTypeDef^.W06_ := ProcedureIdentifierDataPtr^.ProcedureTypeDefinition.W06_;
        For N := 0 to ProcTypeDefAndParametersSize - 1 do
          If PChar (ProcTypeDef)^ <> PChar (@ProcedureIdentifierDataPtr^.ProcedureTypeDefinition)^ then
            Error (HeaderDoesNotMatchPreviousDefinition);
        NewTempSymbolTableCurrentPointerOfs := SymbolTable [stTemporary].UsedSize;
        SymbolTable [stTemporary].UsedSize := SavedTempSymbolTableCurrentPointerOfs;
        DIPtr := Ptr (SymbolTable [stTemporary].Segment, ProcedureIdentifierDataPtr^.LocalIdentifiersList);
        SIPtr := Ptr (SymbolTable [stTemporary].Segment, SavedTempSymbolTableCurrentPointerOfs);
        While Ofs (SIPtr^) <> NewTempSymbolTableCurrentPointerOfs do
          begin
            Inc (SIPtr, 2);
            Inc (DIPtr, 2);
            If not IdentifiersEqual (PString (DIPtr), PString (SIPtr)) then Error (HeaderDoesNotMatchPreviousDefinition);
            Inc (DIPtr, Length (PString (DIPtr)^) + 1);
          end;
      end;
    ExpectTokenAndGetNext (TOKEN_Semicolon);
    ProcessProcedureDeclaractionsAndProgramBlock;
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
Procedure ProcessProcedureDeclaration;
Var IdentifierDataPtr: Pointer;
    ProcedureIdentifierDataPtr: PProcedureIdentifierData absolute IdentifierDataPtr;
    InlineCodeSize: Word;
    TypePointer: PTypeDefinition;
    NewIdentifier: PIdentifier;
    IdentifierOffset: Word absolute NewIdentifier;
    IdentifierToken: TToken;
    Saved_TemporarySymbolTablePosition: Word;
    ProcTypeDef: PTypeDefinition;
    Flags: TProcedureFlagsSet;

Label SetProcedureFlags;

begin
  SavedToken := Token;
  ProcedureStartLineNumber := CurrentSourceFile^.LineCounter;
  GetNextToken;
  ExpectIdentifier;
  If CurrentIdentifierDeclaredInCurrentScope (IdentifierOffset, IdentifierDataPtr, IdentifierToken) then
    begin
      If SourceType = stUnitInterface then Error (DuplicateIdentifier);
      If IdentifierToken = Token_ProcedureIdentifier then
        begin
          If pfMethod in ProcedureIdentifierDataPtr^.Flags then Error (DuplicateIdentifier);
          ProcessAlreadyDeclaredProcedure;
          Exit;
        end;
      If IdentifierToken = Token_TypeIdentifier then
        begin
          TypePointer := PointerFromOffsets (PTypeIdentifierData (ProcedureIdentifierDataPtr)^.UnitTypeOffsets);
          If TypePointer^.BaseType = btObject then
            begin
              GetNextToken;
              ExpectTokenAndGetNext (TOKEN_Period);
              ExpectIdentifier;
              If not IsIdentifierInSymbolTable (Ptr (Seg (TypePointer^),
                                                PObjectTypeDefinition (TypePointer)^.FieldsListOffset),
                                                IdentifierToken,
                                                IdentifierDataPtr,
                                                IdentifierOffset) then
                Error (MethodIdentifierExpected);
              If IdentifierToken <> Token_ProcedureIdentifier then Error (MethodIdentifierExpected);
              ProcessAlreadyDeclaredProcedure;
              Exit;
            end;
        end;
      Case SavedToken of
        Token_CONSTRUCTOR,
        Token_DESTRUCTOR: Error (ObjectTypeExpected);
        else Error (DuplicateIdentifier);
      end;
    end;
  Case SavedToken of
    Token_CONSTRUCTOR,
    Token_DESTRUCTOR: Error (ObjectTypeExpected);
  end;
  ProcedureIdentifierDataPtr := StoreCurrentIdentifierToSymbolTable (CurrentScopeIdentifierTableAddress, 10, NewIdentifier);
  NewIdentifier^.Token := Token_ProcedureIdentifier;
  GetNextToken;
  Saved_TemporarySymbolTablePosition := SymbolTable [stTemporary].NextRecordOffset;
  ProcTypeDef := ProcessProcedureHeader (SavedToken);
  ExpectTokenAndGetNext (TOKEN_Semicolon);
  If CheckAndGetNextToken (TOKEN_INLINE) then
    begin
      Process_INLINE (InlineCodeSize);
      Include (ProcedureIdentifierDataPtr^.Flags, pfInline);
      ProcedureIdentifierDataPtr^.ProceduresRecordOffset := InlineCodeSize;
      ExpectTokenAndGetNext (TOKEN_Semicolon);
      Exit;
    end;
  ProcedureIdentifierDataPtr^.LocalIdentifiersList := Saved_TemporarySymbolTablePosition;
  CreateProcedureRecord (NewIdentifier, ProcedureIdentifierDataPtr);
  ProcedureIdentifierDataPtr^.OuterBlockProcedureIdentifier := CurrentProcedureIdentifier;
  If CurrentProcedureIdentifier = 0 then
    begin
      Flags := [pfInterrupt];
      If CompareIdentifierToDireciveAndSkipSemicolon (_INTERRUPT) then GoTo SetProcedureFlags;
    end;
  Flags := [pfFar];
  If CompareIdentifierToDireciveAndSkipSemicolon (_FAR) then GoTo SetProcedureFlags;
  If SourceType <> stUnitInterface then
    begin
      Flags := [];
      If CompareIdentifierToDireciveAndSkipSemicolon (_NEAR) then GoTo SetProcedureFlags;
      If ForceFarCalls in StatementCompilerSwitches then Flags := [pfFar];
    end;
SetProcedureFlags:
  ProcedureIdentifierDataPtr^.Flags := ProcedureIdentifierDataPtr^.Flags + Flags;
  If SourceType = stUnitInterface then Exit;
  If CompareIdentifierToDireciveAndSkipSemicolon (_FORWARD) then Exit;
  ProcessProcedureDeclaractionsAndProgramBlock;
end;
```

### Code Block 2

```pascal
  Procedure ProcessProcedureDeclaractionsAndProgramBlock;
  Var Saved_PushedParametersSize, Saved_OffsetAfterLastParameter, Saved_FunctionResultNegativeSize, Saved_MaxStackFrameOffset,
      SavedProcedureIdentifierDataOffset: Word;
      SavedCurrentProcedureIdentifier, SavedProceduresNextRecordOffset: Word;
      Saved_ProcedureStartLineNumber: Word;
  begin
    With ProcedureIdentifierDataPtr^ do
      begin
        If (CurrentProcedureIdentifier = 0) and CompareIdentifierToDirectiveAndGetNextToken (_EXTERNAL) then
          begin
            Include (Flags, pfExternal);
            LocalIdentifiersList := 0;
            ExpectTokenAndGetNext (TOKEN_Semicolon);
            Exit;
          end;
        If CompareIdentifierToDireciveAndSkipSemicolon (_ASSEMBLER) then Include (Flags, pfAssembler);
        Saved_PushedParametersSize       := PushedParametersSize;
        Saved_OffsetAfterLastParameter   := OffsetAfterLastParameter;
        Saved_FunctionResultNegativeSize := FunctionResultNegativeSize;
        Saved_MaxStackFrameOffset        := ProgramBlockMaxStackFrameOffset;
        SavedProcedureIdentifierDataOffset := ProcedureIdentifierDataOffset;
        SavedCurrentProcedureIdentifier := CurrentProcedureIdentifier;
        SavedProceduresNextRecordOffset := SymbolTable [stProcedures].UsedSize;
        CurrentProcedureIdentifier := IdentifierOffset;
        ProcedureIdentifierDataOffset := Ofs (ProcedureIdentifierDataPtr^);
        TemporaryStoredParameters := LocalIdentifiersList;
        LocalIdentifiersList := SymbolTable [stMain].UsedSize;
        With PProceduresBlockRecord (Ptr (SymbolTable [stProcedures].Segment, ProceduresRecordOffset))^ do
          ProgramCodeBlockRecordOffset := $FFFE;
        CreateSymbolTable (4);
        CreateParametersAsLocalVariables;
        Saved_ProcedureStartLineNumber := ProcedureStartLineNumber;
        ProcessDeclarations;
        ProcedureStartLineNumber := Saved_ProcedureStartLineNumber;
        PProceduresBlockRecord (Ptr (SymbolTable [stProcedures].Segment,
                                ProceduresRecordOffset))^.SizeOfConstants := ProcessProgramBlock;

      { SymbolTable [stProcedures].Segment might change }

        PProceduresBlockRecord (Ptr (SymbolTable [stProcedures].Segment,
                                ProceduresRecordOffset))^.ProgramCodeBlockRecordOffset := SymbolTable [stCodeBlocks].UsedSize;

        CreateProgramCodeBlockRecord;
        CreateTypedConstantsBlockRecord;
        CheckForUndefined_FORWARD_Or_EXTERNAL (Ptr (SymbolTable [stProcedures].Segment, SavedProceduresNextRecordOffset));
        PushedParametersSize            := Saved_PushedParametersSize;
        OffsetAfterLastParameter        := Saved_OffsetAfterLastParameter;
        FunctionResultNegativeSize      := Saved_FunctionResultNegativeSize;
        ProgramBlockMaxStackFrameOffset := Saved_MaxStackFrameOffset;
        ProcedureIdentifierDataOffset := SavedProcedureIdentifierDataOffset;
        CurrentProcedureIdentifier := SavedCurrentProcedureIdentifier;
        If not (LocalDebugSymbols in ModuleCompilerSwitches) then
          begin
            SymbolTable [stMain].UsedSize := LocalIdentifiersList;
            LocalIdentifiersList := 0;
          end;
        ExpectTokenAndGetNext (TOKEN_Semicolon);
      end;
  end;
```

### Code Block 3

```pascal
    Procedure CreateParametersAsLocalVariables;
    Var ProcedureIdentifierData: PProcedureIdentifierData;
        ProcedureParameterData: PProcedureParameterData absolute ProcedureIdentifierData;
        ProcedureParameterDataOfs: Word absolute ProcedureParameterData;
        AssemblerProcedure: Boolean;
        CurrentParameterOffset: Integer;
        Saved_TemporaryStoredParameters, Parameter, NumberOfParameters: Word;
        SelfIdentifier: PIdentifier;
        SelfIdentifierData: PVariableIdentifierData;
    begin
      ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
      AssemblerProcedure := pfAssembler in ProcedureIdentifierData^.Flags;
      PushedParametersSize := SizeOfPushedParameters (ProcedureIdentifierData, OffsetAfterLastParameter);
      CurrentParameterOffset := OffsetAfterLastParameter;
      FunctionResultNegativeSize := FunctionResultStackFrameSize;
      ProgramBlockMaxStackFrameOffset := FunctionResultNegativeSize;
      Saved_TemporaryStoredParameters := TemporaryStoredParameters;
      NumberOfParameters := ProcedureIdentifierData^.ProcedureTypeDefinition.NumberOfParameters;
      Inc (ProcedureParameterDataOfs, 24);
      For Parameter := 1 to NumberOfParameters do
        begin
          CreateParameterAsLocalVariable;
          Inc (ProcedureParameterData);
        end;
      ProcedureParameterDataOfs := ProcedureIdentifierDataOffset;
      If pfMethod in ProcedureIdentifierData^.Flags then
        begin
          VariableData_Flags := [vfVar, vf1];
          RecordTypeDefinitionOffset.TypeOffset := $0006;
          RecordTypeDefinitionOffset.UnitIdentifierData := CurrentProcedureIdentifier;
          VariableData_NextMemberOffset := 0;
          GetTypeAndUnitIdentifierOffsets (Ptr (Seg (ProcedureIdentifierData^),
                                          ProcedureIdentifierData^.OuterBlockProcedureIdentifier), CurrentVarUnitTypeOffsets);

          CopyStringToCurrentIdentifier ('Self');
          SelfIdentifierData := StoreNewIdentifierToSymbolTable (11, SelfIdentifier);
          SelfIdentifier^.Token := Token_VariableIdentifier;
          Move (VariableData_Flags, SelfIdentifierData^, 11);
        end;
      If TemporaryStoredParameters = SymbolTable [stTemporary].UsedSize then
        SymbolTable [stTemporary].UsedSize := Saved_TemporaryStoredParameters;
    end;

      Function FunctionResultStackFrameSize: Integer;
      Var ResultSize: Integer;
      begin
        ResultSize := 0;
        With ProcedureIdentifierData^ do
          If (ProcedureTypeDefinition.ResultTypeOffset.UnitIdentifierData <> 0) and not (pfAssembler in Flags) then
            With PTypeDefinition (PointerFromOffsets (ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset))^ do
              If BaseType <> btString then Dec (ResultSize, Size);
        FunctionResultStackFrameSize := ResultSize;
      end;

      Procedure CreateParameterAsLocalVariable;
      Var ParameterTypeDef: PTypeDefinition;
          ArrayTypeDefinition: PArrayTypeDefinition;
          ProcedureParameterVarFlags: TVarFlagsSet;
          ValueParameterCopySize, StackFrameSizeOfPassedParameter: Word;
          ParameterIdentifier: PIdentifier;
          ParameterIdentifierData: PVariableIdentifierData;
          Offset: Integer;
      begin
        ParameterTypeDef := PointerFromOffsets (ProcedureParameterData^.UnitTypeOffsets);
        ProcedureParameterVarFlags := ProcedureParameterData^.VarFlags;
        If vfArray in ProcedureParameterVarFlags then
          begin
            ArrayTypeDefinition := CreateTypeDefinition (16, 0, [], btArray);
            With ArrayTypeDefinition^ do
              begin
                GetTypeAndUnitIdentifierOffsets (ParameterTypeDef, ElementTypeOffset);
                GetTypeAndUnitIdentifierOffsets (Ptr (SystemUnitSegment, Word_TypeOffset), IndexTypeOffset);
              end;
            ParameterTypeDef := ArrayTypeDefinition;
          end;
        StackFrameSizeOfPassedParameter :=
          SizeOfPassedParameter (ParameterTypeDef, ProcedureParameterVarFlags, ValueParameterCopySize, AssemblerProcedure);
        Include (ProcedureParameterVarFlags, vfArray);
        VariableData_Flags := ProcedureParameterVarFlags;
        GetTypeAndUnitIdentifierOffsets (ParameterTypeDef, CurrentVarUnitTypeOffsets);
        Dec (CurrentParameterOffset, StackFrameSizeOfPassedParameter);
        If ValueParameterCopySize <> 0 then
          begin
            Offset := ProgramBlockMaxStackFrameOffset - ValueParameterCopySize;
            If (WordAlignment in ModuleCompilerSwitches) and (ValueParameterCopySize <> 1) then Offset := Offset and $FFFE;
            ProgramBlockMaxStackFrameOffset := Offset;
          end else begin
                     Offset := CurrentParameterOffset;
                     If vfOpenParameter in VariableData_Flags then Inc (Offset, 2);
                   end;
        RecordTypeDefinitionOffset.TypeOffset := Offset;
        RecordTypeDefinitionOffset.UnitIdentifierData := CurrentProcedureIdentifier;
        VariableData_NextMemberOffset := 0;
        CopyStringFromTemporaryBlockToCurrentIdentifier (TemporaryStoredParameters);
        Inc (TemporaryStoredParameters, Length (CurrentIdentifier) + 3);
        ParameterIdentifierData := StoreNewIdentifierToSymbolTable (11, ParameterIdentifier);
        ParameterIdentifier^.Token := Token_VariableIdentifier;
        Move (VariableData_Flags, ParameterIdentifierData^, 11);
      end;
```

### Code Block 4

```pascal
  Procedure ProcessAlreadyDeclaredProcedure;
  Var SavedTempSymbolTableCurrentPointerOfs: Word;
      ProcTypeDefAndParametersSize: Word;
      N, NewTempSymbolTableCurrentPointerOfs: Word;
      ExpectedToken: TToken;
      DIPtr, SIPtr: PChar;
      ProceduresRecord: PProceduresBlockRecord;
  begin
    ProceduresRecord := Ptr (SymbolTable [stProcedures].Segment, ProcedureIdentifierDataPtr^.ProceduresRecordOffset);
    If ProceduresRecord^.ProgramCodeBlockRecordOffset <> $FFFF then Error (DuplicateIdentifier);
    GetNextToken;
    If Word (Ptr (Seg (ProcedureIdentifierDataPtr^), Ofs (ProcedureIdentifierDataPtr^) + 18)^) = 0 then
      begin
        If pfConstructor in ProcedureIdentifierDataPtr^.Flags then ExpectedToken := Token_CONSTRUCTOR else
          If pfDestructor in ProcedureIdentifierDataPtr^.Flags then ExpectedToken := Token_DESTRUCTOR else
            ExpectedToken := Token_PROCEDURE;
      end else ExpectedToken := Token_FUNCTION;
    If SavedToken <> ExpectedToken then Error (HeaderDoesNotMatchPreviousDefinition);
    If (Token = Token_LeftParenthesis) or (Token = Token_Colon) then
      begin
        SavedTempSymbolTableCurrentPointerOfs := SymbolTable [stTemporary].NextRecordOffset;
        ProcTypeDef := ProcessProcedureHeader (SavedToken);
        ProcTypeDefAndParametersSize := SymbolTable [stMain].UsedSize - Ofs (ProcTypeDef^);
        SymbolTable [stMain].UsedSize := Ofs (ProcTypeDef^);
        ProcTypeDef^.Size := ProcedureIdentifierDataPtr^.ProcedureTypeDefinition.Size;
        ProcTypeDef^.W06_ := ProcedureIdentifierDataPtr^.ProcedureTypeDefinition.W06_;
        For N := 0 to ProcTypeDefAndParametersSize - 1 do
          If PChar (ProcTypeDef)^ <> PChar (@ProcedureIdentifierDataPtr^.ProcedureTypeDefinition)^ then
            Error (HeaderDoesNotMatchPreviousDefinition);
        NewTempSymbolTableCurrentPointerOfs := SymbolTable [stTemporary].UsedSize;
        SymbolTable [stTemporary].UsedSize := SavedTempSymbolTableCurrentPointerOfs;
        DIPtr := Ptr (SymbolTable [stTemporary].Segment, ProcedureIdentifierDataPtr^.LocalIdentifiersList);
        SIPtr := Ptr (SymbolTable [stTemporary].Segment, SavedTempSymbolTableCurrentPointerOfs);
        While Ofs (SIPtr^) <> NewTempSymbolTableCurrentPointerOfs do
          begin
            Inc (SIPtr, 2);
            Inc (DIPtr, 2);
            If not IdentifiersEqual (PString (DIPtr), PString (SIPtr)) then Error (HeaderDoesNotMatchPreviousDefinition);
            Inc (DIPtr, Length (PString (DIPtr)^) + 1);
          end;
      end;
    ExpectTokenAndGetNext (TOKEN_Semicolon);
    ProcessProcedureDeclaractionsAndProgramBlock;
  end;
```

### Code Block 5

```pascal
Procedure ProcessProcedureDeclaration;
Var IdentifierDataPtr: Pointer;
    ProcedureIdentifierDataPtr: PProcedureIdentifierData absolute IdentifierDataPtr;
    InlineCodeSize: Word;
    TypePointer: PTypeDefinition;
    NewIdentifier: PIdentifier;
    IdentifierOffset: Word absolute NewIdentifier;
    IdentifierToken: TToken;
    Saved_TemporarySymbolTablePosition: Word;
    ProcTypeDef: PTypeDefinition;
    Flags: TProcedureFlagsSet;

Label SetProcedureFlags;

begin
  SavedToken := Token;
  ProcedureStartLineNumber := CurrentSourceFile^.LineCounter;
  GetNextToken;
  ExpectIdentifier;
  If CurrentIdentifierDeclaredInCurrentScope (IdentifierOffset, IdentifierDataPtr, IdentifierToken) then
    begin
      If SourceType = stUnitInterface then Error (DuplicateIdentifier);
      If IdentifierToken = Token_ProcedureIdentifier then
        begin
          If pfMethod in ProcedureIdentifierDataPtr^.Flags then Error (DuplicateIdentifier);
          ProcessAlreadyDeclaredProcedure;
          Exit;
        end;
      If IdentifierToken = Token_TypeIdentifier then
        begin
          TypePointer := PointerFromOffsets (PTypeIdentifierData (ProcedureIdentifierDataPtr)^.UnitTypeOffsets);
          If TypePointer^.BaseType = btObject then
            begin
              GetNextToken;
              ExpectTokenAndGetNext (TOKEN_Period);
              ExpectIdentifier;
              If not IsIdentifierInSymbolTable (Ptr (Seg (TypePointer^),
                                                PObjectTypeDefinition (TypePointer)^.FieldsListOffset),
                                                IdentifierToken,
                                                IdentifierDataPtr,
                                                IdentifierOffset) then
                Error (MethodIdentifierExpected);
              If IdentifierToken <> Token_ProcedureIdentifier then Error (MethodIdentifierExpected);
              ProcessAlreadyDeclaredProcedure;
              Exit;
            end;
        end;
      Case SavedToken of
        Token_CONSTRUCTOR,
        Token_DESTRUCTOR: Error (ObjectTypeExpected);
        else Error (DuplicateIdentifier);
      end;
    end;
  Case SavedToken of
    Token_CONSTRUCTOR,
    Token_DESTRUCTOR: Error (ObjectTypeExpected);
  end;
  ProcedureIdentifierDataPtr := StoreCurrentIdentifierToSymbolTable (CurrentScopeIdentifierTableAddress, 10, NewIdentifier);
  NewIdentifier^.Token := Token_ProcedureIdentifier;
  GetNextToken;
  Saved_TemporarySymbolTablePosition := SymbolTable [stTemporary].NextRecordOffset;
  ProcTypeDef := ProcessProcedureHeader (SavedToken);
  ExpectTokenAndGetNext (TOKEN_Semicolon);
  If CheckAndGetNextToken (TOKEN_INLINE) then
    begin
      Process_INLINE (InlineCodeSize);
      Include (ProcedureIdentifierDataPtr^.Flags, pfInline);
      ProcedureIdentifierDataPtr^.ProceduresRecordOffset := InlineCodeSize;
      ExpectTokenAndGetNext (TOKEN_Semicolon);
      Exit;
    end;
  ProcedureIdentifierDataPtr^.LocalIdentifiersList := Saved_TemporarySymbolTablePosition;
  CreateProcedureRecord (NewIdentifier, ProcedureIdentifierDataPtr);
  ProcedureIdentifierDataPtr^.OuterBlockProcedureIdentifier := CurrentProcedureIdentifier;
  If CurrentProcedureIdentifier = 0 then
    begin
      Flags := [pfInterrupt];
      If CompareIdentifierToDireciveAndSkipSemicolon (_INTERRUPT) then GoTo SetProcedureFlags;
    end;
  Flags := [pfFar];
  If CompareIdentifierToDireciveAndSkipSemicolon (_FAR) then GoTo SetProcedureFlags;
  If SourceType <> stUnitInterface then
    begin
      Flags := [];
      If CompareIdentifierToDireciveAndSkipSemicolon (_NEAR) then GoTo SetProcedureFlags;
      If ForceFarCalls in StatementCompilerSwitches then Flags := [pfFar];
    end;
SetProcedureFlags:
  ProcedureIdentifierDataPtr^.Flags := ProcedureIdentifierDataPtr^.Flags + Flags;
  If SourceType = stUnitInterface then Exit;
  If CompareIdentifierToDireciveAndSkipSemicolon (_FORWARD) then Exit;
  ProcessProcedureDeclaractionsAndProgramBlock;
end;
```

### Code Block 6

```pascal
  Procedure ProcessProcedureDeclaractionsAndProgramBlock;
  Var Saved_PushedParametersSize, Saved_OffsetAfterLastParameter, Saved_FunctionResultNegativeSize, Saved_MaxStackFrameOffset,
      SavedProcedureIdentifierDataOffset: Word;
      SavedCurrentProcedureIdentifier, SavedProceduresNextRecordOffset: Word;
      Saved_ProcedureStartLineNumber: Word;
  begin
    With ProcedureIdentifierDataPtr^ do
      begin
        If (CurrentProcedureIdentifier = 0) and CompareIdentifierToDirectiveAndGetNextToken (_EXTERNAL) then
          begin
            Include (Flags, pfExternal);
            LocalIdentifiersList := 0;
            ExpectTokenAndGetNext (TOKEN_Semicolon);
            Exit;
          end;
        If CompareIdentifierToDireciveAndSkipSemicolon (_ASSEMBLER) then Include (Flags, pfAssembler);
        Saved_PushedParametersSize       := PushedParametersSize;
        Saved_OffsetAfterLastParameter   := OffsetAfterLastParameter;
        Saved_FunctionResultNegativeSize := FunctionResultNegativeSize;
        Saved_MaxStackFrameOffset        := ProgramBlockMaxStackFrameOffset;
        SavedProcedureIdentifierDataOffset := ProcedureIdentifierDataOffset;
        SavedCurrentProcedureIdentifier := CurrentProcedureIdentifier;
        SavedProceduresNextRecordOffset := SymbolTable [stProcedures].UsedSize;
        CurrentProcedureIdentifier := IdentifierOffset;
        ProcedureIdentifierDataOffset := Ofs (ProcedureIdentifierDataPtr^);
        TemporaryStoredParameters := LocalIdentifiersList;
        LocalIdentifiersList := SymbolTable [stMain].UsedSize;
        With PProceduresBlockRecord (Ptr (SymbolTable [stProcedures].Segment, ProceduresRecordOffset))^ do
          ProgramCodeBlockRecordOffset := $FFFE;
        CreateSymbolTable (4);
        CreateParametersAsLocalVariables;
        Saved_ProcedureStartLineNumber := ProcedureStartLineNumber;
        ProcessDeclarations;
        ProcedureStartLineNumber := Saved_ProcedureStartLineNumber;
        PProceduresBlockRecord (Ptr (SymbolTable [stProcedures].Segment,
                                ProceduresRecordOffset))^.SizeOfConstants := ProcessProgramBlock;

      { SymbolTable [stProcedures].Segment might change }

        PProceduresBlockRecord (Ptr (SymbolTable [stProcedures].Segment,
                                ProceduresRecordOffset))^.ProgramCodeBlockRecordOffset := SymbolTable [stCodeBlocks].UsedSize;

        CreateProgramCodeBlockRecord;
        CreateTypedConstantsBlockRecord;
        CheckForUndefined_FORWARD_Or_EXTERNAL (Ptr (SymbolTable [stProcedures].Segment, SavedProceduresNextRecordOffset));
        PushedParametersSize            := Saved_PushedParametersSize;
        OffsetAfterLastParameter        := Saved_OffsetAfterLastParameter;
        FunctionResultNegativeSize      := Saved_FunctionResultNegativeSize;
        ProgramBlockMaxStackFrameOffset := Saved_MaxStackFrameOffset;
        ProcedureIdentifierDataOffset := SavedProcedureIdentifierDataOffset;
        CurrentProcedureIdentifier := SavedCurrentProcedureIdentifier;
        If not (LocalDebugSymbols in ModuleCompilerSwitches) then
          begin
            SymbolTable [stMain].UsedSize := LocalIdentifiersList;
            LocalIdentifiersList := 0;
          end;
        ExpectTokenAndGetNext (TOKEN_Semicolon);
      end;
  end;
```

### Code Block 7

```pascal
    Procedure CreateParametersAsLocalVariables;
    Var ProcedureIdentifierData: PProcedureIdentifierData;
        ProcedureParameterData: PProcedureParameterData absolute ProcedureIdentifierData;
        ProcedureParameterDataOfs: Word absolute ProcedureParameterData;
        AssemblerProcedure: Boolean;
        CurrentParameterOffset: Integer;
        Saved_TemporaryStoredParameters, Parameter, NumberOfParameters: Word;
        SelfIdentifier: PIdentifier;
        SelfIdentifierData: PVariableIdentifierData;
    begin
      ProcedureIdentifierData := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierDataOffset);
      AssemblerProcedure := pfAssembler in ProcedureIdentifierData^.Flags;
      PushedParametersSize := SizeOfPushedParameters (ProcedureIdentifierData, OffsetAfterLastParameter);
      CurrentParameterOffset := OffsetAfterLastParameter;
      FunctionResultNegativeSize := FunctionResultStackFrameSize;
      ProgramBlockMaxStackFrameOffset := FunctionResultNegativeSize;
      Saved_TemporaryStoredParameters := TemporaryStoredParameters;
      NumberOfParameters := ProcedureIdentifierData^.ProcedureTypeDefinition.NumberOfParameters;
      Inc (ProcedureParameterDataOfs, 24);
      For Parameter := 1 to NumberOfParameters do
        begin
          CreateParameterAsLocalVariable;
          Inc (ProcedureParameterData);
        end;
      ProcedureParameterDataOfs := ProcedureIdentifierDataOffset;
      If pfMethod in ProcedureIdentifierData^.Flags then
        begin
          VariableData_Flags := [vfVar, vf1];
          RecordTypeDefinitionOffset.TypeOffset := $0006;
          RecordTypeDefinitionOffset.UnitIdentifierData := CurrentProcedureIdentifier;
          VariableData_NextMemberOffset := 0;
          GetTypeAndUnitIdentifierOffsets (Ptr (Seg (ProcedureIdentifierData^),
                                          ProcedureIdentifierData^.OuterBlockProcedureIdentifier), CurrentVarUnitTypeOffsets);

          CopyStringToCurrentIdentifier ('Self');
          SelfIdentifierData := StoreNewIdentifierToSymbolTable (11, SelfIdentifier);
          SelfIdentifier^.Token := Token_VariableIdentifier;
          Move (VariableData_Flags, SelfIdentifierData^, 11);
        end;
      If TemporaryStoredParameters = SymbolTable [stTemporary].UsedSize then
        SymbolTable [stTemporary].UsedSize := Saved_TemporaryStoredParameters;
    end;

      Function FunctionResultStackFrameSize: Integer;
      Var ResultSize: Integer;
      begin
        ResultSize := 0;
        With ProcedureIdentifierData^ do
          If (ProcedureTypeDefinition.ResultTypeOffset.UnitIdentifierData <> 0) and not (pfAssembler in Flags) then
            With PTypeDefinition (PointerFromOffsets (ProcedureIdentifierData^.ProcedureTypeDefinition.ResultTypeOffset))^ do
              If BaseType <> btString then Dec (ResultSize, Size);
        FunctionResultStackFrameSize := ResultSize;
      end;

      Procedure CreateParameterAsLocalVariable;
      Var ParameterTypeDef: PTypeDefinition;
          ArrayTypeDefinition: PArrayTypeDefinition;
          ProcedureParameterVarFlags: TVarFlagsSet;
          ValueParameterCopySize, StackFrameSizeOfPassedParameter: Word;
          ParameterIdentifier: PIdentifier;
          ParameterIdentifierData: PVariableIdentifierData;
          Offset: Integer;
      begin
        ParameterTypeDef := PointerFromOffsets (ProcedureParameterData^.UnitTypeOffsets);
        ProcedureParameterVarFlags := ProcedureParameterData^.VarFlags;
        If vfArray in ProcedureParameterVarFlags then
          begin
            ArrayTypeDefinition := CreateTypeDefinition (16, 0, [], btArray);
            With ArrayTypeDefinition^ do
              begin
                GetTypeAndUnitIdentifierOffsets (ParameterTypeDef, ElementTypeOffset);
                GetTypeAndUnitIdentifierOffsets (Ptr (SystemUnitSegment, Word_TypeOffset), IndexTypeOffset);
              end;
            ParameterTypeDef := ArrayTypeDefinition;
          end;
        StackFrameSizeOfPassedParameter :=
          SizeOfPassedParameter (ParameterTypeDef, ProcedureParameterVarFlags, ValueParameterCopySize, AssemblerProcedure);
        Include (ProcedureParameterVarFlags, vfArray);
        VariableData_Flags := ProcedureParameterVarFlags;
        GetTypeAndUnitIdentifierOffsets (ParameterTypeDef, CurrentVarUnitTypeOffsets);
        Dec (CurrentParameterOffset, StackFrameSizeOfPassedParameter);
        If ValueParameterCopySize <> 0 then
          begin
            Offset := ProgramBlockMaxStackFrameOffset - ValueParameterCopySize;
            If (WordAlignment in ModuleCompilerSwitches) and (ValueParameterCopySize <> 1) then Offset := Offset and $FFFE;
            ProgramBlockMaxStackFrameOffset := Offset;
          end else begin
                     Offset := CurrentParameterOffset;
                     If vfOpenParameter in VariableData_Flags then Inc (Offset, 2);
                   end;
        RecordTypeDefinitionOffset.TypeOffset := Offset;
        RecordTypeDefinitionOffset.UnitIdentifierData := CurrentProcedureIdentifier;
        VariableData_NextMemberOffset := 0;
        CopyStringFromTemporaryBlockToCurrentIdentifier (TemporaryStoredParameters);
        Inc (TemporaryStoredParameters, Length (CurrentIdentifier) + 3);
        ParameterIdentifierData := StoreNewIdentifierToSymbolTable (11, ParameterIdentifier);
        ParameterIdentifier^.Token := Token_VariableIdentifier;
        Move (VariableData_Flags, ParameterIdentifierData^, 11);
      end;
```

### Code Block 8

```pascal
  Procedure ProcessAlreadyDeclaredProcedure;
  Var SavedTempSymbolTableCurrentPointerOfs: Word;
      ProcTypeDefAndParametersSize: Word;
      N, NewTempSymbolTableCurrentPointerOfs: Word;
      ExpectedToken: TToken;
      DIPtr, SIPtr: PChar;
      ProceduresRecord: PProceduresBlockRecord;
  begin
    ProceduresRecord := Ptr (SymbolTable [stProcedures].Segment, ProcedureIdentifierDataPtr^.ProceduresRecordOffset);
    If ProceduresRecord^.ProgramCodeBlockRecordOffset <> $FFFF then Error (DuplicateIdentifier);
    GetNextToken;
    If Word (Ptr (Seg (ProcedureIdentifierDataPtr^), Ofs (ProcedureIdentifierDataPtr^) + 18)^) = 0 then
      begin
        If pfConstructor in ProcedureIdentifierDataPtr^.Flags then ExpectedToken := Token_CONSTRUCTOR else
          If pfDestructor in ProcedureIdentifierDataPtr^.Flags then ExpectedToken := Token_DESTRUCTOR else
            ExpectedToken := Token_PROCEDURE;
      end else ExpectedToken := Token_FUNCTION;
    If SavedToken <> ExpectedToken then Error (HeaderDoesNotMatchPreviousDefinition);
    If (Token = Token_LeftParenthesis) or (Token = Token_Colon) then
      begin
        SavedTempSymbolTableCurrentPointerOfs := SymbolTable [stTemporary].NextRecordOffset;
        ProcTypeDef := ProcessProcedureHeader (SavedToken);
        ProcTypeDefAndParametersSize := SymbolTable [stMain].UsedSize - Ofs (ProcTypeDef^);
        SymbolTable [stMain].UsedSize := Ofs (ProcTypeDef^);
        ProcTypeDef^.Size := ProcedureIdentifierDataPtr^.ProcedureTypeDefinition.Size;
        ProcTypeDef^.W06_ := ProcedureIdentifierDataPtr^.ProcedureTypeDefinition.W06_;
        For N := 0 to ProcTypeDefAndParametersSize - 1 do
          If PChar (ProcTypeDef)^ <> PChar (@ProcedureIdentifierDataPtr^.ProcedureTypeDefinition)^ then
            Error (HeaderDoesNotMatchPreviousDefinition);
        NewTempSymbolTableCurrentPointerOfs := SymbolTable [stTemporary].UsedSize;
        SymbolTable [stTemporary].UsedSize := SavedTempSymbolTableCurrentPointerOfs;
        DIPtr := Ptr (SymbolTable [stTemporary].Segment, ProcedureIdentifierDataPtr^.LocalIdentifiersList);
        SIPtr := Ptr (SymbolTable [stTemporary].Segment, SavedTempSymbolTableCurrentPointerOfs);
        While Ofs (SIPtr^) <> NewTempSymbolTableCurrentPointerOfs do
          begin
            Inc (SIPtr, 2);
            Inc (DIPtr, 2);
            If not IdentifiersEqual (PString (DIPtr), PString (SIPtr)) then Error (HeaderDoesNotMatchPreviousDefinition);
            Inc (DIPtr, Length (PString (DIPtr)^) + 1);
          end;
      end;
    ExpectTokenAndGetNext (TOKEN_Semicolon);
    ProcessProcedureDeclaractionsAndProgramBlock;
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
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-declarations
- https://turbopascal.org/processing-procedure-declarations
- https://turbopascal.org/processing-program-block
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

