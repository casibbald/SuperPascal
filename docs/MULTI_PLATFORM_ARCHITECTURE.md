# SuperPascal Multi-Platform Architecture Proposal

## Executive Summary

**Current State:** ZealPascal is tightly coupled to the Zeal 8-bit Computer (Z80) platform.

**Proposed State:** SuperPascal - A modern Pascal superset compiler supporting multiple retro computing platforms:
- **ZealZ80** - Zeal 8-bit Computer (Z80 @ 10 MHz)
- **Foenix65C816** - Foenix F256x systems (WDC W65C816S @ 6.29 MHz, 16-bit microprocessor with 24-bit addressing, 16MB address space)
- **FoenixA2560M** - Foenix A2560M/A2560X (MC68LC060 @ 66 MHz, 32-bit system with 1GB DDR3)

**Note:** Only WDC65C816 is supported for Foenix due to its extended memory capabilities (24-bit addressing, 16MB address space). The 6502 is not supported.

**Key Insight:** The language specification (lexical, grammar, types, semantics) is **platform-agnostic**. Only intrinsics, ABI, and runtime are platform-specific.

---

## Architecture Principles

### 1. Platform-Agnostic Core
- **Lexical Structure** - Same across all platforms
- **Grammar** - Same syntax, same EBNF
- **Type System** - Same types, same rules
- **Semantics** - Same meaning, same behavior
- **Language Extensions** - Same struct syntax, same features

### 2. Platform-Specific Components
- **Intrinsics** - Hardware-specific operations
- **ABI** - CPU-specific calling conventions
- **Runtime** - Platform-specific runtime support
- **Standard Library** - Some platform-specific units

### 3. Compiler Architecture
- **Frontend** (Lexer, Parser, AST, Semantics) - Shared across all platforms
- **IR** - Platform-agnostic intermediate representation
- **Backend** (Codegen) - Platform-specific code generators
- **Linker** - Platform-specific object file formats

---

## Proposed Directory Structure

```
SuperPascal/
├── README.md                    # Updated for multi-platform
├── ROADMAP.md                   # Updated roadmap
├── CONTRIBUTING.md              # Updated contribution guide
│
├── languageSpecification/      # PLATFORM-AGNOSTIC
│   ├── 00_Overview.md
│   ├── 01_LexicalStructure.md
│   ├── 02_Grammar.md
│   ├── 03_TypeSystem.md
│   ├── 04_Semantics.md
│   ├── 05_LanguageReference.md
│   ├── 06_Exceptions.md
│   ├── 07_LanguageExtensions.md
│   └── intrinsicsAndDirectives/
│       ├── 00_Overview.md       # Platform-agnostic intrinsic concepts
│       └── ...                  # Platform-agnostic directives
│
├── platforms/                   # PLATFORM-SPECIFIC
│   ├── README.md                # Platform selection guide
│   │
│   ├── ZealZ80/                 # Zeal 8-bit Computer (Z80)
│   │   ├── README.md
│   │   ├── ABI.md               # Z80 calling conventions
│   │   ├── intrinsics/          # Z80-specific intrinsics
│   │   │   ├── 01_Overview.md
│   │   │   ├── 02_ZVBIntrinsics.md
│   │   │   ├── 03_MemoryIntrinsics.md
│   │   │   ├── 04_AudioIntrinsics.md
│   │   │   ├── 05_GameEngineIntrinsics.md
│   │   │   └── 06_Summary.md
│   │   ├── runtime/             # Z80 runtime spec
│   │   └── stdlib/              # Z80-specific stdlib units
│   │
│   ├── Foenix65C816/            # Foenix (WDC W65C816)
│   │   ├── README.md
│   │   ├── ABI.md               # 65C816 calling conventions
│   │   ├── intrinsics/          # 65C816-specific intrinsics
│   │   ├── runtime/
│   │   └── stdlib/
│   │
│   └── FoenixA2560M/            # Foenix A2560M (MC68LC060)
│       ├── README.md
│       ├── ABI.md               # 68060 calling conventions
│       ├── intrinsics/          # A2560M-specific intrinsics
│       ├── runtime/
│       └── stdlib/
│
├── compiler-rs/                 # Rust compiler (shared frontend)
│   ├── src/
│   │   ├── frontend/            # Shared: Lexer, Parser, AST, Semantics
│   │   ├── ir/                  # Platform-agnostic IR
│   │   ├── backend/             # Platform-specific backends
│   │   │   ├── mod.rs
│   │   │   ├── zealz80/
│   │   │   ├── foenix65c816/
│   │   │   └── foenix2560m/
│   │   └── linker/              # Platform-specific linkers
│   └── Cargo.toml
│
├── runtime/                     # Platform-specific runtimes
│   ├── zealz80/
│   ├── foenix65c816/
│   └── foenix2560m/
│
├── stdlib/                      # Standard library
│   ├── common/                  # Platform-agnostic units
│   │   ├── System.pas
│   │   ├── SysUtils.pas
│   │   └── Math.pas
│   └── platforms/               # Platform-specific units
│       ├── zealz80/
│       │   ├── ZVB_Graphics.pas
│       │   ├── ZVB_Audio.pas
│       │   └── ZealOS.pas
│       └── foenix65c816/
│           ├── Foenix_Graphics.pas
│           ├── Foenix_Audio.pas
│           └── FoenixOS.pas
│
├── examples/                    # Example programs
│   ├── common/                  # Platform-agnostic examples
│   └── platforms/               # Platform-specific examples
│       ├── zealz80/
│       └── foenix65c816/
│
└── tests/                       # Test suites
    ├── common/                  # Platform-agnostic tests
    └── platforms/               # Platform-specific tests
        ├── zealz80/
        └── foenix65c816/
```

