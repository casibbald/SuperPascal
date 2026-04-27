# Mayfly Lessons for SuperPascal on Modern Multicore Processors

**Analysis Date:** December 2024  
**Focus:** Applying Mayfly scheduler patterns to SuperPascal for modern multicore systems (Raspberry Pi 5, etc.)

---

## Executive Summary

Mayfly's scheduler provides **critical patterns** for SuperPascal to leverage modern multicore processors effectively:

1. **Priority-based scheduling** with FIFO tie-breaking
2. **DAG dependency tracking** for parallel algorithms
3. **I/O event coordination** (async I/O with efficient polling)
4. **Virtual clock** for deterministic time management
5. **Task state machine** for lifecycle management
6. **System call interface** for task-scheduler communication

These patterns enable SuperPascal to:
- **Scale to multiple cores** (work-stealing, load balancing)
- **Support complex parallel algorithms** (dependency graphs)
- **Handle I/O efficiently** (event-driven, non-blocking)
- **Maintain determinism** (virtual clock, priority ordering)

---

## 1. Priority-Based Scheduling

### 1.1 Mayfly's Approach

```rust
// From mayfly/crates/scheduler/src/ready_queue.rs
pub struct ReadyEntry {
    pub pri: u8,    // Priority (lower = higher priority)
    pub seq: u64,   // Sequence counter for FIFO tie-breaking
    pub tid: TaskId,
}

pub struct ReadyQueue {
    heap: BinaryHeap<ReadyEntry>,  // Priority queue
    set: HashSet<TaskId>,           // Deduplication
}
```

**Key Features:**
- **Priority levels:** System tasks (0) vs user tasks (10+)
- **FIFO tie-breaking:** Equal priorities scheduled in order
- **Deduplication:** Prevents duplicate entries
- **O(log n) operations:** Efficient for large task counts

### 1.2 Application to SuperPascal

**For Raspberry Pi 5 (4 cores, ARM Cortex-A76):**

```pascal
{ Priority-based parallel execution }
type
  TaskPriority = 0..255;  { 0 = highest priority }

{ System tasks (real-time, I/O) }
parallel
  process priority 0
    handle_interrupts()
  endprocess |
  process priority 0
    handle_network_io()
  endprocess |
  process priority 10
    user_computation()
  endprocess
endparallel
```

**Benefits:**
- ✅ **Real-time responsiveness:** System tasks run first
- ✅ **Fair scheduling:** User tasks get equal time
- ✅ **Predictable ordering:** FIFO for equal priorities
- ✅ **Multicore efficiency:** Each core can have its own priority queue

**Implementation Strategy:**
1. **Per-core ready queues:** Each CPU core has its own priority queue
2. **Work-stealing:** Idle cores steal from busy cores' queues
3. **Priority-aware stealing:** Steal high-priority tasks first
4. **Global system queue:** For system tasks that can run on any core

---

## 2. DAG Dependency Tracking

### 2.1 Mayfly's Dependency System

```rust
// From mayfly/crates/scheduler/src/scheduler.rs
pub struct Scheduler {
    dependencies: HashMap<TaskId, HashSet<TaskId>>,  // What this task depends on
    dependents: HashMap<TaskId, HashSet<TaskId>>,    // What depends on this task
    unresolved_deps: HashMap<TaskId, usize>,         // Count of unresolved deps
}

// Task states
pub enum TaskState {
    PendingDependencies,  // Waiting for dependencies
    Ready,                // Can run now
    Running,              // Currently executing
    Finished(Reason),     // Completed
}
```

**Key Features:**
- **Cycle detection:** Prevents deadlock from circular dependencies
- **Automatic resolution:** When dependency completes, dependents become ready
- **Efficient tracking:** O(1) dependency checks, O(d) resolution (d = dependents)

### 2.2 Application to SuperPascal

**For Parallel Algorithms (Matrix Operations, Graph Processing):**

```pascal
{ Parallel matrix multiplication with dependencies }
type
  TaskId = integer;

procedure matrix_multiply(
  var a, b, c: matrix;
  n: integer);
var
  tasks: array [1..n, 1..n] of TaskId;
begin
  { Create tasks with dependencies }
  forall i := 1 to n do
    forall j := 1 to n do
      { Task depends on row i and column j being ready }
      tasks[i, j] := spawn_with_deps(
        [row_task[i], col_task[j]],
        procedure
        begin
          { Compute c[i, j] = sum of a[i, k] * b[k, j] }
          compute_cell(i, j, a, b, c)
        end
      )
end;
```

