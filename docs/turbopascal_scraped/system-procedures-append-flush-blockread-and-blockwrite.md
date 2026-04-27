# System Procedures Append, Flush, BlockRead and BlockWrite - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-procedures-append-flush-blockread-and-blockwrite

---

## Table of Contents

- System Procedures Append, Flush, BlockRead and BlockWrite

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Procedures Append, Flush, BlockRead and BlockWrite
This function processes system procedures
Append
and
Flush
.
Function Proc_Append_Flush: Word; Far;
Var FileExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileExpression.ExpectFileVariableAndPushPointerToMemory;
  If FileExpression.TypeDefPtr^.BaseType <> btText then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileExpression.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_Append_Flush := EndSubroutine;
end;
This function processes system procedures
BlockRead
and
BlockWrite
.
Function Proc_BlockRead_BlockWrite: Word; Far;
Var FileVariable, Count, Buffer, ErrorVariable: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  If PFileTypeDefinition (FileVariable.TypeDefPtr)^.BaseType <> btFile then Error (InvalidFileType);
  If PFileTypeDefinition (FileVariable.TypeDefPtr)^.BaseFileTypeOffset.TypeOffset <> 0 then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_Comma);
  Buffer.ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  ExpectTokenAndGetNext (Token_Comma);
  Count.ExpectIntegerExpression;
  Count.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  Count.PushExpression;
  If not CheckAndGetNextToken (Token_Comma) then
    begin
      GenerateInstruction_TwoBytes ($31, $C0);
      GenerateInstruction_TwoBytes ($50, $50);
      ErrorVariable.EndIntermediateCodeSubroutine;
    end else ErrorVariable.Expect16BitIntegerVariable;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  StoreCode_icGoSub (Buffer.IntermediateCodeOffset);
  StoreCode_icGoSub (Count.IntermediateCodeOffset);
  StoreCode_icGoSub (ErrorVariable.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_BlockRead_BlockWrite := EndSubroutine;
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
Function Proc_Append_Flush: Word; Far;
Var FileExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileExpression.ExpectFileVariableAndPushPointerToMemory;
  If FileExpression.TypeDefPtr^.BaseType <> btText then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileExpression.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_Append_Flush := EndSubroutine;
end;
```

### Code Block 2

```pascal
Function Proc_BlockRead_BlockWrite: Word; Far;
Var FileVariable, Count, Buffer, ErrorVariable: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  If PFileTypeDefinition (FileVariable.TypeDefPtr)^.BaseType <> btFile then Error (InvalidFileType);
  If PFileTypeDefinition (FileVariable.TypeDefPtr)^.BaseFileTypeOffset.TypeOffset <> 0 then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_Comma);
  Buffer.ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  ExpectTokenAndGetNext (Token_Comma);
  Count.ExpectIntegerExpression;
  Count.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  Count.PushExpression;
  If not CheckAndGetNextToken (Token_Comma) then
    begin
      GenerateInstruction_TwoBytes ($31, $C0);
      GenerateInstruction_TwoBytes ($50, $50);
      ErrorVariable.EndIntermediateCodeSubroutine;
    end else ErrorVariable.Expect16BitIntegerVariable;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  StoreCode_icGoSub (Buffer.IntermediateCodeOffset);
  StoreCode_icGoSub (Count.IntermediateCodeOffset);
  StoreCode_icGoSub (ErrorVariable.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_BlockRead_BlockWrite := EndSubroutine;
end;
```

### Code Block 3

```pascal
Function Proc_Append_Flush: Word; Far;
Var FileExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileExpression.ExpectFileVariableAndPushPointerToMemory;
  If FileExpression.TypeDefPtr^.BaseType <> btText then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileExpression.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_Append_Flush := EndSubroutine;
end;
```

### Code Block 4

```pascal
Function Proc_BlockRead_BlockWrite: Word; Far;
Var FileVariable, Count, Buffer, ErrorVariable: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  If PFileTypeDefinition (FileVariable.TypeDefPtr)^.BaseType <> btFile then Error (InvalidFileType);
  If PFileTypeDefinition (FileVariable.TypeDefPtr)^.BaseFileTypeOffset.TypeOffset <> 0 then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_Comma);
  Buffer.ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  ExpectTokenAndGetNext (Token_Comma);
  Count.ExpectIntegerExpression;
  Count.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  Count.PushExpression;
  If not CheckAndGetNextToken (Token_Comma) then
    begin
      GenerateInstruction_TwoBytes ($31, $C0);
      GenerateInstruction_TwoBytes ($50, $50);
      ErrorVariable.EndIntermediateCodeSubroutine;
    end else ErrorVariable.Expect16BitIntegerVariable;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  StoreCode_icGoSub (Buffer.IntermediateCodeOffset);
  StoreCode_icGoSub (Count.IntermediateCodeOffset);
  StoreCode_icGoSub (ErrorVariable.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_BlockRead_BlockWrite := EndSubroutine;
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
- https://turbopascal.org/system-procedures
- https://turbopascal.org/system-procedures-append-flush-blockread-and-blockwrite
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

