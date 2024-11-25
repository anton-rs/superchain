//! Contains the core `HiloDriver`.

use alloy_transport::TransportResult;
use std::sync::Arc;

use hilo_providers_local::{InMemoryChainProvider, InMemoryL2ChainProvider};

use crate::{
    Config, ConfigError, Context, HiloExecutorConstructor, HiloPipeline, StandaloneContext,
};

/// HiloDriver is a wrapper around the `Driver` that
/// provides methods of constructing the driver.
#[derive(Debug)]
pub struct HiloDriver<C: Context> {
    /// The driver context.
    pub ctx: C,
    /// The driver config.
    pub cfg: Config,
    /// A constructor for execution.
    pub exec: HiloExecutorConstructor,
}

impl HiloDriver<StandaloneContext> {
    /// Creates a new [HiloDriver] with a standalone context.
    pub async fn standalone(cfg: Config, exec: HiloExecutorConstructor) -> TransportResult<Self> {
        let ctx = StandaloneContext::new(cfg.l1_rpc_url.clone()).await?;
        Ok(Self::new(cfg, ctx, exec))
    }
}

impl<C> HiloDriver<C>
where
    C: Context,
{
    /// Constructs a new [HiloDriver].
    pub fn new(cfg: Config, ctx: C, exec: HiloExecutorConstructor) -> Self {
        Self { cfg, ctx, exec }
    }

    /// Initializes the pipeline.
    pub async fn init_pipeline(&self) -> Result<HiloPipeline, ConfigError> {
        let cursor = self.cfg.tip_cursor().await?;
        let chain_provider = InMemoryChainProvider::with_capacity(self.cfg.cache_size);
        let l2_chain_provider = InMemoryL2ChainProvider::with_capacity(self.cfg.cache_size);
        Ok(HiloPipeline::new(
            Arc::new(self.cfg.rollup_config.clone()),
            cursor.clone(),
            self.cfg.blob_provider().await?,
            chain_provider.clone(),
            l2_chain_provider,
        ))
    }

    /// Continuously run the [HiloDriver].
    pub async fn start(&mut self) -> Result<(), ConfigError> {
        // Step 1: Wait for the L2 origin block to be available
        self.wait_for_l2_genesis_l1_block().await;
        info!("L1 chain synced to the rollup genesis block");

        // Step 2: Initialize the rollup pipeline
        let _ = self.init_pipeline().await?;
        info!("Derivation pipeline initialized");

        // Step 3: Start the processing loop
        // loop {
        //     // Try to advance the pipeline until there's no more data to process
        //     if self.step(&mut pipeline).await {
        //         continue;
        //     }
        //
        //     // Handle any incoming notifications from the context
        //     if let Some(notification) = self.ctx.recv_notification().await {
        //         self.handle_notification(notification, &mut pipeline).await?;
        //     }
        // }

        Ok(())
    }

    /// Wait for the L2 genesis' corresponding L1 block to be available in the L1 chain.
    async fn wait_for_l2_genesis_l1_block(&mut self) {
        loop {
            if let Some(notification) = self.ctx.recv_notification().await {
                if let Some(new_chain) = notification.new_chain() {
                    let tip = new_chain.tip();
                    self.ctx.send_processed_tip_event(tip);

                    if tip.number >= self.cfg.rollup_config.genesis.l1.number {
                        break;
                    }
                    debug!(
                        "Chain not yet synced to rollup genesis. L1 block number: {}",
                        tip.number
                    );
                }
            }
        }
    }
}
