// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Players {
    /// id of the player
    #[serde(rename = "id")]
    pub id: String,
    /// Type of the player
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl Players {
    pub fn new(id: String, r#type: Type) -> Players {
        Players { id, r#type }
    }
}
/// Type of the player
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "MOBILE")]
    Mobile,
    #[serde(rename = "CLOUD")]
    Cloud,
}

impl Default for Type {
    fn default() -> Type {
        Self::Mobile
    }
}
