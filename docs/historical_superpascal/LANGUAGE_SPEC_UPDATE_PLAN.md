# Language Specification Update Plan: Concurrency Features

**Date:** December 2024  
**Based on:** Combined analysis of Historical SuperPascal and May/Mayfly implementations  
**Purpose:** Detailed plan for updating SuperPascal language specification to include concurrency features

---

## Overview

This document provides a detailed, file-by-file plan for updating the SuperPascal language specification to include concurrency features based on:
1. Historical SuperPascal's cooperative multitasking model
2. Modern May/Mayfly stack frame handling techniques
3. Hybrid approach combining both for optimal platform support

---

## 1. Grammar Specification Updates (`02_Grammar.md`)

### 1.1 Add Parallel Statement Grammar

**Location:** Section 6 (Statements), after Section 6.13 (Block)

**Add:**

```markdown
### 6.14 Parallel Statement

```
parallel-stmt ::= "parallel" process-list "endparallel"
process-list ::= process-item ("|" process-item)*
process-item ::= "process" statement-seq "endprocess"
```

**Examples:**
```pascal
parallel
  process
    ComputeA;
  endprocess |
  process
    ComputeB;
  endprocess
endparallel
```

**Rules:**
- Processes execute concurrently
- All processes must complete before `endparallel`
- Processes cannot share mutable state (compile-time checked)
- Processes can share read-only data
```

### 1.2 Add Forall Statement Grammar

**Location:** Section 6 (Statements), after Section 6.14 (Parallel Statement)

**Add:**

```markdown
### 6.15 Forall Statement

```
forall-stmt ::= "forall" ident ":=" expr "to" expr "do" statement
```

**Examples:**
```pascal
forall i := 1 to 100 do
  Process(i);

forall i := 0 to n-1 do
  array[i] := array[i] * 2;
```

**Rules:**
- Creates dynamic number of processes (one per iteration)
- Loop variable is read-only in body
- All processes execute concurrently
- All processes must complete before continuing
- Processes cannot share mutable state (compile-time checked)
```

### 1.3 Add Channel Type Grammar

**Location:** Section 4 (Type Specifications), after Section 4.10 (String Type)

**Add:**

```markdown
### 4.12 Channel Types

```
channel-type ::= "channel" "[" type-list "]" buffer-size?
                | "async" "channel" "[" type-list "]" buffer-size?
type-list ::= type-spec ("," type-spec)*
buffer-size ::= "[" const-expr "]"  // Optional buffering capacity
```

**Examples:**
```pascal
type
  IntChannel = channel[integer];                    // Synchronous, unbuffered
  MultiChannel = channel[boolean, integer];         // Synchronous, multiple types
  BufferedChannel = channel[integer][10];          // Synchronous, buffered (capacity 10)
  AsyncChannel = async channel[integer];           // Asynchronous, unbuffered
  AsyncBuffered = async channel[string][5];        // Asynchronous, buffered
```

**Rules:**
- `channel` = synchronous rendezvous (SuperPascal-style)
- `async channel` = asynchronous buffered (May-style)
- Buffer size is optional (default: unbuffered)
- Type list specifies allowed message types
- Channels must be opened before use
```

### 1.4 Add Channel Operations Grammar

**Location:** Section 6 (Statements), add to Section 6.2 (Simple Statements)

**Add to `simple-stmt`:**

```
simple-stmt ::= assign-stmt
              | call-stmt
              | goto-stmt
              | intrinsic-stmt
              | channel-open-stmt
              | channel-send-stmt
              | channel-receive-stmt
```

**Add new sections:**

```markdown
### 6.16 Channel Operations

#### 6.16.1 Open Channel

```
channel-open-stmt ::= "open" "(" variable ")"
```

**Example:**
```pascal
var c: channel[integer];
open(c);
```

**Rules:**
- Must be called before using channel
- Channel can only be opened once
- Opening an already-open channel is an error

#### 6.16.2 Send Message

```
channel-send-stmt ::= "send" "(" variable "," expr ")"
```

**Example:**
```pascal
var c: channel[integer];
send(c, 42);
```

**Rules:**
- Expression type must match channel type
- For synchronous channels: blocks until receiver ready
- For asynchronous channels: non-blocking if buffer space available

#### 6.16.3 Receive Message

```
channel-receive-stmt ::= "receive" "(" variable "," variable ")"
```

**Example:**
```pascal
var c: channel[integer];
var value: integer;
receive(c, value);
```

**Rules:**
- Variable type must match channel type
- For synchronous channels: blocks until sender ready
- For asynchronous channels: blocks if buffer empty
- Message is copied to variable
```
```

---

## 2. Type System Updates (`03_TypeSystem.md`)

### 2.1 Add Channel Types Section

**Location:** After Section 2 (Primitive Types), add new Section 2.5

**Add:**

```markdown
### 2.5 Channel Types

