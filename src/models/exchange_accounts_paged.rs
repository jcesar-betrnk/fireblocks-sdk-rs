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
pub struct ExchangeAccountsPaged {
    #[serde(rename = "ExchangeAccount", skip_serializing_if = "Option::is_none")]
    pub exchange_account: Option<Vec<models::ExchangeAccount>>,
    #[serde(rename = "paging", skip_serializing_if = "Option::is_none")]
    pub paging: Option<models::ExchangeAccountsPagedPaging>,
    #[serde(rename = "prevUrl", skip_serializing_if = "Option::is_none")]
    pub prev_url: Option<String>,
    #[serde(rename = "nextUrl", skip_serializing_if = "Option::is_none")]
    pub next_url: Option<String>,
}

impl ExchangeAccountsPaged {
    pub fn new() -> ExchangeAccountsPaged {
        ExchangeAccountsPaged {
            exchange_account: None,
            paging: None,
            prev_url: None,
            next_url: None,
        }
    }
}
