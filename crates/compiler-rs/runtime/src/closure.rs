//! Closure Runtime Support
//!
//! Provides runtime functions for closure (anonymous function) operations including:
//! - Closure creation with captured variables
//! - Closure invocation
//! - Memory management for closures

use std::collections::HashMap;

/// Closure data structure
/// Represents an anonymous function with captured variables
#[derive(Debug, Clone)]
pub struct Closure {
    /// Function pointer (in actual implementation, this would be a code address)
    /// For now, we use a string identifier to represent the function
    function_id: String,
    /// Captured variables: name -> value (as bytes)
    /// In actual implementation, values would be stored with proper types
    captured_vars: HashMap<String, Vec<u8>>,
    /// Number of parameters the closure expects
    param_count: usize,
    /// Whether this closure returns a value (function) or not (procedure)
    returns_value: bool,
}

impl Closure {
    /// Create a new closure
    pub fn new(
        function_id: String,
        captured_vars: HashMap<String, Vec<u8>>,
        param_count: usize,
        returns_value: bool,
    ) -> Self {
        Self {
            function_id,
            captured_vars,
            param_count,
            returns_value,
        }
    }

    /// Get the function identifier
    pub fn function_id(&self) -> &str {
        &self.function_id
    }

    /// Get a captured variable value
    pub fn get_captured(&self, name: &str) -> Option<&Vec<u8>> {
        self.captured_vars.get(name)
    }

    /// Set a captured variable value (for mutable captures)
    pub fn set_captured(&mut self, name: String, value: Vec<u8>) {
        self.captured_vars.insert(name, value);
    }

    /// Get all captured variable names
    pub fn captured_names(&self) -> Vec<String> {
        self.captured_vars.keys().cloned().collect()
    }

    /// Get the number of parameters
    pub fn param_count(&self) -> usize {
        self.param_count
    }

    /// Check if this closure returns a value
    pub fn returns_value(&self) -> bool {
        self.returns_value
    }
}

/// Runtime function: Create a new closure
/// Parameters:
/// - function_id: Identifier for the function code
/// - captured_count: Number of captured variables
/// - captured_names: Array of captured variable names
/// - captured_values: Array of captured variable values (as bytes)
/// - param_count: Number of parameters
/// - returns_value: Whether the closure returns a value
pub fn closure_new(
    function_id: String,
    captured_names: Vec<String>,
    captured_values: Vec<Vec<u8>>,
    param_count: usize,
    returns_value: bool,
) -> Closure {
    let mut captured_vars = HashMap::new();
    for (name, value) in captured_names.into_iter().zip(captured_values.into_iter()) {
        captured_vars.insert(name, value);
    }
    Closure::new(function_id, captured_vars, param_count, returns_value)
}

/// Runtime function: Create a closure without captured variables
pub fn closure_new_simple(function_id: String, param_count: usize, returns_value: bool) -> Closure {
    Closure::new(function_id, HashMap::new(), param_count, returns_value)
}

/// Runtime function: Get a captured variable from a closure
/// Returns the value as bytes (caller must know the type)
pub fn closure_get_captured(closure: &Closure, name: &str) -> Option<Vec<u8>> {
    closure.get_captured(name).cloned()
}

/// Runtime function: Set a captured variable in a closure (for mutable captures)
pub fn closure_set_captured(closure: &mut Closure, name: String, value: Vec<u8>) {
    closure.set_captured(name, value);
}

/// Runtime function: Call a closure
/// In actual implementation, this would:
/// 1. Set up the captured variables in the closure's environment
/// 2. Push parameters onto the stack (or into registers)
/// 3. Call the function code
/// 4. Return the result (if any)
///
/// For now, this is a placeholder that returns success/failure
pub fn closure_call(_closure: &Closure, _params: Vec<Vec<u8>>) -> Result<Option<Vec<u8>>, String> {
    // TODO: Actual implementation would:
    // 1. Set up captured variables in the closure's environment
    // 2. Call the function with parameters
    // 3. Return the result

    // Placeholder: return success
    Ok(None)
}

/// Runtime function: Free a closure (release its memory)
/// In actual implementation, this would free the closure's memory
pub fn closure_free(_closure: Closure) {
    // TODO: Actual implementation would free memory
    // For now, closure is dropped when it goes out of scope
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_new_simple() {
        let closure = closure_new_simple("func_123".to_string(), 2, true);
        assert_eq!(closure.function_id(), "func_123");
        assert_eq!(closure.param_count(), 2);
        assert!(closure.returns_value());
        assert_eq!(closure.captured_names().len(), 0);
    }

    #[test]
    fn test_closure_new_with_captures() {
        let names = vec!["x".to_string(), "y".to_string()];
        let values = vec![vec![42, 0], vec![100, 0]]; // 16-bit integers as bytes
        let closure = closure_new("func_456".to_string(), names, values, 1, false);

        assert_eq!(closure.function_id(), "func_456");
        assert_eq!(closure.param_count(), 1);
        assert!(!closure.returns_value());
        assert_eq!(closure.captured_names().len(), 2);
        assert!(closure.get_captured("x").is_some());
        assert!(closure.get_captured("y").is_some());
    }

    #[test]
    fn test_closure_get_captured() {
        let names = vec!["value".to_string()];
        let values = vec![vec![10, 0]]; // integer 10
        let closure = closure_new("func_789".to_string(), names, values, 0, true);

        let captured = closure_get_captured(&closure, "value");
        assert!(captured.is_some());
        assert_eq!(captured.unwrap(), vec![10, 0]);
    }

    #[test]
    fn test_closure_set_captured() {
        let mut closure = closure_new_simple("func_abc".to_string(), 0, false);

        closure_set_captured(&mut closure, "counter".to_string(), vec![5, 0]);

        let captured = closure_get_captured(&closure, "counter");
        assert!(captured.is_some());
        assert_eq!(captured.unwrap(), vec![5, 0]);
    }

    #[test]
    fn test_closure_call() {
        let closure = closure_new_simple("func_test".to_string(), 1, true);
        let params = vec![vec![42, 0]]; // Single integer parameter

        let result = closure_call(&closure, params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_closure_multiple_captures() {
        let names = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let values = vec![
            vec![1, 0], // a = 1
            vec![2, 0], // b = 2
            vec![3, 0], // c = 3
        ];
        let closure = closure_new("func_multi".to_string(), names, values, 0, true);

        assert_eq!(closure.captured_names().len(), 3);
        assert_eq!(closure_get_captured(&closure, "a").unwrap(), vec![1, 0]);
        assert_eq!(closure_get_captured(&closure, "b").unwrap(), vec![2, 0]);
        assert_eq!(closure_get_captured(&closure, "c").unwrap(), vec![3, 0]);
    }
}