**Benefits:**
- ✅ **Automatic parallelization:** Compiler can generate dependency graphs
- ✅ **Optimal scheduling:** Tasks run as soon as dependencies are met
- ✅ **Deadlock prevention:** Cycle detection at compile-time
- ✅ **Scalability:** Works across multiple cores

**Implementation Strategy:**
1. **Compile-time analysis:** Build dependency graph during semantic analysis
2. **Runtime tracking:** Scheduler tracks dependencies and resolves automatically
3. **Multicore distribution:** Distribute ready tasks across cores
4. **Priority inheritance:** High-priority dependencies boost dependent priority

---

## 3. I/O Event Coordination

### 3.1 Mayfly's I/O Model

```rust
// From mayfly/crates/scheduler/src/scheduler.rs
#[cfg(feature = "async-io")]
pub struct Scheduler {
    poll: Poll,                              // mio event poller
    sources: HashMap<Token, Arc<dyn IoSource>>,  // Registered I/O sources
    wait_map: WaitMap,                       // I/O ID → waiting tasks
}

// I/O registration
pub fn register_io(&mut self, src: Arc<dyn IoSource>) {
    let token = Token(self.next_token);
    self.poll.registry().register(&mut source, token, Interest::READABLE)?;
    self.sources.insert(token, src);
}

// Event loop
pub fn run(&mut self) {
    let mut events = Events::with_capacity(8);
    loop {
        self.poll.poll(&mut events, Some(timeout))?;
        for ev in events.iter() {
            if let Some(src) = self.sources.get(&ev.token()) {
                for tid in self.wait_map.complete_io(src.id()) {
                    self.push_ready(tid);  // Wake waiting tasks
                }
            }
        }
    }
}
```

**Key Features:**
- **Efficient polling:** Uses `mio` (epoll/kqueue/IOCP) for O(1) I/O readiness
- **Non-blocking:** Tasks yield while waiting for I/O
- **Scalable:** Handles thousands of concurrent I/O operations
- **Cross-platform:** Works on Linux (epoll), macOS (kqueue), Windows (IOCP)

### 3.2 Application to SuperPascal

**For Network Servers, File I/O, Graphics:**

```pascal
{ Async I/O with event-driven scheduling }
procedure handle_network_request(socket: TcpSocket);
begin
  { Non-blocking read }
  var data: array [1..1024] of byte;
  var n: integer;
  
  { Yield while waiting for data }
  n := async_read(socket, data);
  
  { Process data when ready }
  process_request(data, n);
  
  { Non-blocking write }
  async_write(socket, response)
end;

{ Multiple concurrent connections }
forall i := 1 to max_connections do
  handle_network_request(sockets[i])
```

**Benefits:**
- ✅ **High throughput:** Thousands of concurrent connections
- ✅ **Low latency:** Event-driven, no thread overhead
- ✅ **CPU efficiency:** No busy-waiting, tasks yield during I/O
- ✅ **Scalability:** Works across multiple cores

**Implementation Strategy:**
1. **I/O syscalls:** Tasks call `async_read()`, `async_write()`, etc.
2. **Event registration:** Scheduler registers file descriptors with epoll/kqueue
3. **Task blocking:** Tasks yield and wait in `wait_map`
4. **Event wake-up:** When I/O ready, scheduler wakes waiting tasks
5. **Multicore distribution:** I/O events distributed across cores

---

## 4. Virtual Clock for Determinism

### 4.1 Mayfly's Virtual Clock

```rust
// From mayfly/crates/scheduler/src/clock.rs
pub struct TickClock {
    start: Instant,
    virtual_time: Duration,
}

impl TickClock {
    pub fn now(&self) -> Instant {
        self.start + self.virtual_time
    }
    
    pub fn tick(&mut self, delta: Duration) {
        self.virtual_time += delta;
    }
}
```

**Key Features:**
- **Deterministic time:** Virtual time advances predictably
- **Sleep management:** Tasks sleep based on virtual time
- **Timeout handling:** Timeouts use virtual clock
- **Reproducibility:** Same input → same timing

### 4.2 Application to SuperPascal

**For Deterministic Parallel Algorithms:**

