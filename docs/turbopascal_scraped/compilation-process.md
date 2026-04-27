# The Compilation Process - Turbo Pascal Compiler

**URL:** https://turbopascal.org/compilation-process

---

## Table of Contents

- The Compilation Process

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
The Compilation Process
This is where the actual compilation starts. Turbo Pascal first clears and prepares some variables and data structures and then compiles the module (unit or program). If program was compiled then compiler checks for indirect references and calls a procedure to
link the code and data and to create the executable file
.
Const

  stProgram            = - 1;
  stUnit               =   0;
  stUnitInterface      =   1;
  stUnitImplementation =   2;

Procedure Compile;
Var CurrentUnitInstance: PUnitHeader;
begin
  SetErrorAddress (@CompilerError);
  PushedUnitLevel := 0;
  Saved_PushedUnitLevel := $FFFF;
  CurrentSourceFile := @EndOfFileStructure;
  FirstSourceFile := @EndOfFileStructure;
  TempBufferFreePositionOffset := Ofs (TempBuffer);
  If FPU then Include (InitialEnvironmentFlags,  enfCPU87);
  InitialUnitFlags := [];
  ClearCurrentModuleVarsAndCreateUnit;
  CompileModule;
  If SourceType >= stUnit then
    If IsLastUnitAlreadyLoaded (CurrentUnitInstance) then RemoveLastLoadedUnit;
  FindReferencedModules;
  If SourceType >= stUnit then
    begin
      ModuleStack := 0;
      ModuleHeapMin := 0;
      ModuleHeapMax := 0;
    end else Link;
end;
Before each module is compiled Turbo Pascal resets token data and module flags,
opens source file
, creates conditional defines and according to the first token it compiles the module either as a unit or as a program.
Procedure CompileModule;
Var SavedFirstSourceFile: PFileStructure;
begin
  TokenForSlash := Token_Slash;
  TokenForEqual := Token_Equal;
  ModuleUnitFlags := InitialUnitFlags;
  OpenSourceFile (StoreFileNameToFilesBlock (CurrentFileName + CurrentSourceFilePathLength,
                                                                 ufPascalSource), CurrentFileName);
  SetFileDateTime;
  SavedFirstSourceFile := FirstSourceFile;
  FirstSourceFile := CurrentSourceFile;
  CreateConditionalDefines;
  GetNextToken;
  If not (cmoCompileOnlyInterfaceUsedUnits in CompilerModeOptions) and (Token <> Token_UNIT) then CompileProgram
    else CompileUnit;
  FirstSourceFile := SavedFirstSourceFile;
  ReturnToPreviousSourceFile;
end;
If the program has optional
Program
name declaration it is processed and the name is
stored into main symbol table
, otherwise the default
PROGRAM
name is used. Next
used units are processed
and
loaded
,
declarations are processed
and
main program block is compiled
. At the end object files are imported, checks are made for undefined Forward and External declarations and
symbol tables are joined and compacted
.
Procedure CompileProgram;
begin
  SourceType := stProgram;
  If CheckAndGetNextToken (Token_PROGRAM) then
    begin
      ExpectIdentifier;
      CreateSymbolTableAndStoreMainModuleNameIdentifier;
      GetNextToken;
      If CheckAndGetNextToken (Token_LeftParenthesis) then
        begin
          Repeat
            ExpectTokenAndGetNext (Token_Identifier);
          until not CheckAndGetNextToken (Token_Comma);
          ExpectTokenAndGetNext (Token_RightParenthesis);
        end;
      ExpectTokenAndGetNext (Token_Semicolon);
    end else begin
               CopyStringToCurrentIdentifier ('PROGRAM');
               CreateSymbolTableAndStoreMainModuleNameIdentifier;
             end;
  SetModuleFlagsAndProcessUsedUnits;
  ProcessDeclarations;
  ProcessMainProgramBlock;
  CheckForPeriodAndModuleEnd;
  ImportObjectFiles;
  CheckForUndefined_FORWARD_Or_EXTERNAL (Ptr (SymbolTable [stProcedures].Segment, SizeOf (TProceduresBlockRecord)));
  JoinSymbolTablesAndCreateUnit;
  CopySegmentsOfSymbolTablesToUnitHeader;
