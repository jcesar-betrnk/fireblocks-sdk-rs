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
pub struct AssetResponse {
    #[serde(rename = "legacyId")]
    pub legacy_id: String,
    #[serde(rename = "assetClass")]
    pub asset_class: AssetClass,
    #[serde(rename = "onchain")]
    pub onchain: models::AssetOnchain,
    #[serde(rename = "metadata")]
    pub metadata: models::AssetMetadata,
}

impl AssetResponse {
    pub fn new(
        legacy_id: String,
        asset_class: AssetClass,
        onchain: models::AssetOnchain,
        metadata: models::AssetMetadata,
    ) -> AssetResponse {
        AssetResponse {
            legacy_id,
            asset_class,
            onchain,
            metadata,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssetClass {
    #[serde(rename = "NATIVE")]
    Native,
    #[serde(rename = "FT")]
    Ft,
    #[serde(rename = "NFT")]
    Nft,
    #[serde(rename = "SFT")]
    Sft,
}

impl Default for AssetClass {
    fn default() -> AssetClass {
        Self::Native
    }
}
