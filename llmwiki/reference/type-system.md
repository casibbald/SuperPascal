# Type System

- Status: partially-verified
- Source docs: `languageSpecification/03_TypeSystem.md`, `languageSpecification/08_LanguageReference.md`
- Code anchors: `types/src/lib.rs`, `semantics/src/types.rs`

## Type categories

### Basic types
- `integer` — signed integer
- `real` — floating point
- `char` — character
- `string` — single unified string type
- `boolean` — true/false
- `byte` — 8-bit unsigned
- `word` — 16-bit unsigned
- `longint` — 32-bit signed
- `dword` — 32-bit unsigned
- `qword` — 64-bit unsigned
- `float` — 32-bit float
- `double` — 64-bit float

### Structured types
- `array of T` — static arrays: `array[0..9] of T`
- `array of T` — dynamic arrays: `array of T`
- `record` — struct with fields
- `class` — OOP class with vtable
- `set of T` — set type
- `pointer ^T` — pointer type

### Advanced types
- `procedure of object` — method pointer
- `procedure` — procedure type
- `TList<T>` — generic types

## Hybrid OOP + Struct model

SuperPascal uses a hybrid model:
- **Records (structs)** — primary data carrier, optimized for SoA (Structure of Arrays) storage
- **Classes** — lightweight behavior containers with small vtables
- Records can have methods (lowered to free functions)
- Classes have vtables at offset 0
- Ideal for ECS-driven game logic

## Type compatibility

Types are compatible when:
- Same type identity
- Subrange types compatible with base type
- Compatible structured types with matching field order

## Fixed-point arithmetic

SuperPascal supports fixed-point math:
- `Q8.8` — 8 integer bits, 8 fractional bits
- `Q12.12` — 12 integer bits, 12 fractional bits
- Implemented in `lib/math/fixed.pas`
- Optional FPGA-accelerated trig and vector math
