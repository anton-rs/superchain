#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/anton-rs/hilo/issues/")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

mod validation;
pub use validation::ValidationMode;

mod api;
pub use api::EngineApi;

mod validator;
pub use validator::{TrustedPayloadValidator, TrustedValidationError};
