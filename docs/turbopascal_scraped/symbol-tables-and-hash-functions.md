# Symbol Tables and Hash Functions - Turbo Pascal Compiler

**URL:** https://turbopascal.org/symbol-tables-and-hash-functions

---

## Table of Contents

- Symbol Tables and Hash Functions

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Symbol Tables and Hash Functions
Every compiler stores data in different symbol tables. Symbol table is a special data structure that is designed for easy insertion of new symbol with associated data and quick search through all elements in the table.
Turbo Pascal uses many
symbol tables
. Main symbol table stores all identifiers with their associated data while other tables store data created and needed during compilation process. Unit file (
tpu
extension) is a collection of all symol tables that are created after unit source file compilation.
Type TSymbolTable = (
    stMain,
    stProcedures,
    stCodeBlocks,
    stTypedConstantsBlocks,
    stVariablesBlocks,
    st5,
    stReferencedModules,
    stUsedFiles,
    stSourceLines,
    stSourceLineCodeOffsets,
    stTemporary,
    st11,
    stCode,
    stTypedConstants,
    stCodeReferences,
    stTypedConstantsReferences,
    st16,
    st17,
    st18,
    stIntermediateCode);
Identifiers are stored
in one-way linked lists. Each list item (identifier with data) has a link to the next item. This link is relative offset so the link does not depend on the table address. To speed up the search process hash function is used. For each identifier a hash value based on the case insensitive name characters is calculated:
Procedure CopyStringToCurrentIdentifier (Const Str: String);
Var N: Byte;
begin
  CurrentIdentifier := Str;
  CurrentIdentifierHash := - Length (CurrentIdentifier);
  For N := 1 to Length (CurrentIdentifier) do
    Inc (CurrentIdentifierHash, Byte (CurrentIdentifier [N]) and $DF);
  CurrentIdentifierHash := CurrentIdentifierHash shl 1;
end;
Last few bits of this hash value (symbol table Mask parameter) are used to define one linked list for this identifier. Instead of searching through all identifiers, only one shorter list with identifiers that have particular hash value has to be searched. Therefore, each identifier symbol table consists of many linked lists.
Type
    PIdentifierList = ^TIdentifierList;
    TIdentifierList = Record
                        Mask: Word;
                        Offset: Array [0..255] of Word;
                      end;

    PIdentifier = ^TIdentifier;
    TIdentifier = Record
                    Next: Word;
                    Token: TToken;
                    Name: Str64Rec;
                  end;
