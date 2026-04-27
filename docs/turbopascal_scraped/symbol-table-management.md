# Symbol Table Management - Turbo Pascal Compiler

**URL:** https://turbopascal.org/symbol-table-management

---

## Table of Contents

- Symbol Table Management

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Symbol Table Management
Each identifier symbol table must be created before it can be used. Turbo Pascal uses diferent parameters (number of linked lists) for different identifier symbol tables.
Procedure CreateSymbolTable (Size: Word);
Var List: PIdentifierList;
begin
  List := IncreaseSymbolTable (stMain, 2 * Size + 2);
  List^.Mask := (Size - 1) * 2;
  FillChar (List^.Offset, 2 * Size, 0);
end;
During compilation process data is stored into symbol tables and they need to be increased. Turbo Pascal maintains for each symbol table a simple data structure which contains address of the symbol table (defined by
Segment
), data table
Size
, actual
UsedSize
and default compiler error is the size of data table exceeds 64 KB (size of one segment in x86).
UsedSize
is also offset to the next free location in the data table.
Type
    TSymbolTableData = Record
                         Case Byte of
                           0: (UsedSize: Word; Segment: Word; Size: Word; Error: TCompilerError);
                           1: (NextRecordOffset, Seg: Word; BSize: Word; Err: TCompilerError);
                           2: (Ptr: Pointer; BlockSize: Word; ErrorWord: Word);
                       end;
When the compiler needs to insert a symbol into the table it needs to increase its size for the exact amount as the size of the symbol (including associated data).
IncreaseSymbolTable
take care for this function. Since each increase of the table space needs to move following symbol tables this function tries to minimize such movements. It increases the size of symbol table is blocks according to the table size.
Function IncreaseSymbolTable (SymbolTableIndex: TSymbolTable; AdditionalSize: Word): Pointer;
Var NewUsedSize: Longint;
    NewBlockSize: Longint;
    FreeParagraphs, NewBlockParagraphSize: Word;
    LastSymbolTableIndex: TSymbolTable;
begin
  NewUsedSize := Longint (SymbolTable [SymbolTableIndex].UsedSize) + AdditionalSize;
  If NewUsedSize &gt; $FFFF then Error (SymbolTable [SymbolTableIndex].Error);
  IncreaseSymbolTable := SymbolTable [SymbolTableIndex].Ptr;
  SymbolTable [SymbolTableIndex].UsedSize := NewUsedSize;
  If NewUsedSize &gt; SymbolTable [SymbolTableIndex].Size then
    begin                                                 { Make additional space }
      If NewUsedSize &gt; $FFF0 then Error (SymbolTable [SymbolTableIndex].Error);
      NewUsedSize := (NewUsedSize + $000F) and $FFF0;
      NewBlockSize := NewUsedSize;
      If not CreateExactSymbolTableSize then
        begin
          NewBlockSize := $0200;    { 512 }
          While NewBlockSize &lt;= SymbolTable [SymbolTableIndex].UsedSize do
            begin
              NewBlockSize := NewBlockSize shl 1;
              If NewBlockSize &lt; $2000 then Continue;
              While NewBlockSize &lt;= SymbolTable [SymbolTableIndex].UsedSize do
                begin
                  Inc (NewBlockSize, $2000);
                  If NewBlockSize &lt; $10000 then Continue;
                  NewBlockSize := $FFF0;
                  Break;
                end;
            end;
        end;
      Dec (NewBlockSize, SymbolTable [SymbolTableIndex].Size);
      NewBlockParagraphSize := NewBlockSize shr 4;
      FreeParagraphs := HeapEndRec.Seg - HeapPtrRec.Seg;
      If NewBlockParagraphSize &gt; FreeParagraphs then
        begin
          NewBlockSize := FreeParagraphs shl 4 + SymbolTable [SymbolTableIndex].Size;
          If NewBlockSize &lt; SymbolTable [SymbolTableIndex].UsedSize then Error (OutOfMemory);
          NewBlockParagraphSize := FreeParagraphs;
        end;
      HeapPtrRec.Seg := HeapPtrRec.Seg + NewBlockParagraphSize;
      Inc (SymbolTable [SymbolTableIndex].Size, NewBlockParagraphSize shl 4);
      LastSymbolTableIndex := High (TSymbolTable);
      While LastSymbolTableIndex &lt;&gt; SymbolTableIndex do
        begin
          If SymbolTable [LastSymbolTableIndex].Size &lt;&gt; 0 then
            MoveForward (Ptr (SymbolTable [LastSymbolTableIndex].Segment, 0)^,
                         Ptr (SymbolTable [LastSymbolTableIndex].Segment + NewBlockParagraphSize, 0)^,
                         SymbolTable [LastSymbolTableIndex].UsedSize);
          Inc (SymbolTable [LastSymbolTableIndex].Segment, NewBlockParagraphSize);
          Dec (LastSymbolTableIndex);
        end;
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
Procedure CreateSymbolTable (Size: Word);
Var List: PIdentifierList;
begin
  List := IncreaseSymbolTable (stMain, 2 * Size + 2);
  List^.Mask := (Size - 1) * 2;
  FillChar (List^.Offset, 2 * Size, 0);
