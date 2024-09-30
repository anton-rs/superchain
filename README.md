<h1 align="center">
    <img src="./assets/banner.png" alt="Superchain" width="100%" align="center">
</h1>

<h4 align="center">
    Rust bindings for the <a href="https://github.com/ethereum-optimism/superchain-registry">superchain-registry</a>.
</h4>

<p align="center">
  <a href="https://github.com/anton-rs/superchain/actions/workflows/rust_ci.yaml"><img src="https://github.com/anton-rs/superchain/actions/workflows/rust_ci.yaml/badge.svg?label=ci" alt="CI"></a>
  <a href="https://crates.io/crates/superchain"><img src="https://img.shields.io/crates/v/superchain.svg" alt="Superchain Crate"></a>
  <a href="https://github.com/anton-rs/superchain?tab=MIT-1-ov-file"><img src="https://img.shields.io/badge/License-MIT-green.svg?label=license&labelColor=2a2f35" alt="License"></a>
  <a href="https://docs.optimism.io/"><img src="https://img.shields.io/badge/Docs-optimism.io-854a15?logo=mdBook&labelColor=2a2f35" alt="OP Stack Docs"></a>
  <!-- <a href="https://github.com/ethereum-optimism/monorepo"><img src="https://img.shields.io/badge/OP%20Stack-config-red?labelColor=2a2f35" alt="OP Stack"></a> -->
</p>

<p align="center">
  <a href="#whats-superchain">What's Superchain?</a> •
  <a href="#usage">Usage</a> •
  <a href="#feature-flags">Feature Flags</a> •
  <a href="#credits">Credits</a>
</p>


## What's Superchain?

The [Superchain][op-superchain] is a network of chains that share bridging,
decentralized governance, upgrades, a communication layer and more.

Chain configurations for the [Superchain][op-superchain] are defined in the
[superchain][s] directory. This repository provides rust bindings
for the [ethereum-optimism/superchain-registry][osr].

Interface with the rust bindings through [`superchain`][s], an optionally
`no_std` crate that binds to the [ethereum-optimism/superchain-registry][osr]
at compile-time.


### `superchain`

[`superchain`][sc] is a `no_std` crate that exports rust type definitions for chains
in the [`superchain-registry`][osr]. Since it reads static files to read configurations for
various chains into instantiated objects, the [`superchain`][sc] crate requires
[`serde`][serde] as a dependency. To use the [`superchain`][sc] crate, add the crate
as a dependency to a `Cargo.toml`.

```toml
superchain = "0.7"
```

[`superchain`][sc] declares lazy evaluated statics that expose `ChainConfig`s, `RollupConfig`s,
and `Chain` objects for all chains with static definitions in the superchain registry. The way this works
is the the golang side of the superchain registry contains an "internal code generation" script that has
been modified to output configuration files to the [`crates/superchain`][s] directory in the
`etc` folder that are read by the [`superchain`][sc] rust crate. These static config files
contain an up-to-date list of all superchain configurations with their chain configs.

There are three core statics exposed by the [`superchain`][sc].
- `CHAINS`: A list of chain objects containing the superchain metadata for this chain.
- `OPCHAINS`: A map from chain id to `ChainConfig`.
- `ROLLUP_CONFIGS`: A map from chain id to `RollupConfig`.

While the [`op-alloy-genesis`][oag] crate contains a few hardcoded `RollupConfig` objects, the
[`superchain`][sc] exports the _complete_ list of superchains and their chain's `RollupConfig`s
and `ChainConfig`s.

[`CHAINS`][chains], [`OPCHAINS`][opchains], and [`ROLLUP_CONFIGS`][rollups] are exported at the top-level
of the [`superchain`][sc] crate and can be used directly. See the [usage](#usage) section
below for how to work with [`superchain`][sc] mappings.


## Usage

Add the following to your `Cargo.toml`.

```toml
[dependencies]
superchain = "0.7"
```

To make `superchain` `no_std`, toggle `default-features` off like so.

```toml
[dependencies]
superchain = { version = "0.7", default-features = false }
```

Below demonstrates getting the `RollupConfig` for OP Mainnet (Chain ID `10`).

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

## Credits

[superchain-registry][osr] contributors for building and maintaining superchain types.

[alloy] and [op-alloy] for creating and maintaining high quality Ethereum and Optimism types in rust.


<!-- Hyperlinks -->

[serde]: https://crates.io/crates/serde
[alloy]: https://github.com/alloy-rs/alloy
[op-alloy]: https://github.com/alloy-rs/op-alloy
[superchain-repo]: https://github.com/anton-rs/superchain
[op-superchain]: https://docs.optimism.io/stack/explainer
[osr]: https://github.com/ethereum-optimism/superchain-registry

[s]: ./crates/superchain
[sc]: https://crates.io/crates/superchain

[oag]: https://crates.io/crates/op-alloy-genesis
[chains]: https://docs.rs/superchain-registry/latest/superchain/struct.CHAINS.html
[opchains]: https://docs.rs/superchain-registry/latest/superchain/struct.OPCHAINS.html
[rollups]: https://docs.rs/superchain-registry/latest/superchain/struct.ROLLUP_CONFIGS.html
