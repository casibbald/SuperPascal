# Loading Variables - Turbo Pascal Compiler

**URL:** https://turbopascal.org/loading-variables

---

## Table of Contents

- Loading Variables

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Loading Variables
This procedure loads variable data from symbol table and sets all expression fields accordingly. Variables declared outside procedures and functions are called global variables, and they reside in the data segment. Variables declared within procedures and functions are called local variables, and they reside in the stack segment. Absolute variables are loaded from the target variable which is in memory and for record or object fields base address is loaded.
Procedure TExpression.LoadVariable;
Var IdentifierDataPtr: PVariableIdentifierData;
begin
  GetNextToken;
  IdentifierDataPtr := CurrentVariableIdentifierData;
  TypeDefPtr := PointerFromOffsets (IdentifierDataPtr^.UnitTypeOffsets);
  DataType := TypeDefPtr^.DataType;
  Location := elMemory;
  UsedRegisters := [];
  IdentifierDataPtr := CurrentVariableIdentifierData;
  While vfAbsoluteVar in IdentifierDataPtr^.Flags do
    IdentifierDataPtr := PVariableIdentifierData (PointerFromOffsets (IdentifierDataPtr^.AbsoluteVarDataOffsets));
  If Field in IdentifierDataPtr^.Flags then
    begin
      LoadSelfAddress;
      Inc (Value.VarOffset, IdentifierDataPtr^.W1.Ofs);
      Exit;
    end;
  Value.VarOffset := IdentifierDataPtr^.W1.Ofs;
  If IdentifierDataPtr^.Flags * VariableTypeMask = GlobalVariable then
    begin
      LocationData.Flags := [segDS];
      Value.VarSegment := IdentifierDataPtr^.W1.Seg;
      Value.W14 := Seg (IdentifierDataPtr^);
    end else If IdentifierDataPtr^.Flags * VariableTypeMask = TypedConstant then
               begin
                 LocationData.Flags := [efTypedConstant, segDS];
                 Value.VarSegment := IdentifierDataPtr^.W1.Seg;
                 Value.W14 := Seg (IdentifierDataPtr^);
               end else If IdentifierDataPtr^.Flags * VariableTypeMask = LocalStackVariable then
                          begin
                            Value.IdentifierDataPointer := IdentifierDataPtr;
                            LoadAddressOfLocalVariable (IdentifierDataPtr^.Flags, IdentifierDataPtr^.W1.Seg);
                          end else begin
                                     Value.VarSegment := IdentifierDataPtr^.W1.Seg;
                                     LocationData.Flags := [];
                                   end;
  If vfConst in IdentifierDataPtr^.Flags then Include (LocationData.Flags, efReadOnly);
  EndIntermediateCodeSubroutine;
end;
This procedure generates code to load the address of local variable (on stack).
Procedure TExpression.LoadAddressOfLocalVariable (Flags: TVarFlagsSet; ProcIdentifier: Word);
begin
  If Flags * [vfOpenParameter, vfVar] <> [] then
    begin
      LocationData.Flags := [ofsDI, segES];
      Load_ES_DI (ProcIdentifier, Value.Offset, 0);
      Value.Offset := 0;
    end else begin
               LocationData.Flags := [ofsBP, segSS];
               If ProcIdentifier <> CurrentProcedureIdentifier then
                 begin
                   LocationData.Flags := [ofsDI, segSS];
                   Load_ES_DI (ProcIdentifier, 0, 0);
                 end;
             end;
end;
This procedure loads the address of
Record
or
Object
field variable.
Procedure TExpression.LoadSelfAddress;
Var MethodIdentifier: PIdentifier;
    MethodIdentifierData: PProcedureIdentifierData;
    WithRecord: PWithRecord;
