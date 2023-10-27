#![allow(non_camel_case_types, dead_code)]
#![allow(clippy::missing_safety_doc)]
#![allow(unused_variables)]

use windows_sys::Win32::System::Threading::SwitchToThread;

use crate::posix::types::*;

pub unsafe fn sched_get_priority_max(policy: int) -> int {
    3
}

pub unsafe fn sched_get_priority_min(policy: int) -> int {
    -3
}

pub unsafe fn sched_yield() -> int {
    SwitchToThread();
    0
}

pub unsafe fn sched_getparam(pid: pid_t, param: *mut sched_param) -> int {
    -1
}

pub unsafe fn sched_getscheduler(pid: pid_t) -> int {
    -1
}

pub unsafe fn sched_setparam(pid: pid_t, param: *const sched_param) -> int {
    -1
}

pub unsafe fn sched_setscheduler(pid: pid_t, policy: int, param: *const sched_param) -> int {
    -1
}
