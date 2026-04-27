# Processing Variable Declarations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-variable-declarations

---

## Table of Contents

- Processing Variable Declarations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Variable Declarations
This procedure expects a
list of identifiers
followed by a colon and variable type and processes declaration of variables: global, local and absolute.
Procedure ProcessVariableDeclarations;
begin
  GetNextToken;
  Repeat
    StoreCommaSeparatedIdentifiers;
    ExpectTokenAndGetNext (Token_Colon);
    GetVariableType;
    If CheckAndGetNextToken (Token_ABSOLUTE) then
      begin
        CheckForDeclaredIdentifier;
        If CheckAndGetNextToken (Token_VariableIdentifier) then
          begin
            GetTypeAndUnitIdentifierOffsets (CurrentIdentifierDataPtr, RecordTypeDefinitionOffset);
            VariableData_Flags := [vfAbsoluteVar];
          end else begin
                     RecordTypeDefinitionOffset.Segment := ExpectIntegerConstant;
                     ExpectTokenAndGetNext (Token_Colon);
                     RecordTypeDefinitionOffset.Offset := ExpectIntegerConstant;
                     VariableData_Flags := VariableAbsoluteAddress;
                   end;

      end else begin
                 If CurrentProcedureIdentifier <> 0 then
                    begin
                      RecordTypeDefinitionOffset.UnitIdentifierData := CurrentProcedureIdentifier;
                      VariableData_Flags := [vf1];
                    end else begin
                               ForceCreationOfVariablesBlockRecord := True;
                               RecordTypeDefinitionOffset.UnitIdentifierData := SymbolTable [stVariablesBlocks].UsedSize;
                               VariableData_Flags := [];
                             end;
               end;
    CalculateVariableOffsets;
    ExpectTokenAndGetNext (TOKEN_Semicolon);
  until Token <> Token_Identifier;
  CreateVariablesBlockRecord;
end;
Here are offsets of variables calculated. Their addresses will be determined later when the program will be linked.
Procedure CalculateVariableOffsets;
Var VarIdentifier: PIdentifier;
    VariableIdentifierData: PVariableIdentifierData;
    TempPtr: Pointer;
begin
  VarIdentifier := Ptr (SymbolTable [stMain].Segment, FirstDeclaredVariableIdentifierOffset);
  Repeat
    VarIdentifier^.Token := TToken (PrivateFlagMask or Byte (Token_VariableIdentifier));
    VariableIdentifierData := Ptr (Seg (VarIdentifier^), Ofs (VarIdentifier^) + VarIdentifier^.Name.Len + 4);
    If VariableData_Flags = [] then
      begin
        If WordAlignment in ModuleCompilerSwitches then
          If VariableDataSize <> 1 then
            begin
              If SizeOfVariablesInDataSegment = $FFFF then Error (TooManyVariables);
              SizeOfVariablesInDataSegment := (SizeOfVariablesInDataSegment + 1) and $FFFE;
            end;
        If LongInt (SizeOfVariablesInDataSegment) + VariableDataSize > $FFFF then Error (TooManyVariables);
        RecordTypeDefinitionOffset.Offset := SizeOfVariablesInDataSegment - LastSizeOfVariablesInDataSegment;
        Inc (SizeOfVariablesInDataSegment,  VariableDataSize);
      end else
        If VariableData_Flags = [vf1] then
          begin
            If LongInt (Integer (ProgramBlockMaxStackFrameOffset - 1)) - VariableDataSize < - LongInt ($FFFF)
              then Error (TooManyVariables);
            ProgramBlockMaxStackFrameOffset := ProgramBlockMaxStackFrameOffset - VariableDataSize;
            If WordAlignment in ModuleCompilerSwitches then
              If VariableDataSize <> 1 then ProgramBlockMaxStackFrameOffset := ProgramBlockMaxStackFrameOffset and $FFFE;
            RecordTypeDefinitionOffset.Offset := ProgramBlockMaxStackFrameOffset;
          end else
            If Field in VariableData_Flags then
              begin
                TempPtr := Ptr (SymbolTable [stMain].Segment, OffsetToNextMemberOffset);
                Word (TempPtr^) := Ofs (VarIdentifier^);
                OffsetToNextMemberOffset := Ofs (VariableIdentifierData^.W5);
                TempPtr := Ptr (SymbolTable [stMain].Segment, CurrentRecordOrObjectTypeDefinitionOffset + 2);
                RecordTypeDefinitionOffset.Offset := Word (TempPtr^);
                If LongInt (Word (TempPtr^)) + VariableDataSize > $FFFF then Error (StructureTooLarge);
                Inc (Word (TempPtr^), VariableDataSize);
              end;
    With VariableIdentifierData^ do
      begin
        Flags := VariableData_Flags;
        W1.Ofs := RecordTypeDefinitionOffset.TypeOffset;
        W1.Seg := RecordTypeDefinitionOffset.UnitIdentifierData;
        W5 := VariableData_NextMemberOffset;
        UnitTypeOffsets := CurrentVarUnitTypeOffsets;
      end;
    VarIdentifier := Ptr (Seg (VariableIdentifierData^), Ofs (VariableIdentifierData^) + SizeOf (TVariableIdentifierData));
    Dec (NumberOfDeclaredVariableIdentifiers);
  until NumberOfDeclaredVariableIdentifiers = 0;
