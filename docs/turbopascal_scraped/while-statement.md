# WHILE Statement - Turbo Pascal Compiler

**URL:** https://turbopascal.org/while-statement

---

## Table of Contents

- WHILE Statement

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
WHILE Statement
This procedure processes
While
statement. If condition expression is constant at compile time and evaluates to
False
, no code is generated. All jumps out of the loop (
Break
,
Continue
) are also taken into account.
Procedure TStatement.Process_WHILE_Statement;
Var WhileCondition: TExpression;
    Statement: TStatement;
    SavedLastJumpOutOfBlock, SavedLastJumpToNextBlockIteration: Word;
begin
  SavedLastJumpOutOfBlock := LastJumpOutOfBlock;
  SavedLastJumpToNextBlockIteration := LastJumpToNextBlockIteration;
  LastJumpOutOfBlock := 0;
  LastJumpToNextBlockIteration := 0;
  GetNextToken;
  WhileCondition.ExpectBooleanExpression;
  ExpectTokenAndGetNext (Token_DO);
  Statement.ProcessStatement;
  If WhileCondition.Location = elConstant then
    begin
      If WhileCondition.Value.Byte = 0 then
        begin
          StatementCode := 0;
          LastJumpOutOfBlock := SavedLastJumpOutOfBlock;
          LastJumpToNextBlockIteration := SavedLastJumpToNextBlockIteration;
          Exit;
        end;
      WhileCondition.Value.LastJumpToFalse := 0;
    end;
  StoreCode_icGoSub (Statement.StatementCode);
  GenerateCodeForNearJump (LastJumpToNextBlockIteration, JMP_ShortDirect);
  StatementCode := EndSubroutine;
  GenerateLabelAndSetJumpsToIt (LastJumpToNextBlockIteration);
  StoreCode_icGoSub (WhileCondition.IntermediateCodeOffset);
  StoreCode_icGoSub (StatementCode);
  JoinJumpsOfBothExpressions (LastJumpOutOfBlock, WhileCondition.Value.LastJumpToFalse);
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
Procedure TStatement.Process_WHILE_Statement;
Var WhileCondition: TExpression;
    Statement: TStatement;
    SavedLastJumpOutOfBlock, SavedLastJumpToNextBlockIteration: Word;
begin
  SavedLastJumpOutOfBlock := LastJumpOutOfBlock;
  SavedLastJumpToNextBlockIteration := LastJumpToNextBlockIteration;
  LastJumpOutOfBlock := 0;
  LastJumpToNextBlockIteration := 0;
  GetNextToken;
  WhileCondition.ExpectBooleanExpression;
  ExpectTokenAndGetNext (Token_DO);
  Statement.ProcessStatement;
  If WhileCondition.Location = elConstant then
    begin
      If WhileCondition.Value.Byte = 0 then
        begin
          StatementCode := 0;
          LastJumpOutOfBlock := SavedLastJumpOutOfBlock;
          LastJumpToNextBlockIteration := SavedLastJumpToNextBlockIteration;
          Exit;
        end;
      WhileCondition.Value.LastJumpToFalse := 0;
    end;
  StoreCode_icGoSub (Statement.StatementCode);
  GenerateCodeForNearJump (LastJumpToNextBlockIteration, JMP_ShortDirect);
  StatementCode := EndSubroutine;
  GenerateLabelAndSetJumpsToIt (LastJumpToNextBlockIteration);
  StoreCode_icGoSub (WhileCondition.IntermediateCodeOffset);
  StoreCode_icGoSub (StatementCode);
  JoinJumpsOfBothExpressions (LastJumpOutOfBlock, WhileCondition.Value.LastJumpToFalse);
  GenerateLabelAndSetJumpsToIt (LastJumpOutOfBlock);
  StatementCode := EndSubroutine;
  LastJumpOutOfBlock := SavedLastJumpOutOfBlock;
  LastJumpToNextBlockIteration := SavedLastJumpToNextBlockIteration;
end;
```

### Code Block 2

```pascal
Procedure TStatement.Process_WHILE_Statement;
Var WhileCondition: TExpression;
    Statement: TStatement;
    SavedLastJumpOutOfBlock, SavedLastJumpToNextBlockIteration: Word;
begin
  SavedLastJumpOutOfBlock := LastJumpOutOfBlock;
  SavedLastJumpToNextBlockIteration := LastJumpToNextBlockIteration;
  LastJumpOutOfBlock := 0;
  LastJumpToNextBlockIteration := 0;
  GetNextToken;
  WhileCondition.ExpectBooleanExpression;
  ExpectTokenAndGetNext (Token_DO);
  Statement.ProcessStatement;
  If WhileCondition.Location = elConstant then
    begin
      If WhileCondition.Value.Byte = 0 then
        begin
          StatementCode := 0;
          LastJumpOutOfBlock := SavedLastJumpOutOfBlock;
          LastJumpToNextBlockIteration := SavedLastJumpToNextBlockIteration;
          Exit;
        end;
      WhileCondition.Value.LastJumpToFalse := 0;
    end;
  StoreCode_icGoSub (Statement.StatementCode);
  GenerateCodeForNearJump (LastJumpToNextBlockIteration, JMP_ShortDirect);
  StatementCode := EndSubroutine;
  GenerateLabelAndSetJumpsToIt (LastJumpToNextBlockIteration);
  StoreCode_icGoSub (WhileCondition.IntermediateCodeOffset);
  StoreCode_icGoSub (StatementCode);
  JoinJumpsOfBothExpressions (LastJumpOutOfBlock, WhileCondition.Value.LastJumpToFalse);
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
- https://turbopascal.org/while-statement

