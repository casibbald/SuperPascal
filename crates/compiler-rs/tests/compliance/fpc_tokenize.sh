#!/bin/bash
# FreePascal Tokenizer Wrapper
# Uses FPC compiler to verify compilation and capture any token-related information

set -e

if [ $# -lt 1 ]; then
    echo "Usage: $0 <input.pas>"
    exit 1
fi

INPUT_FILE="$1"
OUTPUT_FILE="${INPUT_FILE%.pas}.fpc_tokens.txt"
BASENAME=$(basename "$INPUT_FILE" .pas)

echo "Analyzing $INPUT_FILE with FreePascal compiler..."

# FPC doesn't provide direct token dumping, but we can:
# 1. Verify the file compiles correctly (validates tokenization)
# 2. Capture error messages (which show token positions)
# 3. Use preprocessor output to see processed source

# Method 1: Try to compile and capture output
echo "=== Compilation Output ===" > "$OUTPUT_FILE"
if fpc -v "$INPUT_FILE" 2>&1 >> "$OUTPUT_FILE"; then
    echo "✓ Compilation successful" >> "$OUTPUT_FILE"
else
    echo "✗ Compilation failed (check errors above)" >> "$OUTPUT_FILE"
fi

# Method 2: Use preprocessor to see processed source
echo "" >> "$OUTPUT_FILE"
echo "=== Preprocessor Output ===" >> "$OUTPUT_FILE"
fpc -E "$INPUT_FILE" 2>&1 | head -50 >> "$OUTPUT_FILE" || true

# Method 3: Try to get assembly output (shows how tokens were processed)
echo "" >> "$OUTPUT_FILE"
echo "=== Assembly Output (first 50 lines) ===" >> "$OUTPUT_FILE"
if [ -f "${BASENAME}.s" ]; then
    head -50 "${BASENAME}.s" >> "$OUTPUT_FILE" 2>/dev/null || true
    rm -f "${BASENAME}.s" "${BASENAME}.o" "${BASENAME}" 2>/dev/null || true
fi

echo "Output written to: $OUTPUT_FILE"
echo ""
echo "Note: FPC doesn't provide direct token dumping."
echo "Expected token sequences are in: expected_tokens/${BASENAME}.tokens.txt"
echo ""
echo "To compare tokens:"
echo "  1. Check expected_tokens/${BASENAME}.tokens.txt for reference"
echo "  2. Run SuperPascal tokenizer on the same file"
echo "  3. Compare outputs manually or with comparison tool"

