//! Contains the engine api client.

use async_trait::async_trait;
use http_body_util::Full;
use tower::ServiceBuilder;
use url::Url;

use kona_driver::Executor;

use alloy_consensus::Header;
use alloy_eips::eip1898::BlockNumberOrTag;
use alloy_network::AnyNetwork;
use alloy_primitives::{Bytes, B256};
use alloy_provider::RootProvider;
use alloy_rpc_client::RpcClient;
use alloy_rpc_types_engine::{
    ExecutionPayloadV3, ForkchoiceState, ForkchoiceUpdated, JwtSecret, PayloadId, PayloadStatus,
};
use alloy_transport_http::{
    hyper_util::{
        client::legacy::{connect::HttpConnector, Client},
        rt::TokioExecutor,
    },
    AuthLayer, AuthService, Http, HyperClient,
};

use op_alloy_protocol::L2BlockInfo;
use op_alloy_provider::ext::engine::OpEngineApi;
use op_alloy_rpc_types_engine::{OpExecutionPayloadEnvelopeV3, OpPayloadAttributes};

use crate::{Engine, EngineApiError};

/// A Hyper HTTP client with a JWT authentication layer.
type HyperAuthClient<B = Full<Bytes>> = HyperClient<B, AuthService<Client<HttpConnector, B>>>;

/// An external engine api client
#[derive(Debug, Clone)]
pub struct EngineClient {
    /// The inner provider
    provider: RootProvider<Http<HyperAuthClient>, AnyNetwork>,
}

impl EngineClient {
    /// Creates a new [`EngineClient`] from the provided [Url] and [JwtSecret].
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
}

impl Executor for EngineClient {
    type Error = EngineApiError;

    /// Execute the given payload attributes.
    fn execute_payload(&mut self, _: OpPayloadAttributes) -> Result<&Header, Self::Error> {
        todo!()
    }

    /// Computes the output root.
    fn compute_output_root(&mut self) -> Result<B256, Self::Error> {
        todo!()
    }
}

#[async_trait]
impl Engine for EngineClient {
    type Error = EngineApiError;

    async fn get_payload(
        &self,
        payload_id: PayloadId,
    ) -> Result<OpExecutionPayloadEnvelopeV3, Self::Error> {
        self.provider.get_payload_v3(payload_id).await.map_err(|_| EngineApiError::PayloadError)
    }

    async fn forkchoice_update(
        &self,
        state: ForkchoiceState,
        attr: OpPayloadAttributes,
    ) -> Result<ForkchoiceUpdated, Self::Error> {
        self.provider
            .fork_choice_updated_v2(state, Some(attr))
            .await
            .map_err(|_| EngineApiError::PayloadError)
    }

    async fn new_payload(
        &self,
        payload: ExecutionPayloadV3,
        parent_beacon_block_root: B256,
    ) -> Result<PayloadStatus, Self::Error> {
        self.provider
            .new_payload_v3(payload, parent_beacon_block_root)
            .await
            .map_err(|_| EngineApiError::PayloadError)
    }

    async fn l2_block_ref_by_label(&self, _: BlockNumberOrTag) -> Result<L2BlockInfo, Self::Error> {
        // Convert the payload into an L2 block info.
        // go impl uses an L2 client and fetches block by number, converting block to payload and
        // payload to L2 block info.
        todo!("implement l2_block_ref_by_label for the engine client")
    }
}

impl std::ops::Deref for EngineClient {
    type Target = RootProvider<Http<HyperAuthClient>, AnyNetwork>;

    fn deref(&self) -> &Self::Target {
        &self.provider
    }
}
