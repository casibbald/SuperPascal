# SuperPascal Compiler - Justfile
# Command runner for common development tasks
# Located at project root, commands run in crates/compiler-rs

# Default recipe: show available commands
default:
    @just --list

# ===== Testing =====

# Run all tests with nextest (fast, parallel, clean output)
test:
    @cd crates/compiler-rs && cargo nextest run --show-progress=running

# Run all tests with nextest (verbose output)
test-verbose:
    @cd crates/compiler-rs && cargo nextest run --verbose

# Run tests for a specific crate
test-crate CRATE:
    @cd crates/compiler-rs && cargo nextest run -p {{CRATE}}

# Run tests with coverage
test-coverage:
    @cd crates/compiler-rs && cargo nextest run --profile coverage

# Run only unit tests
test-unit:
    @cd crates/compiler-rs && cargo nextest run --lib

# Run only integration tests
test-integration:
    @cd crates/compiler-rs && cargo nextest run --test '*'

# Run compliance tests
test-compliance:
    @cd crates/compiler-rs && cargo nextest run --test compliance

# Run tests in watch mode (requires cargo-watch)
test-watch:
    @cd crates/compiler-rs && cargo watch -x "nextest run"

# ===== Building =====

# Build all crates
build:
    @cd crates/compiler-rs && cargo build

# Build in release mode
build-release:
    @cd crates/compiler-rs && cargo build --release

# Build a specific crate
build-crate CRATE:
    @cd crates/compiler-rs && cargo build -p {{CRATE}}

# Check code (without building)
check:
    @cd crates/compiler-rs && cargo check

# Check all targets
check-all:
    @cd crates/compiler-rs && cargo check --all-targets

# ===== Code Quality =====

# Format code
fmt:
    @cd crates/compiler-rs && cargo fmt

# Format and check
fmt-check:
    @cd crates/compiler-rs && cargo fmt --check

# Run clippy
clippy:
    @cd crates/compiler-rs && cargo clippy --all-targets -- -D warnings

# Run clippy with fixes
clippy-fix:
    @cd crates/compiler-rs && cargo clippy --fix --allow-dirty

# ===== Documentation =====

# Build documentation
doc:
    @cd crates/compiler-rs && cargo doc --no-deps --open

# Build documentation for all crates
doc-all:
    @cd crates/compiler-rs && cargo doc --all --no-deps --open

# ===== Development =====

# Clean build artifacts
clean:
    @cd crates/compiler-rs && cargo clean

# Clean and rebuild
rebuild:
    @cd crates/compiler-rs && cargo clean && cargo build

# Run a specific example
run-example EXAMPLE:
    @cd crates/compiler-rs && cargo run --example {{EXAMPLE}}

# ===== CI/CD =====

# Run all checks (for CI)
ci:
    @cd crates/compiler-rs && cargo fmt --check
    @cd crates/compiler-rs && cargo clippy --all-targets -- -D warnings
    @cd crates/compiler-rs && cargo nextest run
    @cd crates/compiler-rs && cargo doc --no-deps

# ===== Workspace Management =====

# Add a new crate to the workspace
new-crate NAME:
    @cd crates/compiler-rs && cargo new --lib {{NAME}}
    @echo "Don't forget to add {{NAME}} to Cargo.toml members!"

# Update dependencies
update:
    @cd crates/compiler-rs && cargo update

# ===== FreePascal Integration =====

# Build FreePascal compiler from source
build-fpc:
    @cd ../freePascal && make build

# Install FreePascal compiler
install-fpc:
    @cd ../freePascal && sudo make install

# Run FPC tokenizer wrapper script on test case
fpc-tokenize FILE:
    @cd crates/compiler-rs/tests/compliance && ./fpc_tokenize.sh test_cases/{{FILE}}

# ===== Utilities =====

# Show workspace structure
tree:
    @cd crates/compiler-rs && cargo tree

# Show workspace members
members:
    @cd crates/compiler-rs && grep -E '^\s*"[^"]+",' Cargo.toml | sed 's/.*"\(.*\)".*/\1/'

# Show test statistics
test-stats:
    @cd crates/compiler-rs && cargo nextest run --test-threads 1 --no-capture 2>&1 | grep -E "(test result|running)"
