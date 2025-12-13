//! SuperPascal Token Definitions
//!
//! This crate defines all token types for the SuperPascal compiler.
//! Tokens are the atomic units of the language that the lexer produces.

/// Source code location information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Starting byte offset in source file
    pub start: usize,
    /// Ending byte offset (exclusive)
    pub end: usize,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
}

impl Span {
    /// Create a new span
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }

    /// Create a zero-length span at a position
    pub fn at(pos: usize, line: usize, column: usize) -> Self {
        Self {
            start: pos,
            end: pos,
            line,
            column,
        }
    }

    /// Merge two spans (from start of first to end of second)
    pub fn merge(self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line,
            column: self.column,
        }
    }
}

/// Token kinds for SuperPascal
///
/// Based on the lexical structure specification:
/// - Keywords (Tier 1, Tier 2, Tier 3)
/// - Identifiers
/// - Literals (integer, character, string, boolean)
/// - Operators
/// - Delimiters
/// - Directives
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // ===== Keywords (Tier 1: Core) =====
    KwAnd,
    KwArray,
    KwBegin,
    KwBoolean,
    KwByte,
    KwCase,
    KwChar,
    KwConst,
    KwDiv,
    KwDo,
    KwDownto,
    KwElse,
    KwEnd,
    KwFalse,
    KwFor,
    KwFunction,
    KwGoto,
    KwIf,
    KwInteger,
    KwMod,
    KwNot,
    KwOf,
    KwOr,
    KwProcedure,
    KwProgram,
    KwRecord,
    KwRepeat,
    KwSet,
    KwStruct,  // SuperPascal extension
    KwThen,
    KwTo,
    KwTrue,
    KwType,
    KwUntil,
    KwVar,
    KwWhile,
    KwWord,

    // ===== Keywords (Tier 2: Units) =====
    KwImplementation,
    KwInterface,
    KwUnit,
    KwUses,
    KwNamespace,  // Future
    KwUsing,      // Future

    // ===== Keywords (Tier 3: Object Pascal) =====
    KwClass,
    KwConstructor,
    KwDestructor,
    KwOverride,
    KwPrivate,
    KwProtected,
    KwPublic,
    KwVirtual,

    // ===== Keywords (Exceptions) =====
    KwExcept,
    KwFinally,
    KwRaise,
    KwTry,

    // ===== Keywords (Special) =====
    KwNil,
    KwSelf,
    KwInherited,

    // ===== Identifiers =====
    Identifier(String),

    // ===== Literals =====
    /// Integer literal (decimal or hexadecimal)
    IntegerLiteral {
        value: u16,
        is_hex: bool,
    },
    /// Character literal
    CharLiteral(u8),
    /// String literal
    StringLiteral(String),
    /// Boolean literal
    BooleanLiteral(bool),

    // ===== Operators =====
    // Arithmetic
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    // Comparison
    Equal,     // =
    NotEqual,  // <>
    Less,      // <
    LessEqual, // <=
    Greater,   // >
    GreaterEqual, // >=
    // Logical
    // (and, or, not are keywords)
    // Assignment
    Assign,    // :=
    // Other
    Dot,       // .
    DotDot,    // ..
    Caret,     // ^

    // ===== Delimiters =====
    Semicolon,  // ;
    Comma,      // ,
    Colon,      // :
    LeftParen,  // (
    RightParen, // )
    LeftBracket, // [
    RightBracket, // ]
    LeftBrace,  // {
    RightBrace, // }
    At,         // @

    // ===== Directives =====
    /// Compiler directive: {$...}
    Directive(String),

    // ===== Special =====
    /// End of file
    Eof,
    /// Invalid token (for error recovery)
    Invalid(String),
}

