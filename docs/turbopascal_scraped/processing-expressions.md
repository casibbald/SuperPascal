# Processing Expressions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-expressions

---

## Table of Contents

- Processing Expressions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Expressions
These routines provide basic expression processing. Each expression is divided into simpler elements according to the operator precedence:
Expression
,
Simple Expression
,
Term
and
Factor
. Once the operation and operands are determined, the
mathematical operation
can be performed.
Procedure TExpression.ProcessExpression;
Var RightSimpleExpression: TExpression;
    Operation: TCalcOperation;
begin
  ProcessSimpleExpression;
  Case Token of
    Token_Equal:          Operation := Calc_IsEqual;
    Token_NotEqual:       Operation := Calc_IsNotEqual;
    Token_Greater:        Operation := Calc_IsGreater;
    Token_Lower:          Operation := Calc_IsLower;
    Token_GreaterOrEqual: Operation := Calc_IsGreaterOrEqual;
    Token_LowerOrEqual:   Operation := Calc_IsLowerOrEqual;
    Token_IN:             Operation := Calc_IN;
    else begin
           Exit;
         end;
  end;
  GetNextToken;
  RightSimpleExpression.ProcessSimpleExpression;
  If Operation = Calc_IN then GenerateCodeForOperator_IN (Self, RightSimpleExpression)
    else CalculateOperation (Operation, Self, RightSimpleExpression);
end;

Procedure TExpression.ProcessSimpleExpression;
Var RightTerm: TExpression;
    Operation: TCalcOperation;
begin
  ProcessTerm;
  Repeat
    Case Token of
      Token_Plus:  Operation := Calc_Add;
      Token_Minus: Operation := Calc_Subtract;
      Token_OR:    Operation := Calc_OR;
      Token_XOR:   Operation := Calc_XOR;
      else Exit;
    end;
    GetNextToken;
    RightTerm.ProcessTerm;
    CalculateOperation (Operation, Self, RightTerm);
  until False;
end;

Procedure TExpression.ProcessTerm;
Var RightFactor: TExpression;
    Operation: TCalcOperation;
begin
  ProcessFactor;
  Repeat
    Case Token of
      Token_Asterisk: Operation := Calc_Multiply;
      Token_Slash:    Operation := Calc_Divide;
      Token_DIV:      Operation := Calc_DIV;
      Token_MOD:      Operation := Calc_MOD;
      Token_AND:      Operation := Calc_AND;
      Token_SHL:      Operation := Calc_SHL;
      Token_SHR:      Operation := Calc_SHR;
      else Exit;
    end;
    GetNextToken;
    RightFactor.ProcessFactor;
    CalculateOperation (Operation, Self, RightFactor);
  until False;
end;
Factor is the smallest element of any Turbo Pascal expression. It can be a
variable
,
constant
, system function, special operator or another expression in parenthesis.
Procedure TExpression.ProcessFactor;
Var ProcedureIdentifierData: PProcedureIdentifierData;
begin
  CheckForStringConstantWithControlCharacter;
  CheckForDeclaredIdentifier;
  Case Token of
    Token_VariableIdentifier:        LoadVariable;
    Token_Constant,
    Token_ConstantIdentifier:        LoadConstant;
    Token_ProcedureIdentifier:       ProcessProcedureIdentifier;
    Token_LeftParenthesis:           ProcessLeftParenthesis;
    Token_Minus,
    Token_Plus:                      ProcessUnaryMinus_Plus;
    Token_NOT:                       Process_NOT;
    Token_NIL:                       LoadNilPointer;
    Token_TypeIdentifier,
    Token_STRING,
    Token_FILE:                      ProcessValueTypecastOrMethodCall;
    Token_INHERITED:                 Process_INHERITED;
    Token_LeftBracket:               ProcessSetConstant;
    Token_SystemFunction,
    Token_SystemFunctionOrProcedure: Process_SystemFunction;
    Token_At:                        Process_At;
    Token_Mem:                       Process_Mem;
    Token_Port:                      Process_Port;
    else Error (ErrorInExpression);
  end;
  Repeat
    If TypeDefPtr^.BaseType = btFunctionProcedure then
      Case PProcedureTypeDefinition (TypeDefPtr)^.ResultTypeOffset.TypeOffset = 0 of
        True: begin
                If Location <> elProcedure then Error (InvalidProcedureOrFunctionReference);
                ProcedureIdentifierData := PProcedureIdentifierData (PChar (TypeDefPtr) - 10);
                If not (pfConstructor in ProcedureIdentifierData^.Flags) then Error (InvalidProcedureOrFunctionReference);
                CallProcedure;
                Calculate;
                GenerateInstruction_TwoBytes ($09, $D0);          { OR     AX, DX }
                EndIntermediateCodeSubroutine;
                UsedRegisters := [urSP, urDX, urAX];
                SetExpressionToBooleanJump (JNE);
                Exit;
              end;
        else begin
               CallProcedure;
               SetExpressionToFunctionResult;
             end;
      end;
  until not ProcessQualifiers;
