//! Creates a Sha1 [`Hash`]. **Shall not be used for security critical use cases.**

use crate::hash::*;
use sha1_smol::Digest;

pub struct Sha1 {
    hash: Digest,
}

impl ToB64 for Sha1 {
    fn to_b64(&self) -> String {
        let mut adjusted_bytes = [0u8; 16];
        adjusted_bytes.copy_from_slice(&self.hash.bytes()[0..16]);
        adjusted_bytes.to_b64()
        //        self.hash.bytes().to_b64()
    }
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
}