---

## Migration Strategy

### Phase 1: Restructure Existing Code (Non-Breaking)

1. **Create `platforms/` directory**
   ```bash
   mkdir -p platforms/ZealZ80/{intrinsics,runtime,stdlib}
   ```

2. **Move platform-specific specs**
   - `languageSpecification/05_ABI.md` → `platforms/ZealZ80/ABI.md`
   - `languageSpecification/intrinsicsAndDirectives/02A_ZVBIntrinsics.md` → `platforms/ZealZ80/intrinsics/02_ZVBIntrinsics.md`
   - All Z80-specific intrinsics → `platforms/ZealZ80/intrinsics/`

3. **Update references**
   - Update all cross-references in spec files
   - Update README, ROADMAP, CONTRIBUTING

4. **Create platform index**
   - `platforms/ZealZ80/README.md` - Platform overview
   - `platforms/README.md` - Platform selection guide

### Phase 2: Rename Project (Breaking)

1. **Rename repository**
   - `ZealPascal` → `SuperPascal`

2. **Update all references**
   - Find/replace "ZealPascal" → "SuperPascal" in docs
   - Update package names, module names
   - Update CI/CD configurations

3. **Update branding**
   - README.md
   - All documentation headers
   - License headers

### Phase 3: Add New Platforms (Incremental)

