// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

/// BlockInfo : The block hash and height of the block that this transaction was
/// mined in.      **Note**: If an outgoing transaction uses the destinations
/// object with more than one value in the array, blockHash is set to null.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockInfo {
    #[serde(rename = "blockHeight", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<String>,
    #[serde(
        rename = "blockHash",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_hash: Option<Option<String>>,
}

impl BlockInfo {
    /// The block hash and height of the block that this transaction was mined
    /// in.      **Note**: If an outgoing transaction uses the destinations
    /// object with more than one value in the array, blockHash is set to null.
    pub fn new() -> BlockInfo {
        BlockInfo {
            block_height: None,
            block_hash: None,
        }
    }
}
