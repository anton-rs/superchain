//! A constructor wrapping the engine api client.

use crate::EngineApi;
use alloy_consensus::{Header, Sealed};
use alloy_rpc_types_engine::JwtSecret;
use kona_driver::ExecutorConstructor;
use url::Url;

/// An executor constructor.
#[derive(Clone, Debug)]
pub struct HiloExecutorConstructor {
    /// The L2 engine API URL
    pub l2_engine_url: Url,
    /// Engine API JWT Secret.
    /// This is used to authenticate with the engine API
    pub jwt_secret: JwtSecret,
}

impl HiloExecutorConstructor {
    /// Creates a new executor constructor.
    pub const fn new_http(engine: Url, jwt: JwtSecret) -> Self {
        Self { l2_engine_url: engine, jwt_secret: jwt }
    }
}

impl ExecutorConstructor<EngineApi> for HiloExecutorConstructor {
    /// Constructs the executor.
    fn new_executor(&self, _: Sealed<Header>) -> EngineApi {
        EngineApi::new_http(self.l2_engine_url.clone(), self.jwt_secret)
    }
}
