//! Creates hashes from arbitrary byte slices.
//!
//! # Example
//!
//! ```
//! use elkodon_cal::hash::*;
//!
//! fn create_hash<H: Hash>() {
//!     let some_text = "Hello World".to_string();
//!     let hash = H::new(some_text.as_bytes());
//!
//!     println!("Hash as hex: {}", hash.as_hex_string());
//! }
//! ```

pub mod sha1;

/// Interface to generate hashes.
pub trait Hash {
    /// Creates a new hash from `bytes`.
    fn new(bytes: &[u8]) -> Self;

    /// Converts the hash into as string of hex-characters
    fn as_hex_string(&self) -> String;
}
