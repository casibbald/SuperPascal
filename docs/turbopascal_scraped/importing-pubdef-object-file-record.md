# Importing PUBDEF Object File Record - Turbo Pascal Compiler

**URL:** https://turbopascal.org/importing-pubdef-object-file-record

---

## Table of Contents

- Importing PUBDEF Object File Record

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Importing PUBDEF Object File Record
This procedure processes
PUBDEF
object file record (
$90
) and sets data in
ProceduresRecord
to the procedure defined in the object file.
Procedure OMF_PUBDEF; Far;
Var PublicIdOffset: Word;
    IdentifierOffset: Word;
    TypeIdentifierDataPtr: PTypeIdentifierData;
    ProcedureIdentifierDataPtr: PProcedureIdentifierData absolute TypeIdentifierDataPtr;
    IdToken: TToken;
    ObjectTypeDefinition: PObjectTypeDefinition;
    ProceduresRecord: PProceduresBlockRecord;

{$IFDEF DEBUGOMF}
    PubNameCounter: Word;
{$ENDIF}


  Procedure Error_Invalid_PUBLIC_Definition;
  begin
    IdentifierError (PString (Ptr (ObjectFileSegment, ObjectErrorStringOfs))^, Invalid_PUBLIC_Definition);
  end;

begin

{$IFDEF DEBUGOMF}
  Write ('        PUBDEF: ');
  PubNameCounter := 0;
{$ENDIF}

  If ObjectFilePtr^.BaseGroupIndex <> 0 then FileError (@Identifier, Invalid_PUBLIC_Definition);
  If ObjectFilePtr^.BaseSegmentIndex <> CodeSegmentIndex then FileError (@Identifier, Invalid_PUBLIC_Definition);
  Inc (PChar (ObjectFilePtr), 5);
  Repeat

{$IFDEF DEBUGOMF}
    If PubNameCounter <> 0 then Write ('                ');
    Writeln (PString (ObjectFilePtr)^);
    Inc (PubNameCounter);
{$ENDIF}

    SetObjectNameData (PString (ObjectFilePtr));
    PublicIdOffset := Word (Pointer (ObjectFilePtr)^);
    Inc (PChar (ObjectFilePtr), 2);
    If not Copy_OMF_RecordIdentifierToCurrentIdentifier then
      begin
        If not CurrentIdentifierDeclaredInCurrentScope (IdentifierOffset, Pointer (TypeIdentifierDataPtr), IdToken) then
          Error_Invalid_PUBLIC_Definition;
        If IdToken <> Token_TypeIdentifier then Error_Invalid_PUBLIC_Definition;
        ObjectTypeDefinition := PointerFromOffsets (TypeIdentifierDataPtr^.UnitTypeOffsets);
        If ObjectTypeDefinition^.BaseType <> btObject then Error_Invalid_PUBLIC_Definition;
        Copy_OMF_RecordIdentifierToCurrentIdentifier;
        If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (ObjectTypeDefinition,
                                                                    PVariableIdentifierData (TypeIdentifierDataPtr),
                                                                    IdToken,
                                                                    IdentifierOffset) then
          Error_Invalid_PUBLIC_Definition;
      end else If not CurrentIdentifierDeclaredInCurrentScope (IdentifierOffset, Pointer (TypeIdentifierDataPtr), IdToken) then
                 AddProcedureIdentifier (IdToken, ProcedureIdentifierDataPtr);
    If IdToken <> Token_ProcedureIdentifier then Error_Invalid_PUBLIC_Definition;
    If not (pfExternal in ProcedureIdentifierDataPtr^.Flags) then Error_Invalid_PUBLIC_Definition;
    ProceduresRecord := Ptr (SymbolTable [stProcedures].Segment, ProcedureIdentifierDataPtr^.ProceduresRecordOffset);
    If ProceduresRecord^.ProgramCodeBlockRecordOffset <> $FFFF then Error_Invalid_PUBLIC_Definition;
    ProceduresRecord^.ProgramCodeBlockRecordOffset := SymbolTable [stCodeBlocks].UsedSize;
    ProceduresRecord^.ProcIdentifierOffset := PublicIdOffset;
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
Procedure OMF_PUBDEF; Far;
Var PublicIdOffset: Word;
    IdentifierOffset: Word;
    TypeIdentifierDataPtr: PTypeIdentifierData;
    ProcedureIdentifierDataPtr: PProcedureIdentifierData absolute TypeIdentifierDataPtr;
    IdToken: TToken;
    ObjectTypeDefinition: PObjectTypeDefinition;
    ProceduresRecord: PProceduresBlockRecord;

