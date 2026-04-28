//! Backend Capabilities and Feature Flags
//!
//! This module defines which Pascal language features are supported by each backend.
//! This allows the compiler to warn or error when code uses features not available
//! on the target platform.

use crate::TargetPlatform;

/// Language feature flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageFeature {
    // Core features (all backends)
    BasicTypes,  // integer, boolean, char, byte, word
    Arrays,      // Static arrays
    Records,     // Basic records
    Procedures,  // Procedures and functions
    ControlFlow, // if, while, for, repeat, case

    // Advanced types
    DynamicArrays,   // array of T (not fixed-size)
    Sets,            // SET OF type
    Strings,         // STRING and STRING[n]
    VariantRecords,  // RECORD ... CASE ... OF ... END
    EnumeratedTypes, // type Color = (Red, Green, Blue)
    Pointers,        // ^type syntax
    ProceduralTypes, // PROCEDURE/FUNCTION types
    FileTypes,       // FILE and FILE OF type

    // Object Pascal
    Classes,             // CLASS declarations
    Interfaces,          // INTERFACE types
    Properties,          // PROPERTY declarations
    OperatorOverloading, // OPERATOR keyword
    MethodPointers,      // OF OBJECT for procedural types

    // Advanced features
    Generics,           // Generic types and routines
    AnonymousFunctions, // Lambda expressions
    NestedRoutines,     // Procedures/functions inside other routines
    ExceptionHandling,  // TRY/EXCEPT/FINALLY
    WithStatement,      // WITH record_expr DO statement
    GotoLabels,         // LABEL and GOTO
    InlineAssembly,     // ASM ... END blocks
    ForInLoops,         // for item in collection do

    // Advanced declarations
    ThreadVar,       // THREADVAR declarations
    ConstRef,        // CONSTREF parameters
    OutParams,       // OUT parameters
    Resourcestring,  // RESOURCESTRING declarations
    Absolute,        // ABSOLUTE variable addressing
    DefaultParams,   // Default parameter values
    ForwardExternal, // FORWARD and EXTERNAL declarations

    // Class features
    ClassMethods,    // class procedure/function
    ClassProperties, // Properties on the class itself
    ClassVariables,  // Variables shared across all instances
    ClassHelpers,    // Extension mechanism for existing classes
    NestedClasses,   // Classes inside other classes

    // Runtime features
    ReferenceCounting, // Automatic memory management
    GarbageCollection, // GC support
    Multithreading,    // Thread support
    DynamicLinking,    // Dynamic library loading
}

/// Backend capabilities configuration
#[derive(Debug, Clone)]
pub struct BackendCapabilities {
    pub platform: TargetPlatform,
    pub features: std::collections::HashSet<LanguageFeature>,
    pub name: String,
    pub description: String,
}

impl BackendCapabilities {
    /// Check if a feature is supported
    pub fn supports(&self, feature: LanguageFeature) -> bool {
        self.features.contains(&feature)
    }

    /// Get all unsupported features from a list
    pub fn unsupported(&self, features: &[LanguageFeature]) -> Vec<LanguageFeature> {
        features
            .iter()
            .filter(|f| !self.supports(**f))
            .copied()
            .collect()
    }
}

/// Get capabilities for a target platform
pub fn get_capabilities(platform: TargetPlatform) -> BackendCapabilities {
    match platform {
        TargetPlatform::ZealZ80 => zealz80_capabilities(),
        TargetPlatform::CommanderX16 => commanderx16_capabilities(),
        TargetPlatform::Foenix65C816 => foenix65c816_capabilities(),
        TargetPlatform::FoenixA2560M => foenix_a2560m_capabilities(),
        TargetPlatform::Intel8051 => intel8051_capabilities(),
        TargetPlatform::RaspberryPi5 => raspberry_pi5_capabilities(),
    }
}

