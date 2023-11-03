use elkodon_bb_posix::clock::*;
use elkodon_bb_posix::process::*;
use elkodon_bb_posix::signal::*;
use elkodon_bb_testing::assert_that;
use elkodon_bb_testing::test_requires;
use elkodon_pal_posix::posix::POSIX_SUPPORT_ADVANCED_SIGNAL_HANDLING;
use elkodon_pal_posix::*;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread;
use std::time::Duration;

static mut COUNTER: usize = 0;
static mut SIGNAL: usize = posix::MAX_SIGNAL_VALUE;
static LOCK: Mutex<i32> = Mutex::new(0);

struct TestFixture {
    _guard: MutexGuard<'static, i32>,
}

impl TestFixture {
    fn new() -> Self {
        let new_self = Self {
            _guard: LOCK.lock().unwrap(),
        };

        unsafe {
            COUNTER = 0;
            SIGNAL = posix::MAX_SIGNAL_VALUE;
        }

        new_self
    }

    pub fn signal_callback(signal: FetchableSignal) {
        unsafe { COUNTER += 1 };
        unsafe { SIGNAL = signal as usize };
    }

    pub fn verify(&self, signal: FetchableSignal, counter: usize) {
        assert_that!(SignalHandler::last_signal(), eq Some(signal));
        assert_that!(unsafe { COUNTER }, eq counter);
        assert_that!(unsafe { SIGNAL }, eq signal as usize);
    }
}

#[test]
fn signal_register_single_handler_works() {
    test_requires!(POSIX_SUPPORT_ADVANCED_SIGNAL_HANDLING);

    let test = TestFixture::new();
    let _guard =
        SignalHandler::register(FetchableSignal::UserDefined1, &TestFixture::signal_callback);

    Process::from_self().send_signal(Signal::UserDefined1).ok();
    nanosleep(Duration::from_millis(1)).ok();
    test.verify(FetchableSignal::UserDefined1, 1)
}

#[test]
fn signal_register_multiple_handler_works() {
    test_requires!(POSIX_SUPPORT_ADVANCED_SIGNAL_HANDLING);

    let test = TestFixture::new();
    let _guard1 =
        SignalHandler::register(FetchableSignal::UserDefined1, &TestFixture::signal_callback);

    let _guard2 =
        SignalHandler::register(FetchableSignal::UserDefined2, &TestFixture::signal_callback);

    Process::from_self().send_signal(Signal::UserDefined1).ok();
    nanosleep(Duration::from_millis(1)).ok();
    test.verify(FetchableSignal::UserDefined1, 1);

    Process::from_self().send_signal(Signal::UserDefined2).ok();
    nanosleep(Duration::from_millis(1)).ok();
    test.verify(FetchableSignal::UserDefined2, 2);
}

#[test]
fn signal_register_handler_with_multiple_signals_works() {
    test_requires!(POSIX_SUPPORT_ADVANCED_SIGNAL_HANDLING);

    let test = TestFixture::new();
    let s = vec![FetchableSignal::UserDefined1, FetchableSignal::UserDefined2];
    let _guard1 = SignalHandler::register_multiple_signals(&s, &TestFixture::signal_callback);

    Process::from_self().send_signal(Signal::UserDefined1).ok();
    nanosleep(Duration::from_millis(1)).ok();
    test.verify(FetchableSignal::UserDefined1, 1);

    Process::from_self().send_signal(Signal::UserDefined2).ok();
    nanosleep(Duration::from_millis(1)).ok();
    test.verify(FetchableSignal::UserDefined2, 2);
}

#[test]
fn signal_guard_unregisters_on_drop() {
    test_requires!(POSIX_SUPPORT_ADVANCED_SIGNAL_HANDLING);

    let test = TestFixture::new();
    let guard1 =
        SignalHandler::register(FetchableSignal::UserDefined1, &TestFixture::signal_callback);

    drop(guard1);

    let _guard1 = SignalHandler::register(FetchableSignal::UserDefined1, &|signal| unsafe {
        COUNTER += 10;
        SIGNAL = signal as usize;
    });

    Process::from_self().send_signal(Signal::UserDefined1).ok();
    nanosleep(Duration::from_millis(1)).ok();
    test.verify(FetchableSignal::UserDefined1, 10);
}

#[test]
fn signal_register_signal_twice_fails() {
    test_requires!(POSIX_SUPPORT_ADVANCED_SIGNAL_HANDLING);

    let _test = TestFixture::new();
    let s = vec![FetchableSignal::UserDefined1, FetchableSignal::UserDefined2];
    let _guard1 = SignalHandler::register_multiple_signals(&s, &TestFixture::signal_callback);

    assert_that!(
        SignalHandler::register(FetchableSignal::UserDefined2, &TestFixture::signal_callback),
        is_err
    );
}

#[test]
fn signal_call_and_fetch_works() {
    test_requires!(POSIX_SUPPORT_ADVANCED_SIGNAL_HANDLING);

    let _test = TestFixture::new();

    let result = SignalHandler::call_and_fetch(|| {
        Process::from_self().send_signal(Signal::Interrupt).ok();
        nanosleep(Duration::from_millis(1)).ok();
    });

    assert_that!(result, eq Some(FetchableSignal::Interrupt));
}

