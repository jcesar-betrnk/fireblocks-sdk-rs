// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAssetsBulkRequest {
    /// The ID of the new asset
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// The smallest vault account ID in the range
    #[serde(rename = "vaultAccountIdFrom", skip_serializing_if = "Option::is_none")]
    pub vault_account_id_from: Option<String>,
    /// The largest vault account ID in the range
    #[serde(rename = "vaultAccountIdTo", skip_serializing_if = "Option::is_none")]
    pub vault_account_id_to: Option<String>,
}

impl CreateAssetsBulkRequest {
    pub fn new() -> CreateAssetsBulkRequest {
        CreateAssetsBulkRequest {
            asset_id: None,
            vault_account_id_from: None,
            vault_account_id_to: None,
        }
    }
}
