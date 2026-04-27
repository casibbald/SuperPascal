# Importing EXTDEF Object File Record - Turbo Pascal Compiler

**URL:** https://turbopascal.org/importing-extdef-object-file-record

---

## Table of Contents

- Importing EXTDEF Object File Record

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Importing EXTDEF Object File Record
This procedure processes
EXTDEF
object file record (
$8C
) and adds record to
ExtrnDefinitions
array for each
EXTRN
definition.
Type ExtrnDefinitions: Array [- 22..$FF] of TExtrnRecord absolute FileBuffer;

    TExtrnRecord = Record
                     edReferencedModuleAndFlags: Word;
                     edBlockRecordOffset: Word;
                     edOffset: Word;
                   end;

Procedure OMF_EXTDEF; Far;
Var IdentifierOffset: Word;
    TypeIdentifierData: PTypeIdentifierData;
    VarIdentifierData: PVariableIdentifierData absolute TypeIdentifierData;
    ProcedureIdentifierData: PProcedureIdentifierData absolute TypeIdentifierData;
    IdentifierToken: TToken;
    ObjectTypeDefinition: PObjectTypeDefinition;
    ReferenceFlagSet: Word;
    ReferencedModuleAndFlags, BlockRecordOffset, Offset: Word;

{$IFDEF DEBUGOMF}
    ExtNameCounter: Word;
{$ENDIF}

Label AddExtrnProcDefinitionRecord, AddExtrnDefinitionRecord;

  Procedure Error_Invalid_EXTRN_Definition;
  begin
    IdentifierError (PString (Ptr (ObjectFileSegment, ObjectErrorStringOfs))^, Invalid_EXTRN_Definition);
  end;

  Function IsFixup: Boolean;
  Type TData6 = Record
                  Name: String [6];
                  Data, Fixup: Word;
                end;
  Const Data6: Array [1..9] of TData6 = (
    (Name: 'FIARQQ'; Data: 0001; Fixup: $FE32),
    (Name: 'FISRQQ'; Data: 0002; Fixup: $0632),
    (Name: 'FICRQQ'; Data: 0003; Fixup: $0E32),
    (Name: 'FIERQQ'; Data: 0004; Fixup: $1632),
    (Name: 'FIDRQQ'; Data: 0005; Fixup: $5C32),
    (Name: 'FIWRQQ'; Data: 0006; Fixup: $A23D),
    (Name: 'FJARQQ'; Data: 0000; Fixup: $4000),
    (Name: 'FJCRQQ'; Data: 0000; Fixup: $C000),
    (Name: 'FJSRQQ'; Data: 0000; Fixup: $8000));
  Var N: Byte;

  begin
    IsFixup := False;
    If Length (CurrentIdentifier) <> 6 then Exit;
    For N := 1 to 9 do
      If CurrentIdentifier = Data6 [N].Name then
        begin
          BlockRecordOffset := Data6 [N].Data;
          Offset            := Data6 [N].Fixup;
          IsFixup := True;
          Exit;
        end;
  end;

begin
  Inc (PChar (ObjectFilePtr), 3);

{$IFDEF DEBUGOMF}
  Write ('        EXTDEF: ');
  ExtNameCounter := 0;
{$ENDIF}

  Repeat

