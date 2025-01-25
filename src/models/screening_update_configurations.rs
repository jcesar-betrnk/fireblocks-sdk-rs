// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreeningUpdateConfigurations {
    /// Flag to enable or disable bypass screening on workspace configuration.
    #[serde(rename = "disableBypass", skip_serializing_if = "Option::is_none")]
    pub disable_bypass: Option<bool>,
    /// Flag to enable or disable unfreeze of transaction frozen by policy rule
    /// on workspace configuration.
    #[serde(rename = "disableUnfreeze", skip_serializing_if = "Option::is_none")]
    pub disable_unfreeze: Option<bool>,
}

impl ScreeningUpdateConfigurations {
    pub fn new() -> ScreeningUpdateConfigurations {
        ScreeningUpdateConfigurations {
            disable_bypass: None,
            disable_unfreeze: None,
        }
    }
}
