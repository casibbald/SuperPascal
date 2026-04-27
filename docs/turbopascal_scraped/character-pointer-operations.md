# Character Pointer Operations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/character-pointer-operations

---

## Table of Contents

- Character Pointer Operations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Character Pointer Operations
This procedure checks if operation is
PChar
± Integer or
PChar
±
PChar
. If so, it generates code and returns
True
otherwise it returns
False
.
Function CharacterPointerOperations: Boolean;

    Function IsOperation_PChar_Integer (Var LeftExpression, RightExpression: TExpression): Boolean;
    begin
      IsOperation_PChar_Integer := False;
      If LeftExpression.TypeDefPtr <> Ptr (SystemUnitSegment, PChar_TypeOffset) then Exit;
      IsOperation_PChar_Integer := RightExpression.TypeDefPtr^.BaseType = btInteger;
    end;

    Procedure CharacterPointerOperation;
    begin
      CharacterPointerOperations := True;
      If (RightExpression.Location = elConstant) and ((LeftExpression.Location = elConstant) or
         (LeftExpression.Location = elPointerToMemory) and not (efSegment in LeftExpression.LocationData.Flags)) then
           begin
             Case Operation of
               Calc_Add: Inc (LeftExpression.Value.Word, RightExpression.Value.Word);
               Calc_Subtract: Dec (LeftExpression.Value.Word, RightExpression.Value.Word);
             end;
             Exit;
           end;
      LeftExpression.LoadPointerToMemoryTo_DX_AX;
      RightExpression.LoadPointerToMemoryTo_DX_AX;
      RightExpression.ExtendInteger ([it16Bit, itUnsigned]);
      LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urDX, urAX]);
      Case Operation of
        Calc_Add: RightExpression.GenerateArithmeticInstructionWith_ACC (mod_ADD_rm);
        Calc_Subtract: RightExpression.GenerateArithmeticInstructionWith_ACC (mod_SUB_rm);
      end;
      LeftExpression.EndIntermediateCodeSubroutine;
      LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
    end;

  begin
    CharacterPointerOperations := False;
    LeftExpression.CheckForConversionOfZeroBasedCharacterArrayToPChar (RightExpression.TypeDefPtr);
    RightExpression.CheckForConversionOfZeroBasedCharacterArrayToPChar (LeftExpression.TypeDefPtr);
    Case Operation of
      Calc_Subtract: If IsOperation_PChar_Integer (LeftExpression, RightExpression) then CharacterPointerOperation;
      Calc_Add: begin
                  If IsOperation_PChar_Integer (RightExpression, LeftExpression) then ExchangeLeftAndRightExpression;
                  If IsOperation_PChar_Integer (LeftExpression, RightExpression) then CharacterPointerOperation;
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
  Function CharacterPointerOperations: Boolean;

    Function IsOperation_PChar_Integer (Var LeftExpression, RightExpression: TExpression): Boolean;
    begin
      IsOperation_PChar_Integer := False;
      If LeftExpression.TypeDefPtr <> Ptr (SystemUnitSegment, PChar_TypeOffset) then Exit;
      IsOperation_PChar_Integer := RightExpression.TypeDefPtr^.BaseType = btInteger;
    end;

    Procedure CharacterPointerOperation;
    begin
      CharacterPointerOperations := True;
      If (RightExpression.Location = elConstant) and ((LeftExpression.Location = elConstant) or
         (LeftExpression.Location = elPointerToMemory) and not (efSegment in LeftExpression.LocationData.Flags)) then
           begin
             Case Operation of
               Calc_Add: Inc (LeftExpression.Value.Word, RightExpression.Value.Word);
               Calc_Subtract: Dec (LeftExpression.Value.Word, RightExpression.Value.Word);
             end;
             Exit;
           end;
      LeftExpression.LoadPointerToMemoryTo_DX_AX;
      RightExpression.LoadPointerToMemoryTo_DX_AX;
      RightExpression.ExtendInteger ([it16Bit, itUnsigned]);
      LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urDX, urAX]);
      Case Operation of
        Calc_Add: RightExpression.GenerateArithmeticInstructionWith_ACC (mod_ADD_rm);
        Calc_Subtract: RightExpression.GenerateArithmeticInstructionWith_ACC (mod_SUB_rm);
      end;
      LeftExpression.EndIntermediateCodeSubroutine;
      LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
    end;

  begin
    CharacterPointerOperations := False;
    LeftExpression.CheckForConversionOfZeroBasedCharacterArrayToPChar (RightExpression.TypeDefPtr);
    RightExpression.CheckForConversionOfZeroBasedCharacterArrayToPChar (LeftExpression.TypeDefPtr);
    Case Operation of
      Calc_Subtract: If IsOperation_PChar_Integer (LeftExpression, RightExpression) then CharacterPointerOperation;
      Calc_Add: begin
                  If IsOperation_PChar_Integer (RightExpression, LeftExpression) then ExchangeLeftAndRightExpression;
                  If IsOperation_PChar_Integer (LeftExpression, RightExpression) then CharacterPointerOperation;
                end;
    end;
  end;
