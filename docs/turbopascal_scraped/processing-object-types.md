# Processing Object Types - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-object-types

---

## Table of Contents

- Processing Object Types

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Object Types
Turbo Pascal includes many functions and procedures to process object type declaration. The reason is in the complexity of object type. It can contain fields, methods, constructors, destructors, private and public members, static methods, dynamic methods, virtual methods, etc.
Function ProcessObjectTypeDeclaration (TypeIdentifier: PIdentifier): PTypeDefinition;
Type TProc2 = Procedure  (MethodIdentifierData: PProcedureIdentifierData; Index: Word);
Var ObjectType, AncestorTypeDefinition: PObjectTypeDefinition;
    Saved_LastPointerToTypeDefinitionOffset: Word;
    Last_UnitW30: Word;
    UnitIdentifierData: PUnitIdentifierData;
    AncestorUnitTypeOffset: TUnitOffsets;
    ObjectDataType: TIntegerTypeSet;
    Data_W02, Data_W10, Data_W14, Data_W1E, Data_W20, Data_W22, Data_W24: Word;
begin
  Saved_LastPointerToTypeDefinitionOffset := LastPointerToTypeDefinitionOffset;
  If CurrentProcedureIdentifier <> 0 then Error (LocalObjectTypesAreNotAllowed);
  GetNextToken;
  ObjectType := CreateTypeDefinition (SizeOf (TObjectTypeDefinition), 0, [], btObject);
  With ObjectType^ do
    begin
      TypeIdentifierOffset := Ofs (TypeIdentifier^);
      W1A := 0;
      W1C := 0;
    end;
  Last_UnitW30 := 0;
  If (SourceType <> stUnitImplementation) or (LocalDebugSymbols in ModuleCompilerSwitches) then
    begin
      Last_UnitW30 := PUnitHeader (Ptr (Seg (ObjectType^), 0))^.W30;
      PUnitHeader (Ptr (Seg (ObjectType^), 0))^.W30 := Ofs (ObjectType^);
    end;
  ObjectType^.W18 := Last_UnitW30;
  CurrentRecordOrObjectTypeDefinitionOffset := Ofs (ObjectType^);
  If CheckAndGetNextToken (Token_LeftParenthesis) then
    begin
      AncestorTypeDefinition := PObjectTypeDefinition (ExpectTypeIdentifier);
      If AncestorTypeDefinition^.BaseType <> btObject then Error (ObjectTypeExpected);
      ExpectTokenAndGetNext (Token_RightParenthesis);
      ObjectDataType := [];
      UnitIdentifierData := Ptr (Seg (AncestorTypeDefinition^), AncestorTypeDefinition^.W24);
      Data_W24 := UnitIdentifierData^.UnitSegment;
      Data_W22 := AncestorTypeDefinition^.W22;
      UnitIdentifierData := Ptr (Seg (AncestorTypeDefinition^), AncestorTypeDefinition^.W20);
      Data_W20 := UnitIdentifierData^.UnitSegment;
      Data_W1E := AncestorTypeDefinition^.W1E;
      Data_W14 := AncestorTypeDefinition^.OffsetOf_VMT_Offset;
      Data_W10 := AncestorTypeDefinition^.VMT_Size;
      Data_W02 := AncestorTypeDefinition^.Size;
      GetTypeAndUnitIdentifierOffsets (Pointer (AncestorTypeDefinition), AncestorUnitTypeOffset);
    end else begin
               ObjectDataType := [];
               Data_W24 := SystemUnitSegment;
               Data_W22 := $0118;
               Data_W20 := SystemUnitSegment;
               Data_W1E := $0110;
               Data_W14 := $FFFF;
               Data_W10 := $0000;
               Data_W02 := $0000;
               AncestorUnitTypeOffset.UnitAndTypeOffset := 0;
             end;
  With ObjectType^ do
    begin
      DataType := ObjectDataType;
      AncestorTypeOffset := AncestorUnitTypeOffset;
      Size := Data_W02;
      VMT_Size := Data_W10;
      OffsetOf_VMT_Offset := Data_W14;
      W1E := Data_W1E;
      W20 := UnitIdentifierDataOffset (Data_W20);
      W22 := Data_W22;
      W24 := UnitIdentifierDataOffset (Data_W24);
      W0A := 0;
      W1A := 0;
      W1C := 0;
      VMT_TypedConstantsBlockRecordOffset := $FFFF;
      W16 := $FFFF;
      FieldsListOffset := SymbolTable [stMain].NextRecordOffset;
    end;
  OffsetToNextMemberOffset := Ofs (ObjectType^.W0A);
  NumberOfDynamicMethods := 0;
  CreateSymbolTable (8);
  ProcessSectionOfMembers;
  Repeat
    If CheckAndGetNextToken (Token_PUBLIC) then PrivateFlagMask := $00 else
      If CheckAndGetNextToken (Token_PRIVATE) then PrivateFlagMask := $80 else Break;
    ProcessSectionOfMembers;
  until False;
  PrivateFlagMask := $00;
  ExpectTokenAndGetNext (Token_END);
  CreateVMT;
  CurrentRecordOrObjectTypeDefinitionOffset := 0;
  LastPointerToTypeDefinitionOffset := Saved_LastPointerToTypeDefinitionOffset;
  ProcessObjectTypeDeclaration := ObjectType;
end;
Dynamic methods need Dynamic Method Table (DMT) and virtual methods need Virtual Method Table (VMT). Few procedures take care for this.
Procedure CreateVMT;
  Var VMT: PVMT;
  begin
    With ObjectType^ do If VMT_Size <> 0 then
      begin
        If OffsetOf_VMT_Offset = $FFFF then
          begin
            OffsetOf_VMT_Offset := Size;
            Inc (Size, 2);
          end;
        CreateDMT;
        VMT := IncreaseSymbolTable (stTypedConstants, VMT_Size);
        With VMT^.Header do
          begin
            SizeOfObjectInstance := Size;
            NegativeSizeOfObjectInstance := - Size;
            DMT_Offset := 0;
            AlwaysZero := 0;
          end;
        FillChar (VMT^.VirtualMethodPointer [0], VMT_Size - SizeOf (TVMTHeader), $FF);
        SetOffsetToDMT (0, Ofs (VMT^.Header.DMT_Offset));
        Create_VMT_Entries;
        VMT_TypedConstantsBlockRecordOffset := SymbolTable [stTypedConstantsBlocks].UsedSize;
        CreateTypedConstantsBlockRecord;
      end;
  end;

    Procedure SetOffsetToDMT (B: Byte; TypedConstantOffset: Word);
    Label AncestorType;
    Var ObjectTypeDefinition: PObjectTypeDefinition;
    begin
      ObjectTypeDefinition := ObjectType;
      If B <> 0 then GoTo AncestorType;
      While ObjectTypeDefinition^.W16 = $FFFF do
        begin

AncestorType:

          If ObjectTypeDefinition^.AncestorTypeOffset.TypeOffset = 0 then Exit;
          ObjectTypeDefinition := PointerFromOffsets (ObjectTypeDefinition^.AncestorTypeOffset);
        end;
      AddReferenceRecordForTypedConstant (Seg (ObjectTypeDefinition^), ObjectTypeDefinition^.W16,
                                          [rfDataSegment, rfConstant, rfOffset], 0, TypedConstantOffset);
    end;

    Procedure Create_VMT_Entries;
    Var ObjectTypeDefinition: PObjectTypeDefinition;
    begin
      Repeat
        ObjectTypeDefinition := ObjectType;
        CallProcedureForEachMethod (ObjectTypeDefinition, Create_VMT_Entry, 0);
        If ObjectTypeDefinition^.AncestorTypeOffset.TypeOffset = 0 then Break;
        ObjectTypeDefinition := PointerFromOffsets (ObjectTypeDefinition^.AncestorTypeOffset);
      until False;
    end;

