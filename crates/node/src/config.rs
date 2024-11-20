//! Contains the configuration for the hilo-node.

use crate::SyncMode;
use alloy_rpc_types_engine::JwtSecret;
use op_alloy_genesis::RollupConfig;
use serde::{Deserialize, Serialize};
use url::Url;

/// The global node configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// The L2 Chain ID.
    pub l2_chain_id: u64,
    /// The L1 chain RPC URL
    pub l1_rpc_url: Url,
    /// The base chain beacon client RPC URL
    pub l1_beacon_url: Url,
    /// The L2 chain RPC URL
    pub l2_rpc_url: Url,
    /// The L2 engine API URL
    pub l2_engine_url: Url,
    /// The rollup config
    pub rollup_config: RollupConfig,
    /// Engine API JWT Secret.
    /// This is used to authenticate with the engine API
    #[serde(deserialize_with = "deserialize_jwt_secret")]
    pub jwt_secret: JwtSecret,
    /// A trusted L2 RPC URL to use for fast/checkpoint syncing
    pub checkpoint_sync_url: Option<Url>,
    /// The hilo-node RPC server
    pub rpc_url: Option<Url>,
    /// The devnet mode.
    /// If devnet is enabled.
    pub devnet: bool,
    /// The mode to sync.
    pub sync_mode: SyncMode,
}

impl Serialize for Config {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Config", 9)?;
        state.serialize_field("l1_rpc_url", &self.l1_rpc_url)?;
        state.serialize_field("l1_beacon_url", &self.l1_beacon_url)?;
        state.serialize_field("l2_rpc_url", &self.l2_rpc_url)?;
        state.serialize_field("l2_engine_url", &self.l2_engine_url)?;
        state.serialize_field("rollup_config", &self.rollup_config)?;
        state.serialize_field(
            "jwt_secret",
            &alloy_primitives::hex::encode(self.jwt_secret.as_bytes()),
        )?;
        state.serialize_field("checkpoint_sync_url", &self.checkpoint_sync_url)?;
        state.serialize_field("rpc_url", &self.rpc_url)?;
        state.serialize_field("devnet", &self.devnet)?;
        state.end()
    }
}

fn deserialize_jwt_secret<'de, D>(deserializer: D) -> Result<JwtSecret, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: &str = serde::de::Deserialize::deserialize(deserializer)?;
    JwtSecret::from_hex(s).map_err(serde::de::Error::custom)
}
