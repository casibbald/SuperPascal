# Processing Typecast or Method Call - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-typecast-or-method-call

---

## Table of Contents

- Processing Typecast or Method Call

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Typecast or Method Call
This procedure processes type identifiers in expression. If a period follows an object type then this is a method call. If left parenthesis follows then this is a typecast.
Procedure TExpression.ProcessValueTypecastOrMethodCall;
Var NewType: PTypeDefinition;
    OldDataType: TBaseType;
    OldSize: Word;

begin
  NewType := ExpectTypeIdentifier;
  If NewType^.BaseType = btObject then If CheckAndGetNextToken (Token_Period) then
    begin
      IntermediateCodeOffset := 0;
      UsedRegisters := [];
      FindAndProcessMethodCall (NewType, [pfMethod], [mcStatic], MethodIdentifierExpected);
      Exit;
    end;
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  If (NewType^.BaseType >= btInteger) or (NewType^.BaseType = btPointer) then
    begin
      CalculateExpression;
      If ExtendedSyntax in ModuleCompilerSwitches then CheckForStringConversionToPChar (@NewType);
    end else ExpectVariableReferenceExceptProcedureOrFunction;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  OldDataType := TypeDefPtr^.BaseType;
  OldSize := TypeDefPtr^.Size;
  TypeDefPtr := NewType;
  If Location = elMemory then
    If (OldDataType = btUntyped) or (OldSize = NewType^.Size) then
      begin
        DataType := NewType^.DataType;
        Exit;
      end;
  If (OldDataType < btInteger) and (OldDataType <> btPointer) then Error (InvalidTypeCast);
  If (NewType^.BaseType < btInteger) and (NewType^.BaseType <> btPointer) then Error (InvalidTypeCast);
  ExtendInteger (NewType^.DataType);
end;
This procedure finds method in object’s domain and sets expression data.
Procedure TExpression.FindAndProcessMethodCall (TempPtr: Pointer; Flags: TProcedureFlagsSet; IntType: TIntegerTypeSet; CompilerError: TCompilerError);
Var IdentifierDataPtr: PProcedureIdentifierData;
    IdentifierPtr: Word;
    FoundToken: TToken;
begin
  If Token <> Token_Identifier then Error (CompilerError);
  If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (TempPtr, PVariableIdentifierData (IdentifierDataPtr), FoundToken, IdentifierPtr) then Error (CompilerError);
  If FoundToken <> Token_ProcedureIdentifier then Error (CompilerError);
  If IdentifierDataPtr^.Flags * Flags = [] then Error (CompilerError);
  TypeDefPtr := @IdentifierDataPtr^.ProcedureTypeDefinition;
  Location := elProcedure;
  DataType := IntType;
  Value.W16 := Ofs (CurrentRecordTypeDef^);
  Value.W18 := Seg (CurrentRecordTypeDef^);
  GetNextToken;
end;
This procedure expects type identifier and returns pointer to
type definition record
.
Function ExpectTypeIdentifier: PTypeDefinition;
begin
  Case Token of
    Token_STRING: ExpectTypeIdentifier := Ptr (SystemUnitSegment, String_TypeOffset);
    Token_FILE:   ExpectTypeIdentifier := Ptr (SystemUnitSegment, File_TypeOffset);
    else begin
           CheckForDeclaredIdentifier;
           If Token <> Token_TypeIdentifier then Error (TypeIdentifierExpected);
           ExpectTypeIdentifier := ProcessTypeIdentifier;
           Exit;
         end;
  end;
  GetNextToken;
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
Procedure TExpression.ProcessValueTypecastOrMethodCall;
Var NewType: PTypeDefinition;
    OldDataType: TBaseType;
    OldSize: Word;

begin
  NewType := ExpectTypeIdentifier;
  If NewType^.BaseType = btObject then If CheckAndGetNextToken (Token_Period) then
    begin
      IntermediateCodeOffset := 0;
      UsedRegisters := [];
      FindAndProcessMethodCall (NewType, [pfMethod], [mcStatic], MethodIdentifierExpected);
      Exit;
    end;
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  If (NewType^.BaseType >= btInteger) or (NewType^.BaseType = btPointer) then
    begin
      CalculateExpression;
      If ExtendedSyntax in ModuleCompilerSwitches then CheckForStringConversionToPChar (@NewType);
    end else ExpectVariableReferenceExceptProcedureOrFunction;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  OldDataType := TypeDefPtr^.BaseType;
  OldSize := TypeDefPtr^.Size;
  TypeDefPtr := NewType;
  If Location = elMemory then
    If (OldDataType = btUntyped) or (OldSize = NewType^.Size) then
      begin
        DataType := NewType^.DataType;
        Exit;
      end;
  If (OldDataType < btInteger) and (OldDataType <> btPointer) then Error (InvalidTypeCast);
  If (NewType^.BaseType < btInteger) and (NewType^.BaseType <> btPointer) then Error (InvalidTypeCast);
  ExtendInteger (NewType^.DataType);
