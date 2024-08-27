<h1 align="center">
    <img src="./assets/banner.png" alt="Superchain" width="100%" align="center">
</h1>

<h4 align="center">
    Rust bindings for the <a href="https://github.com/ethereum-optimism/superchain-registry">superchain-registry</a>.
</h4>

<p align="center">
  <a href="https://github.com/anton-rs/superchain/actions/workflows/rust_ci.yaml"><img src="https://github.com/anton-rs/superchain/actions/workflows/rust_ci.yaml/badge.svg?label=ci" alt="CI"></a>
  <img src="https://img.shields.io/badge/License-MIT-green.svg?label=license&labelColor=2a2f35" alt="License">
  <a href="https://github.com/ethereum-optimism/monorepo"><img src="https://img.shields.io/badge/OP%20Stack-monorepo-red?labelColor=2a2f35" alt="OP Stack"></a>
</p>

<p align="center">
  <a href="#whats-superchain">What's Superchain?</a> •
  <a href="#usage">Usage</a> •
  <a href="#example">Example</a> •
  <a href="#credits">Credits</a>
</p>

## What's Superchain?

The [Superchain](https://docs.optimism.io/stack/explainer) is a network of chains that share bridging, decentralized governance, upgrades, a communication layer and more.

This repository contains rust bindings for the [superchain-registry][sr].

The best way to work with this repo is through the [`superchain`][sup]
crate. [`superchain`][sup] is an optionally `no_std` crate that provides
core types and bindings for the Superchain.

It re-exports two crates:
- [`superchain-primitives`][scp]
- [`superchain-registry`][scr] _Only available if `serde` feature flag is enabled_

[`superchain-primitives`][scp] defines core types used in the `superchain-registry`
along with a few default values for core chains.

[`superchain-registry`][scr] provides bindings to all chains in the `superchain`.

## Usage

Add the following to your `Cargo.toml`.

```toml
[dependencies]
superchain = "0.2"
```

To make make `superchain` `no_std`, toggle `default-features` off like so.

```toml
[dependencies]
superchain = { version = "0.2", default-features = false }
```

## Example

[`superchain-registry`][scr] exposes lazily defined mappings from chain id
to chain configurations that the [`superchain`][sup] re-exports. Below
demonstrates getting the `RollupConfig` for OP Mainnet (Chain ID `10`).

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

- `serde`: Enables [`serde`][s] support for types and makes [`superchain-registry`][scr] types available.
- `std`: Uses the standard library to pull in environment variables.

## Credits

This repository could not be built without OP Labs contributors working on the [superchain-registry][scr] and [alloy-rs](https://github.com/alloy-rs) contributors.

<!-- Hyperlinks -->

[sp]: ./crates/superchain-primitives

[s]: https://crates.io/crates/serde
[sr]: https://github.com/ethereum-optimism/superchain-registry
[scr]: https://crates.io/crates/superchain-registry
[sup]: https://crates.io/crates/superchain
[scp]: https://crates.io/crates/superchain-primitives
[superchain]: https://github.com/anton-rs/superchain
