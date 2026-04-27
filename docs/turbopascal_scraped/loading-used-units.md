# Loading Used Units - Turbo Pascal Compiler

**URL:** https://turbopascal.org/loading-used-units

---

## Table of Contents

- Loading Used Units

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Loading Used Units
Each used unit is checked if it is already loaded. If it is not it needs to be found and checked if recompilation is needed
Function LoadUsedUnits: Boolean;
Var SavedUnitPathAndName: PChar;
    LoadedUnitPublicIdentifiersChecksum: Word;
    UsedUnitPublicIdentifiersChecksum: Word;
    LoadedUnitSegment: Word;
    CurrentUsedUnitIdentifier: Word;
    LoadedUnit, BaseUnit: PUnitHeader;
    UnitIdentifierData: PUnitIdentifierData;
    UnitNameLen: Byte;

  Function FindUsedUnit: PUnitHeader;
  Var FoundUnit, UnitToCheck: PUnitHeader;
      PreviousUnitSegment: Word;

    Function CurrentUsedUnitName: PString;
    begin
      CurrentUsedUnitName :=
       @PIdentifier (Ptr (BaseUnitSegment, PUnitHeader (Ptr (BaseUnitSegment, 0))^.CurrentUsedUnitIdentifierOffset))^.Name.Str;
    end;

    Procedure CopyUnitPathAndNameToFileStrucBuffer;
    begin
      UnitPathAndName := StartOfSourceFileStructures;
      If (Ofs (UnitPathAndName^) + 80) > Ofs (CurrentSourceFile^) then Error (TooManyFiles);
      StartOfSourceFileStructures := StrECopy (UnitPathAndName, @Identifier) + 1;
    end;

    Function LoadUsedUnit: PUnitHeader;
    Var DirInfo: SearchRec;

      Function RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded: PUnitHeader;
      Var UnitPtr: PUnitHeader;
      begin
        RemoveSymbolTables_Code_To_TypedConstantsReferences (Ofs (UnitPathAndName^));
        If IsLastUnitAlreadyLoaded (UnitPtr) then RemoveLastLoadedUnit
          else begin
                UnitPtr := Ptr (LastLoadedUsedUnit, 0);
                PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.CodeSegment := 0;
                PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.OverlayedUnitCodeSize := 0;
              end;
        RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded := UnitPtr;
        CheckStack;
      end;

      Function FindCurrentUsedUnitFile (Options: Word): Byte;
      begin
        StrPCopy (@Identifier, CurrentUsedUnitName^);
        StrUpper (@Identifier);
        FindCurrentUsedUnitFile := FindFilePath (@Identifier, Options);
      end;

      Function IncludeOrObjectFilesChanged: Boolean;
      Var FilesBlockPtr: PUsedFilesBlockRecord;
          FilesBlockPtrRec: PtrRec absolute FilesBlockPtr;
          UnitHeader: PUnitHeader;
      begin
        IncludeOrObjectFilesChanged := True;
        UnitHeader := Ptr (LastLoadedUsedUnit, 0);
        FilesBlockPtr := Ptr (LastLoadedUsedUnit, UnitHeader^.BlockOffset [stUsedFiles]);
        Inc (FilesBlockPtrRec.Ofs, FilesBlockPtr^.Name.Len + 8);
        While FilesBlockPtrRec.Ofs <> UnitHeader^.BlockOffset [Succ (stUsedFiles)] do
          begin
            FindUsedFilePath (FilesBlockPtr, @Identifier);
            FindFirst (StrPas (@Identifier), AnyFile, DirInfo);
            If UnitDateTime < (DirInfo.Time + 1) then Exit;
            Inc (FilesBlockPtrRec.Ofs, FilesBlockPtr^.Name.Len + 8);
          end;
        IncludeOrObjectFilesChanged := False;
      end;

      Function CheckUnitName: PString;
      Var UsedUnitName: PString;
      begin
        UsedUnitName := CurrentUsedUnitName;
        If not IdentifiersEqual (@PIdentifier (Ptr (LastLoadedUsedUnit,
                                PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.UnitNameIdentifierOffset))^.Name, UsedUnitName) then
          begin
            AdditionalErrorStr := @Identifier;
            StrPCopy (@Identifier, UsedUnitName^);
            Error (UnitNameMismatch);
          end;
        CheckUnitName := UsedUnitName;
      end;

    begin
      If PushedUnitLevel <> UsedUnitLevel then
        begin
          PushUnit;
          CompilerModeOptions := CompilerModeOptions + [cmoCreateExeFile, cmoCompileToDisk, cmoCompileOnlyInterfaceUsedUnits];
          Saved_PushedUnitLevel := PushedUnitLevel;
          PushedUnitLevel := UsedUnitLevel;
        end;
      ClearCurrentModuleVarsAndCreateUnit;
      If cmoSearchForUnitSource in CompilerModeOptions then
        begin                                                               { Check for UNIT source }
          FindCurrentUsedUnitFile (Dir_Unit or Ext_PAS);
          FindFirst (StrPas (@Identifier), AnyFile, DirInfo);
          UnitSourceDateTime := DirInfo.Time + 1;
          If DOSError = 0 then                                              { UNIT source found }
            begin
              FindFilePath (@Identifier, Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_TPU);
              CopyUnitPathAndNameToFileStrucBuffer;
              If cmoMakeModifiedUnitsOnly in CompilerModeOptions then
                Repeat
                  FindFirst (StrPas (@Identifier), AnyFile, DirInfo);
                  If DOSError <> 0 then Break;                              { no TPU file }
                  UnitDateTime := DirInfo.Time + 1;
                  If UnitDateTime < UnitSourceDateTime then Break;
                  If ReadUnit (UnitPathAndName) then                        { TPU not older than PAS }
                    Repeat
                      CheckUnitName;
                      If IncludeOrObjectFilesChanged then Break;
                      If not LoadUsedUnits then Break;
                      LoadUsedUnit := RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded;
                      Exit;
                    until True;
                  ClearUnitHeader;
                until True;
                CurrentSourceFilePathLength := FindCurrentUsedUnitFile (Dir_Unit or Ext_PAS);
                CurrentFileName := @Identifier;
                CompileModule;
                CheckUnitName;
                LoadUsedUnit := RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded;
                Exit;
            end;
        end;
      FindCurrentUsedUnitFile (Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_TPU);
      FindFirst (StrPas (@Identifier), AnyFile, DirInfo);
      If DOSError <> 0 then FindCurrentUsedUnitFile (Dir_Unit or Ext_Forced or Ext_TPU);
      CopyUnitPathAndNameToFileStrucBuffer;
      If not ReadUnit (UnitPathAndName) then Error (UnitVersionMismatch);
      CheckUnitName;
      If not LoadUsedUnits then Error (UnitVersionMismatch);
      LoadUsedUnit := RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded;
    end;

    Function IsCurrentUsedUnitUsedInInterface (UnitPtr: PUnitHeader): Boolean;
    Var UsedUnitId: PIdentifier;
        UsedUnitIdData: PUnitIdentifierData;
    begin
      UsedUnitId := Ptr (Seg (UnitPtr^), UnitPtr^.CurrentUsedUnitIdentifierOffset);
      UsedUnitIdData := Ptr (Seg (UnitPtr^), Ofs (UsedUnitId^) + UsedUnitId^.Name.Len + 4);
      IsCurrentUsedUnitUsedInInterface := UsedInInterface in UsedUnitIdData^.UnitIdentifierFlags
    end;

    Procedure UnitError (Err: TCompilerError);
    Var UnitName: PString;
    begin
      UnitName := Ptr (BaseUnitSegment, PUnitHeader (Ptr (BaseUnitSegment, 0))^.CurrentUsedUnitIdentifierOffset + 3);
      AdditionalErrorStr := @Identifier;
      StrPCopy (@Identifier, UnitName^);
      Error (Err);
    end;

  begin
    CurrentIdentifier := CurrentUsedUnitName^;
    If FindUnitWithName (LastLoadedUsedUnit, 4, CurrentIdentifier, FoundUnit) then
      begin
        CheckStack;
        FindUsedUnit := FoundUnit;
        Exit;
      end;
    If not FindUnitWithName (BaseUnitSegment, 4, CurrentIdentifier, FoundUnit) then
      begin
        If not FindUnitWithName (LastLibraryUnitSegment, 6, CurrentIdentifier, FoundUnit) then
          begin
            FindUsedUnit := LoadUsedUnit;
            Exit;
          end;
        FoundUnit^.PreviousUnitSegment := LastLoadedUsedUnit;
        LastLoadedUsedUnit := Seg (FoundUnit^);
        If not LoadUsedUnits then UnitError (UnitVersionMismatch);
        PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.CodeSegment := 0;
        PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.OverlayedUnitCodeSize := 0;
        FindUsedUnit := FoundUnit;
        CheckStack;
        Exit;
      end;
    If not IsCurrentUsedUnitUsedInInterface (FoundUnit) then
      begin
        CheckStack;
        FindUsedUnit := FoundUnit;
        Exit;
      end;
    UnitToCheck := Ptr (BaseUnitSegment, 0);
    Repeat
      If not IsCurrentUsedUnitUsedInInterface (UnitToCheck) then
        begin
          FindUsedUnit := LoadUsedUnit;
          Exit;
        end;
      PreviousUnitSegment := UnitToCheck^.PreviousUnitSegment;
      UnitToCheck := Ptr (PreviousUnitSegment, 0);
    until PreviousUnitSegment = Seg (FoundUnit^);
    UnitError (CircularUnitReference);
  end;

