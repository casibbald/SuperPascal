# System Procedures Reset, Rewrite and SetTextBuf - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-procedures-reset-rewrite-and-settextbuf

---

## Table of Contents

- System Procedures Reset, Rewrite and SetTextBuf

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Procedures Reset, Rewrite and SetTextBuf
This function processes system procedures
Reset
and
Rewrite
.
Function Proc_Reset_Rewrite: Word; Far;
Var FileExpression, RecordSizeExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileExpression.ExpectFileVariableAndPushPointerToMemory;
  RecordSizeExpression.IntermediateCodeOffset := 0;
  With PFileTypeDefinition (FileExpression.TypeDefPtr)^ do
    If BaseType <> btText then
      begin
        If BaseFileTypeOffset.TypeOffset <> 0 then
          begin
            GenerateCodeToPushWord (PTypeDefinition (PointerFromOffsets (BaseFileTypeOffset))^.Size);
            RecordSizeExpression.EndIntermediateCodeSubroutine;
          end else begin
                     If CheckAndGetNextToken (Token_Comma) then
                       begin
                         RecordSizeExpression.ExpectIntegerExpression;
                         RecordSizeExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
                         RecordSizeExpression.PushExpression;
                       end else begin
                                  GenerateCodeToPushWord (128);
                                  RecordSizeExpression.EndIntermediateCodeSubroutine;
                                end;
                   end;

      end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileExpression.IntermediateCodeOffset);
  StoreCode_icGoSub (RecordSizeExpression.IntermediateCodeOffset);
  Case FileExpression.TypeDefPtr^.BaseType of
    btFile: GenerateInstruction_CALL_FAR (ProcParameter);
    else GenerateInstruction_CALL_FAR (ProcParameter - $0010);
  end;
  GenerateIOErrorCheckingCode;
  Proc_Reset_Rewrite := EndSubroutine;
end;
This function processes system procedure
SetTextBuf
.
Function Proc_SetTextBuf: Word; Far;
Var TextFile, Buffer, Size: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  TextFile.ExpectFileVariableAndPushPointerToMemory;
  If TextFile.TypeDefPtr^.BaseType <> btText then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_Comma);
  Buffer.ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  With Size do
    If CheckAndGetNextToken (Token_Comma) then
      begin
        ExpectIntegerExpression;
        CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
        PushExpression
      end else begin
                 GenerateCodeToPushWord (Buffer.TypeDefPtr^.Size);
                 EndIntermediateCodeSubroutine;
               end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (TextFile.IntermediateCodeOffset);
  StoreCode_icGoSub (Buffer.IntermediateCodeOffset);
  StoreCode_icGoSub (Size.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_BufferText);
  GenerateIOErrorCheckingCode;
  Proc_SetTextBuf := EndSubroutine;
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
Function Proc_Reset_Rewrite: Word; Far;
Var FileExpression, RecordSizeExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileExpression.ExpectFileVariableAndPushPointerToMemory;
  RecordSizeExpression.IntermediateCodeOffset := 0;
  With PFileTypeDefinition (FileExpression.TypeDefPtr)^ do
    If BaseType <> btText then
      begin
        If BaseFileTypeOffset.TypeOffset <> 0 then
          begin
            GenerateCodeToPushWord (PTypeDefinition (PointerFromOffsets (BaseFileTypeOffset))^.Size);
            RecordSizeExpression.EndIntermediateCodeSubroutine;
          end else begin
                     If CheckAndGetNextToken (Token_Comma) then
                       begin
                         RecordSizeExpression.ExpectIntegerExpression;
                         RecordSizeExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
                         RecordSizeExpression.PushExpression;
                       end else begin
                                  GenerateCodeToPushWord (128);
                                  RecordSizeExpression.EndIntermediateCodeSubroutine;
                                end;
                   end;

      end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileExpression.IntermediateCodeOffset);
  StoreCode_icGoSub (RecordSizeExpression.IntermediateCodeOffset);
  Case FileExpression.TypeDefPtr^.BaseType of
    btFile: GenerateInstruction_CALL_FAR (ProcParameter);
    else GenerateInstruction_CALL_FAR (ProcParameter - $0010);
  end;
  GenerateIOErrorCheckingCode;
  Proc_Reset_Rewrite := EndSubroutine;