end;
For each
Var
declaration Turbo Pascal creates a record in symbol table for variables.
Procedure CreateVariablesBlockRecord;
begin
  If SizeOfVariablesInDataSegment = $FFFF then Error (TooManyVariables);
  SizeOfVariablesInDataSegment := (SizeOfVariablesInDataSegment + 1) and $FFFE;
  If (SizeOfVariablesInDataSegment <> LastSizeOfVariablesInDataSegment) or ForceCreationOfVariablesBlockRecord then
    begin
      ForceCreationOfVariablesBlockRecord := False;
      With PVariablesBlockRecord (IncreaseSymbolTable (stVariablesBlocks, SizeOf (TVariablesBlockRecord)))^ do
        begin
          Offset := 0;
          VariablesSize := SizeOfVariablesInDataSegment - LastSizeOfVariablesInDataSegment;
          ReferencesSize := 0;
          vrW6 := 0;
        end;
      LastSizeOfVariablesInDataSegment := SizeOfVariablesInDataSegment;
    end;
end;
Var
declarations are using processing of
non-object types
.
Function GetVariableType: PTypeDefinition;
Var UnitTypeOffsets: PTypeDefinition;
begin
  UnitTypeOffsets := ProcessVariableTypeDeclaration;
  VariableDataSize := UnitTypeOffsets^.Size;
  VariableData_NextMemberOffset := 0;
  GetVariableType := UnitTypeOffsets;
  GetTypeAndUnitIdentifierOffsets (UnitTypeOffsets, CurrentVarUnitTypeOffsets);
end;

Function ProcessVariableTypeDeclaration: PTypeDefinition;
begin
  LastPointerToTypeDefinitionOffset := 0;
  ProcessVariableTypeDeclaration := ProcessNonObjectTypeDeclaration;
  CheckIfDirecive (_ABSOLUTE, Token_ABSOLUTE);
  ResolvePointerToTypeDefinitions;
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
Procedure ProcessVariableDeclarations;
begin
  GetNextToken;
  Repeat
    StoreCommaSeparatedIdentifiers;
    ExpectTokenAndGetNext (Token_Colon);
    GetVariableType;
    If CheckAndGetNextToken (Token_ABSOLUTE) then
      begin
        CheckForDeclaredIdentifier;
        If CheckAndGetNextToken (Token_VariableIdentifier) then
          begin
            GetTypeAndUnitIdentifierOffsets (CurrentIdentifierDataPtr, RecordTypeDefinitionOffset);
            VariableData_Flags := [vfAbsoluteVar];
          end else begin
                     RecordTypeDefinitionOffset.Segment := ExpectIntegerConstant;
                     ExpectTokenAndGetNext (Token_Colon);
                     RecordTypeDefinitionOffset.Offset := ExpectIntegerConstant;
                     VariableData_Flags := VariableAbsoluteAddress;
                   end;

      end else begin
                 If CurrentProcedureIdentifier <> 0 then
                    begin
                      RecordTypeDefinitionOffset.UnitIdentifierData := CurrentProcedureIdentifier;
                      VariableData_Flags := [vf1];
                    end else begin
                               ForceCreationOfVariablesBlockRecord := True;
                               RecordTypeDefinitionOffset.UnitIdentifierData := SymbolTable [stVariablesBlocks].UsedSize;
                               VariableData_Flags := [];
                             end;
               end;
    CalculateVariableOffsets;
    ExpectTokenAndGetNext (TOKEN_Semicolon);
  until Token <> Token_Identifier;
  CreateVariablesBlockRecord;