end;
Units have
Interface
and
implementation
part. When used units are compiled only used units that are used in the
Interface
part (of those units that are compiled) are processed and compiled (if necessary).
Used units
and
declarations
are processed twice – in the interface and implementation section. The unit’s
initialization
part is optional. Once the unit is compiled the symbol tables are
compacted
and
saved as unit file
.
Procedure CompileUnit;
Var CurrentUnitInstance: PUnitHeader;
begin
  SourceType := stUnitInterface;
  ExpectTokenAndGetNext (Token_UNIT);
  ExpectIdentifier;
  CreateUnitIdentiferSymbolTableAndStoreMainUnitName;
  GetNextToken;
  ExpectTokenAndGetNext (Token_Semicolon);
  ExpectTokenAndGetNext (Token_INTERFACE);
  SetModuleFlagsAndProcessUsedUnits;
  If not (cmoCompileOnlyInterfaceUsedUnits in CompilerModeOptions) or not IsLastUnitAlreadyLoaded (CurrentUnitInstance) then
    begin
      ProcessDeclarations;
      CalculateIdentifiersChecksum;
      SourceType := stUnitImplementation;
      ExpectTokenAndGetNext (Token_IMPLEMENTATION);
      ProcessUsedUnits;
      ProcessDeclarations;
      If Token = Token_BEGIN then ProcessMainProgramBlock         { Initialization part }
        else ExpectTokenAndGetNext (Token_END);
      CheckForPeriodAndModuleEnd;
      ImportObjectFiles;
      CheckForUndefined_FORWARD_Or_EXTERNAL (Ptr (SymbolTable [stProcedures].Segment, 8));
      RemovePrivateIdentifiersFromUnit;
      JoinSymbolTablesAndCreateUnit;
      CreateUnitFile;
    end else JoinSymbolTablesAndCreateUnit;
  CopySegmentsOfSymbolTablesToUnitHeader;
end;
This procedure
creates main symbol table
and stores module name. If
System unit
is compiled it loads boot-strap symbol table
SYSTEM.TPS
.
Procedure CreateUnitIdentiferSymbolTableAndStoreMainUnitName;
begin
  If CompareIdentifierToWord (_System) then
    begin
      SystemUnitCompilation := True;
      StrPCopy (@Identifier, 'SYSTEM.TPS');
      FindFilePath (@Identifier, Dir_Unit or Ext_Original);
      ReadUnit (@Identifier);
      StoreMainModuleNameAndCreateProcedureRecord;
    end else CreateSymbolTableAndStoreMainModuleNameIdentifier;
end;
This procedure stores module name into symbol table and creates record for the main procedure (unit’s initialization or main program).
Procedure StoreMainModuleNameAndCreateProcedureRecord;
Var UnitIdentifierData: PUnitIdentifierData;
    UnitIdentifier: PIdentifier;
begin
  PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^.UnitNameIdentifierOffset := MainSymbolTable.UsedSize;
  UnitIdentifierData := StoreNewIdentifierToSymbolTable (SizeOf (TUnitIdentifierData), UnitIdentifier);
  UnitIdentifier^.Token := Token_UnitIdentifier;
  UnitIdentifierData^.UnitSegment := Seg (UnitIdentifierData^);
  LastUnitIdentifierData := Ofs (UnitIdentifierData^);
  With PProceduresBlockRecord (IncreaseSymbolTable (stProcedures, SizeOf (TProceduresBlockRecord)))^ do
    begin
      OverlayedProcedureOffset := 0;
      prW2 := 0;
      ProgramCodeBlockRecordOffset := $FFFF;
      SizeOfConstants := $FFFF;
    end;
end;
Procedure CreateSymbolTableAndStoreMainModuleNameIdentifier;
begin
  With PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^ do
    begin
      PublicIdentifiersListOffset := MainSymbolTable.NextRecordOffset;
      PrivateIdentifiersListOffset := MainSymbolTable.NextRecordOffset;
    end;
  CreateSymbolTable ($40);
  StoreMainModuleNameAndCreateProcedureRecord;
end;
Once the unit is compiled, private identifiers are removed from the main symbol table if local debugging symbols are not needed.
Procedure RemovePrivateIdentifiersFromUnit;
Var UnitHeader: PUnitHeader;
begin
  If not (LocalDebugSymbols in ModuleCompilerSwitches) then
    begin
      UnitHeader := Ptr (SymbolTable [stMain].Segment, 0);
      SymbolTable [stMain].UsedSize := UnitHeader^.PrivateIdentifiersListOffset;
      UnitHeader^.PrivateIdentifiersListOffset := UnitHeader^.PublicIdentifiersListOffset;
    end;
end;
This procedure expects period, checks if source file is last,
writes compilation progress
and creates source file record.
Procedure CheckForPeriodAndModuleEnd;
begin
  If Token <> Token_Period then Error (PeriodExpected);
  If CurrentSourceFile <> FirstSourceFile then Error (UnexpectedEndOfFile);
  WriteCompilationProgress (CurrentSourceFile);
  CreateSourceFilesRecord;
