use elkodon_bb_container::semantic_string::*;
use elkodon_bb_posix::user::*;
use elkodon_bb_system_types::user_name::UserName;
use elkodon_bb_testing::{assert_that, test_requires};
use elkodon_pal_posix::posix::POSIX_SUPPORT_USERS_AND_GROUPS;

#[test]
fn user_works() {
    test_requires!(POSIX_SUPPORT_USERS_AND_GROUPS);

    let root = User::from_name(&UserName::new(b"root").unwrap()).unwrap();
    let root_from_uid = User::from_uid(0).unwrap();

    assert_that!(root.uid(), eq root_from_uid.uid());
    assert_that!(root.uid(), eq 0);

    assert_that!(root.gid(), eq root_from_uid.gid());
    assert_that!(root.gid(), eq 0);

    assert_that!(root.name(), eq root_from_uid.name());
    assert_that!(root.name().as_bytes(), eq b"root");

    assert_that!(root.info(), eq root_from_uid.info());

    assert_that!(root.home_dir(), eq root_from_uid.home_dir());
    assert_that!(root.home_dir(), eq "/root");

    assert_that!(root.shell(), eq root_from_uid.shell());
    assert_that!(root.password(), eq root_from_uid.password());
}
