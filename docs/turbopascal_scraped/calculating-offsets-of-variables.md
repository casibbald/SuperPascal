# Calculating Offsets of Variables - Turbo Pascal Compiler

**URL:** https://turbopascal.org/calculating-offsets-of-variables

---

## Table of Contents

- Calculating Offsets of Variables

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Calculating Offsets of Variables
This procedure calculates offset in data segment for each block of typed constants and variables and checks total data size.
Procedure CalculateVariableOffsetsAndTotalDataSize;
Var DataSegmentOffset: LongRec;

  Procedure CalculateOffsetsOfVariables (Block: TSymbolTable);
  Var ConstOrVarBlock: PConstVarBlockRecord;
      ConstOrVarBlockRec: PtrRec absolute ConstOrVarBlock;
  begin
    UnitPtrRec.Seg := LastLoadedUsedUnit;
    Repeat
      ConstOrVarBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [Block]);
      While ConstOrVarBlockRec.Ofs <> UnitPtr^.BlockOffset [Succ (Block)] do
        begin
          If ConstOrVarBlock^.Offset <> BlockUnused then
            begin
              ConstOrVarBlock^.Offset := DataSegmentOffset.WordL;
              Inc (DataSegmentOffset.Long, ConstOrVarBlock^.Size);
              If DataSegmentOffset.WordH <> 0 then Error (DataSegmentTooLarge);
            end;
          Inc (ConstOrVarBlock);
        end;
      UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
    until UnitPtrRec.Seg = 0;
  end;

begin
  DataSegmentOffset.Long := 2;
  CalculateOffsetsOfVariables (stTypedConstantsBlocks);
  OffsetOfVariables := DataSegmentOffset.WordL;
  CalculateOffsetsOfVariables (stVariablesBlocks);
  DataBytes := DataSegmentOffset.WordL;
  If DataSegmentOffset.WordL > $FFF0 then Error (DataSegmentTooLarge);
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
Procedure CalculateVariableOffsetsAndTotalDataSize;
Var DataSegmentOffset: LongRec;

  Procedure CalculateOffsetsOfVariables (Block: TSymbolTable);
  Var ConstOrVarBlock: PConstVarBlockRecord;
      ConstOrVarBlockRec: PtrRec absolute ConstOrVarBlock;
  begin
    UnitPtrRec.Seg := LastLoadedUsedUnit;
    Repeat
      ConstOrVarBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [Block]);
      While ConstOrVarBlockRec.Ofs <> UnitPtr^.BlockOffset [Succ (Block)] do
        begin
          If ConstOrVarBlock^.Offset <> BlockUnused then
            begin
              ConstOrVarBlock^.Offset := DataSegmentOffset.WordL;
              Inc (DataSegmentOffset.Long, ConstOrVarBlock^.Size);
              If DataSegmentOffset.WordH <> 0 then Error (DataSegmentTooLarge);
            end;
          Inc (ConstOrVarBlock);
        end;
      UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
    until UnitPtrRec.Seg = 0;
  end;

begin
  DataSegmentOffset.Long := 2;
  CalculateOffsetsOfVariables (stTypedConstantsBlocks);
  OffsetOfVariables := DataSegmentOffset.WordL;
  CalculateOffsetsOfVariables (stVariablesBlocks);
  DataBytes := DataSegmentOffset.WordL;
  If DataSegmentOffset.WordL > $FFF0 then Error (DataSegmentTooLarge);
end;
```

### Code Block 2

```pascal
Procedure CalculateVariableOffsetsAndTotalDataSize;
Var DataSegmentOffset: LongRec;

  Procedure CalculateOffsetsOfVariables (Block: TSymbolTable);
  Var ConstOrVarBlock: PConstVarBlockRecord;
      ConstOrVarBlockRec: PtrRec absolute ConstOrVarBlock;
  begin
    UnitPtrRec.Seg := LastLoadedUsedUnit;
    Repeat
      ConstOrVarBlock := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [Block]);
      While ConstOrVarBlockRec.Ofs <> UnitPtr^.BlockOffset [Succ (Block)] do
        begin
          If ConstOrVarBlock^.Offset <> BlockUnused then
            begin
              ConstOrVarBlock^.Offset := DataSegmentOffset.WordL;
              Inc (DataSegmentOffset.Long, ConstOrVarBlock^.Size);
              If DataSegmentOffset.WordH <> 0 then Error (DataSegmentTooLarge);
            end;
          Inc (ConstOrVarBlock);
        end;
      UnitPtrRec.Seg := UnitPtr^.PreviousUnitSegment;
    until UnitPtrRec.Seg = 0;
  end;

begin
  DataSegmentOffset.Long := 2;
  CalculateOffsetsOfVariables (stTypedConstantsBlocks);
  OffsetOfVariables := DataSegmentOffset.WordL;
  CalculateOffsetsOfVariables (stVariablesBlocks);
  DataBytes := DataSegmentOffset.WordL;
  If DataSegmentOffset.WordL > $FFF0 then Error (DataSegmentTooLarge);
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculating-offsets-of-variables
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