end;
This procedure checks all procedures and reports error if there is some undefined
Forward
or
External
procedure.
Procedure CheckForUndefined_FORWARD_Or_EXTERNAL (CurrentProcedureToCheck: PProceduresBlockRecord);
Var ProcedureIdentifier, UnitIdentifier: PIdentifier;
    ProcedureIdentifierData: PProcedureIdentifierData;
    ErrorIdentifier: PChar;
begin
  While CurrentProcedureToCheck <> SymbolTable [stProcedures].Ptr do
    begin
      If CurrentProcedureToCheck^.ProgramCodeBlockRecordOffset = $FFFF then
        begin
          ProcedureIdentifier := Ptr (SymbolTable [stMain].Segment, CurrentProcedureToCheck^.ProcIdentifierOffset);
          ErrorIdentifier := @CurrentIdentifier;
          ProcedureIdentifierData := Pointer (PChar (ProcedureIdentifier) + ProcedureIdentifier^.Name.Len + 4);
          If pfMethod in ProcedureIdentifierData^.Flags then
            begin
              UnitIdentifier := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierData^.OuterBlockProcedureIdentifier + 4);
              StrPCopy (ErrorIdentifier, UnitIdentifier^.Name.Str);
              Inc (ErrorIdentifier, UnitIdentifier^.Name.Len);
              ErrorIdentifier^ := '.';
              Inc (ErrorIdentifier);
            end;
          StrPCopy (ErrorIdentifier, ProcedureIdentifier^.Name.Str);
          If pfExternal in ProcedureIdentifierData^.Flags then FileError (@CurrentIdentifier, UndefinedExternal)
            else FileError (@CurrentIdentifier, UndefinedForward)
        end;
      Inc (CurrentProcedureToCheck);
    end;
end;
This procedure checks if referenced module references loaded module. If such module is found its address is stored in reference record otherwise error is reported.
Procedure FindReferencedModules;
Var UnitSegment, PossibleReferencedUnitSegment: Word;
    ReferencedUnitRecord: Word;
    UnitPtr, ReferencedUnit: PUnitHeader;
    PossibleReferencedUnitName: PString;
begin
  UnitSegment := LastLoadedUsedUnit;
  Repeat
    UnitPtr := Ptr (UnitSegment, 0);
    ReferencedUnitRecord := UnitPtr^.BlockOffset [stReferencedModules];
    While ReferencedUnitRecord <> UnitPtr^.BlockOffset [Succ (stReferencedModules)] do
      begin
        PossibleReferencedUnitSegment := LastLoadedUsedUnit;
        PossibleReferencedUnitName := @PReferencedModulesBlockRecord (Ptr (UnitSegment, ReferencedUnitRecord))^.UnitName;
        Repeat
          ReferencedUnit := Ptr (PossibleReferencedUnitSegment, 0);
          If IdentifiersEqual (@PIdentifier (Ptr (PossibleReferencedUnitSegment,
                                             ReferencedUnit^.UnitNameIdentifierOffset))^.Name.Str,
                               PossibleReferencedUnitName) then Break;
          PossibleReferencedUnitSegment := ReferencedUnit^.PreviousUnitSegment;
        until PossibleReferencedUnitSegment = 0;
        If PossibleReferencedUnitSegment = 0 then Error (InvalidIndirectReference);
        PReferencedModulesBlockRecord (Ptr (UnitSegment, ReferencedUnitRecord))^.ModuleSegment :=
                                                                                                PossibleReferencedUnitSegment;
        ReferencedUnitRecord := Ofs (PossibleReferencedUnitName^);
      end;
    UnitSegment := UnitPtr^.PreviousUnitSegment;
  until UnitSegment = 0;
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

  stProgram            = - 1;
  stUnit               =   0;
  stUnitInterface      =   1;
  stUnitImplementation =   2;

Procedure Compile;
Var CurrentUnitInstance: PUnitHeader;
begin
  SetErrorAddress (@CompilerError);
  PushedUnitLevel := 0;
  Saved_PushedUnitLevel := $FFFF;
  CurrentSourceFile := @EndOfFileStructure;
  FirstSourceFile := @EndOfFileStructure;
  TempBufferFreePositionOffset := Ofs (TempBuffer);
  If FPU then Include (InitialEnvironmentFlags,  enfCPU87);
  InitialUnitFlags := [];
  ClearCurrentModuleVarsAndCreateUnit;
  CompileModule;
  If SourceType >= stUnit then
    If IsLastUnitAlreadyLoaded (CurrentUnitInstance) then RemoveLastLoadedUnit;
  FindReferencedModules;
  If SourceType >= stUnit then
    begin
      ModuleStack := 0;
      ModuleHeapMin := 0;
      ModuleHeapMax := 0;
    end else Link;
