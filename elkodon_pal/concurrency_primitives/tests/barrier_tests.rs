use std::sync::atomic::{AtomicI32, Ordering};

use elkodon_bb_testing::assert_that;
use elkodon_pal_concurrency_primitives::barrier::*;

#[test]
fn barrier_with_multiple_waiter_works() {
    let counter = AtomicI32::new(0);
    let sut = Barrier::new(4);
    let sut2 = Barrier::new(4);
    let sut3 = Barrier::new(4);

    std::thread::scope(|s| {
        s.spawn(|| {
            sut.wait(|_, _| {}, |_| {});
            sut2.wait(|_, _| {}, |_| {});
            counter.fetch_add(1, Ordering::Relaxed);
            sut3.wait(|_, _| {}, |_| {});
        });

        s.spawn(|| {
            sut.wait(|_, _| {}, |_| {});
            sut2.wait(|_, _| {}, |_| {});
            counter.fetch_add(1, Ordering::Relaxed);
            sut3.wait(|_, _| {}, |_| {});
        });

        s.spawn(|| {
            sut.wait(|_, _| {}, |_| {});
            sut2.wait(|_, _| {}, |_| {});
            counter.fetch_add(1, Ordering::Relaxed);
            sut3.wait(|_, _| {}, |_| {});
        });

        sut.wait(|_, _| {}, |_| {});
        let counter_old = counter.load(Ordering::Relaxed);
        sut2.wait(|_, _| {}, |_| {});

        sut3.wait(|_, _| {}, |_| {});

        assert_that!(counter_old, eq 0);
        assert_that!(counter.load(Ordering::Relaxed), eq 3);
    });
}
