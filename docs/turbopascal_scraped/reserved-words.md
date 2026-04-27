# Reserved Words - Turbo Pascal Compiler

**URL:** https://turbopascal.org/reserved-words

---

## Table of Contents

- Reserved Words

---

## Content

Toggle navigation
Pascal is easy to read and easy to write
Reserved Words
All Turbo Pascal reserved words are stored in a dedicated
symbol table
.
Const NumResWords = 49;
      ReservedWord: Array [1..NumResWords] of String = (
        'AND',
        'ARRAY',
        'BEGIN',
        'CASE',
        'CONST',
        'CONSTRUCTOR',
        'DESTRUCTOR',
        'DIV',
        'DO',
        'DOWNTO',
        'ELSE',
        'END',
        'ASM',
        'FILE',
        'FOR',
        'FUNCTION',
        'GOTO',
        'IF',
        'IMPLEMENTATION',
        'IN',
        'INLINE',
        'INTERFACE',
        'LABEL',
        'MOD',
        'NIL',
        'NOT',
        'OBJECT',
        'OF',
        'OR',
        'PACKED',
        'PROCEDURE',
        'PROGRAM',
        'RECORD',
        'REPEAT',
        'SET',
        'SHL',
        'SHR',
        'STRING',
        'THEN',
        'TYPE',
        'TO',
        'UNIT',
        'UNTIL',
        'USES',
        'VAR',
        'INHERITED',
        'WHILE',
        'WITH',
        'XOR');