{$IFDEF DEBUGOMF}
    If ExtNameCounter <> 0 then Write ('                ');
    Writeln (PString (ObjectFilePtr)^);
    Inc (ExtNameCounter);
{$ENDIF}

    SetObjectNameData (PString (ObjectFilePtr));
    If Copy_OMF_RecordIdentifierToCurrentIdentifier then
      begin
        ReferencedModuleAndFlags := $FFFF;
        If IsFixup then GoTo AddExtrnDefinitionRecord;
        If not FindCurrentIdentifier (IdentifierToken, IdentifierOffset, Pointer (VarIdentifierData)) then
          AddProcedureIdentifier (IdentifierToken, ProcedureIdentifierData);
        Case IdentifierToken of
          Token_ProcedureIdentifier: GoTo AddExtrnProcDefinitionRecord;
          Token_VariableIdentifier:
            begin
              While vfAbsoluteVar in VarIdentifierData^.Flags do
                VarIdentifierData := PVariableIdentifierData (PointerFromOffsets (VarIdentifierData^.AbsoluteVarDataOffsets));
              ReferenceFlagSet := $8000;
              If (VarIdentifierData^.Flags * VariableTypeMask) <> GlobalVariable then
                begin
                  ReferenceFlagSet := $C000;
                  If (VarIdentifierData^.Flags * VariableTypeMask) <> TypedConstant then
                    Error_Invalid_EXTRN_Definition;
                end;
              ReferencedModuleAndFlags := ReferenceFlagSet or AddReferencedModule (Seg (VarIdentifierData^));
              BlockRecordOffset := VarIdentifierData^.W1.Seg;
              Offset := VarIdentifierData^.W1.Ofs;
              GoTo AddExtrnDefinitionRecord;
            end;
          else Error_Invalid_EXTRN_Definition;
        end;
      end;
    If not FindCurrentIdentifier (IdentifierToken, IdentifierOffset, Pointer (TypeIdentifierData)) then
      Error_Invalid_EXTRN_Definition;
    If IdentifierToken <> Token_TypeIdentifier then Error_Invalid_EXTRN_Definition;
    ObjectTypeDefinition := PointerFromOffsets (TypeIdentifierData^.UnitTypeOffsets);
    If ObjectTypeDefinition^.BaseType <> btObject then Error_Invalid_EXTRN_Definition;
    Copy_OMF_RecordIdentifierToCurrentIdentifier;
    If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (ObjectTypeDefinition,
                                                         PVariableIdentifierData (ProcedureIdentifierData),
                                                         IdentifierToken, IdentifierOffset) then
      Error_Invalid_EXTRN_Definition;
    If IdentifierToken <> Token_ProcedureIdentifier then Error_Invalid_EXTRN_Definition;

AddExtrnProcDefinitionRecord:
    If pfInline in ProcedureIdentifierData^.Flags then Error_Invalid_EXTRN_Definition;
    ReferencedModuleAndFlags := ReferenceFlagSet or AddReferencedModule (Seg (VarIdentifierData^));
    BlockRecordOffset := ProcedureIdentifierData^.ProceduresRecordOffset;
    Offset := 0;

AddExtrnDefinitionRecord:
    If ExtrnDefinitionsCounter = $100 then FileError (@Identifier, TooMany_EXTRN_Definitions);
    With ExtrnDefinitions [ExtrnDefinitionsCounter] do
      begin
        edReferencedModuleAndFlags := ReferencedModuleAndFlags;
        edBlockRecordOffset        := BlockRecordOffset;
        edOffset := Offset;
      end;
    Inc (ExtrnDefinitionsCounter);
    Inc (PChar (ObjectFilePtr));
  until Ofs (ObjectFilePtr^) >= ObjectRecordChecksumOffset;
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
Type ExtrnDefinitions: Array [- 22..$FF] of TExtrnRecord absolute FileBuffer;

    TExtrnRecord = Record
                     edReferencedModuleAndFlags: Word;
                     edBlockRecordOffset: Word;
                     edOffset: Word;
                   end;

Procedure OMF_EXTDEF; Far;
Var IdentifierOffset: Word;
    TypeIdentifierData: PTypeIdentifierData;
    VarIdentifierData: PVariableIdentifierData absolute TypeIdentifierData;
    ProcedureIdentifierData: PProcedureIdentifierData absolute TypeIdentifierData;
    IdentifierToken: TToken;
    ObjectTypeDefinition: PObjectTypeDefinition;
    ReferenceFlagSet: Word;
    ReferencedModuleAndFlags, BlockRecordOffset, Offset: Word;

{$IFDEF DEBUGOMF}
    ExtNameCounter: Word;
{$ENDIF}

