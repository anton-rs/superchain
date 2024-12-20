//! Contains the core `Node` runner.

use crate::{Config, NodeError, SyncMode};
use hilo_driver::HiloDriver;

/// The core node runner.
#[derive(Debug)]
pub struct Node {
    /// The node config.
    config: Config,
    /// The [SyncMode] - currently full & checkpoint sync are supported
    sync_mode: SyncMode,
    /// The L2 block hash to begin syncing from
    checkpoint_hash: Option<String>,
}

impl From<Config> for Node {
    fn from(config: Config) -> Self {
        Self { config, sync_mode: SyncMode::Full, checkpoint_hash: None }
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

    /// Creates and starts the [HiloDriver] which handles the derivation sync process.
    async fn start_driver(&self) -> Result<(), NodeError> {
        let cfg = self.config.clone().into();
        let mut driver = HiloDriver::standalone(cfg).await?;
        driver.start().await?;
        Ok(())
    }
}
