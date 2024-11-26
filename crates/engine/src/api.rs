//! Contains the engine api client.

use http_body_util::Full;
use tower::ServiceBuilder;
use tracing::warn;
use url::Url;

use alloy_consensus::Header;
use alloy_network::AnyNetwork;
use alloy_primitives::{Bytes, B256};
use alloy_provider::RootProvider;
use alloy_rpc_client::RpcClient;
use alloy_rpc_types_engine::{ForkchoiceState, JwtSecret};
use alloy_transport_http::{
    hyper_util::{
        client::legacy::{connect::HttpConnector, Client},
        rt::TokioExecutor,
    },
    AuthLayer, AuthService, Http, HyperClient,
};
use kona_driver::Executor;
use op_alloy_provider::ext::engine::OpEngineApi;
use op_alloy_rpc_types_engine::{OpAttributesWithParent, OpPayloadAttributes};

/// A Hyper HTTP client with a JWT authentication layer.
type HyperAuthClient<B = Full<Bytes>> = HyperClient<B, AuthService<Client<HttpConnector, B>>>;

/// An external op-geth engine api client
#[derive(Debug, Clone)]
pub struct EngineApi {
    /// The inner provider
    provider: RootProvider<Http<HyperAuthClient>, AnyNetwork>,
}
/// A validation error
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// An RPC error
    #[error("RPC error")]
    RpcError,
}

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

impl Executor for EngineApi {
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

impl EngineApi {
    /// Creates a new [`EngineApi`] from the provided [Url] and [JwtSecret].
    pub fn new_http(url: Url, jwt: JwtSecret) -> Self {
        let hyper_client = Client::builder(TokioExecutor::new()).build_http::<Full<Bytes>>();

        let auth_layer = AuthLayer::new(jwt);
        let service = ServiceBuilder::new().layer(auth_layer).service(hyper_client);

        let layer_transport = HyperClient::with_service(service);
        let http_hyper = Http::with_client(layer_transport, url);
        let rpc_client = RpcClient::new(http_hyper, true);
        let provider = RootProvider::<_, AnyNetwork>::new(rpc_client);

        Self { provider }
    }

    /// Validates the payload using the Fork Choice Update API.
    pub async fn validate_payload_fcu(
        &self,
        attributes: &OpAttributesWithParent,
    ) -> Result<bool, ValidationError> {
        // TODO: use the correct values
        let fork_choice_state = ForkchoiceState {
            head_block_hash: attributes.parent.block_info.hash,
            finalized_block_hash: attributes.parent.block_info.hash,
            safe_block_hash: attributes.parent.block_info.hash,
        };

        let attributes = Some(attributes.attributes.clone());
        let fcu = self
            .provider
            .fork_choice_updated_v2(fork_choice_state, attributes)
            .await
            .map_err(|_| ValidationError::RpcError)?;

        if fcu.is_valid() {
            Ok(true)
        } else {
            warn!(status = %fcu.payload_status, "Engine API returned invalid fork choice update");
            Ok(false)
        }
    }
}

impl std::ops::Deref for EngineApi {
    type Target = RootProvider<Http<HyperAuthClient>, AnyNetwork>;

    fn deref(&self) -> &Self::Target {
        &self.provider
    }
}
