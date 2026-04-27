# Intermediate Code - Turbo Pascal Compiler

**URL:** https://turbopascal.org/intermediate-code

---

## Table of Contents

- Intermediate Code

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Intermediate Code
Most compilers use some kind of intermediate code which is a program representation somewhere betwen the high-level source code and executable code for target processor.
This means that the compiler does not generate the target code directly but it generates some intermediate pseudo instructions which are converted into target code after the program is compiled. There are many reasons to use intermediate code. In most cases it is not possible to generate the final instructions directly because the code is not generated linearly as is seen in the compiled program and many compiler optimizations are possible at the level of intermediate code before the final target code is generated.
Turbo Pascal uses a simple version of low-level intermediate code. Intermediate code is stored as records in a dedicated symbol table. This code can contain subroutines as there are
GoSub
and
Return
instructions. Turbo Pascal uses heavily this approach as the easiest way to generate code is to process some element, generate code for it as intermediate code subroutine and then generate the rest of the code before and after this subroutine. This way complex expressions can be compiled and very effective code can be generated.
Expression
and
Statement
objects which generate most of the code have a field which holds offset in the intermediate code table. This is in fact a link to the intermediate code subroutine for the expression or statement.
Type TicRecordType = (
      icGoSub,
      ic01,
      icReturn,
      ic03,
      icByte,
      ic05,
      icWord,
      ic07,
      icJumpNear,
      ic09,
      icJumpShort,
      ic0B,
      icLabel,
      ic0D,
      icReference,
      ic0F,
      icReferenceAsmBlock,
      ic11,
      ic12,
      ic13,
      icSourceLineNumber,
      ic15,
      icLoad_ES_DI,
      ic17,
      ic_ES_DI_Loaded,
      ic19,
      ic_ES_DI_Destroyed,
      ic1B,
      icSkippedGoSubSaveAddress,
      ic1D,
      icEnd);
Turbo Pascal uses intermediate code instructions for all target instructions.
References
are processed separately and use own intermediate code record.
Jumps and labels
also have own intermediate code records. Jump records are optimized to generate short jump instructions whenever possible. Source line numbers are also stored as records in the intermediate code.
Loading of register pair ES:DI
is done with dedicated intermediate code instructions to prevent redundant loading. This is another compiler optimizations that Turbo Pascal does at the intermediate code level. Most intermediate code records have additional fields to store some data.
Type PIntermediateCodeRecord = ^TIntermediateCodeRecord;
     TIntermediateCodeRecord = Record
                                Case RecordType: TicRecordType of
                                  icGoSub:                   (NewOffset: Word);
                                  icReturn:                  ();
                                  icByte:                    (ByteCode: Byte);
                                  icWord:                    (WordCode: WordRec);
                                  icJumpNear:                (JumpCode: Byte; LabelIdentifierOffset, Next_GOTO_Record,
                                                              Previous_icJumpNear: Word);
                                  icJumpShort:               (JumpOpCode: Byte; LabelRecordOffset, JumpCodeOffset: Word);
                                  icJumpNear:                (JC: Byte; PreviousJumpRecord: Word);
                                  icLabel:                   (LabelAddress: Word);
                                  icReference:               (ReferencedUnitRecordOfs, ReferencedBlockRecordOfs,
                                                              ReferencedOfs: Word);
                                  icReferenceAsmBlock:       (Flags: TReferenceFlagSet; LabelIdentifierOfs, W5_10: Word);
                                  icSourceLineNumber:        (StatementLineNumber: Word);
                                  icLoad_ES_DI,
                                  ic_ES_DI_Loaded:           (DestinationProcedure: Word; Displacement: Integer;
                                                              SaveAddressCode: Word);
                                  ic_ES_DI_Destroyed:        ();
                                  icSkippedGoSubSaveAddress: (SaveAddress: Word);
                              end;
Turbo Pascal compiler uses two important procedures for intermediate code generation. After some intermediate code has been generated and compiler wants to finish the subroutine it calles this procedure. It sets the intermediate code field to the offset of last intermediate code subroutine (or to 0 is no code was generated), generates
Return
intermediate code instruction and resets last intermediate code variable. From now on the expression holds the offset to intermediate code routine (ended with
Return
instruction) and new intermediate code routine can be generated.
Procedure TExpression.EndIntermediateCodeSubroutine;
begin
  IntermediateCodeOffset := EndSubroutine;