end;
```

### Code Block 2

```pascal
Procedure CalculateVariableOffsets;
Var VarIdentifier: PIdentifier;
    VariableIdentifierData: PVariableIdentifierData;
    TempPtr: Pointer;
begin
  VarIdentifier := Ptr (SymbolTable [stMain].Segment, FirstDeclaredVariableIdentifierOffset);
  Repeat
    VarIdentifier^.Token := TToken (PrivateFlagMask or Byte (Token_VariableIdentifier));
    VariableIdentifierData := Ptr (Seg (VarIdentifier^), Ofs (VarIdentifier^) + VarIdentifier^.Name.Len + 4);
    If VariableData_Flags = [] then
      begin
        If WordAlignment in ModuleCompilerSwitches then
          If VariableDataSize <> 1 then
            begin
              If SizeOfVariablesInDataSegment = $FFFF then Error (TooManyVariables);
              SizeOfVariablesInDataSegment := (SizeOfVariablesInDataSegment + 1) and $FFFE;
            end;
        If LongInt (SizeOfVariablesInDataSegment) + VariableDataSize > $FFFF then Error (TooManyVariables);
        RecordTypeDefinitionOffset.Offset := SizeOfVariablesInDataSegment - LastSizeOfVariablesInDataSegment;
        Inc (SizeOfVariablesInDataSegment,  VariableDataSize);
      end else
        If VariableData_Flags = [vf1] then
          begin
            If LongInt (Integer (ProgramBlockMaxStackFrameOffset - 1)) - VariableDataSize < - LongInt ($FFFF)
              then Error (TooManyVariables);
            ProgramBlockMaxStackFrameOffset := ProgramBlockMaxStackFrameOffset - VariableDataSize;
            If WordAlignment in ModuleCompilerSwitches then
              If VariableDataSize <> 1 then ProgramBlockMaxStackFrameOffset := ProgramBlockMaxStackFrameOffset and $FFFE;
            RecordTypeDefinitionOffset.Offset := ProgramBlockMaxStackFrameOffset;
          end else
            If Field in VariableData_Flags then
              begin
                TempPtr := Ptr (SymbolTable [stMain].Segment, OffsetToNextMemberOffset);
                Word (TempPtr^) := Ofs (VarIdentifier^);
                OffsetToNextMemberOffset := Ofs (VariableIdentifierData^.W5);
                TempPtr := Ptr (SymbolTable [stMain].Segment, CurrentRecordOrObjectTypeDefinitionOffset + 2);
                RecordTypeDefinitionOffset.Offset := Word (TempPtr^);
                If LongInt (Word (TempPtr^)) + VariableDataSize > $FFFF then Error (StructureTooLarge);
                Inc (Word (TempPtr^), VariableDataSize);
              end;
    With VariableIdentifierData^ do
      begin
        Flags := VariableData_Flags;
        W1.Ofs := RecordTypeDefinitionOffset.TypeOffset;
        W1.Seg := RecordTypeDefinitionOffset.UnitIdentifierData;
        W5 := VariableData_NextMemberOffset;
        UnitTypeOffsets := CurrentVarUnitTypeOffsets;
      end;
    VarIdentifier := Ptr (Seg (VariableIdentifierData^), Ofs (VariableIdentifierData^) + SizeOf (TVariableIdentifierData));
    Dec (NumberOfDeclaredVariableIdentifiers);
  until NumberOfDeclaredVariableIdentifiers = 0;