{$IFDEF DebugUsedUnits}
  Var LevelCounter: Word;
{$ENDIF}

begin
  SavedUnitPathAndName := UnitPathAndName;
  Inc (UsedUnitLevel);
  BaseUnit := Ptr (LastLoadedUsedUnit, 0);
  LastLoadedUsedUnit := BaseUnit^.PreviousUnitSegment;
  BaseUnit^.PreviousUnitSegment := BaseUnitSegment;
  BaseUnitSegment := Seg (BaseUnit^);
  UnitNameLen := PIdentifier (Ptr (Seg (BaseUnit^), BaseUnit^.UnitNameIdentifierOffset))^.Name.Len;
  UnitIdentifierData := Ptr (Seg (BaseUnit^), BaseUnit^.UnitNameIdentifierOffset + UnitNameLen + 4); { Base unit data }
  LoadedUnitSegment := Seg (BaseUnit^);

{$IFDEF DebugUsedUnits}
  If UsedUnitLevel = 1 then Writeln;
  For LevelCounter := 2 to UsedUnitLevel do Write ('  ');
  If UsedUnitLevel = 1 then Write ('#') else Write (' ');
  Writeln ('LOAD USED UNITS BY: ',
            PIdentifier (Ptr (Seg (BaseUnit^), BaseUnit^.UnitNameIdentifierOffset))^.Name.Str);
{$ENDIF}

  Repeat
    UnitIdentifierData^.UnitSegment := LoadedUnitSegment;
    CurrentUsedUnitIdentifier := UnitIdentifierData^.NextUnitIdentifier;          { Next used unit }
    BaseUnit^.CurrentUsedUnitIdentifierOffset := CurrentUsedUnitIdentifier;
    If CurrentUsedUnitIdentifier = 0 then Break;                                  { No more used units }

{$IFDEF DebugUsedUnits}
    For LevelCounter := 1 to UsedUnitLevel do Write ('  ');
    Writeln (' USES: ', PIdentifier (Ptr (Seg (BaseUnit^), CurrentUsedUnitIdentifier))^.Name.Str);
{$ENDIF}

    LoadedUnit := FindUsedUnit;
    UnitNameLen := PIdentifier (Ptr (Seg (LoadedUnit^), LoadedUnit^.UnitNameIdentifierOffset))^.Name.Len;
    UnitIdentifierData := Ptr (Seg (LoadedUnit^), LoadedUnit^.UnitNameIdentifierOffset + UnitNameLen + 4);

{$IFDEF DebugUsedUnits}
    For LevelCounter := 1 to UsedUnitLevel do Write ('  ');
    Writeln (' LOADED: ', PIdentifier (Ptr (Seg (LoadedUnit^), LoadedUnit^.UnitNameIdentifierOffset))^.Name.Str);
{$ENDIF}

    LoadedUnitPublicIdentifiersChecksum := UnitIdentifierData^.PublicIdentifiersChecksum;
    LoadedUnitSegment := Seg (LoadedUnit^);
    BaseUnit := Ptr (BaseUnitSegment, 0);                                   { BaseUnitSegment might change during unit PUSH}
    UnitNameLen := PIdentifier (Ptr (Seg (BaseUnit^), CurrentUsedUnitIdentifier))^.Name.Len;
    UnitIdentifierData := Ptr (Seg (BaseUnit^), CurrentUsedUnitIdentifier + UnitNameLen + 4);
    UsedUnitPublicIdentifiersChecksum := UnitIdentifierData^.PublicIdentifiersChecksum;
    If UsedUnitPublicIdentifiersChecksum <> 0 then
      begin
        If UsedUnitPublicIdentifiersChecksum <> LoadedUnitPublicIdentifiersChecksum then
          begin
            Inc (CurrentUsedUnitIdentifier, 3);    { name of unit which has different PublicIdentifiersChecksum }
            Break;
          end;
      end else UnitIdentifierData^.PublicIdentifiersChecksum := LoadedUnitPublicIdentifiersChecksum;
  until False;                                                                    { Load next used unit }

  BaseUnitSegment := BaseUnit^.PreviousUnitSegment;
  BaseUnit^.PreviousUnitSegment := LastLoadedUsedUnit;
  LastLoadedUsedUnit := Seg (BaseUnit^);
  Dec (UsedUnitLevel);
  If UsedUnitLevel = Saved_PushedUnitLevel then PopUnit;
  UnitPathAndName := SavedUnitPathAndName;
  LoadUsedUnits := CurrentUsedUnitIdentifier = 0;                                    { All used units loaded }

{$IFDEF DebugUsedUnits}
  For LevelCounter := 1 to UsedUnitLevel do Write ('  ');
  If UsedUnitLevel = 0 then Write ('#') else Write (' ');
  Writeln ('LOAD USED UNITS END ',
             PIdentifier (Ptr (Seg (BaseUnit^), BaseUnit^.UnitNameIdentifierOffset))^.Name.Str);
  If UsedUnitLevel = 0 then Writeln;
{$ENDIF}

end;
PushUnit
and
PopUnit
save and restore current module when compiler is needed to compile used unit.
Procedure PushUnit;
Var ReferencedUnitRecord: PReferencedModulesBlockRecord;
    Segment: Word;
    Block: TSymbolTable;

  Procedure AdjustUnitSegments (Var Segment: Word);
  Var UnitPtr: PUnitHeader;
      UnitIdentifier: PIdentifier;
      UnitIdentifierData: PUnitIdentifierData;
  begin
    If Segment = HeapPtrRec.Seg then Segment := HeapEndRec.Seg;
    UnitPtr := Ptr (Segment, 0);
    While Seg (UnitPtr^) <> 0 do
      begin
        UnitIdentifier := Ptr (Seg (UnitPtr^), UnitPtr^.UnitNameIdentifierOffset);
        Repeat
          UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
          If UnitIdentifierData^.UnitSegment = HeapPtrRec.Seg then UnitIdentifierData^.UnitSegment := HeapEndRec.Seg;
          UnitIdentifier := Ptr (Seg (UnitPtr^), UnitIdentifierData^.NextUnitIdentifier);
        until Ofs (UnitIdentifier^) = 0;
        If UnitPtr^.PreviousUnitSegment = HeapPtrRec.Seg then UnitPtr^.PreviousUnitSegment := HeapEndRec.Seg;
        UnitPtr := Ptr (UnitPtr^.PreviousUnitSegment, 0);
      end;
  end;

