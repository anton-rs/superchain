//! Contains the core `HiloDriver`.
#![allow(unused)]

use crate::{
    Context, HiloDerivationPipeline, HiloExecutor, HiloExecutorConstructor, HiloPipeline,
    StandaloneContext,
};
use alloy_transport::TransportResult;
use kona_driver::{Driver, PipelineCursor};
use op_alloy_genesis::RollupConfig;
use std::sync::Arc;
use url::Url;

/// HiloDriver is a wrapper around the `Driver` that
/// provides methods of constructing the driver.
#[derive(Debug)]
pub struct HiloDriver<C: Context> {
    /// The driver context.
    pub ctx: C,
    /// The rollup config.
    cfg: Arc<RollupConfig>,
    /// The driver instance.
    pub driver: Driver<HiloExecutor, HiloExecutorConstructor, HiloPipeline, HiloDerivationPipeline>,
}

impl HiloDriver<StandaloneContext> {
    /// Creates a new [HiloDriver] with a standalone context.
    pub async fn standalone(
        cfg: Arc<RollupConfig>,
        l1_rpc_url: Url,
        cursor: PipelineCursor,
        exec: HiloExecutorConstructor,
        pipeline: HiloPipeline,
    ) -> TransportResult<Self> {
        let ctx = StandaloneContext::new(l1_rpc_url).await?;
        Ok(Self::new(cfg, ctx, cursor, exec, pipeline))
    }
}

impl<C> HiloDriver<C>
where
    C: Context,
{
    /// Constructs a new [HiloDriver].
    pub fn new(
        cfg: Arc<RollupConfig>,
        ctx: C,
        cursor: PipelineCursor,
        exec: HiloExecutorConstructor,
        pipeline: HiloPipeline,
    ) -> Self {
        Self { cfg, ctx, driver: Driver::new(cursor, exec, pipeline) }
    }

    /// Continuously run the [Driver].
    pub async fn start(&mut self) {
        todo!("upstream implementation of a continuous driver func")
    }

    /// Wait for the L2 genesis' corresponding L1 block to be available in the L1 chain.
    async fn wait_for_l2_genesis_l1_block(&mut self) {
        loop {
            if let Some(notification) = self.ctx.recv_notification().await {
                if let Some(new_chain) = notification.new_chain() {
                    let tip = new_chain.tip();
                    self.ctx.send_processed_tip_event(tip);

                    if tip.number >= self.cfg.genesis.l1.number {
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
