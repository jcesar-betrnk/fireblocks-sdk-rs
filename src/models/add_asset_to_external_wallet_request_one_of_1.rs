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
pub struct AddAssetToExternalWalletRequestOneOf1 {
    #[serde(rename = "additionalInfo")]
    pub additional_info: models::AddAssetToExternalWalletRequestOneOf1AdditionalInfo,
}

impl AddAssetToExternalWalletRequestOneOf1 {
    pub fn new(
        additional_info: models::AddAssetToExternalWalletRequestOneOf1AdditionalInfo,
    ) -> AddAssetToExternalWalletRequestOneOf1 {
        AddAssetToExternalWalletRequestOneOf1 { additional_info }
    }
}
