# Expecting Different Expressions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/expecting-different-expressions

---

## Table of Contents

- Expecting Different Expressions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Expecting Different Expressions
Turbo Pascal expects different expression types depending on the context or language syntax. For this purposes there are many functions and procedures that
process expression
and report error if the expression doesn’t meet expectations.
This procedure expects string expression. Char and array compatible with string are converted to string.
Procedure TExpression.ExpectStringExpression;
begin
  CalculateExpression;
  ConvertCharToString;
  CopyCharArrayCompatibleWithStringToStackFrameAsString;
  If TypeDefPtr^.BaseType <> btString then Error (StringExpressionExpected);
end;
This procedure expects string variable and pushes pointer to memory.
Procedure TExpression.ExpectStringVariable;
begin
  ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  If TypeDefPtr^.BaseType <> btString then Error (StringVariableExpected);
end;
This procedure expects integer or floating-point expression.
Procedure TExpression.ExpectIntegerOrFloatingPointExpression;
begin
  CalculateExpression;
  If not (TypeDefPtr^.BaseType in [btInteger, btReal, btExtended]) then Error (IntegerOrRealExpressionExpected);
end;
This procedure expects integer or floating-point expression in parentheses.
Procedure TExpression.ExpectIntegerOrFloatingPointExpressionInParentheses;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  ExpectIntegerOrFloatingPointExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
This procedure expects integer expression.
Procedure TExpression.ExpectIntegerExpression;
begin
  CalculateExpression;
  If TypeDefPtr^.BaseType <> btInteger then Error (IntegerExpressionExpected);
end;
This procedure expects integer expression in parenthesis.
Procedure TExpression.ExpectIntegerExpressionInParentheses;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  ExpectIntegerExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
This procedure expects 16-bit integer variable and pushes pointer to memory. It is used for error variable for Val, BlockRead and BlockWrite system procedures.
Procedure TExpression.Expect16BitIntegerVariable;
begin
  ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  With TypeDefPtr^ do If (BaseType <> btInteger) or (Size <> 2) then Error (IntegerVariableExpected);
end;
This procedure expects ordinal expression.
Procedure TExpression.ExpectOrdinalExpression;
begin
  CalculateExpression;
  ConvertToBooleanByte;
  If TypeDefPtr^.BaseType < btInteger then Error (OrdinalExpressionExpected);
end;
This procedure expects ordinal expression in parentheses.
Procedure TExpression.ExpectOrdinalExpressionInParentheses;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  ExpectOrdinalExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
This procedure expects boolean expression and generates jump to code which executes when the expression eavluates to
False
. Code which executes when expression evaluates to
True
immediately follows the code for expression calculation therefore a label is placed there.
Procedure TExpression.ExpectBooleanExpression;
begin
  ProcessExpression;
  If TypeDefPtr^.BaseType <> btBoolean then Error (BooleanExpressionExpected);
  If Location = elConstant then Exit;
  ConvertExpressionToBooleanJump;
  Calculate;
  GenerateCodeForNearJump (Value.LastJumpToFalse, LocationData.JumpIfTrueOpCode xor 1);
  GenerateLabelAndSetJumpsToIt (Value.LastJumpToTrue);
  EndIntermediateCodeSubroutine;
end;
This procedure expects pointer expression.
Procedure TExpression.ExpectPointerExpression;
begin
  CalculateExpression;
  If TypeDefPtr^.BaseType <> btPointer then Error (PointerExpressionExpected);
end;
This procedure expects assignable pointer variable.
Procedure TExpression.ExpectPointerVariable;
begin
  ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  If TypeDefPtr^.BaseType <> btPointer then Error (PointerVariableExpected);
end;
This procedure expects and processes variable identifier.
Procedure TExpression.ExpectVariableIdentifier;
begin
  CheckForDeclaredIdentifier;
  Case Token of
    Token_VariableIdentifier:  LoadVariable;
    Token_ProcedureIdentifier: ProcessProcedureIdentifier;
    Token_TypeIdentifier,
    Token_STRING,
    Token_FILE:                ProcessValueTypecastOrMethodCall;
    Token_INHERITED:           Process_INHERITED;
    Token_LeftParenthesis:     ProcessLeftParenthesis;
    Token_SystemFunction:      Process_SystemFunction;
    Token_Mem:                 Process_Mem;
    Token_At:                  Process_At;
    else Error (VariableIdentifierExpected);
  end;
