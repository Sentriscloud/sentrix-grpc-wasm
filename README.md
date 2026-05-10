# sentrix-grpc-wasm

[![CI](https://github.com/Sentriscloud/sentrix-grpc-wasm/actions/workflows/ci.yml/badge.svg)](https://github.com/Sentriscloud/sentrix-grpc-wasm/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Rust + WASM gRPC-Web client for **Sentrix Chain**. Compiles to WASM via [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) and is consumable from any browser dApp that wants typed gRPC-Web access to the chain's `sentrix.v1.Sentrix` service.

Extracted from [`sentrix-explorer-v2`](https://github.com/Sentriscloud/sentrix-explorer-v2) so other browser apps can reuse the wrapper instead of re-implementing the [`tonic-web-wasm-client`](https://crates.io/crates/tonic-web-wasm-client) glue.

## Picking the right SDK

| Use case | Crate / package |
|---|---|
| **Browser dApp, Rust+WASM** (Leptos, Yew, plain wasm-bindgen) | **this crate** |
| **Browser dApp, TypeScript** | [`@sentrix/chain/grpc-web`](https://github.com/Sentriscloud/sdk-ts) (uses `@protobuf-ts/grpcweb-transport`) |
| **Node.js service** (Rust) | [`sentrix-chain`](https://github.com/Sentriscloud/sdk-rs) `grpc` feature |
| **Node.js service** (TypeScript) | [`@sentrix/chain/grpc`](https://github.com/Sentriscloud/sdk-ts) (uses `@grpc/grpc-js`) |

All four hit the same `grpc.sentrixchain.com:443` endpoint — Caddy at the edge transcodes between gRPC-Web and native gRPC via `tonic-web`.

## Quick start

```rust
use sentrix_grpc_wasm::SentrixGrpcClient;

#[wasm_bindgen]
pub async fn read_tip() -> Result<String, JsError> {
    let mut client = SentrixGrpcClient::new("https://grpc.sentrixchain.com");
    let block = client.get_latest_block().await
        .map_err(|s| JsError::new(&format!("rpc: {s}")))?;
    Ok(format!("tip: {}", block.index))
}
```

Build to WASM:

```bash
wasm-pack build --target web --release
```

Then `import` the generated `pkg/` from your bundler (Vite, webpack, etc).

## Available calls (chain v0.4+)

| Method | Returns |
|---|---|
| `get_latest_block()` / `get_block_by_height(h)` | `pb::Block` |
| `get_balance(address)` | `pb::Account` (20-byte address) |
| `get_validator_set()` | `pb::ValidatorSet` |
| `get_supply()` | `pb::Supply` |
| `get_mempool(limit)` | `pb::Mempool` |
| `subscribe_events(filters)` | server-stream `pb::ChainEvent` |

Older chain hosts (v0.2 / v0.3) return `Status::unimplemented` for the newer methods; the SDK forwards the error verbatim so callers can fall back to JSON-RPC / REST.

## Endpoints

| Network | URL |
|---|---|
| Mainnet | `https://grpc.sentrixchain.com` |
| Testnet | `https://grpc-testnet.sentrixchain.com` |

## Roadmap

- [x] Extract from `sentrix-explorer-v2/src/grpc/`
- [x] Standalone crate with `tonic-build` codegen
- [x] CI: `cargo build` + `cargo build --target wasm32-unknown-unknown`
- [ ] Smoke test: `wasm-pack test --headless --chrome` against staging endpoint
- [ ] Publish to crates.io once API surface stabilises
- [ ] Optional: pre-built WASM artifact npm package (skip the local `wasm-pack` step for JS consumers)

## Status

`v0.1.0-alpha.0` — works, used in production by [`sentrix-explorer-v2`](https://github.com/Sentriscloud/sentrix-explorer-v2). Surface is the same as the explorer was using; nothing here is speculative.

## License

MIT — see [LICENSE](./LICENSE).
