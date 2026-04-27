# Processing Non-object Types - Turbo Pascal Compiler

**URL:** https://turbopascal.org/non-object-types

---

## Table of Contents

- Processing Non-object Types

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Non-object Types
This procedure processes all types except
Objects
and creates
type definition records
. One one hand objects need different and more complex processing and on the other hand they can not be declared anywhere else as in the program, interface or implementation part. This procedure is used also to process types in
variable declarations
, ordinal types and it is also used recursively for
Array
and
File
elements. Multi-dimensional arrays are created as arrays of simple arrays as Pascal language defines.
Function ProcessNonObjectTypeDeclaration: PTypeDefinition;

  Function ProcessPointerType: PTypeDefinition;
  Var PointerTypeDefinition: PPointerTypeDefinition;
      TemporaryTableRecordOffset: Word;
      PointerBaseTypeUnitTypeOffsets: TUnitOffsets;

    Procedure CreatePointerToSystemType (SystemTypeOffset: Word);
    begin
      GetTypeAndUnitIdentifierOffsets (Ptr (SystemUnitSegment, SystemTypeOffset), PointerBaseTypeUnitTypeOffsets);
      PointerTypeDefinition := CreateTypeDefinition (SizeOf (TPointerTypeDefinition), 4, itLongInt, btPointer);
      PointerTypeDefinition^.PointerBaseTypeOffset := PointerBaseTypeUnitTypeOffsets;
      GetNextToken;
    end;

  begin
    GetNextToken;
    Case Token of
      Token_STRING: CreatePointerToSystemType (String_TypeOffset);
      Token_FILE: CreatePointerToSystemType (File_TypeOffset);
      else begin
            TemporaryTableRecordOffset := SymbolTable [stTemporary].NextRecordOffset;
            StoreNextIdentifierToTemporaryBlock;
            PointerTypeDefinition :=
              CreateTypeDefinition (SizeOf (TPointerTypeDefinition), 4, itLongInt, btPointer);
            With PointerTypeDefinition^.PointerBaseTypeOffset do
              begin
                PreviousPointerToTypeDefinitionOffset := LastPointerToTypeDefinitionOffset;
                TemporarySymbolTableRecordOffset := TemporaryTableRecordOffset;
              end;
            LastPointerToTypeDefinitionOffset := Ofs (PointerTypeDefinition^);
          end;
    end;
    ProcessPointerType := Pointer (PointerTypeDefinition);
  end;

  Function ProcessArrayType: PTypeDefinition;
  Var IndexCounter: Word;
      TypeDef: PTypeDefinition;
      ArrayTypeDefinition: PArrayTypeDefinition;
      ArrayElementUnitTypeOffsets,
      ArrayIndexUnitTypeOffsets: TUnitOffsets;
      ArraySize: LongRec;
      TypeDefinition: PTypeDefinition;
  begin
    GetNextToken;
    ExpectTokenAndGetNext (Token_LeftBracket);
    IndexCounter := 0;
    Repeat
      TypeDef := ExpectOrdinalType;
      Asm
        PUSH  WORD PTR TypeDef + 2
        PUSH  WORD PTR TypeDef
      end;
      Inc (IndexCounter);
    until not CheckAndGetNextToken (Token_Comma);
    ExpectTokenAndGetNext (Token_RightBracket);
    ExpectTokenAndGetNext (Token_OF);
    TypeDef := ProcessNonObjectTypeDeclaration;         { Base array element type }
    Repeat
      GetTypeAndUnitIdentifierOffsets (TypeDef, ArrayElementUnitTypeOffsets);{ Array element type }
      ArraySize.Long := TypeDef^.Size;                  { Array element size }
      Asm
        POP   WORD PTR TypeDef                          { Array index TyepDef }
        POP   WORD PTR TypeDef + 2
      end;
      With POrdinalTypeDefinition (TypeDef)^ do
        begin
          If UpperLimit - LowerLimit = $FFFF then Error (StructureTooLarge);
          ArraySize.Long := ArraySize.Long * (UpperLimit - LowerLimit + 1);
        end;
      If ArraySize.WordH <> 0 then Error (StructureTooLarge);
      GetTypeAndUnitIdentifierOffsets (TypeDef, ArrayIndexUnitTypeOffsets);
      ArrayTypeDefinition := CreateTypeDefinition (SizeOf (TArrayTypeDefinition), ArraySize.WordL, [], btArray);
      With ArrayTypeDefinition^ do
        begin
          IndexTypeOffset := ArrayIndexUnitTypeOffsets;
          ElementTypeOffset := ArrayElementUnitTypeOffsets
        end;
      TypeDef := ArrayTypeDefinition;
      Dec (IndexCounter);
    until IndexCounter = 0;
    ProcessArrayType := ArrayTypeDefinition;
  end;

  Function ProcessRecordType: PTypeDefinition;
  Var RecordTypeDefinition: PRecordTypeDefinition;
      SavedLastPointerToTypeDefinitionOffset: Word;
      Saved_OffsetToNextMemberOffset: Word;
      SavedFirstDeclaredVariableIdentifierOffset: Word;
      SavedNumberOfDeclaredVariableIdentifiers: Word;

    Function ProcessRecordFieldsBetweenTokens (StartToken, EndToken: TToken): PTypeDefinition;
    Var TempToken: TToken;
        DataOfs,
        RecordSizeBeforeCase,
        LargestRecordSize: Word;
        IdDataPtr: Pointer;
        RecordTypePtr: PTypeDefinition;
        DummyTypeDef: PTypeDefinition;
        Expression: TExpression;
    begin
      ExpectTokenAndGetNext (StartToken);
      Repeat
        Case Token of
          Token_Identifier:
            begin
              ProcessCommaSeparatedFieldsAndType;
              CalculateVariableOffsets;
              If CheckAndGetNextToken (EndToken) then Exit;
              ExpectTokenAndGetNext (Token_Semicolon);
            end;
          Token_CASE:
            begin
              GetNextToken;
              ExpectIdentifier;
              If (not FindCurrentIdentifier (TempToken, DataOfs, IdDataPtr)) or (TempToken <> Token_TypeIdentifier) then
                  begin
                    ProcessCommaSeparatedFieldsAndType;
                    CalculateVariableOffsets;
                  end else GetNextToken;
              ExpectTokenAndGetNext (Token_OF);
              RecordTypePtr := Ptr (SymbolTable [stMain].Segment, CurrentRecordOrObjectTypeDefinitionOffset);
              RecordSizeBeforeCase := RecordTypePtr^.Size;
              Repeat
                LargestRecordSize := RecordTypePtr^.Size;
                RecordTypePtr^.Size := RecordSizeBeforeCase;
                Repeat
                  Expression.ExpectConstantExpression;
                Until not CheckAndGetNextToken (Token_Comma);
                ExpectTokenAndGetNext (Token_Colon);
                DummyTypeDef := ProcessRecordFieldsBetweenTokens (Token_LeftParenthesis, Token_RightParenthesis);
                If RecordTypePtr^.Size < LargestRecordSize then RecordTypePtr^.Size := LargestRecordSize;
                If CheckAndGetNextToken (EndToken) then Exit;
                ExpectTokenAndGetNext (Token_Semicolon);
                If CheckAndGetNextToken (EndToken) then Exit;
              until False;
            end;
          else begin
                 ExpectTokenAndGetNext (EndToken);
                 Exit;
               end;
        end;
      until False;
    end;

  begin
    SavedLastPointerToTypeDefinitionOffset := LastPointerToTypeDefinitionOffset;
    Saved_OffsetToNextMemberOffset := OffsetToNextMemberOffset;
    SavedFirstDeclaredVariableIdentifierOffset := FirstDeclaredVariableIdentifierOffset;
    SavedNumberOfDeclaredVariableIdentifiers := NumberOfDeclaredVariableIdentifiers;
    RecordTypeDefinition := CreateTypeDefinition (SizeOf (TRecordTypeDefinition), 0, [], btRecord);
    CurrentRecordOrObjectTypeDefinitionOffset := Ofs (RecordTypeDefinition^);
    RecordTypeDefinition^.FieldsListOffset := SymbolTable [stMain].NextRecordOffset;
    RecordTypeDefinition^.W0A := 0;
    OffsetToNextMemberOffset := Ofs (RecordTypeDefinition^.W0A);
    CreateSymbolTable (4);
    ProcessRecordFieldsBetweenTokens (Token_RECORD, Token_END);
    CurrentRecordOrObjectTypeDefinitionOffset := 0;
    NumberOfDeclaredVariableIdentifiers := SavedNumberOfDeclaredVariableIdentifiers;
    FirstDeclaredVariableIdentifierOffset := SavedFirstDeclaredVariableIdentifierOffset;
    OffsetToNextMemberOffset := Saved_OffsetToNextMemberOffset;
    LastPointerToTypeDefinitionOffset := SavedLastPointerToTypeDefinitionOffset;
    ProcessRecordType := RecordTypeDefinition;
  end;

  Function ProcessStringType: PTypeDefinition;
  Var StringLength: LongRec;
      CharUnitTypeOffsets, OrdinalUnitTypeOffsets: TUnitOffsets;
      LongIntUnitTypeOffsets: TUnitOffsets absolute OrdinalUnitTypeOffsets;
      OrdinalTypeDefinition: POrdinalTypeDefinition;
      StringTypeDefinition: PStringTypeDefinition;
  begin
    GetNextToken;
    If not CheckAndGetNextToken (Token_LeftBracket) then
      begin
        ProcessStringType := Ptr (SystemUnitSegment, String_TypeOffset);
        Exit;
      end;
    StringLength.Long := ExpectIntegerConstant;
    If StringLength.WordH <> 0 then Error (InvalidStringLength);
    If StringLength.Byte1 <> 0 then Error (InvalidStringLength);
    If StringLength.Byte0 = 0 then Error (InvalidStringLength);
    GetTypeAndUnitIdentifierOffsets (Ptr (SystemUnitSegment, LongInt_TypeOffset), LongIntUnitTypeOffsets);
    OrdinalTypeDefinition := CreateTypeDefinition (SizeOf (TOrdinalTypeDefinition), 1, [itUnsigned], btInteger);
    With OrdinalTypeDefinition^ do
      begin
        OrdinalType := LongIntUnitTypeOffsets;
        LowerLimit := 0;
        UpperLimit := StringLength.Byte0;
      end;
    GetTypeAndUnitIdentifierOffsets (Pointer (OrdinalTypeDefinition), OrdinalUnitTypeOffsets);
    GetTypeAndUnitIdentifierOffsets (Ptr (SystemUnitSegment, Char_TypeOffset), CharUnitTypeOffsets);
    StringTypeDefinition := CreateTypeDefinition (SizeOf (TStringTypeDefinition), StringLength.Byte0 + 1, [], btString);
    With StringTypeDefinition^ do
      begin
        ElementTypeOffset := CharUnitTypeOffsets;
        IndexTypeOffset := OrdinalUnitTypeOffsets;
      end;
    ProcessStringType := StringTypeDefinition;
    ExpectTokenAndGetNext (Token_RightBracket);
  end;

  Function ProcessFileType: PTypeDefinition;
  Var FileElementType: PTypeDefinition;
      FileTypeDefinition: PFileTypeDefinition;
      FileElementTypeUnitTypeOffsets: TUnitOffsets;
  begin
    GetNextToken;
    If not CheckAndGetNextToken (Token_OF) then
      begin
        ProcessFileType := Ptr (SystemUnitSegment, File_TypeOffset);
        Exit;
      end;
    FileElementType := ProcessNonObjectTypeDeclaration;
    Case FileElementType^.BaseType of
      btObject,
      btFile,
      btText: Error (FileComponentsMayNotBeFilesOrObjects);
    end;
    GetTypeAndUnitIdentifierOffsets (FileElementType, FileElementTypeUnitTypeOffsets);
    FileTypeDefinition := CreateTypeDefinition (SizeOf (TFileTypeDefinition), $80, [], btFile);
    FileTypeDefinition^.BaseFileTypeOffset := FileElementTypeUnitTypeOffsets;
    ProcessFileType := Pointer (FileTypeDefinition);
  end;

  Function ProcessSetType: Pointer;
  Var SetBaseType: POrdinalTypeDefinition;
      SetTypeDefinition: PSetTypeDefinition;
      SetBaseTypeUnitTypeOffsets: TUnitOffsets;
      SetSize: Byte;
      IntType: TIntegerTypeSet;
  begin
    GetNextToken;
    ExpectTokenAndGetNext (Token_OF);
    SetBaseType := Pointer (ExpectOrdinalType);
    With SetBaseType^ do
      begin
        If (LowerLimit or UpperLimit) and $FF00 <> 0 then Error (SetBaseTypeOutOfRange);
        SetSize := (UpperLimit - LowerLimit) shr 3 + 1;
        IntType := [it32Bytes];
        If LowerLimit = 0 then
          Case SetSize of
            1: IntType := [itUnsigned];
            2: IntType := [itUnsigned, it16Bit];
          end;
      end;
    GetTypeAndUnitIdentifierOffsets (Pointer (SetBaseType), SetBaseTypeUnitTypeOffsets);
    SetTypeDefinition := CreateTypeDefinition (SizeOf (TSetTypeDefinition), SetSize, IntType, btSet);
    SetTypeDefinition^.BaseSetTypeOffset := SetBaseTypeUnitTypeOffsets;
    ProcessSetType := Pointer (SetTypeDefinition);
  end;

  Function ProcessEnumeratedType: PTypeDefinition;
  Var EnumerationType: POrdinalTypeDefinition;
      SetTypeDefinition: PSetTypeDefinition;
      EnumerationIdData: PConstantIdentifierData;
      EnumerationId: PIdentifier;
      Counter: Word;
      EnumerationUnitTypeOffsets: TUnitOffsets;
  begin
    GetNextToken;
    EnumerationType := CreateTypeDefinition (SizeOf (TOrdinalTypeDefinition), 0, [], btEnumeration);
    GetTypeAndUnitIdentifierOffsets (Pointer (EnumerationType), EnumerationUnitTypeOffsets);
    EnumerationType^.OrdinalType := EnumerationUnitTypeOffsets;
    SetTypeDefinition := CreateTypeDefinition (SizeOf (TSetTypeDefinition), 32, [it32Bytes], btSet);
    SetTypeDefinition^.BaseSetTypeOffset := EnumerationUnitTypeOffsets;
    Counter := 0;
    Repeat
      EnumerationId := ExpectAndStoreIdentifier (8, Pointer (EnumerationIdData));
      EnumerationId^.Token := Token_ConstantIdentifier;
      With EnumerationIdData^ do
        begin
          UnitTypeOffsets := EnumerationUnitTypeOffsets;
          OrdinalValue := Counter;
          W2 := 0;
        end;
      Inc (Counter);
    until not CheckAndGetNextToken (Token_Comma);
    Dec (Counter);
    ExpectTokenAndGetNext (Token_RightParenthesis);
    With EnumerationType^ do
      begin
        LowestIntegerType (Counter, DataType);
        Size := 1;
        If it16Bit in DataType then Size := 2;
        LowerLimit := 0;
        UpperLimit := Counter;
      end;
    SetTypeDefinition^.W06_ := 0;
    ProcessEnumeratedType := Pointer (EnumerationType);
  end;

  Function ProcessFunctionProcedureType: PTypeDefinition;
  Var SavedTemporaryBlockSize: Word;
      ProcToken: TToken;
  begin
    ProcToken := Token;
    GetNextToken;
    SavedTemporaryBlockSize := SymbolTable [stTemporary].UsedSize;
    ProcessFunctionProcedureType := ProcessProcedureHeader (ProcToken);
    SymbolTable [stTemporary].UsedSize := SavedTemporaryBlockSize;
  end;

  Function ProcessSubrangeType: PTypeDefinition;
  Var LowerLimitExpression, UpperLimitExpression: TExpression;
      CommonOrdinalType: TIntegerTypeSet;
      UnitTypeOffsets: TUnitOffsets;
      Size: Byte;
      SubRangeType: POrdinalTypeDefinition;
      LowerLimitLowestIntegerType, UpperLimitLowestIntegerType: TIntegerTypeSet;
  begin
   LowerLimitExpression.ExpectConstantExpression;
    If LowerLimitExpression.TypeDefPtr^.BaseType < btInteger then Error (InvalidSubrangeBaseType);
    ExpectTokenAndGetNext (Token_PeriodPeriod);
   UpperLimitExpression.ExpectConstantExpression;
    If LowerLimitExpression.TypeDefPtr <> UpperLimitExpression.TypeDefPtr then Error (TypeMismatch);
    If LowerLimitExpression.Value.LongInt > UpperLimitExpression.Value.LongInt then
      Error (LowerBoundGreaterThanUpperBound);
    LowestIntegerType (LowerLimitExpression.Value.LongInt, LowerLimitLowestIntegerType);
    LowestIntegerType (UpperLimitExpression.Value.LongInt, UpperLimitLowestIntegerType);
    SetLowestCommonIntegerType (LowerLimitLowestIntegerType, UpperLimitLowestIntegerType, CommonOrdinalType);
    Size := 1;
    If it16Bit in CommonOrdinalType then
      begin
        Size := 2;
        If it32Bit in CommonOrdinalType then Size := 4;
      end;
    GetTypeAndUnitIdentifierOffsets (LowerLimitExpression.TypeDefPtr, UnitTypeOffsets);
    SubRangeType := CreateTypeDefinition (20, Size, CommonOrdinalType, LowerLimitExpression.TypeDefPtr^.BaseType);
    With SubRangeType^ do
      begin
        LowerLimit := LowerLimitExpression.Value.LongInt;
        UpperLimit := UpperLimitExpression.Value.LongInt;
        OrdinalType := UnitTypeOffsets;
      end;
    ProcessSubrangeType := Pointer (SubRangeType);
  end;

