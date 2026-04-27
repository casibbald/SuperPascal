# Processing Command Line Parameters - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-command-line-parameters

---

## Table of Contents

- Processing Command Line Parameters

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Command Line Parameters
This procedure processes command line parameters and sets compiler flags and data accordingly.
Procedure ProcessCompilerParameters;

  Function CommaIsIntegerNumericConstant (Var ChrPtr: PChar): Boolean;
  begin
    If ChrPtr [0] = ',' then Inc (ChrPtr);
    CommaIsIntegerNumericConstant := IsIntegerNumericConstant (ChrPtr);
  end;

  Function IsSegment (Var ChrPtr: PChar): Boolean;
  begin
    IsSegment := False;
    If CommaIsIntegerNumericConstant (ChrPtr) then
      begin
        If IntegerNumericConstantRec.WordH = $FFFF then
          If IntegerNumericConstantRec.WordL > $FFF0 then Exit;
        IntegerNumericConstant := (IntegerNumericConstant + $F) shr 4;
        IsSegment := IntegerNumericConstantRec.WordH = 0;
      end;
  end;

  Procedure AddNameToList (List, Name: PChar);
  begin
    If (List^ = #0) or (Name^ = #0) then StrCopy (List, Name)
      else If StrLen (List) + StrLen (Name) <= 126 then
        begin
          List := StrEnd (List);
          List^ := ';';
          StrCopy (List + 1, Name);
        end;
  end;

  Procedure ProcessCommandLineCompilerDirectives (Var ChrPtr: PChar);

    Procedure ProcessSwitch (Switch: TCompilerSwitches);
    begin
      Inc (ChrPtr);
      With GlobalModuleData^ do
        begin
          Case ChrPtr [0] of
            '+': Include (CompilerSwitches, Switch);
            '-': Exclude (CompilerSwitches, Switch);
            else CmdErr := True;
          end;
        end;
    end;

  begin
    CmdErr := False;
    Repeat
      Inc (ChrPtr);
      Case Upcase (ChrPtr [0]) of
        'A': ProcessSwitch (WordAlignment);
        'B': ProcessSwitch (FullBooleanEval);
        'D': ProcessSwitch (DebugInformation);
        'E': ProcessSwitch (Emulation80x87);
        'F': ProcessSwitch (ForceFarCalls);
        'G': ProcessSwitch (Instructions80286);
        'I': ProcessSwitch (IOErrorChecking);
        'K': ProcessSwitch (CompilerDirective_K);
        'L': ProcessSwitch (LocalDebugSymbols);
        'M': begin
               CmdErr := True;
               Inc (ChrPtr);
               If CommaIsIntegerNumericConstant (ChrPtr) then
                 If IntegerNumericConstantRec.WordH = 0 then
                   begin
                     IntegerNumericConstantRec.WordL := IntegerNumericConstantRec.WordL and $FFFE;
                     If (IntegerNumericConstantRec.WordL >= $0400) and
                        (IntegerNumericConstantRec.WordL <= $FFF0) then
                       begin
                         GlobalModuleData^.Stack := IntegerNumericConstantRec.WordL;
                         If IsSegment (ChrPtr) then
                           If IntegerNumericConstantRec.WordL <= $A000 then
                             begin
                               GlobalModuleData^.HeapMin := IntegerNumericConstantRec.WordL;
                               If IsSegment (ChrPtr) then
                                 If IntegerNumericConstantRec.WordL >= ModuleHeapMin then
                                   If IntegerNumericConstantRec.WordL <= $A000 then
                                     begin
                                       GlobalModuleData^.HeapMax := IntegerNumericConstantRec.WordL;
                                       CmdErr := False;
                                     end;
                             end;

                       end;
                   end;
             end;
        'N': ProcessSwitch (Instructions80x87);
        'O': ProcessSwitch (OverlaysAllowed);
        'P': ProcessSwitch (OpenStringParams);
        'Q': ProcessSwitch (OverflowChecking);
        'R': ProcessSwitch (RangeChecking);
        'S': ProcessSwitch (StackChecking);
        'T': ProcessSwitch (TypedPointers);
        'V': ProcessSwitch (StrictVarStrings);
        'W': ProcessSwitch (CompilerDirective_W);
        'X': ProcessSwitch (ExtendedSyntax);
        'Y': ProcessSwitch (CompilerDirective_Y);
        else CmdErr := True;
      end;
      If CmdErr then Exit;
      Inc (ChrPtr);
    until ChrPtr [0] &lt;&gt; ',';
  end;

begin
  ReadParameter;
  While TempStr [0] <> #0 do
    begin
      CmdErr := False;
      Case TempStr [0] of
        '/', '-': Case TempStr [1] of
                    'B': CompilerModeOptions := [cmoCreateExeFile, cmoCompileToDisk, cmoSearchForUnitSource];
                    'D': AddNameToList (InitialConditionalDefines, TempStr + 2);
                    'E': StrCopy (EXE_TPU_Directory, TempStr + 2);
                    'F': begin
                           CmdErr := True;
                           CompilerModeOptions := [cmoCreateExeFile, cmoFindError];
                           Inc (TempStr, 2);
                           If IsHexConstant (TempStr) then
                             begin
                               FindErrorAddress.Seg := IntegerNumericConstantRec.WordL;
                               If TempStr [0] = ':' then
                                 begin
                                   Inc (TempStr);
                                   If IsHexConstant (TempStr) then
                                     begin
                                       CmdErr := False;
                                       FindErrorAddress.Ofs := IntegerNumericConstantRec.WordL;
                                     end;
                                 end;
                             end;
                         end;
                    'G': begin
                           LinkerOptions := LinkerOptions - [MapFileWithSegments, MapFileWithPublics];
                           Case TempStr [2] of
                             'S': LinkerOptions := LinkerOptions + [MapFileWithSegments];
                             'P': LinkerOptions := LinkerOptions + [MapFileWithPublics];
                             'D': LinkerOptions := LinkerOptions + [MapFileWithSegments, MapFileWithPublics];
                             else CmdErr := True;
                           end;
                         end;
                    'I': AddNameToList (IncludeDirectories, TempStr + 2);
                    'L': LinkerOptions := LinkerOptions + [LinkBufferOnDisk];
                    'M': CompilerModeOptions := [cmoCreateExeFile, cmoCompileToDisk,
                                                 cmoSearchForUnitSource, cmoMakeModifiedUnitsOnly];
                    'O': AddNameToList (ObjectsDirectories, TempStr + 2);
                    'Q': QuietCompile := True;
                    'T': AddNameToList (TPL_CFG_Directories, TempStr + 2);
                    'U': AddNameToList (UnitDirectories, TempStr + 2);
                    'V': LinkerOptions := LinkerOptions + [DebugInformationInExe];
                    '$': begin
                           Inc (TempStr);
                           ProcessCommandLineCompilerDirectives (TempStr);
                         end;
                    '#':
                    else CmdErr := True;
                  end;
        else StrCopy (MainFileName, TempStr);
      end;
      If CmdErr then
        begin
           Writeln ('Invalid command line option: "', TempStrBuff, '"');
           Halt (1);
        end;
      ReadParameter;
    end;
  If MainFileName [0] = #0 then
    begin
      WriteSyntax;
      Halt (0);
    end;
end;
This procedure reads first configuration file stored in FileBuffer (
CfgPtr
points to it) and then command line parameters (
CmdPtr
) and stores the parameter in
TempStr
.
Procedure ReadParameter;
Var Ch: Char;

  Function NextParameterChar: Char;
  begin
    If CfgPtr [0] <> #0 then NextParameterChar := CfgPtr [0]
      else NextParameterChar := CmdPtr [0];
  end;

  Procedure IncParameterCharPtr;
  begin
    If CfgPtr [0] <> #0 then Inc (CfgPtr)
      else Inc (CmdPtr);
 end;

 Procedure StoreParameterChar (C: Char);
 begin
   If TempStrPtr - TempStr >= 127 then Exit;
   TempStrPtr^ := C;
   Inc (TempStrPtr);
 end;

begin
  TempStrPtr := TempStr;
  Ch := NextParameterChar;
  While Ch <> #0 do
    begin
      If Ch > ' ' then
        begin
          IncParameterCharPtr;
          If Ch = '"' then
            Repeat
              Ch := NextParameterChar;
              If Ch <> #0 then
                begin
                  IncParameterCharPtr;
                  If Ch <> '"' then StoreParameterChar (Ch);
                end;
            until (Ch = '"') or (Ch = #0)
              else StoreParameterChar (UpCase (Ch));
          If Ch = #0 then Break;
          Ch := NextParameterChar;
          If (Ch <= ' ') or (Ch = '/') then Break;
        end else begin
                   IncParameterCharPtr;
                   Ch := NextParameterChar;
                 end;
    end;
  TempStrPtr^ := #0;
end;
This procedure checks and extracts directory for Turbo Pascal Library (
TPL
file) and Turbo Pascal configuration file (
CFG
).
Procedure Get_TPL_CFG_Directory;
Var TempCmdStr: Pchar;
    N, I: Byte;
begin
  TempCmdStr := CmdPtr;
  ReadParameter;
  If (TempStr [0] in ['/', '-']) and (TempStr [1] = 'T') then StrCopy (TPL_CFG_Directories, TempStr + 2)
    else begin
            CmdPtr := TempCmdStr;
            If Lo (DosVersion) > 3 then
              begin
                StrPCopy (TPL_CFG_Directories, ParamStr (0));
                I := 0;
                For N := 0 to Length (ParamStr (0)) do
                  If TPL_CFG_Directories [N] = '\' then I := N + 1;
                TPL_CFG_Directories [I] := #0;
              end;
          end;
end;
This procedure reads configuration file. This file is processed as initial command line parameters, i.e. all parameters specified in the command line override parameters in the configuration file.
Procedure Read_TPC_CFG;
Var CfgFile: File;
begin
  StrCopy (TempStr, CfgFileName);
  FindFilePath (TempStr, Dir_TPL_CFG or Ext_Original);
  Assign (CfgFile, StrPas (TempStr));
  {$I-}
    Reset (CfgFile, 1);
    If IOResult = 0 then
      begin
        BlockRead (CfgFile, FileBuffer, SizeOf (FileBuffer) - 2);
        Close (CfgFile);
        If IOResult = 0 then;
      end;
  {$I+}
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
Procedure ProcessCompilerParameters;

  Function CommaIsIntegerNumericConstant (Var ChrPtr: PChar): Boolean;
  begin
    If ChrPtr [0] = ',' then Inc (ChrPtr);
    CommaIsIntegerNumericConstant := IsIntegerNumericConstant (ChrPtr);
  end;

  Function IsSegment (Var ChrPtr: PChar): Boolean;
  begin
    IsSegment := False;
    If CommaIsIntegerNumericConstant (ChrPtr) then
      begin
        If IntegerNumericConstantRec.WordH = $FFFF then
          If IntegerNumericConstantRec.WordL > $FFF0 then Exit;
        IntegerNumericConstant := (IntegerNumericConstant + $F) shr 4;
        IsSegment := IntegerNumericConstantRec.WordH = 0;
      end;
  end;

  Procedure AddNameToList (List, Name: PChar);
  begin
    If (List^ = #0) or (Name^ = #0) then StrCopy (List, Name)
      else If StrLen (List) + StrLen (Name) <= 126 then
        begin
          List := StrEnd (List);
          List^ := ';';
          StrCopy (List + 1, Name);
        end;
  end;

  Procedure ProcessCommandLineCompilerDirectives (Var ChrPtr: PChar);

    Procedure ProcessSwitch (Switch: TCompilerSwitches);
    begin
      Inc (ChrPtr);
      With GlobalModuleData^ do
        begin
          Case ChrPtr [0] of
            '+': Include (CompilerSwitches, Switch);
            '-': Exclude (CompilerSwitches, Switch);
            else CmdErr := True;
          end;
        end;
    end;

  begin
    CmdErr := False;
    Repeat
      Inc (ChrPtr);
      Case Upcase (ChrPtr [0]) of
        'A': ProcessSwitch (WordAlignment);
        'B': ProcessSwitch (FullBooleanEval);
        'D': ProcessSwitch (DebugInformation);
        'E': ProcessSwitch (Emulation80x87);
        'F': ProcessSwitch (ForceFarCalls);
        'G': ProcessSwitch (Instructions80286);
        'I': ProcessSwitch (IOErrorChecking);
        'K': ProcessSwitch (CompilerDirective_K);
        'L': ProcessSwitch (LocalDebugSymbols);
        'M': begin
               CmdErr := True;
               Inc (ChrPtr);
               If CommaIsIntegerNumericConstant (ChrPtr) then
                 If IntegerNumericConstantRec.WordH = 0 then
                   begin
                     IntegerNumericConstantRec.WordL := IntegerNumericConstantRec.WordL and $FFFE;
                     If (IntegerNumericConstantRec.WordL >= $0400) and
                        (IntegerNumericConstantRec.WordL <= $FFF0) then
                       begin
                         GlobalModuleData^.Stack := IntegerNumericConstantRec.WordL;
                         If IsSegment (ChrPtr) then
                           If IntegerNumericConstantRec.WordL <= $A000 then
                             begin
                               GlobalModuleData^.HeapMin := IntegerNumericConstantRec.WordL;
                               If IsSegment (ChrPtr) then
                                 If IntegerNumericConstantRec.WordL >= ModuleHeapMin then
                                   If IntegerNumericConstantRec.WordL <= $A000 then
                                     begin
                                       GlobalModuleData^.HeapMax := IntegerNumericConstantRec.WordL;
                                       CmdErr := False;
                                     end;
                             end;

                       end;
                   end;
             end;
        'N': ProcessSwitch (Instructions80x87);
        'O': ProcessSwitch (OverlaysAllowed);
        'P': ProcessSwitch (OpenStringParams);
        'Q': ProcessSwitch (OverflowChecking);
        'R': ProcessSwitch (RangeChecking);
        'S': ProcessSwitch (StackChecking);
        'T': ProcessSwitch (TypedPointers);
        'V': ProcessSwitch (StrictVarStrings);
        'W': ProcessSwitch (CompilerDirective_W);
        'X': ProcessSwitch (ExtendedSyntax);
        'Y': ProcessSwitch (CompilerDirective_Y);
        else CmdErr := True;
      end;
      If CmdErr then Exit;
      Inc (ChrPtr);
    until ChrPtr [0] &lt;&gt; ',';
  end;

begin
  ReadParameter;
  While TempStr [0] <> #0 do
    begin
      CmdErr := False;
      Case TempStr [0] of
        '/', '-': Case TempStr [1] of
                    'B': CompilerModeOptions := [cmoCreateExeFile, cmoCompileToDisk, cmoSearchForUnitSource];
                    'D': AddNameToList (InitialConditionalDefines, TempStr + 2);
                    'E': StrCopy (EXE_TPU_Directory, TempStr + 2);
                    'F': begin
                           CmdErr := True;
                           CompilerModeOptions := [cmoCreateExeFile, cmoFindError];
                           Inc (TempStr, 2);
                           If IsHexConstant (TempStr) then
                             begin
                               FindErrorAddress.Seg := IntegerNumericConstantRec.WordL;
                               If TempStr [0] = ':' then
                                 begin
                                   Inc (TempStr);
                                   If IsHexConstant (TempStr) then
                                     begin
                                       CmdErr := False;
                                       FindErrorAddress.Ofs := IntegerNumericConstantRec.WordL;
                                     end;
                                 end;
                             end;
                         end;
                    'G': begin
                           LinkerOptions := LinkerOptions - [MapFileWithSegments, MapFileWithPublics];
                           Case TempStr [2] of
                             'S': LinkerOptions := LinkerOptions + [MapFileWithSegments];
                             'P': LinkerOptions := LinkerOptions + [MapFileWithPublics];
                             'D': LinkerOptions := LinkerOptions + [MapFileWithSegments, MapFileWithPublics];
                             else CmdErr := True;
                           end;
                         end;
                    'I': AddNameToList (IncludeDirectories, TempStr + 2);
                    'L': LinkerOptions := LinkerOptions + [LinkBufferOnDisk];
                    'M': CompilerModeOptions := [cmoCreateExeFile, cmoCompileToDisk,
                                                 cmoSearchForUnitSource, cmoMakeModifiedUnitsOnly];
                    'O': AddNameToList (ObjectsDirectories, TempStr + 2);
                    'Q': QuietCompile := True;
                    'T': AddNameToList (TPL_CFG_Directories, TempStr + 2);
                    'U': AddNameToList (UnitDirectories, TempStr + 2);
                    'V': LinkerOptions := LinkerOptions + [DebugInformationInExe];
                    '$': begin
                           Inc (TempStr);
                           ProcessCommandLineCompilerDirectives (TempStr);
                         end;
                    '#':
                    else CmdErr := True;
                  end;
        else StrCopy (MainFileName, TempStr);
      end;
      If CmdErr then
        begin
           Writeln ('Invalid command line option: "', TempStrBuff, '"');
           Halt (1);
        end;
      ReadParameter;
    end;
  If MainFileName [0] = #0 then
    begin
      WriteSyntax;
      Halt (0);
    end;
end;
```

### Code Block 2

```pascal
Procedure ReadParameter;
Var Ch: Char;

  Function NextParameterChar: Char;
  begin
    If CfgPtr [0] <> #0 then NextParameterChar := CfgPtr [0]
      else NextParameterChar := CmdPtr [0];
  end;

  Procedure IncParameterCharPtr;
  begin
    If CfgPtr [0] <> #0 then Inc (CfgPtr)
      else Inc (CmdPtr);
 end;

 Procedure StoreParameterChar (C: Char);
 begin
   If TempStrPtr - TempStr >= 127 then Exit;
   TempStrPtr^ := C;
   Inc (TempStrPtr);
 end;

begin
  TempStrPtr := TempStr;
  Ch := NextParameterChar;
  While Ch <> #0 do
    begin
      If Ch > ' ' then
        begin
          IncParameterCharPtr;
          If Ch = '"' then
            Repeat
              Ch := NextParameterChar;
              If Ch <> #0 then
                begin
                  IncParameterCharPtr;
                  If Ch <> '"' then StoreParameterChar (Ch);
                end;
            until (Ch = '"') or (Ch = #0)
              else StoreParameterChar (UpCase (Ch));
          If Ch = #0 then Break;
          Ch := NextParameterChar;
          If (Ch <= ' ') or (Ch = '/') then Break;
        end else begin
                   IncParameterCharPtr;
                   Ch := NextParameterChar;
                 end;
    end;
  TempStrPtr^ := #0;
end;
```

### Code Block 3

```pascal
Procedure Get_TPL_CFG_Directory;
Var TempCmdStr: Pchar;
    N, I: Byte;
begin
  TempCmdStr := CmdPtr;
  ReadParameter;
  If (TempStr [0] in ['/', '-']) and (TempStr [1] = 'T') then StrCopy (TPL_CFG_Directories, TempStr + 2)
    else begin
            CmdPtr := TempCmdStr;
            If Lo (DosVersion) > 3 then
              begin
                StrPCopy (TPL_CFG_Directories, ParamStr (0));
                I := 0;
                For N := 0 to Length (ParamStr (0)) do
                  If TPL_CFG_Directories [N] = '\' then I := N + 1;
                TPL_CFG_Directories [I] := #0;
              end;
          end;
end;
```

### Code Block 4

```pascal
Procedure Read_TPC_CFG;
Var CfgFile: File;
begin
  StrCopy (TempStr, CfgFileName);
  FindFilePath (TempStr, Dir_TPL_CFG or Ext_Original);
  Assign (CfgFile, StrPas (TempStr));
  {$I-}
    Reset (CfgFile, 1);
    If IOResult = 0 then
      begin
        BlockRead (CfgFile, FileBuffer, SizeOf (FileBuffer) - 2);
        Close (CfgFile);
        If IOResult = 0 then;
      end;
  {$I+}
end;
```

### Code Block 5

```pascal
Procedure ProcessCompilerParameters;

  Function CommaIsIntegerNumericConstant (Var ChrPtr: PChar): Boolean;
  begin
    If ChrPtr [0] = ',' then Inc (ChrPtr);
    CommaIsIntegerNumericConstant := IsIntegerNumericConstant (ChrPtr);
  end;

  Function IsSegment (Var ChrPtr: PChar): Boolean;
  begin
    IsSegment := False;
    If CommaIsIntegerNumericConstant (ChrPtr) then
      begin
        If IntegerNumericConstantRec.WordH = $FFFF then
          If IntegerNumericConstantRec.WordL > $FFF0 then Exit;
        IntegerNumericConstant := (IntegerNumericConstant + $F) shr 4;
        IsSegment := IntegerNumericConstantRec.WordH = 0;
      end;
  end;

  Procedure AddNameToList (List, Name: PChar);
  begin
    If (List^ = #0) or (Name^ = #0) then StrCopy (List, Name)
      else If StrLen (List) + StrLen (Name) <= 126 then
        begin
          List := StrEnd (List);
          List^ := ';';
          StrCopy (List + 1, Name);
        end;
  end;

  Procedure ProcessCommandLineCompilerDirectives (Var ChrPtr: PChar);

    Procedure ProcessSwitch (Switch: TCompilerSwitches);
    begin
      Inc (ChrPtr);
      With GlobalModuleData^ do
        begin
          Case ChrPtr [0] of
            '+': Include (CompilerSwitches, Switch);
            '-': Exclude (CompilerSwitches, Switch);
            else CmdErr := True;
          end;
        end;
    end;

  begin
    CmdErr := False;
    Repeat
      Inc (ChrPtr);
      Case Upcase (ChrPtr [0]) of
        'A': ProcessSwitch (WordAlignment);
        'B': ProcessSwitch (FullBooleanEval);
        'D': ProcessSwitch (DebugInformation);
        'E': ProcessSwitch (Emulation80x87);
        'F': ProcessSwitch (ForceFarCalls);
        'G': ProcessSwitch (Instructions80286);
        'I': ProcessSwitch (IOErrorChecking);
        'K': ProcessSwitch (CompilerDirective_K);
        'L': ProcessSwitch (LocalDebugSymbols);
        'M': begin
               CmdErr := True;
               Inc (ChrPtr);
               If CommaIsIntegerNumericConstant (ChrPtr) then
                 If IntegerNumericConstantRec.WordH = 0 then
                   begin
                     IntegerNumericConstantRec.WordL := IntegerNumericConstantRec.WordL and $FFFE;
                     If (IntegerNumericConstantRec.WordL >= $0400) and
                        (IntegerNumericConstantRec.WordL <= $FFF0) then
                       begin
                         GlobalModuleData^.Stack := IntegerNumericConstantRec.WordL;
                         If IsSegment (ChrPtr) then
                           If IntegerNumericConstantRec.WordL <= $A000 then
                             begin
                               GlobalModuleData^.HeapMin := IntegerNumericConstantRec.WordL;
                               If IsSegment (ChrPtr) then
                                 If IntegerNumericConstantRec.WordL >= ModuleHeapMin then
                                   If IntegerNumericConstantRec.WordL <= $A000 then
                                     begin
                                       GlobalModuleData^.HeapMax := IntegerNumericConstantRec.WordL;
                                       CmdErr := False;
                                     end;
                             end;

                       end;
                   end;
             end;
        'N': ProcessSwitch (Instructions80x87);
        'O': ProcessSwitch (OverlaysAllowed);
        'P': ProcessSwitch (OpenStringParams);
        'Q': ProcessSwitch (OverflowChecking);
        'R': ProcessSwitch (RangeChecking);
        'S': ProcessSwitch (StackChecking);
        'T': ProcessSwitch (TypedPointers);
        'V': ProcessSwitch (StrictVarStrings);
        'W': ProcessSwitch (CompilerDirective_W);
        'X': ProcessSwitch (ExtendedSyntax);
        'Y': ProcessSwitch (CompilerDirective_Y);
        else CmdErr := True;
      end;
      If CmdErr then Exit;
      Inc (ChrPtr);
    until ChrPtr [0] &lt;&gt; ',';
  end;

begin
  ReadParameter;
  While TempStr [0] <> #0 do
    begin
      CmdErr := False;
      Case TempStr [0] of
        '/', '-': Case TempStr [1] of
                    'B': CompilerModeOptions := [cmoCreateExeFile, cmoCompileToDisk, cmoSearchForUnitSource];
                    'D': AddNameToList (InitialConditionalDefines, TempStr + 2);
                    'E': StrCopy (EXE_TPU_Directory, TempStr + 2);
                    'F': begin
                           CmdErr := True;
                           CompilerModeOptions := [cmoCreateExeFile, cmoFindError];
                           Inc (TempStr, 2);
                           If IsHexConstant (TempStr) then
                             begin
                               FindErrorAddress.Seg := IntegerNumericConstantRec.WordL;
                               If TempStr [0] = ':' then
                                 begin
                                   Inc (TempStr);
                                   If IsHexConstant (TempStr) then
                                     begin
                                       CmdErr := False;
                                       FindErrorAddress.Ofs := IntegerNumericConstantRec.WordL;
                                     end;
                                 end;
                             end;
                         end;
                    'G': begin
                           LinkerOptions := LinkerOptions - [MapFileWithSegments, MapFileWithPublics];
                           Case TempStr [2] of
                             'S': LinkerOptions := LinkerOptions + [MapFileWithSegments];
                             'P': LinkerOptions := LinkerOptions + [MapFileWithPublics];
                             'D': LinkerOptions := LinkerOptions + [MapFileWithSegments, MapFileWithPublics];
                             else CmdErr := True;
                           end;
                         end;
                    'I': AddNameToList (IncludeDirectories, TempStr + 2);
                    'L': LinkerOptions := LinkerOptions + [LinkBufferOnDisk];
                    'M': CompilerModeOptions := [cmoCreateExeFile, cmoCompileToDisk,
                                                 cmoSearchForUnitSource, cmoMakeModifiedUnitsOnly];
                    'O': AddNameToList (ObjectsDirectories, TempStr + 2);
                    'Q': QuietCompile := True;
                    'T': AddNameToList (TPL_CFG_Directories, TempStr + 2);
                    'U': AddNameToList (UnitDirectories, TempStr + 2);
                    'V': LinkerOptions := LinkerOptions + [DebugInformationInExe];
                    '$': begin
                           Inc (TempStr);
                           ProcessCommandLineCompilerDirectives (TempStr);
                         end;
                    '#':
                    else CmdErr := True;
                  end;
        else StrCopy (MainFileName, TempStr);
      end;
      If CmdErr then
        begin
           Writeln ('Invalid command line option: "', TempStrBuff, '"');
           Halt (1);
        end;
      ReadParameter;
    end;
  If MainFileName [0] = #0 then
    begin
      WriteSyntax;
      Halt (0);
    end;
end;
```

### Code Block 6

```pascal
Procedure ReadParameter;
Var Ch: Char;

  Function NextParameterChar: Char;
  begin
    If CfgPtr [0] <> #0 then NextParameterChar := CfgPtr [0]
      else NextParameterChar := CmdPtr [0];
  end;

  Procedure IncParameterCharPtr;
  begin
    If CfgPtr [0] <> #0 then Inc (CfgPtr)
      else Inc (CmdPtr);
 end;

 Procedure StoreParameterChar (C: Char);
 begin
   If TempStrPtr - TempStr >= 127 then Exit;
   TempStrPtr^ := C;
   Inc (TempStrPtr);
 end;

begin
  TempStrPtr := TempStr;
  Ch := NextParameterChar;
  While Ch <> #0 do
    begin
      If Ch > ' ' then
        begin
          IncParameterCharPtr;
          If Ch = '"' then
            Repeat
              Ch := NextParameterChar;
              If Ch <> #0 then
                begin
                  IncParameterCharPtr;
                  If Ch <> '"' then StoreParameterChar (Ch);
                end;
            until (Ch = '"') or (Ch = #0)
              else StoreParameterChar (UpCase (Ch));
          If Ch = #0 then Break;
          Ch := NextParameterChar;
          If (Ch <= ' ') or (Ch = '/') then Break;
        end else begin
                   IncParameterCharPtr;
                   Ch := NextParameterChar;
                 end;
    end;
  TempStrPtr^ := #0;
end;
```

### Code Block 7

```pascal
Procedure Get_TPL_CFG_Directory;
Var TempCmdStr: Pchar;
    N, I: Byte;
begin
  TempCmdStr := CmdPtr;
  ReadParameter;
  If (TempStr [0] in ['/', '-']) and (TempStr [1] = 'T') then StrCopy (TPL_CFG_Directories, TempStr + 2)
    else begin
            CmdPtr := TempCmdStr;
            If Lo (DosVersion) > 3 then
              begin
                StrPCopy (TPL_CFG_Directories, ParamStr (0));
                I := 0;
                For N := 0 to Length (ParamStr (0)) do
                  If TPL_CFG_Directories [N] = '\' then I := N + 1;
                TPL_CFG_Directories [I] := #0;
              end;
          end;
end;
```

### Code Block 8

```pascal
Procedure Read_TPC_CFG;
Var CfgFile: File;
begin
  StrCopy (TempStr, CfgFileName);
  FindFilePath (TempStr, Dir_TPL_CFG or Ext_Original);
  Assign (CfgFile, StrPas (TempStr));
  {$I-}
    Reset (CfgFile, 1);
    If IOResult = 0 then
      begin
        BlockRead (CfgFile, FileBuffer, SizeOf (FileBuffer) - 2);
        Close (CfgFile);
        If IOResult = 0 then;
      end;
  {$I+}
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
- https://turbopascal.org/processing-command-line-parameters
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

