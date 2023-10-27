use elkodon_bb_posix::permission::*;
use elkodon_bb_testing::assert_that;

#[test]
pub fn permission_setting_and_reading_works() {
    let mut v1 = Permission::OWNER_READ
        | Permission::OTHERS_WRITE
        | Permission::GROUP_EXEC
        | Permission::GROUP_READ;
    v1 |= Permission::SET_GID;

    assert_that!(v1.has(Permission::OWNER_READ), eq true);
    assert_that!(v1.has(Permission::OWNER_WRITE), eq false);
    assert_that!(v1.has(Permission::OWNER_EXEC), eq false);
    assert_that!(v1.has(Permission::GROUP_READ), eq true);
    assert_that!(v1.has(Permission::GROUP_WRITE), eq false);
    assert_that!(v1.has(Permission::GROUP_EXEC), eq true);
    assert_that!(v1.has(Permission::OTHERS_READ), eq false);
    assert_that!(v1.has(Permission::OTHERS_WRITE), eq true);
    assert_that!(v1.has(Permission::OTHERS_EXEC), eq false);
    assert_that!(v1.has(Permission::SET_GID), eq true);
    assert_that!(v1.has(Permission::SET_UID), eq false);
    assert_that!(v1.has(Permission::STICKY_BIT), eq false);
}
