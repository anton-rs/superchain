//! Executor types.

use alloy_consensus::{Header, Sealed};
use alloy_primitives::B256;
use kona_driver::{Executor, ExecutorConstructor};
use op_alloy_rpc_types_engine::OpPayloadAttributes;

/// An executor error.
#[derive(Debug, thiserror::Error)]
pub enum ExecutorError {
    /// An error occurred while executing the payload.
    #[error("An error occurred while executing the payload")]
    PayloadError,
    /// An error occurred while computing the output root.
    #[error("An error occurred while computing the output root")]
    OutputRootError,
}

/// An executor wrapper type.
#[derive(Default, Debug)]
pub struct HiloExecutor {}

impl HiloExecutor {
    /// Creates a new executor.
    pub const fn new() -> Self {
        Self {}
    }
}

impl Executor for HiloExecutor {
    type Error = ExecutorError;

    /// Execute the given payload attributes.
    fn execute_payload(&mut self, _: OpPayloadAttributes) -> Result<&Header, Self::Error> {
        todo!()
    }

    /// Computes the output root.
    fn compute_output_root(&mut self) -> Result<B256, Self::Error> {
        todo!()
    }
}

/// An executor constructor.
#[derive(Default, Debug)]
pub struct HiloExecutorConstructor {
    // todo: see kona executor constructor
}

impl HiloExecutorConstructor {
    /// Creates a new executor constructor.
    pub const fn new() -> Self {
        Self {}
    }
}

impl ExecutorConstructor<HiloExecutor> for HiloExecutorConstructor {
    /// Constructs the executor.
    fn new_executor(&self, _: Sealed<Header>) -> HiloExecutor {
        todo!()
    }
}
