# System Procedures GetDir, ChDir, MkDir and RmDir - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-procedures-getdir-chdir-mkdir-and-rmdir

---

## Table of Contents

- System Procedures GetDir, ChDir, MkDir and RmDir

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Procedures GetDir, ChDir, MkDir and RmDir
This function processes system procedure
GetDir
.
Function Proc_GetDir: Word; Far;
Var DriveExpression, DirExpression: Texpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  With DriveExpression do
    begin
      ExpectIntegerExpression;
      CheckOrdinalRange (Ptr (SystemUnitSegment, Byte_TypeOffset));
      PushExpression
    end;
  ExpectTokenAndGetNext (Token_Comma);
  DirExpression.ExpectStringVariable;
  DirExpression.PushArrayPointerAndHighestIndex;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (DriveExpression.IntermediateCodeOffset);
  StoreCode_icGoSub (DirExpression.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_DirGet);
  Proc_GetDir := EndSubroutine;
end;
This function processes system procedures
ChDir
,
MkDir
and
RmDir
.
Function Proc_ChDir_MkDir_RmDir: Word; Far;
Var DirExpression: Texpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  DirExpression.ExpectAndStoreStringAndPushPointer;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  DirExpression.Calculate;
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_ChDir_MkDir_RmDir := EndSubroutine;
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
Function Proc_GetDir: Word; Far;
Var DriveExpression, DirExpression: Texpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  With DriveExpression do
    begin
      ExpectIntegerExpression;
      CheckOrdinalRange (Ptr (SystemUnitSegment, Byte_TypeOffset));
      PushExpression
    end;
  ExpectTokenAndGetNext (Token_Comma);
  DirExpression.ExpectStringVariable;
  DirExpression.PushArrayPointerAndHighestIndex;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (DriveExpression.IntermediateCodeOffset);
  StoreCode_icGoSub (DirExpression.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_DirGet);
  Proc_GetDir := EndSubroutine;
end;
```

### Code Block 2

```pascal
Function Proc_ChDir_MkDir_RmDir: Word; Far;
Var DirExpression: Texpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  DirExpression.ExpectAndStoreStringAndPushPointer;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  DirExpression.Calculate;
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_ChDir_MkDir_RmDir := EndSubroutine;
end;
```

### Code Block 3

```pascal
Function Proc_GetDir: Word; Far;
Var DriveExpression, DirExpression: Texpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  With DriveExpression do
    begin
      ExpectIntegerExpression;
      CheckOrdinalRange (Ptr (SystemUnitSegment, Byte_TypeOffset));
      PushExpression
    end;
  ExpectTokenAndGetNext (Token_Comma);
  DirExpression.ExpectStringVariable;
  DirExpression.PushArrayPointerAndHighestIndex;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (DriveExpression.IntermediateCodeOffset);
  StoreCode_icGoSub (DirExpression.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_DirGet);
  Proc_GetDir := EndSubroutine;
end;
```

### Code Block 4

```pascal
Function Proc_ChDir_MkDir_RmDir: Word; Far;
Var DirExpression: Texpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  DirExpression.ExpectAndStoreStringAndPushPointer;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  DirExpression.Calculate;
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_ChDir_MkDir_RmDir := EndSubroutine;
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
- https://turbopascal.org/system-procedures-getdir-chdir-mkdir-and-rmdir
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