{$IFDEF DEBUGOMF}
    PubNameCounter: Word;
{$ENDIF}


  Procedure Error_Invalid_PUBLIC_Definition;
  begin
    IdentifierError (PString (Ptr (ObjectFileSegment, ObjectErrorStringOfs))^, Invalid_PUBLIC_Definition);
  end;

begin

{$IFDEF DEBUGOMF}
  Write ('        PUBDEF: ');
  PubNameCounter := 0;
{$ENDIF}

  If ObjectFilePtr^.BaseGroupIndex <> 0 then FileError (@Identifier, Invalid_PUBLIC_Definition);
  If ObjectFilePtr^.BaseSegmentIndex <> CodeSegmentIndex then FileError (@Identifier, Invalid_PUBLIC_Definition);
  Inc (PChar (ObjectFilePtr), 5);
  Repeat

{$IFDEF DEBUGOMF}
    If PubNameCounter <> 0 then Write ('                ');
    Writeln (PString (ObjectFilePtr)^);
    Inc (PubNameCounter);
{$ENDIF}

    SetObjectNameData (PString (ObjectFilePtr));
    PublicIdOffset := Word (Pointer (ObjectFilePtr)^);
    Inc (PChar (ObjectFilePtr), 2);
    If not Copy_OMF_RecordIdentifierToCurrentIdentifier then
      begin
        If not CurrentIdentifierDeclaredInCurrentScope (IdentifierOffset, Pointer (TypeIdentifierDataPtr), IdToken) then
          Error_Invalid_PUBLIC_Definition;
        If IdToken <> Token_TypeIdentifier then Error_Invalid_PUBLIC_Definition;
        ObjectTypeDefinition := PointerFromOffsets (TypeIdentifierDataPtr^.UnitTypeOffsets);
        If ObjectTypeDefinition^.BaseType <> btObject then Error_Invalid_PUBLIC_Definition;
        Copy_OMF_RecordIdentifierToCurrentIdentifier;
        If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (ObjectTypeDefinition,
                                                                    PVariableIdentifierData (TypeIdentifierDataPtr),
                                                                    IdToken,
                                                                    IdentifierOffset) then
          Error_Invalid_PUBLIC_Definition;
      end else If not CurrentIdentifierDeclaredInCurrentScope (IdentifierOffset, Pointer (TypeIdentifierDataPtr), IdToken) then
                 AddProcedureIdentifier (IdToken, ProcedureIdentifierDataPtr);
    If IdToken <> Token_ProcedureIdentifier then Error_Invalid_PUBLIC_Definition;
    If not (pfExternal in ProcedureIdentifierDataPtr^.Flags) then Error_Invalid_PUBLIC_Definition;
    ProceduresRecord := Ptr (SymbolTable [stProcedures].Segment, ProcedureIdentifierDataPtr^.ProceduresRecordOffset);
    If ProceduresRecord^.ProgramCodeBlockRecordOffset <> $FFFF then Error_Invalid_PUBLIC_Definition;
    ProceduresRecord^.ProgramCodeBlockRecordOffset := SymbolTable [stCodeBlocks].UsedSize;
    ProceduresRecord^.ProcIdentifierOffset := PublicIdOffset;
    Inc (PChar (ObjectFilePtr));
  until Ofs (ObjectFilePtr^) >= ObjectRecordChecksumOffset;
