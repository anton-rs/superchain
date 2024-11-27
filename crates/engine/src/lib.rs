#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/anton-rs/hilo/issues/")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

mod validation;
pub use validation::ValidationMode;

mod traits;
pub use traits::Engine;

mod errors;
pub use errors::EngineError;

mod controller;
pub use controller::EngineController;

mod client;
pub use client::EngineClient;

mod validator;
pub use validator::{TrustedPayloadValidator, TrustedValidationError};
