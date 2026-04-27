# SuperPascal Target Architectures

## Current Status

This document lists all target architectures currently documented and planned for SuperPascal.

---

## 1. ZealZ80 ✅ (Fully Specified)

**Platform Name:** ZealZ80  
**CPU:** Zilog Z80 @ 10 MHz  
**System:** Zeal 8-bit Computer  
**Status:** ✅ **Fully Specified** (Primary Platform)

### Architecture Details
- **CPU:** Zilog Z80
- **Clock Speed:** 10 MHz
- **Data Width:** 8-bit
- **Address Width:** 16-bit (logical), banked via MMU
- **Endianness:** Little-endian
- **Memory:** 512 KB SRAM (banked via MMU)
- **Address Space:** 64 KB logical (banked to 512 KB physical)

### Hardware Features
- **Graphics:** ZVB (Zeal Video Board) with FPGA-accelerated rendering
- **Audio:** ZVB audio controller (4 voices + sample table)
- **Storage:** TF card via SPI
- **I/O:** PIO (Parallel I/O) ports
- **OS:** ZealOS

### Documentation Status
- ✅ **ABI:** `platforms/ZealZ80/ABI.md` - Complete Z80 calling conventions
- ✅ **Intrinsics:** `platforms/ZealZ80/intrinsics/` - Complete ZVB intrinsics
- ✅ **Game Engine:** `platforms/ZealZ80/gameEngine/` - ECS implementation
- ✅ **Runtime:** Specified
- ✅ **Standard Library:** Specified

### Compiler Backend
- **Status:** Specified (implementation pending)
- **Backend Crate:** `backend-zealz80`
- **Object Format:** `.ZOF` (Zeal Object Format)

### Key Specifications
- **Calling Convention:** Pascal-style (callee cleans stack)
- **Frame Pointer:** `IX` register
- **Exception Frame:** `IY` register
- **Return Values:** `HL` (16-bit), `A` (8-bit)
- **Stack Growth:** Downward (toward smaller addresses)

---

## 2. Foenix65C816 🔄 (Planned)

**Platform Name:** Foenix65C816  
**CPU:** WDC W65C816S @ 6.29 MHz  
**System:** Foenix F256x retrocomputing systems  
**Status:** 🔄 **Planned** (Structure Created, Intrinsics Planned)

### Architecture Details
- **CPU:** WDC W65C816S
- **Clock Speed:** 6.29 MHz (6,293,750 Hz, derived from 25.175 MHz / 4)
- **Data Width:** 8-bit external, 16-bit internal
- **Address Width:** 24-bit (16 MB address space)
- **Endianness:** Little-endian
- **Modes:** 
  - Emulation mode (65C02 compatible)
  - Native mode (16-bit)
- **Registers:** 16-bit in native mode (A, X, Y, SP, D, PBR, DBR)

### Hardware Features
- **Graphics:** VICKY graphics chips (TinyVicky, TinyVicky II, Vicky "The Fourth" depending on model)
- **Audio:** PSG (FPGA emulation), SID (FPGA emulation), OPL3 (FPGA emulation or real), CODEC, PWM, SAM2695, VS1053b
- **Memory:** 512KB-2MB SRAM (model-dependent), optional 256KB expansion, DDR3 on K2 (128MB)
- **I/O:** IO Page architecture, PS/2 keyboard/mouse, SD card, serial ports, joystick ports, MIDI
- **Networking:** WiFi (Wiznet or ESP32), Ethernet (Wiznet on K2)

### Supported Models
- F256Jr
- F256JrJr
- F256K
- F256K2

### Documentation Status
- 🔄 **ABI:** Not yet specified (needs 65C816 calling conventions)
- 🔄 **Intrinsics:** `platforms/Foenix65C816/intrinsics/` - Structure created, planned
  - VICKY Intrinsics (planned)
  - Direct Memory Access (planned)
  - I/O Port Access (planned)
  - Absolute Addressing (planned)
  - Interrupt Control (planned)
- ❌ **Game Engine:** Not yet specified
- ❌ **Runtime:** Not yet specified
- ❌ **Standard Library:** Not yet specified

### Compiler Backend
- **Status:** Planned (not implemented)
- **Backend Crate:** `backend-foenix816` (planned)
- **Object Format:** `.FOF` (Foenix Object Format) - planned