Label AddExtrnProcDefinitionRecord, AddExtrnDefinitionRecord;

  Procedure Error_Invalid_EXTRN_Definition;
  begin
    IdentifierError (PString (Ptr (ObjectFileSegment, ObjectErrorStringOfs))^, Invalid_EXTRN_Definition);
  end;

  Function IsFixup: Boolean;
  Type TData6 = Record
                  Name: String [6];
                  Data, Fixup: Word;
                end;
  Const Data6: Array [1..9] of TData6 = (
    (Name: 'FIARQQ'; Data: 0001; Fixup: $FE32),
    (Name: 'FISRQQ'; Data: 0002; Fixup: $0632),
    (Name: 'FICRQQ'; Data: 0003; Fixup: $0E32),
    (Name: 'FIERQQ'; Data: 0004; Fixup: $1632),
    (Name: 'FIDRQQ'; Data: 0005; Fixup: $5C32),
    (Name: 'FIWRQQ'; Data: 0006; Fixup: $A23D),
    (Name: 'FJARQQ'; Data: 0000; Fixup: $4000),
    (Name: 'FJCRQQ'; Data: 0000; Fixup: $C000),
    (Name: 'FJSRQQ'; Data: 0000; Fixup: $8000));
  Var N: Byte;

  begin
    IsFixup := False;
    If Length (CurrentIdentifier) <> 6 then Exit;
    For N := 1 to 9 do
      If CurrentIdentifier = Data6 [N].Name then
        begin
          BlockRecordOffset := Data6 [N].Data;
          Offset            := Data6 [N].Fixup;
          IsFixup := True;
          Exit;
        end;
  end;

begin
  Inc (PChar (ObjectFilePtr), 3);

{$IFDEF DEBUGOMF}
  Write ('        EXTDEF: ');
  ExtNameCounter := 0;
{$ENDIF}

  Repeat

{$IFDEF DEBUGOMF}
    If ExtNameCounter <> 0 then Write ('                ');
    Writeln (PString (ObjectFilePtr)^);
    Inc (ExtNameCounter);
{$ENDIF}

    SetObjectNameData (PString (ObjectFilePtr));
    If Copy_OMF_RecordIdentifierToCurrentIdentifier then
      begin
        ReferencedModuleAndFlags := $FFFF;
        If IsFixup then GoTo AddExtrnDefinitionRecord;
        If not FindCurrentIdentifier (IdentifierToken, IdentifierOffset, Pointer (VarIdentifierData)) then
          AddProcedureIdentifier (IdentifierToken, ProcedureIdentifierData);
        Case IdentifierToken of
          Token_ProcedureIdentifier: GoTo AddExtrnProcDefinitionRecord;
          Token_VariableIdentifier:
            begin
              While vfAbsoluteVar in VarIdentifierData^.Flags do
                VarIdentifierData := PVariableIdentifierData (PointerFromOffsets (VarIdentifierData^.AbsoluteVarDataOffsets));
              ReferenceFlagSet := $8000;
              If (VarIdentifierData^.Flags * VariableTypeMask) <> GlobalVariable then
                begin
                  ReferenceFlagSet := $C000;
                  If (VarIdentifierData^.Flags * VariableTypeMask) <> TypedConstant then
                    Error_Invalid_EXTRN_Definition;
                end;
              ReferencedModuleAndFlags := ReferenceFlagSet or AddReferencedModule (Seg (VarIdentifierData^));
              BlockRecordOffset := VarIdentifierData^.W1.Seg;
              Offset := VarIdentifierData^.W1.Ofs;
              GoTo AddExtrnDefinitionRecord;
            end;
          else Error_Invalid_EXTRN_Definition;
        end;
      end;
    If not FindCurrentIdentifier (IdentifierToken, IdentifierOffset, Pointer (TypeIdentifierData)) then
      Error_Invalid_EXTRN_Definition;
    If IdentifierToken <> Token_TypeIdentifier then Error_Invalid_EXTRN_Definition;
    ObjectTypeDefinition := PointerFromOffsets (TypeIdentifierData^.UnitTypeOffsets);
    If ObjectTypeDefinition^.BaseType <> btObject then Error_Invalid_EXTRN_Definition;
    Copy_OMF_RecordIdentifierToCurrentIdentifier;
    If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (ObjectTypeDefinition,
                                                         PVariableIdentifierData (ProcedureIdentifierData),
                                                         IdentifierToken, IdentifierOffset) then
      Error_Invalid_EXTRN_Definition;
    If IdentifierToken <> Token_ProcedureIdentifier then Error_Invalid_EXTRN_Definition;

