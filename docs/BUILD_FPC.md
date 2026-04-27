# Building and Installing FreePascal Compiler

## Overview

This document describes how to build and install FreePascal (FPC) from source for use in SuperPascal development, particularly for tokenizer comparison testing.

## Prerequisites

### Option 1: Bootstrap with Existing FPC

If you have FPC already installed:
```bash
# Check if FPC is installed
fpc -v
```

### Option 2: Install Pre-built FPC First

**macOS (using Homebrew):**
```bash
brew install fpc
```

**Linux (using package manager):**
```bash
# Debian/Ubuntu
sudo apt-get install fpc

# Fedora
sudo dnf install fpc

# Arch
sudo pacman -S fpc
```

**Download from Source:**
- Visit https://www.freepascal.org/download.html
- Download pre-built binaries for your platform
- Extract and add to PATH

## Building FPC from Source

### Step 1: Check Prerequisites

```bash
# Required tools
which make
which gcc  # or clang on macOS
```

### Step 2: Build FPC Compiler

```bash
cd /Users/casibbald/Workspace/casibbald/SuperPascal/freePascal

# Build for current platform (native)
make build

# Or build specific target (e.g., for cross-compilation)
# make build OS_TARGET=linux CPU_TARGET=x86_64
```

### Step 3: Install FPC

```bash
# Install to default location (/usr/local)
sudo make install

# Or install to custom location
sudo make install INSTALL_PREFIX=/opt/fpc
```

### Step 4: Update PATH

Add FPC to your PATH:
```bash
# For bash/zsh
export PATH=/usr/local/bin:$PATH

# Or add to ~/.zshrc or ~/.bashrc
echo 'export PATH=/usr/local/bin:$PATH' >> ~/.zshrc
```

### Step 5: Verify Installation

```bash
fpc -v
ppcx64 -v  # Platform-specific compiler
```

## Building FPC Tokenizer Test Tool

Once FPC is installed:

```bash
cd /Users/casibbald/Workspace/casibbald/SuperPascal/freePascal/compiler

# Compile the tokenizer test tool
fpc tokenizer_test.pas

# This will create tokenizer_test (or tokenizer_test.exe on Windows)
```

**Note:** The tokenizer_test.pas tool may need adjustments to work with the FPC source tree structure. It requires:
- `scanner` unit
- `tokens` unit
- `globals` unit
- `verbose` unit
- `finput` unit

## Alternative: Use FPC Compiler Directly

Instead of building a standalone tokenizer tool, we can use FPC's built-in tokenization:

```bash
# FPC with verbose token output (if available)
fpc -vt input.pas

# Or use FPC's preprocessor to see tokens
fpc -E input.pas  # Preprocess only
```

## Troubleshooting

### Build Fails

1. **Missing dependencies:**
   ```bash
   # macOS
   xcode-select --install
   
   # Linux
   sudo apt-get install build-essential
   ```

2. **Bootstrap compiler needed:**
   - Install pre-built FPC first (see Option 2 above)
   - Then rebuild from source

3. **Path issues:**
   - Ensure FPC is in PATH
   - Check `which fpc` and `which ppcx64`

### Tokenizer Test Tool Won't Compile

The tokenizer_test.pas may need to be integrated into the FPC build system or use FPC's internal APIs differently. Alternative approaches:

1. **Use FPC compiler with debug output**
2. **Create a simpler test harness**
3. **Manually verify tokenization on test cases**

## Integration with SuperPascal

Once FPC is built and installed:

1. **Build tokenizer test tool:**
   ```bash
   cd freePascal/compiler
   fpc tokenizer_test.pas
   ```

2. **Run comparison tests:**
   ```bash
   cd SuperPascal/crates/compiler-rs
   just test-compliance
   ```

3. **Compare outputs:**
   - FPC tokenizer output → `tests/compliance/fpc_output/`
   - SuperPascal tokenizer output → `tests/compliance/sp_output/`
   - Comparison tool → `tests/compliance/compare_tokens.rs`

## References

- [FPC Build Documentation](https://wiki.freepascal.org/Building_FPC)
- [FPC Installation Guide](https://wiki.freepascal.org/Installing_Free_Pascal)
- [FPC Source Repository](https://gitlab.com/freepascal.org/fpc/source)

