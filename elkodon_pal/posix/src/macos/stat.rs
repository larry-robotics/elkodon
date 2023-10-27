#![allow(non_camel_case_types, non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use crate::posix::types::*;
use crate::posix::Struct;

pub unsafe fn stat(path: *const char, buf: *mut stat_t) -> int {
    let mut os_specific_buffer = crate::internal::stat::new();
    match crate::internal::stat(path, &mut os_specific_buffer) {
        0 => {
            *buf = os_specific_buffer.into();
            0
        }
        v => v,
    }
}

pub unsafe fn umask(mask: mode_t) -> mode_t {
    crate::internal::umask(mask)
}
