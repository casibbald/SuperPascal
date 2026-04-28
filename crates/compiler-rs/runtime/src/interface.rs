//! Reference-Counted Interface Runtime Support
//!
//! Provides runtime functions for COM-style reference-counted interfaces including:
//! - Interface reference counting (AddRef, Release)
//! - Automatic memory management
//! - Interface assignment and lifetime management

use std::sync::atomic::{AtomicU16, Ordering};

/// Reference-counted interface wrapper
/// This represents an interface instance with automatic reference counting
/// Note: This does not implement Clone - use AddRef/Release for reference management
#[derive(Debug)]
pub struct RefCountedInterface {
    /// Pointer to the object implementing the interface
    /// In actual implementation, this would be a pointer to the object's vtable
    object_ptr: usize,
    /// Interface GUID (for type safety and COM compatibility)
    guid: Option<String>,
    /// Reference count (atomic for thread safety)
    /// Using AtomicU16 for 16-bit targets (8-bit/16-bit systems)
    ref_count: AtomicU16,
}

impl RefCountedInterface {
    /// Create a new reference-counted interface from an object pointer
    /// This is called when an object is cast to an interface
    pub fn new(object_ptr: usize, guid: Option<String>) -> Self {
        Self {
            object_ptr,
            guid,
            ref_count: AtomicU16::new(1), // Start with ref count of 1
        }
    }

    /// Get the object pointer
    pub fn object_ptr(&self) -> usize {
        self.object_ptr
    }

    /// Get the interface GUID
    pub fn guid(&self) -> Option<&String> {
        self.guid.as_ref()
    }

    /// Get the current reference count
    pub fn ref_count(&self) -> u16 {
        self.ref_count.load(Ordering::Acquire)
    }
}

/// Runtime function: Add a reference to an interface
/// This increments the reference count
/// Parameters:
/// - interface_ptr: Pointer to the RefCountedInterface structure
/// Returns: The new reference count
pub fn interface_add_ref(interface_ptr: usize) -> u16 {
    // In actual implementation, this would:
    // 1. Cast interface_ptr to RefCountedInterface*
    // 2. Atomically increment ref_count
    // 3. Return the new count

    // For now, we simulate this with a static counter for testing
    // In real implementation, we'd access the actual structure
    unsafe {
        let interface = &*(interface_ptr as *const RefCountedInterface);
        let old_count = interface.ref_count.fetch_add(1, Ordering::AcqRel);
        old_count + 1
    }
}

/// Runtime function: Release a reference to an interface
/// This decrements the reference count and frees the object if count reaches zero
/// Parameters:
/// - interface_ptr: Pointer to the RefCountedInterface structure
/// Returns: The new reference count (0 if object was freed)
pub fn interface_release(interface_ptr: usize) -> u16 {
    // In actual implementation, this would:
    // 1. Cast interface_ptr to RefCountedInterface*
    // 2. Atomically decrement ref_count
    // 3. If count reaches 0, call object destructor and free memory
    // 4. Return the new count

    unsafe {
        let interface = &*(interface_ptr as *const RefCountedInterface);
        let old_count = interface.ref_count.fetch_sub(1, Ordering::AcqRel);

        if old_count == 1 {
            // Last reference - free the object
            // In actual implementation:
            // 1. Get object pointer from interface
            // 2. Call object's destructor (if any)
            // 3. Free the object memory
            // 4. Free the interface wrapper
            0
        } else {
            old_count - 1
        }
    }
}

/// Runtime function: Query interface (COM-style)
/// Check if an object supports a specific interface
/// Parameters:
/// - object_ptr: Pointer to the object
/// - guid: Interface GUID to query
/// Returns: Pointer to interface if supported, 0 otherwise
pub fn interface_query_interface(_object_ptr: usize, _guid: &str) -> usize {
    // In actual implementation, this would:
    // 1. Look up the object's vtable
    // 2. Check if the object implements the interface with the given GUID
    // 3. If yes, create a RefCountedInterface wrapper and return pointer
    // 4. If no, return 0 (null)

    // For now, we return 0 (not implemented in full)
    // This would require vtable support and interface implementation tracking
    0
}

