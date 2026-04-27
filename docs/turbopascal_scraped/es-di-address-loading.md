# ES:DI Address Loading - Turbo Pascal Compiler

**URL:** https://turbopascal.org/es-di-address-loading

---

## Table of Contents

- ES:DI Address Loading

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
ES:DI Address Loading
Turbo Pascal tries to optimize loading of addresses to
ES:DI
register pair. For this purpose it uses few
intermediate code instructions
that track addresses that need to be loaded. The first intermediate code instruction will load local variable address to
ES:DI
registers.
Procedure Load_ES_DI (LocalVarProcedure: Word; LocalVarDisplacement: Integer; IntermediateCodeToSaveAddress: Word);
begin
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 7))^ do
    begin
      RecordType           := icLoad_ES_DI;
      DestinationProcedure := LocalVarProcedure;
      Displacement         := LocalVarDisplacement;
      SaveAddressCode      := IntermediateCodeToSaveAddress;
    end;
end;
Turbo Pascal assumes that the address in the
ES:DI
register pair will remain unchanged. Everywhere where either
ES
or
DI
registers are changed, Turbo Pascal generates intermediate code instruction to indicate that the address is no longer in registers.
Procedure ES_DI_PointerDestroyed;
begin
  PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 1))^.RecordType := ic_ES_DI_Destroyed;
end;
At the code generation phase Turbo Pascal goes through all intermediate code instructions for
ES:DI
loading and removes redundant loads.
Procedure RemoveRedundant_ES_DI_AddressLoading;
  Var IntermediateCodePointer: PIntermediateCodeRecord;
      IntermediateCodePointerRec: PtrRec absolute IntermediateCodePointer;
      CurrentLoadedAddress_Displacement: Integer;
      CurrentLoadedAddress_Procedure: Word;
  begin
    IntermediateCodePointer := Ptr (SymbolTable [stIntermediateCode].Segment, LastIntermediateCodeSubroutine);
    CurrentLoadedAddress_Displacement := 0;
    CurrentLoadedAddress_Procedure := 0;
    Repeat
      Case IntermediateCodePointer^.RecordType of
        icGoSub: begin
                   Asm
                     MOV    AX, WORD PTR IntermediateCodePointer
                     ADD    AX, 3
                     PUSH   AX
                   end;
                   IntermediateCodePointerRec.Ofs := IntermediateCodePointer^.NewOffset;
                 end;
        icReturn: Asm
                    POP    WORD PTR IntermediateCodePointer
                  end;
        icByte,
        ic12: Inc (IntermediateCodePointerRec.Ofs, 2);
        icWord,
        icSourceLineNumber,
        icSkippedGoSubSaveAddress: Inc (IntermediateCodePointerRec.Ofs, 3);
        icJumpNear,
        icJumpShort: Inc (IntermediateCodePointerRec.Ofs, 8);
        icLabel: begin
                   Inc (IntermediateCodePointerRec.Ofs, 3);
                   CurrentLoadedAddress_Displacement := 0;
                   CurrentLoadedAddress_Procedure := 0;
                 end;
        icReference,
        icReferenceAsmBlock: Inc (IntermediateCodePointerRec.Ofs, 7);
        icLoad_ES_DI: With IntermediateCodePointer^ do
                         begin
                           If (CurrentLoadedAddress_Procedure = DestinationProcedure) and
                              (CurrentLoadedAddress_Displacement = Displacement) then RecordType := ic_ES_DI_Loaded
                             else begin
                                    CurrentLoadedAddress_Procedure := DestinationProcedure;
                                    CurrentLoadedAddress_Displacement := Displacement;
                                    If SaveAddressCode <> 0 then PIntermediateCodeRecord (
                                                    Ptr (IntermediateCodePointerRec.Seg,
                                                         SaveAddressCode))^.RecordType := icGoSub;
                                  end;
                           Inc (IntermediateCodePointerRec.Ofs, 7);
                         end;
        ic_ES_DI_Loaded: With IntermediateCodePointer^ do
                           begin
                             CurrentLoadedAddress_Procedure := DestinationProcedure;
                             CurrentLoadedAddress_Displacement := Displacement;
                             Inc (IntermediateCodePointerRec.Ofs, 7);
                           end;
        ic_ES_DI_Destroyed: begin
                              CurrentLoadedAddress_Displacement := 0;
                              CurrentLoadedAddress_Procedure := 0;
                              Inc (IntermediateCodePointerRec.Ofs);
                            end;
        icEnd: Break;
        {$IFDEF DEBUG}
        else Writeln (#13#10'ERROR: Unknown ic CODE'); { ERROR, not in TPC }
        {$ENDIF}
      end;
    until False;
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
Procedure Load_ES_DI (LocalVarProcedure: Word; LocalVarDisplacement: Integer; IntermediateCodeToSaveAddress: Word);
begin
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 7))^ do
    begin
      RecordType           := icLoad_ES_DI;
      DestinationProcedure := LocalVarProcedure;
      Displacement         := LocalVarDisplacement;
      SaveAddressCode      := IntermediateCodeToSaveAddress;
    end;
