# System Procedure Val - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-procedure-val

---

## Table of Contents

- System Procedure Val

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Procedure Val
This function processes system procedure
Val
.
Function Proc_Val: Word; Far;
Var StringExpression, ValueExpression, ErrorExpression: TExpression;
    ValProc: Word;
    ValueBaseType: TBaseType;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  StringExpression.ExpectAndStoreStringCompatibleExpressionAndPushPointer;
  ExpectTokenAndGetNext (Token_Comma);
  ValueExpression.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  ValueBaseType := ValueExpression.ExpressionBaseType;
  Case ValueBaseType of
    btInteger: Case StringExpression.TypeDefPtr^.BaseType of
                 btString: ValProc := SysProc_ValInt;
                 else ValProc := SysProc_ValIntC;
               end;
    btExtended: Case StringExpression.TypeDefPtr^.BaseType of
                  btString: ValProc := SysProc_ValFloat;
                  else ValProc := SysProc_ValFloatC;
                end;
    btReal: Case StringExpression.TypeDefPtr^.BaseType of
              btString: ValProc := SysProc_ValReal;
              else ValProc := SysProc_ValRealC;
            end;
    else Error (IntegerOrRealVariableExpected);
  end;
  ExpectTokenAndGetNext (Token_Comma);
  ErrorExpression.Expect16BitIntegerVariable;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StringExpression.Calculate;
  StoreCode_icGoSub (ErrorExpression.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (ValProc);
  StringExpression.EndIntermediateCodeSubroutine;
  With StringExpression do
    begin
      Case ValueBaseType of
        btInteger: begin
                     Location := elRegister;
                     DataType := [it32Bit, it16Bit, itSigned];
                     LocationData.Register := rDX_AX;
                   end;
        btReal: begin
                  Location := elRegister;
                  DataType := [it16Bit];
                  TypeDefPtr := Ptr (SystemUnitSegment, Real_TypeOffset);
                end;
        btExtended: begin
                      Location := elStackFrame;
                      DataType := [fpExtended];
                      TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
                    end;
      end;
      UsedRegisters := [urBX, urDX, urCX, urAX];
    end;
  StringExpression.AdjustExpressionToType (ValueExpression.TypeDefPtr);
  StringExpression.CheckRange (ValueExpression.TypeDefPtr);
  GenerateAssignmentIntermediateCode (ValueExpression, StringExpression);
  Proc_Val := ValueExpression.IntermediateCodeOffset;
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
Function Proc_Val: Word; Far;
Var StringExpression, ValueExpression, ErrorExpression: TExpression;
    ValProc: Word;
    ValueBaseType: TBaseType;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  StringExpression.ExpectAndStoreStringCompatibleExpressionAndPushPointer;
  ExpectTokenAndGetNext (Token_Comma);
  ValueExpression.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  ValueBaseType := ValueExpression.ExpressionBaseType;
  Case ValueBaseType of
    btInteger: Case StringExpression.TypeDefPtr^.BaseType of
                 btString: ValProc := SysProc_ValInt;
                 else ValProc := SysProc_ValIntC;
               end;
    btExtended: Case StringExpression.TypeDefPtr^.BaseType of
                  btString: ValProc := SysProc_ValFloat;
                  else ValProc := SysProc_ValFloatC;
                end;
    btReal: Case StringExpression.TypeDefPtr^.BaseType of
              btString: ValProc := SysProc_ValReal;
              else ValProc := SysProc_ValRealC;
            end;
    else Error (IntegerOrRealVariableExpected);
  end;
  ExpectTokenAndGetNext (Token_Comma);
  ErrorExpression.Expect16BitIntegerVariable;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StringExpression.Calculate;
  StoreCode_icGoSub (ErrorExpression.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (ValProc);
  StringExpression.EndIntermediateCodeSubroutine;
  With StringExpression do
    begin
      Case ValueBaseType of
        btInteger: begin
                     Location := elRegister;
                     DataType := [it32Bit, it16Bit, itSigned];
                     LocationData.Register := rDX_AX;
                   end;
        btReal: begin
                  Location := elRegister;
                  DataType := [it16Bit];
                  TypeDefPtr := Ptr (SystemUnitSegment, Real_TypeOffset);
                end;
        btExtended: begin
                      Location := elStackFrame;
                      DataType := [fpExtended];
                      TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
                    end;
      end;
      UsedRegisters := [urBX, urDX, urCX, urAX];
    end;
  StringExpression.AdjustExpressionToType (ValueExpression.TypeDefPtr);
  StringExpression.CheckRange (ValueExpression.TypeDefPtr);
  GenerateAssignmentIntermediateCode (ValueExpression, StringExpression);
  Proc_Val := ValueExpression.IntermediateCodeOffset;
end;
```

### Code Block 2

```pascal
Function Proc_Val: Word; Far;
Var StringExpression, ValueExpression, ErrorExpression: TExpression;
    ValProc: Word;
    ValueBaseType: TBaseType;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  StringExpression.ExpectAndStoreStringCompatibleExpressionAndPushPointer;
  ExpectTokenAndGetNext (Token_Comma);
  ValueExpression.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  ValueBaseType := ValueExpression.ExpressionBaseType;
  Case ValueBaseType of
    btInteger: Case StringExpression.TypeDefPtr^.BaseType of
                 btString: ValProc := SysProc_ValInt;
                 else ValProc := SysProc_ValIntC;
               end;
    btExtended: Case StringExpression.TypeDefPtr^.BaseType of
                  btString: ValProc := SysProc_ValFloat;
                  else ValProc := SysProc_ValFloatC;
                end;
    btReal: Case StringExpression.TypeDefPtr^.BaseType of
              btString: ValProc := SysProc_ValReal;
              else ValProc := SysProc_ValRealC;
            end;
    else Error (IntegerOrRealVariableExpected);
  end;
  ExpectTokenAndGetNext (Token_Comma);
  ErrorExpression.Expect16BitIntegerVariable;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StringExpression.Calculate;
  StoreCode_icGoSub (ErrorExpression.IntermediateCodeOffset);
  GenerateInstruction_CALL_FAR (ValProc);
  StringExpression.EndIntermediateCodeSubroutine;
  With StringExpression do
    begin
      Case ValueBaseType of
        btInteger: begin
                     Location := elRegister;
                     DataType := [it32Bit, it16Bit, itSigned];
                     LocationData.Register := rDX_AX;
                   end;
        btReal: begin
                  Location := elRegister;
                  DataType := [it16Bit];
                  TypeDefPtr := Ptr (SystemUnitSegment, Real_TypeOffset);
                end;
        btExtended: begin
                      Location := elStackFrame;
                      DataType := [fpExtended];
                      TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
                    end;
      end;
      UsedRegisters := [urBX, urDX, urCX, urAX];
    end;
  StringExpression.AdjustExpressionToType (ValueExpression.TypeDefPtr);
  StringExpression.CheckRange (ValueExpression.TypeDefPtr);
  GenerateAssignmentIntermediateCode (ValueExpression, StringExpression);
  Proc_Val := ValueExpression.IntermediateCodeOffset;
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
- https://turbopascal.org/system-procedure-val
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