end;
This procedure expects variable reference. If Function variable reference has parameters or it is dereferenced the function is called before it is dereferenced. Variable reference location must be memory.
Procedure TExpression.ExpectVariableReference;
begin
  ExpectVariableIdentifier;
  Repeat
    If TypeDefPtr^.BaseType = btFunctionProcedure then
      begin
        If PProcedureTypeDefinition (TypeDefPtr)^.ResultTypeOffset.TypeOffset = 0 then Exit;
        If (Token <> Token_LeftParenthesis) and (Token <> Token_Caret) then Exit;
        CallProcedure;
        SetExpressionToFunctionResult;
      end;
  until not ProcessQualifiers;
  If Location <> elMemory then Error (InvalidVariableReference);
end;
This procedure expects constant expression. If any code was generated then this is not constant expression.
Procedure TExpression.ExpectConstantExpression;
Var SavedIntermediateCodeUsedSize: Word;
begin
  SavedIntermediateCodeUsedSize := SymbolTable [stIntermediateCode].UsedSize;
  CalculateExpression;
  If SavedIntermediateCodeUsedSize <> SymbolTable [stIntermediateCode].UsedSize then Error (CannotEvaluateThisExpression);
  If Location << elConstant then Error (CannotEvaluateThisExpression);
  If TypeDefPtr^.BaseType >= btInteger then
    TypeDefPtr := PointerFromOffsets (POrdinalTypeDefinition (TypeDefPtr)^.OrdinalType);
end;
This procedure expects file variable and pushes its pointer to memory.
Procedure TExpression.ExpectFileVariableAndPushPointerToMemory;
begin
  ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  Case TypeDefPtr^.BaseType of
    btFile, btText: Exit;
    else Error (FileVariableExpected);
  end;
end;
This procedure expects variable reference which is not read-only except procedure or function.
Procedure TExpression.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
begin
  ExpectVariableReference;
  If (Location = elProcedure) or (efReadOnly in LocationData.Flags) then Error (InvalidVariableReference);
end;
This procedure expects assignable variable except procedure or function and pushes its pointer to memory.
Procedure TExpression.ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
begin
  ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  ConvertToPointerAndPush;
end;
This procedure expects variable reference except procedure or function.
Procedure TExpression.ExpectVariableReferenceExceptProcedureOrFunction;
begin
  ExpectVariableReference;
  If Location = elProcedure then Error (InvalidVariableReference);
end;
This expects string expression, stores it to the code and pushes pointer to it.
Procedure TExpression.ExpectAndStoreStringAndPushPointer;
begin
  ExpectStringExpression;
  StoreStringConstantToCodeBlock;
  If IsExpressionInOverlaidCode then CopyStringToStackFrame ($0100);
  ConvertToPointerAndPush;
end;
This procedure expects string expression, stores it to the code and pushes pointer to it. If extended syntax is enabled it also converts
Char
, zero-based character array and
PChar
.
Procedure TExpression.ExpectAndStoreStringCompatibleExpressionAndPushPointer;
begin
  If ExtendedSyntax in StatementCompilerSwitches then
    begin
      CalculateExpression;
      ConvertCharToString;
      ConvertZeroBasedCharacterArrayToPChar;
      CopyCharArrayCompatibleWithStringToStackFrameAsString;
      If TypeDefPtr^.BaseType = btString then
        begin
          StoreStringConstantToCodeBlock;
          If IsExpressionInOverlaidCode then CopyStringToStackFrame ($0100);
          ConvertToPointerAndPush;
        end else If TypeDefPtr = Ptr (SystemUnitSegment, PChar_TypeOffset) then PushExpression
                   else Error (StringExpressionExpected);
    end else ExpectAndStoreStringAndPushPointer;
end;
This function expects integer constant and returns its value.
Function ExpectIntegerConstant: LongInt;
Var TempExpression: TExpression;
begin
  TempExpression.ExpectConstantExpression;
  If TempExpression.TypeDefPtr^.BaseType <> btInteger then Error (IntegerConstantExpected);
  ExpectIntegerConstant := TempExpression.Value.LongInt;
end;
This procedure processes expression and converts boolean jumps to value in
AL
.
Procedure TExpression.CalculateExpression;
begin
  ProcessExpression;
  ConvertBooleanJumpToByteValue;
