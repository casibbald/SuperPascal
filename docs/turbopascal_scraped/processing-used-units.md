# Processing Used Units - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-used-units

---

## Table of Contents

- Processing Used Units

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Used Units
Every Turbo Pascal module (program or unit) can use other units. Processing them is one of the trickiest parts of Turbo Pascal compiler.
Each unit listed in the
Uses
statement is added to the symbol table and linked in a list of used units.
Procedure SetModuleFlagsAndProcessUsedUnits;
begin
  ModuleHeadingEnded := True;
  If not (DebugInformation in ModuleCompilerSwitches) then Exclude (ModuleCompilerSwitches, LocalDebugSymbols);
  If not (LocalDebugSymbols in ModuleCompilerSwitches) then Exclude (ModuleCompilerSwitches, CompilerDirective_Y);
  With PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^ do
    begin
      Flags := ModuleUnitFlags;
      If OverlaysAllowed in ModuleCompilerSwitches then Include (Flags, ufOverlaysAllowed);
      If Instructions80286 in ModuleCompilerSwitches then Include (Flags, ufInstructions80286);
    end;
  ProcessUsedUnits;
end;
Procedure ProcessUsedUnits;
Var IdLen: Byte;
    UsesClause: Boolean;
    CurrentUnit: PUnitHeader;
    UnitIdentifierData: PUnitIdentifierData;

  Procedure AddUnitNameIdentifierToList;
  Var UnitIdentifierData: PUnitIdentifierData;
      UnitIdentifier: PIdentifier;
  begin
    UnitIdentifierData := StoreNewIdentifierToSymbolTable (9, UnitIdentifier);
    UnitIdentifier^.Token := Token_UnitIdentifier;
    PUnitIdentifierData (Ptr (Seg (UnitIdentifier^), LastUnitIdentifierData))^.NextUnitIdentifier :=
      Ofs (UnitIdentifier^);
    LastUnitIdentifierData := Ofs (UnitIdentifierData^);
    UnitIdentifierData^.CurrentUsedUnitIdentifier := LastUsedUnitIdentifier;
    LastUsedUnitIdentifier := Ofs (UnitIdentifier^);
    If SourceType = stUnitInterface then UnitIdentifierData^.UnitIdentifierFlags := [UsedInInterface];
  end;

  Procedure CreatePrivateIdentifiersList;
  Var Hash: Byte;
      UnitHeader: PUnitHeader;
      IdentifierList: PIdentifierList;
      IdentifierOffset: Word;

  begin
    UnitHeader := Ptr (MainSymbolTable.Segment, 0);
    IdentifierList := Ptr (MainSymbolTable.Segment, UnitHeader^.PublicIdentifiersListOffset);
    UnitHeader^.PrivateIdentifiersListOffset := Ofs (IncreaseSymbolTable (stMain, IdentifierList^.Mask + 4)^);
    Move (IdentifierList^, Ptr (Seg (UnitHeader^), UnitHeader^.PrivateIdentifiersListOffset)^, IdentifierList^.Mask + 4);
    For Hash := 0 to IdentifierList^.Mask div 2 do
      begin
        IdentifierOffset := Ofs (IdentifierList^.Offset [Hash]);
        Repeat
          IdentifierOffset := PIdentifier (Ptr (Seg (UnitHeader^), IdentifierOffset))^.Next;
        until IdentifierOffset < IdListOffsetBeforeImplementationUsedUnits;
        IdentifierList^.Offset [Hash] := IdentifierOffset;
      end;
  end;

