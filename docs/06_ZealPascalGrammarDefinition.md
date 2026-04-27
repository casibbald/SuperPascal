# 📘 ZealPascal Language Specification v0.2 — Grammar Definition (EBNF Draft)

This document defines the **formal grammar** for ZealPascal in Extended Backus–Naur Form (EBNF). It corresponds to the high-level language tiers established in v0.1, covering:

* ISO 7185 Pascal Core (Tier 1)
* Turbo Pascal Extensions (Tier 2)
* ZealPascal Object Pascal Subset (Tier 3)
* ZealPascal-specific extensions (Tilemap DSL, directives, intrinsics)

This is a **compiler-facing grammar**, intended for implementation in the ZealPascal parser. Semantic rules are defined later (v0.3).

---

# 1. Notation

```
A      ::= production
A | B  ::= choice
A?     ::= optional
A*     ::= zero or more
A+     ::= one or more
"x"    ::= literal terminal

Identifiers, keywords, literals, and comments follow the lexical rules defined in the ZealPascal Lexical Specification (forthcoming).
```

---

# 2. Program Structure

```
program         ::= "program" ident ";" block "." ;
unit            ::= "unit" ident ";" interface-part implementation-part init-part? "." ;

interface-part  ::= "interface" decl-section* ;
implementation-part ::= "implementation" decl-section* ;
init-part       ::= "begin" statement-seq "end" ;

decl-section    ::= label-section
                  | const-section
                  | type-section
                  | var-section
                  | routine-decl
                  | class-decl
                  ;
```

---

# 3. Blocks & Declarations

```
block           ::= decl-section* "begin" statement-seq "end" ;
label-section   ::= "label" label ("," label)* ";" ;

const-section   ::= "const" const-decl+ ;
const-decl      ::= ident "=" const-expr ";" ;

type-section    ::= "type" type-decl+ ;
type-decl       ::= ident "=" type-spec ";" ;

var-section     ::= "var" var-decl+ ;
var-decl        ::= ident-list ":" type-spec ";" ;

ident-list      ::= ident ("," ident)* ;
```

---

# 4. Type Specifications

```
type-spec       ::= simple-type
                  | subrange-type
                  | enum-type
                  | array-type
                  | record-type
                  | set-type
                  | pointer-type
                  | class-type
                  ;

simple-type     ::= ident ;

subrange-type   ::= const-expr ".." const-expr ;

enum-type       ::= "(" ident ("," ident)* ")" ;

array-type      ::= "array" "[" index-range-list "]" "of" type-spec ;
index-range-list::= subrange-type ("," subrange-type)* ;

record-type     ::= "record" field-decl* "end" ;
field-decl      ::= ident-list ":" type-spec ";" ;

set-type        ::= "set" "of" ordinal-type ;
ordinal-type    ::= simple-type | subrange-type | enum-type ;

pointer-type    ::= "^" type-spec ;
```

---

# 5. Object Pascal Subset — Classes

```
class-type      ::= "class" class-ancestor? class-body "end" ;
class-ancestor  ::= "(" ident ")" ;

class-body      ::= class-section* ;

class-section   ::= visibility ":" class-member* ;
visibility      ::= "public" | "private" | "protected" ;

class-member    ::= field-decl
                  | method-decl
                  ;
```

## 5.1 Methods

```
method-decl     ::= proc-heading method-modifiers? ";"
                  | func-heading method-modifiers? ":" type-spec ";"
                  ;

method-modifiers::= ("virtual" | "override") ;
```

---

# 6. Routines: Procedures & Functions

```
routine-decl    ::= proc-decl | func-decl ;

proc-decl       ::= proc-heading ";" block ";" ;
func-decl       ::= func-heading ";" block ";" ;

proc-heading    ::= "procedure" ident param-list? ;
func-heading    ::= "function" ident param-list? ":" type-spec ;

param-list      ::= "(" param-group (";" param-group)* ")" ;
param-group     ::= var-param? ident-list ":" type-spec ;
var-param       ::= "var" ;
```

