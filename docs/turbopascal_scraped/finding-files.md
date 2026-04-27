# Finding Files - Turbo Pascal Compiler

**URL:** https://turbopascal.org/finding-files

---

## Table of Contents

- Finding Files

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Finding Files
urbo Pascal needs to access files that may be located in different directories – located where the compiler is run or specified either in the configuration file or at the command line.
FindFilePath
searches for specified file name taking into account options that specify what type of file the compiler is looking for. Function returns the size of path string (
C:\PROJECTS\....\
).
Const  
  Dir_None     = $0000;
  Dir_TPL_CFG  = $0100;
  Dir_EXE_TPU  = $0200;
  Dir_Include  = $0300;
  Dir_Unit     = $0400;
  Dir_Object   = $0500;
  Dir_Forced   = $8000;

  Ext_Original = $00;
  Ext_PAS      = $01;
  Ext_EXE      = $02;
  Ext_TPU      = $03;
  Ext_OBJ      = $04;
  Ext_MAP      = $05;
  Ext_OVR      = $06;
  Ext_Forced   = $80;

Function FindFilePath (FileName: PChar; Options: Word): Byte;
Var N, I: Byte;
    Name, Path: PChar;
    DirInfo: SearchRec;
    TempPathBuff: TLine;
    TempPath, NameStart: PChar;
    LastChar: Char;
