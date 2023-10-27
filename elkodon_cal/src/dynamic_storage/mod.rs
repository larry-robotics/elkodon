//! Traits that provide modifyable memory which can be accessed by multiple processes
//! identified by a name.
//!
//! A [`DynamicStorage`] has to fulfill the following contract:
//!  * zero sized names are not valid
//!  * **unique:** multiple [`DynamicStorage`]s with the same cannot be created
//!  * non-existing [`DynamicStorage`]s cannot be opened
//!
//! The contract is verified by the corresponding unit tests. Every [`DynamicStorage`] must
//! pass the test.
//!
//! **Important:** It is not the task of the [`DynamicStorage`] to ensure a thread-safe access to
//! the underlying object. If the [`DynamicStorage`] is used in an inter-process environment every
//! access must be considered as a concurrent access!
//!
//! # Example
//!
//! ```
//! use elkodon_bb_system_types::file_name::FileName;
//! use elkodon_bb_container::semantic_string::SemanticString;
//! use elkodon_cal::dynamic_storage::*;
//! use elkodon_cal::named_concept::*;
//! use std::sync::atomic::{AtomicU64, Ordering};
//!
//! // the following two functions can be implemented in different processes
//! fn process_one<Storage: DynamicStorage<AtomicU64>>() {
//!     let storage_name = FileName::new(b"myStorageName").unwrap();
//!     let mut storage = Storage::Builder::new(&storage_name)
//!                         .create(AtomicU64::new(873)).unwrap();
//!
//!     println!("Created storage {}", storage.name());
//!     storage.get().store(991, Ordering::Relaxed);
//! }
//!
//! fn process_two<Storage: DynamicStorage<AtomicU64>>() {
//!     let storage_name = FileName::new(b"myStorageName").unwrap();
//!     let mut storage = Storage::Builder::new(&storage_name)
//!                         .open().unwrap();
//!
//!     println!("Opened storage {}", storage.name());
//!     println!("Current value {}", storage.get().swap(1001, Ordering::Relaxed));
//! }
//! ```

use std::fmt::Debug;

use elkodon_bb_memory::bump_allocator::BumpAllocator;
use elkodon_bb_posix::config::TEMP_DIRECTORY;
use elkodon_bb_system_types::file_name::FileName;
use elkodon_bb_system_types::path::Path;

use crate::static_storage::file::{NamedConcept, NamedConceptBuilder, NamedConceptMgmt};

pub mod posix_shared_memory;
pub mod process_local;

/// The default suffix of every dynamic storage
pub const DEFAULT_SUFFIX: FileName = unsafe { FileName::new_unchecked(b".dyn") };

/// The default path hint for every dynamic storage
pub const DEFAULT_PATH_HINT: Path = TEMP_DIRECTORY;

/// Describes failures when creating a new [`DynamicStorage`]
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum DynamicStorageCreateError {
    AlreadyExists,
    Creation,
    Write,
    InitializationFailed,
    InternalError,
}

/// Describes failures when opening a new [`DynamicStorage`]
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum DynamicStorageOpenError {
    DoesNotExist,
    Open,
    InitializationNotYetFinalized,
    InternalError,
}

/// Builder for the [`DynamicStorage`]. T is not allowed to implement the [`Drop`] trait.
pub trait DynamicStorageBuilder<T: Send + Sync, D: DynamicStorage<T>>:
    Debug + Sized + NamedConceptBuilder<D>
{
    /// Defines if a newly created [`DynamicStorage`] owns the underlying resources
    fn has_ownership(self, value: bool) -> Self;

    /// Sets the size of the supplementary data
    fn supplementary_size(self, value: usize) -> Self;

    /// Creates a new [`DynamicStorage`]. The returned object has the ownership of the
    /// [`DynamicStorage`] and when it goes out of scope the underlying resources shall be
    /// removed without corrupting already opened [`DynamicStorage`]s.
    fn create(self, initial_value: T) -> Result<D, DynamicStorageCreateError> {
        self.create_and_initialize(initial_value, |_, _| true)
    }

    /// Creates a new [`DynamicStorage`]. Before the construction is finalized the initializer
    /// with a mutable reference to the new value and a mutable reference to a bump allocator
    /// which provides access to the supplementary memory.
    fn create_and_initialize<F: FnOnce(&mut T, &mut BumpAllocator) -> bool>(
        self,
        initial_value: T,
        initializer: F,
    ) -> Result<D, DynamicStorageCreateError>;

    /// Opens a [`DynamicStorage`]. The implementation must ensure that a [`DynamicStorage`]
    /// which is in the midst of creation cannot be opened.
    fn open(self) -> Result<D, DynamicStorageOpenError>;

    /// Opens a [`DynamicStorage`]. The implementation must ensure that a [`DynamicStorage`]
    /// which is in the midst of creation cannot be opened. In contrast to the counterpart
    /// [`DynamicStorageBuilder::open()`] it does not print an error message when the channel
    /// does not exist or is not yet finalized.
    fn try_open(self) -> Result<D, DynamicStorageOpenError>;
}

/// Is being built by the [`DynamicStorageBuilder`]. The [`DynamicStorage`] trait shall provide
/// inter-process access to a modifyable piece of memory identified by some name.
pub trait DynamicStorage<T: Send + Sync>: Sized + Debug + NamedConceptMgmt + NamedConcept {
    type Builder: DynamicStorageBuilder<T, Self>;

    /// Returns if the dynamic storage supports persistency, meaning that the underlying OS
    /// resource remain even when every dynamic storage instance in every process was removed.
    fn does_support_persistency() -> bool;

    /// Returns true if the storage holds the ownership, otherwise false.
    fn has_ownership(&self) -> bool;

    /// Releases the ownership of the storage. When the object goes out of scope it is no longer
    /// removed.
    fn release_ownership(&mut self);

    /// Acquires the ownership of the storage. When the object goes out of scope the underlying
    /// resources will be removed.
    fn acquire_ownership(&mut self);

    /// Returns a const reference to the underlying object. It is const since the [`DynamicStorage`]
    /// can be accessed by multiple processes concurrently therefore it must be constant or
    /// thread-safe.
    fn get(&self) -> &T;
}
