# Processing Constant Declarations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-constant-declarations

---

## Table of Contents

- Processing Constant Declarations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Constant Declarations
Turbo Pascal constants can be of two types: ordinary constants which are only stored to the symbol table and
typed constants
which are also embedded into the program. Therefore, typed constant values are stored into a separate symbol table.
Procedure ProcessConstantDeclarations;
Var TypePtr: PTypeDefinition;
    Variable: PVariableIdentifierData;
    Constant: PConstantIdentifierData absolute Variable;
    ConstantIdentifier: PIdentifier;
    Expression: TExpression;
    DataPtr, AddData: Pointer;
    Size: Word;
    UnitTypeOffsets: TUnitOffsets;

  Procedure GetTypedConstantInitialValue;
  Var VarData: Pointer;
      Size: Word;
  begin
    Size := TypePtr^.Size;
    VarData := IncreaseSymbolTable (stTypedConstants, Size);
    CurrentTypedConstantValueOffset := Ofs (VarData^);
    FillChar (VarData^, Size, 0);
    TypedConstantsOffsetBeforeConstantValueProcessing := Ofs (VarData^) + Size;
    ProcessTypedConstant (TypePtr);
    TypedConstantsOffsetBeforeConstantValueProcessing := LastTypedConstantsSize;
  end;

begin
  GetNextToken;
  Repeat
    ConstantIdentifier := ExpectAndStoreIdentifier (0, AddData);
    If CheckAndGetNextToken (Token_Colon) then
      begin                                            { Typed constant }
        Variable := IncreaseSymbolTable (stMain, SizeOf (TVariableIdentifierData));
        TokenForEqual := Token_EqualAsSeparator;
        TypePtr := GetVariableType;
        TokenForEqual := Token_Equal;
        If (WordAlignment in ModuleCompilerSwitches) and (TypePtr^.Size <> 1) then WordAlignAddressForTypedConstants;
        ForceCreationOfTypedConstantsBlockRecord := True;
        VariableData_Flags := TypedConstant;
        RecordTypeDefinitionOffset.TypedConstantOffset :=
          SymbolTable [stTypedConstants].NextRecordOffset - LastTypedConstantsSize;
        RecordTypeDefinitionOffset.TypedConstantsBlockRecordOffset := SymbolTable [stTypedConstantsBlocks].NextRecordOffset;
        ExpectTokenAndGetNext (Token_EqualAsSeparator);
        GetTypedConstantInitialValue;
        ConstantIdentifier^.Token := Token_VariableIdentifier;
        With Variable^ do
          begin
            Flags := VariableData_Flags;
            W1.Ofs := RecordTypeDefinitionOffset.TypedConstantOffset;
            W1.Seg := RecordTypeDefinitionOffset.TypedConstantsBlockRecordOffset;
            W5 := VariableData_NextMemberOffset;
            UnitTypeOffsets := CurrentVarUnitTypeOffsets;
          end;
      end else
        begin
          ExpectTokenAndGetNext (Token_Equal);
          Expression.ExpectConstantExpression;
          ConstantIdentifier^.Token := Token_ConstantIdentifier;
          DataPtr := @Expression.Value;
          TypePtr := Expression.TypeDefPtr;
          Size := 4;
          If TypePtr^.BaseType < btInteger then
            Case TypePtr^.BaseType of
              btPointer: Size := 4;
              btExtended: Size := 10;
              else begin
                    DataPtr := Ptr (DSeg, Expression.Value.Word);
                    If TypePtr^.BaseType = btSet then Size := 32
                      else Size := Byte (DataPtr^) + 1;
                 end;
            end;
          GetTypeAndUnitIdentifierOffsets (TypePtr, UnitTypeOffsets);
          Constant := IncreaseSymbolTable (stMain, SizeOf (TUnitOffsets) + Size);
          Constant^.UnitTypeOffsets := UnitTypeOffsets;
          Move (DataPtr^, Constant^.Value, Size);
        end;
    ExpectTokenAndGetNext (TOKEN_Semicolon);
  until Token <> Token_Identifier;
  CreateTypedConstantsBlockRecord;
