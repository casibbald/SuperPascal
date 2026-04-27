# Creating Executable File - Turbo Pascal Compiler

**URL:** https://turbopascal.org/creating-executable-file

---

## Table of Contents

- Creating Executable File

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Creating Executable File
This procedure creates space in heap for exe header and typed constants, creates exe file and positions it to the start of the code.
Procedure CreateExeFile;
begin
  ExeHeaderBuffer.Seg := HeapPtrRec.Seg;
  Inc (HeapPtrRec.Seg, (CurrentRelocationItemOffset + $000F) shr 4);
  If HeapPtrRec.Seg > HeapEndRec.Seg then Error (OutOfMemory);
  TypedConstantsPointer.Seg := HeapPtrRec.Seg;
  Inc (HeapPtrRec.Seg, (OffsetOfVariables + $000F) shr 4);
  If HeapPtrRec.Seg > HeapEndRec.Seg then Error (OutOfMemory);
  StrCopy (ExeFileName, CurrentFileName);
  FindFilePath (ExeFileName, Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_EXE);
  CreateFile (ExeFile, ExeFileName);
  ExeFileHandle := FileRec (ExeFile).Handle;
  Seek (ExeFile, (CurrentRelocationItemOffset + $000F) and $FFF0);
end;

Type PExeHeader = ^TExeHeader;
     TExeHeader = Record
                    Signature: Array [1..2] of Char;  { 00 'MZ' }
                    ExtraBytes: Word;                 { 02 Last page byte count, Filelength mod 512 }
                    Pages: Word;                      { 04 Number of 512 byte blocks including header, Filelength div 512 }
                    RelocItems: Word;                 { 06 Number of relocation items }
                    HeaderSize: Word;                 { 08 Size of header in paragraphs }
                    MinAlloc: Word;                   { 0A Minimum number of memory paragraphs }
                    MaxAlloc: Word;                   { 0C Maximum number of memory paragraphs }
                    InitSS: Word;                     { 0E Initial value of SS }
                    InitSP: Word;                     { 10 Initial value of SP }
                    Checksum: Word;                   { 12 Any value }
                    InitIP: Word;                     { 14 Initial value of IP }
                    InitCS: Word;                     { 16 Initial value of CS }
                    RelocTable: Word;                 { 18 Offset of relocation table }
                    OverlayNumber: Word;              { 1A Usually 0 }
                  end;
This procedure writes typed constants and header to the
exe
file.
Procedure WriteTypedConstantsAndExeHeader;
Var SystemUnitTypedConstants: PWord;
    ExeHeader: PExeHeader;
    ExeFileParagraphs: Word;
    ProgramStartOffset: Word;
    ProceduresBlockRecord: PProceduresBlockRecord;
    CodeConstVarBlockRecord: PCodeConstVarBlockRecord absolute ProceduresBlockRecord;
    CodeConstVarBlockRecordPtrRec: PtrRec absolute CodeConstVarBlockRecord;
