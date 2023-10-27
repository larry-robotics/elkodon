use windows_sys::Win32::Foundation::MAX_PATH;

pub(crate) const MAX_PATH_LENGTH: usize = MAX_PATH as usize;
pub(crate) const SHM_STATE_DIRECTORY: &[u8] = elkodon_pal_settings::TEMP_DIRECTORY;
pub(crate) const SHM_STATE_SUFFIX: &[u8] = b".shm_state";
#[doc(hidden)]
pub const FD_SET_CAPACITY: usize = 64;
