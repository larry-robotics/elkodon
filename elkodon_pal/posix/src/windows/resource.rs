#![allow(non_camel_case_types, non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(unused_variables)]

use crate::posix::types::*;

pub unsafe fn getrlimit(resource: int, rlim: *mut rlimit) -> int {
    0
}

pub unsafe fn setrlimit(resource: int, rlim: *const rlimit) -> int {
    0
}
