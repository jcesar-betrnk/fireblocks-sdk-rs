// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartTransferUpdateTicketTerm {
    /// Asset name
    #[serde(rename = "asset")]
    pub asset: String,
    /// Amount
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Identifier of the origination Network Profile
    #[serde(rename = "fromNetworkId")]
    pub from_network_id: String,
    /// Identifier of the destination Network Profile
    #[serde(rename = "toNetworkId")]
    pub to_network_id: String,
}

impl SmartTransferUpdateTicketTerm {
    pub fn new(
        asset: String,
        amount: f64,
        from_network_id: String,
        to_network_id: String,
    ) -> SmartTransferUpdateTicketTerm {
        SmartTransferUpdateTicketTerm {
            asset,
            amount,
            from_network_id,
            to_network_id,
        }
    }
}
