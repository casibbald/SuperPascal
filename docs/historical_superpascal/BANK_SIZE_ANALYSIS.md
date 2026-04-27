# Memory Bank Size Analysis for PascalOS

**Date:** December 2024  
**Purpose:** Determine optimal bank size and maximum number of banks for 65C816-based PascalOS

---

## 1. Architecture Overview

### 65C816 Memory Architecture

The WDC 65C816 processor has:
- **24-bit address bus** → 16MB total addressable memory
- **16-bit registers** → 64KB visible in direct address space at any time
- **8-bit bank number** → 256 possible banks (0x00 to 0xFF)
- **Bank switching** → Access different 64KB windows via bank registers

**Memory Layout:**
```
Total Address Space: 16MB (24-bit)
├── Bank 0x00 [0x000000-0x00FFFF]: 64KB
├── Bank 0x01 [0x010000-0x01FFFF]: 64KB
├── Bank 0x02 [0x020000-0x02FFFF]: 64KB
├── ...
├── Bank 0xFE [0xFE0000-0xFEFFFF]: 64KB
└── Bank 0xFF [0xFF0000-0xFFFFFF]: 64KB

Total: 256 banks × 64KB = 16MB
```

---

## 2. Optimal Bank Size: 64KB

### Why 64KB is Optimal

**✅ Natural Hardware Alignment:**
- 65C816's 16-bit registers naturally address 64KB (2^16 bytes)
- Bank number is 8-bit (0-255), perfectly mapping to 64KB chunks
- No fragmentation or alignment issues
- Direct mapping: Bank N = addresses [N×64KB, (N+1)×64KB-1]

**✅ Efficient Bank Switching:**
- Single 8-bit bank register change switches entire 64KB window
- No partial bank management needed
- Simple address calculation: `physical = (bank << 16) | offset`
- Hardware-friendly (matches CPU architecture)

**✅ Memory Management Simplicity:**
- One allocation unit = one bank (or multiple banks)
- No sub-bank fragmentation to manage
- Simple free list: one entry per bank
- Predictable allocation patterns

**✅ Educational Clarity:**
- Easy to understand: "Bank 5 = 64KB starting at 0x050000"
- Students can visualize memory layout
- Clear boundaries between processes
- Matches historical 8/16-bit systems

### Is 64KB the Default?

**Yes, 64KB is the default and only bank size.**

The 65C816 architecture **dictates** 64KB banks:
- The 24-bit address space is **naturally divided** into 256 banks of 64KB
- There is no hardware support for different bank sizes
- Bank switching hardware expects 64KB boundaries
- Changing this would require custom hardware and break compatibility

**Conclusion:** 64KB is not just optimal—it's **architecturally required**.

---

## 3. Maximum Number of Banks

### Total Addressable Banks: 256

The 65C816 can address **256 banks** (0x00 to 0xFF):
```
Total Banks = 2^8 = 256 banks
Total Memory = 256 × 64KB = 16MB
```

### Available Banks for User Processes

Not all 256 banks are available for user allocation. Reserved banks:

**Reserved Banks:**
- **Bank 0x00:** PascalOS Kernel (OS code, data, I/O buffers, process tables)
- **Bank 0xFF:** System ROM / BIOS (typically)
- **Banks 0xFE-0xFF:** May be reserved for ROM/BIOS depending on hardware

**OS Subprocesses:**
The OS itself runs multiple subprocesses (system services, drivers, etc.):
- **OS Subprocesses:** 5-10 subprocesses typically
- **Allocation per OS subprocess:** 1-4 banks each
- **Total OS subprocess banks:** ~15-30 banks (conservative estimate)

**Available for User Processes:**
```
Total Banks:          256
Reserved (Kernel):       1  (Bank 0x00)
Reserved (ROM):          1  (Bank 0xFF, possibly 0xFE too)
OS Subprocesses:       15-30 banks (5-10 processes × 1-4 banks)
─────────────────────────────────────
Available for Users:  224-239 banks
```

**Conservative Estimate:**
- **224 banks available** for user processes (worst case: 30 OS subprocess banks)
- **224 × 64KB = 14MB** available for user allocation
- **Realistic:** ~230 banks available (~14.375MB)

### Maximum Number of Processes

**OS Subprocesses:**
- **5-10 OS subprocesses** (system services, drivers, background tasks)
- **1-4 banks per OS subprocess** (typical allocation)
- **Total OS subprocess banks:** 15-30 banks