begin
  SystemUnitTypedConstants := TypedConstantsPointer.Ptr;
  Word (Ptr (TypedConstantsPointer.Seg, 0)^) := 0;
  SystemUnitTypedConstants^ := OverlayCodeList;
  Inc (SystemUnitTypedConstants);
  SystemUnitTypedConstants^ := OverlayHeapSize;
  FillChar (Ptr (TypedConstantsPointer.Seg, OffsetOfVariables)^, 16 - OffsetOfVariables and $000F, 0);
  BlockWrite (ExeFile, Ptr (TypedConstantsPointer.Seg, 0)^, (OffsetOfVariables + $000F) and $FFF0);
  Seek (ExeFile, 0);
  ExeFileSize := FileSize (ExeFile);
  ExeHeader := Ptr (ExeHeaderBuffer.Seg, 0);
  With ExeHeader^ do
    begin
      Signature := 'MZ';
      HeaderSize := (CurrentRelocationItemOffset + $000F) shr 4;
      ExeFileParagraphs := HeaderSize + DataSegment + (OffsetOfVariables + $000F) shr 4;
      ExtraBytes := (ExeFileParagraphs and $001F) shl 4;
      Pages := (ExeFileParagraphs + $001F) shr 5;
      RelocItems := (CurrentRelocationItemOffset - RelocationTableOffset) div 4;
      ExeFileParagraphs := (ModuleStack + $000F) shr 4 + FirstFreeSegment + OverlayHeapSize -
                             (OffsetOfVariables + $000F) shr 4 - DataSegment;
      If LongInt (ExeFileParagraphs) + ModuleHeapMin > $FFFF then MinAlloc := $FFFF
        else MinAlloc := ExeFileParagraphs + ModuleHeapMin;
      If LongInt (ExeFileParagraphs) + ModuleHeapMax > $FFFF then MaxAlloc := $FFFF
        else MaxAlloc := ExeFileParagraphs + ModuleHeapMax;
      InitSS := FirstFreeSegment;
      InitSP := ModuleStack;
      RelocTable := RelocationTableOffset;
      Checksum := 0;
    end;
  UnitPtrRec.Seg := SymbolTable [stMain].Segment;
  ProceduresBlockRecord := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [stProcedures]);
  ProgramStartOffset := ProceduresBlockRecord^.SizeOfConstants;
  CodeConstVarBlockRecordPtrRec.Ofs := UnitPtr^.BlockOffset [stCodeBlocks] +
                                         ProceduresBlockRecord^.ProgramCodeBlockRecordOffset;
  Inc (ProgramStartOffset, CodeConstVarBlockRecord^.Offset);
  With ExeHeader^ do
    begin
      InitIP := ProgramStartOffset;
      InitCS := 0;
      OverlayNumber := 0;
    end;
  FillChar (Ptr (ExeHeaderBuffer.Seg, CurrentRelocationItemOffset)^, 16 - CurrentRelocationItemOffset and $000F, 0);
  BlockWrite (ExeFile, Ptr (ExeHeaderBuffer.Seg, 0)^, (CurrentRelocationItemOffset + $000F) and $FFF0);
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
Procedure CreateExeFile;
begin
  ExeHeaderBuffer.Seg := HeapPtrRec.Seg;
  Inc (HeapPtrRec.Seg, (CurrentRelocationItemOffset + $000F) shr 4);
  If HeapPtrRec.Seg > HeapEndRec.Seg then Error (OutOfMemory);
  TypedConstantsPointer.Seg := HeapPtrRec.Seg;
  Inc (HeapPtrRec.Seg, (OffsetOfVariables + $000F) shr 4);
  If HeapPtrRec.Seg > HeapEndRec.Seg then Error (OutOfMemory);
  StrCopy (ExeFileName, CurrentFileName);
  FindFilePath (ExeFileName, Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_EXE);
  CreateFile (ExeFile, ExeFileName);
  ExeFileHandle := FileRec (ExeFile).Handle;
  Seek (ExeFile, (CurrentRelocationItemOffset + $000F) and $FFF0);
end;

Type PExeHeader = ^TExeHeader;
     TExeHeader = Record
                    Signature: Array [1..2] of Char;  { 00 'MZ' }
                    ExtraBytes: Word;                 { 02 Last page byte count, Filelength mod 512 }
                    Pages: Word;                      { 04 Number of 512 byte blocks including header, Filelength div 512 }
                    RelocItems: Word;                 { 06 Number of relocation items }
                    HeaderSize: Word;                 { 08 Size of header in paragraphs }
                    MinAlloc: Word;                   { 0A Minimum number of memory paragraphs }
                    MaxAlloc: Word;                   { 0C Maximum number of memory paragraphs }
                    InitSS: Word;                     { 0E Initial value of SS }
                    InitSP: Word;                     { 10 Initial value of SP }
                    Checksum: Word;                   { 12 Any value }
                    InitIP: Word;                     { 14 Initial value of IP }
                    InitCS: Word;                     { 16 Initial value of CS }
                    RelocTable: Word;                 { 18 Offset of relocation table }
                    OverlayNumber: Word;              { 1A Usually 0 }
                  end;
```

### Code Block 2

```pascal
Procedure WriteTypedConstantsAndExeHeader;
Var SystemUnitTypedConstants: PWord;
    ExeHeader: PExeHeader;
    ExeFileParagraphs: Word;
    ProgramStartOffset: Word;
    ProceduresBlockRecord: PProceduresBlockRecord;
    CodeConstVarBlockRecord: PCodeConstVarBlockRecord absolute ProceduresBlockRecord;
    CodeConstVarBlockRecordPtrRec: PtrRec absolute CodeConstVarBlockRecord;
