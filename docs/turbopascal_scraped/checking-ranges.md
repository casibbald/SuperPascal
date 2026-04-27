# Checking Ranges - Turbo Pascal Compiler

**URL:** https://turbopascal.org/checking-ranges

---

## Table of Contents

- Checking Ranges

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Checking Ranges
Turbo Pascal checks if values are within limits. If the value is constant it is immediately checked for lower and upper limit. If it is not constant and range checking is enabled, Turbo Pascal will generate range checking code. Botl limits will be stored in code and compiler procedure will be called to check range in run-time.
Procedure TExpression.CheckRange (TypeDef: PTypeDefinition);
begin
  Case TypeDef^.BaseType of
    btInteger..btEnumeration: CheckOrdinalRange (TypeDef);
    btSet: Case it32Bytes in TypeDef^.DataType of
             False: LoadSetAndPopPointer (TypeDef^.DataType);
             True:  ExpandSetToStackFrameAndPushPointer (32);
           end;
  end;
end;

Procedure TExpression.CheckOrdinalRange (TypePtr: PTypeDefinition);
Var TempInt: LongRec;
    UpperLimit, LowerLimit: LongInt;
    ConstantOffset, ProgramCodeBlock: Word;
    PointerToLimits: Pointer;
begin
  If Location = elConstant then
    begin
      If not IsConstantInRange (Value.LongRec, TypePtr) then Error (ConstantOutOfRange);
      DataType := TypePtr^.DataType + LowestDataType;
      Exit;
    end;
  If not (RangeChecking in StatementCompilerSwitches) or (TypePtr^.BaseType = btBoolean) then
    begin
      ExtendInteger (TypePtr^.DataType + LowestDataType);
      Exit;
    end;
  TempInt.Long := 0;                       { Lowest possible value }
  If itSigned in DataType then { Negative }
    begin
      TempInt.Long := $FFFFFF80;
      If it16Bit in DataType then
        begin
          TempInt.Long := $FFFF8000;
          If it32Bit in DataType then TempInt.Long := $80000000;
        end;
    end;
  If IsConstantInRange (TempInt, TypePtr) then
    begin                                       { Check highest possible value }
      TempInt.Long := $0000007F;
      If itUnsigned in DataType then TempInt.Long := $000000FF;
      If it16Bit in DataType then
        begin
          TempInt.WordL := TempInt.WordL shl 8 or $FF;
          If it32Bit in DataType then
            begin
              TempInt.WordH := TempInt.WordL;
              TempInt.WordL := $FFFF;
            end;
        end;
      If IsConstantInRange (TempInt, TypePtr) then
        begin
          ExtendInteger (TypePtr^.DataType + LowestDataType);
          Exit;
        end;
    end;
  LowerLimit := POrdinalTypeDefinition (TypePtr)^.LowerLimit;
  UpperLimit := POrdinalTypeDefinition (TypePtr)^.UpperLimit;
  PointerToLimits := @LowerLimit;
  ConstantOffset := StoreConstantToCodeBlock (8, PointerToLimits, ProgramCodeBlock);
  ExtendInteger ([it32Bit, it16Bit, itSigned]);
  Calculate;
  LoadExpressionToRegisters (urAX);
  GenerateInstruction_Byte (MOV_DI_Immediate);
  GenerateReference (SymbolTable [stMain].Segment, ProgramCodeBlock, ConstantOffset, [rfConstant, rfOffset]);
  GenerateInstruction_CALL_FAR (SysProc_RangeCheck);
  EndIntermediateCodeSubroutine;
end;
Each ordinal type definition contains lower and upper limit. This function checks them.
Function IsConstantInRange (Int: LongRec; TypePtr: PTypeDefinition): Boolean;
begin
  IsConstantInRange := False;
  If Int.Long <; POrdinalTypeDefinition (TypePtr)^.LowerLimit then Exit;
  If Int.Long > POrdinalTypeDefinition (TypePtr)^.UpperLimit then Exit;
  IsConstantInRange := True;
end;
If overflow flag is set this procedure reports error otherwise it stores new integer to expression
Value
and sets lowest integer type.
Procedure TExpression.CheckOrdinalOverflowAndStore (Int: LongInt; Overflow: Boolean);
begin
  If Overflow then Error (OverflowInArithmeticOperation);
  Value.LongInt := Int;
  LowestIntegerType (Int, DataType);