**User Processes:**
- **Theoretical Maximum (1 bank each):**
  - **224 processes maximum** (one bank each, worst case)
  - **239 processes maximum** (one bank each, best case)
  
- **Practical Maximum (1-4 banks each):**
  - Typical user process: **1-4 banks** (code + stack + heap)
  - Realistic maximum: **40-80 user processes** (depending on allocation)
  - Educational use: **10-20 user processes** (more than sufficient)

**Primary Running App:**
- **Foreground/primary app** may receive **5-8 banks** for enhanced performance
- This is an exception to typical 1-4 bank allocation
- Examples: Large games, graphics applications, development tools
- When primary app terminates, banks return to free pool

---

## 4. Bank Allocation Scenarios

### Scenario 1: Minimal Process (1 Bank)

**Allocation:**
```
Bank N [64KB total]:
├── Code:     16KB (0x0000-0x3FFF)
├── Stack:    8KB  (0x4000-0x5FFF, grows down)
├── Heap:     40KB (0x6000-0xFFFF, grows up)
└── Reserved: 0KB
```

**Maximum User Processes:** 224 processes (one bank each, worst case)

### Scenario 2: Typical Process (2 Banks)

**Allocation:**
```
Bank N [64KB]:
├── Code:     32KB
├── Stack:    16KB
└── Heap:     16KB

Bank N+1 [64KB]:
└── Heap Extension: 64KB
```

**Maximum User Processes:** 112 processes (two banks each, worst case)

### Scenario 3: Large Process (4 Banks)

**Allocation:**
```
Bank N:     Code + Stack
Bank N+1:   Heap
Bank N+2:   Large Data Structures
Bank N+3:   Additional Heap
```

**Maximum User Processes:** 56 processes (four banks each, worst case)

### Scenario 4: OS Subprocess (1-4 Banks)

**OS Subprocess Examples:**
- **File System Service:** 2 banks (code + buffers)
- **Network Driver:** 1 bank (minimal, if present)
- **Graphics Driver:** 2-3 banks (framebuffer, code)
- **Audio Service:** 1-2 banks (code + buffers)
- **Scheduler Service:** 1 bank (minimal)
- **Memory Manager:** 1 bank (minimal)
- **Device Manager:** 1-2 banks (code + device tables)

**Total OS Subprocesses:** 5-10 processes, 15-30 banks total

### Scenario 5: Primary Running App (5-8 Banks)

**Primary App Allocation:**
```
Bank N:     Code (main program)
Bank N+1:   Code (libraries/modules)
Bank N+2:   Stack + Initial Heap
Bank N+3:   Heap Extension
Bank N+4:   Large Data Structures
Bank N+5:   Graphics Buffers (if needed)
Bank N+6:   Audio Buffers (if needed)
Bank N+7:   Additional Heap (if needed)
```

**Use Cases:**
- Large games with graphics/audio
- Development tools (compiler, debugger)
- Graphics applications
- Complex simulations

**Note:** When primary app terminates, all banks return to free pool

---

## 5. Memory Layout Example

### Complete System Memory Map