begin
  CheckAndGetNextToken (Token_PACKED);
  CheckForDeclaredIdentifier;
  Case Token of
    Token_TypeIdentifier:     ProcessNonObjectTypeDeclaration := ProcessTypeIdentifier;
    Token_ARRAY:              ProcessNonObjectTypeDeclaration := ProcessArrayType;
    Token_RECORD:             ProcessNonObjectTypeDeclaration := ProcessRecordType;
    Token_Caret:              ProcessNonObjectTypeDeclaration := ProcessPointerType;
    Token_STRING:             ProcessNonObjectTypeDeclaration := ProcessStringType;
    Token_FILE:               ProcessNonObjectTypeDeclaration := ProcessFileType;
    Token_SET:                ProcessNonObjectTypeDeclaration := ProcessSetType;
    Token_LeftParenthesis:    ProcessNonObjectTypeDeclaration := ProcessEnumeratedType;
    Token_PROCEDURE:          ProcessNonObjectTypeDeclaration := ProcessFunctionProcedureType;
    Token_FUNCTION:           ProcessNonObjectTypeDeclaration := ProcessFunctionProcedureType;
    Token_Constant:           ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_ConstantIdentifier: ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_Minus:              ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_Plus:               ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_SystemFunction:     ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_NOT:                ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    else Error (ErrorInType);
  end;