Each identifier has some additional data according to its type which immediately follows identifier name in the symbol table:
Type
    TUnitIdentifierFlags = (UsedInInterface);
    TUnitIdentifierFlagsSet = Set of TUnitIdentifierFlags;

    PUnitIdentifierData = ^TUnitIdentifierData;
    TUnitIdentifierData = Record
                            UnitSegment: Word;
                            PublicIdentifiersChecksum: Word;
                            NextUnitIdentifier: Word;
                            CurrentUsedUnitIdentifier: Word;
                            UnitIdentifierFlags: TUnitIdentifierFlagsSet;
                          end;

    PConstantIdentifierData = ^TConstantIdentifierData;
    TConstantIdentifierData = Record
                                UnitTypeOffsets: TUnitOffsets;
                                Case Byte of
                                  0: (Value: Byte);                    { Constant value (1..n bytes }
                                  1: (OrdinalValue, W2, W4, W6: Word); { Enumerated item }
                              end;

    PVariableIdentifierData = ^TVariableIdentifierData;
    TVariableIdentifierData = Record
                                Flags: TVarFlagsSet;
                                Case Byte of
                                  0: (W1: PtrRec; W5: Word; UnitTypeOffsets: TUnitOffsets;); { Field:    Ofs = Offset of field }
                                  1: (AbsoluteVarDataOffsets: TUnitOffsets);
                              end;

    PTypeIdentifierData = ^TTypeIdentifierData;
    TTypeIdentifierData = Record
                            UnitTypeOffsets: TUnitOffsets;
                          end;

    TProcedureFlags = (pfFar, pfInline, pfInterrupt, pfExternal, pfMethod, pfConstructor, pfDestructor, 
      pfAssembler, pf0100, pf0200, pfDynamic, pfStackFrame);

    TProcedureFlagsSet = Set of TProcedureFlags;

    PProcedureIdentifierData = ^TProcedureIdentifierData;
    TProcedureIdentifierData = Record
                                 Flags: TProcedureFlagsSet;
                                 ProceduresRecordOffset: Word;        { INLINE CodeSize }
                                 OuterBlockProcedureIdentifier: Word; { also ancestor method, ancestor object TypeDef }
                                 LocalIdentifiersList: Word;
                                 W8: Word;                            { Method VMT Table Offset, Dynamic Method Index }
                                 ProcedureTypeDefinition: TProcedureTypeDefinition;
                               end;

    PProcedureParameterData = ^TProcedureParameterData;
    TProcedureParameterData = Record
                                UnitTypeOffsets: TUnitOffsets;
                                VarFlags: TVarFlagsSet;
                              end;

    PLabelIdentifierData = ^TLabelIdentifierData;
    TLabelIdentifierData = Record
                             LabelRecordOffset: Word;
                           end;

    PSystemProcedureIdentifierData = ^TSystemProcedureIdentifierData;
    TSystemProcedureIdentifierData = Record
                                       ProcData: Word;
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
Type TSymbolTable = (
    stMain,
    stProcedures,
    stCodeBlocks,
    stTypedConstantsBlocks,
    stVariablesBlocks,
    st5,
    stReferencedModules,
    stUsedFiles,
    stSourceLines,
    stSourceLineCodeOffsets,
    stTemporary,
    st11,
    stCode,
    stTypedConstants,
    stCodeReferences,
    stTypedConstantsReferences,
    st16,
    st17,
    st18,
    stIntermediateCode);
```

### Code Block 2

```pascal
Procedure CopyStringToCurrentIdentifier (Const Str: String);
Var N: Byte;
begin
  CurrentIdentifier := Str;
  CurrentIdentifierHash := - Length (CurrentIdentifier);
  For N := 1 to Length (CurrentIdentifier) do
    Inc (CurrentIdentifierHash, Byte (CurrentIdentifier [N]) and $DF);
  CurrentIdentifierHash := CurrentIdentifierHash shl 1;
end;
```

### Code Block 3

```pascal
Type
    PIdentifierList = ^TIdentifierList;
    TIdentifierList = Record
                        Mask: Word;
                        Offset: Array [0..255] of Word;
                      end;

    PIdentifier = ^TIdentifier;
    TIdentifier = Record
                    Next: Word;
                    Token: TToken;
                    Name: Str64Rec;
                  end;
```

### Code Block 4

```pascal
Type
    TUnitIdentifierFlags = (UsedInInterface);
    TUnitIdentifierFlagsSet = Set of TUnitIdentifierFlags;

    PUnitIdentifierData = ^TUnitIdentifierData;
    TUnitIdentifierData = Record
                            UnitSegment: Word;
                            PublicIdentifiersChecksum: Word;
                            NextUnitIdentifier: Word;
                            CurrentUsedUnitIdentifier: Word;
                            UnitIdentifierFlags: TUnitIdentifierFlagsSet;
                          end;

    PConstantIdentifierData = ^TConstantIdentifierData;
    TConstantIdentifierData = Record
                                UnitTypeOffsets: TUnitOffsets;
                                Case Byte of
                                  0: (Value: Byte);                    { Constant value (1..n bytes }
                                  1: (OrdinalValue, W2, W4, W6: Word); { Enumerated item }
                              end;

    PVariableIdentifierData = ^TVariableIdentifierData;
    TVariableIdentifierData = Record
                                Flags: TVarFlagsSet;
                                Case Byte of
                                  0: (W1: PtrRec; W5: Word; UnitTypeOffsets: TUnitOffsets;); { Field:    Ofs = Offset of field }
                                  1: (AbsoluteVarDataOffsets: TUnitOffsets);
                              end;

    PTypeIdentifierData = ^TTypeIdentifierData;
    TTypeIdentifierData = Record
                            UnitTypeOffsets: TUnitOffsets;
                          end;

    TProcedureFlags = (pfFar, pfInline, pfInterrupt, pfExternal, pfMethod, pfConstructor, pfDestructor, 
      pfAssembler, pf0100, pf0200, pfDynamic, pfStackFrame);

    TProcedureFlagsSet = Set of TProcedureFlags;

    PProcedureIdentifierData = ^TProcedureIdentifierData;
    TProcedureIdentifierData = Record
                                 Flags: TProcedureFlagsSet;
                                 ProceduresRecordOffset: Word;        { INLINE CodeSize }
                                 OuterBlockProcedureIdentifier: Word; { also ancestor method, ancestor object TypeDef }
                                 LocalIdentifiersList: Word;
                                 W8: Word;                            { Method VMT Table Offset, Dynamic Method Index }
                                 ProcedureTypeDefinition: TProcedureTypeDefinition;
                               end;

    PProcedureParameterData = ^TProcedureParameterData;
    TProcedureParameterData = Record
                                UnitTypeOffsets: TUnitOffsets;
                                VarFlags: TVarFlagsSet;
                              end;

    PLabelIdentifierData = ^TLabelIdentifierData;
    TLabelIdentifierData = Record
                             LabelRecordOffset: Word;
                           end;

    PSystemProcedureIdentifierData = ^TSystemProcedureIdentifierData;
    TSystemProcedureIdentifierData = Record
                                       ProcData: Word;
                                     end;
```

### Code Block 5

```pascal
Type TSymbolTable = (
    stMain,
    stProcedures,
    stCodeBlocks,
    stTypedConstantsBlocks,
    stVariablesBlocks,
    st5,
    stReferencedModules,
    stUsedFiles,
    stSourceLines,
    stSourceLineCodeOffsets,
    stTemporary,
    st11,
    stCode,
    stTypedConstants,
    stCodeReferences,
    stTypedConstantsReferences,
    st16,
    st17,
    st18,
    stIntermediateCode);
```

### Code Block 6

```pascal
Procedure CopyStringToCurrentIdentifier (Const Str: String);
Var N: Byte;
begin
  CurrentIdentifier := Str;
  CurrentIdentifierHash := - Length (CurrentIdentifier);
  For N := 1 to Length (CurrentIdentifier) do
    Inc (CurrentIdentifierHash, Byte (CurrentIdentifier [N]) and $DF);
  CurrentIdentifierHash := CurrentIdentifierHash shl 1;
end;
```

### Code Block 7

```pascal
Type
    PIdentifierList = ^TIdentifierList;
    TIdentifierList = Record
                        Mask: Word;
                        Offset: Array [0..255] of Word;
                      end;

    PIdentifier = ^TIdentifier;
    TIdentifier = Record
                    Next: Word;
                    Token: TToken;
                    Name: Str64Rec;
                  end;
```

### Code Block 8

```pascal
Type
    TUnitIdentifierFlags = (UsedInInterface);
    TUnitIdentifierFlagsSet = Set of TUnitIdentifierFlags;

    PUnitIdentifierData = ^TUnitIdentifierData;
    TUnitIdentifierData = Record
                            UnitSegment: Word;
                            PublicIdentifiersChecksum: Word;
                            NextUnitIdentifier: Word;
                            CurrentUsedUnitIdentifier: Word;
                            UnitIdentifierFlags: TUnitIdentifierFlagsSet;
                          end;

    PConstantIdentifierData = ^TConstantIdentifierData;
    TConstantIdentifierData = Record
                                UnitTypeOffsets: TUnitOffsets;
                                Case Byte of
                                  0: (Value: Byte);                    { Constant value (1..n bytes }
                                  1: (OrdinalValue, W2, W4, W6: Word); { Enumerated item }
                              end;

    PVariableIdentifierData = ^TVariableIdentifierData;
    TVariableIdentifierData = Record
                                Flags: TVarFlagsSet;
                                Case Byte of
                                  0: (W1: PtrRec; W5: Word; UnitTypeOffsets: TUnitOffsets;); { Field:    Ofs = Offset of field }
                                  1: (AbsoluteVarDataOffsets: TUnitOffsets);
                              end;

    PTypeIdentifierData = ^TTypeIdentifierData;
    TTypeIdentifierData = Record
                            UnitTypeOffsets: TUnitOffsets;
                          end;

    TProcedureFlags = (pfFar, pfInline, pfInterrupt, pfExternal, pfMethod, pfConstructor, pfDestructor, 
      pfAssembler, pf0100, pf0200, pfDynamic, pfStackFrame);

    TProcedureFlagsSet = Set of TProcedureFlags;

    PProcedureIdentifierData = ^TProcedureIdentifierData;
    TProcedureIdentifierData = Record
                                 Flags: TProcedureFlagsSet;
                                 ProceduresRecordOffset: Word;        { INLINE CodeSize }
                                 OuterBlockProcedureIdentifier: Word; { also ancestor method, ancestor object TypeDef }
                                 LocalIdentifiersList: Word;
                                 W8: Word;                            { Method VMT Table Offset, Dynamic Method Index }
                                 ProcedureTypeDefinition: TProcedureTypeDefinition;
                               end;

    PProcedureParameterData = ^TProcedureParameterData;
    TProcedureParameterData = Record
                                UnitTypeOffsets: TUnitOffsets;
                                VarFlags: TVarFlagsSet;
                              end;

    PLabelIdentifierData = ^TLabelIdentifierData;
    TLabelIdentifierData = Record
                             LabelRecordOffset: Word;
                           end;

    PSystemProcedureIdentifierData = ^TSystemProcedureIdentifierData;
    TSystemProcedureIdentifierData = Record
                                       ProcData: Word;
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
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/storing-identifiers-into-symbol-table
- https://turbopascal.org/symbol-table-management
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/symbol-tables-and-hash-functions
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