end;
```

### Code Block 3

```pascal
Procedure CreateVariablesBlockRecord;
begin
  If SizeOfVariablesInDataSegment = $FFFF then Error (TooManyVariables);
  SizeOfVariablesInDataSegment := (SizeOfVariablesInDataSegment + 1) and $FFFE;
  If (SizeOfVariablesInDataSegment <> LastSizeOfVariablesInDataSegment) or ForceCreationOfVariablesBlockRecord then
    begin
      ForceCreationOfVariablesBlockRecord := False;
      With PVariablesBlockRecord (IncreaseSymbolTable (stVariablesBlocks, SizeOf (TVariablesBlockRecord)))^ do
        begin
          Offset := 0;
          VariablesSize := SizeOfVariablesInDataSegment - LastSizeOfVariablesInDataSegment;
          ReferencesSize := 0;
          vrW6 := 0;
        end;
      LastSizeOfVariablesInDataSegment := SizeOfVariablesInDataSegment;
    end;
end;
```

### Code Block 4

```pascal
Function GetVariableType: PTypeDefinition;
Var UnitTypeOffsets: PTypeDefinition;
begin
  UnitTypeOffsets := ProcessVariableTypeDeclaration;
  VariableDataSize := UnitTypeOffsets^.Size;
  VariableData_NextMemberOffset := 0;
  GetVariableType := UnitTypeOffsets;
  GetTypeAndUnitIdentifierOffsets (UnitTypeOffsets, CurrentVarUnitTypeOffsets);
end;

Function ProcessVariableTypeDeclaration: PTypeDefinition;
begin
  LastPointerToTypeDefinitionOffset := 0;
  ProcessVariableTypeDeclaration := ProcessNonObjectTypeDeclaration;
  CheckIfDirecive (_ABSOLUTE, Token_ABSOLUTE);
  ResolvePointerToTypeDefinitions;
end;
```

### Code Block 5

```pascal
Procedure ProcessVariableDeclarations;
begin
  GetNextToken;
  Repeat
    StoreCommaSeparatedIdentifiers;
    ExpectTokenAndGetNext (Token_Colon);
    GetVariableType;
    If CheckAndGetNextToken (Token_ABSOLUTE) then
      begin
        CheckForDeclaredIdentifier;
        If CheckAndGetNextToken (Token_VariableIdentifier) then
          begin
            GetTypeAndUnitIdentifierOffsets (CurrentIdentifierDataPtr, RecordTypeDefinitionOffset);
            VariableData_Flags := [vfAbsoluteVar];
          end else begin
                     RecordTypeDefinitionOffset.Segment := ExpectIntegerConstant;
                     ExpectTokenAndGetNext (Token_Colon);
                     RecordTypeDefinitionOffset.Offset := ExpectIntegerConstant;
                     VariableData_Flags := VariableAbsoluteAddress;
                   end;

      end else begin
                 If CurrentProcedureIdentifier <> 0 then
                    begin
                      RecordTypeDefinitionOffset.UnitIdentifierData := CurrentProcedureIdentifier;
                      VariableData_Flags := [vf1];
                    end else begin
                               ForceCreationOfVariablesBlockRecord := True;
                               RecordTypeDefinitionOffset.UnitIdentifierData := SymbolTable [stVariablesBlocks].UsedSize;
                               VariableData_Flags := [];
                             end;
               end;
    CalculateVariableOffsets;
    ExpectTokenAndGetNext (TOKEN_Semicolon);
  until Token <> Token_Identifier;
  CreateVariablesBlockRecord;
end;
```

### Code Block 6

```pascal
Procedure CalculateVariableOffsets;
Var VarIdentifier: PIdentifier;
    VariableIdentifierData: PVariableIdentifierData;
    TempPtr: Pointer;
