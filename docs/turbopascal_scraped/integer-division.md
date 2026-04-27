# Integer Division - Turbo Pascal Compiler

**URL:** https://turbopascal.org/integer-division

---

## Table of Contents

- Integer Division

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Integer Division
This procedure generates code for integer division with constant. Special code is generated if divisor is power of 2.
Procedure IntegerConstantDivision (Divisor: Word);
    Var ShiftCount: Byte;
        TempDivisor: Word;
        DivisionOpCode: Byte;
    begin
      If Divisor = 1 then Exit;
      If not (itSigned in RightExpression.DataType) then
        begin
          ShiftCount := 0;
          TempDivisor := Divisor;
          While (TempDivisor and $01) = 0 do
            begin
              Inc (ShiftCount);
              TempDivisor := TempDivisor shr 1;
            end;
          If TempDivisor  = 1 then
            begin
              If ShiftCount = 1 then
                begin
                  GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHR_AX);
                  Exit;
                end;
              If Instructions80286 in StatementCompilerSwitches then
                begin
                  GenerateInstruction_TwoBytes (ShiftRotateOperationsByValue_16Bit, SHR_AX);
                  GenerateInstruction_Byte (ShiftCount);
                  Exit;
                end;
              If ShiftCount  = 2 then
                begin
                  GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHR_AX);
                  GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHR_AX);
                  Exit;
                end;
              Include (RightExpression.UsedRegisters, urDX);
              GenerateInstruction_TwoBytes (MOV_CL_Immediate, ShiftCount);
              GenerateInstruction_TwoBytes (ShiftRotateOperationsByCL_16Bit, SHR_AX);
              Exit;
            end;
          Case itUnsigned in RightExpression.DataType of
            True: begin
                    GenerateInstruction_Word (XOR_DX_DX);
                    DivisionOpCode := DIV_CX;
                  end;

            else begin
                   GenerateInstruction_Byte (CWD);
                   DivisionOpCode := IDIV_CX;
                 end;
          end;
        end else begin
                   GenerateInstruction_Byte (CWD);
                   DivisionOpCode := IDIV_CX;
                 end;
      GenerateInstruction_Byte (MOV_CX_Immediate);
      GenerateInstruction_Word (Divisor);
      GenerateInstruction_TwoBytes (Unary_16Bit_Operations, DivisionOpCode);
      RightExpression.UsedRegisters := RightExpression.UsedRegisters + [urDX, urCX];
    end;
This procedure generates general code for integer division.
Procedure GeneralIntegerDivision;
    Var OpCode: WordRec;
    begin
      OpCode.Word := IntegerOpCodes [Operation].Word;
      If itUnsigned in RightExpression.DataType then
        begin
          GenerateInstruction_Word (XOR_DX_DX);
          OpCode.ByteH := DIV_CX;
        end else GenerateInstruction_Byte (CWD);
      GenerateCodeForDivisionOrMultiplication (OpCode.ByteL, OpCode.ByteH);
    end;

    Procedure GenerateCodeForDivisionOrMultiplication (OpCode0, OpCode1: Byte);
    begin
      If RightExpression.Location = elConstant then
        RightExpression.LoadExpressionToRegisters (urCX);
      RightExpression.GenerateInstructionWithExpressionInMemOrReg (OpCode0, OpCode1);
      Include (RightExpression.UsedRegisters, urDX);
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
    Procedure IntegerConstantDivision (Divisor: Word);
    Var ShiftCount: Byte;
        TempDivisor: Word;
        DivisionOpCode: Byte;
    begin
      If Divisor = 1 then Exit;
      If not (itSigned in RightExpression.DataType) then
        begin
          ShiftCount := 0;
          TempDivisor := Divisor;
          While (TempDivisor and $01) = 0 do
            begin
              Inc (ShiftCount);
              TempDivisor := TempDivisor shr 1;
            end;
          If TempDivisor  = 1 then
            begin
              If ShiftCount = 1 then
                begin
                  GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHR_AX);
                  Exit;
                end;
              If Instructions80286 in StatementCompilerSwitches then
                begin
                  GenerateInstruction_TwoBytes (ShiftRotateOperationsByValue_16Bit, SHR_AX);
                  GenerateInstruction_Byte (ShiftCount);
                  Exit;
                end;
              If ShiftCount  = 2 then
                begin
                  GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHR_AX);
                  GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHR_AX);
                  Exit;
                end;
              Include (RightExpression.UsedRegisters, urDX);
              GenerateInstruction_TwoBytes (MOV_CL_Immediate, ShiftCount);
              GenerateInstruction_TwoBytes (ShiftRotateOperationsByCL_16Bit, SHR_AX);
              Exit;
            end;
          Case itUnsigned in RightExpression.DataType of
            True: begin
                    GenerateInstruction_Word (XOR_DX_DX);
                    DivisionOpCode := DIV_CX;
                  end;

            else begin
                   GenerateInstruction_Byte (CWD);
                   DivisionOpCode := IDIV_CX;
                 end;
          end;
        end else begin
                   GenerateInstruction_Byte (CWD);
                   DivisionOpCode := IDIV_CX;
                 end;
      GenerateInstruction_Byte (MOV_CX_Immediate);
      GenerateInstruction_Word (Divisor);
      GenerateInstruction_TwoBytes (Unary_16Bit_Operations, DivisionOpCode);
      RightExpression.UsedRegisters := RightExpression.UsedRegisters + [urDX, urCX];
    end;
