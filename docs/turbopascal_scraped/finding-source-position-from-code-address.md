# Finding Source Position From Code Address - Turbo Pascal Compiler

**URL:** https://turbopascal.org/finding-source-position-from-code-address

---

## Table of Contents

- Finding Source Position From Code Address

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Finding Source Position From Code Address
Turbo Pascal has a command line switch
/Fseg:ofs
which can find module name and source line number compiled at this address. The program is normally
compiled
but instead of
compilation status
the source position is displayed.
Procedure FindAndWriteSourceLine;
Var UnitRecord: PUsedFilesBlockRecord;
    LinePointer: PChar absolute UnitRecord;

  Function FindSourceFileAndLineNumberWithError (ErrorAddress: PtrRec; Var ErrorLineNumber: Word;
                                                 Var UnitRecord: PUsedFilesBlockRecord): Boolean;
  Var UnitPointer: PUnitHeader;
      UnitPointerRec: PtrRec absolute UnitPointer;
      Address: LongRec;
      ProgramCodeBlockRecord: PProgramCodeBlockRecord;
      Blockub9Record: PStatementCodeSizesBlockRecord absolute ProgramCodeBlockRecord;
      Blockub9RecordRec: PtrRec absolute Blockub9Record;
      BytePtr: PChar;
      SourceLineNumber, W: Word;
  Label NextUnit, UnitFound;
  begin
    FindSourceFileAndLineNumberWithError := False;
    UnitPointerRec.Ptr := Ptr (LastLoadedUsedUnit, 0);
    While UnitPointerRec.Seg <> 0 do
      begin
        If ErrorAddress.Seg < UnitPointer^.CodeSegment then GoTo NextUnit;
        If ErrorAddress.Seg = UnitPointer^.CodeSegment then
          If ErrorAddress.Ofs = UnitPointer^.OverlayedUnitCodeSize then GoTo UnitFound;
        If ErrorAddress.Seg < $1000 then
          begin
            Address.Long := ErrorAddress.Seg shl 4 + ErrorAddress.Ofs;
            If Address.WordH = 0 then If Address.WordL < UnitPointer^.CodeSize then
              begin
                ErrorAddress.Ofs := Address.WordL;
                GoTo UnitFound;
              end;
          end;
  NextUnit:

      UnitPointerRec.Seg := UnitPointer^.PreviousUnitSegment;
      end;
    Exit;

  UnitFound:

    ProgramCodeBlockRecord := Ptr (UnitPointerRec.Seg, UnitPointer^.BlockOffset [stCodeBlocks]);
    Repeat
      If ProgramCodeBlockRecord^.Offset <> $FFFF then
        begin
          If ErrorAddress.Ofs < ProgramCodeBlockRecord^.CodeSize then Break;
          Dec (ErrorAddress.Ofs, ProgramCodeBlockRecord^.CodeSize);
        end;
      Inc (ProgramCodeBlockRecord);
    until False;
    Address.Long := ProgramCodeBlockRecord^.StatementCodeSizesRecordOffset +
                    UnitPointer^.BlockOffset [stSourceLineCodeOffsets];
    If Address.WordH <> 0 then Exit;
    Blockub9RecordRec.Ofs := Address.WordL;
    Address.Long := ErrorAddress.Ofs - Blockub9Record^.SizeOfConstantsInCode;
    If Address.WordH <> 0 then Exit;
    SourceLineNumber := Blockub9Record^.SizeOfConstantsInCode;
    Repeat
      BytePtr := @Blockub9Record^.StatementsCodeSizeData;
      W := Byte (BytePtr^);
      Inc (BytePtr);
      If W and $80 <> 0 then
        begin
          W := (W and $7F) shl 8 + Byte (BytePtr^);
          Inc (BytePtr);
        end;
      Inc (SourceLineNumber);
      If ErrorAddress.Ofs < W then Break;
    until False;
    Dec (SourceLineNumber);
    ErrorLineNumber := SourceLineNumber;
    UnitRecord := Ptr (UnitPointerRec.Seg, Blockub9Record^.SourceFileRecordOffset + UnitPointer^.BlockOffset [stUsedFiles]);
    FindSourceFileAndLineNumberWithError := True;
  end;