Channels provide typed message passing between concurrent processes.

#### Synchronous Channels

**Syntax:** `channel[type1, type2, ...]`

**Characteristics:**
- Synchronous rendezvous (both sender and receiver must be ready)
- Unbuffered (direct handoff)
- Deterministic execution
- Zero-copy message transfer (stack-to-stack)

**Example:**
```pascal
type
  IntChannel = channel[integer];
  MultiChannel = channel[boolean, integer];
  
var c: IntChannel;
var value: integer;

open(c);
send(c, 42);        // Blocks until receiver ready
receive(c, value);  // Blocks until sender ready
```

#### Asynchronous Channels

**Syntax:** `async channel[type1, type2, ...]`

**Characteristics:**
- Asynchronous (non-blocking send if buffer available)
- Buffered (optional capacity)
- Non-deterministic execution
- Heap-allocated messages

**Example:**
```pascal
type
  AsyncIntChannel = async channel[integer][10];  // Buffered, capacity 10
  
var c: AsyncIntChannel;
var value: integer;

open(c);
send(c, 42);        // Non-blocking if buffer space available
receive(c, value);  // Blocks if buffer empty
```

#### Buffered Channels

**Syntax:** `channel[type][capacity]` or `async channel[type][capacity]`

**Characteristics:**
- Buffer capacity specified at compile time
- Send blocks only if buffer full
- Receive blocks only if buffer empty
- Higher throughput than unbuffered

**Example:**
```pascal
type
  BufferedChannel = channel[integer][5];  // Capacity 5
  
var c: BufferedChannel;
open(c);
// Can send up to 5 messages without blocking
```

#### Channel Type Compatibility

- Channels are **nominal types** (by name)
- Two channels with same type signature are **different types**
- Channel types cannot be assigned or compared
- Channels must be explicitly opened before use
```

### 2.2 Add Process Types Section

**Location:** After Channel Types section

**Add:**

```markdown
### 2.6 Process Types

Process types represent concurrent execution contexts.

#### Process Handles

**Syntax:** `process`

**Characteristics:**
- Opaque handle to a running process
- Used for join/wait operations
- Cannot be directly manipulated
- Automatically cleaned up when process completes

**Example:**
```pascal
var p: process;
p := spawn ProcessTask;
join(p);  // Wait for process to complete
```

**Note:** Process handles are a future feature. Initial implementation uses implicit process management.
```

---

## 3. Semantics Updates (`04_Semantics.md`)

### 3.1 Add Parallel Statement Semantics

**Location:** After Section 7 (Control Flow Semantics), add Section 8

**Add:**

```markdown
## 8. Concurrency Semantics

### 8.1 Parallel Statement Semantics

**Semantics:**
1. All processes in `parallel` block are activated
2. Each process gets its own stack frame
3. Processes execute cooperatively (cooperative multitasking)
4. Execution continues after `endparallel` only when all processes complete
5. If all processes block (deadlock), runtime error is raised

**Process Activation:**
- Each process gets its own activation record
- Activation record contains: Program Counter, Block Pointer, Link
- Processes are added to ready queue
- Scheduler selects next ready process (FIFO by default)

**Context Switching:**
- Occurs at blocking operations (send/receive)
- Occurs when process completes
- Minimal overhead: only 3 words saved/restored per process

**Example:**
```pascal
parallel
  process
    ComputeA;  // Process 1
  endprocess |
  process
    ComputeB;  // Process 2
  endprocess
endparallel
// Execution continues here only after both processes complete
```

### 8.2 Forall Statement Semantics

**Semantics:**
1. Evaluate loop bounds (once)
2. Create one process per iteration value
3. Each process executes statement body with its loop variable value
4. All processes execute concurrently
5. Execution continues after `forall` only when all processes complete

**Process Creation:**
- Number of processes = `upper - lower + 1`
- Each process gets its own copy of loop variable
- All processes share same statement body (code)

**Example:**
```pascal
forall i := 1 to 10 do
  Process(i);  // Creates 10 processes, each with different i value
```

### 8.3 Channel Communication Semantics

#### 8.3.1 Synchronous Channels

**Send Semantics:**
1. Check channel is open
2. Search for matching receiver on same channel
3. If receiver found: copy message data, wake receiver, continue
4. If no receiver: block current process, add to channel wait queue, context switch

**Receive Semantics:**
1. Check channel is open
2. Search for matching sender on same channel
3. If sender found: copy message data, wake sender, continue
4. If no sender: block current process, add to channel wait queue, context switch

**Rendezvous:**
- Both sender and receiver must be ready
- Direct stack-to-stack copy (zero-copy)
- Both processes continue after rendezvous
- Deterministic execution order

#### 8.3.2 Asynchronous Channels

