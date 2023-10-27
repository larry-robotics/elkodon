//! Implements [`Serialize`] for the Common Data Representation (cdr),
//! see: <https://en.wikipedia.org/wiki/Common_Data_Representation>.

use crate::serialize::Serialize;
use cdr::{CdrBe, Infinite};
use elkodon_bb_log::fail;

use super::{DeserializeError, SerializeError};

/// cdr [`Serialize`]
pub struct Cdr {}

impl Serialize for Cdr {
    fn serialize<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, SerializeError> {
        Ok(
            fail!(from "Cdr::serialize", when cdr::serialize::<_, _, CdrBe>(&value, Infinite),
                with SerializeError::InternalError, "Failed to serialize object" ),
        )
    }

    fn deserialize<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, DeserializeError> {
        Ok(
            fail!(from "Cdr::deserialize", when cdr::deserialize::<T>(bytes),
                    with DeserializeError::InternalError, "Failed to deserialize object."),
        )
    }
}
