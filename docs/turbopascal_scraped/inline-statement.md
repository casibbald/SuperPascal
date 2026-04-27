# INLINE Statement - Turbo Pascal Compiler

**URL:** https://turbopascal.org/inline-statement

---

## Table of Contents

- INLINE Statement

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
INLINE Statement
This procedure processes
Inline
statement  and generates intermediate code for inline records. Inline records are storend into the main symbol table but are removed after the intermediate code has been generated.
Procedure TStatement.Process_INLINE_Statement;
Var Saved_MainSymbolTableNextRecordOffset: Word;
    InlineCodeSize: Word;
begin
  GetNextToken;
  Saved_MainSymbolTableNextRecordOffset := SymbolTable [stMain].NextRecordOffset;
  GenerateIntermediateCodeForInlineRecords (Process_INLINE (InlineCodeSize), InlineCodeSize);
  SymbolTable [stMain].NextRecordOffset := Saved_MainSymbolTableNextRecordOffset;
  StatementCode := EndSubroutine;
end;
This procedure processes Inline records (variables, procedures, functions and integer values) and stores them into the main symbol table.
Function Process_INLINE (Var InlineCodeSize: Word): PInlineRecord;
Label StoreInlineRecord;
Var Saved_MainSymbolTableNextRecordOffset: Word;
    CurrentInlineIdentifierData: PVariableIdentifierData;
    InlineRecord: TInlineRecord;
    FunctionType: PTypeDefinition;
    PlusMinusToken: TToken;
    TempPtr: Pointer;
    RecSize: Byte;
begin
  Saved_MainSymbolTableNextRecordOffset := SymbolTable [stMain].NextRecordOffset;
  Process_INLINE := SymbolTable [stMain].Ptr;
  TokenForSlash := Token_INLINE_Separator;
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Repeat
    If CheckAndGetNextToken (Token_Greater) then InlineRecord.RecordType := irWord else
      If CheckAndGetNextToken (Token_Lower) then InlineRecord.RecordType := irByte else InlineRecord.RecordType := irDefault;
    CheckForDeclaredIdentifier;
    Case Token of
      Token_VariableIdentifier,
      Token_ProcedureIdentifier:
        begin
          Case Token of
            Token_VariableIdentifier: begin
                                        CurrentInlineIdentifierData := CurrentIdentifierDataPtr;
                                        While vfAbsoluteVar in CurrentInlineIdentifierData^.Flags do
                                          CurrentInlineIdentifierData := PVariableIdentifierData (
                                            PointerFromOffsets (CurrentInlineIdentifierData^.AbsoluteVarDataOffsets));
                                        With CurrentInlineIdentifierData^ do
                                          begin
                                            If Field in Flags then Error (INLINE_Error);
                                            If vf1 in Flags then InlineRecord.W1 := W1.Ofs else
                                              begin
                                                If InlineRecord.RecordType = irByte then Error (INLINE_Error);
                                                Case vf0 in Flags of
                                                  True: InlineRecord.RecordType := irVarAbsoluteAddress;
                                                  else InlineRecord.RecordType  := irVar;
                                                end;
                                                InlineRecord.W3 := W1.Seg;
                                                InlineRecord.W5 :=
                                                  UnitIdentifierDataOffset (Seg (CurrentInlineIdentifierData^));
                                                InlineRecord.W1 := W1.Ofs;
                                              end;
                                          end;
                                      end;
            Token_ProcedureIdentifier: If not FunctionAsResult (FunctionType, InlineRecord.W1) then
                                         begin
                                           InlineRecord.W1 := ExpectIntegerConstant;
                                           Goto StoreInlineRecord;
                                         end;
          end;
          If InlineRecord.RecordType = irDefault then InlineRecord.RecordType := irWord;
          GetNextToken;
          PlusMinusToken := Token;
          If CheckPlusMinusAndGetNextToken then
            Case PlusMinusToken of
              Token_Minus: Dec (InlineRecord.W1, ExpectIntegerConstant);
              else         Inc (InlineRecord.W1, ExpectIntegerConstant);
            end;
        end;
      else InlineRecord.W1 := ExpectIntegerConstant;
    end;

