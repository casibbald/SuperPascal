# SuperPascal Compiler - Z80 Backend: MUL/DIV/MOD Implementation

## Overview
Complete the Z80 backend's MUL, DIV, MOD code generation with proper software-implemented arithmetic (Z80 has no native multiply/divide).

## Tasks

### Task 1: Complete Z80 MUL (Shift-and-Add Loop)

**Current state:** Stub in `generate_mul()` - loads src1→HL, src2→DE, sets B=8 counter, creates labels but loop body is just TODOs.

**Implementation:**
1. Remove TODO comment at loop_body
2. Before each iteration, test LSB of HL (the multiplier being shifted)
3. If LSB is set, add DE (multiplicand) to BC (accumulator)
4. Shift BC left by 1 bit
5. Shift HL left by 1 bit
6. Decrement B counter, jump back if not zero
7. Result is in BC (low byte = result)

**Z80 instructions needed:**
- `AND A` (to set flags based on A, then use `RRCA` or `BIT 0, L`)
- `JP Z` / `JP NZ` for conditional jumps
- `ADD HL, HL` for shift left
- `ADD BC, DE` for add to accumulator
- `DJNZ` for counter decrement + loop

**Verification:**
- Run `cargo test --workspace` - all 689 must pass
- Check generated instructions contain mul_entry/mul_body/mul_exit labels
- Verify no TODO comments remain in `generate_mul`

**Test to add:**
```rust
#[test]
fn test_mul_instruction_generation() {
    // Create IR: result = src1 * src2
    // Verify codegen produces shift-and-add loop
}
```

**Acceptance:** No TODOs, generates working shift-add loop, tests pass.

---

### Task 2: Complete Z80 DIV (Shift-and-Subtract Loop)

**Current state:** Stub in `generate_div()` - loads src1→A, src2→B, but loop body is TODO.

**Implementation:**
1. Remove TODO comment
2. Implement shift-and-subtract algorithm:
   - Load src1 (dividend) into A
   - Load src2 (divisor) into B
   - Use C as quotient accumulator (start at 0)
   - Shift dividend left, compare with divisor
   - If dividend >= divisor, subtract divisor, set quotient bit
   - Shift quotient left
   - Repeat 8 times
3. Store quotient (from A) to destination

**Z80 instructions needed:**
- `SLA A` for shift left accumulator
- `CP` / `SUB` for comparison and subtraction
- `CCF` + `ADC A, 0` for quotient bit set
- `RLC C` for quotient shift

**Verification:**
- Run `cargo test --workspace` - all 689 must pass
- Check generated instructions contain division loop
- Verify no TODO comments remain in `generate_div`

**Test to add:**
```rust
#[test]
fn test_div_instruction_generation() {
    // Create IR: result = src1 / src2
    // Verify codegen produces shift-and-subtract loop
}
```

**Acceptance:** No TODOs, generates working shift-and-subtract loop, tests pass.

---

### Task 3: Complete Z80 MOD (Remainder Calculation)

**Current state:** Stub in `generate_mod()` - computes quotient via DIV path, but remainder calculation after quotient is TODO.

**Implementation:**
1. Keep existing quotient computation
2. After quotient is in A:
   - Push quotient to stack
   - Reload original src1 (already on stack from save)
   - Multiply quotient × src2 using shift-and-add (inline, since MUL now exists)
   - Subtract result from original src1
   - Store remainder to destination

**Alternative:** Use the identity: `MOD = src1 - (quotient × src2)`

**Z80 instructions needed:**
- `PUSH`/`POP` for saving/restoring values
- Reuse MUL logic (or inline it)
- `SUB` / `SBC` for subtraction

**Verification:**
- Run `cargo test --workspace` - all 689 must pass
- Check generated instructions for MOD path
- Verify no TODO comments remain in `generate_mod`

**Test to add:**
```rust
#[test]
fn test_mod_instruction_generation() {
    // Create IR: result = src1 % src2
    // Verify codegen produces quotient-compute + multiply-subtract
}
```

**Acceptance:** No TODOs, generates working MOD computation, tests pass.

---

### Task 4: Add Backend Tests

**Current state:** No tests for MUL/DIV/MOD code generation in backend-zealz80.

**Implementation:**
1. Add `test_mul_instruction_generation()` - verify MUL produces correct instructions
2. Add `test_div_instruction_generation()` - verify DIV produces correct instructions
3. Add `test_mod_instruction_generation()` - verify MOD produces correct instructions
4. Each test should:
   - Create a minimal IR program with the operation
   - Run codegen
   - Assert generated instructions contain expected labels/patterns

**Verification:**
- Run `cargo test --workspace` - all 689+3 must pass (692)

**Acceptance:** 3 new tests added, all passing, cover MUL/DIV/MOD code paths.

---

### Task 5: Cleanup & Verify No Regressions

**Current state:** Some TODOs in other areas of backend-zealz80.

**Implementation:**
1. Search for remaining TODO comments in `backend-zealz80/src/lib.rs`
2. For MUL/DIV/MOD area only - ensure no TODOs remain
3. Other TODOs can be left for later iterations (not in scope)
4. Run full test suite: `cargo test --workspace`

**Verification:**
- `grep -n "TODO" backend-zealz80/src/lib.rs` - no TODOs in Mul/Div/Mod functions
- All 692 tests pass (689 original + 3 new)

**Acceptance:** Clean MUL/DIV/MOD, full test suite green.

---

## PRD Structure

**Each task is one commit:**
- Implement → Test → Commit → Wiki update → Next task

**Task 1** → MUL complete → commit 1
**Task 2** → DIV complete → commit 2
**Task 3** → MOD complete → commit 3
**Task 4** → Tests added → commit 4
**Task 5** → Cleanup verified → commit 5 (or merge with previous)

## Completion Criteria
- [ ] MUL generates working shift-and-add loop
- [ ] DIV generates working shift-and-subtract loop
- [ ] MOD computes remainder correctly
- [ ] 3 new tests added for codegen verification
- [ ] All 692 tests pass
- [ ] No TODOs in MUL/DIV/MOD handler functions
