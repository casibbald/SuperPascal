# Processing Mathematical Operations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-mathematical-operations

---

## Table of Contents

- Processing Mathematical Operations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Mathematical Operations
Turbo Pascal generates code for all mathemathical operations with two operands by the following procedure. The input of this procedure are
left and right expression
and the operation.
Type TCalcOperation = (
      Calc_Add,                 { 00 }
      Calc_Subtract,            { 01 }
      Calc_Multiply,            { 02 }
      Calc_DIV,                 { 03 }
      Calc_MOD,                 { 04 }
      Calc_SHL,                 { 05 }
      Calc_SHR,                 { 06 }
      Calc_AND,                 { 07 }
      Calc_OR,                  { 08 }
      Calc_XOR,                 { 09 }
      Calc_IsEqual,             { 0A }
      Calc_IsNotEqual,          { 0B }
      Calc_IsGreater,           { 0C }
      Calc_IsLower,             { 0D }
      Calc_IsGreaterOrEqual,    { 0E }
      Calc_IsLowerOrEqual,      { 0F }
      Calc_Divide,              { 10 }
      Calc_IN);                 { 11 }
Table
AllowedOperations
defines which operations are allowed. Conditional jump opcodes define which jump code will be generated for comparison operations. The procedure first checks for
PChar operations
, then it copies
string-compatible character arrays
to
stack frame
, next some type adjustments are made,
type compatibility
is checked and finally the methematical operation is performed. If both operands are constant then no code needs to be generated and special procedures are used. Finally, left expression holds the result (including the code) of the mathematical operation.
Procedure CalculateOperation (Operation: TCalcOperation; Var LeftExpression, RightExpression: TExpression);
Const AllowedOperations: Array [Calc_Add..Calc_Divide] of TBaseTypeSet = (
{  +  } [               btChar,            btInteger, btReal, btExtended, btString,            btSet],
{  -  } [                                  btInteger, btReal, btExtended,           btPointer, btSet],
{  *  } [                                  btInteger, btReal, btExtended,                      btSet],
{ DIV } [                                  btInteger],
{ MOD } [                                  btInteger],
{ SHL } [                                  btInteger],
{ SHR } [                                  btInteger],
{ AND } [                       btBoolean, btInteger],
{ OR  } [                       btBoolean, btInteger],
{ XOR } [                       btBoolean, btInteger],
{  =  } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer, btSet],
{  <> } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer, btSet],
{  >  } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer],
{  <  } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer],
{  >= } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer, btSet],
{  <= } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer, btSet],
{  /  } [                                  btInteger, btReal, btExtended]);

  UnsignedConditionalJumpOpCode: Array [Calc_IsEqual..Calc_IsLowerOrEqual] of Byte = (
    JE,
    JNE,
    JA,
    JB,
    JNB,
    JBE);

  SignedConditionalJumpOpCode: Array [Calc_IsEqual..Calc_IsLowerOrEqual] of Byte = (
    JE,
    JNE,
    JG,
    JL,
    JNL,
    JLE);

begin
  If ExtendedSyntax in ModuleCompilerSwitches then If CharacterPointerOperations then Exit;
  LeftExpression.CopyCharArrayCompatibleWithStringToStackFrameAsString;
  RightExpression.CopyCharArrayCompatibleWithStringToStackFrameAsString;

  If LeftExpression.TypeDefPtr^.BaseType = RightExpression.TypeDefPtr^.BaseType then
    begin
      If (LeftExpression.TypeDefPtr^.BaseType = btInteger) and (Operation = Calc_Divide) then ConvertToRealOrExtended else
        If (LeftExpression.TypeDefPtr^.BaseType = btChar) and (Operation = Calc_Add) then
          begin
            LeftExpression.ConvertCharToString;
            RightExpression.ConvertCharToString;
          end;
    end else begin
               LeftExpression.ConvertCharToString;
               RightExpression.ConvertCharToString;
               ConvertToRealOrExtended;
             end;

  LeftExpression.CheckTypeCompatibility (RightExpression.TypeDefPtr);
  If not (RightExpression.TypeDefPtr^.BaseType in AllowedOperations [Operation]) then
    Error (OperandTypesDoNotMatchOperator);
  If (LeftExpression.Location = elConstant) and (RightExpression.Location = elConstant) then
    Case LeftExpression.TypeDefPtr^.BaseType of
      btSet:         ConstantSetOperations;
      btPointer:     ConstantPointerOperations;
      btString:      ConstantStringOperations;
      btExtended:    ConstantExtendedOperations;
      btReal:        GenerateCodeForRealOperations;
      btInteger:     ConstantIntegerOperations;
      btBoolean:     ConstantBooleanOperations;
      btChar:        ConstantIntegerOperations;
      btEnumeration: ConstantIntegerOperations;
    end else Case LeftExpression.TypeDefPtr^.BaseType of
               btSet:         GenerateCodeForSetOperations;
               btPointer:     GenerateCodeForPointerOperations;
               btString:      GenerateCodeForStringOperations;
               btExtended:    GenerateCodeForExtendedOperations;
               btReal:        GenerateCodeForRealOperations;
               btInteger:     GenerateCodeForIntegerOperations;
               btBoolean:     GenerateCodeForBooleanOperations;
               btChar:        GenerateCodeForIntegerOperations;
               btEnumeration: GenerateCodeForIntegerOperations;
             end;
