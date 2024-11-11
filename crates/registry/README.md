## `super-registry`

<a href="https://github.com/anton-rs/super/actions/workflows/rust_ci.yaml"><img src="https://github.com/anton-rs/super/actions/workflows/rust_ci.yaml/badge.svg?label=ci" alt="CI"></a>
<a href="https://crates.io/crates/super-registry"><img src="https://img.shields.io/crates/v/super-derive.svg?label=super-registry&labelColor=2a2f35" alt="Kona Derive Alloy"></a>
<a href="https://github.com/anton-rs/super/blob/main/LICENSE.md"><img src="https://img.shields.io/badge/License-MIT-d1d1f6.svg?label=license&labelColor=2a2f35" alt="License"></a>
<a href="https://img.shields.io/codecov/c/github/anton-rs/super"><img src="https://img.shields.io/codecov/c/github/anton-rs/super" alt="Codecov"></a>


[`super-registry`][sc] is a `no_std` crate that exports rust type definitions for chains
in the [`superchain-registry`][osr]. Since it reads static files to read configurations for
various chains into instantiated objects, the [`super-registry`][sc] crate requires
[`serde`][serde] as a dependency. To use the [`super-registry`][sc] crate, add the crate
as a dependency to a `Cargo.toml`.

```toml
super-registry = "0.9"
```

[`super-registry`][sc] declares lazy evaluated statics that expose `ChainConfig`s, `RollupConfig`s,
and `Chain` objects for all chains with static definitions in the superchain registry. The way this works
is the the golang side of the superchain registry contains an "internal code generation" script that has
been modified to output configuration files to the [`crates/registry`][s] directory in the
`etc` folder that are read by the [`super-registry`][sc] rust crate. These static config files
contain an up-to-date list of all superchain configurations with their chain configs.

There are three core statics exposed by the [`super-registry`][sc].
- `CHAINS`: A list of chain objects containing the superchain metadata for this chain.
- `OPCHAINS`: A map from chain id to `ChainConfig`.
- `ROLLUP_CONFIGS`: A map from chain id to `RollupConfig`.

While the [`op-alloy-genesis`][oag] crate contains a few hardcoded `RollupConfig` objects, the
[`super-registry`][sc] exports the _complete_ list of superchains and their chain's `RollupConfig`s
and `ChainConfig`s.

[`CHAINS`][chains], [`OPCHAINS`][opchains], and [`ROLLUP_CONFIGS`][rollups] are exported at the top-level
of the [`superchain`][sc] crate and can be used directly. See the [usage](#usage) section
below for how to work with [`superchain`][sc] mappings.


### Usage

Add the following to your `Cargo.toml`.

```toml
[dependencies]
super-registry = "0.9"
```

To make `super-registry` `no_std`, toggle `default-features` off like so.

```toml
[dependencies]
super-registry = { version = "0.9", default-features = false }
```

Below demonstrates getting the `RollupConfig` for OP Mainnet (Chain ID `10`).

```rust
use super_registry::ROLLUP_CONFIGS;

let op_chain_id = 10;
let op_rollup_config = ROLLUP_CONFIGS.get(&op_chain_id);
println!("OP Mainnet Rollup Config: {:?}", op_rollup_config);
```

A mapping from chain id to `ChainConfig` is also available.

```rust
use super_registry::OPCHAINS;

let op_chain_id = 10;
let op_chain_config = OPCHAINS.get(&op_chain_id);
println!("OP Mainnet Chain Config: {:?}", op_chain_config);
```


### Feature Flags

- `std`: Uses the standard library to pull in environment variables.


### Credits

[superchain-registry][osr] contributors for building and maintaining superchain types.

[alloy] and [op-alloy] for creating and maintaining high quality Ethereum and Optimism types in rust.


<!-- Hyperlinks -->

[serde]: https://crates.io/crates/serde
[alloy]: https://github.com/alloy-rs/alloy
[op-alloy]: https://github.com/alloy-rs/op-alloy
[super]: https://github.com/anton-rs/super
[op-superchain]: https://docs.optimism.io/stack/explainer
[osr]: https://github.com/ethereum-optimism/superchain-registry

[s]: ./crates/registry
[sc]: https://crates.io/crates/super-registry

[oag]: https://crates.io/crates/op-alloy-genesis
[chains]: https://docs.rs/superchain-registry/latest/superchain/struct.CHAINS.html
[opchains]: https://docs.rs/superchain-registry/latest/superchain/struct.OPCHAINS.html
[rollups]: https://docs.rs/superchain-registry/latest/superchain/struct.ROLLUP_CONFIGS.html
