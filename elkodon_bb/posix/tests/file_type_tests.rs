use elkodon_bb_posix::file_type::*;
use elkodon_bb_testing::assert_that;
use elkodon_pal_posix::*;

#[test]
fn file_type_mode_t_conversion_works() {
    assert_that!(FileType::File, eq FileType::from_mode_t(posix::S_IFREG));
    assert_that!(FileType::Character, eq FileType::from_mode_t(posix::S_IFCHR));
    assert_that!(FileType::Block, eq FileType::from_mode_t(posix::S_IFBLK));
    assert_that!(FileType::Directory, eq FileType::from_mode_t(posix::S_IFDIR));
    assert_that!(
        FileType::SymbolicLink, eq
        FileType::from_mode_t(posix::S_IFLNK)
    );
    assert_that!(FileType::Socket, eq FileType::from_mode_t(posix::S_IFSOCK));
    assert_that!(FileType::FiFo, eq FileType::from_mode_t(posix::S_IFIFO));
}