end;
If both expressions are constant or 80×87 instructions are enabled then both expressions are
converted to Extended
, otherwise they are
converted to Real
.
Function ConvertToRealOrExtended: Boolean;
  begin
    If (LeftExpression.Location = elConstant) and (RightExpression.Location = elConstant) then
      ConvertIntegerOrRealToExtended else If Instructions80x87 in StatementCompilerSwitches then
        ConvertIntegerOrRealToExtended else ConvertIntegerOrExtendedToReal;
  end;

  Procedure ConvertIntegerOrRealToExtended;
  begin
    LeftExpression.ConvertIntegerToExtended;
    LeftExpression.ConvertRealToExtended;
    RightExpression.ConvertIntegerToExtended;
    RightExpression.ConvertRealToExtended;
  end;

  Procedure ConvertIntegerOrExtendedToReal;
  begin
    LeftExpression.ConvertIntegerToReal;
    LeftExpression.ConvertExtendedToReal;
    RightExpression.ConvertIntegerToReal;
    RightExpression.ConvertExtendedToReal;
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
Type TCalcOperation = (
      Calc_Add,                 { 00 }
      Calc_Subtract,            { 01 }
      Calc_Multiply,            { 02 }
      Calc_DIV,                 { 03 }
      Calc_MOD,                 { 04 }
      Calc_SHL,                 { 05 }
      Calc_SHR,                 { 06 }
      Calc_AND,                 { 07 }
      Calc_OR,                  { 08 }
      Calc_XOR,                 { 09 }
      Calc_IsEqual,             { 0A }
      Calc_IsNotEqual,          { 0B }
      Calc_IsGreater,           { 0C }
      Calc_IsLower,             { 0D }
      Calc_IsGreaterOrEqual,    { 0E }
      Calc_IsLowerOrEqual,      { 0F }
      Calc_Divide,              { 10 }
      Calc_IN);                 { 11 }
```

### Code Block 2

```pascal
Procedure CalculateOperation (Operation: TCalcOperation; Var LeftExpression, RightExpression: TExpression);
Const AllowedOperations: Array [Calc_Add..Calc_Divide] of TBaseTypeSet = (
{  +  } [               btChar,            btInteger, btReal, btExtended, btString,            btSet],
{  -  } [                                  btInteger, btReal, btExtended,           btPointer, btSet],
{  *  } [                                  btInteger, btReal, btExtended,                      btSet],
{ DIV } [                                  btInteger],
{ MOD } [                                  btInteger],
{ SHL } [                                  btInteger],
{ SHR } [                                  btInteger],
{ AND } [                       btBoolean, btInteger],
{ OR  } [                       btBoolean, btInteger],
{ XOR } [                       btBoolean, btInteger],
{  =  } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer, btSet],
{  <> } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer, btSet],
{  >  } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer],
{  <  } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer],
{  >= } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer, btSet],
{  <= } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer, btSet],
{  /  } [                                  btInteger, btReal, btExtended]);

  UnsignedConditionalJumpOpCode: Array [Calc_IsEqual..Calc_IsLowerOrEqual] of Byte = (
    JE,
    JNE,
    JA,
    JB,
    JNB,
    JBE);

  SignedConditionalJumpOpCode: Array [Calc_IsEqual..Calc_IsLowerOrEqual] of Byte = (
    JE,
    JNE,
    JG,
    JL,
    JNL,
    JLE);