begin
  HeapPtrRec.Seg := SymbolTable [stMain].Segment;                                     { PUSH Memory Blocks }
  Segment := HeapEndRec.Seg;
  For Block := stIntermediateCode downto stMain do
    begin
      SymbolTable [Block].Size := (SymbolTable [Block].UsedSize + $000F) and $FFF0;
      Dec (Segment, SymbolTable [Block].Size shr 4);
      Move (Ptr (SymbolTable [Block].Segment, 0)^, Ptr (Segment, 0)^, SymbolTable [Block].UsedSize);
      SymbolTable [Block].Segment := Segment;
    end;
  HeapEndRec.Seg := Segment;

  AdjustUnitSegments (LastLoadedUsedUnit);
  AdjustUnitSegments (BaseUnitSegment);

  ReferencedUnitRecord := Ptr (SymbolTable [stReferencedModules].Segment, 0);
  While ReferencedUnitRecord <> SymbolTable [stReferencedModules].Ptr do
    begin
      If ReferencedUnitRecord^.ModuleSegment = HeapPtrRec.Seg then ReferencedUnitRecord^.ModuleSegment := HeapEndRec.Seg;
      Inc (PChar (ReferencedUnitRecord), ReferencedUnitRecord^.UnitName.Len + 5);
    end;

  Dec (HeapEndRec.Seg, (Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions) +                   { PUSH Variables }
                        Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines)+ $000F) shr 4);
  Move (CompilerModeOptions, HeapEnd^, Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions));
  Move (ConditionalDefines, Ptr (HeapEndRec.Seg, Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions))^,
                             Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines));        { PUSH Conditional Defines }
end;

Procedure PopUnit;
Var TempHeapPtr, TempHeapEnd: Word;
    ReferencedUnitRecord: PReferencedModulesBlockRecord;
    Segment: Word;
    Block: TSymbolTable;

  Procedure AdjustUnitSegments (Var Segment: Word);
  Var UnitPtr: PUnitHeader;
      UnitIdentifier: PIdentifier;
      UnitIdentifierData: PUnitIdentifierData;
  begin
    If Segment = TempHeapEnd then Segment := TempHeapPtr;
    UnitPtr := Ptr (Segment, 0);
    While Seg (UnitPtr^) <> 0 do
      begin
        UnitIdentifier := Ptr (Seg (UnitPtr^), UnitPtr^.UnitNameIdentifierOffset);
        Repeat
          UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
          If UnitIdentifierData^.UnitSegment = TempHeapEnd then UnitIdentifierData^.UnitSegment := TempHeapPtr;
          UnitIdentifier := Ptr (Seg (UnitPtr^), UnitIdentifierData^.NextUnitIdentifier);
        until Ofs (UnitIdentifier^) = 0;
        If UnitPtr^.PreviousUnitSegment = TempHeapEnd then UnitPtr^.PreviousUnitSegment := TempHeapPtr;
        UnitPtr := Ptr (UnitPtr^.PreviousUnitSegment, 0);
      end;
  end;