/// ZealZ80 (Z80 @ 10 MHz) - Retro 8-bit platform
/// Limited memory and processing power, supports core Pascal features
fn zealz80_capabilities() -> BackendCapabilities {
    let mut features = std::collections::HashSet::new();

    // Core features
    features.insert(LanguageFeature::BasicTypes);
    features.insert(LanguageFeature::Arrays);
    features.insert(LanguageFeature::Records);
    features.insert(LanguageFeature::Procedures);
    features.insert(LanguageFeature::ControlFlow);

    // Advanced types (limited)
    features.insert(LanguageFeature::Sets);
    features.insert(LanguageFeature::Strings);
    features.insert(LanguageFeature::VariantRecords);
    features.insert(LanguageFeature::EnumeratedTypes);
    features.insert(LanguageFeature::Pointers);
    features.insert(LanguageFeature::FileTypes);

    // Object Pascal (limited)
    features.insert(LanguageFeature::Classes);
    features.insert(LanguageFeature::Properties);
    features.insert(LanguageFeature::MethodPointers);

    // Advanced features (limited)
    features.insert(LanguageFeature::NestedRoutines);
    features.insert(LanguageFeature::WithStatement);
    features.insert(LanguageFeature::GotoLabels);
    features.insert(LanguageFeature::InlineAssembly);
    features.insert(LanguageFeature::ForInLoops);

    // Advanced declarations
    features.insert(LanguageFeature::ForwardExternal);
    features.insert(LanguageFeature::Absolute);

    // NOT SUPPORTED:
    // - DynamicArrays (no heap management)
    // - ProceduralTypes (complex for 8-bit)
    // - Interfaces (too complex)
    // - OperatorOverloading (performance)
    // - Generics (too complex)
    // - AnonymousFunctions (too complex)
    // - ExceptionHandling (no runtime support)
    // - ThreadVar, ConstRef, OutParams, Resourcestring, DefaultParams
    // - ClassMethods, ClassProperties, ClassVariables, ClassHelpers, NestedClasses
    // - ReferenceCounting, GarbageCollection, Multithreading, DynamicLinking

    BackendCapabilities {
        platform: TargetPlatform::ZealZ80,
        features,
        name: "ZealZ80".to_string(),
        description: "Zilog Z80 @ 10 MHz - Retro 8-bit platform with limited memory".to_string(),
    }
}

/// CommanderX16 (65C02 @ 8 MHz) - Retro 8-bit platform
/// Similar to ZealZ80 but 6502-compatible
fn commanderx16_capabilities() -> BackendCapabilities {
    let mut features = std::collections::HashSet::new();

    // Core features
    features.insert(LanguageFeature::BasicTypes);
    features.insert(LanguageFeature::Arrays);
    features.insert(LanguageFeature::Records);
    features.insert(LanguageFeature::Procedures);
    features.insert(LanguageFeature::ControlFlow);

    // Advanced types (limited)
    features.insert(LanguageFeature::Sets);
    features.insert(LanguageFeature::Strings);
    features.insert(LanguageFeature::VariantRecords);
    features.insert(LanguageFeature::EnumeratedTypes);
    features.insert(LanguageFeature::Pointers);
    features.insert(LanguageFeature::FileTypes);

    // Object Pascal (limited)
    features.insert(LanguageFeature::Classes);
    features.insert(LanguageFeature::Properties);
    features.insert(LanguageFeature::MethodPointers);

    // Advanced features (limited)
    features.insert(LanguageFeature::NestedRoutines);
    features.insert(LanguageFeature::WithStatement);
    features.insert(LanguageFeature::GotoLabels);
    features.insert(LanguageFeature::InlineAssembly);
    features.insert(LanguageFeature::ForInLoops);

    // Advanced declarations
    features.insert(LanguageFeature::ForwardExternal);
    features.insert(LanguageFeature::Absolute);

    BackendCapabilities {
        platform: TargetPlatform::CommanderX16,
        features,
        name: "CommanderX16".to_string(),
        description: "WDC 65C02 @ 8 MHz - Retro 8-bit platform".to_string(),
    }
}

/// Foenix65C816 (W65C816S @ 6.29 MHz) - Retro 16-bit platform
/// More capable than 8-bit platforms but still limited
fn foenix65c816_capabilities() -> BackendCapabilities {
    let mut features = std::collections::HashSet::new();

    // Core features
    features.insert(LanguageFeature::BasicTypes);
    features.insert(LanguageFeature::Arrays);
    features.insert(LanguageFeature::Records);
    features.insert(LanguageFeature::Procedures);
    features.insert(LanguageFeature::ControlFlow);

    // Advanced types
    features.insert(LanguageFeature::Sets);
    features.insert(LanguageFeature::Strings);
    features.insert(LanguageFeature::VariantRecords);
    features.insert(LanguageFeature::EnumeratedTypes);
    features.insert(LanguageFeature::Pointers);
    features.insert(LanguageFeature::ProceduralTypes);
    features.insert(LanguageFeature::FileTypes);

    // Object Pascal
    features.insert(LanguageFeature::Classes);
    features.insert(LanguageFeature::Interfaces);
    features.insert(LanguageFeature::Properties);
    features.insert(LanguageFeature::OperatorOverloading);
    features.insert(LanguageFeature::MethodPointers);

    // Advanced features
    features.insert(LanguageFeature::NestedRoutines);
    features.insert(LanguageFeature::ExceptionHandling);
    features.insert(LanguageFeature::WithStatement);
    features.insert(LanguageFeature::GotoLabels);
    features.insert(LanguageFeature::InlineAssembly);
    features.insert(LanguageFeature::ForInLoops);

    // Advanced declarations
    features.insert(LanguageFeature::ThreadVar);
    features.insert(LanguageFeature::ConstRef);
    features.insert(LanguageFeature::OutParams);
    features.insert(LanguageFeature::Resourcestring);
    features.insert(LanguageFeature::Absolute);
    features.insert(LanguageFeature::DefaultParams);
    features.insert(LanguageFeature::ForwardExternal);

    // Class features (limited)
    features.insert(LanguageFeature::ClassMethods);

    // NOT SUPPORTED:
    // - DynamicArrays (limited heap)
    // - Generics (too complex)
    // - AnonymousFunctions (too complex)
    // - ClassProperties, ClassVariables, ClassHelpers, NestedClasses
    // - ReferenceCounting, GarbageCollection, Multithreading, DynamicLinking

    BackendCapabilities {
        platform: TargetPlatform::Foenix65C816,
        features,
        name: "Foenix65C816".to_string(),
        description: "WDC W65C816S @ 6.29 MHz - Retro 16-bit platform".to_string(),
    }
}

