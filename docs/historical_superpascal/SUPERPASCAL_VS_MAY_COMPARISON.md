# Historical SuperPascal vs Rust May/Mayfly: Concurrency Model Comparison

**Analysis Date:** December 2024  
**Historical SuperPascal:** Per Brinch Hansen (1993)  
**Rust May:** Stackful coroutine library (similar to Go goroutines)  
**Rust Mayfly:** Coroutine-based task scheduler built on May

---

## Executive Summary

The historical SuperPascal and modern Rust May/Mayfly implementations share **striking architectural similarities** despite being separated by 30+ years. Both use:

1. **Cooperative multitasking** with explicit yield points
2. **Ready queue** for process/task scheduling
3. **Stackful coroutines** (May) or stack frames (SuperPascal)
4. **Synchronous message passing** (SuperPascal channels) or async I/O (May)
5. **Single-threaded scheduler** (SuperPascal) or multi-threaded with work-stealing (May)

The key difference is that **SuperPascal was designed for deterministic parallelism** (scientific computing), while **May/Mayfly are designed for high-performance concurrent I/O** (network servers, async systems).

---

## 1. Architectural Similarities

### 1.1 Ready Queue Pattern

**Historical SuperPascal:**
```pascal
{ From interpret.p }
var ready: integer;  { Pointer to ready queue }
st: store;          { Global memory array }

procedure activate(bvalue, svalue, pvalue: integer);
begin
  svalue := svalue + 3;
  st[svalue - 2] := pvalue;  { PC }
  st[svalue - 1] := bvalue;   { Block pointer }
  st[svalue] := ready;        { Link to next }
  ready := svalue             { Add to queue }
end;

procedure select(lineno: integer);
begin
  if ready = 0 then
    error(lineno, deadlock4)
  else
    begin
      s := ready;
      ready := st[s];    { Remove from queue }
      b := st[s - 1];    { Restore context }
      p := st[s - 2];
      s := s - 3
    end
end;
```

**Rust Mayfly:**
```rust
// From mayfly/crates/scheduler/src/ready_queue.rs
pub struct ReadyQueue {
    heap: BinaryHeap<ReadyEntry>,  // Priority queue
    set: HashSet<TaskId>,           // Deduplication
}

pub struct ReadyEntry {
    pub pri: u8,    // Priority
    pub seq: u64,   // FIFO ordering
    pub tid: TaskId, // Task ID
}

impl ReadyQueue {
    pub fn push(&mut self, entry: ReadyEntry) {
        if self.set.insert(entry.tid) {
            self.heap.push(entry);
        }
    }
    
    pub fn pop(&mut self) -> Option<TaskId> {
        self.heap.pop().map(|entry| {
            self.set.remove(&entry.tid);
            entry.tid
        })
    }
}
```

**Rust May:**
```rust
// From may/src/scheduler.rs
pub struct Scheduler {
    local_queues: Vec<Local<CoroutineImpl>>,  // Per-worker queues
    global_queues: Vec<Queue<CoroutineImpl>>, // Global queues
    // ... work-stealing support
}

impl Scheduler {
    pub fn run_queued_tasks(&self, id: usize) {
        let local = unsafe { self.local_queues.get_unchecked(id) };
        while let Some(co) = local.pop() {
            run_coroutine(co);  // Execute coroutine
        }
    }
}
```

**Key Similarities:**
- ✅ Both use a ready queue to track runnable tasks/processes
- ✅ Both support FIFO scheduling (SuperPascal) or priority-based (Mayfly)
- ✅ Both remove tasks from queue when selected for execution
- ✅ Both add tasks back to queue when they become ready

**Key Differences:**
- **SuperPascal:** Single linked list in global memory array
- **Mayfly:** Priority queue (BinaryHeap) with deduplication
- **May:** Multiple queues (local + global) with work-stealing

---

### 1.2 Process/Task Activation

**Historical SuperPascal:**
```pascal
{ Minimal context: 3 words }
procedure activate(bvalue, svalue, pvalue: integer);
begin
  svalue := svalue + 3;
  st[svalue - 2] := pvalue;  { Program Counter }
  st[svalue - 1] := bvalue;   { Block Pointer }
  st[svalue] := ready;        { Link }
  ready := svalue
end;
```

