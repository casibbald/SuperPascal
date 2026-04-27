# Docs Catalog (Initial Ingest)

- Status: partially-verified
- Scope: full `docs/**/*.md` inventory plus `languageSpecification/**`, `book/**`, `lib/**`, `platforms/**`

## Inventory by major area

| Area | Count | Notes |
|---|---:|---|
| `docs/` (root-level) | ~45 | Strategy, architecture, FPC, audits, reference |
| `docs/historical_superpascal/` | ~8 | Historical SP analysis, concurrency, spec updates |
| `docs/mikro_docs_archive/` | ~80 | Archived mikro computing docs (coding, Atari, ST_E, GEM, etc.) |
| `docs/scripts/` | ~5 | Python scrapers for docs collection |
| `docs/lib/` | ~12 | Standard library design docs (ecs, physics, math, testing) |
| `languageSpecification/` | ~50 | Language spec (00–17, algorithms, ecs, intrinsicsAndDirectives) |
| `book/` | ~36 | Educational book chapters (00–33) |
| `lib/` | ~60 | Standard library Pascal modules |
| `platforms/` | ~30 | Platform-specific docs (ZealZ80, CommanderX16, Foenix, etc.) |
| `tasks/` | ~9 | Task planning, compliance tracking, specs |

## Root-level docs

- `docs/00_ZealPascal.md` — ZealPascal project overview
- `docs/01_ZealPascalGraphics.md` — Graphics system spec
- `docs/03_ZealPascalGameEngine.md` — Game engine PRD
- `docs/04_ZealPascalAudoSystem.md` — Audio system spec
- `docs/04_ZealPascalAudoSystem2.md` — Audio system v2
- `docs/05_ZealPascalLanguageSpecification_v1.md` — Language spec v1
- `docs/05_ZealPascalLanguageSpecification_v2.md` — Language spec v2
- `docs/05_ZealPascalLanguageSpecification_v3.md` — Language spec v3
- `docs/06_ZealPascalGrammarDefinition.md` — Grammar definition
- `docs/07_ZealPascalSymanticRules.md` — Semantic rules
- `docs/08_ZealPascalLanguageSpecificationABIcodeGenerationRules.md` — ABI + codegen
- `docs/09_ZealPascalStandardLibrarySpecification.md` — StdLib spec
- `docs/14_ZealPascalTeacherGuide.md` — Teacher guide
- `docs/16_ZealPascalBootstrappingPlan.md` — Bootstrapping plan
- `docs/17_ZealPascalBootstrapCompilerChecklist.md` — Bootstrap checklist
- `docs/18_ZealPascalZC1CompilerChecklist.md` — ZC1 compiler checklist
- `docs/21_ZealPascalRoadmap(RustToolchainOnly).md` — Rust toolchain roadmap
- `docs/25_ZealPascalFutureDeliverables.md` — Future deliverables
- `docs/27_Pageable_RAM.md` — Pageable RAM
- `docs/28_PascalOS.md` — Pascal OS
- `docs/26_HardwareInterfacing/` — Hardware interfacing docs
- `docs/ALGORITHM_ORGANIZATION_PLAN.md` — Algorithm organization
- `docs/BOOK_RESTRUCTURING_PLAN.md` — Book restructuring
- `docs/BOOK_WRITING_PLAN.md` — Book writing plan
- `docs/BUILD_FPC.md` — FPC build instructions
- `docs/CHAPTER_SIZE_GUIDELINES.md` — Chapter size guidelines
- `docs/COMPILER_IMPLEMENTATION_STRATEGY.md` — Implementation strategy
- `docs/DEVELOPER_FAQ.md` — Developer FAQ
- `docs/ECS_LIBRARY_INTEGRATION_PLAN.md` — ECS integration plan
- `docs/ERROR_DESIGN.md` — Error design
- `docs/EXTERNAL_RESOURCES.md` — External resources
- `docs/FPC_BUILD_SUCCESS.md` — FPC build success log
- `docs/FPC_INTEGRATION_SUMMARY.md` — FPC integration summary
- `docs/FPC_PATCH_PLAN.md` — FPC patch plan
- `docs/FPC_PATCH_SUMMARY.md` — FPC patch summary
- `docs/FPC_SOURCE_REFERENCE_GUIDE.md` — FPC source reference
- `docs/FPC_BACKEND_COVERAGE.md` — FPC backend coverage
- `docs/FPC_Z80_BACKEND_REFERENCE.md` — FPC Z80 backend ref
- `docs/LIBRARY_ORGANIZATION.md` — Library organization
- `docs/LIBRARY_ORGANIZATION.md` — Library organization
- `docs/LLVM_BACKEND_PATH.md` — LLVM backend path
- `docs/LLVM_MOS_SDK_REFERENCE.md` — LLVM-MOS SDK reference
- `docs/MARKDOWN_ANALYSIS.md` — Markdown analysis
- `docs/MARKDOWN_ANALYSIS.md` — Markdown analysis
- `docs/MULTI_PLATFORM_ARCHITECTURE.md` — Multi-platform architecture
- `docs/OPTIMIZATION_ROADMAP.md` — Optimization roadmap
- `docs/PDF_ANALYSIS.md` — PDF analysis
- `docs/PHASE2_VALIDATION_REPORT.md` — Phase 2 validation
- `docs/PHASE4_INTEGRATION_SUMMARY.md` — Phase 4 integration
- `docs/QUICK_START_COMPILER.md` — Quick start guide
- `docs/RUST_CRATE_LAYOUT_PROPOSAL.md` — Crate layout proposal
- `docs/SUPERPASCAL_VS_FPC_FEATURE_MATRIX.md` — Feature matrix vs FPC
- `docs/TARGET_ARCHITECTURES.md` — Target architectures
- `docs/TREE-SITTER-CORPUS-INTEGRATION.md` — Tree-sitter integration
- `docs/TREE-SITTER-PASCAL-INTEGRATION-ANALYSIS.md` — Tree-sitter analysis
- `docs/TURBOPASCAL_EXTRACTION.md` — TurboPascal extraction
- `docs/TURBOPASCAL_TO_SUPERPASCAL_MAP.md` — TurboPascal → SuperPascal map
- `docs/TURBO51_ANALYSIS.md` — Turbo51 analysis
- `docs/TURBOPASCAL_ON_DEVICE_ANALYSIS.md` — TurboPascal on-device analysis
- `docs/Z80_MICRO_COMPILER_SUBSET.md` — Z80 micro compiler subset
- `docs/WIRTH_BOOK_REFERENCE.md` — Wirth book reference
- `docs/8051_TARGET_ANALYSIS.md` — 8051 target analysis
- `docs/MOTOROLA_SOURCE_CODE_REFERENCE.md` — Motorola reference
- `docs/BUILD_FPC.md` — Build FPC
- `docs/MIKRO_FILE_INVENTORY.md` — Mikro docs inventory
- `docs/MIKRO_COMPLETE_EXTRACTION_PLAN.md` — Mikro extraction plan
- `docs/RECONSILIATION/` — Reconciliation docs

## LLM wiki synthesis index

| Wiki page | Maps to |
|-----------|---------|
| `topics/compiler-pipeline.md` | Full compiler pipeline overview |
| `reference/crate-layout.md` | 14-crate workspace |
| `topics/language-spec-overview.md` | `languageSpecification/00_Overview.md` |
| `topics/implementation-status.md` | `NEXT-STEPS.md`, `tasks/modern-pascal-compliance-todo.md` |