```pascal
{ Deterministic parallel execution with virtual time }
procedure parallel_simulation(
  var state: SimulationState;
  duration: real);
var
  start_time: real;
begin
  start_time := virtual_clock();
  
  parallel
    process
      { Task 1: Physics simulation }
      while virtual_clock() < start_time + duration do
        update_physics(state);
        sleep(0.016)  { 60 FPS }
    endprocess |
    process
      { Task 2: AI processing }
      while virtual_clock() < start_time + duration do
        update_ai(state);
        sleep(0.1)  { 10 Hz }
    endprocess
  endparallel
end;
```

**Benefits:**
- ✅ **Reproducibility:** Same input → same output (critical for scientific computing)
- ✅ **Debugging:** Predictable timing makes bugs easier to find
- ✅ **Testing:** Can test time-dependent behavior deterministically
- ✅ **Simulation:** Virtual time for game engines, simulations

**Implementation Strategy:**
1. **Virtual clock:** Scheduler maintains virtual time (separate from wall-clock)
2. **Sleep syscall:** Tasks sleep based on virtual time
3. **Time advancement:** Scheduler advances virtual time when no tasks ready
4. **Multicore sync:** All cores share same virtual clock (synchronized)

---

## 5. Task State Machine

### 5.1 Mayfly's State Management

```rust
// From mayfly/crates/scheduler/src/task.rs
pub enum TaskState {
    PendingDependencies,  // Waiting for dependencies
    Ready,                // Ready to run
    Running,              // Currently executing
    Finished(Reason),     // Completed
}

pub struct Task {
    tid: TaskId,
    pri: u8,
    state: TaskState,
    cancelled: bool,
    handle: JoinHandle<()>,
}
```

**Key Features:**
- **Explicit states:** Clear lifecycle management
- **State transitions:** Well-defined transitions (Ready → Running → Finished)
- **Cancellation:** Tasks can be cancelled gracefully
- **Completion tracking:** Know when tasks finish and why

### 5.2 Application to SuperPascal

**For Process Lifecycle Management:**

```pascal
{ Task state tracking in SuperPascal runtime }
type
  TaskState = (
    PendingDependencies,  { Waiting for dependencies }
    Ready,                { Ready to run }
    Running,              { Currently executing }
    BlockedOnIO,         { Waiting for I/O }
    BlockedOnChannel,    { Waiting for channel }
    Finished             { Completed }
  );

  Task = record
    id: TaskId;
    priority: TaskPriority;
    state: TaskState;
    dependencies: set of TaskId;
    dependents: set of TaskId;
  end;
```

**Benefits:**
- ✅ **Debugging:** Can inspect task state at any time
- ✅ **Monitoring:** Track task lifecycle for performance analysis
- ✅ **Cancellation:** Gracefully cancel long-running tasks
- ✅ **Error handling:** Know why tasks finished (success, error, cancelled)

**Implementation Strategy:**
1. **State tracking:** Runtime tracks state for each process/task
2. **State transitions:** Explicit transitions at yield points
3. **State queries:** Tasks can query their own state
4. **Multicore visibility:** State visible across cores (for debugging)

---

## 6. System Call Interface

### 6.1 Mayfly's Syscall Model

```rust
// From mayfly/crates/scheduler/src/task.rs
pub struct TaskContext {
    tid: TaskId,
    syscall_tx: Sender<(TaskId, SystemCall)>,
}

impl TaskContext {
    pub fn syscall(&self, call: SystemCall) {
        self.syscall_tx.send((self.tid, call)).unwrap();
        may::coroutine::yield_now();  // Yield to scheduler
    }
}

// System calls
pub enum SystemCall {
    Yield,           // Voluntary yield
    Sleep(Duration), // Sleep for duration
    WaitIO(u64),     // Wait for I/O event
    Join(TaskId),    // Wait for task completion
}
```

**Key Features:**
- **Structured communication:** Tasks communicate with scheduler via syscalls
- **Non-blocking:** Syscalls return immediately, task yields
- **Type-safe:** Enum-based syscalls prevent errors
- **Extensible:** Easy to add new syscalls

### 6.2 Application to SuperPascal

**For Task-Scheduler Communication:**