Procedure Create_VMT_Entry (MethodIdentifierData: PProcedureIdentifierData; Index: Word); Far;
begin
  With MethodIdentifierData^ do If (W8 << 0) and not (pfDynamic in Flags) and
      (Word (Ptr (SymbolTable [stTypedConstants].Segment, LastTypedConstantsSize + W8)^) <> 0) then
    begin
      Word (Ptr (SymbolTable [stTypedConstants].Segment, LastTypedConstantsSize + W8)^) := 0;
      Word (Ptr (SymbolTable [stTypedConstants].Segment, LastTypedConstantsSize + W8 + 2)^) := 0;
      AddReferenceRecordForTypedConstant (Seg (MethodIdentifierData^), ProceduresRecordOffset,
                                          [rfSegment, rfOffset], 0, LastTypedConstantsSize + W8);
    end;
end;

    Procedure Create_DMT_Entries;
    Var ObjectTypeDefinition: PObjectTypeDefinition;
    begin
      Repeat
        ObjectTypeDefinition := ObjectType;
        CallProcedureForEachMethod (ObjectTypeDefinition, Create_DMT_Entry, 0);
        If ObjectTypeDefinition^.AncestorTypeOffset.TypeOffset = 0 then Break;
        ObjectTypeDefinition := PointerFromOffsets (ObjectTypeDefinition^.AncestorTypeOffset);
      until False;
    end;

Procedure Create_DMT_Entry (MethodIdentifierData: PProcedureIdentifierData; Index: Word); Far;
begin
  With MethodIdentifierData^ do If pfDynamic in Flags then
    begin
      Word (Ptr (SymbolTable [stTypedConstants].Segment, CurrentTypedConstantValueOffset + DynamicMethodCounter * 2)^) := W8;
      AddReferenceRecordForTypedConstant (Seg (MethodIdentifierData^), ProceduresRecordOffset,
                    [rfSegment, rfOffset], 0, CurrentTypedConstantValueOffset + NumberOfDynamicMethods * 6);
      Inc (DynamicMethodCounter);
    end;
end;

    Procedure CreateDMT;
    Var DMT: PDMT;
    begin
      If NumberOfDynamicMethods <> 0 then
        begin
           DMT := IncreaseSymbolTable (stTypedConstants, NumberOfDynamicMethods * (SizeOf (Integer) + SizeOf (Pointer)) +
                                                         SizeOf (TDMTHeader));
           With DMT^.Header do
             begin
               Parent_DMT_Offset := 0;
               CachedIndex := 0;
               CachedEntryOffset := 0;
               EntryCount := NumberOfDynamicMethods;
             end;
          CurrentTypedConstantValueOffset := Ofs (DMT^.DynamicMethodIndexTable [0]);
          FillChar (DMT^.DynamicMethodIndexTable [0], NumberOfDynamicMethods * (SizeOf (Integer) + SizeOf (Pointer)), 0);
          SetOffsetToDMT (1, Ofs (DMT^.Header.Parent_DMT_Offset));
          Create_DMT_Entries;
          ObjectType^.W16 := SymbolTable [stTypedConstantsBlocks].UsedSize;
          CreateTypedConstantsBlockRecord;
        end;
    end;
This procedure goes through all object’s methods and calls specified procedure for each method. It is used to check for unique index for dynamic methods and to create entries in DMT and VMT.
Procedure CallProcedureForEachMethod (ObjectTypeDefinition: PObjectTypeDefinition; Proc2: TProc2; DynamicMethodIndex: Word);
  Var ObjectIdentifier: PIdentifier;
      ObjectIdentifierOfs: Word absolute ObjectIdentifier;
      VariableIdentifierData: PVariableIdentifierData;
      MethodIdentifierData: PProcedureIdentifierData absolute VariableIdentifierData;
  begin
    DynamicMethodCounter := 0;
    ObjectIdentifier := Pointer (ObjectTypeDefinition);
    ObjectIdentifierOfs := ObjectTypeDefinition^.W0A;
    While ObjectIdentifierOfs <> 0 do
      begin
        VariableIdentifierData := PVariableIdentifierData (PChar (ObjectIdentifier) + ObjectIdentifier^.Name.Len + 4);
        Case TToken (Ord (ObjectIdentifier^.Token) and $7F) of
          Token_VariableIdentifier: ObjectIdentifierOfs := VariableIdentifierData^.W5;
          else begin
                 Proc2 (MethodIdentifierData, DynamicMethodIndex);
                 ObjectIdentifierOfs := MethodIdentifierData^.ProcedureTypeDefinition.W06_;
               end;
        end;
      end;
  end;