end;
This procedure sets expression data for functions according to the result type. For String result it first reserves space in the
stack frame
.
Procedure TExpression.SetExpressionToFunctionResult;
begin
  TypeDefPtr := PointerFromOffsets (PProcedureTypeDefinition (TypeDefPtr)^.ResultTypeOffset);
  UsedRegisters := [urBX, urDX, urCX, urAX];
  Case TypeDefPtr^.BaseType of
    btPointer,
    btReal,
    btInteger..btEnumeration: begin
                                Location := elRegister;
                                DataType := TypeDefPtr^.DataType;
                                LocationData.Register := rDX_AX;
                              end;
    btExtended: begin
                  Location := elStackFrame;
                  DataType := [fpExtended];
                end;
    else begin
           CreateSpaceInStackFrameAndPushAddressToIt ($0100);
           Calculate;
           EndIntermediateCodeSubroutine;
         end;
  end;
end;
Different order of expression processing can be achieved with parentheses. Simple recursive call solves the problem.
Procedure TExpression.ProcessLeftParenthesis;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  ProcessExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
Nil
pointer is a simple constant that generates no code.
Procedure LoadNilPointer;
begin
  GetNextToken;
  Location := elConstant;
  IntermediateCodeOffset := 0;
  DataType := [it32Bit, it16Bit, itSigned];
  UsedRegisters := [];
  Value.Pointer := nil;
  TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
end;
For
Procedures
and
Functions
the location is set and for methods Self address is loaded. The procedure or function will be called later at the factor processing procedure.
Procedure TExpression.ProcessProcedureIdentifier;
begin
  IntermediateCodeOffset := 0;
  UsedRegisters := [];
  If pfMethod in PProcedureIdentifierData (CurrentIdentifierDataPtr)^.Flags then LoadSelfAddress;
  TypeDefPtr := @PProcedureIdentifierData (CurrentIdentifierDataPtr)^.ProcedureTypeDefinition;
  Location := elProcedure;
  DataType := [];
  Value.W16 := Ofs (CurrentRecordTypeDef^);
  Value.W18 := Seg (CurrentRecordTypeDef^);
  GetNextToken;
end;
This procedure processes call to inherited method.
Procedure TExpression.Process_INHERITED;
Var MethodIdentifier: PIdentifier;
    MethodIdentifierData: PProcedureIdentifierData;
    UnitTypeOffsets: PUnitOffsets;
    TypeDef: PTypeDefinition;
begin
  If not FindCurrentMethod (MethodIdentifier, MethodIdentifierData) then Error (NoInheritedMethodsAreAccessibleHere);
  UnitTypeOffsets := Ptr (Seg (MethodIdentifierData^), MethodIdentifierData^.OuterBlockProcedureIdentifier + 12);
  If UnitTypeOffsets^.UnitIdentifierData = 0 then Error (NoInheritedMethodsAreAccessibleHere);
  TypeDef := PointerFromOffsets (UnitTypeOffsets^);
  GetNextToken;
  IntermediateCodeOffset := 0;
  UsedRegisters := [];
  FindAndProcessMethodCall (TypeDef, [pfMethod], [mcStatic], MethodIdentifierExpected);