/// A token with source location information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    /// Create a new token
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Check if token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::KwAnd
                | TokenKind::KwArray
                | TokenKind::KwBegin
                | TokenKind::KwBoolean
                | TokenKind::KwByte
                | TokenKind::KwCase
                | TokenKind::KwChar
                | TokenKind::KwConst
                | TokenKind::KwDiv
                | TokenKind::KwDo
                | TokenKind::KwDownto
                | TokenKind::KwElse
                | TokenKind::KwEnd
                | TokenKind::KwFalse
                | TokenKind::KwFor
                | TokenKind::KwFunction
                | TokenKind::KwGoto
                | TokenKind::KwIf
                | TokenKind::KwInteger
                | TokenKind::KwMod
                | TokenKind::KwNot
                | TokenKind::KwOf
                | TokenKind::KwOr
                | TokenKind::KwProcedure
                | TokenKind::KwProgram
                | TokenKind::KwRecord
                | TokenKind::KwRepeat
                | TokenKind::KwSet
                | TokenKind::KwStruct
                | TokenKind::KwThen
                | TokenKind::KwTo
                | TokenKind::KwTrue
                | TokenKind::KwType
                | TokenKind::KwUntil
                | TokenKind::KwVar
                | TokenKind::KwWhile
                | TokenKind::KwWord
                | TokenKind::KwImplementation
                | TokenKind::KwInterface
                | TokenKind::KwUnit
                | TokenKind::KwUses
                | TokenKind::KwNamespace
                | TokenKind::KwUsing
                | TokenKind::KwClass
                | TokenKind::KwConstructor
                | TokenKind::KwDestructor
                | TokenKind::KwOverride
                | TokenKind::KwPrivate
                | TokenKind::KwProtected
                | TokenKind::KwPublic
                | TokenKind::KwVirtual
                | TokenKind::KwExcept
                | TokenKind::KwFinally
                | TokenKind::KwRaise
                | TokenKind::KwTry
                | TokenKind::KwNil
                | TokenKind::KwSelf
                | TokenKind::KwInherited
        )
    }

    /// Check if token is an operator
    pub fn is_operator(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Equal
                | TokenKind::NotEqual
                | TokenKind::Less
                | TokenKind::LessEqual
                | TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Assign
                | TokenKind::Dot
                | TokenKind::DotDot
                | TokenKind::Caret
        )
    }

    /// Check if token is a literal
    pub fn is_literal(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::IntegerLiteral { .. }
                | TokenKind::CharLiteral(_)
                | TokenKind::StringLiteral(_)
                | TokenKind::BooleanLiteral(_)
        )
    }
}

/// Operator precedence levels (higher = tighter binding)
///
/// Based on Pascal operator precedence rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    /// Lowest precedence (assignment, etc.)
    Lowest = 0,
    /// Logical OR
    Or = 1,
    /// Logical AND
    And = 2,
    /// Comparison operators (=, <>, <, <=, >, >=)
    Comparison = 3,
    /// Addition/subtraction (+, -)
    Add = 4,
    /// Multiplication/division (*, /, div, mod)
    Mul = 5,
    /// Unary operators (+, -, not, ^)
    Unary = 6,
    /// Highest precedence (parentheses, function calls)
    Highest = 7,
}

impl TokenKind {
    /// Get operator precedence (if this is an operator)
    ///
    /// Note: Plus and Minus can be unary or binary. This returns their binary precedence.
    /// The parser will determine if they're unary based on context.
    pub fn precedence(&self) -> Option<Precedence> {
        match self {
            // Unary-only operators
            TokenKind::KwNot | TokenKind::Caret => Some(Precedence::Unary),
            // Multiplicative
            TokenKind::Star | TokenKind::Slash | TokenKind::KwDiv | TokenKind::KwMod => {
                Some(Precedence::Mul)
            }
            // Additive (binary - parser handles unary case)
            TokenKind::Plus | TokenKind::Minus => Some(Precedence::Add),
            // Comparison
            TokenKind::Equal
            | TokenKind::NotEqual
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual => Some(Precedence::Comparison),
            // Logical AND
            TokenKind::KwAnd => Some(Precedence::And),
            // Logical OR
            TokenKind::KwOr => Some(Precedence::Or),
            // Assignment
            TokenKind::Assign => Some(Precedence::Lowest),
            _ => None,
        }
    }

    /// Check if this is a binary operator
    pub fn is_binary_operator(&self) -> bool {
        matches!(
            self,
            TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::KwDiv
                | TokenKind::KwMod
                | TokenKind::Equal
                | TokenKind::NotEqual
                | TokenKind::Less
                | TokenKind::LessEqual
                | TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::KwAnd
                | TokenKind::KwOr
        )
    }

    /// Check if this is a unary operator
    pub fn is_unary_operator(&self) -> bool {
        matches!(
            self,
            TokenKind::Plus | TokenKind::Minus | TokenKind::KwNot | TokenKind::Caret
        )
    }
}

