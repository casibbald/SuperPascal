# 📘 ZealPascal Language Specification v0.7 — Exceptions & Advanced Features

## Structured Error Handling, Exception ABI, and Selected Advanced Language Constructs

This document defines the **exception model** and selected **advanced language features** for ZealPascal.

It builds on:

* v0.3 Semantic Rules
* v0.4 ABI & Codegen
* v0.5 / StdLib Rewrite

And is designed to remain:

* Educationally friendly
* Z80-feasible
* ABI-compatible with existing code

---

# 1. Goals for Exception Handling in ZealPascal

1. **Structured error handling** for real-world programs.
2. **Simple mental model** suitable for teaching.
3. **Low runtime overhead when unused**.
4. **Coexistence with existing error-return idioms**.
5. Clear **interaction with the Game Engine**, scripting, and IO.

We adopt a **setjmp/longjmp-style stack unwinding model** tailored for Z80.

---

# 2. Language Syntax

Exceptions use a Pascal-style syntax similar to Delphi/Object Pascal:

```pascal
try
  DangerousOperation;
except
  HandleError;
end;

try
  UseResource;
finally
  CleanupResource;
end;
```

We also support a basic `raise` form:

```pascal
raise;                // re-raises current exception
raise EError.Create;  // future typed exceptions
```

In the initial implementation, `raise;` rethrows the last exception; `raise <expr>` may be restricted or mapped to simple error signaling.

---

# 3. Semantic Model

## 3.1 Exception Frames

Each `try` block creates an **exception frame** that is pushed on a chain referenced from `IY`.

Frame contents (conceptual):

```
PrevFramePtr  : 2 bytes
HandlerAddr   : 2 bytes
SavedSP       : 2 bytes
SavedIX       : 2 bytes
Flags         : 1 byte
Reserved      : 1 byte
```

## 3.2 Raising an Exception

When `raise` is called or a runtime error occurs:

1. Runtime reads current frame from `IY`.
2. Restores `SP` and `IX` from frame.
3. Jumps to `HandlerAddr`.
4. `try..except` handlers execute from that point.

If no frame exists, program aborts with a clear error message.

## 3.3 `try..except` Semantics

* Code in the `try` block runs normally.
* If no exception occurs, `except` block is skipped.
* If an exception occurs, control jumps to `except` block.
* After `except` finishes, execution continues after the whole `try..except` statement.

## 3.4 `try..finally` Semantics

* `try` block always runs first.
* `finally` block **always executes**, whether or not an exception occurred.
* If an exception is active when entering finally, it is re-raised after finally completes.

---

# 4. Interaction With `SysUtils_Lite`

The `SysUtils_Lite` unit provides a basic error object and raising function:

```pascal
type
  EError = class
    Code: Integer;
    Msg: string;
  end;

procedure RaiseError(Code: Integer; const Msg: string);
```

Semantics:

* In non-exception builds, `RaiseError` may `Halt` with a message.
* In exception-enabled builds, `RaiseError` triggers `raise`.

Future versions may allow typed exceptions (inheriting from `EError`).

---

# 5. ABI & Codegen for Exceptions

## 5.1 Compiler Responsibilities

At each `try` block entry:

* Allocate an exception frame on stack or in a reserved area.
* Link it into frame chain via `IY`.

At each `try` block exit (normal):

* Unlink its frame from the chain.

On `raise`:

* Jump to runtime helper that performs unwinding.

## 5.2 Cost Model

* No cost for functions without `try`/`raise`.
* Small cost per `try` for pushing frame.
* Larger cost only when an exception is actually raised.

---

# 6. Engine & Scripting Integration

## 6.1 Game Loop

The main game loop may be wrapped in a top-level `try..except` to prevent crashes:

```pascal
try
  Game.Run;
except
  Engine_HandleFatalError;
end;
```

## 6.2 Scripting Layer

Script calls into Pascal may:

* Catch exceptions and convert to script-level errors.
* Or propagate them to an engine-level handler.

## 6.3 IO & Save/Load

File operations may use exceptions for catastrophic errors while returning error codes for expected conditions (e.g., missing save file).

---

# 7. Advanced Language Features (Selected)

This section sketches advanced but **optional** features compatible with the exception model.

## 7.1 Assertions

```pascal
procedure Assert(Condition: Boolean; const Msg: string = '');
```

* In debug builds: raises an exception if `Condition = false`.
* In release builds: may compile out.

## 7.2 Contracts (Future Idea)

Potential syntax for pre/post-conditions:

```pascal
procedure Foo(x: Integer);
requires x > 0;
ensures Result >= x;
```

These may compile down to assertions and exceptions.

## 7.3 Defer-Like Helpers Using `try..finally`

Recommended teaching pattern:

```pascal
Acquire;
try
  Use;
finally
  Release;
end;
```

This encourages resource-safe programming.

---

# 8. Educational Considerations

Exceptions should be **optional** in early teaching stages.

* Stage A: No exceptions; use result codes.
* Stage B: Introduce `try..finally` for cleanup.
* Stage C: Introduce `try..except` for game engine and IO.

Error messages must be clear:

> "The program tried to read beyond the end of an array in `UpdatePlayer`. This raised an exception."

---

# 9. Implementation Notes

* Exception runtime must be small & efficient.
* Compilers may offer a flag to **disable exceptions entirely** for ultra-low-cost builds.
* Debugger integration: show current exception, frame chain, and source location.

---

# ✔️ ZealPascal Exceptions & Advanced Features v0.7 — Draft Complete

This document defines:

* Syntax for `try..except`, `try..finally`, and `raise`
* Exception frame model and ABI interaction
* Runtime responsibilities and cost model
* Integration with SysUtils_Lite, engine, and scripting
* Teaching strategy for introducing exceptions

It completes the first pass of advanced language features for ZealPascal, ready for refinement and eventual stabilization.