begin
  Move (HeapEnd^, CompilerModeOptions, Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions));    { POP Variables }
  Move (Ptr (HeapEndRec.Seg, Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions))^, ConditionalDefines,
          Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines));                           { POP Conditional Defines }
  TempHeapEnd := HeapEndRec.Seg + (Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions) +
                   Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines) + $000F) shr 4;

  TempHeapPtr := HeapPtrRec.Seg;

  HeapEndRec.Seg := SymbolTable [stIntermediateCode].Segment + SymbolTable [stIntermediateCode].Size shr 4;
  Segment := HeapPtrRec.Seg;
  For Block := stMain to stIntermediateCode do
    begin
      Move (Ptr (SymbolTable [Block].Segment, 0)^, Ptr (Segment, 0)^, SymbolTable [Block].UsedSize);
      SymbolTable [Block].Segment := Segment;
      Inc (Segment, (SymbolTable [Block].UsedSize + $000F) shr 4);
    end;
  HeapPtrRec.Seg := Segment;

  AdjustUnitSegments (LastLoadedUsedUnit);
  AdjustUnitSegments (BaseUnitSegment);

  ReferencedUnitRecord := Ptr (SymbolTable [stReferencedModules].Segment, 0);
  While ReferencedUnitRecord <> SymbolTable [stReferencedModules].Ptr do
    begin
      If ReferencedUnitRecord^.ModuleSegment = TempHeapEnd then ReferencedUnitRecord^.ModuleSegment := TempHeapPtr;
      Inc (PChar (ReferencedUnitRecord), ReferencedUnitRecord^.UnitName.Len + 5);
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
Function LoadUsedUnits: Boolean;
Var SavedUnitPathAndName: PChar;
    LoadedUnitPublicIdentifiersChecksum: Word;
    UsedUnitPublicIdentifiersChecksum: Word;
    LoadedUnitSegment: Word;
    CurrentUsedUnitIdentifier: Word;
    LoadedUnit, BaseUnit: PUnitHeader;
    UnitIdentifierData: PUnitIdentifierData;
    UnitNameLen: Byte;

  Function FindUsedUnit: PUnitHeader;
  Var FoundUnit, UnitToCheck: PUnitHeader;
      PreviousUnitSegment: Word;

    Function CurrentUsedUnitName: PString;
    begin
      CurrentUsedUnitName :=
       @PIdentifier (Ptr (BaseUnitSegment, PUnitHeader (Ptr (BaseUnitSegment, 0))^.CurrentUsedUnitIdentifierOffset))^.Name.Str;
    end;

    Procedure CopyUnitPathAndNameToFileStrucBuffer;
    begin
      UnitPathAndName := StartOfSourceFileStructures;
      If (Ofs (UnitPathAndName^) + 80) > Ofs (CurrentSourceFile^) then Error (TooManyFiles);
      StartOfSourceFileStructures := StrECopy (UnitPathAndName, @Identifier) + 1;
    end;

    Function LoadUsedUnit: PUnitHeader;
    Var DirInfo: SearchRec;

      Function RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded: PUnitHeader;
      Var UnitPtr: PUnitHeader;
      begin
        RemoveSymbolTables_Code_To_TypedConstantsReferences (Ofs (UnitPathAndName^));
        If IsLastUnitAlreadyLoaded (UnitPtr) then RemoveLastLoadedUnit
          else begin
                UnitPtr := Ptr (LastLoadedUsedUnit, 0);
                PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.CodeSegment := 0;
                PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.OverlayedUnitCodeSize := 0;
              end;
        RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded := UnitPtr;
        CheckStack;
      end;

      Function FindCurrentUsedUnitFile (Options: Word): Byte;
      begin
        StrPCopy (@Identifier, CurrentUsedUnitName^);
        StrUpper (@Identifier);
        FindCurrentUsedUnitFile := FindFilePath (@Identifier, Options);
      end;

      Function IncludeOrObjectFilesChanged: Boolean;
      Var FilesBlockPtr: PUsedFilesBlockRecord;
          FilesBlockPtrRec: PtrRec absolute FilesBlockPtr;
          UnitHeader: PUnitHeader;
      begin
        IncludeOrObjectFilesChanged := True;
        UnitHeader := Ptr (LastLoadedUsedUnit, 0);
        FilesBlockPtr := Ptr (LastLoadedUsedUnit, UnitHeader^.BlockOffset [stUsedFiles]);
        Inc (FilesBlockPtrRec.Ofs, FilesBlockPtr^.Name.Len + 8);
        While FilesBlockPtrRec.Ofs <> UnitHeader^.BlockOffset [Succ (stUsedFiles)] do
          begin
            FindUsedFilePath (FilesBlockPtr, @Identifier);
            FindFirst (StrPas (@Identifier), AnyFile, DirInfo);
            If UnitDateTime < (DirInfo.Time + 1) then Exit;
            Inc (FilesBlockPtrRec.Ofs, FilesBlockPtr^.Name.Len + 8);
          end;
        IncludeOrObjectFilesChanged := False;
      end;

      Function CheckUnitName: PString;
      Var UsedUnitName: PString;
      begin
        UsedUnitName := CurrentUsedUnitName;
        If not IdentifiersEqual (@PIdentifier (Ptr (LastLoadedUsedUnit,
                                PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.UnitNameIdentifierOffset))^.Name, UsedUnitName) then
          begin
            AdditionalErrorStr := @Identifier;
            StrPCopy (@Identifier, UsedUnitName^);
            Error (UnitNameMismatch);
          end;
        CheckUnitName := UsedUnitName;
      end;

    begin
      If PushedUnitLevel <> UsedUnitLevel then
        begin
          PushUnit;
          CompilerModeOptions := CompilerModeOptions + [cmoCreateExeFile, cmoCompileToDisk, cmoCompileOnlyInterfaceUsedUnits];
          Saved_PushedUnitLevel := PushedUnitLevel;
          PushedUnitLevel := UsedUnitLevel;
        end;
      ClearCurrentModuleVarsAndCreateUnit;
      If cmoSearchForUnitSource in CompilerModeOptions then
        begin                                                               { Check for UNIT source }
          FindCurrentUsedUnitFile (Dir_Unit or Ext_PAS);
          FindFirst (StrPas (@Identifier), AnyFile, DirInfo);
          UnitSourceDateTime := DirInfo.Time + 1;
          If DOSError = 0 then                                              { UNIT source found }
            begin
              FindFilePath (@Identifier, Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_TPU);
              CopyUnitPathAndNameToFileStrucBuffer;
              If cmoMakeModifiedUnitsOnly in CompilerModeOptions then
                Repeat
                  FindFirst (StrPas (@Identifier), AnyFile, DirInfo);
                  If DOSError <> 0 then Break;                              { no TPU file }
                  UnitDateTime := DirInfo.Time + 1;
                  If UnitDateTime < UnitSourceDateTime then Break;
                  If ReadUnit (UnitPathAndName) then                        { TPU not older than PAS }
                    Repeat
                      CheckUnitName;
                      If IncludeOrObjectFilesChanged then Break;
                      If not LoadUsedUnits then Break;
                      LoadUsedUnit := RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded;
                      Exit;
                    until True;
                  ClearUnitHeader;
                until True;
                CurrentSourceFilePathLength := FindCurrentUsedUnitFile (Dir_Unit or Ext_PAS);
                CurrentFileName := @Identifier;
                CompileModule;
                CheckUnitName;
                LoadUsedUnit := RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded;
                Exit;
            end;
        end;
      FindCurrentUsedUnitFile (Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_TPU);
      FindFirst (StrPas (@Identifier), AnyFile, DirInfo);
      If DOSError <> 0 then FindCurrentUsedUnitFile (Dir_Unit or Ext_Forced or Ext_TPU);
      CopyUnitPathAndNameToFileStrucBuffer;
      If not ReadUnit (UnitPathAndName) then Error (UnitVersionMismatch);
      CheckUnitName;
      If not LoadUsedUnits then Error (UnitVersionMismatch);
      LoadUsedUnit := RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded;
    end;

    Function IsCurrentUsedUnitUsedInInterface (UnitPtr: PUnitHeader): Boolean;
    Var UsedUnitId: PIdentifier;
        UsedUnitIdData: PUnitIdentifierData;
    begin
      UsedUnitId := Ptr (Seg (UnitPtr^), UnitPtr^.CurrentUsedUnitIdentifierOffset);
      UsedUnitIdData := Ptr (Seg (UnitPtr^), Ofs (UsedUnitId^) + UsedUnitId^.Name.Len + 4);
      IsCurrentUsedUnitUsedInInterface := UsedInInterface in UsedUnitIdData^.UnitIdentifierFlags
    end;

    Procedure UnitError (Err: TCompilerError);
    Var UnitName: PString;
    begin
      UnitName := Ptr (BaseUnitSegment, PUnitHeader (Ptr (BaseUnitSegment, 0))^.CurrentUsedUnitIdentifierOffset + 3);
      AdditionalErrorStr := @Identifier;
      StrPCopy (@Identifier, UnitName^);
      Error (Err);
    end;

  begin
    CurrentIdentifier := CurrentUsedUnitName^;
    If FindUnitWithName (LastLoadedUsedUnit, 4, CurrentIdentifier, FoundUnit) then
      begin
        CheckStack;
        FindUsedUnit := FoundUnit;
        Exit;
      end;
    If not FindUnitWithName (BaseUnitSegment, 4, CurrentIdentifier, FoundUnit) then
      begin
        If not FindUnitWithName (LastLibraryUnitSegment, 6, CurrentIdentifier, FoundUnit) then
          begin
            FindUsedUnit := LoadUsedUnit;
            Exit;
          end;
        FoundUnit^.PreviousUnitSegment := LastLoadedUsedUnit;
        LastLoadedUsedUnit := Seg (FoundUnit^);
        If not LoadUsedUnits then UnitError (UnitVersionMismatch);
        PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.CodeSegment := 0;
        PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.OverlayedUnitCodeSize := 0;
        FindUsedUnit := FoundUnit;
        CheckStack;
        Exit;
      end;
    If not IsCurrentUsedUnitUsedInInterface (FoundUnit) then
      begin
        CheckStack;
        FindUsedUnit := FoundUnit;
        Exit;
      end;
    UnitToCheck := Ptr (BaseUnitSegment, 0);
    Repeat
      If not IsCurrentUsedUnitUsedInInterface (UnitToCheck) then
        begin
          FindUsedUnit := LoadUsedUnit;
          Exit;
        end;
      PreviousUnitSegment := UnitToCheck^.PreviousUnitSegment;
      UnitToCheck := Ptr (PreviousUnitSegment, 0);
    until PreviousUnitSegment = Seg (FoundUnit^);
    UnitError (CircularUnitReference);
  end;

{$IFDEF DebugUsedUnits}
  Var LevelCounter: Word;
{$ENDIF}

begin
  SavedUnitPathAndName := UnitPathAndName;
  Inc (UsedUnitLevel);
  BaseUnit := Ptr (LastLoadedUsedUnit, 0);
  LastLoadedUsedUnit := BaseUnit^.PreviousUnitSegment;
  BaseUnit^.PreviousUnitSegment := BaseUnitSegment;
  BaseUnitSegment := Seg (BaseUnit^);
  UnitNameLen := PIdentifier (Ptr (Seg (BaseUnit^), BaseUnit^.UnitNameIdentifierOffset))^.Name.Len;
  UnitIdentifierData := Ptr (Seg (BaseUnit^), BaseUnit^.UnitNameIdentifierOffset + UnitNameLen + 4); { Base unit data }
  LoadedUnitSegment := Seg (BaseUnit^);