**Send Semantics:**
1. Check channel is open
2. If buffer space available: add message to buffer, continue
3. If buffer full: block current process, add to wait queue, context switch

**Receive Semantics:**
1. Check channel is open
2. If buffer not empty: remove message from buffer, continue
3. If buffer empty: block current process, add to wait queue, context switch

**Buffering:**
- Messages stored in heap-allocated buffer
- FIFO ordering (first in, first out)
- Non-deterministic execution (depends on scheduling)

### 8.4 Disjointness Checking (Compile-Time)

**Purpose:** Prevent data races by ensuring parallel processes don't write to same variables.

**Rules:**
1. Track all variables **written** (target variables) in each process
2. Track all variables **read** (expression variables) in each process
3. Check that target variable sets are **disjoint** across all processes
4. If overlap found: compile-time error

**Allowed:**
- Multiple processes can **read** same variable (no interference)
- Processes can read variables written by other processes (after synchronization)

**Not Allowed:**
- Multiple processes **writing** to same variable (data race)
- Process writing variable that another process reads without synchronization

**Example:**
```pascal
var x, y: integer;

parallel
  process
    x := 1;  // ERROR: writes to x
  endprocess |
  process
    x := 2;  // ERROR: writes to x (overlap with process 1)
  endprocess
endparallel

// Allowed:
parallel
  process
    x := 1;  // OK: only process 1 writes to x
  endprocess |
  process
    y := x;  // OK: process 2 reads x (no write conflict)
  endprocess
endparallel
```

**Implementation:**
- Performed during semantic analysis
- Builds variable usage sets for each process
- Compares sets for intersection
- Reports errors with process and variable information
```

### 3.2 Update Memory Model Section

**Location:** Section 9 (Memory Model), add subsection

**Add:**

```markdown
### 9.4 Concurrent Memory Model

**Single Address Space:**
- All processes share same global memory
- Processes have separate stack frames
- No memory protection between processes
- Compile-time interference checking prevents data races

**Stack Frame Isolation:**
- Each process has its own stack frame
- Local variables are isolated per process
- Global variables are shared (read-only access allowed, write access checked)

**Channel Communication:**
- Messages copied between process stacks
- Synchronous: direct stack-to-stack copy
- Asynchronous: heap-allocated buffer

**Memory Safety:**
- Compile-time: Disjointness checking prevents data races
- Runtime: Stack overflow detection (optional, debug mode)
- No automatic garbage collection (manual memory management)
```
```

---

## 4. ABI Concepts Updates (`05_ABI_Concepts.md`)

### 4.1 Add Process Activation Records Section

**Location:** After Section 3 (Return Value Strategies), add Section 3.3

**Add:**

```markdown
### 3.3 Process Activation Records

**Concept:** Minimal context for cooperative multitasking.

**Historical SuperPascal Layout (Minimal):**
```
Process Activation Record (3 words):
[offset -2] = Program Counter (PC)
[offset -1] = Block Pointer (BP)
[offset  0] = Link to next process in ready queue
```

**Characteristics:**
- Minimal overhead: only 3 words per process
- Suitable for single-core systems
- Fast context switching
- No register save/restore (all state in memory)

**Platform-Specific:**
- **Z80/6502:** Use minimal layout (3 words)
- **Raspberry Pi:** Can use minimal or full stackful (May-style)

### 3.4 Full Stackful Coroutines (May-Style)

**Concept:** Complete stack for each coroutine (modern platforms).

**Layout:**
```
Coroutine Stack:
- Full function call stack
- All local variables
- Saved registers (platform-specific)
- Return addresses
- Exception frames (if applicable)
```

**Characteristics:**
- Higher memory overhead (~1-8 KB per coroutine)
- Complete isolation between coroutines
- Supports deep call stacks
- Platform-specific register save/restore

**Platform-Specific:**
- **Raspberry Pi:** Use full stackful coroutines
- **Z80/6502:** Use minimal activation records only

**Note:** May provides extensive assembly implementations for stack frame manipulation. These should be referenced for platform-specific implementations.
```

### 4.2 Add Context Switching Section

**Location:** After Process Activation Records, add new section

**Add:**

```markdown
## 11. Context Switching Conventions

### 11.1 Minimal Context Switching (SuperPascal-Style)

**Process:**
1. Save current PC, BP to stack
2. Add current stack pointer to ready queue
3. Select next process from ready queue
4. Restore PC, BP from selected process stack
5. Continue execution at restored PC

**Assembly Sequence (Conceptual):**
```assembly
; Save current context
push PC
push BP
mov [ready_queue], SP  ; Add to ready queue