end;
```

### Code Block 2

```pascal
Procedure CompileModule;
Var SavedFirstSourceFile: PFileStructure;
begin
  TokenForSlash := Token_Slash;
  TokenForEqual := Token_Equal;
  ModuleUnitFlags := InitialUnitFlags;
  OpenSourceFile (StoreFileNameToFilesBlock (CurrentFileName + CurrentSourceFilePathLength,
                                                                 ufPascalSource), CurrentFileName);
  SetFileDateTime;
  SavedFirstSourceFile := FirstSourceFile;
  FirstSourceFile := CurrentSourceFile;
  CreateConditionalDefines;
  GetNextToken;
  If not (cmoCompileOnlyInterfaceUsedUnits in CompilerModeOptions) and (Token <> Token_UNIT) then CompileProgram
    else CompileUnit;
  FirstSourceFile := SavedFirstSourceFile;
  ReturnToPreviousSourceFile;
end;
```

### Code Block 3

```pascal
Procedure CompileProgram;
begin
  SourceType := stProgram;
  If CheckAndGetNextToken (Token_PROGRAM) then
    begin
      ExpectIdentifier;
      CreateSymbolTableAndStoreMainModuleNameIdentifier;
      GetNextToken;
      If CheckAndGetNextToken (Token_LeftParenthesis) then
        begin
          Repeat
            ExpectTokenAndGetNext (Token_Identifier);
          until not CheckAndGetNextToken (Token_Comma);
          ExpectTokenAndGetNext (Token_RightParenthesis);
        end;
      ExpectTokenAndGetNext (Token_Semicolon);
    end else begin
               CopyStringToCurrentIdentifier ('PROGRAM');
               CreateSymbolTableAndStoreMainModuleNameIdentifier;
             end;
  SetModuleFlagsAndProcessUsedUnits;
  ProcessDeclarations;
  ProcessMainProgramBlock;
  CheckForPeriodAndModuleEnd;
  ImportObjectFiles;
  CheckForUndefined_FORWARD_Or_EXTERNAL (Ptr (SymbolTable [stProcedures].Segment, SizeOf (TProceduresBlockRecord)));
  JoinSymbolTablesAndCreateUnit;
  CopySegmentsOfSymbolTablesToUnitHeader;
end;
```

### Code Block 4

```pascal
Procedure CompileUnit;
Var CurrentUnitInstance: PUnitHeader;
begin
  SourceType := stUnitInterface;
  ExpectTokenAndGetNext (Token_UNIT);
  ExpectIdentifier;
  CreateUnitIdentiferSymbolTableAndStoreMainUnitName;
  GetNextToken;
  ExpectTokenAndGetNext (Token_Semicolon);
  ExpectTokenAndGetNext (Token_INTERFACE);
  SetModuleFlagsAndProcessUsedUnits;
  If not (cmoCompileOnlyInterfaceUsedUnits in CompilerModeOptions) or not IsLastUnitAlreadyLoaded (CurrentUnitInstance) then
    begin
      ProcessDeclarations;
      CalculateIdentifiersChecksum;
      SourceType := stUnitImplementation;
      ExpectTokenAndGetNext (Token_IMPLEMENTATION);
      ProcessUsedUnits;
      ProcessDeclarations;
      If Token = Token_BEGIN then ProcessMainProgramBlock         { Initialization part }
        else ExpectTokenAndGetNext (Token_END);
      CheckForPeriodAndModuleEnd;
      ImportObjectFiles;
      CheckForUndefined_FORWARD_Or_EXTERNAL (Ptr (SymbolTable [stProcedures].Segment, 8));
      RemovePrivateIdentifiersFromUnit;
      JoinSymbolTablesAndCreateUnit;
      CreateUnitFile;
    end else JoinSymbolTablesAndCreateUnit;
  CopySegmentsOfSymbolTablesToUnitHeader;
end;
```

### Code Block 5

```pascal
Procedure CreateUnitIdentiferSymbolTableAndStoreMainUnitName;
begin
  If CompareIdentifierToWord (_System) then
    begin
      SystemUnitCompilation := True;
      StrPCopy (@Identifier, 'SYSTEM.TPS');
      FindFilePath (@Identifier, Dir_Unit or Ext_Original);
      ReadUnit (@Identifier);
      StoreMainModuleNameAndCreateProcedureRecord;
    end else CreateSymbolTableAndStoreMainModuleNameIdentifier;