/// Keyword lookup table
///
/// Maps keyword strings (case-insensitive) to TokenKind
pub fn lookup_keyword(s: &str) -> Option<TokenKind> {
    // Convert to lowercase for case-insensitive lookup
    let lower = s.to_lowercase();
    match lower.as_str() {
        // Tier 1: Core keywords
        "and" => Some(TokenKind::KwAnd),
        "array" => Some(TokenKind::KwArray),
        "begin" => Some(TokenKind::KwBegin),
        "boolean" => Some(TokenKind::KwBoolean),
        "byte" => Some(TokenKind::KwByte),
        "case" => Some(TokenKind::KwCase),
        "char" => Some(TokenKind::KwChar),
        "const" => Some(TokenKind::KwConst),
        "div" => Some(TokenKind::KwDiv),
        "do" => Some(TokenKind::KwDo),
        "downto" => Some(TokenKind::KwDownto),
        "else" => Some(TokenKind::KwElse),
        "end" => Some(TokenKind::KwEnd),
        "false" => Some(TokenKind::KwFalse),
        "for" => Some(TokenKind::KwFor),
        "function" => Some(TokenKind::KwFunction),
        "goto" => Some(TokenKind::KwGoto),
        "if" => Some(TokenKind::KwIf),
        "integer" => Some(TokenKind::KwInteger),
        "mod" => Some(TokenKind::KwMod),
        "not" => Some(TokenKind::KwNot),
        "of" => Some(TokenKind::KwOf),
        "or" => Some(TokenKind::KwOr),
        "procedure" => Some(TokenKind::KwProcedure),
        "program" => Some(TokenKind::KwProgram),
        "record" => Some(TokenKind::KwRecord),
        "repeat" => Some(TokenKind::KwRepeat),
        "set" => Some(TokenKind::KwSet),
        "struct" => Some(TokenKind::KwStruct),
        "then" => Some(TokenKind::KwThen),
        "to" => Some(TokenKind::KwTo),
        "true" => Some(TokenKind::KwTrue),
        "type" => Some(TokenKind::KwType),
        "until" => Some(TokenKind::KwUntil),
        "var" => Some(TokenKind::KwVar),
        "while" => Some(TokenKind::KwWhile),
        "word" => Some(TokenKind::KwWord),
        // Tier 2: Unit keywords
        "implementation" => Some(TokenKind::KwImplementation),
        "interface" => Some(TokenKind::KwInterface),
        "unit" => Some(TokenKind::KwUnit),
        "uses" => Some(TokenKind::KwUses),
        "namespace" => Some(TokenKind::KwNamespace),
        "using" => Some(TokenKind::KwUsing),
        // Tier 3: Object Pascal
        "class" => Some(TokenKind::KwClass),
        "constructor" => Some(TokenKind::KwConstructor),
        "destructor" => Some(TokenKind::KwDestructor),
        "override" => Some(TokenKind::KwOverride),
        "private" => Some(TokenKind::KwPrivate),
        "protected" => Some(TokenKind::KwProtected),
        "public" => Some(TokenKind::KwPublic),
        "virtual" => Some(TokenKind::KwVirtual),
        // Exceptions
        "except" => Some(TokenKind::KwExcept),
        "finally" => Some(TokenKind::KwFinally),
        "raise" => Some(TokenKind::KwRaise),
        "try" => Some(TokenKind::KwTry),
        // Special
        "nil" => Some(TokenKind::KwNil),
        "self" => Some(TokenKind::KwSelf),
        "inherited" => Some(TokenKind::KwInherited),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_lookup() {
        // Case-insensitive lookup
        assert_eq!(lookup_keyword("if"), Some(TokenKind::KwIf));
        assert_eq!(lookup_keyword("IF"), Some(TokenKind::KwIf));
        assert_eq!(lookup_keyword("If"), Some(TokenKind::KwIf));
        
        // Non-keywords return None
        assert_eq!(lookup_keyword("myvar"), None);
        assert_eq!(lookup_keyword("x"), None);
    }

    #[test]
    fn test_token_kind_precedence() {
        // Unary operators (not keyword has unary precedence)
        assert_eq!(
            TokenKind::KwNot.precedence(),
            Some(Precedence::Unary)
        );
        
        // Multiplicative operators
        assert_eq!(
            TokenKind::Star.precedence(),
            Some(Precedence::Mul)
        );
        
        // Additive operators (Plus/Minus can be binary, so they return Add)
        // Note: Parser will determine if they're unary or binary based on context
        assert_eq!(
            TokenKind::Plus.precedence(),
            Some(Precedence::Add)  // Binary plus has Add precedence
        );
        
        // Comparison operators
        assert_eq!(
            TokenKind::Equal.precedence(),
            Some(Precedence::Comparison)
        );
        
        // Logical operators
        assert_eq!(
            TokenKind::KwAnd.precedence(),
            Some(Precedence::And)
        );
        
        // Non-operators return None
        assert_eq!(TokenKind::KwIf.precedence(), None);
    }

    #[test]
    fn test_span_merge() {
        let span1 = Span::new(0, 5, 1, 1);
        let span2 = Span::new(10, 15, 1, 11);
        let merged = span1.merge(span2);
        
        assert_eq!(merged.start, 0);
        assert_eq!(merged.end, 15);
    }

    #[test]
    fn test_token_checks() {
        let token = Token::new(
            TokenKind::KwIf,
            Span::new(0, 2, 1, 1),
        );
        
        assert!(token.is_keyword());
        assert!(!token.is_operator());
        assert!(!token.is_literal());
        
        let op_token = Token::new(
            TokenKind::Plus,
            Span::new(0, 1, 1, 1),
        );
        
        assert!(!op_token.is_keyword());
        assert!(op_token.is_operator());
        assert!(!op_token.is_literal());
    }
}