This procedure processes members of Turbo Pascal object:
Private
and
Public
directives, fields, methods (procedures and functions), constructors and destructors.
Procedure ProcessSectionOfMembers;
  Var MethodsDeclared: Boolean;

  begin
    MethodsDeclared := False;
    Repeat
      CheckIfDirecive (_PUBLIC, Token_PUBLIC);
      CheckIfDirecive (_PRIVATE, Token_PRIVATE);
      Case Token of
        Token_PROCEDURE,
        Token_FUNCTION: begin
                          ProcessMethodDeclaration (Token);
                          MethodsDeclared := True;
                          Continue;
                        end;
        Token_CONSTRUCTOR,
        Token_DESTRUCTOR: begin
                            ProcessMethodDeclaration (Token);
                            MethodsDeclared := True;
                            Continue;
                            With PObjectTypeDefinition (Ptr (SymbolTable [stMain].Segment,
                                                             CurrentRecordOrObjectTypeDefinitionOffset))^ do
                              If VMT_Size = 0 then VMT_Size := 8;
                          end;
      end;
      If MethodsDeclared then Exit;
      Case Token of
        Token_PUBLIC,
        Token_PRIVATE,
        Token_END: Exit;
      end;
      ProcessCommaSeparatedFieldsAndType;
      CalculateVariableOffsets;
      ExpectTokenAndGetNext (Token_Semicolon);
      MethodsDeclared := False;
    until False;
  end;

    Procedure ProcessMethodDeclaration (MethodToken: TToken);
    Var MethodIdentifierOffset: Word;
        MethodIdentifier: PIdentifier;
        MethodIdentifierData: PProcedureIdentifierData;
        MethodIdentifierToken: TToken;
        MethodType: PProcedureTypeDefinition;
        AncestorMethodIdentifierData: PProcedureIdentifierData;
    begin
      GetNextToken;
      ExpectIdentifier;
      If CurrentIdentifierDeclaredInCurrentScope (MethodIdentifierOffset, Pointer (MethodIdentifierData),
                                                    MethodIdentifierToken) then
        begin
          If MethodIdentifierToken <> Token_ProcedureIdentifier then Error (DuplicateIdentifier);
          If Ptr (Seg (MethodIdentifierData^), MethodIdentifierData^.OuterBlockProcedureIdentifier) =
             Ptr (SymbolTable [stMain].Segment, CurrentRecordOrObjectTypeDefinitionOffset) then Error (DuplicateIdentifier);
          AncestorMethodIdentifierData := MethodIdentifierData;
        end else AncestorMethodIdentifierData := nil;
      MethodIdentifierData := StoreCurrentIdentifierToSymbolTable (CurrentScopeIdentifierTableAddress, 10, MethodIdentifier);
      GetNextToken;
      MethodIdentifier^.Token := TToken (Ord (Token_ProcedureIdentifier) or PrivateFlagMask);
      MethodIdentifierData^.OuterBlockProcedureIdentifier := CurrentRecordOrObjectTypeDefinitionOffset;
      MethodIdentifierData^.LocalIdentifiersList := SymbolTable [stTemporary].UsedSize;
      Word (Ptr (Seg (MethodIdentifierData^), OffsetToNextMemberOffset)^) := Ofs (MethodIdentifier^);
      OffsetToNextMemberOffset := Ofs (MethodIdentifierData^.ProcedureTypeDefinition.W06_);
      CreateProcedureRecord (MethodIdentifier, MethodIdentifierData);
      Case MethodToken of
        Token_CONSTRUCTOR: MethodIdentifierData^.Flags := [pfConstructor, pfMethod, pfFar];
        Token_DESTRUCTOR : MethodIdentifierData^.Flags := [pfDestructor,  pfMethod, pfFar];
        else               MethodIdentifierData^.Flags := [               pfMethod, pfFar];
      end;
      MethodType := ProcessProcedureHeader (MethodToken);
      MethodType^.Size := 8;
      ExpectTokenAndGetNext (Token_Semicolon);
      If (AncestorMethodIdentifierData = nil) or (AncestorMethodIdentifierData^.W8 = 0) then
        CheckIfVirtualMethod else
          CheckOverridenVirtualMethod;
    end;

      Procedure CheckIfVirtualMethod;
      Var ObjectTypeDefinition: PObjectTypeDefinition;
          DynamicMethodIndex: Word;

      begin
        If CompareIdentifierToDirectiveAndGetNextToken (_VIRTUAL) then
          begin
            ObjectTypeDefinition := Ptr (SymbolTable [stMain].Segment, CurrentRecordOrObjectTypeDefinitionOffset);
            With ObjectTypeDefinition^ do If VMT_Size = 0 then VMT_Size := SizeOf (TVMTHeader);
        If pfConstructor in MethodIdentifierData^.Flags then Error (VirtualConstructorsAreNotAllowed);
            Case Token of
              Token_Semicolon: begin
                                 DynamicMethodIndex := ExpectIntegerConstant;
                                 If DynamicMethodIndex = 0 then Error (DuplicateDynamicMethodIndex);
                                 Repeat
                                   CallProcedureForEachMethod (ObjectTypeDefinition,
                                                               CheckIfUniqueDynamicMethodIndex,
                                                               DynamicMethodIndex);
                                   If ObjectTypeDefinition^.AncestorTypeOffset.TypeOffset = 0 then Break;
                                   ObjectTypeDefinition := PointerFromOffsets (ObjectTypeDefinition^.AncestorTypeOffset);
                                 until False;
                                 MethodIdentifierData^.W8 := DynamicMethodIndex;
                                 Include (MethodIdentifierData^.Flags, pfDynamic);
                               end;
              else begin
                     MethodIdentifierData^.W8 := ObjectTypeDefinition^.VMT_Size;
                     Inc (ObjectTypeDefinition^.VMT_Size, SizeOf (Pointer));
                   end;
            end;
            ExpectTokenAndGetNext (Token_Semicolon);
          end else MethodIdentifierData^.W8 := 0;
      end;

      Procedure CheckOverridenVirtualMethod;
      Var DynamicMethodIndex: Word;
      begin
        If CompareIdentifierToDirectiveAndGetNextToken (_VIRTUAL) then
          begin
            If pfDynamic in AncestorMethodIdentifierData^.Flags then
              begin
                If Token = Token_Semicolon then Error (IntegerConstantExpected);
                DynamicMethodIndex := ExpectIntegerConstant;
                If DynamicMethodIndex <> AncestorMethodIdentifierData^.W8 then
                  Error (HeaderDoesNotMatchPreviousDefinition);
                If (AncestorMethodIdentifierData^.Flags * [pfConstructor, pfDestructor]) <>
                   (MethodIdentifierData^.Flags * [pfConstructor, pfDestructor]) then
                     Error (HeaderDoesNotMatchPreviousDefinition);
                If not CheckProcedureTypeCompatibility (@MethodIdentifierData^.ProcedureTypeDefinition,
                                                        @AncestorMethodIdentifierData^.ProcedureTypeDefinition) then
                  Error (HeaderDoesNotMatchPreviousDefinition);
                ExpectTokenAndGetNext (Token_Semicolon);
                MethodIdentifierData^.W8 := DynamicMethodIndex;
                MethodIdentifierData^.Flags :=
                  MethodIdentifierData^.Flags + AncestorMethodIdentifierData^.Flags * [pfDynamic, pf0100];
              end;
          end else Error (VIRTUAL_Expected);
      end;
Dynamic method indexes need to be unique.
Procedure CheckIfUniqueDynamicMethodIndex (MethodIdentifierData: PProcedureIdentifierData; Index: Word); Far;
begin
  With MethodIdentifierData^ do
    If pfDynamic in Flags then If Index = W8 then Error (DuplicateDynamicMethodIndex);
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
Function ProcessObjectTypeDeclaration (TypeIdentifier: PIdentifier): PTypeDefinition;
Type TProc2 = Procedure  (MethodIdentifierData: PProcedureIdentifierData; Index: Word);
Var ObjectType, AncestorTypeDefinition: PObjectTypeDefinition;
    Saved_LastPointerToTypeDefinitionOffset: Word;
    Last_UnitW30: Word;
    UnitIdentifierData: PUnitIdentifierData;
    AncestorUnitTypeOffset: TUnitOffsets;
    ObjectDataType: TIntegerTypeSet;
    Data_W02, Data_W10, Data_W14, Data_W1E, Data_W20, Data_W22, Data_W24: Word;