### Key Specifications (To Be Defined)
- **Calling Convention:** To be specified (likely register-based with stack for overflow)
- **Frame Pointer:** To be specified
- **Exception Frame:** To be specified
- **Return Values:** To be specified
- **Stack Growth:** To be specified

### References
- [WDC 65C816 Wikipedia](https://en.wikipedia.org/wiki/WDC_65C816)
- [WDC 65C816S Datasheet](https://www.westerndesigncenter.com/wdc/documentation/w65c816s.pdf)
- [Foenix Hardware Wiki](https://wiki.f256foenix.com/index.php?title=Hardware)

### SDK Location
- `../Foenix/`

---

## 3. FoenixA2560M 🔄 (Planned)

**Platform Name:** FoenixA2560M  
**CPU:** MC68LC060RC66 @ 66 MHz  
**System:** Foenix A2560M/A2560X Mini-ITX retro computer  
**Status:** 🔄 **Planned** (Not Yet Specified)

### Architecture Details
- **CPU:** MC68LC060RC66 (Motorola 68060 variant, no FPU)
- **Clock Speed:** 66 MHz
- **Data Width:** 32-bit
- **Address Width:** 32-bit (4 GB address space)
- **Endianness:** Big-endian (Motorola convention)
- **Memory:** 
  - 8 MB Static RAM
  - 4 MB Flash memory
  - 1 GB DDR3 memory

### Hardware Features
- **Graphics:** VICKY III Mark II graphics engine
- **Audio:** Multiple audio chips (OPM, OPN2, PSG, OPL3, SID, SAM2695, VS1053B)
- **Expansion:** PCI Express (Gen 2, 4 lanes)
- **Connectivity:** USB OTG (2x ports), Ethernet (Wiznet W6100), WiFi (WizFi360)

### Documentation Status
- ❌ **ABI:** Not yet specified (will reference FPC m68k backend)
- ❌ **Intrinsics:** Not yet specified
- ❌ **Game Engine:** Not yet specified
- ❌ **Runtime:** Not yet specified
- ❌ **Standard Library:** Not yet specified

### Compiler Backend
- **Status:** Planned (not implemented)
- **Backend Crate:** `backend-foenix68k` (planned)
- **Object Format:** `.AOF` (A2560M Object Format) - planned

### Key Specifications (To Be Defined)
- **Calling Convention:** To be specified (will reference FPC m68k ABI)
- **Frame Pointer:** To be specified (likely A6)
- **Exception Frame:** To be specified
- **Return Values:** To be specified (likely D0 for 32-bit)
- **Stack Growth:** To be specified (likely downward)

### Reference Implementation
- **FreePascal m68k Backend:** Will use FPC's m68k backend as semantic reference
- **Location:** `../freePascal/compiler/m68k/`

### References
- [A2560M Specifications](https://c256foenix.com/a2560m/)

---

## 4. RaspberryPi5 🔄 (Planned - Tier 2)

**Platform Name:** RaspberryPi5  
**CPU:** Broadcom BCM2712 (ARM Cortex-A76 @ 2.4 GHz)  
**System:** Raspberry Pi 5 / Raspberry Pi 500  
**Status:** 🔄 **Planned** (Tier 2 Platform)

### Architecture Details
- **CPU:** Broadcom BCM2712 SoC
  - **Cores:** 4x ARM Cortex-A76
  - **Clock Speed:** 2.4 GHz
  - **Architecture:** ARMv8-A (64-bit)
  - **Instruction Set:** AArch64
- **Data Width:** 64-bit
- **Address Width:** 48-bit virtual, 48-bit physical
- **Endianness:** Little-endian (standard)
- **Memory:** 4 GB or 8 GB LPDDR4X-4267 SDRAM
- **Address Space:** 256 TB virtual, 256 TB physical (48-bit)

### Hardware Features
- **GPU:** VideoCore VII (OpenGL ES 3.1, Vulkan 1.2)
- **Storage:** MicroSD card slot (UHS-I), optional NVMe SSD via PCIe 2.0
- **Connectivity:** Gigabit Ethernet, Wi-Fi 5 (802.11ac), Bluetooth 5.0/BLE
- **I/O:** 2x USB 3.0, 2x USB 2.0, 2x MIPI camera/display, PCIe 2.0, 40-pin GPIO
- **OS:** Raspberry Pi OS (Debian-based Linux)

### Documentation Status
- ❌ **ABI:** Not yet specified (will use ARM64 Procedure Call Standard - AAPCS64)
- ❌ **Intrinsics:** Not yet specified (GPIO, I2C, SPI, UART planned)
- ❌ **Game Engine:** Not applicable (Linux-based, different model)
- ❌ **Runtime:** Not yet specified (Linux system calls)
- ❌ **Standard Library:** Not yet specified

### Compiler Backend
- **Status:** Planned (not implemented)
- **Backend Crate:** `backend-raspberrypi5` (planned)
- **Object Format:** ELF64 (standard Linux format)
- **Target Triple:** `aarch64-unknown-linux-gnu`

### Key Specifications (To Be Defined)
- **Calling Convention:** ARM64 Procedure Call Standard (AAPCS64)
- **Frame Pointer:** X29 (FP register)
- **Exception Frame:** Linux signal handling
- **Return Values:** X0-X7 (64-bit), V0-V7 (SIMD/FP)
- **Stack Growth:** Downward (toward smaller addresses)
- **Register Usage:** 31 general-purpose (X0-X30), 32 SIMD/FP (V0-V31)

### Educational Value (Tier 2)
- **Modern Architecture:** 64-bit ARM is industry-standard
- **Real-World Applications:** Students can build actual applications
- **Rich Ecosystem:** Vast hardware and software ecosystem
- **Performance:** Enough power for advanced algorithms
- **Industry Relevance:** ARM64 used in servers, mobile, embedded

### References
- [Raspberry Pi 5 Product Page](https://www.raspberrypi.com/products/raspberry-pi-5/)
- [Raspberry Pi 5 Product Brief](https://datasheets.raspberrypi.com/rpi5/raspberry-pi-5-product-brief.pdf)
- [Raspberry Pi Documentation](https://www.raspberrypi.com/documentation/computers/raspberry-pi.html)
- [ARM Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/)
- [ARM64 Procedure Call Standard](https://github.com/ARM-software/abi-aa/blob/main/aapcs64/aapcs64.rst)

---

## 5. Intel8051 🔄 (Planned - Tier 1)

**Platform Name:** Intel8051  
**CPU:** Intel 8051 (MCS-51 family) @ variable speed  
**System:** 8051 family microcontrollers  
**Status:** 🔄 **Planned** (Tier 1 Platform)

### Architecture Details
- **CPU:** Intel 8051 (MCS-51 family)
- **Clock Speed:** Variable (12 MHz typical, up to 100+ MHz in enhanced cores)
- **Data Width:** 8-bit
- **Address Width:** 16-bit (64 KB each for code and data)
- **Endianness:** Little-endian
- **Architecture:** Harvard architecture (separate code and data memory spaces)
- **Memory Model:**
  - **CODE Memory:** 4 KB on-chip ROM (up to 64 KB external)
  - **DATA Memory:** 128 bytes on-chip RAM (16 bytes bit-addressable)
  - **IDATA Memory:** 128 bytes internal data (overlaps DATA)
  - **XDATA Memory:** Up to 64 KB external RAM
  - **SFR:** Special Function Registers (128 bytes)

### Hardware Features
- **I/O Ports:** Four 8-bit bi-directional ports (P0, P1, P2, P3)
- **UART:** Serial communication port
- **Timers:** Two 16-bit counter/timers
- **Interrupts:** Two-level interrupt priority system
- **Power Management:** Power saving modes
- **Modern Variants:** Enhanced cores with:
  - Single-cycle execution (vs. 12-cycle original)
  - Up to 150 MHz clock speeds
  - Flash ROM, EEPROM
  - I²C, SPI, USB interfaces
  - PWM generators
  - A/D and D/A converters
  - RTC (Real-Time Clock)

### Register Architecture
- **Register Banks:** 4 banks of 8 registers each (R0-R7)
  - Bank 0: R0-R7 (default)
  - Bank 1: R0-R7
  - Bank 2: R0-R7
  - Bank 3: R0-R7
- **Accumulator:** A (8-bit)
- **B Register:** B (8-bit, used for MUL/DIV)
- **Data Pointer:** DPTR (16-bit, DPH + DPL)
- **Program Counter:** PC (16-bit)
- **Stack Pointer:** SP (8-bit, points to internal RAM)
- **Special Function Registers:** 128 bytes (ports, timers, UART, etc.)

### Documentation Status
- ✅ **Reference:** Turbo51 compiler documentation (scraped)
- 🔄 **ABI:** To be specified (reference Turbo51 calling conventions)
- 🔄 **Intrinsics:** To be specified (I/O ports, timers, UART, interrupts)
- ❌ **Game Engine:** Not applicable (embedded microcontroller)
- 🔄 **Runtime:** To be specified (minimal runtime for embedded)
- 🔄 **Standard Library:** To be specified (embedded-focused)

### Compiler Backend
- **Status:** Planned (not implemented)
- **Backend Crate:** `backend-8051` (planned)
- **Object Format:** `.OMF51` (OMF-51 format, compatible with Turbo51) or `.HEX` (Intel HEX)
- **Reference Implementation:** Turbo51 compiler

### Key Specifications (Based on Turbo51)
- **Calling Convention:** 
  - Parameters stored as local variables (static allocation)
  - Return values in ACC (8-bit) or register set R5R4R3R2 (16-bit/32-bit)
  - String functions return pointer in R0 (DATA/IDATA) or DPTR (CODE/XDATA)
- **Register Usage:** 
  - Turbo51 uses two register sets: R5R4R3R2 and R9R8R7R6
  - R8 and R9 are DATA variables
  - 8-bit data in R2 (R6)
  - 16-bit data in R3R2 (R7R6)
  - 32-bit data uses whole register set
- **Stack:** XDATA stack for reentrant procedures (XSP pointer)
- **Memory Types:**
  - DATA: Internal RAM (128 bytes, default)
  - IDATA: Internal data (128 bytes, overlaps DATA)
  - XDATA: External RAM (up to 64 KB)
  - CODE: Program memory (up to 64 KB)
- **Boolean Variables:** Stored as bits in bit-addressable DATA memory
- **Data Storage:** Little-endian format

### Educational Value (Tier 1)
- **Classic 8-bit Architecture:** 8051 is one of the most popular microcontrollers
- **Harvard Architecture:** Separate code/data spaces (educational value)
- **Register Banks:** Advanced register management concepts
- **Embedded Systems:** Real-world embedded programming
- **Widely Used:** Still in production, vast ecosystem
- **Proven Pascal Support:** Turbo51 demonstrates Pascal viability

### Reference Implementation
- **Turbo51 Compiler:** Free Pascal compiler for 8051
- **Location:** `docs/turbo51_scraped/` - Complete Turbo51 documentation
- **Key References:**
  - Compiler internals
  - Memory organization
  - Register usage
  - Calling conventions
  - Optimization techniques

### References
- [Turbo51 Homepage](https://turbo51.com/)
- [Turbo51 Documentation](https://turbo51.com/documentation/)
- [8051 Microcontroller Wikipedia](https://en.wikipedia.org/wiki/Intel_8051)
- [MCS-51 Architecture](https://en.wikipedia.org/wiki/Intel_MCS-51)
- **Local Documentation:** `docs/turbo51_scraped/` - Complete Turbo51 documentation

---

## 6. CommanderX16 🔄 (Planned - Tier 1)

**Platform Name:** CommanderX16  
**CPU:** WDC 65C02 @ 8 MHz  
**System:** Commander X16 Retro Computer  
**Status:** 🔄 **Planned** (Tier 1 Platform)

### Architecture Details
- **CPU:** WDC 65C02S (enhanced 6502)
- **Clock Speed:** 8 MHz
- **Data Width:** 8-bit
- **Address Width:** 16-bit (logical), banked to 512 KB-2 MB physical
- **Endianness:** Little-endian
- **Memory:** 512 KB RAM (standard) or 2 MB RAM (Developer Edition), 512 KB ROM
- **Address Space:** 64 KB logical (banked to 512 KB-2 MB physical, up to 3.5 MB via cartridges)

### Hardware Features
- **Graphics:** VERA (Versatile Embedded Retro Adapter, Lattice ICE40UP5K FPGA)
  - Fixed resolution: 640x480 @ 60 Hz
  - 256 colors from 4096-color palette
  - 128 hardware sprites, up to 64x64 pixels each
  - 2 layers (bitmap or tilemap modes) with hardware scrolling
  - 128 KB embedded VRAM
  - Output: VGA, NTSC Composite, NTSC S-Video, RGB
- **Audio:** 
  - YM2151 (OPM): 8 channels, 4-operator FM synthesis
  - VERA PSG: 16 channels, 4 waveforms (Pulse, Sawtooth, Triangle, Noise)
  - VERA PCM: Up to 48 kHz, 16-bit, stereo (4 KB FIFO buffer)
- **Storage:** SD card slot (FAT32 via CMDR-DOS), IEC serial bus (Fast IEC compatible)
- **I/O:** PS/2 keyboard/mouse, 4x SNES controller ports, IEC serial port, user port, expansion slots
- **OS:** KERNAL (Commodore-style), X16 BASIC (enhanced Commodore BASIC)

### Documentation Status
- ❌ **ABI:** Not yet specified (will use 6502-style stack-based calling)
- ❌ **Intrinsics:** Not yet specified (VERA, YM2151, memory banking planned)
- ❌ **Game Engine:** Not yet specified
- ❌ **Runtime:** Not yet specified (KERNAL integration)
- ❌ **Standard Library:** Not yet specified

### Compiler Backend
- **Status:** Planned (not implemented)
- **Backend Crate:** `backend-commanderx16` (planned)
- **Object Format:** `.X16` (Commander X16 Object Format) - planned

### Key Specifications (To Be Defined)
- **Calling Convention:** 6502-style stack-based (to be specified)
- **Frame Pointer:** Stack-based (6502 has limited registers)
- **Exception Frame:** To be specified
- **Return Values:** Accumulator A (8-bit), A+X (16-bit)
- **Stack Growth:** Downward (toward smaller addresses, $0100-$01FF)
- **Register Usage:** A (accumulator), X, Y (index), SP (stack pointer)

### Educational Value (Tier 1)
- **Classic 8-bit Architecture:** 65C02 is a classic, well-understood CPU
- **6502 Compatibility:** Shares heritage with Commodore 64, Apple II, NES
- **Modern Features:** VERA graphics chip provides modern capabilities
- **Active Community:** Active development and community support
- **Retro Computing:** Perfect for learning retro computing concepts

### References
- [Commander X16 Official Website](https://www.commanderx16.com/)
- [Commander X16 Documentation](https://github.com/commanderx16/x16-docs)
- **Local Documentation:** `../CommandX16/x16-docs/`
  - Complete Programmer's Reference Guide
  - Memory Map, VERA, Sound, I/O, Hardware specifications
- [VERA Original Documentation](https://github.com/fvdhoef/vera-module/blob/rev4/doc/VERA%20Programmer's%20Reference.md)
- [WDC 65C02 Datasheet](https://www.westerndesigncenter.com/wdc/documentation/w65c02s.pdf)

---

## Architecture Comparison

| Feature | ZealZ80 | CommanderX16 | Intel8051 | Foenix65C816 | FoenixA2560M | RaspberryPi5 |
|---------|---------|--------------|-----------|--------------|--------------|--------------|
| **CPU** | Z80 @ 10 MHz | 65C02 @ 8 MHz | 8051 @ variable | W65C816S @ 6.29 MHz | MC68LC060 @ 66 MHz | ARM Cortex-A76 @ 2.4 GHz |
| **Data Width** | 8-bit | 8-bit | 8-bit | 8-bit ext, 16-bit int | 32-bit | 64-bit |
| **Address Width** | 16-bit (banked) | 16-bit (banked) | 16-bit (Harvard) | 24-bit | 32-bit | 48-bit |
| **Address Space** | 64 KB logical (512 KB physical) | 64 KB logical (512 KB physical) | 64 KB code + 64 KB data | 16 MB | 4 GB | 256 TB |
| **Endianness** | Little-endian | Little-endian | Little-endian | Little-endian | Big-endian | Little-endian |
| **Architecture** | Von Neumann | Von Neumann | Harvard | Von Neumann | Von Neumann | Von Neumann |
| **Tier** | Tier 1 | Tier 1 | Tier 1 | Tier 1/2 | Tier 2 | Tier 2 |
| **Status** | ✅ Fully Specified | 🔄 Planned | 🔄 Planned | 🔄 Planned | 🔄 Planned | 🔄 Planned |
| **ABI** | ✅ Complete | ❌ Not specified | 🔄 Reference Turbo51 | ❌ Not specified | ❌ Not specified | ❌ Not specified |
| **Intrinsics** | ✅ Complete | ❌ Not specified | 🔄 Planned | 🔄 Planned | ❌ Not specified | ❌ Not specified |
| **Game Engine** | ✅ Complete | ❌ Not specified | N/A (embedded) | ❌ Not specified | ❌ Not specified | N/A (Linux) |
| **Backend** | Specified | Planned | Planned | Planned | Planned | Planned |

---

## Implementation Priority

### Phase 1: ZealZ80 (Primary Platform)
- ✅ Complete specification
- 🔄 Compiler implementation (in progress)
- ✅ ABI defined
- ✅ Intrinsics defined
- ✅ Game engine specified

### Phase 2: Foenix65C816
- 🔄 ABI specification needed
- 🔄 Intrinsics specification (structure created)
- 🔄 Backend implementation
- 🔄 Runtime specification
- 🔄 Standard library units

### Phase 3: FoenixA2560M
- 🔄 ABI specification (reference FPC m68k)
- 🔄 Intrinsics specification
- 🔄 Backend implementation
- 🔄 Runtime specification
- 🔄 Standard library units

### Phase 4: Intel8051 (Tier 1)
- 🔄 ABI specification (reference Turbo51 calling conventions)
- 🔄 Intrinsics specification (I/O ports, timers, UART, interrupts)
- 🔄 Backend implementation
- 🔄 Runtime specification (minimal embedded runtime)
- 🔄 Standard library units (embedded-focused)

### Phase 5: CommanderX16 (Tier 1)
- 🔄 ABI specification (6502-style stack-based)
- 🔄 Intrinsics specification (VERA, YM2151, memory banking)
- 🔄 Backend implementation
- 🔄 Runtime specification (KERNAL integration)
- 🔄 Standard library units

### Phase 6: RaspberryPi5 (Tier 2)
- 🔄 ABI specification (ARM64 AAPCS64)
- 🔄 Intrinsics specification (GPIO, I2C, SPI, UART)
- 🔄 Backend implementation
- 🔄 Runtime specification (Linux system calls)
- 🔄 Standard library units

---

## Architecture-Specific Notes

### ZealZ80 (8-bit)
- **Primary platform** for SuperPascal
- **Educational focus:** 8-bit computing, simple architecture
- **Memory model:** Banked MMU extends 16-bit address space to 512 KB
- **Optimizations:** Z80-specific peephole optimizations

### CommanderX16 (8-bit)
- **Classic 6502 architecture** - shares heritage with Commodore 64, Apple II, NES
- **VERA graphics chip** - modern graphics capabilities on classic CPU
- **Memory model:** Banked memory controller extends 16-bit address space to 512 KB
- **Educational focus:** 6502/65C02 programming, retro computing

### Intel8051 (8-bit)
- **Harvard architecture** - separate code and data memory spaces (educational value)
- **Register banks** - 4 banks of 8 registers (R0-R7) for advanced register management
- **Memory model:** Multiple memory types (DATA, IDATA, XDATA, CODE)
- **Embedded systems** - real-world microcontroller programming
- **Proven Pascal support** - Turbo51 demonstrates full Pascal compiler for 8051
- **Reference:** Complete Turbo51 documentation available in `docs/turbo51_scraped/`

### Foenix65C816 (16-bit)
- **Extended memory:** 24-bit addressing enables 16 MB address space
- **Compatibility:** Emulation mode for 65C02 compatibility
- **Native mode:** 16-bit operations for performance
- **Note:** Only WDC65C816 supported, not 6502

### FoenixA2560M (32-bit)
- **High performance:** 66 MHz 68060 with 1 GB DDR3
- **Reference:** FreePascal m68k backend as semantic oracle
- **Big-endian:** Motorola convention (different from Z80/65C816)
- **Modern features:** PCIe, USB, Ethernet, WiFi

### RaspberryPi5 (64-bit ARM - Tier 2)
- **Modern architecture:** 64-bit ARM Cortex-A76, industry-standard
- **High performance:** 2.4 GHz quad-core, up to 8 GB RAM
- **Real-world platform:** Linux-based, vast ecosystem
- **Educational value:** Modern systems programming, advanced algorithms
- **Reference:** ARM64 Procedure Call Standard (AAPCS64), Linux ABI

---

## Next Steps

1. **Lock down ZealZ80** (already done ✅)
2. **Specify Intel8051 ABI** - Reference Turbo51 calling conventions from scraped docs
3. **Specify Foenix65C816 ABI** - Define calling conventions for 65C816
4. **Specify FoenixA2560M ABI** - Extract from FPC m68k backend
5. **Implement backends** - Start with ZealZ80, then 8051 (Turbo51 reference), then 65C816, then 68k
6. **Create platform-specific runtimes** - One per platform
7. **Implement platform-specific stdlib units** - Graphics, audio, I/O (embedded for 8051)

---

**Version:** 1.0  
**Date:** 2024  
**Status:** Architecture Inventory

