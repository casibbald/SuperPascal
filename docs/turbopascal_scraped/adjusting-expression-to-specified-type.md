# Adjusting Expression To Specified Type - Turbo Pascal Compiler

**URL:** https://turbopascal.org/adjusting-expression-to-specified-type

---

## Table of Contents

- Adjusting Expression To Specified Type

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Adjusting Expression To Specified Type
This procedure converts expression between compatible types. For example,
Integer
values can be assigned to
Real
variables, only the expression and type need to be converted.
Procedure TExpression.AdjustExpressionToType (TypeDef: PTypeDefinition);
Var NewTypeDefinition: PTypeDefinition;
begin
  Case TypeDef^.BaseType of
    btString: begin
                ConvertCharToString;
                CopyCharArrayCompatibleWithStringToStackFrameAsString;
              end;
    btExtended: begin
                  ConvertIntegerToExtended;
                  ConvertRealToExtended;
                end;
    btReal: begin
              ConvertIntegerToReal;
              ConvertExtendedToReal;
            end;
    btBoolean: If not (it32Bytes in TypeDef^.DataType) then ConvertToBooleanByte;
    btObject: begin
                NewTypeDefinition := TypeDef;
                If TypeDefPtr^.BaseType <> btObject then Exit;
                If InObjectTypeDomain (PObjectTypeDefinition (TypeDefPtr), PObjectTypeDefinition (NewTypeDefinition)) then
                  TypeDefPtr := TypeDef;
              end;
    btPointer: begin
                 If ExtendedSyntax in ModuleCompilerSwitches then
                   begin
                     CheckForStringConversionToPChar (TypeDef);
                     CheckForConversionOfZeroBasedCharacterArrayToPChar (TypeDef);
                   end;
                 NewTypeDefinition := PointerFromOffsets (PPointerTypeDefinition (TypeDef)^.PointerBaseTypeOffset);
                 If NewTypeDefinition^.BaseType <> btObject then Exit;
                 If TypeDefPtr^.BaseType <> btPointer then Exit;
                 If InObjectTypeDomain (PObjectTypeDefinition (
                             PointerFromOffsets (PPointerTypeDefinition (TypeDefPtr)^.PointerBaseTypeOffset)),
                             PObjectTypeDefinition (NewTypeDefinition)) then
                   TypeDefPtr := TypeDef;
               end;
  end;
end;
This procedure checks if
ExpressionType
is compatible with (assignable to)
ObjectTypeDefinition
.
Function InObjectTypeDomain (ExpressionType, ObjectTypeDefinition: PObjectTypeDefinition): Boolean;
begin
  InObjectTypeDomain := False;
  If ExpressionType^.BaseType <> btObject then Exit;
  Repeat
    If ExpressionType = ObjectTypeDefinition then
      begin
        InObjectTypeDomain := True;
        Exit;
      end;
    If ExpressionType^.AncestorTypeOffset.TypeOffset = 0 then Exit;
    ExpressionType := PObjectTypeDefinition (PointerFromOffsets (ExpressionType^.AncestorTypeOffset));
  until False;
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
Procedure TExpression.AdjustExpressionToType (TypeDef: PTypeDefinition);
Var NewTypeDefinition: PTypeDefinition;
begin
  Case TypeDef^.BaseType of
    btString: begin
                ConvertCharToString;
                CopyCharArrayCompatibleWithStringToStackFrameAsString;
              end;
    btExtended: begin
                  ConvertIntegerToExtended;
                  ConvertRealToExtended;
                end;
    btReal: begin
              ConvertIntegerToReal;
              ConvertExtendedToReal;
            end;
    btBoolean: If not (it32Bytes in TypeDef^.DataType) then ConvertToBooleanByte;
    btObject: begin
                NewTypeDefinition := TypeDef;
                If TypeDefPtr^.BaseType <> btObject then Exit;
                If InObjectTypeDomain (PObjectTypeDefinition (TypeDefPtr), PObjectTypeDefinition (NewTypeDefinition)) then
                  TypeDefPtr := TypeDef;
              end;
    btPointer: begin
                 If ExtendedSyntax in ModuleCompilerSwitches then
                   begin
                     CheckForStringConversionToPChar (TypeDef);
                     CheckForConversionOfZeroBasedCharacterArrayToPChar (TypeDef);
                   end;
                 NewTypeDefinition := PointerFromOffsets (PPointerTypeDefinition (TypeDef)^.PointerBaseTypeOffset);
                 If NewTypeDefinition^.BaseType <> btObject then Exit;
                 If TypeDefPtr^.BaseType <> btPointer then Exit;
                 If InObjectTypeDomain (PObjectTypeDefinition (
                             PointerFromOffsets (PPointerTypeDefinition (TypeDefPtr)^.PointerBaseTypeOffset)),
                             PObjectTypeDefinition (NewTypeDefinition)) then
                   TypeDefPtr := TypeDef;
               end;
  end;
