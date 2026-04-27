# System Functions Ptr, Addr, Ofs and Seg - Turbo Pascal Compiler

**URL:** https://turbopascal.org/system-functions-ptr-addr-ofs-and-seg

---

## Table of Contents

- System Functions Ptr, Addr, Ofs and Seg

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
System Functions Ptr, Addr, Ofs and Seg
These procedures process system function
Ptr
,
Addr
,
Seg
and
Ofs
.
Procedure Func_Ptr; Far;
Var TempExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  TempExpression.ExpectIntegerExpression;
  TempExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_Comma);
  Expression^.ExpectIntegerExpression;
  Expression^.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_RightParenthesis);
  With Expression^ do
    begin
      If (Location = elConstant) and (TempExpression.Location = elConstant) then Value.PtrSegment := TempExpression.Value.Word
        else begin
               CalculateExpressionsAndLoadLeftTo_rAX_AndRightTo_rDX (Expression^, TempExpression);
               Expression^.EndIntermediateCodeSubroutine;
               UsedRegisters := UsedRegisters + TempExpression.UsedRegisters;
             end;
      DataType := itLongInt;
      TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
    end;
end;
This procedure processes system functions
Addr
,
Ofs
and
Seg
.
Procedure Func_Addr_Ofs_Seg; Far;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectVariableReference;
  Expression^.CreatePointerToVariableReference;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  Case FuncParameter of
    0: Expression^.TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
    1: begin
         Expression^.TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
         Expression^.ExtendInteger (itWord);
       end;
    2: begin
         Expression^.SwitchBetweenLoWordAndHiWord (2);
         Expression^.TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
         Expression^.ExtendInteger (itWord);
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
Procedure Func_Ptr; Far;
Var TempExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  TempExpression.ExpectIntegerExpression;
  TempExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_Comma);
  Expression^.ExpectIntegerExpression;
  Expression^.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_RightParenthesis);
  With Expression^ do
    begin
      If (Location = elConstant) and (TempExpression.Location = elConstant) then Value.PtrSegment := TempExpression.Value.Word
        else begin
               CalculateExpressionsAndLoadLeftTo_rAX_AndRightTo_rDX (Expression^, TempExpression);
               Expression^.EndIntermediateCodeSubroutine;
               UsedRegisters := UsedRegisters + TempExpression.UsedRegisters;
             end;
      DataType := itLongInt;
      TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
    end;
end;
```

### Code Block 2

```pascal
Procedure Func_Addr_Ofs_Seg; Far;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectVariableReference;
  Expression^.CreatePointerToVariableReference;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  Case FuncParameter of
    0: Expression^.TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
    1: begin
         Expression^.TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
         Expression^.ExtendInteger (itWord);
       end;
    2: begin
         Expression^.SwitchBetweenLoWordAndHiWord (2);
         Expression^.TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
         Expression^.ExtendInteger (itWord);
       end;
  end;
end;
```

### Code Block 3

```pascal
Procedure Func_Ptr; Far;
Var TempExpression: TExpression;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  TempExpression.ExpectIntegerExpression;
  TempExpression.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_Comma);
  Expression^.ExpectIntegerExpression;
  Expression^.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_RightParenthesis);
  With Expression^ do
    begin
      If (Location = elConstant) and (TempExpression.Location = elConstant) then Value.PtrSegment := TempExpression.Value.Word
        else begin
               CalculateExpressionsAndLoadLeftTo_rAX_AndRightTo_rDX (Expression^, TempExpression);
               Expression^.EndIntermediateCodeSubroutine;
               UsedRegisters := UsedRegisters + TempExpression.UsedRegisters;
             end;
      DataType := itLongInt;
      TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
    end;
end;
```

### Code Block 4

```pascal
Procedure Func_Addr_Ofs_Seg; Far;
begin
  ExpectTokenAndGetNext (Token_LeftParenthesis);
  Expression^.ExpectVariableReference;
  Expression^.CreatePointerToVariableReference;
  ExpectTokenAndGetNext (Token_RightParenthesis);
  Case FuncParameter of
    0: Expression^.TypeDefPtr := Ptr (SystemUnitSegment, Pointer_TypeOffset);
    1: begin
         Expression^.TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
         Expression^.ExtendInteger (itWord);
       end;
    2: begin
         Expression^.SwitchBetweenLoWordAndHiWord (2);
         Expression^.TypeDefPtr := Ptr (SystemUnitSegment, LongInt_TypeOffset);
         Expression^.ExtendInteger (itWord);
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
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
- https://turbopascal.org/symbol-tables
- https://turbopascal.org/system-functions
- https://turbopascal.org/system-functions-ptr-addr-ofs-and-seg
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