end;
```

### Code Block 2

```pascal
Procedure OMF_PUBDEF; Far;
Var PublicIdOffset: Word;
    IdentifierOffset: Word;
    TypeIdentifierDataPtr: PTypeIdentifierData;
    ProcedureIdentifierDataPtr: PProcedureIdentifierData absolute TypeIdentifierDataPtr;
    IdToken: TToken;
    ObjectTypeDefinition: PObjectTypeDefinition;
    ProceduresRecord: PProceduresBlockRecord;

{$IFDEF DEBUGOMF}
    PubNameCounter: Word;
{$ENDIF}


  Procedure Error_Invalid_PUBLIC_Definition;
  begin
    IdentifierError (PString (Ptr (ObjectFileSegment, ObjectErrorStringOfs))^, Invalid_PUBLIC_Definition);
  end;

begin

{$IFDEF DEBUGOMF}
  Write ('        PUBDEF: ');
  PubNameCounter := 0;
{$ENDIF}

  If ObjectFilePtr^.BaseGroupIndex <> 0 then FileError (@Identifier, Invalid_PUBLIC_Definition);
  If ObjectFilePtr^.BaseSegmentIndex <> CodeSegmentIndex then FileError (@Identifier, Invalid_PUBLIC_Definition);
  Inc (PChar (ObjectFilePtr), 5);
  Repeat

{$IFDEF DEBUGOMF}
    If PubNameCounter <> 0 then Write ('                ');
    Writeln (PString (ObjectFilePtr)^);
    Inc (PubNameCounter);
{$ENDIF}

    SetObjectNameData (PString (ObjectFilePtr));
    PublicIdOffset := Word (Pointer (ObjectFilePtr)^);
    Inc (PChar (ObjectFilePtr), 2);
    If not Copy_OMF_RecordIdentifierToCurrentIdentifier then
      begin
        If not CurrentIdentifierDeclaredInCurrentScope (IdentifierOffset, Pointer (TypeIdentifierDataPtr), IdToken) then
          Error_Invalid_PUBLIC_Definition;
        If IdToken <> Token_TypeIdentifier then Error_Invalid_PUBLIC_Definition;
        ObjectTypeDefinition := PointerFromOffsets (TypeIdentifierDataPtr^.UnitTypeOffsets);
        If ObjectTypeDefinition^.BaseType <> btObject then Error_Invalid_PUBLIC_Definition;
        Copy_OMF_RecordIdentifierToCurrentIdentifier;
        If not IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (ObjectTypeDefinition,
                                                                    PVariableIdentifierData (TypeIdentifierDataPtr),
                                                                    IdToken,
                                                                    IdentifierOffset) then
          Error_Invalid_PUBLIC_Definition;
      end else If not CurrentIdentifierDeclaredInCurrentScope (IdentifierOffset, Pointer (TypeIdentifierDataPtr), IdToken) then
                 AddProcedureIdentifier (IdToken, ProcedureIdentifierDataPtr);
    If IdToken <> Token_ProcedureIdentifier then Error_Invalid_PUBLIC_Definition;
    If not (pfExternal in ProcedureIdentifierDataPtr^.Flags) then Error_Invalid_PUBLIC_Definition;
    ProceduresRecord := Ptr (SymbolTable [stProcedures].Segment, ProcedureIdentifierDataPtr^.ProceduresRecordOffset);
    If ProceduresRecord^.ProgramCodeBlockRecordOffset <> $FFFF then Error_Invalid_PUBLIC_Definition;
    ProceduresRecord^.ProgramCodeBlockRecordOffset := SymbolTable [stCodeBlocks].UsedSize;
    ProceduresRecord^.ProcIdentifierOffset := PublicIdOffset;
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
- https://turbopascal.org/importing-pubdef-object-file-record
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

