# Importing SEGDEF Object File Record - Turbo Pascal Compiler

**URL:** https://turbopascal.org/importing-segdef-object-file-record

---

## Table of Contents

- Importing SEGDEF Object File Record

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Importing SEGDEF Object File Record
This procedure processes
SEGDEF
record (
$98
), finds appropriate Turbo Pascal segment and creates space in the symbol table for that segment.
Procedure OMF_SEGDEF; Far;
Var SegmentClass: Byte;
    SegmentLength: Word;
    SegDef: PSegDef;

  Function FindSegmentClass (SegmentNameIndex: Byte): Boolean;
  Type TSegData = Record
                    Name: String [5];
                    Class: Byte;
                  end;
  Const SegData: Array [1..8] of TSegData = (
          (Name:  'CODE'; Class: 0),
          (Name:  'CSEG'; Class: 0),
          (Name: '_TEXT'; Class: 0),
          (Name: 'CONST'; Class: 1),
          (Name: '_DATA'; Class: 1),
          (Name:  'DATA'; Class: 2),
          (Name:  'DSEG'; Class: 2),
          (Name:  '_BSS'; Class: 2));
  Var TempObjectFileRecordPtr,
      NextObjectFileRecordPtr: PObjectFileRecord;
      TempObjectFileRecordPtrRec: PtrRec absolute TempObjectFileRecordPtr;
      SegmentNameStr: String;
      N: Byte;
      NamePtr: PString;

{$IFDEF DEBUGOMF}
      SegmentName: String;
{$ENDIF}

  begin
    FindSegmentClass := False;
    TempObjectFileRecordPtr := Ptr (ObjectFileSegment, 0);
    Repeat
      NextObjectFileRecordPtr := Ptr (ObjectFileSegment,
                                      Ofs (TempObjectFileRecordPtr^) + TempObjectFileRecordPtr^.RecordLength + 3);
      If TempObjectFileRecordPtr^.RecordType = $96 then { LNAMES }
        begin
          Dec (PChar (NextObjectFileRecordPtr));
          NamePtr := @TempObjectFileRecordPtr^.Name;
          Repeat
            Dec (SegmentNameIndex);
            If SegmentNameIndex = 0 then
              begin
                For N := 1 to 8 do
                  begin
                    SegmentNameStr := NamePtr^;
{$IFDEF DEBUGOMF}
                    SegmentName := SegmentNameStr;
{$ENDIF}
                    If (Length (SegData [N].Name) < Length (SegmentNameStr)) and (SegData [N].Name [1] = '_') then
                      SegmentNameStr := Copy (SegmentNameStr, Length (SegmentNameStr) - Length (SegData [N].Name) + 1,
                                                              Length (SegData [N].Name));
                    If SegmentNameStr = SegData [N].Name then
                      begin
                        SegmentClass := SegData [N].Class;
{$IFDEF DEBUGOMF}
                        Writeln (SegmentName, ', Class: ', SegmentClass);
{$ENDIF}
                        FindSegmentClass := True;
                        Exit;
                      end;
                  end;
                Exit;
              end;
            Inc (PChar (NamePtr), Length (NamePtr^) + 1);
          until Pointer (NamePtr) = NextObjectFileRecordPtr;
          Inc (PChar (NextObjectFileRecordPtr));
        end;
      TempObjectFileRecordPtr := NextObjectFileRecordPtr;
    until False;
  end;

begin

{$IFDEF DEBUGOMF}
  Write ('        SEGDEF: ');
{$ENDIF}

  SegDef := @ObjectFilePtr^.SegDef;
  Repeat
    Inc (SegmentCounter);
    If SegDef^.SegmentAttributes and $E0 <> 0 then       { not absolute segment }
      begin
        SegmentLength := SegDef^.SegmentLength;
        If FindSegmentClass (SegDef^.SegmentNameIndex) then
          begin
            Case SegmentClass of
              0: If CodeSegmentIndex = 0 then
                   begin
                     CodeSegmentIndex := SegmentCounter;
                     FillChar (IncreaseSymbolTable (stCode, SegmentLength)^, SegmentLength, 0);
                   end;
              1: If TypedConstantsSegmentIndex = 0 then
                   begin
                     TypedConstantsSegmentIndex := SegmentCounter;
                     FillChar (IncreaseSymbolTable (stTypedConstants, SegmentLength)^, SegmentLength, 0);
                   end;
              else If VariablesSegmentIndex = 0 then
                     begin
                       VariablesSegmentIndex := SegmentCounter;
                       If LongInt (SizeOfVariablesInDataSegment) + SegmentLength > $FFFF then
                         FileError (@Identifier, DataSegmentTooLarge);
                       Inc (SizeOfVariablesInDataSegment, SegmentLength);
                     end;
            end;
          end;
        Inc (PChar (SegDef), 6);
      end else Inc (PChar (SegDef), 9);
  until Ofs (SegDef^) >= ObjectRecordChecksumOffset;
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
Procedure OMF_SEGDEF; Far;
Var SegmentClass: Byte;
    SegmentLength: Word;
    SegDef: PSegDef;

  Function FindSegmentClass (SegmentNameIndex: Byte): Boolean;
  Type TSegData = Record
                    Name: String [5];
                    Class: Byte;
                  end;
  Const SegData: Array [1..8] of TSegData = (
          (Name:  'CODE'; Class: 0),
          (Name:  'CSEG'; Class: 0),
          (Name: '_TEXT'; Class: 0),
          (Name: 'CONST'; Class: 1),
          (Name: '_DATA'; Class: 1),
          (Name:  'DATA'; Class: 2),
          (Name:  'DSEG'; Class: 2),
          (Name:  '_BSS'; Class: 2));
  Var TempObjectFileRecordPtr,
      NextObjectFileRecordPtr: PObjectFileRecord;
      TempObjectFileRecordPtrRec: PtrRec absolute TempObjectFileRecordPtr;
      SegmentNameStr: String;
      N: Byte;
      NamePtr: PString;

