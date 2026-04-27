# Extracting Tokens - Turbo Pascal Compiler

**URL:** https://turbopascal.org/extracting-tokens

---

## Table of Contents

- Extracting Tokens

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Extracting Tokens
Parser
operates only with tokens – basic elements of the language. Tokens like identifiers and constants have also additional data structures where actual data is stored.
Type TToken = (
      Token_EndOfLine,
      Token_Identifier,
      Token_Constant,
      Token_Plus,
      Token_Minus,
      Token_Asterisk,
      Token_Slash,
      Token_Equal,
      Token_NotEqual,
      Token_Greater,
      Token_Lower,
      Token_GreaterOrEqual,
      Token_LowerOrEqual,
      Token_LeftParenthesis,
      Token_RightParenthesis,
      Token_LeftBracket,
      Token_RightBracket,
      Token_Period,
      Token_Comma,
      Token_Colon,
      Token_Semicolon,
      Token_Caret,
      Token_Assignment,
      Token_PeriodPeriod,
      Token_At,
      Token_INLINE_Separator,
      Token_EqualAsSeparator,
      Token_1B,
      Token_AND,
      Token_ARRAY,
      Token_BEGIN,
      Token_CASE,
      Token_CONST,
      Token_CONSTRUCTOR,
      Token_DESTRUCTOR,
      Token_DIV,
      Token_DO,
      Token_DOWNTO,
      Token_ELSE,
      Token_END,
      Token_ASM,
      Token_FILE,
      Token_FOR,
      Token_2B,
      Token_FUNCTION,
      Token_GOTO,
      Token_IF,
      Token_IMPLEMENTATION,
      Token_IN,
      Token_INLINE,
      Token_INTERFACE,
      Token_33,
      Token_LABEL,
      Token_MOD,
      Token_NIL,
      Token_NOT,
      Token_OBJECT,
      Token_OF,
      Token_OR,
      Token_PACKED,
      Token_PROCEDURE,
      Token_PROGRAM,
      Token_RECORD,
      Token_REPEAT,
      Token_SET,
      Token_SHL,
      Token_SHR,
      Token_STRING,
      Token_THEN,
      Token_TYPE,
      Token_TO,
      Token_UNIT,
      Token_UNTIL,
      Token_USES,
      Token_VAR,
      Token_INHERITED,
      Token_WHILE,
      Token_WITH,
      Token_XOR,
      Token_ConstantIdentifier,
      Token_TypeIdentifier,
      Token_VariableIdentifier,
      Token_ProcedureIdentifier,
      Token_UnitIdentifier,
      Token_OBJECT_PROCEDURE_FUNCTION_TypeIdentifier,
      Token_LabelIdentifier,
      Token_SystemProcedure,
      Token_SystemFunction,
      Token_SystemFunctionOrProcedure,
      Token_Port,
      Token_Mem,
      Token_OpenString,
      Token_AsmSegmentReference,
      Token_AsmOffsetReference,
      Token_5E,
      Token_5F,
      Token_60,
      Token_61,
      Token_62,
      Token_63,
      Token_ABSOLUTE,
      Token_PUBLIC,
      Token_PRIVATE);
