//! P2P network message types.
//!
//! Transport layer implementation lives in `apex-node`.
//! This module defines protocol-level message envelopes.

use crate::types::{BlockHeight, ChainId, Hash};
use serde::{Deserialize, Serialize};

/// Wire protocol version.
pub const PROTOCOL_VERSION: u32 = 1;

/// Messages exchanged between APEX nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Message {
    /// Peer handshake — establishes identity and protocol version.
    Handshake {
        peer_id: [u8; 32],
        protocol_version: u32,
        chain_id: ChainId,
        tip_height: BlockHeight,
        tip_hash: Hash,
    },
    /// Broadcast a pending transaction to the mempool.
    Transaction(crate::mempool::PendingTransaction),
    /// Propose a new block (validator only).
    BlockProposal(crate::state::Block),
    /// Request blocks in a given height range.
    BlockRequest { from: BlockHeight, to: BlockHeight },
}
