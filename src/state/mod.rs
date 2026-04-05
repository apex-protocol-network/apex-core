//! State module — account model, transactions, and block structure.

use crate::crypto::Signature;
use crate::types::{Amount, BlockHeight, ChainId, Hash, Nonce};
use serde::{Deserialize, Serialize};

/// An APEX account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Native token balance in atomic units.
    pub balance: Amount,
    /// Transaction nonce — incremented on each valid transaction.
    pub nonce: Nonce,
    /// SHA3-256 commitment to the account's Dilithium3 public key.
    pub pubkey_hash: Hash,
    /// Hash of deployed contract bytecode. None for EOAs.
    pub code_hash: Option<Hash>,
}

impl Account {
    pub fn new(pubkey_hash: Hash) -> Self {
        Self { balance: 0, nonce: 0, pubkey_hash, code_hash: None }
    }
}

/// A signed APEX transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub version: u8,
    pub chain_id: ChainId,
    pub sender: [u8; 20],
    pub recipient: [u8; 20],
    pub value: Amount,
    pub nonce: Nonce,
    pub gas_limit: u64,
    pub gas_price: Amount,
    /// Contract call data, or empty for native token transfers.
    pub data: Vec<u8>,
    /// Full Dilithium3 public key — revealed at first spend.
    pub public_key: Vec<u8>,
    pub signature: Signature,
}

/// A finalized block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

/// Block header — chain linkage and consensus metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u8,
    pub height: BlockHeight,
    pub parent_hash: Hash,
    /// Merkle root of all transactions in this block.
    pub tx_root: Hash,
    /// Root of the post-execution state trie.
    pub state_root: Hash,
    /// Unix timestamp in milliseconds.
    pub timestamp_ms: u64,
    /// Address of the block proposer.
    pub proposer: [u8; 20],
    /// Aggregated Dilithium3 validator signatures.
    pub commit_sigs: Vec<Signature>,
}

impl Block {
    /// Returns the SHA3-256 hash of this block's header.
    pub fn hash(&self) -> Hash {
        use sha3::{Digest, Sha3_256};
        let encoded = serde_json::to_vec(&self.header)
            .expect("block header serialization must not fail");
        let mut h = Sha3_256::new();
        h.update(&encoded);
        let result = h.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&result);
        Hash::from_bytes(bytes)
    }
}
