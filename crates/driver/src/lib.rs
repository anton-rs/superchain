#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/anton-rs/hilo/issues/")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

#[macro_use]
extern crate tracing;

mod config;
pub use config::{Config, ConfigError};

mod executor;
pub use executor::{HiloExecutor, HiloExecutorConstructor};

mod driver;
pub use driver::HiloDriver;

mod context;
pub use context::{Context, StandaloneContext};

mod pipeline;
pub use pipeline::{
    HiloAttributesBuilder, HiloAttributesQueue, HiloDataProvider, HiloDerivationPipeline,
    HiloPipeline,
};