```

### Code Block 2

```pascal
  Function CharacterPointerOperations: Boolean;

    Function IsOperation_PChar_Integer (Var LeftExpression, RightExpression: TExpression): Boolean;
    begin
      IsOperation_PChar_Integer := False;
      If LeftExpression.TypeDefPtr <> Ptr (SystemUnitSegment, PChar_TypeOffset) then Exit;
      IsOperation_PChar_Integer := RightExpression.TypeDefPtr^.BaseType = btInteger;
    end;

    Procedure CharacterPointerOperation;
    begin
      CharacterPointerOperations := True;
      If (RightExpression.Location = elConstant) and ((LeftExpression.Location = elConstant) or
         (LeftExpression.Location = elPointerToMemory) and not (efSegment in LeftExpression.LocationData.Flags)) then
           begin
             Case Operation of
               Calc_Add: Inc (LeftExpression.Value.Word, RightExpression.Value.Word);
               Calc_Subtract: Dec (LeftExpression.Value.Word, RightExpression.Value.Word);
             end;
             Exit;
           end;
      LeftExpression.LoadPointerToMemoryTo_DX_AX;
      RightExpression.LoadPointerToMemoryTo_DX_AX;
      RightExpression.ExtendInteger ([it16Bit, itUnsigned]);
      LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urDX, urAX]);
      Case Operation of
        Calc_Add: RightExpression.GenerateArithmeticInstructionWith_ACC (mod_ADD_rm);
        Calc_Subtract: RightExpression.GenerateArithmeticInstructionWith_ACC (mod_SUB_rm);
      end;
      LeftExpression.EndIntermediateCodeSubroutine;
      LeftExpression.UsedRegisters := LeftExpression.UsedRegisters + RightExpression.UsedRegisters;
    end;

  begin
    CharacterPointerOperations := False;
    LeftExpression.CheckForConversionOfZeroBasedCharacterArrayToPChar (RightExpression.TypeDefPtr);
    RightExpression.CheckForConversionOfZeroBasedCharacterArrayToPChar (LeftExpression.TypeDefPtr);
    Case Operation of
      Calc_Subtract: If IsOperation_PChar_Integer (LeftExpression, RightExpression) then CharacterPointerOperation;
      Calc_Add: begin
                  If IsOperation_PChar_Integer (RightExpression, LeftExpression) then ExchangeLeftAndRightExpression;
                  If IsOperation_PChar_Integer (LeftExpression, RightExpression) then CharacterPointerOperation;
                end;
    end;
  end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/character-pointer-operations
- https://turbopascal.org/code-generator
- https://turbopascal.org/compiler
- https://turbopascal.org/expressions
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

