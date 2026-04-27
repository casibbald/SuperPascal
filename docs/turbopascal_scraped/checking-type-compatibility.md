# Checking Type Compatibility - Turbo Pascal Compiler

**URL:** https://turbopascal.org/checking-type-compatibility

---

## Table of Contents

- Checking Type Compatibility

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Checking Type Compatibility
This procedure checks type compatibility between expression and specified type and sets expression to the specified type if needed.
Procedure TExpression.CheckTypeCompatibility (SpecifiedType: PTypeDefinition);
Var LeftOrdinalType, RightOrdinalType: PTypeDefinition;
    SetTypeDefinition:     PSetTypeDefinition     absolute SpecifiedType;
    PointerTypeDefinition: PPointerTypeDefinition absolute SpecifiedType;
    OrdinalTypeDefinition: POrdinalTypeDefinition absolute SpecifiedType;
begin
  If TypeDefPtr^.BaseType <> SpecifiedType^.BaseType then Error (TypeMismatch);
  Case TypeDefPtr^.BaseType of
    btUntyped..btText:   If TypeDefPtr <> SpecifiedType then Error (TypeMismatch);
    btFunctionProcedure: If not CheckProcedureTypeCompatibility (Pointer (TypeDefPtr), Pointer (SpecifiedType)) then
                           Error (TypeMismatch);
    btSet: begin
             If PSetTypeDefinition (TypeDefPtr)^.BaseSetTypeOffset.TypeOffset = 0 then TypeDefPtr := SpecifiedType else
               begin
                 LeftOrdinalType := PointerFromOffsets (POrdinalTypeDefinition (
                                      PointerFromOffsets (PSetTypeDefinition (TypeDefPtr)^.BaseSetTypeOffset))^.OrdinalType);
                 If SetTypeDefinition^.BaseSetTypeOffset.TypeOffset = 0 then Exit;
                 RightOrdinalType := PointerFromOffsets (POrdinalTypeDefinition (
                                       PointerFromOffsets (SetTypeDefinition^.BaseSetTypeOffset))^.OrdinalType);
                 If LeftOrdinalType <> RightOrdinalType then Error (TypeMismatch);
               end;
           end;
    btPointer: begin
                 If TypeDefPtr = SpecifiedType then Exit;
                 If SpecifiedType = Ptr (SystemUnitSegment, Pointer_TypeOffset) then Exit;
                 If TypeDefPtr = Ptr (SystemUnitSegment, Pointer_TypeOffset) then
                   TypeDefPtr := SpecifiedType else
                     If TypedPointers in ModuleCompilerSwitches then
                       If PointerFromOffsets (PPointerTypeDefinition (TypeDefPtr)^.PointerBaseTypeOffset) <>
                          PointerFromOffsets (PointerTypeDefinition^.PointerBaseTypeOffset) then Error (TypeMismatch);
               end;
    btEnumeration: begin
                     If PointerFromOffsets (POrdinalTypeDefinition (TypeDefPtr)^.OrdinalType) <>
                        PointerFromOffsets (OrdinalTypeDefinition^.OrdinalType) then Error (TypeMismatch);
                   end;
  end;
end;
This procedure checks if procedure types are compatibe:
Base types must be equal
Data types must be equal
Result types must be equal (for functions)
Number of parameters must be equal
Equivalent parameter types must be equal
Equivalent parameter flags must be equal
Function CheckProcedureTypeCompatibility (LeftType, RightType: PProcedureTypeDefinition): Boolean;
Var LeftParameter: PProcedureParameterData absolute LeftType;
    RightParameter: PProcedureParameterData absolute RightType;
    NumberOfParameters: Word;

  Function CompareUnitTypeOffsets (LeftData: Pointer; LeftUnitTypeOffsets: TUnitOffsets;
                                   RightData: Pointer; RightUnitTypeOffsets: TUnitOffsets): Boolean;
  Var LeftUnitIdData: PUnitIdentifierData absolute LeftData;
      LeftUnitIdDataOfs: Word absolute LeftData;
      RightUnitIdData: PUnitIdentifierData absolute RightData;
      RightUnitIdDataOfs: Word absolute RightData;
  begin
    CompareUnitTypeOffsets := False;
    If LeftUnitTypeOffsets.TypeOffset <> RightUnitTypeOffsets.TypeOffset then Exit;
    LeftUnitIdDataOfs := LeftUnitTypeOffsets.UnitIdentifierData;
    RightUnitIdDataOfs := RightUnitTypeOffsets.UnitIdentifierData;
    If LeftUnitIdData^.UnitSegment <> 0 then
      If LeftUnitIdData^.UnitSegment <> RightUnitIdData^.UnitSegment then Exit;
    CompareUnitTypeOffsets := True;
  end;

