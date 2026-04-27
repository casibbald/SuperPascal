# System Functions Low and High - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-functions-low-and-high

---

## Table of Contents

- System Functions Low and High

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Functions Low and High
This procedure processes system function
Low
and
High
.
Procedure Func_Low_High; Far;
Var TypeDefinition: PTypeDefinition;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  If IsTypeIdentifier (Expression^.TypeDefPtr) or not OpenArrayOrOpenString (Expression^.TypeDefPtr) then
    begin
      If (Expression^.TypeDefPtr^.BaseType = btArray) or (Expression^.TypeDefPtr^.BaseType = btString) then
        TypeDefinition := PointerFromOffsets (PArrayTypeDefinition (Expression^.TypeDefPtr)^.IndexTypeOffset) else
          TypeDefinition := Expression^.TypeDefPtr;
      If TypeDefinition^.BaseType < btInteger then Error (OrdinalTypeExpected);
      With Expression^ do
        begin
          TypeDefPtr := TypeDefinition;
          IntermediateCodeOffset := 0;
          Location := elConstant;
          Expression^.CheckOrdinalOverflowAndStore (PInteger (Pchar (TypeDefPtr) + FuncParameter)^, False);
          UsedRegisters := [];
        end;
      ExpectTokenAndGetNext (Token_RightParenthesis);
      Exit;
    end;
  Expression^.TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
  With Expression^ do
    begin
      If FuncParameter = 12 then begin
                        Expression^.PositionToOpenParameterHighestIndex;
                        Expression^.EndIntermediateCodeSubroutine;
                        Include (LocationData.Flags, efSegment);
                      end else begin
                                 IntermediateCodeOffset := 0;
                                 Location := elConstant;
                                 Expression^.CheckOrdinalOverflowAndStore (0, False);
                               end;
      UsedRegisters := [];
    end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
This function returns identifier type and
True
if next identifier is type identifier or
False
if identifier is variable.
Function IsTypeIdentifier (Var TypeDefPtr: PTypeDefinition): Boolean;
begin
  CheckForDeclaredIdentifier;
  IsTypeIdentifier := True;
  Case Token of
    Token_TypeIdentifier,
    Token_STRING,
    Token_FILE: TypeDefPtr := ExpectTypeIdentifier;
    else begin
           Expression^.ExpectVariableReferenceExceptProcedureOrFunction;
           TypeDefPtr := Expression^.TypeDefPtr;
           IsTypeIdentifier := False;
         end;
  end;
end;
This function returns
True
if array or string has size 0.
Function OpenArrayOrOpenString (TypeDef: PTypeDefinition): Boolean;
begin
  OpenArrayOrOpenString := False;
  Case TypeDef^.BaseType of
    btArray,
    btString: OpenArrayOrOpenString := TypeDef^.Size = 0;
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
Procedure Func_Low_High; Far;
Var TypeDefinition: PTypeDefinition;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  If IsTypeIdentifier (Expression^.TypeDefPtr) or not OpenArrayOrOpenString (Expression^.TypeDefPtr) then
    begin
      If (Expression^.TypeDefPtr^.BaseType = btArray) or (Expression^.TypeDefPtr^.BaseType = btString) then
        TypeDefinition := PointerFromOffsets (PArrayTypeDefinition (Expression^.TypeDefPtr)^.IndexTypeOffset) else
          TypeDefinition := Expression^.TypeDefPtr;
      If TypeDefinition^.BaseType < btInteger then Error (OrdinalTypeExpected);
      With Expression^ do
        begin
          TypeDefPtr := TypeDefinition;
          IntermediateCodeOffset := 0;
          Location := elConstant;
          Expression^.CheckOrdinalOverflowAndStore (PInteger (Pchar (TypeDefPtr) + FuncParameter)^, False);
          UsedRegisters := [];
        end;
      ExpectTokenAndGetNext (Token_RightParenthesis);
      Exit;
    end;
  Expression^.TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
  With Expression^ do
    begin
      If FuncParameter = 12 then begin
                        Expression^.PositionToOpenParameterHighestIndex;
                        Expression^.EndIntermediateCodeSubroutine;
                        Include (LocationData.Flags, efSegment);
                      end else begin
                                 IntermediateCodeOffset := 0;
                                 Location := elConstant;
                                 Expression^.CheckOrdinalOverflowAndStore (0, False);
                               end;
      UsedRegisters := [];
    end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
