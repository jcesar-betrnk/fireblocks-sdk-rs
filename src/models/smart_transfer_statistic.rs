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

/// SmartTransferStatistic : Smart transfers statistic
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartTransferStatistic {
    #[serde(rename = "inflow")]
    pub inflow: models::SmartTransferStatisticInflow,
    #[serde(rename = "outflow")]
    pub outflow: models::SmartTransferStatisticOutflow,
    /// Number of total active tickets
    #[serde(rename = "totalActiveTickets")]
    pub total_active_tickets: i32,
    /// Number of total inactive tickets (expired, canceled, completed)
    #[serde(rename = "totalInactiveTickets")]
    pub total_inactive_tickets: i32,
}

impl SmartTransferStatistic {
    /// Smart transfers statistic
    pub fn new(
        inflow: models::SmartTransferStatisticInflow,
        outflow: models::SmartTransferStatisticOutflow,
        total_active_tickets: i32,
        total_inactive_tickets: i32,
    ) -> SmartTransferStatistic {
        SmartTransferStatistic {
            inflow,
            outflow,
            total_active_tickets,
            total_inactive_tickets,
        }
    }
}
