# Processing Comments and Compiler Directives - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-comments-and-compiler-directives

---

## Table of Contents

- Processing Comments and Compiler Directives

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Comments and Compiler Directives
This procedure processes compiler directives and skips comment. Here also conditional compilation directives (with support for nested directives) are processed.
Procedure ProcessComment (Var NextChar: PChar; StartChar: Char);
Var SecondDirective, Directive: TDirective;
    Switch: TCompilerSwitches;
    FileRecord: Pointer;
    UnitPtr: PUnitHeader;
    IntegerNumericConstant: LongInt;

  Procedure SkipComment (Var NextChar: PChar);
  Var C: Char;
      TwoChars: TTwoChars;
  begin
    If CommentCharacter = '{' then
      Repeat
        C := NextChar^;
        Inc (NextChar);
        If C = #0 then
          begin
            If not NextLine then Error (UnexpectedEndOfFile);
            NextChar := CurrentLine;
            Continue;
          end;
      until C = '}'
        else Repeat
               TwoChars := PTwoChars (NextChar - 1)^;
               Inc (NextChar);
               If TwoChars.Second = #0 then
                 begin
                   If not NextLine then Error (UnexpectedEndOfFile);
                   NextChar := CurrentLine;
                   Continue;
                 end;
             until TwoChars.Both = '*)';
  end;

  Procedure ExpectIdentifierInDirective;
  begin
    SkipSeparators (NextChar);
    ErrorSourcePosition := NextChar;
    CopyToIdentifier (NextChar);
    If Identifier = '' then Error (IdentifierExpected);
  end;

  Function Find_ENDIF: Boolean;
  Type TDirectiveData = Record
                          EndsENDIF: Boolean;
                          IfLevelIncrement: Integer;
                        end;
  Var IfLevel: Byte;
      StringFlag: Boolean;
      DirectiveData: TDirectiveData;
      NewDirective: TDirective;
      TwoChars: TTwoChars;
  begin
    IfLevel := 0;
    StringFlag := False;
    Repeat
      Case NextChar^ of
        #0: begin
              If not NextLine then Error (ENDIF_directiveMissing);
              NextChar := CurrentLine;
              StringFlag := False;
              Continue;
            end;
        '''': begin
                StringFlag := not StringFlag;
                Inc (NextChar);
                Continue;
              end;
      end;
      If StringFlag then
        begin
          Inc (NextChar);
          Continue;
        end;
      TwoChars := PTwoChars (NextChar)^;
      Inc (NextChar);
      If (TwoChars.First = '{') or (TwoChars.Both = '(*') then
        begin
          If TwoChars.First &lt;&gt; '{' then Inc (NextChar);
          CommentCharacter := TwoChars.First;
          With DirectiveData do
            begin
              EndsENDIF := False;
              IfLevelIncrement := 0;
            end;
          If NextChar^ = '$' then
            begin
              Inc (NextChar);
              If IsCompilerDirective (NextChar, NewDirective) then With DirectiveData do
                Case NewDirective of
                  Directive_ELSE:   begin
                                      EndsENDIF := True;
                                      IfLevelIncrement :=   0;
                                    end;
                  Directive_ENDIF:  begin
                                      EndsENDIF := True;
                                      IfLevelIncrement := - 1;
                                    end;
                  Directive_IFDEF,
                  Directive_IFNDEF,
                  Directive_IFOPT:  begin
                                      EndsENDIF := False;
                                      IfLevelIncrement :=   1;
                                    end;
                end;
            end;
          SkipComment (NextChar);
          If DirectiveData.EndsENDIF and (IfLevel = 0) then
            begin
              Find_ENDIF := DirectiveData.IfLevelIncrement &lt;&gt; 0;
              Exit;
            end;
          Inc (IfLevel, DirectiveData.IfLevelIncrement);
          StringFlag := False;
        end;
    until False;
  end;

  Procedure GetDirectiveFileName;
  Var Len: Byte;
  begin
    SkipSeparators (NextChar);
    ErrorSourcePosition := NextChar;
    Len := 0;
    Repeat
      Case NextChar^ of
        #0..' ',
        '*', '}': Break;
        else Identifier [Len] := UpCase (NextChar^);
      end;
      Inc (Len);
      Inc (NextChar);
    until Len = 79;
    Identifier [Len] := #0;
  end;

  Procedure ProcessIF (Cond: Boolean);
  begin
    SkipComment (NextChar);
    Inc (CurrentSourceFile^.IF_Level);
    If not Cond then
      If Find_ENDIF then Dec (CurrentSourceFile^.IF_Level);
  end;

  Function IsIntegerNumericConstantNext: Boolean;
  begin
    SkipSeparators (NextChar);
    ErrorSourcePosition := NextChar;
    IsIntegerNumericConstantNext := IsIntegerNumericConstant (NextChar);
  end;

  Function IsSegmentNext: Boolean;
  begin
    IsSegmentNext := False;
    If not IsIntegerNumericConstantNext then Exit;
    With IntegerNumericConstantRec do If (WordH = $FFFF) and (WordL &gt; $FFF0) then Exit;
    IntegerNumericConstant := (IntegerNumericConstant + $F) shr 4;
    IsSegmentNext := IntegerNumericConstantRec.WordH = 0;
  end;

  Procedure RemoveConditionalDefine;
  Var SrcPtr, DestPtr: PChar;
      Len: Byte;
  begin
    If not IdentifierInConditionalDefines then Exit;
    Len := Byte (DestPtr^);
    SrcPtr := DestPtr + Len + 1;
    Move (SrcPtr^, DestPtr^, Ofs (ConditionalDefinesEnd^) - Ofs (SrcPtr^));
    ConditionalDefinesEnd := Ptr (Seg (ConditionalDefinesEnd^), Ofs (ConditionalDefinesEnd^) - Len - 1);
  end;

  Function UnitFound (Name: PString; Var TempUnit: PUnitHeader): Boolean;
  Var UnitSegment: Word;
      UnitName: PString;
  begin
    UnitFound := True;
    UnitSegment := LastLoadedUsedUnit;
    Repeat
      TempUnit := Ptr (UnitSegment, 0);
      UnitName := @PIdentifier (Ptr (UnitSegment, TempUnit^.UnitNameIdentifierOffset))^.Name;
      If IdentifiersEqual (Name, UnitName) then Exit;
      UnitSegment := TempUnit^.PreviousUnitSegment;
    until UnitSegment = 0;
    UnitFound := False;
  end;

  Function SetSwitch (CompilerSwitch: TCompilerSwitches; ReportError: Boolean): Boolean;
  begin
    SetSwitch := False;
    If not (NextChar^ in ['+', '-']) then
      Case ReportError of
        True: Error (InvalidCompilerDirective);
        else Exit;
      end;
    If ModuleHeadingEnded then If CompilerSwitch in GlobalCompilerSwitches then Error (InvalidCompilerDirective);
    Case NextChar^ of
      '+': Include (ModuleCompilerSwitches, CompilerSwitch);
      '-': Exclude (ModuleCompilerSwitches, CompilerSwitch);
    end;
    SetSwitch := True;
    Inc (NextChar);
  end;

begin
  CommentCharacter := StartChar;
  If NextChar^ = '$' then
    Repeat
      Inc (NextChar);
      If not IsCompilerDirective (NextChar, Directive) then Error (InvalidCompilerDirective);
      Case Directive of
        Directive_A: SetSwitch (WordAlignment, True);
        Directive_B: SetSwitch (FullBooleanEval, True);
        Directive_C: Break;
        Directive_D: SetSwitch (DebugInformation, True);
        Directive_DEFINE: begin
                            ExpectIdentifierInDirective;
                            AddConditionalDefine;
                            Break;
                          end;
        Directive_E: SetSwitch (Emulation80x87, True);
        Directive_ELSE: begin
                          If CurrentSourceFile^.IF_Level = 0 then Error (MisplacedConditionalDirective);
                          SkipComment (NextChar);
                          If Find_ENDIF then Dec (CurrentSourceFile^.IF_Level);
                          Exit;
                        end;
        Directive_ENDIF: begin
                           If CurrentSourceFile^.IF_Level = 0 then Error (MisplacedConditionalDirective);
                           Dec (CurrentSourceFile^.IF_Level);
                           Break;
                         end;
        Directive_F: SetSwitch (ForceFarCalls, True);
        Directive_G: If not SetSwitch (Instructions80286, False) then Break;
        Directive_I: begin
                       If SetSwitch (IOErrorChecking, False) then Continue;
                       GetDirectiveFileName;
                       SkipComment (NextChar);
                       CurrentSourceFile^.CurrentPosition := Ofs (NextChar^);
                       ErrorSourcePosition := NextChar;
                       CreateSourceFilesRecord;
                       FindFilePath (@Identifier, Dir_None or Ext_PAS);
                       FileRecord := StoreFileNameToFilesBlock (@Identifier, ufIncludeFile);
                       FindFilePath (@Identifier, Dir_Include or Ext_Original);
                       OpenSourceFile (FileRecord, @Identifier);
                       SetFileDateTime;
                       NextChar := Ptr (Seg (NextChar^), CurrentSourceFile^.CurrentPosition);
                       Exit;
                     end;
        Directive_IFDEF: begin
                           ExpectIdentifierInDirective;
                           ProcessIF (IdentifierInConditionalDefines);
                           Exit;
                         end;
        Directive_IFNDEF: begin
                            ExpectIdentifierInDirective;
                            ProcessIF (not IdentifierInConditionalDefines);
                            Exit;
                          end;
        Directive_IFOPT: begin
                           SkipSeparators (NextChar);
                           If not IsCompilerDirective (NextChar, SecondDirective) then Error (InvalidCompilerDirective);
                           Case SecondDirective of
                             Directive_A: Switch := WordAlignment;
                             Directive_B: Switch := FullBooleanEval;
                             Directive_D: Switch := DebugInformation;
                             Directive_E: Switch := Emulation80x87;
                             Directive_F: Switch := ForceFarCalls;
                             Directive_G: Switch := Instructions80286;
                             Directive_I: Switch := IOErrorChecking;
                             Directive_K: Switch := CompilerDirective_K;
                             Directive_L: Switch := LocalDebugSymbols;
                             Directive_N: Switch := Instructions80x87;
                             Directive_O: Switch := OverlaysAllowed;
                             Directive_P: Switch := OpenStringParams;
                             Directive_Q: Switch := OverflowChecking;
                             Directive_R: Switch := RangeChecking;
                             Directive_S: Switch := StackChecking;
                             Directive_T: Switch := TypedPointers;
                             Directive_V: Switch := StrictVarStrings;
                             Directive_W: Switch := CompilerDirective_W;
                             Directive_X: Switch := ExtendedSyntax;
                             Directive_Y: Switch := CompilerDirective_Y;
                             else Error (InvalidCompilerDirective);
                           end;
                           Case NextChar^ of
                             '+': ProcessIF (Switch in ModuleCompilerSwitches);
                             '-': ProcessIF (not (Switch in ModuleCompilerSwitches));
                             else Error (InvalidCompilerDirective);
                           end;
                           Exit;
                         end;
        Directive_K: SetSwitch (CompilerDirective_K, True);
        Directive_L: begin
                       If SetSwitch (LocalDebugSymbols, False) then Continue;
                       GetDirectiveFileName;
                       FindFilePath (@Identifier, Dir_None or Ext_OBJ);
                       StoreFileNameToFilesBlock (@Identifier, ufObjectFile);
                       Break;
                     end;
        Directive_M: begin
                       If not IsIntegerNumericConstantNext then Error (InvalidCompilerDirective);
                       If IntegerNumericConstantRec.WordH &lt;&gt; 0 then Error (InvalidCompilerDirective);
                       IntegerNumericConstantRec.WordL := IntegerNumericConstantRec.WordL and $FFFE;
                       If (IntegerNumericConstantRec.WordL &lt; $4000) or (IntegerNumericConstantRec.WordL &gt; $FFF0) then
                         Error (InvalidCompilerDirective);
                       ModuleStack := IntegerNumericConstantRec.WordL;
                       If not IsSegmentNext then Error (InvalidCompilerDirective);
                       If IntegerNumericConstantRec.WordL &gt; $A000 then Error (InvalidCompilerDirective);
                       ModuleHeapMin := IntegerNumericConstantRec.WordL;
                       If not IsSegmentNext then Error (InvalidCompilerDirective);
                       If IntegerNumericConstantRec.WordL &lt; ModuleHeapMin then Error (InvalidCompilerDirective);
                       If IntegerNumericConstantRec.WordL &gt; $A000 then Error (InvalidCompilerDirective);
                       ModuleHeapMax := IntegerNumericConstantRec.WordL;
                       Break;
                     end;
        Directive_N: SetSwitch (Instructions80x87, True);
        Directive_O: begin
                       If SetSwitch (OverlaysAllowed, False) then Continue;
                       If SourceType &gt;= stUnit then Error (InvalidCompilerDirective);
                       If not (cmoCompileToDisk in CompilerModeOptions) then Error (CannotCompileOverlaysToMemory);
                       ExpectIdentifierInDirective;
                       If not UnitFound (@Identifier, UnitPtr) then Error (UnknownIdentifier);
                       If not (ufOverlaysAllowed in UnitPtr^.Flags) then Error (CannotOverlayThisUnit);
                       UnitPtr^.OverlayedUnitCodeSize := $FFFF;
                       Break;
                     end;
        Directive_P: SetSwitch (OpenStringParams, True);
        Directive_Q: SetSwitch (OverflowChecking, True);
        Directive_R: If not SetSwitch (RangeChecking, False) then Break;
        Directive_S: If not SetSwitch (StackChecking, False) then Break;
        Directive_T: SetSwitch (TypedPointers, True);
        Directive_UNDEF: begin
                           ExpectIdentifierInDirective;
                           RemoveConditionalDefine;
                           Break;
                         end;
        Directive_V: SetSwitch (StrictVarStrings, True);
        Directive_W: SetSwitch (CompilerDirective_W, True);
        Directive_X: SetSwitch (ExtendedSyntax, True);
        Directive_Y: SetSwitch (CompilerDirective_Y, True);
      end;
    until NextChar^ &lt;&gt; ',';
  SkipComment (NextChar);
end;
This function checks if comment is actually a compiler directive.
TDirective = (
    Directive_A,
    Directive_B,
    Directive_C,
    Directive_D,
    Directive_DEFINE,
    Directive_E,
    Directive_ELSE,
    Directive_ENDIF,
    Directive_F,
    Directive_G,
    Directive_I,
    Directive_IFDEF,
    Directive_IFNDEF,
    Directive_IFOPT,
    Directive_K,
    Directive_L,
    Directive_M,
    Directive_N,
    Directive_O,
    Directive_P,
    Directive_Q,
    Directive_R,
    Directive_S,
    Directive_T,
    Directive_UNDEF,
    Directive_V,
    Directive_W,
    Directive_X,
    Directive_Y,
    InvalidDirective);

Function IsCompilerDirective (Var NextChar: PChar; Var Directive: TDirective): Boolean;
Const DirectiveStr: Array [Directive_A..Directive_Y] of PChar = (
        'A',
        'B',
        'C',
        'D',
        'DEFINE',
        'E',
        'ELSE',
        'ENDIF',
        'F',
        'G',
        'I',
        'IFDEF',
        'IFNDEF',
        'IFOPT',
        'K',
        'L',
        'M',
        'N',
        'O',
        'P',
        'Q',
        'R',
        'S',
        'T',
        'UNDEF',
        'V',
        'W',
        'X',
        'Y');
begin
  ErrorSourcePosition := NextChar;
  CopyToIdentifier (NextChar);
  IsCompilerDirective := True;
  Directive := Directive_A;
  Repeat
    If Identifier = StrPas (DirectiveStr [Directive]) then Exit;
    Inc (Directive);
  until Directive = InvalidDirective;
  IsCompilerDirective := False;
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
Procedure ProcessComment (Var NextChar: PChar; StartChar: Char);
Var SecondDirective, Directive: TDirective;
    Switch: TCompilerSwitches;
    FileRecord: Pointer;
    UnitPtr: PUnitHeader;
    IntegerNumericConstant: LongInt;

  Procedure SkipComment (Var NextChar: PChar);
  Var C: Char;
      TwoChars: TTwoChars;
  begin
    If CommentCharacter = '{' then
      Repeat
        C := NextChar^;
        Inc (NextChar);
        If C = #0 then
          begin
            If not NextLine then Error (UnexpectedEndOfFile);
            NextChar := CurrentLine;
            Continue;
          end;
      until C = '}'
        else Repeat
               TwoChars := PTwoChars (NextChar - 1)^;
               Inc (NextChar);
               If TwoChars.Second = #0 then
                 begin
                   If not NextLine then Error (UnexpectedEndOfFile);
                   NextChar := CurrentLine;
                   Continue;
                 end;
             until TwoChars.Both = '*)';
  end;

  Procedure ExpectIdentifierInDirective;
  begin
    SkipSeparators (NextChar);
    ErrorSourcePosition := NextChar;
    CopyToIdentifier (NextChar);
    If Identifier = '' then Error (IdentifierExpected);
  end;

  Function Find_ENDIF: Boolean;
  Type TDirectiveData = Record
                          EndsENDIF: Boolean;
                          IfLevelIncrement: Integer;
                        end;
  Var IfLevel: Byte;
      StringFlag: Boolean;
      DirectiveData: TDirectiveData;
      NewDirective: TDirective;
      TwoChars: TTwoChars;
  begin
    IfLevel := 0;
    StringFlag := False;
    Repeat
      Case NextChar^ of
        #0: begin
              If not NextLine then Error (ENDIF_directiveMissing);
              NextChar := CurrentLine;
              StringFlag := False;
              Continue;
            end;
        '''': begin
                StringFlag := not StringFlag;
                Inc (NextChar);
                Continue;
              end;
      end;
      If StringFlag then
        begin
          Inc (NextChar);
          Continue;
        end;
      TwoChars := PTwoChars (NextChar)^;
      Inc (NextChar);
      If (TwoChars.First = '{') or (TwoChars.Both = '(*') then
        begin
          If TwoChars.First &lt;&gt; '{' then Inc (NextChar);
          CommentCharacter := TwoChars.First;
          With DirectiveData do
            begin
              EndsENDIF := False;
              IfLevelIncrement := 0;
            end;
          If NextChar^ = '$' then
            begin
              Inc (NextChar);
              If IsCompilerDirective (NextChar, NewDirective) then With DirectiveData do
                Case NewDirective of
                  Directive_ELSE:   begin
                                      EndsENDIF := True;
                                      IfLevelIncrement :=   0;
                                    end;
                  Directive_ENDIF:  begin
                                      EndsENDIF := True;
                                      IfLevelIncrement := - 1;
                                    end;
                  Directive_IFDEF,
                  Directive_IFNDEF,
                  Directive_IFOPT:  begin
                                      EndsENDIF := False;
                                      IfLevelIncrement :=   1;
                                    end;
                end;
            end;
          SkipComment (NextChar);
          If DirectiveData.EndsENDIF and (IfLevel = 0) then
            begin
              Find_ENDIF := DirectiveData.IfLevelIncrement &lt;&gt; 0;
              Exit;
            end;
          Inc (IfLevel, DirectiveData.IfLevelIncrement);
          StringFlag := False;
        end;
    until False;
  end;

  Procedure GetDirectiveFileName;
  Var Len: Byte;
  begin
    SkipSeparators (NextChar);
    ErrorSourcePosition := NextChar;
    Len := 0;
    Repeat
      Case NextChar^ of
        #0..' ',
        '*', '}': Break;
        else Identifier [Len] := UpCase (NextChar^);
      end;
      Inc (Len);
      Inc (NextChar);
    until Len = 79;
    Identifier [Len] := #0;
  end;

  Procedure ProcessIF (Cond: Boolean);
  begin
    SkipComment (NextChar);
    Inc (CurrentSourceFile^.IF_Level);
    If not Cond then
      If Find_ENDIF then Dec (CurrentSourceFile^.IF_Level);
  end;

  Function IsIntegerNumericConstantNext: Boolean;
  begin
    SkipSeparators (NextChar);
    ErrorSourcePosition := NextChar;
    IsIntegerNumericConstantNext := IsIntegerNumericConstant (NextChar);
  end;

  Function IsSegmentNext: Boolean;
  begin
    IsSegmentNext := False;
    If not IsIntegerNumericConstantNext then Exit;
    With IntegerNumericConstantRec do If (WordH = $FFFF) and (WordL &gt; $FFF0) then Exit;
    IntegerNumericConstant := (IntegerNumericConstant + $F) shr 4;
    IsSegmentNext := IntegerNumericConstantRec.WordH = 0;
  end;

  Procedure RemoveConditionalDefine;
  Var SrcPtr, DestPtr: PChar;
      Len: Byte;
  begin
    If not IdentifierInConditionalDefines then Exit;
    Len := Byte (DestPtr^);
    SrcPtr := DestPtr + Len + 1;
    Move (SrcPtr^, DestPtr^, Ofs (ConditionalDefinesEnd^) - Ofs (SrcPtr^));
    ConditionalDefinesEnd := Ptr (Seg (ConditionalDefinesEnd^), Ofs (ConditionalDefinesEnd^) - Len - 1);
  end;

  Function UnitFound (Name: PString; Var TempUnit: PUnitHeader): Boolean;
  Var UnitSegment: Word;
      UnitName: PString;
  begin
    UnitFound := True;
    UnitSegment := LastLoadedUsedUnit;
    Repeat
      TempUnit := Ptr (UnitSegment, 0);
      UnitName := @PIdentifier (Ptr (UnitSegment, TempUnit^.UnitNameIdentifierOffset))^.Name;
      If IdentifiersEqual (Name, UnitName) then Exit;
      UnitSegment := TempUnit^.PreviousUnitSegment;
    until UnitSegment = 0;
    UnitFound := False;
  end;

  Function SetSwitch (CompilerSwitch: TCompilerSwitches; ReportError: Boolean): Boolean;
  begin
    SetSwitch := False;
    If not (NextChar^ in ['+', '-']) then
      Case ReportError of
        True: Error (InvalidCompilerDirective);
        else Exit;
      end;
    If ModuleHeadingEnded then If CompilerSwitch in GlobalCompilerSwitches then Error (InvalidCompilerDirective);
    Case NextChar^ of
      '+': Include (ModuleCompilerSwitches, CompilerSwitch);
      '-': Exclude (ModuleCompilerSwitches, CompilerSwitch);
    end;
    SetSwitch := True;
    Inc (NextChar);
  end;

