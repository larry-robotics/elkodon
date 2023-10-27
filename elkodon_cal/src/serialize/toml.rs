//! Implements [`Serialize`] for TOML files.

use elkodon_bb_log::fail;

use crate::serialize::Serialize;

use super::{DeserializeError, SerializeError};

/// toml [`Serialize`]
pub struct Toml {}

impl Serialize for Toml {
    fn serialize<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, SerializeError> {
        let msg = "Failed to serialize object";
        let mut buffer = String::new();
        let mut serializer = toml::ser::Serializer::new(&mut buffer);
        match value.serialize(&mut serializer) {
            Ok(()) => Ok(unsafe { buffer.as_mut_vec().clone() }),
            Err(toml::ser::Error::UnsupportedType) => {
                fail!(from "Toml::serialize",
                with SerializeError::UnsupportedType,
                    "{} since the type \"{}\" is not supported.", msg, std::any::type_name::<T>());
            }
            Err(e) => {
                fail!(from "Toml::serialize",
                with SerializeError::InternalError,
                    "{} since the error ({}) occurred.", msg, e);
            }
        }
    }

    fn deserialize<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, DeserializeError> {
        let value = String::from_utf8_lossy(bytes);
        let mut deserializer = toml::Deserializer::new(&value);
        match T::deserialize(&mut deserializer) {
            Ok(result) => Ok(result),
            Err(e) => {
                fail!(from "Toml::deserialize",
                with DeserializeError::InternalError, "Failed to deserialize object ({}).", e);
            }
        }
    }
}
