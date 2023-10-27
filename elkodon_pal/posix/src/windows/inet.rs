#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

pub unsafe fn htonl(hostlong: u32) -> u32 {
    windows_sys::Win32::Networking::WinSock::htonl(hostlong)
}

pub unsafe fn htons(hostshort: u16) -> u16 {
    windows_sys::Win32::Networking::WinSock::htons(hostshort)
}

pub unsafe fn ntohl(netlong: u32) -> u32 {
    windows_sys::Win32::Networking::WinSock::ntohl(netlong)
}

pub unsafe fn ntohs(netshort: u16) -> u16 {
    windows_sys::Win32::Networking::WinSock::ntohs(netshort)
}
