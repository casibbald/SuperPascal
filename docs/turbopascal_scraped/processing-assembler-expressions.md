# Processing Assembler Expressions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-assembler-expressions

---

## Table of Contents

- Processing Assembler Expressions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Assembler Expressions
Assembler expressions (instruction parameters) are processed similarly to
Pascal expressions
. The expression is divided into smaller parts according to the operator precedence. For assembler expressions Turbo Pascal uses few data structures and procedures.
TAsmExpression
contains fields that hold data for each expression (instruction parameter).
Type TRegisterType = (rt8BitRegister, rt16BitRegister, rtSegmentRegister, rt03, rtFPURegister);

     TMemoryReferenceRegisters =  (mrDI, mrSI, mrBP, mrBX, mr10, mr20, mrMemoryLocation, mr80);

     TMemoryReferenceRegistersSet = Set of TMemoryReferenceRegisters;

     PAsmValue = ^TAsmValue;
     TAsmValue = Record
                   Case Byte of
                     0: (LongInt: LongInt);
                     1: (Word, Word2: Word);
                     2: (Byte, Byte1, Byte2, Byte3: Byte);
                   end;

     PAsmExpression = ^TAsmExpression;
     TAsmExpression = Record
                        Value: TAsmValue;
                        AdditionalIdentifierData: Pointer;
                        RelocatableIdentifier: LongInt;
                        TypeSize: Word;
                        MemoryReferences: TMemoryReferenceRegistersSet;
                        SegmentOverrideOpcode: Byte;
                        RegisterType: TRegisterType;
                        Register: Byte;
                        Token: TAsmToken;
                        ReferenceFlags: Byte;
                      end;
Assembler instructions can have up to three comma-separated parameters.
Procedure ProcessInstructionExpresions;
begin
  ExpressionCounter := 0;
  ErrorPositionInstructionExpresion1 := AsmSourceErrorPosition;
  If CheckAndGetNextAsmToken (AsmToken_Semicolon) then Exit;
  LeftExpression := @AsmExpression1;
  ProcessAsmExpressionAndCheck;
  Inc (ExpressionCounter);
  If CheckAndGetNextAsmToken (AsmToken_Semicolon) then Exit;
  ExpectAsmTokenAndGetNext (AsmToken_Comma);
  ErrorPositionInstructionExpresion2 := AsmSourceErrorPosition;
  LeftExpression := @AsmExpression2;
  ProcessAsmExpressionAndCheck;
  Inc (ExpressionCounter);
  LeftExpression  := @AsmExpression1;
  RightExpression := @AsmExpression2;
  SavedRightExpressionTypeSize := RightExpression^.TypeSize;
  If RightExpression^.TypeSize <> 0 then
    begin
      If LeftExpression^.TypeSize = 0 then LeftExpression^.TypeSize := RightExpression^.TypeSize;
    end else RightExpression^.TypeSize := LeftExpression^.TypeSize;
  If CheckAndGetNextAsmToken (AsmToken_Semicolon) then Exit;
  ExpectAsmTokenAndGetNext (AsmToken_Comma);
  LeftExpression := @AsmExpression3;
  ProcessAsmExpressionAndCheck;
  Inc (ExpressionCounter);
  ExpectAsmTokenAndGetNext (AsmToken_Semicolon);
end;
This procedure processes assembler expression and checks for valid memory reference.
Procedure ProcessAsmExpressionAndCheck;
begin
  CheckAsmStack;
  ClearLeftExpression;
  ProcessAsmExpression;
  With LeftExpression^ do
    If (Token = AsmToken_MemoryReference) and not (mrMemoryLocation in MemoryReferences)
      then AsmError (MemoryReferenceExpected);
end;
Processing of assembler expressions:
SimpleTerm
SimpleTerm
OR
SimpleTerm
SimpleTerm
XOR
SimpleTerm
Procedure OR_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt or RightExpression^.Value.LongInt;
end;

Procedure XOR_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt xor RightExpression^.Value.LongInt;
end;

Procedure ProcessAsmExpression;
Const OR_XOR_Table:  TOperatorProc = (Token: AsmToken_OR;  Proc: OR_Proc);
      OR_XOR_Table1: TOperatorProc = (Token: AsmToken_XOR; Proc: XOR_Proc);
      OR_XOR_Table_End: Byte = 0;
Var ExpressionProc: TExpressionProc;
begin
  ProcessAsmTerm;
  While FindAsmTokenInTable (@OR_XOR_Table, ExpressionProc) do
    begin
      CheckConstantExpression;
      PushLeftAndCreateNewExpression (ExpressionProc);
      ProcessAsmTerm;
      CheckConstantExpression;
      MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
      ExpressionProc;
      Asm
        ADD   SP, 20
      end;
    end;
end;
Processing of factor elements in assembler expression:
SimpleAsmTerm
SimpleAsmTerm
AND
SimpleAsmTerm
Processing of simple assembler term for operators:
NOT
+
-
Procedure Plus_Proc; Far;
begin
  AddAsmExpressions;
  AsmString := '';
end;

Procedure Minus_Proc; Far;
begin
  SwapLeftAndRightExpression;
  CheckNoRelocatableIdentifier;
  SwapLeftAndRightExpression;
  RightExpression^.Value.LongInt := - RightExpression^.Value.LongInt;
  AddAsmExpressions;
  AsmString := '';
end;

Procedure ProcessAsmTerm;
Var ExpressionProc: TExpressionProc;

  Procedure Process_NOT_PLUS_MINUS;
  Const NOT_Table: TOperatorProc = (Token: AsmToken_NOT; Proc: nil);
        NOT_Table_End: Byte = 0;

        PlusMinus_Table:  TOperatorProc = (Token: AsmToken_Plus; Proc: Plus_Proc);
        PlusMinus_Table1: TOperatorProc = (Token: AsmToken_Minus; Proc: Minus_Proc);
        PlusMinus_Table_End: Byte = 0;
  Var ExpressionProc: TExpressionProc;
  begin
    CheckAsmStack;
    If FindAsmTokenInTable (@NOT_Table, ExpressionProc) then
      begin
        Process_NOT_PLUS_MINUS;
        CheckConstantExpression;
        LeftExpression^.Value.LongInt := not LeftExpression^.Value.LongInt;
        AsmString := '';
        Exit;
      end;
    ProcessAsmFactor;
    While FindAsmTokenInTable (@PlusMinus_Table, ExpressionProc) do
      begin
        PushLeftAndCreateNewExpression (ExpressionProc);
        ProcessAsmFactor;
        MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
        ExpressionProc;
        Asm
          ADD   SP, 20
        end;
      end;
  end;

begin
  Process_NOT_PLUS_MINUS;
  While CheckAndGetNextAsmToken (AsmToken_AND) do
    begin
      CheckConstantExpression;
      PushLeftAndCreateNewExpression (ExpressionProc);
      Process_NOT_PLUS_MINUS;
      CheckConstantExpression;
      MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
      LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt and RightExpression^.Value.LongInt;
      Asm
        ADD   SP, 20
      end;
    end;
end;
This procedure reports error if expression is not constant.
Procedure CheckConstantExpression;
begin
  If (LeftExpression^.Token <> AsmToken_Constant) or (LeftExpression^.RelocatableIdentifier <> 0) then
    AsmError (ConstantExpected);
end;
Processing of factors in assembler expression for the following operations:
*
/
DIV
MOD
SHR
SHL
Procedure Asterisk_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt * RightExpression^.Value.LongInt;
end;

