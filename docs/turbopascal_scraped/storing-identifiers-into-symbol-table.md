# Storing Identifiers Into Symbol Table - Turbo Pascal Compiler

**URL:** https://turbopascal.org/storing-identifiers-into-symbol-table

---

## Table of Contents

- Storing Identifiers Into Symbol Table

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Storing Identifiers Into Symbol Table
Storing identifiers into symbol table is simple because of the way how Turbo Pascal organizes symbol tables.
StoreCurrentIdentifierToSymbolTable
increases main symbol table
, adds the identifier at the end of appropriate list and clears identifier data.
Function StoreCurrentIdentifierToSymbolTable (IdentifierList: PIdentifierList; IdentifierDataSize: Word;
                                Var NewIdentifier: PIdentifier): Pointer;
Var DataStart: PChar;
    LastIdentifier: PIdentifier;
    ListEntry: Byte;

begin
  NewIdentifier := IncreaseSymbolTable (stMain, 3 + Length (CurrentIdentifier) + 1 + IdentifierDataSize);
  ListEntry := (CurrentIdentifierHash and IdentifierList^.Mask) shr 1;
  LastIdentifier := Ptr (Seg (IdentifierList^), IdentifierList^.Offset [ListEntry]);
  IdentifierList^.Offset [ListEntry] := Ofs (NewIdentifier^);
  NewIdentifier^.Next := Ofs (LastIdentifier^);
  NewIdentifier^.Token := Token_EndOfLine;
  NewIdentifier^.Name.Str := CurrentIdentifier;
  DataStart := PChar (NewIdentifier) + 3 + 1 + Length (CurrentIdentifier);
  StoreCurrentIdentifierToSymbolTable := DataStart;
  FillChar (DataStart^, IdentifierDataSize, 0);
end;
StoreNewIdentifierToSymbolTable
first checks if the identifier is already defined in the current scope. If it is found you get the famous
Duplicate Identifier
error
.
Function StoreNewIdentifierToSymbolTable (NewIdentifierDataSize: Word; Var NewIdentifier: PIdentifier): Pointer;
Var IdentifierDataPtr: Pointer;
    IdentifierOffset: Word;
    IdToken: TToken;
begin
  If CurrentIdentifierDeclaredInCurrentScope (IdentifierOffset, IdentifierDataPtr, IdToken) then
    CurrentIdentifierError (DuplicateIdentifier);
  StoreNewIdentifierToSymbolTable := StoreCurrentIdentifierToSymbolTable (CurrentScopeIdentifierTableAddress, NewIdentifierDataSize, NewIdentifier);
end;
This function finds the symbol table according to the current scope.
Function CurrentScopeIdentifierTableAddress: PIdentifierList;
Var UnitHeader: PUnitHeader;
    ListPtr: PtrRec absolute UnitHeader;
begin
  UnitHeader := Ptr (MainSymbolTable.Segment, 0);
  If CurrentRecordOrObjectTypeDefinitionOffset <> 0 then
    ListPtr.Ofs := PRecordTypeDefinition (Ptr (MainSymbolTable.Segment,
                                               CurrentRecordOrObjectTypeDefinitionOffset))^.FieldsListOffset else
      If ProcedureIdentifierDataOffset <> 0 then
        ListPtr.Ofs := PProcedureIdentifierData (Ptr (MainSymbolTable.Segment,
                                                      ProcedureIdentifierDataOffset))^.LocalIdentifiersList else
          ListPtr.Ofs := UnitHeader^.PrivateIdentifiersListOffset;
  CurrentScopeIdentifierTableAddress := ListPtr.Ptr;
end;
StoreCommaSeparatedIdentifiers
simply stores one or more identifiers separated with comma. This procedure is used for storing declared variables and fields in records and objects.
Procedure StoreCommaSeparatedIdentifiers;
Var IdData: Pointer;
begin
  FirstDeclaredVariableIdentifierOffset := SymbolTable [stMain].UsedSize;
  NumberOfDeclaredVariableIdentifiers := 0;
  Repeat
    ExpectAndStoreIdentifier (SizeOf (TVariableIdentifierData), IdData);
    Inc (NumberOfDeclaredVariableIdentifiers);
  until not CheckAndGetNextToken (Token_Comma);
end;

Procedure ExpectIdentifier;
begin
  If Token <> Token_Identifier then Error (IdentifierExpected);