end;
This procedure calculates expression with specified type. If expected base type is btProcedure it creates pointer to procedure (if expression is not nil or procedure variable) otherwise it calls CalculateExpression. If expected type is array and expression is compatible string it is converted to array.
Procedure TExpression.CalculateExpressionWithType (ExpectedType: PTypeDefinition);
Var AL: Byte;
    BX: Word;
    StringLen: Byte;
begin
  If ExpectedType^.BaseType = btFunctionProcedure  then
    begin
      If CheckAndGetNextToken (Token_NIL) then
        begin
          TypeDefPtr := ExpectedType;
          IntermediateCodeOffset := 0;
          Location := elConstant;
          DataType := itLongInt;
          UsedRegisters := [];
          Value.LongInt := 0;
          Value.W14 := 0;
          Value.W16 := 0;
          Exit;
        end;
      ExpectVariableReference;
      If Location = elProcedure then
        begin
          ExpectedType := TypeDefPtr;
          AL := Byte (Ptr (Seg (ExpectedType^), Ofs (ExpectedType^) - 10)^);
          If (AL and $16 <> 0) or (AL and $01 = 0) then Error (InvalidProcedureOrFunctionReference);
          If Word (Ptr (Seg (ExpectedType^), Ofs (ExpectedType^) - 6)^) <> 0 then Error (InvalidProcedureOrFunctionReference);
          BX := Word (Ptr (Seg (ExpectedType^), Ofs (ExpectedType^) - 8)^);
          Location := elPointerToMemory;
          LocationData.Flags := [efRelocatable];
          Value.Word := 0;
          Value.W12 := BX;
          Value.W14 := Seg (ExpectedType^);
          DataType := itLongInt;
          EndIntermediateCodeSubroutine;
        end;
      Exit;
    end;
  If PArrayTypeDefinition (ExpectedType)^.IsCharacterArrayCompatibleWithString (StringLen) then
    begin
      CalculateExpression;                                                 { BUG: Never executed ???!!! }
      If IsConstantString (ExpectedType^.Size, StringLen) then TypeDefPtr := ExpectedType;
    end else CalculateExpression;
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
Procedure TExpression.ExpectStringExpression;
begin
  CalculateExpression;
  ConvertCharToString;
  CopyCharArrayCompatibleWithStringToStackFrameAsString;
  If TypeDefPtr^.BaseType <> btString then Error (StringExpressionExpected);
end;
```

### Code Block 2

```pascal
Procedure TExpression.ExpectStringVariable;
begin
  ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  If TypeDefPtr^.BaseType <> btString then Error (StringVariableExpected);
end;
```

### Code Block 3

```pascal
Procedure TExpression.ExpectIntegerOrFloatingPointExpression;
begin
  CalculateExpression;
  If not (TypeDefPtr^.BaseType in [btInteger, btReal, btExtended]) then Error (IntegerOrRealExpressionExpected);
end;
```

### Code Block 4

```pascal
Procedure TExpression.ExpectIntegerOrFloatingPointExpressionInParentheses;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  ExpectIntegerOrFloatingPointExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
```

### Code Block 5

```pascal
Procedure TExpression.ExpectIntegerExpression;
begin
  CalculateExpression;
  If TypeDefPtr^.BaseType <> btInteger then Error (IntegerExpressionExpected);
end;
```

### Code Block 6

```pascal
Procedure TExpression.ExpectIntegerExpressionInParentheses;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  ExpectIntegerExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
```

### Code Block 7

```pascal
Procedure TExpression.Expect16BitIntegerVariable;
begin
  ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  With TypeDefPtr^ do If (BaseType <> btInteger) or (Size <> 2) then Error (IntegerVariableExpected);
end;
```

### Code Block 8

```pascal
Procedure TExpression.ExpectOrdinalExpression;
begin
  CalculateExpression;
  ConvertToBooleanByte;
  If TypeDefPtr^.BaseType < btInteger then Error (OrdinalExpressionExpected);
