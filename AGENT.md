# SuperPascal — agent rules

Strict operational rules for AI assistants working in this repository. **Knowledge about how SuperPascal works is in [`llmwiki/`](./llmwiki/), not here.** This file only holds rules the agent must obey.

---

## Before you do anything

1. Read [`llmwiki/index.md`](./llmwiki/index.md) — the content catalog.
2. Read [`llmwiki/SCHEMA.md`](./llmwiki/SCHEMA.md) — wiki conventions and workflows.
3. Tail [`llmwiki/log.md`](./llmwiki/log.md) for recent context from other sessions.
4. Read the specific topic / entity / reference pages relevant to your work. Drill into prose `docs/` or source only when the wiki flags drift or a gap.

Sessions that skip this waste work. The wiki is the compounding artifact that makes knowledge persist across sessions.

---

## Repository shape

- Primary language: **Rust** (workspace with 14 crates under `crates/compiler-rs/`).
- Primary language spec: **Pascal** (SuperPascal / FreePascal-compatible superset).
- Standard library: **Pascal modules** under `lib/`.
- Educational content: **Markdown books** under `book/` (36 chapters, 33 weeks).
- Compiler targets: **Z80** (Zeal 8-bit), **65C816** (Foenix), **68K** (Foenix A2560M), **ARM** (Raspberry Pi 5).
- Compliance reference: **FreePascal fork** at `../freePascal/` — forked and patched with `-dt` (dump tokens) flag for tokenizer comparison.

### Compiler crate layout

| Crate | Purpose |
|-------|---------|
| `tokens` | Token definitions |
| `lexer` | Lexical analyzer (tokenizer) |
| `ast` | AST node types |
| `errors` | Error types and reporting |
| `parser` | Recursive descent parser (statements, declarations, expressions, types, classes, properties, units, directives) |
| `types` | Type system |
| `symbols` | Symbol table |
| `semantics` | Semantic analysis (types, constants, feature checker, L-values) — modularized into 7 modules |
| `ir` | Intermediate representation (IR1 linear, IR2 optional three-address) |
| `runtime-spec` | Runtime capabilities spec |
| `runtime` | Runtime library (array, closure, interface, variant) |
| `backends/backend-zealz80` | Z80 backend codegen |
| `objects/object-zealz80` | Z80 object file format |
| `driver` | Compiler driver (`compiler.rs`, `main.rs`) |
| `tests/tree-sitter-corpus` | Tree-sitter-style corpus tests |

### Key source files

- `crates/compiler-rs/driver/src/compiler.rs` — main compilation pipeline entry
- `crates/compiler-rs/parser/src/lib.rs` — parser facade
- `crates/compiler-rs/semantics/MODULARIZATION_PLAN.md` — semantics module structure
- `crates/compiler-rs/tests/compliance/` — FPC compliance tests
- `languageSpecification/` — language spec documents (00–17, algorithms, ecs, intrinsicsAndDirectives)
- `book/` — educational book content (36 chapters)
- `lib/` — standard library Pascal modules (10 modules)
- `platforms/` — platform-specific documentation (ZealZ80, CommanderX16, Foenix65C816, FoenixA2560M, RaspberryPi5)
- `docs/` — project documentation (strategy, architecture, audits, reference)

---

## Build, lint, test commands

All commands run with working directory at `SuperPascal/`.

### Recommended entry points (justfile)

- `just build` — `cargo build` in `crates/compiler-rs/`
- `just build-release` — `cargo build --release`
- `just test` — `cargo nextest run --show-progress=running`
- `just test-verbose` — `cargo nextest run --verbose`
- `just test-crate CRATE` — `cargo nextest run -p <CRATE>`
- `just test-unit` — `cargo nextest run --lib`
- `just test-integration` — `cargo nextest run --test '*'`
- `just test-compliance` — `cargo nextest run --test compliance`
- `just test-coverage` — `cargo llvm-cov --all-features --workspace --lcov`
- `just test-coverage-summary` — `cargo llvm-cov --summary-only`
- `just test-coverage-html` — `cargo llvm-cov --html`
- `just ci` — `fmt --check` + `clippy -D warnings` + `nextest run` + `doc --no-deps`
- `just fmt` — `cargo fmt`
- `just clippy` — `cargo clippy --all-targets -- -D warnings`
- `just clippy-fix` — `cargo clippy --fix --allow-dirty`
- `just clean` — `cargo clean`
- `just rebuild` — `cargo clean && cargo build`
- `just check` — `cargo check`
- `just check-all` — `cargo check --all-targets`
- `just fpc-tokenize FILE` — run patched FPC tokenizer on a test case
- `just build-fpc` — build patched FreePascal from `../freePascal/`