begin
  SystemUnitTypedConstants := TypedConstantsPointer.Ptr;
  Word (Ptr (TypedConstantsPointer.Seg, 0)^) := 0;
  SystemUnitTypedConstants^ := OverlayCodeList;
  Inc (SystemUnitTypedConstants);
  SystemUnitTypedConstants^ := OverlayHeapSize;
  FillChar (Ptr (TypedConstantsPointer.Seg, OffsetOfVariables)^, 16 - OffsetOfVariables and $000F, 0);
  BlockWrite (ExeFile, Ptr (TypedConstantsPointer.Seg, 0)^, (OffsetOfVariables + $000F) and $FFF0);
  Seek (ExeFile, 0);
  ExeFileSize := FileSize (ExeFile);
  ExeHeader := Ptr (ExeHeaderBuffer.Seg, 0);
  With ExeHeader^ do
    begin
      Signature := 'MZ';
      HeaderSize := (CurrentRelocationItemOffset + $000F) shr 4;
      ExeFileParagraphs := HeaderSize + DataSegment + (OffsetOfVariables + $000F) shr 4;
      ExtraBytes := (ExeFileParagraphs and $001F) shl 4;
      Pages := (ExeFileParagraphs + $001F) shr 5;
      RelocItems := (CurrentRelocationItemOffset - RelocationTableOffset) div 4;
      ExeFileParagraphs := (ModuleStack + $000F) shr 4 + FirstFreeSegment + OverlayHeapSize -
                             (OffsetOfVariables + $000F) shr 4 - DataSegment;
      If LongInt (ExeFileParagraphs) + ModuleHeapMin > $FFFF then MinAlloc := $FFFF
        else MinAlloc := ExeFileParagraphs + ModuleHeapMin;
      If LongInt (ExeFileParagraphs) + ModuleHeapMax > $FFFF then MaxAlloc := $FFFF
        else MaxAlloc := ExeFileParagraphs + ModuleHeapMax;
      InitSS := FirstFreeSegment;
      InitSP := ModuleStack;
      RelocTable := RelocationTableOffset;
      Checksum := 0;
    end;
  UnitPtrRec.Seg := SymbolTable [stMain].Segment;
  ProceduresBlockRecord := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [stProcedures]);
  ProgramStartOffset := ProceduresBlockRecord^.SizeOfConstants;
  CodeConstVarBlockRecordPtrRec.Ofs := UnitPtr^.BlockOffset [stCodeBlocks] +
                                         ProceduresBlockRecord^.ProgramCodeBlockRecordOffset;
  Inc (ProgramStartOffset, CodeConstVarBlockRecord^.Offset);
  With ExeHeader^ do
    begin
      InitIP := ProgramStartOffset;
      InitCS := 0;
      OverlayNumber := 0;
    end;
  FillChar (Ptr (ExeHeaderBuffer.Seg, CurrentRelocationItemOffset)^, 16 - CurrentRelocationItemOffset and $000F, 0);
  BlockWrite (ExeFile, Ptr (ExeHeaderBuffer.Seg, 0)^, (CurrentRelocationItemOffset + $000F) and $FFF0);
end;
```

### Code Block 3

```pascal
Procedure CreateExeFile;
begin
  ExeHeaderBuffer.Seg := HeapPtrRec.Seg;
  Inc (HeapPtrRec.Seg, (CurrentRelocationItemOffset + $000F) shr 4);
  If HeapPtrRec.Seg > HeapEndRec.Seg then Error (OutOfMemory);
  TypedConstantsPointer.Seg := HeapPtrRec.Seg;
  Inc (HeapPtrRec.Seg, (OffsetOfVariables + $000F) shr 4);
  If HeapPtrRec.Seg > HeapEndRec.Seg then Error (OutOfMemory);
  StrCopy (ExeFileName, CurrentFileName);
  FindFilePath (ExeFileName, Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_EXE);
  CreateFile (ExeFile, ExeFileName);
  ExeFileHandle := FileRec (ExeFile).Handle;
  Seek (ExeFile, (CurrentRelocationItemOffset + $000F) and $FFF0);
end;

