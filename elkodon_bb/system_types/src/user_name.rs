//! Relocatable (inter-process shared memory compatible) [`semantic_string::SemanticString`] implementations for
//! [`UserName`].
//!
//! # Example
//!
//! ```
//! use elkodon_bb_container::semantic_string::SemanticString;
//! use elkodon_bb_system_types::user_name::*;
//!
//! let user = UserName::new(b"some-user").expect("invalid user name");
//!
//! let invalid_user = UserName::new(b"some*!?user");
//! assert!(invalid_user.is_err());
//! ```
//!
use elkodon_bb_container::semantic_string;

const USER_NAME_LENGTH: usize = 31;
semantic_string! {
  /// Abstracts a user name. Ensures via construction & modification that the contents is always a
  /// valid user name.
  name: UserName,
  capacity: USER_NAME_LENGTH,
  invalid_content: |string: &[u8]| {
    if string.is_empty() {
        return true;
    }

    matches!(string[0], b'-' | b'0'..=b'9')
  },
  invalid_characters: |string: &[u8]| {
    for value in string {
        match value {
            b'a'..=b'z' | b'0'..=b'9' | b'-' => (),
            _ => return true,
        }
    }

    false
  }
}