---

# 7. Statements

```
statement       ::= simple-stmt | structured-stmt | empty ;
empty           ::= ;

simple-stmt     ::= assign-stmt
                  | call-stmt
                  | goto-stmt
                  | intrinsic-stmt
                  ;

assign-stmt     ::= variable ":=" expr ;
call-stmt       ::= ident param-args? ;
goto-stmt       ::= "goto" label ;

param-args      ::= "(" expr-list? ")" ;
expr-list       ::= expr ("," expr)* ;
```

### 7.1 Structured Statements

```
structured-stmt ::= if-stmt | while-stmt | repeat-stmt | for-stmt
                  | case-stmt | with-stmt | try-stmt ;

if-stmt         ::= "if" expr "then" statement ("else" statement)? ;
while-stmt      ::= "while" expr "do" statement ;
repeat-stmt     ::= "repeat" statement-seq "until" expr ;

for-stmt        ::= "for" ident ":=" expr ("to" | "downto") expr "do" statement ;

case-stmt       ::= "case" expr "of" case-branch+ "end" ;
case-branch     ::= case-labels ":" statement ";" ;
case-labels     ::= const-expr ("," const-expr)* ;
```

### 7.2 Exception Handling (Later Tier)

```
try-stmt        ::= "try" statement-seq
                     ("except" statement-seq)?
                     ("finally" statement-seq)?
                     "end" ;
```

---

# 8. Expressions

```
expr            ::= simple-expr (rel-op simple-expr)? ;
simple-expr     ::= add-op? term (add-op term)* ;
term            ::= factor (mul-op factor)* ;
```

### 8.1 Operators

```
add-op          ::= "+" | "-" | "or" ;
mul-op          ::= "*" | "/" | "div" | "mod" | "and" ;
rel-op          ::= "=" | "<>" | "<" | "<=" | ">" | ">=" ;
```

### 8.2 Factors

```
factor          ::= ident
                  | literal
                  | "(" expr ")"
                  | "not" factor
                  | set-constructor
                  | pointer-deref
                  ;

pointer-deref   ::= "^" factor ;
```

---

# 9. ZealPascal Extensions

## 9.1 Intrinsics

```
intrinsic-stmt  ::= intrinsic-name param-args? ;

intrinsic-name  ::= "WaitVBlank" | "WriteReg" | "DMA_Copy"
                  | "MapPage" | "SpriteSet" | "SpriteShow"
                  | "FadeMusic" | "Crossfade"
                  ;
```

## 9.2 Tilemap DSL

```
tilemap-ctor    ::= "(" "Width:" number ";"
                        "Height:" number ";"
                        "Data:" "(" tilemap-rows ")" ")" ;

tilemap-rows    ::= tilemap-row ("," tilemap-row)* ;
tilemap-row     ::= "ROW" "(" string ")" ;
```

## 9.3 Compiler Directives

```
directive       ::= "{$" directive-name directive-args? "}" ;
directive-name  ::= "INLINE" | "UNROLL" | "VBLANK_AWARE" ;
```

---

# 10. Grammar Coverage Summary

This v0.2 grammar includes:

* Complete Pascal core syntax
* Type system constructs
* Procedures, functions, units
* Object Pascal subset (classes, methods, modifiers)
* Statements, expressions, operators
* Exception syntax (later phase)
* Intrinsics for ZVB, ECS, and audio systems
* Tilemap DSL syntax
* Compiler directives

Remaining work for v0.3:

* Formal lexical specification
* Semantic typing rules
* Operator precedence tables
* Class inheritance & vtable resolution rules
* Unit import/export resolution
* Error recovery productions for the parser

---

# 11. Next Steps

Tell me when you want to proceed to:

* **ZealPascal v0.3 Semantic Rules Specification**, or
* **ZealPascal v0.2.1 Grammar Expansion (full operator precedence charts + ambiguities resolved)**.