Type PExeHeader = ^TExeHeader;
     TExeHeader = Record
                    Signature: Array [1..2] of Char;  { 00 'MZ' }
                    ExtraBytes: Word;                 { 02 Last page byte count, Filelength mod 512 }
                    Pages: Word;                      { 04 Number of 512 byte blocks including header, Filelength div 512 }
                    RelocItems: Word;                 { 06 Number of relocation items }
                    HeaderSize: Word;                 { 08 Size of header in paragraphs }
                    MinAlloc: Word;                   { 0A Minimum number of memory paragraphs }
                    MaxAlloc: Word;                   { 0C Maximum number of memory paragraphs }
                    InitSS: Word;                     { 0E Initial value of SS }
                    InitSP: Word;                     { 10 Initial value of SP }
                    Checksum: Word;                   { 12 Any value }
                    InitIP: Word;                     { 14 Initial value of IP }
                    InitCS: Word;                     { 16 Initial value of CS }
                    RelocTable: Word;                 { 18 Offset of relocation table }
                    OverlayNumber: Word;              { 1A Usually 0 }
                  end;
```

### Code Block 4

```pascal
Procedure WriteTypedConstantsAndExeHeader;
Var SystemUnitTypedConstants: PWord;
    ExeHeader: PExeHeader;
    ExeFileParagraphs: Word;
    ProgramStartOffset: Word;
    ProceduresBlockRecord: PProceduresBlockRecord;
    CodeConstVarBlockRecord: PCodeConstVarBlockRecord absolute ProceduresBlockRecord;
    CodeConstVarBlockRecordPtrRec: PtrRec absolute CodeConstVarBlockRecord;
begin
  SystemUnitTypedConstants := TypedConstantsPointer.Ptr;
  Word (Ptr (TypedConstantsPointer.Seg, 0)^) := 0;
  SystemUnitTypedConstants^ := OverlayCodeList;
  Inc (SystemUnitTypedConstants);
  SystemUnitTypedConstants^ := OverlayHeapSize;
  FillChar (Ptr (TypedConstantsPointer.Seg, OffsetOfVariables)^, 16 - OffsetOfVariables and $000F, 0);
  BlockWrite (ExeFile, Ptr (TypedConstantsPointer.Seg, 0)^, (OffsetOfVariables + $000F) and $FFF0);
  Seek (ExeFile, 0);
  ExeFileSize := FileSize (ExeFile);
  ExeHeader := Ptr (ExeHeaderBuffer.Seg, 0);
  With ExeHeader^ do
    begin
      Signature := 'MZ';
      HeaderSize := (CurrentRelocationItemOffset + $000F) shr 4;
      ExeFileParagraphs := HeaderSize + DataSegment + (OffsetOfVariables + $000F) shr 4;
      ExtraBytes := (ExeFileParagraphs and $001F) shl 4;
      Pages := (ExeFileParagraphs + $001F) shr 5;
      RelocItems := (CurrentRelocationItemOffset - RelocationTableOffset) div 4;
      ExeFileParagraphs := (ModuleStack + $000F) shr 4 + FirstFreeSegment + OverlayHeapSize -
                             (OffsetOfVariables + $000F) shr 4 - DataSegment;
      If LongInt (ExeFileParagraphs) + ModuleHeapMin > $FFFF then MinAlloc := $FFFF
        else MinAlloc := ExeFileParagraphs + ModuleHeapMin;
      If LongInt (ExeFileParagraphs) + ModuleHeapMax > $FFFF then MaxAlloc := $FFFF
        else MaxAlloc := ExeFileParagraphs + ModuleHeapMax;
      InitSS := FirstFreeSegment;
      InitSP := ModuleStack;
      RelocTable := RelocationTableOffset;
      Checksum := 0;
    end;
  UnitPtrRec.Seg := SymbolTable [stMain].Segment;
  ProceduresBlockRecord := Ptr (UnitPtrRec.Seg, UnitPtr^.BlockOffset [stProcedures]);
  ProgramStartOffset := ProceduresBlockRecord^.SizeOfConstants;
  CodeConstVarBlockRecordPtrRec.Ofs := UnitPtr^.BlockOffset [stCodeBlocks] +
                                         ProceduresBlockRecord^.ProgramCodeBlockRecordOffset;
  Inc (ProgramStartOffset, CodeConstVarBlockRecord^.Offset);
  With ExeHeader^ do
    begin
      InitIP := ProgramStartOffset;
      InitCS := 0;
      OverlayNumber := 0;
    end;
  FillChar (Ptr (ExeHeaderBuffer.Seg, CurrentRelocationItemOffset)^, 16 - CurrentRelocationItemOffset and $000F, 0);
  BlockWrite (ExeFile, Ptr (ExeHeaderBuffer.Seg, 0)^, (CurrentRelocationItemOffset + $000F) and $FFF0);
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-executable-file
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

