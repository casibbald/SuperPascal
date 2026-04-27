# Processing Set Constants - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-set-constants

---

## Table of Contents

- Processing Set Constants

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Set Constants
This procedure processes set constants. This can be an empty set, or a collection of set elements and ranges. If the elements are not constant the code is generated to include them into the set. Individual elements as well as range limits are treated as
expressions
.
Procedure ProcessSetConstant;
  Var TempSet: Set of Byte;
      LowerLimitExpression, UpperLimitExpression: TExpression;
      Element, LowerLimit, UpperLimit: Byte;
      NonConstantElementCode: TIntermediateCodeOffsets;
      TypePtr: PtrRec;
      N: Byte;
      DataOffset: Word;

    Procedure ExpectSetElement (Var Expression: TExpression);
    begin
      Expression.ExpectOrdinalExpression;
      If TypePtr.Ptr = nil then
        begin
          TypePtr.Ofs := POrdinalTypeDefinition (Expression.TypeDefPtr)^.OrdinalType.TypeOffset + 20;
          TypePtr.Seg := PUnitIdentifierData (Ptr (Seg (Expression.TypeDefPtr^),
                             POrdinalTypeDefinition (Expression.TypeDefPtr)^.OrdinalType.UnitIdentifierData))^.UnitSegment;
        end;
      If (TypePtr.Seg <> PUnitIdentifierData (Ptr (Seg (Expression.TypeDefPtr^),
                          POrdinalTypeDefinition (Expression.TypeDefPtr)^.OrdinalType.UnitIdentifierData))^.UnitSegment) or
       (TypePtr.Ofs <> POrdinalTypeDefinition (Expression.TypeDefPtr)^.OrdinalType.TypeOffset + 20) then Error (TypeMismatch);
      Expression.CheckOrdinalRange (Ptr (SystemUnitSegment, Byte_TypeOffset));
    end;

  begin
    GetNextToken;
    TempSet := [];
    TypePtr.Ptr := nil;
    NonConstantElementCode.NumberOfStatements := 0;
    If not CheckAndGetNextToken (Token_RightBracket) then
      begin
        Repeat
          ExpectSetElement (LowerLimitExpression);
          If CheckAndGetNextToken (Token_PeriodPeriod) then
            begin                                                       { Range }
              ExpectSetElement (UpperLimitExpression);
              If (LowerLimitExpression.Location = elConstant) and (UpperLimitExpression.Location = elConstant) then
                begin
                  LowerLimit := LowerLimitExpression.Value.Byte;
                  UpperLimit := UpperLimitExpression.Value.Byte;
                end else begin
                           LowerLimitExpression.Calculate;
                           LowerLimitExpression.Push;
                           UpperLimitExpression.Calculate;
                           UpperLimitExpression.Push;
                           GenerateInstruction_CALL_FAR (SysProc_ZAddRange);
                           AddCallToIntermediateCodeSubroutine (NonConstantElementCode, EndSubroutine);
                           Continue;
                         end;
            end else begin                                              { Single set element }
                       If LowerLimitExpression.Location = elConstant then
                         begin
                           LowerLimit := LowerLimitExpression.Value.Byte;
                           UpperLimit := LowerLimit;
                         end else begin
                                   LowerLimitExpression.Calculate;
                                   LowerLimitExpression.Push;
                                   GenerateInstruction_CALL_FAR (SysProc_ZAddComp);
                                   AddCallToIntermediateCodeSubroutine (NonConstantElementCode, EndSubroutine);
                                   Continue;
                                  end;
                     end;
          If LowerLimit <= UpperLimit then
            For Element := LowerLimit to UpperLimit do Include (TempSet, Element);
        until not CheckAndGetNextToken (Token_Comma);
        ExpectTokenAndGetNext (TOKEN_RightBracket);
      end else begin                                    { Empty set }
                 TypePtr.Ofs := EmptySet_TypeOffset;
                 TypePtr.Seg := SystemUnitSegment;
               end;
    DataOffset := GetTempBufferOffset (32);
    Move (TempSet, Ptr (DSeg, DataOffset)^, 32);
    TypeDefPtr := TypePtr.Ptr;
    IntermediateCodeOffset := 0;
    Location := elConstant;
    UsedRegisters := [];
    Value.Word := DataOffset;
    SetDataTypeOfSet;
    If NonConstantElementCode.NumberOfStatements <> 0 then
      begin
        ExpandSetToStackFrameAndPushPointer (0);
        Calculate;
        Generate_icGoSub_ForEachSubroutine (NonConstantElementCode);
        EndIntermediateCodeSubroutine;
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
  Procedure ProcessSetConstant;
  Var TempSet: Set of Byte;
      LowerLimitExpression, UpperLimitExpression: TExpression;
      Element, LowerLimit, UpperLimit: Byte;
      NonConstantElementCode: TIntermediateCodeOffsets;
      TypePtr: PtrRec;
      N: Byte;
      DataOffset: Word;

    Procedure ExpectSetElement (Var Expression: TExpression);
    begin
      Expression.ExpectOrdinalExpression;
      If TypePtr.Ptr = nil then
        begin
          TypePtr.Ofs := POrdinalTypeDefinition (Expression.TypeDefPtr)^.OrdinalType.TypeOffset + 20;
          TypePtr.Seg := PUnitIdentifierData (Ptr (Seg (Expression.TypeDefPtr^),
                             POrdinalTypeDefinition (Expression.TypeDefPtr)^.OrdinalType.UnitIdentifierData))^.UnitSegment;
        end;
      If (TypePtr.Seg <> PUnitIdentifierData (Ptr (Seg (Expression.TypeDefPtr^),
                          POrdinalTypeDefinition (Expression.TypeDefPtr)^.OrdinalType.UnitIdentifierData))^.UnitSegment) or
       (TypePtr.Ofs <> POrdinalTypeDefinition (Expression.TypeDefPtr)^.OrdinalType.TypeOffset + 20) then Error (TypeMismatch);
      Expression.CheckOrdinalRange (Ptr (SystemUnitSegment, Byte_TypeOffset));
    end;

  begin
    GetNextToken;
    TempSet := [];
    TypePtr.Ptr := nil;
    NonConstantElementCode.NumberOfStatements := 0;
    If not CheckAndGetNextToken (Token_RightBracket) then
      begin
        Repeat
          ExpectSetElement (LowerLimitExpression);
          If CheckAndGetNextToken (Token_PeriodPeriod) then
            begin                                                       { Range }
              ExpectSetElement (UpperLimitExpression);
              If (LowerLimitExpression.Location = elConstant) and (UpperLimitExpression.Location = elConstant) then
                begin
                  LowerLimit := LowerLimitExpression.Value.Byte;
                  UpperLimit := UpperLimitExpression.Value.Byte;
                end else begin
                           LowerLimitExpression.Calculate;
                           LowerLimitExpression.Push;
                           UpperLimitExpression.Calculate;
                           UpperLimitExpression.Push;
                           GenerateInstruction_CALL_FAR (SysProc_ZAddRange);
                           AddCallToIntermediateCodeSubroutine (NonConstantElementCode, EndSubroutine);
                           Continue;
                         end;
            end else begin                                              { Single set element }
                       If LowerLimitExpression.Location = elConstant then
                         begin
                           LowerLimit := LowerLimitExpression.Value.Byte;
                           UpperLimit := LowerLimit;
                         end else begin
                                   LowerLimitExpression.Calculate;
                                   LowerLimitExpression.Push;
                                   GenerateInstruction_CALL_FAR (SysProc_ZAddComp);
                                   AddCallToIntermediateCodeSubroutine (NonConstantElementCode, EndSubroutine);
                                   Continue;
                                  end;
                     end;
          If LowerLimit <= UpperLimit then
            For Element := LowerLimit to UpperLimit do Include (TempSet, Element);
        until not CheckAndGetNextToken (Token_Comma);
        ExpectTokenAndGetNext (TOKEN_RightBracket);
      end else begin                                    { Empty set }
                 TypePtr.Ofs := EmptySet_TypeOffset;
                 TypePtr.Seg := SystemUnitSegment;
               end;
    DataOffset := GetTempBufferOffset (32);
    Move (TempSet, Ptr (DSeg, DataOffset)^, 32);
    TypeDefPtr := TypePtr.Ptr;
    IntermediateCodeOffset := 0;
    Location := elConstant;
    UsedRegisters := [];
    Value.Word := DataOffset;
    SetDataTypeOfSet;
    If NonConstantElementCode.NumberOfStatements <> 0 then
      begin
        ExpandSetToStackFrameAndPushPointer (0);
        Calculate;
        Generate_icGoSub_ForEachSubroutine (NonConstantElementCode);
        EndIntermediateCodeSubroutine;
      end;
  end;
