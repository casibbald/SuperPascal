# Backend Capabilities - Quick Reference

## Summary

SuperPascal now supports **backend-specific feature validation** to ensure code compatibility with target platforms.

## Key Points

1. **Retro Platforms** (ZealZ80, CommanderX16, Foenix65C816, FoenixA2560M, Intel8051)
   - Limited feature sets based on hardware constraints
   - Compiler will **error** when unsupported features are used
   - Focus on core Pascal features suitable for 8-bit/16-bit/32-bit retro systems

2. **Modern Platform** (RaspberryPi5 - "Full Fat" Backend)
   - ✅ **ALL** modern Pascal features supported
   - Complete Object Pascal as per [Modern Object Pascal Introduction](https://castle-engine.io/modern_pascal)
   - Suitable for students building on modern 64-bit ARM or x86-64 systems

## Implementation

- **Feature Detection**: Automatically detects feature usage in AST
- **Backend Capabilities**: Each platform has a capability profile
- **Validation**: Runs after semantic analysis, before code generation
- **Error Messages**: Clear diagnostics showing which features are unsupported

## Usage Example

```rust
// Retro platform - will error on unsupported features
let mut compiler = Compiler::new_with_target(TargetPlatform::ZealZ80);
compiler.compile_file("program.pas", None)?;

// Modern platform - full feature support
let mut compiler = Compiler::new_with_target(TargetPlatform::RaspberryPi5);
compiler.compile_file("program.pas", None)?;
```

## Feature Support by Platform

| Platform | Dynamic Arrays | Generics | Exception Handling | Multithreading |
|----------|---------------|----------|-------------------|----------------|
| ZealZ80 | ❌ | ❌ | ❌ | ❌ |
| CommanderX16 | ❌ | ❌ | ❌ | ❌ |
| Foenix65C816 | ❌ | ❌ | ✅ | ❌ |
| FoenixA2560M | ✅ | ❌ | ✅ | ❌ |
| Intel8051 | ❌ | ❌ | ❌ | ❌ |
| **RaspberryPi5** | ✅ | ✅ | ✅ | ✅ |

**See [backend-capabilities.md](./backend-capabilities.md) for complete feature matrix.**

