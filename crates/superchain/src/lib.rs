#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/anton-rs/superchain/main/assets/superchain.png",
    html_favicon_url = "https://avatars.githubusercontent.com/u/139668603?s=256",
    issue_tracker_base_url = "https://github.com/anton-rs/superchain/issues/"
)]
#![warn(missing_debug_implementations, missing_docs, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub use superchain_primitives::*;
pub use superchain_primitives as primitives;

#[cfg(feature = "serde")]
pub use superchain_registry as registry;

#[cfg(feature = "serde")]
pub use superchain_registry::{
    Chain, ChainList, HashMap, Registry, CHAINS, OPCHAINS, ROLLUP_CONFIGS,
};
