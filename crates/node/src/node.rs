//! Contains the core `Node` runner.

use crate::{Config, SyncMode};
use hilo_driver::{
    HiloDriver, HiloExecutorConstructor, HiloPipeline, InMemoryChainProvider,
    InMemoryL2ChainProvider, OnlineBeaconClient, OnlineBlobProvider,
    OnlineBlobProviderWithFallback,
};
use kona_driver::PipelineCursor;
use op_alloy_protocol::{BlockInfo, L2BlockInfo};
use std::sync::Arc;
use tokio::sync::watch::{channel, Receiver};

/// A high-level `Node` error.
#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    /// An error occurred during standalone initialization.
    #[error("standalone initialization failed")]
    StandaloneInit,
}

/// The core node runner.
#[derive(Debug)]
pub struct Node {
    /// The node config.
    config: Config,
    /// The [SyncMode] - currently full & checkpoint sync are supported
    sync_mode: SyncMode,
    /// The L2 block hash to begin syncing from
    checkpoint_hash: Option<String>,
    /// Receiver to listen for SIGINT signals
    shutdown_recv: Receiver<bool>,
}

impl From<Config> for Node {
    fn from(config: Config) -> Self {
        let (shutdown_sender, shutdown_recv) = channel(false);
        ctrlc::set_handler(move || {
            tracing::info!("shutting down");
            shutdown_sender.send(true).expect("could not send shutdown signal");
        })
        .expect("could not register shutdown handler");

        Self { config, sync_mode: SyncMode::Full, checkpoint_hash: None, shutdown_recv }
    }
}

impl Node {
    /// Sets the [SyncMode]
    pub fn with_sync_mode(mut self, sync_mode: SyncMode) -> Self {
        self.sync_mode = sync_mode;
        self
    }

    /// Sets the `checkpoint_hash` if running in checkpoint [SyncMode]
    pub fn with_checkpoint_hash(mut self, checkpoint_hash: Option<String>) -> Self {
        self.checkpoint_hash = checkpoint_hash;
        self
    }

    /// Begins the syncing process
    pub async fn run(self) -> Result<(), NodeError> {
        match self.sync_mode {
            SyncMode::Fast => self.fast_sync().await,
            SyncMode::Challenge => self.challenge_sync().await,
            SyncMode::Full => self.full_sync().await,
            SyncMode::Checkpoint => self.checkpoint_sync().await,
        }
    }

    /// Fast sync mode - currently unsupported
    pub async fn fast_sync(&self) -> Result<(), NodeError> {
        error!("fast sync is not implemented yet");
        unimplemented!();
    }

    /// Fast challenge mode - currently unsupported
    pub async fn challenge_sync(&self) -> Result<(), NodeError> {
        error!("challenge sync is not implemented yet");
        unimplemented!();
    }

    /// Full sync mode.
    ///
    /// Syncs via L1 block derivation from the latest finalized block
    /// the execution client has synced to.
    /// Otherwise syncs from genesis
    pub async fn full_sync(&self) -> Result<(), NodeError> {
        self.start_driver().await?;
        Ok(())
    }

    /// Checkpoint sync mode.
    ///
    /// Syncs the execution client to a given checkpoint block, and then
    /// begins the normal derivation sync process via the [HiloDriver].
    ///
    /// Note: the `admin` RPC method must be available on the execution client
    /// as checkpoint_sync relies on `admin_addPeer`
    pub async fn checkpoint_sync(&self) -> Result<(), NodeError> {
        unimplemented!();
    }

    /// Construct a blob provider from the config.
    pub fn blob_provider(
        &self,
    ) -> OnlineBlobProviderWithFallback<OnlineBeaconClient, OnlineBeaconClient> {
        let beacon_client =
            OnlineBeaconClient::new_http(String::from(self.config.l1_beacon_url.clone()));
        let genesis = Some(self.config.rollup_config.genesis.l1.number);
        // TODO: fix the slot interval here
        let blob = OnlineBlobProvider::new(beacon_client, genesis, Some(10));
        OnlineBlobProviderWithFallback::new(blob, None)
    }

    /// Creates and starts the [HiloDriver] which handles the derivation sync process.
    async fn start_driver(&self) -> Result<(), NodeError> {
        // TODO: use the proper safe head info.
        // This should be pulled in using the checkpoint hash.
        let safe_head_info = L2BlockInfo::default();
        // let l1_origin = BlockInfo::default();
        let channel_timeout =
            self.config.rollup_config.channel_timeout(safe_head_info.block_info.timestamp);
        // let mut l1_origin_number = l1_origin.number.saturating_sub(channel_timeout);
        // if l1_origin_number < self.config.rollup_config.genesis.l1.number {
        //     l1_origin_number = self.config.rollup_config.genesis.l1.number;
        // }

        // TODO: pull in the correct origin using the chain provider
        // let origin = chain_provider.block_info_by_number(l1_origin_number).await?;
        let origin = BlockInfo::default();
        let cursor = PipelineCursor::new(channel_timeout, origin);

        // TODO: pull in chain capacity from config and cli
        let chain_provider = InMemoryChainProvider::with_capacity(100);
        let l2_chain_provider = InMemoryL2ChainProvider::with_capacity(100);
        let pipeline = HiloPipeline::new(
            Arc::new(self.config.rollup_config.clone()),
            cursor.clone(),
            self.blob_provider(),
            chain_provider.clone(),
            l2_chain_provider,
        );
        let executor = HiloExecutorConstructor::new();
        let mut driver = HiloDriver::standalone(
            Arc::new(self.config.rollup_config.clone()),
            self.config.l1_rpc_url.clone(),
            cursor,
            executor,
            pipeline,
        )
        .await
        .map_err(|_| NodeError::StandaloneInit)?;

        // Run the derivation pipeline until we are able to produce the output root of the claimed
        // L2 block.
        // let (number, output_root) =
        //     driver.advance_to_target(&boot.rollup_config, boot.claimed_l2_block_number).await?;

        driver.start().await;
        Ok(())
    }

    /// Exits if a SIGINT signal is received
    #[allow(unused)]
    fn check_shutdown(&self) -> Result<(), NodeError> {
        if *self.shutdown_recv.borrow() {
            tracing::warn!("shutting down");
            std::process::exit(0);
        }

        Ok(())
    }
}
