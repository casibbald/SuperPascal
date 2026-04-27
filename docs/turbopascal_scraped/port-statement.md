# Port Statement - Turbo Pascal Compiler

**URL:** https://turbopascal.org/port-statement

---

## Table of Contents

- Port Statement

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Port Statement
This procedure processes
Port
and
PortW
statements. This stetement accesses I/O address space and generates
OUT
instruction. Reading from
Port
and
PortW
is done at
expression processing
.
Procedure TStatement.ProcessPortStatement;
Var PortAddress, PortData: TExpression;
    OutOpCode: Byte;
begin
  GetNextToken;
  OutOpCode := Byte (CurrentIdentifierDataPtr^) + $E6;  { Port = 0, PortW = 1 => OUT_Address_AX }
  ExpectTokenAndGetNext (Token_LeftBracket);
  PortAddress.ExpectIntegerExpression;
  PortAddress.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_RightBracket);
  ExpectTokenAndGetNext (Token_Assignment);
  PortData.ExpectIntegerExpression;
  Case OutOpCode and $01 <> 0 of
    True: PortData.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
    else PortData.CheckOrdinalRange (Ptr (SystemUnitSegment, Byte_TypeOffset));
  end;
  If (PortAddress.Location = elConstant) and (PortAddress.Value.Byte1 = 0) then
    begin
      PortData.Calculate;
      PortData.LoadExpressionToRegisters (urAX);
      GenerateInstruction_TwoBytes (OutOpCode, PortAddress.Value.Byte);
    end else begin
               CalculateExpressionsAndLoadLeftTo_rAX_AndRightTo_rDX (PortData, PortAddress);
               GenerateInstruction_Byte (OutOpCode or $08);
             end;

  StatementCode := EndSubroutine;
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
Procedure TStatement.ProcessPortStatement;
Var PortAddress, PortData: TExpression;
    OutOpCode: Byte;
begin
  GetNextToken;
  OutOpCode := Byte (CurrentIdentifierDataPtr^) + $E6;  { Port = 0, PortW = 1 => OUT_Address_AX }
  ExpectTokenAndGetNext (Token_LeftBracket);
  PortAddress.ExpectIntegerExpression;
  PortAddress.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_RightBracket);
  ExpectTokenAndGetNext (Token_Assignment);
  PortData.ExpectIntegerExpression;
  Case OutOpCode and $01 <> 0 of
    True: PortData.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
    else PortData.CheckOrdinalRange (Ptr (SystemUnitSegment, Byte_TypeOffset));
  end;
  If (PortAddress.Location = elConstant) and (PortAddress.Value.Byte1 = 0) then
    begin
      PortData.Calculate;
      PortData.LoadExpressionToRegisters (urAX);
      GenerateInstruction_TwoBytes (OutOpCode, PortAddress.Value.Byte);
    end else begin
               CalculateExpressionsAndLoadLeftTo_rAX_AndRightTo_rDX (PortData, PortAddress);
               GenerateInstruction_Byte (OutOpCode or $08);
             end;

  StatementCode := EndSubroutine;
end;
```

### Code Block 2

```pascal
Procedure TStatement.ProcessPortStatement;
Var PortAddress, PortData: TExpression;
    OutOpCode: Byte;
begin
  GetNextToken;
  OutOpCode := Byte (CurrentIdentifierDataPtr^) + $E6;  { Port = 0, PortW = 1 => OUT_Address_AX }
  ExpectTokenAndGetNext (Token_LeftBracket);
  PortAddress.ExpectIntegerExpression;
  PortAddress.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
  ExpectTokenAndGetNext (Token_RightBracket);
  ExpectTokenAndGetNext (Token_Assignment);
  PortData.ExpectIntegerExpression;
  Case OutOpCode and $01 <> 0 of
    True: PortData.CheckOrdinalRange (Ptr (SystemUnitSegment, Word_TypeOffset));
    else PortData.CheckOrdinalRange (Ptr (SystemUnitSegment, Byte_TypeOffset));
  end;
  If (PortAddress.Location = elConstant) and (PortAddress.Value.Byte1 = 0) then
    begin
      PortData.Calculate;
      PortData.LoadExpressionToRegisters (urAX);
      GenerateInstruction_TwoBytes (OutOpCode, PortAddress.Value.Byte);
    end else begin
               CalculateExpressionsAndLoadLeftTo_rAX_AndRightTo_rDX (PortData, PortAddress);
               GenerateInstruction_Byte (OutOpCode or $08);
             end;

  StatementCode := EndSubroutine;
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
- https://turbopascal.org/port-statement
- https://turbopascal.org/processing-expressions
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