begin
  CheckProcedureTypeCompatibility := False;
  If LeftType^.BaseType <> RightType^.BaseType then Exit;
  If LeftType^.DataType <> RightType^.DataType then Exit;
  If LeftType^.DataType <> RightType^.DataType then Exit;
  If not CompareUnitTypeOffsets (LeftType, LeftType^.ResultTypeOffset, RightType, RightType^.ResultTypeOffset) then Exit;
  If LeftType^.NumberOfParameters <> RightType^.NumberOfParameters then Exit;
  NumberOfParameters := LeftType^.NumberOfParameters;
  LeftParameter := PProcedureParameterData (PChar (LeftType) + 14);
  RightParameter := PProcedureParameterData (PChar (RightType) + 14);
  While NumberOfParameters <> 0 do
    begin
      If not CompareUnitTypeOffsets (LeftParameter, LeftParameter^.UnitTypeOffsets,
                                     RightParameter, RightParameter^.UnitTypeOffsets) then Exit;
      If LeftParameter^.VarFlags <> RightParameter^.VarFlags then Exit;
      Inc (PChar (LeftParameter), SizeOf (TProcedureParameterData));
      Inc (PChar (RightParameter), SizeOf (TProcedureParameterData));
      Dec (NumberOfParameters);
    end;
  CheckProcedureTypeCompatibility := True;
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
Procedure TExpression.CheckTypeCompatibility (SpecifiedType: PTypeDefinition);
Var LeftOrdinalType, RightOrdinalType: PTypeDefinition;
    SetTypeDefinition:     PSetTypeDefinition     absolute SpecifiedType;
    PointerTypeDefinition: PPointerTypeDefinition absolute SpecifiedType;
    OrdinalTypeDefinition: POrdinalTypeDefinition absolute SpecifiedType;
begin
  If TypeDefPtr^.BaseType <> SpecifiedType^.BaseType then Error (TypeMismatch);
  Case TypeDefPtr^.BaseType of
    btUntyped..btText:   If TypeDefPtr <> SpecifiedType then Error (TypeMismatch);
    btFunctionProcedure: If not CheckProcedureTypeCompatibility (Pointer (TypeDefPtr), Pointer (SpecifiedType)) then
                           Error (TypeMismatch);
    btSet: begin
             If PSetTypeDefinition (TypeDefPtr)^.BaseSetTypeOffset.TypeOffset = 0 then TypeDefPtr := SpecifiedType else
               begin
                 LeftOrdinalType := PointerFromOffsets (POrdinalTypeDefinition (
                                      PointerFromOffsets (PSetTypeDefinition (TypeDefPtr)^.BaseSetTypeOffset))^.OrdinalType);
                 If SetTypeDefinition^.BaseSetTypeOffset.TypeOffset = 0 then Exit;
                 RightOrdinalType := PointerFromOffsets (POrdinalTypeDefinition (
                                       PointerFromOffsets (SetTypeDefinition^.BaseSetTypeOffset))^.OrdinalType);
                 If LeftOrdinalType <> RightOrdinalType then Error (TypeMismatch);
               end;
           end;
    btPointer: begin
                 If TypeDefPtr = SpecifiedType then Exit;
                 If SpecifiedType = Ptr (SystemUnitSegment, Pointer_TypeOffset) then Exit;
                 If TypeDefPtr = Ptr (SystemUnitSegment, Pointer_TypeOffset) then
                   TypeDefPtr := SpecifiedType else
                     If TypedPointers in ModuleCompilerSwitches then
                       If PointerFromOffsets (PPointerTypeDefinition (TypeDefPtr)^.PointerBaseTypeOffset) <>
                          PointerFromOffsets (PointerTypeDefinition^.PointerBaseTypeOffset) then Error (TypeMismatch);
               end;
    btEnumeration: begin
                     If PointerFromOffsets (POrdinalTypeDefinition (TypeDefPtr)^.OrdinalType) <>
                        PointerFromOffsets (OrdinalTypeDefinition^.OrdinalType) then Error (TypeMismatch);
                   end;
  end;
