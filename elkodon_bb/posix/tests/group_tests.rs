use elkodon_bb_container::semantic_string::*;
use elkodon_bb_posix::group::*;
use elkodon_bb_system_types::group_name::GroupName;
use elkodon_bb_testing::{assert_that, test_requires};
use elkodon_pal_posix::posix::POSIX_SUPPORT_USERS_AND_GROUPS;

#[test]
fn group_works() {
    test_requires!(POSIX_SUPPORT_USERS_AND_GROUPS);

    let root = GroupName::new(b"root").unwrap();
    let wheel = GroupName::new(b"wheel").unwrap();

    let (group_from_name, group_name) = if let Ok(group) = Group::from_name(&root) {
        (group, root)
    } else if let Ok(group) = Group::from_name(&wheel) {
        (group, wheel)
    } else {
        unreachable!("Neither group 'root' not group 'wheel' is found!")
    };

    let group_from_gid = Group::from_gid(0).unwrap();

    assert_that!(group_from_name.gid(), eq group_from_gid.gid());
    assert_that!(group_from_name.gid(), eq 0);

    assert_that!(group_from_name.name(), eq group_from_gid.name());
    assert_that!(*group_from_name.name(), eq group_name);
}
