//! Consensus module.
//!
//! APEX uses a Tendermint-derived BFT consensus protocol
//! with CRYSTALS-Dilithium3 validator signing.
//! Full BFT implementation is staged through the testnet phase.

/// Consensus round number.
pub type Round = u64;

/// Consensus configuration parameters.
#[derive(Debug, Clone)]
pub struct ConsensusConfig {
    /// Target block production interval (milliseconds).
    pub block_time_ms: u64,
    /// Maximum active validator set size.
    pub max_validators: u32,
    /// Minimum bonded stake to participate as a validator (atomic units).
    pub min_validator_stake: u128,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            block_time_ms: 2_000,
            max_validators: 128,
            min_validator_stake: 1_000_000 * 10u128.pow(18),
        }
    }
}
