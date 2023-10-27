use elkodon_bb_container::semantic_string::*;
use elkodon_bb_system_types::file_name::FileName;
use elkodon_bb_system_types::path::*;
use elkodon_bb_testing::assert_that;

#[cfg(target_os = "windows")]
mod windows {
    use super::*;

    #[test]
    fn path_new_with_illegal_name_fails() {
        let sut = Path::new(b"\0a");
        assert_that!(sut, is_err);

        let sut = Path::new(b";?!@");
        assert_that!(sut, is_err);

        let sut = Path::new(b"\\weird\\&^relative!@#$%^&*()\\path\\..");
        assert_that!(sut, is_err);
    }

    #[test]
    fn path_new_with_legal_name_works() {
        let sut = Path::new(b"C:\\some\\file\\path");
        assert_that!(sut, is_ok);

        let sut = Path::new(b"C:\\some\\file\\p\\");
        assert_that!(sut, is_ok);

        let sut = Path::new(b"C:\\some\\file\\.p\\");
        assert_that!(sut, is_ok);

        let sut = Path::new(b"C:\\some\\file\\p\\.\\");
        assert_that!(sut, is_ok);

        let sut = Path::new(b"C:\\some\\file\\p\\..\\");
        assert_that!(sut, is_ok);
    }

    #[test]
    fn path_add_works() {
        let mut sut = Path::new(b"C:\\some").unwrap();
        sut.add_path_entry(&FileName::new(b"file").unwrap())
            .unwrap();
        sut.add_path_entry(&FileName::new(b"path").unwrap())
            .unwrap();
        assert_that!(sut, eq b"C:\\some\\file\\path");

        let mut sut = Path::new(b"").unwrap();
        sut.add_path_entry(&FileName::new(b"another").unwrap())
            .unwrap();
        sut.add_path_entry(&FileName::new(b"testy").unwrap())
            .unwrap();
        assert_that!(sut, eq b"another\\testy");

        let mut sut = Path::new(b"fuu\\").unwrap();
        sut.add_path_entry(&FileName::new(b"blaaaha").unwrap())
            .unwrap();
        sut.add_path_entry(&FileName::new(b"blub.ma").unwrap())
            .unwrap();
        assert_that!(sut, eq b"fuu\\blaaaha\\blub.ma");
    }
}

#[cfg(not(target_os = "windows"))]
mod unix {
    use super::*;

    #[test]
    fn path_new_with_illegal_name_fails() {
        let sut = Path::new(b"\0a");
        assert_that!(sut, is_err);

        let sut = Path::new(b";?!@");
        assert_that!(sut, is_err);

        let sut = Path::new(b"/weird/&^relative!@#$%^&*()/path/..");
        assert_that!(sut, is_err);
    }

    #[test]
    fn path_new_with_legal_name_works() {
        let sut = Path::new(b"/some/file/path");
        assert_that!(sut, is_ok);

        let sut = Path::new(b"/some/file/p/");
        assert_that!(sut, is_ok);

        let sut = Path::new(b"/some/file/.p/");
        assert_that!(sut, is_ok);

        let sut = Path::new(b"/some/file/p/./");
        assert_that!(sut, is_ok);

        let sut = Path::new(b"/some/file/p/../");
        assert_that!(sut, is_ok);
    }

    #[test]
    fn path_add_works() {
        let mut sut = Path::new(b"/some").unwrap();
        sut.add_path_entry(&FileName::new(b"file").unwrap())
            .unwrap();
        sut.add_path_entry(&FileName::new(b"path").unwrap())
            .unwrap();
        assert_that!(sut, eq b"/some/file/path");

        let mut sut = Path::new(b"").unwrap();
        sut.add_path_entry(&FileName::new(b"another").unwrap())
            .unwrap();
        sut.add_path_entry(&FileName::new(b"testy").unwrap())
            .unwrap();
        assert_that!(sut, eq b"another/testy");

        let mut sut = Path::new(b"fuu/").unwrap();
        sut.add_path_entry(&FileName::new(b"blaaaha").unwrap())
            .unwrap();
        sut.add_path_entry(&FileName::new(b"blub.ma").unwrap())
            .unwrap();
        assert_that!(sut, eq b"fuu/blaaaha/blub.ma");
    }
}
