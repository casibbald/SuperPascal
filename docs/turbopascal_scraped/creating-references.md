# Creating References - Turbo Pascal Compiler

**URL:** https://turbopascal.org/creating-references

---

## Table of Contents

- Creating References

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Creating References
Whenever Turbo Pascal needs to reference anything that doesn’t have fixed location, i.e. its address is not known yet like variables, typed constants or procedures, the compiler generates
intermediate code
reference record which contains all information to later resolve the reference.
Procedure GenerateReference (Segment, BlockRecord, RefOfs: Word; RefFlags: TReferenceFlagSet);
Var ReferencedUnitRecord: Word;
begin
  ReferencedUnitRecord := AddReferencedModule (Segment);
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 7))^ do
    begin
      RecordType := icReference;
      ReferencedUnitRecordOfs := ReferencedUnitRecord or Word (RefFlags);
      ReferencedBlockRecordOfs := BlockRecord;
      ReferencedOfs := RefOfs;
    end;
end;
All referenced modules are kept in a symbol table.
Function AddReferencedModule (ReferencedModuleSegment: Word): Word;
Var ReferencedModuleRecord: PReferencedModulesBlockRecord;
    ReferencedModuleRecordOfs: Word absolute ReferencedModuleRecord;
    UnitNameIdentifier: PIdentifier;
begin
  ReferencedModuleRecord := Ptr (SymbolTable [stReferencedModules].Segment, 0);
  While ReferencedModuleRecordOfs <> SymbolTable [stReferencedModules].NextRecordOffset do
    begin
      If ReferencedModuleRecord^.ModuleSegment = ReferencedModuleSegment then
        begin
          AddReferencedModule := ReferencedModuleRecordOfs;
          Exit;
        end;
      Inc (ReferencedModuleRecordOfs, ReferencedModuleRecord^.UnitName.Len + 5);
    end;
  If ReferencedModuleRecordOfs >= $1000 then Error (TooManySymbols);
  UnitNameIdentifier :=                      { ReferencedModuleRecord Offset must be < $1000 to add flags in highest nibble }
    PIdentifier (Ptr (ReferencedModuleSegment, PUnitHeader (Ptr (ReferencedModuleSegment, 0))^.UnitNameIdentifierOffset));
  ReferencedModuleRecord := IncreaseSymbolTable (stReferencedModules, UnitNameIdentifier^.Name.Len + 5);
  With ReferencedModuleRecord^ do
    begin
      ModuleSegment := ReferencedModuleSegment;
      rmrW2 := 0;
      UnitName.Str := UnitNameIdentifier^.Name.Str;
    end;
  AddReferencedModule := Ofs (ReferencedModuleRecord^);
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
Procedure GenerateReference (Segment, BlockRecord, RefOfs: Word; RefFlags: TReferenceFlagSet);
Var ReferencedUnitRecord: Word;
begin
  ReferencedUnitRecord := AddReferencedModule (Segment);
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 7))^ do
    begin
      RecordType := icReference;
      ReferencedUnitRecordOfs := ReferencedUnitRecord or Word (RefFlags);
      ReferencedBlockRecordOfs := BlockRecord;
      ReferencedOfs := RefOfs;
    end;
end;
```

### Code Block 2

```pascal
Function AddReferencedModule (ReferencedModuleSegment: Word): Word;
Var ReferencedModuleRecord: PReferencedModulesBlockRecord;
    ReferencedModuleRecordOfs: Word absolute ReferencedModuleRecord;
    UnitNameIdentifier: PIdentifier;
begin
  ReferencedModuleRecord := Ptr (SymbolTable [stReferencedModules].Segment, 0);
  While ReferencedModuleRecordOfs <> SymbolTable [stReferencedModules].NextRecordOffset do
    begin
      If ReferencedModuleRecord^.ModuleSegment = ReferencedModuleSegment then
        begin
          AddReferencedModule := ReferencedModuleRecordOfs;
          Exit;
        end;
      Inc (ReferencedModuleRecordOfs, ReferencedModuleRecord^.UnitName.Len + 5);
    end;
  If ReferencedModuleRecordOfs >= $1000 then Error (TooManySymbols);
  UnitNameIdentifier :=                      { ReferencedModuleRecord Offset must be < $1000 to add flags in highest nibble }
    PIdentifier (Ptr (ReferencedModuleSegment, PUnitHeader (Ptr (ReferencedModuleSegment, 0))^.UnitNameIdentifierOffset));
  ReferencedModuleRecord := IncreaseSymbolTable (stReferencedModules, UnitNameIdentifier^.Name.Len + 5);
  With ReferencedModuleRecord^ do
    begin
      ModuleSegment := ReferencedModuleSegment;
      rmrW2 := 0;
      UnitName.Str := UnitNameIdentifier^.Name.Str;
    end;
  AddReferencedModule := Ofs (ReferencedModuleRecord^);
end;
```

### Code Block 3

```pascal
Procedure GenerateReference (Segment, BlockRecord, RefOfs: Word; RefFlags: TReferenceFlagSet);
Var ReferencedUnitRecord: Word;
begin
  ReferencedUnitRecord := AddReferencedModule (Segment);
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 7))^ do
    begin
      RecordType := icReference;
      ReferencedUnitRecordOfs := ReferencedUnitRecord or Word (RefFlags);
      ReferencedBlockRecordOfs := BlockRecord;
      ReferencedOfs := RefOfs;
    end;
end;
```

### Code Block 4

```pascal
Function AddReferencedModule (ReferencedModuleSegment: Word): Word;
Var ReferencedModuleRecord: PReferencedModulesBlockRecord;
    ReferencedModuleRecordOfs: Word absolute ReferencedModuleRecord;
    UnitNameIdentifier: PIdentifier;
begin
  ReferencedModuleRecord := Ptr (SymbolTable [stReferencedModules].Segment, 0);
  While ReferencedModuleRecordOfs <> SymbolTable [stReferencedModules].NextRecordOffset do
    begin
      If ReferencedModuleRecord^.ModuleSegment = ReferencedModuleSegment then
        begin
          AddReferencedModule := ReferencedModuleRecordOfs;
          Exit;
        end;
      Inc (ReferencedModuleRecordOfs, ReferencedModuleRecord^.UnitName.Len + 5);
    end;
  If ReferencedModuleRecordOfs >= $1000 then Error (TooManySymbols);
  UnitNameIdentifier :=                      { ReferencedModuleRecord Offset must be < $1000 to add flags in highest nibble }
    PIdentifier (Ptr (ReferencedModuleSegment, PUnitHeader (Ptr (ReferencedModuleSegment, 0))^.UnitNameIdentifierOffset));
  ReferencedModuleRecord := IncreaseSymbolTable (stReferencedModules, UnitNameIdentifier^.Name.Len + 5);
  With ReferencedModuleRecord^ do
    begin
      ModuleSegment := ReferencedModuleSegment;
      rmrW2 := 0;
      UnitName.Str := UnitNameIdentifier^.Name.Str;
    end;
  AddReferencedModule := Ofs (ReferencedModuleRecord^);
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/creating-references
- https://turbopascal.org/expressions
- https://turbopascal.org/intermediate-code
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

