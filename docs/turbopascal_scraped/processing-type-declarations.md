# Processing Type Declarations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-type-declarations

---

## Table of Contents

- Processing Type Declarations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Type Declarations
This procedure processes type declarations – identifiers that denote types. Object types have their
own processing
.
Procedure ProcessTypeDeclarations;
Var TypeIdentifier: PIdentifier;
    TypeIdentifierData: PTypeIdentifierData;
    CurrentType: PTypeDefinition;

  Function ProcessTypeDeclaration: PTypeDefinition;
  begin
    Case Token of
      Token_Object: ProcessTypeDeclaration := ProcessObjectTypeDeclaration (TypeIdentifier);
      else          ProcessTypeDeclaration := ProcessNonObjectTypeDeclaration;
    end;
  end;

begin
  GetNextToken;
  LastPointerToTypeDefinitionOffset := 0;
  Repeat
    TypeIdentifier := ExpectAndStoreIdentifier (SizeOf (TTypeIdentifierData), Pointer (TypeIdentifierData));
    ExpectTokenAndGetNext (Token_Equal);
    CheckFor_OBJECT_PROCEDURE_FUNCTION_Type (TypeIdentifier, TypeIdentifierData^.UnitTypeOffsets);
    CurrentType := ProcessTypeDeclaration;
    If (Seg (CurrentType^) = MainSymbolTable.Segment) and (CurrentType^.TypeIdentifierOffset = 0) then
      CurrentType^.TypeIdentifierOffset := Ofs (TypeIdentifier^);
    GetTypeAndUnitIdentifierOffsets (CurrentType, TypeIdentifierData^.UnitTypeOffsets);
    TypeIdentifier^.Token := Token_TypeIdentifier;
    ExpectTokenAndGetNext (TOKEN_Semicolon);
  until Token <> TOKEN_Identifier;
  ResolvePointerToTypeDefinitions;
end;
Pascal allows to declare inside type definition statement pointers to types that will be declared later. This is useful in Object and Record types which may include pointers to themselves. Such pointer types are first stored into temporary table for later processing. This procedure resolves pointers to forward type declarations.
Procedure ResolvePointerToTypeDefinitions;
Var TempToken: TToken;
    BaseTypeIdDataPtr: PTypeIdentifierData;
    DataOfs: Word;
    PointerTypeDefinition: PPointerTypeDefinition;
begin
  While LastPointerToTypeDefinitionOffset <> 0 do
    begin
      PointerTypeDefinition := Ptr (MainSymbolTable.Segment, LastPointerToTypeDefinitionOffset);
      CopyStringFromTemporaryBlockToCurrentIdentifier (
        PointerTypeDefinition^.PointerBaseTypeOffset.TemporarySymbolTableRecordOffset);
      If not FindCurrentIdentifier (TempToken, DataOfs, Pointer (BaseTypeIdDataPtr)) then
        CurrentIdentifierError (UndefinedTypeInPointerDefinition);
      If TempToken <> Token_TypeIdentifier then CurrentIdentifierError (UndefinedTypeInPointerDefinition);
      LastPointerToTypeDefinitionOffset := PointerTypeDefinition^.PointerBaseTypeOffset.PreviousPointerToTypeDefinitionOffset;
      GetTypeAndUnitIdentifierOffsets (PointerFromOffsets (BaseTypeIdDataPtr^.UnitTypeOffsets),
                                       PointerTypeDefinition^.PointerBaseTypeOffset);
    end;
end;

Procedure CheckFor_OBJECT_PROCEDURE_FUNCTION_Type (TypeId: PIdentifier; Var UnitTypeOffsets: TUnitOffsets);
begin
  Case Token of
    Token_OBJECT,
    Token_PROCEDURE,
    Token_FUNCTION: begin
                      TypeId^.Token := Token_OBJECT_PROCEDURE_FUNCTION_TypeIdentifier;
                      GetTypeAndUnitIdentifierOffsets (SymbolTable [stMain].Ptr, UnitTypeOffsets);
                    end;
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
Procedure ProcessTypeDeclarations;
Var TypeIdentifier: PIdentifier;
    TypeIdentifierData: PTypeIdentifierData;
    CurrentType: PTypeDefinition;

  Function ProcessTypeDeclaration: PTypeDefinition;
  begin
    Case Token of
      Token_Object: ProcessTypeDeclaration := ProcessObjectTypeDeclaration (TypeIdentifier);
      else          ProcessTypeDeclaration := ProcessNonObjectTypeDeclaration;
    end;
  end;

begin
  GetNextToken;
  LastPointerToTypeDefinitionOffset := 0;
  Repeat
    TypeIdentifier := ExpectAndStoreIdentifier (SizeOf (TTypeIdentifierData), Pointer (TypeIdentifierData));
    ExpectTokenAndGetNext (Token_Equal);
    CheckFor_OBJECT_PROCEDURE_FUNCTION_Type (TypeIdentifier, TypeIdentifierData^.UnitTypeOffsets);
    CurrentType := ProcessTypeDeclaration;
    If (Seg (CurrentType^) = MainSymbolTable.Segment) and (CurrentType^.TypeIdentifierOffset = 0) then
      CurrentType^.TypeIdentifierOffset := Ofs (TypeIdentifier^);
    GetTypeAndUnitIdentifierOffsets (CurrentType, TypeIdentifierData^.UnitTypeOffsets);
    TypeIdentifier^.Token := Token_TypeIdentifier;
    ExpectTokenAndGetNext (TOKEN_Semicolon);
  until Token <> TOKEN_Identifier;
  ResolvePointerToTypeDefinitions;
