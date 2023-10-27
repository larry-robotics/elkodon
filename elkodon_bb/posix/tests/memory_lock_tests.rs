use elkodon_bb_posix::memory_lock::*;
use elkodon_bb_testing::{assert_that, test_requires};
use elkodon_pal_posix::posix;
use elkodon_pal_posix::posix::POSIX_SUPPORT_MEMORY_LOCK;

#[test]
fn memory_lock_works() {
    test_requires!(POSIX_SUPPORT_MEMORY_LOCK);

    let some_memory = [0u8; 1024];

    {
        let mem_lock = unsafe {
            MemoryLock::new(
                some_memory.as_ptr() as *const posix::void,
                some_memory.len(),
            )
        };
        assert_that!(mem_lock, is_ok);
    }
}
