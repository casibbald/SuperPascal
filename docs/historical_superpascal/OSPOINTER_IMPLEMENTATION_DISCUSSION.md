# OSPointer Implementation Discussion

**Date:** December 2024  
**Purpose:** Discuss implementation details for OSPointer capability system before finalizing specification

---

## Overview

This document explores implementation options for OSPointer[T] - the capability-based pointer system for PascalOS. We need to decide on:

1. **Capability Representation** - How OSPointer is stored and validated
2. **Bounds Checking Strategy** - Where and how bounds are checked
3. **Memory Zeroing** - When and how memory is zeroed
4. **Access Mechanism** - How `p[i]` translates to actual memory access

---

## 1. Capability Representation Options

### Option 1A: Direct Capability Structure (Embedded)

**Structure:**
```pascal
type
  OSPointerInternal = record
    BankID: Byte;      // 8-bit bank identifier
    Base: Word;        // 16-bit offset within bank
    Length: Word;      // 16-bit size (bytes or elements)
    Rights: Byte;      // 8-bit permission mask
    Tag: Word;         // 16-bit validation token
  end;  // Total: 8 bytes
```

**Pros:**
- ✅ Fast access (no table lookup)
- ✅ Simple implementation
- ✅ Self-contained (all info in pointer)
- ✅ Good for 65C816 (fits in registers)

**Cons:**
- ❌ User could potentially modify fields (if not properly protected)
- ❌ Larger size (8 bytes vs 2-4 bytes for handle)
- ❌ Tag validation still requires OS check

**Implementation:**
- Compiler treats OSPointer as opaque type
- User code cannot access fields directly
- OS validates tag on each access
- Fields are read-only from user perspective

### Option 1B: Handle/Index (Table-Based)

**Structure:**
```pascal
type
  OSPointer[T] = Word;  // 16-bit handle (index into OS table)
  
  // OS maintains per-process table:
  OSPointerEntry = record
    BankID: Byte;
    Base: Word;
    Length: Word;
    Rights: Byte;
    Tag: Word;
    Valid: Boolean;
    Generation: Word;  // Incremented on free
  end;
```

**Pros:**
- ✅ Small size (2 bytes)
- ✅ Easy to invalidate (mark entry invalid)
- ✅ Generation counter prevents use-after-free
- ✅ Can store more metadata in table

**Cons:**
- ❌ Table lookup on every access (performance)
- ❌ Requires per-process table management
- ❌ More complex implementation

**Implementation:**
- Handle = (table_index << 8) | generation_low_byte
- OS validates: table[handle >> 8].Generation == (handle & 0xFF)
- Fast validation (single table lookup)

### Option 1C: Hybrid (Recommended)

**Structure:**
```pascal
type
  OSPointer[T] = record
    BankID: Byte;      // Direct (fast access)
    Base: Word;        // Direct (fast access)
    Handle: Word;      // Index for validation/lookup
  end;  // Total: 5 bytes
```

**Pros:**
- ✅ Fast access (BankID/Base directly available)
- ✅ Validation via handle (prevents forgery)
- ✅ Can store Length/Rights in table (can change)
- ✅ Best of both worlds

**Cons:**
- ❌ Still requires table lookup for validation
- ❌ More complex than pure direct

**Implementation:**
- BankID and Base stored directly (for fast address calculation)
- Handle used for validation and to lookup Length/Rights
- Compiler can optimize: use direct fields for address, handle for validation

---

## 2. Bounds Checking Strategy

### Option 2A: Compiler-Generated Checks (Inline)

**Approach:**
```pascal
// User code:
value := p[i];

// Compiler generates:
if i >= p.Length then
  RuntimeError('Out of bounds');
value := MemRead(p.BankID, p.Base + i * sizeof(T));
```

**Pros:**
- ✅ Fast (no syscall overhead)
- ✅ Can optimize away known bounds
- ✅ Compile-time constant folding
- ✅ Clear error location

**Cons:**
- ❌ Requires Length in OSPointer (larger size)
- ❌ User could potentially bypass (if compiler bug)
- ❌ Still needs OS validation for capability authenticity

**Implementation:**
- Compiler inserts bounds check before every `p[i]` access
- Check can be optimized if bounds are compile-time known
- OS still validates capability handle on access

### Option 2B: OS Syscall Checks (Every Access)