Procedure Slash_Proc; Far;
begin
  If RightExpression^.Value.LongInt = 0 then AsmError (DivisionByZero);
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt div RightExpression^.Value.LongInt;
end;

Procedure MOD_Proc; Far;
begin
  If RightExpression^.Value.LongInt = 0 then AsmError (DivisionByZero);
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt mod RightExpression^.Value.LongInt;
end;

Procedure SHR_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt shr RightExpression^.Value.LongInt;
end;

Procedure SHL_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt shl RightExpression^.Value.LongInt;
end;

Procedure ProcessAsmFactor;

Const MultDivShift_Table:  TOperatorProc = (Token: AsmToken_Asterisk; Proc: Asterisk_Proc);
      MultDivShift_Table1: TOperatorProc = (Token: AsmToken_Slash;    Proc: Slash_Proc);
      MultDivShift_Table2: TOperatorProc = (Token: AsmToken_MOD;      Proc: MOD_Proc);
      MultDivShift_Table3: TOperatorProc = (Token: AsmToken_SHR;      Proc: SHR_Proc);
      MultDivShift_Table4: TOperatorProc = (Token: AsmToken_SHL;      Proc: SHL_Proc);
      MultDivShift_Table_End: Byte = 0;
Var ExpressionProc: TExpressionProc;
begin
  ProcessAsmFactorElement;
  While FindAsmTokenInTable (@MultDivShift_Table, ExpressionProc) do
    begin
      CheckConstantExpression;
      PushLeftAndCreateNewExpression (ExpressionProc);
      ProcessAsmFactorElement;
      CheckConstantExpression;
      MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
      ExpressionProc;
      Asm
        ADD   SP, 20
      end;
    end;