AddExtrnProcDefinitionRecord:
    If pfInline in ProcedureIdentifierData^.Flags then Error_Invalid_EXTRN_Definition;
    ReferencedModuleAndFlags := ReferenceFlagSet or AddReferencedModule (Seg (VarIdentifierData^));
    BlockRecordOffset := ProcedureIdentifierData^.ProceduresRecordOffset;
    Offset := 0;

AddExtrnDefinitionRecord:
    If ExtrnDefinitionsCounter = $100 then FileError (@Identifier, TooMany_EXTRN_Definitions);
    With ExtrnDefinitions [ExtrnDefinitionsCounter] do
      begin
        edReferencedModuleAndFlags := ReferencedModuleAndFlags;
        edBlockRecordOffset        := BlockRecordOffset;
        edOffset := Offset;
      end;
    Inc (ExtrnDefinitionsCounter);
    Inc (PChar (ObjectFilePtr));
  until Ofs (ObjectFilePtr^) >= ObjectRecordChecksumOffset;
end;
```

### Code Block 2

```pascal
Type ExtrnDefinitions: Array [- 22..$FF] of TExtrnRecord absolute FileBuffer;

    TExtrnRecord = Record
                     edReferencedModuleAndFlags: Word;
                     edBlockRecordOffset: Word;
                     edOffset: Word;
                   end;

Procedure OMF_EXTDEF; Far;
Var IdentifierOffset: Word;
    TypeIdentifierData: PTypeIdentifierData;
    VarIdentifierData: PVariableIdentifierData absolute TypeIdentifierData;
    ProcedureIdentifierData: PProcedureIdentifierData absolute TypeIdentifierData;
    IdentifierToken: TToken;
    ObjectTypeDefinition: PObjectTypeDefinition;
    ReferenceFlagSet: Word;
    ReferencedModuleAndFlags, BlockRecordOffset, Offset: Word;

{$IFDEF DEBUGOMF}
    ExtNameCounter: Word;
{$ENDIF}

Label AddExtrnProcDefinitionRecord, AddExtrnDefinitionRecord;

  Procedure Error_Invalid_EXTRN_Definition;
  begin
    IdentifierError (PString (Ptr (ObjectFileSegment, ObjectErrorStringOfs))^, Invalid_EXTRN_Definition);
  end;

  Function IsFixup: Boolean;
  Type TData6 = Record
                  Name: String [6];
                  Data, Fixup: Word;
                end;
  Const Data6: Array [1..9] of TData6 = (
    (Name: 'FIARQQ'; Data: 0001; Fixup: $FE32),
    (Name: 'FISRQQ'; Data: 0002; Fixup: $0632),
    (Name: 'FICRQQ'; Data: 0003; Fixup: $0E32),
    (Name: 'FIERQQ'; Data: 0004; Fixup: $1632),
    (Name: 'FIDRQQ'; Data: 0005; Fixup: $5C32),
    (Name: 'FIWRQQ'; Data: 0006; Fixup: $A23D),
    (Name: 'FJARQQ'; Data: 0000; Fixup: $4000),
    (Name: 'FJCRQQ'; Data: 0000; Fixup: $C000),
    (Name: 'FJSRQQ'; Data: 0000; Fixup: $8000));
  Var N: Byte;

  begin
    IsFixup := False;
    If Length (CurrentIdentifier) <> 6 then Exit;
    For N := 1 to 9 do
      If CurrentIdentifier = Data6 [N].Name then
        begin
          BlockRecordOffset := Data6 [N].Data;
          Offset            := Data6 [N].Fixup;
          IsFixup := True;
          Exit;
        end;
  end;