end;
```

### Code Block 2

```pascal
Procedure ES_DI_PointerDestroyed;
begin
  PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 1))^.RecordType := ic_ES_DI_Destroyed;
end;
```

### Code Block 3

```pascal
  Procedure RemoveRedundant_ES_DI_AddressLoading;
  Var IntermediateCodePointer: PIntermediateCodeRecord;
      IntermediateCodePointerRec: PtrRec absolute IntermediateCodePointer;
      CurrentLoadedAddress_Displacement: Integer;
      CurrentLoadedAddress_Procedure: Word;
  begin
    IntermediateCodePointer := Ptr (SymbolTable [stIntermediateCode].Segment, LastIntermediateCodeSubroutine);
    CurrentLoadedAddress_Displacement := 0;
    CurrentLoadedAddress_Procedure := 0;
    Repeat
      Case IntermediateCodePointer^.RecordType of
        icGoSub: begin
                   Asm
                     MOV    AX, WORD PTR IntermediateCodePointer
                     ADD    AX, 3
                     PUSH   AX
                   end;
                   IntermediateCodePointerRec.Ofs := IntermediateCodePointer^.NewOffset;
                 end;
        icReturn: Asm
                    POP    WORD PTR IntermediateCodePointer
                  end;
        icByte,
        ic12: Inc (IntermediateCodePointerRec.Ofs, 2);
        icWord,
        icSourceLineNumber,
        icSkippedGoSubSaveAddress: Inc (IntermediateCodePointerRec.Ofs, 3);
        icJumpNear,
        icJumpShort: Inc (IntermediateCodePointerRec.Ofs, 8);
        icLabel: begin
                   Inc (IntermediateCodePointerRec.Ofs, 3);
                   CurrentLoadedAddress_Displacement := 0;
                   CurrentLoadedAddress_Procedure := 0;
                 end;
        icReference,
        icReferenceAsmBlock: Inc (IntermediateCodePointerRec.Ofs, 7);
        icLoad_ES_DI: With IntermediateCodePointer^ do
                         begin
                           If (CurrentLoadedAddress_Procedure = DestinationProcedure) and
                              (CurrentLoadedAddress_Displacement = Displacement) then RecordType := ic_ES_DI_Loaded
                             else begin
                                    CurrentLoadedAddress_Procedure := DestinationProcedure;
                                    CurrentLoadedAddress_Displacement := Displacement;
                                    If SaveAddressCode <> 0 then PIntermediateCodeRecord (
                                                    Ptr (IntermediateCodePointerRec.Seg,
                                                         SaveAddressCode))^.RecordType := icGoSub;
                                  end;
                           Inc (IntermediateCodePointerRec.Ofs, 7);
                         end;
        ic_ES_DI_Loaded: With IntermediateCodePointer^ do
                           begin
                             CurrentLoadedAddress_Procedure := DestinationProcedure;
                             CurrentLoadedAddress_Displacement := Displacement;
                             Inc (IntermediateCodePointerRec.Ofs, 7);
                           end;
        ic_ES_DI_Destroyed: begin
                              CurrentLoadedAddress_Displacement := 0;
                              CurrentLoadedAddress_Procedure := 0;
                              Inc (IntermediateCodePointerRec.Ofs);
                            end;
        icEnd: Break;
        {$IFDEF DEBUG}
        else Writeln (#13#10'ERROR: Unknown ic CODE'); { ERROR, not in TPC }
        {$ENDIF}
      end;
    until False;
  end;
```

### Code Block 4

```pascal
Procedure Load_ES_DI (LocalVarProcedure: Word; LocalVarDisplacement: Integer; IntermediateCodeToSaveAddress: Word);
begin
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 7))^ do
    begin
      RecordType           := icLoad_ES_DI;
      DestinationProcedure := LocalVarProcedure;
      Displacement         := LocalVarDisplacement;
      SaveAddressCode      := IntermediateCodeToSaveAddress;
    end;
