# Grammar Reference

- Status: partially-verified
- Source docs: `languageSpecification/02_Grammar.md`, `languageSpecification/01_LexicalStructure.md`
- Code anchors: `parser/src/lib.rs`, `parser/src/expressions.rs`, `parser/src/statements.rs`, `parser/src/declarations.rs`

## Grammar overview

SuperPascal extends classic Pascal with modern features. The grammar is parsed via recursive descent.

## Expression grammar (highest priority)

```
Expression := AdditiveExpr
AdditiveExpr := MultiplicativeExpr ( ( '+' | '-' ) MultiplicativeExpr )*
MultiplicativeExpr := UnaryExpr ( ( '*' | '/' | 'div' | 'mod' | 'and' ) UnaryExpr )*
UnaryExpr := ( '-' | 'not' ) UnaryExpr | PrimaryExpr
PrimaryExpr := Identifier | Literal | '(' Expression ')' | FunctionCall | ArrayAccess | FieldAccess | Lambda
```

Implemented in: `parser/src/expressions.rs`

## Statement grammar

```
Statement :=
  | Assignment
  | IfStatement
  | WhileStatement
  | ForStatement
  | RepeatStatement
  | CaseStatement
  | ProcedureCall
  | BeginEndBlock
  | EmptyStatement

IfStatement := 'if' Expression 'then' Statement [ 'else' Statement ]
WhileStatement := 'while' Expression 'do' Statement
ForStatement := 'for' Identifier ':=' Expression 'to' Expression 'do' Statement
RepeatStatement := 'repeat' StatementList 'until' Expression
CaseStatement := 'case' Expression 'of' CaseList [ 'else' StatementList ] 'end'
```

Implemented in: `parser/src/statements.rs`

## Declaration grammar

```
Declaration :=
  | TypeDeclaration
  | ConstDeclaration
  | VarDeclaration
  | ProcedureDeclaration
  | FunctionDeclaration
  | UnitDeclaration
  | ClassDeclaration
  | PropertyDeclaration
  | ForwardDeclaration

TypeDeclaration := 'type' Identifier '=' Type
Type := SimpleType | StructuredType | PointerType | ProcedureType
ProcedureDeclaration := [ 'function' | 'procedure' ] Identifier FormalParams [ ': ReturnType' ] [ 'forward' ] Block
ClassDeclaration := 'class' Identifier [ '(' ParentClass ')' ] [ 'of object' ] ClassBody
ClassBody := '{' ClassMember { ClassMember } '}'
PropertyDeclaration := 'class property' Identifier ':' Type [ 'read' Field ] [ 'write' Field ]
```

Implemented in: `parser/src/declarations.rs`

## Type grammar

```
Type :=
  | BasicType
  | ArrayType [ 'of' Type ]
  | RecordType [ 'record' FieldList 'end' ]
  | ClassType
  | PointerType [ '^' Type ]
  | ProcedureType [ 'of object' ]
  | DynamicArrayType 'of' Type
  | SubrangeType
  | SetType [ 'of' Type ]

BasicType := 'integer' | 'real' | 'char' | 'string' | 'boolean' | 'byte' | 'word' | 'longint' | 'longint' | 'dword' | 'qword' | 'float' | 'double'
```

Implemented in: `parser/src/types.rs`

## Modern extensions (beyond classic Pascal)

- `for x in collection do` (for..in loops)
- `class property` and `class var` (class-level members)
- Generics: `List<T>` with constraints
- Anonymous functions (lambdas)
- Attributes: `[Attribute(arg)]`
- Compiler directives: `{$DEFINE}`, `{$IFDEF}`, `{$IF}`, `{$ELSE}`, `{$ENDIF}`, `{$I}`
- Dynamic arrays: `array of T`
- Record methods
- Inline variable declarations

## Grammar docs

- `languageSpecification/02_Grammar.md` — BNF grammar spec
- `languageSpecification/01_LexicalStructure.md` — token definitions, lexical rules
- `languageSpecification/03_TypeSystem.md` — type system rules