begin
  VarIdentifier := Ptr (SymbolTable [stMain].Segment, FirstDeclaredVariableIdentifierOffset);
  Repeat
    VarIdentifier^.Token := TToken (PrivateFlagMask or Byte (Token_VariableIdentifier));
    VariableIdentifierData := Ptr (Seg (VarIdentifier^), Ofs (VarIdentifier^) + VarIdentifier^.Name.Len + 4);
    If VariableData_Flags = [] then
      begin
        If WordAlignment in ModuleCompilerSwitches then
          If VariableDataSize <> 1 then
            begin
              If SizeOfVariablesInDataSegment = $FFFF then Error (TooManyVariables);
              SizeOfVariablesInDataSegment := (SizeOfVariablesInDataSegment + 1) and $FFFE;
            end;
        If LongInt (SizeOfVariablesInDataSegment) + VariableDataSize > $FFFF then Error (TooManyVariables);
        RecordTypeDefinitionOffset.Offset := SizeOfVariablesInDataSegment - LastSizeOfVariablesInDataSegment;
        Inc (SizeOfVariablesInDataSegment,  VariableDataSize);
      end else
        If VariableData_Flags = [vf1] then
          begin
            If LongInt (Integer (ProgramBlockMaxStackFrameOffset - 1)) - VariableDataSize < - LongInt ($FFFF)
              then Error (TooManyVariables);
            ProgramBlockMaxStackFrameOffset := ProgramBlockMaxStackFrameOffset - VariableDataSize;
            If WordAlignment in ModuleCompilerSwitches then
              If VariableDataSize <> 1 then ProgramBlockMaxStackFrameOffset := ProgramBlockMaxStackFrameOffset and $FFFE;
            RecordTypeDefinitionOffset.Offset := ProgramBlockMaxStackFrameOffset;
          end else
            If Field in VariableData_Flags then
              begin
                TempPtr := Ptr (SymbolTable [stMain].Segment, OffsetToNextMemberOffset);
                Word (TempPtr^) := Ofs (VarIdentifier^);
                OffsetToNextMemberOffset := Ofs (VariableIdentifierData^.W5);
                TempPtr := Ptr (SymbolTable [stMain].Segment, CurrentRecordOrObjectTypeDefinitionOffset + 2);
                RecordTypeDefinitionOffset.Offset := Word (TempPtr^);
                If LongInt (Word (TempPtr^)) + VariableDataSize > $FFFF then Error (StructureTooLarge);
                Inc (Word (TempPtr^), VariableDataSize);
              end;
    With VariableIdentifierData^ do
      begin
        Flags := VariableData_Flags;
        W1.Ofs := RecordTypeDefinitionOffset.TypeOffset;
        W1.Seg := RecordTypeDefinitionOffset.UnitIdentifierData;
        W5 := VariableData_NextMemberOffset;
        UnitTypeOffsets := CurrentVarUnitTypeOffsets;
      end;
    VarIdentifier := Ptr (Seg (VariableIdentifierData^), Ofs (VariableIdentifierData^) + SizeOf (TVariableIdentifierData));
    Dec (NumberOfDeclaredVariableIdentifiers);
  until NumberOfDeclaredVariableIdentifiers = 0;
end;
```

### Code Block 7

```pascal
Procedure CreateVariablesBlockRecord;
begin
  If SizeOfVariablesInDataSegment = $FFFF then Error (TooManyVariables);
  SizeOfVariablesInDataSegment := (SizeOfVariablesInDataSegment + 1) and $FFFE;
  If (SizeOfVariablesInDataSegment <> LastSizeOfVariablesInDataSegment) or ForceCreationOfVariablesBlockRecord then
    begin
      ForceCreationOfVariablesBlockRecord := False;
      With PVariablesBlockRecord (IncreaseSymbolTable (stVariablesBlocks, SizeOf (TVariablesBlockRecord)))^ do
        begin
          Offset := 0;
          VariablesSize := SizeOfVariablesInDataSegment - LastSizeOfVariablesInDataSegment;
          ReferencesSize := 0;
          vrW6 := 0;
        end;
      LastSizeOfVariablesInDataSegment := SizeOfVariablesInDataSegment;
    end;
end;
```

### Code Block 8

```pascal
Function GetVariableType: PTypeDefinition;
Var UnitTypeOffsets: PTypeDefinition;
begin
  UnitTypeOffsets := ProcessVariableTypeDeclaration;
  VariableDataSize := UnitTypeOffsets^.Size;
  VariableData_NextMemberOffset := 0;
  GetVariableType := UnitTypeOffsets;
  GetTypeAndUnitIdentifierOffsets (UnitTypeOffsets, CurrentVarUnitTypeOffsets);
end;

Function ProcessVariableTypeDeclaration: PTypeDefinition;
begin
  LastPointerToTypeDefinitionOffset := 0;
  ProcessVariableTypeDeclaration := ProcessNonObjectTypeDeclaration;
  CheckIfDirecive (_ABSOLUTE, Token_ABSOLUTE);
  ResolvePointerToTypeDefinitions;
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
- https://turbopascal.org/non-object-types
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-variable-declarations
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/storing-identifiers-into-symbol-table
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

