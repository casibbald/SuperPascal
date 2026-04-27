# Processing Typed Constants - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-typed-constants

---

## Table of Contents

- Processing Typed Constants

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Typed Constants
This procedure processes and stores the value of typed constants. For complex types like
Arrays
,
Records
and
Objects
recursion is used to process elements and fields.
Procedure ProcessTypedConstant (ConstantTypeDef: PTypeDefinition);
Var Expression: TExpression;
    IntermediateCodeSizeBeforeExpression: Word;
    OffsetOf_VMT_Offset, ReferencedBlockRecord, ReferencedSegment: Word;
    Flags: TReferenceFlagSet;
    ReferencedOffset: Word;

    { btArray }
    ArrayTypeDefinition: PArrayTypeDefinition absolute ConstantTypeDef;
    IndexElements: Word;
    StringLen: Byte;
    StrConst: PString;

    SetTypeDefinition: PSetTypeDefinition absolute ConstantTypeDef;
    SetSize, SetDataOffset: Byte;      { btSet }

    { btRecord, btObject }
    RecordTypeDefinition: PObjectTypeDefinition absolute ConstantTypeDef;
    ObjectTypeDefinition: PObjectTypeDefinition absolute ConstantTypeDef;
    CurrentRecordValueOffset: Word;
    FieldDataPtr: PVariableIdentifierData;
    FieldToken: TToken;
    FieldIdOffset: Word;

  Procedure CopyInitialValueToTypedConstant (Source: Pointer; ActualSize: Word);
  begin
    Move (Source^, Ptr (SymbolTable [stTypedConstants].Segment, CurrentTypedConstantValueOffset)^, ActualSize);
    Inc (CurrentTypedConstantValueOffset, ConstantTypeDef^.Size);
  end;