begin
  Saved_LastPointerToTypeDefinitionOffset := LastPointerToTypeDefinitionOffset;
  If CurrentProcedureIdentifier <> 0 then Error (LocalObjectTypesAreNotAllowed);
  GetNextToken;
  ObjectType := CreateTypeDefinition (SizeOf (TObjectTypeDefinition), 0, [], btObject);
  With ObjectType^ do
    begin
      TypeIdentifierOffset := Ofs (TypeIdentifier^);
      W1A := 0;
      W1C := 0;
    end;
  Last_UnitW30 := 0;
  If (SourceType <> stUnitImplementation) or (LocalDebugSymbols in ModuleCompilerSwitches) then
    begin
      Last_UnitW30 := PUnitHeader (Ptr (Seg (ObjectType^), 0))^.W30;
      PUnitHeader (Ptr (Seg (ObjectType^), 0))^.W30 := Ofs (ObjectType^);
    end;
  ObjectType^.W18 := Last_UnitW30;
  CurrentRecordOrObjectTypeDefinitionOffset := Ofs (ObjectType^);
  If CheckAndGetNextToken (Token_LeftParenthesis) then
    begin
      AncestorTypeDefinition := PObjectTypeDefinition (ExpectTypeIdentifier);
      If AncestorTypeDefinition^.BaseType <> btObject then Error (ObjectTypeExpected);
      ExpectTokenAndGetNext (Token_RightParenthesis);
      ObjectDataType := [];
      UnitIdentifierData := Ptr (Seg (AncestorTypeDefinition^), AncestorTypeDefinition^.W24);
      Data_W24 := UnitIdentifierData^.UnitSegment;
      Data_W22 := AncestorTypeDefinition^.W22;
      UnitIdentifierData := Ptr (Seg (AncestorTypeDefinition^), AncestorTypeDefinition^.W20);
      Data_W20 := UnitIdentifierData^.UnitSegment;
      Data_W1E := AncestorTypeDefinition^.W1E;
      Data_W14 := AncestorTypeDefinition^.OffsetOf_VMT_Offset;
      Data_W10 := AncestorTypeDefinition^.VMT_Size;
      Data_W02 := AncestorTypeDefinition^.Size;
      GetTypeAndUnitIdentifierOffsets (Pointer (AncestorTypeDefinition), AncestorUnitTypeOffset);
    end else begin
               ObjectDataType := [];
               Data_W24 := SystemUnitSegment;
               Data_W22 := $0118;
               Data_W20 := SystemUnitSegment;
               Data_W1E := $0110;
               Data_W14 := $FFFF;
               Data_W10 := $0000;
               Data_W02 := $0000;
               AncestorUnitTypeOffset.UnitAndTypeOffset := 0;
             end;
  With ObjectType^ do
    begin
      DataType := ObjectDataType;
      AncestorTypeOffset := AncestorUnitTypeOffset;
      Size := Data_W02;
      VMT_Size := Data_W10;
      OffsetOf_VMT_Offset := Data_W14;
      W1E := Data_W1E;
      W20 := UnitIdentifierDataOffset (Data_W20);
      W22 := Data_W22;
      W24 := UnitIdentifierDataOffset (Data_W24);
      W0A := 0;
      W1A := 0;
      W1C := 0;
      VMT_TypedConstantsBlockRecordOffset := $FFFF;
      W16 := $FFFF;
      FieldsListOffset := SymbolTable [stMain].NextRecordOffset;
    end;
  OffsetToNextMemberOffset := Ofs (ObjectType^.W0A);
  NumberOfDynamicMethods := 0;
  CreateSymbolTable (8);
  ProcessSectionOfMembers;
  Repeat
    If CheckAndGetNextToken (Token_PUBLIC) then PrivateFlagMask := $00 else
      If CheckAndGetNextToken (Token_PRIVATE) then PrivateFlagMask := $80 else Break;
    ProcessSectionOfMembers;
  until False;
  PrivateFlagMask := $00;
  ExpectTokenAndGetNext (Token_END);
  CreateVMT;
  CurrentRecordOrObjectTypeDefinitionOffset := 0;
  LastPointerToTypeDefinitionOffset := Saved_LastPointerToTypeDefinitionOffset;
  ProcessObjectTypeDeclaration := ObjectType;
end;
```

### Code Block 2

```pascal
  Procedure CreateVMT;
  Var VMT: PVMT;
  begin
    With ObjectType^ do If VMT_Size <> 0 then
      begin
        If OffsetOf_VMT_Offset = $FFFF then
          begin
            OffsetOf_VMT_Offset := Size;
            Inc (Size, 2);
          end;
        CreateDMT;
        VMT := IncreaseSymbolTable (stTypedConstants, VMT_Size);
        With VMT^.Header do
          begin
            SizeOfObjectInstance := Size;
            NegativeSizeOfObjectInstance := - Size;
            DMT_Offset := 0;
            AlwaysZero := 0;
          end;
        FillChar (VMT^.VirtualMethodPointer [0], VMT_Size - SizeOf (TVMTHeader), $FF);
        SetOffsetToDMT (0, Ofs (VMT^.Header.DMT_Offset));
        Create_VMT_Entries;
        VMT_TypedConstantsBlockRecordOffset := SymbolTable [stTypedConstantsBlocks].UsedSize;
        CreateTypedConstantsBlockRecord;
      end;
  end;

    Procedure SetOffsetToDMT (B: Byte; TypedConstantOffset: Word);
    Label AncestorType;
    Var ObjectTypeDefinition: PObjectTypeDefinition;
    begin
      ObjectTypeDefinition := ObjectType;
      If B <> 0 then GoTo AncestorType;
      While ObjectTypeDefinition^.W16 = $FFFF do
        begin

AncestorType:

          If ObjectTypeDefinition^.AncestorTypeOffset.TypeOffset = 0 then Exit;
          ObjectTypeDefinition := PointerFromOffsets (ObjectTypeDefinition^.AncestorTypeOffset);
        end;
      AddReferenceRecordForTypedConstant (Seg (ObjectTypeDefinition^), ObjectTypeDefinition^.W16,
                                          [rfDataSegment, rfConstant, rfOffset], 0, TypedConstantOffset);
    end;

    Procedure Create_VMT_Entries;
    Var ObjectTypeDefinition: PObjectTypeDefinition;
    begin
      Repeat
        ObjectTypeDefinition := ObjectType;
        CallProcedureForEachMethod (ObjectTypeDefinition, Create_VMT_Entry, 0);
        If ObjectTypeDefinition^.AncestorTypeOffset.TypeOffset = 0 then Break;
        ObjectTypeDefinition := PointerFromOffsets (ObjectTypeDefinition^.AncestorTypeOffset);
      until False;
    end;

Procedure Create_VMT_Entry (MethodIdentifierData: PProcedureIdentifierData; Index: Word); Far;
begin
  With MethodIdentifierData^ do If (W8 << 0) and not (pfDynamic in Flags) and
      (Word (Ptr (SymbolTable [stTypedConstants].Segment, LastTypedConstantsSize + W8)^) <> 0) then
    begin
      Word (Ptr (SymbolTable [stTypedConstants].Segment, LastTypedConstantsSize + W8)^) := 0;
      Word (Ptr (SymbolTable [stTypedConstants].Segment, LastTypedConstantsSize + W8 + 2)^) := 0;
      AddReferenceRecordForTypedConstant (Seg (MethodIdentifierData^), ProceduresRecordOffset,
                                          [rfSegment, rfOffset], 0, LastTypedConstantsSize + W8);
    end;
end;

    Procedure Create_DMT_Entries;
    Var ObjectTypeDefinition: PObjectTypeDefinition;
    begin
      Repeat
        ObjectTypeDefinition := ObjectType;
        CallProcedureForEachMethod (ObjectTypeDefinition, Create_DMT_Entry, 0);
        If ObjectTypeDefinition^.AncestorTypeOffset.TypeOffset = 0 then Break;
        ObjectTypeDefinition := PointerFromOffsets (ObjectTypeDefinition^.AncestorTypeOffset);
      until False;
    end;

Procedure Create_DMT_Entry (MethodIdentifierData: PProcedureIdentifierData; Index: Word); Far;
begin
  With MethodIdentifierData^ do If pfDynamic in Flags then
    begin
      Word (Ptr (SymbolTable [stTypedConstants].Segment, CurrentTypedConstantValueOffset + DynamicMethodCounter * 2)^) := W8;
      AddReferenceRecordForTypedConstant (Seg (MethodIdentifierData^), ProceduresRecordOffset,
                    [rfSegment, rfOffset], 0, CurrentTypedConstantValueOffset + NumberOfDynamicMethods * 6);
      Inc (DynamicMethodCounter);
    end;
