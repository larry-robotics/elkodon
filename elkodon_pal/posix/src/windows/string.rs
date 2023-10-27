#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use crate::posix::types::*;

pub unsafe fn memset(s: *mut void, c: int, n: size_t) -> *mut void {
    crate::internal::memset(s, c, n as _)
}

pub unsafe fn memcpy(dest: *mut void, src: *const void, n: size_t) -> *mut void {
    crate::internal::memcpy(dest, src, n as _)
}

pub unsafe fn strncpy(dest: *mut char, src: *const char, n: size_t) -> *mut char {
    crate::internal::strncpy(dest, src, n as _)
}
