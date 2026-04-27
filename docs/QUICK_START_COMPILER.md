# Quick Start: SuperPascal Compiler Implementation

## Immediate Next Steps

This guide provides **actionable steps** to start implementing the SuperPascal compiler today, using FreePascal as a reference.

---

## Step 1: Set Up Rust Workspace (30 minutes)

### 1.1 Create Workspace Structure

```bash
cd ../SuperPascal
mkdir -p crates/compiler-rs
cd crates/compiler-rs
```

### 1.2 Create Workspace Cargo.toml

Create `crates/compiler-rs/Cargo.toml`:

```toml
[workspace]
members = [
    "tokens",
    "lexer",
]

[workspace.package]
version = "0.1.0"
edition = "2024"

[workspace.dependencies]
# Add common dependencies here as we go
```

### 1.3 Create First Crate: `tokens`

```bash
cargo new --lib tokens
cd tokens
```

---

## Step 2: Implement Token Definitions (Day 1-2)

### 2.1 Study FPC Token Definitions

**Reference:** `../freePascal/compiler/tokens.pas`

**Key things to note:**
- Token enum structure
- Keyword token types
- Operator token types
- Literal token types

### 2.2 Create Token Definitions

Edit `tokens/src/lib.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Keywords (Tier 1)
    Program, Var, Const, Type, Procedure, Function,
    Begin, End, If, Then, Else, While, Do, For, To, Downto,
    Repeat, Until, Case, Of, And, Or, Not, Div, Mod,
    
    // Types (Tier 1)
    Integer, Boolean, Char, Byte, Word,
    
    // Operators
    Plus, Minus, Star, Slash,
    Equal, NotEqual, Less, LessEqual, Greater, GreaterEqual,
    
    // Literals
    IntegerLiteral(u16),
    BooleanLiteral(bool),
    CharLiteral(u8),
    StringLiteral(String),
    
    // Identifiers
    Identifier(String),
    
    // Special
    Dot, Comma, Semicolon, Colon, Assign, Arrow,
    LeftParen, RightParen, LeftBracket, RightBracket,
    
    // EOF
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

pub struct KeywordTable;

impl KeywordTable {
    pub fn lookup(ident: &str) -> Option<TokenKind> {
        // Implement keyword lookup
        match ident.to_lowercase().as_str() {
            "program" => Some(TokenKind::Program),
            "var" => Some(TokenKind::Var),
            "const" => Some(TokenKind::Const),
            // ... (add all Tier 1 keywords)
            _ => None,
        }
    }
}
```

### 2.3 Write Tests

Create `tokens/src/lib.rs` with tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_lookup() {
        assert_eq!(KeywordTable::lookup("program"), Some(TokenKind::Program));
        assert_eq!(KeywordTable::lookup("var"), Some(TokenKind::Var));
        assert_eq!(KeywordTable::lookup("unknown"), None);
    }
}
```

### 2.4 Run Tests

```bash
cd tokens
cargo test
```

**Goal:** 100% test coverage for token definitions.

---

## Step 3: Implement Lexer (Day 3-5)

### 3.1 Study FPC Scanner

**Reference:** `../freePascal/compiler/scanner.pas`

**Key functions to study:**
- `scanner()` - Main scanning loop
- `readstring()` - String literal parsing
- `readnumber()` - Number parsing
- `readcomment()` - Comment handling

### 3.2 Create Lexer Crate

```bash
cd ..
cargo new --lib lexer
cd lexer
```

Edit `Cargo.toml`:
```toml
[dependencies]
tokens = { path = "../tokens" }
```

### 3.3 Implement Basic Lexer

Edit `lexer/src/lib.rs`:

```rust
use tokens::{Token, TokenKind, KeywordTable, Span};

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();
        
        if self.is_at_end() {
            return Ok(Token {
                kind: TokenKind::Eof,
                span: self.current_span(),
            });
        }
        
        let start = self.position;
        let ch = self.advance();
        
        match ch {
            '+' => Ok(self.token(TokenKind::Plus, start)),
            '-' => Ok(self.token(TokenKind::Minus, start)),
            '*' => Ok(self.token(TokenKind::Star, start)),
            // ... (implement all operators)
            '0'..='9' => self.scan_number(start),
            'a'..='z' | 'A'..='Z' | '_' => self.scan_identifier(start),
            '{' => self.scan_comment(),
            // ... (implement all cases)
            _ => Err(LexError::UnexpectedCharacter(ch, self.current_span())),
        }
    }
    
    fn scan_identifier(&mut self, start: usize) -> Result<Token, LexError> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        
        let text: String = self.source[start..self.position].iter().collect();
        
        let kind = KeywordTable::lookup(&text)
            .unwrap_or(TokenKind::Identifier(text));
        
        Ok(self.token(kind, start))
    }
    
    fn scan_number(&mut self, start: usize) -> Result<Token, LexError> {
        // Implement number scanning (decimal, hex)
        // ...
    }
    
    // ... (implement other scanning methods)
}

