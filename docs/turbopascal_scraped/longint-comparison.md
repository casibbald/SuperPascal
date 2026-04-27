# LongInt Comparison - Turbo Pascal Compiler

**URL:** https://turbopascal.org/longint-comparison

---

## Table of Contents

- LongInt Comparison

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
LongInt Comparison
This procedure generates code to compare two
LongInt
values. Again, for some special cases shorter code is generated.
Procedure GenerateCodeForLongIntComparisons;

      Procedure GenerateShortCircuitLongIntComparisonJumps;
      Const OpCodes: Array [Calc_IsEqual..Calc_IsLowerOrEqual, Boolean] of Byte = (
        (JNE,   0),
        (  0, JNE),
        ( JL,  JG),
        ( JG,  JL),
        ( JL,  JG),
        ( JG,  JL));
      begin
        If OpCodes [Operation, True] <> 0 then
          GenerateCodeForNearJump (LongIntComparisonLastJumpToTrue, OpCodes [Operation, True]);
        If OpCodes [Operation, False] <> 0 then
          GenerateCodeForNearJump (LongIntComparisonLastJumpToFalse, OpCodes [Operation, False]);
      end;

      Procedure GeneralGenerateCodeForLongIntComparisons;
      begin
        LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urDX, urAX]);
        RightExpression.SwitchBetweenLoWordAndHiWord (2);
        RightExpression.GenerateArithmeticInstructionWith_DX (mod_CMP_rm);
        GenerateShortCircuitLongIntComparisonJumps;
        RightExpression.SwitchBetweenLoWordAndHiWord (- 2);
        RightExpression.GenerateArithmeticInstructionWith_ACC (mod_CMP_rm);
      end;

      Procedure GenerateCodeForLongIntComparisonsWithConstant;
      begin
        If LeftExpression.Location = elMemory then
          begin
            LeftExpression.Calculate;
            Inc (LeftExpression.Value.Word, 2);
            LeftExpression.GenerateArithmeticInstructionWithImmediateValue (
                                            RightExpression.Value.LongRec.WordH,
                                            mod_CMP_rm);
            GenerateShortCircuitLongIntComparisonJumps;
            Dec (LeftExpression.Value.Word, 2);
            LeftExpression.GenerateArithmeticInstructionWithImmediateValue (
                                            RightExpression.Value.LongRec.WordL,
                                            mod_CMP_rm);
          end else GeneralGenerateCodeForLongIntComparisons;
      end;

    begin
      LongIntComparisonLastJumpToTrue := 0;
      LongIntComparisonLastJumpToFalse := 0;
      If RightExpression.Location = elConstant then
        begin
          If RightExpression.Value.LongRec.Long <> 0 then GenerateCodeForLongIntComparisonsWithConstant
            else begin
                   Case Operation of
                     Calc_IsEqual,
                     Calc_IsNotEqual: Case LeftExpression.Location of
                                        elMemory:
                                          begin
                                            LeftExpression.Calculate;
                                            Exclude (LeftExpression.DataType, it32Bit);
                                            LeftExpression.LoadExpressionToRegister (rAX);
                                            Inc (LeftExpression.Value.Word, 2);
                                            LeftExpression.GenerateArithmeticInstructionWith_ACC (mod_OR_rm);
                                            Include (LeftExpression.UsedRegisters, urAX);
                                          end;
                                        else begin
                                               LeftExpression.Calculate;
                                               GenerateInstruction_Word (OR_AX_DX);
                                             end;
                                      end;
                     else GenerateCodeForLongIntComparisonsWithConstant;
                   end;
                 end;
        end else GeneralGenerateCodeForLongIntComparisons;
      LeftExpression.SetExpressionToBooleanJump (UnsignedConditionalJumpOpCode [Operation]);
      With LeftExpression do
        begin
          Value.LastJumpToTrue  := LongIntComparisonLastJumpToTrue;
          Value.LastJumpToFalse := LongIntComparisonLastJumpToFalse;
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
    Procedure GenerateCodeForLongIntComparisons;

      Procedure GenerateShortCircuitLongIntComparisonJumps;
      Const OpCodes: Array [Calc_IsEqual..Calc_IsLowerOrEqual, Boolean] of Byte = (
        (JNE,   0),
        (  0, JNE),
        ( JL,  JG),
        ( JG,  JL),
        ( JL,  JG),
        ( JG,  JL));
      begin
        If OpCodes [Operation, True] <> 0 then
          GenerateCodeForNearJump (LongIntComparisonLastJumpToTrue, OpCodes [Operation, True]);
        If OpCodes [Operation, False] <> 0 then
          GenerateCodeForNearJump (LongIntComparisonLastJumpToFalse, OpCodes [Operation, False]);
      end;

      Procedure GeneralGenerateCodeForLongIntComparisons;
      begin
        LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urDX, urAX]);
        RightExpression.SwitchBetweenLoWordAndHiWord (2);
        RightExpression.GenerateArithmeticInstructionWith_DX (mod_CMP_rm);
        GenerateShortCircuitLongIntComparisonJumps;
        RightExpression.SwitchBetweenLoWordAndHiWord (- 2);
        RightExpression.GenerateArithmeticInstructionWith_ACC (mod_CMP_rm);
      end;

      Procedure GenerateCodeForLongIntComparisonsWithConstant;
      begin
        If LeftExpression.Location = elMemory then
          begin
            LeftExpression.Calculate;
            Inc (LeftExpression.Value.Word, 2);
            LeftExpression.GenerateArithmeticInstructionWithImmediateValue (
                                            RightExpression.Value.LongRec.WordH,
                                            mod_CMP_rm);
            GenerateShortCircuitLongIntComparisonJumps;
            Dec (LeftExpression.Value.Word, 2);
            LeftExpression.GenerateArithmeticInstructionWithImmediateValue (
                                            RightExpression.Value.LongRec.WordL,
                                            mod_CMP_rm);
          end else GeneralGenerateCodeForLongIntComparisons;
      end;

    begin
      LongIntComparisonLastJumpToTrue := 0;
      LongIntComparisonLastJumpToFalse := 0;
      If RightExpression.Location = elConstant then
        begin
          If RightExpression.Value.LongRec.Long <> 0 then GenerateCodeForLongIntComparisonsWithConstant
            else begin
                   Case Operation of
                     Calc_IsEqual,
                     Calc_IsNotEqual: Case LeftExpression.Location of
                                        elMemory:
                                          begin
                                            LeftExpression.Calculate;
                                            Exclude (LeftExpression.DataType, it32Bit);
                                            LeftExpression.LoadExpressionToRegister (rAX);
                                            Inc (LeftExpression.Value.Word, 2);
                                            LeftExpression.GenerateArithmeticInstructionWith_ACC (mod_OR_rm);
                                            Include (LeftExpression.UsedRegisters, urAX);
                                          end;
                                        else begin
                                               LeftExpression.Calculate;
                                               GenerateInstruction_Word (OR_AX_DX);
                                             end;
                                      end;
                     else GenerateCodeForLongIntComparisonsWithConstant;
                   end;
                 end;
        end else GeneralGenerateCodeForLongIntComparisons;
      LeftExpression.SetExpressionToBooleanJump (UnsignedConditionalJumpOpCode [Operation]);
      With LeftExpression do
        begin
          Value.LastJumpToTrue  := LongIntComparisonLastJumpToTrue;
          Value.LastJumpToFalse := LongIntComparisonLastJumpToFalse;
        end;
    end;