end;
This procedure processes system function.
Procedure TExpression.Process_SystemFunction;
Var Func: Word;
begin
  Func := (PSystemProcedureIdentifierData (CurrentIdentifierDataPtr)^.ProcData and $3FFF) shr 2;
  GetNextToken;
  ProcessFunction (Func, Self);
end;
This procedure processes the
@
character and creates pointer to variable reference.
Procedure TExpression.Process_At;
begin
  GetNextToken;
  ExpectVariableReference;
  CreatePointerToVariableReference;
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
Procedure TExpression.ProcessExpression;
Var RightSimpleExpression: TExpression;
    Operation: TCalcOperation;
begin
  ProcessSimpleExpression;
  Case Token of
    Token_Equal:          Operation := Calc_IsEqual;
    Token_NotEqual:       Operation := Calc_IsNotEqual;
    Token_Greater:        Operation := Calc_IsGreater;
    Token_Lower:          Operation := Calc_IsLower;
    Token_GreaterOrEqual: Operation := Calc_IsGreaterOrEqual;
    Token_LowerOrEqual:   Operation := Calc_IsLowerOrEqual;
    Token_IN:             Operation := Calc_IN;
    else begin
           Exit;
         end;
  end;
  GetNextToken;
  RightSimpleExpression.ProcessSimpleExpression;
  If Operation = Calc_IN then GenerateCodeForOperator_IN (Self, RightSimpleExpression)
    else CalculateOperation (Operation, Self, RightSimpleExpression);
end;

Procedure TExpression.ProcessSimpleExpression;
Var RightTerm: TExpression;
    Operation: TCalcOperation;
begin
  ProcessTerm;
  Repeat
    Case Token of
      Token_Plus:  Operation := Calc_Add;
      Token_Minus: Operation := Calc_Subtract;
      Token_OR:    Operation := Calc_OR;
      Token_XOR:   Operation := Calc_XOR;
      else Exit;
    end;
    GetNextToken;
    RightTerm.ProcessTerm;
    CalculateOperation (Operation, Self, RightTerm);
  until False;
end;

Procedure TExpression.ProcessTerm;
Var RightFactor: TExpression;
    Operation: TCalcOperation;
begin
  ProcessFactor;
  Repeat
    Case Token of
      Token_Asterisk: Operation := Calc_Multiply;
      Token_Slash:    Operation := Calc_Divide;
      Token_DIV:      Operation := Calc_DIV;
      Token_MOD:      Operation := Calc_MOD;
      Token_AND:      Operation := Calc_AND;
      Token_SHL:      Operation := Calc_SHL;
      Token_SHR:      Operation := Calc_SHR;
      else Exit;
    end;
    GetNextToken;
    RightFactor.ProcessFactor;
    CalculateOperation (Operation, Self, RightFactor);
  until False;
end;
```

### Code Block 2

```pascal
Procedure TExpression.ProcessFactor;
Var ProcedureIdentifierData: PProcedureIdentifierData;
begin
  CheckForStringConstantWithControlCharacter;
  CheckForDeclaredIdentifier;
  Case Token of
    Token_VariableIdentifier:        LoadVariable;
    Token_Constant,
    Token_ConstantIdentifier:        LoadConstant;
    Token_ProcedureIdentifier:       ProcessProcedureIdentifier;
    Token_LeftParenthesis:           ProcessLeftParenthesis;
    Token_Minus,
    Token_Plus:                      ProcessUnaryMinus_Plus;
    Token_NOT:                       Process_NOT;
    Token_NIL:                       LoadNilPointer;
    Token_TypeIdentifier,
    Token_STRING,
    Token_FILE:                      ProcessValueTypecastOrMethodCall;
    Token_INHERITED:                 Process_INHERITED;
    Token_LeftBracket:               ProcessSetConstant;
    Token_SystemFunction,
    Token_SystemFunctionOrProcedure: Process_SystemFunction;
    Token_At:                        Process_At;
    Token_Mem:                       Process_Mem;
    Token_Port:                      Process_Port;
    else Error (ErrorInExpression);
  end;
  Repeat
    If TypeDefPtr^.BaseType = btFunctionProcedure then
      Case PProcedureTypeDefinition (TypeDefPtr)^.ResultTypeOffset.TypeOffset = 0 of
        True: begin
                If Location <> elProcedure then Error (InvalidProcedureOrFunctionReference);
                ProcedureIdentifierData := PProcedureIdentifierData (PChar (TypeDefPtr) - 10);
                If not (pfConstructor in ProcedureIdentifierData^.Flags) then Error (InvalidProcedureOrFunctionReference);
                CallProcedure;
                Calculate;
                GenerateInstruction_TwoBytes ($09, $D0);          { OR     AX, DX }
                EndIntermediateCodeSubroutine;
                UsedRegisters := [urSP, urDX, urAX];
                SetExpressionToBooleanJump (JNE);
                Exit;
              end;
        else begin
               CallProcedure;
               SetExpressionToFunctionResult;
             end;
      end;
  until not ProcessQualifiers;
