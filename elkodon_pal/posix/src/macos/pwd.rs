#![allow(non_camel_case_types, non_snake_case)]
#![allow(clippy::missing_safety_doc)]
use crate::posix::types::*;

pub unsafe fn getpwnam_r(
    name: *const char,
    pwd: *mut passwd,
    buf: *mut char,
    buflen: size_t,
    result: *mut *mut passwd,
) -> int {
    crate::internal::getpwnam_r(name, pwd, buf, buflen, result)
}

pub unsafe fn getpwuid_r(
    uid: uid_t,
    pwd: *mut passwd,
    buf: *mut char,
    buflen: size_t,
    result: *mut *mut passwd,
) -> int {
    crate::internal::getpwuid_r(uid, pwd, buf, buflen, result)
}

pub unsafe fn getgrnam_r(
    name: *const char,
    grp: *mut group,
    buf: *mut char,
    buflen: size_t,
    result: *mut *mut group,
) -> int {
    crate::internal::getgrnam_r(name, grp, buf, buflen, result)
}

pub unsafe fn getgrgid_r(
    gid: gid_t,
    grp: *mut group,
    buf: *mut char,
    buflen: size_t,
    result: *mut *mut group,
) -> int {
    crate::internal::getgrgid_r(gid, grp, buf, buflen, result)
}
