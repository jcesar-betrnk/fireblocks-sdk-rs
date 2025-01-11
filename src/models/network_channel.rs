// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

/// NetworkChannel : Deprecated in the only used reference -
/// NetworkConnectionResponse
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkChannel {
    #[serde(rename = "networkId", skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl NetworkChannel {
    /// Deprecated in the only used reference - NetworkConnectionResponse
    pub fn new() -> NetworkChannel {
        NetworkChannel {
            network_id: None,
            name: None,
        }
    }
}
