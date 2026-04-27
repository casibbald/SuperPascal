# Language Specification Overview

- Status: partially-verified
- Source docs: `languageSpecification/00_Overview.md`, `languageSpecification/01_LexicalStructure.md`, `languageSpecification/02_Grammar.md`
- Code anchors: `languageSpecification/`

## Spec structure

The language specification is organized into numbered documents:

| Doc | Topic | Version |
|-----|-------|---------|
| `00_Overview.md` | Project overview, language goals, history | v0.1 |
| `01_LexicalStructure.md` | Tokens, identifiers, literals, whitespace | v0.1 |
| `02_Grammar.md` | BNF grammar, parsing rules | v0.2 |
| `03_TypeSystem.md` | Pascal types, hybrid OOP+struct, type compatibility | v0.3 |
| `04_Semantics.md` | Semantic rules, scoping, name resolution | v0.3 |
| `05_ABI_Concepts.md` | Calling convention, frame pointers, class layout | v0.4 |
| `06_IntrinsicsAndDirectives.md` | Overview of intrinsics and directives | v0.4 |
| `07_Exceptions.md` | Exception handling model | v0.7 |
| `08_LanguageReference.md` | Language reference summary | — |
| `09_GameEngine.md` | Game engine design | — |
| `09_ECS_Library.md` | ECS library spec | — |
| `10_AudioSystem.md` | Audio system | — |
| `11_LanguageExtensions.md` | Modern Pascal extensions | — |
| `12_MemoryAndIO.md` | Memory management, I/O | — |
| `13_StandardLibrary.md` | Standard library specification | v1.0 |
| `14_RustStyleModules.md` | Rust-style module system | — |
| `15_MemoryManagement.md` | Memory management | — |
| `16_ModernPascalFeatures.md` | Modern Pascal feature checklist | — |
| `17_CapabilityBasedMemory.md` | Capability-based memory model | — |
| `99_Algorithms_Appendix.md` | Algorithm reference appendix | — |

## Subdirectories

| Directory | Contents |
|-----------|----------|
| `algorithms/` | 9 algorithm categories: fixed-point math, mathematical, sorting, graphics, collision, physics, game, advanced graphics, utility |
| `ecs/` | 6 ECS iterations (1–6) with unit code examples and design docs |
| `intrinsicsAndDirectives/` | 14 docs covering overview, graphics, memory, I/O, audio, ZVB, direct memory, absolute addressing, game engine, implementation, tilemap DSL, intrinsics summary, directives summary, directive processing |

## Language design principles

- Modern Pascal superset (FreePascal-compatible)
- Hybrid OOP + struct model (records for SoA data, classes for behavior)
- ECS-friendly data layout
- Hardware-accelerated math, fixed-point arithmetic
- Multi-platform targeting (Z80, 65C816, 68K, ARM)
- Educational focus with 33-week curriculum
- Deterministic semantics, no GC
- Single unified string type

## Version history

- v0.1–v0.3: Core language spec, grammar, semantics
- v0.4: ABI + codegen
- v0.7: Exceptions model
- v0.8: Compiler architecture
- v1.0: StdLib specification, Teacher Guide