end;
```

### Code Block 2

```pascal
Function InObjectTypeDomain (ExpressionType, ObjectTypeDefinition: PObjectTypeDefinition): Boolean;
begin
  InObjectTypeDomain := False;
  If ExpressionType^.BaseType <> btObject then Exit;
  Repeat
    If ExpressionType = ObjectTypeDefinition then
      begin
        InObjectTypeDomain := True;
        Exit;
      end;
    If ExpressionType^.AncestorTypeOffset.TypeOffset = 0 then Exit;
    ExpressionType := PObjectTypeDefinition (PointerFromOffsets (ExpressionType^.AncestorTypeOffset));
  until False;
end;
```

### Code Block 3

```pascal
Procedure TExpression.AdjustExpressionToType (TypeDef: PTypeDefinition);
Var NewTypeDefinition: PTypeDefinition;
begin
  Case TypeDef^.BaseType of
    btString: begin
                ConvertCharToString;
                CopyCharArrayCompatibleWithStringToStackFrameAsString;
              end;
    btExtended: begin
                  ConvertIntegerToExtended;
                  ConvertRealToExtended;
                end;
    btReal: begin
              ConvertIntegerToReal;
              ConvertExtendedToReal;
            end;
    btBoolean: If not (it32Bytes in TypeDef^.DataType) then ConvertToBooleanByte;
    btObject: begin
                NewTypeDefinition := TypeDef;
                If TypeDefPtr^.BaseType <> btObject then Exit;
                If InObjectTypeDomain (PObjectTypeDefinition (TypeDefPtr), PObjectTypeDefinition (NewTypeDefinition)) then
                  TypeDefPtr := TypeDef;
              end;
    btPointer: begin
                 If ExtendedSyntax in ModuleCompilerSwitches then
                   begin
                     CheckForStringConversionToPChar (TypeDef);
                     CheckForConversionOfZeroBasedCharacterArrayToPChar (TypeDef);
                   end;
                 NewTypeDefinition := PointerFromOffsets (PPointerTypeDefinition (TypeDef)^.PointerBaseTypeOffset);
                 If NewTypeDefinition^.BaseType <> btObject then Exit;
                 If TypeDefPtr^.BaseType <> btPointer then Exit;
                 If InObjectTypeDomain (PObjectTypeDefinition (
                             PointerFromOffsets (PPointerTypeDefinition (TypeDefPtr)^.PointerBaseTypeOffset)),
                             PObjectTypeDefinition (NewTypeDefinition)) then
                   TypeDefPtr := TypeDef;
               end;
  end;
end;
```

### Code Block 4

```pascal
Function InObjectTypeDomain (ExpressionType, ObjectTypeDefinition: PObjectTypeDefinition): Boolean;
begin
  InObjectTypeDomain := False;
  If ExpressionType^.BaseType <> btObject then Exit;
  Repeat
    If ExpressionType = ObjectTypeDefinition then
      begin
        InObjectTypeDomain := True;
        Exit;
      end;
    If ExpressionType^.AncestorTypeOffset.TypeOffset = 0 then Exit;
    ExpressionType := PObjectTypeDefinition (PointerFromOffsets (ExpressionType^.AncestorTypeOffset));
  until False;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/adjusting-expression-to-specified-type
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