begin
  CurrentUnit := Ptr (MainSymbolTable.Segment, 0);
  IdLen := PIdentifier (Ptr (MainSymbolTable.Segment, CurrentUnit^.UnitNameIdentifierOffset))^.Name.Len;
  UnitIdentifierData := Ptr (MainSymbolTable.Segment, CurrentUnit^.UnitNameIdentifierOffset + IdLen + 4);
  LastUsedUnitIdentifier := UnitIdentifierData^.CurrentUsedUnitIdentifier;
  IdListOffsetBeforeImplementationUsedUnits := MainSymbolTable.UsedSize;
  If not SystemUnitCompilation and (SourceType <> stUnitImplementation) then
    begin
      CopyStringToCurrentIdentifier (_System.Text);
      AddUnitNameIdentifierToList;
    end;
  UsesClause := False;
  If CheckAndGetNextToken (Token_USES) then
    begin
      UsesClause := True;
      Repeat
        ExpectIdentifier;
        AddUnitNameIdentifierToList;
        GetNextToken;
      until not CheckAndGetNextToken (Token_Comma);
    end;
  If SourceType = stUnitImplementation then CreatePrivateIdentifiersList;
  LoadUsedUnits;
  If UsesClause then ExpectTokenAndGetNext (Token_Semicolon);

  CurrentUnit := Ptr (MainSymbolTable.Segment, 0);
  IdLen := PIdentifier (Ptr (MainSymbolTable.Segment, CurrentUnit^.UnitNameIdentifierOffset))^.Name.Len;
  UnitIdentifierData := Ptr (MainSymbolTable.Segment, CurrentUnit^.UnitNameIdentifierOffset + IdLen + 4);

  UnitIdentifierData^.CurrentUsedUnitIdentifier := LastUsedUnitIdentifier;
  If not SystemUnitCompilation then
    begin
      IdLen := PIdentifier (Ptr (MainSymbolTable.Segment, UnitIdentifierData^.NextUnitIdentifier))^.Name.Len;
      UnitIdentifierData := Ptr (MainSymbolTable.Segment, UnitIdentifierData^.NextUnitIdentifier + IdLen + 4);
    end;
  SystemUnitSegment := UnitIdentifierData^.UnitSegment;
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
Procedure SetModuleFlagsAndProcessUsedUnits;
begin
  ModuleHeadingEnded := True;
  If not (DebugInformation in ModuleCompilerSwitches) then Exclude (ModuleCompilerSwitches, LocalDebugSymbols);
  If not (LocalDebugSymbols in ModuleCompilerSwitches) then Exclude (ModuleCompilerSwitches, CompilerDirective_Y);
  With PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^ do
    begin
      Flags := ModuleUnitFlags;
      If OverlaysAllowed in ModuleCompilerSwitches then Include (Flags, ufOverlaysAllowed);
      If Instructions80286 in ModuleCompilerSwitches then Include (Flags, ufInstructions80286);
    end;
  ProcessUsedUnits;
end;
```

### Code Block 2

```pascal
Procedure ProcessUsedUnits;
Var IdLen: Byte;
    UsesClause: Boolean;
    CurrentUnit: PUnitHeader;
    UnitIdentifierData: PUnitIdentifierData;

  Procedure AddUnitNameIdentifierToList;
  Var UnitIdentifierData: PUnitIdentifierData;
      UnitIdentifier: PIdentifier;
  begin
    UnitIdentifierData := StoreNewIdentifierToSymbolTable (9, UnitIdentifier);
    UnitIdentifier^.Token := Token_UnitIdentifier;
    PUnitIdentifierData (Ptr (Seg (UnitIdentifier^), LastUnitIdentifierData))^.NextUnitIdentifier :=
      Ofs (UnitIdentifier^);
    LastUnitIdentifierData := Ofs (UnitIdentifierData^);
    UnitIdentifierData^.CurrentUsedUnitIdentifier := LastUsedUnitIdentifier;
    LastUsedUnitIdentifier := Ofs (UnitIdentifier^);
    If SourceType = stUnitInterface then UnitIdentifierData^.UnitIdentifierFlags := [UsedInInterface];
  end;

  Procedure CreatePrivateIdentifiersList;
  Var Hash: Byte;
      UnitHeader: PUnitHeader;
      IdentifierList: PIdentifierList;
      IdentifierOffset: Word;

  begin
    UnitHeader := Ptr (MainSymbolTable.Segment, 0);
    IdentifierList := Ptr (MainSymbolTable.Segment, UnitHeader^.PublicIdentifiersListOffset);
    UnitHeader^.PrivateIdentifiersListOffset := Ofs (IncreaseSymbolTable (stMain, IdentifierList^.Mask + 4)^);
    Move (IdentifierList^, Ptr (Seg (UnitHeader^), UnitHeader^.PrivateIdentifiersListOffset)^, IdentifierList^.Mask + 4);
    For Hash := 0 to IdentifierList^.Mask div 2 do
      begin
        IdentifierOffset := Ofs (IdentifierList^.Offset [Hash]);
        Repeat
          IdentifierOffset := PIdentifier (Ptr (Seg (UnitHeader^), IdentifierOffset))^.Next;
        until IdentifierOffset < IdListOffsetBeforeImplementationUsedUnits;
        IdentifierList^.Offset [Hash] := IdentifierOffset;
      end;
  end;