StoreInlineRecord:

    If InlineRecord.RecordType = irDefault then
      Case InlineRecord.WordRec.ByteH of
        0: InlineRecord.RecordType := irByte;
        else InlineRecord.RecordType := irWord;
      end;
    Case InlineRecord.RecordType of
      irByte: RecSize := 2;
      irWord: RecSize := 3;
      else RecSize := 7;
    end;
    TempPtr := IncreaseSymbolTable (stMain, RecSize);
    Move (InlineRecord, TempPtr^, RecSize);
  until not CheckAndGetNextToken (Token_INLINE_Separator);
  TokenForSlash := Token_Slash;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  InlineCodeSize := SymbolTable [stMain].NextRecordOffset - Saved_MainSymbolTableNextRecordOffset;
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
Procedure TStatement.Process_INLINE_Statement;
Var Saved_MainSymbolTableNextRecordOffset: Word;
    InlineCodeSize: Word;
begin
  GetNextToken;
  Saved_MainSymbolTableNextRecordOffset := SymbolTable [stMain].NextRecordOffset;
  GenerateIntermediateCodeForInlineRecords (Process_INLINE (InlineCodeSize), InlineCodeSize);
  SymbolTable [stMain].NextRecordOffset := Saved_MainSymbolTableNextRecordOffset;
  StatementCode := EndSubroutine;
end;
```

### Code Block 2

```pascal
Function Process_INLINE (Var InlineCodeSize: Word): PInlineRecord;
Label StoreInlineRecord;
Var Saved_MainSymbolTableNextRecordOffset: Word;
    CurrentInlineIdentifierData: PVariableIdentifierData;
    InlineRecord: TInlineRecord;
    FunctionType: PTypeDefinition;
    PlusMinusToken: TToken;
    TempPtr: Pointer;
    RecSize: Byte;
begin
  Saved_MainSymbolTableNextRecordOffset := SymbolTable [stMain].NextRecordOffset;
  Process_INLINE := SymbolTable [stMain].Ptr;
  TokenForSlash := Token_INLINE_Separator;
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Repeat
    If CheckAndGetNextToken (Token_Greater) then InlineRecord.RecordType := irWord else
      If CheckAndGetNextToken (Token_Lower) then InlineRecord.RecordType := irByte else InlineRecord.RecordType := irDefault;
    CheckForDeclaredIdentifier;
    Case Token of
      Token_VariableIdentifier,
      Token_ProcedureIdentifier:
        begin
          Case Token of
            Token_VariableIdentifier: begin
                                        CurrentInlineIdentifierData := CurrentIdentifierDataPtr;
                                        While vfAbsoluteVar in CurrentInlineIdentifierData^.Flags do
                                          CurrentInlineIdentifierData := PVariableIdentifierData (
                                            PointerFromOffsets (CurrentInlineIdentifierData^.AbsoluteVarDataOffsets));
                                        With CurrentInlineIdentifierData^ do
                                          begin
                                            If Field in Flags then Error (INLINE_Error);
                                            If vf1 in Flags then InlineRecord.W1 := W1.Ofs else
                                              begin
                                                If InlineRecord.RecordType = irByte then Error (INLINE_Error);
                                                Case vf0 in Flags of
                                                  True: InlineRecord.RecordType := irVarAbsoluteAddress;
                                                  else InlineRecord.RecordType  := irVar;
                                                end;
                                                InlineRecord.W3 := W1.Seg;
                                                InlineRecord.W5 :=
                                                  UnitIdentifierDataOffset (Seg (CurrentInlineIdentifierData^));
                                                InlineRecord.W1 := W1.Ofs;
                                              end;
                                          end;
                                      end;
            Token_ProcedureIdentifier: If not FunctionAsResult (FunctionType, InlineRecord.W1) then
                                         begin
                                           InlineRecord.W1 := ExpectIntegerConstant;
                                           Goto StoreInlineRecord;
                                         end;
          end;
          If InlineRecord.RecordType = irDefault then InlineRecord.RecordType := irWord;
          GetNextToken;
          PlusMinusToken := Token;
          If CheckPlusMinusAndGetNextToken then
            Case PlusMinusToken of
              Token_Minus: Dec (InlineRecord.W1, ExpectIntegerConstant);
              else         Inc (InlineRecord.W1, ExpectIntegerConstant);
            end;
        end;
      else InlineRecord.W1 := ExpectIntegerConstant;
    end;

StoreInlineRecord:

    If InlineRecord.RecordType = irDefault then
      Case InlineRecord.WordRec.ByteH of
        0: InlineRecord.RecordType := irByte;
        else InlineRecord.RecordType := irWord;
      end;
    Case InlineRecord.RecordType of
      irByte: RecSize := 2;
      irWord: RecSize := 3;
      else RecSize := 7;
    end;
    TempPtr := IncreaseSymbolTable (stMain, RecSize);
    Move (InlineRecord, TempPtr^, RecSize);
  until not CheckAndGetNextToken (Token_INLINE_Separator);
  TokenForSlash := Token_Slash;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  InlineCodeSize := SymbolTable [stMain].NextRecordOffset - Saved_MainSymbolTableNextRecordOffset;
