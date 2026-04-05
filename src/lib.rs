//! # apex-core
//!
//! Core protocol library for the APEX Layer 1 blockchain.
//! Provides canonical types, interfaces, and logic for
//! consensus, cryptography, networking, and state management.

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

pub mod consensus;
pub mod crypto;
pub mod mempool;
pub mod network;
pub mod state;
pub mod types;
