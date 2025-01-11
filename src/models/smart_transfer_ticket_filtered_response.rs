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
pub struct SmartTransferTicketFilteredResponse {
    /// Result message
    #[serde(rename = "message", deserialize_with = "Option::deserialize")]
    pub message: Option<String>,
    /// Unique id of next filtered result
    #[serde(rename = "after")]
    pub after: String,
    /// Result that match given query criteria
    #[serde(rename = "data")]
    pub data: Vec<models::SmartTransferTicket>,
}

impl SmartTransferTicketFilteredResponse {
    pub fn new(
        message: Option<String>,
        after: String,
        data: Vec<models::SmartTransferTicket>,
    ) -> SmartTransferTicketFilteredResponse {
        SmartTransferTicketFilteredResponse {
            message,
            after,
            data,
        }
    }
}