{$IFDEF DebugUsedUnits}
  If UsedUnitLevel = 1 then Writeln;
  For LevelCounter := 2 to UsedUnitLevel do Write ('  ');
  If UsedUnitLevel = 1 then Write ('#') else Write (' ');
  Writeln ('LOAD USED UNITS BY: ',
            PIdentifier (Ptr (Seg (BaseUnit^), BaseUnit^.UnitNameIdentifierOffset))^.Name.Str);
{$ENDIF}

  Repeat
    UnitIdentifierData^.UnitSegment := LoadedUnitSegment;
    CurrentUsedUnitIdentifier := UnitIdentifierData^.NextUnitIdentifier;          { Next used unit }
    BaseUnit^.CurrentUsedUnitIdentifierOffset := CurrentUsedUnitIdentifier;
    If CurrentUsedUnitIdentifier = 0 then Break;                                  { No more used units }

{$IFDEF DebugUsedUnits}
    For LevelCounter := 1 to UsedUnitLevel do Write ('  ');
    Writeln (' USES: ', PIdentifier (Ptr (Seg (BaseUnit^), CurrentUsedUnitIdentifier))^.Name.Str);
{$ENDIF}

    LoadedUnit := FindUsedUnit;
    UnitNameLen := PIdentifier (Ptr (Seg (LoadedUnit^), LoadedUnit^.UnitNameIdentifierOffset))^.Name.Len;
    UnitIdentifierData := Ptr (Seg (LoadedUnit^), LoadedUnit^.UnitNameIdentifierOffset + UnitNameLen + 4);

{$IFDEF DebugUsedUnits}
    For LevelCounter := 1 to UsedUnitLevel do Write ('  ');
    Writeln (' LOADED: ', PIdentifier (Ptr (Seg (LoadedUnit^), LoadedUnit^.UnitNameIdentifierOffset))^.Name.Str);
{$ENDIF}

    LoadedUnitPublicIdentifiersChecksum := UnitIdentifierData^.PublicIdentifiersChecksum;
    LoadedUnitSegment := Seg (LoadedUnit^);
    BaseUnit := Ptr (BaseUnitSegment, 0);                                   { BaseUnitSegment might change during unit PUSH}
    UnitNameLen := PIdentifier (Ptr (Seg (BaseUnit^), CurrentUsedUnitIdentifier))^.Name.Len;
    UnitIdentifierData := Ptr (Seg (BaseUnit^), CurrentUsedUnitIdentifier + UnitNameLen + 4);
    UsedUnitPublicIdentifiersChecksum := UnitIdentifierData^.PublicIdentifiersChecksum;
    If UsedUnitPublicIdentifiersChecksum <> 0 then
      begin
        If UsedUnitPublicIdentifiersChecksum <> LoadedUnitPublicIdentifiersChecksum then
          begin
            Inc (CurrentUsedUnitIdentifier, 3);    { name of unit which has different PublicIdentifiersChecksum }
            Break;
          end;
      end else UnitIdentifierData^.PublicIdentifiersChecksum := LoadedUnitPublicIdentifiersChecksum;
  until False;                                                                    { Load next used unit }

  BaseUnitSegment := BaseUnit^.PreviousUnitSegment;
  BaseUnit^.PreviousUnitSegment := LastLoadedUsedUnit;
  LastLoadedUsedUnit := Seg (BaseUnit^);
  Dec (UsedUnitLevel);
  If UsedUnitLevel = Saved_PushedUnitLevel then PopUnit;
  UnitPathAndName := SavedUnitPathAndName;
  LoadUsedUnits := CurrentUsedUnitIdentifier = 0;                                    { All used units loaded }

{$IFDEF DebugUsedUnits}
  For LevelCounter := 1 to UsedUnitLevel do Write ('  ');
  If UsedUnitLevel = 0 then Write ('#') else Write (' ');
  Writeln ('LOAD USED UNITS END ',
             PIdentifier (Ptr (Seg (BaseUnit^), BaseUnit^.UnitNameIdentifierOffset))^.Name.Str);
  If UsedUnitLevel = 0 then Writeln;
{$ENDIF}

end;
```

### Code Block 2

```pascal
Procedure PushUnit;
Var ReferencedUnitRecord: PReferencedModulesBlockRecord;
    Segment: Word;
    Block: TSymbolTable;

  Procedure AdjustUnitSegments (Var Segment: Word);
  Var UnitPtr: PUnitHeader;
      UnitIdentifier: PIdentifier;
      UnitIdentifierData: PUnitIdentifierData;
  begin
    If Segment = HeapPtrRec.Seg then Segment := HeapEndRec.Seg;
    UnitPtr := Ptr (Segment, 0);
    While Seg (UnitPtr^) <> 0 do
      begin
        UnitIdentifier := Ptr (Seg (UnitPtr^), UnitPtr^.UnitNameIdentifierOffset);
        Repeat
          UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
          If UnitIdentifierData^.UnitSegment = HeapPtrRec.Seg then UnitIdentifierData^.UnitSegment := HeapEndRec.Seg;
          UnitIdentifier := Ptr (Seg (UnitPtr^), UnitIdentifierData^.NextUnitIdentifier);
        until Ofs (UnitIdentifier^) = 0;
        If UnitPtr^.PreviousUnitSegment = HeapPtrRec.Seg then UnitPtr^.PreviousUnitSegment := HeapEndRec.Seg;
        UnitPtr := Ptr (UnitPtr^.PreviousUnitSegment, 0);
      end;
  end;

begin
  HeapPtrRec.Seg := SymbolTable [stMain].Segment;                                     { PUSH Memory Blocks }
  Segment := HeapEndRec.Seg;
  For Block := stIntermediateCode downto stMain do
    begin
      SymbolTable [Block].Size := (SymbolTable [Block].UsedSize + $000F) and $FFF0;
      Dec (Segment, SymbolTable [Block].Size shr 4);
      Move (Ptr (SymbolTable [Block].Segment, 0)^, Ptr (Segment, 0)^, SymbolTable [Block].UsedSize);
      SymbolTable [Block].Segment := Segment;
    end;
  HeapEndRec.Seg := Segment;

  AdjustUnitSegments (LastLoadedUsedUnit);
  AdjustUnitSegments (BaseUnitSegment);

  ReferencedUnitRecord := Ptr (SymbolTable [stReferencedModules].Segment, 0);
  While ReferencedUnitRecord <> SymbolTable [stReferencedModules].Ptr do
    begin
      If ReferencedUnitRecord^.ModuleSegment = HeapPtrRec.Seg then ReferencedUnitRecord^.ModuleSegment := HeapEndRec.Seg;
      Inc (PChar (ReferencedUnitRecord), ReferencedUnitRecord^.UnitName.Len + 5);
    end;

  Dec (HeapEndRec.Seg, (Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions) +                   { PUSH Variables }
                        Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines)+ $000F) shr 4);
  Move (CompilerModeOptions, HeapEnd^, Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions));
  Move (ConditionalDefines, Ptr (HeapEndRec.Seg, Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions))^,
                             Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines));        { PUSH Conditional Defines }
end;

