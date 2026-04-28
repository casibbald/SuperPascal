# Next Steps Plan - SuperPascal Development

**Last Updated:** 2025-01-01  
**Current Status:** ✅ Parser & Semantics complete, ❌ End-to-end compilation incomplete  
**Codebase Status:** ✅ Zero warnings, 502+ tests passing, 11,421 LOC parser, 2,914 LOC semantics

---

## 📊 Actual Current State (Verified 2025-01-01)

### ✅ What's Actually Done

| Component | LOC | Tests | Status |
|-----------|-----|-------|--------|
| Parser | 11,421 | 232 | ✅ Complete — expressions, statements, types, classes, declarations, for-in, with, try/except/raise, enums, properties, dynamic arrays |
| Semantics | 2,914 | 32 | ✅ Modularized — declarations, expressions, statements, types, feature checking, constants, lvalues |
| AST | 2,032 | 46 | ✅ Comprehensive — 57 node types including DynamicArrayType, AnonymousFunction, InterfaceType, HelperType, GenericParam |
| Lexer | ~1,500 | 73 | ✅ Complete |
| Preprocessor | 500+ | - | ✅ Implemented — `parser/src/directives.rs` with DirectiveEvaluator (IFDEF/IF/DEFINE/UNDEF/ELSE/ENDIF) |
| Runtime | ~1,273 | - | ✅ Arrays, closures, interfaces, variants |
| IR | 3,183 | 69 | ✅ All `build_*` methods declared |
| Driver | 441 | 0 | ⚠️ Pipeline orchestration exists but assembler is stubbed |

### ❌ What's Broken / Missing

1. **Assembler stubbed** — `instructions_to_bytes()` in `driver/src/compiler.rs` returns empty vec (`// TODO: Implement proper assembler`)
2. **Symbol offsets/sizes zero** — Object file symbols have `offset: 0` and `size: 0` (line 106-107)
3. **Zero driver tests** — `driver/src/compiler.rs` has 0 `#[test]` attributes
4. **IR→bytes never tested end-to-end** — No integration test goes through parse → sem → IR → bytes
5. **No multi-error recovery** — Parser likely stops at first error

---

## 🎯 Strategic Priorities (Revised Based on Actual State)

### Phase 1: Make the Compiler Actually Compile End-to-End (CRITICAL)
**Goal:** Go from source file → .zof object file with real machine code

#### 1. Implement Z80 Assembler (16-24 hours) ⭐ HIGHEST PRIORITY
**Why:** Everything else is built, but zero working binaries exist. This is the single biggest gap.

**Current State:**
- `Z80Instruction` enum exists in `backends/backend-zealz80/src/lib.rs`
- `instructions_to_bytes()` is a stub returning `vec![]`
- `CodeGenerator::generate()` returns instructions but they never get encoded

**Tasks:**
- [ ] Examine all `Z80Instruction` variants in `backends/backend-zealz80/src/lib.rs`
- [ ] Map each instruction variant to its Z80 opcode bytes
- [ ] Handle operands (registers, immediate values, addresses, labels)
- [ ] Update `instructions_to_bytes()` in `driver/src/compiler.rs`
- [ ] Fix symbol offset tracking in `compile_file()` (currently offset: 0 for all)

**Files to Modify:**
- `driver/src/compiler.rs` — `instructions_to_bytes()` method
- Potentially `backends/backend-zealz80/src/lib.rs` — if instruction encoding is better placed in backend

#### 2. Integration Tests - Full Pipeline (4-8 hours)
**Why:** Verify the pipeline actually works end-to-end

**Tasks:**
- [ ] Write test that compiles a simple `program Hello; begin WriteLn('Hello'); end.` to .zof
- [ ] Write test that verifies object file contains non-empty code section
- [ ] Write test for IR emission (`emit_ir`)
- [ ] Write test for assembly emission (`emit_assembly`)

**Files to Create/Modify:**
- `driver/tests/integration.rs` (new)
- Or add `#[cfg(test)]` module to `driver/src/compiler.rs`

#### 3. Fix Symbol Offsets (2-4 hours)
**Why:** Current object files have all symbol offsets at 0

**Tasks:**
- [ ] Track instruction byte lengths as they're encoded
- [ ] Update symbol `offset` field to actual position in code section
- [ ] Update symbol `size` field to function body byte length

---

### Phase 2: Parser Polish & Error Handling
**Goal:** Better developer experience

#### 4. Error Recovery - Source Snippets (2-4 hours)
**Why:** Shows code context around errors

**Tasks:**
- [ ] Add `source: String` field to `Parser` struct
- [ ] Store source in `Parser::new()` and `Parser::new_with_file()`
- [ ] Implement `get_source_snippet()` to extract lines around span
- [ ] Create `CodeSnippet` with line numbers and highlighting

