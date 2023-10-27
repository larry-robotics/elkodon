//! Contains two building blocks to generate unique ids. Useful for parallized unit test to
//! generate names which point to OS resources or to enumerate constructs uniquely.
//!
//! # Examples
//!
//! ## [`UniqueId`]
//!
//! The [`UniqueId`] is globally unique.
//!
//! ```
//! use elkodon_bb_elementary::unique_id::UniqueId;
//!
//! struct MyThing {
//!     unique_id: UniqueId,
//! }
//!
//! impl MyThing {
//!     fn new() -> Self {
//!         Self {
//!             unique_id: UniqueId::new()
//!         }
//!     }
//!
//!     fn id(&self) -> u64 {
//!         self.unique_id.value()
//!     }
//! }
//! ```
//!
//! ## [`TypedUniqueId`]
//!
//! The [`TypedUniqueId`] is unique for a given type.
//!
//! ```
//! use elkodon_bb_elementary::unique_id::TypedUniqueId;
//!
//! struct MyThing {
//!     unique_id: TypedUniqueId<MyThing>,
//! }
//!
//! impl MyThing {
//!     fn new() -> Self {
//!         Self {
//!             unique_id: TypedUniqueId::new()
//!         }
//!     }
//!
//!     fn id(&self) -> u64 {
//!         self.unique_id.value()
//!     }
//! }
//! ```

use std::{
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

/// A building block to generate global unique ids
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct UniqueId {
    value: u64,
}

impl Default for UniqueId {
    fn default() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        UniqueId {
            value: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }
}

impl UniqueId {
    /// Creates a new unique id
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the underlying integer value of the unique id
    pub fn value(&self) -> u64 {
        self.value
    }
}

/// A building block to generate per type global unique ids. It is allowed that different types
/// have the same id but never the same type.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct TypedUniqueId<T> {
    value: u64,
    _phantom: PhantomData<T>,
}

impl<T> Default for TypedUniqueId<T> {
    fn default() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        Self {
            value: COUNTER.fetch_add(1, Ordering::Relaxed),
            _phantom: PhantomData,
        }
    }
}

impl<T> TypedUniqueId<T> {
    /// Creates a new unique id
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the underlying integer value of the unique id
    pub fn value(&self) -> u64 {
        self.value
    }
}
