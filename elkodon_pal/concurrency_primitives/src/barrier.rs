use core::{
    hint::spin_loop,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::SPIN_REPETITIONS;

#[derive(Debug)]
pub struct Barrier {
    waiters: AtomicU32,
}

impl Barrier {
    pub fn new(number_of_waiters: u32) -> Self {
        Self {
            waiters: AtomicU32::new(number_of_waiters),
        }
    }

    pub fn wait<Wait: Fn(&AtomicU32, &u32), WakeAll: Fn(&AtomicU32)>(
        &self,
        wait: Wait,
        wake_all: WakeAll,
    ) {
        if self.waiters.fetch_sub(1, Ordering::AcqRel) == 1 {
            wake_all(&self.waiters);
            return;
        }

        let mut retry_counter = 0;
        while self.waiters.load(Ordering::Acquire) > 0 {
            spin_loop();
            retry_counter += 1;

            if SPIN_REPETITIONS == retry_counter {
                break;
            }
        }

        loop {
            let current_value = self.waiters.load(Ordering::Acquire);
            if current_value == 0 {
                return;
            }

            wait(&self.waiters, &current_value);
        }
    }
}