end;
```

### Code Block 9

```pascal
Procedure TExpression.ExpectOrdinalExpressionInParentheses;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  ExpectOrdinalExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
```

### Code Block 10

```pascal
Procedure TExpression.ExpectBooleanExpression;
begin
  ProcessExpression;
  If TypeDefPtr^.BaseType <> btBoolean then Error (BooleanExpressionExpected);
  If Location = elConstant then Exit;
  ConvertExpressionToBooleanJump;
  Calculate;
  GenerateCodeForNearJump (Value.LastJumpToFalse, LocationData.JumpIfTrueOpCode xor 1);
  GenerateLabelAndSetJumpsToIt (Value.LastJumpToTrue);
  EndIntermediateCodeSubroutine;
end;
```

### Code Block 11

```pascal
Procedure TExpression.ExpectPointerExpression;
begin
  CalculateExpression;
  If TypeDefPtr^.BaseType <> btPointer then Error (PointerExpressionExpected);
end;
```

### Code Block 12

```pascal
Procedure TExpression.ExpectPointerVariable;
begin
  ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  If TypeDefPtr^.BaseType <> btPointer then Error (PointerVariableExpected);
end;
```

### Code Block 13

```pascal
Procedure TExpression.ExpectVariableIdentifier;
begin
  CheckForDeclaredIdentifier;
  Case Token of
    Token_VariableIdentifier:  LoadVariable;
    Token_ProcedureIdentifier: ProcessProcedureIdentifier;
    Token_TypeIdentifier,
    Token_STRING,
    Token_FILE:                ProcessValueTypecastOrMethodCall;
    Token_INHERITED:           Process_INHERITED;
    Token_LeftParenthesis:     ProcessLeftParenthesis;
    Token_SystemFunction:      Process_SystemFunction;
    Token_Mem:                 Process_Mem;
    Token_At:                  Process_At;
    else Error (VariableIdentifierExpected);
  end;
end;
```

### Code Block 14

```pascal
Procedure TExpression.ExpectVariableReference;
begin
  ExpectVariableIdentifier;
  Repeat
    If TypeDefPtr^.BaseType = btFunctionProcedure then
      begin
        If PProcedureTypeDefinition (TypeDefPtr)^.ResultTypeOffset.TypeOffset = 0 then Exit;
        If (Token <> Token_LeftParenthesis) and (Token <> Token_Caret) then Exit;
        CallProcedure;
        SetExpressionToFunctionResult;
      end;
  until not ProcessQualifiers;
  If Location <> elMemory then Error (InvalidVariableReference);
end;
```

### Code Block 15

```pascal
Procedure TExpression.ExpectConstantExpression;
Var SavedIntermediateCodeUsedSize: Word;
begin
  SavedIntermediateCodeUsedSize := SymbolTable [stIntermediateCode].UsedSize;
  CalculateExpression;
  If SavedIntermediateCodeUsedSize <> SymbolTable [stIntermediateCode].UsedSize then Error (CannotEvaluateThisExpression);
  If Location << elConstant then Error (CannotEvaluateThisExpression);
  If TypeDefPtr^.BaseType >= btInteger then
    TypeDefPtr := PointerFromOffsets (POrdinalTypeDefinition (TypeDefPtr)^.OrdinalType);
end;
```

### Code Block 16

```pascal
Procedure TExpression.ExpectFileVariableAndPushPointerToMemory;
begin
  ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  Case TypeDefPtr^.BaseType of
    btFile, btText: Exit;
    else Error (FileVariableExpected);
  end;
end;
```

### Code Block 17

```pascal
Procedure TExpression.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
begin
  ExpectVariableReference;
  If (Location = elProcedure) or (efReadOnly in LocationData.Flags) then Error (InvalidVariableReference);
end;
```

### Code Block 18

```pascal
Procedure TExpression.ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
begin
  ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  ConvertToPointerAndPush;
end;
```

### Code Block 19

```pascal
Procedure TExpression.ExpectVariableReferenceExceptProcedureOrFunction;
begin
  ExpectVariableReference;
  If Location = elProcedure then Error (InvalidVariableReference);
end;
```

### Code Block 20

```pascal
Procedure TExpression.ExpectAndStoreStringAndPushPointer;
begin
  ExpectStringExpression;
  StoreStringConstantToCodeBlock;
  If IsExpressionInOverlaidCode then CopyStringToStackFrame ($0100);
  ConvertToPointerAndPush;
end;