**Files to Modify:**
- `parser/src/core.rs` (add source field)
- `parser/src/lib.rs` (implement get_source_snippet)

#### 5. Multi-Error Reporting (8-16 hours)
**Why:** Find multiple errors in one pass instead of stopping at first

**Tasks:**
- [ ] Add `recover_from_error()` method to `Parser`
- [ ] Implement sync points (skip to `;`, next declaration, BEGIN/END)
- [ ] Modify parsing functions to attempt recovery
- [ ] Collect multiple errors instead of stopping at first

**Files to Modify:**
- `parser/src/core.rs` (recovery logic)
- `parser/src/lib.rs` (integrate recovery)
- All parsing modules (add recovery points)

#### 6. AST Query System - Complete Coverage (4-8 hours)
**Why:** Needed for IDE features (go to definition, refactoring)

**Tasks:**
- [ ] Review all AST node types
- [ ] Implement `get_children()` for missing node types
- [ ] Add tests for each node type

**Files to Modify:**
- `parser/src/query.rs`

---

### Phase 3: Language Compliance (Lower Priority Now)
*The NEXT-STEPS.md from 2024-12-23 listed these as Phase 1-2. After verifying actual codebase state, these features are already implemented in the parser/semantics. The real blocker is end-to-end compilation.*

#### 7. Class Properties & Variables (4-8 hours)
**Status:** ✅ Likely done — parser has `parser/src/properties.rs` and class parsing in `parser/src/classes.rs`
**Action:** Verify with tests, mark complete

#### 8. Dynamic Arrays (8-16 hours)
**Status:** ✅ Likely done — AST has `DynamicArrayType`, parser has type parsing
**Action:** Verify with tests, mark complete

#### 9. Generics (40-60 hours)
**Status:** Partial — AST has `GenericParam` but may not be fully implemented
**Action:** Audit parser for generic type/routine parsing

#### 10. Preprocessor Directives (24-40 hours)
**Status:** ✅ Done — `parser/src/directives.rs` exists with DirectiveEvaluator
**Action:** Verify completeness, add tests

---

### Phase 4: Runtime & Standard Library
**Goal:** Enable real-world Pascal programs

#### 11. Core I/O Routines
**Tasks:**
- [ ] WriteLn, ReadLn implementations in runtime
- [ ] File I/O support
- [ ] String manipulation functions

#### 12. Math Library
**Tasks:**
- [ ] Standard math functions (sin, cos, sqrt, etc.)
- [ ] Random number generation

---

### Phase 5: Backend Expansion
**Goal:** Support multiple target platforms

#### 13. Z80 Backend Improvements
**Tasks:**
- [ ] Complete assembler (see Phase 1)
- [ ] Register allocation improvements
- [ ] Peephole optimizations

#### 14. Other Platforms
**Tasks:**
- [ ] 65C816 backend (platforms/Foenix65C816)
- [ ] 68K backend (platforms/)
- [ ] Commander X16 backend (platforms/CommanderX16)

#### 15. x86_64 Backend (`backend-x86_64`) — ⭐ NEW
**Goal:** Compile SuperPascal → x86_64 ELF64 binary (runnable on Linux locally)
**Rationale:** Z80 requires emulator to test. x86_64 binary can run directly on developer machine for rapid iteration.

**Step 1 — Crate scaffolding**
- Create `crates/compiler-rs/backends/backend-x86_64/Cargo.toml`
- Add `backend-x86_64` to workspace `members` in `Cargo.toml`
- Define module structure: `lib.rs` with types and code generator

**Step 2 — Code generator core**
- Define `CodeGenerator` struct with state: current function, local stack offset, temp counter
- Implement `generate(program: &Program) -> Vec<AssemblyInstruction>`
- Implement function prologue: `push rbp; mov rbp, rsp; sub rsp, local_size`
- Implement function epilogue: `mov rsp, rbp; pop rbp; ret`

**Step 3 — IR opcode → x86_64 translation**
Map each IR opcode to x86_64 instructions:
- `Mov` → `mov` (register, register; register, immediate; register, [mem])
- `Add/Sub` → `add r64, src` / `sub r64, src`
- `Mul/Div` → `imul r64, src` / `idiv` (dividend in RAX)
- `Cmp` → `cmp r64, src`
- `Jump` → `jmp label`
- `CJump` → `cmp` + `je/jne/jl/jle/jg/jge label`
- `Call` → `call function_name` (cdecl: RDI, RSI, RDX, RCX, R8, R9 for params; RAX for return)
- `Ret` → `ret`
- `Load` → `mov r64, [mem]`
- `Store` → `mov [mem], r64`
- `Push` → `push r64`
- `Pop` → `pop r64`

