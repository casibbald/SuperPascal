//! Variant Type Runtime Support
//!
//! Provides runtime functions for Variant type operations including:
//! - Variant creation and initialization
//! - Type checking and conversion
//! - Memory management

use types::Type;

/// Variant type enumeration
/// Maps to Pascal's TVariantType enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VariantType {
    Empty = 0,
    Integer = 1,
    String = 2,
    Boolean = 3,
    Char = 4,
    Byte = 5,
    Word = 6,
    Array = 7,
    Record = 8,
    Pointer = 9,
}

impl VariantType {
    /// Convert from Pascal type to VariantType
    pub fn from_type(ty: &Type) -> Self {
        match ty {
            Type::Primitive(prim) => match prim {
                types::PrimitiveType::Integer => VariantType::Integer,
                types::PrimitiveType::Boolean => VariantType::Boolean,
                types::PrimitiveType::Char => VariantType::Char,
                types::PrimitiveType::Byte => VariantType::Byte,
                types::PrimitiveType::Word => VariantType::Word,
            },
            Type::Array { .. } => VariantType::Array,
            Type::DynamicArray { .. } => VariantType::Array,
            Type::Record { .. } => VariantType::Record,
            Type::Pointer { .. } => VariantType::Pointer,
            Type::Variant => VariantType::Empty, // Variant containing Variant (not supported)
            _ => VariantType::Empty,
        }
    }

    /// Get the size in bytes for a VariantType value
    pub fn value_size(&self) -> usize {
        match self {
            VariantType::Empty => 0,
            VariantType::Integer => 2, // 16-bit integer
            VariantType::Boolean => 1,
            VariantType::Char => 1,
            VariantType::Byte => 1,
            VariantType::Word => 2,
            VariantType::String => 2,  // Pointer to string
            VariantType::Array => 2,   // Pointer to array
            VariantType::Record => 2,  // Pointer to record
            VariantType::Pointer => 2, // Pointer value
        }
    }
}

/// Variant value storage
/// This represents a Variant at runtime
#[derive(Debug, Clone)]
pub struct Variant {
    var_type: VariantType,
    // Value storage - using a union-like structure
    // For simplicity, we'll use an enum to represent the value
    value: VariantValue,
}

/// Variant value representation
#[derive(Debug, Clone)]
pub enum VariantValue {
    Empty,
    Integer(i16),
    Boolean(bool),
    Char(u8),
    Byte(u8),
    Word(u16),
    String(String),  // Owned string
    Array(Vec<u8>),  // Generic array storage (bytes)
    Record(Vec<u8>), // Generic record storage (bytes)
    Pointer(usize),  // Pointer value (address)
}

impl Variant {
    /// Create a new empty Variant
    pub fn new_empty() -> Self {
        Self {
            var_type: VariantType::Empty,
            value: VariantValue::Empty,
        }
    }

    /// Create a new Variant with a value
    pub fn new(var_type: VariantType, value: VariantValue) -> Self {
        Self { var_type, value }
    }

    /// Get the type of this Variant
    pub fn get_type(&self) -> VariantType {
        self.var_type
    }

    /// Check if this Variant is of a specific type
    pub fn is_type(&self, var_type: VariantType) -> bool {
        self.var_type == var_type
    }

    /// Assign a value to this Variant
    pub fn assign(&mut self, var_type: VariantType, value: VariantValue) {
        self.var_type = var_type;
        self.value = value;
    }

    /// Copy from another Variant
    pub fn copy_from(&mut self, other: &Variant) {
        self.var_type = other.var_type;
        self.value = other.value.clone();
    }

    /// Convert to integer (with type check)
    pub fn to_integer(&self) -> Result<i16, String> {
        match &self.value {
            VariantValue::Integer(i) => Ok(*i),
            VariantValue::Byte(b) => Ok(*b as i16),
            VariantValue::Word(w) => Ok(*w as i16),
            VariantValue::Char(c) => Ok(*c as i16),
            _ => Err(format!("Cannot convert {:?} to integer", self.var_type)),
        }
    }

    /// Convert to boolean (with type check)
    pub fn to_boolean(&self) -> Result<bool, String> {
        match &self.value {
            VariantValue::Boolean(b) => Ok(*b),
            _ => Err(format!("Cannot convert {:?} to boolean", self.var_type)),
        }
    }

    /// Convert to char (with type check)
    pub fn to_char(&self) -> Result<u8, String> {
        match &self.value {
            VariantValue::Char(c) => Ok(*c),
            VariantValue::Byte(b) => Ok(*b),
            _ => Err(format!("Cannot convert {:?} to char", self.var_type)),
        }
    }

    /// Convert to byte (with type check)
    pub fn to_byte(&self) -> Result<u8, String> {
        match &self.value {
            VariantValue::Byte(b) => Ok(*b),
            VariantValue::Char(c) => Ok(*c),
            VariantValue::Integer(i) if *i >= 0 && *i <= 255 => Ok(*i as u8),
            VariantValue::Word(w) if *w <= 255 => Ok(*w as u8),
            _ => Err(format!("Cannot convert {:?} to byte", self.var_type)),
        }
    }

    /// Convert to word (with type check)
    pub fn to_word(&self) -> Result<u16, String> {
        match &self.value {
            VariantValue::Word(w) => Ok(*w),
            VariantValue::Byte(b) => Ok(*b as u16),
            VariantValue::Char(c) => Ok(*c as u16),
            VariantValue::Integer(i) if *i >= 0 => Ok(*i as u16),
            _ => Err(format!("Cannot convert {:?} to word", self.var_type)),
        }
    }

