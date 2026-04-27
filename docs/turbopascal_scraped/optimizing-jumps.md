# Optimizing Jumps - Turbo Pascal Compiler

**URL:** https://turbopascal.org/optimizing-jumps

---

## Table of Contents

- Optimizing Jumps

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Optimizing Jumps
Turbo Pascal tries to convert all near jumps to short. This is only possible if jump displacement can be stored in a byte. This procedure is called just before the
target code is generated
. Since all changes reduce code size and therefore provide new opportunities for optimizations, the function is called in a loop until there are no more changes.
Function ConvertNearJumpsToShort: Boolean;
  Var JumpRecord: PIntermediateCodeRecord;
      JumpRecordOfs: Word absolute JumpRecord;
      LabelRecord: PIntermediateCodeRecord;
      LabelRecordOfs: Word absolute LabelRecord;
      JumpDisplacement: Integer;
      NumberOfConvertedJumps: Word;
  begin
    NumberOfConvertedJumps := 0;
    JumpRecord := Ptr (SymbolTable [stIntermediateCode].Segment, Last_icJumpNear);
    LabelRecord := Ptr (SymbolTable [stIntermediateCode].Segment, 0);
    While JumpRecordOfs <> 0 do
      With JumpRecord^ do
        begin
          LabelRecordOfs := LabelRecordOffset - 1;
          JumpDisplacement := LabelRecord^.LabelAddress - JumpCodeOffset;
          If (JumpDisplacement <= 129) and (JumpDisplacement >= - 126) then
            begin
              RecordType := icJumpShort;
              Inc (NumberOfConvertedJumps);
            end;
          JumpRecordOfs := Previous_icJumpNear;
        end;
    ConvertNearJumpsToShort := NumberOfConvertedJumps <> 0;
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
  Function ConvertNearJumpsToShort: Boolean;
  Var JumpRecord: PIntermediateCodeRecord;
      JumpRecordOfs: Word absolute JumpRecord;
      LabelRecord: PIntermediateCodeRecord;
      LabelRecordOfs: Word absolute LabelRecord;
      JumpDisplacement: Integer;
      NumberOfConvertedJumps: Word;
  begin
    NumberOfConvertedJumps := 0;
    JumpRecord := Ptr (SymbolTable [stIntermediateCode].Segment, Last_icJumpNear);
    LabelRecord := Ptr (SymbolTable [stIntermediateCode].Segment, 0);
    While JumpRecordOfs <> 0 do
      With JumpRecord^ do
        begin
          LabelRecordOfs := LabelRecordOffset - 1;
          JumpDisplacement := LabelRecord^.LabelAddress - JumpCodeOffset;
          If (JumpDisplacement <= 129) and (JumpDisplacement >= - 126) then
            begin
              RecordType := icJumpShort;
              Inc (NumberOfConvertedJumps);
            end;
          JumpRecordOfs := Previous_icJumpNear;
        end;
    ConvertNearJumpsToShort := NumberOfConvertedJumps <> 0;
  end;
```

### Code Block 2

```pascal
  Function ConvertNearJumpsToShort: Boolean;
  Var JumpRecord: PIntermediateCodeRecord;
      JumpRecordOfs: Word absolute JumpRecord;
      LabelRecord: PIntermediateCodeRecord;
      LabelRecordOfs: Word absolute LabelRecord;
      JumpDisplacement: Integer;
      NumberOfConvertedJumps: Word;
  begin
    NumberOfConvertedJumps := 0;
    JumpRecord := Ptr (SymbolTable [stIntermediateCode].Segment, Last_icJumpNear);
    LabelRecord := Ptr (SymbolTable [stIntermediateCode].Segment, 0);
    While JumpRecordOfs <> 0 do
      With JumpRecord^ do
        begin
          LabelRecordOfs := LabelRecordOffset - 1;
          JumpDisplacement := LabelRecord^.LabelAddress - JumpCodeOffset;
          If (JumpDisplacement <= 129) and (JumpDisplacement >= - 126) then
            begin
              RecordType := icJumpShort;
              Inc (NumberOfConvertedJumps);
            end;
          JumpRecordOfs := Previous_icJumpNear;
        end;
    ConvertNearJumpsToShort := NumberOfConvertedJumps <> 0;
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
- https://turbopascal.org/generating-code
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/optimizing-jumps
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

