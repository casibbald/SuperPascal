# String Operations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/string-operations

---

## Table of Contents

- String Operations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
String Operations
Constant strings are added in the temporary buffer, while all non-constant string operations are performed in
stack frame
.
Procedure ConstantStringOperations;
  Var BoolResult: LongBool;
      LeftString, RightString: PString;
      CommonLength, NewStringOffset: Word;
  begin
    LeftString  := Ptr (DSeg, LeftExpression.Value.Word);
    RightString := Ptr (DSeg, RightExpression.Value.Word);
    Case Operation of
      Calc_Add: begin
                  If (Ofs (LeftString^) + Length (LeftString^) + 1 = Ofs (RightString^)) and
                     (Ofs (RightString^) + Length (RightString^) + 1 = TempBufferFreePositionOffset) then
                       TempBufferFreePositionOffset := Ofs (LeftString^);
                  CommonLength := Length (LeftString^) + Length (RightString^);
                  If CommonLength > $FF then CommonLength := $FF;
                  NewStringOffset := GetTempBufferOffset (CommonLength + 1);
                  PString (Ptr (DSeg, NewStringOffset))^ := LeftString^ + RightString^;
                  LeftExpression.Value.Word := NewStringOffset;
                end;
      else begin
             Case Operation of
               Calc_IsEqual:          BoolResult := LeftString^  = RightString^;
               Calc_IsNotEqual:       BoolResult := LeftString^ <> RightString^;
               Calc_IsGreater:        BoolResult := LeftString^  > RightString^;
               Calc_IsLower:          BoolResult := LeftString^  > RightString^;
               Calc_IsGreaterOrEqual: BoolResult := LeftString^ >= RightString^;
               Calc_IsLowerOrEqual:   BoolResult := LeftString^ <= RightString^;
             end;
             SetLeftExpressionToConstantBooleanValue (BoolResult);
           end;
    end;
  end;

  Procedure GenerateCodeForStringOperations;
  Label NoEmptyString;
  begin
    Case Operation of
      Calc_ADD: begin
                  LeftExpression.CopyStringToStackFrame ($0100);
                  RightExpression.StoreStringConstantToCodeBlock;
                  ConvertToPointerAndPushBothExpressions (LeftExpression, RightExpression);
                  CalculateBothExpressions (LeftExpression, RightExpression);
                  GenerateInstruction_CALL_FAR (SysProc_SConcat);
                  With LeftExpression do
                    begin
                      LeftExpression.EndIntermediateCodeSubroutine;
                      UsedRegisters := [urBX, urDX, urCX, urAX];
                    end;
                end;
      else begin
             If (RightExpression.Location <> elConstant) or (PString (Ptr (DSeg, RightExpression.Value.Word))^ <> '') then
               begin
                 If (LeftExpression.Location <> elConstant) then GoTo NoEmptyString;
                 ExchangeLeftAndRightExpression;
                 If PString (Ptr (DSeg, RightExpression.Value.Word))^ <> '' then GoTo NoEmptyString;
               end;
             LeftExpression.PositionToStringLength;
             With RightExpression do
               begin
                 DataType := [];
                 Value.LongInt := 0;
               end;
             GenerateCodeForIntegerComparisons;
             LeftExpression.EndIntermediateCodeSubroutine;
             Exit;
 NoEmptyString:

             LeftExpression.StoreStringConstantToCodeBlock;
             RightExpression.StoreStringConstantToCodeBlock;
             ConvertToPointerAndPushBothExpressions (LeftExpression, RightExpression);
             CalculateBothExpressions (LeftExpression, RightExpression);
             GenerateInstruction_CALL_FAR (SysProc_SCompare);
             With LeftExpression do
               begin
                 LeftExpression.EndIntermediateCodeSubroutine;
                 UsedRegisters := [urBX, urDX, urCX, urAX];
               end;
             LeftExpression.SetExpressionToBooleanJump (UnsignedConditionalJumpOpCode [Operation]);
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
  Procedure ConstantStringOperations;
  Var BoolResult: LongBool;
      LeftString, RightString: PString;
      CommonLength, NewStringOffset: Word;
  begin
    LeftString  := Ptr (DSeg, LeftExpression.Value.Word);
    RightString := Ptr (DSeg, RightExpression.Value.Word);
    Case Operation of
      Calc_Add: begin
                  If (Ofs (LeftString^) + Length (LeftString^) + 1 = Ofs (RightString^)) and
                     (Ofs (RightString^) + Length (RightString^) + 1 = TempBufferFreePositionOffset) then
                       TempBufferFreePositionOffset := Ofs (LeftString^);
                  CommonLength := Length (LeftString^) + Length (RightString^);
                  If CommonLength > $FF then CommonLength := $FF;
                  NewStringOffset := GetTempBufferOffset (CommonLength + 1);
                  PString (Ptr (DSeg, NewStringOffset))^ := LeftString^ + RightString^;
                  LeftExpression.Value.Word := NewStringOffset;
                end;
      else begin
             Case Operation of
               Calc_IsEqual:          BoolResult := LeftString^  = RightString^;
               Calc_IsNotEqual:       BoolResult := LeftString^ <> RightString^;
               Calc_IsGreater:        BoolResult := LeftString^  > RightString^;
               Calc_IsLower:          BoolResult := LeftString^  > RightString^;
               Calc_IsGreaterOrEqual: BoolResult := LeftString^ >= RightString^;
               Calc_IsLowerOrEqual:   BoolResult := LeftString^ <= RightString^;
             end;
             SetLeftExpressionToConstantBooleanValue (BoolResult);
           end;
    end;
  end;

  Procedure GenerateCodeForStringOperations;
  Label NoEmptyString;
  begin
    Case Operation of
      Calc_ADD: begin
                  LeftExpression.CopyStringToStackFrame ($0100);
                  RightExpression.StoreStringConstantToCodeBlock;
                  ConvertToPointerAndPushBothExpressions (LeftExpression, RightExpression);
                  CalculateBothExpressions (LeftExpression, RightExpression);
                  GenerateInstruction_CALL_FAR (SysProc_SConcat);
                  With LeftExpression do
                    begin
                      LeftExpression.EndIntermediateCodeSubroutine;
                      UsedRegisters := [urBX, urDX, urCX, urAX];
                    end;
                end;
      else begin
             If (RightExpression.Location <> elConstant) or (PString (Ptr (DSeg, RightExpression.Value.Word))^ <> '') then
               begin
                 If (LeftExpression.Location <> elConstant) then GoTo NoEmptyString;
                 ExchangeLeftAndRightExpression;
                 If PString (Ptr (DSeg, RightExpression.Value.Word))^ <> '' then GoTo NoEmptyString;
               end;
             LeftExpression.PositionToStringLength;
             With RightExpression do
               begin
                 DataType := [];
                 Value.LongInt := 0;
               end;
             GenerateCodeForIntegerComparisons;
             LeftExpression.EndIntermediateCodeSubroutine;
             Exit;
 NoEmptyString:

             LeftExpression.StoreStringConstantToCodeBlock;
             RightExpression.StoreStringConstantToCodeBlock;
             ConvertToPointerAndPushBothExpressions (LeftExpression, RightExpression);
             CalculateBothExpressions (LeftExpression, RightExpression);
             GenerateInstruction_CALL_FAR (SysProc_SCompare);
             With LeftExpression do
               begin
                 LeftExpression.EndIntermediateCodeSubroutine;
                 UsedRegisters := [urBX, urDX, urCX, urAX];
               end;
             LeftExpression.SetExpressionToBooleanJump (UnsignedConditionalJumpOpCode [Operation]);
           end;
    end;
  end;