#[derive(Debug)]
pub enum LexError {
    UnexpectedCharacter(char, Span),
    UnterminatedString(Span),
    InvalidNumber(Span),
}
```

### 3.4 Write Lexer Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_keywords() {
        let mut lexer = Lexer::new("program var const");
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Program);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Var);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Const);
    }
    
    #[test]
    fn test_tokenize_numbers() {
        let mut lexer = Lexer::new("123 $FF");
        // ... (test number parsing)
    }
    
    // ... (more tests)
}
```

### 3.5 Test Against FPC

**Create test program:**
```pascal
program Test;
var x: integer;
begin
  x := 10;
end.
```

**Verify:**
1. Tokenize with SuperPascal lexer
2. Compare tokens with what FPC would produce
3. Ensure compatibility

**Goal:** Lexer can tokenize all Tier 1 Pascal programs correctly.

---

## Step 4: First Milestone - Tokenize "Hello Zeal" (End of Week 1)

### Test Program
```pascal
program HelloZeal;
uses System;
begin
  WriteLn('Hello, Zeal!');
end.
```

### Success Criteria
- ✅ All tokens recognized correctly
- ✅ String literal parsed correctly
- ✅ Comments handled correctly
- ✅ Error messages are helpful

### Next Steps After Milestone
- Move to parser implementation
- Study FPC `parser.pas`
- Implement AST definitions

---

## Recommended Study Order

### Week 1: Tokens and Lexer
1. **Day 1:** Study `freePascal/compiler/tokens.pas`
2. **Day 2:** Implement `tokens` crate
3. **Day 3:** Study `freePascal/compiler/scanner.pas`
4. **Day 4-5:** Implement `lexer` crate
5. **Day 6-7:** Test and refine

### Week 2: Parser and AST
1. **Day 1:** Study `freePascal/compiler/node.pas`
2. **Day 2:** Implement `ast` crate
3. **Day 3:** Study `freePascal/compiler/parser.pas`
4. **Day 4-5:** Implement `parser` crate
5. **Day 6-7:** Test and refine

### Week 3-4: Continue with Semantics
- Follow implementation strategy document
- One crate at a time
- Test thoroughly

---

## Development Workflow

### Daily Routine

1. **Morning:**
   - Review FPC source file for current component
   - Understand algorithm/data structure
   - Plan Rust implementation

2. **Afternoon:**
   - Write tests first (TDD)
   - Implement minimal code to pass tests
   - Refactor

3. **Evening:**
   - Run full test suite
   - Compare with FPC behavior
   - Document decisions

### Testing Strategy

**For each component:**
1. **Unit tests:** Test individual functions
2. **Integration tests:** Test component interactions
3. **Compliance tests:** Test against FPC behavior
4. **Negative tests:** Test error cases

**Test coverage target:** 65% minimum, 80% preferred.

---

## Tools Setup

### Required Tools
- **Rust:** `rustup install stable`
- **Cargo:** Comes with Rust
- **Git:** For version control

### Recommended Tools
- **rust-analyzer:** IDE support
- **cargo-watch:** Auto-run tests: `cargo install cargo-watch`
- **cargo-llvm-cov:** Coverage: `cargo install cargo-llvm-cov`

### Development Commands

```bash
# Run tests
cargo test

# Run tests with coverage
cargo llvm-cov --all-features

# Watch for changes and run tests
cargo watch -x test

# Format code
cargo fmt

# Lint code
cargo clippy

# Generate documentation
cargo doc --open
```

---

## Getting Help

### When Stuck

1. **Study FPC source:** Check how FPC does it
2. **Check language spec:** `languageSpecification/`
3. **Check platform spec:** `platforms/ZealZ80/`
4. **Review implementation strategy:** `docs/COMPILER_IMPLEMENTATION_STRATEGY.md`
5. **Check FPC reference guide:** `docs/FPC_SOURCE_REFERENCE_GUIDE.md`

### Key Documents

- **Implementation Strategy:** `docs/COMPILER_IMPLEMENTATION_STRATEGY.md`
- **FPC Reference Guide:** `docs/FPC_SOURCE_REFERENCE_GUIDE.md`
- **Feature Matrix:** `docs/SUPERPASCAL_VS_FPC_FEATURE_MATRIX.md`
- **Crate Layout:** `docs/RUST_CRATE_LAYOUT_PROPOSAL.md`
- **Language Spec:** `languageSpecification/00_Overview.md`
- **ZealZ80 ABI:** `platforms/ZealZ80/ABI.md`

---

## First Command to Run

```bash
cd ../SuperPascal
mkdir -p crates/compiler-rs
cd crates/compiler-rs
cargo new --lib tokens
cd tokens
# Start implementing!
```

---

**Ready to start?** Begin with `tokens` crate and study `freePascal/compiler/tokens.pas`!

---

**Version:** 1.0  
**Date:** 2024  
**Status:** Quick Start Guide

