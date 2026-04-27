# FreePascal Tokenizer Comparison - Alternative Approach

## Problem

The original `tokenizer_test.pas` approach failed because:
- FPC's internal compiler units (`scanner`, `globtype`, etc.) require the full compiler build context
- Types like `PInt`, `PUInt`, `AWord`, `AInt` are defined in platform-specific include files
- These units are designed for use within the FPC compiler, not as standalone tools

## Alternative Approaches

### Option 1: Use FPC Compiler Directly (Current)

**Script:** `crates/compiler-rs/tests/compliance/fpc_tokenize.sh`

Uses FPC compiler flags to extract token information:
- `-E` - Preprocessor output (shows processed source)
- `-vt` - Verbose tokenization (if available)
- `-v` - Verbose compilation (shows compilation process)

**Limitations:**
- FPC doesn't provide direct token dumping
- Output is compilation-focused, not token-focused
- May not show all tokens in a parseable format

**Usage:**
```bash
cd crates/compiler-rs/tests/compliance
./fpc_tokenize.sh test_cases/simple_program.pas
```

### Option 2: Build FPC from Source and Use Internal Tools

If we build FPC from source, we can:
1. Use FPC's internal scanner within the build system
2. Create a test harness that's part of the FPC build
3. Access FPC's tokenization directly

**Steps:**
```bash
cd ../freePascal
make build
# Then create tokenizer test as part of FPC build system
```

### Option 3: Manual Token Verification

For now, we can:
1. Manually verify tokenization on test cases
2. Create expected token sequences based on FPC behavior
3. Compare SuperPascal tokenizer against these expected sequences

**Test Cases:**
- `test_cases/simple_program.pas`
- `test_cases/comments.pas`
- `test_cases/numbers.pas`
- `test_cases/strings.pas`
- `test_cases/operators.pas`

### Option 4: Use FPC's Test Suite

FPC has an extensive test suite in `freePascal/tests/`. We can:
1. Study how FPC tests tokenization
2. Use FPC's test framework
3. Adapt tests for SuperPascal

## Recommended Approach

**For now:** Use Option 1 (FPC compiler directly) combined with Option 3 (manual verification).

**Future:** Once we have a working SuperPascal lexer, we can:
1. Create a Rust comparison tool
2. Manually verify token sequences
3. Build automated tests based on expected token sequences
4. Optionally integrate with FPC's build system later

## Implementation Status

- [x] Created `fpc_tokenize.sh` wrapper script
- [ ] Test script on sample files
- [ ] Create expected token sequences
- [ ] Implement SuperPascal lexer
- [ ] Create Rust comparison tool
- [ ] Automated comparison tests

## Next Steps

1. **Test the wrapper script:**
   ```bash
   cd crates/compiler-rs/tests/compliance
   ./fpc_tokenize.sh test_cases/simple_program.pas
   ```

2. **Manually verify tokens:**
   - Run FPC on test cases
   - Document expected token sequences
   - Create `expected_tokens/` directory with reference outputs

3. **Implement SuperPascal lexer:**
   - Create `lexer` crate
   - Implement tokenization
   - Output tokens in comparable format

4. **Create comparison tool:**
   - Parse FPC output (or expected sequences)
   - Parse SuperPascal output
   - Compare and report differences

