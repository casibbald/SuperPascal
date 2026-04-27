# Converting Number Types - Turbo Pascal Compiler

**URL:** https://turbopascal.org/converting-number-types

---

## Table of Contents

- Converting Number Types

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Converting Number Types
Turbo Pascal sometimes needs to convert number types.
Integer
and
Real
can be used as
Extended
and
Integer
or
Extended
can be used as
Real
.
Procedure TExpression.ConvertIntegerToExtended;
Var OpCode: Byte;
begin
  If TypeDefPtr^.BaseType <> btInteger then Exit;
  Case Location of
    elConstant: begin
                  Value.Extended := Value.LongInt;
                  DataType := [fpExtended];
                  TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
                  Exit;
                end;
    elMemory: If it32Bit in DataType then OpCode := FILD_32Bit else
                If DataType = [it16Bit] then OpCode := FILD_16Bit else
                  begin
                    ExtendInteger ([it32Bit, it16Bit, itSigned]);
                    StoreIntegerToStackFrame;
                    OpCode := FILD_32Bit;
                  end;
    else begin
           ExtendInteger ([it32Bit, it16Bit, itSigned]);
           StoreIntegerToStackFrame;
           OpCode := FILD_32Bit;
         end;
  end;
  Calculate;
  GenerateFPUInstructionWithExpressionInMemory (OpCode, $00);
  EndIntermediateCodeSubroutine;
  Location := elStackFrame;
  DataType := [fpExtended];
  TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
end;

Procedure TExpression.ConvertRealToExtended;
begin
  If TypeDefPtr^.BaseType = btReal then
    begin
      Calculate;
      LoadRealExpressionToRegisterSet (rAX_BX_DX);
      GenerateInstruction_CALL_FAR (SysProc_FRealExt);
      EndIntermediateCodeSubroutine;
      Location := elStackFrame;
      DataType := [itUnsigned];
      UsedRegisters := [urBX, urDX, urCX, urAX];
      TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
    end;
end;

Procedure TExpression.ConvertIntegerToReal;
begin
  If TypeDefPtr^.BaseType = btInteger then
    begin
      If Location = elConstant then Value.Real := Value.LongInt else
        begin
          ExtendInteger ([it32Bit, it16Bit, itSigned]);
          Calculate;
          LoadExpressionToRegisters (urDX_AX);
          GenerateInstruction_CALL_FAR (SysProc_RFloat);
          EndIntermediateCodeSubroutine;
          Location := elRegister;
          UsedRegisters := [urBX, urDX, urCX, urAX];
        end;
      DataType := [it16Bit];
      TypeDefPtr := Ptr (SystemUnitSegment, Real_TypeOffset);
    end;
end;

Procedure TExpression.ConvertExtendedToReal;
begin
  If TypeDefPtr^.BaseType = btExtended then
    begin
      Case Location of
        elConstant: begin
                      If Value.Extended > 1.701E38 then Error (ConstantOutOfRange);
                      Value.Real := Value.Extended;
                    end;
      else begin
             LoadExpressionToFPU;
             Calculate;
             GenerateInstruction_CALL_FAR (SysProc_FExtReal);
             EndIntermediateCodeSubroutine;
             Location := elRegister;
             UsedRegisters := [urBX, urDX, urCX, urAX];
           end;
      end;
      DataType := [it16Bit]; { fpDouble ? }
      TypeDefPtr := Ptr (SystemUnitSegment, Real_TypeOffset);
    end;
end;