end;
```

### Code Block 2

```pascal
Type
    TSymbolTableData = Record
                         Case Byte of
                           0: (UsedSize: Word; Segment: Word; Size: Word; Error: TCompilerError);
                           1: (NextRecordOffset, Seg: Word; BSize: Word; Err: TCompilerError);
                           2: (Ptr: Pointer; BlockSize: Word; ErrorWord: Word);
                       end;
```

### Code Block 3

```pascal
Function IncreaseSymbolTable (SymbolTableIndex: TSymbolTable; AdditionalSize: Word): Pointer;
Var NewUsedSize: Longint;
    NewBlockSize: Longint;
    FreeParagraphs, NewBlockParagraphSize: Word;
    LastSymbolTableIndex: TSymbolTable;
begin
  NewUsedSize := Longint (SymbolTable [SymbolTableIndex].UsedSize) + AdditionalSize;
  If NewUsedSize &gt; $FFFF then Error (SymbolTable [SymbolTableIndex].Error);
  IncreaseSymbolTable := SymbolTable [SymbolTableIndex].Ptr;
  SymbolTable [SymbolTableIndex].UsedSize := NewUsedSize;
  If NewUsedSize &gt; SymbolTable [SymbolTableIndex].Size then
    begin                                                 { Make additional space }
      If NewUsedSize &gt; $FFF0 then Error (SymbolTable [SymbolTableIndex].Error);
      NewUsedSize := (NewUsedSize + $000F) and $FFF0;
      NewBlockSize := NewUsedSize;
      If not CreateExactSymbolTableSize then
        begin
          NewBlockSize := $0200;    { 512 }
          While NewBlockSize &lt;= SymbolTable [SymbolTableIndex].UsedSize do
            begin
              NewBlockSize := NewBlockSize shl 1;
              If NewBlockSize &lt; $2000 then Continue;
              While NewBlockSize &lt;= SymbolTable [SymbolTableIndex].UsedSize do
                begin
                  Inc (NewBlockSize, $2000);
                  If NewBlockSize &lt; $10000 then Continue;
                  NewBlockSize := $FFF0;
                  Break;
                end;
            end;
        end;
      Dec (NewBlockSize, SymbolTable [SymbolTableIndex].Size);
      NewBlockParagraphSize := NewBlockSize shr 4;
      FreeParagraphs := HeapEndRec.Seg - HeapPtrRec.Seg;
      If NewBlockParagraphSize &gt; FreeParagraphs then
        begin
          NewBlockSize := FreeParagraphs shl 4 + SymbolTable [SymbolTableIndex].Size;
          If NewBlockSize &lt; SymbolTable [SymbolTableIndex].UsedSize then Error (OutOfMemory);
          NewBlockParagraphSize := FreeParagraphs;
        end;
      HeapPtrRec.Seg := HeapPtrRec.Seg + NewBlockParagraphSize;
      Inc (SymbolTable [SymbolTableIndex].Size, NewBlockParagraphSize shl 4);
      LastSymbolTableIndex := High (TSymbolTable);
      While LastSymbolTableIndex &lt;&gt; SymbolTableIndex do
        begin
          If SymbolTable [LastSymbolTableIndex].Size &lt;&gt; 0 then
            MoveForward (Ptr (SymbolTable [LastSymbolTableIndex].Segment, 0)^,
                         Ptr (SymbolTable [LastSymbolTableIndex].Segment + NewBlockParagraphSize, 0)^,
                         SymbolTable [LastSymbolTableIndex].UsedSize);
          Inc (SymbolTable [LastSymbolTableIndex].Segment, NewBlockParagraphSize);
          Dec (LastSymbolTableIndex);
        end;
    end;