end;
For each Const declaration Turbo Pascal creates a record in symbol table for typed constants (if they are declared).
Procedure CreateTypedConstantsBlockRecord;
begin
  WordAlignAddressForTypedConstants;
  If (SymbolTable [stTypedConstants].UsedSize <> LastTypedConstantsSize) or ForceCreationOfTypedConstantsBlockRecord then
    begin
      ForceCreationOfTypedConstantsBlockRecord := False;
      With PTypedConstantsBlockRecord (IncreaseSymbolTable (stTypedConstantsBlocks, SizeOf (TTypedConstantsBlockRecord)))^ do
        begin
          Offset := 0;
          ConstantsSize := SymbolTable [stTypedConstants].UsedSize - LastTypedConstantsSize;
          ReferencesSize := SymbolTable [stTypedConstantsReferences].UsedSize - LastTypedConstantsReferencesSize;
          tcrW6 := CurrentRecordOrObjectTypeDefinitionOffset;
        end;
      LastTypedConstantsSize := SymbolTable [stTypedConstants].UsedSize;
      TypedConstantsOffsetBeforeConstantValueProcessing := SymbolTable [stTypedConstants].NextRecordOffset;
      LastTypedConstantsReferencesSize := SymbolTable [stTypedConstantsReferences].UsedSize;
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
Procedure ProcessConstantDeclarations;
Var TypePtr: PTypeDefinition;
    Variable: PVariableIdentifierData;
    Constant: PConstantIdentifierData absolute Variable;
    ConstantIdentifier: PIdentifier;
    Expression: TExpression;
    DataPtr, AddData: Pointer;
    Size: Word;
    UnitTypeOffsets: TUnitOffsets;

  Procedure GetTypedConstantInitialValue;
  Var VarData: Pointer;
      Size: Word;
  begin
    Size := TypePtr^.Size;
    VarData := IncreaseSymbolTable (stTypedConstants, Size);
    CurrentTypedConstantValueOffset := Ofs (VarData^);
    FillChar (VarData^, Size, 0);
    TypedConstantsOffsetBeforeConstantValueProcessing := Ofs (VarData^) + Size;
    ProcessTypedConstant (TypePtr);
    TypedConstantsOffsetBeforeConstantValueProcessing := LastTypedConstantsSize;
  end;

begin
  GetNextToken;
  Repeat
    ConstantIdentifier := ExpectAndStoreIdentifier (0, AddData);
    If CheckAndGetNextToken (Token_Colon) then
      begin                                            { Typed constant }
        Variable := IncreaseSymbolTable (stMain, SizeOf (TVariableIdentifierData));
        TokenForEqual := Token_EqualAsSeparator;
        TypePtr := GetVariableType;
        TokenForEqual := Token_Equal;
        If (WordAlignment in ModuleCompilerSwitches) and (TypePtr^.Size <> 1) then WordAlignAddressForTypedConstants;
        ForceCreationOfTypedConstantsBlockRecord := True;
        VariableData_Flags := TypedConstant;
        RecordTypeDefinitionOffset.TypedConstantOffset :=
          SymbolTable [stTypedConstants].NextRecordOffset - LastTypedConstantsSize;
        RecordTypeDefinitionOffset.TypedConstantsBlockRecordOffset := SymbolTable [stTypedConstantsBlocks].NextRecordOffset;
        ExpectTokenAndGetNext (Token_EqualAsSeparator);
        GetTypedConstantInitialValue;
        ConstantIdentifier^.Token := Token_VariableIdentifier;
        With Variable^ do
          begin
            Flags := VariableData_Flags;
            W1.Ofs := RecordTypeDefinitionOffset.TypedConstantOffset;
            W1.Seg := RecordTypeDefinitionOffset.TypedConstantsBlockRecordOffset;
            W5 := VariableData_NextMemberOffset;
            UnitTypeOffsets := CurrentVarUnitTypeOffsets;
          end;
      end else
        begin
          ExpectTokenAndGetNext (Token_Equal);
          Expression.ExpectConstantExpression;
          ConstantIdentifier^.Token := Token_ConstantIdentifier;
          DataPtr := @Expression.Value;
          TypePtr := Expression.TypeDefPtr;
          Size := 4;
          If TypePtr^.BaseType < btInteger then
            Case TypePtr^.BaseType of
              btPointer: Size := 4;
              btExtended: Size := 10;
              else begin
                    DataPtr := Ptr (DSeg, Expression.Value.Word);
                    If TypePtr^.BaseType = btSet then Size := 32
                      else Size := Byte (DataPtr^) + 1;
                 end;
            end;
          GetTypeAndUnitIdentifierOffsets (TypePtr, UnitTypeOffsets);
          Constant := IncreaseSymbolTable (stMain, SizeOf (TUnitOffsets) + Size);
          Constant^.UnitTypeOffsets := UnitTypeOffsets;
          Move (DataPtr^, Constant^.Value, Size);
        end;
    ExpectTokenAndGetNext (TOKEN_Semicolon);
  until Token <> Token_Identifier;
  CreateTypedConstantsBlockRecord;