This is one of the most important procedures in Turbo Pascal. It reads source file by calling
NextNonCommentCharacter
function and extracts tokens with associated data. This procedure identifies reserved words,
string constants
,
integer constants
, real constants, symbols, identifiers and other Pascal language elements.
Procedure GetNextToken;
Var NextChar, TempPtr: PChar;

  Procedure CheckIfIdentifierOrReservedWord;
  Var IdLen: Byte;
      DummyPtr: Pointer;
      DummyWord: Word;
  begin
    IdLen := 0;
    CurrentIdentifierHash := 0;
    While NextChar^ in ['0'..'9', '_', 'A'..'Z', 'a'..'z'] do
      begin
        If IdLen < High (CurrentIdentifier) then
          begin
            Inc (IdLen);
            CurrentIdentifier [IdLen] := NextChar^;
            Inc (CurrentIdentifierHash, Byte (NextChar^) and $DF);
          end;
        Inc (NextChar);
      end;
    CurrentIdentifier [0] := Char (IdLen);
    CurrentIdentifierHash := (CurrentIdentifierHash - IdLen) shl 1;
    If not IsIdentifierInSymbolTable (@ReservedWords, Token, DummyPtr, DummyWord) then Token := Token_Identifier;
  end;

  Function IsRealConstant: Boolean;
  Var Word64Lo, Word64Hi, TempWord64Lo, TempWord64Hi: LongRec;
      MantissaExponent, MantissaExponentIncrement, Exponent: Word;
      ExponentSign: Char;
      SavedFirstChar: PChar;
      Err: Integer;
      RealStr: String [128];
  begin
    IsRealConstant := False;
    SavedFirstChar := NextChar;
    Word64Lo.Long := 0;
    Word64Hi.Long := 0;
    MantissaExponent := 0;
    MantissaExponentIncrement := 0;
    Repeat
      Case NextChar^ of
        '0'..'9': begin
                    If Word64Hi.WordH < $0CCC then
                      begin
                        { Word64 := Word64 * 10 + Digit }
                        Inc (MantissaExponent, MantissaExponentIncrement);
                      end else Inc (MantissaExponent, MantissaExponentIncrement + 1);
                  end;
        '.': If MantissaExponentIncrement = 0 then Dec (MantissaExponentIncrement)
               else Break;
      end;
      Inc (NextChar);
    until not (NextChar^ in ['0'..'9', '.']);
    TempWord64Lo := Word64Lo;
    TempWord64Hi := Word64Hi;
    If UpCase (NextChar^) = 'E' then
      begin
        Inc (NextChar);
        ExponentSign := NextChar^;
        If NextChar^ in ['+', '-'] then Inc (NextChar);
        Exponent := 0;
        While NextChar^ in ['0'..'9'] do
          If Exponent < 1000 then  { If exponent is greater than 9999 TPC evaluates up to 4 digits of }
            begin                  { the exponent and then takes the next digit for next token.       }
              Exponent := 10 * Exponent + Byte (NextChar [0]) - Ord ('0');
              Inc (NextChar);
            end else Exit;         { BUG in TPC:  1e-4932 is not calculated correctly }
        If ExponentSign = '-' then Exponent := MantissaExponent - Exponent
          else Exponent := MantissaExponent + Exponent;

      end;
    RealStr [0] := Char (NextChar - SavedFirstChar);
    Move (SavedFirstChar^, RealStr [1], Byte (RealStr [0]));
    Val (RealStr, NumericConstant.Extended, Err);
    IsRealConstant := Err = 0;
  end;


  Procedure CheckRealConstant;
  begin
    If not IsRealConstant then
      begin
        ErrorSourcePosition := NextChar;
        Error (ErrorInRealConstant);
      end;
    ConstantTypeDef := Ptr (SystemUnitSegment, Extended_TypeOffset);
    Token := Token_Constant;
  end;

  Function CheckIntegerNumericConstant (Var NextChar: PChar): TToken;
  begin
    If not IsIntegerNumericConstant (NextChar) then
      begin
        ErrorSourcePosition := NextChar;
        Error (ErrorInIntegerConstant);
      end;
    NumericConstant.LongInt := IntegerNumericConstant;
    ConstantTypeDef := Ptr (SystemUnitSegment, LongInt_TypeOffset);
    CheckIntegerNumericConstant := Token_Constant;
  end;

