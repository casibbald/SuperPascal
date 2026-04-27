# Z80 Micro-Compiler Minimal Pascal Subset

## Overview

This document defines the **minimal Pascal subset** for the **Tier 1 micro-compiler** that runs directly on the Zeal 8-bit Computer (Z80). This compiler is designed for:

- **Educational purposes:** Students can study compiler internals
- **On-device compilation:** Compile small programs directly on Zeal
- **Pedagogical clarity:** Simple, one-pass compiler following Wirth's principles
- **Foundation for learning:** Gateway to understanding full SuperPascal

**Design Philosophy:**
- **Wirth-style simplicity:** One-pass compilation, straightforward codegen
- **Minimal but complete:** Enough to write meaningful programs
- **Z80-friendly:** Optimized for 8-bit constraints
- **Educational focus:** Clear, understandable implementation

---

## Language Subset (Tier 1)

### 1. Lexical Structure

#### Identifiers
- Alphanumeric characters + underscore
- Case-insensitive
- Max length: 31 characters (Z80 memory constraints)

#### Keywords
```
program, var, const, type, procedure, function,
begin, end, if, then, else, while, do, for, to, downto,
repeat, until, case, of, and, or, not, div, mod,
integer, boolean, char, byte, word
```

#### Literals
- **Integers:** Decimal (`123`), hex (`$FF`)
- **Booleans:** `true`, `false`
- **Characters:** `'A'`, `' '`
- **Strings:** `'Hello'` (max 255 chars)

#### Operators
```
+  -  *  /  div  mod
=  <>  <  <=  >  >=
and  or  not
```

#### Comments
- `{ ... }` (standard Pascal)
- `(* ... *)` (alternative)

**Excluded:**
- `//` comments (Tier 2 only)
- Compiler directives `{$...}` (Tier 2 only)
- Tilemap DSL (Tier 2 only)

---

### 2. Types

#### Primitive Types
```pascal
integer    // 16-bit signed (-32768..32767)
byte       // 8-bit unsigned (0..255)
word       // 16-bit unsigned (0..65535)
boolean    // true/false
char       // 8-bit ASCII
```

**Excluded:**
- `real` / floating-point (Tier 2 only)
- `string` type (Tier 2 only)
- Pointers (Tier 2 only)
- Sets (Tier 2 only)

#### Arrays
```pascal
var
  arr: array[1..10] of integer;
  matrix: array[0..9, 0..9] of byte;
```

- Static arrays only
- One or two dimensions
- Index bounds: 0..255 (byte range)

**Excluded:**
- Dynamic arrays
- Packed arrays (Tier 2 only)
- Multi-dimensional beyond 2D

#### Records
```pascal
type
  TPoint = record
    X, Y: integer;
  end;
```

- Simple records only
- No record methods (Tier 2 only)
- No variant records
- No packed records (Tier 2 only)

**Excluded:**
- Classes (Tier 3 only)
- Pointers to records (Tier 2 only)

---

### 3. Constants and Variables

#### Constants
```pascal
const
  MAX_SIZE = 100;
  PI = 3.14159;  // Not supported (no real)
  TRUE_VALUE = true;
```

- Integer, boolean, char constants only
- No string constants (Tier 2 only)

#### Variables
```pascal
var
  x, y: integer;
  flag: boolean;
  ch: char;
  arr: array[1..10] of integer;
```

- Global and local variables
- No initialization in declaration (Tier 2 only)

---

### 4. Expressions

#### Arithmetic
```pascal
x := 10 + 20;
y := x * 2;
z := x div 3;  // Integer division
w := x mod 5;  // Modulo
```

#### Relational
```pascal
if x = 10 then ...
if x <> 0 then ...
if x < 100 then ...
if x <= 50 then ...
if x > 0 then ...
if x >= 10 then ...
```

#### Boolean
```pascal
if (x > 0) and (x < 100) then ...
if (flag) or (x = 0) then ...
if not flag then ...
```

**Excluded:**
- String concatenation (Tier 2 only)
- Set operations (Tier 2 only)
- Pointer operations (Tier 2 only)

---

### 5. Statements

#### Assignment
```pascal
x := 10;
arr[5] := 20;
point.X := 100;
```

#### Conditional
```pascal
if x > 0 then
  y := 1
else
  y := 0;

if flag then
  x := 10;
```

#### Loops
```pascal
while x > 0 do
begin
  x := x - 1;
end;

for i := 1 to 10 do
  arr[i] := i * 2;

for i := 10 downto 1 do
  arr[i] := i;

repeat
  x := x - 1;
until x = 0;
```

#### Case
```pascal
case x of
  1: y := 10;
  2: y := 20;
  3: y := 30;
else
  y := 0;
end;
```

**Excluded:**
- `break` / `continue` (Tier 2 only)
- `goto` (never supported)
- `exit` (Tier 2 only)

---

### 6. Procedures and Functions

#### Procedures
```pascal
procedure PrintNumber(n: integer);
begin
  // Implementation
end;
```

#### Functions
```pascal
function Add(a, b: integer): integer;
begin
  Add := a + b;
end;
```

