use elkodon_bb_posix::creation_mode::*;
use elkodon_bb_testing::assert_that;
use elkodon_pal_posix::*;

#[test]
fn creation_mode_o_flag_conversion_works() {
    assert_that!(
        CreationMode::CreateExclusive.as_oflag(), eq
        posix::O_CREAT | posix::O_EXCL
    );
    assert_that!(
        CreationMode::PurgeAndCreate.as_oflag(), eq
        posix::O_CREAT | posix::O_EXCL
    );
    assert_that!(CreationMode::OpenOrCreate.as_oflag(), eq posix::O_CREAT);
}