#[test]
fn signal_call_and_fetch_with_registered_handler_works() {
    test_requires!(POSIX_SUPPORT_ADVANCED_SIGNAL_HANDLING);

    let test = TestFixture::new();

    let _guard =
        SignalHandler::register(FetchableSignal::UserDefined1, &TestFixture::signal_callback);

    let result = SignalHandler::call_and_fetch(|| {
        Process::from_self().send_signal(Signal::UserDefined1).ok();
        nanosleep(Duration::from_millis(1)).ok();
    });
    nanosleep(Duration::from_millis(1)).ok();

    assert_that!(result, eq Some(FetchableSignal::UserDefined1));
    test.verify(FetchableSignal::UserDefined1, 1);
}

#[test]
fn signal_wait_for_signal_blocks() {
    test_requires!(POSIX_SUPPORT_ADVANCED_SIGNAL_HANDLING);

    let _test = TestFixture::new();

    let signals = vec![FetchableSignal::UserDefined2, FetchableSignal::UserDefined1];
    let counter = AtomicI32::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            SignalHandler::wait_for_multiple_signals(&signals).unwrap();
            counter.store(1, Ordering::Relaxed);
        });

        nanosleep(Duration::from_millis(10)).ok();
        let counter_old = counter.load(Ordering::Relaxed);
        Process::from_self().send_signal(Signal::UserDefined2).ok();
        nanosleep(Duration::from_millis(10)).ok();

        assert_that!(counter_old, eq 0);
        assert_that!(counter.load(Ordering::Relaxed), eq 1);
    });
}

#[test]
fn signal_wait_twice_for_same_signal_blocks() {
    test_requires!(POSIX_SUPPORT_ADVANCED_SIGNAL_HANDLING);

    let _test = TestFixture::new();

    let counter = AtomicI32::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            SignalHandler::wait_for_signal(FetchableSignal::UserDefined2).unwrap();
        });

        nanosleep(Duration::from_millis(10)).ok();
        Process::from_self().send_signal(Signal::UserDefined2).ok();

        s.spawn(|| {
            SignalHandler::wait_for_signal(FetchableSignal::UserDefined2).unwrap();
            counter.store(1, Ordering::Relaxed);
        });

        nanosleep(Duration::from_millis(10)).ok();
        let counter_old = counter.load(Ordering::Relaxed);
        Process::from_self().send_signal(Signal::UserDefined2).ok();
        nanosleep(Duration::from_millis(10)).ok();

        assert_that!(counter_old, eq 0);
        assert_that!(counter.load(Ordering::Relaxed), eq 1);
    });
}

#[test]
fn signal_timed_wait_blocks_at_least_for_timeout() {
    test_requires!(POSIX_SUPPORT_ADVANCED_SIGNAL_HANDLING);

    let _test = TestFixture::new();
    const TIMEOUT: Duration = Duration::from_millis(100);

    let start = Time::now_with_clock(ClockType::Monotonic).unwrap();
    SignalHandler::timed_wait_for_signal(FetchableSignal::UserDefined2, TIMEOUT).unwrap();
    assert_that!(start.elapsed().unwrap(), ge TIMEOUT);
}

#[test]
fn signal_timed_wait_blocks_until_signal() {
    test_requires!(POSIX_SUPPORT_ADVANCED_SIGNAL_HANDLING);

    let _test = TestFixture::new();
    const TIMEOUT: Duration = Duration::from_millis(100);

    let signals = vec![FetchableSignal::UserDefined2, FetchableSignal::UserDefined1];
    let counter = AtomicI32::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            SignalHandler::timed_wait_for_multiple_signals(&signals, TIMEOUT).unwrap();
            counter.store(1, Ordering::Relaxed);
        });

        nanosleep(Duration::from_millis(10)).ok();
        let counter_old = counter.load(Ordering::Relaxed);
        Process::from_self().send_signal(Signal::UserDefined2).ok();
        nanosleep(Duration::from_millis(10)).ok();

        assert_that!(counter_old, eq 0);
        assert_that!(counter.load(Ordering::Relaxed), eq 1);
    });
}

#[test]
fn signal_was_ctrl_c_pressed_with_terminate_works() {
    test_requires!(POSIX_SUPPORT_ADVANCED_SIGNAL_HANDLING);

    let _test = TestFixture::new();

    assert_that!(!SignalHandler::was_ctrl_c_pressed(), eq true);
    assert_that!(Process::from_self().send_signal(Signal::Terminate), is_ok);
    std::thread::sleep(std::time::Duration::from_millis(10));

    assert_that!(SignalHandler::was_ctrl_c_pressed(), eq true);
    assert_that!(SignalHandler::was_ctrl_c_pressed(), eq false);
}

#[test]
fn signal_was_ctrl_c_pressed_with_interrupt_works() {
    test_requires!(POSIX_SUPPORT_ADVANCED_SIGNAL_HANDLING);

    let _test = TestFixture::new();

    assert_that!(SignalHandler::was_ctrl_c_pressed(), eq false);
    assert_that!(Process::from_self().send_signal(Signal::Interrupt), is_ok);
    std::thread::sleep(std::time::Duration::from_millis(10));

    assert_that!(SignalHandler::was_ctrl_c_pressed(), eq true);
    assert_that!(SignalHandler::was_ctrl_c_pressed(), eq false);
}
