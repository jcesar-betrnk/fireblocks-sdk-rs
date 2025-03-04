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
pub struct ListOwnedCollectionsResponse {
    #[serde(rename = "paging", skip_serializing_if = "Option::is_none")]
    pub paging: Option<models::Paging>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::CollectionOwnershipResponse>>,
}

impl ListOwnedCollectionsResponse {
    pub fn new() -> ListOwnedCollectionsResponse {
        ListOwnedCollectionsResponse {
            paging: None,
            data: None,
        }
    }
}
