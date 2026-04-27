# System Procedures Read and ReadLn - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-procedures-read-and-readln

---

## Table of Contents

- System Procedures Read and ReadLn

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Procedures Read and ReadLn
This procedure processes system procedures
Read
and
ReadLn
.
Function Proc_Read_Readln: Word; Far;
Var Parameter, Expression: TExpression;
    TextFileCode, ReadProc: Word;
    BP_AC: TIntermediateCodeOffsets;
Label ProcessParameter, CheckParameters;
begin
  TextFileCode := 0;
  BP_AC.NumberOfStatements := 0;
  If CheckAndGetNextToken (Token_LeftParenthesis) then
    begin
      Parameter.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
      Case Parameter.TypeDefPtr^.BaseType of
        btFile: begin
                  Parameter.ConvertToPointerAndPush;
                  Proc_Read_Readln :=
                    ProcessReadWriteTypedParameters (PFileTypeDefinition (Parameter.TypeDefPtr),
                                                     Parameter.IntermediateCodeOffset, SysProc_ReadFile);
                  Exit
                end;
        btText: begin
                  Parameter.ConvertToPointerAndPush;
                  TextFileCode := Parameter.IntermediateCodeOffset;
                  GoTo CheckParameters;
                end;
      end;
      GoTo ProcessParameter;

CheckParameters:

      While CheckAndGetNextToken (Token_Comma) do
        begin
          Parameter.ExpectAssignableVariableReferenceExceptProcedureOrFunction;

ProcessParameter:

          With Parameter do If (ExtendedSyntax in StatementCompilerSwitches) and
                              PArrayTypeDefinition (TypeDefPtr)^.IsZeroBasedCharacterArray then
              begin
                PushArrayPointerAndHighestIndex;
                Calculate;
                GenerateInstruction_CALL_FAR (SysProc_ReadPChar);
                AddCallToIntermediateCodeSubroutine (BP_AC, EndSubroutine);
                Continue;
              end;
          With Expression do
            Case Parameter.ExpressionBaseType of
              btString: begin
                          Parameter.PushArrayPointerAndHighestIndex;
                          Parameter.Calculate;
                          GenerateInstruction_CALL_FAR (SysProc_ReadStr);
                          AddCallToIntermediateCodeSubroutine (BP_AC, EndSubroutine);
                          Continue;
                        end;
              btExtended: begin
                            TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
                            Location := elStackFrame;
                            DataType := [fpExtended];
                            ReadProc := SysProc_ReadFloat;
                          end;
              btReal: begin
                        TypeDefPtr := Ptr (SystemUnitSegment, Real_TypeOffset);
                        Location := elRegister;
                        DataType := [fpDouble];
                        ReadProc := SysProc_ReadReal;
                      end;
              btInteger: begin
                           TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
                           Location := elRegister;
                           DataType := [it32Bit, it16Bit, itSigned];
                           LocationData.Register := rDX_AX;
                           ReadProc := SysProc_ReadInt;
                         end;
              btChar: begin
                        TypeDefPtr := Ptr (SystemUnitSegment, Char_TypeOffset);
                        Location := elRegister;
                        DataType := [itUnsigned];
                        LocationData.Register := rAX;
                        ReadProc := SysProc_ReadChar;
                      end;
              else Error (CannotReadOrWriteVariablesOfThisType);
            end;
          GenerateInstruction_CALL_FAR (ReadProc);
          With Expression do
            begin
              EndIntermediateCodeSubroutine;
              UsedRegisters := [urBX, urDX, urCX, urAX];
              AdjustExpressionToType (Parameter.TypeDefPtr);
              CheckRange (Parameter.TypeDefPtr);
            end;
          GenerateAssignmentIntermediateCode (Parameter, Expression);
          AddCallToIntermediateCodeSubroutine (BP_AC, Parameter.IntermediateCodeOffset);
        end;
      ExpectTokenAndGetNext (Token_RightParenthesis);
    end;
  PushPointerToTextRec (TextFileCode, $0000);
  Generate_icGoSub_ForEachSubroutine (BP_AC);
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_Read_Readln := EndSubroutine;
end;
This function processes typed parameters for
Read
and
Write
procedures, pushes them on stack and generates a call to compiler routine for each.
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
Function Proc_Read_Readln: Word; Far;
Var Parameter, Expression: TExpression;
    TextFileCode, ReadProc: Word;
    BP_AC: TIntermediateCodeOffsets;