**Step 4 — Register allocation**
- Map IR temps (T0, T1...) to x86_64 callee-saved and volatile registers
- Register pool: RAX, RCX, RDX, RSI, RDI, R8, R9, R10, R11 (callee-volatile) + RBX, RBP, R12-R15 (callee-saved)
- Implement spilling to stack frame when register pressure exceeds 9 registers
- Handle value-to-register loading and register-to-value storing

**Step 5 — Assembly text emission**
- Define `AssemblyInstruction` enum: `Label`, `Nop`, `Comment`, `Mov`, `Add`, `Sub`, `Imul`, `Idiv`, `Cmp`, `Jmp`, `Jcc`, `Call`, `Ret`, `Push`, `Pop`, `Lea`, `Nop`
- Implement `Display` for each variant with Intel syntax, proper operand formatting
- Handle 32-bit vs 64-bit operand sizing (byte, word, dword, qword)
- Handle RIP-relative addressing for external symbols

**Step 6 — ELF64 object file generation**
- Generate a `.s` assembly text file or directly emit ELF64 binary
- ELF64 structure: ELF header (64 bytes) → program header table → sections
- Sections: `.text` (code), `.data` (initialized globals), `.bss` (zero-initialized), `.symtab`, `.strtab`
- Section alignment: 16-byte for code
- Symbol table: export function names as global symbols

**Step 7 — Linker integration in driver**
- Add `x86_64` target variant to `TargetPlatform` enum
- In `Compiler::compile_file()`, detect target and choose codegen path
- For x86_64: generate `.s` file → invoke `as` → generate `.o` → invoke `ld` → produce final ELF64
- Support `--target=x86_64` CLI flag or output extension detection (`.elf`, `.bin`, `.o`)

**Step 8 — Runtime stubs**
- Minimal `_start` entry point: call `main`, exit via `syscall 60` (exit_group)
- For I/O: link against libc (`printf`, `scanf`, `fopen`, etc.)
- Alternative: provide libc-free syscall wrappers for write/read/exit/open/close
- Handle C calling convention (cdecl): caller saves RAX, RCX, RDX, RDI, RSI, R8-R11

**Step 9 — End-to-end test**
- Compile a Pascal program with: `if/else`, `for` loop, `function`, `local variables`, `WriteLn`
- Run the produced ELF64 binary directly on Linux
- Verify correct stdout output and exit code
- Compare against expected output

---

## 📊 Implementation Roadmap (Revised)

### This Week: Phase 1
1. Implement Z80 assembler (`instructions_to_bytes`)
2. Fix symbol offset tracking
3. Write integration tests for full pipeline

**Expected Outcome:** Can compile `program Foo; begin ... end.` → `.zof` object file with real machine code

### Next Week: Phase 2
4. Source snippets for error messages
5. Multi-error recovery
6. AST query system completeness

### Following Weeks: Phase 4
7. Standard library (I/O, strings, math)
8. More integration tests with real Pascal programs

---

## 🎯 Success Metrics

### Critical (Must Do)
- ✅ Zero compiler warnings (achieved)
- ✅ 500+ tests passing (achieved)
- ❌ End-to-end compilation: source → .zof (NOT achieved — blocker)
- ❌ Driver has integration tests (0 tests currently)
- ❌ Symbol offsets are correct (currently all 0)

### Should Do
- Error messages include source snippets
- Parser reports multiple errors per run
- Standard library has basic I/O

### Nice to Have
- 100% Modern Pascal compliance (parser is ~100% there)
- Multiple backend targets
- Advanced IR optimizations

---

## 📝 Notes

### Why This Order Is Different From 2024-12-23 Plan

The previous plan over-indexed on language-compliance features (class properties, dynamic arrays, generics) when **all of these are already implemented in the parser and semantics**. The actual blocker is that the IR→bytes pipeline is incomplete.

**Key insight:** You can have 100% language compliance but still not be able to compile anything. The priority is making the compiler functional end-to-end, not adding more parser features.

### Dependencies
- Assembler depends on `Z80Instruction` enum being complete in backend
- Integration tests depend on assembler working
- Standard library depends on runtime infrastructure

### Files of Interest for Phase 1
- `driver/src/compiler.rs` — pipeline orchestration, `instructions_to_bytes()` stub
- `backends/backend-zealz80/src/lib.rs` — `Z80Instruction` enum definition
- `ir/src/lib.rs` — IR builder and `Program` struct
- `object_zealz80/src/lib.rs` — object file format
