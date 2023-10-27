#![allow(non_camel_case_types, non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use crate::posix::types::*;

pub unsafe fn getrlimit(resource: int, rlim: *mut rlimit) -> int {
    crate::internal::getrlimit(resource, rlim)
}

pub unsafe fn setrlimit(resource: int, rlim: *const rlimit) -> int {
    crate::internal::setrlimit(resource, rlim)
}