```

### Code Block 2

```pascal
    Procedure GenerateCodeForLongIntComparisons;

      Procedure GenerateShortCircuitLongIntComparisonJumps;
      Const OpCodes: Array [Calc_IsEqual..Calc_IsLowerOrEqual, Boolean] of Byte = (
        (JNE,   0),
        (  0, JNE),
        ( JL,  JG),
        ( JG,  JL),
        ( JL,  JG),
        ( JG,  JL));
      begin
        If OpCodes [Operation, True] <> 0 then
          GenerateCodeForNearJump (LongIntComparisonLastJumpToTrue, OpCodes [Operation, True]);
        If OpCodes [Operation, False] <> 0 then
          GenerateCodeForNearJump (LongIntComparisonLastJumpToFalse, OpCodes [Operation, False]);
      end;

      Procedure GeneralGenerateCodeForLongIntComparisons;
      begin
        LoadBothExpressionsToRegisters (LeftExpression, RightExpression, [urDX, urAX]);
        RightExpression.SwitchBetweenLoWordAndHiWord (2);
        RightExpression.GenerateArithmeticInstructionWith_DX (mod_CMP_rm);
        GenerateShortCircuitLongIntComparisonJumps;
        RightExpression.SwitchBetweenLoWordAndHiWord (- 2);
        RightExpression.GenerateArithmeticInstructionWith_ACC (mod_CMP_rm);
      end;

      Procedure GenerateCodeForLongIntComparisonsWithConstant;
      begin
        If LeftExpression.Location = elMemory then
          begin
            LeftExpression.Calculate;
            Inc (LeftExpression.Value.Word, 2);
            LeftExpression.GenerateArithmeticInstructionWithImmediateValue (
                                            RightExpression.Value.LongRec.WordH,
                                            mod_CMP_rm);
            GenerateShortCircuitLongIntComparisonJumps;
            Dec (LeftExpression.Value.Word, 2);
            LeftExpression.GenerateArithmeticInstructionWithImmediateValue (
                                            RightExpression.Value.LongRec.WordL,
                                            mod_CMP_rm);
          end else GeneralGenerateCodeForLongIntComparisons;
      end;

    begin
      LongIntComparisonLastJumpToTrue := 0;
      LongIntComparisonLastJumpToFalse := 0;
      If RightExpression.Location = elConstant then
        begin
          If RightExpression.Value.LongRec.Long <> 0 then GenerateCodeForLongIntComparisonsWithConstant
            else begin
                   Case Operation of
                     Calc_IsEqual,
                     Calc_IsNotEqual: Case LeftExpression.Location of
                                        elMemory:
                                          begin
                                            LeftExpression.Calculate;
                                            Exclude (LeftExpression.DataType, it32Bit);
                                            LeftExpression.LoadExpressionToRegister (rAX);
                                            Inc (LeftExpression.Value.Word, 2);
                                            LeftExpression.GenerateArithmeticInstructionWith_ACC (mod_OR_rm);
                                            Include (LeftExpression.UsedRegisters, urAX);
                                          end;
                                        else begin
                                               LeftExpression.Calculate;
                                               GenerateInstruction_Word (OR_AX_DX);
                                             end;
                                      end;
                     else GenerateCodeForLongIntComparisonsWithConstant;
                   end;
                 end;
        end else GeneralGenerateCodeForLongIntComparisons;
      LeftExpression.SetExpressionToBooleanJump (UnsignedConditionalJumpOpCode [Operation]);
      With LeftExpression do
        begin
          Value.LastJumpToTrue  := LongIntComparisonLastJumpToTrue;
          Value.LastJumpToFalse := LongIntComparisonLastJumpToFalse;
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
- https://turbopascal.org/longint-comparison
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