end;
```

### Code Block 4

```pascal
Procedure CreateSymbolTable (Size: Word);
Var List: PIdentifierList;
begin
  List := IncreaseSymbolTable (stMain, 2 * Size + 2);
  List^.Mask := (Size - 1) * 2;
  FillChar (List^.Offset, 2 * Size, 0);
end;
```

### Code Block 5

```pascal
Type
    TSymbolTableData = Record
                         Case Byte of
                           0: (UsedSize: Word; Segment: Word; Size: Word; Error: TCompilerError);
                           1: (NextRecordOffset, Seg: Word; BSize: Word; Err: TCompilerError);
                           2: (Ptr: Pointer; BlockSize: Word; ErrorWord: Word);
                       end;
```

### Code Block 6

```pascal
Function IncreaseSymbolTable (SymbolTableIndex: TSymbolTable; AdditionalSize: Word): Pointer;
Var NewUsedSize: Longint;
    NewBlockSize: Longint;
    FreeParagraphs, NewBlockParagraphSize: Word;
    LastSymbolTableIndex: TSymbolTable;
begin
  NewUsedSize := Longint (SymbolTable [SymbolTableIndex].UsedSize) + AdditionalSize;
  If NewUsedSize &gt; $FFFF then Error (SymbolTable [SymbolTableIndex].Error);
  IncreaseSymbolTable := SymbolTable [SymbolTableIndex].Ptr;
  SymbolTable [SymbolTableIndex].UsedSize := NewUsedSize;
  If NewUsedSize &gt; SymbolTable [SymbolTableIndex].Size then
    begin                                                 { Make additional space }
      If NewUsedSize &gt; $FFF0 then Error (SymbolTable [SymbolTableIndex].Error);
      NewUsedSize := (NewUsedSize + $000F) and $FFF0;
      NewBlockSize := NewUsedSize;
      If not CreateExactSymbolTableSize then
        begin
          NewBlockSize := $0200;    { 512 }
          While NewBlockSize &lt;= SymbolTable [SymbolTableIndex].UsedSize do
            begin
              NewBlockSize := NewBlockSize shl 1;
              If NewBlockSize &lt; $2000 then Continue;
              While NewBlockSize &lt;= SymbolTable [SymbolTableIndex].UsedSize do
                begin
                  Inc (NewBlockSize, $2000);
                  If NewBlockSize &lt; $10000 then Continue;
                  NewBlockSize := $FFF0;
                  Break;
                end;
            end;
        end;
      Dec (NewBlockSize, SymbolTable [SymbolTableIndex].Size);
      NewBlockParagraphSize := NewBlockSize shr 4;
      FreeParagraphs := HeapEndRec.Seg - HeapPtrRec.Seg;
      If NewBlockParagraphSize &gt; FreeParagraphs then
        begin
          NewBlockSize := FreeParagraphs shl 4 + SymbolTable [SymbolTableIndex].Size;
          If NewBlockSize &lt; SymbolTable [SymbolTableIndex].UsedSize then Error (OutOfMemory);
          NewBlockParagraphSize := FreeParagraphs;
        end;
      HeapPtrRec.Seg := HeapPtrRec.Seg + NewBlockParagraphSize;
      Inc (SymbolTable [SymbolTableIndex].Size, NewBlockParagraphSize shl 4);
      LastSymbolTableIndex := High (TSymbolTable);
      While LastSymbolTableIndex &lt;&gt; SymbolTableIndex do
        begin
          If SymbolTable [LastSymbolTableIndex].Size &lt;&gt; 0 then
            MoveForward (Ptr (SymbolTable [LastSymbolTableIndex].Segment, 0)^,
                         Ptr (SymbolTable [LastSymbolTableIndex].Segment + NewBlockParagraphSize, 0)^,
                         SymbolTable [LastSymbolTableIndex].UsedSize);
          Inc (SymbolTable [LastSymbolTableIndex].Segment, NewBlockParagraphSize);
          Dec (LastSymbolTableIndex);
        end;
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
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/symbol-table-management
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