end;
```

### Code Block 3

```pascal
Procedure TStatement.Process_INLINE_Statement;
Var Saved_MainSymbolTableNextRecordOffset: Word;
    InlineCodeSize: Word;
begin
  GetNextToken;
  Saved_MainSymbolTableNextRecordOffset := SymbolTable [stMain].NextRecordOffset;
  GenerateIntermediateCodeForInlineRecords (Process_INLINE (InlineCodeSize), InlineCodeSize);
  SymbolTable [stMain].NextRecordOffset := Saved_MainSymbolTableNextRecordOffset;
  StatementCode := EndSubroutine;
end;
```

### Code Block 4

```pascal
Function Process_INLINE (Var InlineCodeSize: Word): PInlineRecord;
Label StoreInlineRecord;
Var Saved_MainSymbolTableNextRecordOffset: Word;
    CurrentInlineIdentifierData: PVariableIdentifierData;
    InlineRecord: TInlineRecord;
    FunctionType: PTypeDefinition;
    PlusMinusToken: TToken;
    TempPtr: Pointer;
    RecSize: Byte;
begin
  Saved_MainSymbolTableNextRecordOffset := SymbolTable [stMain].NextRecordOffset;
  Process_INLINE := SymbolTable [stMain].Ptr;
  TokenForSlash := Token_INLINE_Separator;
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Repeat
    If CheckAndGetNextToken (Token_Greater) then InlineRecord.RecordType := irWord else
      If CheckAndGetNextToken (Token_Lower) then InlineRecord.RecordType := irByte else InlineRecord.RecordType := irDefault;
    CheckForDeclaredIdentifier;
    Case Token of
      Token_VariableIdentifier,
      Token_ProcedureIdentifier:
        begin
          Case Token of
            Token_VariableIdentifier: begin
                                        CurrentInlineIdentifierData := CurrentIdentifierDataPtr;
                                        While vfAbsoluteVar in CurrentInlineIdentifierData^.Flags do
                                          CurrentInlineIdentifierData := PVariableIdentifierData (
                                            PointerFromOffsets (CurrentInlineIdentifierData^.AbsoluteVarDataOffsets));
                                        With CurrentInlineIdentifierData^ do
                                          begin
                                            If Field in Flags then Error (INLINE_Error);
                                            If vf1 in Flags then InlineRecord.W1 := W1.Ofs else
                                              begin
                                                If InlineRecord.RecordType = irByte then Error (INLINE_Error);
                                                Case vf0 in Flags of
                                                  True: InlineRecord.RecordType := irVarAbsoluteAddress;
                                                  else InlineRecord.RecordType  := irVar;
                                                end;
                                                InlineRecord.W3 := W1.Seg;
                                                InlineRecord.W5 :=
                                                  UnitIdentifierDataOffset (Seg (CurrentInlineIdentifierData^));
                                                InlineRecord.W1 := W1.Ofs;
                                              end;
                                          end;
                                      end;
            Token_ProcedureIdentifier: If not FunctionAsResult (FunctionType, InlineRecord.W1) then
                                         begin
                                           InlineRecord.W1 := ExpectIntegerConstant;
                                           Goto StoreInlineRecord;
                                         end;
          end;
          If InlineRecord.RecordType = irDefault then InlineRecord.RecordType := irWord;
          GetNextToken;
          PlusMinusToken := Token;
          If CheckPlusMinusAndGetNextToken then
            Case PlusMinusToken of
              Token_Minus: Dec (InlineRecord.W1, ExpectIntegerConstant);
              else         Inc (InlineRecord.W1, ExpectIntegerConstant);
            end;
        end;
      else InlineRecord.W1 := ExpectIntegerConstant;
    end;

StoreInlineRecord:

    If InlineRecord.RecordType = irDefault then
      Case InlineRecord.WordRec.ByteH of
        0: InlineRecord.RecordType := irByte;
        else InlineRecord.RecordType := irWord;
      end;
    Case InlineRecord.RecordType of
      irByte: RecSize := 2;
      irWord: RecSize := 3;
      else RecSize := 7;
    end;
    TempPtr := IncreaseSymbolTable (stMain, RecSize);
    Move (InlineRecord, TempPtr^, RecSize);
  until not CheckAndGetNextToken (Token_INLINE_Separator);
  TokenForSlash := Token_Slash;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  InlineCodeSize := SymbolTable [stMain].NextRecordOffset - Saved_MainSymbolTableNextRecordOffset;
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
- https://turbopascal.org/inline-statement
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

