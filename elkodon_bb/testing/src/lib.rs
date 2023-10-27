#![no_std]

#[macro_use]
pub mod assert;

#[macro_export(local_inner_macros)]
macro_rules! test_requires {
    { $condition:expr } => {
        if !$condition { return; }
    }
}
