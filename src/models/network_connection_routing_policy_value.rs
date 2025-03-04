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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkConnectionRoutingPolicyValue {
    CustomRoutingDest(models::CustomRoutingDest),
    DefaultNetworkRoutingDest(models::DefaultNetworkRoutingDest),
    NoneNetworkRoutingDest(models::NoneNetworkRoutingDest),
}

impl Default for NetworkConnectionRoutingPolicyValue {
    fn default() -> Self {
        Self::CustomRoutingDest(Default::default())
    }
}
/// No network routing logic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scheme {
    #[serde(rename = "NONE")]
    None,
}

impl Default for Scheme {
    fn default() -> Scheme {
        Self::None
    }
}
/// The account the funds are being sent to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DstType {
    #[serde(rename = "FIAT_ACCOUNT")]
    FiatAccount,
    #[serde(rename = "VAULT")]
    Vault,
    #[serde(rename = "EXCHANGE")]
    Exchange,
}

impl Default for DstType {
    fn default() -> DstType {
        Self::FiatAccount
    }
}