Procedure ConvertExtendedToOtherFloatingPointTypes (FloatingTypeSet: TFloatingTypeSet; Var Value: TValue);
begin
  Case Byte (FloatingTypeSet) of
    fptExtended: Exit;
    fptSingle: begin
                 Value.Single := Value.Extended;
                 If Value.W12 and $7F80 = $7F80 then Error (ConstantOutOfRange);
               end;
    fptDouble: begin
                 Value.Double := Value.Extended;
                 If Value.W16 and $7FF0 = $7FF0 then Error (ConstantOutOfRange);
               end;
    else begin
          Value.Comp := Value.Extended;
          If (Value.W16 = $8000) and (Value.Word or Value.W12 or Value.W14 = 0) then
            Error (ConstantOutOfRange);
         end;
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
Procedure TExpression.ConvertIntegerToExtended;
Var OpCode: Byte;
begin
  If TypeDefPtr^.BaseType <> btInteger then Exit;
  Case Location of
    elConstant: begin
                  Value.Extended := Value.LongInt;
                  DataType := [fpExtended];
                  TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
                  Exit;
                end;
    elMemory: If it32Bit in DataType then OpCode := FILD_32Bit else
                If DataType = [it16Bit] then OpCode := FILD_16Bit else
                  begin
                    ExtendInteger ([it32Bit, it16Bit, itSigned]);
                    StoreIntegerToStackFrame;
                    OpCode := FILD_32Bit;
                  end;
    else begin
           ExtendInteger ([it32Bit, it16Bit, itSigned]);
           StoreIntegerToStackFrame;
           OpCode := FILD_32Bit;
         end;
  end;
  Calculate;
  GenerateFPUInstructionWithExpressionInMemory (OpCode, $00);
  EndIntermediateCodeSubroutine;
  Location := elStackFrame;
  DataType := [fpExtended];
  TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
end;

Procedure TExpression.ConvertRealToExtended;
begin
  If TypeDefPtr^.BaseType = btReal then
    begin
      Calculate;
      LoadRealExpressionToRegisterSet (rAX_BX_DX);
      GenerateInstruction_CALL_FAR (SysProc_FRealExt);
      EndIntermediateCodeSubroutine;
      Location := elStackFrame;
      DataType := [itUnsigned];
      UsedRegisters := [urBX, urDX, urCX, urAX];
      TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
    end;
end;

Procedure TExpression.ConvertIntegerToReal;
begin
  If TypeDefPtr^.BaseType = btInteger then
    begin
      If Location = elConstant then Value.Real := Value.LongInt else
        begin
          ExtendInteger ([it32Bit, it16Bit, itSigned]);
          Calculate;
          LoadExpressionToRegisters (urDX_AX);
          GenerateInstruction_CALL_FAR (SysProc_RFloat);
          EndIntermediateCodeSubroutine;
          Location := elRegister;
          UsedRegisters := [urBX, urDX, urCX, urAX];
        end;
      DataType := [it16Bit];
      TypeDefPtr := Ptr (SystemUnitSegment, Real_TypeOffset);
    end;
end;

Procedure TExpression.ConvertExtendedToReal;
begin
  If TypeDefPtr^.BaseType = btExtended then
    begin
      Case Location of
        elConstant: begin
                      If Value.Extended > 1.701E38 then Error (ConstantOutOfRange);
                      Value.Real := Value.Extended;
                    end;
      else begin
             LoadExpressionToFPU;
             Calculate;
             GenerateInstruction_CALL_FAR (SysProc_FExtReal);
             EndIntermediateCodeSubroutine;
             Location := elRegister;
             UsedRegisters := [urBX, urDX, urCX, urAX];
           end;
      end;
      DataType := [it16Bit]; { fpDouble ? }
      TypeDefPtr := Ptr (SystemUnitSegment, Real_TypeOffset);
    end;
end;

Procedure ConvertExtendedToOtherFloatingPointTypes (FloatingTypeSet: TFloatingTypeSet; Var Value: TValue);
begin
  Case Byte (FloatingTypeSet) of
    fptExtended: Exit;
    fptSingle: begin
                 Value.Single := Value.Extended;
                 If Value.W12 and $7F80 = $7F80 then Error (ConstantOutOfRange);
               end;
    fptDouble: begin
                 Value.Double := Value.Extended;
                 If Value.W16 and $7FF0 = $7FF0 then Error (ConstantOutOfRange);
               end;
    else begin
          Value.Comp := Value.Extended;
          If (Value.W16 = $8000) and (Value.Word or Value.W12 or Value.W14 = 0) then
            Error (ConstantOutOfRange);
         end;
  end;
