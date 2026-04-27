# System Procedures Write and WriteLn - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-procedures-write-and-writeln

---

## Table of Contents

- System Procedures Write and WriteLn

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Procedures Write and WriteLn
This function processes system procedures
Write
and
WriteLn
.
Function Proc_Write_Writeln: Word; Far;
Var Parameter, FieldWidth, NumOfDecimalPlaces: TExpression;
    TextFileCode, WriteProc: Word;
    BP_C0: TIntermediateCodeOffsets;
Label WriteParameter, ProcessParameter;
begin
  TextFileCode := 0;
  BP_C0.NumberOfStatements := 0;
  If CheckAndGetNextToken (Token_LeftParenthesis) then
    begin
      Parameter.CalculateExpression;
      Case Parameter.TypeDefPtr^.BaseType of
        btFile: begin
                  Parameter.ConvertToPointerAndPush;
                  Proc_Write_Writeln := ProcessReadWriteTypedParameters (PFileTypeDefinition (Parameter.TypeDefPtr),
                                                     Parameter.IntermediateCodeOffset, SysProc_WriteFile);
                  Exit;
                end;
        btText: begin
                  Parameter.ConvertToPointerAndPush;
                  TextFileCode := Parameter.IntermediateCodeOffset;
                end;
        else GoTo ProcessParameter;
      end;

      While CheckAndGetNextToken (Token_Comma) do
        begin
          Parameter.CalculateExpression;

ProcessParameter:

          NumOfDecimalPlaces.IntermediateCodeOffset := 0;
          If ExtendedSyntax in StatementCompilerSwitches then Parameter.ConvertZeroBasedCharacterArrayToPChar;
          Parameter.CopyCharArrayCompatibleWithStringToStackFrameAsString;
          With Parameter do If (ExtendedSyntax in StatementCompilerSwitches) and
            (TypeDefPtr = Ptr (SystemUnitSegment, PChar_TypeOffset)) then
              begin
                PushExpression;
                PushParameterSpecifier (0, FieldWidth);
                WriteProc := SysProc_WritePChar;
              end else
          Case TypeDefPtr^.BaseType of
            btString: begin
                        StoreStringConstantToCodeBlock;
                        If IsExpressionInOverlaidCode then CopyStringToStackFrame ($0100);
                        ConvertToPointerAndPush;
                        PushParameterSpecifier (0, FieldWidth);
                        WriteProc := SysProc_WriteStr;
                      end;
            btExtended,
            btReal: begin
                      If Instructions80x87 in StatementCompilerSwitches then
                        begin
                          ConvertRealToExtended;
                          LoadExpressionToFPU;
                          PushParameterSpecifier (23, FieldWidth);
                          PushParameterSpecifier ($FFFF, NumOfDecimalPlaces);
                          WriteProc := SysProc_WriteFloat;
                        end else
                          begin
                            ConvertExtendedToReal;
                            PushRealExpression;
                            PushParameterSpecifier (17, FieldWidth);
                            PushParameterSpecifier ($FFFF, NumOfDecimalPlaces);
                            WriteProc := SysProc_WriteReal
                          end;
                    end;
              btInteger: begin
                           CheckOrdinalRange (Ptr (SystemUnitSegment, LongInt_TypeOffset));
                           PushExpression;
                           PushParameterSpecifier (0, FieldWidth);
                           WriteProc := SysProc_WriteInt;
                         end;
              btBoolean: begin
                           ExtendInteger ([]);
                           PushExpression;
                           PushParameterSpecifier (0, FieldWidth);
                           WriteProc := SysProc_WriteBool;
                         end;
              btChar: begin
                        PushExpression;
                        PushParameterSpecifier (0, FieldWidth);
                        WriteProc := SysProc_WriteChar;
                      end;
            else Error (CannotReadOrWriteVariablesOfThisType);
          end;
          StoreCode_icGoSub (Parameter.IntermediateCodeOffset);
          StoreCode_icGoSub (FieldWidth.IntermediateCodeOffset);
          StoreCode_icGoSub (NumOfDecimalPlaces.IntermediateCodeOffset);
          GenerateInstruction_CALL_FAR (WriteProc);
          AddCallToIntermediateCodeSubroutine (BP_C0, EndSubroutine);
        end;
      ExpectTokenAndGetNext (Token_RightParenthesis);
    end;
  PushPointerToTextRec (TextFileCode, $0100);
  Generate_icGoSub_ForEachSubroutine (BP_C0);
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_Write_Writeln := EndSubroutine;
end;
This function processes typed parameters for
Read
and
Write
procedures, pushes them on stack and generates a call to a compiler routine for each.
Function ProcessReadWriteTypedParameters (FileTypeDefinition: PFileTypeDefinition;
                                          FileVarIntermediateCodeOffset, ParameterSysProc: Word): Word;
