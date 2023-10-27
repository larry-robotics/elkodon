//! The [`CreationMode`] describes how certain posix resources should be created.

use elkodon_pal_posix::*;
use std::fmt::Display;

/// Describes how new resources like [`crate::file::File`], [`crate::shared_memory::SharedMemory`]
/// or others should be created.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Default)]
pub enum CreationMode {
    /// Create resource, if its already existing fail.
    #[default]
    CreateExclusive,
    /// Always remove existing resource and override it with new one
    PurgeAndCreate,
    /// Either open the new resource or create it when it is not existing
    OpenOrCreate,
}

impl CreationMode {
    pub fn as_oflag(&self) -> posix::int {
        match self {
            CreationMode::PurgeAndCreate => posix::O_CREAT | posix::O_EXCL,
            CreationMode::CreateExclusive => posix::O_CREAT | posix::O_EXCL,
            CreationMode::OpenOrCreate => posix::O_CREAT,
        }
    }
}

impl Display for CreationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CreationMode: {}",
            match self {
                CreationMode::CreateExclusive => "CreateExclusive",
                CreationMode::PurgeAndCreate => "PurgeAndCreate",
                CreationMode::OpenOrCreate => "OpenOrCreate",
            }
        )
    }
}
