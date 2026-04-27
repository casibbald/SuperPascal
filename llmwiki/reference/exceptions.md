# Exceptions Model

- Status: partially-verified
- Source docs: `languageSpecification/07_Exceptions.md`, `languageSpecification/05_ABI_Concepts.md`
- Code anchors: `languageSpecification/07_Exceptions.md`

## Exception handling model

SuperPascal uses a stack-based exception model:
- `try..except..end` blocks
- `try..finally..end` blocks
- `raise` statements
- `on E: TypeError do` — typed exception handling

## Exception frame chain

At the ABI level:
- `IY` register maintains the exception frame chain
- Each `try` block pushes a frame onto the IY chain
- Frame contains: previous IY value, handler address, cleanup code pointer
- When exception raised: walk the IY chain looking for matching handler
- When unwind completes: restore IY to previous value

## Exception types

- Base exception type
- Custom exception classes (derived from base)
- Typed exception handling: `on E: SomeType do`

## Exception intrinsics

Platform-specific exception intrinsics documented in:
- `languageSpecification/05_ABI_Concepts.md` — ABI-level exception frame management
- `platforms/ZealZ80/intrinsics/` — platform-specific exception handling

## Compliance

Exception handling is part of the compliance test suite:
- Test exception propagation
- Test exception unwinding
- Test typed exception handling
- Test finally blocks
