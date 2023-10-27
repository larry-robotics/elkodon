#![allow(non_camel_case_types, non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use crate::posix::types::*;
use crate::posix::Struct;

pub unsafe fn open_with_mode(pathname: *const char, flags: int, mode: mode_t) -> int {
    crate::internal::open(pathname, flags, mode as core::ffi::c_uint)
}

pub unsafe fn fstat(fd: int, buf: *mut stat_t) -> int {
    let mut os_specific_buffer = crate::internal::stat::new();
    match crate::internal::fstat(fd, &mut os_specific_buffer) {
        0 => {
            *buf = os_specific_buffer.into();
            0
        }
        v => v,
    }
}

pub unsafe fn fcntl_int(fd: int, cmd: int, arg: int) -> int {
    crate::internal::fcntl(fd, cmd, arg)
}

pub unsafe fn fcntl(fd: int, cmd: int, arg: *mut flock) -> int {
    crate::internal::fcntl(fd, cmd, arg)
}

pub unsafe fn fcntl2(fd: int, cmd: int) -> int {
    crate::internal::fcntl(fd, cmd)
}

pub unsafe fn fchmod(fd: int, mode: mode_t) -> int {
    crate::internal::fchmod(fd, mode)
}

pub unsafe fn open(pathname: *const char, flags: int) -> int {
    crate::internal::open(pathname, flags)
}
