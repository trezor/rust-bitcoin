// SPDX-License-Identifier: CC0-1.0

//! Cryptography
//!
//! Cryptography related functionality: keys and signatures.
//!

use hashes::hash160;

use crate::internal_macros::impl_asref_push_bytes;

#[cfg(feature = "secp256k1")]
pub mod ecdsa;
#[cfg(feature = "secp256k1")]
pub mod key;
pub mod sighash;
// Contents re-exported in `bitcoin::taproot`.
#[cfg(feature = "secp256k1")]
pub(crate) mod taproot;

/// Bitcoin keys.
///
/// Only key hashes are available without the `secp256k1` feature.
#[cfg(not(feature = "secp256k1"))]
pub mod key {
    #[rustfmt::skip]                // Keep public re-exports separate.
    pub use crate::crypto::{PubkeyHash, WPubkeyHash};
}

hashes::hash_newtype! {
    /// A hash of a public key.
    pub struct PubkeyHash(hash160::Hash);
    /// SegWit version of a public key hash.
    pub struct WPubkeyHash(hash160::Hash);
}
impl_asref_push_bytes!(PubkeyHash, WPubkeyHash);