Label ProcessParameter, CheckParameters;
begin
  TextFileCode := 0;
  BP_AC.NumberOfStatements := 0;
  If CheckAndGetNextToken (Token_LeftParenthesis) then
    begin
      Parameter.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
      Case Parameter.TypeDefPtr^.BaseType of
        btFile: begin
                  Parameter.ConvertToPointerAndPush;
                  Proc_Read_Readln :=
                    ProcessReadWriteTypedParameters (PFileTypeDefinition (Parameter.TypeDefPtr),
                                                     Parameter.IntermediateCodeOffset, SysProc_ReadFile);
                  Exit
                end;
        btText: begin
                  Parameter.ConvertToPointerAndPush;
                  TextFileCode := Parameter.IntermediateCodeOffset;
                  GoTo CheckParameters;
                end;
      end;
      GoTo ProcessParameter;

CheckParameters:

      While CheckAndGetNextToken (Token_Comma) do
        begin
          Parameter.ExpectAssignableVariableReferenceExceptProcedureOrFunction;

ProcessParameter:

          With Parameter do If (ExtendedSyntax in StatementCompilerSwitches) and
                              PArrayTypeDefinition (TypeDefPtr)^.IsZeroBasedCharacterArray then
              begin
                PushArrayPointerAndHighestIndex;
                Calculate;
                GenerateInstruction_CALL_FAR (SysProc_ReadPChar);
                AddCallToIntermediateCodeSubroutine (BP_AC, EndSubroutine);
                Continue;
              end;
          With Expression do
            Case Parameter.ExpressionBaseType of
              btString: begin
                          Parameter.PushArrayPointerAndHighestIndex;
                          Parameter.Calculate;
                          GenerateInstruction_CALL_FAR (SysProc_ReadStr);
                          AddCallToIntermediateCodeSubroutine (BP_AC, EndSubroutine);
                          Continue;
                        end;
              btExtended: begin
                            TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
                            Location := elStackFrame;
                            DataType := [fpExtended];
                            ReadProc := SysProc_ReadFloat;
                          end;
              btReal: begin
                        TypeDefPtr := Ptr (SystemUnitSegment, Real_TypeOffset);
                        Location := elRegister;
                        DataType := [fpDouble];
                        ReadProc := SysProc_ReadReal;
                      end;
              btInteger: begin
                           TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
                           Location := elRegister;
                           DataType := [it32Bit, it16Bit, itSigned];
                           LocationData.Register := rDX_AX;
                           ReadProc := SysProc_ReadInt;
                         end;
              btChar: begin
                        TypeDefPtr := Ptr (SystemUnitSegment, Char_TypeOffset);
                        Location := elRegister;
                        DataType := [itUnsigned];
                        LocationData.Register := rAX;
                        ReadProc := SysProc_ReadChar;
                      end;
              else Error (CannotReadOrWriteVariablesOfThisType);
            end;
          GenerateInstruction_CALL_FAR (ReadProc);
          With Expression do
            begin
              EndIntermediateCodeSubroutine;
              UsedRegisters := [urBX, urDX, urCX, urAX];
              AdjustExpressionToType (Parameter.TypeDefPtr);
              CheckRange (Parameter.TypeDefPtr);
            end;
          GenerateAssignmentIntermediateCode (Parameter, Expression);
          AddCallToIntermediateCodeSubroutine (BP_AC, Parameter.IntermediateCodeOffset);
        end;
      ExpectTokenAndGetNext (Token_RightParenthesis);
    end;
  PushPointerToTextRec (TextFileCode, $0000);
  Generate_icGoSub_ForEachSubroutine (BP_AC);
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_Read_Readln := EndSubroutine;
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
Function Proc_Read_Readln: Word; Far;
Var Parameter, Expression: TExpression;
    TextFileCode, ReadProc: Word;
    BP_AC: TIntermediateCodeOffsets;