**Rust May:**
```rust
// From may/src/coroutine_impl.rs (conceptual)
pub fn spawn<F>(f: F) -> JoinHandle<F::Output>
where
    F: FnOnce() -> T + Send + 'static,
{
    let co = CoroutineImpl::new(f);
    get_scheduler().schedule(co);
    JoinHandle::new(co)
}
```

**Rust Mayfly:**
```rust
// From mayfly/crates/scheduler/src/scheduler.rs
pub fn spawn<F>(&mut self, f: F, pri: u8) -> JoinHandle<F::Output>
where
    F: FnOnce() -> T + Send + 'static,
{
    let tid = self.next_id;
    self.next_id += 1;
    
    let task = Task::new(tid, f, pri);
    self.tasks.insert(tid, task);
    
    // Add to ready queue
    self.ready.push(ReadyEntry {
        pri,
        seq: self.seq,
        tid,
    });
    self.seq += 1;
    
    JoinHandle::new(tid)
}
```

**Key Similarities:**
- ✅ Both create a new execution context (process/coroutine/task)
- ✅ Both add the new context to the ready queue
- ✅ Both use minimal context switching overhead

**Key Differences:**
- **SuperPascal:** 3 words (PC, block pointer, link) - minimal!
- **May:** Full stackful coroutine (generator-based)
- **Mayfly:** Task with priority and metadata

---

### 1.3 Cooperative Scheduling

**Historical SuperPascal:**
```pascal
{ Main interpreter loop }
while running do
  case st[p] of
    { Instructions... }
    send2, receive2: { Block on channel }
      begin
        { Find matching operation }
        { If not found, block and select() }
        select(lineno)
      end;
    endprocess2: { Process completes }
      begin
        { Remove from queue }
        if st[s] > 0 then
          select(lineno)  { Switch to next }
      end;
  end;
```

**Rust May:**
```rust
// Coroutines yield at I/O operations
pub fn run_coroutine(mut co: CoroutineImpl) {
    loop {
        match co.resume() {
            GeneratorState::Yielded(_) => {
                // Coroutine yielded (I/O, sleep, etc.)
                // Add back to queue or wait for event
            }
            GeneratorState::Complete(_) => {
                // Coroutine finished
                break;
            }
        }
    }
}
```

**Rust Mayfly:**
```rust
// From mayfly scheduler main loop
pub fn run(&mut self) {
    loop {
        // Process ready tasks
        while let Some(tid) = self.ready.pop() {
            let task = self.tasks.get_mut(&tid).unwrap();
            match task.run() {
                TaskState::Ready => {
                    // Task yielded, add back to queue
                    self.ready.push(ReadyEntry { ... });
                }
                TaskState::Blocked => {
                    // Task blocked on I/O, wait for event
                }
                TaskState::Finished => {
                    // Task completed
                }
            }
        }
        
        // Wait for I/O events or timeouts
        self.wait_for_events();
    }
}
```

**Key Similarities:**
- ✅ Both use cooperative scheduling (tasks yield voluntarily)
- ✅ Both switch tasks at explicit yield points
- ✅ Both handle blocking operations (I/O, channels, sleep)

**Key Differences:**
- **SuperPascal:** Explicit `select()` call for context switch
- **May:** Generator-based yield (automatic at I/O boundaries)
- **Mayfly:** Task state machine with explicit state transitions

---

## 2. Communication Models

### 2.1 Synchronous Message Passing

**Historical SuperPascal:**
```pascal
{ Synchronous channel communication }
type channel = *(integer);
var c: channel;

send(c, value);    { Blocks until receiver ready }
receive(c, var);   { Blocks until sender ready }

{ Implementation: Rendezvous }
send2{typeno, length, channelno, lineno}:
  begin
    si := open[channelno];
    if si = 0 then
      begin
        { No receiver, block }
        open[channelno] := s;  { Save our stack position }
        select(lineno)          { Switch to next process }
      end
    else
      begin
        { Receiver found! }
        { Copy data from sender stack to receiver stack }
        { Wake receiver }
        activate(bi, si, pi);
      end
  end;
```

**Rust May:**
```rust
// Asynchronous channels (MPMC)
use may::sync::mpsc;

let (tx, rx) = mpsc::channel();

// Spawn sender
may::go!(move || {
    tx.send(42).unwrap();
});

// Spawn receiver
may::go!(move || {
    let value = rx.recv().unwrap();
});
```

