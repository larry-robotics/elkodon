#![no_std]

#[cfg(not(target_os = "windows"))]
pub mod settings {
    pub const TEMP_DIRECTORY: &[u8] = b"/tmp/";
    pub const TEST_DIRECTORY: &[u8] = b"/tmp/elkodon/tests/";
    pub const SHARED_MEMORY_DIRECTORY: &[u8] = b"/dev/shm/";
    pub const PATH_SEPARATOR: u8 = b'/';
    pub const ROOT: &[u8] = b"/";
    pub const FILENAME_LENGTH: usize = 255;
    pub const PATH_LENGTH: usize = 4096;
    pub const AT_LEAST_TIMING_VARIANCE: f32 = 0.1;
}
#[cfg(not(target_os = "windows"))]
pub use settings::*;

#[cfg(target_os = "windows")]
pub mod settings {
    pub const TEMP_DIRECTORY: &[u8] = b"C:\\Windows\\Temp\\";
    pub const TEST_DIRECTORY: &[u8] = b"C:\\Windows\\Temp\\elkodon\\tests\\";
    pub const SHARED_MEMORY_DIRECTORY: &[u8] = b"C:\\Windows\\Temp\\Shm\\";
    pub const PATH_SEPARATOR: u8 = b'\\';
    pub const ROOT: &[u8] = b"C:\\";
    pub const FILENAME_LENGTH: usize = 255;
    pub const PATH_LENGTH: usize = 255;
    pub const AT_LEAST_TIMING_VARIANCE: f32 = 1.0;
}
#[cfg(target_os = "windows")]
pub use settings::*;