; Select next process
mov SP, [ready_queue]    ; Get next process
pop BP                   ; Restore block pointer
pop PC                   ; Restore program counter
ret                      ; Continue execution
```

**Overhead:** ~10-20 instructions per context switch

### 11.2 Full Stackful Context Switching (May-Style)

**Process:**
1. Save all callee-saved registers
2. Save stack pointer
3. Switch to new coroutine stack
4. Restore all callee-saved registers
5. Restore stack pointer
6. Continue execution

**Assembly Sequence (Platform-Specific):**
- **ARM (Raspberry Pi):** Save/restore r4-r11, lr, sp
- **x86-64:** Save/restore rbx, rbp, r12-r15
- **Z80:** Not applicable (use minimal context only)

**Overhead:** ~50-100 instructions per context switch (but supports deep stacks)

### 11.3 Platform-Specific Recommendations

| Platform | Context Type | Reason |
|----------|--------------|--------|
| Z80 | Minimal (3 words) | Limited memory, simple architecture |
| 6502 | Minimal (3 words) | Limited memory, simple architecture |
| Raspberry Pi | Full stackful | Ample memory, modern architecture |
| Foenix (65C816) | Minimal or Full | Depends on application requirements |

**Note:** May's assembly implementations provide excellent reference for full stackful context switching on modern platforms.
```

### 4.3 Update Stack Frame Layout Section

**Location:** Section 4 (Data Layout Principles), add subsection

**Add:**

```markdown
### 4.4 Process Stack Frame Layout

**Minimal Layout (SuperPascal):**
```
Stack Frame (per process):
[SP-2] = Program Counter
[SP-1] = Block Pointer  
[SP]   = Link to next process
```

**Full Layout (May-Style):**
```
Stack Frame (per coroutine):
[SP]   = Saved registers (platform-specific)
[SP+N] = Local variables
[SP+M] = Function parameters
[SP+P] = Return addresses
```

**Platform-Specific:**
- Document exact layout for each platform
- Reference May's assembly code for modern platforms
- Provide assembly examples for context switching
```
```

---

## 5. New Documentation Files

### 5.1 Create Concurrency Specification

**File:** `languageSpecification/16_Concurrency.md`

**Contents:**
- Complete concurrency model specification
- Parallel and forall statements
- Channel types and operations
- Scheduling algorithms
- Disjointness checking rules
- Examples and use cases

### 5.2 Create Platform-Specific ABI Docs

**Files:**
- `platforms/Z80/Concurrency_ABI.md` - Z80 context switching
- `platforms/6502/Concurrency_ABI.md` - 6502 context switching
- `platforms/RaspberryPi/Concurrency_ABI.md` - Raspberry Pi (May-style)

**Contents:**
- Platform-specific stack frame layouts
- Context switching assembly code
- Register preservation rules
- Performance characteristics

---

## 6. Implementation Checklist

### Phase 1: Grammar and Types
- [ ] Add parallel statement to grammar
- [ ] Add forall statement to grammar
- [ ] Add channel types to grammar
- [ ] Add channel operations to grammar
- [ ] Update type system with channel types
- [ ] Document channel type compatibility

### Phase 2: Semantics
- [ ] Document parallel statement semantics
- [ ] Document forall statement semantics
- [ ] Document channel communication semantics
- [ ] Document disjointness checking rules
- [ ] Update memory model section

### Phase 3: ABI and Implementation
- [ ] Document process activation records
- [ ] Document context switching conventions
- [ ] Create platform-specific ABI docs
- [ ] Reference May's assembly implementations
- [ ] Provide assembly examples

### Phase 4: Examples and Testing
- [ ] Add concurrency examples to specification
- [ ] Document common patterns (producer-consumer, etc.)
- [ ] Provide migration guide from historical SuperPascal
- [ ] Document performance characteristics

---

## 7. Key Design Decisions Summary

1. **Support both synchronous and asynchronous channels**
   - `channel` = synchronous (SuperPascal-style)
   - `async channel` = asynchronous (May-style)

2. **Platform-dependent context switching**
   - Retro platforms: Minimal (3 words)
   - Modern platforms: Full stackful (May-style)

3. **Compile-time safety + optional runtime checks**
   - Disjointness checking (compile-time)
   - Stack overflow detection (runtime, debug mode)

4. **Multiple scheduling modes**
   - Deterministic (FIFO) for scientific computing
   - Performance (work-stealing) for high-throughput
   - Real-time (priority queues) for time-critical systems

5. **Reference May's assembly implementations**
   - Document stack frame manipulation
   - Provide platform-specific examples
   - Leverage proven techniques

---

## 8. References

- **Combined Analysis:** `COMBINED_CONCURRENCY_ANALYSIS.md`
- **Historical SuperPascal:** `CONCURRENCY_ANALYSIS.md`
- **May/Mayfly Comparison:** `SUPERPASCAL_VS_MAY_COMPARISON.md`
- **May Library:** External reference for assembly implementations
- **Language Specification:** `languageSpecification/` directory

---

**End of Update Plan**