```

### Code Block 2

```pascal
    Procedure GeneralIntegerDivision;
    Var OpCode: WordRec;
    begin
      OpCode.Word := IntegerOpCodes [Operation].Word;
      If itUnsigned in RightExpression.DataType then
        begin
          GenerateInstruction_Word (XOR_DX_DX);
          OpCode.ByteH := DIV_CX;
        end else GenerateInstruction_Byte (CWD);
      GenerateCodeForDivisionOrMultiplication (OpCode.ByteL, OpCode.ByteH);
    end;

    Procedure GenerateCodeForDivisionOrMultiplication (OpCode0, OpCode1: Byte);
    begin
      If RightExpression.Location = elConstant then
        RightExpression.LoadExpressionToRegisters (urCX);
      RightExpression.GenerateInstructionWithExpressionInMemOrReg (OpCode0, OpCode1);
      Include (RightExpression.UsedRegisters, urDX);
    end;
```

### Code Block 3

```pascal
    Procedure IntegerConstantDivision (Divisor: Word);
    Var ShiftCount: Byte;
        TempDivisor: Word;
        DivisionOpCode: Byte;
    begin
      If Divisor = 1 then Exit;
      If not (itSigned in RightExpression.DataType) then
        begin
          ShiftCount := 0;
          TempDivisor := Divisor;
          While (TempDivisor and $01) = 0 do
            begin
              Inc (ShiftCount);
              TempDivisor := TempDivisor shr 1;
            end;
          If TempDivisor  = 1 then
            begin
              If ShiftCount = 1 then
                begin
                  GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHR_AX);
                  Exit;
                end;
              If Instructions80286 in StatementCompilerSwitches then
                begin
                  GenerateInstruction_TwoBytes (ShiftRotateOperationsByValue_16Bit, SHR_AX);
                  GenerateInstruction_Byte (ShiftCount);
                  Exit;
                end;
              If ShiftCount  = 2 then
                begin
                  GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHR_AX);
                  GenerateInstruction_TwoBytes (ShiftRotateOperationsBy1_16Bit, SHR_AX);
                  Exit;
                end;
              Include (RightExpression.UsedRegisters, urDX);
              GenerateInstruction_TwoBytes (MOV_CL_Immediate, ShiftCount);
              GenerateInstruction_TwoBytes (ShiftRotateOperationsByCL_16Bit, SHR_AX);
              Exit;
            end;
          Case itUnsigned in RightExpression.DataType of
            True: begin
                    GenerateInstruction_Word (XOR_DX_DX);
                    DivisionOpCode := DIV_CX;
                  end;

            else begin
                   GenerateInstruction_Byte (CWD);
                   DivisionOpCode := IDIV_CX;
                 end;
          end;
        end else begin
                   GenerateInstruction_Byte (CWD);
                   DivisionOpCode := IDIV_CX;
                 end;
      GenerateInstruction_Byte (MOV_CX_Immediate);
      GenerateInstruction_Word (Divisor);
      GenerateInstruction_TwoBytes (Unary_16Bit_Operations, DivisionOpCode);
      RightExpression.UsedRegisters := RightExpression.UsedRegisters + [urDX, urCX];
    end;
```

### Code Block 4

```pascal
    Procedure GeneralIntegerDivision;
    Var OpCode: WordRec;
    begin
      OpCode.Word := IntegerOpCodes [Operation].Word;
      If itUnsigned in RightExpression.DataType then
        begin
          GenerateInstruction_Word (XOR_DX_DX);
          OpCode.ByteH := DIV_CX;
        end else GenerateInstruction_Byte (CWD);
      GenerateCodeForDivisionOrMultiplication (OpCode.ByteL, OpCode.ByteH);
    end;

    Procedure GenerateCodeForDivisionOrMultiplication (OpCode0, OpCode1: Byte);
    begin
      If RightExpression.Location = elConstant then
        RightExpression.LoadExpressionToRegisters (urCX);
      RightExpression.GenerateInstructionWithExpressionInMemOrReg (OpCode0, OpCode1);
      Include (RightExpression.UsedRegisters, urDX);
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
- https://turbopascal.org/integer-division
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

