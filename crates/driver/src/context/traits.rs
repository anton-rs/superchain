//! Context traits.

use crate::context::ChainNotification;
use alloy_eips::BlockNumHash;
use async_trait::async_trait;

/// Context for the driver.
///
/// The context is responsible for handling notifications from the state of the
/// canonical chain updates (new blocks, reorgs, etc) and translating them into
/// events that the rollup driver can use to make progress.
#[async_trait]
pub trait Context {
    /// Receives a notification from the execution client.
    async fn recv_notification(&mut self) -> Option<ChainNotification>;

    /// Sends an event indicating that the processed tip has been updated.
    fn send_processed_tip_event(&mut self, tip: BlockNumHash);
}
