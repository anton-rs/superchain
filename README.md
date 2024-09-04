<h1 align="center">
    <img src="./assets/banner.png" alt="Superchain" width="100%" align="center">
</h1>

<h4 align="center">
    Rust bindings for the <a href="https://github.com/ethereum-optimism/superchain-registry">superchain-registry</a>.
</h4>

<p align="center">
  <a href="https://github.com/anton-rs/superchain/actions/workflows/rust_ci.yaml"><img src="https://github.com/anton-rs/superchain/actions/workflows/rust_ci.yaml/badge.svg?label=ci" alt="CI"></a>
  <img src="https://img.shields.io/badge/License-MIT-green.svg?label=license&labelColor=2a2f35" alt="License">
  <a href="https://docs.optimism.io/"><img src="https://img.shields.io/badge/Docs-optimism.io-854a15?logo=mdBook&labelColor=2a2f35" alt="OP Stack Docs"></a>
  <a href="https://github.com/ethereum-optimism/monorepo"><img src="https://img.shields.io/badge/OP%20Stack-config-red?labelColor=2a2f35" alt="OP Stack"></a>
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
[superchain-registry][sr] directory. This repository provides rust bindings
for the [ethereum-optimism/superchain-registry][osr].

The best way to interface with these rust bindings is through [`superchain`][s],
an optionally `no_std` crate that re-exports two core crates in this workspace.
- [`superchain-primitives`][sp]
- [`superchain-registry`][sr] _Only available if `serde` feature flag is enabled_


### `superchain-primitives`

[`superchain-primitives`][spc] is a `no_std` crate that contains rust types for the
configuration objects defined in the [`superchain-registry`][osr]. There are two
feature flags available on [`superchain-primitives`][spc], `std` and `serde`,
enabling the `std` library use and [`serde`][serde] serialization and deserialization
support.

Both `serde` and `std` are enabled by default but can be individually enabled when
`default-features = false`. In a project's `Cargo.toml`, to use the `superchain-primitives`
crate with `serde` support, add the following to the `[dependencies]` section.

```toml
# By default, superchain-primitives enables the `std` and `serde` feature flags.
superchain-primitives = { version = "0.3", features = [ "serde" ], default-features = false }
```

Alternatively, the [`superchain`][sc] crate can be used which re-exports the
[`superchain-primitives`][spc].

```toml
superchain = { version = "0.3", default-features = false }
```

[`superchain-primitives`][spc] has minimal dependencies itself and uses [alloy][alloy] Ethereum
types internally. Below provides a breakdown of core types defined in [`superchain-primitives`][spc].

**`ChainConfig`**

The [`ChainConfig`][cc-html] is an execution-layer construct defining a configuration for this chain.
It's output from the `add-chain` command in the [`superchain-registry`][osr]. It contains genesis
data, addresses for the onchain system config, hardfork timestamps, rpc information, and other
superchain-specific information. Static chain config files are defined in the
[`superchain-registry`][osr].

**`RollupConfig`**

The [`RollupConfig`][rc-html] defines the configuration for a rollup node, the consensus-layer.
The [`RollupConfig`][rc-html] defines the parameters used for deriving the L2 chain as well as
batch-submitting data on L1.

[`superchain-primitives`][spc] also exposes a few default `RollupConfig`s for convenience,
providing an alternative to depending on [`superchain-registry`][osr] with `serde` required.

**`Superchain`**

[`Superchain`][s-html] defines a superchain for a given L1 network. It holds metadata
such as the name of the superchain, the L1 anchor information (chain id, rpc, explorer), and
default hardfork configuration values. Within the [`Superchain`][s-html], there's a list
of [`ChainConfig`][cc-html]s that belong to this superchain.


### `superchain-registry`

[`superchain-registry`][src] is a `no_std` crate that exports rust type definitions for chains
in the [`superchain-registry`][osr]. Since it reads static files to read configurations for
various chains into instantiated objects, the [`superchain-registry`][osr] crate requires
`serde` as a dependency and enables `serde` features on dependencies including
[`superchain-primitives`][spc]. To use the [`superchain-regsitry`][src] crate, add the crate
as a dependency to a `Cargo.toml`.

```toml
# By default, superchain-registry enables the `std` feature, disabling `no_std`.
superchain-registry = { version = "0.3", default-features = false }
```

