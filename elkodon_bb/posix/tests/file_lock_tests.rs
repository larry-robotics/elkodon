use elkodon_bb_container::semantic_string::SemanticString;
use elkodon_bb_posix::config::*;
use elkodon_bb_posix::file::*;
use elkodon_bb_posix::file_lock::*;
use elkodon_bb_posix::process::*;
use elkodon_bb_posix::read_write_mutex::ReadWriteMutexHandle;
use elkodon_bb_posix::unique_system_id::UniqueSystemId;
use elkodon_bb_system_types::file_name::FileName;
use elkodon_bb_system_types::file_path::FilePath;
use elkodon_bb_testing::assert_that;
use elkodon_bb_testing::test_requires;
use elkodon_pal_posix::posix::POSIX_SUPPORT_FILE_LOCK;

use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

fn generate_file_name() -> FilePath {
    let mut file = FileName::new(b"file_lock_tests_").unwrap();
    file.push_bytes(
        UniqueSystemId::new()
            .unwrap()
            .value()
            .to_string()
            .as_bytes(),
    )
    .unwrap();

    FilePath::from_path_and_file(&TEMP_DIRECTORY, &file).unwrap()
}

const TIMEOUT: Duration = Duration::from_millis(10);

struct TestFixture<'a> {
    file_name: FilePath,
    sut: FileLock<'a, File>,
}

impl<'a> TestFixture<'a> {
    fn new(handle: &'a ReadWriteMutexHandle<File>) -> TestFixture {
        let file_name = generate_file_name();
        let file = FileBuilder::new(&file_name)
            .creation_mode(CreationMode::PurgeAndCreate)
            .permission(Permission::OWNER_ALL)
            .create()
            .expect("");

        TestFixture {
            file_name,
            sut: FileLockBuilder::new().create(file, handle).expect(""),
        }
    }
}

impl<'a> Drop for TestFixture<'a> {
    fn drop(&mut self) {
        File::remove(&self.file_name).expect("");
    }
}

#[test]
fn file_lock_unlocked_by_default() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Unlock);
    assert_that!(result.pid_of_owner().value(), eq 0);
}

#[test]
fn file_lock_write_lock_blocks_other_write_locks() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);
    let guard = test.sut.write_lock().unwrap();

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Write);
    assert_that!(result.pid_of_owner(), eq Process::from_self().id());

    drop(guard);

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Unlock);
    assert_that!(result.pid_of_owner().value(), eq 0);
}

#[test]
fn file_lock_write_try_lock_denies_other_try_locks() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);
    let guard = test.sut.write_try_lock();

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Write);
    assert_that!(result.pid_of_owner(), eq Process::from_self().id());

    assert_that!(test.sut.write_try_lock().unwrap(), is_none);

    drop(guard);

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Unlock);
    assert_that!(result.pid_of_owner().value(), eq 0);

    assert_that!(test.sut.write_try_lock().unwrap(), is_some);
}

#[test]
fn file_lock_write_timed_lock_dinies_other_timed_locks() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);
    let guard = test.sut.write_timed_lock(TIMEOUT).unwrap();

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Write);
    assert_that!(result.pid_of_owner(), eq Process::from_self().id());

    assert_that!(test.sut.write_timed_lock(TIMEOUT).unwrap(), is_none);

    drop(guard);

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Unlock);
    assert_that!(result.pid_of_owner().value(), eq 0);

    assert_that!(test.sut.write_timed_lock(TIMEOUT).unwrap(), is_some);
}

#[test]
fn file_lock_read_lock_allows_other_read_locks() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);
    let guard = test.sut.read_lock().unwrap();

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Read);
    assert_that!(result.pid_of_owner(), eq Process::from_self().id());

    let guard2 = test.sut.read_lock().unwrap();
    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Read);
    assert_that!(result.pid_of_owner(), eq Process::from_self().id());

    drop(guard);

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Read);
    assert_that!(result.pid_of_owner(), eq Process::from_self().id());

    drop(guard2);

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Unlock);
    assert_that!(result.pid_of_owner().value(), eq 0);
}

#[test]
fn file_lock_read_try_lock_allows_other_read_try_locks() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);
    let guard = test.sut.read_try_lock().unwrap();

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Read);
    assert_that!(result.pid_of_owner(), eq Process::from_self().id());

    let guard2 = test.sut.read_try_lock().unwrap();
    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Read);
    assert_that!(result.pid_of_owner(), eq Process::from_self().id());

    drop(guard);

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Read);
    assert_that!(result.pid_of_owner(), eq Process::from_self().id());

    drop(guard2);

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Unlock);
    assert_that!(result.pid_of_owner().value(), eq 0);
}