#### Parameters
- Value parameters: `procedure Foo(x: integer);`
- Reference parameters: `procedure Foo(var x: integer);`
- No `const` parameters (Tier 2 only)
- No default parameters
- No variadic functions

**Excluded:**
- Nested procedures (Tier 2 only)
- Forward declarations (Tier 2 only)
- Function/procedure types (Tier 2 only)
- Overloading (Tier 2 only)

---

### 7. Program Structure

#### Simple Program
```pascal
program Hello;
var
  x: integer;
begin
  x := 10;
  WriteLn(x);
end.
```

#### With Procedures/Functions
```pascal
program Calculator;

procedure Add(a, b: integer);
begin
  WriteLn(a + b);
end;

function Multiply(a, b: integer): integer;
begin
  Multiply := a * b;
end;

var
  result: integer;
begin
  Add(10, 20);
  result := Multiply(5, 6);
  WriteLn(result);
end.
```

**Excluded:**
- Units (Tier 2 only)
- Initialization sections (Tier 2 only)

---

### 8. Standard Procedures and Functions

#### Input/Output
```pascal
Write(x);        // Write integer
WriteLn(x);      // Write integer + newline
Write(ch);       // Write character
WriteLn;         // Write newline only
Read(x);         // Read integer
ReadLn(x);       // Read integer + consume newline
Read(ch);        // Read character
ReadLn;          // Consume newline
```

#### Arithmetic
```pascal
Abs(x)           // Absolute value
Sqr(x)           // Square
```

#### Boolean
```pascal
Odd(x)           // Is odd?
```

#### Type Conversion
```pascal
Ord(ch)          // Character to integer
Chr(n)           // Integer to character
```

**Excluded:**
- String functions (Tier 2 only)
- Math functions (Tier 2 only)
- File I/O (Tier 2 only)

---

## Compiler Architecture

### One-Pass Compilation

Following Wirth's "Compiler Construction" principles:

```
Source Text
    │
    ▼
Lexer (Token Stream)
    │
    ▼
Parser (AST + Symbol Table)
    │
    ▼
Code Generator (Z80 Assembly)
    │
    ▼
Object File (.ZOF)
```

### Key Constraints

1. **Single Pass:** Parse and generate code in one pass
2. **Forward Declarations:** Not supported (simplifies symbol table)
3. **No Separate Type Checking:** Type checking integrated into parser
4. **Simple Symbol Table:** Flat or shallow nesting only
5. **Direct Code Generation:** No IR (too complex for Z80)

---

## Implementation Guidelines

### 1. Lexer

**Requirements:**
- Single-pass scanner
- Minimal memory usage
- Token lookahead (1-2 tokens)

**Z80 Optimizations:**
- Use lookup tables for keywords
- Cache current token
- Avoid string allocations where possible

### 2. Parser

**Strategy:** Recursive descent

**Grammar (EBNF):**
```
program = "program" identifier ";" block "." .

block = [declarations] "begin" statements "end" .

declarations = {const_decl | var_decl | type_decl | proc_decl | func_decl} .

statements = statement {";" statement} .

statement = assignment | if_statement | while_statement | 
            for_statement | repeat_statement | case_statement |
            procedure_call | block .

expression = simple_expression [relational_op simple_expression] .

simple_expression = [sign] term {additive_op term} .

term = factor {multiplicative_op factor} .

factor = identifier | number | char_literal | boolean_literal |
         "(" expression ")" | "not" factor | function_call .
```

**Error Recovery:**
- Panic mode: Skip to next statement
- Report errors but continue parsing

### 3. Symbol Table

**Structure:**
```pascal
type
  SymbolKind = (skVar, skConst, skType, skProc, skFunc);
  
  Symbol = record
    Name: string[31];
    Kind: SymbolKind;
    Type: TypeInfo;
    Address: word;  // Stack offset or global address
  end;
```

**Scope:**
- Global scope only (no nested procedures)
- Simple hash table or linear search
- Max symbols: ~256 (Z80 memory constraints)

### 4. Type System

**Type Representation:**
```pascal
type
  TypeKind = (tkInteger, tkByte, tkWord, tkBoolean, tkChar, 
              tkArray, tkRecord);
  
  TypeInfo = record
    Kind: TypeKind;
    Size: word;      // Size in bytes
    // Array/Record details...
  end;
```

**Type Checking:**
- Assignment compatibility
- Expression type inference
- Parameter type matching
- Return type checking

### 5. Code Generation

**Z80 Assembly Output:**
- Direct assembly generation (no IR)
- sjasmplus-compatible syntax
- ABI compliance:
  - `IX` as frame pointer
  - Parameters on stack
  - Return values in `HL` (16-bit) or `A` (8-bit)

**Code Generation Strategy:**
- Expression evaluation on stack
- Register allocation: `HL`, `DE`, `BC` for temporaries
- Stack frame management via `IX`
- Simple peephole optimizations (optional)

**Example Codegen:**
```pascal
// Source: x := 10 + 20;
// Generated:
  ld hl, 10
  push hl
  ld hl, 20
  pop de
  add hl, de
  ld (x), hl
```

