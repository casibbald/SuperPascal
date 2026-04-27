# Identifiers In Assembler Instructions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/identifiers-in-assembler-instructions

---

## Table of Contents

- Identifiers In Assembler Instructions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Identifiers In Assembler Instructions
This function checks if identifier in instruction is valid and returns its data. Identifier can be a local lable starting with the
@
sign or any other
declared Turbo Pascal identifier
.
Function IsValidAsmIdentifier (P: Pointer; Var Tok: TToken; Var IdentifierData: Pointer; Var IdentifierPointer: Word): Boolean;
Var RecordTypeDefinition: PRecordTypeDefinition absolute P;
    VariableIdentifierData: PVariableIdentifierData absolute IdentifierData;
    NewIdentifier: PIdentifier;
begin
  If CurrentIdentifier [1] = '@' then
    begin
      If IsIdentifierInSymbolTable (@SpecialAsmIdentifiers, Tok, IdentifierData, IdentifierPointer) then
        IsValidAsmIdentifier := True else
          If IsIdentifierInSymbolTable (Ptr (SymbolTable [stMain].Segment, TemporaryAssemblerStatementIdentifierTable),
                                 Tok, IdentifierData, IdentifierPointer) then
            IsValidAsmIdentifier := True else
              begin
                IdentifierData := StoreCurrentIdentifierToSymbolTable (Ptr (SymbolTable [stMain].Segment,
                                    TemporaryAssemblerStatementIdentifierTable), 2, NewIdentifier);
                NewIdentifier^.Token := Token_LabelIdentifier;
                IdentifierPointer := Ofs (NewIdentifier^);
                Tok := Token_LabelIdentifier;
                IsValidAsmIdentifier := True;
              end;
      Exit;
    end;
  If P = nil then IsValidAsmIdentifier := FindCurrentIdentifier (Tok, IdentifierPointer, IdentifierData) else
    If Ofs (P^) = 0 then
      begin
        If FindIdentifierInUnit (P, Tok, IdentifierPointer, IdentifierData) then IsValidAsmIdentifier := True else
          IsValidAsmIdentifier := FindCurrentIdentifier (Tok, IdentifierPointer, IdentifierData);
      end else
        begin
          If IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (RecordTypeDefinition,
                                                                  VariableIdentifierData,
                                                                  Tok,
                                                                  IdentifierPointer) then
            IsValidAsmIdentifier := True else
              IsValidAsmIdentifier := FindCurrentIdentifier (Tok, IdentifierPointer, IdentifierData);
        end;
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
Function IsValidAsmIdentifier (P: Pointer; Var Tok: TToken; Var IdentifierData: Pointer; Var IdentifierPointer: Word): Boolean;
Var RecordTypeDefinition: PRecordTypeDefinition absolute P;
    VariableIdentifierData: PVariableIdentifierData absolute IdentifierData;
    NewIdentifier: PIdentifier;
begin
  If CurrentIdentifier [1] = '@' then
    begin
      If IsIdentifierInSymbolTable (@SpecialAsmIdentifiers, Tok, IdentifierData, IdentifierPointer) then
        IsValidAsmIdentifier := True else
          If IsIdentifierInSymbolTable (Ptr (SymbolTable [stMain].Segment, TemporaryAssemblerStatementIdentifierTable),
                                 Tok, IdentifierData, IdentifierPointer) then
            IsValidAsmIdentifier := True else
              begin
                IdentifierData := StoreCurrentIdentifierToSymbolTable (Ptr (SymbolTable [stMain].Segment,
                                    TemporaryAssemblerStatementIdentifierTable), 2, NewIdentifier);
                NewIdentifier^.Token := Token_LabelIdentifier;
                IdentifierPointer := Ofs (NewIdentifier^);
                Tok := Token_LabelIdentifier;
                IsValidAsmIdentifier := True;
              end;
      Exit;
    end;
  If P = nil then IsValidAsmIdentifier := FindCurrentIdentifier (Tok, IdentifierPointer, IdentifierData) else
    If Ofs (P^) = 0 then
      begin
        If FindIdentifierInUnit (P, Tok, IdentifierPointer, IdentifierData) then IsValidAsmIdentifier := True else
          IsValidAsmIdentifier := FindCurrentIdentifier (Tok, IdentifierPointer, IdentifierData);
      end else
        begin
          If IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (RecordTypeDefinition,
                                                                  VariableIdentifierData,
                                                                  Tok,
                                                                  IdentifierPointer) then
            IsValidAsmIdentifier := True else
              IsValidAsmIdentifier := FindCurrentIdentifier (Tok, IdentifierPointer, IdentifierData);
        end;
end;
```

### Code Block 2

```pascal
Function IsValidAsmIdentifier (P: Pointer; Var Tok: TToken; Var IdentifierData: Pointer; Var IdentifierPointer: Word): Boolean;
Var RecordTypeDefinition: PRecordTypeDefinition absolute P;
    VariableIdentifierData: PVariableIdentifierData absolute IdentifierData;
    NewIdentifier: PIdentifier;
begin
  If CurrentIdentifier [1] = '@' then
    begin
      If IsIdentifierInSymbolTable (@SpecialAsmIdentifiers, Tok, IdentifierData, IdentifierPointer) then
        IsValidAsmIdentifier := True else
          If IsIdentifierInSymbolTable (Ptr (SymbolTable [stMain].Segment, TemporaryAssemblerStatementIdentifierTable),
                                 Tok, IdentifierData, IdentifierPointer) then
            IsValidAsmIdentifier := True else
              begin
                IdentifierData := StoreCurrentIdentifierToSymbolTable (Ptr (SymbolTable [stMain].Segment,
                                    TemporaryAssemblerStatementIdentifierTable), 2, NewIdentifier);
                NewIdentifier^.Token := Token_LabelIdentifier;
                IdentifierPointer := Ofs (NewIdentifier^);
                Tok := Token_LabelIdentifier;
                IsValidAsmIdentifier := True;
              end;
      Exit;
    end;
  If P = nil then IsValidAsmIdentifier := FindCurrentIdentifier (Tok, IdentifierPointer, IdentifierData) else
    If Ofs (P^) = 0 then
      begin
        If FindIdentifierInUnit (P, Tok, IdentifierPointer, IdentifierData) then IsValidAsmIdentifier := True else
          IsValidAsmIdentifier := FindCurrentIdentifier (Tok, IdentifierPointer, IdentifierData);
      end else
        begin
          If IsCurrentIdentifierDeclaredAsMemberInRecordOrObject (RecordTypeDefinition,
                                                                  VariableIdentifierData,
                                                                  Tok,
                                                                  IdentifierPointer) then
            IsValidAsmIdentifier := True else
              IsValidAsmIdentifier := FindCurrentIdentifier (Tok, IdentifierPointer, IdentifierData);
        end;
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
- https://turbopascal.org/identifiers-in-assembler-instructions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/scanner
- https://turbopascal.org/searching-for-identifiers
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

