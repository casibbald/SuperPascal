# SuperPascal Rust Compiler Crate Layout Proposal

## Overview

This document proposes a **modular Rust crate structure** for the SuperPascal compiler (`compiler-rs`), inspired by FreePascal's phase-based architecture but organized for modern Rust development and multi-platform support.

**Key Principles:**
- **Separation of concerns:** Each crate handles one compiler phase
- **Platform-agnostic frontend:** Shared lexer, parser, semantics
- **Platform-specific backends:** Separate codegen crates per platform
- **Testability:** Each crate is independently testable
- **Extensibility:** Easy to add new platforms or features

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend (Platform-Agnostic)            │
│  lexer → parser → ast → semantics → types → symbol_table   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    IR (Platform-Agnostic)                   │
│                    ir1 → ir2 (optional)                      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              Backends (Platform-Specific)                    │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                   │
│  │ zealz80  │  │foenix816 │  │foenix68k│                   │
│  └──────────┘  └──────────┘  └──────────┘                   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              Object Writers (Platform-Specific)              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                   │
│  │  .ZOF    │  │  .FOF    │  │  .AOF    │                   │
│  └──────────┘  └──────────┘  └──────────┘                   │
└─────────────────────────────────────────────────────────────┘
```

---

## Proposed Crate Structure

### Workspace Root: `crates/compiler-rs/`

```
compiler-rs/
├── Cargo.toml                    # Workspace manifest
├── README.md
│
├── crates/
│   ├── lexer/        # Lexical analysis
│   ├── parser/        # Syntax analysis
│   ├── ast/           # AST node definitions
│   ├── tokens/        # Token definitions
│   ├── semantics/     # Semantic analysis
│   ├── types/        # Type system
│   ├── symbols/       # Symbol table
│   ├── ir/            # Intermediate representation
│   ├── errors/        # Error handling
│   ├── diagnostics/   # Diagnostic formatting
│   │
│   ├── backend-zealz80/    # Z80 codegen
│   ├── backend-foenix816/  # 65C816 codegen
│   ├── backend-foenix68k/  # 68060 codegen
│   │
│   ├── object-zealz80/     # .ZOF writer
│   ├── object-foenix816/    # .FOF writer
│   ├── object-foenix68k/   # .AOF writer
│   │
│   ├── runtime-spec/  # Runtime ABI contracts
│   │
│   └── driver/        # CLI driver (main binary)
│
└── tests/                         # Integration tests
```

---

## Crate Descriptions

### 1. `lexer`

**Purpose:** Convert source text → token stream.

**Dependencies:**
- `tokens` (token definitions)

**Public API:**
```rust
pub struct Lexer {
    source: &str,
    position: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self;
    pub fn next_token(&mut self) -> Result<Token, LexError>;
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError>;
}
```

**Features:**
- Single-pass scanning
- Pascal comments: `{ }`, `(* *)`, `//`
- Compiler directive tokenization: `{$...}`
- Tilemap DSL: `ROW("...")`
- Zero-allocation token streaming (optional)

**Tests:**
- Token patterns
- Malformed input handling
- Directive parsing
- Unicode/ASCII handling

---

### 2. `tokens`

**Purpose:** Define all token types and keyword tables.

**Dependencies:** None (pure data structures)

**Public API:**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Program, Unit, Uses, Interface, Implementation,
    Var, Const, Type, Procedure, Function,
    Begin, End, If, Then, Else, While, For, Repeat, Until,
    Case, Of, Try, Except, Finally, Raise,
    Class, Constructor, Destructor, Virtual, Override,
    // Operators
    Plus, Minus, Multiply, Divide, Mod, Div,
    Equal, NotEqual, Less, LessEqual, Greater, GreaterEqual,
    And, Or, Not, Xor,
    // Literals
    IntegerLiteral(u16),
    StringLiteral(String),
    BooleanLiteral(bool),
    // Identifiers
    Identifier(String),
    // Special
    Dot, Comma, Semicolon, Colon, Assign, Arrow,
    LeftParen, RightParen, LeftBracket, RightBracket,
    // Directives
    Directive(String),
    // EOF
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,  // Source location
}