end;
```

### Code Block 2

```pascal
Function Proc_SetTextBuf: Word; Far;
Var TextFile, Buffer, Size: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  TextFile.ExpectFileVariableAndPushPointerToMemory;
  If TextFile.TypeDefPtr^.BaseType <> btText then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_Comma);
  Buffer.ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  With Size do
    If CheckAndGetNextToken (Token_Comma) then
      begin
        ExpectIntegerExpression;
        CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
        PushExpression
      end else begin
                 GenerateCodeToPushWord (Buffer.TypeDefPtr^.Size);
                 EndIntermediateCodeSubroutine;
               end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (TextFile.IntermediateCodeOffset);
  StoreCode_icGoSub (Buffer.IntermediateCodeOffset);
  StoreCode_icGoSub (Size.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_BufferText);
  GenerateIOErrorCheckingCode;
  Proc_SetTextBuf := EndSubroutine;
end;
```

### Code Block 3

```pascal
Function Proc_Reset_Rewrite: Word; Far;
Var FileExpression, RecordSizeExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  FileExpression.ExpectFileVariableAndPushPointerToMemory;
  RecordSizeExpression.IntermediateCodeOffset := 0;
  With PFileTypeDefinition (FileExpression.TypeDefPtr)^ do
    If BaseType <> btText then
      begin
        If BaseFileTypeOffset.TypeOffset <> 0 then
          begin
            GenerateCodeToPushWord (PTypeDefinition (PointerFromOffsets (BaseFileTypeOffset))^.Size);
            RecordSizeExpression.EndIntermediateCodeSubroutine;
          end else begin
                     If CheckAndGetNextToken (Token_Comma) then
                       begin
                         RecordSizeExpression.ExpectIntegerExpression;
                         RecordSizeExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
                         RecordSizeExpression.PushExpression;
                       end else begin
                                  GenerateCodeToPushWord (128);
                                  RecordSizeExpression.EndIntermediateCodeSubroutine;
                                end;
                   end;

      end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileExpression.IntermediateCodeOffset);
  StoreCode_icGoSub (RecordSizeExpression.IntermediateCodeOffset);
  Case FileExpression.TypeDefPtr^.BaseType of
    btFile: GenerateInstruction_CALL_FAR (ProcParameter);
    else GenerateInstruction_CALL_FAR (ProcParameter - $0010);
  end;
  GenerateIOErrorCheckingCode;
  Proc_Reset_Rewrite := EndSubroutine;
end;
```

### Code Block 4

```pascal
Function Proc_SetTextBuf: Word; Far;
Var TextFile, Buffer, Size: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  TextFile.ExpectFileVariableAndPushPointerToMemory;
  If TextFile.TypeDefPtr^.BaseType <> btText then Error (InvalidFileType);
  ExpectTokenAndGetNext (Token_Comma);
  Buffer.ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  With Size do
    If CheckAndGetNextToken (Token_Comma) then
      begin
        ExpectIntegerExpression;
        CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
        PushExpression
      end else begin
                 GenerateCodeToPushWord (Buffer.TypeDefPtr^.Size);
                 EndIntermediateCodeSubroutine;
               end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (TextFile.IntermediateCodeOffset);
  StoreCode_icGoSub (Buffer.IntermediateCodeOffset);
  StoreCode_icGoSub (Size.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (SysProc_BufferText);
  GenerateIOErrorCheckingCode;
  Proc_SetTextBuf := EndSubroutine;
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
- https://turbopascal.org/system-procedures-reset-rewrite-and-settextbuf
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

