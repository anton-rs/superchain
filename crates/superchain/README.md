# `superchain`

The `superchain` is an optionally `no_std` crate that provides core types
and bindings for the Superchain.

[`superchain`][sup] is an optionally `no_std` crate, by disabling
the `std` feature flag. By default, `std` is enabled, providing standard
library support.

[`serde`][s] is a required dependency.

## Usage

Add the following to your `Cargo.toml`.

```toml
[dependencies]
superchain = "0.5"
```

To make `superchain` work in a `no_std` environment, toggle `default-features` off like so.

```toml
[dependencies]
superchain = { version = "0.5", default-features = false }
```

## Example

[`superchain`][sup] exposes lazily defined mappings from chain id
to chain configurations. Below demonstrates getting the `RollupConfig`
for OP Mainnet (Chain ID `10`).

```rust
use superchain::ROLLUP_CONFIGS;

let op_chain_id = 10;
let op_rollup_config = ROLLUP_CONFIGS.get(&op_chain_id);
println!("OP Mainnet Rollup Config: {:?}", op_rollup_config);
```

A mapping from chain id to `ChainConfig` is also available.

```rust
use superchain::OPCHAINS;

let op_chain_id = 10;
let op_chain_config = OPCHAINS.get(&op_chain_id);
println!("OP Mainnet Chain Config: {:?}", op_chain_config);
```

## Feature Flags

- `std`: Uses the standard library to pull in environment variables.

<!-- Hyperlinks -->

[sp]: ../superchain-primitives

[gsr]: https://github.com/ethereum-optimism/superchain-registry
[s]: https://crates.io/crates/serde
[sr]: https://github.com/anton-rs/superchain
[sup]: https://crates.io/crates/superchain