**Approach:**
```pascal
// User code:
value := p[i];

// Compiler generates:
value := OS_MemRead(p, i);  // Syscall with bounds check

// OS implementation:
function OS_MemRead(ptr: OSPointer[Byte]; index: Word): Byte;
begin
  ValidateCapability(ptr);           // Check handle valid
  if index >= ptr.Length then        // Check bounds
    RaiseError('Out of bounds');
  Result := ReadMemory(ptr.BankID, ptr.Base + index);
end;
```

**Pros:**
- ✅ Centralized validation
- ✅ Cannot be bypassed
- ✅ Consistent error handling
- ✅ Can track all memory accesses (debugging)

**Cons:**
- ❌ Syscall overhead on every access
- ❌ Slower performance
- ❌ May be too slow for tight loops

**Implementation:**
- Every `p[i]` becomes OS syscall
- OS validates capability and bounds
- Returns value or raises error

### Option 2C: Hybrid (Recommended)

**Approach:**
```pascal
// Compiler generates:
if i >= p.Length then
  RuntimeError('Out of bounds');
value := OS_MemReadFast(p, i);  // Fast path (validated capability)

// OS provides two paths:
// 1. Fast path (capability pre-validated, just bounds check)
// 2. Slow path (full validation)
```

**Pros:**
- ✅ Compiler does bounds check (fast, can optimize)
- ✅ OS validates capability (security)
- ✅ Best performance with safety

**Cons:**
- ❌ More complex (two code paths)
- ❌ Requires capability validation strategy

**Implementation:**
- Compiler: Bounds check + fast OS call
- OS: Capability validation (can cache validation result)
- Fast path: Assume capability valid if recently validated
- Slow path: Full validation if capability changed

### Option 2D: Compiler Bounds + OS Capability Only

**Approach:**
```pascal
// Compiler generates:
if i >= p.Length then
  RuntimeError('Out of bounds');
// Direct memory access (compiler calculates address)
value := MemReadDirect(p.BankID, p.Base + i * sizeof(T));

// But OS validates capability on:
// - Allocation
// - Free
// - Grant/Revoke
// - Context switch (verify process owns capabilities)
```

**Pros:**
- ✅ Fastest (no syscall on access)
- ✅ Compiler can optimize heavily
- ✅ OS validates on operations (allocation, free)

**Cons:**
- ❌ Requires hardware/software memory protection
- ❌ May not work on all platforms
- ❌ Less runtime safety

**Implementation:**
- Compiler: Full bounds checking + direct memory access
- OS: Validates capabilities on allocation/free operations
- Runtime: Assumes valid capabilities (validated at creation)
- Platform: May require MMU or memory windows for isolation

---

## 3. Memory Zeroing Strategy

### Kernel Bank Table Design

**Critical:** Bank discovery must use a kernel-maintained table, not a search operation.

**Bank Table Structure:**
```pascal
type
  BankState = (bsFree, bsAllocated, bsReserved);
  
  BankTableEntry = record
    State: BankState;
    OwnerPID: Word;        // Process that owns this bank
    Size: Word;            // Size allocated
    AllocTime: DWord;      // For debugging/tracking
  end;
  
var
  BankTable: array[0..255] of BankTableEntry;  // One entry per bank
  FreeBankList: array[0..255] of Byte;         // Free bank indices
  FreeBankCount: Byte;                         // Number of free banks
```

**Allocation Algorithm:**
```pascal
function BankAlloc(size: Cardinal): OSPointer[Byte];
var
  bank: Byte;
begin
  // Get from free list (O(1) operation)
  if FreeBankCount = 0 then
    RaiseError('No free banks available');
  
  bank := FreeBankList[FreeBankCount - 1];
  FreeBankCount := FreeBankCount - 1;
  
  // Mark as allocated in table
  BankTable[bank].State := bsAllocated;
  BankTable[bank].OwnerPID := CurrentProcess.PID;
  BankTable[bank].Size := size;
  
  ZeroMemory(bank, size);  // Zero before returning
  Result := CreateCapability(bank, 0, size, READ_WRITE);
end;
```

**Free Algorithm:**
```pascal
function BankFree(ptr: OSPointer[Byte]): Boolean;
begin
  ValidateOwnership(ptr);
  
  // Return to free list (O(1) operation)
  FreeBankList[FreeBankCount] := ptr.BankID;
  FreeBankCount := FreeBankCount + 1;
  
  // Mark as free in table
  BankTable[ptr.BankID].State := bsFree;
  BankTable[ptr.BankID].OwnerPID := 0;
  
  ZeroMemory(ptr.BankID, ptr.Length);  // Zero before freeing
  InvalidateCapability(ptr);
end;
```

