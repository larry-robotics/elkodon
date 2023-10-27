#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

pub unsafe fn htonl(hostlong: u32) -> u32 {
    crate::internal::htonl(hostlong)
}

pub unsafe fn htons(hostshort: u16) -> u16 {
    crate::internal::htons(hostshort)
}

pub unsafe fn ntohl(netlong: u32) -> u32 {
    crate::internal::ntohl(netlong)
}

pub unsafe fn ntohs(netshort: u16) -> u16 {
    crate::internal::ntohs(netshort)
}