begin
  CurrentUnit := Ptr (MainSymbolTable.Segment, 0);
  IdLen := PIdentifier (Ptr (MainSymbolTable.Segment, CurrentUnit^.UnitNameIdentifierOffset))^.Name.Len;
  UnitIdentifierData := Ptr (MainSymbolTable.Segment, CurrentUnit^.UnitNameIdentifierOffset + IdLen + 4);
  LastUsedUnitIdentifier := UnitIdentifierData^.CurrentUsedUnitIdentifier;
  IdListOffsetBeforeImplementationUsedUnits := MainSymbolTable.UsedSize;
  If not SystemUnitCompilation and (SourceType <> stUnitImplementation) then
    begin
      CopyStringToCurrentIdentifier (_System.Text);
      AddUnitNameIdentifierToList;
    end;
  UsesClause := False;
  If CheckAndGetNextToken (Token_USES) then
    begin
      UsesClause := True;
      Repeat
        ExpectIdentifier;
        AddUnitNameIdentifierToList;
        GetNextToken;
      until not CheckAndGetNextToken (Token_Comma);
    end;
  If SourceType = stUnitImplementation then CreatePrivateIdentifiersList;
  LoadUsedUnits;
  If UsesClause then ExpectTokenAndGetNext (Token_Semicolon);

  CurrentUnit := Ptr (MainSymbolTable.Segment, 0);
  IdLen := PIdentifier (Ptr (MainSymbolTable.Segment, CurrentUnit^.UnitNameIdentifierOffset))^.Name.Len;
  UnitIdentifierData := Ptr (MainSymbolTable.Segment, CurrentUnit^.UnitNameIdentifierOffset + IdLen + 4);

  UnitIdentifierData^.CurrentUsedUnitIdentifier := LastUsedUnitIdentifier;
  If not SystemUnitCompilation then
    begin
      IdLen := PIdentifier (Ptr (MainSymbolTable.Segment, UnitIdentifierData^.NextUnitIdentifier))^.Name.Len;
      UnitIdentifierData := Ptr (MainSymbolTable.Segment, UnitIdentifierData^.NextUnitIdentifier + IdLen + 4);
    end;
  SystemUnitSegment := UnitIdentifierData^.UnitSegment;
end;
```

### Code Block 3

```pascal
Procedure SetModuleFlagsAndProcessUsedUnits;
begin
  ModuleHeadingEnded := True;
  If not (DebugInformation in ModuleCompilerSwitches) then Exclude (ModuleCompilerSwitches, LocalDebugSymbols);
  If not (LocalDebugSymbols in ModuleCompilerSwitches) then Exclude (ModuleCompilerSwitches, CompilerDirective_Y);
  With PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^ do
    begin
      Flags := ModuleUnitFlags;
      If OverlaysAllowed in ModuleCompilerSwitches then Include (Flags, ufOverlaysAllowed);
      If Instructions80286 in ModuleCompilerSwitches then Include (Flags, ufInstructions80286);
    end;
  ProcessUsedUnits;