begin
  CommentCharacter := StartChar;
  If NextChar^ = '$' then
    Repeat
      Inc (NextChar);
      If not IsCompilerDirective (NextChar, Directive) then Error (InvalidCompilerDirective);
      Case Directive of
        Directive_A: SetSwitch (WordAlignment, True);
        Directive_B: SetSwitch (FullBooleanEval, True);
        Directive_C: Break;
        Directive_D: SetSwitch (DebugInformation, True);
        Directive_DEFINE: begin
                            ExpectIdentifierInDirective;
                            AddConditionalDefine;
                            Break;
                          end;
        Directive_E: SetSwitch (Emulation80x87, True);
        Directive_ELSE: begin
                          If CurrentSourceFile^.IF_Level = 0 then Error (MisplacedConditionalDirective);
                          SkipComment (NextChar);
                          If Find_ENDIF then Dec (CurrentSourceFile^.IF_Level);
                          Exit;
                        end;
        Directive_ENDIF: begin
                           If CurrentSourceFile^.IF_Level = 0 then Error (MisplacedConditionalDirective);
                           Dec (CurrentSourceFile^.IF_Level);
                           Break;
                         end;
        Directive_F: SetSwitch (ForceFarCalls, True);
        Directive_G: If not SetSwitch (Instructions80286, False) then Break;
        Directive_I: begin
                       If SetSwitch (IOErrorChecking, False) then Continue;
                       GetDirectiveFileName;
                       SkipComment (NextChar);
                       CurrentSourceFile^.CurrentPosition := Ofs (NextChar^);
                       ErrorSourcePosition := NextChar;
                       CreateSourceFilesRecord;
                       FindFilePath (@Identifier, Dir_None or Ext_PAS);
                       FileRecord := StoreFileNameToFilesBlock (@Identifier, ufIncludeFile);
                       FindFilePath (@Identifier, Dir_Include or Ext_Original);
                       OpenSourceFile (FileRecord, @Identifier);
                       SetFileDateTime;
                       NextChar := Ptr (Seg (NextChar^), CurrentSourceFile^.CurrentPosition);
                       Exit;
                     end;
        Directive_IFDEF: begin
                           ExpectIdentifierInDirective;
                           ProcessIF (IdentifierInConditionalDefines);
                           Exit;
                         end;
        Directive_IFNDEF: begin
                            ExpectIdentifierInDirective;
                            ProcessIF (not IdentifierInConditionalDefines);
                            Exit;
                          end;
        Directive_IFOPT: begin
                           SkipSeparators (NextChar);
                           If not IsCompilerDirective (NextChar, SecondDirective) then Error (InvalidCompilerDirective);
                           Case SecondDirective of
                             Directive_A: Switch := WordAlignment;
                             Directive_B: Switch := FullBooleanEval;
                             Directive_D: Switch := DebugInformation;
                             Directive_E: Switch := Emulation80x87;
                             Directive_F: Switch := ForceFarCalls;
                             Directive_G: Switch := Instructions80286;
                             Directive_I: Switch := IOErrorChecking;
                             Directive_K: Switch := CompilerDirective_K;
                             Directive_L: Switch := LocalDebugSymbols;
                             Directive_N: Switch := Instructions80x87;
                             Directive_O: Switch := OverlaysAllowed;
                             Directive_P: Switch := OpenStringParams;
                             Directive_Q: Switch := OverflowChecking;
                             Directive_R: Switch := RangeChecking;
                             Directive_S: Switch := StackChecking;
                             Directive_T: Switch := TypedPointers;
                             Directive_V: Switch := StrictVarStrings;
                             Directive_W: Switch := CompilerDirective_W;
                             Directive_X: Switch := ExtendedSyntax;
                             Directive_Y: Switch := CompilerDirective_Y;
                             else Error (InvalidCompilerDirective);
                           end;
                           Case NextChar^ of
                             '+': ProcessIF (Switch in ModuleCompilerSwitches);
                             '-': ProcessIF (not (Switch in ModuleCompilerSwitches));
                             else Error (InvalidCompilerDirective);
                           end;
                           Exit;
                         end;
        Directive_K: SetSwitch (CompilerDirective_K, True);
        Directive_L: begin
                       If SetSwitch (LocalDebugSymbols, False) then Continue;
                       GetDirectiveFileName;
                       FindFilePath (@Identifier, Dir_None or Ext_OBJ);
                       StoreFileNameToFilesBlock (@Identifier, ufObjectFile);
                       Break;
                     end;
        Directive_M: begin
                       If not IsIntegerNumericConstantNext then Error (InvalidCompilerDirective);
                       If IntegerNumericConstantRec.WordH &lt;&gt; 0 then Error (InvalidCompilerDirective);
                       IntegerNumericConstantRec.WordL := IntegerNumericConstantRec.WordL and $FFFE;
                       If (IntegerNumericConstantRec.WordL &lt; $4000) or (IntegerNumericConstantRec.WordL &gt; $FFF0) then
                         Error (InvalidCompilerDirective);
                       ModuleStack := IntegerNumericConstantRec.WordL;
                       If not IsSegmentNext then Error (InvalidCompilerDirective);
                       If IntegerNumericConstantRec.WordL &gt; $A000 then Error (InvalidCompilerDirective);
                       ModuleHeapMin := IntegerNumericConstantRec.WordL;
                       If not IsSegmentNext then Error (InvalidCompilerDirective);
                       If IntegerNumericConstantRec.WordL &lt; ModuleHeapMin then Error (InvalidCompilerDirective);
                       If IntegerNumericConstantRec.WordL &gt; $A000 then Error (InvalidCompilerDirective);
                       ModuleHeapMax := IntegerNumericConstantRec.WordL;
                       Break;
                     end;
        Directive_N: SetSwitch (Instructions80x87, True);
        Directive_O: begin
                       If SetSwitch (OverlaysAllowed, False) then Continue;
                       If SourceType &gt;= stUnit then Error (InvalidCompilerDirective);
                       If not (cmoCompileToDisk in CompilerModeOptions) then Error (CannotCompileOverlaysToMemory);
                       ExpectIdentifierInDirective;
                       If not UnitFound (@Identifier, UnitPtr) then Error (UnknownIdentifier);
                       If not (ufOverlaysAllowed in UnitPtr^.Flags) then Error (CannotOverlayThisUnit);
                       UnitPtr^.OverlayedUnitCodeSize := $FFFF;
                       Break;
                     end;
        Directive_P: SetSwitch (OpenStringParams, True);
        Directive_Q: SetSwitch (OverflowChecking, True);
        Directive_R: If not SetSwitch (RangeChecking, False) then Break;
        Directive_S: If not SetSwitch (StackChecking, False) then Break;
        Directive_T: SetSwitch (TypedPointers, True);
        Directive_UNDEF: begin
                           ExpectIdentifierInDirective;
                           RemoveConditionalDefine;
                           Break;
                         end;
        Directive_V: SetSwitch (StrictVarStrings, True);
        Directive_W: SetSwitch (CompilerDirective_W, True);
        Directive_X: SetSwitch (ExtendedSyntax, True);
        Directive_Y: SetSwitch (CompilerDirective_Y, True);
      end;
    until NextChar^ &lt;&gt; ',';
  SkipComment (NextChar);