pub struct KeywordTable;
impl KeywordTable {
    pub fn lookup(ident: &str) -> Option<TokenKind>;
}
```

**Features:**
- Complete token set
- Keyword lookup table
- Operator precedence (for parser)
- Source span tracking

---

### 3. `parser`

**Purpose:** Convert token stream → AST.

**Dependencies:**
- `tokens`
- `ast`
- `errors`

**Public API:**
```rust
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self;
    pub fn parse(&mut self) -> Result<ast::Program, ParseError>;
    pub fn parse_unit(&mut self) -> Result<ast::Unit, ParseError>;
    pub fn parse_expression(&mut self) -> Result<ast::Expr, ParseError>;
}
```

**Features:**
- Recursive descent parsing
- Pratt parser for expressions
- Error recovery (panic mode)
- Full SuperPascal grammar support
- Tilemap DSL parsing

**Tests:**
- Grammar conformance
- Parse tree validation
- Error recovery
- Edge cases

---

### 4. `ast`

**Purpose:** Define AST node structures.

**Dependencies:**
- `tokens` (for spans)

**Public API:**
```rust
// Program structure
pub struct Program {
    pub name: String,
    pub uses: Vec<String>,
    pub block: Block,
    pub span: Span,
}

pub struct Unit {
    pub name: String,
    pub interface: InterfaceSection,
    pub implementation: ImplementationSection,
    pub span: Span,
}

// Declarations
pub enum Decl {
    Var(VarDecl),
    Const(ConstDecl),
    Type(TypeDecl),
    Proc(ProcDecl),
    Func(FuncDecl),
}

// Types
pub enum Type {
    Primitive(PrimitiveType),
    Array(ArrayType),
    Record(RecordType),
    Set(SetType),
    Pointer(PointerType),
    Class(ClassType),
    // ...
}

// Statements
pub enum Stmt {
    Assign(AssignStmt),
    If(IfStmt),
    While(WhileStmt),
    For(ForStmt),
    Case(CaseStmt),
    Try(TryStmt),
    Raise(RaiseStmt),
    // ...
}

// Expressions
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(LiteralExpr),
    Call(CallExpr),
    Field(FieldExpr),
    Index(IndexExpr),
    // ...
}
```

**Features:**
- Immutable AST nodes
- Source span preservation
- Arena allocation support (via `bumpalo` or similar)
- Visitor pattern support

---

### 5. `symbols`

**Purpose:** Symbol table and scope management.

**Dependencies:**
- `ast`
- `types`

**Public API:**
```rust
pub struct SymbolTable {
    scopes: Vec<Scope>,
}

pub struct Scope {
    symbols: HashMap<String, Symbol>,
    parent: Option<usize>,  // Parent scope index
}

pub enum Symbol {
    Var(VarSymbol),
    Const(ConstSymbol),
    Type(TypeSymbol),
    Proc(ProcSymbol),
    Func(FuncSymbol),
    Class(ClassSymbol),
    Unit(UnitSymbol),
}

impl SymbolTable {
    pub fn new() -> Self;
    pub fn enter_scope(&mut self) -> usize;
    pub fn leave_scope(&mut self);
    pub fn define(&mut self, name: String, symbol: Symbol) -> Result<(), SymbolError>;
    pub fn lookup(&self, name: &str) -> Option<&Symbol>;
    pub fn lookup_in_scope(&self, name: &str, scope: usize) -> Option<&Symbol>;
}
```

**Features:**
- Nested scopes (stack model)
- Unit-level interface/implementation separation
- Class member tables
- VTable slot tracking
- Overload resolution support

---

### 6. `types`

**Purpose:** Type system and type checking.

**Dependencies:**
- `ast`

**Public API:**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    Array(ArrayType),
    Record(RecordType),
    Set(SetType),
    Pointer(PointerType),
    Class(ClassType),
    Proc(ProcType),
    Func(FuncType),
}

pub struct ArrayType {
    pub element: Box<Type>,
    pub bounds: ArrayBounds,
}

pub struct RecordType {
    pub fields: Vec<Field>,
    pub size: usize,
    pub alignment: usize,
}

pub struct ClassType {
    pub base: Option<Box<ClassType>>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub vtable: VTableLayout,
}

impl Type {
    pub fn size(&self) -> usize;
    pub fn alignment(&self) -> usize;
    pub fn is_assignable_to(&self, other: &Type) -> bool;
    pub fn is_compatible_with(&self, other: &Type) -> bool;
}
```