end;
```

### Code Block 2

```pascal
Procedure TExpression.FindAndProcessMethodCall (TempPtr: Pointer; Flags: TProcedureFlagsSet; IntType: TIntegerTypeSet; CompilerError: TCompilerError);
Var IdentifierDataPtr: PProcedureIdentifierData;
    IdentifierPtr: Word;
    FoundToken: TToken;
begin
  If Token <> Token_Identifier then Error (CompilerError);
  If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (TempPtr, PVariableIdentifierData (IdentifierDataPtr), FoundToken, IdentifierPtr) then Error (CompilerError);
  If FoundToken <> Token_ProcedureIdentifier then Error (CompilerError);
  If IdentifierDataPtr^.Flags * Flags = [] then Error (CompilerError);
  TypeDefPtr := @IdentifierDataPtr^.ProcedureTypeDefinition;
  Location := elProcedure;
  DataType := IntType;
  Value.W16 := Ofs (CurrentRecordTypeDef^);
  Value.W18 := Seg (CurrentRecordTypeDef^);
  GetNextToken;
end;
```

### Code Block 3

```pascal
Function ExpectTypeIdentifier: PTypeDefinition;
begin
  Case Token of
    Token_STRING: ExpectTypeIdentifier := Ptr (SystemUnitSegment, String_TypeOffset);
    Token_FILE:   ExpectTypeIdentifier := Ptr (SystemUnitSegment, File_TypeOffset);
    else begin
           CheckForDeclaredIdentifier;
           If Token <> Token_TypeIdentifier then Error (TypeIdentifierExpected);
           ExpectTypeIdentifier := ProcessTypeIdentifier;
           Exit;
         end;
  end;
  GetNextToken;
end;
```

### Code Block 4

```pascal
Procedure TExpression.ProcessValueTypecastOrMethodCall;
Var NewType: PTypeDefinition;
    OldDataType: TBaseType;
    OldSize: Word;

begin
  NewType := ExpectTypeIdentifier;
  If NewType^.BaseType = btObject then If CheckAndGetNextToken (Token_Period) then
    begin
      IntermediateCodeOffset := 0;
      UsedRegisters := [];
      FindAndProcessMethodCall (NewType, [pfMethod], [mcStatic], MethodIdentifierExpected);
      Exit;
    end;
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  If (NewType^.BaseType >= btInteger) or (NewType^.BaseType = btPointer) then
    begin
      CalculateExpression;
      If ExtendedSyntax in ModuleCompilerSwitches then CheckForStringConversionToPChar (@NewType);
    end else ExpectVariableReferenceExceptProcedureOrFunction;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  OldDataType := TypeDefPtr^.BaseType;
  OldSize := TypeDefPtr^.Size;
  TypeDefPtr := NewType;
  If Location = elMemory then
    If (OldDataType = btUntyped) or (OldSize = NewType^.Size) then
      begin
        DataType := NewType^.DataType;
        Exit;
      end;
  If (OldDataType < btInteger) and (OldDataType <> btPointer) then Error (InvalidTypeCast);
  If (NewType^.BaseType < btInteger) and (NewType^.BaseType <> btPointer) then Error (InvalidTypeCast);
  ExtendInteger (NewType^.DataType);
end;
```

### Code Block 5

```pascal
Procedure TExpression.FindAndProcessMethodCall (TempPtr: Pointer; Flags: TProcedureFlagsSet; IntType: TIntegerTypeSet; CompilerError: TCompilerError);
Var IdentifierDataPtr: PProcedureIdentifierData;
    IdentifierPtr: Word;
    FoundToken: TToken;
begin
  If Token <> Token_Identifier then Error (CompilerError);
  If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (TempPtr, PVariableIdentifierData (IdentifierDataPtr), FoundToken, IdentifierPtr) then Error (CompilerError);
  If FoundToken <> Token_ProcedureIdentifier then Error (CompilerError);
  If IdentifierDataPtr^.Flags * Flags = [] then Error (CompilerError);
  TypeDefPtr := @IdentifierDataPtr^.ProcedureTypeDefinition;
  Location := elProcedure;
  DataType := IntType;
  Value.W16 := Ofs (CurrentRecordTypeDef^);
  Value.W18 := Seg (CurrentRecordTypeDef^);
  GetNextToken;
end;
```

### Code Block 6

```pascal
Function ExpectTypeIdentifier: PTypeDefinition;
begin
  Case Token of
    Token_STRING: ExpectTypeIdentifier := Ptr (SystemUnitSegment, String_TypeOffset);
    Token_FILE:   ExpectTypeIdentifier := Ptr (SystemUnitSegment, File_TypeOffset);
    else begin
           CheckForDeclaredIdentifier;
           If Token <> Token_TypeIdentifier then Error (TypeIdentifierExpected);
           ExpectTypeIdentifier := ProcessTypeIdentifier;
           Exit;
         end;
  end;
  GetNextToken;
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
- https://turbopascal.org/processing-typecast-or-method-call
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