```

### Code Block 21

```pascal
Procedure TExpression.ExpectAndStoreStringCompatibleExpressionAndPushPointer;
begin
  If ExtendedSyntax in StatementCompilerSwitches then
    begin
      CalculateExpression;
      ConvertCharToString;
      ConvertZeroBasedCharacterArrayToPChar;
      CopyCharArrayCompatibleWithStringToStackFrameAsString;
      If TypeDefPtr^.BaseType = btString then
        begin
          StoreStringConstantToCodeBlock;
          If IsExpressionInOverlaidCode then CopyStringToStackFrame ($0100);
          ConvertToPointerAndPush;
        end else If TypeDefPtr = Ptr (SystemUnitSegment, PChar_TypeOffset) then PushExpression
                   else Error (StringExpressionExpected);
    end else ExpectAndStoreStringAndPushPointer;
end;
```

### Code Block 22

```pascal
Function ExpectIntegerConstant: LongInt;
Var TempExpression: TExpression;
begin
  TempExpression.ExpectConstantExpression;
  If TempExpression.TypeDefPtr^.BaseType <> btInteger then Error (IntegerConstantExpected);
  ExpectIntegerConstant := TempExpression.Value.LongInt;
end;
```

### Code Block 23

```pascal
Procedure TExpression.CalculateExpression;
begin
  ProcessExpression;
  ConvertBooleanJumpToByteValue;
end;
```

### Code Block 24

```pascal
Procedure TExpression.CalculateExpressionWithType (ExpectedType: PTypeDefinition);
Var AL: Byte;
    BX: Word;
    StringLen: Byte;
begin
  If ExpectedType^.BaseType = btFunctionProcedure  then
    begin
      If CheckAndGetNextToken (Token_NIL) then
        begin
          TypeDefPtr := ExpectedType;
          IntermediateCodeOffset := 0;
          Location := elConstant;
          DataType := itLongInt;
          UsedRegisters := [];
          Value.LongInt := 0;
          Value.W14 := 0;
          Value.W16 := 0;
          Exit;
        end;
      ExpectVariableReference;
      If Location = elProcedure then
        begin
          ExpectedType := TypeDefPtr;
          AL := Byte (Ptr (Seg (ExpectedType^), Ofs (ExpectedType^) - 10)^);
          If (AL and $16 <> 0) or (AL and $01 = 0) then Error (InvalidProcedureOrFunctionReference);
          If Word (Ptr (Seg (ExpectedType^), Ofs (ExpectedType^) - 6)^) <> 0 then Error (InvalidProcedureOrFunctionReference);
          BX := Word (Ptr (Seg (ExpectedType^), Ofs (ExpectedType^) - 8)^);
          Location := elPointerToMemory;
          LocationData.Flags := [efRelocatable];
          Value.Word := 0;
          Value.W12 := BX;
          Value.W14 := Seg (ExpectedType^);
          DataType := itLongInt;
          EndIntermediateCodeSubroutine;
        end;
      Exit;
    end;
  If PArrayTypeDefinition (ExpectedType)^.IsCharacterArrayCompatibleWithString (StringLen) then
    begin
      CalculateExpression;                                                 { BUG: Never executed ???!!! }
      If IsConstantString (ExpectedType^.Size, StringLen) then TypeDefPtr := ExpectedType;
    end else CalculateExpression;
end;
```

### Code Block 25

```pascal
Procedure TExpression.ExpectStringExpression;
begin
  CalculateExpression;
  ConvertCharToString;
  CopyCharArrayCompatibleWithStringToStackFrameAsString;
  If TypeDefPtr^.BaseType <> btString then Error (StringExpressionExpected);
end;
```

### Code Block 26

```pascal
Procedure TExpression.ExpectStringVariable;
begin
  ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  If TypeDefPtr^.BaseType <> btString then Error (StringVariableExpected);
end;
```

### Code Block 27

```pascal
Procedure TExpression.ExpectIntegerOrFloatingPointExpression;
begin
  CalculateExpression;
  If not (TypeDefPtr^.BaseType in [btInteger, btReal, btExtended]) then Error (IntegerOrRealExpressionExpected);
end;
```

### Code Block 28

```pascal
Procedure TExpression.ExpectIntegerOrFloatingPointExpressionInParentheses;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  ExpectIntegerOrFloatingPointExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
```

### Code Block 29

```pascal
Procedure TExpression.ExpectIntegerExpression;
begin
  CalculateExpression;
  If TypeDefPtr^.BaseType <> btInteger then Error (IntegerExpressionExpected);
