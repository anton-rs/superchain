//! Error types

use alloy_rpc_types_engine::PayloadStatus;

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
    /// Invalid payload attributes were received from a fork choice update.
    #[error("Invalid payload attributes were received from a fork choice update")]
    InvalidForkChoiceAttributes,
    /// Invalid payload attributes were received from a new payload method.
    #[error("Invalid payload attributes were received from a new payload method")]
    InvalidNewPayloadAttributes,
    /// Missing payload id.
    #[error("Missing payload id")]
    MissingPayloadId,
}

/// An error that originated one level above the engine api,
/// in the [crate::EngineController].
#[derive(Debug, thiserror::Error)]
pub enum EngineControllerError {
    /// Invalid payload attributes were processed.
    #[error("Invalid payload attributes were processed")]
    InvalidPayloadAttributes,
    /// An error from the engine api.
    #[error("An error from the engine api: {0}")]
    EngineError(#[from] EngineError),
    /// The forkchoice update was rejected with the given payload status.
    #[error("The forkchoice update was rejected with the given payload status: {0:?}")]
    ForkchoiceRejected(PayloadStatus),
    /// Failed to fetch block.
    #[error("Failed to fetch block {0}")]
    BlockFetchFailed(u64),
}
