//! Block Types

use alloy_primitives::B256;
use core::fmt::Display;

/// Block identifier.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
pub struct BlockID {
    /// Block hash
    pub hash: B256,
    /// Block number
    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity"))]
    pub number: u64,
}

impl Display for BlockID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "BlockID {{ hash: {}, number: {} }}",
            self.hash, self.number
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_id_display() {
        let block_id = BlockID {
            hash: B256::from([0; 32]),
            number: 0,
        };
        let expected = "BlockID { hash: 0x0000000000000000000000000000000000000000000000000000000000000000, number: 0 }";
        assert_eq!(block_id.to_string(), expected);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn block_id_serde() {
        let block_id = BlockID {
            hash: B256::from([1; 32]),
            number: 1,
        };

        let block_id2: BlockID = serde_json::from_str(r#"{"hash":"0x0101010101010101010101010101010101010101010101010101010101010101","number":1}"#).unwrap();
        assert_eq!(block_id, block_id2);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_block_id_serde_with_hex() {
        let block_id = BlockID {
            hash: B256::from([1; 32]),
            number: 1,
        };

        let json = serde_json::to_string(&block_id).unwrap();
        assert_eq!(
            json,
            r#"{"hash":"0x0101010101010101010101010101010101010101010101010101010101010101","number":"0x1"}"#
        );

        let block_id2: BlockID = serde_json::from_str(&json).unwrap();
        assert_eq!(block_id, block_id2);
    }
}