---

## Memory Layout

### Program Memory Map (Z80)

```
$0000 - $3FFF  : ROM (ZealOS)
$4000 - $7FFF  : RAM (available)
$8000 - $FFFF  : RAM (available)

Compiler uses:
- $4000 - $4FFF : Symbol table
- $5000 - $5FFF : Code generation buffer
- $6000 - $6FFF : Stack
- $7000 - $7FFF : Global variables
```

### Stack Frame Layout

```
High Memory
    │
    ├─ Return Address (2 bytes)
    ├─ Old IX (2 bytes)
    ├─ Local Variables
    ├─ Parameters (pushed right-to-left)
    │
Low Memory (SP)
```

---

## Limitations

### Memory Constraints
- **Max program size:** ~16KB (Z80 address space)
- **Max symbols:** ~256
- **Max array size:** 255 elements (byte index)
- **Max string length:** 255 characters
- **Stack depth:** ~1KB

### Performance Constraints
- **Compilation time:** Acceptable for small programs (< 1 minute)
- **Code quality:** Basic optimizations only
- **Error messages:** Simple, clear messages

### Language Constraints
- **No recursion:** Stack depth limits
- **No dynamic allocation:** Static memory only
- **No file I/O:** Console I/O only
- **No units:** Single-file programs only

---

## Example Programs

### Example 1: Hello World
```pascal
program Hello;
begin
  WriteLn('Hello, Zeal!');
end.
```

### Example 2: Factorial
```pascal
program Factorial;

function Fact(n: integer): integer;
var
  result: integer;
  i: integer;
begin
  result := 1;
  for i := 1 to n do
    result := result * i;
  Fact := result;
end;

var
  n: integer;
begin
  Write('Enter n: ');
  ReadLn(n);
  WriteLn('Factorial: ', Fact(n));
end.
```

### Example 3: Array Processing
```pascal
program ArrayDemo;

var
  arr: array[1..10] of integer;
  i: integer;
  sum: integer;
begin
  // Initialize array
  for i := 1 to 10 do
    arr[i] := i * 2;
  
  // Sum array
  sum := 0;
  for i := 1 to 10 do
    sum := sum + arr[i];
  
  WriteLn('Sum: ', sum);
end.
```

---

## Testing Strategy

### Unit Tests
- Lexer: Token recognition
- Parser: Grammar conformance
- Type checker: Type compatibility
- Codegen: Assembly correctness

### Integration Tests
- End-to-end compilation
- Program execution on emulator
- Memory usage validation
- Error message quality

### Test Programs
- Simple arithmetic
- Control flow (if, loops)
- Procedures and functions
- Arrays and records
- Edge cases (bounds, overflow)

---

## Educational Value

### Learning Objectives
1. **Lexical Analysis:** Token recognition, keyword lookup
2. **Parsing:** Recursive descent, grammar implementation
3. **Symbol Tables:** Name resolution, scope management
4. **Type Checking:** Type compatibility, inference
5. **Code Generation:** Assembly generation, register allocation
6. **Compiler Architecture:** One-pass compilation, error handling

### Pedagogical Benefits
- **Simplicity:** Easy to understand and modify
- **Completeness:** Can compile real programs
- **Hands-on:** Students can run compiler on device
- **Foundation:** Prepares for full SuperPascal compiler

---

## Migration Path to Tier 2

When students are ready for the full compiler:

1. **Add missing features:**
   - Units
   - Strings
   - Pointers
   - Sets
   - More control flow

2. **Improve compiler:**
   - Multi-pass compilation
   - IR generation
   - Better optimizations
   - More error recovery

3. **Platform expansion:**
   - Support 65C816
   - Support 68060
   - Cross-compilation

---

## Implementation Checklist

### Phase 1: Lexer
- [ ] Token recognition
- [ ] Keyword table
- [ ] Number/string parsing
- [ ] Comment handling
- [ ] Error reporting

### Phase 2: Parser
- [ ] Grammar implementation
- [ ] AST construction
- [ ] Error recovery
- [ ] Expression parsing

### Phase 3: Symbol Table
- [ ] Symbol storage
- [ ] Name resolution
- [ ] Type storage
- [ ] Scope management

### Phase 4: Type Checking
- [ ] Type compatibility
- [ ] Expression typing
- [ ] Assignment checking
- [ ] Parameter matching

### Phase 5: Code Generation
- [ ] Expression codegen
- [ ] Statement codegen
- [ ] Procedure codegen
- [ ] Stack frame management
- [ ] Register allocation

### Phase 6: Integration
- [ ] End-to-end compilation
- [ ] Error message formatting
- [ ] Object file generation
- [ ] Testing suite

---

## References

- **Wirth, N. (1996):** "Compiler Construction" - One-pass compiler design
- **Wirth, N. (2005):** "Compiler Construction" (revised) - Updated principles
- **Zeal 8-bit Computer:** Hardware specifications
- **Z80 CPU:** Instruction set reference

---

**Version:** 1.0  
**Date:** 2024  
**Status:** Specification for Tier 1 Micro-Compiler

