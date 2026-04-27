# System Function New - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-function-new

---

## Table of Contents

- System Function New

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Function New
This procedure processes system function
New
. It uses a common procedure for
New
as function and procedure.
Procedure Func_New; Far;
Var TypeDefPtr: PTypeDefinition;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  TypeDefPtr := ExpectTypeIdentifier;
  If TypeDefPtr^.BaseType <> btPointer then Error (PointerTypeIdentifierExpected);
  Expression^.Process_NEW;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;

begin
  Saved_TypeDefPtr := TypeDefPtr;
  PointerBaseType := PointerFromOffsets (PPointerTypeDefinition (TypeDefPtr)^.PointerBaseTypeOffset);
  If (PointerBaseType^.BaseType = btObject) and (CheckAndGetNextToken (Token_Comma)) then
    begin
      IntermediateCodeOffset := 0;
      UsedRegisters := [];
      FindAndProcessMethodCall (PointerBaseType, [pfConstructor], [mcNew], ConstructorIdentifierExpected);
      CallProcedure;
    end else begin
               GenerateCodeToPushWord (PointerBaseType^.Size);
               GenerateInstruction_CALL_FAR (SysProc_NewPtr);
               EndIntermediateCodeSubroutine;
             end;
  TypeDefPtr := Saved_TypeDefPtr;
  Location := elRegister;
  DataType := itLongInt;
  UsedRegisters := [urBX, urDX, urCX, urAX];
  LocationData.Register := rDX_AX;
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
Procedure Func_New; Far;
Var TypeDefPtr: PTypeDefinition;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  TypeDefPtr := ExpectTypeIdentifier;
  If TypeDefPtr^.BaseType <> btPointer then Error (PointerTypeIdentifierExpected);
  Expression^.Process_NEW;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;

begin
  Saved_TypeDefPtr := TypeDefPtr;
  PointerBaseType := PointerFromOffsets (PPointerTypeDefinition (TypeDefPtr)^.PointerBaseTypeOffset);
  If (PointerBaseType^.BaseType = btObject) and (CheckAndGetNextToken (Token_Comma)) then
    begin
      IntermediateCodeOffset := 0;
      UsedRegisters := [];
      FindAndProcessMethodCall (PointerBaseType, [pfConstructor], [mcNew], ConstructorIdentifierExpected);
      CallProcedure;
    end else begin
               GenerateCodeToPushWord (PointerBaseType^.Size);
               GenerateInstruction_CALL_FAR (SysProc_NewPtr);
               EndIntermediateCodeSubroutine;
             end;
  TypeDefPtr := Saved_TypeDefPtr;
  Location := elRegister;
  DataType := itLongInt;
  UsedRegisters := [urBX, urDX, urCX, urAX];
  LocationData.Register := rDX_AX;
end;
```

### Code Block 2

```pascal
Procedure Func_New; Far;
Var TypeDefPtr: PTypeDefinition;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  TypeDefPtr := ExpectTypeIdentifier;
  If TypeDefPtr^.BaseType <> btPointer then Error (PointerTypeIdentifierExpected);
  Expression^.Process_NEW;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;

begin
  Saved_TypeDefPtr := TypeDefPtr;
  PointerBaseType := PointerFromOffsets (PPointerTypeDefinition (TypeDefPtr)^.PointerBaseTypeOffset);
  If (PointerBaseType^.BaseType = btObject) and (CheckAndGetNextToken (Token_Comma)) then
    begin
      IntermediateCodeOffset := 0;
      UsedRegisters := [];
      FindAndProcessMethodCall (PointerBaseType, [pfConstructor], [mcNew], ConstructorIdentifierExpected);
      CallProcedure;
    end else begin
               GenerateCodeToPushWord (PointerBaseType^.Size);
               GenerateInstruction_CALL_FAR (SysProc_NewPtr);
               EndIntermediateCodeSubroutine;
             end;
  TypeDefPtr := Saved_TypeDefPtr;
  Location := elRegister;
  DataType := itLongInt;
  UsedRegisters := [urBX, urDX, urCX, urAX];
  LocationData.Register := rDX_AX;
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
- https://turbopascal.org/system-function-new
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