begin
  CheckStack;
  NextChar := NextNonCommentCharacter;
  ErrorSourcePosition := NextChar;
  Case NextChar [0] of
    #0: begin
          Token := Token_EndOfLine;
          Inc (NextChar);
        end;
    '!', '"',
    '%', '&',
    '('..'/',
    ':'..'@',
    '['..'^',
    '`':      begin
                Case NextChar [0] of
                  '!',
                  '"',
                  '%',
                  '&',
                  '?',
                  '\',
                  '`': Error (SyntaxError);
                  '(': If NextChar [1] = '.' then
                        begin
                          Token := Token_LeftBracket;
                          Inc (NextChar);
                        end else Token := Token_LeftParenthesis;
                  ')': Token := Token_RightParenthesis;
                  '*': Token := Token_Asterisk;
                  '+': Token := Token_Plus;
                  ',': Token := Token_Comma;
                  '-': Token := Token_Minus;
                  '.': Case NextChar [1] of
                        '.': begin
                                Token := Token_PeriodPeriod;
                                Inc (NextChar);
                              end;
                        ')': begin
                                Token := Token_RightBracket;
                                Inc (NextChar);
                              end;
                        else Token := Token_Period;
                      end;
                  '/': Token := TokenForSlash;
                  ':': If NextChar [1] = '=' then
                        begin
                          Token := Token_Assignment;
                          Inc (NextChar);
                        end else Token := Token_Colon;
                  ';': Token := Token_Semicolon;
                  '<': Case NextChar [1] of
                        '>': begin
                                Token := Token_NotEqual;
                                Inc (NextChar);
                              end;
                        '=': begin
                                Token := Token_LowerOrEqual;
                                Inc (NextChar);
                              end;
                        else Token := Token_Lower;
                      end;
                  '=': Token := TokenForEqual;
                  '>': If NextChar [1] = '=' then
                        begin
                          Token := Token_GreaterOrEqual;
                          Inc (NextChar);
                        end else Token := Token_Greater;
                  '@': Token := Token_At;
                  '[': Token := Token_LeftBracket;
                  ']': Token := Token_RightBracket;
                  '^': Token := Token_Caret;
                end;
                Inc (NextChar);
              end;
    '#',
    '''': Token := ProcessStringConstant (NextChar);
    '$':  Token := CheckIntegerNumericConstant (NextChar);
    '0'..'9': begin
                TempPtr := NextChar;
                Repeat
                  Inc (TempPtr);
                until not (TempPtr [0] in ['0'..'9']);
                Case UpCase (TempPtr [0]) of
                  'E': CheckRealConstant;
                  '.': Case TempPtr [1] of
                         '.',
                         ')': Token := CheckIntegerNumericConstant (NextChar);
                         else CheckRealConstant;
                       end;
                  else Token := CheckIntegerNumericConstant (NextChar);
                end;
              end;
    'A'..'Z',
    'a'..'z',
    '_':      CheckIfIdentifierOrReservedWord;
    else Error (SyntaxError);
  end;
  CurrentSourceFile^.CurrentPosition := Ofs (NextChar^);
end;
This function expects specified token and gets the next one or reports error if the expected token is not found.
Procedure ExpectTokenAndGetNext (ExpectedToken: TToken);
Const SpecialErrors = 22;
      ExpectedTokenErrors: Array [1..SpecialErrors] of TExpectedTokenError = (
        (Token: Token_Identifier;       Error: IdentifierExpected),
        (Token: Token_LabelIdentifier;  Error: LabelIdentifierExpected),
        (Token: Token_BEGIN;            Error: BEGIN_expected),
        (Token: Token_END;              Error: END_expected),
        (Token: Token_DO;               Error: DO_expected),
        (Token: Token_OF;               Error: OF_expected),
        (Token: Token_INTERFACE;        Error: INTERFACE_expected),
        (Token: Token_THEN;             Error: THEN_expected),
        (Token: Token_IMPLEMENTATION;   Error: IMPLEMENTATION_expected),
        (Token: Token_UNIT;             Error: UNIT_expected),
        (Token: Token_Semicolon;        Error: SemicolonExpected),
        (Token: Token_Colon;            Error: ColonExpected),
        (Token: Token_Comma;            Error: CommaExpected),
        (Token: Token_LeftParenthesis;  Error: LeftParenthesisExpected),
        (Token: Token_RightParenthesis; Error: RightParenthesisExpected),
        (Token: Token_Equal;            Error: EqualExpected),
        (Token: Token_EqualAsSeparator; Error: EqualExpected),
        (Token: Token_Assignment;       Error: AssignmentExpected),
        (Token: Token_LeftBracket;      Error: LeftBracketExpected),
        (Token: Token_RightBracket;     Error: RightBracketExpected),
        (Token: Token_Period;           Error: PeriodExpected),
        (Token: Token_PeriodPeriod;     Error: PeriodPeriodExpected));
Var N: Byte;
begin
  If ExpectedToken <> Token then
    begin
      For N := 1 to SpecialErrors do
        If ExpectedTokenErrors [N].Token = ExpectedToken then Error (ExpectedTokenErrors [N].Error);
      Error (SyntaxError);
    end;
  GetNextToken;
end;
Another function that checks for specified token but doesn’t report error if it is not found.
Function CheckAndGetNextToken (TokenToCheck: TToken): Boolean;
begin
  CheckAndGetNextToken := False;
  If Token <> TokenToCheck then Exit;
  CheckAndGetNextToken := True;
  GetNextToken;
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
Type TToken = (
      Token_EndOfLine,
      Token_Identifier,
      Token_Constant,
      Token_Plus,
      Token_Minus,
      Token_Asterisk,
      Token_Slash,
      Token_Equal,
      Token_NotEqual,
      Token_Greater,
      Token_Lower,
      Token_GreaterOrEqual,
      Token_LowerOrEqual,
      Token_LeftParenthesis,
      Token_RightParenthesis,
      Token_LeftBracket,
      Token_RightBracket,
      Token_Period,
      Token_Comma,
      Token_Colon,
      Token_Semicolon,
      Token_Caret,
      Token_Assignment,
      Token_PeriodPeriod,
      Token_At,
      Token_INLINE_Separator,
      Token_EqualAsSeparator,
      Token_1B,
      Token_AND,
      Token_ARRAY,
      Token_BEGIN,
      Token_CASE,
      Token_CONST,
      Token_CONSTRUCTOR,
      Token_DESTRUCTOR,
      Token_DIV,
      Token_DO,
      Token_DOWNTO,
      Token_ELSE,
      Token_END,
      Token_ASM,
      Token_FILE,
      Token_FOR,
      Token_2B,
      Token_FUNCTION,
      Token_GOTO,
      Token_IF,
      Token_IMPLEMENTATION,
      Token_IN,
      Token_INLINE,
      Token_INTERFACE,
      Token_33,
      Token_LABEL,
      Token_MOD,
      Token_NIL,
      Token_NOT,
      Token_OBJECT,
      Token_OF,
      Token_OR,
      Token_PACKED,
      Token_PROCEDURE,
      Token_PROGRAM,
      Token_RECORD,
      Token_REPEAT,
      Token_SET,
      Token_SHL,
      Token_SHR,
      Token_STRING,
      Token_THEN,
      Token_TYPE,
      Token_TO,
      Token_UNIT,
      Token_UNTIL,
      Token_USES,
      Token_VAR,
      Token_INHERITED,
      Token_WHILE,
      Token_WITH,
      Token_XOR,
      Token_ConstantIdentifier,
      Token_TypeIdentifier,
      Token_VariableIdentifier,
      Token_ProcedureIdentifier,
      Token_UnitIdentifier,
      Token_OBJECT_PROCEDURE_FUNCTION_TypeIdentifier,
      Token_LabelIdentifier,
      Token_SystemProcedure,
      Token_SystemFunction,
      Token_SystemFunctionOrProcedure,
      Token_Port,
      Token_Mem,
      Token_OpenString,
      Token_AsmSegmentReference,
      Token_AsmOffsetReference,
      Token_5E,
      Token_5F,
      Token_60,
      Token_61,
      Token_62,
      Token_63,
      Token_ABSOLUTE,
      Token_PUBLIC,
      Token_PRIVATE);
```

### Code Block 2

```pascal
Procedure GetNextToken;
Var NextChar, TempPtr: PChar;

  Procedure CheckIfIdentifierOrReservedWord;
  Var IdLen: Byte;
      DummyPtr: Pointer;
      DummyWord: Word;
  begin
    IdLen := 0;
    CurrentIdentifierHash := 0;
    While NextChar^ in ['0'..'9', '_', 'A'..'Z', 'a'..'z'] do
      begin
        If IdLen < High (CurrentIdentifier) then
          begin
            Inc (IdLen);
            CurrentIdentifier [IdLen] := NextChar^;
            Inc (CurrentIdentifierHash, Byte (NextChar^) and $DF);
          end;
        Inc (NextChar);
      end;
    CurrentIdentifier [0] := Char (IdLen);
    CurrentIdentifierHash := (CurrentIdentifierHash - IdLen) shl 1;
    If not IsIdentifierInSymbolTable (@ReservedWords, Token, DummyPtr, DummyWord) then Token := Token_Identifier;
  end;

  Function IsRealConstant: Boolean;
  Var Word64Lo, Word64Hi, TempWord64Lo, TempWord64Hi: LongRec;
      MantissaExponent, MantissaExponentIncrement, Exponent: Word;
      ExponentSign: Char;
      SavedFirstChar: PChar;
      Err: Integer;
      RealStr: String [128];
  begin
    IsRealConstant := False;
    SavedFirstChar := NextChar;
    Word64Lo.Long := 0;
    Word64Hi.Long := 0;
    MantissaExponent := 0;
    MantissaExponentIncrement := 0;
    Repeat
      Case NextChar^ of
        '0'..'9': begin
                    If Word64Hi.WordH < $0CCC then
                      begin
                        { Word64 := Word64 * 10 + Digit }
                        Inc (MantissaExponent, MantissaExponentIncrement);
                      end else Inc (MantissaExponent, MantissaExponentIncrement + 1);
                  end;
        '.': If MantissaExponentIncrement = 0 then Dec (MantissaExponentIncrement)
               else Break;
      end;
      Inc (NextChar);
    until not (NextChar^ in ['0'..'9', '.']);
    TempWord64Lo := Word64Lo;
    TempWord64Hi := Word64Hi;
    If UpCase (NextChar^) = 'E' then
      begin
        Inc (NextChar);
        ExponentSign := NextChar^;
        If NextChar^ in ['+', '-'] then Inc (NextChar);
        Exponent := 0;
        While NextChar^ in ['0'..'9'] do
          If Exponent < 1000 then  { If exponent is greater than 9999 TPC evaluates up to 4 digits of }
            begin                  { the exponent and then takes the next digit for next token.       }
              Exponent := 10 * Exponent + Byte (NextChar [0]) - Ord ('0');
              Inc (NextChar);
            end else Exit;         { BUG in TPC:  1e-4932 is not calculated correctly }
        If ExponentSign = '-' then Exponent := MantissaExponent - Exponent
          else Exponent := MantissaExponent + Exponent;

      end;
    RealStr [0] := Char (NextChar - SavedFirstChar);
    Move (SavedFirstChar^, RealStr [1], Byte (RealStr [0]));
    Val (RealStr, NumericConstant.Extended, Err);
    IsRealConstant := Err = 0;
  end;


  Procedure CheckRealConstant;
  begin
    If not IsRealConstant then
      begin
        ErrorSourcePosition := NextChar;
        Error (ErrorInRealConstant);
      end;
    ConstantTypeDef := Ptr (SystemUnitSegment, Extended_TypeOffset);
    Token := Token_Constant;
  end;

  Function CheckIntegerNumericConstant (Var NextChar: PChar): TToken;
  begin
    If not IsIntegerNumericConstant (NextChar) then
      begin
        ErrorSourcePosition := NextChar;
        Error (ErrorInIntegerConstant);
      end;
    NumericConstant.LongInt := IntegerNumericConstant;
    ConstantTypeDef := Ptr (SystemUnitSegment, LongInt_TypeOffset);
    CheckIntegerNumericConstant := Token_Constant;
  end;

begin
  CheckStack;
  NextChar := NextNonCommentCharacter;
  ErrorSourcePosition := NextChar;
  Case NextChar [0] of
    #0: begin
          Token := Token_EndOfLine;
          Inc (NextChar);
        end;
    '!', '"',
    '%', '&',
    '('..'/',
    ':'..'@',
    '['..'^',
    '`':      begin
                Case NextChar [0] of
                  '!',
                  '"',
                  '%',
                  '&',
                  '?',
                  '\',
                  '`': Error (SyntaxError);
                  '(': If NextChar [1] = '.' then
                        begin
                          Token := Token_LeftBracket;
                          Inc (NextChar);
                        end else Token := Token_LeftParenthesis;
                  ')': Token := Token_RightParenthesis;
                  '*': Token := Token_Asterisk;
                  '+': Token := Token_Plus;
                  ',': Token := Token_Comma;
                  '-': Token := Token_Minus;
                  '.': Case NextChar [1] of
                        '.': begin
                                Token := Token_PeriodPeriod;
                                Inc (NextChar);
                              end;
                        ')': begin
                                Token := Token_RightBracket;
                                Inc (NextChar);
                              end;
                        else Token := Token_Period;
                      end;
                  '/': Token := TokenForSlash;
                  ':': If NextChar [1] = '=' then
                        begin
                          Token := Token_Assignment;
                          Inc (NextChar);
                        end else Token := Token_Colon;
                  ';': Token := Token_Semicolon;
                  '<': Case NextChar [1] of
                        '>': begin
                                Token := Token_NotEqual;
                                Inc (NextChar);
                              end;
                        '=': begin
                                Token := Token_LowerOrEqual;
                                Inc (NextChar);
                              end;
                        else Token := Token_Lower;
                      end;
                  '=': Token := TokenForEqual;
                  '>': If NextChar [1] = '=' then
                        begin
                          Token := Token_GreaterOrEqual;
                          Inc (NextChar);
                        end else Token := Token_Greater;
                  '@': Token := Token_At;
                  '[': Token := Token_LeftBracket;
                  ']': Token := Token_RightBracket;
                  '^': Token := Token_Caret;
                end;
                Inc (NextChar);
              end;
    '#',
    '''': Token := ProcessStringConstant (NextChar);
    '$':  Token := CheckIntegerNumericConstant (NextChar);
    '0'..'9': begin
                TempPtr := NextChar;
                Repeat
                  Inc (TempPtr);
                until not (TempPtr [0] in ['0'..'9']);
                Case UpCase (TempPtr [0]) of
                  'E': CheckRealConstant;
                  '.': Case TempPtr [1] of
                         '.',
                         ')': Token := CheckIntegerNumericConstant (NextChar);
                         else CheckRealConstant;
                       end;
                  else Token := CheckIntegerNumericConstant (NextChar);
                end;
              end;
    'A'..'Z',
    'a'..'z',
    '_':      CheckIfIdentifierOrReservedWord;
    else Error (SyntaxError);
  end;
  CurrentSourceFile^.CurrentPosition := Ofs (NextChar^);