end;
This procedure loads constant set into
Value
or pops set pointer if set is on stack frame. Location is either
elConstant
or
elMemory
.
Procedure TExpression.LoadSetAndPopPointer (TypeDefDataType: TIntegerTypeSet);
begin
  LoadConstantSmallSetIntoValue;
  If it32Bytes in DataType then
    begin
      DataType := TypeDefDataType;
      If Location = elStackFrame then PopPointerAndConvertToMemory (Self);
    end else ExtendInteger (TypeDefDataType);
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
Procedure TExpression.CheckRange (TypeDef: PTypeDefinition);
begin
  Case TypeDef^.BaseType of
    btInteger..btEnumeration: CheckOrdinalRange (TypeDef);
    btSet: Case it32Bytes in TypeDef^.DataType of
             False: LoadSetAndPopPointer (TypeDef^.DataType);
             True:  ExpandSetToStackFrameAndPushPointer (32);
           end;
  end;
end;

Procedure TExpression.CheckOrdinalRange (TypePtr: PTypeDefinition);
Var TempInt: LongRec;
    UpperLimit, LowerLimit: LongInt;
    ConstantOffset, ProgramCodeBlock: Word;
    PointerToLimits: Pointer;
begin
  If Location = elConstant then
    begin
      If not IsConstantInRange (Value.LongRec, TypePtr) then Error (ConstantOutOfRange);
      DataType := TypePtr^.DataType + LowestDataType;
      Exit;
    end;
  If not (RangeChecking in StatementCompilerSwitches) or (TypePtr^.BaseType = btBoolean) then
    begin
      ExtendInteger (TypePtr^.DataType + LowestDataType);
      Exit;
    end;
  TempInt.Long := 0;                       { Lowest possible value }
  If itSigned in DataType then { Negative }
    begin
      TempInt.Long := $FFFFFF80;
      If it16Bit in DataType then
        begin
          TempInt.Long := $FFFF8000;
          If it32Bit in DataType then TempInt.Long := $80000000;
        end;
    end;
  If IsConstantInRange (TempInt, TypePtr) then
    begin                                       { Check highest possible value }
      TempInt.Long := $0000007F;
      If itUnsigned in DataType then TempInt.Long := $000000FF;
      If it16Bit in DataType then
        begin
          TempInt.WordL := TempInt.WordL shl 8 or $FF;
          If it32Bit in DataType then
            begin
              TempInt.WordH := TempInt.WordL;
              TempInt.WordL := $FFFF;
            end;
        end;
      If IsConstantInRange (TempInt, TypePtr) then
        begin
          ExtendInteger (TypePtr^.DataType + LowestDataType);
          Exit;
        end;
    end;
  LowerLimit := POrdinalTypeDefinition (TypePtr)^.LowerLimit;
  UpperLimit := POrdinalTypeDefinition (TypePtr)^.UpperLimit;
  PointerToLimits := @LowerLimit;
  ConstantOffset := StoreConstantToCodeBlock (8, PointerToLimits, ProgramCodeBlock);
  ExtendInteger ([it32Bit, it16Bit, itSigned]);
  Calculate;
  LoadExpressionToRegisters (urAX);
  GenerateInstruction_Byte (MOV_DI_Immediate);
  GenerateReference (SymbolTable [stMain].Segment, ProgramCodeBlock, ConstantOffset, [rfConstant, rfOffset]);
  GenerateInstruction_CALL_FAR (SysProc_RangeCheck);
  EndIntermediateCodeSubroutine;
end;
```

### Code Block 2

```pascal
Function IsConstantInRange (Int: LongRec; TypePtr: PTypeDefinition): Boolean;
begin
  IsConstantInRange := False;
  If Int.Long <; POrdinalTypeDefinition (TypePtr)^.LowerLimit then Exit;
  If Int.Long > POrdinalTypeDefinition (TypePtr)^.UpperLimit then Exit;
  IsConstantInRange := True;
end;

```

### Code Block 3

```pascal
Procedure TExpression.CheckOrdinalOverflowAndStore (Int: LongInt; Overflow: Boolean);
begin
  If Overflow then Error (OverflowInArithmeticOperation);
  Value.LongInt := Int;
  LowestIntegerType (Int, DataType);
end;
```

### Code Block 4

```pascal
Procedure TExpression.LoadSetAndPopPointer (TypeDefDataType: TIntegerTypeSet);
begin
  LoadConstantSmallSetIntoValue;
  If it32Bytes in DataType then
    begin
      DataType := TypeDefDataType;
      If Location = elStackFrame then PopPointerAndConvertToMemory (Self);
    end else ExtendInteger (TypeDefDataType);