end;
```

### Code Block 6

```pascal
Procedure StoreMainModuleNameAndCreateProcedureRecord;
Var UnitIdentifierData: PUnitIdentifierData;
    UnitIdentifier: PIdentifier;
begin
  PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^.UnitNameIdentifierOffset := MainSymbolTable.UsedSize;
  UnitIdentifierData := StoreNewIdentifierToSymbolTable (SizeOf (TUnitIdentifierData), UnitIdentifier);
  UnitIdentifier^.Token := Token_UnitIdentifier;
  UnitIdentifierData^.UnitSegment := Seg (UnitIdentifierData^);
  LastUnitIdentifierData := Ofs (UnitIdentifierData^);
  With PProceduresBlockRecord (IncreaseSymbolTable (stProcedures, SizeOf (TProceduresBlockRecord)))^ do
    begin
      OverlayedProcedureOffset := 0;
      prW2 := 0;
      ProgramCodeBlockRecordOffset := $FFFF;
      SizeOfConstants := $FFFF;
    end;
end;
```

### Code Block 7

```pascal
Procedure CreateSymbolTableAndStoreMainModuleNameIdentifier;
begin
  With PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^ do
    begin
      PublicIdentifiersListOffset := MainSymbolTable.NextRecordOffset;
      PrivateIdentifiersListOffset := MainSymbolTable.NextRecordOffset;
    end;
  CreateSymbolTable ($40);
  StoreMainModuleNameAndCreateProcedureRecord;
end;
```

### Code Block 8

```pascal
Procedure RemovePrivateIdentifiersFromUnit;
Var UnitHeader: PUnitHeader;
begin
  If not (LocalDebugSymbols in ModuleCompilerSwitches) then
    begin
      UnitHeader := Ptr (SymbolTable [stMain].Segment, 0);
      SymbolTable [stMain].UsedSize := UnitHeader^.PrivateIdentifiersListOffset;
      UnitHeader^.PrivateIdentifiersListOffset := UnitHeader^.PublicIdentifiersListOffset;
    end;
end;
```

### Code Block 9

```pascal
Procedure CheckForPeriodAndModuleEnd;
begin
  If Token <> Token_Period then Error (PeriodExpected);
  If CurrentSourceFile <> FirstSourceFile then Error (UnexpectedEndOfFile);
  WriteCompilationProgress (CurrentSourceFile);
  CreateSourceFilesRecord;
end;
```

### Code Block 10

```pascal
Procedure CheckForUndefined_FORWARD_Or_EXTERNAL (CurrentProcedureToCheck: PProceduresBlockRecord);
Var ProcedureIdentifier, UnitIdentifier: PIdentifier;
    ProcedureIdentifierData: PProcedureIdentifierData;
    ErrorIdentifier: PChar;
begin
  While CurrentProcedureToCheck <> SymbolTable [stProcedures].Ptr do
    begin
      If CurrentProcedureToCheck^.ProgramCodeBlockRecordOffset = $FFFF then
        begin
          ProcedureIdentifier := Ptr (SymbolTable [stMain].Segment, CurrentProcedureToCheck^.ProcIdentifierOffset);
          ErrorIdentifier := @CurrentIdentifier;
          ProcedureIdentifierData := Pointer (PChar (ProcedureIdentifier) + ProcedureIdentifier^.Name.Len + 4);
          If pfMethod in ProcedureIdentifierData^.Flags then
            begin
              UnitIdentifier := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierData^.OuterBlockProcedureIdentifier + 4);
              StrPCopy (ErrorIdentifier, UnitIdentifier^.Name.Str);
              Inc (ErrorIdentifier, UnitIdentifier^.Name.Len);
              ErrorIdentifier^ := '.';
              Inc (ErrorIdentifier);
            end;
          StrPCopy (ErrorIdentifier, ProcedureIdentifier^.Name.Str);
          If pfExternal in ProcedureIdentifierData^.Flags then FileError (@CurrentIdentifier, UndefinedExternal)
            else FileError (@CurrentIdentifier, UndefinedForward)
        end;
      Inc (CurrentProcedureToCheck);
    end;
end;
```

### Code Block 11

```pascal
Procedure FindReferencedModules;
Var UnitSegment, PossibleReferencedUnitSegment: Word;
    ReferencedUnitRecord: Word;
    UnitPtr, ReferencedUnit: PUnitHeader;
    PossibleReferencedUnitName: PString;