end;
```

### Code Block 3

```pascal
Procedure ExpectTokenAndGetNext (ExpectedToken: TToken);
Const SpecialErrors = 22;
      ExpectedTokenErrors: Array [1..SpecialErrors] of TExpectedTokenError = (
        (Token: Token_Identifier;       Error: IdentifierExpected),
        (Token: Token_LabelIdentifier;  Error: LabelIdentifierExpected),
        (Token: Token_BEGIN;            Error: BEGIN_expected),
        (Token: Token_END;              Error: END_expected),
        (Token: Token_DO;               Error: DO_expected),
        (Token: Token_OF;               Error: OF_expected),
        (Token: Token_INTERFACE;        Error: INTERFACE_expected),
        (Token: Token_THEN;             Error: THEN_expected),
        (Token: Token_IMPLEMENTATION;   Error: IMPLEMENTATION_expected),
        (Token: Token_UNIT;             Error: UNIT_expected),
        (Token: Token_Semicolon;        Error: SemicolonExpected),
        (Token: Token_Colon;            Error: ColonExpected),
        (Token: Token_Comma;            Error: CommaExpected),
        (Token: Token_LeftParenthesis;  Error: LeftParenthesisExpected),
        (Token: Token_RightParenthesis; Error: RightParenthesisExpected),
        (Token: Token_Equal;            Error: EqualExpected),
        (Token: Token_EqualAsSeparator; Error: EqualExpected),
        (Token: Token_Assignment;       Error: AssignmentExpected),
        (Token: Token_LeftBracket;      Error: LeftBracketExpected),
        (Token: Token_RightBracket;     Error: RightBracketExpected),
        (Token: Token_Period;           Error: PeriodExpected),
        (Token: Token_PeriodPeriod;     Error: PeriodPeriodExpected));
