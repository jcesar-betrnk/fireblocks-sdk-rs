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

/// SmartTransferStatisticOutflow : Outflow tickets data
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartTransferStatisticOutflow {
    #[serde(rename = "coins", skip_serializing_if = "Option::is_none")]
    pub coins: Option<Vec<models::SmartTransferCoinStatistic>>,
    #[serde(rename = "ticketCount", skip_serializing_if = "Option::is_none")]
    pub ticket_count: Option<i32>,
}

impl SmartTransferStatisticOutflow {
    /// Outflow tickets data
    pub fn new() -> SmartTransferStatisticOutflow {
        SmartTransferStatisticOutflow {
            coins: None,
            ticket_count: None,
        }
    }
}
