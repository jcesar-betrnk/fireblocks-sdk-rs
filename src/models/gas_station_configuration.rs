// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GasStationConfiguration {
    /// The Gas Station will initiate a fueling transaction when the vault
    /// account's balance falls below the gasThreshold value
    #[serde(rename = "gasThreshold", skip_serializing_if = "Option::is_none")]
    pub gas_threshold: Option<String>,
    /// The Gas Station will top up the vault account with gas until the balance
    /// reaches the gasCap value
    #[serde(rename = "gasCap", skip_serializing_if = "Option::is_none")]
    pub gas_cap: Option<String>,
    /// Maximum gas price for the fueling transaction (in gwei)
    #[serde(
        rename = "maxGasPrice",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_gas_price: Option<Option<String>>,
}

impl GasStationConfiguration {
    pub fn new() -> GasStationConfiguration {
        GasStationConfiguration {
            gas_threshold: None,
            gas_cap: None,
            max_gas_price: None,
        }
    }
}
