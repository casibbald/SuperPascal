# Importing LEDATA and LIDATA Object File Records - Turbo Pascal Compiler

**URL:** https://turbopascal.org/importing-ledata-and-lidata-object-file-records

---

## Table of Contents

- Importing LEDATA and LIDATA Object File Records

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Importing LEDATA and LIDATA Object File Records
This procedure processes
LEDATA
object file record (
$A0
) and copies data from object file to symbol table.
Procedure OMF_LEDATA; Far;
Var DestinationOffset: Word;
begin

{$IFDEF DEBUGOMF}
  Writeln ('        LEDATA');
{$ENDIF}

  If FindSegmentForEnumeratedData (DestinationOffset) then
    Move (ObjectFilePtr^.EnumData, Ptr (DataBlockSegment, DestinationOffset)^,
            ObjectRecordChecksumOffset - Ofs (ObjectFilePtr^.EnumData));
end;
This procedure processes
LIDATA
object file record (
$A2
) and copies data from object file to symbol table.
Procedure OMF_LIDATA; Far;
Type PDataBlock = ^TDataBlock;
     TDataBlock = Record
                    RepeatCount: Word;
                    BlockCount: Word;
                    Case Byte of
                      0: (BlockSize: Byte; Block: Byte);
                      1: (NextDataBlock: Byte);
                  end;
Var DestinationOffset: Word;

  Procedure RepeatBlocks (Var BlockStartPtr: PDataBlock);
  Var RepeatCount, BlockCount, N, M: Word;
      NextBlockStartPtr: PDataBlock;
  begin
    RepeatCount := BlockStartPtr^.RepeatCount;
    BlockCount := BlockStartPtr^.BlockCount;
    For N := 1 to RepeatCount do
      Case BlockCount of
        0: begin
             Move (BlockStartPtr^.Block, Ptr (DataBlockSegment, DestinationOffset)^, BlockStartPtr^.BlockSize);
             Inc (DestinationOffset, BlockStartPtr^.BlockSize);
             NextBlockStartPtr := PDataBlock (PChar (BlockStartPtr) + BlockStartPtr^.BlockSize + 5);
           end;
        else For M := 1 to BlockCount do
               begin
                 RepeatBlocks (BlockStartPtr);
                 NextBlockStartPtr := BlockStartPtr
               end;
      end;
    BlockStartPtr := NextBlockStartPtr
  end;

begin

{$IFDEF DEBUGOMF}
  Writeln ('        LIDATA');
{$ENDIF}

  If FindSegmentForEnumeratedData (DestinationOffset) then
    begin
      Inc (PChar (ObjectFilePtr), 3);
      Repeat
        RepeatBlocks (PDataBlock (ObjectFilePtr));
      until Ofs (ObjectFilePtr^) &amp;gt;= ObjectRecordChecksumOffset;
    end;
end;
This function returns True nad sets data if EnumSegmentIndex is either for code or typed constants.
Function FindSegmentForEnumeratedData (Var DI: Word): Boolean;
begin
  FindSegmentForEnumeratedData := False;
  ReferencesSymbolTable := stMain;
  If ObjectFilePtr^.EnumSegmentIndex = CodeSegmentIndex then
    begin
      EnumeratedDataOffset := ObjectFilePtr^.EnumeratedDataOffset;
      LastBlockSize := LastProgramBlockCodeSize;
      DataBlockSegment := SymbolTable [stCode].Segment;
      ReferencesSymbolTable := stCodeReferences;
      DI := LastProgramBlockCodeSize + ObjectFilePtr^.EnumeratedDataOffset;
      FindSegmentForEnumeratedData := True;
    end else If ObjectFilePtr^.EnumSegmentIndex = TypedConstantsSegmentIndex then
               begin
                 EnumeratedDataOffset := ObjectFilePtr^.EnumeratedDataOffset;
                 LastBlockSize := LastTypedConstantsSize;
                 DataBlockSegment := SymbolTable [stTypedConstants].Segment;
                 ReferencesSymbolTable := stTypedConstantsReferences;
                 DI := LastTypedConstantsSize + ObjectFilePtr^.EnumeratedDataOffset;
                 FindSegmentForEnumeratedData := True;
               end;
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
Procedure OMF_LEDATA; Far;
Var DestinationOffset: Word;
begin