Var N: Byte;
begin
  If ExpectedToken <> Token then
    begin
      For N := 1 to SpecialErrors do
        If ExpectedTokenErrors [N].Token = ExpectedToken then Error (ExpectedTokenErrors [N].Error);
      Error (SyntaxError);
    end;
  GetNextToken;
end;
```

### Code Block 4

```pascal
Function CheckAndGetNextToken (TokenToCheck: TToken): Boolean;
begin
  CheckAndGetNextToken := False;
  If Token <> TokenToCheck then Exit;
  CheckAndGetNextToken := True;
  GetNextToken;
end;
```

### Code Block 5

```pascal
Type TToken = (
      Token_EndOfLine,
      Token_Identifier,
      Token_Constant,
      Token_Plus,
      Token_Minus,
      Token_Asterisk,
      Token_Slash,
      Token_Equal,
      Token_NotEqual,
      Token_Greater,
      Token_Lower,
      Token_GreaterOrEqual,
      Token_LowerOrEqual,
      Token_LeftParenthesis,
      Token_RightParenthesis,
      Token_LeftBracket,
      Token_RightBracket,
      Token_Period,
      Token_Comma,
      Token_Colon,
      Token_Semicolon,
      Token_Caret,
      Token_Assignment,
      Token_PeriodPeriod,
      Token_At,
      Token_INLINE_Separator,
      Token_EqualAsSeparator,
      Token_1B,
      Token_AND,
      Token_ARRAY,
      Token_BEGIN,
      Token_CASE,
      Token_CONST,
      Token_CONSTRUCTOR,
      Token_DESTRUCTOR,
      Token_DIV,
      Token_DO,
      Token_DOWNTO,
      Token_ELSE,
      Token_END,
      Token_ASM,
      Token_FILE,
      Token_FOR,
      Token_2B,
      Token_FUNCTION,
      Token_GOTO,
      Token_IF,
      Token_IMPLEMENTATION,
      Token_IN,
      Token_INLINE,
      Token_INTERFACE,
      Token_33,
      Token_LABEL,
      Token_MOD,
      Token_NIL,
      Token_NOT,
      Token_OBJECT,
      Token_OF,
      Token_OR,
      Token_PACKED,
      Token_PROCEDURE,
      Token_PROGRAM,
      Token_RECORD,
      Token_REPEAT,
      Token_SET,
      Token_SHL,
      Token_SHR,
      Token_STRING,
      Token_THEN,
      Token_TYPE,
      Token_TO,
      Token_UNIT,
      Token_UNTIL,
      Token_USES,
      Token_VAR,
      Token_INHERITED,
      Token_WHILE,
      Token_WITH,
      Token_XOR,
      Token_ConstantIdentifier,
      Token_TypeIdentifier,
      Token_VariableIdentifier,
      Token_ProcedureIdentifier,
      Token_UnitIdentifier,
      Token_OBJECT_PROCEDURE_FUNCTION_TypeIdentifier,
      Token_LabelIdentifier,
      Token_SystemProcedure,
      Token_SystemFunction,
      Token_SystemFunctionOrProcedure,
      Token_Port,
      Token_Mem,
      Token_OpenString,
      Token_AsmSegmentReference,
      Token_AsmOffsetReference,
      Token_5E,
      Token_5F,
      Token_60,
      Token_61,
      Token_62,
      Token_63,
      Token_ABSOLUTE,
      Token_PUBLIC,
      Token_PRIVATE);
