//! Block Types

use alloy_primitives::{B256, U64};

/// Block identifier.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BlockID {
    /// Block hash
    pub hash: B256,
    /// Block number
    pub number: U64,
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BlockID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut value = serde_json::Value::deserialize(deserializer)?;

        if let Some(obj) = value.as_object_mut() {
            let value = obj
                .get("number")
                .ok_or_else(|| serde::de::Error::custom("Missing key \"number\""))?;
            if let serde_json::Value::Number(n) = value {
                let n = n.as_u64().ok_or_else(|| {
                    serde::de::Error::custom("failed to deserialize number as u64")
                })?;
                let n = serde_json::to_string(&U64::from(n)).map_err(|_| {
                    serde::de::Error::custom(
                        "failed to serialize U64 to hex string during deserialization",
                    )
                })?;
                obj.insert("number".to_string(), serde_json::Value::String(n));
            }
        }

        if let Some(obj) = value.as_object() {
            let number = obj
                .get("number")
                .ok_or_else(|| serde::de::Error::custom("Missing key \"number\""))?;
            let serde_json::Value::String(s) = number else {
                return Err(serde::de::Error::custom("invalid type for key \"number\""));
            };
            let s = s.replace("\"", "");
            let number = match s.starts_with("0x") {
                true => {
                    let s = s.strip_prefix("0x").ok_or_else(|| {
                        serde::de::Error::custom("failed to deserialize number string")
                    })?;
                    U64::from_str_radix(s, 16).map_err(|_| {
                        serde::de::Error::custom("failed to deserialize number string")
                    })?
                }
                false => U64::from_str_radix(&s, 10)
                    .map_err(|_| serde::de::Error::custom("failed to deserialize number string"))?,
            };
            let hash = obj
                .get("hash")
                .ok_or_else(|| serde::de::Error::custom("Missing key \"hash\""))?;
            let serde_json::Value::String(s) = hash else {
                return Err(serde::de::Error::custom("invalid type for key \"hash\""));
            };
            use core::str::FromStr;
            let hash = match s.starts_with("0x") {
                true => B256::from_str(s)
                    .map_err(|_| serde::de::Error::custom("failed to deserialize number string"))?,
                false => B256::from_str(s)
                    .map_err(|_| serde::de::Error::custom("failed to deserialize number string"))?,
            };
            return Ok(BlockID { hash, number });
        }

        Err(serde::de::Error::custom("failed to deserialized BlockID"))
    }
}

impl BlockID {
    /// Instantiates a new [BlockID].
    pub const fn new(hash: B256, number: U64) -> BlockID {
        Self { hash, number }
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod tests {
    use super::*;
    use alloy_primitives::b256;

    #[test]
    fn block_id_toml() {
        let b1 = BlockID::new(
            b256!("438335a20d98863a4c0c97999eb2481921ccd28553eac6f913af7c12aec04108"),
            U64::from(17422590),
        );
        let b2 = toml::from_str("hash = \"0x438335a20d98863a4c0c97999eb2481921ccd28553eac6f913af7c12aec04108\"\nnumber = 17422590\n").unwrap();
        assert_eq!(b1, b2);
    }

    #[test]
    fn block_id_serde() {
        let block_id = BlockID {
            hash: B256::from([1; 32]),
            number: U64::from(1),
        };

        let block_id2: BlockID = serde_json::from_str(r#"{"hash":"0x0101010101010101010101010101010101010101010101010101010101010101","number":1}"#).unwrap();
        assert_eq!(block_id, block_id2);
    }

    #[test]
    fn test_block_id_serde_with_hex() {
        let block_id = BlockID {
            hash: B256::from([1; 32]),
            number: U64::from(1),
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
