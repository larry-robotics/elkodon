//! [`FileType`] describes the type of files which can be used in a POSIX system.

use elkodon_pal_posix::*;

/// Represents a file type in a POSIX system.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Default)]
#[repr(u32)]
pub enum FileType {
    #[default]
    File = posix::S_IFREG as _,
    Character = posix::S_IFCHR as _,
    Block = posix::S_IFBLK as _,
    Directory = posix::S_IFDIR as _,
    SymbolicLink = posix::S_IFLNK as _,
    Socket = posix::S_IFSOCK as _,
    FiFo = posix::S_IFIFO as _,
    Unknown = u32::MAX as _,
}

impl FileType {
    /// creates a FileType from the c representation [`posix::mode_t`].
    pub fn from_mode_t(value: posix::mode_t) -> FileType {
        let v = value & posix::S_IFMT;

        match v {
            posix::S_IFREG => FileType::File,
            posix::S_IFCHR => FileType::Character,
            posix::S_IFBLK => FileType::Block,
            posix::S_IFDIR => FileType::Directory,
            posix::S_IFLNK => FileType::SymbolicLink,
            posix::S_IFSOCK => FileType::Socket,
            posix::S_IFIFO => FileType::FiFo,
            _ => FileType::Unknown,
        }
    }
}