end;
ExpectAndStoreIdentifier
checks if the current token is identifier and if it is, it is stored in the main symbol table.
Function ExpectAndStoreIdentifier (DataSize: Word; Var IdentifierData: Pointer): PIdentifier;
Var NewIdentifier: PIdentifier;
begin
  ExpectIdentifier;
  IdentifierData := StoreNewIdentifierToSymbolTable (DataSize, NewIdentifier);
  GetNextToken;
  ExpectAndStoreIdentifier := NewIdentifier;
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
Function StoreCurrentIdentifierToSymbolTable (IdentifierList: PIdentifierList; IdentifierDataSize: Word;
                                Var NewIdentifier: PIdentifier): Pointer;
Var DataStart: PChar;
    LastIdentifier: PIdentifier;
    ListEntry: Byte;

begin
  NewIdentifier := IncreaseSymbolTable (stMain, 3 + Length (CurrentIdentifier) + 1 + IdentifierDataSize);
  ListEntry := (CurrentIdentifierHash and IdentifierList^.Mask) shr 1;
  LastIdentifier := Ptr (Seg (IdentifierList^), IdentifierList^.Offset [ListEntry]);
  IdentifierList^.Offset [ListEntry] := Ofs (NewIdentifier^);
  NewIdentifier^.Next := Ofs (LastIdentifier^);
  NewIdentifier^.Token := Token_EndOfLine;
  NewIdentifier^.Name.Str := CurrentIdentifier;
  DataStart := PChar (NewIdentifier) + 3 + 1 + Length (CurrentIdentifier);
  StoreCurrentIdentifierToSymbolTable := DataStart;
  FillChar (DataStart^, IdentifierDataSize, 0);
end;
```

### Code Block 2

```pascal
Function StoreNewIdentifierToSymbolTable (NewIdentifierDataSize: Word; Var NewIdentifier: PIdentifier): Pointer;
Var IdentifierDataPtr: Pointer;
    IdentifierOffset: Word;
    IdToken: TToken;
begin
  If CurrentIdentifierDeclaredInCurrentScope (IdentifierOffset, IdentifierDataPtr, IdToken) then
    CurrentIdentifierError (DuplicateIdentifier);
  StoreNewIdentifierToSymbolTable := StoreCurrentIdentifierToSymbolTable (CurrentScopeIdentifierTableAddress, NewIdentifierDataSize, NewIdentifier);
end;
```

### Code Block 3

```pascal
Function CurrentScopeIdentifierTableAddress: PIdentifierList;
Var UnitHeader: PUnitHeader;
    ListPtr: PtrRec absolute UnitHeader;
begin
  UnitHeader := Ptr (MainSymbolTable.Segment, 0);
  If CurrentRecordOrObjectTypeDefinitionOffset <> 0 then
    ListPtr.Ofs := PRecordTypeDefinition (Ptr (MainSymbolTable.Segment,
                                               CurrentRecordOrObjectTypeDefinitionOffset))^.FieldsListOffset else
      If ProcedureIdentifierDataOffset <> 0 then
        ListPtr.Ofs := PProcedureIdentifierData (Ptr (MainSymbolTable.Segment,
                                                      ProcedureIdentifierDataOffset))^.LocalIdentifiersList else
          ListPtr.Ofs := UnitHeader^.PrivateIdentifiersListOffset;
  CurrentScopeIdentifierTableAddress := ListPtr.Ptr;
end;
```

### Code Block 4

```pascal
Procedure StoreCommaSeparatedIdentifiers;
Var IdData: Pointer;
begin
  FirstDeclaredVariableIdentifierOffset := SymbolTable [stMain].UsedSize;
  NumberOfDeclaredVariableIdentifiers := 0;
  Repeat
    ExpectAndStoreIdentifier (SizeOf (TVariableIdentifierData), IdData);
    Inc (NumberOfDeclaredVariableIdentifiers);
  until not CheckAndGetNextToken (Token_Comma);
end;

Procedure ExpectIdentifier;
begin
  If Token <> Token_Identifier then Error (IdentifierExpected);
end;
```

### Code Block 5

```pascal
Function ExpectAndStoreIdentifier (DataSize: Word; Var IdentifierData: Pointer): PIdentifier;
Var NewIdentifier: PIdentifier;
begin
  ExpectIdentifier;
  IdentifierData := StoreNewIdentifierToSymbolTable (DataSize, NewIdentifier);
  GetNextToken;
  ExpectAndStoreIdentifier := NewIdentifier;