begin
  Case ConstantTypeDef^.BaseType of
    btUntyped,
    btFile,
    btText: Error (FileTypesAreNotAllowedHere);
    btArray: begin
               If Token = Token_LeftParenthesis then
                begin
                  ExpectTokenAndGetNext (Token_LeftParenthesis);
                  With POrdinalTypeDefinition (PointerFromOffsets (ArrayTypeDefinition^.IndexTypeOffset))^ do
                    IndexElements := UpperLimit - LowerLimit + 1;
                  Repeat
                    ProcessTypedConstant (PointerFromOffsets (ArrayTypeDefinition^.ElementTypeOffset));
                    Dec (IndexElements);
                    If IndexElements <> 0 then ExpectTokenAndGetNext (Token_Comma);
                  until IndexElements = 0;
                  ExpectTokenAndGetNext (Token_RightParenthesis);
                end else begin
                           If ExtendedSyntax in ModuleCompilerSwitches then
                             begin
                               If not PArrayTypeDefinition (ConstantTypeDef)^.IsZeroBasedCharacterArray then
                                 If not ArrayTypeDefinition^.IsCharacterArrayCompatibleWithString (StringLen) then
                                   ExpectTokenAndGetNext (Token_LeftParenthesis);
                             end else If not ArrayTypeDefinition^.IsCharacterArrayCompatibleWithString (StringLen) then
                                        ExpectTokenAndGetNext (Token_LeftParenthesis);
                           StrConst := ExpectStringConstant;
                           If Length (StrConst^) > ConstantTypeDef^.Size then Error (StringLengthMismatch);
                           If (Length (StrConst^) < ConstantTypeDef^.Size) and not (ExtendedSyntax in ModuleCompilerSwitches)
                             then Error (StringLengthMismatch);
                           CopyInitialValueToTypedConstant (@StrConst^ [1], Length (StrConst^));
                         end;
             end;
    btRecord,
    btObject: begin
                CurrentRecordValueOffset := CurrentTypedConstantValueOffset;
                OffsetOf_VMT_Offset :=$FFFF;
                If (ConstantTypeDef^.BaseType = btObject) and (ObjectTypeDefinition^.VMT_Size <> 0) then
                  begin
                    AddReferenceRecordForTypedConstant (Seg (ConstantTypeDef^),
                                                        ObjectTypeDefinition^.VMT_TypedConstantsBlockRecordOffset,
                      [rfDataSegment, rfConstant, rfOffset], 0,
                      CurrentTypedConstantValueOffset + ObjectTypeDefinition^.OffsetOf_VMT_Offset);
                    OffsetOf_VMT_Offset := ObjectTypeDefinition^.OffsetOf_VMT_Offset;
                  end;
                ExpectTokenAndGetNext (Token_LeftParenthesis);
                If Token <> Token_RightParenthesis then
                  Repeat
                    If CurrentTypedConstantValueOffset - CurrentRecordValueOffset = OffsetOf_VMT_Offset then
                      Inc (CurrentTypedConstantValueOffset, 2);
                    If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (RecordTypeDefinition, FieldDataPtr,
                                                                  FieldToken, FieldIdOffset) or
                      (FieldToken <> Token_VariableIdentifier) then Error (FieldIdentifierExpected);
                    GetNextToken;
                    If CurrentTypedConstantValueOffset - CurrentRecordValueOffset <> FieldDataPtr^.W1.Ofs then
                      Error (InvalidOrderingOfFields);
                    ExpectTokenAndGetNext (Token_Colon);
                    ProcessTypedConstant (PointerFromOffsets (FieldDataPtr^.UnitTypeOffsets));
                  until not CheckAndGetNextToken (Token_Semicolon);
                ExpectTokenAndGetNext (Token_RightParenthesis);
                CurrentTypedConstantValueOffset := CurrentRecordValueOffset + ConstantTypeDef^.Size;
              end;
    btSet: begin
            Expression.ExpectConstantExpression;
             Expression.CheckTypeCompatibility (ConstantTypeDef);
             SetSize := SetTypeDefinition^.GetSetSizeAndLowestElementDataOffset (SetDataOffset);
             CopyInitialValueToTypedConstant (Ptr (DSeg, Expression.Value.Word + SetDataOffset), SetSize);
           end;
    btString: begin
                StrConst := ExpectStringConstant;
                If Length (StrConst^) > (ConstantTypeDef^.Size - 1) then
                  StrConst^ [0] := Char (ConstantTypeDef^.Size - 1);
                CopyInitialValueToTypedConstant (StrConst, Length (StrConst^) + 1);
              end;
    else begin
           IntermediateCodeSizeBeforeExpression := SymbolTable [stIntermediateCode].UsedSize;
           With Expression do
             begin
               CalculateExpressionWithType (ConstantTypeDef);
               AdjustExpressionToType (ConstantTypeDef);
               CheckTypeCompatibility (ConstantTypeDef);
               CheckRange (ConstantTypeDef);
             end;
           If IntermediateCodeSizeBeforeExpression <> SymbolTable [stIntermediateCode].UsedSize then
             Error (CannotEvaluateThisExpression);
           Case Expression.Location of
             elConstant: begin
                           If ConstantTypeDef^.BaseType = btExtended then
                             ConvertExtendedToOtherFloatingPointTypes (ConstantTypeDef^.DataType, Expression.Value);
                           CopyInitialValueToTypedConstant (@Expression.Value, ConstantTypeDef^.Size);
                         end;
             elPointerToMemory: begin
                                  If Expression.LocationData.Flags * [ofsDI, ofsBP, segES, segSS] <> [] then
                                    Error (CannotEvaluateThisExpression);
                                  ReferencedSegment := Expression.Value.Segment;
                                  ReferencedBlockRecord := Expression.Value.BlockRecord;
                                  ReferencedOffset := Expression.Value.Offset;
                                  If segDS in Expression.LocationData.Flags then
                                    begin
                                      Flags := [rfDataSegment];
                                      If efTypedConstant in Expression.LocationData.Flags then
                                        Flags := [rfDataSegment, rfConstant];
                                    end else begin
                                               Flags := [];
                                               If segCS in Expression.LocationData.Flags then Flags := [rfConstant];
                                             end;
                                  If efSegment in Expression.LocationData.Flags then Include (Flags, rfSegment)
                                    else begin
                                           Include (Flags, rfOffset);
                                           If it32Bit in Expression.DataType then Include (Flags, rfSegment);
                                         end;
                                  AddReferenceRecordForTypedConstant (ReferencedSegment, ReferencedBlockRecord, Flags,
                                                                        ReferencedOffset, CurrentTypedConstantValueOffset);
                                  Inc (CurrentTypedConstantValueOffset, ConstantTypeDef^.Size);
                                end;
             else Error (CannotEvaluateThisExpression);
           end;
         end;
  end;
