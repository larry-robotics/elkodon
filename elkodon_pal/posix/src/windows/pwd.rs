#![allow(non_camel_case_types, non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(unused_variables)]

use crate::posix::types::*;

pub unsafe fn getpwnam_r(
    name: *const char,
    pwd: *mut passwd,
    buf: *mut char,
    buflen: size_t,
    result: *mut *mut passwd,
) -> int {
    -1
}

pub unsafe fn getpwuid_r(
    uid: uid_t,
    pwd: *mut passwd,
    buf: *mut char,
    buflen: size_t,
    result: *mut *mut passwd,
) -> int {
    -1
}

pub unsafe fn getgrnam_r(
    name: *const char,
    grp: *mut group,
    buf: *mut char,
    buflen: size_t,
    result: *mut *mut group,
) -> int {
    -1
}

pub unsafe fn getgrgid_r(
    gid: gid_t,
    grp: *mut group,
    buf: *mut char,
    buflen: size_t,
    result: *mut *mut group,
) -> int {
    -1
}
