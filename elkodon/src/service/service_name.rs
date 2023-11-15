//! # Example
//!
//! ```
//! use elkodon::prelude::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let service_name = ServiceName::new(b"My/Funk/ServiceName")?;
//!
//! # Ok(())
//! # }
//! ```

use elkodon_bb_container::semantic_string;
use elkodon_bb_container::semantic_string::SemanticString;
use serde::{de::Visitor, Deserialize, Serialize};

const SERVICE_NAME_LENGTH: usize = 255;

semantic_string! {
  /// The unique name for a service.
  name: ServiceName,
  capacity: SERVICE_NAME_LENGTH,
  invalid_content: |value: &[u8]| {
                        matches!(value, b"")
                    },
  invalid_characters: |_: &[u8]| { false },
  comparision: |lhs: &[u8], rhs: &[u8]| {
      *lhs == *rhs
  }
}

struct ServiceNameVisitor;

impl<'de> Visitor<'de> for ServiceNameVisitor {
    type Value = ServiceName;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string containing the service name")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match ServiceName::new(v.as_bytes()) {
            Ok(v) => Ok(v),
            Err(v) => Err(E::custom(format!("invalid service name provided {:?}.", v))),
        }
    }
}

impl<'de> Deserialize<'de> for ServiceName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ServiceNameVisitor)
    }
}

impl Serialize for ServiceName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(std::str::from_utf8(self.as_bytes()).unwrap())
    }
}