end;
```

### Code Block 5

```pascal
Procedure ES_DI_PointerDestroyed;
begin
  PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 1))^.RecordType := ic_ES_DI_Destroyed;
end;
```

### Code Block 6

```pascal
  Procedure RemoveRedundant_ES_DI_AddressLoading;
  Var IntermediateCodePointer: PIntermediateCodeRecord;
      IntermediateCodePointerRec: PtrRec absolute IntermediateCodePointer;
      CurrentLoadedAddress_Displacement: Integer;
      CurrentLoadedAddress_Procedure: Word;
  begin
    IntermediateCodePointer := Ptr (SymbolTable [stIntermediateCode].Segment, LastIntermediateCodeSubroutine);
    CurrentLoadedAddress_Displacement := 0;
    CurrentLoadedAddress_Procedure := 0;
    Repeat
      Case IntermediateCodePointer^.RecordType of
        icGoSub: begin
                   Asm
                     MOV    AX, WORD PTR IntermediateCodePointer
                     ADD    AX, 3
                     PUSH   AX
                   end;
                   IntermediateCodePointerRec.Ofs := IntermediateCodePointer^.NewOffset;
                 end;
        icReturn: Asm
                    POP    WORD PTR IntermediateCodePointer
                  end;
        icByte,
        ic12: Inc (IntermediateCodePointerRec.Ofs, 2);
        icWord,
        icSourceLineNumber,
        icSkippedGoSubSaveAddress: Inc (IntermediateCodePointerRec.Ofs, 3);
        icJumpNear,
        icJumpShort: Inc (IntermediateCodePointerRec.Ofs, 8);
        icLabel: begin
                   Inc (IntermediateCodePointerRec.Ofs, 3);
                   CurrentLoadedAddress_Displacement := 0;
                   CurrentLoadedAddress_Procedure := 0;
                 end;
        icReference,
        icReferenceAsmBlock: Inc (IntermediateCodePointerRec.Ofs, 7);
        icLoad_ES_DI: With IntermediateCodePointer^ do
                         begin
                           If (CurrentLoadedAddress_Procedure = DestinationProcedure) and
                              (CurrentLoadedAddress_Displacement = Displacement) then RecordType := ic_ES_DI_Loaded
                             else begin
                                    CurrentLoadedAddress_Procedure := DestinationProcedure;
                                    CurrentLoadedAddress_Displacement := Displacement;
                                    If SaveAddressCode <> 0 then PIntermediateCodeRecord (
                                                    Ptr (IntermediateCodePointerRec.Seg,
                                                         SaveAddressCode))^.RecordType := icGoSub;
                                  end;
                           Inc (IntermediateCodePointerRec.Ofs, 7);
                         end;
        ic_ES_DI_Loaded: With IntermediateCodePointer^ do
                           begin
                             CurrentLoadedAddress_Procedure := DestinationProcedure;
                             CurrentLoadedAddress_Displacement := Displacement;
                             Inc (IntermediateCodePointerRec.Ofs, 7);
                           end;
        ic_ES_DI_Destroyed: begin
                              CurrentLoadedAddress_Displacement := 0;
                              CurrentLoadedAddress_Procedure := 0;
                              Inc (IntermediateCodePointerRec.Ofs);
                            end;
        icEnd: Break;
        {$IFDEF DEBUG}
        else Writeln (#13#10'ERROR: Unknown ic CODE'); { ERROR, not in TPC }
        {$ENDIF}
      end;
    until False;
  end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/es-di-address-loading
- https://turbopascal.org/expressions
- https://turbopascal.org/intermediate-code
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

