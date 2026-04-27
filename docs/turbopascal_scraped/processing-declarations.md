# Processing Declarations - Turbo Pascal Compiler

**URL:** https://turbopascal.org/processing-declarations

---

## Table of Contents

- Processing Declarations

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Processing Declarations
This procedure processes Pascal declarations: Constants,
Types
,
Variables
,
Procedures
,
Functions
, and if declarations are not in the
Interface
part of the unit, also
Lables
,
Constructors
and
Destructors
are processed. Before declarations are processes, statement compiler switches are reset to module compiler switches. This procedure is called from
program compilation
,
unit interface and implementation compilation
and
Procedure/Function compilation
.
Procedure ProcessDeclarations;
begin
  Repeat
    StatementCompilerSwitches := ModuleCompilerSwitches;
    Case Token of
      Token_CONST:     ProcessConstantDeclarations;
      Token_TYPE:      ProcessTypeDeclarations;
      Token_VAR:       ProcessVariableDeclarations;
      Token_PROCEDURE: ProcessProcedureDeclaration;
      Token_FUNCTION:  ProcessProcedureDeclaration;
      else If SourceType <> stUnitInterface then
        Case Token of
          Token_LABEL:       ProcessLabelDeclarations;
          Token_CONSTRUCTOR: ProcessProcedureDeclaration;
          Token_DESTRUCTOR:  ProcessProcedureDeclaration;
          else Exit;
        end else Exit;
    end;
  until False;
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
Procedure ProcessDeclarations;
begin
  Repeat
    StatementCompilerSwitches := ModuleCompilerSwitches;
    Case Token of
      Token_CONST:     ProcessConstantDeclarations;
      Token_TYPE:      ProcessTypeDeclarations;
      Token_VAR:       ProcessVariableDeclarations;
      Token_PROCEDURE: ProcessProcedureDeclaration;
      Token_FUNCTION:  ProcessProcedureDeclaration;
      else If SourceType <> stUnitInterface then
        Case Token of
          Token_LABEL:       ProcessLabelDeclarations;
          Token_CONSTRUCTOR: ProcessProcedureDeclaration;
          Token_DESTRUCTOR:  ProcessProcedureDeclaration;
          else Exit;
        end else Exit;
    end;
  until False;
end;
```

### Code Block 2

```pascal
Procedure ProcessDeclarations;
begin
  Repeat
    StatementCompilerSwitches := ModuleCompilerSwitches;
    Case Token of
      Token_CONST:     ProcessConstantDeclarations;
      Token_TYPE:      ProcessTypeDeclarations;
      Token_VAR:       ProcessVariableDeclarations;
      Token_PROCEDURE: ProcessProcedureDeclaration;
      Token_FUNCTION:  ProcessProcedureDeclaration;
      else If SourceType <> stUnitInterface then
        Case Token of
          Token_LABEL:       ProcessLabelDeclarations;
          Token_CONSTRUCTOR: ProcessProcedureDeclaration;
          Token_DESTRUCTOR:  ProcessProcedureDeclaration;
          else Exit;
        end else Exit;
    end;
  until False;
end;
```

## Links

- https://turbopascal.org/
- https://turbopascal.org/about
- https://turbopascal.org/assembler
- https://turbopascal.org/calculator
- https://turbopascal.org/code-generator
- https://turbopascal.org/compilation-process
- https://turbopascal.org/compiler
- https://turbopascal.org/expressions
- https://turbopascal.org/io-utilities
- https://turbopascal.org/linker
- https://turbopascal.org/object-files
- https://turbopascal.org/parser
- https://turbopascal.org/processing-declarations
- https://turbopascal.org/processing-label-declarations
- https://turbopascal.org/processing-object-types
- https://turbopascal.org/processing-procedure-declarations
- https://turbopascal.org/processing-type-declarations
- https://turbopascal.org/processing-variable-declarations
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

