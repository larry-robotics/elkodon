use elkodon_bb_posix::group::*;
use elkodon_bb_posix::ownership::*;
use elkodon_bb_posix::user::*;
use elkodon_bb_testing::assert_that;
use elkodon_bb_testing::test_requires;
use elkodon_pal_posix::posix::POSIX_SUPPORT_USERS_AND_GROUPS;

#[test]
fn ownership_builder_defaults_are_correct() {
    test_requires!(POSIX_SUPPORT_USERS_AND_GROUPS);

    let ownership = OwnershipBuilder::new().create();

    assert_that!(ownership.uid(), eq u32::MAX);
    assert_that!(ownership.gid(), eq u32::MAX);
}

#[test]
fn ownership_builder_works() {
    test_requires!(POSIX_SUPPORT_USERS_AND_GROUPS);

    let ownership = OwnershipBuilder::new()
        .uid("root".as_user().expect("no such user").uid())
        .gid("root".as_group().expect("no such group").gid())
        .create();

    assert_that!(ownership.uid(), eq 0);
    assert_that!(ownership.gid(), eq 0);
}
