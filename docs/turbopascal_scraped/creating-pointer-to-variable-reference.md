# Creating Pointer to Variable Reference - Turbo Pascal Compiler

**URL:** https://turbopascal.org/creating-pointer-to-variable-reference

---

## Table of Contents

- Creating Pointer to Variable Reference

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Creating Pointer to Variable Reference
This procedure creates a pointer to any variable reference including procedures and functions. It is used for the
@
operator and system functions
Addr
,
Seg
and
Ofs
. Its reverse operation is
pointer dereference
.
Procedure TExpression.CreatePointerToVariableReference;
Var PointerTypeDefinition: PPointerTypeDefinition;
    UnitIdentifierData: PUnitIdentifierData;
    ProcedureIdentifierData: PProcedureIdentifierData;
begin
  If Location = elProcedure then
    begin
      ProcedureIdentifierData := PProcedureIdentifierData (PChar (TypeDefPtr) - 10);
      If ((pfMethod in ProcedureIdentifierData^.Flags) and not (mcStatic in DataType)) or
         (pfInline in ProcedureIdentifierData^.Flags) then Error (InvalidProcedureOrFunctionReference);
      Location := elPointerToMemory;
      DataType := [it32Bit, it16Bit, itSigned];
      UsedRegisters := [];
      LocationData.Flags := [efTypedConstant];
      Value.Offset := 0;
      Value.BlockRecord := ProcedureIdentifierData^.ProceduresRecordOffset;
      Value.Segment := Seg (ProcedureIdentifierData^);
      TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
      Exit;
    end;
  If TypeDefPtr^.BaseType <> btFunctionProcedure then
    begin
      Location := elPointerToMemory;
      DataType := itLongInt;
      Exclude (LocationData.Flags, efReadOnly);
      If TypedPointers in StatementCompilerSwitches then
        begin
          PointerTypeDefinition := Ptr (DSeg, GetTempBufferOffset (SizeOf (TPointerTypeDefinition) + 2));
          UnitIdentifierData := Ptr (DSeg, Ofs (PointerTypeDefinition^) + SizeOf (TPointerTypeDefinition));
          With PointerTypeDefinition^ do
            begin
              BaseType := btPointer;
              DataType := [it32Bit, it16Bit, itSigned];
              Size := 4;
              W06_ := 0;
              PointerBaseTypeOffset.TypeOffset := Ofs (TypeDefPtr^);
              PointerBaseTypeOffset.UnitIdentifierData := Ofs (UnitIdentifierData^);
            end;
          UnitIdentifierData^.UnitSegment := Seg (TypeDefPtr^);
          TypeDefPtr := Pointer (PointerTypeDefinition);
        end else TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
    end else TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
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
Procedure TExpression.CreatePointerToVariableReference;
Var PointerTypeDefinition: PPointerTypeDefinition;
    UnitIdentifierData: PUnitIdentifierData;
    ProcedureIdentifierData: PProcedureIdentifierData;
begin
  If Location = elProcedure then
    begin
      ProcedureIdentifierData := PProcedureIdentifierData (PChar (TypeDefPtr) - 10);
      If ((pfMethod in ProcedureIdentifierData^.Flags) and not (mcStatic in DataType)) or
         (pfInline in ProcedureIdentifierData^.Flags) then Error (InvalidProcedureOrFunctionReference);
      Location := elPointerToMemory;
      DataType := [it32Bit, it16Bit, itSigned];
      UsedRegisters := [];
      LocationData.Flags := [efTypedConstant];
      Value.Offset := 0;
      Value.BlockRecord := ProcedureIdentifierData^.ProceduresRecordOffset;
      Value.Segment := Seg (ProcedureIdentifierData^);
      TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
      Exit;
    end;
  If TypeDefPtr^.BaseType <> btFunctionProcedure then
    begin
      Location := elPointerToMemory;
      DataType := itLongInt;
      Exclude (LocationData.Flags, efReadOnly);
      If TypedPointers in StatementCompilerSwitches then
        begin
          PointerTypeDefinition := Ptr (DSeg, GetTempBufferOffset (SizeOf (TPointerTypeDefinition) + 2));
          UnitIdentifierData := Ptr (DSeg, Ofs (PointerTypeDefinition^) + SizeOf (TPointerTypeDefinition));
          With PointerTypeDefinition^ do
            begin
              BaseType := btPointer;
              DataType := [it32Bit, it16Bit, itSigned];
              Size := 4;
              W06_ := 0;
              PointerBaseTypeOffset.TypeOffset := Ofs (TypeDefPtr^);
              PointerBaseTypeOffset.UnitIdentifierData := Ofs (UnitIdentifierData^);
            end;
          UnitIdentifierData^.UnitSegment := Seg (TypeDefPtr^);
          TypeDefPtr := Pointer (PointerTypeDefinition);
        end else TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
    end else TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
end;
```

### Code Block 2

```pascal
Procedure TExpression.CreatePointerToVariableReference;
Var PointerTypeDefinition: PPointerTypeDefinition;
    UnitIdentifierData: PUnitIdentifierData;
    ProcedureIdentifierData: PProcedureIdentifierData;
begin
  If Location = elProcedure then
    begin
      ProcedureIdentifierData := PProcedureIdentifierData (PChar (TypeDefPtr) - 10);
      If ((pfMethod in ProcedureIdentifierData^.Flags) and not (mcStatic in DataType)) or
         (pfInline in ProcedureIdentifierData^.Flags) then Error (InvalidProcedureOrFunctionReference);
      Location := elPointerToMemory;
      DataType := [it32Bit, it16Bit, itSigned];
      UsedRegisters := [];
      LocationData.Flags := [efTypedConstant];
      Value.Offset := 0;
      Value.BlockRecord := ProcedureIdentifierData^.ProceduresRecordOffset;
      Value.Segment := Seg (ProcedureIdentifierData^);
      TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
      Exit;
    end;
  If TypeDefPtr^.BaseType <> btFunctionProcedure then
    begin
      Location := elPointerToMemory;
      DataType := itLongInt;
      Exclude (LocationData.Flags, efReadOnly);
      If TypedPointers in StatementCompilerSwitches then
        begin
          PointerTypeDefinition := Ptr (DSeg, GetTempBufferOffset (SizeOf (TPointerTypeDefinition) + 2));
          UnitIdentifierData := Ptr (DSeg, Ofs (PointerTypeDefinition^) + SizeOf (TPointerTypeDefinition));
          With PointerTypeDefinition^ do
            begin
              BaseType := btPointer;
              DataType := [it32Bit, it16Bit, itSigned];
              Size := 4;
              W06_ := 0;
              PointerBaseTypeOffset.TypeOffset := Ofs (TypeDefPtr^);
              PointerBaseTypeOffset.UnitIdentifierData := Ofs (UnitIdentifierData^);
            end;
          UnitIdentifierData^.UnitSegment := Seg (TypeDefPtr^);
          TypeDefPtr := Pointer (PointerTypeDefinition);
        end else TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
    end else TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-pointer-to-variable-reference
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-qualifiers
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

