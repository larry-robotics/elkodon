//! Contains basic constructs which do not have any kind of dependency.

#[macro_use]
pub mod enum_gen;
pub mod allocator;
pub mod lazy_singleton;
pub mod math;
pub mod owning_pointer;
pub mod pointer_trait;
pub mod relocatable_container;
pub mod relocatable_ptr;
pub mod scope_guard;
pub mod unique_id;