end;
```

### Code Block 30

```pascal
Procedure TExpression.ExpectIntegerExpressionInParentheses;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  ExpectIntegerExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
```

### Code Block 31

```pascal
Procedure TExpression.Expect16BitIntegerVariable;
begin
  ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  With TypeDefPtr^ do If (BaseType <> btInteger) or (Size <> 2) then Error (IntegerVariableExpected);
end;
```

### Code Block 32

```pascal
Procedure TExpression.ExpectOrdinalExpression;
begin
  CalculateExpression;
  ConvertToBooleanByte;
  If TypeDefPtr^.BaseType < btInteger then Error (OrdinalExpressionExpected);
end;
```

### Code Block 33

```pascal
Procedure TExpression.ExpectOrdinalExpressionInParentheses;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  ExpectOrdinalExpression;
  ExpectTokenAndGetNext (Token_RightParenthesis);
end;
```

### Code Block 34

```pascal
Procedure TExpression.ExpectBooleanExpression;
begin
  ProcessExpression;
  If TypeDefPtr^.BaseType <> btBoolean then Error (BooleanExpressionExpected);
  If Location = elConstant then Exit;
  ConvertExpressionToBooleanJump;
  Calculate;
  GenerateCodeForNearJump (Value.LastJumpToFalse, LocationData.JumpIfTrueOpCode xor 1);
  GenerateLabelAndSetJumpsToIt (Value.LastJumpToTrue);
  EndIntermediateCodeSubroutine;
end;
```

### Code Block 35

```pascal
Procedure TExpression.ExpectPointerExpression;
begin
  CalculateExpression;
  If TypeDefPtr^.BaseType <> btPointer then Error (PointerExpressionExpected);
end;
```

### Code Block 36

```pascal
Procedure TExpression.ExpectPointerVariable;
begin
  ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  If TypeDefPtr^.BaseType <> btPointer then Error (PointerVariableExpected);
end;
```

### Code Block 37

```pascal
Procedure TExpression.ExpectVariableIdentifier;
begin
  CheckForDeclaredIdentifier;
  Case Token of
    Token_VariableIdentifier:  LoadVariable;
    Token_ProcedureIdentifier: ProcessProcedureIdentifier;
    Token_TypeIdentifier,
    Token_STRING,
    Token_FILE:                ProcessValueTypecastOrMethodCall;
    Token_INHERITED:           Process_INHERITED;
    Token_LeftParenthesis:     ProcessLeftParenthesis;
    Token_SystemFunction:      Process_SystemFunction;
    Token_Mem:                 Process_Mem;
    Token_At:                  Process_At;
    else Error (VariableIdentifierExpected);
  end;
end;
```

### Code Block 38

```pascal
Procedure TExpression.ExpectVariableReference;
begin
  ExpectVariableIdentifier;
  Repeat
    If TypeDefPtr^.BaseType = btFunctionProcedure then
      begin
        If PProcedureTypeDefinition (TypeDefPtr)^.ResultTypeOffset.TypeOffset = 0 then Exit;
        If (Token <> Token_LeftParenthesis) and (Token <> Token_Caret) then Exit;
        CallProcedure;
        SetExpressionToFunctionResult;
      end;
  until not ProcessQualifiers;
  If Location <> elMemory then Error (InvalidVariableReference);
end;
```

### Code Block 39

```pascal
Procedure TExpression.ExpectConstantExpression;
Var SavedIntermediateCodeUsedSize: Word;
begin
  SavedIntermediateCodeUsedSize := SymbolTable [stIntermediateCode].UsedSize;
  CalculateExpression;
  If SavedIntermediateCodeUsedSize <> SymbolTable [stIntermediateCode].UsedSize then Error (CannotEvaluateThisExpression);
  If Location << elConstant then Error (CannotEvaluateThisExpression);
  If TypeDefPtr^.BaseType >= btInteger then
    TypeDefPtr := PointerFromOffsets (POrdinalTypeDefinition (TypeDefPtr)^.OrdinalType);
end;
```

### Code Block 40

```pascal
Procedure TExpression.ExpectFileVariableAndPushPointerToMemory;
begin
  ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
  Case TypeDefPtr^.BaseType of
    btFile, btText: Exit;
    else Error (FileVariableExpected);
  end;