end;
```

### Code Block 2

```pascal
Procedure CreateTypedConstantsBlockRecord;
begin
  WordAlignAddressForTypedConstants;
  If (SymbolTable [stTypedConstants].UsedSize <> LastTypedConstantsSize) or ForceCreationOfTypedConstantsBlockRecord then
    begin
      ForceCreationOfTypedConstantsBlockRecord := False;
      With PTypedConstantsBlockRecord (IncreaseSymbolTable (stTypedConstantsBlocks, SizeOf (TTypedConstantsBlockRecord)))^ do
        begin
          Offset := 0;
          ConstantsSize := SymbolTable [stTypedConstants].UsedSize - LastTypedConstantsSize;
          ReferencesSize := SymbolTable [stTypedConstantsReferences].UsedSize - LastTypedConstantsReferencesSize;
          tcrW6 := CurrentRecordOrObjectTypeDefinitionOffset;
        end;
      LastTypedConstantsSize := SymbolTable [stTypedConstants].UsedSize;
      TypedConstantsOffsetBeforeConstantValueProcessing := SymbolTable [stTypedConstants].NextRecordOffset;
      LastTypedConstantsReferencesSize := SymbolTable [stTypedConstantsReferences].UsedSize;
    end;
end;
```

### Code Block 3

```pascal
Procedure ProcessConstantDeclarations;
Var TypePtr: PTypeDefinition;
    Variable: PVariableIdentifierData;
    Constant: PConstantIdentifierData absolute Variable;
    ConstantIdentifier: PIdentifier;
    Expression: TExpression;
    DataPtr, AddData: Pointer;
    Size: Word;
    UnitTypeOffsets: TUnitOffsets;

  Procedure GetTypedConstantInitialValue;
  Var VarData: Pointer;
      Size: Word;
  begin
    Size := TypePtr^.Size;
    VarData := IncreaseSymbolTable (stTypedConstants, Size);
    CurrentTypedConstantValueOffset := Ofs (VarData^);
    FillChar (VarData^, Size, 0);
    TypedConstantsOffsetBeforeConstantValueProcessing := Ofs (VarData^) + Size;
    ProcessTypedConstant (TypePtr);
    TypedConstantsOffsetBeforeConstantValueProcessing := LastTypedConstantsSize;
  end;