{$IFDEF DEBUGOMF}
  Writeln ('        LEDATA');
{$ENDIF}

  If FindSegmentForEnumeratedData (DestinationOffset) then
    Move (ObjectFilePtr^.EnumData, Ptr (DataBlockSegment, DestinationOffset)^,
            ObjectRecordChecksumOffset - Ofs (ObjectFilePtr^.EnumData));
end;
```

### Code Block 2

```pascal
Procedure OMF_LIDATA; Far;
Type PDataBlock = ^TDataBlock;
     TDataBlock = Record
                    RepeatCount: Word;
                    BlockCount: Word;
                    Case Byte of
                      0: (BlockSize: Byte; Block: Byte);
                      1: (NextDataBlock: Byte);
                  end;
Var DestinationOffset: Word;

  Procedure RepeatBlocks (Var BlockStartPtr: PDataBlock);
  Var RepeatCount, BlockCount, N, M: Word;
      NextBlockStartPtr: PDataBlock;
  begin
    RepeatCount := BlockStartPtr^.RepeatCount;
    BlockCount := BlockStartPtr^.BlockCount;
    For N := 1 to RepeatCount do
      Case BlockCount of
        0: begin
             Move (BlockStartPtr^.Block, Ptr (DataBlockSegment, DestinationOffset)^, BlockStartPtr^.BlockSize);
             Inc (DestinationOffset, BlockStartPtr^.BlockSize);
             NextBlockStartPtr := PDataBlock (PChar (BlockStartPtr) + BlockStartPtr^.BlockSize + 5);
           end;
        else For M := 1 to BlockCount do
               begin
                 RepeatBlocks (BlockStartPtr);
                 NextBlockStartPtr := BlockStartPtr
               end;
      end;
    BlockStartPtr := NextBlockStartPtr
  end;

begin

{$IFDEF DEBUGOMF}
  Writeln ('        LIDATA');
{$ENDIF}

  If FindSegmentForEnumeratedData (DestinationOffset) then
    begin
      Inc (PChar (ObjectFilePtr), 3);
      Repeat
        RepeatBlocks (PDataBlock (ObjectFilePtr));
      until Ofs (ObjectFilePtr^) &amp;gt;= ObjectRecordChecksumOffset;
    end;
end;
```

### Code Block 3

```pascal
Function FindSegmentForEnumeratedData (Var DI: Word): Boolean;
begin
  FindSegmentForEnumeratedData := False;
  ReferencesSymbolTable := stMain;
  If ObjectFilePtr^.EnumSegmentIndex = CodeSegmentIndex then
    begin
      EnumeratedDataOffset := ObjectFilePtr^.EnumeratedDataOffset;
      LastBlockSize := LastProgramBlockCodeSize;
      DataBlockSegment := SymbolTable [stCode].Segment;
      ReferencesSymbolTable := stCodeReferences;
      DI := LastProgramBlockCodeSize + ObjectFilePtr^.EnumeratedDataOffset;
      FindSegmentForEnumeratedData := True;
    end else If ObjectFilePtr^.EnumSegmentIndex = TypedConstantsSegmentIndex then
               begin
                 EnumeratedDataOffset := ObjectFilePtr^.EnumeratedDataOffset;
                 LastBlockSize := LastTypedConstantsSize;
                 DataBlockSegment := SymbolTable [stTypedConstants].Segment;
                 ReferencesSymbolTable := stTypedConstantsReferences;
                 DI := LastTypedConstantsSize + ObjectFilePtr^.EnumeratedDataOffset;
                 FindSegmentForEnumeratedData := True;
               end;