end;
```

### Code Block 6

```pascal
StoreCurrentIdentifierToSymbolTable
```

### Code Block 7

```pascal
Function StoreCurrentIdentifierToSymbolTable (IdentifierList: PIdentifierList; IdentifierDataSize: Word;
                                Var NewIdentifier: PIdentifier): Pointer;
Var DataStart: PChar;
    LastIdentifier: PIdentifier;
    ListEntry: Byte;

begin
  NewIdentifier := IncreaseSymbolTable (stMain, 3 + Length (CurrentIdentifier) + 1 + IdentifierDataSize);
  ListEntry := (CurrentIdentifierHash and IdentifierList^.Mask) shr 1;
  LastIdentifier := Ptr (Seg (IdentifierList^), IdentifierList^.Offset [ListEntry]);
  IdentifierList^.Offset [ListEntry] := Ofs (NewIdentifier^);
  NewIdentifier^.Next := Ofs (LastIdentifier^);
  NewIdentifier^.Token := Token_EndOfLine;
  NewIdentifier^.Name.Str := CurrentIdentifier;
  DataStart := PChar (NewIdentifier) + 3 + 1 + Length (CurrentIdentifier);
  StoreCurrentIdentifierToSymbolTable := DataStart;
  FillChar (DataStart^, IdentifierDataSize, 0);
end;
```

### Code Block 8

```pascal
StoreNewIdentifierToSymbolTable
```

### Code Block 9

```pascal
Function StoreNewIdentifierToSymbolTable (NewIdentifierDataSize: Word; Var NewIdentifier: PIdentifier): Pointer;
Var IdentifierDataPtr: Pointer;
    IdentifierOffset: Word;
    IdToken: TToken;
begin
  If CurrentIdentifierDeclaredInCurrentScope (IdentifierOffset, IdentifierDataPtr, IdToken) then
    CurrentIdentifierError (DuplicateIdentifier);
  StoreNewIdentifierToSymbolTable := StoreCurrentIdentifierToSymbolTable (CurrentScopeIdentifierTableAddress, NewIdentifierDataSize, NewIdentifier);
end;
```

### Code Block 10

```pascal
Function CurrentScopeIdentifierTableAddress: PIdentifierList;
Var UnitHeader: PUnitHeader;
    ListPtr: PtrRec absolute UnitHeader;
begin
  UnitHeader := Ptr (MainSymbolTable.Segment, 0);
  If CurrentRecordOrObjectTypeDefinitionOffset <> 0 then
    ListPtr.Ofs := PRecordTypeDefinition (Ptr (MainSymbolTable.Segment,
                                               CurrentRecordOrObjectTypeDefinitionOffset))^.FieldsListOffset else
      If ProcedureIdentifierDataOffset <> 0 then
        ListPtr.Ofs := PProcedureIdentifierData (Ptr (MainSymbolTable.Segment,
                                                      ProcedureIdentifierDataOffset))^.LocalIdentifiersList else
          ListPtr.Ofs := UnitHeader^.PrivateIdentifiersListOffset;
  CurrentScopeIdentifierTableAddress := ListPtr.Ptr;
end;
```

### Code Block 11

```pascal
StoreCommaSeparatedIdentifiers
```

### Code Block 12

```pascal
Procedure StoreCommaSeparatedIdentifiers;
Var IdData: Pointer;
begin
  FirstDeclaredVariableIdentifierOffset := SymbolTable [stMain].UsedSize;
  NumberOfDeclaredVariableIdentifiers := 0;
  Repeat
    ExpectAndStoreIdentifier (SizeOf (TVariableIdentifierData), IdData);
    Inc (NumberOfDeclaredVariableIdentifiers);
  until not CheckAndGetNextToken (Token_Comma);
end;

Procedure ExpectIdentifier;
begin
  If Token <> Token_Identifier then Error (IdentifierExpected);
end;
```

### Code Block 13

```pascal
ExpectAndStoreIdentifier
```

### Code Block 14

```pascal
Function ExpectAndStoreIdentifier (DataSize: Word; Var IdentifierData: Pointer): PIdentifier;
Var NewIdentifier: PIdentifier;
begin
  ExpectIdentifier;
  IdentifierData := StoreNewIdentifierToSymbolTable (DataSize, NewIdentifier);
  GetNextToken;
  ExpectAndStoreIdentifier := NewIdentifier;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/error-reporting
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/storing-identifiers-into-symbol-table
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

