# Importing FIXUPP Object File Record - Turbo Pascal Compiler

**URL:** https://turbopascal.org/importing-fixupp-object-file-record

---

## Table of Contents

- Importing FIXUPP Object File Record

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Importing FIXUPP Object File Record
This procedure processes
FIXUPP
record (
$9C
) and creates reference record in the symbol table.
Procedure OMF_FIXUPP; Far;
Type P_FIXUP_Subrecord = ^T_FIXUP_Subrecord;
     T_FIXUP_Subrecord = Record
                           Locat: WordRec;
                           FixData: Byte;
                         end;
Var PSubrecord: P_FIXUP_Subrecord;
    PSubrecordByte: PChar absolute PSubrecord;
    ReferencedModuleAndFlags: Word;
    FixData: Byte;
    TargetDatum: Word;
    TargetDisplacement: Word;
    DataRecordOffset: Word;
    P: Pword;

  Procedure CreateRecord (OMF_ReferencedOffset: Word);
  Var OMF_ReferencedBlockRecordOffset: Word;
  begin
    If FixData and $02 <> 0 then
      begin
        ReferencedModuleAndFlags := ReferencedModuleAndFlags or ExtrnDefinitions [TargetDatum - 1].edReferencedModuleAndFlags;
        Inc (OMF_ReferencedOffset, ExtrnDefinitions [TargetDatum - 1].edOffset);
        OMF_ReferencedBlockRecordOffset := ExtrnDefinitions [TargetDatum - 1].edBlockRecordOffset;
        If ReferencedModuleAndFlags = $FFFF then
          begin
            MemW [DataBlockSegment : DataRecordOffset + LastBlockSize] := OMF_ReferencedOffset;
            Exit;
          end;
      end else
        If FixData and $01 <> 0 then
          begin
            ReferencedModuleAndFlags := ReferencedModuleAndFlags or $8000 or OMF_ReferencedModule;
            OMF_ReferencedBlockRecordOffset := SymbolTable [stVariablesBlocks].UsedSize;
          end else
            If TargetDatum = CodeSegmentIndex then
              begin
                ReferencedModuleAndFlags := ReferencedModuleAndFlags or $4000 or OMF_ReferencedModule;
                OMF_ReferencedBlockRecordOffset := SymbolTable [stCodeBlocks].UsedSize;
              end else
                If TargetDatum = TypedConstantsSegmentIndex then
                  begin
                    ForceCreationOfTypedConstantsBlockRecord := True;
                    ReferencedModuleAndFlags := ReferencedModuleAndFlags or $C000 or OMF_ReferencedModule;
                    OMF_ReferencedBlockRecordOffset := SymbolTable [stTypedConstantsBlocks].UsedSize;
                  end else
                    If TargetDatum = VariablesSegmentIndex then
                      begin
                        ForceCreationOfVariablesBlockRecord := True;
                        ReferencedModuleAndFlags := ReferencedModuleAndFlags or $8000 or OMF_ReferencedModule;
                        OMF_ReferencedBlockRecordOffset := SymbolTable [stVariablesBlocks].UsedSize;
                      end else FileError (@Identifier, InvalidRelocatableReference);
    P := IncreaseSymbolTable (ReferencesSymbolTable, 8);
    P^ := ReferencedModuleAndFlags;
    Inc (P);
    P^ := OMF_ReferencedBlockRecordOffset;
    Inc (P);
    P^ := OMF_ReferencedOffset;
    Inc (P);
    P^ := DataRecordOffset;
  end;

begin

