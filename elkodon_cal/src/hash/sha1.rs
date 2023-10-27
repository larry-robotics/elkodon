//! Creates a Sha1 [`Hash`]. **Shall not be used for security critical use cases.**

use crate::hash::*;
use sha1_smol::Digest;

pub struct Sha1 {
    hash: Digest,
}

impl Hash for Sha1 {
    fn new(bytes: &[u8]) -> Self {
        Self {
            hash: {
                let mut hash = sha1_smol::Sha1::new();
                hash.update(bytes);
                hash.digest()
            },
        }
    }

    fn as_hex_string(&self) -> String {
        self.hash.to_string()
    }
}