Label ProcessParameter, CheckParameters;
begin
  TextFileCode := 0;
  BP_AC.NumberOfStatements := 0;
  If CheckAndGetNextToken (Token_LeftParenthesis) then
    begin
      Parameter.ExpectAssignableVariableReferenceExceptProcedureOrFunction;
      Case Parameter.TypeDefPtr^.BaseType of
        btFile: begin
                  Parameter.ConvertToPointerAndPush;
                  Proc_Read_Readln :=
                    ProcessReadWriteTypedParameters (PFileTypeDefinition (Parameter.TypeDefPtr),
                                                     Parameter.IntermediateCodeOffset, SysProc_ReadFile);
                  Exit
                end;
        btText: begin
                  Parameter.ConvertToPointerAndPush;
                  TextFileCode := Parameter.IntermediateCodeOffset;
                  GoTo CheckParameters;
                end;
      end;
      GoTo ProcessParameter;

CheckParameters:

      While CheckAndGetNextToken (Token_Comma) do
        begin
          Parameter.ExpectAssignableVariableReferenceExceptProcedureOrFunction;

ProcessParameter:

          With Parameter do If (ExtendedSyntax in StatementCompilerSwitches) and
                              PArrayTypeDefinition (TypeDefPtr)^.IsZeroBasedCharacterArray then
              begin
                PushArrayPointerAndHighestIndex;
                Calculate;
                GenerateInstruction_CALL_FAR (SysProc_ReadPChar);
                AddCallToIntermediateCodeSubroutine (BP_AC, EndSubroutine);
                Continue;
              end;
          With Expression do
            Case Parameter.ExpressionBaseType of
              btString: begin
                          Parameter.PushArrayPointerAndHighestIndex;
                          Parameter.Calculate;
                          GenerateInstruction_CALL_FAR (SysProc_ReadStr);
                          AddCallToIntermediateCodeSubroutine (BP_AC, EndSubroutine);
                          Continue;
                        end;
              btExtended: begin
                            TypeDefPtr := Ptr (SystemUnitSegment, Extended_TypeOffset);
                            Location := elStackFrame;
                            DataType := [fpExtended];
                            ReadProc := SysProc_ReadFloat;
                          end;
              btReal: begin
                        TypeDefPtr := Ptr (SystemUnitSegment, Real_TypeOffset);
                        Location := elRegister;
                        DataType := [fpDouble];
                        ReadProc := SysProc_ReadReal;
                      end;
              btInteger: begin
                           TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
                           Location := elRegister;
                           DataType := [it32Bit, it16Bit, itSigned];
                           LocationData.Register := rDX_AX;
                           ReadProc := SysProc_ReadInt;
                         end;
              btChar: begin
                        TypeDefPtr := Ptr (SystemUnitSegment, Char_TypeOffset);
                        Location := elRegister;
                        DataType := [itUnsigned];
                        LocationData.Register := rAX;
                        ReadProc := SysProc_ReadChar;
                      end;
              else Error (CannotReadOrWriteVariablesOfThisType);
            end;
          GenerateInstruction_CALL_FAR (ReadProc);
          With Expression do
            begin
              EndIntermediateCodeSubroutine;
              UsedRegisters := [urBX, urDX, urCX, urAX];
              AdjustExpressionToType (Parameter.TypeDefPtr);
              CheckRange (Parameter.TypeDefPtr);
            end;
          GenerateAssignmentIntermediateCode (Parameter, Expression);
          AddCallToIntermediateCodeSubroutine (BP_AC, Parameter.IntermediateCodeOffset);
        end;
      ExpectTokenAndGetNext (Token_RightParenthesis);
    end;
  PushPointerToTextRec (TextFileCode, $0000);
  Generate_icGoSub_ForEachSubroutine (BP_AC);
  GenerateInstruction_CALL_FAR (ProcParameter);
  GenerateIOErrorCheckingCode;
  Proc_Read_Readln := EndSubroutine;
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
- https://turbopascal.org/system-procedures-read-and-readln
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