end;

    Procedure CreateDMT;
    Var DMT: PDMT;
    begin
      If NumberOfDynamicMethods <> 0 then
        begin
           DMT := IncreaseSymbolTable (stTypedConstants, NumberOfDynamicMethods * (SizeOf (Integer) + SizeOf (Pointer)) +
                                                         SizeOf (TDMTHeader));
           With DMT^.Header do
             begin
               Parent_DMT_Offset := 0;
               CachedIndex := 0;
               CachedEntryOffset := 0;
               EntryCount := NumberOfDynamicMethods;
             end;
          CurrentTypedConstantValueOffset := Ofs (DMT^.DynamicMethodIndexTable [0]);
          FillChar (DMT^.DynamicMethodIndexTable [0], NumberOfDynamicMethods * (SizeOf (Integer) + SizeOf (Pointer)), 0);
          SetOffsetToDMT (1, Ofs (DMT^.Header.Parent_DMT_Offset));
          Create_DMT_Entries;
          ObjectType^.W16 := SymbolTable [stTypedConstantsBlocks].UsedSize;
          CreateTypedConstantsBlockRecord;
        end;
    end;
```

### Code Block 3

```pascal
  Procedure CallProcedureForEachMethod (ObjectTypeDefinition: PObjectTypeDefinition; Proc2: TProc2; DynamicMethodIndex: Word);
  Var ObjectIdentifier: PIdentifier;
      ObjectIdentifierOfs: Word absolute ObjectIdentifier;
      VariableIdentifierData: PVariableIdentifierData;
      MethodIdentifierData: PProcedureIdentifierData absolute VariableIdentifierData;
  begin
    DynamicMethodCounter := 0;
    ObjectIdentifier := Pointer (ObjectTypeDefinition);
    ObjectIdentifierOfs := ObjectTypeDefinition^.W0A;
    While ObjectIdentifierOfs <> 0 do
      begin
        VariableIdentifierData := PVariableIdentifierData (PChar (ObjectIdentifier) + ObjectIdentifier^.Name.Len + 4);
        Case TToken (Ord (ObjectIdentifier^.Token) and $7F) of
          Token_VariableIdentifier: ObjectIdentifierOfs := VariableIdentifierData^.W5;
          else begin
                 Proc2 (MethodIdentifierData, DynamicMethodIndex);
                 ObjectIdentifierOfs := MethodIdentifierData^.ProcedureTypeDefinition.W06_;
               end;
        end;
      end;
  end;
```

### Code Block 4

```pascal
  Procedure ProcessSectionOfMembers;
  Var MethodsDeclared: Boolean;

  begin
    MethodsDeclared := False;
    Repeat
      CheckIfDirecive (_PUBLIC, Token_PUBLIC);
      CheckIfDirecive (_PRIVATE, Token_PRIVATE);
      Case Token of
        Token_PROCEDURE,
        Token_FUNCTION: begin
                          ProcessMethodDeclaration (Token);
                          MethodsDeclared := True;
                          Continue;
                        end;
        Token_CONSTRUCTOR,
        Token_DESTRUCTOR: begin
                            ProcessMethodDeclaration (Token);
                            MethodsDeclared := True;
                            Continue;
                            With PObjectTypeDefinition (Ptr (SymbolTable [stMain].Segment,
                                                             CurrentRecordOrObjectTypeDefinitionOffset))^ do
                              If VMT_Size = 0 then VMT_Size := 8;
                          end;
      end;
      If MethodsDeclared then Exit;
      Case Token of
        Token_PUBLIC,
        Token_PRIVATE,
        Token_END: Exit;
      end;
      ProcessCommaSeparatedFieldsAndType;
      CalculateVariableOffsets;
      ExpectTokenAndGetNext (Token_Semicolon);
      MethodsDeclared := False;
    until False;
  end;

    Procedure ProcessMethodDeclaration (MethodToken: TToken);
    Var MethodIdentifierOffset: Word;
        MethodIdentifier: PIdentifier;
        MethodIdentifierData: PProcedureIdentifierData;
        MethodIdentifierToken: TToken;
        MethodType: PProcedureTypeDefinition;
        AncestorMethodIdentifierData: PProcedureIdentifierData;
    begin
      GetNextToken;
      ExpectIdentifier;
      If CurrentIdentifierDeclaredInCurrentScope (MethodIdentifierOffset, Pointer (MethodIdentifierData),
                                                    MethodIdentifierToken) then
        begin
          If MethodIdentifierToken <> Token_ProcedureIdentifier then Error (DuplicateIdentifier);
          If Ptr (Seg (MethodIdentifierData^), MethodIdentifierData^.OuterBlockProcedureIdentifier) =
             Ptr (SymbolTable [stMain].Segment, CurrentRecordOrObjectTypeDefinitionOffset) then Error (DuplicateIdentifier);
          AncestorMethodIdentifierData := MethodIdentifierData;
        end else AncestorMethodIdentifierData := nil;
      MethodIdentifierData := StoreCurrentIdentifierToSymbolTable (CurrentScopeIdentifierTableAddress, 10, MethodIdentifier);
      GetNextToken;
      MethodIdentifier^.Token := TToken (Ord (Token_ProcedureIdentifier) or PrivateFlagMask);
      MethodIdentifierData^.OuterBlockProcedureIdentifier := CurrentRecordOrObjectTypeDefinitionOffset;
      MethodIdentifierData^.LocalIdentifiersList := SymbolTable [stTemporary].UsedSize;
      Word (Ptr (Seg (MethodIdentifierData^), OffsetToNextMemberOffset)^) := Ofs (MethodIdentifier^);
      OffsetToNextMemberOffset := Ofs (MethodIdentifierData^.ProcedureTypeDefinition.W06_);
      CreateProcedureRecord (MethodIdentifier, MethodIdentifierData);
      Case MethodToken of
        Token_CONSTRUCTOR: MethodIdentifierData^.Flags := [pfConstructor, pfMethod, pfFar];
        Token_DESTRUCTOR : MethodIdentifierData^.Flags := [pfDestructor,  pfMethod, pfFar];
        else               MethodIdentifierData^.Flags := [               pfMethod, pfFar];
      end;
      MethodType := ProcessProcedureHeader (MethodToken);
      MethodType^.Size := 8;
      ExpectTokenAndGetNext (Token_Semicolon);
      If (AncestorMethodIdentifierData = nil) or (AncestorMethodIdentifierData^.W8 = 0) then
        CheckIfVirtualMethod else
          CheckOverridenVirtualMethod;
    end;

      Procedure CheckIfVirtualMethod;
      Var ObjectTypeDefinition: PObjectTypeDefinition;
          DynamicMethodIndex: Word;

      begin
        If CompareIdentifierToDirectiveAndGetNextToken (_VIRTUAL) then
          begin
            ObjectTypeDefinition := Ptr (SymbolTable [stMain].Segment, CurrentRecordOrObjectTypeDefinitionOffset);
            With ObjectTypeDefinition^ do If VMT_Size = 0 then VMT_Size := SizeOf (TVMTHeader);
        If pfConstructor in MethodIdentifierData^.Flags then Error (VirtualConstructorsAreNotAllowed);
            Case Token of
              Token_Semicolon: begin
                                 DynamicMethodIndex := ExpectIntegerConstant;
                                 If DynamicMethodIndex = 0 then Error (DuplicateDynamicMethodIndex);
                                 Repeat
                                   CallProcedureForEachMethod (ObjectTypeDefinition,
                                                               CheckIfUniqueDynamicMethodIndex,
                                                               DynamicMethodIndex);
                                   If ObjectTypeDefinition^.AncestorTypeOffset.TypeOffset = 0 then Break;
                                   ObjectTypeDefinition := PointerFromOffsets (ObjectTypeDefinition^.AncestorTypeOffset);
                                 until False;
                                 MethodIdentifierData^.W8 := DynamicMethodIndex;
                                 Include (MethodIdentifierData^.Flags, pfDynamic);
                               end;
              else begin
                     MethodIdentifierData^.W8 := ObjectTypeDefinition^.VMT_Size;
                     Inc (ObjectTypeDefinition^.VMT_Size, SizeOf (Pointer));
                   end;
            end;
            ExpectTokenAndGetNext (Token_Semicolon);
          end else MethodIdentifierData^.W8 := 0;
      end;

      Procedure CheckOverridenVirtualMethod;
      Var DynamicMethodIndex: Word;
      begin
        If CompareIdentifierToDirectiveAndGetNextToken (_VIRTUAL) then
          begin
            If pfDynamic in AncestorMethodIdentifierData^.Flags then
              begin
                If Token = Token_Semicolon then Error (IntegerConstantExpected);
                DynamicMethodIndex := ExpectIntegerConstant;
                If DynamicMethodIndex <> AncestorMethodIdentifierData^.W8 then
                  Error (HeaderDoesNotMatchPreviousDefinition);
                If (AncestorMethodIdentifierData^.Flags * [pfConstructor, pfDestructor]) <>
                   (MethodIdentifierData^.Flags * [pfConstructor, pfDestructor]) then
                     Error (HeaderDoesNotMatchPreviousDefinition);
                If not CheckProcedureTypeCompatibility (@MethodIdentifierData^.ProcedureTypeDefinition,
                                                        @AncestorMethodIdentifierData^.ProcedureTypeDefinition) then
                  Error (HeaderDoesNotMatchPreviousDefinition);
                ExpectTokenAndGetNext (Token_Semicolon);
                MethodIdentifierData^.W8 := DynamicMethodIndex;
                MethodIdentifierData^.Flags :=
                  MethodIdentifierData^.Flags + AncestorMethodIdentifierData^.Flags * [pfDynamic, pf0100];
              end;
          end else Error (VIRTUAL_Expected);
      end;