{$IFDEF DEBUGOMF}
  Writeln ('        FIXUPP');
{$ENDIF}

  Inc (PChar (ObjectFilePtr), 3);
  PSubrecord := P_FIXUP_Subrecord (ObjectFilePtr);
  If ReferencesSymbolTable = stMain then FileError (@Identifier, InvalidRelocatableReference);
  Repeat
    Case PSubrecord^.Locat.ByteL and $FC of
      $84: ReferencedModuleAndFlags := $0000;
      $C4: ReferencedModuleAndFlags := $1000;
      $C8: ReferencedModuleAndFlags := $2000;
      $CC: ReferencedModuleAndFlags := $3000;
      else FileError (@Identifier, InvalidRelocatableReference);
    end;
    DataRecordOffset := Swap (PSubrecord^.Locat.Word) and $3FF + EnumeratedDataOffset;
    FixData := PSubrecord^.FixData;
    Inc (PSubrecordByte, 3);
    If FixData and $88 <> 0 then FileError (@Identifier, InvalidRelocatableReference);
    If FixData and $40 = 0 then
      begin                                                                           { Skip Frame Datum }
        If Byte (PSubrecordByte^) and $80 <> 0 then Inc (PSubrecordByte, 2)
          else Inc (PSubrecordByte);
      end;
    TargetDatum := Byte (PSubrecordByte^);
    Inc (PSubrecordByte);
    If TargetDatum and $80 <> 0 then
      begin
        TargetDatum := Swap (TargetDatum and $7F) or Byte (PSubrecordByte^);
        Inc (PSubrecordByte);
      end;
    TargetDisplacement := 0;
    If FixData and $04 = 0 then
      begin
        TargetDisplacement := PWord (PSubrecordByte)^;
        Inc (PSubrecordByte, 2);
      end;
    CreateRecord (TargetDisplacement + MemW [DataBlockSegment : DataRecordOffset + LastBlockSize]);
  until Ofs (PSubrecordByte^) >= ObjectRecordChecksumOffset;
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
Procedure OMF_FIXUPP; Far;
Type P_FIXUP_Subrecord = ^T_FIXUP_Subrecord;
     T_FIXUP_Subrecord = Record
                           Locat: WordRec;
                           FixData: Byte;
                         end;
Var PSubrecord: P_FIXUP_Subrecord;
    PSubrecordByte: PChar absolute PSubrecord;
    ReferencedModuleAndFlags: Word;
    FixData: Byte;
    TargetDatum: Word;
    TargetDisplacement: Word;
    DataRecordOffset: Word;
    P: Pword;

  Procedure CreateRecord (OMF_ReferencedOffset: Word);
  Var OMF_ReferencedBlockRecordOffset: Word;
  begin
    If FixData and $02 <> 0 then
      begin
        ReferencedModuleAndFlags := ReferencedModuleAndFlags or ExtrnDefinitions [TargetDatum - 1].edReferencedModuleAndFlags;
        Inc (OMF_ReferencedOffset, ExtrnDefinitions [TargetDatum - 1].edOffset);
        OMF_ReferencedBlockRecordOffset := ExtrnDefinitions [TargetDatum - 1].edBlockRecordOffset;
        If ReferencedModuleAndFlags = $FFFF then
          begin
            MemW [DataBlockSegment : DataRecordOffset + LastBlockSize] := OMF_ReferencedOffset;
            Exit;
          end;
      end else
        If FixData and $01 <> 0 then
          begin
            ReferencedModuleAndFlags := ReferencedModuleAndFlags or $8000 or OMF_ReferencedModule;
            OMF_ReferencedBlockRecordOffset := SymbolTable [stVariablesBlocks].UsedSize;
          end else
            If TargetDatum = CodeSegmentIndex then
              begin
                ReferencedModuleAndFlags := ReferencedModuleAndFlags or $4000 or OMF_ReferencedModule;
                OMF_ReferencedBlockRecordOffset := SymbolTable [stCodeBlocks].UsedSize;
              end else
                If TargetDatum = TypedConstantsSegmentIndex then
                  begin
                    ForceCreationOfTypedConstantsBlockRecord := True;
                    ReferencedModuleAndFlags := ReferencedModuleAndFlags or $C000 or OMF_ReferencedModule;
                    OMF_ReferencedBlockRecordOffset := SymbolTable [stTypedConstantsBlocks].UsedSize;
                  end else
                    If TargetDatum = VariablesSegmentIndex then
                      begin
                        ForceCreationOfVariablesBlockRecord := True;
                        ReferencedModuleAndFlags := ReferencedModuleAndFlags or $8000 or OMF_ReferencedModule;
                        OMF_ReferencedBlockRecordOffset := SymbolTable [stVariablesBlocks].UsedSize;
                      end else FileError (@Identifier, InvalidRelocatableReference);
    P := IncreaseSymbolTable (ReferencesSymbolTable, 8);
    P^ := ReferencedModuleAndFlags;
    Inc (P);
    P^ := OMF_ReferencedBlockRecordOffset;
    Inc (P);
    P^ := OMF_ReferencedOffset;
    Inc (P);
    P^ := DataRecordOffset;
  end;

