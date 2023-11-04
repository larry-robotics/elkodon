#[macro_use]
pub mod assert;
pub mod watch_dog;

#[macro_export(local_inner_macros)]
macro_rules! test_requires {
    { $condition:expr } => {
        if !$condition { return; }
    }
}

pub const AT_LEAST_TIMING_VARIANCE: f32 = elkodon_pal_settings::settings::AT_LEAST_TIMING_VARIANCE;
