//! Relocatable (inter-process shared memory compatible) [`SemanticString`] implementation for
//! [`Path`]. All modification operations ensure that never an
//! invalid file or path name can be generated. All strings have a fixed size so that the maximum
//! path or file name length the system supports can be stored.
//!
//! # Example
//!
//! ```
//! use elkodon_bb_container::semantic_string::SemanticString;
//! use elkodon_bb_system_types::path::*;
//!
//! let name = Path::new(b"some/path/../bla/some_file.txt");
//!
//! let invalid_name = Path::new(b"/contains/illegal/\0/zero");
//! assert!(invalid_name.is_err());
//! ```

use elkodon_bb_container::semantic_string;

use elkodon_bb_log::fail;
use elkodon_pal_settings::PATH_SEPARATOR;

use crate::{file_name::FileName, file_path::FilePath};
use elkodon_bb_container::semantic_string::*;

const PATH_LENGTH: usize = elkodon_pal_settings::PATH_LENGTH;

semantic_string! {
  name: Path,
  capacity: PATH_LENGTH,
  invalid_content: |_: &[u8]| {
    false
  },
  invalid_characters: |string: &[u8]| {
    for value in string {
        match value {
            // linux & windows
            0 => return true,
            // windows only
            1..=31 => return true,
            b'<' => return true,
            b'>' => return true,
            b'"' => return true,
            b'|' => return true,
            b'?' => return true,
            b'*' => return true,
            _ => (),
        }
    }

    false
  },
  comparision: |lhs: &[u8], rhs: &[u8]| {
      let lhs_normalized = normalize(lhs);
      if lhs_normalized.is_err() {
          return false;
      }

      let rhs_normalized = normalize(rhs);
      if rhs_normalized.is_err() {
          return false;
      }

      *lhs_normalized.unwrap() == *rhs_normalized.unwrap()
  }
}

pub fn normalize(value: &[u8]) -> Result<Path, SemanticStringError> {
    let mut raw_path = [0u8; PATH_LENGTH];

    let mut previous_char_is_path_separator = false;
    let mut n = 0;
    for i in 0..value.len() {
        if i + 1 == value.len() && value[i] == PATH_SEPARATOR {
            break;
        }

        if !(previous_char_is_path_separator && value[i] == PATH_SEPARATOR) {
            raw_path[n] = value[i];
            n += 1;
        }

        previous_char_is_path_separator = value[i] == PATH_SEPARATOR
    }

    Path::new(&raw_path[0..n])
}

impl Path {
    /// Adds a new file or directory entry to the path. It adds it in a fashion that a slash is
    /// added when the path does not end with a slash - except when it is empty.
    pub fn add_path_entry(&mut self, entry: &FileName) -> Result<(), SemanticStringError> {
        let msg = format!("Unable to add entry \"{}\" to path since it would exceed the maximum supported path length of {}.",
            entry, PATH_LENGTH);
        if !self.is_empty()
            && self.as_bytes()[self.len() - 1] != elkodon_pal_settings::PATH_SEPARATOR
        {
            fail!(from self, when self.push(elkodon_pal_settings::PATH_SEPARATOR),
                with SemanticStringError::ExceedsMaximumLength,
                "{}", msg);
        }

        fail!(from self, when self.push_bytes(entry.as_bytes()),
            with SemanticStringError::ExceedsMaximumLength,
            "{}", msg);

        Ok(())
    }
}

impl From<FilePath> for Path {
    fn from(value: FilePath) -> Self {
        unsafe { Path::new_unchecked(value.as_bytes()) }
    }
}