end;
Function EndSubroutine: Word;
begin
  EndSubroutine := 0;
  If LastIntermediateCodeSubroutine = SymbolTable [stIntermediateCode].NextRecordOffset then Exit;
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 1))^ do RecordType := icReturn;
  EndSubroutine := LastIntermediateCodeSubroutine;
  LastIntermediateCodeSubroutine := SymbolTable [stIntermediateCode].NextRecordOffset;
end;
This is an excellent example of using intermediate code subroutines to load two expressions to registers.
Procedure LoadBothExpressionsToRegisters (Var LeftExpression, RightExpression: TExpression; RegistersToPreserve: TUsedRegistersSet);
begin
  If RightExpression.UsedRegisters * RegistersToPreserve = [] then
    begin
      LeftExpression.Calculate;
      LeftExpression.LoadExpressionToRegisters (urDX_AX);
      RightExpression.Calculate;
    end else begin
               RightExpression.Calculate;
               RightExpression.Save (RegistersToPreserve + LeftExpression.UsedRegisters);
               LeftExpression.Calculate;
               LeftExpression.LoadExpressionToRegisters (urDX_AX);
               RightExpression.PopToRegisters (RegistersToPreserve);
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
Type TicRecordType = (
      icGoSub,
      ic01,
      icReturn,
      ic03,
      icByte,
      ic05,
      icWord,
      ic07,
      icJumpNear,
      ic09,
      icJumpShort,
      ic0B,
      icLabel,
      ic0D,
      icReference,
      ic0F,
      icReferenceAsmBlock,
      ic11,
      ic12,
      ic13,
      icSourceLineNumber,
      ic15,
      icLoad_ES_DI,
      ic17,
      ic_ES_DI_Loaded,
      ic19,
      ic_ES_DI_Destroyed,
      ic1B,
      icSkippedGoSubSaveAddress,
      ic1D,
      icEnd);
```

### Code Block 2

```pascal
Type PIntermediateCodeRecord = ^TIntermediateCodeRecord;
     TIntermediateCodeRecord = Record
                                Case RecordType: TicRecordType of
                                  icGoSub:                   (NewOffset: Word);
                                  icReturn:                  ();
                                  icByte:                    (ByteCode: Byte);
                                  icWord:                    (WordCode: WordRec);
                                  icJumpNear:                (JumpCode: Byte; LabelIdentifierOffset, Next_GOTO_Record,
                                                              Previous_icJumpNear: Word);
                                  icJumpShort:               (JumpOpCode: Byte; LabelRecordOffset, JumpCodeOffset: Word);
                                  icJumpNear:                (JC: Byte; PreviousJumpRecord: Word);
                                  icLabel:                   (LabelAddress: Word);
                                  icReference:               (ReferencedUnitRecordOfs, ReferencedBlockRecordOfs,
                                                              ReferencedOfs: Word);
                                  icReferenceAsmBlock:       (Flags: TReferenceFlagSet; LabelIdentifierOfs, W5_10: Word);
                                  icSourceLineNumber:        (StatementLineNumber: Word);
                                  icLoad_ES_DI,
                                  ic_ES_DI_Loaded:           (DestinationProcedure: Word; Displacement: Integer;
                                                              SaveAddressCode: Word);
                                  ic_ES_DI_Destroyed:        ();
                                  icSkippedGoSubSaveAddress: (SaveAddress: Word);
                              end;
```

### Code Block 3

```pascal
Procedure TExpression.EndIntermediateCodeSubroutine;
begin
  IntermediateCodeOffset := EndSubroutine;
end;
```

### Code Block 4

```pascal
Function EndSubroutine: Word;
begin
  EndSubroutine := 0;
  If LastIntermediateCodeSubroutine = SymbolTable [stIntermediateCode].NextRecordOffset then Exit;
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 1))^ do RecordType := icReturn;
  EndSubroutine := LastIntermediateCodeSubroutine;
  LastIntermediateCodeSubroutine := SymbolTable [stIntermediateCode].NextRecordOffset;
end;
```

### Code Block 5

```pascal
Procedure LoadBothExpressionsToRegisters (Var LeftExpression, RightExpression: TExpression; RegistersToPreserve: TUsedRegistersSet);
begin
  If RightExpression.UsedRegisters * RegistersToPreserve = [] then
    begin
      LeftExpression.Calculate;
      LeftExpression.LoadExpressionToRegisters (urDX_AX);
      RightExpression.Calculate;
    end else begin
               RightExpression.Calculate;
               RightExpression.Save (RegistersToPreserve + LeftExpression.UsedRegisters);
               LeftExpression.Calculate;
               LeftExpression.LoadExpressionToRegisters (urDX_AX);
               RightExpression.PopToRegisters (RegistersToPreserve);
             end;