/// Runtime function: Assign interface (with reference counting)
/// This handles interface assignment: dest := source
/// Parameters:
/// - dest_ptr: Pointer to destination interface variable
/// - source_ptr: Pointer to source interface
/// Returns: Pointer to the assigned interface
pub fn interface_assign(dest_ptr: usize, source_ptr: usize) -> usize {
    // In actual implementation, this would:
    // 1. Release the old interface at dest_ptr (if not null)
    // 2. Add reference to source interface
    // 3. Store source_ptr in dest_ptr
    // 4. Return dest_ptr

    // Release old interface
    if dest_ptr != 0 {
        interface_release(dest_ptr);
    }

    // Add reference to source
    if source_ptr != 0 {
        interface_add_ref(source_ptr);
    }

    // Store source in destination (in actual implementation)
    // *dest_ptr = source_ptr;

    source_ptr
}

/// Runtime function: Create interface from object
/// This is called when casting an object to an interface
/// Parameters:
/// - object_ptr: Pointer to the object
/// - guid: Interface GUID
/// Returns: Pointer to RefCountedInterface wrapper
pub fn interface_from_object(object_ptr: usize, guid: Option<String>) -> usize {
    // In actual implementation, this would:
    // 1. Allocate memory for RefCountedInterface structure
    // 2. Initialize with object_ptr and guid
    // 3. Set ref_count to 1
    // 4. Return pointer to the structure

    // For now, we simulate this
    // In real implementation, we'd use a memory allocator
    let interface = RefCountedInterface::new(object_ptr, guid);
    Box::into_raw(Box::new(interface)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refcounted_interface_creation() {
        let interface = RefCountedInterface::new(
            0x1000,
            Some("{12345678-1234-1234-1234-123456789ABC}".to_string()),
        );

        assert_eq!(interface.object_ptr(), 0x1000);
        assert_eq!(interface.ref_count(), 1);
        assert_eq!(
            interface.guid(),
            Some(&"{12345678-1234-1234-1234-123456789ABC}".to_string())
        );
    }

    #[test]
    fn test_refcounted_interface_without_guid() {
        let interface = RefCountedInterface::new(0x2000, None);

        assert_eq!(interface.object_ptr(), 0x2000);
        assert_eq!(interface.ref_count(), 1);
        assert_eq!(interface.guid(), None);
    }

    #[test]
    fn test_interface_add_ref() {
        let interface = Box::new(RefCountedInterface::new(0x3000, None));
        let ptr = Box::into_raw(interface) as usize;

        let count1 = interface_add_ref(ptr);
        assert_eq!(count1, 2);

        let count2 = interface_add_ref(ptr);
        assert_eq!(count2, 3);

        // Clean up
        unsafe {
            let _ = Box::from_raw(ptr as *mut RefCountedInterface);
        }
    }

    #[test]
    fn test_interface_release() {
        let interface = Box::new(RefCountedInterface::new(0x4000, None));
        let ptr = Box::into_raw(interface) as usize;

        // Add some references
        interface_add_ref(ptr);
        interface_add_ref(ptr);

        // Release references
        let count1 = interface_release(ptr);
        assert_eq!(count1, 2);

        let count2 = interface_release(ptr);
        assert_eq!(count2, 1);

        // Last release should free the object
        let count3 = interface_release(ptr);
        assert_eq!(count3, 0);

        // Clean up (interface should already be freed)
    }

    #[test]
    fn test_interface_assign() {
        let source = Box::new(RefCountedInterface::new(0x5000, None));
        let source_ptr = Box::into_raw(source) as usize;

        // Simulate destination (initially null)
        let dest_ptr = 0;

        // Assign interface (this will add a reference)
        let result = interface_assign(dest_ptr, source_ptr);
        assert_eq!(result, source_ptr);

        // Source should now have 2 references (original creation + assignment)
        unsafe {
            let interface = &*(source_ptr as *const RefCountedInterface);
            assert_eq!(interface.ref_count(), 2);
        }

        // Clean up
        // interface_release is a safe function (handles unsafe internally)
        interface_release(source_ptr);
        interface_release(source_ptr);
    }

    #[test]
    fn test_interface_assign_replaces_old() {
        let old_interface = Box::new(RefCountedInterface::new(0x6000, None));
        let old_ptr = Box::into_raw(old_interface) as usize;

        let new_interface = Box::new(RefCountedInterface::new(0x7000, None));
        let new_ptr = Box::into_raw(new_interface) as usize;

        // Assign new interface (replacing old)
        // This will release old_ptr and add reference to new_ptr
        let result = interface_assign(old_ptr, new_ptr);
        assert_eq!(result, new_ptr);

        // Old interface should be released (ref_count should be 0)
        unsafe {
            let old_iface = &*(old_ptr as *const RefCountedInterface);
            assert_eq!(old_iface.ref_count(), 0); // Was released
        }

        // New interface should have 2 references (original creation + assignment)
        unsafe {
            let new_iface = &*(new_ptr as *const RefCountedInterface);
            assert_eq!(new_iface.ref_count(), 2);
        }

        // Clean up
        // interface_release is a safe function (handles unsafe internally)
        interface_release(new_ptr);
        interface_release(new_ptr);
    }

    #[test]
    fn test_interface_from_object() {
        let object_ptr = 0x8000;
        let guid = Some("{87654321-4321-4321-4321-CBA987654321}".to_string());

        let interface_ptr = interface_from_object(object_ptr, guid.clone());

        assert_ne!(interface_ptr, 0);

        unsafe {
            let interface = &*(interface_ptr as *const RefCountedInterface);
            assert_eq!(interface.object_ptr(), object_ptr);
            assert_eq!(interface.ref_count(), 1);
            assert_eq!(interface.guid(), guid.as_ref());
        }

        // Clean up
        unsafe {
            let _ = Box::from_raw(interface_ptr as *mut RefCountedInterface);
        }
    }

    #[test]
    fn test_interface_query_interface() {
        // Query interface is not fully implemented yet
        let object_ptr = 0x9000;
        let guid = "{11111111-1111-1111-1111-111111111111}";

        let result = interface_query_interface(object_ptr, guid);

        // Currently returns 0 (not implemented)
        assert_eq!(result, 0);
    }

    #[test]
    fn test_multiple_interface_references() {
        let interface = Box::new(RefCountedInterface::new(0xA000, None));
        let ptr = Box::into_raw(interface) as usize;

        // Create multiple references
        interface_add_ref(ptr);
        interface_add_ref(ptr);
        interface_add_ref(ptr);

        unsafe {
            let iface = &*(ptr as *const RefCountedInterface);
            assert_eq!(iface.ref_count(), 4); // 1 initial + 3 added
        }

        // Release all references
        interface_release(ptr);
        interface_release(ptr);
        interface_release(ptr);

        // Last release
        let final_count = interface_release(ptr);
        assert_eq!(final_count, 0);
    }

    #[test]
    fn test_interface_lifetime_management() {
        // Test that interfaces are properly managed through their lifetime
        let interface1 = Box::new(RefCountedInterface::new(0xB000, None));
        let ptr1 = Box::into_raw(interface1) as usize;

        let interface2 = Box::new(RefCountedInterface::new(0xC000, None));
        let ptr2 = Box::into_raw(interface2) as usize;

        // Assign interface2 to interface1's location
        interface_assign(ptr1, ptr2);

        // Both should have proper reference counts
        // (In actual implementation, we'd verify the old interface was released)

        unsafe {
            let iface2 = &*(ptr2 as *const RefCountedInterface);
            assert_eq!(iface2.ref_count(), 2); // Original + assignment
        }

        // Clean up
        // interface_release is a safe function (handles unsafe internally)
        interface_release(ptr2);
        interface_release(ptr2);
    }
}
