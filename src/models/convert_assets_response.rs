// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConvertAssetsResponse {
    /// Indicates whether the conversion was successful
    #[serde(rename = "success")]
    pub success: bool,
}

impl ConvertAssetsResponse {
    pub fn new(success: bool) -> ConvertAssetsResponse {
        ConvertAssetsResponse { success }
    }
}