end;
```

### Code Block 6

```pascal
Type TicRecordType = (
      icGoSub,
      ic01,
      icReturn,
      ic03,
      icByte,
      ic05,
      icWord,
      ic07,
      icJumpNear,
      ic09,
      icJumpShort,
      ic0B,
      icLabel,
      ic0D,
      icReference,
      ic0F,
      icReferenceAsmBlock,
      ic11,
      ic12,
      ic13,
      icSourceLineNumber,
      ic15,
      icLoad_ES_DI,
      ic17,
      ic_ES_DI_Loaded,
      ic19,
      ic_ES_DI_Destroyed,
      ic1B,
      icSkippedGoSubSaveAddress,
      ic1D,
      icEnd);
```

### Code Block 7

```pascal
Type PIntermediateCodeRecord = ^TIntermediateCodeRecord;
     TIntermediateCodeRecord = Record
                                Case RecordType: TicRecordType of
                                  icGoSub:                   (NewOffset: Word);
                                  icReturn:                  ();
                                  icByte:                    (ByteCode: Byte);
                                  icWord:                    (WordCode: WordRec);
                                  icJumpNear:                (JumpCode: Byte; LabelIdentifierOffset, Next_GOTO_Record,
                                                              Previous_icJumpNear: Word);
                                  icJumpShort:               (JumpOpCode: Byte; LabelRecordOffset, JumpCodeOffset: Word);
                                  icJumpNear:                (JC: Byte; PreviousJumpRecord: Word);
                                  icLabel:                   (LabelAddress: Word);
                                  icReference:               (ReferencedUnitRecordOfs, ReferencedBlockRecordOfs,
                                                              ReferencedOfs: Word);
                                  icReferenceAsmBlock:       (Flags: TReferenceFlagSet; LabelIdentifierOfs, W5_10: Word);
                                  icSourceLineNumber:        (StatementLineNumber: Word);
                                  icLoad_ES_DI,
                                  ic_ES_DI_Loaded:           (DestinationProcedure: Word; Displacement: Integer;
                                                              SaveAddressCode: Word);
                                  ic_ES_DI_Destroyed:        ();
                                  icSkippedGoSubSaveAddress: (SaveAddress: Word);
                              end;
```

### Code Block 8

```pascal
Procedure TExpression.EndIntermediateCodeSubroutine;
begin
  IntermediateCodeOffset := EndSubroutine;
end;
```

### Code Block 9

```pascal
Function EndSubroutine: Word;
begin
  EndSubroutine := 0;
  If LastIntermediateCodeSubroutine = SymbolTable [stIntermediateCode].NextRecordOffset then Exit;
  With PIntermediateCodeRecord (IncreaseSymbolTable (stIntermediateCode, 1))^ do RecordType := icReturn;
  EndSubroutine := LastIntermediateCodeSubroutine;
  LastIntermediateCodeSubroutine := SymbolTable [stIntermediateCode].NextRecordOffset;
end;
```

### Code Block 10

```pascal
Procedure LoadBothExpressionsToRegisters (Var LeftExpression, RightExpression: TExpression; RegistersToPreserve: TUsedRegistersSet);
begin
  If RightExpression.UsedRegisters * RegistersToPreserve = [] then
    begin
      LeftExpression.Calculate;
      LeftExpression.LoadExpressionToRegisters (urDX_AX);
      RightExpression.Calculate;
    end else begin
               RightExpression.Calculate;
               RightExpression.Save (RegistersToPreserve + LeftExpression.UsedRegisters);
               LeftExpression.Calculate;
               LeftExpression.LoadExpressionToRegisters (urDX_AX);
               RightExpression.PopToRegisters (RegistersToPreserve);
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
- https://turbopascal.org/creating-references
- https://turbopascal.org/es-di-address-loading
- https://turbopascal.org/expression-elements
- https://turbopascal.org/expressions
- https://turbopascal.org/intermediate-code
- https://turbopascal.org/io-utilities
- https://turbopascal.org/jumps-and-labels
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-statements
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