end;

Procedure AddReferenceRecordForTypedConstant (UnitSegment, BlockRecord: Word; ReferenceFlags: TReferenceFlagSet;
                                              DX, TypedConstantOffset: Word);
Var ReferencedUnitRec: Word;
begin
  ReferencedUnitRec := AddReferencedModule (UnitSegment);
  With PReferencesBlockRecord (IncreaseSymbolTable (stTypedConstantsReferences, SizeOf (TReferencesBlockRecord)))^ do
    begin
      ReferencedUnitRecord := ReferencedUnitRec or Word (ReferenceFlags);
      ReferencedBlockRecordOffset := BlockRecord;
      ReferencedOffset := DX;
      PositionOfReference := TypedConstantOffset - LastTypedConstantsSize;
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
Procedure ProcessTypedConstant (ConstantTypeDef: PTypeDefinition);
Var Expression: TExpression;
    IntermediateCodeSizeBeforeExpression: Word;
    OffsetOf_VMT_Offset, ReferencedBlockRecord, ReferencedSegment: Word;
    Flags: TReferenceFlagSet;
    ReferencedOffset: Word;

    { btArray }
    ArrayTypeDefinition: PArrayTypeDefinition absolute ConstantTypeDef;
    IndexElements: Word;
    StringLen: Byte;
    StrConst: PString;

    SetTypeDefinition: PSetTypeDefinition absolute ConstantTypeDef;
    SetSize, SetDataOffset: Byte;      { btSet }

    { btRecord, btObject }
    RecordTypeDefinition: PObjectTypeDefinition absolute ConstantTypeDef;
    ObjectTypeDefinition: PObjectTypeDefinition absolute ConstantTypeDef;
    CurrentRecordValueOffset: Word;
    FieldDataPtr: PVariableIdentifierData;
    FieldToken: TToken;
    FieldIdOffset: Word;

  Procedure CopyInitialValueToTypedConstant (Source: Pointer; ActualSize: Word);
  begin
    Move (Source^, Ptr (SymbolTable [stTypedConstants].Segment, CurrentTypedConstantValueOffset)^, ActualSize);
    Inc (CurrentTypedConstantValueOffset, ConstantTypeDef^.Size);
  end;