```
Bank 0x00 [0x000000-0x00FFFF]: PascalOS Kernel
├── Kernel Code:       32KB
├── Kernel Data:       16KB
├── I/O Buffers:       8KB
└── Process Tables:    8KB

─────────────────────────────────────────────────────────
OS Subprocesses (System Services)
─────────────────────────────────────────────────────────

Bank 0x01 [0x010000-0x01FFFF]: File System Service
├── Code:  32KB
├── Buffers: 32KB

Bank 0x02 [0x020000-0x02FFFF]: Graphics Driver
├── Code:  24KB
└── Framebuffer: 40KB

Bank 0x03 [0x030000-0x03FFFF]: Audio Service
├── Code:  16KB
└── Buffers: 48KB

Bank 0x04 [0x040000-0x04FFFF]: Network Driver (if present)
├── Code:  32KB
└── Buffers: 32KB

Bank 0x05 [0x050000-0x05FFFF]: Scheduler Service
└── Code + Data: 64KB

Bank 0x06 [0x060000-0x06FFFF]: Memory Manager
└── Code + Tables: 64KB

Bank 0x07 [0x070000-0x07FFFF]: Device Manager
├── Code:  32KB
└── Device Tables: 32KB

─────────────────────────────────────────────────────────
Primary Running App (Foreground Application)
─────────────────────────────────────────────────────────

Bank 0x08 [0x080000-0x08FFFF]: Primary App - Code
└── Main Program: 64KB

Bank 0x09 [0x090000-0x09FFFF]: Primary App - Libraries
└── Libraries/Modules: 64KB

Bank 0x0A [0x0A0000-0x0AFFFF]: Primary App - Stack/Heap
├── Stack: 16KB
└── Heap: 48KB

Bank 0x0B [0x0B0000-0x0BFFFF]: Primary App - Heap Extension
└── Heap: 64KB

Bank 0x0C [0x0C0000-0x0CFFFF]: Primary App - Data Structures
└── Large Arrays/Records: 64KB

Bank 0x0D [0x0D0000-0x0DFFFF]: Primary App - Graphics Buffers
└── Framebuffer/Sprites: 64KB

─────────────────────────────────────────────────────────
User Processes (Background Applications)
─────────────────────────────────────────────────────────

Bank 0x0E [0x0E0000-0x0EFFFF]: User Process 1
├── Code:  16KB
├── Stack: 8KB
└── Heap:  40KB

Bank 0x0F [0x0F0000-0x0FFFFF]: User Process 1 (Extension)
└── Heap: 64KB

Bank 0x10 [0x100000-0x10FFFF]: User Process 2
├── Code:  20KB
├── Stack: 8KB
└── Heap:  36KB

Bank 0x11 [0x110000-0x11FFFF]: User Process 3
...

Bank 0xFD [0xFD0000-0xFDFFFF]: User Process N
Bank 0xFE [0xFE0000-0xFEFFFF]: Reserved/Unused
Bank 0xFF [0xFF0000-0xFFFFFF]: System ROM/BIOS
```

**Memory Allocation Summary:**
- **Bank 0x00:** Kernel (1 bank)
- **Banks 0x01-0x07:** OS Subprocesses (7 banks, ~15-30 typical)
- **Banks 0x08-0x0D:** Primary Running App (6 banks, 5-8 typical)
- **Banks 0x0E-0xFC:** User Processes (~239 banks available)
- **Banks 0xFD-0xFF:** Reserved/ROM (3 banks)

---

## 6. Bank Size Alternatives (Not Recommended)

### Why Smaller Banks (e.g., 16KB) Don't Work

**❌ Hardware Incompatibility:**
- 65C816 bank registers expect 64KB boundaries
- Would require custom hardware modification
- Breaks standard 65C816 addressing model

**❌ Increased Complexity:**
- More banks to manage (1024 banks for 16MB)
- More complex bank switching logic
- More fragmentation
- More overhead in capability structures

**❌ No Benefit:**
- Doesn't improve memory utilization
- Doesn't improve isolation (already per-bank)
- Adds unnecessary complexity

### Why Larger Banks (e.g., 128KB) Don't Work

**❌ Hardware Limitation:**
- 65C816 can only address 64KB at a time
- Larger banks would require multiple bank register changes
- Defeats purpose of bank switching
- Would need custom MMU (defeats educational purpose)

**❌ Wasted Address Space:**
- 24-bit address space = 16MB max
- Can't have banks larger than addressable space
- 128KB banks would only give 128 banks (worse utilization)

---

## 7. Optimal Configuration Recommendations

### Recommended Bank Configuration

**Bank Size:** 64KB (fixed by architecture)  
**Total Banks:** 256 (0x00 to 0xFF)  
**Reserved Banks:** 2 (Kernel + ROM)  
**OS Subprocess Banks:** 15-30 banks (5-10 subprocesses × 1-4 banks)  
**Available for Users:** 224-239 banks (conservative: 224)  
**Maximum User Processes:** 224 (if 1 bank each, worst case)  
**Practical Maximum:** 40-80 user processes (typical 1-4 bank allocation)  
**Primary App:** 5-8 banks (exception case, returns to pool on termination)

### Kernel Bank Table Design

**Bank Table Structure:**
```pascal
type
  BankState = (bsFree, bsAllocated, bsReserved);
  
  BankTableEntry = record
    State: BankState;
    OwnerPID: Word;        // Process that owns this bank
    Size: Word;            // Always 65536 (64KB) for simplicity
    AllocTime: DWord;      // For debugging/tracking
  end;
  
var
  BankTable: array[0..255] of BankTableEntry;  // One entry per bank
  FreeBankList: array[0..255] of Byte;         // Free bank indices
  FreeBankCount: Byte;                          // Number of free banks
```

