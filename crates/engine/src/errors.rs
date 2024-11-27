//! Error types

/// An error that originated from the engine api.
#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    /// An error occurred while executing the payload.
    #[error("An error occurred while executing the payload")]
    PayloadError,
    /// An error occurred while computing the output root.
    #[error("An error occurred while computing the output root")]
    OutputRootError,
    /// Invalid block tag used to fetch the L2 block ref.
    #[error("Invalid block tag. Use `latest` or a block number.")]
    InvalidBlockTag,
    /// Failed to fetch the latest block number from the l2 rpc provider.
    #[error("Failed to fetch the latest block number from the l2 rpc provider")]
    LatestBlockNumber,
    /// Failed to get the `L2BlockInfo` for the given block number.
    #[error("Failed to get the `L2BlockInfo` for the given block number")]
    L2BlockInfoFetch,
}