end;
```

### Code Block 2

```pascal
Function CheckProcedureTypeCompatibility (LeftType, RightType: PProcedureTypeDefinition): Boolean;
Var LeftParameter: PProcedureParameterData absolute LeftType;
    RightParameter: PProcedureParameterData absolute RightType;
    NumberOfParameters: Word;

  Function CompareUnitTypeOffsets (LeftData: Pointer; LeftUnitTypeOffsets: TUnitOffsets;
                                   RightData: Pointer; RightUnitTypeOffsets: TUnitOffsets): Boolean;
  Var LeftUnitIdData: PUnitIdentifierData absolute LeftData;
      LeftUnitIdDataOfs: Word absolute LeftData;
      RightUnitIdData: PUnitIdentifierData absolute RightData;
      RightUnitIdDataOfs: Word absolute RightData;
  begin
    CompareUnitTypeOffsets := False;
    If LeftUnitTypeOffsets.TypeOffset <> RightUnitTypeOffsets.TypeOffset then Exit;
    LeftUnitIdDataOfs := LeftUnitTypeOffsets.UnitIdentifierData;
    RightUnitIdDataOfs := RightUnitTypeOffsets.UnitIdentifierData;
    If LeftUnitIdData^.UnitSegment <> 0 then
      If LeftUnitIdData^.UnitSegment <> RightUnitIdData^.UnitSegment then Exit;
    CompareUnitTypeOffsets := True;
  end;

begin
  CheckProcedureTypeCompatibility := False;
  If LeftType^.BaseType <> RightType^.BaseType then Exit;
  If LeftType^.DataType <> RightType^.DataType then Exit;
  If LeftType^.DataType <> RightType^.DataType then Exit;
  If not CompareUnitTypeOffsets (LeftType, LeftType^.ResultTypeOffset, RightType, RightType^.ResultTypeOffset) then Exit;
  If LeftType^.NumberOfParameters <> RightType^.NumberOfParameters then Exit;
  NumberOfParameters := LeftType^.NumberOfParameters;
  LeftParameter := PProcedureParameterData (PChar (LeftType) + 14);
  RightParameter := PProcedureParameterData (PChar (RightType) + 14);
  While NumberOfParameters <> 0 do
    begin
      If not CompareUnitTypeOffsets (LeftParameter, LeftParameter^.UnitTypeOffsets,
                                     RightParameter, RightParameter^.UnitTypeOffsets) then Exit;
      If LeftParameter^.VarFlags <> RightParameter^.VarFlags then Exit;
      Inc (PChar (LeftParameter), SizeOf (TProcedureParameterData));
      Inc (PChar (RightParameter), SizeOf (TProcedureParameterData));
      Dec (NumberOfParameters);
    end;
  CheckProcedureTypeCompatibility := True;
end;
```

### Code Block 3

```pascal
Procedure TExpression.CheckTypeCompatibility (SpecifiedType: PTypeDefinition);
Var LeftOrdinalType, RightOrdinalType: PTypeDefinition;
    SetTypeDefinition:     PSetTypeDefinition     absolute SpecifiedType;
    PointerTypeDefinition: PPointerTypeDefinition absolute SpecifiedType;
    OrdinalTypeDefinition: POrdinalTypeDefinition absolute SpecifiedType;
