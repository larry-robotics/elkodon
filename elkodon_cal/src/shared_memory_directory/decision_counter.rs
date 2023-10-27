use std::sync::atomic::{AtomicU64, Ordering};

const UNDECIDED: u64 = u64::MAX;
const LOST: u64 = u64::MAX;

#[derive(Debug)]
pub(crate) struct DecisionCounter(AtomicU64);

impl DecisionCounter {
    pub(crate) const fn new() -> Self {
        DecisionCounter(AtomicU64::new(UNDECIDED))
    }

    pub(crate) fn set_to_undecided(&self) {
        self.0.store(UNDECIDED, Ordering::Relaxed);
    }

    pub(crate) fn set(&self, value: u64) -> bool {
        self.0
            .compare_exchange(UNDECIDED, value, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
    }

    pub(crate) fn does_value_win(&self, competing_value: u64) -> bool {
        let my_value = self.0.load(Ordering::Relaxed);

        if my_value == UNDECIDED {
            match self
                .0
                .compare_exchange(UNDECIDED, LOST, Ordering::Relaxed, Ordering::Relaxed)
            {
                Err(v) => competing_value < v,
                Ok(_) => true,
            }
        } else {
            competing_value < my_value
        }
    }
}