end;
```

### Code Block 4

```pascal
Procedure ProcessUsedUnits;
Var IdLen: Byte;
    UsesClause: Boolean;
    CurrentUnit: PUnitHeader;
    UnitIdentifierData: PUnitIdentifierData;

  Procedure AddUnitNameIdentifierToList;
  Var UnitIdentifierData: PUnitIdentifierData;
      UnitIdentifier: PIdentifier;
  begin
    UnitIdentifierData := StoreNewIdentifierToSymbolTable (9, UnitIdentifier);
    UnitIdentifier^.Token := Token_UnitIdentifier;
    PUnitIdentifierData (Ptr (Seg (UnitIdentifier^), LastUnitIdentifierData))^.NextUnitIdentifier :=
      Ofs (UnitIdentifier^);
    LastUnitIdentifierData := Ofs (UnitIdentifierData^);
    UnitIdentifierData^.CurrentUsedUnitIdentifier := LastUsedUnitIdentifier;
    LastUsedUnitIdentifier := Ofs (UnitIdentifier^);
    If SourceType = stUnitInterface then UnitIdentifierData^.UnitIdentifierFlags := [UsedInInterface];
  end;

  Procedure CreatePrivateIdentifiersList;
  Var Hash: Byte;
      UnitHeader: PUnitHeader;
      IdentifierList: PIdentifierList;
      IdentifierOffset: Word;

  begin
    UnitHeader := Ptr (MainSymbolTable.Segment, 0);
    IdentifierList := Ptr (MainSymbolTable.Segment, UnitHeader^.PublicIdentifiersListOffset);
    UnitHeader^.PrivateIdentifiersListOffset := Ofs (IncreaseSymbolTable (stMain, IdentifierList^.Mask + 4)^);
    Move (IdentifierList^, Ptr (Seg (UnitHeader^), UnitHeader^.PrivateIdentifiersListOffset)^, IdentifierList^.Mask + 4);
    For Hash := 0 to IdentifierList^.Mask div 2 do
      begin
        IdentifierOffset := Ofs (IdentifierList^.Offset [Hash]);
        Repeat
          IdentifierOffset := PIdentifier (Ptr (Seg (UnitHeader^), IdentifierOffset))^.Next;
        until IdentifierOffset < IdListOffsetBeforeImplementationUsedUnits;
        IdentifierList^.Offset [Hash] := IdentifierOffset;
      end;
  end;

begin
  CurrentUnit := Ptr (MainSymbolTable.Segment, 0);
  IdLen := PIdentifier (Ptr (MainSymbolTable.Segment, CurrentUnit^.UnitNameIdentifierOffset))^.Name.Len;
  UnitIdentifierData := Ptr (MainSymbolTable.Segment, CurrentUnit^.UnitNameIdentifierOffset + IdLen + 4);
  LastUsedUnitIdentifier := UnitIdentifierData^.CurrentUsedUnitIdentifier;
  IdListOffsetBeforeImplementationUsedUnits := MainSymbolTable.UsedSize;
  If not SystemUnitCompilation and (SourceType <> stUnitImplementation) then
    begin
      CopyStringToCurrentIdentifier (_System.Text);
      AddUnitNameIdentifierToList;
    end;
  UsesClause := False;
  If CheckAndGetNextToken (Token_USES) then
    begin
      UsesClause := True;
      Repeat
        ExpectIdentifier;
        AddUnitNameIdentifierToList;
        GetNextToken;
      until not CheckAndGetNextToken (Token_Comma);
    end;
  If SourceType = stUnitImplementation then CreatePrivateIdentifiersList;
  LoadUsedUnits;
  If UsesClause then ExpectTokenAndGetNext (Token_Semicolon);

  CurrentUnit := Ptr (MainSymbolTable.Segment, 0);
  IdLen := PIdentifier (Ptr (MainSymbolTable.Segment, CurrentUnit^.UnitNameIdentifierOffset))^.Name.Len;
  UnitIdentifierData := Ptr (MainSymbolTable.Segment, CurrentUnit^.UnitNameIdentifierOffset + IdLen + 4);

  UnitIdentifierData^.CurrentUsedUnitIdentifier := LastUsedUnitIdentifier;
  If not SystemUnitCompilation then
    begin
      IdLen := PIdentifier (Ptr (MainSymbolTable.Segment, UnitIdentifierData^.NextUnitIdentifier))^.Name.Len;
      UnitIdentifierData := Ptr (MainSymbolTable.Segment, UnitIdentifierData^.NextUnitIdentifier + IdLen + 4);
    end;
  SystemUnitSegment := UnitIdentifierData^.UnitSegment;
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
- https://turbopascal.org/processing-used-units
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