```

### Code Block 2

```pascal
  Procedure ProcessSetConstant;
  Var TempSet: Set of Byte;
      LowerLimitExpression, UpperLimitExpression: TExpression;
      Element, LowerLimit, UpperLimit: Byte;
      NonConstantElementCode: TIntermediateCodeOffsets;
      TypePtr: PtrRec;
      N: Byte;
      DataOffset: Word;

    Procedure ExpectSetElement (Var Expression: TExpression);
    begin
      Expression.ExpectOrdinalExpression;
      If TypePtr.Ptr = nil then
        begin
          TypePtr.Ofs := POrdinalTypeDefinition (Expression.TypeDefPtr)^.OrdinalType.TypeOffset + 20;
          TypePtr.Seg := PUnitIdentifierData (Ptr (Seg (Expression.TypeDefPtr^),
                             POrdinalTypeDefinition (Expression.TypeDefPtr)^.OrdinalType.UnitIdentifierData))^.UnitSegment;
        end;
      If (TypePtr.Seg <> PUnitIdentifierData (Ptr (Seg (Expression.TypeDefPtr^),
                          POrdinalTypeDefinition (Expression.TypeDefPtr)^.OrdinalType.UnitIdentifierData))^.UnitSegment) or
       (TypePtr.Ofs <> POrdinalTypeDefinition (Expression.TypeDefPtr)^.OrdinalType.TypeOffset + 20) then Error (TypeMismatch);
      Expression.CheckOrdinalRange (Ptr (SystemUnitSegment, Byte_TypeOffset));
    end;

  begin
    GetNextToken;
    TempSet := [];
    TypePtr.Ptr := nil;
    NonConstantElementCode.NumberOfStatements := 0;
    If not CheckAndGetNextToken (Token_RightBracket) then
      begin
        Repeat
          ExpectSetElement (LowerLimitExpression);
          If CheckAndGetNextToken (Token_PeriodPeriod) then
            begin                                                       { Range }
              ExpectSetElement (UpperLimitExpression);
              If (LowerLimitExpression.Location = elConstant) and (UpperLimitExpression.Location = elConstant) then
                begin
                  LowerLimit := LowerLimitExpression.Value.Byte;
                  UpperLimit := UpperLimitExpression.Value.Byte;
                end else begin
                           LowerLimitExpression.Calculate;
                           LowerLimitExpression.Push;
                           UpperLimitExpression.Calculate;
                           UpperLimitExpression.Push;
                           GenerateInstruction_CALL_FAR (SysProc_ZAddRange);
                           AddCallToIntermediateCodeSubroutine (NonConstantElementCode, EndSubroutine);
                           Continue;
                         end;
            end else begin                                              { Single set element }
                       If LowerLimitExpression.Location = elConstant then
                         begin
                           LowerLimit := LowerLimitExpression.Value.Byte;
                           UpperLimit := LowerLimit;
                         end else begin
                                   LowerLimitExpression.Calculate;
                                   LowerLimitExpression.Push;
                                   GenerateInstruction_CALL_FAR (SysProc_ZAddComp);
                                   AddCallToIntermediateCodeSubroutine (NonConstantElementCode, EndSubroutine);
                                   Continue;
                                  end;
                     end;
          If LowerLimit <= UpperLimit then
            For Element := LowerLimit to UpperLimit do Include (TempSet, Element);
        until not CheckAndGetNextToken (Token_Comma);
        ExpectTokenAndGetNext (TOKEN_RightBracket);
      end else begin                                    { Empty set }
                 TypePtr.Ofs := EmptySet_TypeOffset;
                 TypePtr.Seg := SystemUnitSegment;
               end;
    DataOffset := GetTempBufferOffset (32);
    Move (TempSet, Ptr (DSeg, DataOffset)^, 32);
    TypeDefPtr := TypePtr.Ptr;
    IntermediateCodeOffset := 0;
    Location := elConstant;
    UsedRegisters := [];
    Value.Word := DataOffset;
    SetDataTypeOfSet;
    If NonConstantElementCode.NumberOfStatements <> 0 then
      begin
        ExpandSetToStackFrameAndPushPointer (0);
        Calculate;
        Generate_icGoSub_ForEachSubroutine (NonConstantElementCode);
        EndIntermediateCodeSubroutine;
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
- https://turbopascal.org/processing-expressions
- https://turbopascal.org/processing-set-constants
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

