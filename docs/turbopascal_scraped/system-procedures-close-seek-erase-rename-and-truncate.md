# System Procedures Close, Seek, Erase, Rename and Truncate - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-procedures-close-seek-erase-rename-and-truncate

---

## Table of Contents

- System Procedures Close, Seek, Erase, Rename and Truncate

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Procedures Close, Seek, Erase, Rename and Truncate
These functions process system procedures
Close
,
Seek
,
Erase
and
Rename
.
The first function processes system procedure
Close
.
Function Proc_Close: Word; Far;
Var FileVariable: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  Case FileVariable.TypeDefPtr^.BaseType of
    btFile: GenerateInstruction_CALL_FAR (SysProc_CloseFile);
    else GenerateInstruction_CALL_FAR (SysProc_CloseText);
  end;
  GenerateIOErrorCheckingCode;
  Proc_Close := EndSubroutine;
end;
This function processes system procedure
Seek
.
Function Proc_Seek: Word; Far;
Var FileVariable, Position: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  If FileVariable.TypeDefPtr^.BaseType <>; btFile then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_Comma);
  Position.ExpectIntegerExpression;
  Position.CheckOrdinalRange (Ptr (SystemUnitSegment, LongInt_TypeOffset));
  Position.PushExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  StoreCode_icGoSub (Position.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_SeekFile);
  GenerateIOErrorCheckingCode;
  Proc_Seek := EndSubroutine;
end;
This function processes system procedure
Erase
.
Function Proc_Erase: Word; Far;
Var FileVariable: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_EraseFile);
  GenerateIOErrorCheckingCode;
  Proc_Erase := EndSubroutine;
end;
This function processes system procedure
Rename
.
Function Proc_Rename: Word; Far;
Var FileVariable, NewName: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  ExpectTokenAndGetNext (Token_Comma);
  NewName.ExpectAndStoreStringCompatibleExpressionAndPushPointer;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  StoreCode_icGoSub (NewName.IntermediateCodeOffset);
  Case NewName.TypeDefPtr^.BaseType of
    btString: GenerateInstruction_CALL_FAR (SysProc_RenameFile);
    else GenerateInstruction_CALL_FAR (SysProc_RenameFileC);
  end;
  GenerateIOErrorCheckingCode;
  Proc_Rename := EndSubroutine;
end;
This function processes system procedure
Truncate
.
Function Proc_Truncate: Word; Far;
Var FileVariable: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  If FileVariable.TypeDefPtr^.BaseType <> btFile then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_TruncFile);
  GenerateIOErrorCheckingCode;
  Proc_Truncate := EndSubRoutine;
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
Function Proc_Close: Word; Far;
Var FileVariable: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  Case FileVariable.TypeDefPtr^.BaseType of
    btFile: GenerateInstruction_CALL_FAR (SysProc_CloseFile);
    else GenerateInstruction_CALL_FAR (SysProc_CloseText);
  end;
  GenerateIOErrorCheckingCode;
  Proc_Close := EndSubroutine;
end;
```

### Code Block 2

```pascal
Function Proc_Seek: Word; Far;
Var FileVariable, Position: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  If FileVariable.TypeDefPtr^.BaseType <>; btFile then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_Comma);
  Position.ExpectIntegerExpression;
  Position.CheckOrdinalRange (Ptr (SystemUnitSegment, LongInt_TypeOffset));
  Position.PushExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  StoreCode_icGoSub (Position.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_SeekFile);
  GenerateIOErrorCheckingCode;
  Proc_Seek := EndSubroutine;
end;
```

### Code Block 3

```pascal
Function Proc_Erase: Word; Far;
Var FileVariable: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_EraseFile);
  GenerateIOErrorCheckingCode;
  Proc_Erase := EndSubroutine;
end;
```

### Code Block 4

```pascal
Function Proc_Rename: Word; Far;
Var FileVariable, NewName: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  ExpectTokenAndGetNext (Token_Comma);
  NewName.ExpectAndStoreStringCompatibleExpressionAndPushPointer;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  StoreCode_icGoSub (NewName.IntermediateCodeOffset);
  Case NewName.TypeDefPtr^.BaseType of
    btString: GenerateInstruction_CALL_FAR (SysProc_RenameFile);
    else GenerateInstruction_CALL_FAR (SysProc_RenameFileC);
  end;
  GenerateIOErrorCheckingCode;
  Proc_Rename := EndSubroutine;
end;
```

### Code Block 5

```pascal
Function Proc_Truncate: Word; Far;
Var FileVariable: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  If FileVariable.TypeDefPtr^.BaseType <> btFile then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_TruncFile);
  GenerateIOErrorCheckingCode;
  Proc_Truncate := EndSubRoutine;
end;
```

### Code Block 6

```pascal
Function Proc_Close: Word; Far;
Var FileVariable: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  Case FileVariable.TypeDefPtr^.BaseType of
    btFile: GenerateInstruction_CALL_FAR (SysProc_CloseFile);
    else GenerateInstruction_CALL_FAR (SysProc_CloseText);
  end;
  GenerateIOErrorCheckingCode;
  Proc_Close := EndSubroutine;
end;
```

### Code Block 7

```pascal
Function Proc_Seek: Word; Far;
Var FileVariable, Position: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  If FileVariable.TypeDefPtr^.BaseType <>; btFile then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_Comma);
  Position.ExpectIntegerExpression;
  Position.CheckOrdinalRange (Ptr (SystemUnitSegment, LongInt_TypeOffset));
  Position.PushExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  StoreCode_icGoSub (Position.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_SeekFile);
  GenerateIOErrorCheckingCode;
  Proc_Seek := EndSubroutine;
end;
```

### Code Block 8

```pascal
Function Proc_Erase: Word; Far;
Var FileVariable: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_EraseFile);
  GenerateIOErrorCheckingCode;
  Proc_Erase := EndSubroutine;
end;
```

### Code Block 9

```pascal
Function Proc_Rename: Word; Far;
Var FileVariable, NewName: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  ExpectTokenAndGetNext (Token_Comma);
  NewName.ExpectAndStoreStringCompatibleExpressionAndPushPointer;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  StoreCode_icGoSub (NewName.IntermediateCodeOffset);
  Case NewName.TypeDefPtr^.BaseType of
    btString: GenerateInstruction_CALL_FAR (SysProc_RenameFile);
    else GenerateInstruction_CALL_FAR (SysProc_RenameFileC);
  end;
  GenerateIOErrorCheckingCode;
  Proc_Rename := EndSubroutine;
end;
```

### Code Block 10

```pascal
Function Proc_Truncate: Word; Far;
Var FileVariable: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileVariable.ExpectFileVariableAndPushPointerToMemory;
  If FileVariable.TypeDefPtr^.BaseType <> btFile then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVariable.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_TruncFile);
  GenerateIOErrorCheckingCode;
  Proc_Truncate := EndSubRoutine;
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
- https://turbopascal.org/system-procedures-close-seek-erase-rename-and-truncate
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

