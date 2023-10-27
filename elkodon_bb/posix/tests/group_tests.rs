use elkodon_bb_container::semantic_string::*;
use elkodon_bb_posix::group::*;
use elkodon_bb_system_types::group_name::GroupName;
use elkodon_bb_testing::{assert_that, test_requires};
use elkodon_pal_posix::posix::POSIX_SUPPORT_USERS_AND_GROUPS;

#[test]
fn group_works() {
    test_requires!(POSIX_SUPPORT_USERS_AND_GROUPS);

    let root = Group::from_name(&GroupName::new(b"root").unwrap()).unwrap();
    let root_from_gid = Group::from_gid(0).unwrap();

    assert_that!(root.gid(), eq root_from_gid.gid());
    assert_that!(root.gid(), eq 0);

    assert_that!(root.name(), eq root_from_gid.name());
    assert_that!(root.name().as_bytes(), eq b"root");
}
