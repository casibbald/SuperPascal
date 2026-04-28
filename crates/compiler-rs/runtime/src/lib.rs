//! SuperPascal Runtime Library
//!
//! This crate provides runtime support functions for SuperPascal language features
//! that require runtime behavior, such as Variant types, dynamic arrays, closures,
//! and reference-counted interfaces.

pub mod array;
pub mod closure;
pub mod interface;
pub mod variant;

pub use array::*;
pub use closure::*;
pub use interface::*;
/// Re-export modules for convenience
pub use variant::*;