### Fallback cargo commands (if just is not installed)

```bash
cd crates/compiler-rs
cargo build                              # build
cargo nextest run                        # test
cargo test --lib                         # unit tests
cargo clippy --all-targets -- -D warnings # lint
cargo fmt                                # format
cargo llvm-cov --all-features --workspace --lcov # coverage
```

---

## Core rules the agent must obey

### 1. Language spec is authoritative

The `languageSpecification/` directory is the source of truth for SuperPascal language features. The spec is versioned (v0.1–v0.8) and covers: lexical structure, grammar, type system, semantics, intrinsics & directives, ABI, exceptions, game engine, ECS library, standard library, memory management, and modern Pascal features. Always check the spec before implementing a feature.

### 2. Follow Rust conventions

- `snake_case` for fns / modules, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Group imports: std, external crates, local modules. Prefer explicit imports.
- `Result<T, E>` + `?` over `panic!` in library paths. No `unwrap()` / `expect()` in production.
- Follow the coding guidelines in [`tasks/rust-guidelines.txt`](./tasks/rust-guidelines.txt).

### 3. Compliance testing with patched FPC

SuperPascal maintains a forked FreePascal compiler with a `-dt` (dump tokens) flag for tokenizer comparison. Compliance tests are in `crates/compiler-rs/tests/compliance/` and compare SuperPascal's token output against FPC. Always run compliance tests when modifying lexer, tokens, or parser.

**Key files in the FPC fork:**
- `freePascal/compiler/globals.pas` — `para_dump_tokens` flag
- `freePascal/compiler/options.pas` — `-dt` option handler
- `freePascal/compiler/tokens.pas` — `TokenKindToString` function
- `freePascal/compiler/scanner.pas` — modified `readtoken` to output tokens

The FPC fork is at `../freePascal/` (sibling of SuperPascal). Releases at `https://github.com/casibbald/freePascal/releases`.

### 4. Semantic analysis is modular

The semantics crate (`crates/compiler-rs/semantics/`) was recently modularized from a single `lib.rs` into 7 focused modules:
- `expressions.rs` — expression analysis
- `statements.rs` — statement analysis
- `declarations.rs` — type/procedure/variable declarations
- `types.rs` — type system analysis
- `constants.rs` — constant folding and evaluation
- `lvalues.rs` — L-value analysis
- `feature_checker.rs` — modern Pascal feature gate checks

Read `semantics/MODULARIZATION_PLAN.md` for the architecture.

### 5. Testing discipline

- Run `just test` or `just ci` before submitting.
- Keep tests deterministic; avoid global state.
- Tree-sitter corpus tests in `tests/tree-sitter-corpus/corpus/` cover: statements, modules, declarations, routines, expressions, literals, lambdas, attributes, preprocessor, generics (FPC + Delphi), inline variables, Pascalocoa.
- Compliance tests in `tests/compliance/` compare against FreePascal.

### 6. Documentation location

Prose docs go in `docs/`. Language spec goes in `languageSpecification/`. Educational book goes in `book/`. Do NOT create ad-hoc notes in random `docs/` files — use the wiki (`llmwiki/`) for session-persistent knowledge.

### 7. Compiler architecture is multi-phase