end;

Function ExpectOrdinalType: PTypeDefinition;
Var TypeDef: PTypeDefinition;
begin
  CheckForStringConstantWithControlCharacter;
  TypeDef := ProcessNonObjectTypeDeclaration;
  If TypeDef^.BaseType < btInteger then Error (OrdinalTypeExpected);
  If TypeDef^.Size > 2 then Error (OrdinalTypeExpected);
  ExpectOrdinalType := TypeDef;
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
Function ProcessNonObjectTypeDeclaration: PTypeDefinition;

  Function ProcessPointerType: PTypeDefinition;
  Var PointerTypeDefinition: PPointerTypeDefinition;
      TemporaryTableRecordOffset: Word;
      PointerBaseTypeUnitTypeOffsets: TUnitOffsets;

    Procedure CreatePointerToSystemType (SystemTypeOffset: Word);
    begin
      GetTypeAndUnitIdentifierOffsets (Ptr (SystemUnitSegment, SystemTypeOffset), PointerBaseTypeUnitTypeOffsets);
      PointerTypeDefinition := CreateTypeDefinition (SizeOf (TPointerTypeDefinition), 4, itLongInt, btPointer);
      PointerTypeDefinition^.PointerBaseTypeOffset := PointerBaseTypeUnitTypeOffsets;
      GetNextToken;
    end;

  begin
    GetNextToken;
    Case Token of
      Token_STRING: CreatePointerToSystemType (String_TypeOffset);
      Token_FILE: CreatePointerToSystemType (File_TypeOffset);
      else begin
            TemporaryTableRecordOffset := SymbolTable [stTemporary].NextRecordOffset;
            StoreNextIdentifierToTemporaryBlock;
            PointerTypeDefinition :=
              CreateTypeDefinition (SizeOf (TPointerTypeDefinition), 4, itLongInt, btPointer);
            With PointerTypeDefinition^.PointerBaseTypeOffset do
              begin
                PreviousPointerToTypeDefinitionOffset := LastPointerToTypeDefinitionOffset;
                TemporarySymbolTableRecordOffset := TemporaryTableRecordOffset;
              end;
            LastPointerToTypeDefinitionOffset := Ofs (PointerTypeDefinition^);
          end;
    end;
    ProcessPointerType := Pointer (PointerTypeDefinition);
  end;

  Function ProcessArrayType: PTypeDefinition;
  Var IndexCounter: Word;
      TypeDef: PTypeDefinition;
      ArrayTypeDefinition: PArrayTypeDefinition;
      ArrayElementUnitTypeOffsets,
      ArrayIndexUnitTypeOffsets: TUnitOffsets;
      ArraySize: LongRec;
      TypeDefinition: PTypeDefinition;
  begin
    GetNextToken;
    ExpectTokenAndGetNext (Token_LeftBracket);
    IndexCounter := 0;
    Repeat
      TypeDef := ExpectOrdinalType;
      Asm
        PUSH  WORD PTR TypeDef + 2
        PUSH  WORD PTR TypeDef
      end;
      Inc (IndexCounter);
    until not CheckAndGetNextToken (Token_Comma);
    ExpectTokenAndGetNext (Token_RightBracket);
    ExpectTokenAndGetNext (Token_OF);
    TypeDef := ProcessNonObjectTypeDeclaration;         { Base array element type }
    Repeat
      GetTypeAndUnitIdentifierOffsets (TypeDef, ArrayElementUnitTypeOffsets);{ Array element type }
      ArraySize.Long := TypeDef^.Size;                  { Array element size }
      Asm
        POP   WORD PTR TypeDef                          { Array index TyepDef }
        POP   WORD PTR TypeDef + 2
      end;
      With POrdinalTypeDefinition (TypeDef)^ do
        begin
          If UpperLimit - LowerLimit = $FFFF then Error (StructureTooLarge);
          ArraySize.Long := ArraySize.Long * (UpperLimit - LowerLimit + 1);
        end;
      If ArraySize.WordH <> 0 then Error (StructureTooLarge);
      GetTypeAndUnitIdentifierOffsets (TypeDef, ArrayIndexUnitTypeOffsets);
      ArrayTypeDefinition := CreateTypeDefinition (SizeOf (TArrayTypeDefinition), ArraySize.WordL, [], btArray);
      With ArrayTypeDefinition^ do
        begin
          IndexTypeOffset := ArrayIndexUnitTypeOffsets;
          ElementTypeOffset := ArrayElementUnitTypeOffsets
        end;
      TypeDef := ArrayTypeDefinition;
      Dec (IndexCounter);
    until IndexCounter = 0;
    ProcessArrayType := ArrayTypeDefinition;
  end;

  Function ProcessRecordType: PTypeDefinition;
  Var RecordTypeDefinition: PRecordTypeDefinition;
      SavedLastPointerToTypeDefinitionOffset: Word;
      Saved_OffsetToNextMemberOffset: Word;
      SavedFirstDeclaredVariableIdentifierOffset: Word;
      SavedNumberOfDeclaredVariableIdentifiers: Word;

    Function ProcessRecordFieldsBetweenTokens (StartToken, EndToken: TToken): PTypeDefinition;
    Var TempToken: TToken;
        DataOfs,
        RecordSizeBeforeCase,
        LargestRecordSize: Word;
        IdDataPtr: Pointer;
        RecordTypePtr: PTypeDefinition;
        DummyTypeDef: PTypeDefinition;
        Expression: TExpression;
    begin
      ExpectTokenAndGetNext (StartToken);
      Repeat
        Case Token of
          Token_Identifier:
            begin
              ProcessCommaSeparatedFieldsAndType;
              CalculateVariableOffsets;
              If CheckAndGetNextToken (EndToken) then Exit;
              ExpectTokenAndGetNext (Token_Semicolon);
            end;
          Token_CASE:
            begin
              GetNextToken;
              ExpectIdentifier;
              If (not FindCurrentIdentifier (TempToken, DataOfs, IdDataPtr)) or (TempToken <> Token_TypeIdentifier) then
                  begin
                    ProcessCommaSeparatedFieldsAndType;
                    CalculateVariableOffsets;
                  end else GetNextToken;
              ExpectTokenAndGetNext (Token_OF);
              RecordTypePtr := Ptr (SymbolTable [stMain].Segment, CurrentRecordOrObjectTypeDefinitionOffset);
              RecordSizeBeforeCase := RecordTypePtr^.Size;
              Repeat
                LargestRecordSize := RecordTypePtr^.Size;
                RecordTypePtr^.Size := RecordSizeBeforeCase;
                Repeat
                  Expression.ExpectConstantExpression;
                Until not CheckAndGetNextToken (Token_Comma);
                ExpectTokenAndGetNext (Token_Colon);
                DummyTypeDef := ProcessRecordFieldsBetweenTokens (Token_LeftParenthesis, Token_RightParenthesis);
                If RecordTypePtr^.Size < LargestRecordSize then RecordTypePtr^.Size := LargestRecordSize;
                If CheckAndGetNextToken (EndToken) then Exit;
                ExpectTokenAndGetNext (Token_Semicolon);
                If CheckAndGetNextToken (EndToken) then Exit;
              until False;
            end;
          else begin
                 ExpectTokenAndGetNext (EndToken);
                 Exit;
               end;
        end;
      until False;
    end;

  begin
    SavedLastPointerToTypeDefinitionOffset := LastPointerToTypeDefinitionOffset;
    Saved_OffsetToNextMemberOffset := OffsetToNextMemberOffset;
    SavedFirstDeclaredVariableIdentifierOffset := FirstDeclaredVariableIdentifierOffset;
    SavedNumberOfDeclaredVariableIdentifiers := NumberOfDeclaredVariableIdentifiers;
    RecordTypeDefinition := CreateTypeDefinition (SizeOf (TRecordTypeDefinition), 0, [], btRecord);
    CurrentRecordOrObjectTypeDefinitionOffset := Ofs (RecordTypeDefinition^);
    RecordTypeDefinition^.FieldsListOffset := SymbolTable [stMain].NextRecordOffset;
    RecordTypeDefinition^.W0A := 0;
    OffsetToNextMemberOffset := Ofs (RecordTypeDefinition^.W0A);
    CreateSymbolTable (4);
    ProcessRecordFieldsBetweenTokens (Token_RECORD, Token_END);
    CurrentRecordOrObjectTypeDefinitionOffset := 0;
    NumberOfDeclaredVariableIdentifiers := SavedNumberOfDeclaredVariableIdentifiers;
    FirstDeclaredVariableIdentifierOffset := SavedFirstDeclaredVariableIdentifierOffset;
    OffsetToNextMemberOffset := Saved_OffsetToNextMemberOffset;
    LastPointerToTypeDefinitionOffset := SavedLastPointerToTypeDefinitionOffset;
    ProcessRecordType := RecordTypeDefinition;
  end;

  Function ProcessStringType: PTypeDefinition;
  Var StringLength: LongRec;
      CharUnitTypeOffsets, OrdinalUnitTypeOffsets: TUnitOffsets;
      LongIntUnitTypeOffsets: TUnitOffsets absolute OrdinalUnitTypeOffsets;
      OrdinalTypeDefinition: POrdinalTypeDefinition;
      StringTypeDefinition: PStringTypeDefinition;
  begin
    GetNextToken;
    If not CheckAndGetNextToken (Token_LeftBracket) then
      begin
        ProcessStringType := Ptr (SystemUnitSegment, String_TypeOffset);
        Exit;
      end;
    StringLength.Long := ExpectIntegerConstant;
    If StringLength.WordH <> 0 then Error (InvalidStringLength);
    If StringLength.Byte1 <> 0 then Error (InvalidStringLength);
    If StringLength.Byte0 = 0 then Error (InvalidStringLength);
    GetTypeAndUnitIdentifierOffsets (Ptr (SystemUnitSegment, LongInt_TypeOffset), LongIntUnitTypeOffsets);
    OrdinalTypeDefinition := CreateTypeDefinition (SizeOf (TOrdinalTypeDefinition), 1, [itUnsigned], btInteger);
    With OrdinalTypeDefinition^ do
      begin
        OrdinalType := LongIntUnitTypeOffsets;
        LowerLimit := 0;
        UpperLimit := StringLength.Byte0;
      end;
    GetTypeAndUnitIdentifierOffsets (Pointer (OrdinalTypeDefinition), OrdinalUnitTypeOffsets);
    GetTypeAndUnitIdentifierOffsets (Ptr (SystemUnitSegment, Char_TypeOffset), CharUnitTypeOffsets);
    StringTypeDefinition := CreateTypeDefinition (SizeOf (TStringTypeDefinition), StringLength.Byte0 + 1, [], btString);
    With StringTypeDefinition^ do
      begin
        ElementTypeOffset := CharUnitTypeOffsets;
        IndexTypeOffset := OrdinalUnitTypeOffsets;
      end;
    ProcessStringType := StringTypeDefinition;
    ExpectTokenAndGetNext (Token_RightBracket);
  end;

  Function ProcessFileType: PTypeDefinition;
  Var FileElementType: PTypeDefinition;
      FileTypeDefinition: PFileTypeDefinition;
      FileElementTypeUnitTypeOffsets: TUnitOffsets;
  begin
    GetNextToken;
    If not CheckAndGetNextToken (Token_OF) then
      begin
        ProcessFileType := Ptr (SystemUnitSegment, File_TypeOffset);
        Exit;
      end;
    FileElementType := ProcessNonObjectTypeDeclaration;
    Case FileElementType^.BaseType of
      btObject,
      btFile,
      btText: Error (FileComponentsMayNotBeFilesOrObjects);
    end;
    GetTypeAndUnitIdentifierOffsets (FileElementType, FileElementTypeUnitTypeOffsets);
    FileTypeDefinition := CreateTypeDefinition (SizeOf (TFileTypeDefinition), $80, [], btFile);
    FileTypeDefinition^.BaseFileTypeOffset := FileElementTypeUnitTypeOffsets;
    ProcessFileType := Pointer (FileTypeDefinition);
  end;

  Function ProcessSetType: Pointer;
  Var SetBaseType: POrdinalTypeDefinition;
      SetTypeDefinition: PSetTypeDefinition;
      SetBaseTypeUnitTypeOffsets: TUnitOffsets;
      SetSize: Byte;
      IntType: TIntegerTypeSet;
  begin
    GetNextToken;
    ExpectTokenAndGetNext (Token_OF);
    SetBaseType := Pointer (ExpectOrdinalType);
    With SetBaseType^ do
      begin
        If (LowerLimit or UpperLimit) and $FF00 <> 0 then Error (SetBaseTypeOutOfRange);
        SetSize := (UpperLimit - LowerLimit) shr 3 + 1;
        IntType := [it32Bytes];
        If LowerLimit = 0 then
          Case SetSize of
            1: IntType := [itUnsigned];
            2: IntType := [itUnsigned, it16Bit];
          end;
      end;
    GetTypeAndUnitIdentifierOffsets (Pointer (SetBaseType), SetBaseTypeUnitTypeOffsets);
    SetTypeDefinition := CreateTypeDefinition (SizeOf (TSetTypeDefinition), SetSize, IntType, btSet);
    SetTypeDefinition^.BaseSetTypeOffset := SetBaseTypeUnitTypeOffsets;
    ProcessSetType := Pointer (SetTypeDefinition);
  end;

  Function ProcessEnumeratedType: PTypeDefinition;
  Var EnumerationType: POrdinalTypeDefinition;
      SetTypeDefinition: PSetTypeDefinition;
      EnumerationIdData: PConstantIdentifierData;
      EnumerationId: PIdentifier;
      Counter: Word;
      EnumerationUnitTypeOffsets: TUnitOffsets;
  begin
    GetNextToken;
    EnumerationType := CreateTypeDefinition (SizeOf (TOrdinalTypeDefinition), 0, [], btEnumeration);
    GetTypeAndUnitIdentifierOffsets (Pointer (EnumerationType), EnumerationUnitTypeOffsets);
    EnumerationType^.OrdinalType := EnumerationUnitTypeOffsets;
    SetTypeDefinition := CreateTypeDefinition (SizeOf (TSetTypeDefinition), 32, [it32Bytes], btSet);
    SetTypeDefinition^.BaseSetTypeOffset := EnumerationUnitTypeOffsets;
    Counter := 0;
    Repeat
      EnumerationId := ExpectAndStoreIdentifier (8, Pointer (EnumerationIdData));
      EnumerationId^.Token := Token_ConstantIdentifier;
      With EnumerationIdData^ do
        begin
          UnitTypeOffsets := EnumerationUnitTypeOffsets;
          OrdinalValue := Counter;
          W2 := 0;
        end;
      Inc (Counter);
    until not CheckAndGetNextToken (Token_Comma);
    Dec (Counter);
    ExpectTokenAndGetNext (Token_RightParenthesis);
    With EnumerationType^ do
      begin
        LowestIntegerType (Counter, DataType);
        Size := 1;
        If it16Bit in DataType then Size := 2;
        LowerLimit := 0;
        UpperLimit := Counter;
      end;
    SetTypeDefinition^.W06_ := 0;
    ProcessEnumeratedType := Pointer (EnumerationType);
  end;

  Function ProcessFunctionProcedureType: PTypeDefinition;
  Var SavedTemporaryBlockSize: Word;
      ProcToken: TToken;
  begin
    ProcToken := Token;
    GetNextToken;
    SavedTemporaryBlockSize := SymbolTable [stTemporary].UsedSize;
    ProcessFunctionProcedureType := ProcessProcedureHeader (ProcToken);
    SymbolTable [stTemporary].UsedSize := SavedTemporaryBlockSize;
  end;

  Function ProcessSubrangeType: PTypeDefinition;
  Var LowerLimitExpression, UpperLimitExpression: TExpression;
      CommonOrdinalType: TIntegerTypeSet;
      UnitTypeOffsets: TUnitOffsets;
      Size: Byte;
      SubRangeType: POrdinalTypeDefinition;
      LowerLimitLowestIntegerType, UpperLimitLowestIntegerType: TIntegerTypeSet;
  begin
   LowerLimitExpression.ExpectConstantExpression;
    If LowerLimitExpression.TypeDefPtr^.BaseType < btInteger then Error (InvalidSubrangeBaseType);
    ExpectTokenAndGetNext (Token_PeriodPeriod);
   UpperLimitExpression.ExpectConstantExpression;
    If LowerLimitExpression.TypeDefPtr <> UpperLimitExpression.TypeDefPtr then Error (TypeMismatch);
    If LowerLimitExpression.Value.LongInt > UpperLimitExpression.Value.LongInt then
      Error (LowerBoundGreaterThanUpperBound);
    LowestIntegerType (LowerLimitExpression.Value.LongInt, LowerLimitLowestIntegerType);
    LowestIntegerType (UpperLimitExpression.Value.LongInt, UpperLimitLowestIntegerType);
    SetLowestCommonIntegerType (LowerLimitLowestIntegerType, UpperLimitLowestIntegerType, CommonOrdinalType);
    Size := 1;
    If it16Bit in CommonOrdinalType then
      begin
        Size := 2;
        If it32Bit in CommonOrdinalType then Size := 4;
      end;
    GetTypeAndUnitIdentifierOffsets (LowerLimitExpression.TypeDefPtr, UnitTypeOffsets);
    SubRangeType := CreateTypeDefinition (20, Size, CommonOrdinalType, LowerLimitExpression.TypeDefPtr^.BaseType);
    With SubRangeType^ do
      begin
        LowerLimit := LowerLimitExpression.Value.LongInt;
        UpperLimit := UpperLimitExpression.Value.LongInt;
        OrdinalType := UnitTypeOffsets;
      end;
    ProcessSubrangeType := Pointer (SubRangeType);
  end;

