# FreePascal Tokenizer Comparison

## Goal

Compare FreePascal's tokenizer output with SuperPascal's tokenizer to ensure compatibility and correctness.

## Approach

### Option 1: Build FPC Scanner Standalone Tool

Create a minimal FPC program that:
1. Uses FPC's scanner unit
2. Reads a Pascal source file
3. Outputs tokens in a parseable format (JSON or text)
4. Can be compiled and run independently

### Option 2: Use FPC Compiler Debug Output

If FPC supports token dumping:
- Use `fpc -vt` or similar flags
- Parse the debug output
- Compare with our tokenizer

### Option 3: Manual Test Suite

1. Create test Pascal files
2. Manually tokenize with FPC (or use FPC compiler)
3. Create expected token sequences
4. Compare our tokenizer output

## Test Files

Create test cases in `tests/compliance/test_cases/`:

1. **Simple programs:**
   - `simple_program.pas` - Basic program structure
   - `variables.pas` - Variable declarations
   - `procedures.pas` - Procedure declarations

2. **Edge cases:**
   - `comments.pas` - All comment styles
   - `strings.pas` - String literals with escapes
   - `numbers.pas` - Decimal and hex numbers
   - `operators.pas` - All operators

3. **Tier 1 Pascal:**
   - `control_flow.pas` - if, while, for, repeat
   - `types.pas` - Type declarations
   - `expressions.pas` - Complex expressions

## Comparison Format

Compare token sequences:
- Token kind
- Token value (for literals)
- Source position (line, column)

## Implementation Plan

1. Create FPC tokenizer wrapper (if possible)
2. Create test case generator
3. Create comparison tool in Rust
4. Add to CI/CD pipeline

