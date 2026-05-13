//! Rust + WASM gRPC-Web client for Sentrix Chain.
//!
//! Compiles to WASM via `wasm-pack build --target web` and is usable
//! from any browser dApp that wants typed gRPC-Web access to the
//! chain's `sentrix.v1.Sentrix` service. Extracted from
//! `sentrix-explorer-v2` so other browser apps can reuse the wrapper
//! instead of re-implementing the tonic-web-wasm-client glue.
//!
//! For non-WASM Rust services (validators, indexers, bridges) use
//! `sentrix-chain` (https://github.com/Sentriscloud/sdk-rs) `grpc`
//! feature instead — that one wraps `@grpc/grpc-js`-equivalent
//! `tonic` over native HTTP/2.
//!
//! Endpoints:
//!   - mainnet: `https://grpc.sentrixchain.com`
//!   - testnet: `https://grpc-testnet.sentrixchain.com`
//!
//! Caddy at the edge transcodes between gRPC-Web and native gRPC via
//! `tonic-web` so the same hostname serves both browser and server
//! consumers.

#![deny(unsafe_code)]
#![allow(missing_docs)]
#![allow(clippy::doc_lazy_continuation)]

/// Generated tonic + prost types for the `sentrix.v1` schema. Re-export
/// of the `sentrix-proto` crate published from the chain repo, so this
/// crate stays in sync with the server schema without vendoring its
/// own copy.
pub mod pb {
    pub use sentrix_proto::*;
}

pub mod client;

pub use client::{hash_short, SentrixGrpcClient};
