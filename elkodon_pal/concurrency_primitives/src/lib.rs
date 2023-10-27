#![no_std]

const SPIN_REPETITIONS: u64 = 10000;

pub mod barrier;
pub mod condition_variable;
pub mod mutex;
pub mod rwlock;
pub mod semaphore;