**Rust Mayfly:**
```rust
// Uses crossbeam channels (async) or custom sync primitives
use crossbeam::channel;

let (tx, rx) = channel::unbounded();
```

**Key Similarities:**
- ✅ Both support message passing between processes/tasks
- ✅ Both use channels for communication

**Key Differences:**
- **SuperPascal:** Synchronous rendezvous (both must be ready)
- **May/Mayfly:** Asynchronous channels (buffered, non-blocking)
- **SuperPascal:** Direct stack-to-stack copy (zero-copy!)
- **May/Mayfly:** Heap-allocated messages

---

## 3. Scheduling Algorithms

### 3.1 Historical SuperPascal: FIFO Round-Robin

```pascal
{ Simple FIFO queue }
ready := 0;  { Empty queue }

{ Add process }
activate(b, s, p);  { Adds to head of queue }

{ Select next }
select();  { Removes from head (FIFO) }
```

**Characteristics:**
- ✅ Deterministic (same input → same execution order)
- ✅ Fair (all processes get equal time)
- ✅ Simple (single linked list)
- ❌ No priority support
- ❌ No work-stealing

### 3.2 Rust May: Work-Stealing Multi-Threaded

```rust
pub struct Scheduler {
    local_queues: Vec<Local<CoroutineImpl>>,  // Per-worker
    global_queues: Vec<Queue<CoroutineImpl>>, // Global
    #[cfg(feature = "work_steal")]
    stealers: Vec<Steal<CoroutineImpl>>,      // Work-stealing
}

impl Scheduler {
    pub fn run_queued_tasks(&self, id: usize) {
        // 1. Try local queue first
        while let Some(co) = self.local_queues[id].pop() {
            run_coroutine(co);
        }
        
        // 2. Try global queue
        while let Some(co) = self.global_queues[id].pop() {
            run_coroutine(co);
        }
        
        // 3. Steal from other workers
        #[cfg(feature = "work_steal")]
        for stealer in &self.stealers {
            if let Some(co) = stealer.steal() {
                run_coroutine(co);
                break;
            }
        }
    }
}
```

**Characteristics:**
- ✅ Multi-threaded (parallel execution)
- ✅ Work-stealing (load balancing)
- ✅ High performance (lock-free queues)
- ❌ Non-deterministic (race conditions possible)
- ❌ More complex (multiple queues, synchronization)

### 3.3 Rust Mayfly: Priority Queue

```rust
pub struct ReadyQueue {
    heap: BinaryHeap<ReadyEntry>,  // Priority queue
    set: HashSet<TaskId>,           // Deduplication
}

impl ReadyQueue {
    pub fn pop(&mut self) -> Option<TaskId> {
        self.heap.pop().map(|entry| {
            self.set.remove(&entry.tid);
            entry.tid
        })
    }
}
```

**Characteristics:**
- ✅ Priority-based scheduling
- ✅ FIFO for equal priorities (via sequence counter)
- ✅ Deduplication (prevents duplicate entries)
- ❌ O(log n) insertion/removal (vs O(1) for linked list)
- ❌ More memory overhead

---

## 4. Context Switching Overhead

### 4.1 Historical SuperPascal: Minimal (3 words)

```pascal
{ Stack frame layout }
[s-2] = Program Counter (p)
[s-1] = Block Pointer (b)
[s]   = Link to next process

{ Context switch }
select():
  s := ready;           { Get next process }
  ready := st[s];       { Remove from queue }
  b := st[s - 1];       { Restore block pointer }
  p := st[s - 2];       { Restore PC }
  s := s - 3            { Pop context }
```

**Overhead:** ~12 bytes per process (3 × 4 bytes on 32-bit)

### 4.2 Rust May: Stackful Coroutine

```rust
// Full stack copy (generator-based)
pub struct CoroutineImpl {
    stack: Vec<u8>,        // Full stack
    stack_size: usize,
    // ... generator state
}
```

**Overhead:** ~1-8 KB per coroutine (configurable stack size)

### 4.3 Rust Mayfly: Task Metadata

```rust
pub struct Task {
    id: TaskId,
    state: TaskState,
    priority: u8,
    // ... other metadata
}
```

**Overhead:** ~100-200 bytes per task (metadata only, stack separate)

---

## 5. Memory Model

### 5.1 Historical SuperPascal: Single Address Space

```pascal
var
  st: store;  { Single global memory array }
  b, s, p: integer;  { Current context }
```