begin
  UnitSegment := LastLoadedUsedUnit;
  Repeat
    UnitPtr := Ptr (UnitSegment, 0);
    ReferencedUnitRecord := UnitPtr^.BlockOffset [stReferencedModules];
    While ReferencedUnitRecord <> UnitPtr^.BlockOffset [Succ (stReferencedModules)] do
      begin
        PossibleReferencedUnitSegment := LastLoadedUsedUnit;
        PossibleReferencedUnitName := @PReferencedModulesBlockRecord (Ptr (UnitSegment, ReferencedUnitRecord))^.UnitName;
        Repeat
          ReferencedUnit := Ptr (PossibleReferencedUnitSegment, 0);
          If IdentifiersEqual (@PIdentifier (Ptr (PossibleReferencedUnitSegment,
                                             ReferencedUnit^.UnitNameIdentifierOffset))^.Name.Str,
                               PossibleReferencedUnitName) then Break;
          PossibleReferencedUnitSegment := ReferencedUnit^.PreviousUnitSegment;
        until PossibleReferencedUnitSegment = 0;
        If PossibleReferencedUnitSegment = 0 then Error (InvalidIndirectReference);
        PReferencedModulesBlockRecord (Ptr (UnitSegment, ReferencedUnitRecord))^.ModuleSegment :=
                                                                                                PossibleReferencedUnitSegment;
        ReferencedUnitRecord := Ofs (PossibleReferencedUnitName^);
      end;
    UnitSegment := UnitPtr^.PreviousUnitSegment;
  until UnitSegment = 0;
end;
```

### Code Block 12

```pascal
Const

  stProgram            = - 1;
  stUnit               =   0;
  stUnitInterface      =   1;
  stUnitImplementation =   2;

Procedure Compile;
Var CurrentUnitInstance: PUnitHeader;
begin
  SetErrorAddress (@CompilerError);
  PushedUnitLevel := 0;
  Saved_PushedUnitLevel := $FFFF;
  CurrentSourceFile := @EndOfFileStructure;
  FirstSourceFile := @EndOfFileStructure;
  TempBufferFreePositionOffset := Ofs (TempBuffer);
  If FPU then Include (InitialEnvironmentFlags,  enfCPU87);
  InitialUnitFlags := [];
  ClearCurrentModuleVarsAndCreateUnit;
  CompileModule;
  If SourceType >= stUnit then
    If IsLastUnitAlreadyLoaded (CurrentUnitInstance) then RemoveLastLoadedUnit;
  FindReferencedModules;
  If SourceType >= stUnit then
    begin
      ModuleStack := 0;
      ModuleHeapMin := 0;
      ModuleHeapMax := 0;
    end else Link;
end;
```

### Code Block 13

```pascal
Procedure CompileModule;
Var SavedFirstSourceFile: PFileStructure;
begin
  TokenForSlash := Token_Slash;
  TokenForEqual := Token_Equal;
  ModuleUnitFlags := InitialUnitFlags;
  OpenSourceFile (StoreFileNameToFilesBlock (CurrentFileName + CurrentSourceFilePathLength,
                                                                 ufPascalSource), CurrentFileName);
  SetFileDateTime;
  SavedFirstSourceFile := FirstSourceFile;
  FirstSourceFile := CurrentSourceFile;
  CreateConditionalDefines;
  GetNextToken;
  If not (cmoCompileOnlyInterfaceUsedUnits in CompilerModeOptions) and (Token <> Token_UNIT) then CompileProgram
    else CompileUnit;
  FirstSourceFile := SavedFirstSourceFile;
  ReturnToPreviousSourceFile;
end;
```

### Code Block 14

```pascal
Procedure CompileProgram;
begin
  SourceType := stProgram;
  If CheckAndGetNextToken (Token_PROGRAM) then
    begin
      ExpectIdentifier;
      CreateSymbolTableAndStoreMainModuleNameIdentifier;
      GetNextToken;
      If CheckAndGetNextToken (Token_LeftParenthesis) then
        begin
          Repeat
            ExpectTokenAndGetNext (Token_Identifier);
          until not CheckAndGetNextToken (Token_Comma);
          ExpectTokenAndGetNext (Token_RightParenthesis);
        end;
      ExpectTokenAndGetNext (Token_Semicolon);
    end else begin
               CopyStringToCurrentIdentifier ('PROGRAM');
               CreateSymbolTableAndStoreMainModuleNameIdentifier;
             end;
  SetModuleFlagsAndProcessUsedUnits;
  ProcessDeclarations;
  ProcessMainProgramBlock;
  CheckForPeriodAndModuleEnd;
  ImportObjectFiles;
  CheckForUndefined_FORWARD_Or_EXTERNAL (Ptr (SymbolTable [stProcedures].Segment, SizeOf (TProceduresBlockRecord)));
  JoinSymbolTablesAndCreateUnit;
  CopySegmentsOfSymbolTablesToUnitHeader;
