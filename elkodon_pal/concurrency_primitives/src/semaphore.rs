use core::{
    hint::spin_loop,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::SPIN_REPETITIONS;

pub struct Semaphore {
    value: AtomicU32,
}

impl Semaphore {
    pub fn new(initial_value: u32) -> Self {
        Self {
            value: AtomicU32::new(initial_value),
        }
    }

    pub fn value(&self) -> u32 {
        self.value.load(Ordering::Relaxed)
    }

    pub fn post<WakeUp: Fn(&AtomicU32)>(&self, wakeup: WakeUp) {
        self.value.fetch_add(1, Ordering::Acquire);
        wakeup(&self.value);
    }

    pub fn wait<Wait: Fn(&AtomicU32, &u32) -> bool>(&self, wait: Wait) -> bool {
        let mut retry_counter = 0;
        let mut current_value = self.value.load(Ordering::Relaxed);

        let mut keep_running = true;
        loop {
            loop {
                if current_value != 0 {
                    break;
                }

                if !keep_running {
                    return false;
                }

                if retry_counter < SPIN_REPETITIONS {
                    spin_loop();
                    retry_counter += 1;
                } else if !wait(&self.value, &current_value) {
                    keep_running = false;
                }
                current_value = self.value.load(Ordering::Relaxed);
            }

            match self.value.compare_exchange_weak(
                current_value,
                current_value - 1,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(v) => current_value = v,
            }
        }

        true
    }

    pub fn try_wait(&self) -> bool {
        let mut current_value = self.value.load(Ordering::Relaxed);

        loop {
            if current_value == 0 {
                return false;
            }

            match self.value.compare_exchange_weak(
                current_value,
                current_value - 1,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => return true,
                Err(v) => current_value = v,
            }
        }
    }
}
