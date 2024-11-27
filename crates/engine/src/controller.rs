//! Contains the engine controller.
//!
//! See: <https://github.com/ethereum-optimism/optimism/blob/develop/op-node/rollup/engine/engine_controller.go#L46>

use alloy_consensus::{Header, Sealed};
use alloy_rpc_types_engine::JwtSecret;
use kona_driver::ExecutorConstructor;
use url::Url;

use crate::EngineClient;

/// The engine controller.
#[derive(Debug, Clone)]
pub struct EngineController {
    /// The L2 engine API URL
    pub l2_engine_url: Url,
    /// Engine API JWT Secret.
    /// This is used to authenticate with the engine API
    pub jwt_secret: JwtSecret,
    /// The inner engine client which implements [crate::Engine].
    #[allow(unused)]
    client: EngineClient,
}

impl EngineController {
    /// Creates a new engine controller.
    pub fn new(l2_engine_url: Url, jwt_secret: JwtSecret) -> Self {
        let client = EngineClient::new_http(l2_engine_url.clone(), jwt_secret);
        Self { l2_engine_url, jwt_secret, client }
    }
}

impl ExecutorConstructor<EngineClient> for EngineController {
    fn new_executor(&self, _: Sealed<Header>) -> EngineClient {
        EngineClient::new_http(self.l2_engine_url.clone(), self.jwt_secret)
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
