//! A **threadsafe** and **lock-free** [`Allocator`] which acquires the memory from the heap.

use std::{alloc::Layout, ptr::NonNull};

use elkodon_bb_elementary::allocator::{
    AllocationGrowError, AllocationShrinkError, DeallocationError,
};
use elkodon_bb_log::fail;
use elkodon_bb_posix::memory::heap;

pub use elkodon_bb_elementary::allocator::{AllocationError, Allocator, BaseAllocator};

#[derive(Debug)]
pub struct HeapAllocator {}

impl HeapAllocator {
    pub const fn new() -> HeapAllocator {
        HeapAllocator {}
    }
}

impl BaseAllocator for HeapAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocationError> {
        Ok(fail!(from self, when heap::allocate(layout),
                "Failed to allocate {} bytes with an alignment of {}.", layout.size(), layout.align()))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) -> Result<(), DeallocationError> {
        heap::deallocate(ptr, layout);
        Ok(())
    }
}

impl Allocator for HeapAllocator {
    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocationGrowError> {
        if old_layout.size() >= new_layout.size() {
            fail!(from self, with AllocationGrowError::GrowWouldShrink,
                "Failed to grow memory from (size: {}, align: {}) to (size: {}, align: {}).", old_layout.size(),old_layout.align(), new_layout.size(), new_layout.align());
        }
        Ok(
            fail!(from self, when heap::resize(ptr, old_layout, new_layout),
                "Failed to grow memory from (size: {}, align: {}) to (size: {}, align: {}).", old_layout.size(),old_layout.align(), new_layout.size(), new_layout.align()),
        )
    }

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocationShrinkError> {
        if old_layout.size() <= new_layout.size() {
            fail!(from self, with AllocationShrinkError::ShrinkWouldGrow,
                "Failed to shrink memory from (size: {}, align: {}) to (size: {}, align: {}).", old_layout.size(),old_layout.align(), new_layout.size(), new_layout.align());
        }
        Ok(
            fail!(from self, when heap::resize(ptr, old_layout, new_layout),
                "Failed to shrink memory from (size: {}, align: {}) to (size: {}, align: {}).", old_layout.size(),old_layout.align(), new_layout.size(), new_layout.align()),
        )
    }
}
