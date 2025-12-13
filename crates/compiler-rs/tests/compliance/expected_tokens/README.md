# Expected Token Sequences

This directory contains expected token sequences for test cases, based on FreePascal's tokenization behavior.

## Format

Each `.tokens.txt` file contains the expected token sequence in the format:
```
TOKEN_KIND [value] (line:column)
```

Where:
- `TOKEN_KIND` - The token type (e.g., `PROGRAM`, `IDENTIFIER`, `STRING`)
- `[value]` - Optional token value (for identifiers, strings, numbers)
- `(line:column)` - Source position

## Token Kinds

Based on SuperPascal's `TokenKind` enum:
- Keywords: `PROGRAM`, `BEGIN`, `END`, `VAR`, `CONST`, `TYPE`, etc.
- Identifiers: `IDENTIFIER [name]`
- Literals: `INTEGER [value]`, `STRING [value]`, `CHAR [value]`
- Operators: `PLUS`, `MINUS`, `STAR`, `SLASH`, `EQ`, `NE`, etc.
- Delimiters: `LPAREN`, `RPAREN`, `LBRACKET`, `RBRACKET`, `SEMICOLON`, `COMMA`, `COLON`, `POINT`
- Special: `ASSIGNMENT` (`:=`), `POINTPOINT` (`..`), `EOF`

## Creating Expected Sequences

1. Study FreePascal's behavior on the test case
2. Manually tokenize the source code
3. Document the expected sequence
4. Use this as a reference for SuperPascal tokenizer comparison

## Usage

Once SuperPascal lexer is implemented:
1. Run SuperPascal tokenizer on test case
2. Compare output with expected sequence
3. Report differences

## Files

- `simple_program.tokens.txt` - Expected tokens for `test_cases/simple_program.pas`
- (More files to be added as test cases are verified)