{$IFDEF DEBUGOMF}
      SegmentName: String;
{$ENDIF}

  begin
    FindSegmentClass := False;
    TempObjectFileRecordPtr := Ptr (ObjectFileSegment, 0);
    Repeat
      NextObjectFileRecordPtr := Ptr (ObjectFileSegment,
                                      Ofs (TempObjectFileRecordPtr^) + TempObjectFileRecordPtr^.RecordLength + 3);
      If TempObjectFileRecordPtr^.RecordType = $96 then { LNAMES }
        begin
          Dec (PChar (NextObjectFileRecordPtr));
          NamePtr := @TempObjectFileRecordPtr^.Name;
          Repeat
            Dec (SegmentNameIndex);
            If SegmentNameIndex = 0 then
              begin
                For N := 1 to 8 do
                  begin
                    SegmentNameStr := NamePtr^;
{$IFDEF DEBUGOMF}
                    SegmentName := SegmentNameStr;
{$ENDIF}
                    If (Length (SegData [N].Name) < Length (SegmentNameStr)) and (SegData [N].Name [1] = '_') then
                      SegmentNameStr := Copy (SegmentNameStr, Length (SegmentNameStr) - Length (SegData [N].Name) + 1,
                                                              Length (SegData [N].Name));
                    If SegmentNameStr = SegData [N].Name then
                      begin
                        SegmentClass := SegData [N].Class;
{$IFDEF DEBUGOMF}
                        Writeln (SegmentName, ', Class: ', SegmentClass);
{$ENDIF}
                        FindSegmentClass := True;
                        Exit;
                      end;
                  end;
                Exit;
              end;
            Inc (PChar (NamePtr), Length (NamePtr^) + 1);
          until Pointer (NamePtr) = NextObjectFileRecordPtr;
          Inc (PChar (NextObjectFileRecordPtr));
        end;
      TempObjectFileRecordPtr := NextObjectFileRecordPtr;
    until False;
  end;

begin

{$IFDEF DEBUGOMF}
  Write ('        SEGDEF: ');
{$ENDIF}

  SegDef := @ObjectFilePtr^.SegDef;
  Repeat
    Inc (SegmentCounter);
    If SegDef^.SegmentAttributes and $E0 <> 0 then       { not absolute segment }
      begin
        SegmentLength := SegDef^.SegmentLength;
        If FindSegmentClass (SegDef^.SegmentNameIndex) then
          begin
            Case SegmentClass of
              0: If CodeSegmentIndex = 0 then
                   begin
                     CodeSegmentIndex := SegmentCounter;
                     FillChar (IncreaseSymbolTable (stCode, SegmentLength)^, SegmentLength, 0);
                   end;
              1: If TypedConstantsSegmentIndex = 0 then
                   begin
                     TypedConstantsSegmentIndex := SegmentCounter;
                     FillChar (IncreaseSymbolTable (stTypedConstants, SegmentLength)^, SegmentLength, 0);
                   end;
              else If VariablesSegmentIndex = 0 then
                     begin
                       VariablesSegmentIndex := SegmentCounter;
                       If LongInt (SizeOfVariablesInDataSegment) + SegmentLength > $FFFF then
                         FileError (@Identifier, DataSegmentTooLarge);
                       Inc (SizeOfVariablesInDataSegment, SegmentLength);
                     end;
            end;
          end;
        Inc (PChar (SegDef), 6);
      end else Inc (PChar (SegDef), 9);
  until Ofs (SegDef^) >= ObjectRecordChecksumOffset;
