# Platforms

- Status: partially-verified
- Source docs: `platforms/README.md`, `platforms/ZealZ80/README.md`, `docs/TARGET_ARCHITECTURES.md`
- Code anchors: `platforms/`, `backends/backend-zealz80/`

## Platform matrix

| Platform | CPU | Clock | Address Bus | Tier | Target formats |
|----------|-----|-------|-------------|------|----------------|
| **Zeal 8-bit** | Z80 | 10 MHz | 16-bit | Tier 1 | .ZOF, .ZOU, .ZPK |
| **Commander X16** | 65C02 | 8 MHz | 16-bit | Tier 1 | — |
| **Foenix F256x** | WDC65C816S | 6.29 MHz | 24-bit | Tier 2 | — |
| **Foenix A2560M** | MC68LC060 | 66 MHz | 32-bit | Tier 2 | — |
| **Raspberry Pi 5** | ARM Cortex-A76 | 2.4 GHz | 64-bit | Tier 2 | ELF |

## ZealZ80 (Tier 1)

The primary target platform.

### Hardware
- Z80 @ 10 MHz
- Custom ZVB (Zeal Video Board) graphics
- Custom ZVB audio
- Page-aware memory paging

### ZVB Intrinsics
Located in `platforms/ZealZ80/intrinsics/`:
- `01_ZVBIntrinsics.md` — ZVB graphics intrinsics
- `02_MemoryIntrinsics.md` — Memory operations
- `03_DirectMemoryAccess.md` — DMA operations
- `03A_DirectMemoryAccess.md` — DMA details
- `03C_AbsoluteAddressing.md` — Absolute addressing
- `04_IOPortAccess.md` — I/O port access
- `05_AbsoluteAddressing.md` — More absolute addressing
- `06_GameEngineIntrinsics.md` — Game engine intrinsics
- `07_ExtendedAudioIntrinsics.md` — Extended audio

### ABI
- Pascal calling convention
- IX = frame pointer
- IY = exception frame chain
- Class layout: vtable at offset 0
- Record methods → free functions
- Large returns via hidden pointer parameter

### Output formats
- `.ZOF` — Zeal Object Format
- `.ZOU` — Zeal Union format
- `.ZPK` — Zeal Package (bundle)

### Game engine intrinsics
- Tilemap DSL
- Sprite rendering
- Animation system
- Scene management

## Commander X16 (Tier 1)

- 65C02 @ 8 MHz
- 16-bit address bus
- Commodore-era compatible
- Reference: `platforms/CommanderX16/README.md`

## Foenix 65C816 (Tier 2)

- WDC65C816S @ 6.29 MHz
- 24-bit addressing (16 MB address space)
- 16-bit core with 8-bit/16-bit modes
- Intrinsics in `platforms/Foenix65C816/intrinsics/`

## Foenix A2560M (Tier 2)

- MC68LC060 @ 66 MHz
- 32-bit Motorola 68K family
- Full 32-bit addressing

## Raspberry Pi 5 (Tier 2)

- ARM Cortex-A76 @ 2.4 GHz
- 64-bit ARM
- Reference: `platforms/RaspberryPi5/README.md`
- Likely target for ELF/native binaries

## Platform documentation

Each platform has a subdirectory with:
- `README.md` — Platform overview
- `intrinsics/` — Platform-specific intrinsics (where applicable)
- `gameEngine/` — Game engine integration (ZealZ80 only currently)