begin
  Case CurrentWithRecord = 0 of
    True: begin
            FindCurrentMethod (MethodIdentifier, MethodIdentifierData);
            Load_ES_DI (Ofs (MethodIdentifier^), 6, 0);
            LocationData.Flags := [ofsDI, segES];
            Value.Word := 0;
          end;
    else begin
           WithRecord := Ptr (SSeg, CurrentWithRecord);
           Location := elMemory;
           LocationData.Flags := WithRecord^.ExpressionFlags;
           Case WithRecord^.RecordType of
             1: begin
                  Value.Offset      := WithRecord^.AddressOffset;
                  Value.BlockRecord := WithRecord^.BlockRecord;
                  Value.Segment     := WithRecord^.AddressSegment;
                end;
             2: begin      { Save Address }
                  Load_ES_DI (CurrentProcedureIdentifier, WithRecord^.AddressOffset, WithRecord^.BlockRecord);
                  Value.Word := 0;
                end;
           end;
         end;
  end;
  EndIntermediateCodeSubroutine;
end;
This procedure finds the current method. It jumps out of nested procedures until it finds a method. If method is found it returns
True
and method’s identifier and data address, otherwise it returns
False
.
Function FindCurrentMethod (Var MethodIdentifier: PIdentifier; Var MethodIdentifierData: PProcedureIdentifierData): Boolean;
Var MethodIdentifierOfs: Word absolute MethodIdentifier;
    MethodIdentifierDataOfs: Word absolute MethodIdentifierData;
begin
  MethodIdentifier := Ptr (MainSymbolTable.Segment, CurrentProcedureIdentifier);
  MethodIdentifierData := Ptr (MainSymbolTable.Segment, CurrentProcedureIdentifier);
  FindCurrentMethod := True;
  While MethodIdentifierOfs <> 0 do
    begin
      MethodIdentifierDataOfs := MethodIdentifierOfs + MethodIdentifier^.Name.Len + 4;
      If pfMethod in MethodIdentifierData^.Flags then Exit;
      MethodIdentifierOfs := MethodIdentifierData^.OuterBlockProcedureIdentifier;
    end;
  FindCurrentMethod := False;
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
Procedure TExpression.LoadVariable;
Var IdentifierDataPtr: PVariableIdentifierData;
begin
  GetNextToken;
  IdentifierDataPtr := CurrentVariableIdentifierData;
  TypeDefPtr := PointerFromOffsets (IdentifierDataPtr^.UnitTypeOffsets);
  DataType := TypeDefPtr^.DataType;
  Location := elMemory;
  UsedRegisters := [];
  IdentifierDataPtr := CurrentVariableIdentifierData;
  While vfAbsoluteVar in IdentifierDataPtr^.Flags do
    IdentifierDataPtr := PVariableIdentifierData (PointerFromOffsets (IdentifierDataPtr^.AbsoluteVarDataOffsets));
  If Field in IdentifierDataPtr^.Flags then
    begin
      LoadSelfAddress;
      Inc (Value.VarOffset, IdentifierDataPtr^.W1.Ofs);
      Exit;
    end;
  Value.VarOffset := IdentifierDataPtr^.W1.Ofs;
  If IdentifierDataPtr^.Flags * VariableTypeMask = GlobalVariable then
    begin
      LocationData.Flags := [segDS];
      Value.VarSegment := IdentifierDataPtr^.W1.Seg;
      Value.W14 := Seg (IdentifierDataPtr^);
    end else If IdentifierDataPtr^.Flags * VariableTypeMask = TypedConstant then
               begin
                 LocationData.Flags := [efTypedConstant, segDS];
                 Value.VarSegment := IdentifierDataPtr^.W1.Seg;
                 Value.W14 := Seg (IdentifierDataPtr^);
               end else If IdentifierDataPtr^.Flags * VariableTypeMask = LocalStackVariable then
                          begin
                            Value.IdentifierDataPointer := IdentifierDataPtr;
                            LoadAddressOfLocalVariable (IdentifierDataPtr^.Flags, IdentifierDataPtr^.W1.Seg);
                          end else begin
                                     Value.VarSegment := IdentifierDataPtr^.W1.Seg;
                                     LocationData.Flags := [];
                                   end;
  If vfConst in IdentifierDataPtr^.Flags then Include (LocationData.Flags, efReadOnly);
  EndIntermediateCodeSubroutine;