end;
```

### Code Block 2

```pascal
Procedure OMF_SEGDEF; Far;
Var SegmentClass: Byte;
    SegmentLength: Word;
    SegDef: PSegDef;

  Function FindSegmentClass (SegmentNameIndex: Byte): Boolean;
  Type TSegData = Record
                    Name: String [5];
                    Class: Byte;
                  end;
  Const SegData: Array [1..8] of TSegData = (
          (Name:  'CODE'; Class: 0),
          (Name:  'CSEG'; Class: 0),
          (Name: '_TEXT'; Class: 0),
          (Name: 'CONST'; Class: 1),
          (Name: '_DATA'; Class: 1),
          (Name:  'DATA'; Class: 2),
          (Name:  'DSEG'; Class: 2),
          (Name:  '_BSS'; Class: 2));
  Var TempObjectFileRecordPtr,
      NextObjectFileRecordPtr: PObjectFileRecord;
      TempObjectFileRecordPtrRec: PtrRec absolute TempObjectFileRecordPtr;
      SegmentNameStr: String;
      N: Byte;
      NamePtr: PString;

{$IFDEF DEBUGOMF}
      SegmentName: String;
{$ENDIF}

  begin
    FindSegmentClass := False;
    TempObjectFileRecordPtr := Ptr (ObjectFileSegment, 0);
    Repeat
      NextObjectFileRecordPtr := Ptr (ObjectFileSegment,
                                      Ofs (TempObjectFileRecordPtr^) + TempObjectFileRecordPtr^.RecordLength + 3);
      If TempObjectFileRecordPtr^.RecordType = $96 then { LNAMES }
        begin
          Dec (PChar (NextObjectFileRecordPtr));
          NamePtr := @TempObjectFileRecordPtr^.Name;
          Repeat
            Dec (SegmentNameIndex);
            If SegmentNameIndex = 0 then
              begin
                For N := 1 to 8 do
                  begin
                    SegmentNameStr := NamePtr^;
{$IFDEF DEBUGOMF}
                    SegmentName := SegmentNameStr;
{$ENDIF}
                    If (Length (SegData [N].Name) < Length (SegmentNameStr)) and (SegData [N].Name [1] = '_') then
                      SegmentNameStr := Copy (SegmentNameStr, Length (SegmentNameStr) - Length (SegData [N].Name) + 1,
                                                              Length (SegData [N].Name));
                    If SegmentNameStr = SegData [N].Name then
                      begin
                        SegmentClass := SegData [N].Class;
{$IFDEF DEBUGOMF}
                        Writeln (SegmentName, ', Class: ', SegmentClass);
{$ENDIF}
                        FindSegmentClass := True;
                        Exit;
                      end;
                  end;
                Exit;
              end;
            Inc (PChar (NamePtr), Length (NamePtr^) + 1);
          until Pointer (NamePtr) = NextObjectFileRecordPtr;
          Inc (PChar (NextObjectFileRecordPtr));
        end;
      TempObjectFileRecordPtr := NextObjectFileRecordPtr;
    until False;
  end;

begin

{$IFDEF DEBUGOMF}
  Write ('        SEGDEF: ');
{$ENDIF}

  SegDef := @ObjectFilePtr^.SegDef;
  Repeat
    Inc (SegmentCounter);
    If SegDef^.SegmentAttributes and $E0 <> 0 then       { not absolute segment }
      begin
        SegmentLength := SegDef^.SegmentLength;
        If FindSegmentClass (SegDef^.SegmentNameIndex) then
          begin
            Case SegmentClass of
              0: If CodeSegmentIndex = 0 then
                   begin
                     CodeSegmentIndex := SegmentCounter;
                     FillChar (IncreaseSymbolTable (stCode, SegmentLength)^, SegmentLength, 0);
                   end;
              1: If TypedConstantsSegmentIndex = 0 then
                   begin
                     TypedConstantsSegmentIndex := SegmentCounter;
                     FillChar (IncreaseSymbolTable (stTypedConstants, SegmentLength)^, SegmentLength, 0);
                   end;
              else If VariablesSegmentIndex = 0 then
                     begin
                       VariablesSegmentIndex := SegmentCounter;
                       If LongInt (SizeOfVariablesInDataSegment) + SegmentLength > $FFFF then
                         FileError (@Identifier, DataSegmentTooLarge);
                       Inc (SizeOfVariablesInDataSegment, SegmentLength);
                     end;
            end;
          end;
        Inc (PChar (SegDef), 6);
      end else Inc (PChar (SegDef), 9);
  until Ofs (SegDef^) >= ObjectRecordChecksumOffset;
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

