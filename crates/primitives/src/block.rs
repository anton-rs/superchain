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
}
