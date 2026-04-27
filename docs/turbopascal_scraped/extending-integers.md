# Extending Integers - Turbo Pascal Compiler

**URL:** https://turbopascal.org/extending-integers

---

## Table of Contents

- Extending Integers

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Extending Integers
This procedure extends integers to the new integer type.
Procedure TExpression.ExtendInteger (NewIntegerType: TIntegerTypeSet);
Var upCode: LongRec;
    TempValue: LongRec absolute upCode;
begin
  TempValue := Value.LongRec;
  Case Location of
    elConstant: begin
                  If not (it16Bit in NewIntegerType) then
                    begin
                      Integer (TempValue.WordL) := ShortInt (Lo (TempValue.WordL));
                      If itUnsigned in NewIntegerType then TempValue.Byte1 := 0;
                    end;
                  If not (it32Bit in NewIntegerType)then
                    begin
                      TempValue.Long := Integer (TempValue.WordL);
                      If itUnsigned in NewIntegerType then TempValue.WordH := 0;
                    end;
                  Value.LongRec := TempValue;
                end;
    else begin
           upCode.Long := 0;
           If (it16Bit in NewIntegerType) and not (it16Bit in DataType) then
             begin
               upCode.Byte0 := CBW;
               If itUnsigned in DataType then upCode.WordL := XOR_AH_AH;
             end;
           If (it32Bit in NewIntegerType) and not (it32Bit in DataType) then
             begin
               Include (UsedRegisters, urDX);
               upCode.Byte2 := CWD;
               If itUnsigned in DataType then upCode.WordH := XOR_DX_DX;
             end;
           If (upCode.Long <> 0) or ((Location = elPointerToMemory) and not (it16Bit in NewIntegerType)) then
             begin
               Calculate;
               LoadExpressionToRegisters (urAX);
               If upCode.WordL <> 0 then
                 begin
                   If upCode.Byte1 = 0 then GenerateInstruction_Byte (upCode.Byte0)
                     else GenerateInstruction_TwoBytes (upCode.Byte0, upCode.Byte1);
                 end;
               If upCode.WordH <> 0 then
                 begin
                   If upCode.Byte3 = 0 then GenerateInstruction_Byte (upCode.Byte2)
                     else GenerateInstruction_TwoBytes (upCode.Byte2, upCode.Byte3);
                 end;
               EndIntermediateCodeSubroutine;
             end;
         end;
  end;
  DataType := NewIntegerType;
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
Procedure TExpression.ExtendInteger (NewIntegerType: TIntegerTypeSet);
Var upCode: LongRec;
    TempValue: LongRec absolute upCode;
begin
  TempValue := Value.LongRec;
  Case Location of
    elConstant: begin
                  If not (it16Bit in NewIntegerType) then
                    begin
                      Integer (TempValue.WordL) := ShortInt (Lo (TempValue.WordL));
                      If itUnsigned in NewIntegerType then TempValue.Byte1 := 0;
                    end;
                  If not (it32Bit in NewIntegerType)then
                    begin
                      TempValue.Long := Integer (TempValue.WordL);
                      If itUnsigned in NewIntegerType then TempValue.WordH := 0;
                    end;
                  Value.LongRec := TempValue;
                end;
    else begin
           upCode.Long := 0;
           If (it16Bit in NewIntegerType) and not (it16Bit in DataType) then
             begin
               upCode.Byte0 := CBW;
               If itUnsigned in DataType then upCode.WordL := XOR_AH_AH;
             end;
           If (it32Bit in NewIntegerType) and not (it32Bit in DataType) then
             begin
               Include (UsedRegisters, urDX);
               upCode.Byte2 := CWD;
               If itUnsigned in DataType then upCode.WordH := XOR_DX_DX;
             end;
           If (upCode.Long <> 0) or ((Location = elPointerToMemory) and not (it16Bit in NewIntegerType)) then
             begin
               Calculate;
               LoadExpressionToRegisters (urAX);
               If upCode.WordL <> 0 then
                 begin
                   If upCode.Byte1 = 0 then GenerateInstruction_Byte (upCode.Byte0)
                     else GenerateInstruction_TwoBytes (upCode.Byte0, upCode.Byte1);
                 end;
               If upCode.WordH <> 0 then
                 begin
                   If upCode.Byte3 = 0 then GenerateInstruction_Byte (upCode.Byte2)
                     else GenerateInstruction_TwoBytes (upCode.Byte2, upCode.Byte3);
                 end;
               EndIntermediateCodeSubroutine;
             end;
         end;
  end;
  DataType := NewIntegerType;
end;
```

### Code Block 2

```pascal
Procedure TExpression.ExtendInteger (NewIntegerType: TIntegerTypeSet);
Var upCode: LongRec;
    TempValue: LongRec absolute upCode;
begin
  TempValue := Value.LongRec;
  Case Location of
    elConstant: begin
                  If not (it16Bit in NewIntegerType) then
                    begin
                      Integer (TempValue.WordL) := ShortInt (Lo (TempValue.WordL));
                      If itUnsigned in NewIntegerType then TempValue.Byte1 := 0;
                    end;
                  If not (it32Bit in NewIntegerType)then
                    begin
                      TempValue.Long := Integer (TempValue.WordL);
                      If itUnsigned in NewIntegerType then TempValue.WordH := 0;
                    end;
                  Value.LongRec := TempValue;
                end;
    else begin
           upCode.Long := 0;
           If (it16Bit in NewIntegerType) and not (it16Bit in DataType) then
             begin
               upCode.Byte0 := CBW;
               If itUnsigned in DataType then upCode.WordL := XOR_AH_AH;
             end;
           If (it32Bit in NewIntegerType) and not (it32Bit in DataType) then
             begin
               Include (UsedRegisters, urDX);
               upCode.Byte2 := CWD;
               If itUnsigned in DataType then upCode.WordH := XOR_DX_DX;
             end;
           If (upCode.Long <> 0) or ((Location = elPointerToMemory) and not (it16Bit in NewIntegerType)) then
             begin
               Calculate;
               LoadExpressionToRegisters (urAX);
               If upCode.WordL <> 0 then
                 begin
                   If upCode.Byte1 = 0 then GenerateInstruction_Byte (upCode.Byte0)
                     else GenerateInstruction_TwoBytes (upCode.Byte0, upCode.Byte1);
                 end;
               If upCode.WordH <> 0 then
                 begin
                   If upCode.Byte3 = 0 then GenerateInstruction_Byte (upCode.Byte2)
                     else GenerateInstruction_TwoBytes (upCode.Byte2, upCode.Byte3);
                 end;
               EndIntermediateCodeSubroutine;
             end;
         end;
  end;
  DataType := NewIntegerType;
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
- https://turbopascal.org/extending-integers
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