    /// Convert to string (with type check)
    pub fn to_string(&self) -> Result<String, String> {
        match &self.value {
            VariantValue::String(s) => Ok(s.clone()),
            VariantValue::Char(c) => Ok((*c as char).to_string()),
            VariantValue::Integer(i) => Ok(i.to_string()),
            VariantValue::Boolean(b) => Ok(b.to_string()),
            _ => Err(format!("Cannot convert {:?} to string", self.var_type)),
        }
    }

    /// Get the value (for internal use)
    pub fn value(&self) -> &VariantValue {
        &self.value
    }
}

/// Runtime function: Create a new empty Variant
pub fn variant_new_empty() -> Variant {
    Variant::new_empty()
}

/// Runtime function: Create a new Variant with a value
pub fn variant_new(var_type: VariantType, value: VariantValue) -> Variant {
    Variant::new(var_type, value)
}

/// Runtime function: Assign a value to a Variant
pub fn variant_assign(variant: &mut Variant, var_type: VariantType, value: VariantValue) {
    variant.assign(var_type, value);
}

/// Runtime function: Copy one Variant to another
pub fn variant_copy(dest: &mut Variant, src: &Variant) {
    dest.copy_from(src);
}

/// Runtime function: Get the type of a Variant
pub fn variant_get_type(variant: &Variant) -> VariantType {
    variant.get_type()
}

/// Runtime function: Check if a Variant is of a specific type
pub fn variant_is_type(variant: &Variant, var_type: VariantType) -> bool {
    variant.is_type(var_type)
}

/// Runtime function: Convert Variant to integer
pub fn variant_to_integer(variant: &Variant) -> Result<i16, String> {
    variant.to_integer()
}

/// Runtime function: Convert Variant to boolean
pub fn variant_to_boolean(variant: &Variant) -> Result<bool, String> {
    variant.to_boolean()
}

/// Runtime function: Convert Variant to char
pub fn variant_to_char(variant: &Variant) -> Result<u8, String> {
    variant.to_char()
}

/// Runtime function: Convert Variant to byte
pub fn variant_to_byte(variant: &Variant) -> Result<u8, String> {
    variant.to_byte()
}

/// Runtime function: Convert Variant to word
pub fn variant_to_word(variant: &Variant) -> Result<u16, String> {
    variant.to_word()
}

/// Runtime function: Convert Variant to string
pub fn variant_to_string(variant: &Variant) -> Result<String, String> {
    variant.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_new_empty() {
        let v = variant_new_empty();
        assert_eq!(v.get_type(), VariantType::Empty);
    }

    #[test]
    fn test_variant_integer() {
        let v = Variant::new(VariantType::Integer, VariantValue::Integer(42));
        assert_eq!(v.get_type(), VariantType::Integer);
        assert_eq!(v.to_integer().unwrap(), 42);
    }

    #[test]
    fn test_variant_string() {
        let v = Variant::new(
            VariantType::String,
            VariantValue::String("Hello".to_string()),
        );
        assert_eq!(v.get_type(), VariantType::String);
        assert_eq!(v.to_string().unwrap(), "Hello");
    }

    #[test]
    fn test_variant_boolean() {
        let v = Variant::new(VariantType::Boolean, VariantValue::Boolean(true));
        assert_eq!(v.get_type(), VariantType::Boolean);
        assert_eq!(v.to_boolean().unwrap(), true);
    }

    #[test]
    fn test_variant_assign() {
        let mut v = variant_new_empty();
        variant_assign(&mut v, VariantType::Integer, VariantValue::Integer(100));
        assert_eq!(v.get_type(), VariantType::Integer);
        assert_eq!(v.to_integer().unwrap(), 100);
    }

    #[test]
    fn test_variant_copy() {
        let src = Variant::new(VariantType::Integer, VariantValue::Integer(42));
        let mut dest = variant_new_empty();
        variant_copy(&mut dest, &src);
        assert_eq!(dest.get_type(), VariantType::Integer);
        assert_eq!(dest.to_integer().unwrap(), 42);
    }

    #[test]
    fn test_variant_type_checking() {
        let v = Variant::new(VariantType::Integer, VariantValue::Integer(42));
        assert!(variant_is_type(&v, VariantType::Integer));
        assert!(!variant_is_type(&v, VariantType::String));
    }

    #[test]
    fn test_variant_conversion_integer() {
        let v = Variant::new(VariantType::Integer, VariantValue::Integer(42));
        assert_eq!(variant_to_integer(&v).unwrap(), 42);
    }

    #[test]
    fn test_variant_conversion_byte_to_integer() {
        let v = Variant::new(VariantType::Byte, VariantValue::Byte(100));
        assert_eq!(variant_to_integer(&v).unwrap(), 100);
    }

    #[test]
    fn test_variant_conversion_invalid() {
        let v = Variant::new(
            VariantType::String,
            VariantValue::String("Hello".to_string()),
        );
        assert!(variant_to_integer(&v).is_err());
    }

    #[test]
    fn test_variant_conversion_string() {
        let v = Variant::new(
            VariantType::String,
            VariantValue::String("Test".to_string()),
        );
        assert_eq!(variant_to_string(&v).unwrap(), "Test");
    }

    #[test]
    fn test_variant_conversion_integer_to_string() {
        let v = Variant::new(VariantType::Integer, VariantValue::Integer(123));
        assert_eq!(variant_to_string(&v).unwrap(), "123");
    }

    #[test]
    fn test_variant_from_type() {
        let int_type = Type::integer();
        assert_eq!(VariantType::from_type(&int_type), VariantType::Integer);

        let bool_type = Type::boolean();
        assert_eq!(VariantType::from_type(&bool_type), VariantType::Boolean);

        let variant_type = Type::variant();
        assert_eq!(VariantType::from_type(&variant_type), VariantType::Empty);
    }
}