**Benefits:**
- ✅ O(1) allocation (no search)
- ✅ O(1) free (direct table update)
- ✅ Kernel maintains authoritative state
- ✅ Fast and predictable

### Option 3A: Zero-on-Alloc Only

**Approach:**
```pascal
function BankAlloc(size: Cardinal): OSPointer[Byte];
var
  bank: Byte;
begin
  // Get from kernel-maintained free list (O(1))
  if FreeBankCount = 0 then
    RaiseError('No free banks available');
  
  bank := FreeBankList[FreeBankCount - 1];
  FreeBankCount := FreeBankCount - 1;
  
  BankTable[bank].State := bsAllocated;
  BankTable[bank].OwnerPID := CurrentProcess.PID;
  BankTable[bank].Size := size;
  
  ZeroMemory(bank, size);  // Zero before returning
  Result := CreateCapability(bank, 0, size, READ_WRITE);
end;
```

**Pros:**
- ✅ Prevents reading uninitialized data
- ✅ Simple (one zeroing operation)
- ✅ Fast allocation

**Cons:**
- ❌ Use-after-free may see old data
- ❌ Information leakage possible
- ❌ Less secure

**Implementation:**
- Zero memory when allocating
- No zeroing on free
- Stale pointers may access old data (until reallocated)

### Option 3B: Zero-on-Free Only

**Approach:**
```pascal
function BankFree(ptr: OSPointer[Byte]): Boolean;
begin
  ValidateOwnership(ptr);
  
  // Return to kernel-maintained free list (O(1))
  FreeBankList[FreeBankCount] := ptr.BankID;
  FreeBankCount := FreeBankCount + 1;
  
  BankTable[ptr.BankID].State := bsFree;
  BankTable[ptr.BankID].OwnerPID := 0;
  
  ZeroMemory(ptr.BankID, ptr.Length);  // Zero before freeing
  InvalidateCapability(ptr);
end;
```

**Pros:**
- ✅ Prevents use-after-free from seeing data
- ✅ Information leakage prevention
- ✅ Security benefit

**Cons:**
- ❌ Slower free operation
- ❌ Uninitialized data on alloc (if not zeroed)
- ❌ May zero data that's about to be overwritten

**Implementation:**
- Zero memory when freeing
- No zeroing on alloc
- New allocations may contain garbage (until written)

### Option 3C: Zero-on-Both (Recommended)

**Approach:**
```pascal
function BankAlloc(size: Cardinal): OSPointer[Byte];
var
  bank: Byte;
begin
  // Get from kernel-maintained free list (O(1))
  if FreeBankCount = 0 then
    RaiseError('No free banks available');
  
  bank := FreeBankList[FreeBankCount - 1];
  FreeBankCount := FreeBankCount - 1;
  
  BankTable[bank].State := bsAllocated;
  BankTable[bank].OwnerPID := CurrentProcess.PID;
  BankTable[bank].Size := size;
  
  ZeroMemory(bank, size);  // Zero before returning
  Result := CreateCapability(bank, 0, size, READ_WRITE);
end;

function BankFree(ptr: OSPointer[Byte]): Boolean;
begin
  ValidateOwnership(ptr);
  
  // Return to kernel-maintained free list (O(1))
  FreeBankList[FreeBankCount] := ptr.BankID;
  FreeBankCount := FreeBankCount + 1;
  
  BankTable[ptr.BankID].State := bsFree;
  BankTable[ptr.BankID].OwnerPID := 0;
  
  ZeroMemory(ptr.BankID, ptr.Length);  // Zero before freeing
  InvalidateCapability(ptr);
end;
```

**Pros:**
- ✅ Maximum security (zero on both)
- ✅ Prevents information leakage
- ✅ Prevents use-after-free data access
- ✅ Clean state guarantee

**Cons:**
- ❌ Slower (two zeroing operations)
- ❌ May be overkill for educational use

**Implementation:**
- Zero on alloc: Prevents reading garbage
- Zero on free: Prevents information leakage
- Both operations: Maximum safety
- Can be optimized: Only zero if security-critical

### Option 3D: Zero-on-Free with Poison Pattern