Of course, this symbol table was not created manually but with a software which calculated hashes and put reserved words into right linked lists.
Const

  Ofs_00_01: Word = 0;
  Tok_00_01: TToken = Token_AND;
  ResWordLen_AND: Byte = 3;
  ResWord_AND: Array [1..3] of Char = 'AND';

  Ofs_00_02: Word = Ofs (Ofs_00_01);
  Tok_00_02: TToken = Token_BEGIN;
  ResWordLen_BEGIN: Byte = 5;
  ResWord_BEGIN: Array [1..5] of Char = 'BEGIN';

  Ofs_00_03: Word = Ofs (Ofs_00_02);
  Tok_00_03: TToken = Token_DIV;
  ResWordLen_DIV: Byte = 3;
  ResWord_DIV: Array [1..3] of Char = 'DIV';

  Ofs_00_04: Word = Ofs (Ofs_00_03);
  Tok_00_04: TToken = Token_NIL;
  ResWordLen_NIL: Byte = 3;
  ResWord_NIL: Array [1..3] of Char = 'NIL';

  Ofs_00_05: Word = Ofs (Ofs_00_04);
  Tok_00_05: TToken = Token_PROCEDURE;
  ResWordLen_PROCEDURE: Byte = 9;
  ResWord_PROCEDURE: Array [1..9] of Char = 'PROCEDURE';

  Ofs_01_01: Word = 0;
  Tok_01_01: TToken = Token_DO;
  ResWordLen_DO: Byte = 2;
  ResWord_DO: Array [1..2] of Char = 'DO';

  Ofs_01_02: Word = Ofs (Ofs_01_01);
  Tok_01_02: TToken = Token_OBJECT;
  ResWordLen_OBJECT: Byte = 6;
  ResWord_OBJECT: Array [1..6] of Char = 'OBJECT';

  Ofs_01_03: Word = Ofs (Ofs_01_02);
  Tok_01_03: TToken = Token_PROGRAM;
  ResWordLen_PROGRAM: Byte = 7;
  ResWord_PROGRAM: Array [1..7] of Char = 'PROGRAM';

  Ofs_01_04: Word = Ofs (Ofs_01_03);
  Tok_01_04: TToken = Token_STRING;
  ResWordLen_STRING: Byte = 6;
  ResWord_STRING: Array [1..6] of Char = 'STRING';

  Ofs_01_05: Word = Ofs (Ofs_01_04);
  Tok_01_05: TToken = Token_TO;
  ResWordLen_TO: Byte = 2;
  ResWord_TO: Array [1..2] of Char = 'TO';

  Ofs_02_01: Word = 0;
  Tok_02_01: TToken = Token_CONST;
  ResWordLen_CONST: Byte = 5;
  ResWord_CONST: Array [1..5] of Char = 'CONST';

  Ofs_02_02: Word = Ofs (Ofs_02_01);
  Tok_02_02: TToken = Token_PACKED;
  ResWordLen_PACKED: Byte = 6;
  ResWord_PACKED: Array [1..6] of Char = 'PACKED';

  Ofs_03_01: Word = 0;
  Tok_03_01: TToken = Token_OF;
  ResWordLen_OF: Byte = 2;
  ResWord_OF: Array [1..2] of Char = 'OF';

  Ofs_03_02: Word = Ofs (Ofs_03_01);
  Tok_03_02: TToken = Token_INHERITED;
  ResWordLen_INHERITED: Byte = 9;
  ResWord_INHERITED: Array [1..9] of Char = 'INHERITED';

  Ofs_04_01: Word = 0;
  Tok_04_01: TToken = Token_END;
  ResWordLen_END: Byte = 3;
  ResWord_END: Array [1..3] of Char = 'END';

  Ofs_04_02: Word = Ofs (Ofs_04_01);
  Tok_04_02: TToken = Token_FOR;
  ResWordLen_FOR: Byte = 3;
  ResWord_FOR: Array [1..3] of Char = 'FOR';

  Ofs_04_03: Word = Ofs (Ofs_04_02);
  Tok_04_03: TToken = Token_SHL;
  ResWordLen_SHL: Byte = 3;
  ResWord_SHL: Array [1..3] of Char = 'SHL';

  Ofs_04_04: Word = Ofs (Ofs_04_03);
  Tok_04_04: TToken = Token_WHILE;
  ResWordLen_WHILE: Byte = 5;
  ResWord_WHILE: Array [1..5] of Char = 'WHILE';

  Ofs_05_01: Word = 0;
  Tok_05_01: TToken = Token_DESTRUCTOR;
  ResWordLen_DESTRUCTOR: Byte = 10;
  ResWord_DESTRUCTOR: Array [1..10] of Char = 'DESTRUCTOR';

  Ofs_05_02: Word = Ofs (Ofs_05_01);
  Tok_05_02: TToken = Token_DOWNTO;
  ResWordLen_DOWNTO: Byte = 6;
  ResWord_DOWNTO: Array [1..6] of Char = 'DOWNTO';

  Ofs_05_03: Word = Ofs (Ofs_05_02);
  Tok_05_03: TToken = Token_ELSE;
  ResWordLen_ELSE: Byte = 4;
  ResWord_ELSE: Array [1..4] of Char = 'ELSE';

  Ofs_05_04: Word = Ofs (Ofs_05_03);
  Tok_05_04: TToken = Token_GOTO;
  ResWordLen_GOTO: Byte = 4;
  ResWord_GOTO: Array [1..4] of Char = 'GOTO';

  Ofs_05_05: Word = Ofs (Ofs_05_04);
  Tok_05_05: TToken = Token_IN;
  ResWordLen_IN: Byte = 2;
  ResWord_IN: Array [1..2] of Char = 'IN';

  Ofs_06_01: Word = 0;
  Tok_06_01: TToken = Token_VAR;
  ResWordLen_VAR: Byte = 3;
  ResWord_VAR: Array [1..3] of Char = 'VAR';

  Ofs_06_02: Word = Ofs (Ofs_06_01);
  Tok_06_02: TToken = Token_XOR;
  ResWordLen_XOR: Byte = 3;
  ResWord_XOR: Array [1..3] of Char = 'XOR';

  Ofs_07_01: Word = 0;
  Tok_07_01: TToken = Token_UNTIL;
  ResWordLen_UNTIL: Byte = 5;
  ResWord_UNTIL: Array [1..5] of Char = 'UNTIL';

  Ofs_08_01: Word = 0;
  Tok_08_01: TToken = Token_CASE;
  ResWordLen_CASE: Byte = 4;
  ResWord_CASE: Array [1..4] of Char = 'CASE';

  Ofs_08_02: Word = Ofs (Ofs_08_01);
  Tok_08_02: TToken = Token_IMPLEMENTATION;
  ResWordLen_IMPLEMENTATION: Byte = 14;
  ResWord_IMPLEMENTATION: Array [1..14] of Char = 'IMPLEMENTATION';

  Ofs_08_03: Word = Ofs (Ofs_08_02);
  Tok_08_03: TToken = Token_INTERFACE;
  ResWordLen_INTERFACE: Byte = 9;
  ResWord_INTERFACE: Array [1..9] of Char = 'INTERFACE';

  Ofs_08_04: Word = Ofs (Ofs_08_03);
  Tok_08_04: TToken = Token_WITH;
  ResWordLen_WITH: Byte = 4;
  ResWord_WITH: Array [1..4] of Char = 'WITH';

  Ofs_09_01: Word = 0;
  Tok_09_01: TToken = Token_INLINE;
  ResWordLen_INLINE: Byte = 6;
  ResWord_INLINE: Array [1..6] of Char = 'INLINE';

  Ofs_09_02: Word = Ofs (Ofs_09_01);
  Tok_09_02: TToken = Token_RECORD;
  ResWordLen_RECORD: Byte = 6;
  ResWord_RECORD: Array [1..6] of Char = 'RECORD';

  Ofs_09_03: Word = Ofs (Ofs_09_02);
  Tok_09_03: TToken = Token_SET;
  ResWordLen_SET: Byte = 3;
  ResWord_SET: Array [1..3] of Char = 'SET';

  Ofs_0A_01: Word = 0;
  Tok_0A_01: TToken = Token_ARRAY;
  ResWordLen_ARRAY: Byte = 5;
  ResWord_ARRAY: Array [1..5] of Char = 'ARRAY';

  Ofs_0A_02: Word = Ofs (Ofs_0A_01);
  Tok_0A_02: TToken = Token_SHR;
  ResWordLen_SHR: Byte = 3;
  ResWord_SHR: Array [1..3] of Char = 'SHR';

  Ofs_0B_01: Word = 0;
  Tok_0B_01: TToken = Token_CONSTRUCTOR;
  ResWordLen_CONSTRUCTOR: Byte = 11;
  ResWord_CONSTRUCTOR: Array [1..11] of Char = 'CONSTRUCTOR';

  Ofs_0B_02: Word = Ofs (Ofs_0B_01);
  Tok_0B_02: TToken = Token_LABEL;
  ResWordLen_LABEL: Byte = 5;
  ResWord_LABEL: Array [1..5] of Char = 'LABEL';

  Ofs_0B_03: Word = Ofs (Ofs_0B_02);
  Tok_0B_03: TToken = Token_REPEAT;
  ResWordLen_REPEAT: Byte = 6;
  ResWord_REPEAT: Array [1..6] of Char = 'REPEAT';

  Ofs_0B_04: Word = Ofs (Ofs_0B_03);
  Tok_0B_04: TToken = Token_THEN;
  ResWordLen_THEN: Byte = 4;
  ResWord_THEN: Array [1..4] of Char = 'THEN';

  Ofs_0C_01: Word = 0;
  Tok_0C_01: TToken = Token_FILE;
  ResWordLen_FILE: Byte = 4;
  ResWord_FILE: Array [1..4] of Char = 'FILE';

  Ofs_0C_02: Word = Ofs (Ofs_0C_01);
  Tok_0C_02: TToken = Token_UNIT;
  ResWordLen_UNIT: Byte = 4;
  ResWord_UNIT: Array [1..4] of Char = 'UNIT';

  Ofs_0C_03: Word = Ofs (Ofs_0C_02);
  Tok_0C_03: TToken = Token_USES;
  ResWordLen_USES: Byte = 4;
  ResWord_USES: Array [1..4] of Char = 'USES';

  Ofs_0D_01: Word = 0;
  Tok_0D_01: TToken = Token_IF;
  ResWordLen_IF: Byte = 2;
  ResWord_IF: Array [1..2] of Char = 'IF';

  Ofs_0D_02: Word = Ofs (Ofs_0D_01);
  Tok_0D_02: TToken = Token_MOD;
  ResWordLen_MOD: Byte = 3;
  ResWord_MOD: Array [1..3] of Char = 'MOD';

  Ofs_0E_01: Word = 0;
  Tok_0E_01: TToken = Token_ASM;
  ResWordLen_ASM: Byte = 3;
  ResWord_ASM: Array [1..3] of Char = 'ASM';

  Ofs_0E_02: Word = Ofs (Ofs_0E_01);
  Tok_0E_02: TToken = Token_FUNCTION;
  ResWordLen_FUNCTION: Byte = 8;
  ResWord_FUNCTION: Array [1..8] of Char = 'FUNCTION';

  Ofs_0E_03: Word = Ofs (Ofs_0E_02);
  Tok_0E_03: TToken = Token_NOT;
  ResWordLen_NOT: Byte = 3;
  ResWord_NOT: Array [1..3] of Char = 'NOT';

  Ofs_0E_04: Word = Ofs (Ofs_0E_03);
  Tok_0E_04: TToken = Token_TYPE;
  ResWordLen_TYPE: Byte = 4;
  ResWord_TYPE: Array [1..4] of Char = 'TYPE';

  Ofs_0F_01: Word = 0;
  Tok_0F_01: TToken = Token_OR;
  ResWordLen_OR: Byte = 2;
  ResWord_OR: Array [1..2] of Char = 'OR';


  ReservedWords: Word = $001E;

  OfsGroup_00: Word = Ofs (Ofs_00_05);
  OfsGroup_01: Word = Ofs (Ofs_01_05);
  OfsGroup_02: Word = Ofs (Ofs_02_02);
  OfsGroup_03: Word = Ofs (Ofs_03_02);
  OfsGroup_04: Word = Ofs (Ofs_04_04);
  OfsGroup_05: Word = Ofs (Ofs_05_05);
  OfsGroup_06: Word = Ofs (Ofs_06_02);
  OfsGroup_07: Word = Ofs (Ofs_07_01);
  OfsGroup_08: Word = Ofs (Ofs_08_04);
  OfsGroup_09: Word = Ofs (Ofs_09_03);
  OfsGroup_0A: Word = Ofs (Ofs_0A_02);
  OfsGroup_0B: Word = Ofs (Ofs_0B_04);
  OfsGroup_0C: Word = Ofs (Ofs_0C_03);
  OfsGroup_0D: Word = Ofs (Ofs_0D_02);
  OfsGroup_0E: Word = Ofs (Ofs_0E_04);
  OfsGroup_0F: Word = Ofs (Ofs_0F_01);
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
Const NumResWords = 49;
      ReservedWord: Array [1..NumResWords] of String = (
        'AND',
        'ARRAY',
        'BEGIN',
        'CASE',
        'CONST',
        'CONSTRUCTOR',
        'DESTRUCTOR',
        'DIV',
        'DO',
        'DOWNTO',
        'ELSE',
        'END',
        'ASM',
        'FILE',
        'FOR',
        'FUNCTION',
        'GOTO',
        'IF',
        'IMPLEMENTATION',
        'IN',
        'INLINE',
        'INTERFACE',
        'LABEL',
        'MOD',
        'NIL',
        'NOT',
        'OBJECT',
        'OF',
        'OR',
        'PACKED',
        'PROCEDURE',
        'PROGRAM',
        'RECORD',
        'REPEAT',
        'SET',
        'SHL',
        'SHR',
        'STRING',
        'THEN',
        'TYPE',
        'TO',
        'UNIT',
        'UNTIL',
        'USES',
        'VAR',
        'INHERITED',
        'WHILE',
        'WITH',
        'XOR');