```

### Code Block 2

```pascal
Function IsTypeIdentifier (Var TypeDefPtr: PTypeDefinition): Boolean;
begin
  CheckForDeclaredIdentifier;
  IsTypeIdentifier := True;
  Case Token of
    Token_TypeIdentifier,
    Token_STRING,
    Token_FILE: TypeDefPtr := ExpectTypeIdentifier;
    else begin
           Expression^.ExpectVariableReferenceExceptProcedureOrFunction;
           TypeDefPtr := Expression^.TypeDefPtr;
           IsTypeIdentifier := False;
         end;
  end;
end;
```

### Code Block 3

```pascal
Function OpenArrayOrOpenString (TypeDef: PTypeDefinition): Boolean;
begin
  OpenArrayOrOpenString := False;
  Case TypeDef^.BaseType of
    btArray,
    btString: OpenArrayOrOpenString := TypeDef^.Size = 0;
  end;
end;
```

### Code Block 4

```pascal
Procedure Func_Low_High; Far;
Var TypeDefinition: PTypeDefinition;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  If IsTypeIdentifier (Expression^.TypeDefPtr) or not OpenArrayOrOpenString (Expression^.TypeDefPtr) then
    begin
      If (Expression^.TypeDefPtr^.BaseType = btArray) or (Expression^.TypeDefPtr^.BaseType = btString) then
        TypeDefinition := PointerFromOffsets (PArrayTypeDefinition (Expression^.TypeDefPtr)^.IndexTypeOffset) else
          TypeDefinition := Expression^.TypeDefPtr;
      If TypeDefinition^.BaseType < btInteger then Error (OrdinalTypeExpected);
      With Expression^ do
        begin
          TypeDefPtr := TypeDefinition;
          IntermediateCodeOffset := 0;
          Location := elConstant;
          Expression^.CheckOrdinalOverflowAndStore (PInteger (Pchar (TypeDefPtr) + FuncParameter)^, False);
          UsedRegisters := [];
        end;
      ExpectTokenAndGetNext (Token_RightParenthesis);
      Exit;
    end;
  Expression^.TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
  With Expression^ do
    begin
      If FuncParameter = 12 then begin
                        Expression^.PositionToOpenParameterHighestIndex;
                        Expression^.EndIntermediateCodeSubroutine;
                        Include (LocationData.Flags, efSegment);
                      end else begin
                                 IntermediateCodeOffset := 0;
                                 Location := elConstant;
                                 Expression^.CheckOrdinalOverflowAndStore (0, False);
                               end;
      UsedRegisters := [];
    end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
```

### Code Block 5

```pascal
Function IsTypeIdentifier (Var TypeDefPtr: PTypeDefinition): Boolean;
begin
  CheckForDeclaredIdentifier;
  IsTypeIdentifier := True;
  Case Token of
    Token_TypeIdentifier,
    Token_STRING,
    Token_FILE: TypeDefPtr := ExpectTypeIdentifier;
    else begin
           Expression^.ExpectVariableReferenceExceptProcedureOrFunction;
           TypeDefPtr := Expression^.TypeDefPtr;
           IsTypeIdentifier := False;
         end;
  end;
end;
```

### Code Block 6

```pascal
Function OpenArrayOrOpenString (TypeDef: PTypeDefinition): Boolean;
begin
  OpenArrayOrOpenString := False;
  Case TypeDef^.BaseType of
    btArray,
    btString: OpenArrayOrOpenString := TypeDef^.Size = 0;
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
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/system-functions
- https://turbopascal.org/system-functions-low-and-high
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

