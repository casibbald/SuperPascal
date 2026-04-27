# Intrinsics and Directives

- Status: partially-verified
- Source docs: `languageSpecification/06_IntrinsicsAndDirectives.md`, `languageSpecification/intrinsicsAndDirectives/`
- Code anchors: `platforms/ZealZ80/intrinsics/`, `parser/src/directives.rs`

## Intrinsics categories

### ZVB Intrinsics (platform-specific)
- `01_ZVBIntrinsics.md` — Zeal Video Board graphics intrinsics
- Graphics: tilemap rendering, sprite operations, color palette
- Video: screen modes, scrolling, display control

### Memory Intrinsics
- `02_MemoryIntrinsics.md` — Memory operations
- `03_DirectMemoryAccess.md` — DMA operations
- `03A_DirectMemoryAccess.md` — DMA details

### I/O Intrinsics
- `05_InputIntrinsics.md` — Input device handling
- `03C_AbsoluteAddressing.md` — Absolute memory addressing

### Audio Intrinsics
- `04_AudioIntrinsics.md` — Basic audio
- `11_ExtendedAudioIntrinsics.md` — Extended audio (mixers, streaming)

### Graphics Intrinsics
- `02_GraphicsIntrinsics.md` — Graphics operations
- `07_TilemapDSL.md` — Tilemap domain-specific language

### Game Engine Intrinsics
- `10_GameEngineIntrinsics.md` — Game engine operations
- `13_GameEngineDirectives.md` — Game engine compiler directives

### Absolute Addressing
- `03A_DirectMemoryAccess.md` — Direct memory access
- `04_AbsoluteAddressing.md` — Absolute address operations
- Port I/O via absolute addressing

### I/O Port Access
- `03B_IOPortAccess.md` — Direct I/O port access
- Hardware register access

### ZVB Intrinsics
- `02A_ZVBIntrinsics.md` — ZVB-specific intrinsics

### Intrinsics Implementation
- `08_IntrinsicImplementation.md` — How intrinsics are implemented in the compiler
- Compiler hooks for intrinsic resolution

### Intrinsic Summary
- `12_StandardIntrinsicsSummary.md` — Catalog of all standard intrinsics

### Directive Implementation
- `09_DirectiveProcessing.md` — How compiler directives are processed
- `06_CompilerDirectives.md` — Compiler directive syntax

### Summary Docs
- `14_StandardDirectivesSummary.md` — Catalog of all standard directives

## Modern Pascal compiler directives

Supported via `{$...}` syntax:
- `{$IFDEF}`, `{$IFNDEF}` — conditional compilation
- `{$IF}`, `{$ELSEIF}`, `{$ENDIF}` — numeric conditionals
- `{$DEFINE}`, `{$UNDEF}` — define/undefine symbols
- `{$INCLUDE}`, `{$I}` — include files
- `{$MODE}`, `{$H+}`, `{$H-}` — mode control

## Intrinsic implementation

Intrinsics are:
1. Recognized by name during semantic analysis
2. Resolved to platform-specific implementations
3. Lowered to IR operations
4. Generated as target-specific code in the backend

Platform intrinsics are documented per-platform in `platforms/<Platform>/intrinsics/`.
