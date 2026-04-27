# ABI and Calling Convention

- Status: partially-verified
- Source docs: `languageSpecification/05_ABI_Concepts.md`, `platforms/ZealZ80/ABI.md`, `docs/08_ZealPascalLanguageSpecificationABIcodeGenerationRules.md`
- Code anchors: `languageSpecification/05_ABI_Concepts.md`, `platforms/ZealZ80/ABI.md`

## Z80 Calling Convention

### Register usage
- `IX` — frame pointer (stack frame base)
- `IY` — exception frame chain
- Parameter passing: stack-based via IX frame pointer

### Function prologue/epilogue
```
Prologue:
  push ix
  ld ix, sp

Epilogue:
  ld sp, ix
  pop ix
  ret
```

### Parameter passing
- Small parameters (< 256 bytes): stack-based via IX offset
- Large parameters (> 256 bytes): hidden pointer parameter (caller allocates)
- Return values:
  - Small types: register
  - Large types (records, arrays): hidden pointer parameter

### Class layout
- Offset 0: vtable pointer
- Remaining offsets: instance fields in declaration order
- Method resolution: vtable lookup at runtime

### Record methods
- Lowered to free functions
- First parameter: self pointer (record instance)
- No vtable needed

## Exception frame chain
- `IY` register points to exception frame chain
- Each frame: [previous IY, handler address, cleanup code]
- Nested exceptions: linked list through IY chain

## Memory model
- Page-aware heap manager
- No garbage collection
- Predictable memory usage
- Manual memory management for dynamic allocations

## ABI versioning
- ABI spec is versioned (v0.4+)
- ABI stability is part of compliance testing
- Changes require spec update and compliance test update