begin
  If ExtendedSyntax in ModuleCompilerSwitches then If CharacterPointerOperations then Exit;
  LeftExpression.CopyCharArrayCompatibleWithStringToStackFrameAsString;
  RightExpression.CopyCharArrayCompatibleWithStringToStackFrameAsString;

  If LeftExpression.TypeDefPtr^.BaseType = RightExpression.TypeDefPtr^.BaseType then
    begin
      If (LeftExpression.TypeDefPtr^.BaseType = btInteger) and (Operation = Calc_Divide) then ConvertToRealOrExtended else
        If (LeftExpression.TypeDefPtr^.BaseType = btChar) and (Operation = Calc_Add) then
          begin
            LeftExpression.ConvertCharToString;
            RightExpression.ConvertCharToString;
          end;
    end else begin
               LeftExpression.ConvertCharToString;
               RightExpression.ConvertCharToString;
               ConvertToRealOrExtended;
             end;

  LeftExpression.CheckTypeCompatibility (RightExpression.TypeDefPtr);
  If not (RightExpression.TypeDefPtr^.BaseType in AllowedOperations [Operation]) then
    Error (OperandTypesDoNotMatchOperator);
  If (LeftExpression.Location = elConstant) and (RightExpression.Location = elConstant) then
    Case LeftExpression.TypeDefPtr^.BaseType of
      btSet:         ConstantSetOperations;
      btPointer:     ConstantPointerOperations;
      btString:      ConstantStringOperations;
      btExtended:    ConstantExtendedOperations;
      btReal:        GenerateCodeForRealOperations;
      btInteger:     ConstantIntegerOperations;
      btBoolean:     ConstantBooleanOperations;
      btChar:        ConstantIntegerOperations;
      btEnumeration: ConstantIntegerOperations;
    end else Case LeftExpression.TypeDefPtr^.BaseType of
               btSet:         GenerateCodeForSetOperations;
               btPointer:     GenerateCodeForPointerOperations;
               btString:      GenerateCodeForStringOperations;
               btExtended:    GenerateCodeForExtendedOperations;
               btReal:        GenerateCodeForRealOperations;
               btInteger:     GenerateCodeForIntegerOperations;
               btBoolean:     GenerateCodeForBooleanOperations;
               btChar:        GenerateCodeForIntegerOperations;
               btEnumeration: GenerateCodeForIntegerOperations;
             end;
end;
```

### Code Block 3

```pascal
  Function ConvertToRealOrExtended: Boolean;
  begin
    If (LeftExpression.Location = elConstant) and (RightExpression.Location = elConstant) then
      ConvertIntegerOrRealToExtended else If Instructions80x87 in StatementCompilerSwitches then
        ConvertIntegerOrRealToExtended else ConvertIntegerOrExtendedToReal;
  end;

  Procedure ConvertIntegerOrRealToExtended;
  begin
    LeftExpression.ConvertIntegerToExtended;
    LeftExpression.ConvertRealToExtended;
    RightExpression.ConvertIntegerToExtended;
    RightExpression.ConvertRealToExtended;
  end;

  Procedure ConvertIntegerOrExtendedToReal;
  begin
    LeftExpression.ConvertIntegerToReal;
    LeftExpression.ConvertExtendedToReal;
    RightExpression.ConvertIntegerToReal;
    RightExpression.ConvertExtendedToReal;
  end;
```

### Code Block 4

```pascal
Type TCalcOperation = (
      Calc_Add,                 { 00 }
      Calc_Subtract,            { 01 }
      Calc_Multiply,            { 02 }
      Calc_DIV,                 { 03 }
      Calc_MOD,                 { 04 }
      Calc_SHL,                 { 05 }
      Calc_SHR,                 { 06 }
      Calc_AND,                 { 07 }
      Calc_OR,                  { 08 }
      Calc_XOR,                 { 09 }
      Calc_IsEqual,             { 0A }
      Calc_IsNotEqual,          { 0B }
      Calc_IsGreater,           { 0C }
      Calc_IsLower,             { 0D }
      Calc_IsGreaterOrEqual,    { 0E }
      Calc_IsLowerOrEqual,      { 0F }
      Calc_Divide,              { 10 }
      Calc_IN);                 { 11 }
