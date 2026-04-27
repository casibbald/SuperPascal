# Extracting Assembler Tokens - Turbo Pascal Compiler

**URL:** https://turbopascal.org/extracting-assembler-tokens

---

## Table of Contents

- Extracting Assembler Tokens

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Extracting Assembler Tokens
This procedure scans assembler instruction and returns the next assembler token.
Procedure GetNextAsmToken;

  Procedure SeparateAsmWord;
  Var C: Char;

    Function IsLetterOrNumberOrSpecialCharacter: Boolean;
    begin
      Case C of
        '0'..'9',
        'A'..'Z',
        'a'..'z',
        '_',
        '$',
        '@',
        '&',
        '?' : IsLetterOrNumberOrSpecialCharacter := True;
        else  IsLetterOrNumberOrSpecialCharacter := False;
      end;
    end;

    Function IsString: Boolean;
    Var StringChar, StringDelimiter: Char;
    begin
      IsString := False;
      If not (C in ['''', '"']) then Exit;
      StringDelimiter := C;
      AsmString := '';
      IsString := True;
      Repeat
        StringChar := GetNextChar;
        If StringChar = #0 then AsmError (StringConstantExceedsLine);
        If StringChar = StringDelimiter then
          begin
            StringChar := GetNextChar;
            If StringChar <> StringDelimiter then Exit;
          end;
        If Byte (AsmString [0]) = 127 then AsmError (StringConstantExceedsLine);
        Inc (AsmString [0]);
        AsmString [Length (AsmString)] := StringChar;
      until False;
    end;

  begin
    AsmTokenString := '';
    AsmTokenUpCaseString := '';
    AsmWordHash := 0;
    C := GetFirstNonBlankCharacter;
    AsmSourceErrorPosition := AsmCharacterPointer;
    If C <> #0 then
      begin
        If IsLetterOrNumberOrSpecialCharacter then
          begin
            Repeat
              If Length (AsmTokenString) < 32 then
                begin
                  Inc (AsmTokenString [0]);
                  Inc (AsmTokenUpCaseString [0]);
                  AsmTokenString [Byte (AsmTokenString [0])] := C;
                  AsmTokenUpCaseString [Byte (AsmTokenUpCaseString [0])] := UpCase (C);
                  Inc (AsmWordHash, Ord (UpCase (C)) - 1);
                end;
              C := GetNextChar;
            until (C = #0) or not IsLetterOrNumberOrSpecialCharacter;
            AsmWordType := 1;
            If AsmTokenUpCaseString <> 'END' then Exit;
          end else begin
                     AsmWordType := 2;
                     If IsString then Exit;
                     AsmWordType := 0;
                     AsmTokenString := C;
                     Case C of
                       ';', '{':
{$B+}
                       else If (C <> '(') or (GetNextChar <> '*') then Exit;
{$B-}
                     end;
                   end;
      end;
    AsmWordType := 0;
    AsmTokenString := ';';
    LineCharacterCounter := 0;
    AsmCharacterPointer := AsmSourceErrorPosition;
  end;

  Function FindAsmWord (AsmList: PIdentifierList; Var AsmWordData: Pointer): Boolean;
  Var AsmWord: PAsmWord absolute AsmWordData;
      AsmWordOffset: Word absolute AsmWordData;
  begin
    FindAsmWord := False;
    If AsmWordType <> 1 then Exit;
    AsmWord := Ptr (DSeg, AsmList^.Offset [AsmWordHash and AsmList^.Mask]);
    While AsmWordOffset <> 0 do
      begin
        AsmToken := AsmToken_00;
        If AsmWord^.Str = AsmTokenUpCaseString then
          begin
            Inc (AsmWordOffset, Length (AsmWord^.Str) + 3);
            FindAsmWord := True;
            Exit;
          end;
        AsmWordOffset := AsmWord^.NextWordOffset;
      end;
  end;

  Function IsAsmInstruction: Boolean;
  begin
    If FindAsmWord (@AsmInstructions, Pointer (InstructionData)) then
      begin
        AsmToken := AsmToken_Instruction;
        IsAsmInstruction := True;
      end else IsAsmInstruction := False;
  end;

  Function IsSymbolToken: Boolean;
  begin
    IsSymbolToken := False;
    If AsmWordType = 0 then
      begin
        AsmToken := TAsmToken (AsmTokenString [1]);
        If (Length (AsmTokenString) = 1) and (AsmTokenString [1] <> #0) then IsSymbolToken := True;
      end;
  end;

  Function IsAsmOperand: Boolean;
  Var OperandData: POperandData;
  begin
    If FindAsmWord (@AsmOperands, Pointer (OperandData)) then
      begin
        AsmToken := OperandData^.Token;
        OperandRegisterType := OperandData^.RegisterType;
        OperandRegister := OperandData^.Register;
        NumericAsmConstant.Long := OperandData^.W1;
        IsAsmOperand := True;
      end else IsAsmOperand := False;
  end;

  Function IsAsmConstant: Boolean;
  Var StartIndex, EndIndex: Byte;
      Index: Integer;
      Constant: LongRec;

    Procedure ProcessHexNumber;
    Var Index: Integer;
    begin
      For Index := StartIndex to EndIndex do
        begin
          If NumericAsmConstant.WordH > $0FFF then AsmError (ConstantOutOfRange);
          NumericAsmConstant.Long := NumericAsmConstant.Long * 16;
          If (NumericAsmConstant.WordH = $FFFF) and (NumericAsmConstant.WordL > $FFF0) then AsmError (ConstantOutOfRange);
          If AsmTokenUpCaseString [Index] in ['0'..'9'] then
            Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('0')) else
              If AsmTokenUpCaseString [Index] in ['A'..'F'] then
                Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('A') + 10) else
                  AsmError (ErrorInIntegerConstant);
        end;
    end;

    Procedure ProcessDecNumber;
    Var Index: Integer;
    begin
      For Index := StartIndex to EndIndex do
        begin
          If NumericAsmConstant.WordH > $1999 then AsmError (ConstantOutOfRange);
          If (NumericAsmConstant.WordH = $1999) and (NumericAsmConstant.WordL > $9999) then AsmError (ConstantOutOfRange);
          NumericAsmConstant.Long := NumericAsmConstant.Long * 10;
          If (NumericAsmConstant.WordH = $FFFF) and (NumericAsmConstant.WordL > $FFFF - 9) then AsmError (ConstantOutOfRange);
          If AsmTokenUpCaseString [Index] in ['0'..'9'] then
            Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('0')) else
              AsmError (ErrorInIntegerConstant);
        end;
    end;

    Procedure ProcessOctNumber;
    Var Index: Integer;
    begin
      For Index := StartIndex to EndIndex do
        begin
          If NumericAsmConstant.WordH > $1FFF then AsmError (ConstantOutOfRange);
          NumericAsmConstant.Long := NumericAsmConstant.Long * 8;
          If (NumericAsmConstant.WordH = $FFFF) and (NumericAsmConstant.WordL > $FFFF - 7) then AsmError (ConstantOutOfRange);
          If AsmTokenUpCaseString [Index] in ['0'..'7'] then
            Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('0')) else
              AsmError (ErrorInIntegerConstant);
        end;
    end;

    Procedure ProcessBinNumber;
    Var Index: Integer;
    begin
      For Index := StartIndex to EndIndex do
        begin
          If NumericAsmConstant.WordH > $7FFF then AsmError (ConstantOutOfRange);
          NumericAsmConstant.Long := NumericAsmConstant.Long * 2;
          If (NumericAsmConstant.WordH = $FFFF) and (NumericAsmConstant.WordL > $FFFF - 1) then AsmError (ConstantOutOfRange);
          If AsmTokenUpCaseString [Index] in ['0'..'1'] then
            Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('0')) else
              AsmError (ErrorInIntegerConstant);
        end;
    end;

  begin
    IsAsmConstant := False;
    NumericAsmConstant.Long := 0;
    Case AsmWordType of
      1: begin
           AsmString := '';
           StartIndex := 2;
           EndIndex := Length (AsmTokenUpCaseString);
           If AsmTokenUpCaseString [1] = '$' then ProcessHexNumber else
             If AsmTokenUpCaseString [1] in ['0'..'9'] then
               begin
                 StartIndex := 1;
                 If AsmTokenUpCaseString [Length (AsmTokenUpCaseString)] in ['0'..'9'] then ProcessDecNumber else
                   begin
                     Dec (EndIndex);
                     Case AsmTokenUpCaseString [Length (AsmTokenUpCaseString)] of
                       'D': ProcessDecNumber;
                       'B': ProcessBinNumber;
                       'O': ProcessOctNumber;
                       'H': ProcessHexNumber;
                       else AsmError (ErrorInIntegerConstant);
                     end;
                   end;
               end else Exit;
           AsmToken := AsmToken_Constant;
           IsAsmConstant := True;
         end;
      2: begin
           For Index := 1 to Length (AsmString) do
             begin
               NumericAsmConstant.WordH := NumericAsmConstant.WordH and $00FF;
               NumericAsmConstant.Long := NumericAsmConstant.Long shl 8 + Byte (AsmString [Index]);
             end;
           AsmToken := AsmToken_Constant;
           IsAsmConstant := True;
         end;
    end;
  end;

  Procedure ProcessAsmIdentifier;
  Var IdToken: TToken;
      IdentifierDataPointer: Pointer;
      TypeDefinition: PTypeDefinition;
      VariableIdentifierData: PVariableIdentifierData;

    Procedure SetAsmVariableIdentifierData;
    begin
      With AsmIdentifierData do
        If vfVar in VariableIdentifierData^.Flags then
          begin
            AdditionalPointer := nil;
            Size := 4;
          end else begin
                     TypeDefinition := PointerFromOffsets (PVariableIdentifierData (
                                                             IdentifierDataPointer)^.UnitTypeOffsets);
                     Case TypeDefinition^.BaseType of
                       btRecord,
                       btObject: AdditionalPointer := TypeDefinition;
                       else      AdditionalPointer := nil;
                     end;
                     Size := TypeDefinition^.Size;
                   end;
    end;

  begin
    If AsmTokenString [1] = '&' then
      begin
        If Length (AsmTokenString) = 1 then AsmError (SyntaxError);
        Delete (AsmTokenString, 1, 1);
      end;
    CopyStringToCurrentIdentifier (AsmTokenString);
    AsmIdentifierData.AdditionalPointer := AsmIdentifierAdditionalData;

    If not IsValidAsmIdentifier (Pointer (Pointer (@AsmIdentifierData)^), IdToken,
                                 IdentifierDataPointer, CurrentIdentifierOffset) then
      AsmError (UnknownIdentifier);
    With AsmIdentifierData do
      begin
        ReferencedData.Long := $FFFFFFFF;
        Value_Lo := 0;
        Value_Hi := 0;
        Case IdToken of
          Token_ConstantIdentifier:
            Case PTypeDefinition (PointerFromOffsets (PConstantIdentifierData (IdentifierDataPointer)^.
                                                                                                UnitTypeOffsets))^.BaseType of
              btPointer,
              btInteger..btEnumeration: begin
                                          AdditionalPointer := nil;
                                          Value_Lo := PConstantIdentifierData (IdentifierDataPointer)^.OrdinalValue;
                                          Value_Hi := PConstantIdentifierData (IdentifierDataPointer)^.W2;
                                          Size := 0;
                                        end;
              else AsmError (InvalidSymbolReference);
            end;
          Token_TypeIdentifier:
            begin
              TypeDefinition := PointerFromOffsets (PTypeIdentifierData (IdentifierDataPointer)^.UnitTypeOffsets);
              begin
                Case TypeDefinition^.BaseType of
                  btRecord,
                  btObject: AdditionalPointer := TypeDefinition;
                  else      AdditionalPointer := nil;
                end;
                Size := TypeDefinition^.Size;
              end;
            end;
          Token_VariableIdentifier:
            begin
              VariableIdentifierData := IdentifierDataPointer;
              While vfAbsoluteVar in VariableIdentifierData^.Flags do
                VariableIdentifierData :=
                  PVariableIdentifierData (PointerFromOffsets (VariableIdentifierData^.AbsoluteVarDataOffsets));

              If Field in VariableIdentifierData^.Flags then
                begin
                  Value_Lo := VariableIdentifierData^.W1.Ofs;
                  Value_Hi := 0;
                  SetAsmVariableIdentifierData;
                end else If VariableIdentifierData^.Flags * VariableTypeMask = VariableAbsoluteAddress then
                  begin
                    Dec (ReferencedData.WordL);
                    Value_Lo := VariableIdentifierData^.W1.Ofs;
                    Value_Hi := VariableIdentifierData^.W1.Seg;
                    SetAsmVariableIdentifierData;
                  end else If VariableIdentifierData^.Flags * VariableTypeMask = LocalStackVariable then
                    begin
                      Dec (ReferencedData.WordL, 2);
                      Value_Lo := VariableIdentifierData^.W1.Ofs;
                      Value_Hi := 0;
                      SetAsmVariableIdentifierData;
                    end else begin
                               ReferencedData.WordL := VariableIdentifierData^.W1.Seg;
                               If VariableIdentifierData^.Flags = [] then
                                 ReferencedData.WordH := $8000 else
                                   ReferencedData.WordH := $C000;
                               ReferencedData.WordH :=
                                 ReferencedData.WordH or AddReferencedModule (Seg (VariableIdentifierData^));
                               Value_Lo := VariableIdentifierData^.W1.Ofs;
                               Value_Hi := 0;
                               SetAsmVariableIdentifierData;
                             end;
            end;
          Token_ProcedureIdentifier:
            begin
              If pfInline in PProcedureIdentifierData (IdentifierDataPointer)^.Flags then AsmError (InvalidSymbolReference);
              ReferencedData.WordL := PProcedureIdentifierData (IdentifierDataPointer)^.ProceduresRecordOffset;
              ReferencedData.WordH := AddReferencedModule (Seg (IdentifierDataPointer^));
              If pfFar in PProcedureIdentifierData (IdentifierDataPointer)^.Flags then Size := $FFFE else Size := $FFFF;
              AdditionalPointer := nil;
            end;
          Token_UnitIdentifier:
            begin
              AdditionalPointer := Ptr (PUnitIdentifierData (IdentifierDataPointer)^.UnitSegment, 0);
              Size := 0;
            end;
          Token_LabelIdentifier:
            begin
              If CurrentIdentifierOffset < Ofs (CurrentScopeIdentifierTableAddress^) then
                AsmError (LabelNotWithinCurrentBlock);
              ReferencedData.WordL := CurrentIdentifierOffset;
              ReferencedData.WordH := $4000 or AddReferencedModule (Seg (IdentifierDataPointer^));
              AdditionalPointer := nil;
              Size := $FFFD;
            end;
          Token_AsmSegmentReference:
            begin
              ReferencedData.WordL := 0;
              ReferencedData.WordH := Swap (Byte (IdentifierDataPointer^)) or
                                      AddReferencedModule (SymbolTable [stMain].Segment);
              AdditionalPointer := nil;
              Size := $FFF0;
            end;
          Token_AsmOffsetReference:
            begin
              If ProcedureIdentifierDataOffset = 0 then AsmError (InvalidSymbolReference);
              Case Byte (IdentifierDataPointer^) of { In TPC only one byte, in TPC16 word }
                0: begin                    { @LOCALS }
                     AdditionalPointer := nil;
                     Value_Lo := - (ProgramBlockMaxStackFrameOffset and $FFFE);
                     Value_Hi := 0;
                     Size := 0;
                   end;
                1: begin                    { @PARAMS }
                     AdditionalPointer := nil;
                     Value_Lo := PushedParametersSize;
                     Value_Hi := 0;
                     Size := 0;
                   end;
                2: begin                    { @RESULT }
                     TypeDefinition := PointerFromOffsets (PProcedureIdentifierData (Ptr (SymbolTable [stMain].Segment,
                       ProcedureIdentifierDataOffset))^.ProcedureTypeDefinition.ResultTypeOffset);
                     If Ofs (TypeDefinition^) = 0 then AsmError (InvalidSymbolReference);
                     Value_Lo := OffsetAfterLastParameter;
                     Value_Hi := 0;
                     Size := 4;
                     If TypeDefinition^.BaseType <> btString then
                       begin
                         If pfAssembler in PProcedureIdentifierData (Ptr (SymbolTable [stMain].Segment,
                                            ProcedureIdentifierDataOffset))^.Flags then AsmError (InvalidSymbolReference);
                         Value_Lo := FunctionResultNegativeSize;
                         Size := TypeDefinition^.Size;
                       end;
                     ReferencedData.Long := $FFFFFFFD;
                     AdditionalPointer := nil;
                   end;
              end;
            end;
          else AsmError (InvalidSymbolReference);
        end;
        If Size >= $FFFD then CodeIdentifierReference := True else CodeIdentifierReference := False;
      end;

    AsmIdentifierAdditionalData := AsmIdentifierData.AdditionalPointer;
    AsmToken := AsmToken_Identifier;
  end;

begin
  AsmToken := AsmToken_Semicolon;
  If LineCharacterCounter <> 0 then
    begin
      SeparateAsmWord;
      If not BeforeOperands or not IsAsmInstruction then
        begin
          If not IsSymbolToken then
            If not IsAsmOperand then
              If not IsAsmConstant then
                If not BeforeOperands then ProcessAsmIdentifier;
        end;
    end;
  If AsmToken = AsmToken_00 then AsmToken := AsmToken_01;
end;
If
AsmToken
is expected assembler token this procedure gets next one, otherwise it reports error.
Procedure ExpectAsmTokenAndGetNext (ExpectedAsmToken: TAsmToken);
begin
  If AsmToken <> ExpectedAsmToken then AsmError (SyntaxError);
  GetNextAsmToken;
end;
If assembler token is
AsmTokenToCheck
, this function gets the next token and returns
True
otherwise it returns
False
.
Function CheckAndGetNextAsmToken (AsmTokenToCheck: TAsmToken): Boolean;
begin
  CheckAndGetNextAsmToken := True;
  If AsmToken = AsmTokenToCheck then GetNextAsmToken else CheckAndGetNextAsmToken := False;
end;
This function finds
AsmToken
in table of operator procedures and returns its procedure and
True
, if token is not found it returns
False
.
Function FindAsmTokenInTable (Table: POperatorProc; Var ExpressionProc: TExpressionProc): Boolean;
begin
  FindAsmTokenInTable := False;
  While Ord (Table^.Token) <> 0 do
    begin
      If Table^.Token = AsmToken then
        begin
          ExpressionProc := Table^.Proc;
          GetNextAsmToken;
          FindAsmTokenInTable := True;
          Exit;
        end;
      Inc (Table);
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
Procedure GetNextAsmToken;

  Procedure SeparateAsmWord;
  Var C: Char;

    Function IsLetterOrNumberOrSpecialCharacter: Boolean;
    begin
      Case C of
        '0'..'9',
        'A'..'Z',
        'a'..'z',
        '_',
        '$',
        '@',
        '&',
        '?' : IsLetterOrNumberOrSpecialCharacter := True;
        else  IsLetterOrNumberOrSpecialCharacter := False;
      end;
    end;

    Function IsString: Boolean;
    Var StringChar, StringDelimiter: Char;
    begin
      IsString := False;
      If not (C in ['''', '"']) then Exit;
      StringDelimiter := C;
      AsmString := '';
      IsString := True;
      Repeat
        StringChar := GetNextChar;
        If StringChar = #0 then AsmError (StringConstantExceedsLine);
        If StringChar = StringDelimiter then
          begin
            StringChar := GetNextChar;
            If StringChar <> StringDelimiter then Exit;
          end;
        If Byte (AsmString [0]) = 127 then AsmError (StringConstantExceedsLine);
        Inc (AsmString [0]);
        AsmString [Length (AsmString)] := StringChar;
      until False;
    end;

  begin
    AsmTokenString := '';
    AsmTokenUpCaseString := '';
    AsmWordHash := 0;
    C := GetFirstNonBlankCharacter;
    AsmSourceErrorPosition := AsmCharacterPointer;
    If C <> #0 then
      begin
        If IsLetterOrNumberOrSpecialCharacter then
          begin
            Repeat
              If Length (AsmTokenString) < 32 then
                begin
                  Inc (AsmTokenString [0]);
                  Inc (AsmTokenUpCaseString [0]);
                  AsmTokenString [Byte (AsmTokenString [0])] := C;
                  AsmTokenUpCaseString [Byte (AsmTokenUpCaseString [0])] := UpCase (C);
                  Inc (AsmWordHash, Ord (UpCase (C)) - 1);
                end;
              C := GetNextChar;
            until (C = #0) or not IsLetterOrNumberOrSpecialCharacter;
            AsmWordType := 1;
            If AsmTokenUpCaseString <> 'END' then Exit;
          end else begin
                     AsmWordType := 2;
                     If IsString then Exit;
                     AsmWordType := 0;
                     AsmTokenString := C;
                     Case C of
                       ';', '{':
{$B+}
                       else If (C <> '(') or (GetNextChar <> '*') then Exit;
{$B-}
                     end;
                   end;
      end;
    AsmWordType := 0;
    AsmTokenString := ';';
    LineCharacterCounter := 0;
    AsmCharacterPointer := AsmSourceErrorPosition;
  end;

  Function FindAsmWord (AsmList: PIdentifierList; Var AsmWordData: Pointer): Boolean;
  Var AsmWord: PAsmWord absolute AsmWordData;
      AsmWordOffset: Word absolute AsmWordData;
  begin
    FindAsmWord := False;
    If AsmWordType <> 1 then Exit;
    AsmWord := Ptr (DSeg, AsmList^.Offset [AsmWordHash and AsmList^.Mask]);
    While AsmWordOffset <> 0 do
      begin
        AsmToken := AsmToken_00;
        If AsmWord^.Str = AsmTokenUpCaseString then
          begin
            Inc (AsmWordOffset, Length (AsmWord^.Str) + 3);
            FindAsmWord := True;
            Exit;
          end;
        AsmWordOffset := AsmWord^.NextWordOffset;
      end;
  end;

  Function IsAsmInstruction: Boolean;
  begin
    If FindAsmWord (@AsmInstructions, Pointer (InstructionData)) then
      begin
        AsmToken := AsmToken_Instruction;
        IsAsmInstruction := True;
      end else IsAsmInstruction := False;
  end;

  Function IsSymbolToken: Boolean;
  begin
    IsSymbolToken := False;
    If AsmWordType = 0 then
      begin
        AsmToken := TAsmToken (AsmTokenString [1]);
        If (Length (AsmTokenString) = 1) and (AsmTokenString [1] <> #0) then IsSymbolToken := True;
      end;
  end;

  Function IsAsmOperand: Boolean;
  Var OperandData: POperandData;
  begin
    If FindAsmWord (@AsmOperands, Pointer (OperandData)) then
      begin
        AsmToken := OperandData^.Token;
        OperandRegisterType := OperandData^.RegisterType;
        OperandRegister := OperandData^.Register;
        NumericAsmConstant.Long := OperandData^.W1;
        IsAsmOperand := True;
      end else IsAsmOperand := False;
  end;

  Function IsAsmConstant: Boolean;
  Var StartIndex, EndIndex: Byte;
      Index: Integer;
      Constant: LongRec;

    Procedure ProcessHexNumber;
    Var Index: Integer;
    begin
      For Index := StartIndex to EndIndex do
        begin
          If NumericAsmConstant.WordH > $0FFF then AsmError (ConstantOutOfRange);
          NumericAsmConstant.Long := NumericAsmConstant.Long * 16;
          If (NumericAsmConstant.WordH = $FFFF) and (NumericAsmConstant.WordL > $FFF0) then AsmError (ConstantOutOfRange);
          If AsmTokenUpCaseString [Index] in ['0'..'9'] then
            Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('0')) else
              If AsmTokenUpCaseString [Index] in ['A'..'F'] then
                Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('A') + 10) else
                  AsmError (ErrorInIntegerConstant);
        end;
    end;

    Procedure ProcessDecNumber;
    Var Index: Integer;
    begin
      For Index := StartIndex to EndIndex do
        begin
          If NumericAsmConstant.WordH > $1999 then AsmError (ConstantOutOfRange);
          If (NumericAsmConstant.WordH = $1999) and (NumericAsmConstant.WordL > $9999) then AsmError (ConstantOutOfRange);
          NumericAsmConstant.Long := NumericAsmConstant.Long * 10;
          If (NumericAsmConstant.WordH = $FFFF) and (NumericAsmConstant.WordL > $FFFF - 9) then AsmError (ConstantOutOfRange);
          If AsmTokenUpCaseString [Index] in ['0'..'9'] then
            Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('0')) else
              AsmError (ErrorInIntegerConstant);
        end;
    end;

    Procedure ProcessOctNumber;
    Var Index: Integer;
    begin
      For Index := StartIndex to EndIndex do
        begin
          If NumericAsmConstant.WordH > $1FFF then AsmError (ConstantOutOfRange);
          NumericAsmConstant.Long := NumericAsmConstant.Long * 8;
          If (NumericAsmConstant.WordH = $FFFF) and (NumericAsmConstant.WordL > $FFFF - 7) then AsmError (ConstantOutOfRange);
          If AsmTokenUpCaseString [Index] in ['0'..'7'] then
            Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('0')) else
              AsmError (ErrorInIntegerConstant);
        end;
    end;

    Procedure ProcessBinNumber;
    Var Index: Integer;
    begin
      For Index := StartIndex to EndIndex do
        begin
          If NumericAsmConstant.WordH > $7FFF then AsmError (ConstantOutOfRange);
          NumericAsmConstant.Long := NumericAsmConstant.Long * 2;
          If (NumericAsmConstant.WordH = $FFFF) and (NumericAsmConstant.WordL > $FFFF - 1) then AsmError (ConstantOutOfRange);
          If AsmTokenUpCaseString [Index] in ['0'..'1'] then
            Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('0')) else
              AsmError (ErrorInIntegerConstant);
        end;
    end;

  begin
    IsAsmConstant := False;
    NumericAsmConstant.Long := 0;
    Case AsmWordType of
      1: begin
           AsmString := '';
           StartIndex := 2;
           EndIndex := Length (AsmTokenUpCaseString);
           If AsmTokenUpCaseString [1] = '$' then ProcessHexNumber else
             If AsmTokenUpCaseString [1] in ['0'..'9'] then
               begin
                 StartIndex := 1;
                 If AsmTokenUpCaseString [Length (AsmTokenUpCaseString)] in ['0'..'9'] then ProcessDecNumber else
                   begin
                     Dec (EndIndex);
                     Case AsmTokenUpCaseString [Length (AsmTokenUpCaseString)] of
                       'D': ProcessDecNumber;
                       'B': ProcessBinNumber;
                       'O': ProcessOctNumber;
                       'H': ProcessHexNumber;
                       else AsmError (ErrorInIntegerConstant);
                     end;
                   end;
               end else Exit;
           AsmToken := AsmToken_Constant;
           IsAsmConstant := True;
         end;
      2: begin
           For Index := 1 to Length (AsmString) do
             begin
               NumericAsmConstant.WordH := NumericAsmConstant.WordH and $00FF;
               NumericAsmConstant.Long := NumericAsmConstant.Long shl 8 + Byte (AsmString [Index]);
             end;
           AsmToken := AsmToken_Constant;
           IsAsmConstant := True;
         end;
    end;
  end;

  Procedure ProcessAsmIdentifier;
  Var IdToken: TToken;
      IdentifierDataPointer: Pointer;
      TypeDefinition: PTypeDefinition;
      VariableIdentifierData: PVariableIdentifierData;

    Procedure SetAsmVariableIdentifierData;
    begin
      With AsmIdentifierData do
        If vfVar in VariableIdentifierData^.Flags then
          begin
            AdditionalPointer := nil;
            Size := 4;
          end else begin
                     TypeDefinition := PointerFromOffsets (PVariableIdentifierData (
                                                             IdentifierDataPointer)^.UnitTypeOffsets);
                     Case TypeDefinition^.BaseType of
                       btRecord,
                       btObject: AdditionalPointer := TypeDefinition;
                       else      AdditionalPointer := nil;
                     end;
                     Size := TypeDefinition^.Size;
                   end;
    end;

  begin
    If AsmTokenString [1] = '&' then
      begin
        If Length (AsmTokenString) = 1 then AsmError (SyntaxError);
        Delete (AsmTokenString, 1, 1);
      end;
    CopyStringToCurrentIdentifier (AsmTokenString);
    AsmIdentifierData.AdditionalPointer := AsmIdentifierAdditionalData;

    If not IsValidAsmIdentifier (Pointer (Pointer (@AsmIdentifierData)^), IdToken,
                                 IdentifierDataPointer, CurrentIdentifierOffset) then
      AsmError (UnknownIdentifier);
    With AsmIdentifierData do
      begin
        ReferencedData.Long := $FFFFFFFF;
        Value_Lo := 0;
        Value_Hi := 0;
        Case IdToken of
          Token_ConstantIdentifier:
            Case PTypeDefinition (PointerFromOffsets (PConstantIdentifierData (IdentifierDataPointer)^.
                                                                                                UnitTypeOffsets))^.BaseType of
              btPointer,
              btInteger..btEnumeration: begin
                                          AdditionalPointer := nil;
                                          Value_Lo := PConstantIdentifierData (IdentifierDataPointer)^.OrdinalValue;
                                          Value_Hi := PConstantIdentifierData (IdentifierDataPointer)^.W2;
                                          Size := 0;
                                        end;
              else AsmError (InvalidSymbolReference);
            end;
          Token_TypeIdentifier:
            begin
              TypeDefinition := PointerFromOffsets (PTypeIdentifierData (IdentifierDataPointer)^.UnitTypeOffsets);
              begin
                Case TypeDefinition^.BaseType of
                  btRecord,
                  btObject: AdditionalPointer := TypeDefinition;
                  else      AdditionalPointer := nil;
                end;
                Size := TypeDefinition^.Size;
              end;
            end;
          Token_VariableIdentifier:
            begin
              VariableIdentifierData := IdentifierDataPointer;
              While vfAbsoluteVar in VariableIdentifierData^.Flags do
                VariableIdentifierData :=
                  PVariableIdentifierData (PointerFromOffsets (VariableIdentifierData^.AbsoluteVarDataOffsets));

              If Field in VariableIdentifierData^.Flags then
                begin
                  Value_Lo := VariableIdentifierData^.W1.Ofs;
                  Value_Hi := 0;
                  SetAsmVariableIdentifierData;
                end else If VariableIdentifierData^.Flags * VariableTypeMask = VariableAbsoluteAddress then
                  begin
                    Dec (ReferencedData.WordL);
                    Value_Lo := VariableIdentifierData^.W1.Ofs;
                    Value_Hi := VariableIdentifierData^.W1.Seg;
                    SetAsmVariableIdentifierData;
                  end else If VariableIdentifierData^.Flags * VariableTypeMask = LocalStackVariable then
                    begin
                      Dec (ReferencedData.WordL, 2);
                      Value_Lo := VariableIdentifierData^.W1.Ofs;
                      Value_Hi := 0;
                      SetAsmVariableIdentifierData;
                    end else begin
                               ReferencedData.WordL := VariableIdentifierData^.W1.Seg;
                               If VariableIdentifierData^.Flags = [] then
                                 ReferencedData.WordH := $8000 else
                                   ReferencedData.WordH := $C000;
                               ReferencedData.WordH :=
                                 ReferencedData.WordH or AddReferencedModule (Seg (VariableIdentifierData^));
                               Value_Lo := VariableIdentifierData^.W1.Ofs;
                               Value_Hi := 0;
                               SetAsmVariableIdentifierData;
                             end;
            end;
          Token_ProcedureIdentifier:
            begin
              If pfInline in PProcedureIdentifierData (IdentifierDataPointer)^.Flags then AsmError (InvalidSymbolReference);
              ReferencedData.WordL := PProcedureIdentifierData (IdentifierDataPointer)^.ProceduresRecordOffset;
              ReferencedData.WordH := AddReferencedModule (Seg (IdentifierDataPointer^));
              If pfFar in PProcedureIdentifierData (IdentifierDataPointer)^.Flags then Size := $FFFE else Size := $FFFF;
              AdditionalPointer := nil;
            end;
          Token_UnitIdentifier:
            begin
              AdditionalPointer := Ptr (PUnitIdentifierData (IdentifierDataPointer)^.UnitSegment, 0);
              Size := 0;
            end;
          Token_LabelIdentifier:
            begin
              If CurrentIdentifierOffset < Ofs (CurrentScopeIdentifierTableAddress^) then
                AsmError (LabelNotWithinCurrentBlock);
              ReferencedData.WordL := CurrentIdentifierOffset;
              ReferencedData.WordH := $4000 or AddReferencedModule (Seg (IdentifierDataPointer^));
              AdditionalPointer := nil;
              Size := $FFFD;
            end;
          Token_AsmSegmentReference:
            begin
              ReferencedData.WordL := 0;
              ReferencedData.WordH := Swap (Byte (IdentifierDataPointer^)) or
                                      AddReferencedModule (SymbolTable [stMain].Segment);
              AdditionalPointer := nil;
              Size := $FFF0;
            end;
          Token_AsmOffsetReference:
            begin
              If ProcedureIdentifierDataOffset = 0 then AsmError (InvalidSymbolReference);
              Case Byte (IdentifierDataPointer^) of { In TPC only one byte, in TPC16 word }
                0: begin                    { @LOCALS }
                     AdditionalPointer := nil;
                     Value_Lo := - (ProgramBlockMaxStackFrameOffset and $FFFE);
                     Value_Hi := 0;
                     Size := 0;
                   end;
                1: begin                    { @PARAMS }
                     AdditionalPointer := nil;
                     Value_Lo := PushedParametersSize;
                     Value_Hi := 0;
                     Size := 0;
                   end;
                2: begin                    { @RESULT }
                     TypeDefinition := PointerFromOffsets (PProcedureIdentifierData (Ptr (SymbolTable [stMain].Segment,
                       ProcedureIdentifierDataOffset))^.ProcedureTypeDefinition.ResultTypeOffset);
                     If Ofs (TypeDefinition^) = 0 then AsmError (InvalidSymbolReference);
                     Value_Lo := OffsetAfterLastParameter;
                     Value_Hi := 0;
                     Size := 4;
                     If TypeDefinition^.BaseType <> btString then
                       begin
                         If pfAssembler in PProcedureIdentifierData (Ptr (SymbolTable [stMain].Segment,
                                            ProcedureIdentifierDataOffset))^.Flags then AsmError (InvalidSymbolReference);
                         Value_Lo := FunctionResultNegativeSize;
                         Size := TypeDefinition^.Size;
                       end;
                     ReferencedData.Long := $FFFFFFFD;
                     AdditionalPointer := nil;
                   end;
              end;
            end;
          else AsmError (InvalidSymbolReference);
        end;
        If Size >= $FFFD then CodeIdentifierReference := True else CodeIdentifierReference := False;
      end;

    AsmIdentifierAdditionalData := AsmIdentifierData.AdditionalPointer;
    AsmToken := AsmToken_Identifier;
  end;