begin
  Inc (PChar (ObjectFilePtr), 3);

{$IFDEF DEBUGOMF}
  Write ('        EXTDEF: ');
  ExtNameCounter := 0;
{$ENDIF}

  Repeat

{$IFDEF DEBUGOMF}
    If ExtNameCounter <> 0 then Write ('                ');
    Writeln (PString (ObjectFilePtr)^);
    Inc (ExtNameCounter);
{$ENDIF}

    SetObjectNameData (PString (ObjectFilePtr));
    If Copy_OMF_RecordIdentifierToCurrentIdentifier then
      begin
        ReferencedModuleAndFlags := $FFFF;
        If IsFixup then GoTo AddExtrnDefinitionRecord;
        If not FindCurrentIdentifier (IdentifierToken, IdentifierOffset, Pointer (VarIdentifierData)) then
          AddProcedureIdentifier (IdentifierToken, ProcedureIdentifierData);
        Case IdentifierToken of
          Token_ProcedureIdentifier: GoTo AddExtrnProcDefinitionRecord;
          Token_VariableIdentifier:
            begin
              While vfAbsoluteVar in VarIdentifierData^.Flags do
                VarIdentifierData := PVariableIdentifierData (PointerFromOffsets (VarIdentifierData^.AbsoluteVarDataOffsets));
              ReferenceFlagSet := $8000;
              If (VarIdentifierData^.Flags * VariableTypeMask) <> GlobalVariable then
                begin
                  ReferenceFlagSet := $C000;
                  If (VarIdentifierData^.Flags * VariableTypeMask) <> TypedConstant then
                    Error_Invalid_EXTRN_Definition;
                end;
              ReferencedModuleAndFlags := ReferenceFlagSet or AddReferencedModule (Seg (VarIdentifierData^));
              BlockRecordOffset := VarIdentifierData^.W1.Seg;
              Offset := VarIdentifierData^.W1.Ofs;
              GoTo AddExtrnDefinitionRecord;
            end;
          else Error_Invalid_EXTRN_Definition;
        end;
      end;
    If not FindCurrentIdentifier (IdentifierToken, IdentifierOffset, Pointer (TypeIdentifierData)) then
      Error_Invalid_EXTRN_Definition;
    If IdentifierToken <> Token_TypeIdentifier then Error_Invalid_EXTRN_Definition;
    ObjectTypeDefinition := PointerFromOffsets (TypeIdentifierData^.UnitTypeOffsets);
    If ObjectTypeDefinition^.BaseType <> btObject then Error_Invalid_EXTRN_Definition;
    Copy_OMF_RecordIdentifierToCurrentIdentifier;
    If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (ObjectTypeDefinition,
                                                         PVariableIdentifierData (ProcedureIdentifierData),
                                                         IdentifierToken, IdentifierOffset) then
      Error_Invalid_EXTRN_Definition;
    If IdentifierToken <> Token_ProcedureIdentifier then Error_Invalid_EXTRN_Definition;

AddExtrnProcDefinitionRecord:
    If pfInline in ProcedureIdentifierData^.Flags then Error_Invalid_EXTRN_Definition;
    ReferencedModuleAndFlags := ReferenceFlagSet or AddReferencedModule (Seg (VarIdentifierData^));
    BlockRecordOffset := ProcedureIdentifierData^.ProceduresRecordOffset;
    Offset := 0;

AddExtrnDefinitionRecord:
    If ExtrnDefinitionsCounter = $100 then FileError (@Identifier, TooMany_EXTRN_Definitions);
    With ExtrnDefinitions [ExtrnDefinitionsCounter] do
      begin
        edReferencedModuleAndFlags := ReferencedModuleAndFlags;
        edBlockRecordOffset        := BlockRecordOffset;
        edOffset := Offset;
      end;
    Inc (ExtrnDefinitionsCounter);
    Inc (PChar (ObjectFilePtr));
  until Ofs (ObjectFilePtr^) >= ObjectRecordChecksumOffset;
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
- https://turbopascal.org/importing-extdef-object-file-record
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

