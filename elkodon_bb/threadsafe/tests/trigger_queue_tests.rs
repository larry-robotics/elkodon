use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

use elkodon_bb_posix::clock::{nanosleep, Time};
use elkodon_bb_posix::mutex::MutexHandle;
use elkodon_bb_testing::{assert_that, test_requires};
use elkodon_bb_threadsafe::trigger_queue::*;
use elkodon_pal_posix::posix::POSIX_SUPPORT_AT_LEAST_TIMEOUTS;

const TIMEOUT: Duration = Duration::from_millis(25);
const SUT_CAPACITY: usize = 128;
type Sut<'a> = TriggerQueue<'a, usize, SUT_CAPACITY>;

#[test]
fn trigger_queue_new_queue_is_empty() {
    let mtx_handle = MutexHandle::new();
    let free_handle = UnnamedSemaphoreHandle::new();
    let used_handle = UnnamedSemaphoreHandle::new();

    let sut = Sut::new(&mtx_handle, &free_handle, &used_handle);

    assert_that!(!sut.is_full(), eq true);
    assert_that!(sut, is_empty);
    assert_that!(sut.capacity(), eq SUT_CAPACITY);
    assert_that!(sut, len 0);
    assert_that!(sut.try_pop(), eq None);
}

#[test]
fn trigger_queue_try_push_pop_works() {
    let mtx_handle = MutexHandle::new();
    let free_handle = UnnamedSemaphoreHandle::new();
    let used_handle = UnnamedSemaphoreHandle::new();

    let sut = Sut::new(&mtx_handle, &free_handle, &used_handle);

    for i in 0..SUT_CAPACITY {
        assert_that!(sut.try_push(i), eq true);
        assert_that!(sut, is_not_empty);
        assert_that!(sut, len i + 1);
    }
    assert_that!(sut.is_full(), eq true);
    assert_that!(!sut.try_push(0), eq true);

    for i in 0..SUT_CAPACITY {
        let value = sut.try_pop();
        assert_that!(value, is_some);
        assert_that!(value.unwrap(), eq i);
        assert_that!(!sut.is_full(), eq true);
    }
    assert_that!(sut, is_empty);
    let value = sut.try_pop();
    assert_that!(value, is_none);
}

#[test]
fn trigger_queue_timed_push_pop_works() {
    let mtx_handle = MutexHandle::new();
    let free_handle = UnnamedSemaphoreHandle::new();
    let used_handle = UnnamedSemaphoreHandle::new();

    let sut = Sut::new(&mtx_handle, &free_handle, &used_handle);

    for i in 0..SUT_CAPACITY {
        assert_that!(sut.timed_push(i, TIMEOUT), eq true);
        assert_that!(sut, is_not_empty);
        assert_that!(sut, len i + 1);
    }
    assert_that!(sut.is_full(), eq true);
    assert_that!(sut.try_push(0), eq false);

    for i in 0..SUT_CAPACITY {
        let value = sut.timed_pop(TIMEOUT);
        assert_that!(value, is_some);
        assert_that!(value.unwrap(), eq i);
        assert_that!(!sut.is_full(), eq true);
    }
    assert_that!(sut, is_empty);
    let value = sut.try_pop();
    assert_that!(value, is_none);
}

#[test]
fn trigger_queue_blocking_push_pop_works() {
    let mtx_handle = MutexHandle::new();
    let free_handle = UnnamedSemaphoreHandle::new();
    let used_handle = UnnamedSemaphoreHandle::new();

    let sut = Sut::new(&mtx_handle, &free_handle, &used_handle);

    for i in 0..SUT_CAPACITY {
        sut.blocking_push(i);
        assert_that!(sut, is_not_empty);
        assert_that!(sut, len i + 1);
    }
    assert_that!(sut.is_full(), eq true);
    assert_that!(sut.try_push(0), eq false);

    for i in 0..SUT_CAPACITY {
        let value = sut.blocking_pop();
        assert_that!(value, eq i);
        assert_that!(sut.is_full(), eq false);
    }
    assert_that!(sut, is_empty);
    let value = sut.try_pop();
    assert_that!(value, is_none);
}