end;
```

### Code Block 15

```pascal
Procedure CompileUnit;
Var CurrentUnitInstance: PUnitHeader;
begin
  SourceType := stUnitInterface;
  ExpectTokenAndGetNext (Token_UNIT);
  ExpectIdentifier;
  CreateUnitIdentiferSymbolTableAndStoreMainUnitName;
  GetNextToken;
  ExpectTokenAndGetNext (Token_Semicolon);
  ExpectTokenAndGetNext (Token_INTERFACE);
  SetModuleFlagsAndProcessUsedUnits;
  If not (cmoCompileOnlyInterfaceUsedUnits in CompilerModeOptions) or not IsLastUnitAlreadyLoaded (CurrentUnitInstance) then
    begin
      ProcessDeclarations;
      CalculateIdentifiersChecksum;
      SourceType := stUnitImplementation;
      ExpectTokenAndGetNext (Token_IMPLEMENTATION);
      ProcessUsedUnits;
      ProcessDeclarations;
      If Token = Token_BEGIN then ProcessMainProgramBlock         { Initialization part }
        else ExpectTokenAndGetNext (Token_END);
      CheckForPeriodAndModuleEnd;
      ImportObjectFiles;
      CheckForUndefined_FORWARD_Or_EXTERNAL (Ptr (SymbolTable [stProcedures].Segment, 8));
      RemovePrivateIdentifiersFromUnit;
      JoinSymbolTablesAndCreateUnit;
      CreateUnitFile;
    end else JoinSymbolTablesAndCreateUnit;
  CopySegmentsOfSymbolTablesToUnitHeader;
end;
```

### Code Block 16

```pascal
Procedure CreateUnitIdentiferSymbolTableAndStoreMainUnitName;
begin
  If CompareIdentifierToWord (_System) then
    begin
      SystemUnitCompilation := True;
      StrPCopy (@Identifier, 'SYSTEM.TPS');
      FindFilePath (@Identifier, Dir_Unit or Ext_Original);
      ReadUnit (@Identifier);
      StoreMainModuleNameAndCreateProcedureRecord;
    end else CreateSymbolTableAndStoreMainModuleNameIdentifier;
end;
```

### Code Block 17

```pascal
Procedure StoreMainModuleNameAndCreateProcedureRecord;
Var UnitIdentifierData: PUnitIdentifierData;
    UnitIdentifier: PIdentifier;
begin
  PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^.UnitNameIdentifierOffset := MainSymbolTable.UsedSize;
  UnitIdentifierData := StoreNewIdentifierToSymbolTable (SizeOf (TUnitIdentifierData), UnitIdentifier);
  UnitIdentifier^.Token := Token_UnitIdentifier;
  UnitIdentifierData^.UnitSegment := Seg (UnitIdentifierData^);
  LastUnitIdentifierData := Ofs (UnitIdentifierData^);
  With PProceduresBlockRecord (IncreaseSymbolTable (stProcedures, SizeOf (TProceduresBlockRecord)))^ do
    begin
      OverlayedProcedureOffset := 0;
      prW2 := 0;
      ProgramCodeBlockRecordOffset := $FFFF;
      SizeOfConstants := $FFFF;
    end;
end;
```

### Code Block 18

```pascal
Procedure CreateSymbolTableAndStoreMainModuleNameIdentifier;
begin
  With PUnitHeader (Ptr (MainSymbolTable.Segment, 0))^ do
    begin
      PublicIdentifiersListOffset := MainSymbolTable.NextRecordOffset;
      PrivateIdentifiersListOffset := MainSymbolTable.NextRecordOffset;
    end;
  CreateSymbolTable ($40);
  StoreMainModuleNameAndCreateProcedureRecord;
end;
```

### Code Block 19

```pascal
Procedure RemovePrivateIdentifiersFromUnit;
Var UnitHeader: PUnitHeader;
begin
  If not (LocalDebugSymbols in ModuleCompilerSwitches) then
    begin
      UnitHeader := Ptr (SymbolTable [stMain].Segment, 0);
      SymbolTable [stMain].UsedSize := UnitHeader^.PrivateIdentifiersListOffset;
      UnitHeader^.PrivateIdentifiersListOffset := UnitHeader^.PublicIdentifiersListOffset;
    end;