end;
```

### Code Block 4

```pascal
Procedure OMF_LEDATA; Far;
Var DestinationOffset: Word;
begin

{$IFDEF DEBUGOMF}
  Writeln ('        LEDATA');
{$ENDIF}

  If FindSegmentForEnumeratedData (DestinationOffset) then
    Move (ObjectFilePtr^.EnumData, Ptr (DataBlockSegment, DestinationOffset)^,
            ObjectRecordChecksumOffset - Ofs (ObjectFilePtr^.EnumData));
end;
```

### Code Block 5

```pascal
Procedure OMF_LIDATA; Far;
Type PDataBlock = ^TDataBlock;
     TDataBlock = Record
                    RepeatCount: Word;
                    BlockCount: Word;
                    Case Byte of
                      0: (BlockSize: Byte; Block: Byte);
                      1: (NextDataBlock: Byte);
                  end;
Var DestinationOffset: Word;

  Procedure RepeatBlocks (Var BlockStartPtr: PDataBlock);
  Var RepeatCount, BlockCount, N, M: Word;
      NextBlockStartPtr: PDataBlock;
  begin
    RepeatCount := BlockStartPtr^.RepeatCount;
    BlockCount := BlockStartPtr^.BlockCount;
    For N := 1 to RepeatCount do
      Case BlockCount of
        0: begin
             Move (BlockStartPtr^.Block, Ptr (DataBlockSegment, DestinationOffset)^, BlockStartPtr^.BlockSize);
             Inc (DestinationOffset, BlockStartPtr^.BlockSize);
             NextBlockStartPtr := PDataBlock (PChar (BlockStartPtr) + BlockStartPtr^.BlockSize + 5);
           end;
        else For M := 1 to BlockCount do
               begin
                 RepeatBlocks (BlockStartPtr);
                 NextBlockStartPtr := BlockStartPtr
               end;
      end;
    BlockStartPtr := NextBlockStartPtr
  end;

begin

{$IFDEF DEBUGOMF}
  Writeln ('        LIDATA');
{$ENDIF}

  If FindSegmentForEnumeratedData (DestinationOffset) then
    begin
      Inc (PChar (ObjectFilePtr), 3);
      Repeat
        RepeatBlocks (PDataBlock (ObjectFilePtr));
      until Ofs (ObjectFilePtr^) &amp;gt;= ObjectRecordChecksumOffset;
    end;
end;
```

### Code Block 6

```pascal
Function FindSegmentForEnumeratedData (Var DI: Word): Boolean;
begin
  FindSegmentForEnumeratedData := False;
  ReferencesSymbolTable := stMain;
  If ObjectFilePtr^.EnumSegmentIndex = CodeSegmentIndex then
    begin
      EnumeratedDataOffset := ObjectFilePtr^.EnumeratedDataOffset;
      LastBlockSize := LastProgramBlockCodeSize;
      DataBlockSegment := SymbolTable [stCode].Segment;
      ReferencesSymbolTable := stCodeReferences;
      DI := LastProgramBlockCodeSize + ObjectFilePtr^.EnumeratedDataOffset;
      FindSegmentForEnumeratedData := True;
    end else If ObjectFilePtr^.EnumSegmentIndex = TypedConstantsSegmentIndex then
               begin
                 EnumeratedDataOffset := ObjectFilePtr^.EnumeratedDataOffset;
                 LastBlockSize := LastTypedConstantsSize;
                 DataBlockSegment := SymbolTable [stTypedConstants].Segment;
                 ReferencesSymbolTable := stTypedConstantsReferences;
                 DI := LastTypedConstantsSize + ObjectFilePtr^.EnumeratedDataOffset;
                 FindSegmentForEnumeratedData := True;
               end;
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
- https://turbopascal.org/importing-ledata-and-lidata-object-file-records
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