**Characteristics:**
- ✅ Single address space (all processes share memory)
- ✅ Explicit stack management (in global array)
- ✅ No memory protection (compile-time safety instead)
- ❌ No isolation (processes can access each other's memory)
- ❌ Manual memory management

### 5.2 Rust May/Mayfly: Separate Stacks

```rust
// Each coroutine has its own stack
pub struct CoroutineImpl {
    stack: Vec<u8>,  // Separate stack per coroutine
}
```

**Characteristics:**
- ✅ Stack isolation (each coroutine has own stack)
- ✅ Rust ownership system (memory safety)
- ✅ Automatic memory management
- ❌ Higher memory overhead (per-coroutine stack)
- ❌ Stack size must be pre-allocated

---

## 6. Determinism vs Performance

### 6.1 Historical SuperPascal: Deterministic

**Goal:** Scientific computing (reproducible results)

```pascal
{ Same input → same output }
parallel
  process compute1() endprocess |
  process compute2() endprocess
endparallel
```

**Guarantees:**
- ✅ Deterministic execution order
- ✅ Reproducible results
- ✅ Compile-time interference checking
- ❌ Single-threaded (no real parallelism)
- ❌ Lower performance (sequential execution)

### 6.2 Rust May/Mayfly: High Performance

**Goal:** High-throughput concurrent I/O

```rust
// Non-deterministic (but fast!)
for _ in 0..1000 {
    may::go!(|| {
        // Handle request
    });
}
```

**Characteristics:**
- ✅ Multi-threaded (real parallelism)
- ✅ High throughput (millions of coroutines)
- ✅ Work-stealing (load balancing)
- ❌ Non-deterministic (race conditions possible)
- ❌ Requires careful synchronization

---

## 7. Safety Guarantees

### 7.1 Historical SuperPascal: Compile-Time Safety

```pascal
{ Compile-time disjointness checking }
procedure parallelstatement(...);
var used: context;
begin
  { Track variables used in each process }
  { Check that targetvar sets are disjoint }
  { Error if processes write to same variables }
end;
```

**Safety:**
- ✅ Compile-time interference checking
- ✅ No data races (statically prevented)
- ✅ Type-safe channels
- ❌ No runtime memory protection
- ❌ Manual memory management

### 7.2 Rust May/Mayfly: Runtime + Type Safety

```rust
// Rust ownership system
pub fn spawn_safe<F>(f: F) -> Result<JoinHandle<F::Output>, SafetyViolation>
where
    F: FnOnce() -> T + Send + 'static,
    F: CoroutineSafe,  // Compile-time check
{
    // Runtime safety checks
    SafetyChecker::validate(&f)?;
    spawn(f)
}
```

**Safety:**
- ✅ Rust ownership system (compile-time)
- ✅ Runtime safety checks (TLS, stack overflow)
- ✅ Memory safety (automatic)
- ✅ Panic isolation (coroutines don't affect each other)
- ❌ Requires `unsafe` for some operations (May)

---

## 8. Use Cases

### 8.1 Historical SuperPascal

**Designed for:**
- Scientific computing
- Parallel algorithms (matrix operations, simulations)
- Deterministic parallelism
- Educational purposes (teaching concurrency)

**Example:**
```pascal
{ Parallel matrix multiplication }
forall i := 1 to n do
  forall j := 1 to n do
    c[i, j] := 0;
    for k := 1 to n do
      c[i, j] := c[i, j] + a[i, k] * b[k, j]
```

### 8.2 Rust May

**Designed for:**
- High-performance network servers
- Concurrent I/O (web servers, databases)
- Massive concurrency (millions of coroutines)
- Real-time systems

**Example:**
```rust
// Echo server
let listener = TcpListener::bind("127.0.0.1:8000")?;
while let Ok((mut stream, _)) = listener.accept() {
    may::go!(move || {
        // Handle connection
    });
}
```

### 8.3 Rust Mayfly

**Designed for:**
- Task scheduling with priorities
- Dependency tracking (DAG execution)
- I/O event coordination
- System daemons

**Example:**
```rust
// Priority-based task scheduling
scheduler.spawn(|| task1(), PRI_SYSTEM);
scheduler.spawn(|| task2(), PRI_DEFAULT);
```

---

## 9. Key Insights

### 9.1 Architectural Patterns

Both implementations share the same **fundamental pattern**:

1. **Ready Queue:** Track runnable tasks/processes
2. **Cooperative Scheduling:** Tasks yield at blocking operations
3. **Context Switching:** Minimal overhead (SuperPascal) or stackful (May)
4. **Event-Driven:** React to I/O events or channel operations

### 9.2 Evolution Path

**1993 SuperPascal → 2024 May/Mayfly:**

1. **Single-threaded → Multi-threaded**
   - SuperPascal: One interpreter thread
   - May: Multiple worker threads with work-stealing

2. **FIFO → Priority Queue**
   - SuperPascal: Simple linked list
   - Mayfly: Priority queue with FIFO tie-breaking

3. **Synchronous → Asynchronous**
   - SuperPascal: Synchronous rendezvous
   - May: Asynchronous channels

4. **Deterministic → High Performance**
   - SuperPascal: Reproducible results
   - May: Maximum throughput

5. **Compile-time → Runtime Safety**
   - SuperPascal: Static analysis
   - May: Runtime checks + Rust type system

### 9.3 Lessons for Modern Compiler Design

**What to Adopt from SuperPascal:**
- ✅ Minimal context switching (3 words!)
- ✅ Compile-time interference checking
- ✅ Deterministic execution (for scientific computing)
- ✅ Synchronous channels (simpler semantics)

**What to Adopt from May/Mayfly:**
- ✅ Multi-threaded scheduling (real parallelism)
- ✅ Work-stealing (load balancing)
- ✅ Priority queues (real-time systems)
- ✅ Stack isolation (memory safety)

**Hybrid Approach for SuperPascal (Modern):**
- Use SuperPascal's minimal context for single-core systems
- Use May's work-stealing for multi-core systems
- Support both synchronous (deterministic) and asynchronous (high-performance) channels
- Add compile-time interference checking (like SuperPascal) + Rust's ownership system

---

## 10. Code Comparison

### 10.1 Process Spawning

**Historical SuperPascal:**
```pascal
parallel
  process
    statement1
  endprocess |
  process
    statement2
  endprocess
endparallel
```

**Rust May:**
```rust
may::go!(|| statement1());
may::go!(|| statement2());
```

**Rust Mayfly:**
```rust
scheduler.spawn(|| statement1(), PRI_DEFAULT);
scheduler.spawn(|| statement2(), PRI_DEFAULT);
```

### 10.2 Channel Communication

**Historical SuperPascal:**
```pascal
type channel = *(integer);
var c: channel;
open(c);
send(c, 42);
receive(c, x);
```

**Rust May:**
```rust
let (tx, rx) = may::sync::mpsc::channel();
tx.send(42).unwrap();
let x = rx.recv().unwrap();
```

**Rust Mayfly:**
```rust
let (tx, rx) = crossbeam::channel::unbounded();
tx.send(42).unwrap();
let x = rx.recv().unwrap();
```

---

## 11. Conclusion

The historical SuperPascal and modern Rust May/Mayfly implementations are **remarkably similar** in their core architecture:

1. **Both use ready queues** for scheduling
2. **Both use cooperative multitasking** (explicit yields)
3. **Both support message passing** (channels)
4. **Both minimize context switching overhead**

The main differences are:

- **SuperPascal:** Deterministic, single-threaded, compile-time safety
- **May/Mayfly:** High-performance, multi-threaded, runtime safety

For a modern SuperPascal compiler targeting retro platforms:

- **Single-core systems (Z80, 65C02):** Use SuperPascal's minimal context model
- **Multi-core systems (Raspberry Pi 5):** Use May's work-stealing scheduler
- **Safety:** Combine SuperPascal's compile-time checking with Rust's ownership system
- **Channels:** Support both synchronous (deterministic) and asynchronous (performance)

The historical SuperPascal implementation demonstrates that **elegant, minimal designs can be highly effective**, even 30+ years later. The modern May/Mayfly implementations show how these same patterns scale to high-performance, multi-threaded systems.

---

## References

- **Historical SuperPascal:** Per Brinch Hansen (1993), `parse.p`, `interpret.p`
- **Rust May:** https://github.com/Xudong-Huang/may
- **Rust Mayfly:** `/Users/casibbald/Workspace/microscaler/mayfly`
- **SuperPascal Analysis:** `CONCURRENCY_ANALYSIS.md`