begin
  Case ConstantTypeDef^.BaseType of
    btUntyped,
    btFile,
    btText: Error (FileTypesAreNotAllowedHere);
    btArray: begin
               If Token = Token_LeftParenthesis then
                begin
                  ExpectTokenAndGetNext (Token_LeftParenthesis);
                  With POrdinalTypeDefinition (PointerFromOffsets (ArrayTypeDefinition^.IndexTypeOffset))^ do
                    IndexElements := UpperLimit - LowerLimit + 1;
                  Repeat
                    ProcessTypedConstant (PointerFromOffsets (ArrayTypeDefinition^.ElementTypeOffset));
                    Dec (IndexElements);
                    If IndexElements <> 0 then ExpectTokenAndGetNext (Token_Comma);
                  until IndexElements = 0;
                  ExpectTokenAndGetNext (Token_RightParenthesis);
                end else begin
                           If ExtendedSyntax in ModuleCompilerSwitches then
                             begin
                               If not PArrayTypeDefinition (ConstantTypeDef)^.IsZeroBasedCharacterArray then
                                 If not ArrayTypeDefinition^.IsCharacterArrayCompatibleWithString (StringLen) then
                                   ExpectTokenAndGetNext (Token_LeftParenthesis);
                             end else If not ArrayTypeDefinition^.IsCharacterArrayCompatibleWithString (StringLen) then
                                        ExpectTokenAndGetNext (Token_LeftParenthesis);
                           StrConst := ExpectStringConstant;
                           If Length (StrConst^) > ConstantTypeDef^.Size then Error (StringLengthMismatch);
                           If (Length (StrConst^) < ConstantTypeDef^.Size) and not (ExtendedSyntax in ModuleCompilerSwitches)
                             then Error (StringLengthMismatch);
                           CopyInitialValueToTypedConstant (@StrConst^ [1], Length (StrConst^));
                         end;
             end;
    btRecord,
    btObject: begin
                CurrentRecordValueOffset := CurrentTypedConstantValueOffset;
                OffsetOf_VMT_Offset :=$FFFF;
                If (ConstantTypeDef^.BaseType = btObject) and (ObjectTypeDefinition^.VMT_Size <> 0) then
                  begin
                    AddReferenceRecordForTypedConstant (Seg (ConstantTypeDef^),
                                                        ObjectTypeDefinition^.VMT_TypedConstantsBlockRecordOffset,
                      [rfDataSegment, rfConstant, rfOffset], 0,
                      CurrentTypedConstantValueOffset + ObjectTypeDefinition^.OffsetOf_VMT_Offset);
                    OffsetOf_VMT_Offset := ObjectTypeDefinition^.OffsetOf_VMT_Offset;
                  end;
                ExpectTokenAndGetNext (Token_LeftParenthesis);
                If Token <> Token_RightParenthesis then
                  Repeat
                    If CurrentTypedConstantValueOffset - CurrentRecordValueOffset = OffsetOf_VMT_Offset then
                      Inc (CurrentTypedConstantValueOffset, 2);
                    If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (RecordTypeDefinition, FieldDataPtr,
                                                                  FieldToken, FieldIdOffset) or
                      (FieldToken <> Token_VariableIdentifier) then Error (FieldIdentifierExpected);
                    GetNextToken;
                    If CurrentTypedConstantValueOffset - CurrentRecordValueOffset <> FieldDataPtr^.W1.Ofs then
                      Error (InvalidOrderingOfFields);
                    ExpectTokenAndGetNext (Token_Colon);
                    ProcessTypedConstant (PointerFromOffsets (FieldDataPtr^.UnitTypeOffsets));
                  until not CheckAndGetNextToken (Token_Semicolon);
                ExpectTokenAndGetNext (Token_RightParenthesis);
                CurrentTypedConstantValueOffset := CurrentRecordValueOffset + ConstantTypeDef^.Size;
              end;
    btSet: begin
            Expression.ExpectConstantExpression;
             Expression.CheckTypeCompatibility (ConstantTypeDef);
             SetSize := SetTypeDefinition^.GetSetSizeAndLowestElementDataOffset (SetDataOffset);
             CopyInitialValueToTypedConstant (Ptr (DSeg, Expression.Value.Word + SetDataOffset), SetSize);
           end;
    btString: begin
                StrConst := ExpectStringConstant;
                If Length (StrConst^) > (ConstantTypeDef^.Size - 1) then
                  StrConst^ [0] := Char (ConstantTypeDef^.Size - 1);
                CopyInitialValueToTypedConstant (StrConst, Length (StrConst^) + 1);
              end;
    else begin
           IntermediateCodeSizeBeforeExpression := SymbolTable [stIntermediateCode].UsedSize;
           With Expression do
             begin
               CalculateExpressionWithType (ConstantTypeDef);
               AdjustExpressionToType (ConstantTypeDef);
               CheckTypeCompatibility (ConstantTypeDef);
               CheckRange (ConstantTypeDef);
             end;
           If IntermediateCodeSizeBeforeExpression <> SymbolTable [stIntermediateCode].UsedSize then
             Error (CannotEvaluateThisExpression);
           Case Expression.Location of
             elConstant: begin
                           If ConstantTypeDef^.BaseType = btExtended then
                             ConvertExtendedToOtherFloatingPointTypes (ConstantTypeDef^.DataType, Expression.Value);
                           CopyInitialValueToTypedConstant (@Expression.Value, ConstantTypeDef^.Size);
                         end;
             elPointerToMemory: begin
                                  If Expression.LocationData.Flags * [ofsDI, ofsBP, segES, segSS] <> [] then
                                    Error (CannotEvaluateThisExpression);
                                  ReferencedSegment := Expression.Value.Segment;
                                  ReferencedBlockRecord := Expression.Value.BlockRecord;
                                  ReferencedOffset := Expression.Value.Offset;
                                  If segDS in Expression.LocationData.Flags then
                                    begin
                                      Flags := [rfDataSegment];
                                      If efTypedConstant in Expression.LocationData.Flags then
                                        Flags := [rfDataSegment, rfConstant];
                                    end else begin
                                               Flags := [];
                                               If segCS in Expression.LocationData.Flags then Flags := [rfConstant];
                                             end;
                                  If efSegment in Expression.LocationData.Flags then Include (Flags, rfSegment)
                                    else begin
                                           Include (Flags, rfOffset);
                                           If it32Bit in Expression.DataType then Include (Flags, rfSegment);
                                         end;
                                  AddReferenceRecordForTypedConstant (ReferencedSegment, ReferencedBlockRecord, Flags,
                                                                        ReferencedOffset, CurrentTypedConstantValueOffset);
                                  Inc (CurrentTypedConstantValueOffset, ConstantTypeDef^.Size);
                                end;
             else Error (CannotEvaluateThisExpression);
           end;
         end;
  end;
