# Contributing to SuperPascal

Thank you for your interest in contributing to SuperPascal! This document outlines the development setup, including the necessary workarounds and why they exist.

## Table of Contents

- [Development Setup](#development-setup)
- [The FreePascal "Jerryrig"](#the-freepascal-jerryrig)
- [Why We Can't Use Upstream FPC](#why-we-cant-use-upstream-fpc)
- [Where the Modified Compiler Lives](#where-the-modified-compiler-lives)
- [Building the Patched FPC](#building-the-patched-fpc)
- [Using the Patched FPC](#using-the-patched-fpc)
- [Development Workflow](#development-workflow)
- [Code Style](#code-style)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)

## Development Setup

### Prerequisites

- **Rust** (stable toolchain) - for the SuperPascal compiler
- **FreePascal Compiler** - see [The FreePascal "Jerryrig"](#the-freepascal-jerryrig) below
- **Git** - for version control
- **Just** - command runner (optional but recommended)

### Quick Start

1. Clone the repository:
   ```bash
   git clone https://github.com/casibbald/SuperPascal.git
   cd SuperPascal/SuperPascal
   ```

2. Set up the patched FreePascal compiler (see below)

3. Build the SuperPascal compiler:
   ```bash
   just build
   ```

4. Run tests:
   ```bash
   just test
   ```

## The FreePascal "Jerryrig"

### What Is It?

SuperPascal maintains a **forked and patched version** of the FreePascal compiler (FPC). This is not the upstream FPC from `freepascal.org` - it's our own modified version with custom patches.

### Why We Call It a "Jerryrig"

This setup is a pragmatic workaround, not an ideal solution. We call it a "jerryrig" because:

1. **We can't use upstream FPC directly** - it doesn't have the features we need
2. **We need token-level output** - for compliance testing and comparison
3. **We maintain our own fork** - with patches that upstream may not accept
4. **It's a temporary solution** - until SuperPascal's own compiler is complete

This is a **development tool**, not part of the final SuperPascal product. Once SuperPascal's compiler is fully functional, we can phase out this dependency.

## Why We Can't Use Upstream FPC

### The Problem

SuperPascal needs to **compare its tokenizer output** with FreePascal's tokenizer to ensure compatibility. This is critical for:

- **Compliance testing** - ensuring SuperPascal tokenizes Pascal code the same way as FPC
- **Reference implementation** - using FPC as a semantic oracle
- **Quality assurance** - catching tokenization bugs early

### What We Need

We need FPC to **dump tokens** in a parseable format, specifically:
- Token kind (keyword, identifier, operator, etc.)
- Token value/pattern (for identifiers, strings, numbers)
- Source position (line and column)

### Why Upstream FPC Doesn't Work

The upstream FreePascal compiler **does not provide** a way to dump tokens:

1. **No token dumping flag** - FPC has `-vt` (verbose tokens) but it's not parseable
2. **Internal-only** - Token information is only used internally during compilation
3. **No API** - There's no way to extract token sequences programmatically
4. **Not a design goal** - FPC is designed to compile, not to be a reference implementation

### Our Solution

We **patch FPC** to add a `-dt` (dump tokens) flag that outputs tokens in a structured, parseable format. This allows SuperPascal to:

- Compare token sequences with FPC
- Validate tokenization correctness
- Build compliance test suites
- Use FPC as a reference implementation

## Where the Modified Compiler Lives

### Repository Structure

```
SuperPascal/
├── SuperPascal/              # Main SuperPascal project
│   ├── crates/compiler-rs/  # SuperPascal compiler (Rust)
│   └── ...
└── freePascal/               # Patched FreePascal fork
    ├── compiler/             # FPC compiler source
    │   ├── globals.pas       # Added para_dump_tokens flag
    │   ├── options.pas       # Added -dt option handler
    │   ├── tokens.pas        # Added TokenKindToString function
    │   └── scanner.pas       # Modified to output tokens
    └── .github/workflows/    # CI/CD for building FPC
```

### Key Files Modified

The patched FPC is located in `../freePascal/` (relative to SuperPascal root):

1. **`freePascal/compiler/globals.pas`**
   - Added `para_dump_tokens: boolean;` global flag
   - Initialized to `false` in `InitGlobals`

2. **`freePascal/compiler/options.pas`**
   - Added `-dt` command-line option handler
   - Sets `para_dump_tokens := true` when `-dt` is specified

3. **`freePascal/compiler/tokens.pas`**
   - Added `TokenKindToString(t: ttoken): string;` function
   - Converts token enum values to string representations

4. **`freePascal/compiler/scanner.pas`**
   - Modified `readtoken` procedure to output tokens when flag is set
   - Outputs: token kind, pattern (for identifiers/literals), line, column

### GitHub Repository

The patched FPC is maintained in a separate repository:
- **Repository**: `casibbald/freePascal`
- **Purpose**: Build and distribute patched FPC binaries
- **Releases**: Pre-built binaries for Linux x86_64 (built on GitHub Actions)
- **macOS Builds**: Built locally for testing (see "Local Build (macOS)" above)

## Building the Patched FPC

### Local Build (macOS)

**Note:** macOS builds are built locally for testing. Only Linux x86_64 is built on GitHub Actions.

To build FPC locally on macOS:

```bash
cd ../freePascal

# Install bootstrap FPC (if not already installed)
brew install fpc

# Find bootstrap FPC
FPC_BIN=$(which ppca64 || which ppcx64 || which fpc)

# Build RTL first
make rtl_clean OS_TARGET=darwin CPU_TARGET=aarch64 FPC="$FPC_BIN" || true  # ARM64
# or
make rtl_clean OS_TARGET=darwin CPU_TARGET=x86_64 FPC="$FPC_BIN" || true   # x86_64

make rtl OS_TARGET=darwin CPU_TARGET=aarch64 FPC="$FPC_BIN" OPT="-n" -j$(sysctl -n hw.ncpu)  # ARM64
# or
make rtl OS_TARGET=darwin CPU_TARGET=x86_64 FPC="$FPC_BIN" OPT="-n" -j$(sysctl -n hw.ncpu)   # x86_64

# Build compiler
cd compiler
make clean || true
make compiler FPC="$FPC_BIN" OS_TARGET=darwin CPU_TARGET=aarch64 OPT="-n" -j$(sysctl -n hw.ncpu)  # ARM64
# or
make compiler FPC="$FPC_BIN" OS_TARGET=darwin CPU_TARGET=x86_64 OPT="-n" -j$(sysctl -n hw.ncpu)  # x86_64

# Build RTL and packages
cd ..
make all OS_TARGET=darwin CPU_TARGET=aarch64 FPC="$FPC_BIN" OPT="-n" -j$(sysctl -n hw.ncpu)  # ARM64
# or
make all OS_TARGET=darwin CPU_TARGET=x86_64 FPC="$FPC_BIN" OPT="-n" -j$(sysctl -n hw.ncpu)  # x86_64

# The binary will be in:
# - macOS ARM64: compiler/ppca64
# - macOS x86_64: compiler/ppcx64
```

### Local Build (Linux)

If you need to build FPC locally on Linux:

```bash
cd ../freePascal/compiler

# Build for your platform
make cycle OPT="-n" -j$(nproc)

# The binary will be in:
# - Linux: compiler/ppcx64
```

### Using Pre-built Binaries

For most developers, you can use the pre-built binaries from GitHub Releases:

```bash
# Download from: https://github.com/casibbald/freePascal/releases
# Extract and add to PATH
export PATH=$PWD/fpc-<platform>/bin:$PATH
```

### CI/CD Builds

The patched FPC is automatically built on GitHub Actions:
- **Workflow**: `freePascal/.github/workflows/build-and-release.yml`
- **Triggers**: Pushes to `main`/`master`, tags starting with `v`
- **Platforms**: Only Linux x86_64 is built on GitHub Actions
- **macOS**: macOS builds (ARM64 and x86_64) are built locally for testing

**Note:** Only Linux x86_64 binaries are published to GitHub Releases. macOS versions should be built locally as needed.

See `docs/FPC_GITHUB_ACTIONS.md` for details.

## Using the Patched FPC

### Token Dumping

The patched FPC adds a `-dt` flag to dump tokens:

```bash
# Dump tokens from a Pascal file
fpc -dt yourfile.pas

# Output format:
# TOKEN_KIND [pattern] (line:column)
# PROGRAM [] (1:1)
# identifier [HELLOWORLD] (1:9)
# ; [] (1:19)
# ...
```

### In SuperPascal CI

SuperPascal's CI automatically downloads and uses the patched FPC:

```yaml
# .github/workflows/ci.yml
- name: Setup FPC
  uses: ./.github/actions/setup-fpc
  with:
    version: 'latest'
```

The `setup-fpc` action:
1. Detects your platform
2. Downloads the appropriate binary from GitHub Releases
3. Adds it to PATH
4. Verifies installation

### Manual Setup

If you're running tests locally:

```bash
# Option 1: Use the setup-fpc action (if in GitHub Actions)
# (See .github/actions/setup-fpc/action.yml)

# Option 2: Download manually
gh release download --repo casibbald/freePascal --pattern "fpc-*.tar.gz"
tar -xzf fpc-*.tar.gz
export PATH=$PWD/fpc-*/bin:$PATH

# Option 3: Build locally (see above)
```

## Development Workflow

### 1. Making Changes to SuperPascal

1. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature
   ```

2. Make your changes

3. Run tests:
   ```bash
   just test
   ```

4. Run compliance tests (requires patched FPC):
   ```bash
   cd crates/compiler-rs
   just fpc-tokenize test_cases/simple_program.pas
   ```

5. Commit and push:
   ```bash
   git commit -m "Add feature X"
   git push origin feature/your-feature
   ```

### 2. Making Changes to FPC Patches

If you need to modify the FPC patches:

1. Edit files in `../freePascal/compiler/`
2. Test locally:
   ```bash
   cd ../freePascal/compiler
   make cycle OPT="-n"
   ./ppca64 -dt test.pas
   ```
3. Commit to the `freePascal` repository
4. Push to trigger CI build
5. Update SuperPascal to use the new release

### 3. Testing Tokenizer Compliance

SuperPascal's tokenizer should match FPC's output:

```bash
# Generate FPC tokens
cd crates/compiler-rs/tests/compliance
fpc -dt test_cases/simple_program.pas > fpc_tokens.txt

# Generate SuperPascal tokens (once implemented)
cargo run --bin tokenizer test_cases/simple_program.pas > sp_tokens.txt

# Compare (manual for now, automated later)
diff fpc_tokens.txt sp_tokens.txt
```

## Code Style

### Rust (Compiler)

- Follow Rust standard formatting: `cargo fmt`
- Use `cargo clippy` for linting
- Document public APIs with `///` doc comments
- Use meaningful variable names

### Pascal (Standard Library)

- Follow Pascal naming conventions
- Use `mod.pas` pattern for library organization
- Document public procedures/functions
- Keep modules focused and reusable

## Testing

### Unit Tests

```bash
# Run all tests
just test

# Run specific crate tests
just test-crate tokens

# Run with coverage
just test-coverage
```

### Compliance Tests

Compliance tests compare SuperPascal with FPC:

```bash
cd crates/compiler-rs/tests/compliance
just fpc-tokenize test_cases/simple_program.pas
```

### Integration Tests

```bash
just test-integration
```

## Submitting Changes

### Pull Request Process

1. **Fork the repository** (if external contributor)
2. **Create a feature branch** from `main`
3. **Make your changes** with tests
4. **Run all tests** locally
5. **Update documentation** if needed
6. **Submit a pull request** with:
   - Clear description of changes
   - Reference to related issues
   - Test results

### Commit Messages

Follow conventional commits:
- `feat: Add new feature`
- `fix: Fix bug in tokenizer`
- `docs: Update CONTRIBUTING.md`
- `test: Add compliance tests`
- `refactor: Simplify parser logic`

### Code Review

- All PRs require review
- Address feedback promptly
- Keep PRs focused and small
- Update documentation as needed

## Questions?

- **Issues**: Open an issue on GitHub
- **Discussions**: Use GitHub Discussions
- **Documentation**: Check `docs/` directory
- **FPC Setup**: See `docs/FPC_GITHUB_ACTIONS.md`

## Acknowledgments

- **FreePascal Team** - For the excellent FPC compiler that we use as a reference
- **Contributors** - Everyone who helps improve SuperPascal

---

**Note**: The FreePascal "jerryrig" is a temporary development tool. Once SuperPascal's compiler is complete, we plan to phase out this dependency. The patches are minimal and focused solely on token dumping for compliance testing.