```pascal
{ System call interface }
type
  SystemCall = (
    SysYield,              { Voluntary yield }
    SysSleep(duration),    { Sleep for duration }
    SysWaitIO(io_id),     { Wait for I/O }
    SysWaitChannel(ch),    { Wait for channel }
    SysJoin(task_id),      { Wait for task }
    SysCancel(task_id)     { Cancel task }
  );

procedure syscall(call: SystemCall);
begin
  { Send syscall to scheduler }
  scheduler_send(current_task_id(), call);
  
  { Yield to scheduler }
  yield()
end;

{ Usage in tasks }
procedure worker_task();
begin
  { Do work }
  compute();
  
  { Yield to other tasks }
  syscall(SysYield);
  
  { Sleep for 100ms }
  syscall(SysSleep(0.1));
  
  { Continue work }
  compute()
end;
```

**Benefits:**
- ✅ **Structured control:** Clear interface between tasks and scheduler
- ✅ **Non-blocking:** Tasks yield, scheduler handles syscalls
- ✅ **Extensibility:** Easy to add new syscalls (e.g., `SysLock`, `SysUnlock`)
- ✅ **Type safety:** Compile-time checking of syscall parameters

**Implementation Strategy:**
1. **Syscall channel:** Tasks send syscalls to scheduler via channel
2. **Scheduler handling:** Scheduler processes syscalls in main loop
3. **Task yielding:** Tasks yield after sending syscall
4. **Multicore distribution:** Syscalls distributed across cores

---

## 7. Multicore Architecture

### 7.1 Hybrid Approach: SuperPascal + Mayfly Patterns

**For Raspberry Pi 5 (4 Cortex-A76 cores):**

```
┌─────────────────────────────────────────────────────────┐
│              SuperPascal Runtime (Multicore)            │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐│
│  │ Core 0   │  │ Core 1   │  │ Core 2   │  │ Core 3   ││
│  │ Scheduler│  │ Scheduler│  │ Scheduler│  │ Scheduler││
│  ├──────────┤  ├──────────┤  ├──────────┤  ├──────────┤│
│  │ Ready    │  │ Ready    │  │ Ready    │  │ Ready    ││
│  │ Queue    │  │ Queue    │  │ Queue    │  │ Queue    ││
│  │ (Priority)│ │ (Priority)│ │ (Priority)│ │ (Priority)││
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘│
│       │              │              │              │      │
│       └──────────────┴──────────────┴──────────────┘      │
│                    │                                      │
│              Work-Stealing                                │
│                    │                                      │
│  ┌──────────────────────────────────────────────────┐   │
│  │         Global System Queue (Priority 0)          │   │
│  └──────────────────────────────────────────────────┘   │
│                    │                                      │
│  ┌──────────────────────────────────────────────────┐   │
│  │         I/O Event Loop (epoll/kqueue)           │   │
│  └──────────────────────────────────────────────────┘   │
│                    │                                      │
│  ┌──────────────────────────────────────────────────┐   │
│  │         Dependency Resolver (DAG)                │   │
│  └──────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────┘
```

**Architecture Components:**

1. **Per-Core Schedulers:**
   - Each core has its own priority queue
   - Local task execution
   - Work-stealing from other cores

2. **Global System Queue:**
   - High-priority system tasks
   - Can run on any core
   - Load-balanced across cores

3. **I/O Event Loop:**
   - Single event loop (or per-core with load balancing)
   - Registers file descriptors
   - Wakes waiting tasks

4. **Dependency Resolver:**
   - Tracks task dependencies
   - Resolves dependencies when tasks complete
   - Distributes ready tasks to cores

### 7.2 Implementation Strategy

**Phase 1: Single-Core (Historical SuperPascal Model)**
```pascal
{ Minimal context switching }
procedure activate(b, s, p: integer);
begin
  st[s] := ready;
  ready := s
end;
```

**Phase 2: Multi-Core (Mayfly-Inspired)**
```rust
// Per-core scheduler
struct CoreScheduler {
    ready_queue: PriorityQueue<Task>,
    work_stealer: WorkStealer<Task>,
}

// Work-stealing
impl CoreScheduler {
    fn steal_work(&self) -> Option<Task> {
        // Try to steal from other cores
        for other_core in &self.other_cores {
            if let Some(task) = other_core.ready_queue.steal() {
                return Some(task);
            }
        }
        None
    }
}
```

**Phase 3: Hybrid (Best of Both)**
- **Single-core systems (Z80, 65C02):** Use SuperPascal's minimal model
- **Multi-core systems (Raspberry Pi 5):** Use Mayfly's work-stealing
- **Unified API:** Same language constructs, different runtime

---

## 8. Specific Lessons for SuperPascal

### 8.1 Priority Queues for Real-Time Systems

