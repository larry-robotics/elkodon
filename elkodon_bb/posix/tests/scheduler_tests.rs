use elkodon_bb_posix::config::DEFAULT_SCHEDULER;
use elkodon_bb_posix::scheduler::*;
use elkodon_bb_testing::assert_that;

#[test]
fn scheduler_default_scheduler_set_correctly() {
    assert_that!(Scheduler::default(), eq DEFAULT_SCHEDULER)
}