end;
Processing of factor element in assembler expression:
QualifiedExpressionElement
QualifiedExpressionElement
PTR
AsmFactorElement
QualifiedExpressionElement
:
AsmFactorElement
Processing of qualified basic expression element:
Unary
+
, –
HIGH
,
LOW
Basic expression element
Qualifiers
.
,
(
,
[
Procedure OFFSET_Proc; Far;
begin
  With LeftExpression^ do
    begin
      ReferenceFlags := 1;
      Token := AsmToken_Constant;
      MemoryReferences := [];
      SegmentOverrideOpcode := 0;
      Value.Word2 := 0;
      TypeSize := 0;
      If RelocatableIdentifier = 0 then ReferenceFlags := 0;
    end;
end;

Procedure SEG_Proc; Far;
begin
  With LeftExpression^ do
    begin
      ReferenceFlags := 2;
      Value.Word := LeftExpression^.Value.Word2;
      Token := AsmToken_Constant;
      MemoryReferences := [];
      SegmentOverrideOpcode := 0;
      Value.Word2 := 0;
      TypeSize := 0;
      If RelocatableIdentifier = 0 then ReferenceFlags := 0;
    end;
end;

Procedure TYPE_Proc; Far;
begin
  With LeftExpression^ do
    begin
      Value.Word := LeftExpression^.TypeSize;
      RelocatableIdentifier := 0;
      Token := AsmToken_Constant;
      MemoryReferences := [];
      SegmentOverrideOpcode := 0;
      Value.Word2 := 0;
      TypeSize := 0;
      If RelocatableIdentifier = 0 then ReferenceFlags := 0;
    end;
end;

Procedure UnaryPlus_Proc; Far;
begin
  AsmString := '';
end;

Procedure UnaryMinus_Proc; Far;
begin
  CheckNoRelocatableIdentifier;
  LeftExpression^.Value.LongInt := - LeftExpression^.Value.LongInt;
  AsmString := '';
end;

Procedure HIGH_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.Byte1;
end;

Procedure LOW_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.Byte;
end;

Procedure ProcessAsmFactorElement;

Const OFFSET_SEG_TYPE_Table:  TOperatorProc = (Token: AsmToken_OFFSET; Proc: OFFSET_Proc);
      OFFSET_SEG_TYPE_Table1: TOperatorProc = (Token: AsmToken_SEG;    Proc: SEG_Proc);
      OFFSET_SEG_TYPE_Table2: TOperatorProc = (Token: AsmToken_TYPE;   Proc: TYPE_Proc);
      OFFSET_SEG_TYPE_Table_End: Byte = 0;
Var SegmentRegister: Byte;
    Saved_TypeSize: Word;
    Saved_LeftExpression_AdditionalIdentifierData: Pointer;
    NumberOfPushedProcs: Word;
    ExpressionProc, TempExpressionProc: TExpressionProc;

  Procedure ProcessQualifiedBasicExpressionElement;
  Const UnaryPlusMinus_Table:  TOperatorProc = (Token: AsmToken_Plus;  Proc: UnaryPlus_Proc);
        UnaryPlusMinus_Table1: TOperatorProc = (Token: AsmToken_Minus; Proc: UnaryMinus_Proc);
        UnaryPlusMinus_Table_End: Byte = 0;

        HIGH_LOW_Table:  TOperatorProc = (Token: AsmToken_HIGH; Proc: HIGH_Proc);
        HIGH_LOW_Table1: TOperatorProc = (Token: AsmToken_LOW;  Proc: LOW_Proc);
        HIGH_LOW_Table_End: Byte = 0;
  Var NumberOfPushedProcs: Word;
      ExpressionProc, TempExpressionProc: TExpressionProc;
  begin
    NumberOfPushedProcs := 0;
    While FindAsmTokenInTable (@UnaryPlusMinus_Table, ExpressionProc) do
      begin
        CheckAsmStack;
        Asm
          PUSH   WORD PTR ExpressionProc + 2
          PUSH   WORD PTR ExpressionProc
        end;
        Inc (NumberOfPushedProcs);
      end;
    While FindAsmTokenInTable (@HIGH_LOW_Table, ExpressionProc) do
      begin
        CheckAsmStack;
        Asm
          PUSH   WORD PTR ExpressionProc + 2
          PUSH   WORD PTR ExpressionProc
        end;
        Inc (NumberOfPushedProcs);
      end;
    CheckAsmStack;
    ProcessBasicExpressionElement;
    Repeat
      Case AsmToken of
        AsmToken_Point: begin
                          AsmIdentifierAdditionalData := LeftExpression^.AdditionalIdentifierData;
                          LeftExpression^.AdditionalIdentifierData := nil;
                          GetNextAsmToken;
                          PushLeftAndCreateNewExpression (ExpressionProc);
                          ProcessAsmFactorElement;
                          MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
                          If RightExpression^.Token = AsmToken_Register then AsmError (FieldIdentifierExpected);
                          If RightExpression^.TypeSize <> 0 then LeftExpression^.TypeSize := 0;
                          AddAsmExpressions;
                          Asm
                            ADD   SP, 20
                          end;
                        end;
        AsmToken_LeftBracket,
        AsmToken_LeftParenthesis: begin
                                    PushLeftAndCreateNewExpression (ExpressionProc);
                                    ProcessBasicExpressionElement;
                                    MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
                                    If LeftExpression^.TypeSize <> 0 then RightExpression^.TypeSize := 0;
                                    AddAsmExpressions;
           Asm
                                      ADD   SP, 20
                                    end;
                                  end;
        else begin
               While NumberOfPushedProcs <> 0 do
                 begin
                   Asm
                     POP    WORD PTR TempExpressionProc
                     POP    WORD PTR TempExpressionProc + 2
                   end;
                   TempExpressionProc;
                   Dec (NumberOfPushedProcs);
                 end;
               Exit;
             end;
      end;
    until False;
  end;

begin
  CheckAsmStack;
  NumberOfPushedProcs := 0;
  While FindAsmTokenInTable (@OFFSET_SEG_TYPE_Table, ExpressionProc) do
    begin
      Asm
        PUSH   WORD PTR ExpressionProc + 2
        PUSH   WORD PTR ExpressionProc
      end;
      Inc (NumberOfPushedProcs);
      ClearLeftExpression;
    end;
  ProcessQualifiedBasicExpressionElement;
  If CheckAndGetNextAsmToken (AsmToken_PTR) then
    begin
      PushLeftAndCreateNewExpression (ExpressionProc);
      ProcessAsmFactorElement;
      If LeftExpression^.Token = AsmToken_Register then AsmError (ConstantExpected);
      MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
      Saved_TypeSize := LeftExpression^.TypeSize;
      Saved_LeftExpression_AdditionalIdentifierData := LeftExpression^.AdditionalIdentifierData;
      LeftExpression^ := RightExpression^;
      LeftExpression^.TypeSize := Saved_TypeSize ;
      LeftExpression^.AdditionalIdentifierData := Saved_LeftExpression_AdditionalIdentifierData;
      LeftExpression^.Token := AsmToken_MemoryReference;
      Include (LeftExpression^.MemoryReferences, mrMemoryLocation);
      Asm
        ADD   SP, 20
      end;
    end else If (LeftExpression^.RegisterType = rtSegmentRegister) and
                (LeftExpression^.Token = AsmToken_Register) and CheckAndGetNextAsmToken (AsmToken_Colon) then
                begin
                  SegmentRegister := LeftExpression^.Register;
                  ClearLeftExpression;
                  ProcessAsmFactorElement;
                  LeftExpression^.SegmentOverrideOpcode := SegmentRegister shl 3 or SegmentOverride;
                  Include (LeftExpression^.MemoryReferences, mrMemoryLocation);
                  SetExpressionToMemoryReference;
                end;
  While NumberOfPushedProcs <> 0 do
    begin
      Asm
        POP    WORD PTR TempExpressionProc
        POP    wORD PTR TempExpressionProc + 2
      end;
      TempExpressionProc;
      Dec (NumberOfPushedProcs);
    end;
end;
Most operations are performed on constant values and processed with procedures above, however, addition is special since many different cases are possible. This procedure adds two assembler expressions.
Procedure AddAsmExpressions;

  Procedure SetBothExpressionsToMemoryReference;
  begin
    SetExpressionToMemoryReference;
    SwapLeftAndRightExpression;
    SetExpressionToMemoryReference;
    SwapLeftAndRightExpression;
  end;

begin
  If RightExpression^.Token <> AsmToken_Constant then SetBothExpressionsToMemoryReference else
    If LeftExpression^.Token <> AsmToken_Constant then
      Case LeftExpression^.Token of
        AsmToken_Register: If LeftExpression^.RegisterType = rtFPURegister then
                              LeftExpression^.Register := (LeftExpression^.Register + RightExpression^.Value.Byte) and $07
                                else SetBothExpressionsToMemoryReference;
        else SetBothExpressionsToMemoryReference;
      end;
  begin
    LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt + RightExpression^.Value.LongInt;
    If RightExpression^.AdditionalIdentifierData <> nil then
      LeftExpression^.AdditionalIdentifierData := RightExpression^.AdditionalIdentifierData;
    If RightExpression^.RelocatableIdentifier <> 0 then
      begin
        LeftExpression^.ReferenceFlags := RightExpression^.ReferenceFlags;
        If LeftExpression^.RelocatableIdentifier <> 0 then AsmError (CannotAddOrSubtractRelocatableSymbols);
        LeftExpression^.RelocatableIdentifier := RightExpression^.RelocatableIdentifier;
      end;
    If RightExpression^.TypeSize <> 0 then LeftExpression^.TypeSize := RightExpression^.TypeSize;
    If (LeftExpression^.MemoryReferences *
        RightExpression^.MemoryReferences * [mrBX, mrBP, mrSI, mrDI]) <> [] then AsmError (InvalidRegisterCombination);
    LeftExpression^.MemoryReferences :=
      LeftExpression^.MemoryReferences + RightExpression^.MemoryReferences;
    If RightExpression^.SegmentOverrideOpcode <> 0 then
      begin
        If LeftExpression^.SegmentOverrideOpcode and $80 <> 0 then AsmError (SyntaxError);
        LeftExpression^.SegmentOverrideOpcode := RightExpression^.SegmentOverrideOpcode;
      end;
  end;
end;
This procedure sets expression to memory reference and checks reference registers.
Procedure SetExpressionToMemoryReference;
Const MemoryReferenceRegister: Array [rAX..rDI] of TMemoryReferenceRegistersSet =
        ([], [], [], [mrBX], [], [mrBP], [mrSI], [mrDI]);
begin
  With LeftExpression^ do
    Case Token of
      AsmToken_Constant: Token := AsmToken_MemoryReference;
      AsmToken_Register: begin
                           If RegisterType <> rt16BitRegister then AsmError (InvalidRegisterCombination);
                           If MemoryReferenceRegister [Register] = [] then AsmError (InvalidRegisterCombination);
                           MemoryReferences := MemoryReferenceRegister [Register];
                           TypeSize := 0;
                           Token := AsmToken_MemoryReference;
                         end;
      AsmToken_MemoryReference:
      else AsmError (MemoryReferenceExpected);
    end;
end;
This procedure swaps left and right assembler expression (only pointers are swapped).
Procedure SwapLeftAndRightExpression;
Var TempExpression: PAsmExpression;
begin
  TempExpression := LeftExpression;
  LeftExpression := RightExpression;
  RightExpression := TempExpression;
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
Type TRegisterType = (rt8BitRegister, rt16BitRegister, rtSegmentRegister, rt03, rtFPURegister);

     TMemoryReferenceRegisters =  (mrDI, mrSI, mrBP, mrBX, mr10, mr20, mrMemoryLocation, mr80);

     TMemoryReferenceRegistersSet = Set of TMemoryReferenceRegisters;

     PAsmValue = ^TAsmValue;
     TAsmValue = Record
                   Case Byte of
                     0: (LongInt: LongInt);
                     1: (Word, Word2: Word);
                     2: (Byte, Byte1, Byte2, Byte3: Byte);
                   end;

     PAsmExpression = ^TAsmExpression;
     TAsmExpression = Record
                        Value: TAsmValue;
                        AdditionalIdentifierData: Pointer;
                        RelocatableIdentifier: LongInt;
                        TypeSize: Word;
                        MemoryReferences: TMemoryReferenceRegistersSet;
                        SegmentOverrideOpcode: Byte;
                        RegisterType: TRegisterType;
                        Register: Byte;
                        Token: TAsmToken;
                        ReferenceFlags: Byte;
                      end;
```

### Code Block 2

```pascal
Procedure ProcessInstructionExpresions;
begin
  ExpressionCounter := 0;
  ErrorPositionInstructionExpresion1 := AsmSourceErrorPosition;
  If CheckAndGetNextAsmToken (AsmToken_Semicolon) then Exit;
  LeftExpression := @AsmExpression1;
  ProcessAsmExpressionAndCheck;
  Inc (ExpressionCounter);
  If CheckAndGetNextAsmToken (AsmToken_Semicolon) then Exit;
  ExpectAsmTokenAndGetNext (AsmToken_Comma);
  ErrorPositionInstructionExpresion2 := AsmSourceErrorPosition;
  LeftExpression := @AsmExpression2;
  ProcessAsmExpressionAndCheck;
  Inc (ExpressionCounter);
  LeftExpression  := @AsmExpression1;
  RightExpression := @AsmExpression2;
  SavedRightExpressionTypeSize := RightExpression^.TypeSize;
  If RightExpression^.TypeSize <> 0 then
    begin
      If LeftExpression^.TypeSize = 0 then LeftExpression^.TypeSize := RightExpression^.TypeSize;
    end else RightExpression^.TypeSize := LeftExpression^.TypeSize;
  If CheckAndGetNextAsmToken (AsmToken_Semicolon) then Exit;
  ExpectAsmTokenAndGetNext (AsmToken_Comma);
  LeftExpression := @AsmExpression3;
  ProcessAsmExpressionAndCheck;
  Inc (ExpressionCounter);
  ExpectAsmTokenAndGetNext (AsmToken_Semicolon);
end;
```

### Code Block 3

```pascal
Procedure ProcessAsmExpressionAndCheck;
begin
  CheckAsmStack;
  ClearLeftExpression;
  ProcessAsmExpression;
  With LeftExpression^ do
    If (Token = AsmToken_MemoryReference) and not (mrMemoryLocation in MemoryReferences)
      then AsmError (MemoryReferenceExpected);
end;
```

### Code Block 4

```pascal
Procedure OR_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt or RightExpression^.Value.LongInt;
end;

Procedure XOR_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt xor RightExpression^.Value.LongInt;
end;

Procedure ProcessAsmExpression;
Const OR_XOR_Table:  TOperatorProc = (Token: AsmToken_OR;  Proc: OR_Proc);
      OR_XOR_Table1: TOperatorProc = (Token: AsmToken_XOR; Proc: XOR_Proc);
      OR_XOR_Table_End: Byte = 0;
Var ExpressionProc: TExpressionProc;
begin
  ProcessAsmTerm;
  While FindAsmTokenInTable (@OR_XOR_Table, ExpressionProc) do
    begin
      CheckConstantExpression;
      PushLeftAndCreateNewExpression (ExpressionProc);
      ProcessAsmTerm;
      CheckConstantExpression;
      MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
      ExpressionProc;
      Asm
        ADD   SP, 20
      end;
    end;
end;
```

### Code Block 5

```pascal
Procedure Plus_Proc; Far;
begin
  AddAsmExpressions;
  AsmString := '';
end;

Procedure Minus_Proc; Far;
begin
  SwapLeftAndRightExpression;
  CheckNoRelocatableIdentifier;
  SwapLeftAndRightExpression;
  RightExpression^.Value.LongInt := - RightExpression^.Value.LongInt;
  AddAsmExpressions;
  AsmString := '';
end;

Procedure ProcessAsmTerm;
Var ExpressionProc: TExpressionProc;

  Procedure Process_NOT_PLUS_MINUS;
  Const NOT_Table: TOperatorProc = (Token: AsmToken_NOT; Proc: nil);
        NOT_Table_End: Byte = 0;

        PlusMinus_Table:  TOperatorProc = (Token: AsmToken_Plus; Proc: Plus_Proc);
        PlusMinus_Table1: TOperatorProc = (Token: AsmToken_Minus; Proc: Minus_Proc);
        PlusMinus_Table_End: Byte = 0;
  Var ExpressionProc: TExpressionProc;
  begin
    CheckAsmStack;
    If FindAsmTokenInTable (@NOT_Table, ExpressionProc) then
      begin
        Process_NOT_PLUS_MINUS;
        CheckConstantExpression;
        LeftExpression^.Value.LongInt := not LeftExpression^.Value.LongInt;
        AsmString := '';
        Exit;
      end;
    ProcessAsmFactor;
    While FindAsmTokenInTable (@PlusMinus_Table, ExpressionProc) do
      begin
        PushLeftAndCreateNewExpression (ExpressionProc);
        ProcessAsmFactor;
        MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
        ExpressionProc;
        Asm
          ADD   SP, 20
        end;
      end;
  end;

begin
  Process_NOT_PLUS_MINUS;
  While CheckAndGetNextAsmToken (AsmToken_AND) do
    begin
      CheckConstantExpression;
      PushLeftAndCreateNewExpression (ExpressionProc);
      Process_NOT_PLUS_MINUS;
      CheckConstantExpression;
      MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
      LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt and RightExpression^.Value.LongInt;
      Asm
        ADD   SP, 20
      end;
    end;
end;
```

### Code Block 6

```pascal
Procedure CheckConstantExpression;
begin
  If (LeftExpression^.Token <> AsmToken_Constant) or (LeftExpression^.RelocatableIdentifier <> 0) then
    AsmError (ConstantExpected);
end;
```

### Code Block 7

```pascal
Procedure Asterisk_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt * RightExpression^.Value.LongInt;
end;

Procedure Slash_Proc; Far;
begin
  If RightExpression^.Value.LongInt = 0 then AsmError (DivisionByZero);
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt div RightExpression^.Value.LongInt;
end;

Procedure MOD_Proc; Far;
begin
  If RightExpression^.Value.LongInt = 0 then AsmError (DivisionByZero);
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt mod RightExpression^.Value.LongInt;
end;

Procedure SHR_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt shr RightExpression^.Value.LongInt;
end;

Procedure SHL_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt shl RightExpression^.Value.LongInt;
end;

Procedure ProcessAsmFactor;

Const MultDivShift_Table:  TOperatorProc = (Token: AsmToken_Asterisk; Proc: Asterisk_Proc);
      MultDivShift_Table1: TOperatorProc = (Token: AsmToken_Slash;    Proc: Slash_Proc);
      MultDivShift_Table2: TOperatorProc = (Token: AsmToken_MOD;      Proc: MOD_Proc);
      MultDivShift_Table3: TOperatorProc = (Token: AsmToken_SHR;      Proc: SHR_Proc);
      MultDivShift_Table4: TOperatorProc = (Token: AsmToken_SHL;      Proc: SHL_Proc);
      MultDivShift_Table_End: Byte = 0;
Var ExpressionProc: TExpressionProc;
begin
  ProcessAsmFactorElement;
  While FindAsmTokenInTable (@MultDivShift_Table, ExpressionProc) do
    begin
      CheckConstantExpression;
      PushLeftAndCreateNewExpression (ExpressionProc);
      ProcessAsmFactorElement;
      CheckConstantExpression;
      MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
      ExpressionProc;
      Asm
        ADD   SP, 20
      end;
    end;
end;
```

### Code Block 8

```pascal
Procedure OFFSET_Proc; Far;
begin
  With LeftExpression^ do
    begin
      ReferenceFlags := 1;
      Token := AsmToken_Constant;
      MemoryReferences := [];
      SegmentOverrideOpcode := 0;
      Value.Word2 := 0;
      TypeSize := 0;
      If RelocatableIdentifier = 0 then ReferenceFlags := 0;
    end;
end;

Procedure SEG_Proc; Far;
begin
  With LeftExpression^ do
    begin
      ReferenceFlags := 2;
      Value.Word := LeftExpression^.Value.Word2;
      Token := AsmToken_Constant;
      MemoryReferences := [];
      SegmentOverrideOpcode := 0;
      Value.Word2 := 0;
      TypeSize := 0;
      If RelocatableIdentifier = 0 then ReferenceFlags := 0;
    end;
end;

Procedure TYPE_Proc; Far;
begin
  With LeftExpression^ do
    begin
      Value.Word := LeftExpression^.TypeSize;
      RelocatableIdentifier := 0;
      Token := AsmToken_Constant;
      MemoryReferences := [];
      SegmentOverrideOpcode := 0;
      Value.Word2 := 0;
      TypeSize := 0;
      If RelocatableIdentifier = 0 then ReferenceFlags := 0;
    end;
end;

Procedure UnaryPlus_Proc; Far;
begin
  AsmString := '';
end;

Procedure UnaryMinus_Proc; Far;
begin
  CheckNoRelocatableIdentifier;
  LeftExpression^.Value.LongInt := - LeftExpression^.Value.LongInt;
  AsmString := '';
end;

Procedure HIGH_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.Byte1;
end;

Procedure LOW_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.Byte;
end;

Procedure ProcessAsmFactorElement;

Const OFFSET_SEG_TYPE_Table:  TOperatorProc = (Token: AsmToken_OFFSET; Proc: OFFSET_Proc);
      OFFSET_SEG_TYPE_Table1: TOperatorProc = (Token: AsmToken_SEG;    Proc: SEG_Proc);
      OFFSET_SEG_TYPE_Table2: TOperatorProc = (Token: AsmToken_TYPE;   Proc: TYPE_Proc);
      OFFSET_SEG_TYPE_Table_End: Byte = 0;
Var SegmentRegister: Byte;
    Saved_TypeSize: Word;
    Saved_LeftExpression_AdditionalIdentifierData: Pointer;
    NumberOfPushedProcs: Word;
    ExpressionProc, TempExpressionProc: TExpressionProc;

  Procedure ProcessQualifiedBasicExpressionElement;
  Const UnaryPlusMinus_Table:  TOperatorProc = (Token: AsmToken_Plus;  Proc: UnaryPlus_Proc);
        UnaryPlusMinus_Table1: TOperatorProc = (Token: AsmToken_Minus; Proc: UnaryMinus_Proc);
        UnaryPlusMinus_Table_End: Byte = 0;

        HIGH_LOW_Table:  TOperatorProc = (Token: AsmToken_HIGH; Proc: HIGH_Proc);
        HIGH_LOW_Table1: TOperatorProc = (Token: AsmToken_LOW;  Proc: LOW_Proc);
        HIGH_LOW_Table_End: Byte = 0;
  Var NumberOfPushedProcs: Word;
      ExpressionProc, TempExpressionProc: TExpressionProc;
  begin
    NumberOfPushedProcs := 0;
    While FindAsmTokenInTable (@UnaryPlusMinus_Table, ExpressionProc) do
      begin
        CheckAsmStack;
        Asm
          PUSH   WORD PTR ExpressionProc + 2
          PUSH   WORD PTR ExpressionProc
        end;
        Inc (NumberOfPushedProcs);
      end;
    While FindAsmTokenInTable (@HIGH_LOW_Table, ExpressionProc) do
      begin
        CheckAsmStack;
        Asm
          PUSH   WORD PTR ExpressionProc + 2
          PUSH   WORD PTR ExpressionProc
        end;
        Inc (NumberOfPushedProcs);
      end;
    CheckAsmStack;
    ProcessBasicExpressionElement;
    Repeat
      Case AsmToken of
        AsmToken_Point: begin
                          AsmIdentifierAdditionalData := LeftExpression^.AdditionalIdentifierData;
                          LeftExpression^.AdditionalIdentifierData := nil;
                          GetNextAsmToken;
                          PushLeftAndCreateNewExpression (ExpressionProc);
                          ProcessAsmFactorElement;
                          MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
                          If RightExpression^.Token = AsmToken_Register then AsmError (FieldIdentifierExpected);
                          If RightExpression^.TypeSize <> 0 then LeftExpression^.TypeSize := 0;
                          AddAsmExpressions;
                          Asm
                            ADD   SP, 20
                          end;
                        end;
        AsmToken_LeftBracket,
        AsmToken_LeftParenthesis: begin
                                    PushLeftAndCreateNewExpression (ExpressionProc);
                                    ProcessBasicExpressionElement;
                                    MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
                                    If LeftExpression^.TypeSize <> 0 then RightExpression^.TypeSize := 0;
                                    AddAsmExpressions;
           Asm
                                      ADD   SP, 20
                                    end;
                                  end;
        else begin
               While NumberOfPushedProcs <> 0 do
                 begin
                   Asm
                     POP    WORD PTR TempExpressionProc
                     POP    WORD PTR TempExpressionProc + 2
                   end;
                   TempExpressionProc;
                   Dec (NumberOfPushedProcs);
                 end;
               Exit;
             end;
      end;
    until False;
  end;

begin
  CheckAsmStack;
  NumberOfPushedProcs := 0;
  While FindAsmTokenInTable (@OFFSET_SEG_TYPE_Table, ExpressionProc) do
    begin
      Asm
        PUSH   WORD PTR ExpressionProc + 2
        PUSH   WORD PTR ExpressionProc
      end;
      Inc (NumberOfPushedProcs);
      ClearLeftExpression;
    end;
  ProcessQualifiedBasicExpressionElement;
  If CheckAndGetNextAsmToken (AsmToken_PTR) then
    begin
      PushLeftAndCreateNewExpression (ExpressionProc);
      ProcessAsmFactorElement;
      If LeftExpression^.Token = AsmToken_Register then AsmError (ConstantExpected);
      MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
      Saved_TypeSize := LeftExpression^.TypeSize;
      Saved_LeftExpression_AdditionalIdentifierData := LeftExpression^.AdditionalIdentifierData;
      LeftExpression^ := RightExpression^;
      LeftExpression^.TypeSize := Saved_TypeSize ;
      LeftExpression^.AdditionalIdentifierData := Saved_LeftExpression_AdditionalIdentifierData;
      LeftExpression^.Token := AsmToken_MemoryReference;
      Include (LeftExpression^.MemoryReferences, mrMemoryLocation);
      Asm
        ADD   SP, 20
      end;
    end else If (LeftExpression^.RegisterType = rtSegmentRegister) and
                (LeftExpression^.Token = AsmToken_Register) and CheckAndGetNextAsmToken (AsmToken_Colon) then
                begin
                  SegmentRegister := LeftExpression^.Register;
                  ClearLeftExpression;
                  ProcessAsmFactorElement;
                  LeftExpression^.SegmentOverrideOpcode := SegmentRegister shl 3 or SegmentOverride;
                  Include (LeftExpression^.MemoryReferences, mrMemoryLocation);
                  SetExpressionToMemoryReference;
                end;
  While NumberOfPushedProcs <> 0 do
    begin
      Asm
        POP    WORD PTR TempExpressionProc
        POP    wORD PTR TempExpressionProc + 2
      end;
      TempExpressionProc;
      Dec (NumberOfPushedProcs);
    end;
end;
```

### Code Block 9

```pascal
Procedure AddAsmExpressions;

  Procedure SetBothExpressionsToMemoryReference;
  begin
    SetExpressionToMemoryReference;
    SwapLeftAndRightExpression;
    SetExpressionToMemoryReference;
    SwapLeftAndRightExpression;
  end;

begin
  If RightExpression^.Token <> AsmToken_Constant then SetBothExpressionsToMemoryReference else
    If LeftExpression^.Token <> AsmToken_Constant then
      Case LeftExpression^.Token of
        AsmToken_Register: If LeftExpression^.RegisterType = rtFPURegister then
                              LeftExpression^.Register := (LeftExpression^.Register + RightExpression^.Value.Byte) and $07
                                else SetBothExpressionsToMemoryReference;
        else SetBothExpressionsToMemoryReference;
      end;
  begin
    LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt + RightExpression^.Value.LongInt;
    If RightExpression^.AdditionalIdentifierData <> nil then
      LeftExpression^.AdditionalIdentifierData := RightExpression^.AdditionalIdentifierData;
    If RightExpression^.RelocatableIdentifier <> 0 then
      begin
        LeftExpression^.ReferenceFlags := RightExpression^.ReferenceFlags;
        If LeftExpression^.RelocatableIdentifier <> 0 then AsmError (CannotAddOrSubtractRelocatableSymbols);
        LeftExpression^.RelocatableIdentifier := RightExpression^.RelocatableIdentifier;
      end;
    If RightExpression^.TypeSize <> 0 then LeftExpression^.TypeSize := RightExpression^.TypeSize;
    If (LeftExpression^.MemoryReferences *
        RightExpression^.MemoryReferences * [mrBX, mrBP, mrSI, mrDI]) <> [] then AsmError (InvalidRegisterCombination);
    LeftExpression^.MemoryReferences :=
      LeftExpression^.MemoryReferences + RightExpression^.MemoryReferences;
    If RightExpression^.SegmentOverrideOpcode <> 0 then
      begin
        If LeftExpression^.SegmentOverrideOpcode and $80 <> 0 then AsmError (SyntaxError);
        LeftExpression^.SegmentOverrideOpcode := RightExpression^.SegmentOverrideOpcode;
      end;
  end;
end;
```

### Code Block 10

```pascal
Procedure SetExpressionToMemoryReference;
Const MemoryReferenceRegister: Array [rAX..rDI] of TMemoryReferenceRegistersSet =
        ([], [], [], [mrBX], [], [mrBP], [mrSI], [mrDI]);
begin
  With LeftExpression^ do
    Case Token of
      AsmToken_Constant: Token := AsmToken_MemoryReference;
      AsmToken_Register: begin
                           If RegisterType <> rt16BitRegister then AsmError (InvalidRegisterCombination);
                           If MemoryReferenceRegister [Register] = [] then AsmError (InvalidRegisterCombination);
                           MemoryReferences := MemoryReferenceRegister [Register];
                           TypeSize := 0;
                           Token := AsmToken_MemoryReference;
                         end;
      AsmToken_MemoryReference:
      else AsmError (MemoryReferenceExpected);
    end;
end;
```

### Code Block 11

```pascal
Procedure SwapLeftAndRightExpression;
Var TempExpression: PAsmExpression;
begin
  TempExpression := LeftExpression;
  LeftExpression := RightExpression;
  RightExpression := TempExpression;
end;
```

### Code Block 12

```pascal
Type TRegisterType = (rt8BitRegister, rt16BitRegister, rtSegmentRegister, rt03, rtFPURegister);

     TMemoryReferenceRegisters =  (mrDI, mrSI, mrBP, mrBX, mr10, mr20, mrMemoryLocation, mr80);

     TMemoryReferenceRegistersSet = Set of TMemoryReferenceRegisters;

     PAsmValue = ^TAsmValue;
     TAsmValue = Record
                   Case Byte of
                     0: (LongInt: LongInt);
                     1: (Word, Word2: Word);
                     2: (Byte, Byte1, Byte2, Byte3: Byte);
                   end;

     PAsmExpression = ^TAsmExpression;
     TAsmExpression = Record
                        Value: TAsmValue;
                        AdditionalIdentifierData: Pointer;
                        RelocatableIdentifier: LongInt;
                        TypeSize: Word;
                        MemoryReferences: TMemoryReferenceRegistersSet;
                        SegmentOverrideOpcode: Byte;
                        RegisterType: TRegisterType;
                        Register: Byte;
                        Token: TAsmToken;
                        ReferenceFlags: Byte;
                      end;
```

### Code Block 13

```pascal
Procedure ProcessInstructionExpresions;
begin
  ExpressionCounter := 0;
  ErrorPositionInstructionExpresion1 := AsmSourceErrorPosition;
  If CheckAndGetNextAsmToken (AsmToken_Semicolon) then Exit;
  LeftExpression := @AsmExpression1;
  ProcessAsmExpressionAndCheck;
  Inc (ExpressionCounter);
  If CheckAndGetNextAsmToken (AsmToken_Semicolon) then Exit;
  ExpectAsmTokenAndGetNext (AsmToken_Comma);
  ErrorPositionInstructionExpresion2 := AsmSourceErrorPosition;
  LeftExpression := @AsmExpression2;
  ProcessAsmExpressionAndCheck;
  Inc (ExpressionCounter);
  LeftExpression  := @AsmExpression1;
  RightExpression := @AsmExpression2;
  SavedRightExpressionTypeSize := RightExpression^.TypeSize;
  If RightExpression^.TypeSize <> 0 then
    begin
      If LeftExpression^.TypeSize = 0 then LeftExpression^.TypeSize := RightExpression^.TypeSize;
    end else RightExpression^.TypeSize := LeftExpression^.TypeSize;
  If CheckAndGetNextAsmToken (AsmToken_Semicolon) then Exit;
  ExpectAsmTokenAndGetNext (AsmToken_Comma);
  LeftExpression := @AsmExpression3;
  ProcessAsmExpressionAndCheck;
  Inc (ExpressionCounter);
  ExpectAsmTokenAndGetNext (AsmToken_Semicolon);
end;
```

### Code Block 14

```pascal
Procedure ProcessAsmExpressionAndCheck;
begin
  CheckAsmStack;
  ClearLeftExpression;
  ProcessAsmExpression;
  With LeftExpression^ do
    If (Token = AsmToken_MemoryReference) and not (mrMemoryLocation in MemoryReferences)
      then AsmError (MemoryReferenceExpected);
end;
```

### Code Block 15

```pascal
Procedure OR_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt or RightExpression^.Value.LongInt;
end;

Procedure XOR_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt xor RightExpression^.Value.LongInt;
end;

Procedure ProcessAsmExpression;
Const OR_XOR_Table:  TOperatorProc = (Token: AsmToken_OR;  Proc: OR_Proc);
      OR_XOR_Table1: TOperatorProc = (Token: AsmToken_XOR; Proc: XOR_Proc);
      OR_XOR_Table_End: Byte = 0;
Var ExpressionProc: TExpressionProc;
begin
  ProcessAsmTerm;
  While FindAsmTokenInTable (@OR_XOR_Table, ExpressionProc) do
    begin
      CheckConstantExpression;
      PushLeftAndCreateNewExpression (ExpressionProc);
      ProcessAsmTerm;
      CheckConstantExpression;
      MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
      ExpressionProc;
      Asm
        ADD   SP, 20
      end;
    end;
end;
```

### Code Block 16

```pascal
Procedure Plus_Proc; Far;
begin
  AddAsmExpressions;
  AsmString := '';
end;

Procedure Minus_Proc; Far;
begin
  SwapLeftAndRightExpression;
  CheckNoRelocatableIdentifier;
  SwapLeftAndRightExpression;
  RightExpression^.Value.LongInt := - RightExpression^.Value.LongInt;
  AddAsmExpressions;
  AsmString := '';
end;

Procedure ProcessAsmTerm;
Var ExpressionProc: TExpressionProc;

  Procedure Process_NOT_PLUS_MINUS;
  Const NOT_Table: TOperatorProc = (Token: AsmToken_NOT; Proc: nil);
        NOT_Table_End: Byte = 0;

        PlusMinus_Table:  TOperatorProc = (Token: AsmToken_Plus; Proc: Plus_Proc);
        PlusMinus_Table1: TOperatorProc = (Token: AsmToken_Minus; Proc: Minus_Proc);
        PlusMinus_Table_End: Byte = 0;
  Var ExpressionProc: TExpressionProc;
  begin
    CheckAsmStack;
    If FindAsmTokenInTable (@NOT_Table, ExpressionProc) then
      begin
        Process_NOT_PLUS_MINUS;
        CheckConstantExpression;
        LeftExpression^.Value.LongInt := not LeftExpression^.Value.LongInt;
        AsmString := '';
        Exit;
      end;
    ProcessAsmFactor;
    While FindAsmTokenInTable (@PlusMinus_Table, ExpressionProc) do
      begin
        PushLeftAndCreateNewExpression (ExpressionProc);
        ProcessAsmFactor;
        MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
        ExpressionProc;
        Asm
          ADD   SP, 20
        end;
      end;
  end;

begin
  Process_NOT_PLUS_MINUS;
  While CheckAndGetNextAsmToken (AsmToken_AND) do
    begin
      CheckConstantExpression;
      PushLeftAndCreateNewExpression (ExpressionProc);
      Process_NOT_PLUS_MINUS;
      CheckConstantExpression;
      MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
      LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt and RightExpression^.Value.LongInt;
      Asm
        ADD   SP, 20
      end;
    end;
end;
```

### Code Block 17

```pascal
Procedure CheckConstantExpression;
begin
  If (LeftExpression^.Token <> AsmToken_Constant) or (LeftExpression^.RelocatableIdentifier <> 0) then
    AsmError (ConstantExpected);
end;
```

### Code Block 18

```pascal
Procedure Asterisk_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt * RightExpression^.Value.LongInt;
end;

Procedure Slash_Proc; Far;
begin
  If RightExpression^.Value.LongInt = 0 then AsmError (DivisionByZero);
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt div RightExpression^.Value.LongInt;
end;

Procedure MOD_Proc; Far;
begin
  If RightExpression^.Value.LongInt = 0 then AsmError (DivisionByZero);
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt mod RightExpression^.Value.LongInt;
end;

Procedure SHR_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt shr RightExpression^.Value.LongInt;
end;

Procedure SHL_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt shl RightExpression^.Value.LongInt;
end;

Procedure ProcessAsmFactor;

Const MultDivShift_Table:  TOperatorProc = (Token: AsmToken_Asterisk; Proc: Asterisk_Proc);
      MultDivShift_Table1: TOperatorProc = (Token: AsmToken_Slash;    Proc: Slash_Proc);
      MultDivShift_Table2: TOperatorProc = (Token: AsmToken_MOD;      Proc: MOD_Proc);
      MultDivShift_Table3: TOperatorProc = (Token: AsmToken_SHR;      Proc: SHR_Proc);
      MultDivShift_Table4: TOperatorProc = (Token: AsmToken_SHL;      Proc: SHL_Proc);
      MultDivShift_Table_End: Byte = 0;
Var ExpressionProc: TExpressionProc;
begin
  ProcessAsmFactorElement;
  While FindAsmTokenInTable (@MultDivShift_Table, ExpressionProc) do
    begin
      CheckConstantExpression;
      PushLeftAndCreateNewExpression (ExpressionProc);
      ProcessAsmFactorElement;
      CheckConstantExpression;
      MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
      ExpressionProc;
      Asm
        ADD   SP, 20
      end;
    end;
end;
```

### Code Block 19

```pascal
QualifiedExpressionElement
```

### Code Block 20

```pascal
QualifiedExpressionElement
```

### Code Block 21

```pascal
QualifiedExpressionElement
```

### Code Block 22

```pascal
Procedure OFFSET_Proc; Far;
begin
  With LeftExpression^ do
    begin
      ReferenceFlags := 1;
      Token := AsmToken_Constant;
      MemoryReferences := [];
      SegmentOverrideOpcode := 0;
      Value.Word2 := 0;
      TypeSize := 0;
      If RelocatableIdentifier = 0 then ReferenceFlags := 0;
    end;
end;

Procedure SEG_Proc; Far;
begin
  With LeftExpression^ do
    begin
      ReferenceFlags := 2;
      Value.Word := LeftExpression^.Value.Word2;
      Token := AsmToken_Constant;
      MemoryReferences := [];
      SegmentOverrideOpcode := 0;
      Value.Word2 := 0;
      TypeSize := 0;
      If RelocatableIdentifier = 0 then ReferenceFlags := 0;
    end;
end;

Procedure TYPE_Proc; Far;
begin
  With LeftExpression^ do
    begin
      Value.Word := LeftExpression^.TypeSize;
      RelocatableIdentifier := 0;
      Token := AsmToken_Constant;
      MemoryReferences := [];
      SegmentOverrideOpcode := 0;
      Value.Word2 := 0;
      TypeSize := 0;
      If RelocatableIdentifier = 0 then ReferenceFlags := 0;
    end;
end;

Procedure UnaryPlus_Proc; Far;
begin
  AsmString := '';
end;

Procedure UnaryMinus_Proc; Far;
begin
  CheckNoRelocatableIdentifier;
  LeftExpression^.Value.LongInt := - LeftExpression^.Value.LongInt;
  AsmString := '';
end;

Procedure HIGH_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.Byte1;
end;

Procedure LOW_Proc; Far;
begin
  LeftExpression^.Value.LongInt := LeftExpression^.Value.Byte;
end;

Procedure ProcessAsmFactorElement;

Const OFFSET_SEG_TYPE_Table:  TOperatorProc = (Token: AsmToken_OFFSET; Proc: OFFSET_Proc);
      OFFSET_SEG_TYPE_Table1: TOperatorProc = (Token: AsmToken_SEG;    Proc: SEG_Proc);
      OFFSET_SEG_TYPE_Table2: TOperatorProc = (Token: AsmToken_TYPE;   Proc: TYPE_Proc);
      OFFSET_SEG_TYPE_Table_End: Byte = 0;
Var SegmentRegister: Byte;
    Saved_TypeSize: Word;
    Saved_LeftExpression_AdditionalIdentifierData: Pointer;
    NumberOfPushedProcs: Word;
    ExpressionProc, TempExpressionProc: TExpressionProc;

  Procedure ProcessQualifiedBasicExpressionElement;
  Const UnaryPlusMinus_Table:  TOperatorProc = (Token: AsmToken_Plus;  Proc: UnaryPlus_Proc);
        UnaryPlusMinus_Table1: TOperatorProc = (Token: AsmToken_Minus; Proc: UnaryMinus_Proc);
        UnaryPlusMinus_Table_End: Byte = 0;

        HIGH_LOW_Table:  TOperatorProc = (Token: AsmToken_HIGH; Proc: HIGH_Proc);
        HIGH_LOW_Table1: TOperatorProc = (Token: AsmToken_LOW;  Proc: LOW_Proc);
        HIGH_LOW_Table_End: Byte = 0;
  Var NumberOfPushedProcs: Word;
      ExpressionProc, TempExpressionProc: TExpressionProc;
  begin
    NumberOfPushedProcs := 0;
    While FindAsmTokenInTable (@UnaryPlusMinus_Table, ExpressionProc) do
      begin
        CheckAsmStack;
        Asm
          PUSH   WORD PTR ExpressionProc + 2
          PUSH   WORD PTR ExpressionProc
        end;
        Inc (NumberOfPushedProcs);
      end;
    While FindAsmTokenInTable (@HIGH_LOW_Table, ExpressionProc) do
      begin
        CheckAsmStack;
        Asm
          PUSH   WORD PTR ExpressionProc + 2
          PUSH   WORD PTR ExpressionProc
        end;
        Inc (NumberOfPushedProcs);
      end;
    CheckAsmStack;
    ProcessBasicExpressionElement;
    Repeat
      Case AsmToken of
        AsmToken_Point: begin
                          AsmIdentifierAdditionalData := LeftExpression^.AdditionalIdentifierData;
                          LeftExpression^.AdditionalIdentifierData := nil;
                          GetNextAsmToken;
                          PushLeftAndCreateNewExpression (ExpressionProc);
                          ProcessAsmFactorElement;
                          MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
                          If RightExpression^.Token = AsmToken_Register then AsmError (FieldIdentifierExpected);
                          If RightExpression^.TypeSize <> 0 then LeftExpression^.TypeSize := 0;
                          AddAsmExpressions;
                          Asm
                            ADD   SP, 20
                          end;
                        end;
        AsmToken_LeftBracket,
        AsmToken_LeftParenthesis: begin
                                    PushLeftAndCreateNewExpression (ExpressionProc);
                                    ProcessBasicExpressionElement;
                                    MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
                                    If LeftExpression^.TypeSize <> 0 then RightExpression^.TypeSize := 0;
                                    AddAsmExpressions;
           Asm
                                      ADD   SP, 20
                                    end;
                                  end;
        else begin
               While NumberOfPushedProcs <> 0 do
                 begin
                   Asm
                     POP    WORD PTR TempExpressionProc
                     POP    WORD PTR TempExpressionProc + 2
                   end;
                   TempExpressionProc;
                   Dec (NumberOfPushedProcs);
                 end;
               Exit;
             end;
      end;
    until False;
  end;

begin
  CheckAsmStack;
  NumberOfPushedProcs := 0;
  While FindAsmTokenInTable (@OFFSET_SEG_TYPE_Table, ExpressionProc) do
    begin
      Asm
        PUSH   WORD PTR ExpressionProc + 2
        PUSH   WORD PTR ExpressionProc
      end;
      Inc (NumberOfPushedProcs);
      ClearLeftExpression;
    end;
  ProcessQualifiedBasicExpressionElement;
  If CheckAndGetNextAsmToken (AsmToken_PTR) then
    begin
      PushLeftAndCreateNewExpression (ExpressionProc);
      ProcessAsmFactorElement;
      If LeftExpression^.Token = AsmToken_Register then AsmError (ConstantExpected);
      MoveLastExpressionToRightAndPopLeftExpression (ExpressionProc);
      Saved_TypeSize := LeftExpression^.TypeSize;
      Saved_LeftExpression_AdditionalIdentifierData := LeftExpression^.AdditionalIdentifierData;
      LeftExpression^ := RightExpression^;
      LeftExpression^.TypeSize := Saved_TypeSize ;
      LeftExpression^.AdditionalIdentifierData := Saved_LeftExpression_AdditionalIdentifierData;
      LeftExpression^.Token := AsmToken_MemoryReference;
      Include (LeftExpression^.MemoryReferences, mrMemoryLocation);
      Asm
        ADD   SP, 20
      end;
    end else If (LeftExpression^.RegisterType = rtSegmentRegister) and
                (LeftExpression^.Token = AsmToken_Register) and CheckAndGetNextAsmToken (AsmToken_Colon) then
                begin
                  SegmentRegister := LeftExpression^.Register;
                  ClearLeftExpression;
                  ProcessAsmFactorElement;
                  LeftExpression^.SegmentOverrideOpcode := SegmentRegister shl 3 or SegmentOverride;
                  Include (LeftExpression^.MemoryReferences, mrMemoryLocation);
                  SetExpressionToMemoryReference;
                end;
  While NumberOfPushedProcs <> 0 do
    begin
      Asm
        POP    WORD PTR TempExpressionProc
        POP    wORD PTR TempExpressionProc + 2
      end;
      TempExpressionProc;
      Dec (NumberOfPushedProcs);
    end;
end;
```

### Code Block 23

```pascal
Procedure AddAsmExpressions;

  Procedure SetBothExpressionsToMemoryReference;
  begin
    SetExpressionToMemoryReference;
    SwapLeftAndRightExpression;
    SetExpressionToMemoryReference;
    SwapLeftAndRightExpression;
  end;

begin
  If RightExpression^.Token <> AsmToken_Constant then SetBothExpressionsToMemoryReference else
    If LeftExpression^.Token <> AsmToken_Constant then
      Case LeftExpression^.Token of
        AsmToken_Register: If LeftExpression^.RegisterType = rtFPURegister then
                              LeftExpression^.Register := (LeftExpression^.Register + RightExpression^.Value.Byte) and $07
                                else SetBothExpressionsToMemoryReference;
        else SetBothExpressionsToMemoryReference;
      end;
  begin
    LeftExpression^.Value.LongInt := LeftExpression^.Value.LongInt + RightExpression^.Value.LongInt;
    If RightExpression^.AdditionalIdentifierData <> nil then
      LeftExpression^.AdditionalIdentifierData := RightExpression^.AdditionalIdentifierData;
    If RightExpression^.RelocatableIdentifier <> 0 then
      begin
        LeftExpression^.ReferenceFlags := RightExpression^.ReferenceFlags;
        If LeftExpression^.RelocatableIdentifier <> 0 then AsmError (CannotAddOrSubtractRelocatableSymbols);
        LeftExpression^.RelocatableIdentifier := RightExpression^.RelocatableIdentifier;
      end;
    If RightExpression^.TypeSize <> 0 then LeftExpression^.TypeSize := RightExpression^.TypeSize;
    If (LeftExpression^.MemoryReferences *
        RightExpression^.MemoryReferences * [mrBX, mrBP, mrSI, mrDI]) <> [] then AsmError (InvalidRegisterCombination);
    LeftExpression^.MemoryReferences :=
      LeftExpression^.MemoryReferences + RightExpression^.MemoryReferences;
    If RightExpression^.SegmentOverrideOpcode <> 0 then
      begin
        If LeftExpression^.SegmentOverrideOpcode and $80 <> 0 then AsmError (SyntaxError);
        LeftExpression^.SegmentOverrideOpcode := RightExpression^.SegmentOverrideOpcode;
      end;
  end;
end;
```

### Code Block 24

```pascal
Procedure SetExpressionToMemoryReference;
Const MemoryReferenceRegister: Array [rAX..rDI] of TMemoryReferenceRegistersSet =
        ([], [], [], [mrBX], [], [mrBP], [mrSI], [mrDI]);
begin
  With LeftExpression^ do
    Case Token of
      AsmToken_Constant: Token := AsmToken_MemoryReference;
      AsmToken_Register: begin
                           If RegisterType <> rt16BitRegister then AsmError (InvalidRegisterCombination);
                           If MemoryReferenceRegister [Register] = [] then AsmError (InvalidRegisterCombination);
                           MemoryReferences := MemoryReferenceRegister [Register];
                           TypeSize := 0;
                           Token := AsmToken_MemoryReference;
                         end;
      AsmToken_MemoryReference:
      else AsmError (MemoryReferenceExpected);
    end;
end;
```

### Code Block 25

```pascal
Procedure SwapLeftAndRightExpression;
Var TempExpression: PAsmExpression;
begin
  TempExpression := LeftExpression;
  LeftExpression := RightExpression;
  RightExpression := TempExpression;
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
- https://turbopascal.org/processing-assembler-expressions
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