**Lesson:** Priority-based scheduling enables real-time responsiveness.

**Application:**
- **System tasks:** Interrupt handlers, I/O drivers (priority 0)
- **User tasks:** Application code (priority 10+)
- **Background tasks:** Low-priority work (priority 20+)

**Implementation:**
```pascal
{ Priority-based parallel execution }
parallel
  process priority 0
    { Real-time: Handle interrupts }
    handle_interrupts()
  endprocess |
  process priority 10
    { Normal: User application }
    user_code()
  endprocess |
  process priority 20
    { Background: Garbage collection }
    gc()
  endprocess
endparallel
```

### 8.2 DAG Dependencies for Parallel Algorithms

**Lesson:** Dependency tracking enables optimal parallel execution.

**Application:**
- **Matrix operations:** Cell computation depends on row/column
- **Graph algorithms:** Node processing depends on neighbors
- **Pipeline processing:** Stage N depends on stage N-1

**Implementation:**
```pascal
{ Compiler generates dependency graph }
forall i := 1 to n do
  forall j := 1 to n do
    { Automatically track dependencies }
    c[i, j] := compute(a[i, *], b[*, j])
    { Scheduler ensures dependencies are met }
```

### 8.3 Async I/O for High Throughput

**Lesson:** Event-driven I/O enables thousands of concurrent operations.

**Application:**
- **Network servers:** Handle thousands of connections
- **File I/O:** Non-blocking file operations
- **Graphics:** Async rendering pipeline

**Implementation:**
```pascal
{ Async I/O with event-driven scheduling }
procedure handle_connection(socket: TcpSocket);
begin
  { Non-blocking read }
  var data: array [1..1024] of byte;
  n := async_read(socket, data);  { Yields if no data }
  
  { Process when ready }
  process_request(data, n);
  
  { Non-blocking write }
  async_write(socket, response)  { Yields if buffer full }
end;
```

### 8.4 Virtual Clock for Determinism

**Lesson:** Virtual time enables reproducible parallel execution.

**Application:**
- **Scientific computing:** Reproducible simulations
- **Game engines:** Deterministic physics
- **Testing:** Predictable timing for tests

**Implementation:**
```pascal
{ Virtual clock for deterministic execution }
procedure parallel_simulation();
begin
  var start := virtual_time();
  
  parallel
    process
      while virtual_time() < start + 1.0 do
        update_physics();
        sleep(0.016)  { 60 FPS }
    endprocess |
    process
      while virtual_time() < start + 1.0 do
        update_ai();
        sleep(0.1)  { 10 Hz }
    endprocess
  endparallel
end;
```

---

## 9. Implementation Roadmap

### 9.1 Phase 1: Single-Core (Current)

**Status:** ✅ Implemented (Historical SuperPascal model)

- Minimal context switching (3 words)
- FIFO ready queue
- Synchronous channels
- Deterministic execution

**Targets:** Z80, 65C02, 65C816 (single-core)

### 9.2 Phase 2: Multi-Core Foundation

**Status:** 🔄 Planned

**Components:**
1. **Priority Queue:** Replace FIFO with priority queue
2. **Task State Machine:** Add explicit state tracking
3. **Syscall Interface:** Add structured task-scheduler communication
4. **Virtual Clock:** Add deterministic time management

**Targets:** FoenixA2560M (single-core, but prepare for multi-core)

### 9.3 Phase 3: Multi-Core Scheduler

**Status:** 🔄 Planned

**Components:**
1. **Per-Core Schedulers:** One scheduler per CPU core
2. **Work-Stealing:** Idle cores steal from busy cores
3. **Global System Queue:** High-priority tasks on any core
4. **I/O Event Loop:** Async I/O with epoll/kqueue

**Targets:** Raspberry Pi 5 (4 cores), future multi-core systems

### 9.4 Phase 4: Advanced Features

**Status:** 🔄 Planned

**Components:**
1. **DAG Dependencies:** Automatic dependency tracking
2. **Priority Inheritance:** Dependencies boost priority
3. **Cancellation:** Graceful task cancellation
4. **Monitoring:** Task lifecycle tracking

**Targets:** All multi-core systems

---

## 10. Code Examples

### 10.1 Priority-Based Parallel Execution

