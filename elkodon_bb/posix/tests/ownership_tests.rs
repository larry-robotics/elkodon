use elkodon_bb_container::semantic_string::*;
use elkodon_bb_posix::group::*;
use elkodon_bb_posix::ownership::*;
use elkodon_bb_posix::user::*;
use elkodon_bb_system_types::group_name::GroupName;
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

    let root = GroupName::new(b"root").unwrap();
    let wheel = GroupName::new(b"wheel").unwrap();

    let group = if let Ok(group) = Group::from_name(&root) {
        group
    } else if let Ok(group) = Group::from_name(&wheel) {
        group
    } else {
        unreachable!("Neither group 'root' not group 'wheel' is found!")
    };

    let ownership = OwnershipBuilder::new()
        .uid("root".as_user().expect("no such user").uid())
        .gid(group.gid())
        .create();

    assert_that!(ownership.uid(), eq 0);
    assert_that!(ownership.gid(), eq 0);
}
