# REPEAT Statement - Turbo Pascal Compiler

**URL:** https://turbopascal.org/repeat-statement

---

## Table of Contents

- REPEAT Statement

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
REPEAT Statement
This procedure processes Repeat statement. It calls routine to
process statements
between tokens
Repeat
and
until
. Loop statements
Break
and
Continue
are also taken into account.
Procedure TStatement.Process_REPEAT_Statement;
Var UntilCondition: TExpression;
    Statement: TStatement;
    SavedLastJumpOutOfBlock, SavedLastJumpToNextBlockIteration: Word;
begin
  SavedLastJumpOutOfBlock := LastJumpOutOfBlock;
  SavedLastJumpToNextBlockIteration := LastJumpToNextBlockIteration;
  LastJumpOutOfBlock := 0;
  LastJumpToNextBlockIteration := 0;
  Statement.ProcessStatementsBetweenTokens (Token_REPEAT, Token_UNTIL);
  UntilCondition.ExpectBooleanExpression;
  If UntilCondition.Location = elConstant then
    begin
      UntilCondition.Value.LastJumpToFalse := 0;
      If UntilCondition.Value.Byte = 0 then
        begin                    { until False creates endless loop }
          GenerateCodeForNearJump (UntilCondition.Value.LastJumpToFalse, JMP_ShortDirect);
          UntilCondition.EndIntermediateCodeSubroutine;
        end;
    end;
  GenerateLabelAndSetJumpsToIt (UntilCondition.Value.LastJumpToFalse);
  StoreCode_icGoSub (Statement.StatementCode);
  GenerateLabelAndSetJumpsToIt (LastJumpToNextBlockIteration);
  StoreCode_icGoSub (UntilCondition.IntermediateCodeOffset);
  GenerateLabelAndSetJumpsToIt (LastJumpOutOfBlock);
  StatementCode := EndSubroutine;
  LastJumpOutOfBlock := SavedLastJumpOutOfBlock;
  LastJumpToNextBlockIteration := SavedLastJumpToNextBlockIteration;
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
Procedure TStatement.Process_REPEAT_Statement;
Var UntilCondition: TExpression;
    Statement: TStatement;
    SavedLastJumpOutOfBlock, SavedLastJumpToNextBlockIteration: Word;
begin
  SavedLastJumpOutOfBlock := LastJumpOutOfBlock;
  SavedLastJumpToNextBlockIteration := LastJumpToNextBlockIteration;
  LastJumpOutOfBlock := 0;
  LastJumpToNextBlockIteration := 0;
  Statement.ProcessStatementsBetweenTokens (Token_REPEAT, Token_UNTIL);
  UntilCondition.ExpectBooleanExpression;
  If UntilCondition.Location = elConstant then
    begin
      UntilCondition.Value.LastJumpToFalse := 0;
      If UntilCondition.Value.Byte = 0 then
        begin                    { until False creates endless loop }
          GenerateCodeForNearJump (UntilCondition.Value.LastJumpToFalse, JMP_ShortDirect);
          UntilCondition.EndIntermediateCodeSubroutine;
        end;
    end;
  GenerateLabelAndSetJumpsToIt (UntilCondition.Value.LastJumpToFalse);
  StoreCode_icGoSub (Statement.StatementCode);
  GenerateLabelAndSetJumpsToIt (LastJumpToNextBlockIteration);
  StoreCode_icGoSub (UntilCondition.IntermediateCodeOffset);
  GenerateLabelAndSetJumpsToIt (LastJumpOutOfBlock);
  StatementCode := EndSubroutine;
  LastJumpOutOfBlock := SavedLastJumpOutOfBlock;
  LastJumpToNextBlockIteration := SavedLastJumpToNextBlockIteration;
end;
```

### Code Block 2

```pascal
Procedure TStatement.Process_REPEAT_Statement;
Var UntilCondition: TExpression;
    Statement: TStatement;
    SavedLastJumpOutOfBlock, SavedLastJumpToNextBlockIteration: Word;
begin
  SavedLastJumpOutOfBlock := LastJumpOutOfBlock;
  SavedLastJumpToNextBlockIteration := LastJumpToNextBlockIteration;
  LastJumpOutOfBlock := 0;
  LastJumpToNextBlockIteration := 0;
  Statement.ProcessStatementsBetweenTokens (Token_REPEAT, Token_UNTIL);
  UntilCondition.ExpectBooleanExpression;
  If UntilCondition.Location = elConstant then
    begin
      UntilCondition.Value.LastJumpToFalse := 0;
      If UntilCondition.Value.Byte = 0 then
        begin                    { until False creates endless loop }
          GenerateCodeForNearJump (UntilCondition.Value.LastJumpToFalse, JMP_ShortDirect);
          UntilCondition.EndIntermediateCodeSubroutine;
        end;
    end;
  GenerateLabelAndSetJumpsToIt (UntilCondition.Value.LastJumpToFalse);
  StoreCode_icGoSub (Statement.StatementCode);
  GenerateLabelAndSetJumpsToIt (LastJumpToNextBlockIteration);
  StoreCode_icGoSub (UntilCondition.IntermediateCodeOffset);
  GenerateLabelAndSetJumpsToIt (LastJumpOutOfBlock);
  StatementCode := EndSubroutine;
  LastJumpOutOfBlock := SavedLastJumpOutOfBlock;
  LastJumpToNextBlockIteration := SavedLastJumpToNextBlockIteration;
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
- https://turbopascal.org/processing-statement-blocks
- https://turbopascal.org/repeat-statement
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

