//! Genesis types.

use crate::BlockID;
use crate::SystemConfig;
use alloy_primitives::Bytes;

/// Chain genesis information.
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
pub struct ChainGenesis {
    /// L1 genesis block
    #[cfg_attr(feature = "serde", serde(rename = "L1"))]
    pub l1: BlockID,
    /// L2 genesis block
    #[cfg_attr(feature = "serde", serde(rename = "L2"))]
    pub l2: BlockID,
    /// Timestamp of the L2 genesis block
    pub l2_time: u64,
    /// Extra data for the genesis block
    #[cfg_attr(feature = "serde", serde(rename = "ExtraData"))]
    pub extra_data: Option<Bytes>,
    /// Optional System configuration
    pub system_config: Option<SystemConfig>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, b256, uint};

    fn ref_genesis() -> ChainGenesis {
        ChainGenesis {
            l1: BlockID {
                hash: b256!("438335a20d98863a4c0c97999eb2481921ccd28553eac6f913af7c12aec04108"),
                number: 17422590,
            },
            l2: BlockID {
                hash: b256!("dbf6a80fef073de06add9b0d14026d6e5a86c85f6d102c36d3d8e9cf89c2afd3"),
                number: 105235063,
            },
            l2_time: 1686068903,
            extra_data: None,
            system_config: Some(SystemConfig {
                batcher_address: address!("6887246668a3b87F54DeB3b94Ba47a6f63F32985"),
                overhead: uint!(0xbc_U256),
                scalar: uint!(0xa6fe0_U256),
                gas_limit: 30000000,
                base_fee_scalar: None,
                blob_base_fee_scalar: None,
            }),
        }
    }

    #[test]
    fn test_genesis_serde() {
        let genesis_str = r#"{
            "L1": {
              "Hash": "0x438335a20d98863a4c0c97999eb2481921ccd28553eac6f913af7c12aec04108",
              "Number": 17422590
            },
            "L2": {
              "Hash": "0xdbf6a80fef073de06add9b0d14026d6e5a86c85f6d102c36d3d8e9cf89c2afd3",
              "Number": 105235063
            },
            "l2_time": 1686068903,
            "ExtraData": null,
            "system_config": {
              "batcherAddr": "0x6887246668a3b87F54DeB3b94Ba47a6f63F32985",
              "overhead": "0x00000000000000000000000000000000000000000000000000000000000000bc",
              "scalar": "0x00000000000000000000000000000000000000000000000000000000000a6fe0",
              "gasLimit": 30000000
            }
          }"#;
        let genesis: ChainGenesis = serde_json::from_str(genesis_str).unwrap();
        assert_eq!(genesis, ref_genesis());
    }
}
