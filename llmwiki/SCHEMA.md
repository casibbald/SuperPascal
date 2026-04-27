# SCHEMA.md

## Purpose
This wiki is a persistent, code-anchored knowledge layer between `docs/`, `languageSpecification/` and the Rust compiler codebase.

## Source of Truth Order
1. Runtime behavior in `crates/compiler-rs/src/**` and `crates/compiler-rs/tests/**`
2. Language spec in `languageSpecification/`
3. Existing prose docs in `docs/**`
4. This wiki (`llmwiki/**`) as reconciled synthesis

## Page Conventions
- Every substantive page includes:
  - **Status** (`verified`, `partially-verified`, `unverified`)
  - **Source docs** (`docs/...` links)
  - **Code anchors** (absolute repository paths)
  - **Gaps / drift** (doc claim vs code reality)
- Prefer explicit file paths and function names over high-level claims.
- Keep operational instructions executable and minimal.

## Operational Workflows
- **Ingest**: add/refresh entries from `docs/**` and `languageSpecification/**` into `llmwiki/docs-catalog.md`, then reconcile with code.
- **Query**: answer from `llmwiki/index.md` + linked pages first, then verify in code when uncertain.
- **Lint**: regularly check for stale claims in wiki pages.

## Logging
- Append session updates to `llmwiki/log.md`.
- Keep entries chronological and append-only.