```

### Code Block 6

```pascal
Procedure GetNextToken;
Var NextChar, TempPtr: PChar;

  Procedure CheckIfIdentifierOrReservedWord;
  Var IdLen: Byte;
      DummyPtr: Pointer;
      DummyWord: Word;
  begin
    IdLen := 0;
    CurrentIdentifierHash := 0;
    While NextChar^ in ['0'..'9', '_', 'A'..'Z', 'a'..'z'] do
      begin
        If IdLen < High (CurrentIdentifier) then
          begin
            Inc (IdLen);
            CurrentIdentifier [IdLen] := NextChar^;
            Inc (CurrentIdentifierHash, Byte (NextChar^) and $DF);
          end;
        Inc (NextChar);
      end;
    CurrentIdentifier [0] := Char (IdLen);
    CurrentIdentifierHash := (CurrentIdentifierHash - IdLen) shl 1;
    If not IsIdentifierInSymbolTable (@ReservedWords, Token, DummyPtr, DummyWord) then Token := Token_Identifier;
  end;

  Function IsRealConstant: Boolean;
  Var Word64Lo, Word64Hi, TempWord64Lo, TempWord64Hi: LongRec;
      MantissaExponent, MantissaExponentIncrement, Exponent: Word;
      ExponentSign: Char;
      SavedFirstChar: PChar;
      Err: Integer;
      RealStr: String [128];
  begin
    IsRealConstant := False;
    SavedFirstChar := NextChar;
    Word64Lo.Long := 0;
    Word64Hi.Long := 0;
    MantissaExponent := 0;
    MantissaExponentIncrement := 0;
    Repeat
      Case NextChar^ of
        '0'..'9': begin
                    If Word64Hi.WordH < $0CCC then
                      begin
                        { Word64 := Word64 * 10 + Digit }
                        Inc (MantissaExponent, MantissaExponentIncrement);
                      end else Inc (MantissaExponent, MantissaExponentIncrement + 1);
                  end;
        '.': If MantissaExponentIncrement = 0 then Dec (MantissaExponentIncrement)
               else Break;
      end;
      Inc (NextChar);
    until not (NextChar^ in ['0'..'9', '.']);
    TempWord64Lo := Word64Lo;
    TempWord64Hi := Word64Hi;
    If UpCase (NextChar^) = 'E' then
      begin
        Inc (NextChar);
        ExponentSign := NextChar^;
        If NextChar^ in ['+', '-'] then Inc (NextChar);
        Exponent := 0;
        While NextChar^ in ['0'..'9'] do
          If Exponent < 1000 then  { If exponent is greater than 9999 TPC evaluates up to 4 digits of }
            begin                  { the exponent and then takes the next digit for next token.       }
              Exponent := 10 * Exponent + Byte (NextChar [0]) - Ord ('0');
              Inc (NextChar);
            end else Exit;         { BUG in TPC:  1e-4932 is not calculated correctly }
        If ExponentSign = '-' then Exponent := MantissaExponent - Exponent
          else Exponent := MantissaExponent + Exponent;

      end;
    RealStr [0] := Char (NextChar - SavedFirstChar);
    Move (SavedFirstChar^, RealStr [1], Byte (RealStr [0]));
    Val (RealStr, NumericConstant.Extended, Err);
    IsRealConstant := Err = 0;
  end;


  Procedure CheckRealConstant;
  begin
    If not IsRealConstant then
      begin
        ErrorSourcePosition := NextChar;
        Error (ErrorInRealConstant);
      end;
    ConstantTypeDef := Ptr (SystemUnitSegment, Extended_TypeOffset);
    Token := Token_Constant;
  end;

  Function CheckIntegerNumericConstant (Var NextChar: PChar): TToken;
  begin
    If not IsIntegerNumericConstant (NextChar) then
      begin
        ErrorSourcePosition := NextChar;
        Error (ErrorInIntegerConstant);
      end;
    NumericConstant.LongInt := IntegerNumericConstant;
    ConstantTypeDef := Ptr (SystemUnitSegment, LongInt_TypeOffset);
    CheckIntegerNumericConstant := Token_Constant;
  end;

