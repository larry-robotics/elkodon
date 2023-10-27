#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use crate::posix::types::*;

pub unsafe fn malloc(size: size_t) -> *mut void {
    crate::internal::malloc(size as _)
}

pub unsafe fn calloc(nmemb: size_t, size: size_t) -> *mut void {
    crate::internal::calloc(nmemb as _, size as _)
}

pub unsafe fn realloc(ptr: *mut void, size: size_t) -> *mut void {
    crate::internal::realloc(ptr, size as _)
}

pub unsafe fn free(ptr: *mut void) {
    crate::internal::free(ptr)
}
