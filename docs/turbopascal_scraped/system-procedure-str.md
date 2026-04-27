# System Procedure Str - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-procedure-str

---

## Table of Contents

- System Procedure Str

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Procedure Str
This function processes system procedure
Str
.
Function Proc_Str: Word; Far;
Var NumericValue, FieldWidth, DecimalWidth, StringVariable: TExpression;
    StringProc, PCharProc: Word;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  With NumericValue do
    begin
      ExpectIntegerOrFloatingPointExpression;
      If TypeDefPtr^.BaseType = btInteger then
        begin
          CheckOrdinalRange (Ptr (SystemUnitSegment, LongInt_TypeOffset));
          PushExpression;
          PushParameterSpecifier (0, FieldWidth);
          DecimalWidth.IntermediateCodeOffset := 0;
          StringProc := SysProc_StrInt;
          PCharProc := SysProc_StrIntC;
        end else If Instructions80x87 in StatementCompilerSwitches then
                   begin
                     ConvertRealToExtended;
                     LoadExpressionToFPU;
                     PushParameterSpecifier (23, FieldWidth);
                     PushParameterSpecifier ($FFFF, DecimalWidth);
                     StringProc := SysProc_StrFloat;
                     PCharProc := SysProc_StrFloatC;
                   end else begin
                              ConvertExtendedToReal;
                              PushRealExpression;
                              PushParameterSpecifier (17, FieldWidth);
                              PushParameterSpecifier ($FFFF, DecimalWidth);
                              StringProc := SysProc_StrReal;
                              PCharProc := SysProc_StrRealC;
                            end;
    end;
  ExpectTokenAndGetNext (Token_Comma);
  StringVariable.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  If StringVariable.TypeDefPtr^.BaseType <> btString then
    begin
       If not (ExtendedSyntax in StatementCompilerSwitches) then Error (StringVariableExpected);
       If not PArrayTypeDefinition (StringVariable.TypeDefPtr)^.IsZeroBasedCharacterArray then Error (StringVariableExpected);
    end;
  StringVariable.PushArrayPointerAndHighestIndex;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (NumericValue.IntermediateCodeOffset);
  StoreCode_icGoSub (FieldWidth.IntermediateCodeOffset);
  StoreCode_icGoSub (DecimalWidth.IntermediateCodeOffset);
  StoreCode_icGoSub (StringVariable.IntermediateCodeOffset);
  Case StringVariable.TypeDefPtr^.BaseType of
    btString: GenerateInstruction_CALL_FAR (StringProc);
    else GenerateInstruction_CALL_FAR (PCharProc);
  end;
  Proc_Str := EndSubroutine;
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
Function Proc_Str: Word; Far;
Var NumericValue, FieldWidth, DecimalWidth, StringVariable: TExpression;
    StringProc, PCharProc: Word;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  With NumericValue do
    begin
      ExpectIntegerOrFloatingPointExpression;
      If TypeDefPtr^.BaseType = btInteger then
        begin
          CheckOrdinalRange (Ptr (SystemUnitSegment, LongInt_TypeOffset));
          PushExpression;
          PushParameterSpecifier (0, FieldWidth);
          DecimalWidth.IntermediateCodeOffset := 0;
          StringProc := SysProc_StrInt;
          PCharProc := SysProc_StrIntC;
        end else If Instructions80x87 in StatementCompilerSwitches then
                   begin
                     ConvertRealToExtended;
                     LoadExpressionToFPU;
                     PushParameterSpecifier (23, FieldWidth);
                     PushParameterSpecifier ($FFFF, DecimalWidth);
                     StringProc := SysProc_StrFloat;
                     PCharProc := SysProc_StrFloatC;
                   end else begin
                              ConvertExtendedToReal;
                              PushRealExpression;
                              PushParameterSpecifier (17, FieldWidth);
                              PushParameterSpecifier ($FFFF, DecimalWidth);
                              StringProc := SysProc_StrReal;
                              PCharProc := SysProc_StrRealC;
                            end;
    end;
  ExpectTokenAndGetNext (Token_Comma);
  StringVariable.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  If StringVariable.TypeDefPtr^.BaseType <> btString then
    begin
       If not (ExtendedSyntax in StatementCompilerSwitches) then Error (StringVariableExpected);
       If not PArrayTypeDefinition (StringVariable.TypeDefPtr)^.IsZeroBasedCharacterArray then Error (StringVariableExpected);
    end;
  StringVariable.PushArrayPointerAndHighestIndex;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (NumericValue.IntermediateCodeOffset);
  StoreCode_icGoSub (FieldWidth.IntermediateCodeOffset);
  StoreCode_icGoSub (DecimalWidth.IntermediateCodeOffset);
  StoreCode_icGoSub (StringVariable.IntermediateCodeOffset);
  Case StringVariable.TypeDefPtr^.BaseType of
    btString: GenerateInstruction_CALL_FAR (StringProc);
    else GenerateInstruction_CALL_FAR (PCharProc);
  end;
  Proc_Str := EndSubroutine;
end;
```

### Code Block 2

```pascal
Function Proc_Str: Word; Far;
Var NumericValue, FieldWidth, DecimalWidth, StringVariable: TExpression;
    StringProc, PCharProc: Word;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  With NumericValue do
    begin
      ExpectIntegerOrFloatingPointExpression;
      If TypeDefPtr^.BaseType = btInteger then
        begin
          CheckOrdinalRange (Ptr (SystemUnitSegment, LongInt_TypeOffset));
          PushExpression;
          PushParameterSpecifier (0, FieldWidth);
          DecimalWidth.IntermediateCodeOffset := 0;
          StringProc := SysProc_StrInt;
          PCharProc := SysProc_StrIntC;
        end else If Instructions80x87 in StatementCompilerSwitches then
                   begin
                     ConvertRealToExtended;
                     LoadExpressionToFPU;
                     PushParameterSpecifier (23, FieldWidth);
                     PushParameterSpecifier ($FFFF, DecimalWidth);
                     StringProc := SysProc_StrFloat;
                     PCharProc := SysProc_StrFloatC;
                   end else begin
                              ConvertExtendedToReal;
                              PushRealExpression;
                              PushParameterSpecifier (17, FieldWidth);
                              PushParameterSpecifier ($FFFF, DecimalWidth);
                              StringProc := SysProc_StrReal;
                              PCharProc := SysProc_StrRealC;
                            end;
    end;
  ExpectTokenAndGetNext (Token_Comma);
  StringVariable.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
  If StringVariable.TypeDefPtr^.BaseType <> btString then
    begin
       If not (ExtendedSyntax in StatementCompilerSwitches) then Error (StringVariableExpected);
       If not PArrayTypeDefinition (StringVariable.TypeDefPtr)^.IsZeroBasedCharacterArray then Error (StringVariableExpected);
    end;
  StringVariable.PushArrayPointerAndHighestIndex;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (NumericValue.IntermediateCodeOffset);
  StoreCode_icGoSub (FieldWidth.IntermediateCodeOffset);
  StoreCode_icGoSub (DecimalWidth.IntermediateCodeOffset);
  StoreCode_icGoSub (StringVariable.IntermediateCodeOffset);
  Case StringVariable.TypeDefPtr^.BaseType of
    btString: GenerateInstruction_CALL_FAR (StringProc);
    else GenerateInstruction_CALL_FAR (PCharProc);
  end;
  Proc_Str := EndSubroutine;
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
- https://turbopascal.org/system-procedure-str
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