begin
  SetErrorAddress (@DummyErrorProc);
  CurrentSourceFile := @EndOfFileStructure;
  FirstSourceFile := @EndOfFileStructure;
  ProgramBlockCompilation := False;
  If SourceType > stUnit then Error (TargetAddressNotFound);
  If not FindSourceFileAndLineNumberWithError (FindErrorAddress, ErrorLineNumber, UnitRecord) then
    Error (TargetAddressNotFound);
  FindUsedFilePath (UnitRecord, @Identifier);
  OpenSourceFile (UnitRecord, @Identifier);
  Repeat
    CheckStack;
    If not NextLine then Error (UnexpectedEndOfFile);
  until CurrentSourceFile^.CurrentLineNumber >= ErrorLineNumber;
  LinePointer := CurrentLine - 1;
  Repeat
    Inc (LinePointer);
  until (LinePointer^ = #0) or (LinePointer^ > ' ');
  CurrentSourceFile^.CurrentPosition := Ofs (LinePointer^);
  ErrorSourceFile := CurrentSourceFile;
  Error (TargetAddressFound);
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
Procedure FindAndWriteSourceLine;
Var UnitRecord: PUsedFilesBlockRecord;
    LinePointer: PChar absolute UnitRecord;

  Function FindSourceFileAndLineNumberWithError (ErrorAddress: PtrRec; Var ErrorLineNumber: Word;
                                                 Var UnitRecord: PUsedFilesBlockRecord): Boolean;
  Var UnitPointer: PUnitHeader;
      UnitPointerRec: PtrRec absolute UnitPointer;
      Address: LongRec;
      ProgramCodeBlockRecord: PProgramCodeBlockRecord;
      Blockub9Record: PStatementCodeSizesBlockRecord absolute ProgramCodeBlockRecord;
      Blockub9RecordRec: PtrRec absolute Blockub9Record;
      BytePtr: PChar;
      SourceLineNumber, W: Word;
  Label NextUnit, UnitFound;
  begin
    FindSourceFileAndLineNumberWithError := False;
    UnitPointerRec.Ptr := Ptr (LastLoadedUsedUnit, 0);
    While UnitPointerRec.Seg <> 0 do
      begin
        If ErrorAddress.Seg < UnitPointer^.CodeSegment then GoTo NextUnit;
        If ErrorAddress.Seg = UnitPointer^.CodeSegment then
          If ErrorAddress.Ofs = UnitPointer^.OverlayedUnitCodeSize then GoTo UnitFound;
        If ErrorAddress.Seg < $1000 then
          begin
            Address.Long := ErrorAddress.Seg shl 4 + ErrorAddress.Ofs;
            If Address.WordH = 0 then If Address.WordL < UnitPointer^.CodeSize then
              begin
                ErrorAddress.Ofs := Address.WordL;
                GoTo UnitFound;
              end;
          end;
  NextUnit:

      UnitPointerRec.Seg := UnitPointer^.PreviousUnitSegment;
      end;
    Exit;

  UnitFound:

    ProgramCodeBlockRecord := Ptr (UnitPointerRec.Seg, UnitPointer^.BlockOffset [stCodeBlocks]);
    Repeat
      If ProgramCodeBlockRecord^.Offset <> $FFFF then
        begin
          If ErrorAddress.Ofs < ProgramCodeBlockRecord^.CodeSize then Break;
          Dec (ErrorAddress.Ofs, ProgramCodeBlockRecord^.CodeSize);
        end;
      Inc (ProgramCodeBlockRecord);
    until False;
    Address.Long := ProgramCodeBlockRecord^.StatementCodeSizesRecordOffset +
                    UnitPointer^.BlockOffset [stSourceLineCodeOffsets];
    If Address.WordH <> 0 then Exit;
    Blockub9RecordRec.Ofs := Address.WordL;
    Address.Long := ErrorAddress.Ofs - Blockub9Record^.SizeOfConstantsInCode;
    If Address.WordH <> 0 then Exit;
    SourceLineNumber := Blockub9Record^.SizeOfConstantsInCode;
    Repeat
      BytePtr := @Blockub9Record^.StatementsCodeSizeData;
      W := Byte (BytePtr^);
      Inc (BytePtr);
      If W and $80 <> 0 then
        begin
          W := (W and $7F) shl 8 + Byte (BytePtr^);
          Inc (BytePtr);
        end;
      Inc (SourceLineNumber);
      If ErrorAddress.Ofs < W then Break;
    until False;
    Dec (SourceLineNumber);
    ErrorLineNumber := SourceLineNumber;
    UnitRecord := Ptr (UnitPointerRec.Seg, Blockub9Record^.SourceFileRecordOffset + UnitPointer^.BlockOffset [stUsedFiles]);
    FindSourceFileAndLineNumberWithError := True;
  end;

begin
  SetErrorAddress (@DummyErrorProc);
  CurrentSourceFile := @EndOfFileStructure;
  FirstSourceFile := @EndOfFileStructure;
  ProgramBlockCompilation := False;
  If SourceType > stUnit then Error (TargetAddressNotFound);
  If not FindSourceFileAndLineNumberWithError (FindErrorAddress, ErrorLineNumber, UnitRecord) then
    Error (TargetAddressNotFound);
  FindUsedFilePath (UnitRecord, @Identifier);
  OpenSourceFile (UnitRecord, @Identifier);
  Repeat
    CheckStack;
    If not NextLine then Error (UnexpectedEndOfFile);
  until CurrentSourceFile^.CurrentLineNumber >= ErrorLineNumber;
  LinePointer := CurrentLine - 1;
  Repeat
    Inc (LinePointer);
  until (LinePointer^ = #0) or (LinePointer^ > ' ');
  CurrentSourceFile^.CurrentPosition := Ofs (LinePointer^);
  ErrorSourceFile := CurrentSourceFile;
  Error (TargetAddressFound);
end;
```

### Code Block 2

```pascal
Procedure FindAndWriteSourceLine;
Var UnitRecord: PUsedFilesBlockRecord;
    LinePointer: PChar absolute UnitRecord;

  Function FindSourceFileAndLineNumberWithError (ErrorAddress: PtrRec; Var ErrorLineNumber: Word;
                                                 Var UnitRecord: PUsedFilesBlockRecord): Boolean;
  Var UnitPointer: PUnitHeader;
      UnitPointerRec: PtrRec absolute UnitPointer;
      Address: LongRec;
      ProgramCodeBlockRecord: PProgramCodeBlockRecord;
      Blockub9Record: PStatementCodeSizesBlockRecord absolute ProgramCodeBlockRecord;
      Blockub9RecordRec: PtrRec absolute Blockub9Record;
      BytePtr: PChar;
      SourceLineNumber, W: Word;
  Label NextUnit, UnitFound;
  begin
    FindSourceFileAndLineNumberWithError := False;
    UnitPointerRec.Ptr := Ptr (LastLoadedUsedUnit, 0);
    While UnitPointerRec.Seg <> 0 do
      begin
        If ErrorAddress.Seg < UnitPointer^.CodeSegment then GoTo NextUnit;
        If ErrorAddress.Seg = UnitPointer^.CodeSegment then
          If ErrorAddress.Ofs = UnitPointer^.OverlayedUnitCodeSize then GoTo UnitFound;
        If ErrorAddress.Seg < $1000 then
          begin
            Address.Long := ErrorAddress.Seg shl 4 + ErrorAddress.Ofs;
            If Address.WordH = 0 then If Address.WordL < UnitPointer^.CodeSize then
              begin
                ErrorAddress.Ofs := Address.WordL;
                GoTo UnitFound;
              end;
          end;
  NextUnit:

      UnitPointerRec.Seg := UnitPointer^.PreviousUnitSegment;
      end;
    Exit;

  UnitFound:

    ProgramCodeBlockRecord := Ptr (UnitPointerRec.Seg, UnitPointer^.BlockOffset [stCodeBlocks]);
    Repeat
      If ProgramCodeBlockRecord^.Offset <> $FFFF then
        begin
          If ErrorAddress.Ofs < ProgramCodeBlockRecord^.CodeSize then Break;
          Dec (ErrorAddress.Ofs, ProgramCodeBlockRecord^.CodeSize);
        end;
      Inc (ProgramCodeBlockRecord);
    until False;
    Address.Long := ProgramCodeBlockRecord^.StatementCodeSizesRecordOffset +
                    UnitPointer^.BlockOffset [stSourceLineCodeOffsets];
    If Address.WordH <> 0 then Exit;
    Blockub9RecordRec.Ofs := Address.WordL;
    Address.Long := ErrorAddress.Ofs - Blockub9Record^.SizeOfConstantsInCode;
    If Address.WordH <> 0 then Exit;
    SourceLineNumber := Blockub9Record^.SizeOfConstantsInCode;
    Repeat
      BytePtr := @Blockub9Record^.StatementsCodeSizeData;
      W := Byte (BytePtr^);
      Inc (BytePtr);
      If W and $80 <> 0 then
        begin
          W := (W and $7F) shl 8 + Byte (BytePtr^);
          Inc (BytePtr);
        end;
      Inc (SourceLineNumber);
      If ErrorAddress.Ofs < W then Break;
    until False;
    Dec (SourceLineNumber);
    ErrorLineNumber := SourceLineNumber;
    UnitRecord := Ptr (UnitPointerRec.Seg, Blockub9Record^.SourceFileRecordOffset + UnitPointer^.BlockOffset [stUsedFiles]);
    FindSourceFileAndLineNumberWithError := True;
  end;

begin
  SetErrorAddress (@DummyErrorProc);
  CurrentSourceFile := @EndOfFileStructure;
  FirstSourceFile := @EndOfFileStructure;
  ProgramBlockCompilation := False;
  If SourceType > stUnit then Error (TargetAddressNotFound);
  If not FindSourceFileAndLineNumberWithError (FindErrorAddress, ErrorLineNumber, UnitRecord) then
    Error (TargetAddressNotFound);
  FindUsedFilePath (UnitRecord, @Identifier);
  OpenSourceFile (UnitRecord, @Identifier);
  Repeat
    CheckStack;
    If not NextLine then Error (UnexpectedEndOfFile);
  until CurrentSourceFile^.CurrentLineNumber >= ErrorLineNumber;
  LinePointer := CurrentLine - 1;
  Repeat
    Inc (LinePointer);
  until (LinePointer^ = #0) or (LinePointer^ > ' ');
  CurrentSourceFile^.CurrentPosition := Ofs (LinePointer^);
  ErrorSourceFile := CurrentSourceFile;
  Error (TargetAddressFound);
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compilation-progress-and-status-display
- https://turbopascal.org/compiler
- https://turbopascal.org/expressions
- https://turbopascal.org/finding-source-position-from-code-address
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/main-program
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

