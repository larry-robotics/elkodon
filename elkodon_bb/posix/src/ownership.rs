//! Represents the [`Ownership`] in a unix environment consisting of user and group. Can be used in
//! combination with [`crate::file_descriptor::FileDescriptorManagement`] to set the
//! credentials of [`crate::file::File`], [`crate::shared_memory::SharedMemory`] and others.
//! # Example
//!
//! ```
//! use elkodon_bb_posix::ownership::*;
//! use elkodon_bb_posix::user::UserExt;
//! use elkodon_bb_posix::group::GroupExt;
//!
//! let ownership = OwnershipBuilder::new().uid("root".as_user().expect("no such user").uid())
//!                                        .gid("root".as_group().expect("no such group").gid())
//!                                        .create();
//!
//! println!("The uid/gid of root/root is: {}/{}", ownership.uid(), ownership.gid());
//! ```

/// Defines the owner in a unix environment consisting of user and group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ownership {
    uid: u32,
    gid: u32,
}

/// The builder to the [`Ownership`] struct.
/// One can use [`crate::user::User`] and [`crate::group::Group`] to acquire the ids quickly from
/// the names.
pub struct OwnershipBuilder {
    ownership: Ownership,
}

impl Default for OwnershipBuilder {
    fn default() -> Self {
        OwnershipBuilder {
            ownership: Ownership {
                uid: u32::MAX,
                gid: u32::MAX,
            },
        }
    }
}

impl OwnershipBuilder {
    pub fn new() -> OwnershipBuilder {
        Self::default()
    }

    /// Sets the user id
    pub fn uid(mut self, uid: u32) -> Self {
        self.ownership.uid = uid;
        self
    }

    /// Sets the group id
    pub fn gid(mut self, gid: u32) -> Self {
        self.ownership.gid = gid;
        self
    }

    pub fn create(self) -> Ownership {
        self.ownership
    }
}

impl Ownership {
    /// returns the user id
    pub fn uid(&self) -> u32 {
        self.uid
    }

    /// returns the group id
    pub fn gid(&self) -> u32 {
        self.gid
    }
}
