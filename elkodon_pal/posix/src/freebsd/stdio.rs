#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use crate::posix::types::*;

pub unsafe fn remove(pathname: *const char) -> int {
    crate::internal::remove(pathname)
}
