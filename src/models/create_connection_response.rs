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
pub struct CreateConnectionResponse {
    /// The ID of the Web3 connection initiated.
    #[serde(rename = "id")]
    pub id: String,
    /// Metadata of the Web3 connection (provided by the dApp).
    #[serde(rename = "sessionMetadata")]
    pub session_metadata: models::SessionMetadata,
}

impl CreateConnectionResponse {
    pub fn new(id: String, session_metadata: models::SessionMetadata) -> CreateConnectionResponse {
        CreateConnectionResponse {
            id,
            session_metadata,
        }
    }
}
