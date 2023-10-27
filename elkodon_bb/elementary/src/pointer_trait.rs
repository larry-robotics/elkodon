/// Trait which describes a form of pointer. Required to distinguish normal pointers from
/// relocatable pointers.
pub trait PointerTrait<T> {
    /// Return a pointer to the underlying const type
    ///
    /// # Safety
    ///
    ///  * Do not call this method when the pointer contains a null pointer.
    ///
    unsafe fn as_ptr(&self) -> *const T;

    /// Return a pointer to the underlying mutable type
    ///
    /// # Safety
    ///
    ///  * Do not call this method when the pointer contains a null pointer.
    ///
    unsafe fn as_mut_ptr(&mut self) -> *mut T;
}
