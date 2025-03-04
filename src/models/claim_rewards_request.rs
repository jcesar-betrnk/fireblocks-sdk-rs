// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use {
    crate::models,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClaimRewardsRequest {
    /// id of position to withdraw rewards from
    #[serde(rename = "id")]
    pub id: String,
    /// Represents the fee for a transaction, which can be specified as a
    /// percentage value. Only one of fee/feeLevel is required.
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename = "feeLevel", skip_serializing_if = "Option::is_none")]
    pub fee_level: Option<models::FeeLevel>,
    /// The note to associate with the transactions.
    #[serde(rename = "txNote", skip_serializing_if = "Option::is_none")]
    pub tx_note: Option<String>,
}

impl ClaimRewardsRequest {
    pub fn new(id: String) -> ClaimRewardsRequest {
        ClaimRewardsRequest {
            id,
            fee: None,
            fee_level: None,
            tx_note: None,
        }
    }
}