/// FoenixA2560M (MC68LC060 @ 66 MHz) - Retro 32-bit platform
/// More capable than 16-bit but still retro
fn foenix_a2560m_capabilities() -> BackendCapabilities {
    let mut features = std::collections::HashSet::new();

    // Core features
    features.insert(LanguageFeature::BasicTypes);
    features.insert(LanguageFeature::Arrays);
    features.insert(LanguageFeature::Records);
    features.insert(LanguageFeature::Procedures);
    features.insert(LanguageFeature::ControlFlow);

    // Advanced types
    features.insert(LanguageFeature::DynamicArrays);
    features.insert(LanguageFeature::Sets);
    features.insert(LanguageFeature::Strings);
    features.insert(LanguageFeature::VariantRecords);
    features.insert(LanguageFeature::EnumeratedTypes);
    features.insert(LanguageFeature::Pointers);
    features.insert(LanguageFeature::ProceduralTypes);
    features.insert(LanguageFeature::FileTypes);

    // Object Pascal
    features.insert(LanguageFeature::Classes);
    features.insert(LanguageFeature::Interfaces);
    features.insert(LanguageFeature::Properties);
    features.insert(LanguageFeature::OperatorOverloading);
    features.insert(LanguageFeature::MethodPointers);

    // Advanced features
    features.insert(LanguageFeature::NestedRoutines);
    features.insert(LanguageFeature::ExceptionHandling);
    features.insert(LanguageFeature::WithStatement);
    features.insert(LanguageFeature::GotoLabels);
    features.insert(LanguageFeature::InlineAssembly);
    features.insert(LanguageFeature::ForInLoops);

    // Advanced declarations
    features.insert(LanguageFeature::ThreadVar);
    features.insert(LanguageFeature::ConstRef);
    features.insert(LanguageFeature::OutParams);
    features.insert(LanguageFeature::Resourcestring);
    features.insert(LanguageFeature::Absolute);
    features.insert(LanguageFeature::DefaultParams);
    features.insert(LanguageFeature::ForwardExternal);

    // Class features
    features.insert(LanguageFeature::ClassMethods);
    features.insert(LanguageFeature::ClassProperties);
    features.insert(LanguageFeature::ClassVariables);

    // NOT SUPPORTED:
    // - Generics (too complex for retro)
    // - AnonymousFunctions (too complex)
    // - ClassHelpers, NestedClasses
    // - ReferenceCounting, GarbageCollection, Multithreading, DynamicLinking

    BackendCapabilities {
        platform: TargetPlatform::FoenixA2560M,
        features,
        name: "FoenixA2560M".to_string(),
        description: "MC68LC060 @ 66 MHz - Retro 32-bit platform".to_string(),
    }
}

/// Intel8051 - Microcontroller platform
/// Very limited, minimal feature set
fn intel8051_capabilities() -> BackendCapabilities {
    let mut features = std::collections::HashSet::new();

    // Core features only
    features.insert(LanguageFeature::BasicTypes);
    features.insert(LanguageFeature::Arrays);
    features.insert(LanguageFeature::Records);
    features.insert(LanguageFeature::Procedures);
    features.insert(LanguageFeature::ControlFlow);

    // Limited advanced types
    features.insert(LanguageFeature::EnumeratedTypes);
    features.insert(LanguageFeature::Pointers);

    // Advanced features (minimal)
    features.insert(LanguageFeature::InlineAssembly);
    features.insert(LanguageFeature::Absolute);

    BackendCapabilities {
        platform: TargetPlatform::Intel8051,
        features,
        name: "Intel8051".to_string(),
        description: "Intel 8051 - Microcontroller platform (minimal features)".to_string(),
    }
}