begin

{$IFDEF DEBUGOMF}
  Writeln ('        FIXUPP');
{$ENDIF}

  Inc (PChar (ObjectFilePtr), 3);
  PSubrecord := P_FIXUP_Subrecord (ObjectFilePtr);
  If ReferencesSymbolTable = stMain then FileError (@Identifier, InvalidRelocatableReference);
  Repeat
    Case PSubrecord^.Locat.ByteL and $FC of
      $84: ReferencedModuleAndFlags := $0000;
      $C4: ReferencedModuleAndFlags := $1000;
      $C8: ReferencedModuleAndFlags := $2000;
      $CC: ReferencedModuleAndFlags := $3000;
      else FileError (@Identifier, InvalidRelocatableReference);
    end;
    DataRecordOffset := Swap (PSubrecord^.Locat.Word) and $3FF + EnumeratedDataOffset;
    FixData := PSubrecord^.FixData;
    Inc (PSubrecordByte, 3);
    If FixData and $88 <> 0 then FileError (@Identifier, InvalidRelocatableReference);
    If FixData and $40 = 0 then
      begin                                                                           { Skip Frame Datum }
        If Byte (PSubrecordByte^) and $80 <> 0 then Inc (PSubrecordByte, 2)
          else Inc (PSubrecordByte);
      end;
    TargetDatum := Byte (PSubrecordByte^);
    Inc (PSubrecordByte);
    If TargetDatum and $80 <> 0 then
      begin
        TargetDatum := Swap (TargetDatum and $7F) or Byte (PSubrecordByte^);
        Inc (PSubrecordByte);
      end;
    TargetDisplacement := 0;
    If FixData and $04 = 0 then
      begin
        TargetDisplacement := PWord (PSubrecordByte)^;
        Inc (PSubrecordByte, 2);
      end;
    CreateRecord (TargetDisplacement + MemW [DataBlockSegment : DataRecordOffset + LastBlockSize]);
  until Ofs (PSubrecordByte^) >= ObjectRecordChecksumOffset;
end;
```

### Code Block 2

```pascal
Procedure OMF_FIXUPP; Far;
Type P_FIXUP_Subrecord = ^T_FIXUP_Subrecord;
     T_FIXUP_Subrecord = Record
                           Locat: WordRec;
                           FixData: Byte;
                         end;