```

### Code Block 5

```pascal
Procedure CheckIfUniqueDynamicMethodIndex (MethodIdentifierData: PProcedureIdentifierData; Index: Word); Far;
begin
  With MethodIdentifierData^ do
    If pfDynamic in Flags then If Index = W8 then Error (DuplicateDynamicMethodIndex);
end;
```

### Code Block 6

```pascal
Function ProcessObjectTypeDeclaration (TypeIdentifier: PIdentifier): PTypeDefinition;
Type TProc2 = Procedure  (MethodIdentifierData: PProcedureIdentifierData; Index: Word);
Var ObjectType, AncestorTypeDefinition: PObjectTypeDefinition;
    Saved_LastPointerToTypeDefinitionOffset: Word;
    Last_UnitW30: Word;
    UnitIdentifierData: PUnitIdentifierData;
    AncestorUnitTypeOffset: TUnitOffsets;
    ObjectDataType: TIntegerTypeSet;
    Data_W02, Data_W10, Data_W14, Data_W1E, Data_W20, Data_W22, Data_W24: Word;
begin
  Saved_LastPointerToTypeDefinitionOffset := LastPointerToTypeDefinitionOffset;
  If CurrentProcedureIdentifier <> 0 then Error (LocalObjectTypesAreNotAllowed);
  GetNextToken;
  ObjectType := CreateTypeDefinition (SizeOf (TObjectTypeDefinition), 0, [], btObject);
  With ObjectType^ do
    begin
      TypeIdentifierOffset := Ofs (TypeIdentifier^);
      W1A := 0;
      W1C := 0;
    end;
  Last_UnitW30 := 0;
  If (SourceType <> stUnitImplementation) or (LocalDebugSymbols in ModuleCompilerSwitches) then
    begin
      Last_UnitW30 := PUnitHeader (Ptr (Seg (ObjectType^), 0))^.W30;
      PUnitHeader (Ptr (Seg (ObjectType^), 0))^.W30 := Ofs (ObjectType^);
    end;
  ObjectType^.W18 := Last_UnitW30;
  CurrentRecordOrObjectTypeDefinitionOffset := Ofs (ObjectType^);
  If CheckAndGetNextToken (Token_LeftParenthesis) then
    begin
      AncestorTypeDefinition := PObjectTypeDefinition (ExpectTypeIdentifier);
      If AncestorTypeDefinition^.BaseType <> btObject then Error (ObjectTypeExpected);
      ExpectTokenAndGetNext (Token_RightParenthesis);
      ObjectDataType := [];
      UnitIdentifierData := Ptr (Seg (AncestorTypeDefinition^), AncestorTypeDefinition^.W24);
      Data_W24 := UnitIdentifierData^.UnitSegment;
      Data_W22 := AncestorTypeDefinition^.W22;
      UnitIdentifierData := Ptr (Seg (AncestorTypeDefinition^), AncestorTypeDefinition^.W20);
      Data_W20 := UnitIdentifierData^.UnitSegment;
      Data_W1E := AncestorTypeDefinition^.W1E;
      Data_W14 := AncestorTypeDefinition^.OffsetOf_VMT_Offset;
      Data_W10 := AncestorTypeDefinition^.VMT_Size;
      Data_W02 := AncestorTypeDefinition^.Size;
      GetTypeAndUnitIdentifierOffsets (Pointer (AncestorTypeDefinition), AncestorUnitTypeOffset);
    end else begin
               ObjectDataType := [];
               Data_W24 := SystemUnitSegment;
               Data_W22 := $0118;
               Data_W20 := SystemUnitSegment;
               Data_W1E := $0110;
               Data_W14 := $FFFF;
               Data_W10 := $0000;
               Data_W02 := $0000;
               AncestorUnitTypeOffset.UnitAndTypeOffset := 0;
             end;
  With ObjectType^ do
    begin
      DataType := ObjectDataType;
      AncestorTypeOffset := AncestorUnitTypeOffset;
      Size := Data_W02;
      VMT_Size := Data_W10;
      OffsetOf_VMT_Offset := Data_W14;
      W1E := Data_W1E;
      W20 := UnitIdentifierDataOffset (Data_W20);
      W22 := Data_W22;
      W24 := UnitIdentifierDataOffset (Data_W24);
      W0A := 0;
      W1A := 0;
      W1C := 0;
      VMT_TypedConstantsBlockRecordOffset := $FFFF;
      W16 := $FFFF;
      FieldsListOffset := SymbolTable [stMain].NextRecordOffset;
    end;
  OffsetToNextMemberOffset := Ofs (ObjectType^.W0A);
  NumberOfDynamicMethods := 0;
  CreateSymbolTable (8);
  ProcessSectionOfMembers;
  Repeat
    If CheckAndGetNextToken (Token_PUBLIC) then PrivateFlagMask := $00 else
      If CheckAndGetNextToken (Token_PRIVATE) then PrivateFlagMask := $80 else Break;
    ProcessSectionOfMembers;
  until False;
  PrivateFlagMask := $00;
  ExpectTokenAndGetNext (Token_END);
  CreateVMT;
  CurrentRecordOrObjectTypeDefinitionOffset := 0;
  LastPointerToTypeDefinitionOffset := Saved_LastPointerToTypeDefinitionOffset;
  ProcessObjectTypeDeclaration := ObjectType;
