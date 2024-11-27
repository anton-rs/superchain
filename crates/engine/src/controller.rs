//! Contains the engine controller.
//!
//! See: <https://github.com/ethereum-optimism/optimism/blob/develop/op-node/rollup/engine/engine_controller.go#L46>

use alloy_consensus::{Header, Sealed};
use alloy_primitives::B256;
use alloy_rpc_types_engine::{ForkchoiceState, JwtSecret};
use async_trait::async_trait;
use kona_driver::Executor;
use op_alloy_genesis::RollupConfig;
use op_alloy_protocol::BlockInfo;
use op_alloy_rpc_types_engine::OpPayloadAttributes;
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use url::Url;

use crate::{Engine, EngineClient, EngineError};

/// L1 epoch block
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Epoch {
    /// The block number
    pub number: u64,
    /// The block hash
    pub hash: B256,
    /// The block timestamp
    pub timestamp: u64,
}

impl From<BlockInfo> for Epoch {
    fn from(block: BlockInfo) -> Self {
        Self { number: block.number, hash: block.hash, timestamp: block.timestamp }
    }
}

/// The engine controller.
#[derive(Debug, Clone)]
pub struct EngineController {
    /// The inner engine client which implements [crate::Engine].
    pub client: EngineClient,
    /// Blocktime of the L2 chain
    pub blocktime: u64,
    /// Most recent block found on the p2p network
    pub unsafe_head: BlockInfo,
    /// Most recent block that can be derived from L1 data
    pub safe_head: BlockInfo,
    /// Batch epoch of the safe head
    pub safe_epoch: Epoch,
    /// Most recent block that can be derived from finalized L1 data
    pub finalized_head: BlockInfo,
    /// Batch epoch of the finalized head
    pub finalized_epoch: Epoch,
}

impl EngineController {
    /// Creates a new engine controller.
    pub fn new(
        l2_engine_url: Url,
        l2_rpc_url: Url,
        jwt_secret: JwtSecret,
        finalized_head: BlockInfo,
        finalized_epoch: Epoch,
        config: &RollupConfig,
    ) -> Self {
        let client = EngineClient::new_http(
            l2_engine_url.clone(),
            l2_rpc_url,
            Arc::new(config.clone()),
            jwt_secret,
        );
        Self {
            blocktime: config.block_time,
            unsafe_head: finalized_head,
            safe_head: finalized_head,
            safe_epoch: finalized_epoch,
            finalized_head,
            finalized_epoch,
            client,
        }
    }

    /// Creates a [ForkchoiceState]:
    /// - `head_block` = `unsafe_head`
    /// - `safe_block` = `safe_head`
    /// - `finalized_block` = `finalized_head`
    pub fn create_forkchoice_state(&self) -> ForkchoiceState {
        ForkchoiceState {
            head_block_hash: self.unsafe_head.hash,
            safe_block_hash: self.safe_head.hash,
            finalized_block_hash: self.finalized_head.hash,
        }
    }
}

#[async_trait]
impl Executor for EngineController {
    type Error = EngineError;

    /// Waits for the engine to be ready.
    async fn wait_until_ready(&mut self) {
        let forkchoice = self.create_forkchoice_state();
        while self.client.forkchoice_update(forkchoice, None).await.is_err() {
            sleep(Duration::from_secs(1)).await;
        }
    }

    /// Updates the safe head.
    fn update_safe_head(&mut self, header: Sealed<Header>) {
        if self.safe_head.number < header.number {
            self.safe_head = BlockInfo {
                number: header.number,
                hash: header.hash_slow(),
                timestamp: header.timestamp,
                parent_hash: header.parent_hash,
            };
            self.safe_epoch = self.safe_head.into();
        }

        if header.number > self.unsafe_head.number {
            self.unsafe_head = BlockInfo {
                number: header.number,
                hash: header.hash_slow(),
                timestamp: header.timestamp,
                parent_hash: header.parent_hash,
            };
        }
    }

    /// Execute the given payload attributes.
    fn execute_payload(&mut self, _: OpPayloadAttributes) -> Result<&Header, Self::Error> {
        todo!()
    }

    /// Computes the output root.
    fn compute_output_root(&mut self) -> Result<B256, Self::Error> {
        todo!()
    }
}

// /// A validation error
// #[derive(Debug, thiserror::Error)]
// pub enum ValidationError {
//     /// An RPC error
//     #[error("RPC error")]
//     RpcError,
// }

// Validates the payload using the Fork Choice Update API.
// pub async fn validate_payload_fcu(
//     &self,
//     attributes: &OpAttributesWithParent,
// ) -> Result<bool, ValidationError> {
//     // TODO: use the correct values
//     let fork_choice_state = ForkchoiceState {
//         head_block_hash: attributes.parent.block_info.hash,
//         finalized_block_hash: attributes.parent.block_info.hash,
//         safe_block_hash: attributes.parent.block_info.hash,
//     };
//
//     let attributes = Some(attributes.attributes.clone());
//     let fcu = self
//         .provider
//         .fork_choice_updated_v2(fork_choice_state, attributes)
//         .await
//         .map_err(|_| ValidationError::RpcError)?;
//
//     if fcu.is_valid() {
//         Ok(true)
//     } else {
//         warn!(status = %fcu.payload_status, "Engine API returned invalid fork choice update");
//         Ok(false)
//     }
// }
