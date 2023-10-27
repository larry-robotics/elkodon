#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
use crate::posix::types::*;

pub unsafe fn sigaction(sig: int, act: *const sigaction_t, oact: *mut sigaction_t) -> int {
    crate::internal::sigaction(
        sig,
        act as *const crate::internal::sigaction,
        oact as *mut crate::internal::sigaction,
    )
}

pub unsafe fn kill(pid: pid_t, sig: int) -> int {
    crate::internal::kill(pid, sig)
}
