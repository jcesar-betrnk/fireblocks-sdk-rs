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
pub struct TokensPaginatedResponse {
    /// The data of the current page
    #[serde(rename = "data")]
    pub data: Vec<models::TokenLinkDto>,
    /// The ID of the next page
    #[serde(
        rename = "next",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub next: Option<Option<String>>,
}

impl TokensPaginatedResponse {
    pub fn new(data: Vec<models::TokenLinkDto>) -> TokensPaginatedResponse {
        TokensPaginatedResponse { data, next: None }
    }
}