begin
  CheckAndGetNextToken (Token_PACKED);
  CheckForDeclaredIdentifier;
  Case Token of
    Token_TypeIdentifier:     ProcessNonObjectTypeDeclaration := ProcessTypeIdentifier;
    Token_ARRAY:              ProcessNonObjectTypeDeclaration := ProcessArrayType;
    Token_RECORD:             ProcessNonObjectTypeDeclaration := ProcessRecordType;
    Token_Caret:              ProcessNonObjectTypeDeclaration := ProcessPointerType;
    Token_STRING:             ProcessNonObjectTypeDeclaration := ProcessStringType;
    Token_FILE:               ProcessNonObjectTypeDeclaration := ProcessFileType;
    Token_SET:                ProcessNonObjectTypeDeclaration := ProcessSetType;
    Token_LeftParenthesis:    ProcessNonObjectTypeDeclaration := ProcessEnumeratedType;
    Token_PROCEDURE:          ProcessNonObjectTypeDeclaration := ProcessFunctionProcedureType;
    Token_FUNCTION:           ProcessNonObjectTypeDeclaration := ProcessFunctionProcedureType;
    Token_Constant:           ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_ConstantIdentifier: ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_Minus:              ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_Plus:               ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_SystemFunction:     ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_NOT:                ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    else Error (ErrorInType);
  end;