Procedure PopUnit;
Var TempHeapPtr, TempHeapEnd: Word;
    ReferencedUnitRecord: PReferencedModulesBlockRecord;
    Segment: Word;
    Block: TSymbolTable;

  Procedure AdjustUnitSegments (Var Segment: Word);
  Var UnitPtr: PUnitHeader;
      UnitIdentifier: PIdentifier;
      UnitIdentifierData: PUnitIdentifierData;
  begin
    If Segment = TempHeapEnd then Segment := TempHeapPtr;
    UnitPtr := Ptr (Segment, 0);
    While Seg (UnitPtr^) <> 0 do
      begin
        UnitIdentifier := Ptr (Seg (UnitPtr^), UnitPtr^.UnitNameIdentifierOffset);
        Repeat
          UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
          If UnitIdentifierData^.UnitSegment = TempHeapEnd then UnitIdentifierData^.UnitSegment := TempHeapPtr;
          UnitIdentifier := Ptr (Seg (UnitPtr^), UnitIdentifierData^.NextUnitIdentifier);
        until Ofs (UnitIdentifier^) = 0;
        If UnitPtr^.PreviousUnitSegment = TempHeapEnd then UnitPtr^.PreviousUnitSegment := TempHeapPtr;
        UnitPtr := Ptr (UnitPtr^.PreviousUnitSegment, 0);
      end;
  end;

begin
  Move (HeapEnd^, CompilerModeOptions, Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions));    { POP Variables }
  Move (Ptr (HeapEndRec.Seg, Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions))^, ConditionalDefines,
          Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines));                           { POP Conditional Defines }
  TempHeapEnd := HeapEndRec.Seg + (Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions) +
                   Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines) + $000F) shr 4;

  TempHeapPtr := HeapPtrRec.Seg;

  HeapEndRec.Seg := SymbolTable [stIntermediateCode].Segment + SymbolTable [stIntermediateCode].Size shr 4;
  Segment := HeapPtrRec.Seg;
  For Block := stMain to stIntermediateCode do
    begin
      Move (Ptr (SymbolTable [Block].Segment, 0)^, Ptr (Segment, 0)^, SymbolTable [Block].UsedSize);
      SymbolTable [Block].Segment := Segment;
      Inc (Segment, (SymbolTable [Block].UsedSize + $000F) shr 4);
    end;
  HeapPtrRec.Seg := Segment;

  AdjustUnitSegments (LastLoadedUsedUnit);
  AdjustUnitSegments (BaseUnitSegment);

  ReferencedUnitRecord := Ptr (SymbolTable [stReferencedModules].Segment, 0);
  While ReferencedUnitRecord <> SymbolTable [stReferencedModules].Ptr do
    begin
      If ReferencedUnitRecord^.ModuleSegment = TempHeapEnd then ReferencedUnitRecord^.ModuleSegment := TempHeapPtr;
      Inc (PChar (ReferencedUnitRecord), ReferencedUnitRecord^.UnitName.Len + 5);
    end;
end;
```

### Code Block 3

```pascal
Function LoadUsedUnits: Boolean;
Var SavedUnitPathAndName: PChar;
    LoadedUnitPublicIdentifiersChecksum: Word;
    UsedUnitPublicIdentifiersChecksum: Word;
    LoadedUnitSegment: Word;
    CurrentUsedUnitIdentifier: Word;
    LoadedUnit, BaseUnit: PUnitHeader;
    UnitIdentifierData: PUnitIdentifierData;
    UnitNameLen: Byte;

  Function FindUsedUnit: PUnitHeader;
  Var FoundUnit, UnitToCheck: PUnitHeader;
      PreviousUnitSegment: Word;

    Function CurrentUsedUnitName: PString;
    begin
      CurrentUsedUnitName :=
       @PIdentifier (Ptr (BaseUnitSegment, PUnitHeader (Ptr (BaseUnitSegment, 0))^.CurrentUsedUnitIdentifierOffset))^.Name.Str;
    end;

    Procedure CopyUnitPathAndNameToFileStrucBuffer;
    begin
      UnitPathAndName := StartOfSourceFileStructures;
      If (Ofs (UnitPathAndName^) + 80) > Ofs (CurrentSourceFile^) then Error (TooManyFiles);
      StartOfSourceFileStructures := StrECopy (UnitPathAndName, @Identifier) + 1;
    end;

    Function LoadUsedUnit: PUnitHeader;
    Var DirInfo: SearchRec;

      Function RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded: PUnitHeader;
      Var UnitPtr: PUnitHeader;
      begin
        RemoveSymbolTables_Code_To_TypedConstantsReferences (Ofs (UnitPathAndName^));
        If IsLastUnitAlreadyLoaded (UnitPtr) then RemoveLastLoadedUnit
          else begin
                UnitPtr := Ptr (LastLoadedUsedUnit, 0);
                PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.CodeSegment := 0;
                PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.OverlayedUnitCodeSize := 0;
              end;
        RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded := UnitPtr;
        CheckStack;
      end;

      Function FindCurrentUsedUnitFile (Options: Word): Byte;
      begin
        StrPCopy (@Identifier, CurrentUsedUnitName^);
        StrUpper (@Identifier);
        FindCurrentUsedUnitFile := FindFilePath (@Identifier, Options);
      end;

      Function IncludeOrObjectFilesChanged: Boolean;
      Var FilesBlockPtr: PUsedFilesBlockRecord;
          FilesBlockPtrRec: PtrRec absolute FilesBlockPtr;
          UnitHeader: PUnitHeader;
      begin
        IncludeOrObjectFilesChanged := True;
        UnitHeader := Ptr (LastLoadedUsedUnit, 0);
        FilesBlockPtr := Ptr (LastLoadedUsedUnit, UnitHeader^.BlockOffset [stUsedFiles]);
        Inc (FilesBlockPtrRec.Ofs, FilesBlockPtr^.Name.Len + 8);
        While FilesBlockPtrRec.Ofs <> UnitHeader^.BlockOffset [Succ (stUsedFiles)] do
          begin
            FindUsedFilePath (FilesBlockPtr, @Identifier);
            FindFirst (StrPas (@Identifier), AnyFile, DirInfo);
            If UnitDateTime < (DirInfo.Time + 1) then Exit;
            Inc (FilesBlockPtrRec.Ofs, FilesBlockPtr^.Name.Len + 8);
          end;
        IncludeOrObjectFilesChanged := False;
      end;

      Function CheckUnitName: PString;
      Var UsedUnitName: PString;
      begin
        UsedUnitName := CurrentUsedUnitName;
        If not IdentifiersEqual (@PIdentifier (Ptr (LastLoadedUsedUnit,
                                PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.UnitNameIdentifierOffset))^.Name, UsedUnitName) then
          begin
            AdditionalErrorStr := @Identifier;
            StrPCopy (@Identifier, UsedUnitName^);
            Error (UnitNameMismatch);
          end;
        CheckUnitName := UsedUnitName;
      end;

    begin
      If PushedUnitLevel <> UsedUnitLevel then
        begin
          PushUnit;
          CompilerModeOptions := CompilerModeOptions + [cmoCreateExeFile, cmoCompileToDisk, cmoCompileOnlyInterfaceUsedUnits];
          Saved_PushedUnitLevel := PushedUnitLevel;
          PushedUnitLevel := UsedUnitLevel;
        end;
      ClearCurrentModuleVarsAndCreateUnit;
      If cmoSearchForUnitSource in CompilerModeOptions then
        begin                                                               { Check for UNIT source }
          FindCurrentUsedUnitFile (Dir_Unit or Ext_PAS);
          FindFirst (StrPas (@Identifier), AnyFile, DirInfo);
          UnitSourceDateTime := DirInfo.Time + 1;
          If DOSError = 0 then                                              { UNIT source found }
            begin
              FindFilePath (@Identifier, Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_TPU);
              CopyUnitPathAndNameToFileStrucBuffer;
              If cmoMakeModifiedUnitsOnly in CompilerModeOptions then
                Repeat
                  FindFirst (StrPas (@Identifier), AnyFile, DirInfo);
                  If DOSError <> 0 then Break;                              { no TPU file }
                  UnitDateTime := DirInfo.Time + 1;
                  If UnitDateTime < UnitSourceDateTime then Break;
                  If ReadUnit (UnitPathAndName) then                        { TPU not older than PAS }
                    Repeat
                      CheckUnitName;
                      If IncludeOrObjectFilesChanged then Break;
                      If not LoadUsedUnits then Break;
                      LoadUsedUnit := RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded;
                      Exit;
                    until True;
                  ClearUnitHeader;
                until True;
                CurrentSourceFilePathLength := FindCurrentUsedUnitFile (Dir_Unit or Ext_PAS);
                CurrentFileName := @Identifier;
                CompileModule;
                CheckUnitName;
                LoadUsedUnit := RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded;
                Exit;
            end;
        end;
      FindCurrentUsedUnitFile (Dir_Forced or Dir_EXE_TPU or Ext_Forced or Ext_TPU);
      FindFirst (StrPas (@Identifier), AnyFile, DirInfo);
      If DOSError <> 0 then FindCurrentUsedUnitFile (Dir_Unit or Ext_Forced or Ext_TPU);
      CopyUnitPathAndNameToFileStrucBuffer;
      If not ReadUnit (UnitPathAndName) then Error (UnitVersionMismatch);
      CheckUnitName;
      If not LoadUsedUnits then Error (UnitVersionMismatch);
      LoadUsedUnit := RemoveSymbolTables_Code_To_TypedConstantsReferencesOrAllIfAlreadyLoaded;
    end;

    Function IsCurrentUsedUnitUsedInInterface (UnitPtr: PUnitHeader): Boolean;
    Var UsedUnitId: PIdentifier;
        UsedUnitIdData: PUnitIdentifierData;
    begin
      UsedUnitId := Ptr (Seg (UnitPtr^), UnitPtr^.CurrentUsedUnitIdentifierOffset);
      UsedUnitIdData := Ptr (Seg (UnitPtr^), Ofs (UsedUnitId^) + UsedUnitId^.Name.Len + 4);
      IsCurrentUsedUnitUsedInInterface := UsedInInterface in UsedUnitIdData^.UnitIdentifierFlags
    end;

    Procedure UnitError (Err: TCompilerError);
    Var UnitName: PString;
    begin
      UnitName := Ptr (BaseUnitSegment, PUnitHeader (Ptr (BaseUnitSegment, 0))^.CurrentUsedUnitIdentifierOffset + 3);
      AdditionalErrorStr := @Identifier;
      StrPCopy (@Identifier, UnitName^);
      Error (Err);
    end;

  begin
    CurrentIdentifier := CurrentUsedUnitName^;
    If FindUnitWithName (LastLoadedUsedUnit, 4, CurrentIdentifier, FoundUnit) then
      begin
        CheckStack;
        FindUsedUnit := FoundUnit;
        Exit;
      end;
    If not FindUnitWithName (BaseUnitSegment, 4, CurrentIdentifier, FoundUnit) then
      begin
        If not FindUnitWithName (LastLibraryUnitSegment, 6, CurrentIdentifier, FoundUnit) then
          begin
            FindUsedUnit := LoadUsedUnit;
            Exit;
          end;
        FoundUnit^.PreviousUnitSegment := LastLoadedUsedUnit;
        LastLoadedUsedUnit := Seg (FoundUnit^);
        If not LoadUsedUnits then UnitError (UnitVersionMismatch);
        PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.CodeSegment := 0;
        PUnitHeader (Ptr (LastLoadedUsedUnit, 0))^.OverlayedUnitCodeSize := 0;
        FindUsedUnit := FoundUnit;
        CheckStack;
        Exit;
      end;
    If not IsCurrentUsedUnitUsedInInterface (FoundUnit) then
      begin
        CheckStack;
        FindUsedUnit := FoundUnit;
        Exit;
      end;
    UnitToCheck := Ptr (BaseUnitSegment, 0);
    Repeat
      If not IsCurrentUsedUnitUsedInInterface (UnitToCheck) then
        begin
          FindUsedUnit := LoadUsedUnit;
          Exit;
        end;
      PreviousUnitSegment := UnitToCheck^.PreviousUnitSegment;
      UnitToCheck := Ptr (PreviousUnitSegment, 0);
    until PreviousUnitSegment = Seg (FoundUnit^);
    UnitError (CircularUnitReference);
  end;