**Approach:**
```pascal
function BankFree(ptr: OSPointer[Byte]): Boolean;
begin
  ValidateOwnership(ptr);
  FillMemory(ptr.BankID, ptr.Length, $DEAD);  // Poison pattern
  InvalidateCapability(ptr);
  MarkBankFree(ptr.BankID);
end;

function BankAlloc(size: Cardinal): OSPointer[Byte];
var
  bank: Byte;
begin
  // Get from kernel-maintained free list (O(1))
  if FreeBankCount = 0 then
    RaiseError('No free banks available');
  
  bank := FreeBankList[FreeBankCount - 1];
  FreeBankCount := FreeBankCount - 1;
  
  BankTable[bank].State := bsAllocated;
  BankTable[bank].OwnerPID := CurrentProcess.PID;
  BankTable[bank].Size := size;
  
  // Check for poison pattern (debugging)
  if ContainsPoison(bank, size) then
    LogWarning('Allocating previously freed memory');
  ZeroMemory(bank, size);  // Zero before returning
  Result := CreateCapability(bank, 0, size, READ_WRITE);
end;
```

**Pros:**
- ✅ Debugging aid (poison pattern visible)
- ✅ Security (zero on alloc)
- ✅ Can detect use-after-free (poison pattern)

**Cons:**
- ❌ More complex
- ❌ Two passes (poison then zero)

**Implementation:**
- Free: Fill with poison pattern ($DEAD, $BEEF, etc.)
- Alloc: Check for poison, then zero
- Use-after-free: Accessing poison pattern is obvious

---

## 4. Access Mechanism Design

### Option 4A: Pure Syscall (Every Access)

**User Code:**
```pascal
var buf: OSPointer[Byte];
var value: Byte;

buf := BankAlloc(1024);
value := buf[5];      // Compiler: OS_MemRead(buf, 5)
buf[5] := 42;        // Compiler: OS_MemWrite(buf, 5, 42)
```

**Compiler Generates:**
```pascal
// Read
value := OS_MemRead8(buf, 5);

// Write
OS_MemWrite8(buf, 5, 42);
```

**OS Implementation:**
```pascal
function OS_MemRead8(ptr: OSPointer[Byte]; index: Word): Byte;
begin
  ValidateCapability(ptr);           // Check handle valid
  if index >= ptr.Length then        // Check bounds
    RaiseError('Out of bounds');
  if not (ptr.Rights and READ) then // Check permissions
    RaiseError('No read permission');
  Result := ReadMemory(ptr.BankID, ptr.Base + index);
end;
```

**Pros:**
- ✅ Maximum safety
- ✅ Cannot be bypassed
- ✅ Centralized validation
- ✅ Can track all accesses

**Cons:**
- ❌ Syscall overhead on every access
- ❌ May be too slow for performance-critical code

### Option 4B: Compiler-Generated Checks + Direct Access

**User Code:**
```pascal
value := buf[5];
buf[5] := 42;
```

**Compiler Generates:**
```pascal
// Read
if 5 >= buf.Length then
  RuntimeError('Out of bounds');
if not (buf.Rights and READ) then
  RuntimeError('No read permission');
value := MemReadDirect(buf.BankID, buf.Base + 5);

// Write
if 5 >= buf.Length then
  RuntimeError('Out of bounds');
if not (buf.Rights and WRITE) then
  RuntimeError('No write permission');
MemWriteDirect(buf.BankID, buf.Base + 5, 42);
```

**Pros:**
- ✅ Fast (no syscall)
- ✅ Compiler can optimize
- ✅ Can eliminate redundant checks

**Cons:**
- ❌ Requires capability validation elsewhere
- ❌ User could potentially bypass (if compiler bug)
- ❌ Less runtime safety

### Option 4C: Hybrid with Validation Caching (Recommended)

**Approach:**
```pascal
// Compiler generates optimized code:
if 5 >= buf.Length then
  RuntimeError('Out of bounds');
// Fast path: Assume capability valid if recently validated
value := OS_MemReadFast(buf, 5);  // Inline or fast syscall

// OS provides:
function OS_MemReadFast(ptr: OSPointer[Byte]; index: Word): Byte;
begin
  // Fast path: Capability validated in current context
  // Only check if capability changed
  if ptr.ValidationCache <> CurrentContext.ValidationEpoch then
    ValidateCapability(ptr);  // Full validation
  Result := ReadMemory(ptr.BankID, ptr.Base + index);
end;
```

**Pros:**
- ✅ Fast (cached validation)
- ✅ Safe (periodic validation)
- ✅ Best performance with safety

**Cons:**
- ❌ More complex (validation caching)
- ❌ Requires context tracking

**Implementation:**
- Compiler: Bounds check + fast access
- OS: Validation caching (epoch-based)
- Context switch: Invalidate cache
- Access: Check cache, validate if needed

---

## 5. Capability Validation Strategy

