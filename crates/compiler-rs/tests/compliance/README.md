# Compliance Tests

## FreePascal Tokenizer Comparison

This directory contains tests to compare SuperPascal's tokenizer with FreePascal's tokenizer to ensure compatibility.

### Test Cases

Test Pascal files are in `test_cases/`:
- `simple_program.pas` - Basic program structure
- `comments.pas` - All comment styles
- `numbers.pas` - Decimal and hexadecimal numbers
- `strings.pas` - String literals with escapes
- `operators.pas` - All operators

### Building and Installing FreePascal

**First, install FPC:**

See `docs/BUILD_FPC.md` for detailed instructions. Quick start:

```bash
# Option 1: Install pre-built FPC (recommended)
brew install fpc  # macOS
# or
sudo apt-get install fpc  # Linux

# Option 2: Build from source
cd ../../../../freePascal
make build
sudo make install
```

**Verify installation:**
```bash
fpc -v
```

### Running FPC Tokenizer

**Note:** The standalone `tokenizer_test.pas` tool requires FPC's full build context and cannot be compiled independently. We use an alternative approach.

**Method 1: Use FPC Compiler Wrapper Script**

```bash
cd crates/compiler-rs/tests/compliance
./fpc_tokenize.sh test_cases/simple_program.pas
```

This script uses FPC compiler flags (`-E`, `-vt`, `-v`) to extract token information from compilation output.

**Method 2: Manual Verification**

1. Compile test files with FPC:
   ```bash
   fpc -v test_cases/simple_program.pas
   ```

2. Study the output for token information

3. Create expected token sequences in `expected_tokens/`

**Method 3: Compare with SuperPascal tokenizer (once implemented):**
   ```bash
   # Once lexer is implemented
   cd ../../../SuperPascal/crates/compiler-rs
   cargo run --bin lexer-test test_cases/simple_program.pas
   ```

**See also:** `docs/FPC_TOKENIZER_ALTERNATIVE.md` for detailed explanation of why the standalone tool approach doesn't work and alternative strategies.

### Expected Output Format

FPC tokenizer outputs:
```
TOKEN [line:column]
```

Example:
```
KW_PROGRAM [1:1]
ID:HelloWorld [1:9]
SEMICOLON [1:19]
KW_BEGIN [3:1]
...
EOF [5:1]
```

### Comparison Tool

A Rust comparison tool will be created to:
1. Parse FPC tokenizer output
2. Parse SuperPascal tokenizer output
3. Compare token sequences
4. Report differences

### Expected Token Sequences

Since FPC doesn't provide direct token dumping, we maintain expected token sequences in `expected_tokens/`:

- `expected_tokens/simple_program.tokens.txt` - Reference token sequence
- More sequences will be added as test cases are verified

These sequences are based on FreePascal's tokenization behavior and serve as the reference for comparison.

### Status

- [x] Test case files created
- [x] FPC tokenizer wrapper script created
- [x] Expected token sequences directory created
- [ ] Expected token sequences for all test cases
- [ ] SuperPascal lexer implemented
- [ ] Comparison tool implemented
- [ ] Automated tests