```

### Code Block 2

```pascal
  Procedure ConstantStringOperations;
  Var BoolResult: LongBool;
      LeftString, RightString: PString;
      CommonLength, NewStringOffset: Word;
  begin
    LeftString  := Ptr (DSeg, LeftExpression.Value.Word);
    RightString := Ptr (DSeg, RightExpression.Value.Word);
    Case Operation of
      Calc_Add: begin
                  If (Ofs (LeftString^) + Length (LeftString^) + 1 = Ofs (RightString^)) and
                     (Ofs (RightString^) + Length (RightString^) + 1 = TempBufferFreePositionOffset) then
                       TempBufferFreePositionOffset := Ofs (LeftString^);
                  CommonLength := Length (LeftString^) + Length (RightString^);
                  If CommonLength > $FF then CommonLength := $FF;
                  NewStringOffset := GetTempBufferOffset (CommonLength + 1);
                  PString (Ptr (DSeg, NewStringOffset))^ := LeftString^ + RightString^;
                  LeftExpression.Value.Word := NewStringOffset;
                end;
      else begin
             Case Operation of
               Calc_IsEqual:          BoolResult := LeftString^  = RightString^;
               Calc_IsNotEqual:       BoolResult := LeftString^ <> RightString^;
               Calc_IsGreater:        BoolResult := LeftString^  > RightString^;
               Calc_IsLower:          BoolResult := LeftString^  > RightString^;
               Calc_IsGreaterOrEqual: BoolResult := LeftString^ >= RightString^;
               Calc_IsLowerOrEqual:   BoolResult := LeftString^ <= RightString^;
             end;
             SetLeftExpressionToConstantBooleanValue (BoolResult);
           end;
    end;
  end;

  Procedure GenerateCodeForStringOperations;
  Label NoEmptyString;
  begin
    Case Operation of
      Calc_ADD: begin
                  LeftExpression.CopyStringToStackFrame ($0100);
                  RightExpression.StoreStringConstantToCodeBlock;
                  ConvertToPointerAndPushBothExpressions (LeftExpression, RightExpression);
                  CalculateBothExpressions (LeftExpression, RightExpression);
                  GenerateInstruction_CALL_FAR (SysProc_SConcat);
                  With LeftExpression do
                    begin
                      LeftExpression.EndIntermediateCodeSubroutine;
                      UsedRegisters := [urBX, urDX, urCX, urAX];
                    end;
                end;
      else begin
             If (RightExpression.Location <> elConstant) or (PString (Ptr (DSeg, RightExpression.Value.Word))^ <> '') then
               begin
                 If (LeftExpression.Location <> elConstant) then GoTo NoEmptyString;
                 ExchangeLeftAndRightExpression;
                 If PString (Ptr (DSeg, RightExpression.Value.Word))^ <> '' then GoTo NoEmptyString;
               end;
             LeftExpression.PositionToStringLength;
             With RightExpression do
               begin
                 DataType := [];
                 Value.LongInt := 0;
               end;
             GenerateCodeForIntegerComparisons;
             LeftExpression.EndIntermediateCodeSubroutine;
             Exit;
 NoEmptyString:

             LeftExpression.StoreStringConstantToCodeBlock;
             RightExpression.StoreStringConstantToCodeBlock;
             ConvertToPointerAndPushBothExpressions (LeftExpression, RightExpression);
             CalculateBothExpressions (LeftExpression, RightExpression);
             GenerateInstruction_CALL_FAR (SysProc_SCompare);
             With LeftExpression do
               begin
                 LeftExpression.EndIntermediateCodeSubroutine;
                 UsedRegisters := [urBX, urDX, urCX, urAX];
               end;
             LeftExpression.SetExpressionToBooleanJump (UnsignedConditionalJumpOpCode [Operation]);
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
- https://turbopascal.org/creating-and-destroying-stack-frame
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/string-operations
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

