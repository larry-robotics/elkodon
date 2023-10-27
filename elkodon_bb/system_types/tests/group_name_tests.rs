use elkodon_bb_container::semantic_string::*;
use elkodon_bb_system_types::group_name::*;
use elkodon_bb_testing::assert_that;

#[test]
fn group_name_new_with_illegal_name_fails() {
    let sut = GroupName::new(b"");
    assert_that!(sut, is_err);

    let sut = GroupName::new(b"-asdf");
    assert_that!(sut, is_err);

    let sut = GroupName::new(b"0asdf");
    assert_that!(sut, is_err);

    let sut = GroupName::new(b"as\0df");
    assert_that!(sut, is_err);
}

#[test]
fn group_name_new_with_legal_name_works() {
    let sut = GroupName::new(b"abcdefghijklmnopqrstuvwxyz-0123");
    assert_that!(sut, is_ok);

    let sut = GroupName::new(b"a456789-");
    assert_that!(sut, is_ok);
}
