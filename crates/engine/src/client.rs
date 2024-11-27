//! Contains the engine api client.

use alloy_eips::eip1898::BlockNumberOrTag;
use alloy_network::AnyNetwork;
use alloy_primitives::{Bytes, B256};
use alloy_provider::{ReqwestProvider, RootProvider};
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
use async_trait::async_trait;
use http_body_util::Full;
use op_alloy_genesis::RollupConfig;
use op_alloy_protocol::{BatchValidationProvider, L2BlockInfo};
use op_alloy_provider::ext::engine::OpEngineApi;
use op_alloy_rpc_types_engine::{OpExecutionPayloadEnvelopeV3, OpPayloadAttributes};
use std::sync::Arc;
use tower::ServiceBuilder;
use url::Url;

use hilo_providers_alloy::AlloyL2ChainProvider;

use crate::{Engine, EngineApiError};

/// A Hyper HTTP client with a JWT authentication layer.
type HyperAuthClient<B = Full<Bytes>> = HyperClient<B, AuthService<Client<HttpConnector, B>>>;

/// An external engine api client
#[derive(Debug, Clone)]
pub struct EngineClient {
    /// The L2 engine provider.
    engine: RootProvider<Http<HyperAuthClient>, AnyNetwork>,
    /// The L2 chain provider.
    rpc: AlloyL2ChainProvider,
}

impl EngineClient {
    /// Creates a new [`EngineClient`] from the provided [Url] and [JwtSecret].
    pub fn new_http(engine: Url, rpc: Url, cfg: Arc<RollupConfig>, jwt: JwtSecret) -> Self {
        let hyper_client = Client::builder(TokioExecutor::new()).build_http::<Full<Bytes>>();

        let auth_layer = AuthLayer::new(jwt);
        let service = ServiceBuilder::new().layer(auth_layer).service(hyper_client);

        let layer_transport = HyperClient::with_service(service);
        let http_hyper = Http::with_client(layer_transport, engine);
        let rpc_client = RpcClient::new(http_hyper, true);
        let engine = RootProvider::<_, AnyNetwork>::new(rpc_client);

        let rpc = ReqwestProvider::new_http(rpc);
        let rpc = AlloyL2ChainProvider::new(rpc, cfg);
        Self { engine, rpc }
    }
}

#[async_trait]
impl Engine for EngineClient {
    type Error = EngineApiError;

    async fn get_payload(
        &self,
        payload_id: PayloadId,
    ) -> Result<OpExecutionPayloadEnvelopeV3, Self::Error> {
        self.engine.get_payload_v3(payload_id).await.map_err(|_| EngineApiError::PayloadError)
    }

    async fn forkchoice_update(
        &self,
        state: ForkchoiceState,
        attr: Option<OpPayloadAttributes>,
    ) -> Result<ForkchoiceUpdated, Self::Error> {
        self.engine
            .fork_choice_updated_v2(state, attr)
            .await
            .map_err(|_| EngineApiError::PayloadError)
    }

    async fn new_payload(
        &self,
        payload: ExecutionPayloadV3,
        parent_beacon_block_root: B256,
    ) -> Result<PayloadStatus, Self::Error> {
        self.engine
            .new_payload_v3(payload, parent_beacon_block_root)
            .await
            .map_err(|_| EngineApiError::PayloadError)
    }

    async fn l2_block_ref_by_label(
        &mut self,
        numtag: BlockNumberOrTag,
    ) -> Result<L2BlockInfo, Self::Error> {
        let number = match numtag {
            BlockNumberOrTag::Number(n) => n,
            BlockNumberOrTag::Latest => {
                self.rpc.latest_block_number().await.map_err(|_| EngineApiError::PayloadError)?
            }
            _ => return Err(EngineApiError::PayloadError),
        };
        self.rpc.l2_block_info_by_number(number).await.map_err(|_| EngineApiError::PayloadError)
    }
}

impl std::ops::Deref for EngineClient {
    type Target = RootProvider<Http<HyperAuthClient>, AnyNetwork>;

    fn deref(&self) -> &Self::Target {
        &self.engine
    }
}