end;
```

### Code Block 3

```pascal
Procedure TExpression.SetExpressionToFunctionResult;
begin
  TypeDefPtr := PointerFromOffsets (PProcedureTypeDefinition (TypeDefPtr)^.ResultTypeOffset);
  UsedRegisters := [urBX, urDX, urCX, urAX];
  Case TypeDefPtr^.BaseType of
    btPointer,
    btReal,
    btInteger..btEnumeration: begin
                                Location := elRegister;
                                DataType := TypeDefPtr^.DataType;
                                LocationData.Register := rDX_AX;
                              end;
    btExtended: begin
                  Location := elStackFrame;
                  DataType := [fpExtended];
                end;
    else begin
           CreateSpaceInStackFrameAndPushAddressToIt ($0100);
           Calculate;
           EndIntermediateCodeSubroutine;
         end;
  end;
end;
```

### Code Block 4

```pascal
Procedure TExpression.ProcessLeftParenthesis;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  ProcessExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
```

### Code Block 5

```pascal
Procedure LoadNilPointer;
begin
  GetNextToken;
  Location := elConstant;
  IntermediateCodeOffset := 0;
  DataType := [it32Bit, it16Bit, itSigned];
  UsedRegisters := [];
  Value.Pointer := nil;
  TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
end;
```

### Code Block 6

```pascal
Procedure TExpression.ProcessProcedureIdentifier;
begin
  IntermediateCodeOffset := 0;
  UsedRegisters := [];
  If pfMethod in PProcedureIdentifierData (CurrentIdentifierDataPtr)^.Flags then LoadSelfAddress;
  TypeDefPtr := @PProcedureIdentifierData (CurrentIdentifierDataPtr)^.ProcedureTypeDefinition;
  Location := elProcedure;
  DataType := [];
  Value.W16 := Ofs (CurrentRecordTypeDef^);
  Value.W18 := Seg (CurrentRecordTypeDef^);
  GetNextToken;
end;
```

### Code Block 7

```pascal
Procedure TExpression.Process_INHERITED;
Var MethodIdentifier: PIdentifier;
    MethodIdentifierData: PProcedureIdentifierData;
    UnitTypeOffsets: PUnitOffsets;
    TypeDef: PTypeDefinition;
begin
  If not FindCurrentMethod (MethodIdentifier, MethodIdentifierData) then Error (NoInheritedMethodsAreAccessibleHere);
  UnitTypeOffsets := Ptr (Seg (MethodIdentifierData^), MethodIdentifierData^.OuterBlockProcedureIdentifier + 12);
  If UnitTypeOffsets^.UnitIdentifierData = 0 then Error (NoInheritedMethodsAreAccessibleHere);
  TypeDef := PointerFromOffsets (UnitTypeOffsets^);
  GetNextToken;
  IntermediateCodeOffset := 0;
  UsedRegisters := [];
  FindAndProcessMethodCall (TypeDef, [pfMethod], [mcStatic], MethodIdentifierExpected);
end;
```

### Code Block 8

```pascal
Procedure TExpression.Process_SystemFunction;
Var Func: Word;
begin
  Func := (PSystemProcedureIdentifierData (CurrentIdentifierDataPtr)^.ProcData and $3FFF) shr 2;
  GetNextToken;
  ProcessFunction (Func, Self);
