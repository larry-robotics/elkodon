#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use crate::posix::types::*;

pub unsafe fn scandir(path: *const char, namelist: *mut *mut *mut dirent) -> int {
    internal::scandir_ext(path, namelist)
}

pub unsafe fn mkdir(pathname: *const char, mode: mode_t) -> int {
    crate::internal::mkdir(pathname, mode)
}

pub unsafe fn opendir(dirname: *const char) -> *mut DIR {
    crate::internal::opendir(dirname)
}

pub unsafe fn closedir(dirp: *mut DIR) -> int {
    crate::internal::closedir(dirp)
}

pub unsafe fn dirfd(dirp: *mut DIR) -> int {
    crate::internal::dirfd(dirp)
}

mod internal {
    use super::*;

    #[cfg_attr(target_os = "freebsd", link(name = "c"))]
    extern "C" {
        pub(super) fn scandir_ext(path: *const char, namelist: *mut *mut *mut dirent) -> int;
    }
}
