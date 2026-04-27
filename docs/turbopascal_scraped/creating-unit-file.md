# Creating Unit File - Turbo Pascal Compiler

**URL:** https://turbopascal.org/creating-unit-file

---

## Table of Contents

- Creating Unit File

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Creating Unit File
CompileUnit
calls this procedure to create unit file which is in fact a collection of symbol tables. All segment addresses are cleared and symbol table data is written to the file.
Procedure CreateUnitFile;
Var SavedUnitSegment: Word;
    UnitFile: File;
    UnitFileSize, BytesToWrite: LongInt;
    UnitPtr: Pointer;
    BytesWritten: Word;

    UnitCounter: Word;
    UnitIdentifier: PIdentifier;
    UnitIdentifierRec: PtrRec absolute UnitIdentifier;
    UnitIdentifierData: PUnitIdentifierData;
    UnitHeader: PUnitHeader;
begin
  If cmoCompileToDisk in CompilerModeOptions then
    begin
      UnitCounter := 0;                                                 { Push And Clear Unit Segments;}
      UnitHeader := Ptr (MainSymbolTable.Segment, 0);
      UnitIdentifier := Ptr (MainSymbolTable.Segment, UnitHeader^.UnitNameIdentifierOffset);
      Repeat
        UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
        Asm
          LES   DI, UnitIdentifierData
          PUSH  WORD PTR ES:[DI]            { UnitIdentifierData^.UnitSegment) }
          PUSH  DI                          { Ofs (UnitIdentifierData^) }
        end;
        UnitIdentifierData^.UnitSegment := 0;
        Inc (UnitCounter);
        UnitIdentifierRec.Ofs := UnitIdentifierData^.NextUnitIdentifier;
      until UnitIdentifierRec.Ofs = 0;

      SavedUnitSegment := PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^.PreviousUnitSegment;
      PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^.PreviousUnitSegment := 0;
      FindFilePath (StrPCopy (@Identifier, CurrentSourceFile^.Name.Str),
                      Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_TPU);
      CreateFile (UnitFile, @Identifier);
      OutputFileHandle := FileRec (UnitFile).Handle;
      UnitFileSize := LongInt (HeapPtrRec.Seg - MainSymbolTable.Segment) * 16;
      UnitPtr := Ptr (MainSymbolTable.Segment, 0);
{$I-}
      While UnitFileSize > 0 do
        begin
          BytesToWrite := UnitFileSize;
          If BytesToWrite > $FFF0 then BytesToWrite := $FFF0;
          BlockWrite (UnitFile, UnitPtr^, BytesToWrite, BytesWritten);
          If IOResult <> 0 then Error (DiskFull);
          Dec (UnitFileSize, BytesWritten);
          UnitPtr := Ptr (Seg (UnitPtr^) + $0FFF, 0);
        end;
      Close (UnitFile);
{$I+}
      OutputFileHandle := 0;
      PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^.PreviousUnitSegment := SavedUnitSegment;
      Asm
        LES   DI, UnitHeader
        MOV   CX, UnitCounter
    @1:
        POP   DI
        POP   WORD PTR ES:[DI]
        LOOP  @1
      end;
    end;
  CodeBytes := SymbolTable [stCode].UsedSize;
  DataBytes := SymbolTable [stTypedConstants].UsedSize + SizeOfVariablesInDataSegment;
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
Procedure CreateUnitFile;
Var SavedUnitSegment: Word;
    UnitFile: File;
    UnitFileSize, BytesToWrite: LongInt;
    UnitPtr: Pointer;
    BytesWritten: Word;

    UnitCounter: Word;
    UnitIdentifier: PIdentifier;
    UnitIdentifierRec: PtrRec absolute UnitIdentifier;
    UnitIdentifierData: PUnitIdentifierData;
    UnitHeader: PUnitHeader;