end;
```

### Code Block 9

```pascal
Procedure TExpression.Process_At;
begin
  GetNextToken;
  ExpectVariableReference;
  CreatePointerToVariableReference;
end;
```

### Code Block 10

```pascal
Procedure TExpression.ProcessExpression;
Var RightSimpleExpression: TExpression;
    Operation: TCalcOperation;
begin
  ProcessSimpleExpression;
  Case Token of
    Token_Equal:          Operation := Calc_IsEqual;
    Token_NotEqual:       Operation := Calc_IsNotEqual;
    Token_Greater:        Operation := Calc_IsGreater;
    Token_Lower:          Operation := Calc_IsLower;
    Token_GreaterOrEqual: Operation := Calc_IsGreaterOrEqual;
    Token_LowerOrEqual:   Operation := Calc_IsLowerOrEqual;
    Token_IN:             Operation := Calc_IN;
    else begin
           Exit;
         end;
  end;
  GetNextToken;
  RightSimpleExpression.ProcessSimpleExpression;
  If Operation = Calc_IN then GenerateCodeForOperator_IN (Self, RightSimpleExpression)
    else CalculateOperation (Operation, Self, RightSimpleExpression);
end;

Procedure TExpression.ProcessSimpleExpression;
Var RightTerm: TExpression;
    Operation: TCalcOperation;
begin
  ProcessTerm;
  Repeat
    Case Token of
      Token_Plus:  Operation := Calc_Add;
      Token_Minus: Operation := Calc_Subtract;
      Token_OR:    Operation := Calc_OR;
      Token_XOR:   Operation := Calc_XOR;
      else Exit;
    end;
    GetNextToken;
    RightTerm.ProcessTerm;
    CalculateOperation (Operation, Self, RightTerm);
  until False;
end;

Procedure TExpression.ProcessTerm;
Var RightFactor: TExpression;
    Operation: TCalcOperation;
begin
  ProcessFactor;
  Repeat
    Case Token of
      Token_Asterisk: Operation := Calc_Multiply;
      Token_Slash:    Operation := Calc_Divide;
      Token_DIV:      Operation := Calc_DIV;
      Token_MOD:      Operation := Calc_MOD;
      Token_AND:      Operation := Calc_AND;
      Token_SHL:      Operation := Calc_SHL;
      Token_SHR:      Operation := Calc_SHR;
      else Exit;
    end;
    GetNextToken;
    RightFactor.ProcessFactor;
    CalculateOperation (Operation, Self, RightFactor);
  until False;
end;
```

### Code Block 11

```pascal
Procedure TExpression.ProcessFactor;
Var ProcedureIdentifierData: PProcedureIdentifierData;
begin
  CheckForStringConstantWithControlCharacter;
  CheckForDeclaredIdentifier;
  Case Token of
    Token_VariableIdentifier:        LoadVariable;
    Token_Constant,
    Token_ConstantIdentifier:        LoadConstant;
    Token_ProcedureIdentifier:       ProcessProcedureIdentifier;
    Token_LeftParenthesis:           ProcessLeftParenthesis;
    Token_Minus,
    Token_Plus:                      ProcessUnaryMinus_Plus;
    Token_NOT:                       Process_NOT;
    Token_NIL:                       LoadNilPointer;
    Token_TypeIdentifier,
    Token_STRING,
    Token_FILE:                      ProcessValueTypecastOrMethodCall;
    Token_INHERITED:                 Process_INHERITED;
    Token_LeftBracket:               ProcessSetConstant;
    Token_SystemFunction,
    Token_SystemFunctionOrProcedure: Process_SystemFunction;
    Token_At:                        Process_At;
    Token_Mem:                       Process_Mem;
    Token_Port:                      Process_Port;
    else Error (ErrorInExpression);
  end;
  Repeat
    If TypeDefPtr^.BaseType = btFunctionProcedure then
      Case PProcedureTypeDefinition (TypeDefPtr)^.ResultTypeOffset.TypeOffset = 0 of
        True: begin
                If Location <> elProcedure then Error (InvalidProcedureOrFunctionReference);
                ProcedureIdentifierData := PProcedureIdentifierData (PChar (TypeDefPtr) - 10);
                If not (pfConstructor in ProcedureIdentifierData^.Flags) then Error (InvalidProcedureOrFunctionReference);
                CallProcedure;
                Calculate;
                GenerateInstruction_TwoBytes ($09, $D0);          { OR     AX, DX }
                EndIntermediateCodeSubroutine;
                UsedRegisters := [urSP, urDX, urAX];
                SetExpressionToBooleanJump (JNE);
                Exit;
              end;
        else begin
               CallProcedure;
               SetExpressionToFunctionResult;
             end;
      end;
  until not ProcessQualifiers;
