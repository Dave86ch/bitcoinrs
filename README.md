bitcoinrs

Goal: a learning implementation of core Bitcoin concepts in Rust, built to understand the protocol by re-creating its moving parts (networking, mining, blocks/txs, simple wallet). 
Corner of Berkshire & Fairfax
GitHub

Scope: readable, modular code > production completeness. Ideal for students and engineers exploring Bitcoin internals.

Features (WIP)

Protocol primitives (lib)

Blocks, headers, transactions, serialization

Hashing (double-SHA256), base58/bech32 address utils

Minimal script ops (subset) for validation experiments

Node

Basic P2P handshake, header/block sync (toy)

In-memory UTXO set (educational)

Miner

Simple PoW loop (adjustable difficulty for demos)

Coinbase construction; header nonce iterations

Wallet

Key generation (BIP-32/39/44 subset or ad-hoc keys)

Address derivation; raw tx building & signing (demo)

Note: this project prioritizes learning. Security, consensus edge-cases, and full mainnet compatibility are out of scope (for now).

Repo layout

lib/ — protocol primitives and shared types

node/ — toy full-node components (p2p, sync)

miner/ — PoW demo miner

wallet/ — simple key/tx tooling

Cargo.toml — workspace manifest

Quick start

Prereqs

Rust toolchain (rustup, stable or newer): curl https://sh.rustup.rs -sSf | sh

OS: Linux/macOS/Windows

Build everything

cargo build --workspace --release

Run (examples; adjust package names if needed)

Node: cargo run -p node

Miner: cargo run -p miner

Wallet (help): cargo run -p wallet -- --help

Test & lint

cargo test --workspace

cargo fmt --all && cargo clippy --workspace -D warnings

Learning roadmap

 Clean, well-commented serialization for blocks/txs

 Minimal script interpreter (verify P2PKH/P2WPKH)

 Header-only sync; SPV demo

 Mempool basics & fee selection (toy)

 Simple block validation rules (timestamps, difficulty)

 Stratum-style miner I/O mock

 Wallet: deterministic keys; PSBT demo

 Testnet/regtest harness with vectors

Design notes

Clarity first: small, isolated modules; heavy inline docs.

Deterministic tests: property tests with known vectors where possible.

No unsafe: stick to safe Rust; minimize dependencies.

Seams for experiments: make it easy to swap hashing, storage, or networking layers.

Contributing

Issues and PRs welcome. Keep changes small and documented.

Please include:

Rationale and references

Tests (unit/property) for new logic

Bench or size notes if performance-related

References

S. Nakamoto, Bitcoin: A Peer-to-Peer Electronic Cash System

A. Antonopoulos, Mastering Bitcoin (O’Reilly)

Bitcoin Core source & developer docs

Braiins educational material on Bitcoin/mining (the inspiration for this learning build). 
Corner of Berkshire & Fairfax

License

MIT (or your preferred OSS license—add LICENSE if different).
