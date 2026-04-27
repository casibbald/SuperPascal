# Processing Assembler Directives - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-assembler-directives

---

## Table of Contents

- Processing Assembler Directives

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Assembler Directives
This function checks and processes assembler directives
DB
,
DW
and
DD
. More then one byte, word or double word can be separated by comma.
Function ProcessAssemblerDirective: Boolean;

  Procedure GenerateDirectiveAsmIntermediateCode;
  Var N: Integer;
  begin
    RightExpression := LeftExpression;
    Case DirectiveSize of
      1: begin
           If LeftExpression^.RelocatableIdentifier <> 0 then AsmError (InvalidCombinationOfOpcodeAndOperands);
           If Length (AsmString) <> 0 then
             begin
               For N := 1 to Length (AsmString) do
                 begin
                   AddAsmIntermediateCodeRecord (acByte, 0, Byte (AsmString [N]), Ord (AsmToken));
                   GenerateAsmStatementIntermediateCode;
                 end;
               Exit;
             end;
           If not RightExpressionNumericConstantAndSize8Bit then AsmError (ConstantOutOfRange)
             else AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte, Ord (AsmToken));
         end;
      2: begin
           If Length (AsmString) > 2 then AsmError (StringConstantExceedsLine); { ??? }
           RightExpression16BitNumericConstant;
           If LeftExpression^.RelocatableIdentifier = 0 then
             begin
               AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte, Ord (AsmToken));
               AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte1, Ord (AsmToken));
               Exit;
             end;
           CurrentRelocatableIdentifier.Long := AsmExpression1.RelocatableIdentifier;
           If LeftExpression^.ReferenceFlags = 0 then
             AddAsmIntermediateCodeRecord (acReference, $01, LeftExpression^.Value.Word, Ord (AsmToken)) else
               AddAsmIntermediateCodeRecord (acReference, LeftExpression^.ReferenceFlags,
                                               LeftExpression^.Value.Word, Ord (AsmToken));
         end;
      else begin
             If Length (AsmString) > 4 then AsmError (StringConstantExceedsLine); { ??? }
             If LeftExpression^.RelocatableIdentifier = 0 then
               begin
                 AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte, Ord (AsmToken));
                 AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte1, Ord (AsmToken));
                 AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte2, Ord (AsmToken));
                 AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte3, Ord (AsmToken));
                 Exit;
               end;
             CurrentRelocatableIdentifier.Long := AsmExpression1.RelocatableIdentifier;
             If LeftExpression^.ReferenceFlags = 0 then
               AddAsmIntermediateCodeRecord (acReference, $03, LeftExpression^.Value.Word, Ord (AsmToken)) else
                 begin
                   AddAsmIntermediateCodeRecord (acReference, LeftExpression^.ReferenceFlags,
                                                 LeftExpression^.Value.Word, Ord (AsmToken));
                   AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Word2, Ord (AsmToken))
                 end;
           end;
    end;
  end;

begin
  If (AsmToken = AsmToken_Instruction) and (InstructionData^.B0 = $EA) then
    begin
      DirectiveSize := InstructionData^.B1;
      If PrefixOpcode [0] <> 0 then AsmError (SyntaxError);
      GetNextAsmToken;
      Repeat
        LeftExpression := @AsmExpression1;
        ProcessAsmExpressionAndCheck;
        GenerateDirectiveAsmIntermediateCode;
        GenerateAsmStatementIntermediateCode;
      until not CheckAndGetNextAsmToken (AsmToken_Comma);
      ExpectAsmTokenAndGetNext (AsmToken_Semicolon);
      ProcessAssemblerDirective := True;
    end else ProcessAssemblerDirective := False;
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
Function ProcessAssemblerDirective: Boolean;

  Procedure GenerateDirectiveAsmIntermediateCode;
  Var N: Integer;
  begin
    RightExpression := LeftExpression;
    Case DirectiveSize of
      1: begin
           If LeftExpression^.RelocatableIdentifier <> 0 then AsmError (InvalidCombinationOfOpcodeAndOperands);
           If Length (AsmString) <> 0 then
             begin
               For N := 1 to Length (AsmString) do
                 begin
                   AddAsmIntermediateCodeRecord (acByte, 0, Byte (AsmString [N]), Ord (AsmToken));
                   GenerateAsmStatementIntermediateCode;
                 end;
               Exit;
             end;
           If not RightExpressionNumericConstantAndSize8Bit then AsmError (ConstantOutOfRange)
             else AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte, Ord (AsmToken));
         end;
      2: begin
           If Length (AsmString) > 2 then AsmError (StringConstantExceedsLine); { ??? }
           RightExpression16BitNumericConstant;
           If LeftExpression^.RelocatableIdentifier = 0 then
             begin
               AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte, Ord (AsmToken));
               AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte1, Ord (AsmToken));
               Exit;
             end;
           CurrentRelocatableIdentifier.Long := AsmExpression1.RelocatableIdentifier;
           If LeftExpression^.ReferenceFlags = 0 then
             AddAsmIntermediateCodeRecord (acReference, $01, LeftExpression^.Value.Word, Ord (AsmToken)) else
               AddAsmIntermediateCodeRecord (acReference, LeftExpression^.ReferenceFlags,
                                               LeftExpression^.Value.Word, Ord (AsmToken));
         end;
      else begin
             If Length (AsmString) > 4 then AsmError (StringConstantExceedsLine); { ??? }
             If LeftExpression^.RelocatableIdentifier = 0 then
               begin
                 AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte, Ord (AsmToken));
                 AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte1, Ord (AsmToken));
                 AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte2, Ord (AsmToken));
                 AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte3, Ord (AsmToken));
                 Exit;
               end;
             CurrentRelocatableIdentifier.Long := AsmExpression1.RelocatableIdentifier;
             If LeftExpression^.ReferenceFlags = 0 then
               AddAsmIntermediateCodeRecord (acReference, $03, LeftExpression^.Value.Word, Ord (AsmToken)) else
                 begin
                   AddAsmIntermediateCodeRecord (acReference, LeftExpression^.ReferenceFlags,
                                                 LeftExpression^.Value.Word, Ord (AsmToken));
                   AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Word2, Ord (AsmToken))
                 end;
           end;
    end;
  end;

