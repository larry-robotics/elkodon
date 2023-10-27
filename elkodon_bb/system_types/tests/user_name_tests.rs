use elkodon_bb_container::semantic_string::*;
use elkodon_bb_system_types::user_name::*;
use elkodon_bb_testing::assert_that;

#[test]
fn user_name_new_with_illegal_name_fails() {
    let sut = UserName::new(b"");
    assert_that!(sut, is_err);

    let sut = UserName::new(b"-asdf");
    assert_that!(sut, is_err);

    let sut = UserName::new(b"0asdf");
    assert_that!(sut, is_err);

    let sut = UserName::new(b"asd\0f");
    assert_that!(sut, is_err);
}

#[test]
fn user_name_new_with_legal_name_works() {
    let sut = UserName::new(b"abcdefghijklmnopqrstuvwxyz-0123");
    assert_that!(sut, is_ok);

    let sut = UserName::new(b"a456789-");
    assert_that!(sut, is_ok);
}