begin
  CheckStack;
  NextChar := NextNonCommentCharacter;
  ErrorSourcePosition := NextChar;
  Case NextChar [0] of
    #0: begin
          Token := Token_EndOfLine;
          Inc (NextChar);
        end;
    '!', '"',
    '%', '&',
    '('..'/',
    ':'..'@',
    '['..'^',
    '`':      begin
                Case NextChar [0] of
                  '!',
                  '"',
                  '%',
                  '&',
                  '?',
                  '\',
                  '`': Error (SyntaxError);
                  '(': If NextChar [1] = '.' then
                        begin
                          Token := Token_LeftBracket;
                          Inc (NextChar);
                        end else Token := Token_LeftParenthesis;
                  ')': Token := Token_RightParenthesis;
                  '*': Token := Token_Asterisk;
                  '+': Token := Token_Plus;
                  ',': Token := Token_Comma;
                  '-': Token := Token_Minus;
                  '.': Case NextChar [1] of
                        '.': begin
                                Token := Token_PeriodPeriod;
                                Inc (NextChar);
                              end;
                        ')': begin
                                Token := Token_RightBracket;
                                Inc (NextChar);
                              end;
                        else Token := Token_Period;
                      end;
                  '/': Token := TokenForSlash;
                  ':': If NextChar [1] = '=' then
                        begin
                          Token := Token_Assignment;
                          Inc (NextChar);
                        end else Token := Token_Colon;
                  ';': Token := Token_Semicolon;
                  '<': Case NextChar [1] of
                        '>': begin
                                Token := Token_NotEqual;
                                Inc (NextChar);
                              end;
                        '=': begin
                                Token := Token_LowerOrEqual;
                                Inc (NextChar);
                              end;
                        else Token := Token_Lower;
                      end;
                  '=': Token := TokenForEqual;
                  '>': If NextChar [1] = '=' then
                        begin
                          Token := Token_GreaterOrEqual;
                          Inc (NextChar);
                        end else Token := Token_Greater;
                  '@': Token := Token_At;
                  '[': Token := Token_LeftBracket;
                  ']': Token := Token_RightBracket;
                  '^': Token := Token_Caret;
                end;
                Inc (NextChar);
              end;
    '#',
    '''': Token := ProcessStringConstant (NextChar);
    '$':  Token := CheckIntegerNumericConstant (NextChar);
    '0'..'9': begin
                TempPtr := NextChar;
                Repeat
                  Inc (TempPtr);
                until not (TempPtr [0] in ['0'..'9']);
                Case UpCase (TempPtr [0]) of
                  'E': CheckRealConstant;
                  '.': Case TempPtr [1] of
                         '.',
                         ')': Token := CheckIntegerNumericConstant (NextChar);
                         else CheckRealConstant;
                       end;
                  else Token := CheckIntegerNumericConstant (NextChar);
                end;
              end;
    'A'..'Z',
    'a'..'z',
    '_':      CheckIfIdentifierOrReservedWord;
    else Error (SyntaxError);
  end;
  CurrentSourceFile^.CurrentPosition := Ofs (NextChar^);
end;
```

### Code Block 7

```pascal
Procedure ExpectTokenAndGetNext (ExpectedToken: TToken);
Const SpecialErrors = 22;
      ExpectedTokenErrors: Array [1..SpecialErrors] of TExpectedTokenError = (
        (Token: Token_Identifier;       Error: IdentifierExpected),
        (Token: Token_LabelIdentifier;  Error: LabelIdentifierExpected),
        (Token: Token_BEGIN;            Error: BEGIN_expected),
        (Token: Token_END;              Error: END_expected),
        (Token: Token_DO;               Error: DO_expected),
        (Token: Token_OF;               Error: OF_expected),
        (Token: Token_INTERFACE;        Error: INTERFACE_expected),
        (Token: Token_THEN;             Error: THEN_expected),
        (Token: Token_IMPLEMENTATION;   Error: IMPLEMENTATION_expected),
        (Token: Token_UNIT;             Error: UNIT_expected),
        (Token: Token_Semicolon;        Error: SemicolonExpected),
        (Token: Token_Colon;            Error: ColonExpected),
        (Token: Token_Comma;            Error: CommaExpected),
        (Token: Token_LeftParenthesis;  Error: LeftParenthesisExpected),
        (Token: Token_RightParenthesis; Error: RightParenthesisExpected),
        (Token: Token_Equal;            Error: EqualExpected),
        (Token: Token_EqualAsSeparator; Error: EqualExpected),
        (Token: Token_Assignment;       Error: AssignmentExpected),
        (Token: Token_LeftBracket;      Error: LeftBracketExpected),
        (Token: Token_RightBracket;     Error: RightBracketExpected),
        (Token: Token_Period;           Error: PeriodExpected),
        (Token: Token_PeriodPeriod;     Error: PeriodPeriodExpected));
Var N: Byte;
begin
  If ExpectedToken <> Token then
    begin
      For N := 1 to SpecialErrors do
        If ExpectedTokenErrors [N].Token = ExpectedToken then Error (ExpectedTokenErrors [N].Error);
      Error (SyntaxError);
    end;
  GetNextToken;
end;
```

### Code Block 8

```pascal
Function CheckAndGetNextToken (TokenToCheck: TToken): Boolean;
begin
  CheckAndGetNextToken := False;
  If Token <> TokenToCheck then Exit;
  CheckAndGetNextToken := True;
  GetNextToken;
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
- https://turbopascal.org/extracting-tokens
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-integer-constants
- https://turbopascal.org/processing-source-line-characters
- https://turbopascal.org/processing-string-constants
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