Var BaseFileTypeOffset: PTypeDefinition;
    IntermediateCodeOffsets: TIntermediateCodeOffsets;
    Parameter: TExpression;
begin
  Case ProcParameter of
    SysProc_ReadLine,
    SysProc_WriteLine: Error (InvalidFileType);
  end;
  If FileTypeDefinition^.BaseFileTypeOffset.TypeOffset = 0 then Error (InvalidFileType);
  BaseFileTypeOffset := PointerFromOffsets (FileTypeDefinition^.BaseFileTypeOffset);
  IntermediateCodeOffsets.NumberOfStatements := 0;
  ExpectTokenAndGetNext (Token_Comma);
  Repeat
    With Parameter do
      begin
        ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
        If TypeDefPtr <> BaseFileTypeOffset then Error (InvalidFileType);
        Calculate;
        GenerateInstruction_CALL_FAR (ParameterSysProc);
        AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
      end;
  until not CheckAndGetNextToken (Token_Comma);
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVarIntermediateCodeOffset);
  Generate_icGoSub_ForEachSubroutine (IntermediateCodeOffsets);
  GenerateInstruction_TwoBytes ($83, $C4);
  GenerateInstruction_Byte ($04);
  GenerateIOErrorCheckingCode;
  ProcessReadWriteTypedParameters := EndSubroutine;
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
Function Proc_Write_Writeln: Word; Far;
Var Parameter, FieldWidth, NumOfDecimalPlaces: TExpression;
    TextFileCode, WriteProc: Word;
    BP_C0: TIntermediateCodeOffsets;
Label WriteParameter, ProcessParameter;
begin
  TextFileCode := 0;
  BP_C0.NumberOfStatements := 0;
  If CheckAndGetNextToken (Token_LeftParenthesis) then
    begin
      Parameter.CalculateExpression;
      Case Parameter.TypeDefPtr^.BaseType of
        btFile: begin
                  Parameter.ConvertToPointerAndPush;
                  Proc_Write_Writeln := ProcessReadWriteTypedParameters (PFileTypeDefinition (Parameter.TypeDefPtr),
                                                     Parameter.IntermediateCodeOffset, SysProc_WriteFile);
                  Exit;
                end;
        btText: begin
                  Parameter.ConvertToPointerAndPush;
                  TextFileCode := Parameter.IntermediateCodeOffset;
                end;
        else GoTo ProcessParameter;
      end;

      While CheckAndGetNextToken (Token_Comma) do
        begin
          Parameter.CalculateExpression;

ProcessParameter:

          NumOfDecimalPlaces.IntermediateCodeOffset := 0;
          If ExtendedSyntax in StatementCompilerSwitches then Parameter.ConvertZeroBasedCharacterArrayToPChar;
          Parameter.CopyCharArrayCompatibleWithStringToStackFrameAsString;
          With Parameter do If (ExtendedSyntax in StatementCompilerSwitches) and
            (TypeDefPtr = Ptr (SystemUnitSegment, PChar_TypeOffset)) then
              begin
                PushExpression;
                PushParameterSpecifier (0, FieldWidth);
                WriteProc := SysProc_WritePChar;
              end else
          Case TypeDefPtr^.BaseType of
            btString: begin
                        StoreStringConstantToCodeBlock;
                        If IsExpressionInOverlaidCode then CopyStringToStackFrame ($0100);
                        ConvertToPointerAndPush;
                        PushParameterSpecifier (0, FieldWidth);
                        WriteProc := SysProc_WriteStr;
                      end;
            btExtended,
            btReal: begin
                      If Instructions80x87 in StatementCompilerSwitches then
                        begin
                          ConvertRealToExtended;
                          LoadExpressionToFPU;
                          PushParameterSpecifier (23, FieldWidth);
                          PushParameterSpecifier ($FFFF, NumOfDecimalPlaces);
                          WriteProc := SysProc_WriteFloat;
                        end else
                          begin
                            ConvertExtendedToReal;
                            PushRealExpression;
                            PushParameterSpecifier (17, FieldWidth);
                            PushParameterSpecifier ($FFFF, NumOfDecimalPlaces);
                            WriteProc := SysProc_WriteReal
                          end;
                    end;
              btInteger: begin
                           CheckOrdinalRange (Ptr (SystemUnitSegment, LongInt_TypeOffset));
                           PushExpression;
                           PushParameterSpecifier (0, FieldWidth);
                           WriteProc := SysProc_WriteInt;
                         end;
              btBoolean: begin
                           ExtendInteger ([]);
                           PushExpression;
                           PushParameterSpecifier (0, FieldWidth);
                           WriteProc := SysProc_WriteBool;
                         end;
              btChar: begin
                        PushExpression;
                        PushParameterSpecifier (0, FieldWidth);
                        WriteProc := SysProc_WriteChar;
                      end;
            else Error (CannotReadOrWriteVariablesOfThisType);
          end;
          StoreCode_icGoSub (Parameter.IntermediateCodeOffset);
          StoreCode_icGoSub (FieldWidth.IntermediateCodeOffset);
          StoreCode_icGoSub (NumOfDecimalPlaces.IntermediateCodeOffset);
          GenerateInstruction_CALL_FAR (WriteProc);
          AddCallToIntermediateCodeSubroutine (BP_C0, EndSubroutine);
        end;
      ExpectTokenAndGetNext (Token_RightParenthesis);
    end;
  PushPointerToTextRec (TextFileCode, $0100);
  Generate_icGoSub_ForEachSubroutine (BP_C0);
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_Write_Writeln := EndSubroutine;
end;
```

### Code Block 2

```pascal
Function ProcessReadWriteTypedParameters (FileTypeDefinition: PFileTypeDefinition;
                                          FileVarIntermediateCodeOffset, ParameterSysProc: Word): Word;