#[test]
fn file_lock_read_timed_lock_allows_other_read_timed_locks() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);
    let guard = test.sut.read_timed_lock(TIMEOUT).unwrap();

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Read);
    assert_that!(result.pid_of_owner(), eq Process::from_self().id());

    let guard2 = test.sut.read_timed_lock(TIMEOUT).unwrap();
    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Read);
    assert_that!(result.pid_of_owner(), eq Process::from_self().id());

    drop(guard);

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Read);
    assert_that!(result.pid_of_owner(), eq Process::from_self().id());

    drop(guard2);

    let result = test.sut.get_lock_state().unwrap();
    assert_that!(result.lock_type(), eq LockType::Unlock);
    assert_that!(result.pid_of_owner().value(), eq 0);
}

#[test]
fn file_lock_one_read_blocks_write() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);
    let guard = test.sut.read_lock().unwrap();

    assert_that!(test.sut.write_try_lock().unwrap(), is_none);
    drop(guard);
    assert_that!(test.sut.write_try_lock().unwrap(), is_some);
}

#[test]
fn file_lock_multiple_readers_blocks_write() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);
    let guard = test.sut.read_lock().unwrap();
    let guard2 = test.sut.read_lock().unwrap();

    assert_that!(test.sut.write_try_lock().unwrap(), is_none);
    drop(guard2);
    assert_that!(test.sut.write_try_lock().unwrap(), is_none);
    drop(guard);
    assert_that!(test.sut.write_try_lock().unwrap(), is_some);
}

#[test]
fn file_lock_write_lock_blocks() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);
    let counter = AtomicU64::new(0);
    thread::scope(|s| {
        let guard = test.sut.write_lock().expect("");

        s.spawn(|| {
            test.sut.read_lock().expect("");
            counter.fetch_add(1, Ordering::Relaxed);
        });

        s.spawn(|| {
            test.sut.write_lock().expect("");
            counter.fetch_add(1, Ordering::Relaxed);
        });

        thread::sleep(std::time::Duration::from_millis(10));
        assert_that!(counter.load(Ordering::Relaxed), eq 0);
        drop(guard);
        thread::sleep(std::time::Duration::from_millis(10));
        assert_that!(counter.load(Ordering::Relaxed), eq 2);
    });
}

#[test]
fn file_lock_read_lock_blocks_write_locks() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);
    let counter = AtomicU64::new(0);
    thread::scope(|s| {
        let guard = test.sut.read_lock().expect("");

        s.spawn(|| {
            test.sut.read_lock().expect("");
            counter.fetch_add(1, Ordering::Relaxed);
        });

        s.spawn(|| {
            test.sut.write_lock().expect("");
            counter.fetch_add(2, Ordering::Relaxed);
        });

        thread::sleep(std::time::Duration::from_millis(10));
        assert_that!(counter.load(Ordering::Relaxed), eq 1);
        drop(guard);
        thread::sleep(std::time::Duration::from_millis(10));
        assert_that!(counter.load(Ordering::Relaxed), eq 3);
    });
}

#[test]
fn file_lock_read_timed_lock_waits_at_least_timeout() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);

    thread::scope(|s| {
        let _guard = test.sut.write_lock().expect("");
        const TIMEOUT: Duration = Duration::from_millis(10);

        s.spawn(|| {
            let start = Instant::now();
            test.sut.read_timed_lock(TIMEOUT).expect("");
            assert_that!(start.elapsed(), ge TIMEOUT);
        });

        thread::sleep(4 * TIMEOUT);
    });
}

#[test]
fn file_lock_write_timed_lock_waits_at_least_timeout() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);

    thread::scope(|s| {
        let _guard = test.sut.write_lock().expect("");
        const TIMEOUT: Duration = Duration::from_millis(10);

        s.spawn(|| {
            let start = Instant::now();
            test.sut.write_timed_lock(TIMEOUT).expect("");
            assert_that!(start.elapsed(), ge TIMEOUT);
        });

        thread::sleep(4 * TIMEOUT);
    });
}

#[test]
fn file_lock_read_try_lock_does_not_block() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);
    let counter = AtomicU64::new(0);

    thread::scope(|s| {
        let _guard = test.sut.write_lock().expect("");

        s.spawn(|| {
            test.sut.read_try_lock().expect("");
            counter.fetch_add(1, Ordering::Relaxed);
        });

        thread::sleep(std::time::Duration::from_millis(10));
        assert_that!(counter.load(Ordering::Relaxed), eq 1);
    });
}

#[test]
fn file_lock_write_try_lock_does_not_block() {
    test_requires!(POSIX_SUPPORT_FILE_LOCK);

    let handle = ReadWriteMutexHandle::new();
    let test = TestFixture::new(&handle);
    let counter = AtomicU64::new(0);

    thread::scope(|s| {
        let _guard = test.sut.write_lock().expect("");

        s.spawn(|| {
            test.sut.write_try_lock().expect("");
            counter.fetch_add(1, Ordering::Relaxed);
        });

        thread::sleep(std::time::Duration::from_millis(10));
        assert_that!(counter.load(Ordering::Relaxed), eq 1);
    });
}