#[test]
fn trigger_queue_timed_push_blocks_at_least_until_timeout_has_passed() {
    test_requires!(POSIX_SUPPORT_AT_LEAST_TIMEOUTS);
    let mtx_handle = MutexHandle::new();
    let free_handle = UnnamedSemaphoreHandle::new();
    let used_handle = UnnamedSemaphoreHandle::new();

    let sut = Sut::new(&mtx_handle, &free_handle, &used_handle);

    for _ in 0..SUT_CAPACITY {
        sut.try_push(0);
    }

    let start = Time::now().unwrap();
    assert_that!(sut.timed_push(0, TIMEOUT), eq false);
    assert_that!(start.elapsed().unwrap(), ge TIMEOUT);
}

#[test]
fn trigger_queue_timed_pop_blocks_at_least_until_timeout_has_passed() {
    test_requires!(POSIX_SUPPORT_AT_LEAST_TIMEOUTS);
    let mtx_handle = MutexHandle::new();
    let free_handle = UnnamedSemaphoreHandle::new();
    let used_handle = UnnamedSemaphoreHandle::new();

    let sut = Sut::new(&mtx_handle, &free_handle, &used_handle);

    let start = Time::now().unwrap();
    assert_that!(sut.timed_pop(TIMEOUT), is_none);
    assert_that!(start.elapsed().unwrap(), ge TIMEOUT);
}

#[test]
fn trigger_queue_blocking_push_blocks_until_there_is_space_again() {
    let mtx_handle = MutexHandle::new();
    let free_handle = UnnamedSemaphoreHandle::new();
    let used_handle = UnnamedSemaphoreHandle::new();

    let sut = Sut::new(&mtx_handle, &free_handle, &used_handle);

    let counter = AtomicU64::new(0);
    for _ in 0..SUT_CAPACITY {
        sut.blocking_push(0);
    }

    thread::scope(|s| {
        s.spawn(|| {
            sut.blocking_push(0);
            counter.store(1, Ordering::Relaxed);
        });

        nanosleep(TIMEOUT).unwrap();
        let counter_old = counter.load(Ordering::Relaxed);
        sut.blocking_pop();
        nanosleep(TIMEOUT).unwrap();

        assert_that!(counter_old, eq 0);
        assert_that!(counter.load(Ordering::Relaxed), eq 1);
    });
}

#[test]
fn trigger_queue_blocking_pop_blocks_until_there_is_something_pushed() {
    let mtx_handle = MutexHandle::new();
    let free_handle = UnnamedSemaphoreHandle::new();
    let used_handle = UnnamedSemaphoreHandle::new();

    let sut = Sut::new(&mtx_handle, &free_handle, &used_handle);

    let counter = AtomicU64::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            sut.blocking_pop();
            counter.store(1, Ordering::Relaxed);
        });

        nanosleep(TIMEOUT).unwrap();
        let counter_old = counter.load(Ordering::Relaxed);
        sut.blocking_push(0);
        nanosleep(TIMEOUT).unwrap();

        assert_that!(counter_old, eq 0);
        assert_that!(counter.load(Ordering::Relaxed), eq 1);
    });
}