begin
  FindFilePath := 0;
  If Lo (Options) <> Ext_Original then
    begin
      Name := FileName;
      N := 0;
      I := 0;
      While Name [N] <> #0 do
        begin
          If Name [N] = '\' then I := 0;
          If Name [N] = '.' then I := N;
          Inc (N);
        end;
      If (I = 0) or (Lo (Options) and Ext_Forced <> 0) then
        begin
          If I = 0 then I := N;
          Name [I] := '.';
          Inc (I);
          Case Lo (Options) and $0F of
            Ext_PAS: StrPCopy (Name + I, 'PAS');
            Ext_EXE: StrPCopy (Name + I, 'EXE');
            Ext_TPU: StrPCopy (Name + I, 'TPU');
            Ext_OBJ: StrPCopy (Name + I, 'OBJ');
            Ext_MAP: StrPCopy (Name + I, 'MAP');
            Ext_OVR: StrPCopy (Name + I, 'OVR');
          end;
        end;
    end;
  Name := FileName;
  If (Hi (Options) and $0F) <> Hi (Dir_None) then
    begin
      Case (Hi (Options) and $0F) of
        Hi (Dir_TPL_CFG): Path := TPL_CFG_Directories;
        Hi (Dir_EXE_TPU): Path := EXE_TPU_Directory;
        Hi (Dir_Include): Path := IncludeDirectories;
        Hi (Dir_Unit): Path := UnitDirectoriesBuff;
        Hi (Dir_Object): Path := ObjectsDirectories;
      end;
      While Path [0] <= ' ' do
        begin
          If Path [0] = #0 then Exit;
          Inc (Path);
        end;
      If (Hi (Options) and Hi (Dir_Forced)) = 0 then
        begin
          FindFirst (StrPas (Name), AnyFile, DirInfo);
          If DosError = 0 then Exit;
          Repeat
            TempPathBuff [0] := #0;
            TempPath := TempPathBuff;
            While Path [0] <= ' ' do
              begin
                If Path [0] = #0 then Exit;
                Inc (Path);
              end;
            While Path [0] <> #0 do
              begin
                If Path [0] = ';' then
                  begin
                    Inc (Path);
                    Break;
                  end;
                TempPath [0] := Path [0];
                LastChar := Path [0];
                Inc (Path);
                Inc (TempPath);
              end;
            If not (LastChar in [':', '\']) then
              begin
                TempPath [0] := '\';
                Inc (TempPath);
              end;
            FindFilePath := Ofs (TempPath^) - Ofs (TempPathBuff);
            StrCopy (TempPath, FileName);
            FindFirst (StrPas (TempPathBuff), AnyFile, DirInfo);
            If DosError = 0 then
              begin
                StrCopy (FileName, TempPathBuff);
                Exit;
              end;
          until False;
        end else
          begin
            TempPath := TempPathBuff;
            While Path [0] > ' ' do
              begin
                TempPath [0] := Path [0];
                LastChar := Path [0];
                Inc (Path);
                Inc (TempPath);
              end;
            If not (LastChar in [':', '\']) then
              begin
                TempPath [0] := '\';
                Inc (TempPath);
              end;
            NameStart := TempPath;
            FindFilePath := Ofs (NameStart^) - Ofs (TempPathBuff);
            While Name [0] > #0 do
              begin
                TempPath [0] := Name [0];
                Inc (Name);
                If TempPath [0] in [':', '\'] then
                  TempPath := NameStart else Inc (TempPath);
              end;
            TempPath [0] := #0;
            StrCopy (FileName, TempPathBuff);
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
Const  
  Dir_None     = $0000;
  Dir_TPL_CFG  = $0100;
  Dir_EXE_TPU  = $0200;
  Dir_Include  = $0300;
  Dir_Unit     = $0400;
  Dir_Object   = $0500;
  Dir_Forced   = $8000;

  Ext_Original = $00;
  Ext_PAS      = $01;
  Ext_EXE      = $02;
  Ext_TPU      = $03;
  Ext_OBJ      = $04;
  Ext_MAP      = $05;
  Ext_OVR      = $06;
  Ext_Forced   = $80;

Function FindFilePath (FileName: PChar; Options: Word): Byte;
Var N, I: Byte;
    Name, Path: PChar;
    DirInfo: SearchRec;
    TempPathBuff: TLine;
    TempPath, NameStart: PChar;
    LastChar: Char;
begin
  FindFilePath := 0;
  If Lo (Options) <> Ext_Original then
    begin
      Name := FileName;
      N := 0;
      I := 0;
      While Name [N] <> #0 do
        begin
          If Name [N] = '\' then I := 0;
          If Name [N] = '.' then I := N;
          Inc (N);
        end;
      If (I = 0) or (Lo (Options) and Ext_Forced <> 0) then
        begin
          If I = 0 then I := N;
          Name [I] := '.';
          Inc (I);
          Case Lo (Options) and $0F of
            Ext_PAS: StrPCopy (Name + I, 'PAS');
            Ext_EXE: StrPCopy (Name + I, 'EXE');
            Ext_TPU: StrPCopy (Name + I, 'TPU');
            Ext_OBJ: StrPCopy (Name + I, 'OBJ');
            Ext_MAP: StrPCopy (Name + I, 'MAP');
            Ext_OVR: StrPCopy (Name + I, 'OVR');
          end;
        end;
    end;
  Name := FileName;
  If (Hi (Options) and $0F) <> Hi (Dir_None) then
    begin
      Case (Hi (Options) and $0F) of
        Hi (Dir_TPL_CFG): Path := TPL_CFG_Directories;
        Hi (Dir_EXE_TPU): Path := EXE_TPU_Directory;
        Hi (Dir_Include): Path := IncludeDirectories;
        Hi (Dir_Unit): Path := UnitDirectoriesBuff;
        Hi (Dir_Object): Path := ObjectsDirectories;
      end;
      While Path [0] <= ' ' do
        begin
          If Path [0] = #0 then Exit;
          Inc (Path);
        end;
      If (Hi (Options) and Hi (Dir_Forced)) = 0 then
        begin
          FindFirst (StrPas (Name), AnyFile, DirInfo);
          If DosError = 0 then Exit;
          Repeat
            TempPathBuff [0] := #0;
            TempPath := TempPathBuff;
            While Path [0] <= ' ' do
              begin
                If Path [0] = #0 then Exit;
                Inc (Path);
              end;
            While Path [0] <> #0 do
              begin
                If Path [0] = ';' then
                  begin
                    Inc (Path);
                    Break;
                  end;
                TempPath [0] := Path [0];
                LastChar := Path [0];
                Inc (Path);
                Inc (TempPath);
              end;
            If not (LastChar in [':', '\']) then
              begin
                TempPath [0] := '\';
                Inc (TempPath);
              end;
            FindFilePath := Ofs (TempPath^) - Ofs (TempPathBuff);
            StrCopy (TempPath, FileName);
            FindFirst (StrPas (TempPathBuff), AnyFile, DirInfo);
            If DosError = 0 then
              begin
                StrCopy (FileName, TempPathBuff);
                Exit;
              end;
          until False;
        end else
          begin
            TempPath := TempPathBuff;
            While Path [0] > ' ' do
              begin
                TempPath [0] := Path [0];
                LastChar := Path [0];
                Inc (Path);
                Inc (TempPath);
              end;
            If not (LastChar in [':', '\']) then
              begin
                TempPath [0] := '\';
                Inc (TempPath);
              end;
            NameStart := TempPath;
            FindFilePath := Ofs (NameStart^) - Ofs (TempPathBuff);
            While Name [0] > #0 do
              begin
                TempPath [0] := Name [0];
                Inc (Name);
                If TempPath [0] in [':', '\'] then
                  TempPath := NameStart else Inc (TempPath);
              end;
            TempPath [0] := #0;
            StrCopy (FileName, TempPathBuff);
          end;
    end;
end;
```

### Code Block 2

```pascal
Const  
  Dir_None     = $0000;
  Dir_TPL_CFG  = $0100;
  Dir_EXE_TPU  = $0200;
  Dir_Include  = $0300;
  Dir_Unit     = $0400;
  Dir_Object   = $0500;
  Dir_Forced   = $8000;

  Ext_Original = $00;
  Ext_PAS      = $01;
  Ext_EXE      = $02;
  Ext_TPU      = $03;
  Ext_OBJ      = $04;
  Ext_MAP      = $05;
  Ext_OVR      = $06;
  Ext_Forced   = $80;

Function FindFilePath (FileName: PChar; Options: Word): Byte;
Var N, I: Byte;
    Name, Path: PChar;
    DirInfo: SearchRec;
    TempPathBuff: TLine;
    TempPath, NameStart: PChar;
    LastChar: Char;
begin
  FindFilePath := 0;
  If Lo (Options) <> Ext_Original then
    begin
      Name := FileName;
      N := 0;
      I := 0;
      While Name [N] <> #0 do
        begin
          If Name [N] = '\' then I := 0;
          If Name [N] = '.' then I := N;
          Inc (N);
        end;
      If (I = 0) or (Lo (Options) and Ext_Forced <> 0) then
        begin
          If I = 0 then I := N;
          Name [I] := '.';
          Inc (I);
          Case Lo (Options) and $0F of
            Ext_PAS: StrPCopy (Name + I, 'PAS');
            Ext_EXE: StrPCopy (Name + I, 'EXE');
            Ext_TPU: StrPCopy (Name + I, 'TPU');
            Ext_OBJ: StrPCopy (Name + I, 'OBJ');
            Ext_MAP: StrPCopy (Name + I, 'MAP');
            Ext_OVR: StrPCopy (Name + I, 'OVR');
          end;
        end;
    end;
  Name := FileName;
  If (Hi (Options) and $0F) <> Hi (Dir_None) then
    begin
      Case (Hi (Options) and $0F) of
        Hi (Dir_TPL_CFG): Path := TPL_CFG_Directories;
        Hi (Dir_EXE_TPU): Path := EXE_TPU_Directory;
        Hi (Dir_Include): Path := IncludeDirectories;
        Hi (Dir_Unit): Path := UnitDirectoriesBuff;
        Hi (Dir_Object): Path := ObjectsDirectories;
      end;
      While Path [0] <= ' ' do
        begin
          If Path [0] = #0 then Exit;
          Inc (Path);
        end;
      If (Hi (Options) and Hi (Dir_Forced)) = 0 then
        begin
          FindFirst (StrPas (Name), AnyFile, DirInfo);
          If DosError = 0 then Exit;
          Repeat
            TempPathBuff [0] := #0;
            TempPath := TempPathBuff;
            While Path [0] <= ' ' do
              begin
                If Path [0] = #0 then Exit;
                Inc (Path);
              end;
            While Path [0] <> #0 do
              begin
                If Path [0] = ';' then
                  begin
                    Inc (Path);
                    Break;
                  end;
                TempPath [0] := Path [0];
                LastChar := Path [0];
                Inc (Path);
                Inc (TempPath);
              end;
            If not (LastChar in [':', '\']) then
              begin
                TempPath [0] := '\';
                Inc (TempPath);
              end;
            FindFilePath := Ofs (TempPath^) - Ofs (TempPathBuff);
            StrCopy (TempPath, FileName);
            FindFirst (StrPas (TempPathBuff), AnyFile, DirInfo);
            If DosError = 0 then
              begin
                StrCopy (FileName, TempPathBuff);
                Exit;
              end;
          until False;
        end else
          begin
            TempPath := TempPathBuff;
            While Path [0] > ' ' do
              begin
                TempPath [0] := Path [0];
                LastChar := Path [0];
                Inc (Path);
                Inc (TempPath);
              end;
            If not (LastChar in [':', '\']) then
              begin
                TempPath [0] := '\';
                Inc (TempPath);
              end;
            NameStart := TempPath;
            FindFilePath := Ofs (NameStart^) - Ofs (TempPathBuff);
            While Name [0] > #0 do
              begin
                TempPath [0] := Name [0];
                Inc (Name);
                If TempPath [0] in [':', '\'] then
                  TempPath := NameStart else Inc (TempPath);
              end;
            TempPath [0] := #0;
            StrCopy (FileName, TempPathBuff);
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
- https://turbopascal.org/expressions
- https://turbopascal.org/finding-files
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

