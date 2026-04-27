# System Procedures Inc and Dec - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-procedures-inc-and-dec

---

## Table of Contents

- System Procedures Inc and Dec

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Procedures Inc and Dec
This function processes system procedures
Inc
and
Dec
.
Function Proc_Inc_Dec: Word; Far;
Var VarExpression, ValueExpression: TExpression;
    ElementSize: Word;

  Procedure GenerateCodeFor_Inc_Dec;
  begin
    Case ValueExpression.Location of
      elConstant: With VarExpression do
                    begin
                      Calculate;
                      If not (it32Bit in DataType) and (ValueExpression.Value.LongInt = 1) then
                        VarExpression.GenerateInstruction_8_16_bit ($FE,
                                                                                                   ProcParameter and $08) else
                          begin
                            GenerateArithmeticInstructionWithImmediateValue (ValueExpression.Value.Word, ProcParameter);
                            If it32Bit in DataType then
                              begin
                                Inc (VarExpression.Value.Offset, 2);
                                GenerateArithmeticInstructionWithImmediateValue (ValueExpression.Value.W12,
                                                                             ProcParameter and $08 or $10);
                                Dec (VarExpression.Value.Offset, 2);
                              end;
                          end;
                      EndIntermediateCodeSubroutine;
                    end;
      else begin
             ValueExpression.Calculate;
             ValueExpression.Save (VarExpression.UsedRegisters);
             VarExpression.Calculate;
             ValueExpression.PopToRegisters ([]);
             VarExpression.GenerateInstruction_8_16_bit (ProcParameter,
                                                                 ValueExpression.LocationData.Register shl 3);
             If it32Bit in VarExpression.DataType then
               begin
                 Inc (VarExpression.Value.Offset, 2);
                 VarExpression.GenerateInstruction_8_16_bit (ProcParameter and $08 or $10,
                                                                                              $10);
                 Dec (VarExpression.Value.Offset, 2);
               end;
             VarExpression.EndIntermediateCodeSubroutine;
           end;
    end;
  end;

begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  VarExpression.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  ElementSize := 1;
  Case VarExpression.TypeDefPtr^.BaseType of
    btPointer: begin
                 ElementSize := PTypeDefinition (PointerFromOffsets (
                                  PPointerTypeDefinition (VarExpression.TypeDefPtr)^.PointerBaseTypeOffset))^.Size;
                 If ElementSize = 0 then Error (OrdinalVariableExpected);
                 VarExpression.DataType := itWord;
               end;
    btInteger,
    btBoolean,
    btChar,
    btEnumeration:
    else Error (OrdinalVariableExpected);
  end;
  If CheckAndGetNextToken (Token_Comma) then
    begin
      ValueExpression.ExpectIntegerExpression;
      ValueExpression.LoadPointerToMemoryTo_DX_AX;
      ValueExpression.ExtendInteger (VarExpression.DataType);
      If ElementSize <> 1 then
        With ValueExpression do
          Case Location of
            elConstant: Value.Word := Value.Word * ElementSize;
            else begin
                   Calculate;
                   ValueExpression.IntegerMultiplicationWithConstant (rAX, ElementSize);
                   EndIntermediateCodeSubroutine;
                 end;
          end;
    end else With ValueExpression do
               begin
                 IntermediateCodeOffset := 0;
                 Location := elConstant;
                 Value.LongInt := ElementSize;
               end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  GenerateCodeFor_Inc_Dec;
  VarExpression.CheckRange (VarExpression.TypeDefPtr);
  Proc_Inc_Dec := VarExpression.IntermediateCodeOffset;
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
Function Proc_Inc_Dec: Word; Far;
Var VarExpression, ValueExpression: TExpression;
    ElementSize: Word;

  Procedure GenerateCodeFor_Inc_Dec;
  begin
    Case ValueExpression.Location of
      elConstant: With VarExpression do
                    begin
                      Calculate;
                      If not (it32Bit in DataType) and (ValueExpression.Value.LongInt = 1) then
                        VarExpression.GenerateInstruction_8_16_bit ($FE,
                                                                                                   ProcParameter and $08) else
                          begin
                            GenerateArithmeticInstructionWithImmediateValue (ValueExpression.Value.Word, ProcParameter);
                            If it32Bit in DataType then
                              begin
                                Inc (VarExpression.Value.Offset, 2);
                                GenerateArithmeticInstructionWithImmediateValue (ValueExpression.Value.W12,
                                                                             ProcParameter and $08 or $10);
                                Dec (VarExpression.Value.Offset, 2);
                              end;
                          end;
                      EndIntermediateCodeSubroutine;
                    end;
      else begin
             ValueExpression.Calculate;
             ValueExpression.Save (VarExpression.UsedRegisters);
             VarExpression.Calculate;
             ValueExpression.PopToRegisters ([]);
             VarExpression.GenerateInstruction_8_16_bit (ProcParameter,
                                                                 ValueExpression.LocationData.Register shl 3);
             If it32Bit in VarExpression.DataType then
               begin
                 Inc (VarExpression.Value.Offset, 2);
                 VarExpression.GenerateInstruction_8_16_bit (ProcParameter and $08 or $10,
                                                                                              $10);
                 Dec (VarExpression.Value.Offset, 2);
               end;
             VarExpression.EndIntermediateCodeSubroutine;
           end;
    end;
  end;

begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  VarExpression.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  ElementSize := 1;
  Case VarExpression.TypeDefPtr^.BaseType of
    btPointer: begin
                 ElementSize := PTypeDefinition (PointerFromOffsets (
                                  PPointerTypeDefinition (VarExpression.TypeDefPtr)^.PointerBaseTypeOffset))^.Size;
                 If ElementSize = 0 then Error (OrdinalVariableExpected);
                 VarExpression.DataType := itWord;
               end;
    btInteger,
    btBoolean,
    btChar,
    btEnumeration:
    else Error (OrdinalVariableExpected);
  end;
  If CheckAndGetNextToken (Token_Comma) then
    begin
      ValueExpression.ExpectIntegerExpression;
      ValueExpression.LoadPointerToMemoryTo_DX_AX;
      ValueExpression.ExtendInteger (VarExpression.DataType);
      If ElementSize <> 1 then
        With ValueExpression do
          Case Location of
            elConstant: Value.Word := Value.Word * ElementSize;
            else begin
                   Calculate;
                   ValueExpression.IntegerMultiplicationWithConstant (rAX, ElementSize);
                   EndIntermediateCodeSubroutine;
                 end;
          end;
    end else With ValueExpression do
               begin
                 IntermediateCodeOffset := 0;
                 Location := elConstant;
                 Value.LongInt := ElementSize;
               end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  GenerateCodeFor_Inc_Dec;
  VarExpression.CheckRange (VarExpression.TypeDefPtr);
  Proc_Inc_Dec := VarExpression.IntermediateCodeOffset;
end;
```

### Code Block 2

```pascal
Function Proc_Inc_Dec: Word; Far;
Var VarExpression, ValueExpression: TExpression;
    ElementSize: Word;

  Procedure GenerateCodeFor_Inc_Dec;
  begin
    Case ValueExpression.Location of
      elConstant: With VarExpression do
                    begin
                      Calculate;
                      If not (it32Bit in DataType) and (ValueExpression.Value.LongInt = 1) then
                        VarExpression.GenerateInstruction_8_16_bit ($FE,
                                                                                                   ProcParameter and $08) else
                          begin
                            GenerateArithmeticInstructionWithImmediateValue (ValueExpression.Value.Word, ProcParameter);
                            If it32Bit in DataType then
                              begin
                                Inc (VarExpression.Value.Offset, 2);
                                GenerateArithmeticInstructionWithImmediateValue (ValueExpression.Value.W12,
                                                                             ProcParameter and $08 or $10);
                                Dec (VarExpression.Value.Offset, 2);
                              end;
                          end;
                      EndIntermediateCodeSubroutine;
                    end;
      else begin
             ValueExpression.Calculate;
             ValueExpression.Save (VarExpression.UsedRegisters);
             VarExpression.Calculate;
             ValueExpression.PopToRegisters ([]);
             VarExpression.GenerateInstruction_8_16_bit (ProcParameter,
                                                                 ValueExpression.LocationData.Register shl 3);
             If it32Bit in VarExpression.DataType then
               begin
                 Inc (VarExpression.Value.Offset, 2);
                 VarExpression.GenerateInstruction_8_16_bit (ProcParameter and $08 or $10,
                                                                                              $10);
                 Dec (VarExpression.Value.Offset, 2);
               end;
             VarExpression.EndIntermediateCodeSubroutine;
           end;
    end;
  end;

begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  VarExpression.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  ElementSize := 1;
  Case VarExpression.TypeDefPtr^.BaseType of
    btPointer: begin
                 ElementSize := PTypeDefinition (PointerFromOffsets (
                                  PPointerTypeDefinition (VarExpression.TypeDefPtr)^.PointerBaseTypeOffset))^.Size;
                 If ElementSize = 0 then Error (OrdinalVariableExpected);
                 VarExpression.DataType := itWord;
               end;
    btInteger,
    btBoolean,
    btChar,
    btEnumeration:
    else Error (OrdinalVariableExpected);
  end;
  If CheckAndGetNextToken (Token_Comma) then
    begin
      ValueExpression.ExpectIntegerExpression;
      ValueExpression.LoadPointerToMemoryTo_DX_AX;
      ValueExpression.ExtendInteger (VarExpression.DataType);
      If ElementSize <> 1 then
        With ValueExpression do
          Case Location of
            elConstant: Value.Word := Value.Word * ElementSize;
            else begin
                   Calculate;
                   ValueExpression.IntegerMultiplicationWithConstant (rAX, ElementSize);
                   EndIntermediateCodeSubroutine;
                 end;
          end;
    end else With ValueExpression do
               begin
                 IntermediateCodeOffset := 0;
                 Location := elConstant;
                 Value.LongInt := ElementSize;
               end;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  GenerateCodeFor_Inc_Dec;
  VarExpression.CheckRange (VarExpression.TypeDefPtr);
  Proc_Inc_Dec := VarExpression.IntermediateCodeOffset;
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
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/system-functions
- https://turbopascal.org/system-procedures
- https://turbopascal.org/system-procedures-inc-and-dec
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