### Option 5A: Validate on Every Access

**Approach:**
```pascal
function OS_MemRead(ptr: OSPointer[Byte]; index: Word): Byte;
begin
  ValidateCapability(ptr);  // Always validate
  // ... rest of access
end;

procedure ValidateCapability(ptr: OSPointer[Byte]);
begin
  entry := CapabilityTable[ptr.Handle];
  if not entry.Valid then
    RaiseError('Invalid capability');
  if entry.Generation <> ptr.Generation then
    RaiseError('Stale capability');
  if entry.OwnerPID <> CurrentProcess.PID then
    RaiseError('Capability not owned');
end;
```

**Pros:**
- ✅ Maximum safety
- ✅ Catches all invalid accesses
- ✅ Simple logic

**Cons:**
- ❌ Performance overhead
- ❌ Table lookup on every access

### Option 5B: Validate on Operations Only

**Approach:**
```pascal
// Validate only on:
// - Allocation (create capability)
// - Free (invalidate capability)
// - Grant/Revoke (transfer capability)
// - Context switch (verify process owns capabilities)

// Access assumes valid (validated at creation)
function OS_MemRead(ptr: OSPointer[Byte]; index: Word): Byte;
begin
  // No validation - assume valid
  Result := ReadMemory(ptr.BankID, ptr.Base + index);
end;
```

**Pros:**
- ✅ Fast (no validation overhead)
- ✅ Simple

**Cons:**
- ❌ Less safe (no runtime validation)
- ❌ Stale pointers may work
- ❌ Requires hardware protection or trust

### Option 5C: Epoch-Based Validation (Recommended)

**Approach:**
```pascal
type
  ValidationEpoch = Word;  // Incremented on capability changes
  
var
  CurrentEpoch: ValidationEpoch;
  ProcessEpoch: array[PID] of ValidationEpoch;

// On capability change:
procedure InvalidateCapabilities(pid: PID);
begin
  ProcessEpoch[pid] := CurrentEpoch;
  CurrentEpoch := CurrentEpoch + 1;
end;

// On access:
function OS_MemRead(ptr: OSPointer[Byte]; index: Word): Byte;
begin
  if ptr.ValidationEpoch < ProcessEpoch[CurrentProcess.PID] then
    ValidateCapability(ptr);  // Full validation if epoch changed
  Result := ReadMemory(ptr.BankID, ptr.Base + index);
end;
```

**Pros:**
- ✅ Fast (epoch check is cheap)
- ✅ Safe (validates when needed)
- ✅ Efficient (batch invalidation)

**Cons:**
- ❌ More complex (epoch tracking)
- ❌ Requires epoch storage

---

## 6. Zeroing Implementation

### Efficient Zeroing Strategies

**Option 6A: Simple Loop**
```pascal
procedure ZeroMemory(bank: Byte; size: Word);
var
  i: Word;
begin
  for i := 0 to size - 1 do
    MemWrite(bank, i, 0);
end;
```

**Pros:**
- ✅ Simple
- ✅ Works on all platforms

**Cons:**
- ❌ Slow (one byte at a time)
- ❌ Not optimal for 65C816

**Option 6B: Word-Aligned Loop**
```pascal
procedure ZeroMemory(bank: Byte; size: Word);
var
  i: Word;
begin
  // Zero word-aligned
  for i := 0 to (size div 2) - 1 do
    MemWriteWord(bank, i * 2, 0);
  // Zero remaining byte if odd
  if (size mod 2) = 1 then
    MemWrite(bank, size - 1, 0);
end;
```

**Pros:**
- ✅ Faster (2x fewer operations)
- ✅ Good for 65C816 (16-bit operations)

**Cons:**
- ❌ Requires word alignment
- ❌ Still sequential

**Option 6C: Block Zeroing (Recommended)**
```pascal
procedure ZeroMemory(bank: Byte; size: Word);
var
  blocks: Word;
  remainder: Word;
  i: Word;
begin
  // Zero in 256-byte blocks (fast)
  blocks := size div 256;
  remainder := size mod 256;
  
  for i := 0 to blocks - 1 do
    ZeroBlock(bank, i * 256, 256);
  
  if remainder > 0 then
    ZeroBlock(bank, blocks * 256, remainder);
end;

procedure ZeroBlock(bank: Byte; offset: Word; count: Word);
begin
  // Use 16-bit stores for speed
  // Can be optimized with unrolled loops
  // Or use DMA if available
end;
```

**Pros:**
- ✅ Fast (block operations)
- ✅ Can use DMA if available
- ✅ Good performance

