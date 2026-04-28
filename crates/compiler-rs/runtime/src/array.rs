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
/// - `start`: Start index (signed, negative means from end), None means from beginning
/// - `end`: End index (signed, negative means from end), None means to end
/// - `step`: Step size (signed, negative means reverse), None means step of 1
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
    start: Option<i32>,
    end: Option<i32>,
    step: Option<i32>,
) -> ArraySlice {
    // Normalize indices (convert negative indices to positive)
    let normalize_index = |idx: i32| -> usize {
        if idx < 0 {
            // Negative index: count from end
            // e.g., -1 means array_length - 1, -2 means array_length - 2
            let abs_idx = (-idx) as usize;
            if abs_idx > array_length {
                0
            } else {
                array_length - abs_idx
            }
        } else {
            idx as usize
        }
    };

    let step_size = step.unwrap_or(1);

    // Handle default start/end for reverse slicing
    let (start_idx, end_idx) = if step_size < 0 {
        // Reverse slicing: defaults are different
        let default_start = start
            .map(normalize_index)
            .unwrap_or(array_length.saturating_sub(1));
        // For reverse, None end means "before index 0", which we represent as 0
        // But we need to allow going past 0, so we use a special value
        // Actually, for reverse slicing, end should be -1 (before first element)
        // But since we normalize -1 to length-1, we need special handling
        let default_end = end
            .map(|e| {
                if e == -1 {
                    // Special case: -1 means "before first element" for reverse slicing
                    0
                } else {
                    normalize_index(e)
                }
            })
            .unwrap_or(0);
        (default_start, default_end)
    } else {
        // Forward slicing: normal defaults
        (
            start.map(normalize_index).unwrap_or(0),
            end.map(normalize_index).unwrap_or(array_length),
        )
    };

    // Clamp indices to valid range
    let start_idx = start_idx.min(array_length);
    let end_idx = end_idx.min(array_length);

    // Calculate slice length based on step direction
    let slice_length = if step_size > 0 {
        // Forward slicing: start to end
        if start_idx < end_idx {
            (end_idx - start_idx + step_size as usize - 1) / step_size as usize
        } else {
            0
        }
    } else if step_size < 0 {
        // Reverse slicing: start to end
        // For reverse, we need start >= end
        // When end is explicitly provided (not None), it's exclusive
        // When end is None (default), we include index 0
        if start_idx >= end_idx {
            let step_abs = (-step_size) as usize;
            // If end was explicitly provided, it's exclusive, so we stop before end_idx
            // If end was None (default for reverse), we include index 0
            if end.is_some() {
                // Explicit end: exclusive, so we want (start_idx - end_idx) / step_abs
                // But we need to ensure we don't include end_idx, so: (start_idx - (end_idx - 1)) / step_abs
                // Actually, for exclusive end, we want: (start_idx - end_idx) / step_abs
                // But that gives us one too many. Let's use: (start_idx - end_idx) / step_abs
                // Wait, for [8:2:-1], we want 6 elements: 8,7,6,5,4,3 (exclusive of 2)
                // (8 - 2) / 1 = 6, which is correct!
                // But we're getting 7, which means we're including 2
                // The issue is our formula includes both start and end
                // For exclusive end: (start_idx - end_idx) / step_abs
                (start_idx - end_idx) / step_abs
            } else {
                // Default end (0): inclusive, so include it
                (start_idx - end_idx + step_abs) / step_abs
            }
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
        unsafe {
            if step_size > 0 {
                // Forward: start from start_idx
                array_ptr.add(start_idx * element_size) as *mut u8
            } else {
                // Reverse: start from start_idx (will iterate backwards)
                array_ptr.add(start_idx * element_size) as *mut u8
            }
        }
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
    fn test_array_slice_negative_start() {
        let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array_ptr = array.as_ptr() as *const u8;

        unsafe {
            // Slice [-3:] should give [7, 8, 9] (last 3 elements)
            let slice = array_slice(array_ptr, 10, 4, Some(-3), None, None);
            assert_eq!(slice.length, 3);
        }
    }

    #[test]
    fn test_array_slice_negative_end() {
        let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array_ptr = array.as_ptr() as *const u8;

        unsafe {
            // Slice [:-3] should give [0, 1, 2, 3, 4, 5, 6] (all but last 3)
            let slice = array_slice(array_ptr, 10, 4, None, Some(-3), None);
            assert_eq!(slice.length, 7);
        }
    }

    #[test]
    fn test_array_slice_negative_both() {
        let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array_ptr = array.as_ptr() as *const u8;

        unsafe {
            // Slice [-5:-2] should give [5, 6, 7] (from -5 to -2)
            let slice = array_slice(array_ptr, 10, 4, Some(-5), Some(-2), None);
            assert_eq!(slice.length, 3);
        }
    }

    #[test]
    fn test_array_slice_reverse_step() {
        let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array_ptr = array.as_ptr() as *const u8;

        unsafe {
            // Slice [::-1] should reverse the entire array
            let slice = array_slice(array_ptr, 10, 4, None, None, Some(-1));
            assert_eq!(slice.length, 10);
        }
    }

    #[test]
    fn test_array_slice_reverse_with_bounds() {
        let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array_ptr = array.as_ptr() as *const u8;

        unsafe {
            // Slice [8:2:-1] should give [8, 7, 6, 5, 4, 3] (reverse from 8 to 2, exclusive of 2)
            let slice = array_slice(array_ptr, 10, 4, Some(8), Some(2), Some(-1));
            assert_eq!(slice.length, 6);
        }
    }

    #[test]
    fn test_array_slice_negative_index_out_of_bounds() {
        let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array_ptr = array.as_ptr() as *const u8;

        unsafe {
            // Slice [-20:] should clamp to start (negative index too large)
            let slice = array_slice(array_ptr, 10, 4, Some(-20), None, None);
            assert_eq!(slice.length, 10); // Should get entire array
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
