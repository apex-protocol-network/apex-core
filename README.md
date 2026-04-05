# apex-core

Core protocol implementation for the APEX Layer 1 blockchain.

## Overview

`apex-core` is the foundational library for the APEX protocol — a post-quantum Layer 1 blockchain designed for long-horizon financial infrastructure. This crate defines canonical data structures, trait interfaces, and logic for consensus, account management, state transitions, and cryptographic operations.

## Modules

| Module | Description |
|--------|-------------|
| `consensus` | BFT consensus engine and validator coordination |
| `crypto` | Post-quantum signature traits and key derivation |
| `mempool` | Pending transaction pool and priority ordering |
| `network` | P2P message types and protocol abstractions |
| `state` | Account model, block structure, and state tree |
| `types` | Shared primitive types (Hash, Address, Amount) |

## Status

> **Pre-alpha.** Architecture scaffold committed. No components are production-ready.

## Requirements

- Rust 1.75+

## Build

```sh
cargo build
```

## Test

```sh
cargo test
```

## Related Repositories

| Repository | Description |
|------------|-------------|
| [apex-crypto](https://github.com/apex-protocol/apex-crypto) | PQC primitive implementations |
| [apex-node](https://github.com/apex-protocol/apex-node) | Node software and P2P networking |
| [apex-docs](https://github.com/apex-protocol/apex-docs) | Protocol specification |

## License

[Apache License 2.0](./LICENSE)