```

### Code Block 2

```pascal
Const

  Ofs_00_01: Word = 0;
  Tok_00_01: TToken = Token_AND;
  ResWordLen_AND: Byte = 3;
  ResWord_AND: Array [1..3] of Char = 'AND';

  Ofs_00_02: Word = Ofs (Ofs_00_01);
  Tok_00_02: TToken = Token_BEGIN;
  ResWordLen_BEGIN: Byte = 5;
  ResWord_BEGIN: Array [1..5] of Char = 'BEGIN';

  Ofs_00_03: Word = Ofs (Ofs_00_02);
  Tok_00_03: TToken = Token_DIV;
  ResWordLen_DIV: Byte = 3;
  ResWord_DIV: Array [1..3] of Char = 'DIV';

  Ofs_00_04: Word = Ofs (Ofs_00_03);
  Tok_00_04: TToken = Token_NIL;
  ResWordLen_NIL: Byte = 3;
  ResWord_NIL: Array [1..3] of Char = 'NIL';

  Ofs_00_05: Word = Ofs (Ofs_00_04);
  Tok_00_05: TToken = Token_PROCEDURE;
  ResWordLen_PROCEDURE: Byte = 9;
  ResWord_PROCEDURE: Array [1..9] of Char = 'PROCEDURE';

  Ofs_01_01: Word = 0;
  Tok_01_01: TToken = Token_DO;
  ResWordLen_DO: Byte = 2;
  ResWord_DO: Array [1..2] of Char = 'DO';

  Ofs_01_02: Word = Ofs (Ofs_01_01);
  Tok_01_02: TToken = Token_OBJECT;
  ResWordLen_OBJECT: Byte = 6;
  ResWord_OBJECT: Array [1..6] of Char = 'OBJECT';

  Ofs_01_03: Word = Ofs (Ofs_01_02);
  Tok_01_03: TToken = Token_PROGRAM;
  ResWordLen_PROGRAM: Byte = 7;
  ResWord_PROGRAM: Array [1..7] of Char = 'PROGRAM';

  Ofs_01_04: Word = Ofs (Ofs_01_03);
  Tok_01_04: TToken = Token_STRING;
  ResWordLen_STRING: Byte = 6;
  ResWord_STRING: Array [1..6] of Char = 'STRING';

  Ofs_01_05: Word = Ofs (Ofs_01_04);
  Tok_01_05: TToken = Token_TO;
  ResWordLen_TO: Byte = 2;
  ResWord_TO: Array [1..2] of Char = 'TO';

  Ofs_02_01: Word = 0;
  Tok_02_01: TToken = Token_CONST;
  ResWordLen_CONST: Byte = 5;
  ResWord_CONST: Array [1..5] of Char = 'CONST';

  Ofs_02_02: Word = Ofs (Ofs_02_01);
  Tok_02_02: TToken = Token_PACKED;
  ResWordLen_PACKED: Byte = 6;
  ResWord_PACKED: Array [1..6] of Char = 'PACKED';

  Ofs_03_01: Word = 0;
  Tok_03_01: TToken = Token_OF;
  ResWordLen_OF: Byte = 2;
  ResWord_OF: Array [1..2] of Char = 'OF';

  Ofs_03_02: Word = Ofs (Ofs_03_01);
  Tok_03_02: TToken = Token_INHERITED;
  ResWordLen_INHERITED: Byte = 9;
  ResWord_INHERITED: Array [1..9] of Char = 'INHERITED';

  Ofs_04_01: Word = 0;
  Tok_04_01: TToken = Token_END;
  ResWordLen_END: Byte = 3;
  ResWord_END: Array [1..3] of Char = 'END';

  Ofs_04_02: Word = Ofs (Ofs_04_01);
  Tok_04_02: TToken = Token_FOR;
  ResWordLen_FOR: Byte = 3;
  ResWord_FOR: Array [1..3] of Char = 'FOR';

  Ofs_04_03: Word = Ofs (Ofs_04_02);
  Tok_04_03: TToken = Token_SHL;
  ResWordLen_SHL: Byte = 3;
  ResWord_SHL: Array [1..3] of Char = 'SHL';

  Ofs_04_04: Word = Ofs (Ofs_04_03);
  Tok_04_04: TToken = Token_WHILE;
  ResWordLen_WHILE: Byte = 5;
  ResWord_WHILE: Array [1..5] of Char = 'WHILE';

  Ofs_05_01: Word = 0;
  Tok_05_01: TToken = Token_DESTRUCTOR;
  ResWordLen_DESTRUCTOR: Byte = 10;
  ResWord_DESTRUCTOR: Array [1..10] of Char = 'DESTRUCTOR';

  Ofs_05_02: Word = Ofs (Ofs_05_01);
  Tok_05_02: TToken = Token_DOWNTO;
  ResWordLen_DOWNTO: Byte = 6;
  ResWord_DOWNTO: Array [1..6] of Char = 'DOWNTO';

  Ofs_05_03: Word = Ofs (Ofs_05_02);
  Tok_05_03: TToken = Token_ELSE;
  ResWordLen_ELSE: Byte = 4;
  ResWord_ELSE: Array [1..4] of Char = 'ELSE';

  Ofs_05_04: Word = Ofs (Ofs_05_03);
  Tok_05_04: TToken = Token_GOTO;
  ResWordLen_GOTO: Byte = 4;
  ResWord_GOTO: Array [1..4] of Char = 'GOTO';

  Ofs_05_05: Word = Ofs (Ofs_05_04);
  Tok_05_05: TToken = Token_IN;
  ResWordLen_IN: Byte = 2;
  ResWord_IN: Array [1..2] of Char = 'IN';

  Ofs_06_01: Word = 0;
  Tok_06_01: TToken = Token_VAR;
  ResWordLen_VAR: Byte = 3;
  ResWord_VAR: Array [1..3] of Char = 'VAR';

  Ofs_06_02: Word = Ofs (Ofs_06_01);
  Tok_06_02: TToken = Token_XOR;
  ResWordLen_XOR: Byte = 3;
  ResWord_XOR: Array [1..3] of Char = 'XOR';

  Ofs_07_01: Word = 0;
  Tok_07_01: TToken = Token_UNTIL;
  ResWordLen_UNTIL: Byte = 5;
  ResWord_UNTIL: Array [1..5] of Char = 'UNTIL';

  Ofs_08_01: Word = 0;
  Tok_08_01: TToken = Token_CASE;
  ResWordLen_CASE: Byte = 4;
  ResWord_CASE: Array [1..4] of Char = 'CASE';

  Ofs_08_02: Word = Ofs (Ofs_08_01);
  Tok_08_02: TToken = Token_IMPLEMENTATION;
  ResWordLen_IMPLEMENTATION: Byte = 14;
  ResWord_IMPLEMENTATION: Array [1..14] of Char = 'IMPLEMENTATION';

  Ofs_08_03: Word = Ofs (Ofs_08_02);
  Tok_08_03: TToken = Token_INTERFACE;
  ResWordLen_INTERFACE: Byte = 9;
  ResWord_INTERFACE: Array [1..9] of Char = 'INTERFACE';

  Ofs_08_04: Word = Ofs (Ofs_08_03);
  Tok_08_04: TToken = Token_WITH;
  ResWordLen_WITH: Byte = 4;
  ResWord_WITH: Array [1..4] of Char = 'WITH';

  Ofs_09_01: Word = 0;
  Tok_09_01: TToken = Token_INLINE;
  ResWordLen_INLINE: Byte = 6;
  ResWord_INLINE: Array [1..6] of Char = 'INLINE';

  Ofs_09_02: Word = Ofs (Ofs_09_01);
  Tok_09_02: TToken = Token_RECORD;
  ResWordLen_RECORD: Byte = 6;
  ResWord_RECORD: Array [1..6] of Char = 'RECORD';

  Ofs_09_03: Word = Ofs (Ofs_09_02);
  Tok_09_03: TToken = Token_SET;
  ResWordLen_SET: Byte = 3;
  ResWord_SET: Array [1..3] of Char = 'SET';

  Ofs_0A_01: Word = 0;
  Tok_0A_01: TToken = Token_ARRAY;
  ResWordLen_ARRAY: Byte = 5;
  ResWord_ARRAY: Array [1..5] of Char = 'ARRAY';

  Ofs_0A_02: Word = Ofs (Ofs_0A_01);
  Tok_0A_02: TToken = Token_SHR;
  ResWordLen_SHR: Byte = 3;
  ResWord_SHR: Array [1..3] of Char = 'SHR';

  Ofs_0B_01: Word = 0;
  Tok_0B_01: TToken = Token_CONSTRUCTOR;
  ResWordLen_CONSTRUCTOR: Byte = 11;
  ResWord_CONSTRUCTOR: Array [1..11] of Char = 'CONSTRUCTOR';

  Ofs_0B_02: Word = Ofs (Ofs_0B_01);
  Tok_0B_02: TToken = Token_LABEL;
  ResWordLen_LABEL: Byte = 5;
  ResWord_LABEL: Array [1..5] of Char = 'LABEL';

  Ofs_0B_03: Word = Ofs (Ofs_0B_02);
  Tok_0B_03: TToken = Token_REPEAT;
  ResWordLen_REPEAT: Byte = 6;
  ResWord_REPEAT: Array [1..6] of Char = 'REPEAT';

  Ofs_0B_04: Word = Ofs (Ofs_0B_03);
  Tok_0B_04: TToken = Token_THEN;
  ResWordLen_THEN: Byte = 4;
  ResWord_THEN: Array [1..4] of Char = 'THEN';

  Ofs_0C_01: Word = 0;
  Tok_0C_01: TToken = Token_FILE;
  ResWordLen_FILE: Byte = 4;
  ResWord_FILE: Array [1..4] of Char = 'FILE';

  Ofs_0C_02: Word = Ofs (Ofs_0C_01);
  Tok_0C_02: TToken = Token_UNIT;
  ResWordLen_UNIT: Byte = 4;
  ResWord_UNIT: Array [1..4] of Char = 'UNIT';

  Ofs_0C_03: Word = Ofs (Ofs_0C_02);
  Tok_0C_03: TToken = Token_USES;
  ResWordLen_USES: Byte = 4;
  ResWord_USES: Array [1..4] of Char = 'USES';

  Ofs_0D_01: Word = 0;
  Tok_0D_01: TToken = Token_IF;
  ResWordLen_IF: Byte = 2;
  ResWord_IF: Array [1..2] of Char = 'IF';

  Ofs_0D_02: Word = Ofs (Ofs_0D_01);
  Tok_0D_02: TToken = Token_MOD;
  ResWordLen_MOD: Byte = 3;
  ResWord_MOD: Array [1..3] of Char = 'MOD';

  Ofs_0E_01: Word = 0;
  Tok_0E_01: TToken = Token_ASM;
  ResWordLen_ASM: Byte = 3;
  ResWord_ASM: Array [1..3] of Char = 'ASM';

  Ofs_0E_02: Word = Ofs (Ofs_0E_01);
  Tok_0E_02: TToken = Token_FUNCTION;
  ResWordLen_FUNCTION: Byte = 8;
  ResWord_FUNCTION: Array [1..8] of Char = 'FUNCTION';

  Ofs_0E_03: Word = Ofs (Ofs_0E_02);
  Tok_0E_03: TToken = Token_NOT;
  ResWordLen_NOT: Byte = 3;
  ResWord_NOT: Array [1..3] of Char = 'NOT';

  Ofs_0E_04: Word = Ofs (Ofs_0E_03);
  Tok_0E_04: TToken = Token_TYPE;
  ResWordLen_TYPE: Byte = 4;
  ResWord_TYPE: Array [1..4] of Char = 'TYPE';

  Ofs_0F_01: Word = 0;
  Tok_0F_01: TToken = Token_OR;
  ResWordLen_OR: Byte = 2;
  ResWord_OR: Array [1..2] of Char = 'OR';


  ReservedWords: Word = $001E;

  OfsGroup_00: Word = Ofs (Ofs_00_05);
  OfsGroup_01: Word = Ofs (Ofs_01_05);
  OfsGroup_02: Word = Ofs (Ofs_02_02);
  OfsGroup_03: Word = Ofs (Ofs_03_02);
  OfsGroup_04: Word = Ofs (Ofs_04_04);
  OfsGroup_05: Word = Ofs (Ofs_05_05);
  OfsGroup_06: Word = Ofs (Ofs_06_02);
  OfsGroup_07: Word = Ofs (Ofs_07_01);
  OfsGroup_08: Word = Ofs (Ofs_08_04);
  OfsGroup_09: Word = Ofs (Ofs_09_03);
  OfsGroup_0A: Word = Ofs (Ofs_0A_02);
  OfsGroup_0B: Word = Ofs (Ofs_0B_04);
  OfsGroup_0C: Word = Ofs (Ofs_0C_03);
  OfsGroup_0D: Word = Ofs (Ofs_0D_02);
  OfsGroup_0E: Word = Ofs (Ofs_0E_04);
  OfsGroup_0F: Word = Ofs (Ofs_0F_01);
