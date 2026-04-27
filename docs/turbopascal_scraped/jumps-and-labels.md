# Jumps and Labels - Turbo Pascal Compiler

**URL:** https://turbopascal.org/jumps-and-labels

---

## Table of Contents

- Jumps and Labels

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Jumps and Labels
Turbo Pascal generates jumps in an interesting way. Since most jumps will jump to the same place, Turbo Pascal keeps them in a linked list.
Each near jump (within segment) can be part of a jump list. This procedure creates intermediate code for a jump instruction and adds it to the list.
Procedure GenerateCodeForNearJump (Var JumpRecordList: Word; OpCode: Byte);
Var Saved_PreviousJumpRecord: Word;
begin
  Saved_PreviousJumpRecord := JumpRecordList;
  JumpRecordList := SymbolTable [stIntermediateCode].UsedSize;
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 8))^ do
    begin
      RecordType := icJumpNear;
      JumpOpCode := OpCode;
      PreviousJumpRecord := Saved_PreviousJumpRecord;
    end
end;
Once the jumps are generated, the compiler can generate a label. This procedure generates label intermediate code and sets all jumps in a list to point to it.
Procedure GenerateLabelAndSetJumpsToIt (Var JumpRecordList: Word);
Var LabelRecord: PIntermediateCodeRecord;
    JumpRecord: PIntermediateCodeRecord;
    JumpRecordOfs: Word absolute JumpRecord;
    CurrentJumpRecord: Word;
begin
  CurrentJumpRecord := JumpRecordList;
  JumpRecordList := 0;
  If CurrentJumpRecord = 0 then Exit;
  LabelRecord := IncreaseSymbolTable (stIntermediateCode, 3);
  LabelRecord^.Recordtype := icLabel;
  JumpRecord := Ptr (Seg (LabelRecord^), 0);
  Repeat
    JumpRecordOfs := CurrentJumpRecord;
    CurrentJumpRecord := JumpRecord^.PreviousJumpRecord;
    JumpRecord^.LabelRecordOffset := Ofs (LabelRecord^) + 1;
  until CurrentJumpRecord = 0;
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
Procedure GenerateCodeForNearJump (Var JumpRecordList: Word; OpCode: Byte);
Var Saved_PreviousJumpRecord: Word;
begin
  Saved_PreviousJumpRecord := JumpRecordList;
  JumpRecordList := SymbolTable [stIntermediateCode].UsedSize;
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 8))^ do
    begin
      RecordType := icJumpNear;
      JumpOpCode := OpCode;
      PreviousJumpRecord := Saved_PreviousJumpRecord;
    end
end;
```

### Code Block 2

```pascal
Procedure GenerateLabelAndSetJumpsToIt (Var JumpRecordList: Word);
Var LabelRecord: PIntermediateCodeRecord;
    JumpRecord: PIntermediateCodeRecord;
    JumpRecordOfs: Word absolute JumpRecord;
    CurrentJumpRecord: Word;
begin
  CurrentJumpRecord := JumpRecordList;
  JumpRecordList := 0;
  If CurrentJumpRecord = 0 then Exit;
  LabelRecord := IncreaseSymbolTable (stIntermediateCode, 3);
  LabelRecord^.Recordtype := icLabel;
  JumpRecord := Ptr (Seg (LabelRecord^), 0);
  Repeat
    JumpRecordOfs := CurrentJumpRecord;
    CurrentJumpRecord := JumpRecord^.PreviousJumpRecord;
    JumpRecord^.LabelRecordOffset := Ofs (LabelRecord^) + 1;
  until CurrentJumpRecord = 0;
end;
```

### Code Block 3

```pascal
Procedure GenerateCodeForNearJump (Var JumpRecordList: Word; OpCode: Byte);
Var Saved_PreviousJumpRecord: Word;
begin
  Saved_PreviousJumpRecord := JumpRecordList;
  JumpRecordList := SymbolTable [stIntermediateCode].UsedSize;
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 8))^ do
    begin
      RecordType := icJumpNear;
      JumpOpCode := OpCode;
      PreviousJumpRecord := Saved_PreviousJumpRecord;
    end
end;
```

### Code Block 4

```pascal
Procedure GenerateLabelAndSetJumpsToIt (Var JumpRecordList: Word);
Var LabelRecord: PIntermediateCodeRecord;
    JumpRecord: PIntermediateCodeRecord;
    JumpRecordOfs: Word absolute JumpRecord;
    CurrentJumpRecord: Word;
begin
  CurrentJumpRecord := JumpRecordList;
  JumpRecordList := 0;
  If CurrentJumpRecord = 0 then Exit;
  LabelRecord := IncreaseSymbolTable (stIntermediateCode, 3);
  LabelRecord^.Recordtype := icLabel;
  JumpRecord := Ptr (Seg (LabelRecord^), 0);
  Repeat
    JumpRecordOfs := CurrentJumpRecord;
    CurrentJumpRecord := JumpRecord^.PreviousJumpRecord;
    JumpRecord^.LabelRecordOffset := Ofs (LabelRecord^) + 1;
  until CurrentJumpRecord = 0;
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
- https://turbopascal.org/jumps-and-labels
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

