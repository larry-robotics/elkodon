//! Simplifies the kind of serialization which shall be used. The implementation has two methods
//!  * [`Serialize::serialize()`] - serialize a given object
//!  * [`Serialize::deserialize()`] - deserialize a given byte reference into the source object
//!
//! # Example
//!
//! ```
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
//! struct TestStruct {
//!     value: u64
//! };
//!
//! fn example<S: elkodon_cal::serialize::Serialize>() {
//!     let data_orig = TestStruct { value: 1234 };
//!
//!     let serialized = S::serialize::<TestStruct>(&data_orig)
//!                                 .expect("serialization failed.");
//!
//!     let data: TestStruct = S::deserialize(serialized.as_slice())
//!                           .expect("deserialization failed.");
//!
//!     assert_eq!(data, data_orig);
//! }
//! ```

pub mod cdr;
pub mod toml;

/// Failure emitted by [`Serialize::serialize()`]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SerializeError {
    UnsupportedType,
    InternalError,
}

/// Failure emitted by [`Serialize::deserialize()`]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeserializeError {
    InternalError,
}

/// Serialize and deserialize constructs which implement [`serde::Serialize`] and
/// [`serde::de::DeserializeOwned`]
pub trait Serialize {
    /// Serializes a value
    fn serialize<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, SerializeError>;

    /// Deserialize a value from a given byte slice
    fn deserialize<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, DeserializeError>;
}