```

### Code Block 3

```pascal
Const NumResWords = 49;
      ReservedWord: Array [1..NumResWords] of String = (
        'AND',
        'ARRAY',
        'BEGIN',
        'CASE',
        'CONST',
        'CONSTRUCTOR',
        'DESTRUCTOR',
        'DIV',
        'DO',
        'DOWNTO',
        'ELSE',
        'END',
        'ASM',
        'FILE',
        'FOR',
        'FUNCTION',
        'GOTO',
        'IF',
        'IMPLEMENTATION',
        'IN',
        'INLINE',
        'INTERFACE',
        'LABEL',
        'MOD',
        'NIL',
        'NOT',
        'OBJECT',
        'OF',
        'OR',
        'PACKED',
        'PROCEDURE',
        'PROGRAM',
        'RECORD',
        'REPEAT',
        'SET',
        'SHL',
        'SHR',
        'STRING',
        'THEN',
        'TYPE',
        'TO',
        'UNIT',
        'UNTIL',
        'USES',
        'VAR',
        'INHERITED',
        'WHILE',
        'WITH',
        'XOR');
```

### Code Block 4

```pascal
Const

  Ofs_00_01: Word = 0;
  Tok_00_01: TToken = Token_AND;
  ResWordLen_AND: Byte = 3;
  ResWord_AND: Array [1..3] of Char = 'AND';

  Ofs_00_02: Word = Ofs (Ofs_00_01);
  Tok_00_02: TToken = Token_BEGIN;
  ResWordLen_BEGIN: Byte = 5;
  ResWord_BEGIN: Array [1..5] of Char = 'BEGIN';

  Ofs_00_03: Word = Ofs (Ofs_00_02);
  Tok_00_03: TToken = Token_DIV;
  ResWordLen_DIV: Byte = 3;
  ResWord_DIV: Array [1..3] of Char = 'DIV';

  Ofs_00_04: Word = Ofs (Ofs_00_03);
  Tok_00_04: TToken = Token_NIL;
  ResWordLen_NIL: Byte = 3;
  ResWord_NIL: Array [1..3] of Char = 'NIL';

  Ofs_00_05: Word = Ofs (Ofs_00_04);
  Tok_00_05: TToken = Token_PROCEDURE;
  ResWordLen_PROCEDURE: Byte = 9;
  ResWord_PROCEDURE: Array [1..9] of Char = 'PROCEDURE';

  Ofs_01_01: Word = 0;
  Tok_01_01: TToken = Token_DO;
  ResWordLen_DO: Byte = 2;
  ResWord_DO: Array [1..2] of Char = 'DO';

  Ofs_01_02: Word = Ofs (Ofs_01_01);
  Tok_01_02: TToken = Token_OBJECT;
  ResWordLen_OBJECT: Byte = 6;
  ResWord_OBJECT: Array [1..6] of Char = 'OBJECT';

  Ofs_01_03: Word = Ofs (Ofs_01_02);
  Tok_01_03: TToken = Token_PROGRAM;
  ResWordLen_PROGRAM: Byte = 7;
  ResWord_PROGRAM: Array [1..7] of Char = 'PROGRAM';

  Ofs_01_04: Word = Ofs (Ofs_01_03);
  Tok_01_04: TToken = Token_STRING;
  ResWordLen_STRING: Byte = 6;
  ResWord_STRING: Array [1..6] of Char = 'STRING';

  Ofs_01_05: Word = Ofs (Ofs_01_04);
  Tok_01_05: TToken = Token_TO;
  ResWordLen_TO: Byte = 2;
  ResWord_TO: Array [1..2] of Char = 'TO';

  Ofs_02_01: Word = 0;
  Tok_02_01: TToken = Token_CONST;
  ResWordLen_CONST: Byte = 5;
  ResWord_CONST: Array [1..5] of Char = 'CONST';

  Ofs_02_02: Word = Ofs (Ofs_02_01);
  Tok_02_02: TToken = Token_PACKED;
  ResWordLen_PACKED: Byte = 6;
  ResWord_PACKED: Array [1..6] of Char = 'PACKED';

  Ofs_03_01: Word = 0;
  Tok_03_01: TToken = Token_OF;
  ResWordLen_OF: Byte = 2;
  ResWord_OF: Array [1..2] of Char = 'OF';

  Ofs_03_02: Word = Ofs (Ofs_03_01);
  Tok_03_02: TToken = Token_INHERITED;
  ResWordLen_INHERITED: Byte = 9;
  ResWord_INHERITED: Array [1..9] of Char = 'INHERITED';

  Ofs_04_01: Word = 0;
  Tok_04_01: TToken = Token_END;
  ResWordLen_END: Byte = 3;
  ResWord_END: Array [1..3] of Char = 'END';

  Ofs_04_02: Word = Ofs (Ofs_04_01);
  Tok_04_02: TToken = Token_FOR;
  ResWordLen_FOR: Byte = 3;
  ResWord_FOR: Array [1..3] of Char = 'FOR';

  Ofs_04_03: Word = Ofs (Ofs_04_02);
  Tok_04_03: TToken = Token_SHL;
  ResWordLen_SHL: Byte = 3;
  ResWord_SHL: Array [1..3] of Char = 'SHL';

  Ofs_04_04: Word = Ofs (Ofs_04_03);
  Tok_04_04: TToken = Token_WHILE;
  ResWordLen_WHILE: Byte = 5;
  ResWord_WHILE: Array [1..5] of Char = 'WHILE';

  Ofs_05_01: Word = 0;
  Tok_05_01: TToken = Token_DESTRUCTOR;
  ResWordLen_DESTRUCTOR: Byte = 10;
  ResWord_DESTRUCTOR: Array [1..10] of Char = 'DESTRUCTOR';

  Ofs_05_02: Word = Ofs (Ofs_05_01);
  Tok_05_02: TToken = Token_DOWNTO;
  ResWordLen_DOWNTO: Byte = 6;
  ResWord_DOWNTO: Array [1..6] of Char = 'DOWNTO';

  Ofs_05_03: Word = Ofs (Ofs_05_02);
  Tok_05_03: TToken = Token_ELSE;
  ResWordLen_ELSE: Byte = 4;
  ResWord_ELSE: Array [1..4] of Char = 'ELSE';

  Ofs_05_04: Word = Ofs (Ofs_05_03);
  Tok_05_04: TToken = Token_GOTO;
  ResWordLen_GOTO: Byte = 4;
  ResWord_GOTO: Array [1..4] of Char = 'GOTO';

  Ofs_05_05: Word = Ofs (Ofs_05_04);
  Tok_05_05: TToken = Token_IN;
  ResWordLen_IN: Byte = 2;
  ResWord_IN: Array [1..2] of Char = 'IN';

  Ofs_06_01: Word = 0;
  Tok_06_01: TToken = Token_VAR;
  ResWordLen_VAR: Byte = 3;
  ResWord_VAR: Array [1..3] of Char = 'VAR';

  Ofs_06_02: Word = Ofs (Ofs_06_01);
  Tok_06_02: TToken = Token_XOR;
  ResWordLen_XOR: Byte = 3;
  ResWord_XOR: Array [1..3] of Char = 'XOR';

  Ofs_07_01: Word = 0;
  Tok_07_01: TToken = Token_UNTIL;
  ResWordLen_UNTIL: Byte = 5;
  ResWord_UNTIL: Array [1..5] of Char = 'UNTIL';

  Ofs_08_01: Word = 0;
  Tok_08_01: TToken = Token_CASE;
  ResWordLen_CASE: Byte = 4;
  ResWord_CASE: Array [1..4] of Char = 'CASE';

  Ofs_08_02: Word = Ofs (Ofs_08_01);
  Tok_08_02: TToken = Token_IMPLEMENTATION;
  ResWordLen_IMPLEMENTATION: Byte = 14;
  ResWord_IMPLEMENTATION: Array [1..14] of Char = 'IMPLEMENTATION';

  Ofs_08_03: Word = Ofs (Ofs_08_02);
  Tok_08_03: TToken = Token_INTERFACE;
  ResWordLen_INTERFACE: Byte = 9;
  ResWord_INTERFACE: Array [1..9] of Char = 'INTERFACE';

  Ofs_08_04: Word = Ofs (Ofs_08_03);
  Tok_08_04: TToken = Token_WITH;
  ResWordLen_WITH: Byte = 4;
  ResWord_WITH: Array [1..4] of Char = 'WITH';

  Ofs_09_01: Word = 0;
  Tok_09_01: TToken = Token_INLINE;
  ResWordLen_INLINE: Byte = 6;
  ResWord_INLINE: Array [1..6] of Char = 'INLINE';

  Ofs_09_02: Word = Ofs (Ofs_09_01);
  Tok_09_02: TToken = Token_RECORD;
  ResWordLen_RECORD: Byte = 6;
  ResWord_RECORD: Array [1..6] of Char = 'RECORD';

  Ofs_09_03: Word = Ofs (Ofs_09_02);
  Tok_09_03: TToken = Token_SET;
  ResWordLen_SET: Byte = 3;
  ResWord_SET: Array [1..3] of Char = 'SET';

  Ofs_0A_01: Word = 0;
  Tok_0A_01: TToken = Token_ARRAY;
  ResWordLen_ARRAY: Byte = 5;
  ResWord_ARRAY: Array [1..5] of Char = 'ARRAY';

  Ofs_0A_02: Word = Ofs (Ofs_0A_01);
  Tok_0A_02: TToken = Token_SHR;
  ResWordLen_SHR: Byte = 3;
  ResWord_SHR: Array [1..3] of Char = 'SHR';

  Ofs_0B_01: Word = 0;
  Tok_0B_01: TToken = Token_CONSTRUCTOR;
  ResWordLen_CONSTRUCTOR: Byte = 11;
  ResWord_CONSTRUCTOR: Array [1..11] of Char = 'CONSTRUCTOR';

  Ofs_0B_02: Word = Ofs (Ofs_0B_01);
  Tok_0B_02: TToken = Token_LABEL;
  ResWordLen_LABEL: Byte = 5;
  ResWord_LABEL: Array [1..5] of Char = 'LABEL';

  Ofs_0B_03: Word = Ofs (Ofs_0B_02);
  Tok_0B_03: TToken = Token_REPEAT;
  ResWordLen_REPEAT: Byte = 6;
  ResWord_REPEAT: Array [1..6] of Char = 'REPEAT';

  Ofs_0B_04: Word = Ofs (Ofs_0B_03);
  Tok_0B_04: TToken = Token_THEN;
  ResWordLen_THEN: Byte = 4;
  ResWord_THEN: Array [1..4] of Char = 'THEN';

  Ofs_0C_01: Word = 0;
  Tok_0C_01: TToken = Token_FILE;
  ResWordLen_FILE: Byte = 4;
  ResWord_FILE: Array [1..4] of Char = 'FILE';

  Ofs_0C_02: Word = Ofs (Ofs_0C_01);
  Tok_0C_02: TToken = Token_UNIT;
  ResWordLen_UNIT: Byte = 4;
  ResWord_UNIT: Array [1..4] of Char = 'UNIT';

  Ofs_0C_03: Word = Ofs (Ofs_0C_02);
  Tok_0C_03: TToken = Token_USES;
  ResWordLen_USES: Byte = 4;
  ResWord_USES: Array [1..4] of Char = 'USES';

  Ofs_0D_01: Word = 0;
  Tok_0D_01: TToken = Token_IF;
  ResWordLen_IF: Byte = 2;
  ResWord_IF: Array [1..2] of Char = 'IF';

  Ofs_0D_02: Word = Ofs (Ofs_0D_01);
  Tok_0D_02: TToken = Token_MOD;
  ResWordLen_MOD: Byte = 3;
  ResWord_MOD: Array [1..3] of Char = 'MOD';

  Ofs_0E_01: Word = 0;
  Tok_0E_01: TToken = Token_ASM;
  ResWordLen_ASM: Byte = 3;
  ResWord_ASM: Array [1..3] of Char = 'ASM';

  Ofs_0E_02: Word = Ofs (Ofs_0E_01);
  Tok_0E_02: TToken = Token_FUNCTION;
  ResWordLen_FUNCTION: Byte = 8;
  ResWord_FUNCTION: Array [1..8] of Char = 'FUNCTION';

  Ofs_0E_03: Word = Ofs (Ofs_0E_02);
  Tok_0E_03: TToken = Token_NOT;
  ResWordLen_NOT: Byte = 3;
  ResWord_NOT: Array [1..3] of Char = 'NOT';

  Ofs_0E_04: Word = Ofs (Ofs_0E_03);
  Tok_0E_04: TToken = Token_TYPE;
  ResWordLen_TYPE: Byte = 4;
  ResWord_TYPE: Array [1..4] of Char = 'TYPE';

  Ofs_0F_01: Word = 0;
  Tok_0F_01: TToken = Token_OR;
  ResWordLen_OR: Byte = 2;
  ResWord_OR: Array [1..2] of Char = 'OR';


  ReservedWords: Word = $001E;

  OfsGroup_00: Word = Ofs (Ofs_00_05);
  OfsGroup_01: Word = Ofs (Ofs_01_05);
  OfsGroup_02: Word = Ofs (Ofs_02_02);
  OfsGroup_03: Word = Ofs (Ofs_03_02);
  OfsGroup_04: Word = Ofs (Ofs_04_04);
  OfsGroup_05: Word = Ofs (Ofs_05_05);
  OfsGroup_06: Word = Ofs (Ofs_06_02);
  OfsGroup_07: Word = Ofs (Ofs_07_01);
  OfsGroup_08: Word = Ofs (Ofs_08_04);
  OfsGroup_09: Word = Ofs (Ofs_09_03);
  OfsGroup_0A: Word = Ofs (Ofs_0A_02);
  OfsGroup_0B: Word = Ofs (Ofs_0B_04);
  OfsGroup_0C: Word = Ofs (Ofs_0C_03);
  OfsGroup_0D: Word = Ofs (Ofs_0D_02);
  OfsGroup_0E: Word = Ofs (Ofs_0E_04);
  OfsGroup_0F: Word = Ofs (Ofs_0F_01);
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
- https://turbopascal.org/reserved-words
- https://turbopascal.org/scanner
- https://turbopascal.org/statements
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

