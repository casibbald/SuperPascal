# Intel 8051 Target Analysis

**Status:** 🔄 Planned (Tier 1 Platform)  
**Reference:** [Turbo51 Compiler](https://turbo51.com/) - Complete Pascal compiler for 8051

## Why 8051?

### Educational Value
1. **Classic 8-bit Architecture** - One of the most popular microcontrollers ever
2. **Harvard Architecture** - Separate code/data memory (educational distinction)
3. **Register Banks** - Advanced register management concepts
4. **Embedded Systems** - Real-world embedded programming
5. **Proven Pascal Support** - Turbo51 demonstrates full Pascal compiler

### Technical Advantages
1. **Complete Reference** - Turbo51 provides full compiler implementation reference
2. **Extensive Documentation** - All Turbo51 docs scraped and available
3. **Optimization Examples** - 13 optimization techniques documented
4. **Memory Model** - Multiple memory types (DATA, IDATA, XDATA, CODE)
5. **Still in Production** - Vast ecosystem, modern variants available

## Architecture Overview

### CPU Characteristics
- **8-bit data bus** - 8 bits per operation
- **16-bit address bus** - 64 KB each for code and data
- **Harvard architecture** - Separate code and data memory spaces
- **Little-endian** - Data stored in little-endian format

### Memory Organization

#### CODE Memory (Program Memory)
- **On-chip:** 4 KB ROM (original 8051)
- **External:** Up to 64 KB
- **Purpose:** Stores program code
- **Access:** Via 16-bit address bus

#### DATA Memory (Internal RAM)
- **Size:** 128 bytes (original 8051)
- **Bit-addressable:** 16 bytes (128 bits)
- **Purpose:** Fast access variables, stack
- **Register banks:** R0-R7 (4 banks = 32 bytes)

#### IDATA Memory (Internal Data)
- **Size:** 128 bytes
- **Overlaps:** DATA memory space
- **Purpose:** Additional internal storage

#### XDATA Memory (External RAM)
- **Size:** Up to 64 KB
- **Purpose:** Large data structures, stack for reentrant procedures
- **Access:** Via DPTR (Data Pointer)

#### Special Function Registers (SFR)
- **Size:** 128 bytes
- **Purpose:** I/O ports, timers, UART, interrupt control
- **Location:** Upper 128 bytes of internal RAM (in some variants)

## Register Architecture

### General Purpose Registers
- **R0-R7:** 8 registers per bank, 4 banks total (32 bytes)
- **Bank Selection:** Via PSW (Program Status Word) bits
- **Usage:** Turbo51 uses R5R4R3R2 and R9R8R7R6 register sets

### Special Registers
- **A (Accumulator):** 8-bit, primary arithmetic register
- **B:** 8-bit, used for MUL/DIV operations
- **DPTR (Data Pointer):** 16-bit (DPH + DPL), points to external memory
- **SP (Stack Pointer):** 8-bit, points to internal RAM stack
- **PC (Program Counter):** 16-bit, current instruction address
- **PSW (Program Status Word):** Flags and register bank selection

## Turbo51 Calling Conventions

### Parameter Passing
- **Storage:** Parameters stored as local variables (static allocation)
- **Memory Type:** Default DATA, can be overridden with $MP directive
- **Passing:** Caller stores data to parameter memory locations
- **No Stack:** Normal procedures don't use stack (simplifies code)

### Return Values
- **8-bit:** Returned in ACC (Accumulator)
- **16-bit:** Returned in R5R4R3R2 register set
- **32-bit:** Returned in R5R4R3R2 register set (full set)
- **Strings:** 
  - DATA/IDATA: Pointer in R0
  - CODE/XDATA: Pointer in DPTR

### Register Usage (Turbo51)
- **Register Set 1:** R5R4R3R2 (primary)
- **Register Set 2:** R9R8R7R6 (secondary)
- **R8, R9:** Declared as DATA variables in System unit
- **8-bit data:** R2 (or R6)
- **16-bit data:** R3R2 (or R7R6)
- **32-bit data:** Full register set

### Reentrant Procedures
- **Stack:** XDATA stack (when XDATA memory available)
- **Pointer:** XSP (declared in System unit)
- **Note:** Avoid reentrant procedures unless necessary (simplifies code)

## Memory Types

### DATA (Default)
- **Location:** Internal RAM (128 bytes)
- **Access:** Fastest
- **Use:** Global variables, local variables, parameters
- **Directive:** `$MG` for globals, `$ML` for locals, `$MP` for parameters

### IDATA
- **Location:** Internal data (128 bytes, overlaps DATA)
- **Access:** Fast
- **Use:** Additional internal storage

### XDATA
- **Location:** External RAM (up to 64 KB)
- **Access:** Slower (via DPTR)
- **Use:** Large data structures, reentrant stack
- **Stack:** XSP points to XDATA stack

### CODE
- **Location:** Program memory (up to 64 KB)
- **Access:** Read-only
- **Use:** Constants, string literals

## Boolean Variables

### Bit Storage
- **Location:** Bit-addressable DATA memory
- **Size:** 1 bit
- **Limitations:**
  - Cannot be passed by reference (no bit address instruction)
  - Cannot be passed as parameter in reentrant procedures
- **Alternative:** Use `ByteBool` (1 byte) when needed

## Optimization Opportunities

### Based on Turbo51
1. **Register Allocation** - Efficient use of register banks
2. **Memory Type Selection** - Choose optimal memory type
3. **Static Allocation** - Avoid stack for normal procedures
4. **Dead Code Elimination** - Critical for code size
5. **Constant Folding** - Evaluate at compile time
6. **Peephole Optimization** - 8051-specific instruction patterns

## Implementation Strategy

### Phase 1: ABI Specification
- [ ] Define calling conventions (reference Turbo51)
- [ ] Specify register usage
- [ ] Define memory type directives
- [ ] Document parameter passing
- [ ] Document return value conventions

### Phase 2: Backend Implementation
- [ ] Instruction selection (8051 instruction set)
- [ ] Register allocation (register banks)
- [ ] Memory type handling (DATA, IDATA, XDATA, CODE)
- [ ] Code generation
- [ ] Object file format (OMF-51 or HEX)

### Phase 3: Runtime
- [ ] Minimal runtime for embedded
- [ ] System unit (register declarations, stack pointers)
- [ ] Interrupt handling
- [ ] I/O port access

### Phase 4: Standard Library
- [ ] I/O port procedures
- [ ] Timer procedures
- [ ] UART procedures
- [ ] Interrupt procedures
- [ ] Delay procedures

## Turbo51 Reference Material

### Available Documentation
All Turbo51 documentation is available in `docs/turbo51_scraped/`:

- **Compiler Internals:** `documentation_8051-compiler-internals-general.md`
- **Memory Organization:** `documentation_8051-memory-organization.md`
- **Syntax:** `documentation_8051-compiler-syntax.md`
- **System Unit:** `documentation_8051-pascal-system-unit.md`
- **System Procedures:** `documentation_8051-pascal-system-procedures.md`
- **System Functions:** `documentation_8051-pascal-system-functions.md`
- **Assembler Integration:** `documentation_8051-assembler-procedures.md`
- **Inline Procedures:** `documentation_8051-pascal-inline-procedures.md`
- **Interrupts:** `documentation_8051-interrupts.md`
- **Instruction Set:** `documentation_8051-instruction-set.md`
- **Opcodes:** `documentation_8051-opcodes.md`

### Key Learnings
1. **Single-pass compiler** - Fast compilation
2. **Multi-pass optimizer** - 13 optimization techniques
3. **Register bank usage** - Advanced register management
4. **Memory type directives** - Fine-grained memory control
5. **Static allocation** - Avoid stack for performance
6. **Compact code** - Critical for embedded systems

## Comparison with Other 8-bit Targets

### vs. Z80 (ZealZ80)
- **Similarities:** 8-bit, little-endian, limited registers
- **Differences:** Harvard vs. Von Neumann, register banks vs. single register set
- **Advantages:** Register banks provide more register space

### vs. 65C02 (CommanderX16)
- **Similarities:** 8-bit, little-endian, embedded focus
- **Differences:** Harvard vs. Von Neumann, register banks vs. A/X/Y
- **Advantages:** More registers available via banks

## Educational Benefits

1. **Harvard Architecture** - Learn about separate code/data memory
2. **Register Banks** - Advanced register management
3. **Memory Types** - Multiple memory spaces with different characteristics
4. **Embedded Programming** - Real-world microcontroller programming
5. **Optimization** - Code size and performance critical
6. **Proven Path** - Turbo51 shows it's possible

## References

- [Turbo51 Homepage](https://turbo51.com/)
- [Turbo51 Documentation](https://turbo51.com/documentation/)
- [8051 Microcontroller Wikipedia](https://en.wikipedia.org/wiki/Intel_8051)
- [MCS-51 Architecture](https://en.wikipedia.org/wiki/Intel_MCS-51)
- **Local Documentation:** `docs/turbo51_scraped/` - Complete Turbo51 documentation
- **Turbo51 Analysis:** `docs/TURBO51_ANALYSIS.md` - Detailed Turbo51 analysis
- **Optimization Roadmap:** `docs/OPTIMIZATION_ROADMAP.md` - Optimization strategies

---

**Status:** Ready for ABI specification and backend planning  
**Priority:** Tier 1 (Educational 8-bit platform)  
**Reference Implementation:** Turbo51 compiler

