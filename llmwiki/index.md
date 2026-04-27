# SuperPascal LLM Wiki Index

## Core

- [Schema](./SCHEMA.md)
- [Log](./log.md)
- [Docs Catalog](./docs-catalog.md)

## Compiler Architecture

- [Compiler Pipeline](./topics/compiler-pipeline.md) — lexer → parser → AST → semantics → IR → codegen → linker
- [Crate Layout](./reference/crate-layout.md) — 14-crate workspace map with responsibilities
- [Entry Points](./reference/codebase-entry-points.md) — file-level entry points and call chains
- [Semantic Analysis Modules](./topics/semantic-modules.md) — 7-module modularization

## Language Specification

- [Spec Overview](./topics/language-spec-overview.md) — language design, versioning, scope
- [Grammar](./reference/grammar.md) — grammar reference from `languageSpecification/02_Grammar.md`
- [Type System](./reference/type-system.md) — Pascal types, hybrid OOP+struct model
- [Intrinsics & Directives](./reference/intrinsics-and-directives.md) — hardware intrinsics, compiler directives
- [ABI & Calling Convention](./reference/abi.md) — Z80 Pascal calling convention, frame pointers, class layout
- [Exceptions Model](./reference/exceptions.md) — exception handling semantics

## Standard Library

- [Stdlib Modules](./reference/stdlib-modules.md) — 10 modules: collision, compression, crypto, ecs, game, graphics, math, physics, sorting, testing
- [ECS Library](./topics/ecs-library.md) — entity/component/system architecture
- [ECS + Physics Integration](./topics/ecs-library.md) — ECS + physics + sprites (in ECS Library page)

## Platforms

- [Platform Matrix](./reference/platforms.md) — ZealZ80, CommanderX16, Foenix65C816, FoenixA2560M, RaspberryPi5
- [ZealZ80 Platform](./topics/platform-zealz80.md) — Z80 @ 10 MHz, ZVB intrinsics, ZOF/ZOU/ZPK bundles
- [Foenix Platforms](./topics/platform-foenix.md) — 65C816 and 68K platforms

## Testing & Compliance

- [Test Architecture](./topics/testing-and-compliance.md) — unit, integration, compliance, tree-sitter corpus
- [FreePascal Fork Integration](./topics/fpc-fork-integration.md) — patched FPC, -dt flag, compliance testing

## Educational Content

- [Book Structure](./topics/educational-book.md) — 36 chapters, 33-week curriculum, teaching approach

## Current Development

- [Implementation Status](./topics/implementation-status.md) — parser complete, semantics complete, IR bridge incomplete
- [Next Steps Plan](./topics/next-steps.md) — priorities from NEXT-STEPS.md
- [Modern Pascal Compliance](./topics/modern-pascal-compliance.md) — 50-feature compliance tracking

## Reconciliation

- [Spec vs Code](./reconciliation/spec-vs-code.md) — language spec claims vs actual compiler implementation