begin
  GetNextToken;
  Repeat
    ConstantIdentifier := ExpectAndStoreIdentifier (0, AddData);
    If CheckAndGetNextToken (Token_Colon) then
      begin                                            { Typed constant }
        Variable := IncreaseSymbolTable (stMain, SizeOf (TVariableIdentifierData));
        TokenForEqual := Token_EqualAsSeparator;
        TypePtr := GetVariableType;
        TokenForEqual := Token_Equal;
        If (WordAlignment in ModuleCompilerSwitches) and (TypePtr^.Size <> 1) then WordAlignAddressForTypedConstants;
        ForceCreationOfTypedConstantsBlockRecord := True;
        VariableData_Flags := TypedConstant;
        RecordTypeDefinitionOffset.TypedConstantOffset :=
          SymbolTable [stTypedConstants].NextRecordOffset - LastTypedConstantsSize;
        RecordTypeDefinitionOffset.TypedConstantsBlockRecordOffset := SymbolTable [stTypedConstantsBlocks].NextRecordOffset;
        ExpectTokenAndGetNext (Token_EqualAsSeparator);
        GetTypedConstantInitialValue;
        ConstantIdentifier^.Token := Token_VariableIdentifier;
        With Variable^ do
          begin
            Flags := VariableData_Flags;
            W1.Ofs := RecordTypeDefinitionOffset.TypedConstantOffset;
            W1.Seg := RecordTypeDefinitionOffset.TypedConstantsBlockRecordOffset;
            W5 := VariableData_NextMemberOffset;
            UnitTypeOffsets := CurrentVarUnitTypeOffsets;
          end;
      end else
        begin
          ExpectTokenAndGetNext (Token_Equal);
          Expression.ExpectConstantExpression;
          ConstantIdentifier^.Token := Token_ConstantIdentifier;
          DataPtr := @Expression.Value;
          TypePtr := Expression.TypeDefPtr;
          Size := 4;
          If TypePtr^.BaseType < btInteger then
            Case TypePtr^.BaseType of
              btPointer: Size := 4;
              btExtended: Size := 10;
              else begin
                    DataPtr := Ptr (DSeg, Expression.Value.Word);
                    If TypePtr^.BaseType = btSet then Size := 32
                      else Size := Byte (DataPtr^) + 1;
                 end;
            end;
          GetTypeAndUnitIdentifierOffsets (TypePtr, UnitTypeOffsets);
          Constant := IncreaseSymbolTable (stMain, SizeOf (TUnitOffsets) + Size);
          Constant^.UnitTypeOffsets := UnitTypeOffsets;
          Move (DataPtr^, Constant^.Value, Size);
        end;
    ExpectTokenAndGetNext (TOKEN_Semicolon);
  until Token <> Token_Identifier;
  CreateTypedConstantsBlockRecord;
end;
```

### Code Block 4

```pascal
Procedure CreateTypedConstantsBlockRecord;
begin
  WordAlignAddressForTypedConstants;
  If (SymbolTable [stTypedConstants].UsedSize <> LastTypedConstantsSize) or ForceCreationOfTypedConstantsBlockRecord then
    begin
      ForceCreationOfTypedConstantsBlockRecord := False;
      With PTypedConstantsBlockRecord (IncreaseSymbolTable (stTypedConstantsBlocks, SizeOf (TTypedConstantsBlockRecord)))^ do
        begin
          Offset := 0;
          ConstantsSize := SymbolTable [stTypedConstants].UsedSize - LastTypedConstantsSize;
          ReferencesSize := SymbolTable [stTypedConstantsReferences].UsedSize - LastTypedConstantsReferencesSize;
          tcrW6 := CurrentRecordOrObjectTypeDefinitionOffset;
        end;
      LastTypedConstantsSize := SymbolTable [stTypedConstants].UsedSize;
      TypedConstantsOffsetBeforeConstantValueProcessing := SymbolTable [stTypedConstants].NextRecordOffset;
      LastTypedConstantsReferencesSize := SymbolTable [stTypedConstantsReferences].UsedSize;
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
- https://turbopascal.org/processing-constant-declarations
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