The compiler pipeline is: Lexer → Parser → AST → Semantic Analyzer → IR Generation → Codegen → Linker. IR has two levels: IR1 (linear, low-level Pascal) and IR2 (optional three-address form). The current implementation focus is on completing the parser and semantics; the IR→codegen bridge is incomplete (only basic variant types are converted). See `NEXT-STEPS.md` for priorities.

---

## Commit discipline

- Commits follow Conventional Commits (`feat(scope):`, `fix(scope):`, `docs(scope):`, `chore(scope):`, `refactor(scope):`).
- **Never push** without explicit human authorization.
- **Never use `--no-verify`** or `--no-verify-commit`. Let pre-commit hooks run.
- **Never commit secrets** (`.env`, credentials, tokens).

---

## Working with the freePascal fork

The FPC fork is a **development tool only** — a temporary "jerryrig" for compliance testing until SuperPascal's own compiler is complete. It is NOT part of the final product.

When modifying FPC patches:
1. Edit files in `../freePascal/compiler/`
2. Test locally: `cd ../freePascal/compiler && make cycle OPT="-n"`
3. Commit to the `freePascal` repository separately
4. Push to trigger CI build
5. Update SuperPascal to use the new release

When modifying SuperPascal compiler code:
1. Run `just test` to verify changes
2. Run `just fpc-tokenize test_cases/*.pas` to verify tokenizer compliance
3. Update compliance test expected outputs if FPC behavior changed

---

## Current state and priorities

From `NEXT-STEPS.md` (last updated 2024-12-23):
- Parser: 100% complete (50/50 Modern Pascal features)
- Semantic analysis: 100% complete
- **AST-to-IR conversion is INCOMPLETE** — highest priority gap. Only `build_var_decl`, `build_assign_stmt`, and basic `build_expression` are implemented.
- Missing from AST→IR: `build_if_stmt`, `build_while_stmt`, `build_for_stmt`, `build_case_stmt`, `build_call_stmt`, `build_block` improvements
- High-priority parser features still needed: Generics (40-60h), Dynamic Arrays (8-16h), Preprocessor (24-40h), Class Properties/Variables (4-8h)
- Codegen: Backend exists but is incomplete for most IR types

---

## Useful files

- [`README.md`](./README.md) — project overview
- [`CONTRIBUTING.md`](./CONTRIBUTING.md) — contributing guide + FPC jerryrig setup
- [`NEXT-STEPS.md`](./NEXT-STEPS.md) — implementation roadmap + priorities
- [`tasks/rust-guidelines.txt`](./tasks/rust-guidelines.txt) — Rust coding guidelines
- [`justfile`](./justfile) — command runner recipes
- [`crates/compiler-rs/Cargo.toml`](./crates/compiler-rs/Cargo.toml) — workspace config
- [`languageSpecification/00_Overview.md`](./languageSpecification/00_Overview.md) — language spec entry point
- [`book/33_LanguageEvolutionAndContributing/README.md`](./book/33_LanguageEvolutionAndContributing/README.md) — compiler architecture chapter
- [`docs/COMPILER_IMPLEMENTATION_STRATEGY.md`](./docs/COMPILER_IMPLEMENTATION_STRATEGY.md) — implementation strategy
- [`docs/FPC_PATCH_SUMMARY.md`](./docs/FPC_PATCH_SUMMARY.md) — FPC patch details
- [`docs/SUPERPASCAL_VS_FPC_FEATURE_MATRIX.md`](./docs/SUPERPASCAL_VS_FPC_FEATURE_MATRIX.md) — feature comparison
- [`tasks/modern-pascal-compliance-todo.md`](./tasks/modern-pascal-compliance-todo.md) — compliance tracking
- [`tasks/parser-phase4-todo.md`](./tasks/parser-phase4-todo.md) — parser TODOs
- [`tasks/variant-runtime-plan.md`](./tasks/variant-runtime-plan.md) — variant type implementation plan

---

## Explicit instruction: read the wiki

**Every session starts with reading [`llmwiki/`](./llmwiki/).** This is not optional.

End-of-session: update the wiki pages your work touched, append a `log.md` entry, flag any `> **Open:**` questions. Leave the wiki one step more useful than you found it.
