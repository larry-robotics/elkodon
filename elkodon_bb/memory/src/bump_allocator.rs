//! A **threadsafe** and **lock-free** bump allocator which implements the [`BaseAllocator`].
//! It can be allocated with [`BumpAllocator::allocate()`] but [`BumpAllocator::deallocate`]
//! deallocate all allocated chunks. See this: `https://os.phil-opp.com/allocator-designs/`
//! for more details.

use std::{
    fmt::Display,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

use elkodon_bb_elementary::{allocator::DeallocationError, math::align};
use elkodon_bb_log::fail;

pub use elkodon_bb_elementary::allocator::{AllocationError, BaseAllocator};

#[derive(Debug)]
pub struct BumpAllocator {
    pub(crate) start: usize,
    size: usize,
    current_position: AtomicUsize,
}

impl Display for BumpAllocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BumpAllocator {{ start: {}, size: {}, current_position: {} }}",
            self.start,
            self.size,
            self.current_position
                .load(std::sync::atomic::Ordering::Relaxed)
        )
    }
}

impl BumpAllocator {
    pub fn new(ptr: NonNull<u8>, size: usize) -> Self {
        Self {
            start: ptr.as_ptr() as usize,
            size,
            current_position: AtomicUsize::new(0),
        }
    }

    pub fn used_space(&self) -> usize {
        self.current_position.load(Ordering::Relaxed)
    }

    pub fn free_space(&self) -> usize {
        self.size - self.used_space()
    }

    pub fn total_space(&self) -> usize {
        self.size
    }
}

impl BaseAllocator for BumpAllocator {
    fn allocate(&self, layout: std::alloc::Layout) -> Result<NonNull<[u8]>, AllocationError> {
        let msg = "Unable to allocate chunk with";
        let mut aligned_position;

        if layout.size() == 0 {
            fail!(from self, with AllocationError::SizeIsZero,
                "{} {:?} since the requested size was zero.", msg, layout);
        }

        let mut current_position = self
            .current_position
            .load(std::sync::atomic::Ordering::Relaxed);
        loop {
            aligned_position = align(self.start + current_position, layout.align()) - self.start;
            if aligned_position + layout.size() > self.size {
                fail!(from self, with AllocationError::OutOfMemory,
                    "{} {:?} since there is not enough memory available.", msg, layout);
            }

            match self.current_position.compare_exchange_weak(
                current_position,
                aligned_position + layout.size(),
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(v) => current_position = v,
            }
        }

        Ok(unsafe {
            NonNull::new_unchecked(std::slice::from_raw_parts_mut(
                (self.start + aligned_position) as *mut u8,
                layout.size(),
            ))
        })
    }

    unsafe fn deallocate(
        &self,
        _ptr: NonNull<u8>,
        _layout: std::alloc::Layout,
    ) -> Result<(), DeallocationError> {
        self.current_position
            .store(0, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}