end;
```

### Code Block 2

```pascal
Procedure TExpression.ConvertIntegerToExtended;
Var OpCode: Byte;
begin
  If TypeDefPtr^.BaseType <> btInteger then Exit;
  Case Location of
    elConstant: begin
                  Value.Extended := Value.LongInt;
                  DataType := [fpExtended];
                  TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
                  Exit;
                end;
    elMemory: If it32Bit in DataType then OpCode := FILD_32Bit else
                If DataType = [it16Bit] then OpCode := FILD_16Bit else
                  begin
                    ExtendInteger ([it32Bit, it16Bit, itSigned]);
                    StoreIntegerToStackFrame;
                    OpCode := FILD_32Bit;
                  end;
    else begin
           ExtendInteger ([it32Bit, it16Bit, itSigned]);
           StoreIntegerToStackFrame;
           OpCode := FILD_32Bit;
         end;
  end;
  Calculate;
  GenerateFPUInstructionWithExpressionInMemory (OpCode, $00);
  EndIntermediateCodeSubroutine;
  Location := elStackFrame;
  DataType := [fpExtended];
  TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
end;

Procedure TExpression.ConvertRealToExtended;
begin
  If TypeDefPtr^.BaseType = btReal then
    begin
      Calculate;
      LoadRealExpressionToRegisterSet (rAX_BX_DX);
      GenerateInstruction_CALL_FAR (SysProc_FRealExt);
      EndIntermediateCodeSubroutine;
      Location := elStackFrame;
      DataType := [itUnsigned];
      UsedRegisters := [urBX, urDX, urCX, urAX];
      TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
    end;
end;

Procedure TExpression.ConvertIntegerToReal;
begin
  If TypeDefPtr^.BaseType = btInteger then
    begin
      If Location = elConstant then Value.Real := Value.LongInt else
        begin
          ExtendInteger ([it32Bit, it16Bit, itSigned]);
          Calculate;
          LoadExpressionToRegisters (urDX_AX);
          GenerateInstruction_CALL_FAR (SysProc_RFloat);
          EndIntermediateCodeSubroutine;
          Location := elRegister;
          UsedRegisters := [urBX, urDX, urCX, urAX];
        end;
      DataType := [it16Bit];
      TypeDefPtr := Ptr (SystemUnitSegment, Real_TypeOffset);
    end;
end;

Procedure TExpression.ConvertExtendedToReal;
begin
  If TypeDefPtr^.BaseType = btExtended then
    begin
      Case Location of
        elConstant: begin
                      If Value.Extended > 1.701E38 then Error (ConstantOutOfRange);
                      Value.Real := Value.Extended;
                    end;
      else begin
             LoadExpressionToFPU;
             Calculate;
             GenerateInstruction_CALL_FAR (SysProc_FExtReal);
             EndIntermediateCodeSubroutine;
             Location := elRegister;
             UsedRegisters := [urBX, urDX, urCX, urAX];
           end;
      end;
      DataType := [it16Bit]; { fpDouble ? }
      TypeDefPtr := Ptr (SystemUnitSegment, Real_TypeOffset);
    end;
end;

Procedure ConvertExtendedToOtherFloatingPointTypes (FloatingTypeSet: TFloatingTypeSet; Var Value: TValue);
begin
  Case Byte (FloatingTypeSet) of
    fptExtended: Exit;
    fptSingle: begin
                 Value.Single := Value.Extended;
                 If Value.W12 and $7F80 = $7F80 then Error (ConstantOutOfRange);
               end;
    fptDouble: begin
                 Value.Double := Value.Extended;
                 If Value.W16 and $7FF0 = $7FF0 then Error (ConstantOutOfRange);
               end;
    else begin
          Value.Comp := Value.Extended;
          If (Value.W16 = $8000) and (Value.Word or Value.W12 or Value.W14 = 0) then
            Error (ConstantOutOfRange);
         end;
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
- https://turbopascal.org/converting-number-types
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