begin
  If (AsmToken = AsmToken_Instruction) and (InstructionData^.B0 = $EA) then
    begin
      DirectiveSize := InstructionData^.B1;
      If PrefixOpcode [0] <> 0 then AsmError (SyntaxError);
      GetNextAsmToken;
      Repeat
        LeftExpression := @AsmExpression1;
        ProcessAsmExpressionAndCheck;
        GenerateDirectiveAsmIntermediateCode;
        GenerateAsmStatementIntermediateCode;
      until not CheckAndGetNextAsmToken (AsmToken_Comma);
      ExpectAsmTokenAndGetNext (AsmToken_Semicolon);
      ProcessAssemblerDirective := True;
    end else ProcessAssemblerDirective := False;
end;
```

### Code Block 2

```pascal
Function ProcessAssemblerDirective: Boolean;

  Procedure GenerateDirectiveAsmIntermediateCode;
  Var N: Integer;
  begin
    RightExpression := LeftExpression;
    Case DirectiveSize of
      1: begin
           If LeftExpression^.RelocatableIdentifier <> 0 then AsmError (InvalidCombinationOfOpcodeAndOperands);
           If Length (AsmString) <> 0 then
             begin
               For N := 1 to Length (AsmString) do
                 begin
                   AddAsmIntermediateCodeRecord (acByte, 0, Byte (AsmString [N]), Ord (AsmToken));
                   GenerateAsmStatementIntermediateCode;
                 end;
               Exit;
             end;
           If not RightExpressionNumericConstantAndSize8Bit then AsmError (ConstantOutOfRange)
             else AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte, Ord (AsmToken));
         end;
      2: begin
           If Length (AsmString) > 2 then AsmError (StringConstantExceedsLine); { ??? }
           RightExpression16BitNumericConstant;
           If LeftExpression^.RelocatableIdentifier = 0 then
             begin
               AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte, Ord (AsmToken));
               AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte1, Ord (AsmToken));
               Exit;
             end;
           CurrentRelocatableIdentifier.Long := AsmExpression1.RelocatableIdentifier;
           If LeftExpression^.ReferenceFlags = 0 then
             AddAsmIntermediateCodeRecord (acReference, $01, LeftExpression^.Value.Word, Ord (AsmToken)) else
               AddAsmIntermediateCodeRecord (acReference, LeftExpression^.ReferenceFlags,
                                               LeftExpression^.Value.Word, Ord (AsmToken));
         end;
      else begin
             If Length (AsmString) > 4 then AsmError (StringConstantExceedsLine); { ??? }
             If LeftExpression^.RelocatableIdentifier = 0 then
               begin
                 AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte, Ord (AsmToken));
                 AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte1, Ord (AsmToken));
                 AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte2, Ord (AsmToken));
                 AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Byte3, Ord (AsmToken));
                 Exit;
               end;
             CurrentRelocatableIdentifier.Long := AsmExpression1.RelocatableIdentifier;
             If LeftExpression^.ReferenceFlags = 0 then
               AddAsmIntermediateCodeRecord (acReference, $03, LeftExpression^.Value.Word, Ord (AsmToken)) else
                 begin
                   AddAsmIntermediateCodeRecord (acReference, LeftExpression^.ReferenceFlags,
                                                 LeftExpression^.Value.Word, Ord (AsmToken));
                   AddAsmIntermediateCodeRecord (acByte, 0, LeftExpression^.Value.Word2, Ord (AsmToken))
                 end;
           end;
    end;
  end;

begin
  If (AsmToken = AsmToken_Instruction) and (InstructionData^.B0 = $EA) then
    begin
      DirectiveSize := InstructionData^.B1;
      If PrefixOpcode [0] <> 0 then AsmError (SyntaxError);
      GetNextAsmToken;
      Repeat
        LeftExpression := @AsmExpression1;
        ProcessAsmExpressionAndCheck;
        GenerateDirectiveAsmIntermediateCode;
        GenerateAsmStatementIntermediateCode;
      until not CheckAndGetNextAsmToken (AsmToken_Comma);
      ExpectAsmTokenAndGetNext (AsmToken_Semicolon);
      ProcessAssemblerDirective := True;
    end else ProcessAssemblerDirective := False;
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
- https://turbopascal.org/processing-assembler-directives
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