end;

Function ExpectOrdinalType: PTypeDefinition;
Var TypeDef: PTypeDefinition;
begin
  CheckForStringConstantWithControlCharacter;
  TypeDef := ProcessNonObjectTypeDeclaration;
  If TypeDef^.BaseType < btInteger then Error (OrdinalTypeExpected);
  If TypeDef^.Size > 2 then Error (OrdinalTypeExpected);
  ExpectOrdinalType := TypeDef;
end;
```

### Code Block 2

```pascal
Function ProcessNonObjectTypeDeclaration: PTypeDefinition;

  Function ProcessPointerType: PTypeDefinition;
  Var PointerTypeDefinition: PPointerTypeDefinition;
      TemporaryTableRecordOffset: Word;
      PointerBaseTypeUnitTypeOffsets: TUnitOffsets;

    Procedure CreatePointerToSystemType (SystemTypeOffset: Word);
    begin
      GetTypeAndUnitIdentifierOffsets (Ptr (SystemUnitSegment, SystemTypeOffset), PointerBaseTypeUnitTypeOffsets);
      PointerTypeDefinition := CreateTypeDefinition (SizeOf (TPointerTypeDefinition), 4, itLongInt, btPointer);
      PointerTypeDefinition^.PointerBaseTypeOffset := PointerBaseTypeUnitTypeOffsets;
      GetNextToken;
    end;

  begin
    GetNextToken;
    Case Token of
      Token_STRING: CreatePointerToSystemType (String_TypeOffset);
      Token_FILE: CreatePointerToSystemType (File_TypeOffset);
      else begin
            TemporaryTableRecordOffset := SymbolTable [stTemporary].NextRecordOffset;
            StoreNextIdentifierToTemporaryBlock;
            PointerTypeDefinition :=
              CreateTypeDefinition (SizeOf (TPointerTypeDefinition), 4, itLongInt, btPointer);
            With PointerTypeDefinition^.PointerBaseTypeOffset do
              begin
                PreviousPointerToTypeDefinitionOffset := LastPointerToTypeDefinitionOffset;
                TemporarySymbolTableRecordOffset := TemporaryTableRecordOffset;
              end;
            LastPointerToTypeDefinitionOffset := Ofs (PointerTypeDefinition^);
          end;
    end;
    ProcessPointerType := Pointer (PointerTypeDefinition);
  end;

  Function ProcessArrayType: PTypeDefinition;
  Var IndexCounter: Word;
      TypeDef: PTypeDefinition;
      ArrayTypeDefinition: PArrayTypeDefinition;
      ArrayElementUnitTypeOffsets,
      ArrayIndexUnitTypeOffsets: TUnitOffsets;
      ArraySize: LongRec;
      TypeDefinition: PTypeDefinition;
  begin
    GetNextToken;
    ExpectTokenAndGetNext (Token_LeftBracket);
    IndexCounter := 0;
    Repeat
      TypeDef := ExpectOrdinalType;
      Asm
        PUSH  WORD PTR TypeDef + 2
        PUSH  WORD PTR TypeDef
      end;
      Inc (IndexCounter);
    until not CheckAndGetNextToken (Token_Comma);
    ExpectTokenAndGetNext (Token_RightBracket);
    ExpectTokenAndGetNext (Token_OF);
    TypeDef := ProcessNonObjectTypeDeclaration;         { Base array element type }
    Repeat
      GetTypeAndUnitIdentifierOffsets (TypeDef, ArrayElementUnitTypeOffsets);{ Array element type }
      ArraySize.Long := TypeDef^.Size;                  { Array element size }
      Asm
        POP   WORD PTR TypeDef                          { Array index TyepDef }
        POP   WORD PTR TypeDef + 2
      end;
      With POrdinalTypeDefinition (TypeDef)^ do
        begin
          If UpperLimit - LowerLimit = $FFFF then Error (StructureTooLarge);
          ArraySize.Long := ArraySize.Long * (UpperLimit - LowerLimit + 1);
        end;
      If ArraySize.WordH <> 0 then Error (StructureTooLarge);
      GetTypeAndUnitIdentifierOffsets (TypeDef, ArrayIndexUnitTypeOffsets);
      ArrayTypeDefinition := CreateTypeDefinition (SizeOf (TArrayTypeDefinition), ArraySize.WordL, [], btArray);
      With ArrayTypeDefinition^ do
        begin
          IndexTypeOffset := ArrayIndexUnitTypeOffsets;
          ElementTypeOffset := ArrayElementUnitTypeOffsets
        end;
      TypeDef := ArrayTypeDefinition;
      Dec (IndexCounter);
    until IndexCounter = 0;
    ProcessArrayType := ArrayTypeDefinition;
  end;

  Function ProcessRecordType: PTypeDefinition;
  Var RecordTypeDefinition: PRecordTypeDefinition;
      SavedLastPointerToTypeDefinitionOffset: Word;
      Saved_OffsetToNextMemberOffset: Word;
      SavedFirstDeclaredVariableIdentifierOffset: Word;
      SavedNumberOfDeclaredVariableIdentifiers: Word;

    Function ProcessRecordFieldsBetweenTokens (StartToken, EndToken: TToken): PTypeDefinition;
    Var TempToken: TToken;
        DataOfs,
        RecordSizeBeforeCase,
        LargestRecordSize: Word;
        IdDataPtr: Pointer;
        RecordTypePtr: PTypeDefinition;
        DummyTypeDef: PTypeDefinition;
        Expression: TExpression;
    begin
      ExpectTokenAndGetNext (StartToken);
      Repeat
        Case Token of
          Token_Identifier:
            begin
              ProcessCommaSeparatedFieldsAndType;
              CalculateVariableOffsets;
              If CheckAndGetNextToken (EndToken) then Exit;
              ExpectTokenAndGetNext (Token_Semicolon);
            end;
          Token_CASE:
            begin
              GetNextToken;
              ExpectIdentifier;
              If (not FindCurrentIdentifier (TempToken, DataOfs, IdDataPtr)) or (TempToken <> Token_TypeIdentifier) then
                  begin
                    ProcessCommaSeparatedFieldsAndType;
                    CalculateVariableOffsets;
                  end else GetNextToken;
              ExpectTokenAndGetNext (Token_OF);
              RecordTypePtr := Ptr (SymbolTable [stMain].Segment, CurrentRecordOrObjectTypeDefinitionOffset);
              RecordSizeBeforeCase := RecordTypePtr^.Size;
              Repeat
                LargestRecordSize := RecordTypePtr^.Size;
                RecordTypePtr^.Size := RecordSizeBeforeCase;
                Repeat
                  Expression.ExpectConstantExpression;
                Until not CheckAndGetNextToken (Token_Comma);
                ExpectTokenAndGetNext (Token_Colon);
                DummyTypeDef := ProcessRecordFieldsBetweenTokens (Token_LeftParenthesis, Token_RightParenthesis);
                If RecordTypePtr^.Size < LargestRecordSize then RecordTypePtr^.Size := LargestRecordSize;
                If CheckAndGetNextToken (EndToken) then Exit;
                ExpectTokenAndGetNext (Token_Semicolon);
                If CheckAndGetNextToken (EndToken) then Exit;
              until False;
            end;
          else begin
                 ExpectTokenAndGetNext (EndToken);
                 Exit;
               end;
        end;
      until False;
    end;

  begin
    SavedLastPointerToTypeDefinitionOffset := LastPointerToTypeDefinitionOffset;
    Saved_OffsetToNextMemberOffset := OffsetToNextMemberOffset;
    SavedFirstDeclaredVariableIdentifierOffset := FirstDeclaredVariableIdentifierOffset;
    SavedNumberOfDeclaredVariableIdentifiers := NumberOfDeclaredVariableIdentifiers;
    RecordTypeDefinition := CreateTypeDefinition (SizeOf (TRecordTypeDefinition), 0, [], btRecord);
    CurrentRecordOrObjectTypeDefinitionOffset := Ofs (RecordTypeDefinition^);
    RecordTypeDefinition^.FieldsListOffset := SymbolTable [stMain].NextRecordOffset;
    RecordTypeDefinition^.W0A := 0;
    OffsetToNextMemberOffset := Ofs (RecordTypeDefinition^.W0A);
    CreateSymbolTable (4);
    ProcessRecordFieldsBetweenTokens (Token_RECORD, Token_END);
    CurrentRecordOrObjectTypeDefinitionOffset := 0;
    NumberOfDeclaredVariableIdentifiers := SavedNumberOfDeclaredVariableIdentifiers;
    FirstDeclaredVariableIdentifierOffset := SavedFirstDeclaredVariableIdentifierOffset;
    OffsetToNextMemberOffset := Saved_OffsetToNextMemberOffset;
    LastPointerToTypeDefinitionOffset := SavedLastPointerToTypeDefinitionOffset;
    ProcessRecordType := RecordTypeDefinition;
  end;

  Function ProcessStringType: PTypeDefinition;
  Var StringLength: LongRec;
      CharUnitTypeOffsets, OrdinalUnitTypeOffsets: TUnitOffsets;
      LongIntUnitTypeOffsets: TUnitOffsets absolute OrdinalUnitTypeOffsets;
      OrdinalTypeDefinition: POrdinalTypeDefinition;
      StringTypeDefinition: PStringTypeDefinition;
  begin
    GetNextToken;
    If not CheckAndGetNextToken (Token_LeftBracket) then
      begin
        ProcessStringType := Ptr (SystemUnitSegment, String_TypeOffset);
        Exit;
      end;
    StringLength.Long := ExpectIntegerConstant;
    If StringLength.WordH <> 0 then Error (InvalidStringLength);
    If StringLength.Byte1 <> 0 then Error (InvalidStringLength);
    If StringLength.Byte0 = 0 then Error (InvalidStringLength);
    GetTypeAndUnitIdentifierOffsets (Ptr (SystemUnitSegment, LongInt_TypeOffset), LongIntUnitTypeOffsets);
    OrdinalTypeDefinition := CreateTypeDefinition (SizeOf (TOrdinalTypeDefinition), 1, [itUnsigned], btInteger);
    With OrdinalTypeDefinition^ do
      begin
        OrdinalType := LongIntUnitTypeOffsets;
        LowerLimit := 0;
        UpperLimit := StringLength.Byte0;
      end;
    GetTypeAndUnitIdentifierOffsets (Pointer (OrdinalTypeDefinition), OrdinalUnitTypeOffsets);
    GetTypeAndUnitIdentifierOffsets (Ptr (SystemUnitSegment, Char_TypeOffset), CharUnitTypeOffsets);
    StringTypeDefinition := CreateTypeDefinition (SizeOf (TStringTypeDefinition), StringLength.Byte0 + 1, [], btString);
    With StringTypeDefinition^ do
      begin
        ElementTypeOffset := CharUnitTypeOffsets;
        IndexTypeOffset := OrdinalUnitTypeOffsets;
      end;
    ProcessStringType := StringTypeDefinition;
    ExpectTokenAndGetNext (Token_RightBracket);
  end;

  Function ProcessFileType: PTypeDefinition;
  Var FileElementType: PTypeDefinition;
      FileTypeDefinition: PFileTypeDefinition;
      FileElementTypeUnitTypeOffsets: TUnitOffsets;
  begin
    GetNextToken;
    If not CheckAndGetNextToken (Token_OF) then
      begin
        ProcessFileType := Ptr (SystemUnitSegment, File_TypeOffset);
        Exit;
      end;
    FileElementType := ProcessNonObjectTypeDeclaration;
    Case FileElementType^.BaseType of
      btObject,
      btFile,
      btText: Error (FileComponentsMayNotBeFilesOrObjects);
    end;
    GetTypeAndUnitIdentifierOffsets (FileElementType, FileElementTypeUnitTypeOffsets);
    FileTypeDefinition := CreateTypeDefinition (SizeOf (TFileTypeDefinition), $80, [], btFile);
    FileTypeDefinition^.BaseFileTypeOffset := FileElementTypeUnitTypeOffsets;
    ProcessFileType := Pointer (FileTypeDefinition);
  end;

  Function ProcessSetType: Pointer;
  Var SetBaseType: POrdinalTypeDefinition;
      SetTypeDefinition: PSetTypeDefinition;
      SetBaseTypeUnitTypeOffsets: TUnitOffsets;
      SetSize: Byte;
      IntType: TIntegerTypeSet;
  begin
    GetNextToken;
    ExpectTokenAndGetNext (Token_OF);
    SetBaseType := Pointer (ExpectOrdinalType);
    With SetBaseType^ do
      begin
        If (LowerLimit or UpperLimit) and $FF00 <> 0 then Error (SetBaseTypeOutOfRange);
        SetSize := (UpperLimit - LowerLimit) shr 3 + 1;
        IntType := [it32Bytes];
        If LowerLimit = 0 then
          Case SetSize of
            1: IntType := [itUnsigned];
            2: IntType := [itUnsigned, it16Bit];
          end;
      end;
    GetTypeAndUnitIdentifierOffsets (Pointer (SetBaseType), SetBaseTypeUnitTypeOffsets);
    SetTypeDefinition := CreateTypeDefinition (SizeOf (TSetTypeDefinition), SetSize, IntType, btSet);
    SetTypeDefinition^.BaseSetTypeOffset := SetBaseTypeUnitTypeOffsets;
    ProcessSetType := Pointer (SetTypeDefinition);
  end;

  Function ProcessEnumeratedType: PTypeDefinition;
  Var EnumerationType: POrdinalTypeDefinition;
      SetTypeDefinition: PSetTypeDefinition;
      EnumerationIdData: PConstantIdentifierData;
      EnumerationId: PIdentifier;
      Counter: Word;
      EnumerationUnitTypeOffsets: TUnitOffsets;
  begin
    GetNextToken;
    EnumerationType := CreateTypeDefinition (SizeOf (TOrdinalTypeDefinition), 0, [], btEnumeration);
    GetTypeAndUnitIdentifierOffsets (Pointer (EnumerationType), EnumerationUnitTypeOffsets);
    EnumerationType^.OrdinalType := EnumerationUnitTypeOffsets;
    SetTypeDefinition := CreateTypeDefinition (SizeOf (TSetTypeDefinition), 32, [it32Bytes], btSet);
    SetTypeDefinition^.BaseSetTypeOffset := EnumerationUnitTypeOffsets;
    Counter := 0;
    Repeat
      EnumerationId := ExpectAndStoreIdentifier (8, Pointer (EnumerationIdData));
      EnumerationId^.Token := Token_ConstantIdentifier;
      With EnumerationIdData^ do
        begin
          UnitTypeOffsets := EnumerationUnitTypeOffsets;
          OrdinalValue := Counter;
          W2 := 0;
        end;
      Inc (Counter);
    until not CheckAndGetNextToken (Token_Comma);
    Dec (Counter);
    ExpectTokenAndGetNext (Token_RightParenthesis);
    With EnumerationType^ do
      begin
        LowestIntegerType (Counter, DataType);
        Size := 1;
        If it16Bit in DataType then Size := 2;
        LowerLimit := 0;
        UpperLimit := Counter;
      end;
    SetTypeDefinition^.W06_ := 0;
    ProcessEnumeratedType := Pointer (EnumerationType);
  end;

  Function ProcessFunctionProcedureType: PTypeDefinition;
  Var SavedTemporaryBlockSize: Word;
      ProcToken: TToken;
  begin
    ProcToken := Token;
    GetNextToken;
    SavedTemporaryBlockSize := SymbolTable [stTemporary].UsedSize;
    ProcessFunctionProcedureType := ProcessProcedureHeader (ProcToken);
    SymbolTable [stTemporary].UsedSize := SavedTemporaryBlockSize;
  end;

  Function ProcessSubrangeType: PTypeDefinition;
  Var LowerLimitExpression, UpperLimitExpression: TExpression;
      CommonOrdinalType: TIntegerTypeSet;
      UnitTypeOffsets: TUnitOffsets;
      Size: Byte;
      SubRangeType: POrdinalTypeDefinition;
      LowerLimitLowestIntegerType, UpperLimitLowestIntegerType: TIntegerTypeSet;
  begin
   LowerLimitExpression.ExpectConstantExpression;
    If LowerLimitExpression.TypeDefPtr^.BaseType < btInteger then Error (InvalidSubrangeBaseType);
    ExpectTokenAndGetNext (Token_PeriodPeriod);
   UpperLimitExpression.ExpectConstantExpression;
    If LowerLimitExpression.TypeDefPtr <> UpperLimitExpression.TypeDefPtr then Error (TypeMismatch);
    If LowerLimitExpression.Value.LongInt > UpperLimitExpression.Value.LongInt then
      Error (LowerBoundGreaterThanUpperBound);
    LowestIntegerType (LowerLimitExpression.Value.LongInt, LowerLimitLowestIntegerType);
    LowestIntegerType (UpperLimitExpression.Value.LongInt, UpperLimitLowestIntegerType);
    SetLowestCommonIntegerType (LowerLimitLowestIntegerType, UpperLimitLowestIntegerType, CommonOrdinalType);
    Size := 1;
    If it16Bit in CommonOrdinalType then
      begin
        Size := 2;
        If it32Bit in CommonOrdinalType then Size := 4;
      end;
    GetTypeAndUnitIdentifierOffsets (LowerLimitExpression.TypeDefPtr, UnitTypeOffsets);
    SubRangeType := CreateTypeDefinition (20, Size, CommonOrdinalType, LowerLimitExpression.TypeDefPtr^.BaseType);
    With SubRangeType^ do
      begin
        LowerLimit := LowerLimitExpression.Value.LongInt;
        UpperLimit := UpperLimitExpression.Value.LongInt;
        OrdinalType := UnitTypeOffsets;
      end;
    ProcessSubrangeType := Pointer (SubRangeType);
  end;