```

### Code Block 5

```pascal
Procedure CalculateOperation (Operation: TCalcOperation; Var LeftExpression, RightExpression: TExpression);
Const AllowedOperations: Array [Calc_Add..Calc_Divide] of TBaseTypeSet = (
{  +  } [               btChar,            btInteger, btReal, btExtended, btString,            btSet],
{  -  } [                                  btInteger, btReal, btExtended,           btPointer, btSet],
{  *  } [                                  btInteger, btReal, btExtended,                      btSet],
{ DIV } [                                  btInteger],
{ MOD } [                                  btInteger],
{ SHL } [                                  btInteger],
{ SHR } [                                  btInteger],
{ AND } [                       btBoolean, btInteger],
{ OR  } [                       btBoolean, btInteger],
{ XOR } [                       btBoolean, btInteger],
{  =  } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer, btSet],
{  <> } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer, btSet],
{  >  } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer],
{  <  } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer],
{  >= } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer, btSet],
{  <= } [btEnumeration, btChar, btBoolean, btInteger, btReal, btExtended, btString, btPointer, btSet],
{  /  } [                                  btInteger, btReal, btExtended]);

  UnsignedConditionalJumpOpCode: Array [Calc_IsEqual..Calc_IsLowerOrEqual] of Byte = (
    JE,
    JNE,
    JA,
    JB,
    JNB,
    JBE);

  SignedConditionalJumpOpCode: Array [Calc_IsEqual..Calc_IsLowerOrEqual] of Byte = (
    JE,
    JNE,
    JG,
    JL,
    JNL,
    JLE);

begin
  If ExtendedSyntax in ModuleCompilerSwitches then If CharacterPointerOperations then Exit;
  LeftExpression.CopyCharArrayCompatibleWithStringToStackFrameAsString;
  RightExpression.CopyCharArrayCompatibleWithStringToStackFrameAsString;

  If LeftExpression.TypeDefPtr^.BaseType = RightExpression.TypeDefPtr^.BaseType then
    begin
      If (LeftExpression.TypeDefPtr^.BaseType = btInteger) and (Operation = Calc_Divide) then ConvertToRealOrExtended else
        If (LeftExpression.TypeDefPtr^.BaseType = btChar) and (Operation = Calc_Add) then
          begin
            LeftExpression.ConvertCharToString;
            RightExpression.ConvertCharToString;
          end;
    end else begin
               LeftExpression.ConvertCharToString;
               RightExpression.ConvertCharToString;
               ConvertToRealOrExtended;
             end;

  LeftExpression.CheckTypeCompatibility (RightExpression.TypeDefPtr);
  If not (RightExpression.TypeDefPtr^.BaseType in AllowedOperations [Operation]) then
    Error (OperandTypesDoNotMatchOperator);
  If (LeftExpression.Location = elConstant) and (RightExpression.Location = elConstant) then
    Case LeftExpression.TypeDefPtr^.BaseType of
      btSet:         ConstantSetOperations;
      btPointer:     ConstantPointerOperations;
      btString:      ConstantStringOperations;
      btExtended:    ConstantExtendedOperations;
      btReal:        GenerateCodeForRealOperations;
      btInteger:     ConstantIntegerOperations;
      btBoolean:     ConstantBooleanOperations;
      btChar:        ConstantIntegerOperations;
      btEnumeration: ConstantIntegerOperations;
    end else Case LeftExpression.TypeDefPtr^.BaseType of
               btSet:         GenerateCodeForSetOperations;
               btPointer:     GenerateCodeForPointerOperations;
               btString:      GenerateCodeForStringOperations;
               btExtended:    GenerateCodeForExtendedOperations;
               btReal:        GenerateCodeForRealOperations;
               btInteger:     GenerateCodeForIntegerOperations;
               btBoolean:     GenerateCodeForBooleanOperations;
               btChar:        GenerateCodeForIntegerOperations;
               btEnumeration: GenerateCodeForIntegerOperations;
             end;
end;
```

### Code Block 6

```pascal
  Function ConvertToRealOrExtended: Boolean;
  begin
    If (LeftExpression.Location = elConstant) and (RightExpression.Location = elConstant) then
      ConvertIntegerOrRealToExtended else If Instructions80x87 in StatementCompilerSwitches then
        ConvertIntegerOrRealToExtended else ConvertIntegerOrExtendedToReal;
  end;

  Procedure ConvertIntegerOrRealToExtended;
  begin
    LeftExpression.ConvertIntegerToExtended;
    LeftExpression.ConvertRealToExtended;
    RightExpression.ConvertIntegerToExtended;
    RightExpression.ConvertRealToExtended;
  end;

  Procedure ConvertIntegerOrExtendedToReal;
  begin
    LeftExpression.ConvertIntegerToReal;
    LeftExpression.ConvertExtendedToReal;
    RightExpression.ConvertIntegerToReal;
    RightExpression.ConvertExtendedToReal;
  end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/character-pointer-operations
- https://turbopascal.org/checking-type-compatibility
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/converting-number-types
- https://turbopascal.org/creating-and-destroying-stack-frame
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-and-converting-strings
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

