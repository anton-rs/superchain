//! Node error types.

use crate::ConfigError;

/// A high-level `Node`error.
#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    /// An error occurred during standalone initialization.
    #[error("standalone initialization failed")]
    StandaloneInit,
    /// An error from a provider method.
    #[error("provider error: {0}")]
    Provider(String),
    /// An error thrown by a [crate::Config] operation.
    #[error("config error: {0}")]
    Beacon(#[from] ConfigError),
}

impl From<alloy_transport::TransportError> for NodeError {
    fn from(e: alloy_transport::TransportError) -> Self {
        Self::Provider(e.to_string())
    }
}

impl From<hilo_driver::ConfigError> for NodeError {
    fn from(e: hilo_driver::ConfigError) -> Self {
        match e {
            hilo_driver::ConfigError::Beacon(e) => Self::Beacon(ConfigError::Beacon(e)),
            hilo_driver::ConfigError::L2ChainProvider(e) => Self::Provider(e),
            hilo_driver::ConfigError::ChainProvider(e) => Self::Provider(e),
        }
    }
}