begin
  CheckAndGetNextToken (Token_PACKED);
  CheckForDeclaredIdentifier;
  Case Token of
    Token_TypeIdentifier:     ProcessNonObjectTypeDeclaration := ProcessTypeIdentifier;
    Token_ARRAY:              ProcessNonObjectTypeDeclaration := ProcessArrayType;
    Token_RECORD:             ProcessNonObjectTypeDeclaration := ProcessRecordType;
    Token_Caret:              ProcessNonObjectTypeDeclaration := ProcessPointerType;
    Token_STRING:             ProcessNonObjectTypeDeclaration := ProcessStringType;
    Token_FILE:               ProcessNonObjectTypeDeclaration := ProcessFileType;
    Token_SET:                ProcessNonObjectTypeDeclaration := ProcessSetType;
    Token_LeftParenthesis:    ProcessNonObjectTypeDeclaration := ProcessEnumeratedType;
    Token_PROCEDURE:          ProcessNonObjectTypeDeclaration := ProcessFunctionProcedureType;
    Token_FUNCTION:           ProcessNonObjectTypeDeclaration := ProcessFunctionProcedureType;
    Token_Constant:           ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_ConstantIdentifier: ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_Minus:              ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_Plus:               ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_SystemFunction:     ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    Token_NOT:                ProcessNonObjectTypeDeclaration := ProcessSubrangeType;
    else Error (ErrorInType);
  end;
end;

Function ExpectOrdinalType: PTypeDefinition;
Var TypeDef: PTypeDefinition;
begin
  CheckForStringConstantWithControlCharacter;
  TypeDef := ProcessNonObjectTypeDeclaration;
  If TypeDef^.BaseType < btInteger then Error (OrdinalTypeExpected);
  If TypeDef^.Size > 2 then Error (OrdinalTypeExpected);
  ExpectOrdinalType := TypeDef;
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
- https://turbopascal.org/non-object-types
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-object-types
- https://turbopascal.org/processing-variable-declarations
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
- https://turbopascal.org/type-definition-data-structures
- https://turbopascal.org/type-definitions

