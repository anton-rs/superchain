//! Error types

/// An error that originated from the engine api.
#[derive(Debug, thiserror::Error)]
pub enum EngineApiError {
    /// An error occurred while executing the payload.
    #[error("An error occurred while executing the payload")]
    PayloadError,
    /// An error occurred while computing the output root.
    #[error("An error occurred while computing the output root")]
    OutputRootError,
}