end;
```

### Code Block 7

```pascal
  Procedure CreateVMT;
  Var VMT: PVMT;
  begin
    With ObjectType^ do If VMT_Size <> 0 then
      begin
        If OffsetOf_VMT_Offset = $FFFF then
          begin
            OffsetOf_VMT_Offset := Size;
            Inc (Size, 2);
          end;
        CreateDMT;
        VMT := IncreaseSymbolTable (stTypedConstants, VMT_Size);
        With VMT^.Header do
          begin
            SizeOfObjectInstance := Size;
            NegativeSizeOfObjectInstance := - Size;
            DMT_Offset := 0;
            AlwaysZero := 0;
          end;
        FillChar (VMT^.VirtualMethodPointer [0], VMT_Size - SizeOf (TVMTHeader), $FF);
        SetOffsetToDMT (0, Ofs (VMT^.Header.DMT_Offset));
        Create_VMT_Entries;
        VMT_TypedConstantsBlockRecordOffset := SymbolTable [stTypedConstantsBlocks].UsedSize;
        CreateTypedConstantsBlockRecord;
      end;
  end;

    Procedure SetOffsetToDMT (B: Byte; TypedConstantOffset: Word);
    Label AncestorType;
    Var ObjectTypeDefinition: PObjectTypeDefinition;
    begin
      ObjectTypeDefinition := ObjectType;
      If B <> 0 then GoTo AncestorType;
      While ObjectTypeDefinition^.W16 = $FFFF do
        begin

AncestorType:

          If ObjectTypeDefinition^.AncestorTypeOffset.TypeOffset = 0 then Exit;
          ObjectTypeDefinition := PointerFromOffsets (ObjectTypeDefinition^.AncestorTypeOffset);
        end;
      AddReferenceRecordForTypedConstant (Seg (ObjectTypeDefinition^), ObjectTypeDefinition^.W16,
                                          [rfDataSegment, rfConstant, rfOffset], 0, TypedConstantOffset);
    end;

    Procedure Create_VMT_Entries;
    Var ObjectTypeDefinition: PObjectTypeDefinition;
    begin
      Repeat
        ObjectTypeDefinition := ObjectType;
        CallProcedureForEachMethod (ObjectTypeDefinition, Create_VMT_Entry, 0);
        If ObjectTypeDefinition^.AncestorTypeOffset.TypeOffset = 0 then Break;
        ObjectTypeDefinition := PointerFromOffsets (ObjectTypeDefinition^.AncestorTypeOffset);
      until False;
    end;

Procedure Create_VMT_Entry (MethodIdentifierData: PProcedureIdentifierData; Index: Word); Far;
begin
  With MethodIdentifierData^ do If (W8 << 0) and not (pfDynamic in Flags) and
      (Word (Ptr (SymbolTable [stTypedConstants].Segment, LastTypedConstantsSize + W8)^) <> 0) then
    begin
      Word (Ptr (SymbolTable [stTypedConstants].Segment, LastTypedConstantsSize + W8)^) := 0;
      Word (Ptr (SymbolTable [stTypedConstants].Segment, LastTypedConstantsSize + W8 + 2)^) := 0;
      AddReferenceRecordForTypedConstant (Seg (MethodIdentifierData^), ProceduresRecordOffset,
                                          [rfSegment, rfOffset], 0, LastTypedConstantsSize + W8);
    end;
end;

    Procedure Create_DMT_Entries;
    Var ObjectTypeDefinition: PObjectTypeDefinition;
    begin
      Repeat
        ObjectTypeDefinition := ObjectType;
        CallProcedureForEachMethod (ObjectTypeDefinition, Create_DMT_Entry, 0);
        If ObjectTypeDefinition^.AncestorTypeOffset.TypeOffset = 0 then Break;
        ObjectTypeDefinition := PointerFromOffsets (ObjectTypeDefinition^.AncestorTypeOffset);
      until False;
    end;

Procedure Create_DMT_Entry (MethodIdentifierData: PProcedureIdentifierData; Index: Word); Far;
begin
  With MethodIdentifierData^ do If pfDynamic in Flags then
    begin
      Word (Ptr (SymbolTable [stTypedConstants].Segment, CurrentTypedConstantValueOffset + DynamicMethodCounter * 2)^) := W8;
      AddReferenceRecordForTypedConstant (Seg (MethodIdentifierData^), ProceduresRecordOffset,
                    [rfSegment, rfOffset], 0, CurrentTypedConstantValueOffset + NumberOfDynamicMethods * 6);
      Inc (DynamicMethodCounter);
    end;
end;

    Procedure CreateDMT;
    Var DMT: PDMT;
    begin
      If NumberOfDynamicMethods <> 0 then
        begin
           DMT := IncreaseSymbolTable (stTypedConstants, NumberOfDynamicMethods * (SizeOf (Integer) + SizeOf (Pointer)) +
                                                         SizeOf (TDMTHeader));
           With DMT^.Header do
             begin
               Parent_DMT_Offset := 0;
               CachedIndex := 0;
               CachedEntryOffset := 0;
               EntryCount := NumberOfDynamicMethods;
             end;
          CurrentTypedConstantValueOffset := Ofs (DMT^.DynamicMethodIndexTable [0]);
          FillChar (DMT^.DynamicMethodIndexTable [0], NumberOfDynamicMethods * (SizeOf (Integer) + SizeOf (Pointer)), 0);
          SetOffsetToDMT (1, Ofs (DMT^.Header.Parent_DMT_Offset));
          Create_DMT_Entries;
          ObjectType^.W16 := SymbolTable [stTypedConstantsBlocks].UsedSize;
          CreateTypedConstantsBlockRecord;
        end;
    end;
```

### Code Block 8

```pascal
  Procedure CallProcedureForEachMethod (ObjectTypeDefinition: PObjectTypeDefinition; Proc2: TProc2; DynamicMethodIndex: Word);
  Var ObjectIdentifier: PIdentifier;
      ObjectIdentifierOfs: Word absolute ObjectIdentifier;
      VariableIdentifierData: PVariableIdentifierData;
      MethodIdentifierData: PProcedureIdentifierData absolute VariableIdentifierData;
  begin
    DynamicMethodCounter := 0;
    ObjectIdentifier := Pointer (ObjectTypeDefinition);
    ObjectIdentifierOfs := ObjectTypeDefinition^.W0A;
    While ObjectIdentifierOfs <> 0 do
      begin
        VariableIdentifierData := PVariableIdentifierData (PChar (ObjectIdentifier) + ObjectIdentifier^.Name.Len + 4);
        Case TToken (Ord (ObjectIdentifier^.Token) and $7F) of
          Token_VariableIdentifier: ObjectIdentifierOfs := VariableIdentifierData^.W5;
          else begin
                 Proc2 (MethodIdentifierData, DynamicMethodIndex);
                 ObjectIdentifierOfs := MethodIdentifierData^.ProcedureTypeDefinition.W06_;
               end;
        end;
      end;
  end;