**Cons:**
- ❌ More complex
- ❌ Platform-specific optimizations

---

## 7. Recommended Implementation

Based on the analysis, here's the recommended approach:

### Capability Representation: Hybrid (Option 1C)

```pascal
type
  OSPointer[T] = record
    BankID: Byte;      // Direct access (fast)
    Base: Word;        // Direct access (fast)
    Handle: Word;      // For validation (security)
  end;  // 5 bytes total
```

**Rationale:**
- Fast address calculation (BankID/Base direct)
- Security via handle validation
- Balance between performance and safety

### Bounds Checking: Compiler + OS (Option 2C)

```pascal
// Compiler generates:
if i >= p.Length then
  RuntimeError('Out of bounds');
value := OS_MemReadFast(p, i);

// OS validates capability (cached)
```

**Rationale:**
- Compiler does bounds (fast, optimizable)
- OS validates capability (security)
- Best performance with safety

### Memory Zeroing: Zero-on-Both (Option 3C)

```pascal
// Alloc: Zero before returning
// Free: Zero before freeing
```

**Rationale:**
- Maximum security
- Prevents information leakage
- Clean state guarantee

### Bank Management: Kernel-Maintained Table

**Critical Design:** Bank allocation uses a kernel-maintained table with free list, not search operations.

```pascal
// Kernel maintains:
// - BankTable: State, owner, size for each bank
// - FreeBankList: Stack of free bank indices
// - O(1) allocation and free operations
```

**Rationale:**
- O(1) performance (no search)
- Kernel maintains authoritative state
- Fast and predictable
- Essential for real-time behavior

### Access Mechanism: Hybrid with Caching (Option 4C)

```pascal
// Compiler: Bounds check + fast OS call
// OS: Validation caching (epoch-based)
```

**Rationale:**
- Fast (cached validation)
- Safe (periodic validation)
- Best performance

### Validation: Epoch-Based (Option 5C)

```pascal
// Epoch incremented on capability changes
// Fast check: if epoch changed, validate
```

**Rationale:**
- Fast (epoch check is cheap)
- Safe (validates when needed)
- Efficient batch invalidation

### Zeroing Implementation: Block Zeroing (Option 6C)

```pascal
// Zero in 256-byte blocks
// Use 16-bit stores
// Can use DMA if available
```

**Rationale:**
- Fast (block operations)
- Platform-optimizable
- Good performance

---

## 8. Implementation Phases

### Phase 1: Basic OSPointer (MVP)

**Goal:** Get OSPointer working with basic safety

**Features:**
- Direct capability structure (Option 1A)
- Compiler bounds checking (Option 2A)
- OS capability validation on every access (Option 5A)
- Zero-on-alloc only (Option 3A)
- Pure syscall access (Option 4A)

**Rationale:** Start simple, get it working, then optimize

### Phase 2: Performance Optimization

**Goal:** Improve performance while maintaining safety

**Features:**
- Hybrid capability (Option 1C)
- Validation caching (Option 4C)
- Epoch-based validation (Option 5C)
- Block zeroing (Option 6C)

**Rationale:** Optimize based on Phase 1 experience

### Phase 3: Security Hardening

**Goal:** Maximum security

**Features:**
- Zero-on-both (Option 3C)
- Poison patterns (Option 3D)
- Enhanced validation

**Rationale:** Add security features after performance is acceptable

---

## 9. Open Questions

1. **Capability Size:** 5 bytes (hybrid) vs 8 bytes (direct) vs 2 bytes (handle)?
   - **Recommendation:** Start with 8 bytes (direct), optimize to 5 bytes (hybrid) later

2. **Validation Frequency:** Every access vs cached vs epoch-based?
   - **Recommendation:** Start with every access, optimize to epoch-based

3. **Zeroing Strategy:** Alloc only vs Free only vs Both?
   - **Recommendation:** Start with alloc only, add free zeroing for security

4. **Access Mechanism:** Pure syscall vs compiler-generated vs hybrid?
   - **Recommendation:** Start with pure syscall, optimize to hybrid

5. **Performance vs Safety:** How much performance are we willing to trade?
   - **Recommendation:** Start safe, optimize based on profiling

---

## 10. Next Steps

1. **Decide on Phase 1 approach** (MVP)
2. **Document chosen approach** in specification
3. **Implement compiler support** for OSPointer
4. **Implement OS support** for capability management
5. **Test and profile** performance
6. **Optimize** based on results

---

**End of Implementation Discussion**
