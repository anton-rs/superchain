#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/anton-rs/hilo/issues/")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

// Re-export the [JwtSecret] type from alloy_rpc_types_engine.
pub use alloy_rpc_types_engine::JwtSecret;

mod api;
pub use api::EngineApi;

mod types;
pub use types::{
    DEFAULT_AUTH_PORT, ENGINE_FORKCHOICE_UPDATED_TIMEOUT, ENGINE_FORKCHOICE_UPDATED_V2,
    ENGINE_GET_PAYLOAD_TIMEOUT, ENGINE_GET_PAYLOAD_V2, ENGINE_NEW_PAYLOAD_TIMEOUT,
    ENGINE_NEW_PAYLOAD_V2, JSONRPC_VERSION, STATIC_ID,
};

mod traits;
pub use traits::Engine;
