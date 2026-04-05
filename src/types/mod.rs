//! Core protocol primitive types.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A 32-byte cryptographic hash.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn from_bytes(b: [u8; 32]) -> Self { Self(b) }
    pub fn as_bytes(&self) -> &[u8; 32] { &self.0 }
    pub fn zero() -> Self { Self([0u8; 32]) }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash({}...)", hex::encode(&self.0[..6]))
    }
}

/// Block height — canonical position in the chain.
pub type BlockHeight = u64;

/// Per-account transaction counter preventing replay.
pub type Nonce = u64;

/// Token amount in atomic units (1 APEX = 10^18 atomic units).
pub type Amount = u128;

/// Chain identifier — prevents cross-chain transaction replay.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ChainId(pub u32);

impl ChainId {
    pub const MAINNET: Self = Self(1);
    pub const TESTNET: Self = Self(100);
    pub const DEVNET:  Self = Self(999);
}