end;
```

### Code Block 2

```pascal
TDirective = (
    Directive_A,
    Directive_B,
    Directive_C,
    Directive_D,
    Directive_DEFINE,
    Directive_E,
    Directive_ELSE,
    Directive_ENDIF,
    Directive_F,
    Directive_G,
    Directive_I,
    Directive_IFDEF,
    Directive_IFNDEF,
    Directive_IFOPT,
    Directive_K,
    Directive_L,
    Directive_M,
    Directive_N,
    Directive_O,
    Directive_P,
    Directive_Q,
    Directive_R,
    Directive_S,
    Directive_T,
    Directive_UNDEF,
    Directive_V,
    Directive_W,
    Directive_X,
    Directive_Y,
    InvalidDirective);

Function IsCompilerDirective (Var NextChar: PChar; Var Directive: TDirective): Boolean;
Const DirectiveStr: Array [Directive_A..Directive_Y] of PChar = (
        'A',
        'B',
        'C',
        'D',
        'DEFINE',
        'E',
        'ELSE',
        'ENDIF',
        'F',
        'G',
        'I',
        'IFDEF',
        'IFNDEF',
        'IFOPT',
        'K',
        'L',
        'M',
        'N',
        'O',
        'P',
        'Q',
        'R',
        'S',
        'T',
        'UNDEF',
        'V',
        'W',
        'X',
        'Y');
