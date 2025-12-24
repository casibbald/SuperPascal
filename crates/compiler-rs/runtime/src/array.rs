//! Array runtime support
//!
//! This module provides runtime functions for array operations, including slicing.

use core;

/// Array slice result
/// Contains the sliced array data
#[derive(Debug, Clone)]
pub struct ArraySlice {
    /// Pointer to the sliced array data
    pub data: *mut u8,
    /// Length of the slice
    pub length: usize,
    /// Element size in bytes
    pub element_size: usize,
}

/// Create a slice from an array
/// 
/// # Parameters
/// - `array_ptr`: Pointer to the source array
/// - `array_length`: Length of the source array
/// - `element_size`: Size of each element in bytes
/// - `start`: Start index (0-based), None means from beginning
/// - `end`: End index (exclusive), None means to end
/// - `step`: Step size, None means step of 1
/// 
/// # Returns
/// A new ArraySlice containing the sliced data
/// 
/// # Safety
/// This function is unsafe because it performs raw pointer operations.
/// The caller must ensure:
/// - `array_ptr` is valid and points to valid array data
/// - `array_length` matches the actual array length
/// - `element_size` is correct for the array element type
pub unsafe fn array_slice(
    array_ptr: *const u8,
    array_length: usize,
    element_size: usize,
    start: Option<usize>,
    end: Option<usize>,
    step: Option<usize>,
) -> ArraySlice {
    // Normalize indices
    let start_idx = start.unwrap_or(0);
    let end_idx = end.unwrap_or(array_length);
    let step_size = step.unwrap_or(1);
    
    // Handle negative indices (Python-style)
    // For now, we'll handle this at the call site if needed
    
    // Clamp indices to valid range
    let start_idx = start_idx.min(array_length);
    let end_idx = end_idx.min(array_length);
    
    // Calculate slice length
    // Note: step_size is usize, so we can't have negative steps here
    // Negative steps would need to be handled at the call site by converting
    // to a signed type first. For now, we only handle positive steps.
    let slice_length = if step_size > 0 {
        if start_idx < end_idx {
            (end_idx - start_idx + step_size - 1) / step_size
        } else {
            0
        }
    } else {
        // Step of 0 is invalid
        0
    };
    
    // Allocate memory for the slice
    let slice_size = slice_length * element_size;
    let slice_data = if slice_size > 0 {
        // In a real implementation, this would use the runtime allocator
        // For now, we'll return a pointer to the original data (copy-on-write would be better)
        // This is a placeholder - actual implementation would allocate new memory
        array_ptr.add(start_idx * element_size) as *mut u8
    } else {
        core::ptr::null_mut()
    };
    
    ArraySlice {
        data: slice_data,
        length: slice_length,
        element_size,
    }
}

/// Copy array slice data to a new array
/// 
/// # Safety
/// This function is unsafe because it performs raw pointer operations.
pub unsafe fn array_slice_copy(slice: &ArraySlice) -> *mut u8 {
    // In a real implementation, this would allocate new memory and copy the data
    // For now, return the original pointer (this is a placeholder)
    slice.data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_slice_basic() {
        // Create a test array: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array_ptr = array.as_ptr() as *const u8;
        
        unsafe {
            // Slice [1:5] should give [1, 2, 3, 4]
            let slice = array_slice(array_ptr, 10, 4, Some(1), Some(5), None);
            assert_eq!(slice.length, 4);
            assert_eq!(slice.element_size, 4);
        }
    }

    #[test]
    fn test_array_slice_from_beginning() {
        let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array_ptr = array.as_ptr() as *const u8;
        
        unsafe {
            // Slice [:5] should give [0, 1, 2, 3, 4]
            let slice = array_slice(array_ptr, 10, 4, None, Some(5), None);
            assert_eq!(slice.length, 5);
        }
    }

    #[test]
    fn test_array_slice_to_end() {
        let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array_ptr = array.as_ptr() as *const u8;
        
        unsafe {
            // Slice [3:] should give [3, 4, 5, 6, 7, 8, 9]
            let slice = array_slice(array_ptr, 10, 4, Some(3), None, None);
            assert_eq!(slice.length, 7);
        }
    }

    #[test]
    fn test_array_slice_with_step() {
        let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array_ptr = array.as_ptr() as *const u8;
        
        unsafe {
            // Slice [0:10:2] should give [0, 2, 4, 6, 8]
            let slice = array_slice(array_ptr, 10, 4, Some(0), Some(10), Some(2));
            assert_eq!(slice.length, 5);
        }
    }

    #[test]
    fn test_array_slice_full() {
        let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array_ptr = array.as_ptr() as *const u8;
        
        unsafe {
            // Slice [:] should give the entire array
            let slice = array_slice(array_ptr, 10, 4, None, None, None);
            assert_eq!(slice.length, 10);
        }
    }

    #[test]
    fn test_array_slice_empty() {
        let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array_ptr = array.as_ptr() as *const u8;
        
        unsafe {
            // Slice [5:5] should give empty array
            let slice = array_slice(array_ptr, 10, 4, Some(5), Some(5), None);
            assert_eq!(slice.length, 0);
        }
    }
}

