# backend-x86_64: Fix List

## Compile errors

### [x] t1. CodeGenerator missing `temp_counter` field
- Line 607-608: uses `self.temp_counter` but `CodeGenerator` doesn't have it
- `temp_counter` exists in `IRBuilder` (ir/src/lib.rs:181) but NOT in `CodeGenerator` (lib.rs:140)
- Fix: either add `temp_counter: usize` to `CodeGenerator`, or remove/refactor its usage

### [ ] t2. IR struct tests don't compile (lines 870-922)
IR structs evolved; test code uses outdated field names:

| Old test field       | Current IR field(s)                         |
|---------------------|---------------------------------------------|
| `Function { blocks: [...] }` | `Function { blocks: [...], entry_block: String, params: Vec<(String, Type)>, return_type: Option<Type> }` |
| `BasicBlock { label, instructions }` | `BasicBlock { label, instructions, successors: Vec<String> }` |
| `Instruction { opcode, operands }` | `Instruction { opcode, operands, span: Option<Span> }` |
| `Program { functions }` | `Program { functions, globals: Vec<(String, Type)> }` |

Affected tests: `test_generate_simple_function`, `test_generate_with_jump`

## Warnings

### [ ] t3. Remove unused variable `count` in errors/src/lib.rs:316
- `let count = self.errors.len();` → prefix with `_` or remove

### [ ] t4. Unused variable `reg` in backend-x86_64:285,581
- `Value::Register(reg)` pattern matches but `reg` is unused → `_reg`

### [ ] t5. Irrefutable `if let` patterns (lines 708, 723)
- `if let MemoryAddress::BaseIndex { ... } = addr` is always true → change to `let`

### [ ] t6. Unnecessary `mut` in test_parse_register (line 862)
- `let mut codegen = CodeGenerator::new();` → `let codegen = ...`

## Steps to fix
1. Fix t1: add `temp_counter` to `CodeGenerator`
2. Fix t2: update test structs to match current IR
3. Clean up warnings t3-t6
4. Run `cargo test -p backend-x86_64` to verify