begin
  ErrorSourcePosition := NextChar;
  CopyToIdentifier (NextChar);
  IsCompilerDirective := True;
  Directive := Directive_A;
  Repeat
    If Identifier = StrPas (DirectiveStr [Directive]) then Exit;
    Inc (Directive);
  until Directive = InvalidDirective;
  IsCompilerDirective := False;
end;
```

### Code Block 3

```pascal
Procedure ProcessComment (Var NextChar: PChar; StartChar: Char);
Var SecondDirective, Directive: TDirective;
    Switch: TCompilerSwitches;
    FileRecord: Pointer;
    UnitPtr: PUnitHeader;
    IntegerNumericConstant: LongInt;

  Procedure SkipComment (Var NextChar: PChar);
  Var C: Char;
      TwoChars: TTwoChars;
  begin
    If CommentCharacter = '{' then
      Repeat
        C := NextChar^;
        Inc (NextChar);
        If C = #0 then
          begin
            If not NextLine then Error (UnexpectedEndOfFile);
            NextChar := CurrentLine;
            Continue;
          end;
      until C = '}'
        else Repeat
               TwoChars := PTwoChars (NextChar - 1)^;
               Inc (NextChar);
               If TwoChars.Second = #0 then
                 begin
                   If not NextLine then Error (UnexpectedEndOfFile);
                   NextChar := CurrentLine;
                   Continue;
                 end;
             until TwoChars.Both = '*)';
  end;

  Procedure ExpectIdentifierInDirective;
  begin
    SkipSeparators (NextChar);
    ErrorSourcePosition := NextChar;
    CopyToIdentifier (NextChar);
    If Identifier = '' then Error (IdentifierExpected);
  end;

  Function Find_ENDIF: Boolean;
  Type TDirectiveData = Record
                          EndsENDIF: Boolean;
                          IfLevelIncrement: Integer;
                        end;
  Var IfLevel: Byte;
      StringFlag: Boolean;
      DirectiveData: TDirectiveData;
      NewDirective: TDirective;
      TwoChars: TTwoChars;
  begin
    IfLevel := 0;
    StringFlag := False;
    Repeat
      Case NextChar^ of
        #0: begin
              If not NextLine then Error (ENDIF_directiveMissing);
              NextChar := CurrentLine;
              StringFlag := False;
              Continue;
            end;
        '''': begin
                StringFlag := not StringFlag;
                Inc (NextChar);
                Continue;
              end;
      end;
      If StringFlag then
        begin
          Inc (NextChar);
          Continue;
        end;
      TwoChars := PTwoChars (NextChar)^;
      Inc (NextChar);
      If (TwoChars.First = '{') or (TwoChars.Both = '(*') then
        begin
          If TwoChars.First &lt;&gt; '{' then Inc (NextChar);
          CommentCharacter := TwoChars.First;
          With DirectiveData do
            begin
              EndsENDIF := False;
              IfLevelIncrement := 0;
            end;
          If NextChar^ = '$' then
            begin
              Inc (NextChar);
              If IsCompilerDirective (NextChar, NewDirective) then With DirectiveData do
                Case NewDirective of
                  Directive_ELSE:   begin
                                      EndsENDIF := True;
                                      IfLevelIncrement :=   0;
                                    end;
                  Directive_ENDIF:  begin
                                      EndsENDIF := True;
                                      IfLevelIncrement := - 1;
                                    end;
                  Directive_IFDEF,
                  Directive_IFNDEF,
                  Directive_IFOPT:  begin
                                      EndsENDIF := False;
                                      IfLevelIncrement :=   1;
                                    end;
                end;
            end;
          SkipComment (NextChar);
          If DirectiveData.EndsENDIF and (IfLevel = 0) then
            begin
              Find_ENDIF := DirectiveData.IfLevelIncrement &lt;&gt; 0;
              Exit;
            end;
          Inc (IfLevel, DirectiveData.IfLevelIncrement);
          StringFlag := False;
        end;
    until False;
  end;

  Procedure GetDirectiveFileName;
  Var Len: Byte;
  begin
    SkipSeparators (NextChar);
    ErrorSourcePosition := NextChar;
    Len := 0;
    Repeat
      Case NextChar^ of
        #0..' ',
        '*', '}': Break;
        else Identifier [Len] := UpCase (NextChar^);
      end;
      Inc (Len);
      Inc (NextChar);
    until Len = 79;
    Identifier [Len] := #0;
  end;

  Procedure ProcessIF (Cond: Boolean);
  begin
    SkipComment (NextChar);
    Inc (CurrentSourceFile^.IF_Level);
    If not Cond then
      If Find_ENDIF then Dec (CurrentSourceFile^.IF_Level);
  end;

  Function IsIntegerNumericConstantNext: Boolean;
  begin
    SkipSeparators (NextChar);
    ErrorSourcePosition := NextChar;
    IsIntegerNumericConstantNext := IsIntegerNumericConstant (NextChar);
  end;

  Function IsSegmentNext: Boolean;
  begin
    IsSegmentNext := False;
    If not IsIntegerNumericConstantNext then Exit;
    With IntegerNumericConstantRec do If (WordH = $FFFF) and (WordL &gt; $FFF0) then Exit;
    IntegerNumericConstant := (IntegerNumericConstant + $F) shr 4;
    IsSegmentNext := IntegerNumericConstantRec.WordH = 0;
  end;

  Procedure RemoveConditionalDefine;
  Var SrcPtr, DestPtr: PChar;
      Len: Byte;
  begin
    If not IdentifierInConditionalDefines then Exit;
    Len := Byte (DestPtr^);
    SrcPtr := DestPtr + Len + 1;
    Move (SrcPtr^, DestPtr^, Ofs (ConditionalDefinesEnd^) - Ofs (SrcPtr^));
    ConditionalDefinesEnd := Ptr (Seg (ConditionalDefinesEnd^), Ofs (ConditionalDefinesEnd^) - Len - 1);
  end;

  Function UnitFound (Name: PString; Var TempUnit: PUnitHeader): Boolean;
  Var UnitSegment: Word;
      UnitName: PString;
  begin
    UnitFound := True;
    UnitSegment := LastLoadedUsedUnit;
    Repeat
      TempUnit := Ptr (UnitSegment, 0);
      UnitName := @PIdentifier (Ptr (UnitSegment, TempUnit^.UnitNameIdentifierOffset))^.Name;
      If IdentifiersEqual (Name, UnitName) then Exit;
      UnitSegment := TempUnit^.PreviousUnitSegment;
    until UnitSegment = 0;
    UnitFound := False;
  end;

  Function SetSwitch (CompilerSwitch: TCompilerSwitches; ReportError: Boolean): Boolean;
  begin
    SetSwitch := False;
    If not (NextChar^ in ['+', '-']) then
      Case ReportError of
        True: Error (InvalidCompilerDirective);
        else Exit;
      end;
    If ModuleHeadingEnded then If CompilerSwitch in GlobalCompilerSwitches then Error (InvalidCompilerDirective);
    Case NextChar^ of
      '+': Include (ModuleCompilerSwitches, CompilerSwitch);
      '-': Exclude (ModuleCompilerSwitches, CompilerSwitch);
    end;
    SetSwitch := True;
    Inc (NextChar);
  end;

begin
  CommentCharacter := StartChar;
  If NextChar^ = '$' then
    Repeat
      Inc (NextChar);
      If not IsCompilerDirective (NextChar, Directive) then Error (InvalidCompilerDirective);
      Case Directive of
        Directive_A: SetSwitch (WordAlignment, True);
        Directive_B: SetSwitch (FullBooleanEval, True);
        Directive_C: Break;
        Directive_D: SetSwitch (DebugInformation, True);
        Directive_DEFINE: begin
                            ExpectIdentifierInDirective;
                            AddConditionalDefine;
                            Break;
                          end;
        Directive_E: SetSwitch (Emulation80x87, True);
        Directive_ELSE: begin
                          If CurrentSourceFile^.IF_Level = 0 then Error (MisplacedConditionalDirective);
                          SkipComment (NextChar);
                          If Find_ENDIF then Dec (CurrentSourceFile^.IF_Level);
                          Exit;
                        end;
        Directive_ENDIF: begin
                           If CurrentSourceFile^.IF_Level = 0 then Error (MisplacedConditionalDirective);
                           Dec (CurrentSourceFile^.IF_Level);
                           Break;
                         end;
        Directive_F: SetSwitch (ForceFarCalls, True);
        Directive_G: If not SetSwitch (Instructions80286, False) then Break;
        Directive_I: begin
                       If SetSwitch (IOErrorChecking, False) then Continue;
                       GetDirectiveFileName;
                       SkipComment (NextChar);
                       CurrentSourceFile^.CurrentPosition := Ofs (NextChar^);
                       ErrorSourcePosition := NextChar;
                       CreateSourceFilesRecord;
                       FindFilePath (@Identifier, Dir_None or Ext_PAS);
                       FileRecord := StoreFileNameToFilesBlock (@Identifier, ufIncludeFile);
                       FindFilePath (@Identifier, Dir_Include or Ext_Original);
                       OpenSourceFile (FileRecord, @Identifier);
                       SetFileDateTime;
                       NextChar := Ptr (Seg (NextChar^), CurrentSourceFile^.CurrentPosition);
                       Exit;
                     end;
        Directive_IFDEF: begin
                           ExpectIdentifierInDirective;
                           ProcessIF (IdentifierInConditionalDefines);
                           Exit;
                         end;
        Directive_IFNDEF: begin
                            ExpectIdentifierInDirective;
                            ProcessIF (not IdentifierInConditionalDefines);
                            Exit;
                          end;
        Directive_IFOPT: begin
                           SkipSeparators (NextChar);
                           If not IsCompilerDirective (NextChar, SecondDirective) then Error (InvalidCompilerDirective);
                           Case SecondDirective of
                             Directive_A: Switch := WordAlignment;
                             Directive_B: Switch := FullBooleanEval;
                             Directive_D: Switch := DebugInformation;
                             Directive_E: Switch := Emulation80x87;
                             Directive_F: Switch := ForceFarCalls;
                             Directive_G: Switch := Instructions80286;
                             Directive_I: Switch := IOErrorChecking;
                             Directive_K: Switch := CompilerDirective_K;
                             Directive_L: Switch := LocalDebugSymbols;
                             Directive_N: Switch := Instructions80x87;
                             Directive_O: Switch := OverlaysAllowed;
                             Directive_P: Switch := OpenStringParams;
                             Directive_Q: Switch := OverflowChecking;
                             Directive_R: Switch := RangeChecking;
                             Directive_S: Switch := StackChecking;
                             Directive_T: Switch := TypedPointers;
                             Directive_V: Switch := StrictVarStrings;
                             Directive_W: Switch := CompilerDirective_W;
                             Directive_X: Switch := ExtendedSyntax;
                             Directive_Y: Switch := CompilerDirective_Y;
                             else Error (InvalidCompilerDirective);
                           end;
                           Case NextChar^ of
                             '+': ProcessIF (Switch in ModuleCompilerSwitches);
                             '-': ProcessIF (not (Switch in ModuleCompilerSwitches));
                             else Error (InvalidCompilerDirective);
                           end;
                           Exit;
                         end;
        Directive_K: SetSwitch (CompilerDirective_K, True);
        Directive_L: begin
                       If SetSwitch (LocalDebugSymbols, False) then Continue;
                       GetDirectiveFileName;
                       FindFilePath (@Identifier, Dir_None or Ext_OBJ);
                       StoreFileNameToFilesBlock (@Identifier, ufObjectFile);
                       Break;
                     end;
        Directive_M: begin
                       If not IsIntegerNumericConstantNext then Error (InvalidCompilerDirective);
                       If IntegerNumericConstantRec.WordH &lt;&gt; 0 then Error (InvalidCompilerDirective);
                       IntegerNumericConstantRec.WordL := IntegerNumericConstantRec.WordL and $FFFE;
                       If (IntegerNumericConstantRec.WordL &lt; $4000) or (IntegerNumericConstantRec.WordL &gt; $FFF0) then
                         Error (InvalidCompilerDirective);
                       ModuleStack := IntegerNumericConstantRec.WordL;
                       If not IsSegmentNext then Error (InvalidCompilerDirective);
                       If IntegerNumericConstantRec.WordL &gt; $A000 then Error (InvalidCompilerDirective);
                       ModuleHeapMin := IntegerNumericConstantRec.WordL;
                       If not IsSegmentNext then Error (InvalidCompilerDirective);
                       If IntegerNumericConstantRec.WordL &lt; ModuleHeapMin then Error (InvalidCompilerDirective);
                       If IntegerNumericConstantRec.WordL &gt; $A000 then Error (InvalidCompilerDirective);
                       ModuleHeapMax := IntegerNumericConstantRec.WordL;
                       Break;
                     end;
        Directive_N: SetSwitch (Instructions80x87, True);
        Directive_O: begin
                       If SetSwitch (OverlaysAllowed, False) then Continue;
                       If SourceType &gt;= stUnit then Error (InvalidCompilerDirective);
                       If not (cmoCompileToDisk in CompilerModeOptions) then Error (CannotCompileOverlaysToMemory);
                       ExpectIdentifierInDirective;
                       If not UnitFound (@Identifier, UnitPtr) then Error (UnknownIdentifier);
                       If not (ufOverlaysAllowed in UnitPtr^.Flags) then Error (CannotOverlayThisUnit);
                       UnitPtr^.OverlayedUnitCodeSize := $FFFF;
                       Break;
                     end;
        Directive_P: SetSwitch (OpenStringParams, True);
        Directive_Q: SetSwitch (OverflowChecking, True);
        Directive_R: If not SetSwitch (RangeChecking, False) then Break;
        Directive_S: If not SetSwitch (StackChecking, False) then Break;
        Directive_T: SetSwitch (TypedPointers, True);
        Directive_UNDEF: begin
                           ExpectIdentifierInDirective;
                           RemoveConditionalDefine;
                           Break;
                         end;
        Directive_V: SetSwitch (StrictVarStrings, True);
        Directive_W: SetSwitch (CompilerDirective_W, True);
        Directive_X: SetSwitch (ExtendedSyntax, True);
        Directive_Y: SetSwitch (CompilerDirective_Y, True);
      end;
    until NextChar^ &lt;&gt; ',';
  SkipComment (NextChar);
end;
```

### Code Block 4

```pascal
TDirective = (
    Directive_A,
    Directive_B,
    Directive_C,
    Directive_D,
    Directive_DEFINE,
    Directive_E,
    Directive_ELSE,
    Directive_ENDIF,
    Directive_F,
    Directive_G,
    Directive_I,
    Directive_IFDEF,
    Directive_IFNDEF,
    Directive_IFOPT,
    Directive_K,
    Directive_L,
    Directive_M,
    Directive_N,
    Directive_O,
    Directive_P,
    Directive_Q,
    Directive_R,
    Directive_S,
    Directive_T,
    Directive_UNDEF,
    Directive_V,
    Directive_W,
    Directive_X,
    Directive_Y,
    InvalidDirective);

Function IsCompilerDirective (Var NextChar: PChar; Var Directive: TDirective): Boolean;
Const DirectiveStr: Array [Directive_A..Directive_Y] of PChar = (
        'A',
        'B',
        'C',
        'D',
        'DEFINE',
        'E',
        'ELSE',
        'ENDIF',
        'F',
        'G',
        'I',
        'IFDEF',
        'IFNDEF',
        'IFOPT',
        'K',
        'L',
        'M',
        'N',
        'O',
        'P',
        'Q',
        'R',
        'S',
        'T',
        'UNDEF',
        'V',
        'W',
        'X',
        'Y');
begin
  ErrorSourcePosition := NextChar;
  CopyToIdentifier (NextChar);
  IsCompilerDirective := True;
  Directive := Directive_A;
  Repeat
    If Identifier = StrPas (DirectiveStr [Directive]) then Exit;
    Inc (Directive);
  until Directive = InvalidDirective;
  IsCompilerDirective := False;
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
- https://turbopascal.org/processing-comments-and-compiler-directives
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