Alternatively, the [`superchain`][sc] crate can be used which re-exports the [`superchain-registry`][src].

```toml
superchain = "0.3"
```

[`superchain-registry`][src] declares lazy evaluated statics that expose `ChainConfig`s, `RollupConfig`s,
and `Chain` objects for all chains with static definitions in the superchain registry. The way this works
is the the golang side of the superchain registry contains an "internal code generation" script that has
been modified to output configuration files to the [`crates/superchain-registry`][sr] directory in the
`etc` folder that are read by the [`superchain-registry`][src] rust crate. These static config files
contain an up-to-date list of all superchain configurations with their chain configs.

There are three core statics exposed by the [`superchain-registry`][src].
- `CHAINS`: A list of chain objects containing the superchain metadata for this chain.
- `OPCHAINS`: A map from chain id to `ChainConfig`.
- `ROLLUP_CONFIGS`: A map from chain id to `RollupConfig`.

Where the [`superchain-primitives`][spc] crate contains a few hardcoded `RollupConfig` objects, the
[`superchain-registry`][src] exports the _complete_ list of superchains and their chain's `RollupConfig`s
and `ChainConfig`s, at the expense of requiring `serde`.

[`CHAINS`][chains], [`OPCHAINS`][opchains], and [`ROLLUP_CONFIGS`][rollups] are exported at the top-level
of the [`superchain-primitives`][spc] crate and can be used directly. See the [usage](#usage) section
below for how to work with [`superchain-registry`][src] mappings.


## Usage

Add the following to your `Cargo.toml`.

```toml
[dependencies]
superchain = "0.3"
```

To make `superchain` `no_std`, toggle `default-features` off like so.

> [!NOTE]
>
> The `superchain-registry` is only available if the `serde` feature is enabled. 


```toml
[dependencies]
superchain = { version = "0.3", default-features = false }
```

The re-exported [`superchain-registry`][src] exposes lazily defined mappings
from L2 chain id to chain configurations. Below demonstrates getting the
`RollupConfig` for OP Mainnet (Chain ID `10`).

```rust
use superchain::registry::ROLLUP_CONFIGS;

let op_chain_id = 10;
let op_rollup_config = ROLLUP_CONFIGS.get(&op_chain_id);
println!("OP Mainnet Rollup Config: {:?}", op_rollup_config);
```

A mapping from chain id to `ChainConfig` is also available.

```rust
use superchain::registry::OPCHAINS;

let op_chain_id = 10;
let op_chain_config = OPCHAINS.get(&op_chain_id);
println!("OP Mainnet Chain Config: {:?}", op_chain_config);
```

## Feature Flags

- `serde`: Enables [`serde`][serde] support for types and makes [`superchain-registry`][src] types available.
- `std`: Uses the standard library to pull in environment variables.

## Credits

[superchain-registry][osr] contributors for building and maintaining superchain types.

[Alloy][alloy] for creating and maintaining high quality Ethereum types in rust.


<!-- Hyperlinks -->

[serde]: https://crates.io/crates/serde
[alloy]: https://github.com/alloy-rs/alloy
[superchain-repo]: https://github.com/anton-rs/superchain
[op-superchain]: https://docs.optimism.io/stack/explainer
[osr]: https://github.com/ethereum-optimism/superchain-registry

[s]: ./crates/superchain
[sp]: ./crates/superchain-primitives
[sr]: ./crates/superchain-primitives
[sc]: https://crates.io/crates/superchain
[src]: https://crates.io/crates/superchain-registry
[spc]: https://crates.io/crates/superchain-primitives

[chains]: https://docs.rs/superchain-registry/latest/superchain_registry/struct.CHAINS.html
[opchains]: https://docs.rs/superchain-registry/latest/superchain_registry/struct.OPCHAINS.html
[rollups]: https://docs.rs/superchain-registry/latest/superchain_registry/struct.ROLLUP_CONFIGS.html

[s-html]: https://docs.rs/superchain-primitives/latest/superchain_primitives/superchain/struct.Superchain.html
[cc-html]: https://docs.rs/superchain-primitives/latest/superchain_primitives/chain_config/struct.ChainConfig.html
[rc-html]: https://docs.rs/superchain-primitives/latest/superchain_primitives/rollup_config/struct.RollupConfig.html
