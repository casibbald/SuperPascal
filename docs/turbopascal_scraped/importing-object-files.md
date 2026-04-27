# Importing Object Files - Turbo Pascal Compiler

**URL:** https://turbopascal.org/importing-object-files

---

## Table of Contents

- Importing Object Files

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Importing Object Files
Turbo Pascal can import and link object files created with any other compiler that can create
Relocatable Object Module Format file
. OMF file is a collection of object records with the following format:
PObjectFileRecord = ^TObjectFileRecord;
     TObjectFileRecord = Record
                           RecordType: Byte;
                           RecordLength: Word;
                           Case Byte of
                             $88: (Comment: Record
                                              ComType: Byte;
                                              Class: Byte;
                                              Case Byte of
                                                $00: (Translator: String);
                                                $98: (SourceFileNumber: Byte; SourceFileName: String);
                                              end);
                             $8C: (ExternalName: String);
                             $90: (BaseGroupIndex, BaseSegmentIndex: Byte);
                             $96: (Name: String);
                             $98: (SegDef: TSegDef);
                             $A0: (EnumSegmentIndex: Byte; EnumeratedDataOffset: Word; EnumData: Byte);
                             $A2: (ItSegmentIndex: Byte; IteratedDataOffset: Word);
                         end;
Turbo Pascal loads object files and processes records – only few object file record types are supported:
COMENT
MODEND
EXTDEF
PUBDEF
LINNUM
SEGDEF
FIXUPP
LEDATA
LIDATA
Procedure ImportObjectFiles;
Var UsedFilesBlockRecord: PUsedFilesBlockRecord;
    ObjectFile: File;
    ObjectFileParagraphSize: Word;

  Procedure ReadObjectFile;
  Var ObjectFileSizeRec: LongRec;
  begin
    FindUsedFilePath (UsedFilesBlockRecord, @Identifier);
    OpenFile (ObjectFile, @Identifier);

{$IFDEF DEBUGOMF}
    Writeln ('        OBJECT FILE: ', PChar (@Identifier));
{$ENDIF}

    ObjectFileSizeRec.Long := FileSize (ObjectFile);
    If (ObjectFileSizeRec.WordH &lt;&gt; 0) or (ObjectFileSizeRec.WordL &gt; $FFF0) then FileError (@Identifier, ObjectFileTooLarge);
    ObjectFileSize := ObjectFileSizeRec.WordL;
    ObjectFileParagraphSize := (ObjectFileSizeRec.WordL + $000F) shr 4;
    If HeapEndRec.Seg - ObjectFileParagraphSize &lt; HeapPtrRec.Seg then Error (OutOfMemory);
    Dec (HeapEndRec.Seg, ObjectFileParagraphSize);
    ObjectFileSegment := HeapEndRec.Seg;
    BlockRead (ObjectFile, HeapEnd^, ObjectFileSizeRec.WordL);
    Close (ObjectFile);
  end;

  Procedure ProcessObjectFileRecords;
  Const LinkProc: Array [0..$11] of Procedure = (
    OMF_Dummy,
    OMF_Dummy,
    OMF_Dummy,
    OMF_Dummy,
    OMF_COMENT,
    OMF_MODEND,
    OMF_EXTDEF,
    OMF_Dummy,
    OMF_PUBDEF,
    OMF_Dummy,
    OMF_LINNUM,
    OMF_Dummy,
    OMF_SEGDEF,
    OMF_Dummy,
    OMF_FIXUPP,
    OMF_Dummy,
    OMF_LEDATA,
    OMF_LIDATA);
  Var RecordType: Byte;
  begin
    SegmentCounter := 0;
    CodeSegmentIndex := 0;
    TypedConstantsSegmentIndex := 0;
    VariablesSegmentIndex := 0;
    ReferencesSymbolTable := stMain;
    LastSourceFileNumber := 0;
    SourceFileNumber := 0;
    ExtrnDefinitionsCounter := 0;
    OMF_ReferencedModule := AddReferencedModule (SymbolTable [stMain].Segment);
    ObjectFilePtr := Ptr (ObjectFileSegment, 0);
    Return := False;
    Repeat
      If Ofs (ObjectFilePtr^) &gt;= ObjectFileSize then FileError (@Identifier, InvalidObjectFileRecord);
      RecordType := ObjectFilePtr^.RecordType;
      If Odd (RecordType) or (RecordType &lt; $80) or (RecordType &gt; $A2) then FileError (@Identifier, InvalidObjectFileRecord);
      ObjectRecordChecksumOffset := ObjectFilePtr^.RecordLength + Ofs (ObjectFilePtr^) + 2;
      LinkProc [(RecordType - $80) shr 1];
      If Return then Exit;
      ObjectFilePtr := Ptr (ObjectFileSegment, ObjectRecordChecksumOffset + 1);
    until False;
  end;