{$IFDEF DebugUsedUnits}
  Var LevelCounter: Word;
{$ENDIF}

begin
  SavedUnitPathAndName := UnitPathAndName;
  Inc (UsedUnitLevel);
  BaseUnit := Ptr (LastLoadedUsedUnit, 0);
  LastLoadedUsedUnit := BaseUnit^.PreviousUnitSegment;
  BaseUnit^.PreviousUnitSegment := BaseUnitSegment;
  BaseUnitSegment := Seg (BaseUnit^);
  UnitNameLen := PIdentifier (Ptr (Seg (BaseUnit^), BaseUnit^.UnitNameIdentifierOffset))^.Name.Len;
  UnitIdentifierData := Ptr (Seg (BaseUnit^), BaseUnit^.UnitNameIdentifierOffset + UnitNameLen + 4); { Base unit data }
  LoadedUnitSegment := Seg (BaseUnit^);

{$IFDEF DebugUsedUnits}
  If UsedUnitLevel = 1 then Writeln;
  For LevelCounter := 2 to UsedUnitLevel do Write ('  ');
  If UsedUnitLevel = 1 then Write ('#') else Write (' ');
  Writeln ('LOAD USED UNITS BY: ',
            PIdentifier (Ptr (Seg (BaseUnit^), BaseUnit^.UnitNameIdentifierOffset))^.Name.Str);
{$ENDIF}

  Repeat
    UnitIdentifierData^.UnitSegment := LoadedUnitSegment;
    CurrentUsedUnitIdentifier := UnitIdentifierData^.NextUnitIdentifier;          { Next used unit }
    BaseUnit^.CurrentUsedUnitIdentifierOffset := CurrentUsedUnitIdentifier;
    If CurrentUsedUnitIdentifier = 0 then Break;                                  { No more used units }

{$IFDEF DebugUsedUnits}
    For LevelCounter := 1 to UsedUnitLevel do Write ('  ');
    Writeln (' USES: ', PIdentifier (Ptr (Seg (BaseUnit^), CurrentUsedUnitIdentifier))^.Name.Str);
{$ENDIF}

    LoadedUnit := FindUsedUnit;
    UnitNameLen := PIdentifier (Ptr (Seg (LoadedUnit^), LoadedUnit^.UnitNameIdentifierOffset))^.Name.Len;
    UnitIdentifierData := Ptr (Seg (LoadedUnit^), LoadedUnit^.UnitNameIdentifierOffset + UnitNameLen + 4);

{$IFDEF DebugUsedUnits}
    For LevelCounter := 1 to UsedUnitLevel do Write ('  ');
    Writeln (' LOADED: ', PIdentifier (Ptr (Seg (LoadedUnit^), LoadedUnit^.UnitNameIdentifierOffset))^.Name.Str);
{$ENDIF}

    LoadedUnitPublicIdentifiersChecksum := UnitIdentifierData^.PublicIdentifiersChecksum;
    LoadedUnitSegment := Seg (LoadedUnit^);
    BaseUnit := Ptr (BaseUnitSegment, 0);                                   { BaseUnitSegment might change during unit PUSH}
    UnitNameLen := PIdentifier (Ptr (Seg (BaseUnit^), CurrentUsedUnitIdentifier))^.Name.Len;
    UnitIdentifierData := Ptr (Seg (BaseUnit^), CurrentUsedUnitIdentifier + UnitNameLen + 4);
    UsedUnitPublicIdentifiersChecksum := UnitIdentifierData^.PublicIdentifiersChecksum;
    If UsedUnitPublicIdentifiersChecksum <> 0 then
      begin
        If UsedUnitPublicIdentifiersChecksum <> LoadedUnitPublicIdentifiersChecksum then
          begin
            Inc (CurrentUsedUnitIdentifier, 3);    { name of unit which has different PublicIdentifiersChecksum }
            Break;
          end;
      end else UnitIdentifierData^.PublicIdentifiersChecksum := LoadedUnitPublicIdentifiersChecksum;
  until False;                                                                    { Load next used unit }

  BaseUnitSegment := BaseUnit^.PreviousUnitSegment;
  BaseUnit^.PreviousUnitSegment := LastLoadedUsedUnit;
  LastLoadedUsedUnit := Seg (BaseUnit^);
  Dec (UsedUnitLevel);
  If UsedUnitLevel = Saved_PushedUnitLevel then PopUnit;
  UnitPathAndName := SavedUnitPathAndName;
  LoadUsedUnits := CurrentUsedUnitIdentifier = 0;                                    { All used units loaded }

