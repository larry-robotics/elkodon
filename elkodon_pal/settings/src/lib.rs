#![no_std]

#[cfg(not(target_os = "windows"))]
pub mod settings {
    pub const TEMP_DIRECTORY: &[u8] = b"/tmp/";
    pub const SHARED_MEMORY_DIRECTORY: &[u8] = b"/dev/shm/";
    pub const PATH_SEPARATOR: u8 = b'/';
    pub const ROOT: &[u8] = b"/";
    pub const FILENAME_LENGTH: usize = 255;
    pub const PATH_LENGTH: usize = 4096;
}
#[cfg(not(target_os = "windows"))]
pub use settings::*;

#[cfg(target_os = "windows")]
pub mod settings {
    pub const TEMP_DIRECTORY: &[u8] = b"C:\\Windows\\Temp\\";
    pub const SHARED_MEMORY_DIRECTORY: &[u8] = b"C:\\Windows\\Temp\\Shm\\";
    pub const PATH_SEPARATOR: u8 = b'\\';
    pub const ROOT: &[u8] = b"C:\\";
    pub const FILENAME_LENGTH: usize = 255;
    pub const PATH_LENGTH: usize = 255;
}
#[cfg(target_os = "windows")]
pub use settings::*;
