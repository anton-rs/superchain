//! Superchain types.

use alloc::{string::String, vec::Vec};
use alloy_primitives::Address;

use crate::ChainConfig;
use crate::HardForkConfiguration;

/// A superchain configuration.
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
pub struct Superchain {
    /// Superchain identifier, without capitalization or display changes.
    pub name: String,
    /// Superchain configuration file contents.
    pub config: SuperchainConfig,
    /// Chain IDs of chains that are part of this superchain.
    pub chains: Vec<ChainConfig>,
}

/// A superchain configuration file format
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct SuperchainConfig {
    /// Superchain name (e.g. "Mainnet")
    pub name: String,
    /// Superchain L1 anchor information
    pub l1: SuperchainL1Info,
    /// Optional addresses for the superchain-wide default protocol versions contract.
    pub protocol_versions_addr: Option<Address>,
    /// Optional address for the superchain-wide default superchain config contract.
    pub superchain_config_addr: Option<Address>,
    /// Hardfork Configuration. These values may be overridden by individual chains.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub hardfork_defaults: HardForkConfiguration,
}

/// Superchain L1 anchor information
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct SuperchainL1Info {
    /// L1 chain ID
    #[cfg_attr(feature = "serde", serde(rename = "ChainID"))]
    pub chain_id: u64,
    /// L1 chain public RPC endpoint
    #[cfg_attr(feature = "serde", serde(rename = "PublicRPC"))]
    pub public_rpc: String,
    /// L1 chain explorer RPC endpoint
    pub explorer: String,
}

/// Level of integration with the superchain.
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[repr(u8)]
pub enum SuperchainLevel {
    /// Frontier chains are chains with customizations beyond the
    /// standard OP Stack configuration and are considered "advanced".
    Frontier = 0,
    /// Standard chains don't have any customizations beyond the
    /// standard OP Stack configuration and are considered "vanilla".
    #[default]
    Standard = 1,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::address;

    fn ref_config() -> SuperchainConfig {
        SuperchainConfig {
            name: "Mainnet".to_string(),
            l1: SuperchainL1Info {
                chain_id: 1,
                public_rpc: "https://ethereum-rpc.publicnode.com".to_string(),
                explorer: "https://etherscan.io".to_string(),
            },
            protocol_versions_addr: Some(address!("8062AbC286f5e7D9428a0Ccb9AbD71e50d93b935")),
            superchain_config_addr: Some(address!("95703e0982140D16f8ebA6d158FccEde42f04a4C")),
            hardfork_defaults: HardForkConfiguration::default(),
        }
    }

    #[test]
    fn test_superchain_l1_info_serde() {
        let l1_str = r#"{
            "ChainID": 1,
            "PublicRPC": "https://ethereum-rpc.publicnode.com",
            "Explorer": "https://etherscan.io"
          }"#;
        let l1: SuperchainL1Info = serde_json::from_str(l1_str).unwrap();
        assert_eq!(
            l1,
            SuperchainL1Info {
                chain_id: 1,
                public_rpc: "https://ethereum-rpc.publicnode.com".to_string(),
                explorer: "https://etherscan.io".to_string(),
            }
        );
    }

    #[test]
    fn test_superchain_config_serde() {
        let cfg_str = r#"{
            "Name": "Mainnet",
            "L1": {
              "ChainID": 1,
              "PublicRPC": "https://ethereum-rpc.publicnode.com",
              "Explorer": "https://etherscan.io"
            },
            "ProtocolVersionsAddr": "0x8062AbC286f5e7D9428a0Ccb9AbD71e50d93b935",
            "SuperchainConfigAddr": "0x95703e0982140D16f8ebA6d158FccEde42f04a4C"
          }"#;
        let cfg: SuperchainConfig = serde_json::from_str(cfg_str).unwrap();
        assert_eq!(cfg, ref_config());
    }

    #[test]
    fn test_superchain_serde() {
        let superchain_str = r#"{
            "name": "Mainnet",
            "config": {
              "Name": "Mainnet",
              "L1": {
                "ChainID": 1,
                "PublicRPC": "https://ethereum-rpc.publicnode.com",
                "Explorer": "https://etherscan.io"
              },
              "ProtocolVersionsAddr": "0x8062AbC286f5e7D9428a0Ccb9AbD71e50d93b935",
              "SuperchainConfigAddr": "0x95703e0982140D16f8ebA6d158FccEde42f04a4C"
            },
            "chains": []
          }"#;
        let superchain: Superchain = serde_json::from_str(superchain_str).unwrap();
        assert_eq!(superchain.name, "Mainnet");
        assert_eq!(superchain.config, ref_config());
        assert!(superchain.chains.is_empty());
    }
}