Var PSubrecord: P_FIXUP_Subrecord;
    PSubrecordByte: PChar absolute PSubrecord;
    ReferencedModuleAndFlags: Word;
    FixData: Byte;
    TargetDatum: Word;
    TargetDisplacement: Word;
    DataRecordOffset: Word;
    P: Pword;

  Procedure CreateRecord (OMF_ReferencedOffset: Word);
  Var OMF_ReferencedBlockRecordOffset: Word;
  begin
    If FixData and $02 <> 0 then
      begin
        ReferencedModuleAndFlags := ReferencedModuleAndFlags or ExtrnDefinitions [TargetDatum - 1].edReferencedModuleAndFlags;
        Inc (OMF_ReferencedOffset, ExtrnDefinitions [TargetDatum - 1].edOffset);
        OMF_ReferencedBlockRecordOffset := ExtrnDefinitions [TargetDatum - 1].edBlockRecordOffset;
        If ReferencedModuleAndFlags = $FFFF then
          begin
            MemW [DataBlockSegment : DataRecordOffset + LastBlockSize] := OMF_ReferencedOffset;
            Exit;
          end;
      end else
        If FixData and $01 <> 0 then
          begin
            ReferencedModuleAndFlags := ReferencedModuleAndFlags or $8000 or OMF_ReferencedModule;
            OMF_ReferencedBlockRecordOffset := SymbolTable [stVariablesBlocks].UsedSize;
          end else
            If TargetDatum = CodeSegmentIndex then
              begin
                ReferencedModuleAndFlags := ReferencedModuleAndFlags or $4000 or OMF_ReferencedModule;
                OMF_ReferencedBlockRecordOffset := SymbolTable [stCodeBlocks].UsedSize;
              end else
                If TargetDatum = TypedConstantsSegmentIndex then
                  begin
                    ForceCreationOfTypedConstantsBlockRecord := True;
                    ReferencedModuleAndFlags := ReferencedModuleAndFlags or $C000 or OMF_ReferencedModule;
                    OMF_ReferencedBlockRecordOffset := SymbolTable [stTypedConstantsBlocks].UsedSize;
                  end else
                    If TargetDatum = VariablesSegmentIndex then
                      begin
                        ForceCreationOfVariablesBlockRecord := True;
                        ReferencedModuleAndFlags := ReferencedModuleAndFlags or $8000 or OMF_ReferencedModule;
                        OMF_ReferencedBlockRecordOffset := SymbolTable [stVariablesBlocks].UsedSize;
                      end else FileError (@Identifier, InvalidRelocatableReference);
    P := IncreaseSymbolTable (ReferencesSymbolTable, 8);
    P^ := ReferencedModuleAndFlags;
    Inc (P);
    P^ := OMF_ReferencedBlockRecordOffset;
    Inc (P);
    P^ := OMF_ReferencedOffset;
    Inc (P);
    P^ := DataRecordOffset;
  end;

begin

{$IFDEF DEBUGOMF}
  Writeln ('        FIXUPP');
{$ENDIF}

  Inc (PChar (ObjectFilePtr), 3);
  PSubrecord := P_FIXUP_Subrecord (ObjectFilePtr);
  If ReferencesSymbolTable = stMain then FileError (@Identifier, InvalidRelocatableReference);
  Repeat
    Case PSubrecord^.Locat.ByteL and $FC of
      $84: ReferencedModuleAndFlags := $0000;
      $C4: ReferencedModuleAndFlags := $1000;
      $C8: ReferencedModuleAndFlags := $2000;
      $CC: ReferencedModuleAndFlags := $3000;
      else FileError (@Identifier, InvalidRelocatableReference);
    end;
    DataRecordOffset := Swap (PSubrecord^.Locat.Word) and $3FF + EnumeratedDataOffset;
    FixData := PSubrecord^.FixData;
    Inc (PSubrecordByte, 3);
    If FixData and $88 <> 0 then FileError (@Identifier, InvalidRelocatableReference);
    If FixData and $40 = 0 then
      begin                                                                           { Skip Frame Datum }
        If Byte (PSubrecordByte^) and $80 <> 0 then Inc (PSubrecordByte, 2)
          else Inc (PSubrecordByte);
      end;
    TargetDatum := Byte (PSubrecordByte^);
    Inc (PSubrecordByte);
    If TargetDatum and $80 <> 0 then
      begin
        TargetDatum := Swap (TargetDatum and $7F) or Byte (PSubrecordByte^);
        Inc (PSubrecordByte);
      end;
    TargetDisplacement := 0;
    If FixData and $04 = 0 then
      begin
        TargetDisplacement := PWord (PSubrecordByte)^;
        Inc (PSubrecordByte, 2);
      end;
    CreateRecord (TargetDisplacement + MemW [DataBlockSegment : DataRecordOffset + LastBlockSize]);
  until Ofs (PSubrecordByte^) >= ObjectRecordChecksumOffset;
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
- https://turbopascal.org/importing-fixupp-object-file-record
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