**Features:**
- Complete type system
- Layout calculation (sizes, offsets)
- Type compatibility rules
- Record/class field offset computation
- Set cardinality calculation

---

### 7. `semantics`

**Purpose:** Semantic analysis and type checking.

**Dependencies:**
- `ast`
- `types`
- `symbols`
- `errors`

**Public API:**
```rust
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    type_system: TypeSystem,
    errors: Vec<SemanticError>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self;
    pub fn analyze(&mut self, program: &ast::Program) -> Result<TypedProgram, SemanticError>;
    pub fn check_types(&mut self, expr: &ast::Expr) -> Result<Type, SemanticError>;
    pub fn resolve_symbol(&mut self, name: &str) -> Result<Symbol, SemanticError>;
    pub fn check_assignment(&mut self, lhs: &Type, rhs: &Type) -> Result<(), SemanticError>;
}
```

**Features:**
- Type checking
- Symbol resolution
- Constant folding
- Control flow analysis
- Method resolution (virtual/override)
- Exception unwinding verification
- Unit visibility rules

**Tests:**
- Type compatibility
- Symbol resolution
- Overload resolution
- Class inheritance rules

---

### 8. `ir`

**Purpose:** Intermediate representation (platform-agnostic).

**Dependencies:**
- `ast`
- `types`

**Public API:**
```rust
// IR1: Linear IR
pub struct IRFunction {
    pub name: String,
    pub params: Vec<IRParam>,
    pub return_type: Option<Type>,
    pub blocks: Vec<BasicBlock>,
}

pub struct BasicBlock {
    pub label: Option<String>,
    pub instructions: Vec<IRInstruction>,
    pub successors: Vec<usize>,  // Block indices
}

pub enum IRInstruction {
    // Data movement
    Mov(IRValue, IRValue),
    Load(IRValue, IRValue),  // Load from memory
    Store(IRValue, IRValue), // Store to memory
    // Arithmetic
    Add(IRValue, IRValue, IRValue),
    Sub(IRValue, IRValue, IRValue),
    Mul(IRValue, IRValue, IRValue),
    Div(IRValue, IRValue, IRValue),
    // Control flow
    Jump(usize),  // Block index
    CJump(IRValue, usize, usize),  // Condition, then, else
    Call(IRValue, Vec<IRValue>),  // Function call
    Ret(Option<IRValue>),
    // OOP
    VTableLoad(IRValue, usize),  // Load vtable slot
    // Exceptions
    ExceptionFramePush,
    ExceptionFramePop,
    Raise(IRValue),  // Exception object
}

pub enum IRValue {
    Immediate(u16),
    Register(String),
    Memory(usize),  // Stack offset
    Temp(usize),    // Temporary variable
}

pub struct IRBuilder;
impl IRBuilder {
    pub fn build_ir(program: &TypedProgram) -> Vec<IRFunction>;
}
```

**Features:**
- Linear IR (IR1) as primary
- Optional three-address IR (IR2) for optimizations
- SSA-like where possible
- Basic block structure
- Platform-agnostic

---

### 9. `errors`

**Purpose:** Error types and error handling.

**Dependencies:**
- `tokens` (for spans)

**Public API:**
```rust
#[derive(Debug)]
pub enum CompilerError {
    Lexical(LexError),
    Parse(ParseError),
    Semantic(SemanticError),
    Codegen(CodegenError),
    Link(LinkError),
}

#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub span: Span,
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
    pub expected: Vec<TokenKind>,
}

#[derive(Debug)]
pub struct SemanticError {
    pub message: String,
    pub span: Span,
    pub hint: Option<String>,
}
```

