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
pub struct ConversionOperationPreviewOutput {
    #[serde(rename = "amount")]
    pub amount: models::AssetAmount,
    #[serde(rename = "fee")]
    pub fee: models::AssetAmount,
    #[serde(rename = "conversionRate")]
    pub conversion_rate: String,
    #[serde(rename = "timeSeconds")]
    pub time_seconds: f64,
}

impl ConversionOperationPreviewOutput {
    pub fn new(
        amount: models::AssetAmount,
        fee: models::AssetAmount,
        conversion_rate: String,
        time_seconds: f64,
    ) -> ConversionOperationPreviewOutput {
        ConversionOperationPreviewOutput {
            amount,
            fee,
            conversion_rate,
            time_seconds,
        }
    }
}