end;
```

### Code Block 20

```pascal
Procedure CheckForPeriodAndModuleEnd;
begin
  If Token <> Token_Period then Error (PeriodExpected);
  If CurrentSourceFile <> FirstSourceFile then Error (UnexpectedEndOfFile);
  WriteCompilationProgress (CurrentSourceFile);
  CreateSourceFilesRecord;
end;
```

### Code Block 21

```pascal
Procedure CheckForUndefined_FORWARD_Or_EXTERNAL (CurrentProcedureToCheck: PProceduresBlockRecord);
Var ProcedureIdentifier, UnitIdentifier: PIdentifier;
    ProcedureIdentifierData: PProcedureIdentifierData;
    ErrorIdentifier: PChar;
begin
  While CurrentProcedureToCheck <> SymbolTable [stProcedures].Ptr do
    begin
      If CurrentProcedureToCheck^.ProgramCodeBlockRecordOffset = $FFFF then
        begin
          ProcedureIdentifier := Ptr (SymbolTable [stMain].Segment, CurrentProcedureToCheck^.ProcIdentifierOffset);
          ErrorIdentifier := @CurrentIdentifier;
          ProcedureIdentifierData := Pointer (PChar (ProcedureIdentifier) + ProcedureIdentifier^.Name.Len + 4);
          If pfMethod in ProcedureIdentifierData^.Flags then
            begin
              UnitIdentifier := Ptr (SymbolTable [stMain].Segment, ProcedureIdentifierData^.OuterBlockProcedureIdentifier + 4);
              StrPCopy (ErrorIdentifier, UnitIdentifier^.Name.Str);
              Inc (ErrorIdentifier, UnitIdentifier^.Name.Len);
              ErrorIdentifier^ := '.';
              Inc (ErrorIdentifier);
            end;
          StrPCopy (ErrorIdentifier, ProcedureIdentifier^.Name.Str);
          If pfExternal in ProcedureIdentifierData^.Flags then FileError (@CurrentIdentifier, UndefinedExternal)
            else FileError (@CurrentIdentifier, UndefinedForward)
        end;
      Inc (CurrentProcedureToCheck);
    end;
end;
```

### Code Block 22

```pascal
Procedure FindReferencedModules;
Var UnitSegment, PossibleReferencedUnitSegment: Word;
    ReferencedUnitRecord: Word;
    UnitPtr, ReferencedUnit: PUnitHeader;
    PossibleReferencedUnitName: PString;
begin
  UnitSegment := LastLoadedUsedUnit;
  Repeat
    UnitPtr := Ptr (UnitSegment, 0);
    ReferencedUnitRecord := UnitPtr^.BlockOffset [stReferencedModules];
    While ReferencedUnitRecord <> UnitPtr^.BlockOffset [Succ (stReferencedModules)] do
      begin
        PossibleReferencedUnitSegment := LastLoadedUsedUnit;
        PossibleReferencedUnitName := @PReferencedModulesBlockRecord (Ptr (UnitSegment, ReferencedUnitRecord))^.UnitName;
        Repeat
          ReferencedUnit := Ptr (PossibleReferencedUnitSegment, 0);
          If IdentifiersEqual (@PIdentifier (Ptr (PossibleReferencedUnitSegment,
                                             ReferencedUnit^.UnitNameIdentifierOffset))^.Name.Str,
                               PossibleReferencedUnitName) then Break;
          PossibleReferencedUnitSegment := ReferencedUnit^.PreviousUnitSegment;
        until PossibleReferencedUnitSegment = 0;
        If PossibleReferencedUnitSegment = 0 then Error (InvalidIndirectReference);
        PReferencedModulesBlockRecord (Ptr (UnitSegment, ReferencedUnitRecord))^.ModuleSegment :=
                                                                                                PossibleReferencedUnitSegment;
        ReferencedUnitRecord := Ofs (PossibleReferencedUnitName^);
      end;
    UnitSegment := UnitPtr^.PreviousUnitSegment;
  until UnitSegment = 0;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compacting-symbol-tables
- https://turbopascal.org/compilation-process
- https://turbopascal.org/compilation-progress-and-status-display
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-unit-file
- https://turbopascal.org/dealing-with-source-files
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/linking-modules-and-creating-exe-file
- https://turbopascal.org/loading-used-units
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-declarations
- https://turbopascal.org/processing-program-block
- https://turbopascal.org/processing-used-units
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/storing-identifiers-into-symbol-table
- https://turbopascal.org/symbol-table-management
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/system-functions
- https://turbopascal.org/system-procedures
- https://turbopascal.org/system-unit
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