**Initialization:**
```pascal
procedure InitializeBankTable;
var
  i: Byte;
begin
  // Reserve Bank 0x00 for kernel
  BankTable[0].State := bsReserved;
  BankTable[0].OwnerPID := 0;  // Kernel PID
  
  // Reserve Bank 0xFF for ROM
  BankTable[255].State := bsReserved;
  BankTable[255].OwnerPID := 0;
  
  // Mark all other banks as free initially
  // (OS subprocesses will allocate their banks during OS startup)
  FreeBankCount := 0;
  for i := 1 to 254 do
  begin
    BankTable[i].State := bsFree;
    BankTable[i].OwnerPID := 0;
    FreeBankList[FreeBankCount] := i;
    FreeBankCount := FreeBankCount + 1;
  end;
end;
```

**OS Startup Sequence:**
```pascal
procedure StartOSSubprocesses;
begin
  // Allocate banks for OS subprocesses during system initialization
  FileSystemService := BankAlloc(2 * 65536);  // 2 banks
  GraphicsDriver := BankAlloc(2 * 65536);      // 2 banks
  AudioService := BankAlloc(2 * 65536);        // 2 banks
  NetworkDriver := BankAlloc(1 * 65536);       // 1 bank (if present)
  SchedulerService := BankAlloc(1 * 65536);     // 1 bank
  MemoryManager := BankAlloc(1 * 65536);       // 1 bank
  DeviceManager := BankAlloc(1 * 65536);       // 1 bank
  // Total: ~10-15 banks for OS subprocesses
end;
```

**Result:** 
- **Initial:** 254 banks available
- **After OS startup:** ~224-239 banks available for user processes
- **Conservative estimate:** 224 banks available

---

## 8. Performance Considerations

### Bank Switching Overhead

**Cost of Bank Switch:**
- Update bank register: ~2-3 CPU cycles
- Cache/TLB effects: Minimal (no hardware cache on 65C816)
- **Total overhead: Negligible** (< 1% for typical access patterns)

**Optimization Strategies:**
- Keep frequently accessed data in same bank
- Minimize cross-bank accesses
- Use bank registers efficiently (D, PBR, DBR)

### Allocation Performance

**O(1) Operations:**
- Allocate: Pop from free list
- Free: Push to free list
- Lookup: Direct table access (BankTable[bank])

**No Search Required:**
- Free list provides O(1) allocation
- Table provides O(1) validation
- No fragmentation within banks (whole-bank allocation)

---

## 9. Summary

### Key Findings

1. **Bank Size:** 64KB (architecturally required, not configurable)
2. **Total Banks:** 256 banks (0x00 to 0xFF)
3. **Reserved Banks:** 2 banks (Kernel + ROM)
4. **OS Subprocess Banks:** 15-30 banks (5-10 subprocesses × 1-4 banks)
5. **Available for User Processes:** 224-239 banks (conservative: 224)
6. **Maximum User Processes:** 224 (if 1 bank each, worst case), 40-80 (practical)
7. **Primary App Exception:** 5-8 banks (returns to pool on termination)
8. **Total Available Memory:** ~14MB for user processes (conservative)

### Recommendations

✅ **Use 64KB banks** (only option, hardware-dictated)  
✅ **Reserve Bank 0x00** for kernel  
✅ **Reserve Bank 0xFF** (and possibly 0xFE) for ROM  
✅ **Allocate 15-30 banks for OS subprocesses** (system services, drivers)  
✅ **Support primary app with 5-8 banks** (foreground application exception)  
✅ **Use kernel-maintained free list** for O(1) allocation  
✅ **Allocate whole banks** (no sub-bank fragmentation)  
✅ **Support multi-bank allocations** (1-4 banks typical, 5-8 for primary app)  

### Implementation Notes

- Bank size is **not a configuration option**—it's fixed by 65C816 architecture
- Focus implementation effort on **efficient bank table management**
- Design for **224 available banks** (conservative estimate, accounting for OS subprocesses)
- **OS subprocesses:** 5-10 processes, 1-4 banks each (15-30 banks total)
- **User processes:** 1-4 banks typical, 5-8 banks for primary app
- **Primary app banks return to free pool** when app terminates
- Use **whole-bank allocation** for simplicity and performance

---

**End of Bank Size Analysis**