```pascal
{ Real-time system with priorities }
program realtime_system;
begin
  parallel
    { High priority: Interrupt handling }
    process priority 0
      while true do
        handle_interrupts();
        yield()
      end
    endprocess |
    
    { Normal priority: User application }
    process priority 10
      user_application()
    endprocess |
    
    { Low priority: Background tasks }
    process priority 20
      while true do
        garbage_collect();
        sleep(1.0)
      end
    endprocess
  endparallel
end.
```

### 10.2 DAG-Based Parallel Algorithm

```pascal
{ Matrix multiplication with dependencies }
procedure parallel_matrix_multiply(
  var a, b, c: matrix;
  n: integer);
var
  tasks: array [1..n, 1..n] of TaskId;
begin
  { Create tasks with row/column dependencies }
  forall i := 1 to n do
    forall j := 1 to n do
      tasks[i, j] := spawn_with_deps(
        [row_ready[i], col_ready[j]],
        procedure
        begin
          var sum := 0;
          for k := 1 to n do
            sum := sum + a[i, k] * b[k, j];
          c[i, j] := sum
        end
      )
end;
```

### 10.3 Async I/O Server

```pascal
{ High-throughput network server }
procedure network_server();
var
  listener: TcpListener;
  socket: TcpSocket;
begin
  listener := TcpListener::bind("0.0.0.0:8080");
  
  while true do
    socket := listener.accept();  { Non-blocking }
    
    { Spawn handler for each connection }
    spawn priority 10, procedure
      handle_connection(socket)
    end
  end
end;

procedure handle_connection(socket: TcpSocket);
var
  data: array [1..1024] of byte;
  n: integer;
begin
  { Async read (yields if no data) }
  n := async_read(socket, data);
  
  { Process request }
  var response := process_request(data, n);
  
  { Async write (yields if buffer full) }
  async_write(socket, response)
end;
```

---

## 11. Performance Considerations

### 11.1 Context Switching Overhead

**Historical SuperPascal:** ~12 bytes per process (3 words)
**Mayfly:** ~100-200 bytes per task (metadata)
**May:** ~1-8 KB per coroutine (full stack)

**Recommendation:**
- **Single-core:** Use SuperPascal's minimal model
- **Multi-core:** Use Mayfly's metadata model (stack separate)
- **Hybrid:** Minimal for simple tasks, full stack for complex

### 11.2 Memory Locality

**Lesson from Mayfly:** Per-core queues improve cache locality.

**Application:**
- **Per-core ready queues:** Tasks stay on same core (cache-friendly)
- **Work-stealing:** Only when core is idle (minimal overhead)
- **NUMA awareness:** On NUMA systems, prefer local memory

### 11.3 I/O Efficiency

**Lesson from Mayfly:** Event-driven I/O scales to thousands of operations.

**Application:**
- **epoll/kqueue:** O(1) I/O readiness (vs O(n) polling)
- **Non-blocking:** Tasks yield during I/O (no thread overhead)
- **Batching:** Process multiple I/O events per iteration

---

## 12. Conclusion

Mayfly's scheduler provides **essential patterns** for SuperPascal to leverage modern multicore processors:

1. ✅ **Priority-based scheduling** → Real-time responsiveness
2. ✅ **DAG dependencies** → Optimal parallel algorithms
3. ✅ **Async I/O** → High-throughput network/file operations
4. ✅ **Virtual clock** → Deterministic execution
5. ✅ **Task state machine** → Lifecycle management
6. ✅ **Syscall interface** → Structured control

**Implementation Strategy:**
- **Single-core systems:** Use SuperPascal's minimal model (3 words)
- **Multi-core systems:** Use Mayfly's patterns (priority queues, work-stealing)
- **Unified API:** Same language constructs, adaptive runtime

**Next Steps:**
1. Implement priority queues for single-core (Phase 2)
2. Add multi-core scheduler with work-stealing (Phase 3)
3. Integrate DAG dependencies for parallel algorithms (Phase 4)
4. Add async I/O for high-throughput systems (Phase 4)

The combination of **SuperPascal's minimalism** (for single-core) and **Mayfly's sophistication** (for multi-core) creates a **hybrid runtime** that adapts to the target platform while maintaining the same language semantics.

---

## References

- **Mayfly Scheduler:** `/Users/casibbald/Workspace/microscaler/mayfly/crates/scheduler/`
- **Historical SuperPascal:** `CONCURRENCY_ANALYSIS.md`
- **Comparison:** `SUPERPASCAL_VS_MAY_COMPARISON.md`
- **Target Architectures:** `TARGET_ARCHITECTURES.md`

