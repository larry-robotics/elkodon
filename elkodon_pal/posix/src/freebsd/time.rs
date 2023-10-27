#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use crate::posix::types::*;

pub unsafe fn clock_gettime(clock_id: clockid_t, tp: *mut timespec) -> int {
    crate::internal::clock_gettime(clock_id, tp)
}

pub unsafe fn clock_settime(clock_id: clockid_t, tp: *const timespec) -> int {
    crate::internal::clock_settime(clock_id, tp)
}

pub unsafe fn clock_nanosleep(
    clock_id: clockid_t,
    flags: int,
    rqtp: *const timespec,
    rmtp: *mut timespec,
) -> int {
    crate::internal::clock_nanosleep(clock_id, flags, rqtp, rmtp)
}