{$IFDEF DebugUsedUnits}
  For LevelCounter := 1 to UsedUnitLevel do Write ('  ');
  If UsedUnitLevel = 0 then Write ('#') else Write (' ');
  Writeln ('LOAD USED UNITS END ',
             PIdentifier (Ptr (Seg (BaseUnit^), BaseUnit^.UnitNameIdentifierOffset))^.Name.Str);
  If UsedUnitLevel = 0 then Writeln;
{$ENDIF}

end;
```

### Code Block 4

```pascal
Procedure PushUnit;
Var ReferencedUnitRecord: PReferencedModulesBlockRecord;
    Segment: Word;
    Block: TSymbolTable;

  Procedure AdjustUnitSegments (Var Segment: Word);
  Var UnitPtr: PUnitHeader;
      UnitIdentifier: PIdentifier;
      UnitIdentifierData: PUnitIdentifierData;
  begin
    If Segment = HeapPtrRec.Seg then Segment := HeapEndRec.Seg;
    UnitPtr := Ptr (Segment, 0);
    While Seg (UnitPtr^) <> 0 do
      begin
        UnitIdentifier := Ptr (Seg (UnitPtr^), UnitPtr^.UnitNameIdentifierOffset);
        Repeat
          UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
          If UnitIdentifierData^.UnitSegment = HeapPtrRec.Seg then UnitIdentifierData^.UnitSegment := HeapEndRec.Seg;
          UnitIdentifier := Ptr (Seg (UnitPtr^), UnitIdentifierData^.NextUnitIdentifier);
        until Ofs (UnitIdentifier^) = 0;
        If UnitPtr^.PreviousUnitSegment = HeapPtrRec.Seg then UnitPtr^.PreviousUnitSegment := HeapEndRec.Seg;
        UnitPtr := Ptr (UnitPtr^.PreviousUnitSegment, 0);
      end;
  end;

begin
  HeapPtrRec.Seg := SymbolTable [stMain].Segment;                                     { PUSH Memory Blocks }
  Segment := HeapEndRec.Seg;
  For Block := stIntermediateCode downto stMain do
    begin
      SymbolTable [Block].Size := (SymbolTable [Block].UsedSize + $000F) and $FFF0;
      Dec (Segment, SymbolTable [Block].Size shr 4);
      Move (Ptr (SymbolTable [Block].Segment, 0)^, Ptr (Segment, 0)^, SymbolTable [Block].UsedSize);
      SymbolTable [Block].Segment := Segment;
    end;
  HeapEndRec.Seg := Segment;

  AdjustUnitSegments (LastLoadedUsedUnit);
  AdjustUnitSegments (BaseUnitSegment);

  ReferencedUnitRecord := Ptr (SymbolTable [stReferencedModules].Segment, 0);
  While ReferencedUnitRecord <> SymbolTable [stReferencedModules].Ptr do
    begin
      If ReferencedUnitRecord^.ModuleSegment = HeapPtrRec.Seg then ReferencedUnitRecord^.ModuleSegment := HeapEndRec.Seg;
      Inc (PChar (ReferencedUnitRecord), ReferencedUnitRecord^.UnitName.Len + 5);
    end;

  Dec (HeapEndRec.Seg, (Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions) +                   { PUSH Variables }
                        Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines)+ $000F) shr 4);
  Move (CompilerModeOptions, HeapEnd^, Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions));
  Move (ConditionalDefines, Ptr (HeapEndRec.Seg, Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions))^,
                             Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines));        { PUSH Conditional Defines }
end;

Procedure PopUnit;
Var TempHeapPtr, TempHeapEnd: Word;
    ReferencedUnitRecord: PReferencedModulesBlockRecord;
    Segment: Word;
    Block: TSymbolTable;

  Procedure AdjustUnitSegments (Var Segment: Word);
  Var UnitPtr: PUnitHeader;
      UnitIdentifier: PIdentifier;
      UnitIdentifierData: PUnitIdentifierData;
  begin
    If Segment = TempHeapEnd then Segment := TempHeapPtr;
    UnitPtr := Ptr (Segment, 0);
    While Seg (UnitPtr^) <> 0 do
      begin
        UnitIdentifier := Ptr (Seg (UnitPtr^), UnitPtr^.UnitNameIdentifierOffset);
        Repeat
          UnitIdentifierData := PUnitIdentifierData (PChar (UnitIdentifier) + UnitIdentifier^.Name.Len + 4);
          If UnitIdentifierData^.UnitSegment = TempHeapEnd then UnitIdentifierData^.UnitSegment := TempHeapPtr;
          UnitIdentifier := Ptr (Seg (UnitPtr^), UnitIdentifierData^.NextUnitIdentifier);
        until Ofs (UnitIdentifier^) = 0;
        If UnitPtr^.PreviousUnitSegment = TempHeapEnd then UnitPtr^.PreviousUnitSegment := TempHeapPtr;
        UnitPtr := Ptr (UnitPtr^.PreviousUnitSegment, 0);
      end;
  end;

begin
  Move (HeapEnd^, CompilerModeOptions, Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions));    { POP Variables }
  Move (Ptr (HeapEndRec.Seg, Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions))^, ConditionalDefines,
          Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines));                           { POP Conditional Defines }
  TempHeapEnd := HeapEndRec.Seg + (Ofs (CurrentLineBuff) - Ofs (CompilerModeOptions) +
                   Ofs (ConditionalDefinesEnd^) - Ofs (ConditionalDefines) + $000F) shr 4;

  TempHeapPtr := HeapPtrRec.Seg;

  HeapEndRec.Seg := SymbolTable [stIntermediateCode].Segment + SymbolTable [stIntermediateCode].Size shr 4;
  Segment := HeapPtrRec.Seg;
  For Block := stMain to stIntermediateCode do
    begin
      Move (Ptr (SymbolTable [Block].Segment, 0)^, Ptr (Segment, 0)^, SymbolTable [Block].UsedSize);
      SymbolTable [Block].Segment := Segment;
      Inc (Segment, (SymbolTable [Block].UsedSize + $000F) shr 4);
    end;
  HeapPtrRec.Seg := Segment;

  AdjustUnitSegments (LastLoadedUsedUnit);
  AdjustUnitSegments (BaseUnitSegment);

  ReferencedUnitRecord := Ptr (SymbolTable [stReferencedModules].Segment, 0);
  While ReferencedUnitRecord <> SymbolTable [stReferencedModules].Ptr do
    begin
      If ReferencedUnitRecord^.ModuleSegment = TempHeapEnd then ReferencedUnitRecord^.ModuleSegment := TempHeapPtr;
      Inc (PChar (ReferencedUnitRecord), ReferencedUnitRecord^.UnitName.Len + 5);
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
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/loading-used-units
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