end;

Procedure AddReferenceRecordForTypedConstant (UnitSegment, BlockRecord: Word; ReferenceFlags: TReferenceFlagSet;
                                              DX, TypedConstantOffset: Word);
Var ReferencedUnitRec: Word;
begin
  ReferencedUnitRec := AddReferencedModule (UnitSegment);
  With PReferencesBlockRecord (IncreaseSymbolTable (stTypedConstantsReferences, SizeOf (TReferencesBlockRecord)))^ do
    begin
      ReferencedUnitRecord := ReferencedUnitRec or Word (ReferenceFlags);
      ReferencedBlockRecordOffset := BlockRecord;
      ReferencedOffset := DX;
      PositionOfReference := TypedConstantOffset - LastTypedConstantsSize;
    end;
end;


```

### Code Block 2

```pascal
Procedure ProcessTypedConstant (ConstantTypeDef: PTypeDefinition);
Var Expression: TExpression;
    IntermediateCodeSizeBeforeExpression: Word;
    OffsetOf_VMT_Offset, ReferencedBlockRecord, ReferencedSegment: Word;
    Flags: TReferenceFlagSet;
    ReferencedOffset: Word;

    { btArray }
    ArrayTypeDefinition: PArrayTypeDefinition absolute ConstantTypeDef;
    IndexElements: Word;
    StringLen: Byte;
    StrConst: PString;

    SetTypeDefinition: PSetTypeDefinition absolute ConstantTypeDef;
    SetSize, SetDataOffset: Byte;      { btSet }

    { btRecord, btObject }
    RecordTypeDefinition: PObjectTypeDefinition absolute ConstantTypeDef;
    ObjectTypeDefinition: PObjectTypeDefinition absolute ConstantTypeDef;
    CurrentRecordValueOffset: Word;
    FieldDataPtr: PVariableIdentifierData;
    FieldToken: TToken;
    FieldIdOffset: Word;

  Procedure CopyInitialValueToTypedConstant (Source: Pointer; ActualSize: Word);
  begin
    Move (Source^, Ptr (SymbolTable [stTypedConstants].Segment, CurrentTypedConstantValueOffset)^, ActualSize);
    Inc (CurrentTypedConstantValueOffset, ConstantTypeDef^.Size);
  end;