**Features:**
- Error codes
- Source span tracking
- Error recovery hints
- Multi-span errors

---

### 10. `diagnostics`

**Purpose:** Human-friendly error messages and diagnostics.

**Dependencies:**
- `errors`
- `tokens`

**Public API:**
```rust
pub struct DiagnosticFormatter {
    source: String,
    color: bool,
    verbose: bool,  // Teaching mode
}

impl DiagnosticFormatter {
    pub fn format_error(&self, error: &CompilerError) -> String;
    pub fn format_warning(&self, warning: &Warning) -> String;
    pub fn format_hint(&self, hint: &Hint) -> String;
    pub fn format_source_snippet(&self, span: &Span) -> String;
}
```

**Features:**
- Source code highlighting
- Multi-line error messages
- Suggestions ("did you mean...?")
- IDE-friendly output (JSON, LSP)
- Teaching mode (verbose explanations)

---

### 11. `backend-zealz80`

**Purpose:** Z80 code generation.

**Dependencies:**
- `ir`
- `types`
- `runtime-spec`

**Public API:**
```rust
pub struct ZealZ80Backend {
    abi: Z80ABI,
    optimizer: PeepholeOptimizer,
}

impl ZealZ80Backend {
    pub fn new() -> Self;
    pub fn generate(&mut self, ir: &IRFunction) -> Result<Z80Code, CodegenError>;
    pub fn emit_assembly(&self, code: &Z80Code) -> String;
    pub fn emit_object(&self, code: &Z80Code) -> Result<Vec<u8>, CodegenError>;
}

pub struct Z80Code {
    pub functions: Vec<Z80Function>,
    pub data: Vec<Z80Data>,
    pub vtables: Vec<VTable>,
}
```

**Features:**
- Z80 assembly generation
- ABI compliance (IX frame, IY exception)
- Peephole optimization
- Register allocation
- Stack frame management
- VTable emission
- Exception handler tables

**Tests:**
- ABI compliance
- Code generation correctness
- Optimization passes

---

### 12. `backend-foenix816`

**Purpose:** WDC65C816 code generation.

**Dependencies:**
- `ir`
- `types`
- `runtime-spec`

**Public API:**
```rust
pub struct Foenix816Backend {
    abi: Foenix816ABI,
    optimizer: PeepholeOptimizer,
}

impl Foenix816Backend {
    pub fn new() -> Self;
    pub fn generate(&mut self, ir: &IRFunction) -> Result<Foenix816Code, CodegenError>;
}
```

**Features:**
- 65C816 assembly generation
- 24-bit addressing support
- ABI compliance
- Register allocation (A, X, Y, S, D, etc.)
- Stack frame management

---

### 13. `backend-foenix68k`

**Purpose:** MC68060 code generation.

**Dependencies:**
- `ir`
- `types`
- `runtime-spec`

**Public API:**
```rust
pub struct Foenix68kBackend {
    abi: Foenix68kABI,
    optimizer: PeepholeOptimizer,
}

impl Foenix68kBackend {
    pub fn new() -> Self;
    pub fn generate(&mut self, ir: &IRFunction) -> Result<Foenix68kCode, CodegenError>;
}
```

**Features:**
- 68060 assembly generation
- ABI compliance (reference FPC m68k backend)
- Register allocation (D0-D7, A0-A7)
- Stack frame management
- FPU support (optional)

**Reference:** Use FPC's m68k backend as semantic reference.

---

### 14. `object-zealz80`

**Purpose:** .ZOF object file writer for Zeal.

**Dependencies:**
- `backend-zealz80`

**Public API:**
```rust
pub struct ZOFWriter;

impl ZOFWriter {
    pub fn write_object(code: &Z80Code, symbols: &SymbolTable) -> Result<Vec<u8>, LinkError>;
    pub fn write_unit(code: &Z80Code, unit: &Unit) -> Result<Vec<u8>, LinkError>;
}
```

**Features:**
- .ZOF binary format
- .ZOU unit format
- Symbol tables
- Relocation entries
- VTable metadata
- Debug information

---