end;

```

### Code Block 5

```pascal
Procedure TExpression.CheckRange (TypeDef: PTypeDefinition);
begin
  Case TypeDef^.BaseType of
    btInteger..btEnumeration: CheckOrdinalRange (TypeDef);
    btSet: Case it32Bytes in TypeDef^.DataType of
             False: LoadSetAndPopPointer (TypeDef^.DataType);
             True:  ExpandSetToStackFrameAndPushPointer (32);
           end;
  end;
end;

Procedure TExpression.CheckOrdinalRange (TypePtr: PTypeDefinition);
Var TempInt: LongRec;
    UpperLimit, LowerLimit: LongInt;
    ConstantOffset, ProgramCodeBlock: Word;
    PointerToLimits: Pointer;
begin
  If Location = elConstant then
    begin
      If not IsConstantInRange (Value.LongRec, TypePtr) then Error (ConstantOutOfRange);
      DataType := TypePtr^.DataType + LowestDataType;
      Exit;
    end;
  If not (RangeChecking in StatementCompilerSwitches) or (TypePtr^.BaseType = btBoolean) then
    begin
      ExtendInteger (TypePtr^.DataType + LowestDataType);
      Exit;
    end;
  TempInt.Long := 0;                       { Lowest possible value }
  If itSigned in DataType then { Negative }
    begin
      TempInt.Long := $FFFFFF80;
      If it16Bit in DataType then
        begin
          TempInt.Long := $FFFF8000;
          If it32Bit in DataType then TempInt.Long := $80000000;
        end;
    end;
  If IsConstantInRange (TempInt, TypePtr) then
    begin                                       { Check highest possible value }
      TempInt.Long := $0000007F;
      If itUnsigned in DataType then TempInt.Long := $000000FF;
      If it16Bit in DataType then
        begin
          TempInt.WordL := TempInt.WordL shl 8 or $FF;
          If it32Bit in DataType then
            begin
              TempInt.WordH := TempInt.WordL;
              TempInt.WordL := $FFFF;
            end;
        end;
      If IsConstantInRange (TempInt, TypePtr) then
        begin
          ExtendInteger (TypePtr^.DataType + LowestDataType);
          Exit;
        end;
    end;
  LowerLimit := POrdinalTypeDefinition (TypePtr)^.LowerLimit;
  UpperLimit := POrdinalTypeDefinition (TypePtr)^.UpperLimit;
  PointerToLimits := @LowerLimit;
  ConstantOffset := StoreConstantToCodeBlock (8, PointerToLimits, ProgramCodeBlock);
  ExtendInteger ([it32Bit, it16Bit, itSigned]);
  Calculate;
  LoadExpressionToRegisters (urAX);
  GenerateInstruction_Byte (MOV_DI_Immediate);
  GenerateReference (SymbolTable [stMain].Segment, ProgramCodeBlock, ConstantOffset, [rfConstant, rfOffset]);
  GenerateInstruction_CALL_FAR (SysProc_RangeCheck);
  EndIntermediateCodeSubroutine;
end;
```

### Code Block 6

```pascal
Function IsConstantInRange (Int: LongRec; TypePtr: PTypeDefinition): Boolean;
begin
  IsConstantInRange := False;
  If Int.Long <; POrdinalTypeDefinition (TypePtr)^.LowerLimit then Exit;
  If Int.Long > POrdinalTypeDefinition (TypePtr)^.UpperLimit then Exit;
  IsConstantInRange := True;
end;

```

### Code Block 7

```pascal
Procedure TExpression.CheckOrdinalOverflowAndStore (Int: LongInt; Overflow: Boolean);
begin
  If Overflow then Error (OverflowInArithmeticOperation);
  Value.LongInt := Int;
  LowestIntegerType (Int, DataType);
end;
```

### Code Block 8

```pascal
Procedure TExpression.LoadSetAndPopPointer (TypeDefDataType: TIntegerTypeSet);
begin
  LoadConstantSmallSetIntoValue;
  If it32Bytes in DataType then
    begin
      DataType := TypeDefDataType;
      If Location = elStackFrame then PopPointerAndConvertToMemory (Self);
    end else ExtendInteger (TypeDefDataType);
end;

```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/checking-ranges
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