begin
  Case ConstantTypeDef^.BaseType of
    btUntyped,
    btFile,
    btText: Error (FileTypesAreNotAllowedHere);
    btArray: begin
               If Token = Token_LeftParenthesis then
                begin
                  ExpectTokenAndGetNext (Token_LeftParenthesis);
                  With POrdinalTypeDefinition (PointerFromOffsets (ArrayTypeDefinition^.IndexTypeOffset))^ do
                    IndexElements := UpperLimit - LowerLimit + 1;
                  Repeat
                    ProcessTypedConstant (PointerFromOffsets (ArrayTypeDefinition^.ElementTypeOffset));
                    Dec (IndexElements);
                    If IndexElements <> 0 then ExpectTokenAndGetNext (Token_Comma);
                  until IndexElements = 0;
                  ExpectTokenAndGetNext (Token_RightParenthesis);
                end else begin
                           If ExtendedSyntax in ModuleCompilerSwitches then
                             begin
                               If not PArrayTypeDefinition (ConstantTypeDef)^.IsZeroBasedCharacterArray then
                                 If not ArrayTypeDefinition^.IsCharacterArrayCompatibleWithString (StringLen) then
                                   ExpectTokenAndGetNext (Token_LeftParenthesis);
                             end else If not ArrayTypeDefinition^.IsCharacterArrayCompatibleWithString (StringLen) then
                                        ExpectTokenAndGetNext (Token_LeftParenthesis);
                           StrConst := ExpectStringConstant;
                           If Length (StrConst^) > ConstantTypeDef^.Size then Error (StringLengthMismatch);
                           If (Length (StrConst^) < ConstantTypeDef^.Size) and not (ExtendedSyntax in ModuleCompilerSwitches)
                             then Error (StringLengthMismatch);
                           CopyInitialValueToTypedConstant (@StrConst^ [1], Length (StrConst^));
                         end;
             end;
    btRecord,
    btObject: begin
                CurrentRecordValueOffset := CurrentTypedConstantValueOffset;
                OffsetOf_VMT_Offset :=$FFFF;
                If (ConstantTypeDef^.BaseType = btObject) and (ObjectTypeDefinition^.VMT_Size <> 0) then
                  begin
                    AddReferenceRecordForTypedConstant (Seg (ConstantTypeDef^),
                                                        ObjectTypeDefinition^.VMT_TypedConstantsBlockRecordOffset,
                      [rfDataSegment, rfConstant, rfOffset], 0,
                      CurrentTypedConstantValueOffset + ObjectTypeDefinition^.OffsetOf_VMT_Offset);
                    OffsetOf_VMT_Offset := ObjectTypeDefinition^.OffsetOf_VMT_Offset;
                  end;
                ExpectTokenAndGetNext (Token_LeftParenthesis);
                If Token <> Token_RightParenthesis then
                  Repeat
                    If CurrentTypedConstantValueOffset - CurrentRecordValueOffset = OffsetOf_VMT_Offset then
                      Inc (CurrentTypedConstantValueOffset, 2);
                    If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (RecordTypeDefinition, FieldDataPtr,
                                                                  FieldToken, FieldIdOffset) or
                      (FieldToken <> Token_VariableIdentifier) then Error (FieldIdentifierExpected);
                    GetNextToken;
                    If CurrentTypedConstantValueOffset - CurrentRecordValueOffset <> FieldDataPtr^.W1.Ofs then
                      Error (InvalidOrderingOfFields);
                    ExpectTokenAndGetNext (Token_Colon);
                    ProcessTypedConstant (PointerFromOffsets (FieldDataPtr^.UnitTypeOffsets));
                  until not CheckAndGetNextToken (Token_Semicolon);
                ExpectTokenAndGetNext (Token_RightParenthesis);
                CurrentTypedConstantValueOffset := CurrentRecordValueOffset + ConstantTypeDef^.Size;
              end;
    btSet: begin
            Expression.ExpectConstantExpression;
             Expression.CheckTypeCompatibility (ConstantTypeDef);
             SetSize := SetTypeDefinition^.GetSetSizeAndLowestElementDataOffset (SetDataOffset);
             CopyInitialValueToTypedConstant (Ptr (DSeg, Expression.Value.Word + SetDataOffset), SetSize);
           end;
    btString: begin
                StrConst := ExpectStringConstant;
                If Length (StrConst^) > (ConstantTypeDef^.Size - 1) then
                  StrConst^ [0] := Char (ConstantTypeDef^.Size - 1);
                CopyInitialValueToTypedConstant (StrConst, Length (StrConst^) + 1);
              end;
    else begin
           IntermediateCodeSizeBeforeExpression := SymbolTable [stIntermediateCode].UsedSize;
           With Expression do
             begin
               CalculateExpressionWithType (ConstantTypeDef);
               AdjustExpressionToType (ConstantTypeDef);
               CheckTypeCompatibility (ConstantTypeDef);
               CheckRange (ConstantTypeDef);
             end;
           If IntermediateCodeSizeBeforeExpression <> SymbolTable [stIntermediateCode].UsedSize then
             Error (CannotEvaluateThisExpression);
           Case Expression.Location of
             elConstant: begin
                           If ConstantTypeDef^.BaseType = btExtended then
                             ConvertExtendedToOtherFloatingPointTypes (ConstantTypeDef^.DataType, Expression.Value);
                           CopyInitialValueToTypedConstant (@Expression.Value, ConstantTypeDef^.Size);
                         end;
             elPointerToMemory: begin
                                  If Expression.LocationData.Flags * [ofsDI, ofsBP, segES, segSS] <> [] then
                                    Error (CannotEvaluateThisExpression);
                                  ReferencedSegment := Expression.Value.Segment;
                                  ReferencedBlockRecord := Expression.Value.BlockRecord;
                                  ReferencedOffset := Expression.Value.Offset;
                                  If segDS in Expression.LocationData.Flags then
                                    begin
                                      Flags := [rfDataSegment];
                                      If efTypedConstant in Expression.LocationData.Flags then
                                        Flags := [rfDataSegment, rfConstant];
                                    end else begin
                                               Flags := [];
                                               If segCS in Expression.LocationData.Flags then Flags := [rfConstant];
                                             end;
                                  If efSegment in Expression.LocationData.Flags then Include (Flags, rfSegment)
                                    else begin
                                           Include (Flags, rfOffset);
                                           If it32Bit in Expression.DataType then Include (Flags, rfSegment);
                                         end;
                                  AddReferenceRecordForTypedConstant (ReferencedSegment, ReferencedBlockRecord, Flags,
                                                                        ReferencedOffset, CurrentTypedConstantValueOffset);
                                  Inc (CurrentTypedConstantValueOffset, ConstantTypeDef^.Size);
                                end;
             else Error (CannotEvaluateThisExpression);
           end;
         end;
  end;
end;

Procedure AddReferenceRecordForTypedConstant (UnitSegment, BlockRecord: Word; ReferenceFlags: TReferenceFlagSet;
                                              DX, TypedConstantOffset: Word);
Var ReferencedUnitRec: Word;
begin
  ReferencedUnitRec := AddReferencedModule (UnitSegment);
  With PReferencesBlockRecord (IncreaseSymbolTable (stTypedConstantsReferences, SizeOf (TReferencesBlockRecord)))^ do
    begin
      ReferencedUnitRecord := ReferencedUnitRec or Word (ReferenceFlags);
      ReferencedBlockRecordOffset := BlockRecord;
      ReferencedOffset := DX;
      PositionOfReference := TypedConstantOffset - LastTypedConstantsSize;
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
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-typed-constants
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