#[test]
fn trigger_queue_one_pop_notifies_exactly_one_blocking_push() {
    let mtx_handle = MutexHandle::new();
    let free_handle = UnnamedSemaphoreHandle::new();
    let used_handle = UnnamedSemaphoreHandle::new();

    let sut = Sut::new(&mtx_handle, &free_handle, &used_handle);

    let counter = AtomicU64::new(0);
    for _ in 0..SUT_CAPACITY {
        sut.blocking_push(0);
    }

    thread::scope(|s| {
        s.spawn(|| {
            sut.blocking_push(0);
            counter.fetch_add(1, Ordering::Relaxed);
        });

        s.spawn(|| {
            sut.blocking_push(0);
            counter.fetch_add(1, Ordering::Relaxed);
        });

        nanosleep(TIMEOUT).unwrap();
        let counter_old_1 = counter.load(Ordering::Relaxed);
        sut.blocking_pop();
        nanosleep(TIMEOUT).unwrap();
        let counter_old_2 = counter.load(Ordering::Relaxed);
        sut.blocking_pop();

        assert_that!(counter_old_1, eq 0);
        assert_that!(counter_old_2, eq 1);
    });
}

#[test]
fn trigger_queue_one_pop_notifies_exactly_one_timed_push() {
    let mtx_handle = MutexHandle::new();
    let free_handle = UnnamedSemaphoreHandle::new();
    let used_handle = UnnamedSemaphoreHandle::new();

    let sut = Sut::new(&mtx_handle, &free_handle, &used_handle);
    let counter = AtomicU64::new(0);
    for _ in 0..SUT_CAPACITY {
        sut.blocking_push(0);
    }

    thread::scope(|s| {
        s.spawn(|| {
            sut.timed_push(0, TIMEOUT * 10);
            counter.fetch_add(1, Ordering::Relaxed);
        });

        s.spawn(|| {
            sut.timed_push(0, TIMEOUT * 10);
            counter.fetch_add(1, Ordering::Relaxed);
        });

        nanosleep(TIMEOUT).unwrap();
        let counter_old_1 = counter.load(Ordering::Relaxed);
        sut.blocking_pop();
        nanosleep(TIMEOUT).unwrap();
        let counter_old_2 = counter.load(Ordering::Relaxed);
        sut.blocking_pop();

        assert_that!(counter_old_1, eq 0);
        assert_that!(counter_old_2, eq 1);
    });
}

#[test]
fn trigger_queue_one_push_notifies_exactly_one_blocking_pop() {
    let mtx_handle = MutexHandle::new();
    let free_handle = UnnamedSemaphoreHandle::new();
    let used_handle = UnnamedSemaphoreHandle::new();

    let sut = Sut::new(&mtx_handle, &free_handle, &used_handle);
    let counter = AtomicU64::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            sut.blocking_pop();
            counter.fetch_add(1, Ordering::Relaxed);
        });
        s.spawn(|| {
            sut.blocking_pop();
            counter.fetch_add(1, Ordering::Relaxed);
        });

        nanosleep(TIMEOUT).unwrap();
        let counter_old_1 = counter.load(Ordering::Relaxed);
        sut.blocking_push(0);
        nanosleep(TIMEOUT).unwrap();
        let counter_old_2 = counter.load(Ordering::Relaxed);
        sut.blocking_push(0);

        assert_that!(counter_old_1, eq 0);
        assert_that!(counter_old_2, eq 1);
    });
}

#[test]
fn trigger_queue_one_push_notifies_exactly_one_timed_pop() {
    let mtx_handle = MutexHandle::new();
    let free_handle = UnnamedSemaphoreHandle::new();
    let used_handle = UnnamedSemaphoreHandle::new();

    let sut = Sut::new(&mtx_handle, &free_handle, &used_handle);
    let counter = AtomicU64::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            sut.timed_pop(TIMEOUT * 10);
            counter.fetch_add(1, Ordering::Relaxed);
        });
        s.spawn(|| {
            sut.timed_pop(TIMEOUT * 10);
            counter.fetch_add(1, Ordering::Relaxed);
        });

        nanosleep(TIMEOUT).unwrap();
        let counter_old_1 = counter.load(Ordering::Relaxed);
        sut.blocking_push(0);
        nanosleep(TIMEOUT).unwrap();
        let counter_old_2 = counter.load(Ordering::Relaxed);
        sut.blocking_push(0);

        assert_that!(counter_old_1, eq 0);
        assert_that!(counter_old_2, eq 1);
    });
}