end;
```

### Code Block 2

```pascal
Procedure ResolvePointerToTypeDefinitions;
Var TempToken: TToken;
    BaseTypeIdDataPtr: PTypeIdentifierData;
    DataOfs: Word;
    PointerTypeDefinition: PPointerTypeDefinition;
begin
  While LastPointerToTypeDefinitionOffset <> 0 do
    begin
      PointerTypeDefinition := Ptr (MainSymbolTable.Segment, LastPointerToTypeDefinitionOffset);
      CopyStringFromTemporaryBlockToCurrentIdentifier (
        PointerTypeDefinition^.PointerBaseTypeOffset.TemporarySymbolTableRecordOffset);
      If not FindCurrentIdentifier (TempToken, DataOfs, Pointer (BaseTypeIdDataPtr)) then
        CurrentIdentifierError (UndefinedTypeInPointerDefinition);
      If TempToken <> Token_TypeIdentifier then CurrentIdentifierError (UndefinedTypeInPointerDefinition);
      LastPointerToTypeDefinitionOffset := PointerTypeDefinition^.PointerBaseTypeOffset.PreviousPointerToTypeDefinitionOffset;
      GetTypeAndUnitIdentifierOffsets (PointerFromOffsets (BaseTypeIdDataPtr^.UnitTypeOffsets),
                                       PointerTypeDefinition^.PointerBaseTypeOffset);
    end;
end;

Procedure CheckFor_OBJECT_PROCEDURE_FUNCTION_Type (TypeId: PIdentifier; Var UnitTypeOffsets: TUnitOffsets);
begin
  Case Token of
    Token_OBJECT,
    Token_PROCEDURE,
    Token_FUNCTION: begin
                      TypeId^.Token := Token_OBJECT_PROCEDURE_FUNCTION_TypeIdentifier;
                      GetTypeAndUnitIdentifierOffsets (SymbolTable [stMain].Ptr, UnitTypeOffsets);
                    end;
  end;
end;
```

### Code Block 3

```pascal
Procedure ProcessTypeDeclarations;
Var TypeIdentifier: PIdentifier;
    TypeIdentifierData: PTypeIdentifierData;
    CurrentType: PTypeDefinition;

  Function ProcessTypeDeclaration: PTypeDefinition;
  begin
    Case Token of
      Token_Object: ProcessTypeDeclaration := ProcessObjectTypeDeclaration (TypeIdentifier);
      else          ProcessTypeDeclaration := ProcessNonObjectTypeDeclaration;
    end;
  end;

begin
  GetNextToken;
  LastPointerToTypeDefinitionOffset := 0;
  Repeat
    TypeIdentifier := ExpectAndStoreIdentifier (SizeOf (TTypeIdentifierData), Pointer (TypeIdentifierData));
    ExpectTokenAndGetNext (Token_Equal);
    CheckFor_OBJECT_PROCEDURE_FUNCTION_Type (TypeIdentifier, TypeIdentifierData^.UnitTypeOffsets);
    CurrentType := ProcessTypeDeclaration;
    If (Seg (CurrentType^) = MainSymbolTable.Segment) and (CurrentType^.TypeIdentifierOffset = 0) then
      CurrentType^.TypeIdentifierOffset := Ofs (TypeIdentifier^);
    GetTypeAndUnitIdentifierOffsets (CurrentType, TypeIdentifierData^.UnitTypeOffsets);
    TypeIdentifier^.Token := Token_TypeIdentifier;
    ExpectTokenAndGetNext (TOKEN_Semicolon);
  until Token <> TOKEN_Identifier;
  ResolvePointerToTypeDefinitions;
end;
```

### Code Block 4

```pascal
Procedure ResolvePointerToTypeDefinitions;
Var TempToken: TToken;
    BaseTypeIdDataPtr: PTypeIdentifierData;
    DataOfs: Word;
    PointerTypeDefinition: PPointerTypeDefinition;
begin
  While LastPointerToTypeDefinitionOffset <> 0 do
    begin
      PointerTypeDefinition := Ptr (MainSymbolTable.Segment, LastPointerToTypeDefinitionOffset);
      CopyStringFromTemporaryBlockToCurrentIdentifier (
        PointerTypeDefinition^.PointerBaseTypeOffset.TemporarySymbolTableRecordOffset);
      If not FindCurrentIdentifier (TempToken, DataOfs, Pointer (BaseTypeIdDataPtr)) then
        CurrentIdentifierError (UndefinedTypeInPointerDefinition);
      If TempToken <> Token_TypeIdentifier then CurrentIdentifierError (UndefinedTypeInPointerDefinition);
      LastPointerToTypeDefinitionOffset := PointerTypeDefinition^.PointerBaseTypeOffset.PreviousPointerToTypeDefinitionOffset;
      GetTypeAndUnitIdentifierOffsets (PointerFromOffsets (BaseTypeIdDataPtr^.UnitTypeOffsets),
                                       PointerTypeDefinition^.PointerBaseTypeOffset);
    end;
end;

Procedure CheckFor_OBJECT_PROCEDURE_FUNCTION_Type (TypeId: PIdentifier; Var UnitTypeOffsets: TUnitOffsets);
begin
  Case Token of
    Token_OBJECT,
    Token_PROCEDURE,
    Token_FUNCTION: begin
                      TypeId^.Token := Token_OBJECT_PROCEDURE_FUNCTION_TypeIdentifier;
                      GetTypeAndUnitIdentifierOffsets (SymbolTable [stMain].Ptr, UnitTypeOffsets);
                    end;
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
- https://turbopascal.org/processing-object-types
- https://turbopascal.org/processing-type-declarations
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