### 15. `runtime-spec`

**Purpose:** Runtime ABI contracts and helper function signatures.

**Dependencies:** None (specification only)

**Public API:**
```rust
// Runtime helper signatures
pub mod runtime {
    // Math helpers
    pub fn mul16(a: u16, b: u16) -> u32;
    pub fn div16(a: u16, b: u16) -> u16;
    pub fn mod16(a: u16, b: u16) -> u16;
    
    // Exception handling
    pub fn push_exception_frame(frame: *const ExceptionFrame);
    pub fn pop_exception_frame();
    pub fn unwind_stack(target_frame: *const ExceptionFrame);
    
    // Memory allocation
    pub fn alloc(size: usize) -> *mut u8;
    pub fn dealloc(ptr: *mut u8);
    
    // String operations
    pub fn str_copy(dest: *mut u8, src: *const u8, len: usize);
    pub fn str_concat(dest: *mut u8, src1: *const u8, src2: *const u8);
}
```

**Features:**
- Platform-specific runtime contracts
- ABI compatibility guarantees
- Helper function signatures

---

### 16. `driver`

**Purpose:** CLI driver and main binary.

**Dependencies:** All other crates

**Public API:**
```rust
pub struct Compiler {
    lexer: Lexer,
    parser: Parser,
    semantics: SemanticAnalyzer,
    backend: Box<dyn Backend>,
    object_writer: Box<dyn ObjectWriter>,
}

impl Compiler {
    pub fn compile(&mut self, source: &str) -> Result<(), CompilerError>;
    pub fn compile_unit(&mut self, source: &str) -> Result<(), CompilerError>;
}

// CLI interface
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    // Parse CLI arguments
    // Invoke compiler
}
```

**CLI Commands:**
```bash
superpascal build program.pas          # Full compilation
superpascal compile file.pas -o file   # Compile to object
superpascal asm file.pas               # Emit assembly
superpascal emit-ir file.pas           # Emit IR
superpascal emit-ast file.pas          # Emit AST
superpascal check file.pas             # Type check only
superpascal unit file.pas              # Compile unit
```

---

## Crate Dependencies Graph

```
driver
├── lexer
│   └── tokens
├── parser
│   ├── tokens
│   ├── ast
│   └── errors
├── semantics
│   ├── ast
│   ├── types
│   ├── symbols
│   └── errors
├── ir
│   ├── ast
│   └── types
├── backend-zealz80
│   ├── ir
│   ├── types
│   └── runtime-spec
├── object-zealz80
│   └── backend-zealz80
└── diagnostics
    ├── errors
    └── tokens
```

---

## Implementation Phases

### Phase 1: Core Frontend
1. `tokens` (pure data)
2. `lexer`
3. `ast`
4. `parser`
5. `errors`

### Phase 2: Semantic Analysis
6. `types`
7. `symbols`
8. `semantics`

### Phase 3: IR and Backend
9. `ir`
10. `runtime-spec`
11. `backend-zealz80`
12. `object-zealz80`

### Phase 4: Driver and Diagnostics
13. `diagnostics`
14. `driver`

### Phase 5: Additional Platforms
15. `backend-foenix816`
16. `backend-foenix68k`
17. `object-foenix816`
18. `object-foenix68k`

---

## Benefits of This Structure

1. **Modularity:** Each crate has a single responsibility
2. **Testability:** Each crate can be tested independently
3. **Reusability:** Frontend crates shared across all platforms
4. **Extensibility:** Easy to add new platforms or features
5. **Parallel Development:** Teams can work on different crates
6. **Clear Dependencies:** Explicit dependency graph
7. **Platform Isolation:** Platform-specific code in separate crates

---

## Migration from Current Structure

The current `crates/compiler-rs/` structure can be migrated incrementally:

1. **Extract tokens** → `tokens`
2. **Extract lexer** → `lexer`
3. **Extract parser** → `parser`
4. **Extract AST** → `ast`
5. **Continue with remaining phases**

This allows incremental migration without breaking existing code.

---

**Version:** 1.0  
**Date:** 2024  
**Status:** Proposal