begin
  If TypeDefPtr^.BaseType <> SpecifiedType^.BaseType then Error (TypeMismatch);
  Case TypeDefPtr^.BaseType of
    btUntyped..btText:   If TypeDefPtr <> SpecifiedType then Error (TypeMismatch);
    btFunctionProcedure: If not CheckProcedureTypeCompatibility (Pointer (TypeDefPtr), Pointer (SpecifiedType)) then
                           Error (TypeMismatch);
    btSet: begin
             If PSetTypeDefinition (TypeDefPtr)^.BaseSetTypeOffset.TypeOffset = 0 then TypeDefPtr := SpecifiedType else
               begin
                 LeftOrdinalType := PointerFromOffsets (POrdinalTypeDefinition (
                                      PointerFromOffsets (PSetTypeDefinition (TypeDefPtr)^.BaseSetTypeOffset))^.OrdinalType);
                 If SetTypeDefinition^.BaseSetTypeOffset.TypeOffset = 0 then Exit;
                 RightOrdinalType := PointerFromOffsets (POrdinalTypeDefinition (
                                       PointerFromOffsets (SetTypeDefinition^.BaseSetTypeOffset))^.OrdinalType);
                 If LeftOrdinalType <> RightOrdinalType then Error (TypeMismatch);
               end;
           end;
    btPointer: begin
                 If TypeDefPtr = SpecifiedType then Exit;
                 If SpecifiedType = Ptr (SystemUnitSegment, Pointer_TypeOffset) then Exit;
                 If TypeDefPtr = Ptr (SystemUnitSegment, Pointer_TypeOffset) then
                   TypeDefPtr := SpecifiedType else
                     If TypedPointers in ModuleCompilerSwitches then
                       If PointerFromOffsets (PPointerTypeDefinition (TypeDefPtr)^.PointerBaseTypeOffset) <>
                          PointerFromOffsets (PointerTypeDefinition^.PointerBaseTypeOffset) then Error (TypeMismatch);
               end;
    btEnumeration: begin
                     If PointerFromOffsets (POrdinalTypeDefinition (TypeDefPtr)^.OrdinalType) <>
                        PointerFromOffsets (OrdinalTypeDefinition^.OrdinalType) then Error (TypeMismatch);
                   end;
  end;
end;
```

### Code Block 4

```pascal
Function CheckProcedureTypeCompatibility (LeftType, RightType: PProcedureTypeDefinition): Boolean;
Var LeftParameter: PProcedureParameterData absolute LeftType;
    RightParameter: PProcedureParameterData absolute RightType;
    NumberOfParameters: Word;

  Function CompareUnitTypeOffsets (LeftData: Pointer; LeftUnitTypeOffsets: TUnitOffsets;
                                   RightData: Pointer; RightUnitTypeOffsets: TUnitOffsets): Boolean;
  Var LeftUnitIdData: PUnitIdentifierData absolute LeftData;
      LeftUnitIdDataOfs: Word absolute LeftData;
      RightUnitIdData: PUnitIdentifierData absolute RightData;
      RightUnitIdDataOfs: Word absolute RightData;
  begin
    CompareUnitTypeOffsets := False;
    If LeftUnitTypeOffsets.TypeOffset <> RightUnitTypeOffsets.TypeOffset then Exit;
    LeftUnitIdDataOfs := LeftUnitTypeOffsets.UnitIdentifierData;
    RightUnitIdDataOfs := RightUnitTypeOffsets.UnitIdentifierData;
    If LeftUnitIdData^.UnitSegment <> 0 then
      If LeftUnitIdData^.UnitSegment <> RightUnitIdData^.UnitSegment then Exit;
    CompareUnitTypeOffsets := True;
  end;

begin
  CheckProcedureTypeCompatibility := False;
  If LeftType^.BaseType <> RightType^.BaseType then Exit;
  If LeftType^.DataType <> RightType^.DataType then Exit;
  If LeftType^.DataType <> RightType^.DataType then Exit;
  If not CompareUnitTypeOffsets (LeftType, LeftType^.ResultTypeOffset, RightType, RightType^.ResultTypeOffset) then Exit;
  If LeftType^.NumberOfParameters <> RightType^.NumberOfParameters then Exit;
  NumberOfParameters := LeftType^.NumberOfParameters;
  LeftParameter := PProcedureParameterData (PChar (LeftType) + 14);
  RightParameter := PProcedureParameterData (PChar (RightType) + 14);
  While NumberOfParameters <> 0 do
    begin
      If not CompareUnitTypeOffsets (LeftParameter, LeftParameter^.UnitTypeOffsets,
                                     RightParameter, RightParameter^.UnitTypeOffsets) then Exit;
      If LeftParameter^.VarFlags <> RightParameter^.VarFlags then Exit;
      Inc (PChar (LeftParameter), SizeOf (TProcedureParameterData));
      Inc (PChar (RightParameter), SizeOf (TProcedureParameterData));
      Dec (NumberOfParameters);
    end;
  CheckProcedureTypeCompatibility := True;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/checking-type-compatibility
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
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