end;
```

### Code Block 12

```pascal
Procedure TExpression.SetExpressionToFunctionResult;
begin
  TypeDefPtr := PointerFromOffsets (PProcedureTypeDefinition (TypeDefPtr)^.ResultTypeOffset);
  UsedRegisters := [urBX, urDX, urCX, urAX];
  Case TypeDefPtr^.BaseType of
    btPointer,
    btReal,
    btInteger..btEnumeration: begin
                                Location := elRegister;
                                DataType := TypeDefPtr^.DataType;
                                LocationData.Register := rDX_AX;
                              end;
    btExtended: begin
                  Location := elStackFrame;
                  DataType := [fpExtended];
                end;
    else begin
           CreateSpaceInStackFrameAndPushAddressToIt ($0100);
           Calculate;
           EndIntermediateCodeSubroutine;
         end;
  end;
end;
```

### Code Block 13

```pascal
Procedure TExpression.ProcessLeftParenthesis;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  ProcessExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
```

### Code Block 14

```pascal
Procedure LoadNilPointer;
begin
  GetNextToken;
  Location := elConstant;
  IntermediateCodeOffset := 0;
  DataType := [it32Bit, it16Bit, itSigned];
  UsedRegisters := [];
  Value.Pointer := nil;
  TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
end;
```

### Code Block 15

```pascal
Procedure TExpression.ProcessProcedureIdentifier;
begin
  IntermediateCodeOffset := 0;
  UsedRegisters := [];
  If pfMethod in PProcedureIdentifierData (CurrentIdentifierDataPtr)^.Flags then LoadSelfAddress;
  TypeDefPtr := @PProcedureIdentifierData (CurrentIdentifierDataPtr)^.ProcedureTypeDefinition;
  Location := elProcedure;
  DataType := [];
  Value.W16 := Ofs (CurrentRecordTypeDef^);
  Value.W18 := Seg (CurrentRecordTypeDef^);
  GetNextToken;
end;
```

### Code Block 16

```pascal
Procedure TExpression.Process_INHERITED;
Var MethodIdentifier: PIdentifier;
    MethodIdentifierData: PProcedureIdentifierData;
    UnitTypeOffsets: PUnitOffsets;
    TypeDef: PTypeDefinition;
begin
  If not FindCurrentMethod (MethodIdentifier, MethodIdentifierData) then Error (NoInheritedMethodsAreAccessibleHere);
  UnitTypeOffsets := Ptr (Seg (MethodIdentifierData^), MethodIdentifierData^.OuterBlockProcedureIdentifier + 12);
  If UnitTypeOffsets^.UnitIdentifierData = 0 then Error (NoInheritedMethodsAreAccessibleHere);
  TypeDef := PointerFromOffsets (UnitTypeOffsets^);
  GetNextToken;
  IntermediateCodeOffset := 0;
  UsedRegisters := [];
  FindAndProcessMethodCall (TypeDef, [pfMethod], [mcStatic], MethodIdentifierExpected);
end;
```

### Code Block 17

```pascal
Procedure TExpression.Process_SystemFunction;
Var Func: Word;
begin
  Func := (PSystemProcedureIdentifierData (CurrentIdentifierDataPtr)^.ProcData and $3FFF) shr 2;
  GetNextToken;
  ProcessFunction (Func, Self);
end;
```

### Code Block 18

```pascal
Procedure TExpression.Process_At;
begin
  GetNextToken;
  ExpectVariableReference;
  CreatePointerToVariableReference;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-and-destroying-stack-frame
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/loading-constants
- https://turbopascal.org/loading-variables
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-expressions
- https://turbopascal.org/processing-mathematical-operations
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

