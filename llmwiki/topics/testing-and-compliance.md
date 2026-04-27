# Testing and Compliance

- Status: partially-verified
- Source docs: `tasks/rust-guidelines.txt`, `tests/` structure
- Code anchors: `crates/compiler-rs/tests/`

## Test architecture

Three categories of tests:

### 1. Unit Tests (`cargo test --lib`)
- Located in each crate's `src/` directory
- Test individual modules in isolation
- Run with: `just test-unit`

### 2. Integration Tests (`cargo nextest run --test '*'`)
- Located in `crates/compiler-rs/tests/`
- Test cross-crate interactions
- Run with: `just test-integration`

### 3. Compliance Tests (`cargo nextest run --test compliance`)
- Located in `crates/compiler-rs/tests/compliance/`
- Compare SuperPascal output against FreePascal
- Run with: `just test-compliance`
- Use `just fpc-tokenize FILE` to compare tokenizer output

## Tree-sitter Corpus Tests

Location: `crates/compiler-rs/tests/tree-sitter-corpus/`

The corpus provides test cases in a tree-sitter-compatible format:

| Corpus file | Coverage |
|-------------|----------|
| `statements.txt` | if, while, for, repeat, case, begin/end, procedures |
| `modules.txt` | unit declarations, imports |
| `declarations.txt` | type, var, const, procedure declarations |
| `expressions.txt` | binary, unary, function calls |
| `literals.txt` | integers, reals, strings, chars |
| `lambdas.txt` | Anonymous functions |
| `routines.txt` | Function/procedure definitions |
| `attributes.txt` | Compiler attributes |
| `preprocessor.txt` | Compiler directives ({$...}) |
| `generics-fpc.txt` | FreePascal generics syntax |
| `generics-delphi.txt` | Delphi generics syntax |
| `inlinevar.txt` | Inline variable declarations |
| `pascocoa.txt` | Cocoa framework integration syntax |

### Corpus tooling
- `src/bin/corpus-validator.rs` — validates corpus against parser
- `src/bin/corpus-test-generator.rs` — generates test cases
- `src/bin/sync-corpus.rs` — syncs corpus with language spec changes
- `sync_corpus.sh` — shell script for corpus operations
- `CORPUS_SETUP.md` — setup instructions

## FPC Compliance

The FreePascal fork at `../freePascal/` is patched with `-dt` (dump tokens) flag:
- `freePascal/compiler/globals.pas` — `para_dump_tokens` flag
- `freePascal/compiler/options.pas` — `-dt` option
- `freePascal/compiler/tokens.pas` — `TokenKindToString`
- `freePascal/compiler/scanner.pas` — modified `readtoken`

Compliance test cases in `tests/compliance/test_cases/`:
- `simple_program.pas` — basic program
- `numbers.pas` — numeric literals
- `strings.pas` — string handling
- `comments.pas` — comment handling
- `operators.pas` — operator parsing

Expected outputs in `tests/compliance/expected_tokens/`:
- `simple_program.tokens.txt` — FPC token output for comparison