```

### Code Block 9

```pascal
  Procedure ProcessSectionOfMembers;
  Var MethodsDeclared: Boolean;

  begin
    MethodsDeclared := False;
    Repeat
      CheckIfDirecive (_PUBLIC, Token_PUBLIC);
      CheckIfDirecive (_PRIVATE, Token_PRIVATE);
      Case Token of
        Token_PROCEDURE,
        Token_FUNCTION: begin
                          ProcessMethodDeclaration (Token);
                          MethodsDeclared := True;
                          Continue;
                        end;
        Token_CONSTRUCTOR,
        Token_DESTRUCTOR: begin
                            ProcessMethodDeclaration (Token);
                            MethodsDeclared := True;
                            Continue;
                            With PObjectTypeDefinition (Ptr (SymbolTable [stMain].Segment,
                                                             CurrentRecordOrObjectTypeDefinitionOffset))^ do
                              If VMT_Size = 0 then VMT_Size := 8;
                          end;
      end;
      If MethodsDeclared then Exit;
      Case Token of
        Token_PUBLIC,
        Token_PRIVATE,
        Token_END: Exit;
      end;
      ProcessCommaSeparatedFieldsAndType;
      CalculateVariableOffsets;
      ExpectTokenAndGetNext (Token_Semicolon);
      MethodsDeclared := False;
    until False;
  end;

    Procedure ProcessMethodDeclaration (MethodToken: TToken);
    Var MethodIdentifierOffset: Word;
        MethodIdentifier: PIdentifier;
        MethodIdentifierData: PProcedureIdentifierData;
        MethodIdentifierToken: TToken;
        MethodType: PProcedureTypeDefinition;
        AncestorMethodIdentifierData: PProcedureIdentifierData;
    begin
      GetNextToken;
      ExpectIdentifier;
      If CurrentIdentifierDeclaredInCurrentScope (MethodIdentifierOffset, Pointer (MethodIdentifierData),
                                                    MethodIdentifierToken) then
        begin
          If MethodIdentifierToken <> Token_ProcedureIdentifier then Error (DuplicateIdentifier);
          If Ptr (Seg (MethodIdentifierData^), MethodIdentifierData^.OuterBlockProcedureIdentifier) =
             Ptr (SymbolTable [stMain].Segment, CurrentRecordOrObjectTypeDefinitionOffset) then Error (DuplicateIdentifier);
          AncestorMethodIdentifierData := MethodIdentifierData;
        end else AncestorMethodIdentifierData := nil;
      MethodIdentifierData := StoreCurrentIdentifierToSymbolTable (CurrentScopeIdentifierTableAddress, 10, MethodIdentifier);
      GetNextToken;
      MethodIdentifier^.Token := TToken (Ord (Token_ProcedureIdentifier) or PrivateFlagMask);
      MethodIdentifierData^.OuterBlockProcedureIdentifier := CurrentRecordOrObjectTypeDefinitionOffset;
      MethodIdentifierData^.LocalIdentifiersList := SymbolTable [stTemporary].UsedSize;
      Word (Ptr (Seg (MethodIdentifierData^), OffsetToNextMemberOffset)^) := Ofs (MethodIdentifier^);
      OffsetToNextMemberOffset := Ofs (MethodIdentifierData^.ProcedureTypeDefinition.W06_);
      CreateProcedureRecord (MethodIdentifier, MethodIdentifierData);
      Case MethodToken of
        Token_CONSTRUCTOR: MethodIdentifierData^.Flags := [pfConstructor, pfMethod, pfFar];
        Token_DESTRUCTOR : MethodIdentifierData^.Flags := [pfDestructor,  pfMethod, pfFar];
        else               MethodIdentifierData^.Flags := [               pfMethod, pfFar];
      end;
      MethodType := ProcessProcedureHeader (MethodToken);
      MethodType^.Size := 8;
      ExpectTokenAndGetNext (Token_Semicolon);
      If (AncestorMethodIdentifierData = nil) or (AncestorMethodIdentifierData^.W8 = 0) then
        CheckIfVirtualMethod else
          CheckOverridenVirtualMethod;
    end;

      Procedure CheckIfVirtualMethod;
      Var ObjectTypeDefinition: PObjectTypeDefinition;
          DynamicMethodIndex: Word;

      begin
        If CompareIdentifierToDirectiveAndGetNextToken (_VIRTUAL) then
          begin
            ObjectTypeDefinition := Ptr (SymbolTable [stMain].Segment, CurrentRecordOrObjectTypeDefinitionOffset);
            With ObjectTypeDefinition^ do If VMT_Size = 0 then VMT_Size := SizeOf (TVMTHeader);
        If pfConstructor in MethodIdentifierData^.Flags then Error (VirtualConstructorsAreNotAllowed);
            Case Token of
              Token_Semicolon: begin
                                 DynamicMethodIndex := ExpectIntegerConstant;
                                 If DynamicMethodIndex = 0 then Error (DuplicateDynamicMethodIndex);
                                 Repeat
                                   CallProcedureForEachMethod (ObjectTypeDefinition,
                                                               CheckIfUniqueDynamicMethodIndex,
                                                               DynamicMethodIndex);
                                   If ObjectTypeDefinition^.AncestorTypeOffset.TypeOffset = 0 then Break;
                                   ObjectTypeDefinition := PointerFromOffsets (ObjectTypeDefinition^.AncestorTypeOffset);
                                 until False;
                                 MethodIdentifierData^.W8 := DynamicMethodIndex;
                                 Include (MethodIdentifierData^.Flags, pfDynamic);
                               end;
              else begin
                     MethodIdentifierData^.W8 := ObjectTypeDefinition^.VMT_Size;
                     Inc (ObjectTypeDefinition^.VMT_Size, SizeOf (Pointer));
                   end;
            end;
            ExpectTokenAndGetNext (Token_Semicolon);
          end else MethodIdentifierData^.W8 := 0;
      end;

      Procedure CheckOverridenVirtualMethod;
      Var DynamicMethodIndex: Word;
      begin
        If CompareIdentifierToDirectiveAndGetNextToken (_VIRTUAL) then
          begin
            If pfDynamic in AncestorMethodIdentifierData^.Flags then
              begin
                If Token = Token_Semicolon then Error (IntegerConstantExpected);
                DynamicMethodIndex := ExpectIntegerConstant;
                If DynamicMethodIndex <> AncestorMethodIdentifierData^.W8 then
                  Error (HeaderDoesNotMatchPreviousDefinition);
                If (AncestorMethodIdentifierData^.Flags * [pfConstructor, pfDestructor]) <>
                   (MethodIdentifierData^.Flags * [pfConstructor, pfDestructor]) then
                     Error (HeaderDoesNotMatchPreviousDefinition);
                If not CheckProcedureTypeCompatibility (@MethodIdentifierData^.ProcedureTypeDefinition,
                                                        @AncestorMethodIdentifierData^.ProcedureTypeDefinition) then
                  Error (HeaderDoesNotMatchPreviousDefinition);
                ExpectTokenAndGetNext (Token_Semicolon);
                MethodIdentifierData^.W8 := DynamicMethodIndex;
                MethodIdentifierData^.Flags :=
                  MethodIdentifierData^.Flags + AncestorMethodIdentifierData^.Flags * [pfDynamic, pf0100];
              end;
          end else Error (VIRTUAL_Expected);
      end;
```

### Code Block 10

```pascal
Procedure CheckIfUniqueDynamicMethodIndex (MethodIdentifierData: PProcedureIdentifierData; Index: Word); Far;
begin
  With MethodIdentifierData^ do
    If pfDynamic in Flags then If Index = W8 then Error (DuplicateDynamicMethodIndex);
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
- https://turbopascal.org/processing-object-types
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

