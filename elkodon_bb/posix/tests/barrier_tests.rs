use elkodon_bb_posix::barrier::*;
use elkodon_bb_testing::assert_that;

use std::{sync::atomic::AtomicU64, sync::atomic::Ordering, thread};

#[test]
fn barrier_blocks() -> Result<(), BarrierCreationError> {
    let handle = BarrierHandle::new();
    let handle2 = BarrierHandle::new();
    let handle3 = BarrierHandle::new();
    let sut = BarrierBuilder::new(3).create(&handle)?;
    let sut2 = BarrierBuilder::new(3).create(&handle2)?;
    let sut3 = BarrierBuilder::new(3).create(&handle3)?;
    let counter = AtomicU64::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            sut.wait();
            sut2.wait();
            counter.fetch_add(10, Ordering::Relaxed);
            sut3.wait();
        });
        s.spawn(|| {
            sut.wait();
            sut2.wait();
            counter.fetch_add(10, Ordering::Relaxed);
            sut3.wait();
        });

        sut.wait();
        assert_that!(counter.load(Ordering::Relaxed), eq 0);
        sut2.wait();
        sut3.wait();
        assert_that!(counter.load(Ordering::Relaxed), eq 20);
    });

    Ok(())
}
