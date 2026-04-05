//! Cryptographic interface module.
//!
//! Trait definitions and address derivation logic.
//! Concrete PQC implementations live in `apex-crypto`.

use serde::{Deserialize, Serialize};

/// Supported signature schemes, tagged for cryptographic agility.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SignatureScheme {
    /// CRYSTALS-Dilithium3 — NIST FIPS 204. Default scheme.
    Dilithium3 = 0x01,
    /// SPHINCS+-SHA2-256f — NIST FIPS 205. Hash-based fallback.
    SphincsPlus = 0x02,
    /// Ed25519 — classical; used only in hybrid transition mode.
    Ed25519 = 0x10,
}

/// A scheme-tagged digital signature.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signature {
    pub scheme: SignatureScheme,
    pub bytes: Vec<u8>,
}

/// Signing interface.
pub trait Signer {
    type Error: std::error::Error;
    fn sign(&self, message: &[u8]) -> Result<Signature, Self::Error>;
    fn public_key_bytes(&self) -> Vec<u8>;
}

/// Verification interface.
pub trait Verifier {
    type Error: std::error::Error;
    fn verify(&self, message: &[u8], sig: &Signature) -> Result<(), Self::Error>;
}

/// Derives a 20-byte APEX address from a Dilithium3 public key.
///
/// addr = SHA3-256(0x01 || pk_bytes)[0..20]
///
/// The 0x01 prefix is a domain separator that supports
/// future key-scheme versioning.
pub fn derive_address(pk: &[u8]) -> [u8; 20] {
    use sha3::{Digest, Sha3_256};
    let mut h = Sha3_256::new();
    h.update([0x01u8]);
    h.update(pk);
    let out = h.finalize();
    let mut addr = [0u8; 20];
    addr.copy_from_slice(&out[..20]);
    addr
}