begin
  If cmoCompileToDisk in CompilerModeOptions then
    begin
      UnitCounter := 0;                                                 { Push And Clear Unit Segments;}
      UnitHeader := Ptr (MainSymbolTable.Segment, 0);
      UnitIdentifier := Ptr (MainSymbolTable.Segment, UnitHeader^.UnitNameIdentifierOffset);
      Repeat
        UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
        Asm
          LES   DI, UnitIdentifierData
          PUSH  WORD PTR ES:[DI]            { UnitIdentifierData^.UnitSegment) }
          PUSH  DI                          { Ofs (UnitIdentifierData^) }
        end;
        UnitIdentifierData^.UnitSegment := 0;
        Inc (UnitCounter);
        UnitIdentifierRec.Ofs := UnitIdentifierData^.NextUnitIdentifier;
      until UnitIdentifierRec.Ofs = 0;

      SavedUnitSegment := PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^.PreviousUnitSegment;
      PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^.PreviousUnitSegment := 0;
      FindFilePath (StrPCopy (@Identifier, CurrentSourceFile^.Name.Str),
                      Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_TPU);
      CreateFile (UnitFile, @Identifier);
      OutputFileHandle := FileRec (UnitFile).Handle;
      UnitFileSize := LongInt (HeapPtrRec.Seg - MainSymbolTable.Segment) * 16;
      UnitPtr := Ptr (MainSymbolTable.Segment, 0);
{$I-}
      While UnitFileSize > 0 do
        begin
          BytesToWrite := UnitFileSize;
          If BytesToWrite > $FFF0 then BytesToWrite := $FFF0;
          BlockWrite (UnitFile, UnitPtr^, BytesToWrite, BytesWritten);
          If IOResult <> 0 then Error (DiskFull);
          Dec (UnitFileSize, BytesWritten);
          UnitPtr := Ptr (Seg (UnitPtr^) + $0FFF, 0);
        end;
      Close (UnitFile);
{$I+}
      OutputFileHandle := 0;
      PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^.PreviousUnitSegment := SavedUnitSegment;
      Asm
        LES   DI, UnitHeader
        MOV   CX, UnitCounter
    @1:
        POP   DI
        POP   WORD PTR ES:[DI]
        LOOP  @1
      end;
    end;
  CodeBytes := SymbolTable [stCode].UsedSize;
  DataBytes := SymbolTable [stTypedConstants].UsedSize + SizeOfVariablesInDataSegment;
end;
```

### Code Block 2

```pascal
Procedure CreateUnitFile;
Var SavedUnitSegment: Word;
    UnitFile: File;
    UnitFileSize, BytesToWrite: LongInt;
    UnitPtr: Pointer;
    BytesWritten: Word;

    UnitCounter: Word;
    UnitIdentifier: PIdentifier;
    UnitIdentifierRec: PtrRec absolute UnitIdentifier;
    UnitIdentifierData: PUnitIdentifierData;
    UnitHeader: PUnitHeader;
begin
  If cmoCompileToDisk in CompilerModeOptions then
    begin
      UnitCounter := 0;                                                 { Push And Clear Unit Segments;}
      UnitHeader := Ptr (MainSymbolTable.Segment, 0);
      UnitIdentifier := Ptr (MainSymbolTable.Segment, UnitHeader^.UnitNameIdentifierOffset);
      Repeat
        UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
        Asm
          LES   DI, UnitIdentifierData
          PUSH  WORD PTR ES:[DI]            { UnitIdentifierData^.UnitSegment) }
          PUSH  DI                          { Ofs (UnitIdentifierData^) }
        end;
        UnitIdentifierData^.UnitSegment := 0;
        Inc (UnitCounter);
        UnitIdentifierRec.Ofs := UnitIdentifierData^.NextUnitIdentifier;
      until UnitIdentifierRec.Ofs = 0;

      SavedUnitSegment := PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^.PreviousUnitSegment;
      PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^.PreviousUnitSegment := 0;
      FindFilePath (StrPCopy (@Identifier, CurrentSourceFile^.Name.Str),
                      Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_TPU);
      CreateFile (UnitFile, @Identifier);
      OutputFileHandle := FileRec (UnitFile).Handle;
      UnitFileSize := LongInt (HeapPtrRec.Seg - MainSymbolTable.Segment) * 16;
      UnitPtr := Ptr (MainSymbolTable.Segment, 0);
{$I-}
      While UnitFileSize > 0 do
        begin
          BytesToWrite := UnitFileSize;
          If BytesToWrite > $FFF0 then BytesToWrite := $FFF0;
          BlockWrite (UnitFile, UnitPtr^, BytesToWrite, BytesWritten);
          If IOResult <> 0 then Error (DiskFull);
          Dec (UnitFileSize, BytesWritten);
          UnitPtr := Ptr (Seg (UnitPtr^) + $0FFF, 0);
        end;
      Close (UnitFile);
{$I+}
      OutputFileHandle := 0;
      PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^.PreviousUnitSegment := SavedUnitSegment;
      Asm
        LES   DI, UnitHeader
        MOV   CX, UnitCounter
    @1:
        POP   DI
        POP   WORD PTR ES:[DI]
        LOOP  @1
      end;
    end;
  CodeBytes := SymbolTable [stCode].UsedSize;
  DataBytes := SymbolTable [stTypedConstants].UsedSize + SizeOfVariablesInDataSegment;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compilation-process
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-unit-file
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