1. **Foenix65C816**
   - Create `platforms/Foenix65C816/`
   - Define ABI for WDC65C816S @ 6.29 MHz (16-bit processor, 24-bit addressing, 16MB address space)
   - Define intrinsics for Foenix hardware (VICKY graphics, IO Pages, audio chips)
   - Implement backend in `compiler-rs/src/backend/foenix65c816/`
   - References:
     - [Foenix Hardware Wiki](https://wiki.f256foenix.com/index.php?title=Hardware)
     - [Foenix Wiki](https://wiki.f256foenix.com/index.php?title=Main_Page)
     - [WDC 65C816 Wikipedia](https://en.wikipedia.org/wiki/WDC_65C816)
     - [WDC 65C816S Datasheet](https://www.westerndesigncenter.com/wdc/documentation/w65c816s.pdf)

2. **FoenixA2560M**
   - Create `platforms/FoenixA2560M/`
   - Define ABI for MC68LC060 @ 66 MHz (32-bit, 1GB DDR3)
   - Define intrinsics for A2560M hardware (VICKY III graphics, audio chips, PCIe, USB)
   - Implement backend in `compiler-rs/src/backend/foenix2560m/`
   - Reference: [A2560M Specifications](https://c256foenix.com/a2560m/)

---

## Platform-Specific vs Platform-Agnostic

### Platform-Agnostic (Shared)

| Component | Location | Notes |
|-----------|----------|-------|
| Lexical Structure | `languageSpecification/01_LexicalStructure.md` | Same tokens, same keywords |
| Grammar | `languageSpecification/02_Grammar.md` | Same EBNF, same syntax |
| Type System | `languageSpecification/03_TypeSystem.md` | Same types, same rules |
| Semantics | `languageSpecification/04_Semantics.md` | Same meaning |
| Language Extensions | `languageSpecification/07_LanguageExtensions.md` | Struct syntax, etc. |
| Exceptions | `languageSpecification/06_Exceptions.md` | Exception model |
| Compiler Frontend | `compiler-rs/src/frontend/` | Shared lexer, parser, AST |
| IR | `compiler-rs/src/ir/` | Platform-agnostic IR |

### Platform-Specific (Per-Platform)

| Component | Location | Notes |
|-----------|----------|-------|
| ABI | `platforms/{Platform}/ABI.md` | CPU-specific calling conventions |
| Intrinsics | `platforms/{Platform}/intrinsics/` | Hardware-specific operations |
| Runtime | `platforms/{Platform}/runtime/` | Platform-specific runtime |
| Codegen | `compiler-rs/src/backend/{platform}/` | CPU-specific code generation |
| Linker | `compiler-rs/src/linker/{platform}/` | Object file format |
| Stdlib Units | `stdlib/platforms/{platform}/` | Hardware-specific units |

---

## Compiler Architecture Changes

### Current (ZealPascal)
```
Source → Lexer → Parser → AST → Semantics → IR → Z80 Codegen → Object File
```

### Proposed (SuperPascal)
```
Source → Lexer → Parser → AST → Semantics → IR → [Platform Backend] → Object File
                                                      ├─ ZealZ80
                                                      ├─ Foenix65C816
                                                      └─ FoenixA2560M
```

### Backend Interface

```rust
// compiler-rs/src/backend/trait.rs
pub trait CodegenBackend {
    fn generate_function(&self, func: &IRFunction) -> Vec<u8>;
    fn generate_data(&self, data: &IRData) -> Vec<u8>;
    fn register_usage(&self) -> RegisterConvention;
    fn calling_convention(&self) -> CallingConvention;
}

// Platform implementations
impl CodegenBackend for ZealZ80Backend { ... }
impl CodegenBackend for Foenix65C816Backend { ... }
impl CodegenBackend for FoenixA2560MBackend { ... }
```

---

## Benefits of This Architecture

### 1. **Code Reuse**
- 80%+ of compiler code is shared
- Language specification is written once
- Frontend development benefits all platforms

### 2. **Clear Separation**
- Easy to identify platform-specific code
- Platform developers can work independently
- Testing is clearly scoped

### 3. **Scalability**
- Adding new platforms is straightforward
- Each platform is self-contained
- No cross-platform contamination

### 4. **Educational Value**
- Students learn platform-agnostic Pascal
- Platform-specific features are clearly marked
- Easy to compare platforms

### 5. **Maintainability**
- Bug fixes in frontend benefit all platforms
- Platform-specific bugs are isolated
- Clear ownership boundaries

---

## Challenges & Solutions

### Challenge 1: Intrinsic Naming Conflicts
**Problem:** Different platforms may have similar intrinsics with different names.

**Solution:** Use platform-specific namespaces:
```pascal
// ZealZ80
ZVB_SpriteSet(...)

// Foenix65C816
Foenix_SpriteSet(...)

// Or use conditional compilation
{$IFDEF ZEALZ80}
  ZVB_SpriteSet(...)
{$ELSEIF DEFINED(FOENIX65C816)}
  Foenix_SpriteSet(...)
{$ENDIF}
```

### Challenge 2: ABI Differences
**Problem:** Different CPUs have different calling conventions.

**Solution:** Platform-specific ABI specs and backend implementations. The IR is platform-agnostic, but codegen is platform-specific.

### Challenge 3: Standard Library Differences
**Problem:** Some stdlib units are platform-specific.

**Solution:** Common units in `stdlib/common/`, platform-specific in `stdlib/platforms/{platform}/`.

### Challenge 4: Testing Complexity
**Problem:** Need to test on multiple platforms.

**Solution:** Platform-specific test suites in `tests/platforms/{platform}/`, plus shared compliance tests.

---

## Implementation Priority

### High Priority (MVP)
1. ✅ Restructure existing ZealZ80 code
2. ✅ Create platform directory structure
3. ✅ Separate platform-agnostic from platform-specific
4. ✅ Update documentation

### Medium Priority (v1.0)
1. Add Foenix65C816 platform
2. Implement 65C816 backend (24-bit addressing support)
3. Create Foenix intrinsics (IO Pages, graphics, audio)
4. Test cross-platform compilation

### Low Priority (Future)
1. Add FoenixA2560M platform (32-bit)
3. Cross-platform tooling
4. Platform comparison documentation

---

## Naming Considerations

### Option 1: SuperPascal ⭐ (RECOMMENDED)
- **Pros:** 
  - Clear, memorable, no ambiguity
  - Indicates "superior" or "superset" naturally
  - No confusion with other languages
  - Easy to say, easy to remember
  - Professional yet approachable
- **Cons:** None significant

### Option 2: Pascal++ (PascalPlusPlus)
- **Pros:** Clear superset relationship, modern feel
- **Cons:** "++" might confuse with C++

### Option 3: PascalPP
- **Pros:** Shorter, less ambiguous
- **Cons:** Less descriptive, still might confuse

### Option 4: ModernPascal
- **Pros:** Descriptive, clear
- **Cons:** Generic, doesn't indicate multi-platform

### Option 5: PascalCross
- **Pros:** Indicates cross-platform
- **Cons:** Less familiar, less catchy

**Recommendation:** **SuperPascal** - Clear, memorable, no ambiguity, professional. Perfect for a modern Pascal superset supporting multiple platforms.

---

## Next Steps

1. **Review this proposal** - Get feedback on architecture
2. **Create migration branch** - `feature/multi-platform-architecture`
3. **Phase 1 implementation** - Restructure without breaking
4. **Update documentation** - Reflect new structure
5. **Rename project** - ZealPascal → SuperPascal
6. **Add first new platform** - Foenix65C816

---

## Questions for Discussion

1. **Project Name:** SuperPascal vs Pascal++ vs PascalPP? ⭐ **SuperPascal recommended**
2. **Platform Naming:** Use full names (ZealZ80) or abbreviations (Z80)?
3. **Intrinsic Namespace:** Platform prefix (ZVB_*) or conditional compilation?
4. **Migration Timeline:** Big bang or incremental?
5. **Backend Priority:** Which platform after ZealZ80?

---

**Status:** Proposal - Awaiting Review  
**Author:** AI Assistant  
**Date:** 2024  
**Version:** 1.0

