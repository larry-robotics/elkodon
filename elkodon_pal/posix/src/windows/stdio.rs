#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
#![allow(unused_variables)]

use windows_sys::Win32::Foundation::{ERROR_ACCESS_DENIED, ERROR_FILE_NOT_FOUND};
use windows_sys::Win32::Storage::FileSystem::DeleteFileA;

use crate::posix::types::*;

use crate::win32call;

use super::win32_handle_translator::HandleTranslator;

pub unsafe fn remove(pathname: *const char) -> int {
    if win32call! { DeleteFileA(pathname as *const u8), ignore ERROR_FILE_NOT_FOUND, ERROR_ACCESS_DENIED }
        == 0
    {
        if HandleTranslator::get_instance().remove_uds(pathname) {
            return 0;
        }

        return -1;
    }

    0
}