end;
```

### Code Block 2

```pascal
Procedure TExpression.LoadAddressOfLocalVariable (Flags: TVarFlagsSet; ProcIdentifier: Word);
begin
  If Flags * [vfOpenParameter, vfVar] <> [] then
    begin
      LocationData.Flags := [ofsDI, segES];
      Load_ES_DI (ProcIdentifier, Value.Offset, 0);
      Value.Offset := 0;
    end else begin
               LocationData.Flags := [ofsBP, segSS];
               If ProcIdentifier <> CurrentProcedureIdentifier then
                 begin
                   LocationData.Flags := [ofsDI, segSS];
                   Load_ES_DI (ProcIdentifier, 0, 0);
                 end;
             end;
end;
```

### Code Block 3

```pascal
Procedure TExpression.LoadSelfAddress;
Var MethodIdentifier: PIdentifier;
    MethodIdentifierData: PProcedureIdentifierData;
    WithRecord: PWithRecord;
begin
  Case CurrentWithRecord = 0 of
    True: begin
            FindCurrentMethod (MethodIdentifier, MethodIdentifierData);
            Load_ES_DI (Ofs (MethodIdentifier^), 6, 0);
            LocationData.Flags := [ofsDI, segES];
            Value.Word := 0;
          end;
    else begin
           WithRecord := Ptr (SSeg, CurrentWithRecord);
           Location := elMemory;
           LocationData.Flags := WithRecord^.ExpressionFlags;
           Case WithRecord^.RecordType of
             1: begin
                  Value.Offset      := WithRecord^.AddressOffset;
                  Value.BlockRecord := WithRecord^.BlockRecord;
                  Value.Segment     := WithRecord^.AddressSegment;
                end;
             2: begin      { Save Address }
                  Load_ES_DI (CurrentProcedureIdentifier, WithRecord^.AddressOffset, WithRecord^.BlockRecord);
                  Value.Word := 0;
                end;
           end;
         end;
  end;
  EndIntermediateCodeSubroutine;
end;
```

### Code Block 4

```pascal
Function FindCurrentMethod (Var MethodIdentifier: PIdentifier; Var MethodIdentifierData: PProcedureIdentifierData): Boolean;
Var MethodIdentifierOfs: Word absolute MethodIdentifier;
    MethodIdentifierDataOfs: Word absolute MethodIdentifierData;
begin
  MethodIdentifier := Ptr (MainSymbolTable.Segment, CurrentProcedureIdentifier);
  MethodIdentifierData := Ptr (MainSymbolTable.Segment, CurrentProcedureIdentifier);
  FindCurrentMethod := True;
  While MethodIdentifierOfs <> 0 do
    begin
      MethodIdentifierDataOfs := MethodIdentifierOfs + MethodIdentifier^.Name.Len + 4;
      If pfMethod in MethodIdentifierData^.Flags then Exit;
      MethodIdentifierOfs := MethodIdentifierData^.OuterBlockProcedureIdentifier;
    end;
  FindCurrentMethod := False;
end;
```

### Code Block 5

```pascal
Procedure TExpression.LoadVariable;
Var IdentifierDataPtr: PVariableIdentifierData;
begin
  GetNextToken;
  IdentifierDataPtr := CurrentVariableIdentifierData;
  TypeDefPtr := PointerFromOffsets (IdentifierDataPtr^.UnitTypeOffsets);
  DataType := TypeDefPtr^.DataType;
  Location := elMemory;
  UsedRegisters := [];
  IdentifierDataPtr := CurrentVariableIdentifierData;
  While vfAbsoluteVar in IdentifierDataPtr^.Flags do
    IdentifierDataPtr := PVariableIdentifierData (PointerFromOffsets (IdentifierDataPtr^.AbsoluteVarDataOffsets));
  If Field in IdentifierDataPtr^.Flags then
    begin
      LoadSelfAddress;
      Inc (Value.VarOffset, IdentifierDataPtr^.W1.Ofs);
      Exit;
    end;
  Value.VarOffset := IdentifierDataPtr^.W1.Ofs;
  If IdentifierDataPtr^.Flags * VariableTypeMask = GlobalVariable then
    begin
      LocationData.Flags := [segDS];
      Value.VarSegment := IdentifierDataPtr^.W1.Seg;
      Value.W14 := Seg (IdentifierDataPtr^);
    end else If IdentifierDataPtr^.Flags * VariableTypeMask = TypedConstant then
               begin
                 LocationData.Flags := [efTypedConstant, segDS];
                 Value.VarSegment := IdentifierDataPtr^.W1.Seg;
                 Value.W14 := Seg (IdentifierDataPtr^);
               end else If IdentifierDataPtr^.Flags * VariableTypeMask = LocalStackVariable then
                          begin
                            Value.IdentifierDataPointer := IdentifierDataPtr;
                            LoadAddressOfLocalVariable (IdentifierDataPtr^.Flags, IdentifierDataPtr^.W1.Seg);
                          end else begin
                                     Value.VarSegment := IdentifierDataPtr^.W1.Seg;
                                     LocationData.Flags := [];
                                   end;
  If vfConst in IdentifierDataPtr^.Flags then Include (LocationData.Flags, efReadOnly);
  EndIntermediateCodeSubroutine;