end;
```

### Code Block 41

```pascal
Procedure TExpression.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
begin
  ExpectVariableReference;
  If (Location = elProcedure) or (efReadOnly in LocationData.Flags) then Error (InvalidVariableReference);
end;
```

### Code Block 42

```pascal
Procedure TExpression.ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
begin
  ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  ConvertToPointerAndPush;
end;
```

### Code Block 43

```pascal
Procedure TExpression.ExpectVariableReferenceExceptProcedureOrFunction;
begin
  ExpectVariableReference;
  If Location = elProcedure then Error (InvalidVariableReference);
end;
```

### Code Block 44

```pascal
Procedure TExpression.ExpectAndStoreStringAndPushPointer;
begin
  ExpectStringExpression;
  StoreStringConstantToCodeBlock;
  If IsExpressionInOverlaidCode then CopyStringToStackFrame ($0100);
  ConvertToPointerAndPush;
end;

```

### Code Block 45

```pascal
Procedure TExpression.ExpectAndStoreStringCompatibleExpressionAndPushPointer;
begin
  If ExtendedSyntax in StatementCompilerSwitches then
    begin
      CalculateExpression;
      ConvertCharToString;
      ConvertZeroBasedCharacterArrayToPChar;
      CopyCharArrayCompatibleWithStringToStackFrameAsString;
      If TypeDefPtr^.BaseType = btString then
        begin
          StoreStringConstantToCodeBlock;
          If IsExpressionInOverlaidCode then CopyStringToStackFrame ($0100);
          ConvertToPointerAndPush;
        end else If TypeDefPtr = Ptr (SystemUnitSegment, PChar_TypeOffset) then PushExpression
                   else Error (StringExpressionExpected);
    end else ExpectAndStoreStringAndPushPointer;
end;
```

### Code Block 46

```pascal
Function ExpectIntegerConstant: LongInt;
Var TempExpression: TExpression;
begin
  TempExpression.ExpectConstantExpression;
  If TempExpression.TypeDefPtr^.BaseType <> btInteger then Error (IntegerConstantExpected);
  ExpectIntegerConstant := TempExpression.Value.LongInt;
end;
```

### Code Block 47

```pascal
Procedure TExpression.CalculateExpression;
begin
  ProcessExpression;
  ConvertBooleanJumpToByteValue;
end;
```

### Code Block 48

```pascal
Procedure TExpression.CalculateExpressionWithType (ExpectedType: PTypeDefinition);
Var AL: Byte;
    BX: Word;
    StringLen: Byte;
begin
  If ExpectedType^.BaseType = btFunctionProcedure  then
    begin
      If CheckAndGetNextToken (Token_NIL) then
        begin
          TypeDefPtr := ExpectedType;
          IntermediateCodeOffset := 0;
          Location := elConstant;
          DataType := itLongInt;
          UsedRegisters := [];
          Value.LongInt := 0;
          Value.W14 := 0;
          Value.W16 := 0;
          Exit;
        end;
      ExpectVariableReference;
      If Location = elProcedure then
        begin
          ExpectedType := TypeDefPtr;
          AL := Byte (Ptr (Seg (ExpectedType^), Ofs (ExpectedType^) - 10)^);
          If (AL and $16 <> 0) or (AL and $01 = 0) then Error (InvalidProcedureOrFunctionReference);
          If Word (Ptr (Seg (ExpectedType^), Ofs (ExpectedType^) - 6)^) <> 0 then Error (InvalidProcedureOrFunctionReference);
          BX := Word (Ptr (Seg (ExpectedType^), Ofs (ExpectedType^) - 8)^);
          Location := elPointerToMemory;
          LocationData.Flags := [efRelocatable];
          Value.Word := 0;
          Value.W12 := BX;
          Value.W14 := Seg (ExpectedType^);
          DataType := itLongInt;
          EndIntermediateCodeSubroutine;
        end;
      Exit;
    end;
  If PArrayTypeDefinition (ExpectedType)^.IsCharacterArrayCompatibleWithString (StringLen) then
    begin
      CalculateExpression;                                                 { BUG: Never executed ???!!! }
      If IsConstantString (ExpectedType^.Size, StringLen) then TypeDefPtr := ExpectedType;
    end else CalculateExpression;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/expecting-different-expressions
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-expressions
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

