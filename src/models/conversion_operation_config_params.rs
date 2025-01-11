// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversionOperationConfigParams {
    /// The amount to convert
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Source account ID
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The asset ID to convert
    #[serde(rename = "srcAssetId", skip_serializing_if = "Option::is_none")]
    pub src_asset_id: Option<String>,
    /// The asset ID to get as a result of the conversion
    #[serde(rename = "destAssetId")]
    pub dest_asset_id: String,
    /// Slippage tolerance (basis points)
    #[serde(
        rename = "slippageBasisPoints",
        skip_serializing_if = "Option::is_none"
    )]
    pub slippage_basis_points: Option<u32>,
}

impl ConversionOperationConfigParams {
    pub fn new(dest_asset_id: String) -> ConversionOperationConfigParams {
        ConversionOperationConfigParams {
            amount: None,
            account_id: None,
            src_asset_id: None,
            dest_asset_id,
            slippage_basis_points: None,
        }
    }
}