end;
```

### Code Block 6

```pascal
Procedure TExpression.LoadAddressOfLocalVariable (Flags: TVarFlagsSet; ProcIdentifier: Word);
begin
  If Flags * [vfOpenParameter, vfVar] <> [] then
    begin
      LocationData.Flags := [ofsDI, segES];
      Load_ES_DI (ProcIdentifier, Value.Offset, 0);
      Value.Offset := 0;
    end else begin
               LocationData.Flags := [ofsBP, segSS];
               If ProcIdentifier <> CurrentProcedureIdentifier then
                 begin
                   LocationData.Flags := [ofsDI, segSS];
                   Load_ES_DI (ProcIdentifier, 0, 0);
                 end;
             end;
end;
```

### Code Block 7

```pascal
Procedure TExpression.LoadSelfAddress;
Var MethodIdentifier: PIdentifier;
    MethodIdentifierData: PProcedureIdentifierData;
    WithRecord: PWithRecord;
begin
  Case CurrentWithRecord = 0 of
    True: begin
            FindCurrentMethod (MethodIdentifier, MethodIdentifierData);
            Load_ES_DI (Ofs (MethodIdentifier^), 6, 0);
            LocationData.Flags := [ofsDI, segES];
            Value.Word := 0;
          end;
    else begin
           WithRecord := Ptr (SSeg, CurrentWithRecord);
           Location := elMemory;
           LocationData.Flags := WithRecord^.ExpressionFlags;
           Case WithRecord^.RecordType of
             1: begin
                  Value.Offset      := WithRecord^.AddressOffset;
                  Value.BlockRecord := WithRecord^.BlockRecord;
                  Value.Segment     := WithRecord^.AddressSegment;
                end;
             2: begin      { Save Address }
                  Load_ES_DI (CurrentProcedureIdentifier, WithRecord^.AddressOffset, WithRecord^.BlockRecord);
                  Value.Word := 0;
                end;
           end;
         end;
  end;
  EndIntermediateCodeSubroutine;
end;
```

### Code Block 8

```pascal
Function FindCurrentMethod (Var MethodIdentifier: PIdentifier; Var MethodIdentifierData: PProcedureIdentifierData): Boolean;
Var MethodIdentifierOfs: Word absolute MethodIdentifier;
    MethodIdentifierDataOfs: Word absolute MethodIdentifierData;
begin
  MethodIdentifier := Ptr (MainSymbolTable.Segment, CurrentProcedureIdentifier);
  MethodIdentifierData := Ptr (MainSymbolTable.Segment, CurrentProcedureIdentifier);
  FindCurrentMethod := True;
  While MethodIdentifierOfs <> 0 do
    begin
      MethodIdentifierDataOfs := MethodIdentifierOfs + MethodIdentifier^.Name.Len + 4;
      If pfMethod in MethodIdentifierData^.Flags then Exit;
      MethodIdentifierOfs := MethodIdentifierData^.OuterBlockProcedureIdentifier;
    end;
  FindCurrentMethod := False;
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
- https://turbopascal.org/loading-variables
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