Var BaseFileTypeOffset: PTypeDefinition;
    IntermediateCodeOffsets: TIntermediateCodeOffsets;
    Parameter: TExpression;
begin
  Case ProcParameter of
    SysProc_ReadLine,
    SysProc_WriteLine: Error (InvalidFileType);
  end;
  If FileTypeDefinition^.BaseFileTypeOffset.TypeOffset = 0 then Error (InvalidFileType);
  BaseFileTypeOffset := PointerFromOffsets (FileTypeDefinition^.BaseFileTypeOffset);
  IntermediateCodeOffsets.NumberOfStatements := 0;
  ExpectTokenAndGetNext (Token_Comma);
  Repeat
    With Parameter do
      begin
        ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
        If TypeDefPtr <> BaseFileTypeOffset then Error (InvalidFileType);
        Calculate;
        GenerateInstruction_CALL_FAR (ParameterSysProc);
        AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
      end;
  until not CheckAndGetNextToken (Token_Comma);
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVarIntermediateCodeOffset);
  Generate_icGoSub_ForEachSubroutine (IntermediateCodeOffsets);
  GenerateInstruction_TwoBytes ($83, $C4);
  GenerateInstruction_Byte ($04);
  GenerateIOErrorCheckingCode;
  ProcessReadWriteTypedParameters := EndSubroutine;
end;
```

### Code Block 3

```pascal
Function Proc_Write_Writeln: Word; Far;
Var Parameter, FieldWidth, NumOfDecimalPlaces: TExpression;
    TextFileCode, WriteProc: Word;
    BP_C0: TIntermediateCodeOffsets;
Label WriteParameter, ProcessParameter;
begin
  TextFileCode := 0;
  BP_C0.NumberOfStatements := 0;
  If CheckAndGetNextToken (Token_LeftParenthesis) then
    begin
      Parameter.CalculateExpression;
      Case Parameter.TypeDefPtr^.BaseType of
        btFile: begin
                  Parameter.ConvertToPointerAndPush;
                  Proc_Write_Writeln := ProcessReadWriteTypedParameters (PFileTypeDefinition (Parameter.TypeDefPtr),
                                                     Parameter.IntermediateCodeOffset, SysProc_WriteFile);
                  Exit;
                end;
        btText: begin
                  Parameter.ConvertToPointerAndPush;
                  TextFileCode := Parameter.IntermediateCodeOffset;
                end;
        else GoTo ProcessParameter;
      end;

      While CheckAndGetNextToken (Token_Comma) do
        begin
          Parameter.CalculateExpression;