begin

{$IFDEF DEBUGOMF}
  Writeln (#13#10'#OMF IMPORT BEGIN');
{$ENDIF}

  UsedFilesBlockRecord := Ptr (SymbolTable [stUsedFiles].Segment, 0);
  While Ofs (UsedFilesBlockRecord^) &lt;&gt; SymbolTable [stUsedFiles].UsedSize do
    begin
      If UsedFilesBlockRecord^.FileType = ufObjectFile then
        begin
          ReadObjectFile;
          ProcessObjectFileRecords;
          Inc (HeapEndRec.Seg, ObjectFileParagraphSize);
          CreateProgramCodeBlockRecord;
          CreateTypedConstantsBlockRecord;
          CreateVariablesBlockRecord;
          UsedFilesBlockRecord := Ptr (SymbolTable [stUsedFiles].Segment, Ofs (UsedFilesBlockRecord^));
        end;
      Inc (PChar (UsedFilesBlockRecord), UsedFilesBlockRecord^.Name.Len + 8);
    end;

{$IFDEF DEBUGOMF}
  Writeln ('#OMF IMPORT END');
{$ENDIF}

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
     PObjectFileRecord = ^TObjectFileRecord;
     TObjectFileRecord = Record
                           RecordType: Byte;
                           RecordLength: Word;
                           Case Byte of
                             $88: (Comment: Record
                                              ComType: Byte;
                                              Class: Byte;
                                              Case Byte of
                                                $00: (Translator: String);
                                                $98: (SourceFileNumber: Byte; SourceFileName: String);
                                              end);
                             $8C: (ExternalName: String);
                             $90: (BaseGroupIndex, BaseSegmentIndex: Byte);
                             $96: (Name: String);
                             $98: (SegDef: TSegDef);
                             $A0: (EnumSegmentIndex: Byte; EnumeratedDataOffset: Word; EnumData: Byte);
                             $A2: (ItSegmentIndex: Byte; IteratedDataOffset: Word);
                         end;
```

### Code Block 2

```pascal
Procedure ImportObjectFiles;
Var UsedFilesBlockRecord: PUsedFilesBlockRecord;
    ObjectFile: File;
    ObjectFileParagraphSize: Word;

  Procedure ReadObjectFile;
  Var ObjectFileSizeRec: LongRec;
  begin
    FindUsedFilePath (UsedFilesBlockRecord, @Identifier);
    OpenFile (ObjectFile, @Identifier);

{$IFDEF DEBUGOMF}
    Writeln ('        OBJECT FILE: ', PChar (@Identifier));
{$ENDIF}

    ObjectFileSizeRec.Long := FileSize (ObjectFile);
    If (ObjectFileSizeRec.WordH &lt;&gt; 0) or (ObjectFileSizeRec.WordL &gt; $FFF0) then FileError (@Identifier, ObjectFileTooLarge);
    ObjectFileSize := ObjectFileSizeRec.WordL;
    ObjectFileParagraphSize := (ObjectFileSizeRec.WordL + $000F) shr 4;
    If HeapEndRec.Seg - ObjectFileParagraphSize &lt; HeapPtrRec.Seg then Error (OutOfMemory);
    Dec (HeapEndRec.Seg, ObjectFileParagraphSize);
    ObjectFileSegment := HeapEndRec.Seg;
    BlockRead (ObjectFile, HeapEnd^, ObjectFileSizeRec.WordL);
    Close (ObjectFile);
  end;

  Procedure ProcessObjectFileRecords;
  Const LinkProc: Array [0..$11] of Procedure = (
    OMF_Dummy,
    OMF_Dummy,
    OMF_Dummy,
    OMF_Dummy,
    OMF_COMENT,
    OMF_MODEND,
    OMF_EXTDEF,
    OMF_Dummy,
    OMF_PUBDEF,
    OMF_Dummy,
    OMF_LINNUM,
    OMF_Dummy,
    OMF_SEGDEF,
    OMF_Dummy,
    OMF_FIXUPP,
    OMF_Dummy,
    OMF_LEDATA,
    OMF_LIDATA);
  Var RecordType: Byte;
  begin
    SegmentCounter := 0;
    CodeSegmentIndex := 0;
    TypedConstantsSegmentIndex := 0;
    VariablesSegmentIndex := 0;
    ReferencesSymbolTable := stMain;
    LastSourceFileNumber := 0;
    SourceFileNumber := 0;
    ExtrnDefinitionsCounter := 0;
    OMF_ReferencedModule := AddReferencedModule (SymbolTable [stMain].Segment);
    ObjectFilePtr := Ptr (ObjectFileSegment, 0);
    Return := False;
    Repeat
      If Ofs (ObjectFilePtr^) &gt;= ObjectFileSize then FileError (@Identifier, InvalidObjectFileRecord);
      RecordType := ObjectFilePtr^.RecordType;
      If Odd (RecordType) or (RecordType &lt; $80) or (RecordType &gt; $A2) then FileError (@Identifier, InvalidObjectFileRecord);
      ObjectRecordChecksumOffset := ObjectFilePtr^.RecordLength + Ofs (ObjectFilePtr^) + 2;
      LinkProc [(RecordType - $80) shr 1];
      If Return then Exit;
      ObjectFilePtr := Ptr (ObjectFileSegment, ObjectRecordChecksumOffset + 1);
    until False;
  end;

begin

{$IFDEF DEBUGOMF}
  Writeln (#13#10'#OMF IMPORT BEGIN');
{$ENDIF}

  UsedFilesBlockRecord := Ptr (SymbolTable [stUsedFiles].Segment, 0);
  While Ofs (UsedFilesBlockRecord^) &lt;&gt; SymbolTable [stUsedFiles].UsedSize do
    begin
      If UsedFilesBlockRecord^.FileType = ufObjectFile then
        begin
          ReadObjectFile;
          ProcessObjectFileRecords;
          Inc (HeapEndRec.Seg, ObjectFileParagraphSize);
          CreateProgramCodeBlockRecord;
          CreateTypedConstantsBlockRecord;
          CreateVariablesBlockRecord;
          UsedFilesBlockRecord := Ptr (SymbolTable [stUsedFiles].Segment, Ofs (UsedFilesBlockRecord^));
        end;
      Inc (PChar (UsedFilesBlockRecord), UsedFilesBlockRecord^.Name.Len + 8);
    end;

{$IFDEF DEBUGOMF}
  Writeln ('#OMF IMPORT END');
{$ENDIF}

end;
```

### Code Block 3

```pascal
     PObjectFileRecord = ^TObjectFileRecord;
     TObjectFileRecord = Record
                           RecordType: Byte;
                           RecordLength: Word;
                           Case Byte of
                             $88: (Comment: Record
                                              ComType: Byte;
                                              Class: Byte;
                                              Case Byte of
                                                $00: (Translator: String);
                                                $98: (SourceFileNumber: Byte; SourceFileName: String);
                                              end);
                             $8C: (ExternalName: String);
                             $90: (BaseGroupIndex, BaseSegmentIndex: Byte);
                             $96: (Name: String);
                             $98: (SegDef: TSegDef);
                             $A0: (EnumSegmentIndex: Byte; EnumeratedDataOffset: Word; EnumData: Byte);
                             $A2: (ItSegmentIndex: Byte; IteratedDataOffset: Word);
                         end;
```

### Code Block 4

```pascal
Procedure ImportObjectFiles;
Var UsedFilesBlockRecord: PUsedFilesBlockRecord;
    ObjectFile: File;
    ObjectFileParagraphSize: Word;

  Procedure ReadObjectFile;
  Var ObjectFileSizeRec: LongRec;
  begin
    FindUsedFilePath (UsedFilesBlockRecord, @Identifier);
    OpenFile (ObjectFile, @Identifier);

{$IFDEF DEBUGOMF}
    Writeln ('        OBJECT FILE: ', PChar (@Identifier));
{$ENDIF}

    ObjectFileSizeRec.Long := FileSize (ObjectFile);
    If (ObjectFileSizeRec.WordH &lt;&gt; 0) or (ObjectFileSizeRec.WordL &gt; $FFF0) then FileError (@Identifier, ObjectFileTooLarge);
    ObjectFileSize := ObjectFileSizeRec.WordL;
    ObjectFileParagraphSize := (ObjectFileSizeRec.WordL + $000F) shr 4;
    If HeapEndRec.Seg - ObjectFileParagraphSize &lt; HeapPtrRec.Seg then Error (OutOfMemory);
    Dec (HeapEndRec.Seg, ObjectFileParagraphSize);
    ObjectFileSegment := HeapEndRec.Seg;
    BlockRead (ObjectFile, HeapEnd^, ObjectFileSizeRec.WordL);
    Close (ObjectFile);
  end;

  Procedure ProcessObjectFileRecords;
  Const LinkProc: Array [0..$11] of Procedure = (
    OMF_Dummy,
    OMF_Dummy,
    OMF_Dummy,
    OMF_Dummy,
    OMF_COMENT,
    OMF_MODEND,
    OMF_EXTDEF,
    OMF_Dummy,
    OMF_PUBDEF,
    OMF_Dummy,
    OMF_LINNUM,
    OMF_Dummy,
    OMF_SEGDEF,
    OMF_Dummy,
    OMF_FIXUPP,
    OMF_Dummy,
    OMF_LEDATA,
    OMF_LIDATA);
  Var RecordType: Byte;
  begin
    SegmentCounter := 0;
    CodeSegmentIndex := 0;
    TypedConstantsSegmentIndex := 0;
    VariablesSegmentIndex := 0;
    ReferencesSymbolTable := stMain;
    LastSourceFileNumber := 0;
    SourceFileNumber := 0;
    ExtrnDefinitionsCounter := 0;
    OMF_ReferencedModule := AddReferencedModule (SymbolTable [stMain].Segment);
    ObjectFilePtr := Ptr (ObjectFileSegment, 0);
    Return := False;
    Repeat
      If Ofs (ObjectFilePtr^) &gt;= ObjectFileSize then FileError (@Identifier, InvalidObjectFileRecord);
      RecordType := ObjectFilePtr^.RecordType;
      If Odd (RecordType) or (RecordType &lt; $80) or (RecordType &gt; $A2) then FileError (@Identifier, InvalidObjectFileRecord);
      ObjectRecordChecksumOffset := ObjectFilePtr^.RecordLength + Ofs (ObjectFilePtr^) + 2;
      LinkProc [(RecordType - $80) shr 1];
      If Return then Exit;
      ObjectFilePtr := Ptr (ObjectFileSegment, ObjectRecordChecksumOffset + 1);
    until False;
  end;

begin

{$IFDEF DEBUGOMF}
  Writeln (#13#10'#OMF IMPORT BEGIN');
{$ENDIF}

  UsedFilesBlockRecord := Ptr (SymbolTable [stUsedFiles].Segment, 0);
  While Ofs (UsedFilesBlockRecord^) &lt;&gt; SymbolTable [stUsedFiles].UsedSize do
    begin
      If UsedFilesBlockRecord^.FileType = ufObjectFile then
        begin
          ReadObjectFile;
          ProcessObjectFileRecords;
          Inc (HeapEndRec.Seg, ObjectFileParagraphSize);
          CreateProgramCodeBlockRecord;
          CreateTypedConstantsBlockRecord;
          CreateVariablesBlockRecord;
          UsedFilesBlockRecord := Ptr (SymbolTable [stUsedFiles].Segment, Ofs (UsedFilesBlockRecord^));
        end;
      Inc (PChar (UsedFilesBlockRecord), UsedFilesBlockRecord^.Name.Len + 8);
    end;

{$IFDEF DEBUGOMF}
  Writeln ('#OMF IMPORT END');
{$ENDIF}

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
- https://turbopascal.org/importing-coment-object-file-record
- https://turbopascal.org/importing-extdef-object-file-record
- https://turbopascal.org/importing-fixupp-object-file-record
- https://turbopascal.org/importing-ledata-and-lidata-object-file-records
- https://turbopascal.org/importing-linnum-object-file-record
- https://turbopascal.org/importing-modend-object-file-record
- https://turbopascal.org/importing-object-files
- https://turbopascal.org/importing-pubdef-object-file-record
- https://turbopascal.org/importing-segdef-object-file-record
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