/// RaspberryPi5 (ARM Cortex-A76 @ 2.4 GHz) - "Full Fat" modern platform
/// Supports ALL modern Pascal features
fn raspberry_pi5_capabilities() -> BackendCapabilities {
    let mut features = std::collections::HashSet::new();

    // ALL features supported on modern platform
    features.insert(LanguageFeature::BasicTypes);
    features.insert(LanguageFeature::Arrays);
    features.insert(LanguageFeature::DynamicArrays);
    features.insert(LanguageFeature::Records);
    features.insert(LanguageFeature::Procedures);
    features.insert(LanguageFeature::ControlFlow);
    features.insert(LanguageFeature::Sets);
    features.insert(LanguageFeature::Strings);
    features.insert(LanguageFeature::VariantRecords);
    features.insert(LanguageFeature::EnumeratedTypes);
    features.insert(LanguageFeature::Pointers);
    features.insert(LanguageFeature::ProceduralTypes);
    features.insert(LanguageFeature::FileTypes);
    features.insert(LanguageFeature::Classes);
    features.insert(LanguageFeature::Interfaces);
    features.insert(LanguageFeature::Properties);
    features.insert(LanguageFeature::OperatorOverloading);
    features.insert(LanguageFeature::MethodPointers);
    features.insert(LanguageFeature::Generics);
    features.insert(LanguageFeature::AnonymousFunctions);
    features.insert(LanguageFeature::NestedRoutines);
    features.insert(LanguageFeature::ExceptionHandling);
    features.insert(LanguageFeature::WithStatement);
    features.insert(LanguageFeature::GotoLabels);
    features.insert(LanguageFeature::InlineAssembly);
    features.insert(LanguageFeature::ForInLoops);
    features.insert(LanguageFeature::ThreadVar);
    features.insert(LanguageFeature::ConstRef);
    features.insert(LanguageFeature::OutParams);
    features.insert(LanguageFeature::Resourcestring);
    features.insert(LanguageFeature::Absolute);
    features.insert(LanguageFeature::DefaultParams);
    features.insert(LanguageFeature::ForwardExternal);
    features.insert(LanguageFeature::ClassMethods);
    features.insert(LanguageFeature::ClassProperties);
    features.insert(LanguageFeature::ClassVariables);
    features.insert(LanguageFeature::ClassHelpers);
    features.insert(LanguageFeature::NestedClasses);
    features.insert(LanguageFeature::ReferenceCounting);
    features.insert(LanguageFeature::GarbageCollection);
    features.insert(LanguageFeature::Multithreading);
    features.insert(LanguageFeature::DynamicLinking);

    BackendCapabilities {
        platform: TargetPlatform::RaspberryPi5,
        features,
        name: "RaspberryPi5".to_string(),
        description: "ARM Cortex-A76 @ 2.4 GHz - Full modern Pascal support".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zealz80_capabilities() {
        let caps = zealz80_capabilities();
        assert!(caps.supports(LanguageFeature::BasicTypes));
        assert!(caps.supports(LanguageFeature::Classes));
        assert!(!caps.supports(LanguageFeature::DynamicArrays));
        assert!(!caps.supports(LanguageFeature::Generics));
        assert!(!caps.supports(LanguageFeature::ExceptionHandling));
    }

    #[test]
    fn test_raspberry_pi5_capabilities() {
        let caps = raspberry_pi5_capabilities();
        assert!(caps.supports(LanguageFeature::BasicTypes));
        assert!(caps.supports(LanguageFeature::DynamicArrays));
        assert!(caps.supports(LanguageFeature::Generics));
        assert!(caps.supports(LanguageFeature::ExceptionHandling));
        assert!(caps.supports(LanguageFeature::AnonymousFunctions));
        assert!(caps.supports(LanguageFeature::Multithreading));
    }

    #[test]
    fn test_unsupported_features() {
        let caps = zealz80_capabilities();
        let requested = vec![
            LanguageFeature::BasicTypes,
            LanguageFeature::Generics,
            LanguageFeature::ExceptionHandling,
        ];
        let unsupported = caps.unsupported(&requested);
        assert_eq!(unsupported.len(), 2);
        assert!(unsupported.contains(&LanguageFeature::Generics));
        assert!(unsupported.contains(&LanguageFeature::ExceptionHandling));
    }
}
