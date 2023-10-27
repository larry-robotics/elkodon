use elkodon_bb_container::semantic_string::SemanticString;
use elkodon_bb_posix::config::*;
use elkodon_bb_posix::creation_mode::*;
use elkodon_bb_posix::file::*;
use elkodon_bb_posix::file_descriptor::*;
use elkodon_bb_posix::file_type::*;
use elkodon_bb_posix::group::*;
use elkodon_bb_posix::user::*;
use elkodon_bb_system_types::file_name::FileName;
use elkodon_bb_system_types::file_path::FilePath;
use elkodon_bb_testing::assert_that;
use elkodon_bb_testing::test_requires;
use elkodon_pal_posix::posix::POSIX_SUPPORT_PERMISSIONS;
use elkodon_pal_posix::posix::POSIX_SUPPORT_USERS_AND_GROUPS;

#[test]
fn metadata_reads_basic_stats_correctly() {
    let file_name =
        FilePath::from_path_and_file(&TEMP_DIRECTORY, &FileName::new(b"meta_test").unwrap())
            .unwrap();

    let mut file = FileBuilder::new(&file_name)
        .creation_mode(CreationMode::PurgeAndCreate)
        .create()
        .unwrap();
    file.write(b"hello world").unwrap();

    let sut = file.metadata().unwrap();

    assert_that!(sut.file_type(), eq FileType::File);
    assert_that!(sut.size(), eq 11);

    File::remove_self(file).unwrap();
}

#[test]
fn metadata_reads_owner_and_permission_stats_correctly() {
    test_requires!(POSIX_SUPPORT_USERS_AND_GROUPS && POSIX_SUPPORT_PERMISSIONS);

    let file_name =
        FilePath::from_path_and_file(&TEMP_DIRECTORY, &FileName::new(b"meta_test_123").unwrap())
            .unwrap();

    let mut file = FileBuilder::new(&file_name)
        .creation_mode(CreationMode::PurgeAndCreate)
        .permission(Permission::ALL)
        .owner(User::from_self().unwrap().uid())
        .group(Group::from_self().unwrap().gid())
        .create()
        .unwrap();
    file.write(b"hello world!!").unwrap();

    let sut = file.metadata().unwrap();

    assert_that!(sut.file_type(), eq FileType::File);
    assert_that!(sut.size(), eq 13);
    assert_that!(sut.permission(), eq Permission::ALL);
    assert_that!(sut.uid(), eq User::from_self().expect("").uid());
    assert_that!(sut.gid(), eq Group::from_self().expect("").gid());

    File::remove_self(file).unwrap();
}