begin
  AsmToken := AsmToken_Semicolon;
  If LineCharacterCounter <> 0 then
    begin
      SeparateAsmWord;
      If not BeforeOperands or not IsAsmInstruction then
        begin
          If not IsSymbolToken then
            If not IsAsmOperand then
              If not IsAsmConstant then
                If not BeforeOperands then ProcessAsmIdentifier;
        end;
    end;
  If AsmToken = AsmToken_00 then AsmToken := AsmToken_01;
end;
```

### Code Block 2

```pascal
Procedure ExpectAsmTokenAndGetNext (ExpectedAsmToken: TAsmToken);
begin
  If AsmToken <> ExpectedAsmToken then AsmError (SyntaxError);
  GetNextAsmToken;
end;
```

### Code Block 3

```pascal
Function CheckAndGetNextAsmToken (AsmTokenToCheck: TAsmToken): Boolean;
begin
  CheckAndGetNextAsmToken := True;
  If AsmToken = AsmTokenToCheck then GetNextAsmToken else CheckAndGetNextAsmToken := False;
end;
```

### Code Block 4

```pascal
Function FindAsmTokenInTable (Table: POperatorProc; Var ExpressionProc: TExpressionProc): Boolean;
begin
  FindAsmTokenInTable := False;
  While Ord (Table^.Token) <> 0 do
    begin
      If Table^.Token = AsmToken then
        begin
          ExpressionProc := Table^.Proc;
          GetNextAsmToken;
          FindAsmTokenInTable := True;
          Exit;
        end;
      Inc (Table);
    end;