ProcessParameter:

          NumOfDecimalPlaces.IntermediateCodeOffset := 0;
          If ExtendedSyntax in StatementCompilerSwitches then Parameter.ConvertZeroBasedCharacterArrayToPChar;
          Parameter.CopyCharArrayCompatibleWithStringToStackFrameAsString;
          With Parameter do If (ExtendedSyntax in StatementCompilerSwitches) and
            (TypeDefPtr = Ptr (SystemUnitSegment, PChar_TypeOffset)) then
              begin
                PushExpression;
                PushParameterSpecifier (0, FieldWidth);
                WriteProc := SysProc_WritePChar;
              end else
          Case TypeDefPtr^.BaseType of
            btString: begin
                        StoreStringConstantToCodeBlock;
                        If IsExpressionInOverlaidCode then CopyStringToStackFrame ($0100);
                        ConvertToPointerAndPush;
                        PushParameterSpecifier (0, FieldWidth);
                        WriteProc := SysProc_WriteStr;
                      end;
            btExtended,
            btReal: begin
                      If Instructions80x87 in StatementCompilerSwitches then
                        begin
                          ConvertRealToExtended;
                          LoadExpressionToFPU;
                          PushParameterSpecifier (23, FieldWidth);
                          PushParameterSpecifier ($FFFF, NumOfDecimalPlaces);
                          WriteProc := SysProc_WriteFloat;
                        end else
                          begin
                            ConvertExtendedToReal;
                            PushRealExpression;
                            PushParameterSpecifier (17, FieldWidth);
                            PushParameterSpecifier ($FFFF, NumOfDecimalPlaces);
                            WriteProc := SysProc_WriteReal
                          end;
                    end;
              btInteger: begin
                           CheckOrdinalRange (Ptr (SystemUnitSegment, LongInt_TypeOffset));
                           PushExpression;
                           PushParameterSpecifier (0, FieldWidth);
                           WriteProc := SysProc_WriteInt;
                         end;
              btBoolean: begin
                           ExtendInteger ([]);
                           PushExpression;
                           PushParameterSpecifier (0, FieldWidth);
                           WriteProc := SysProc_WriteBool;
                         end;
              btChar: begin
                        PushExpression;
                        PushParameterSpecifier (0, FieldWidth);
                        WriteProc := SysProc_WriteChar;
                      end;
            else Error (CannotReadOrWriteVariablesOfThisType);
          end;
          StoreCode_icGoSub (Parameter.IntermediateCodeOffset);
          StoreCode_icGoSub (FieldWidth.IntermediateCodeOffset);
          StoreCode_icGoSub (NumOfDecimalPlaces.IntermediateCodeOffset);
          GenerateInstruction_CALL_FAR (WriteProc);
          AddCallToIntermediateCodeSubroutine (BP_C0, EndSubroutine);
        end;
      ExpectTokenAndGetNext (Token_RightParenthesis);
    end;
  PushPointerToTextRec (TextFileCode, $0100);
  Generate_icGoSub_ForEachSubroutine (BP_C0);
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_Write_Writeln := EndSubroutine;
end;
```

### Code Block 4

```pascal
Function ProcessReadWriteTypedParameters (FileTypeDefinition: PFileTypeDefinition;
                                          FileVarIntermediateCodeOffset, ParameterSysProc: Word): Word;
Var BaseFileTypeOffset: PTypeDefinition;
    IntermediateCodeOffsets: TIntermediateCodeOffsets;
    Parameter: TExpression;
begin
  Case ProcParameter of
    SysProc_ReadLine,
    SysProc_WriteLine: Error (InvalidFileType);
  end;
  If FileTypeDefinition^.BaseFileTypeOffset.TypeOffset = 0 then Error (InvalidFileType);
  BaseFileTypeOffset := PointerFromOffsets (FileTypeDefinition^.BaseFileTypeOffset);
  IntermediateCodeOffsets.NumberOfStatements := 0;
  ExpectTokenAndGetNext (Token_Comma);
  Repeat
    With Parameter do
      begin
        ExpectAssignableVariableReferenceExceptProcAndFuncAndPushPointerToMemory;
        If TypeDefPtr <> BaseFileTypeOffset then Error (InvalidFileType);
        Calculate;
        GenerateInstruction_CALL_FAR (ParameterSysProc);
        AddCallToIntermediateCodeSubroutine (IntermediateCodeOffsets, EndSubroutine);
      end;
  until not CheckAndGetNextToken (Token_Comma);
  ExpectTokenAndGetNext (Token_RightParenthesis);
  StoreCode_icGoSub (FileVarIntermediateCodeOffset);
  Generate_icGoSub_ForEachSubroutine (IntermediateCodeOffsets);
  GenerateInstruction_TwoBytes ($83, $C4);
  GenerateInstruction_Byte ($04);
  GenerateIOErrorCheckingCode;
  ProcessReadWriteTypedParameters := EndSubroutine;
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
- https://turbopascal.org/system-procedures
- https://turbopascal.org/system-procedures-write-and-writeln
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

