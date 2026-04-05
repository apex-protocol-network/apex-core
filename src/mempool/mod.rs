//! Transaction mempool — priority-ordered pending transaction pool.

use crate::types::{Amount, Nonce};
use serde::{Deserialize, Serialize};

/// A transaction awaiting block inclusion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTransaction {
    pub data: Vec<u8>,
    pub gas_price: Amount,
    pub nonce: Nonce,
}

/// Priority-ordered in-memory transaction pool.
pub struct Mempool {
    pending: Vec<PendingTransaction>,
    capacity: usize,
}

impl Mempool {
    pub fn new(capacity: usize) -> Self {
        Self { pending: Vec::with_capacity(capacity), capacity }
    }

    /// Inserts a transaction, maintaining gas-price ordering.
    pub fn insert(&mut self, tx: PendingTransaction) -> Result<(), &'static str> {
        if self.pending.len() >= self.capacity {
            return Err("mempool at capacity");
        }
        self.pending.push(tx);
        self.pending.sort_by(|a, b| b.gas_price.cmp(&a.gas_price));
        Ok(())
    }

    pub fn len(&self) -> usize { self.pending.len() }
    pub fn is_empty(&self) -> bool { self.pending.is_empty() }

    /// Drains up to `limit` highest-priority transactions for block inclusion.
    pub fn drain_for_block(&mut self, limit: usize) -> Vec<PendingTransaction> {
        let n = limit.min(self.pending.len());
        self.pending.drain(..n).collect()
    }
}