end;
```

### Code Block 5

```pascal
Procedure GetNextAsmToken;

  Procedure SeparateAsmWord;
  Var C: Char;

    Function IsLetterOrNumberOrSpecialCharacter: Boolean;
    begin
      Case C of
        '0'..'9',
        'A'..'Z',
        'a'..'z',
        '_',
        '$',
        '@',
        '&',
        '?' : IsLetterOrNumberOrSpecialCharacter := True;
        else  IsLetterOrNumberOrSpecialCharacter := False;
      end;
    end;

    Function IsString: Boolean;
    Var StringChar, StringDelimiter: Char;
    begin
      IsString := False;
      If not (C in ['''', '"']) then Exit;
      StringDelimiter := C;
      AsmString := '';
      IsString := True;
      Repeat
        StringChar := GetNextChar;
        If StringChar = #0 then AsmError (StringConstantExceedsLine);
        If StringChar = StringDelimiter then
          begin
            StringChar := GetNextChar;
            If StringChar <> StringDelimiter then Exit;
          end;
        If Byte (AsmString [0]) = 127 then AsmError (StringConstantExceedsLine);
        Inc (AsmString [0]);
        AsmString [Length (AsmString)] := StringChar;
      until False;
    end;

  begin
    AsmTokenString := '';
    AsmTokenUpCaseString := '';
    AsmWordHash := 0;
    C := GetFirstNonBlankCharacter;
    AsmSourceErrorPosition := AsmCharacterPointer;
    If C <> #0 then
      begin
        If IsLetterOrNumberOrSpecialCharacter then
          begin
            Repeat
              If Length (AsmTokenString) < 32 then
                begin
                  Inc (AsmTokenString [0]);
                  Inc (AsmTokenUpCaseString [0]);
                  AsmTokenString [Byte (AsmTokenString [0])] := C;
                  AsmTokenUpCaseString [Byte (AsmTokenUpCaseString [0])] := UpCase (C);
                  Inc (AsmWordHash, Ord (UpCase (C)) - 1);
                end;
              C := GetNextChar;
            until (C = #0) or not IsLetterOrNumberOrSpecialCharacter;
            AsmWordType := 1;
            If AsmTokenUpCaseString <> 'END' then Exit;
          end else begin
                     AsmWordType := 2;
                     If IsString then Exit;
                     AsmWordType := 0;
                     AsmTokenString := C;
                     Case C of
                       ';', '{':
{$B+}
                       else If (C <> '(') or (GetNextChar <> '*') then Exit;
{$B-}
                     end;
                   end;
      end;
    AsmWordType := 0;
    AsmTokenString := ';';
    LineCharacterCounter := 0;
    AsmCharacterPointer := AsmSourceErrorPosition;
  end;

  Function FindAsmWord (AsmList: PIdentifierList; Var AsmWordData: Pointer): Boolean;
  Var AsmWord: PAsmWord absolute AsmWordData;
      AsmWordOffset: Word absolute AsmWordData;
  begin
    FindAsmWord := False;
    If AsmWordType <> 1 then Exit;
    AsmWord := Ptr (DSeg, AsmList^.Offset [AsmWordHash and AsmList^.Mask]);
    While AsmWordOffset <> 0 do
      begin
        AsmToken := AsmToken_00;
        If AsmWord^.Str = AsmTokenUpCaseString then
          begin
            Inc (AsmWordOffset, Length (AsmWord^.Str) + 3);
            FindAsmWord := True;
            Exit;
          end;
        AsmWordOffset := AsmWord^.NextWordOffset;
      end;
  end;

  Function IsAsmInstruction: Boolean;
  begin
    If FindAsmWord (@AsmInstructions, Pointer (InstructionData)) then
      begin
        AsmToken := AsmToken_Instruction;
        IsAsmInstruction := True;
      end else IsAsmInstruction := False;
  end;

  Function IsSymbolToken: Boolean;
  begin
    IsSymbolToken := False;
    If AsmWordType = 0 then
      begin
        AsmToken := TAsmToken (AsmTokenString [1]);
        If (Length (AsmTokenString) = 1) and (AsmTokenString [1] <> #0) then IsSymbolToken := True;
      end;
  end;

  Function IsAsmOperand: Boolean;
  Var OperandData: POperandData;
  begin
    If FindAsmWord (@AsmOperands, Pointer (OperandData)) then
      begin
        AsmToken := OperandData^.Token;
        OperandRegisterType := OperandData^.RegisterType;
        OperandRegister := OperandData^.Register;
        NumericAsmConstant.Long := OperandData^.W1;
        IsAsmOperand := True;
      end else IsAsmOperand := False;
  end;

  Function IsAsmConstant: Boolean;
  Var StartIndex, EndIndex: Byte;
      Index: Integer;
      Constant: LongRec;

    Procedure ProcessHexNumber;
    Var Index: Integer;
    begin
      For Index := StartIndex to EndIndex do
        begin
          If NumericAsmConstant.WordH > $0FFF then AsmError (ConstantOutOfRange);
          NumericAsmConstant.Long := NumericAsmConstant.Long * 16;
          If (NumericAsmConstant.WordH = $FFFF) and (NumericAsmConstant.WordL > $FFF0) then AsmError (ConstantOutOfRange);
          If AsmTokenUpCaseString [Index] in ['0'..'9'] then
            Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('0')) else
              If AsmTokenUpCaseString [Index] in ['A'..'F'] then
                Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('A') + 10) else
                  AsmError (ErrorInIntegerConstant);
        end;
    end;

    Procedure ProcessDecNumber;
    Var Index: Integer;
    begin
      For Index := StartIndex to EndIndex do
        begin
          If NumericAsmConstant.WordH > $1999 then AsmError (ConstantOutOfRange);
          If (NumericAsmConstant.WordH = $1999) and (NumericAsmConstant.WordL > $9999) then AsmError (ConstantOutOfRange);
          NumericAsmConstant.Long := NumericAsmConstant.Long * 10;
          If (NumericAsmConstant.WordH = $FFFF) and (NumericAsmConstant.WordL > $FFFF - 9) then AsmError (ConstantOutOfRange);
          If AsmTokenUpCaseString [Index] in ['0'..'9'] then
            Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('0')) else
              AsmError (ErrorInIntegerConstant);
        end;
    end;

    Procedure ProcessOctNumber;
    Var Index: Integer;
    begin
      For Index := StartIndex to EndIndex do
        begin
          If NumericAsmConstant.WordH > $1FFF then AsmError (ConstantOutOfRange);
          NumericAsmConstant.Long := NumericAsmConstant.Long * 8;
          If (NumericAsmConstant.WordH = $FFFF) and (NumericAsmConstant.WordL > $FFFF - 7) then AsmError (ConstantOutOfRange);
          If AsmTokenUpCaseString [Index] in ['0'..'7'] then
            Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('0')) else
              AsmError (ErrorInIntegerConstant);
        end;
    end;

    Procedure ProcessBinNumber;
    Var Index: Integer;
    begin
      For Index := StartIndex to EndIndex do
        begin
          If NumericAsmConstant.WordH > $7FFF then AsmError (ConstantOutOfRange);
          NumericAsmConstant.Long := NumericAsmConstant.Long * 2;
          If (NumericAsmConstant.WordH = $FFFF) and (NumericAsmConstant.WordL > $FFFF - 1) then AsmError (ConstantOutOfRange);
          If AsmTokenUpCaseString [Index] in ['0'..'1'] then
            Inc (NumericAsmConstant.Long, Ord (AsmTokenUpCaseString [Index]) - Ord ('0')) else
              AsmError (ErrorInIntegerConstant);
        end;
    end;

  begin
    IsAsmConstant := False;
    NumericAsmConstant.Long := 0;
    Case AsmWordType of
      1: begin
           AsmString := '';
           StartIndex := 2;
           EndIndex := Length (AsmTokenUpCaseString);
           If AsmTokenUpCaseString [1] = '$' then ProcessHexNumber else
             If AsmTokenUpCaseString [1] in ['0'..'9'] then
               begin
                 StartIndex := 1;
                 If AsmTokenUpCaseString [Length (AsmTokenUpCaseString)] in ['0'..'9'] then ProcessDecNumber else
                   begin
                     Dec (EndIndex);
                     Case AsmTokenUpCaseString [Length (AsmTokenUpCaseString)] of
                       'D': ProcessDecNumber;
                       'B': ProcessBinNumber;
                       'O': ProcessOctNumber;
                       'H': ProcessHexNumber;
                       else AsmError (ErrorInIntegerConstant);
                     end;
                   end;
               end else Exit;
           AsmToken := AsmToken_Constant;
           IsAsmConstant := True;
         end;
      2: begin
           For Index := 1 to Length (AsmString) do
             begin
               NumericAsmConstant.WordH := NumericAsmConstant.WordH and $00FF;
               NumericAsmConstant.Long := NumericAsmConstant.Long shl 8 + Byte (AsmString [Index]);
             end;
           AsmToken := AsmToken_Constant;
           IsAsmConstant := True;
         end;
    end;
  end;

  Procedure ProcessAsmIdentifier;
  Var IdToken: TToken;
      IdentifierDataPointer: Pointer;
      TypeDefinition: PTypeDefinition;
      VariableIdentifierData: PVariableIdentifierData;

    Procedure SetAsmVariableIdentifierData;
    begin
      With AsmIdentifierData do
        If vfVar in VariableIdentifierData^.Flags then
          begin
            AdditionalPointer := nil;
            Size := 4;
          end else begin
                     TypeDefinition := PointerFromOffsets (PVariableIdentifierData (
                                                             IdentifierDataPointer)^.UnitTypeOffsets);
                     Case TypeDefinition^.BaseType of
                       btRecord,
                       btObject: AdditionalPointer := TypeDefinition;
                       else      AdditionalPointer := nil;
                     end;
                     Size := TypeDefinition^.Size;
                   end;
    end;

  begin
    If AsmTokenString [1] = '&' then
      begin
        If Length (AsmTokenString) = 1 then AsmError (SyntaxError);
        Delete (AsmTokenString, 1, 1);
      end;
    CopyStringToCurrentIdentifier (AsmTokenString);
    AsmIdentifierData.AdditionalPointer := AsmIdentifierAdditionalData;

    If not IsValidAsmIdentifier (Pointer (Pointer (@AsmIdentifierData)^), IdToken,
                                 IdentifierDataPointer, CurrentIdentifierOffset) then
      AsmError (UnknownIdentifier);
    With AsmIdentifierData do
      begin
        ReferencedData.Long := $FFFFFFFF;
        Value_Lo := 0;
        Value_Hi := 0;
        Case IdToken of
          Token_ConstantIdentifier:
            Case PTypeDefinition (PointerFromOffsets (PConstantIdentifierData (IdentifierDataPointer)^.
                                                                                                UnitTypeOffsets))^.BaseType of
              btPointer,
              btInteger..btEnumeration: begin
                                          AdditionalPointer := nil;
                                          Value_Lo := PConstantIdentifierData (IdentifierDataPointer)^.OrdinalValue;
                                          Value_Hi := PConstantIdentifierData (IdentifierDataPointer)^.W2;
                                          Size := 0;
                                        end;
              else AsmError (InvalidSymbolReference);
            end;
          Token_TypeIdentifier:
            begin
              TypeDefinition := PointerFromOffsets (PTypeIdentifierData (IdentifierDataPointer)^.UnitTypeOffsets);
              begin
                Case TypeDefinition^.BaseType of
                  btRecord,
                  btObject: AdditionalPointer := TypeDefinition;
                  else      AdditionalPointer := nil;
                end;
                Size := TypeDefinition^.Size;
              end;
            end;
          Token_VariableIdentifier:
            begin
              VariableIdentifierData := IdentifierDataPointer;
              While vfAbsoluteVar in VariableIdentifierData^.Flags do
                VariableIdentifierData :=
                  PVariableIdentifierData (PointerFromOffsets (VariableIdentifierData^.AbsoluteVarDataOffsets));

              If Field in VariableIdentifierData^.Flags then
                begin
                  Value_Lo := VariableIdentifierData^.W1.Ofs;
                  Value_Hi := 0;
                  SetAsmVariableIdentifierData;
                end else If VariableIdentifierData^.Flags * VariableTypeMask = VariableAbsoluteAddress then
                  begin
                    Dec (ReferencedData.WordL);
                    Value_Lo := VariableIdentifierData^.W1.Ofs;
                    Value_Hi := VariableIdentifierData^.W1.Seg;
                    SetAsmVariableIdentifierData;
                  end else If VariableIdentifierData^.Flags * VariableTypeMask = LocalStackVariable then
                    begin
                      Dec (ReferencedData.WordL, 2);
                      Value_Lo := VariableIdentifierData^.W1.Ofs;
                      Value_Hi := 0;
                      SetAsmVariableIdentifierData;
                    end else begin
                               ReferencedData.WordL := VariableIdentifierData^.W1.Seg;
                               If VariableIdentifierData^.Flags = [] then
                                 ReferencedData.WordH := $8000 else
                                   ReferencedData.WordH := $C000;
                               ReferencedData.WordH :=
                                 ReferencedData.WordH or AddReferencedModule (Seg (VariableIdentifierData^));
                               Value_Lo := VariableIdentifierData^.W1.Ofs;
                               Value_Hi := 0;
                               SetAsmVariableIdentifierData;
                             end;
            end;
          Token_ProcedureIdentifier:
            begin
              If pfInline in PProcedureIdentifierData (IdentifierDataPointer)^.Flags then AsmError (InvalidSymbolReference);
              ReferencedData.WordL := PProcedureIdentifierData (IdentifierDataPointer)^.ProceduresRecordOffset;
              ReferencedData.WordH := AddReferencedModule (Seg (IdentifierDataPointer^));
              If pfFar in PProcedureIdentifierData (IdentifierDataPointer)^.Flags then Size := $FFFE else Size := $FFFF;
              AdditionalPointer := nil;
            end;
          Token_UnitIdentifier:
            begin
              AdditionalPointer := Ptr (PUnitIdentifierData (IdentifierDataPointer)^.UnitSegment, 0);
              Size := 0;
            end;
          Token_LabelIdentifier:
            begin
              If CurrentIdentifierOffset < Ofs (CurrentScopeIdentifierTableAddress^) then
                AsmError (LabelNotWithinCurrentBlock);
              ReferencedData.WordL := CurrentIdentifierOffset;
              ReferencedData.WordH := $4000 or AddReferencedModule (Seg (IdentifierDataPointer^));
              AdditionalPointer := nil;
              Size := $FFFD;
            end;
          Token_AsmSegmentReference:
            begin
              ReferencedData.WordL := 0;
              ReferencedData.WordH := Swap (Byte (IdentifierDataPointer^)) or
                                      AddReferencedModule (SymbolTable [stMain].Segment);
              AdditionalPointer := nil;
              Size := $FFF0;
            end;
          Token_AsmOffsetReference:
            begin
              If ProcedureIdentifierDataOffset = 0 then AsmError (InvalidSymbolReference);
              Case Byte (IdentifierDataPointer^) of { In TPC only one byte, in TPC16 word }
                0: begin                    { @LOCALS }
                     AdditionalPointer := nil;
                     Value_Lo := - (ProgramBlockMaxStackFrameOffset and $FFFE);
                     Value_Hi := 0;
                     Size := 0;
                   end;
                1: begin                    { @PARAMS }
                     AdditionalPointer := nil;
                     Value_Lo := PushedParametersSize;
                     Value_Hi := 0;
                     Size := 0;
                   end;
                2: begin                    { @RESULT }
                     TypeDefinition := PointerFromOffsets (PProcedureIdentifierData (Ptr (SymbolTable [stMain].Segment,
                       ProcedureIdentifierDataOffset))^.ProcedureTypeDefinition.ResultTypeOffset);
                     If Ofs (TypeDefinition^) = 0 then AsmError (InvalidSymbolReference);
                     Value_Lo := OffsetAfterLastParameter;
                     Value_Hi := 0;
                     Size := 4;
                     If TypeDefinition^.BaseType <> btString then
                       begin
                         If pfAssembler in PProcedureIdentifierData (Ptr (SymbolTable [stMain].Segment,
                                            ProcedureIdentifierDataOffset))^.Flags then AsmError (InvalidSymbolReference);
                         Value_Lo := FunctionResultNegativeSize;
                         Size := TypeDefinition^.Size;
                       end;
                     ReferencedData.Long := $FFFFFFFD;
                     AdditionalPointer := nil;
                   end;
              end;
            end;
          else AsmError (InvalidSymbolReference);
        end;
        If Size >= $FFFD then CodeIdentifierReference := True else CodeIdentifierReference := False;
      end;

    AsmIdentifierAdditionalData := AsmIdentifierData.AdditionalPointer;
    AsmToken := AsmToken_Identifier;
  end;

begin
  AsmToken := AsmToken_Semicolon;
  If LineCharacterCounter <> 0 then
    begin
      SeparateAsmWord;
      If not BeforeOperands or not IsAsmInstruction then
        begin
          If not IsSymbolToken then
            If not IsAsmOperand then
              If not IsAsmConstant then
                If not BeforeOperands then ProcessAsmIdentifier;
        end;
    end;
  If AsmToken = AsmToken_00 then AsmToken := AsmToken_01;
end;
```

### Code Block 6

```pascal
Procedure ExpectAsmTokenAndGetNext (ExpectedAsmToken: TAsmToken);
begin
  If AsmToken <> ExpectedAsmToken then AsmError (SyntaxError);
  GetNextAsmToken;
end;
```

### Code Block 7

```pascal
Function CheckAndGetNextAsmToken (AsmTokenToCheck: TAsmToken): Boolean;
begin
  CheckAndGetNextAsmToken := True;
  If AsmToken = AsmTokenToCheck then GetNextAsmToken else CheckAndGetNextAsmToken := False;
end;
```

### Code Block 8

```pascal
Function FindAsmTokenInTable (Table: POperatorProc; Var ExpressionProc: TExpressionProc): Boolean;
begin
  FindAsmTokenInTable := False;
  While Ord (Table^.Token) <> 0 do
    begin
      If Table^.Token = AsmToken then
        begin
          ExpressionProc := Table^.Proc;
          GetNextAsmToken;
          FindAsmTokenInTable := True;
          Exit;
        end;
      Inc (Table);
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
- https://turbopascal.org/expressions
- https://turbopascal.org/extracting-assembler-tokens
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

