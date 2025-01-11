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

/// AuthorizationInfo : The information about your [Transaction Authorization Policy (TAP).](https://developers.fireblocks.com/docs/capabilities#transaction-authorization-policy-tap)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationInfo {
    #[serde(
        rename = "allowOperatorAsAuthorizer",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_operator_as_authorizer: Option<bool>,
    #[serde(rename = "logic", skip_serializing_if = "Option::is_none")]
    pub logic: Option<Logic>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<models::AuthorizationGroups>>,
}

impl AuthorizationInfo {
    /// The information about your [Transaction Authorization Policy (TAP).](https://developers.fireblocks.com/docs/capabilities#transaction-authorization-policy-tap)
    pub fn new() -> AuthorizationInfo {
        AuthorizationInfo {
            allow_operator_as_authorizer: None,
            logic: None,
            groups: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Logic {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
}

impl Default for Logic {
    fn default() -> Logic {
        Self::And
    }
}
