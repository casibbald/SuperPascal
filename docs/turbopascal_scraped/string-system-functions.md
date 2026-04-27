# String System Functions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/string-system-functions

---

## Table of Contents

- String System Functions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
String System Functions
These procedures process system functions
Pos
,
Length
,
Copy
and
Concat
.
Procedure Func_Length; Far;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectStringExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  With Expression^ do If Location = elMemory then Include (LocationData.Flags, efSegment);
  Expression^.PositionToStringLength;
end;

Procedure Func_Pos; Far;
Var TempExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectAndStoreStringAndPushPointer;
  ExpectTokenAndGetNext (Token_Comma);
  TempExpression.ExpectAndStoreStringAndPushPointer;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  Expression^.Calculate;
  StoreCode_icGoSub (TempExpression.Value.Word);
  GenerateInstruction_CALL_FAR (SysProc_SPos);
  With Expression^ do
    begin
      Expression^.EndIntermediateCodeSubroutine;
      Location := elRegister;
      DataType := [itSigned, it16Bit];
      UsedRegisters := [urSP, urDX, urAX];
      LocationData.Register := rDX_AX;
      TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
    end;
end;

Procedure Func_Copy; Far;
Var StartPosExpression, LengthExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectAndStoreStringAndPushPointer;
  ExpectTokenAndGetNext (Token_Comma);
  StartPosExpression.ExpectIntegerExpression;
  StartPosExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Integer_TypeOffset));
  StartPosExpression.PushExpression;
  ExpectTokenAndGetNext (Token_Comma);
  LengthExpression.ExpectIntegerExpression;
  LengthExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Integer_TypeOffset));
  LengthExpression.PushExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  Expression^.CreateSpaceInStackFrameAndPushAddressToIt ($0100);
  Expression^.Calculate;
  StoreCode_icGoSub (StartPosExpression.IntermediateCodeOffset);
  StoreCode_icGoSub (LengthExpression.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_SCopy);
  Expression^.EndIntermediateCodeSubroutine;
  Expression^.UsedRegisters := [urSP, urDX, urAX];
end;

Procedure Func_Concat; Far;
Var TempString: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectStringExpression;
  Expression^.CopyStringToStackFrame ($0100);
  While CheckAndGetNextToken (Token_Comma) do
    begin
      TempString.ExpectStringExpression;
      TempString.StoreStringConstantToCodeBlock;
      Expression^.ConvertToPointerAndPush;
      Expression^.Calculate;
      StoreCode_icGoSub (Expression^.IntermediateCodeOffset);
      GenerateInstruction_CALL_FAR (SysProc_SConcat);
      Expression^.EndIntermediateCodeSubroutine;
      Expression^.UsedRegisters := [urSP, urDX, urAX];
    end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
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
Procedure Func_Length; Far;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectStringExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  With Expression^ do If Location = elMemory then Include (LocationData.Flags, efSegment);
  Expression^.PositionToStringLength;
end;

Procedure Func_Pos; Far;
Var TempExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectAndStoreStringAndPushPointer;
  ExpectTokenAndGetNext (Token_Comma);
  TempExpression.ExpectAndStoreStringAndPushPointer;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  Expression^.Calculate;
  StoreCode_icGoSub (TempExpression.Value.Word);
  GenerateInstruction_CALL_FAR (SysProc_SPos);
  With Expression^ do
    begin
      Expression^.EndIntermediateCodeSubroutine;
      Location := elRegister;
      DataType := [itSigned, it16Bit];
      UsedRegisters := [urSP, urDX, urAX];
      LocationData.Register := rDX_AX;
      TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
    end;
end;

Procedure Func_Copy; Far;
Var StartPosExpression, LengthExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectAndStoreStringAndPushPointer;
  ExpectTokenAndGetNext (Token_Comma);
  StartPosExpression.ExpectIntegerExpression;
  StartPosExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Integer_TypeOffset));
  StartPosExpression.PushExpression;
  ExpectTokenAndGetNext (Token_Comma);
  LengthExpression.ExpectIntegerExpression;
  LengthExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Integer_TypeOffset));
  LengthExpression.PushExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  Expression^.CreateSpaceInStackFrameAndPushAddressToIt ($0100);
  Expression^.Calculate;
  StoreCode_icGoSub (StartPosExpression.IntermediateCodeOffset);
  StoreCode_icGoSub (LengthExpression.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_SCopy);
  Expression^.EndIntermediateCodeSubroutine;
  Expression^.UsedRegisters := [urSP, urDX, urAX];
end;

Procedure Func_Concat; Far;
Var TempString: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectStringExpression;
  Expression^.CopyStringToStackFrame ($0100);
  While CheckAndGetNextToken (Token_Comma) do
    begin
      TempString.ExpectStringExpression;
      TempString.StoreStringConstantToCodeBlock;
      Expression^.ConvertToPointerAndPush;
      Expression^.Calculate;
      StoreCode_icGoSub (Expression^.IntermediateCodeOffset);
      GenerateInstruction_CALL_FAR (SysProc_SConcat);
      Expression^.EndIntermediateCodeSubroutine;
      Expression^.UsedRegisters := [urSP, urDX, urAX];
    end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
```

### Code Block 2

```pascal
Procedure Func_Length; Far;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectStringExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  With Expression^ do If Location = elMemory then Include (LocationData.Flags, efSegment);
  Expression^.PositionToStringLength;
end;

Procedure Func_Pos; Far;
Var TempExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectAndStoreStringAndPushPointer;
  ExpectTokenAndGetNext (Token_Comma);
  TempExpression.ExpectAndStoreStringAndPushPointer;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  Expression^.Calculate;
  StoreCode_icGoSub (TempExpression.Value.Word);
  GenerateInstruction_CALL_FAR (SysProc_SPos);
  With Expression^ do
    begin
      Expression^.EndIntermediateCodeSubroutine;
      Location := elRegister;
      DataType := [itSigned, it16Bit];
      UsedRegisters := [urSP, urDX, urAX];
      LocationData.Register := rDX_AX;
      TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
    end;
end;

Procedure Func_Copy; Far;
Var StartPosExpression, LengthExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectAndStoreStringAndPushPointer;
  ExpectTokenAndGetNext (Token_Comma);
  StartPosExpression.ExpectIntegerExpression;
  StartPosExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Integer_TypeOffset));
  StartPosExpression.PushExpression;
  ExpectTokenAndGetNext (Token_Comma);
  LengthExpression.ExpectIntegerExpression;
  LengthExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Integer_TypeOffset));
  LengthExpression.PushExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  Expression^.CreateSpaceInStackFrameAndPushAddressToIt ($0100);
  Expression^.Calculate;
  StoreCode_icGoSub (StartPosExpression.IntermediateCodeOffset);
  StoreCode_icGoSub (LengthExpression.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_SCopy);
  Expression^.EndIntermediateCodeSubroutine;
  Expression^.UsedRegisters := [urSP, urDX, urAX];
end;

Procedure Func_Concat; Far;
Var TempString: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectStringExpression;
  Expression^.CopyStringToStackFrame ($0100);
  While CheckAndGetNextToken (Token_Comma) do
    begin
      TempString.ExpectStringExpression;
      TempString.StoreStringConstantToCodeBlock;
      Expression^.ConvertToPointerAndPush;
      Expression^.Calculate;
      StoreCode_icGoSub (Expression^.IntermediateCodeOffset);
      GenerateInstruction_CALL_FAR (SysProc_SConcat);
      Expression^.EndIntermediateCodeSubroutine;
      Expression^.UsedRegisters := [urSP, urDX, urAX];
    end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
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
- https://turbopascal.org/string-system-functions
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

