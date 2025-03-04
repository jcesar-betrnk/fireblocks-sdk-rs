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
pub struct NetworkConnectionResponse {
    #[serde(rename = "id")]
    pub id: String,
    /// Deprecated - Replaced by `localNetworkId`
    #[serde(rename = "localChannel", skip_serializing_if = "Option::is_none")]
    pub local_channel: Option<models::NetworkChannel>,
    /// Deprecated - Replaced by `remoteNetworkId`
    #[serde(rename = "remoteChannel", skip_serializing_if = "Option::is_none")]
    pub remote_channel: Option<models::NetworkChannel>,
    #[serde(rename = "status")]
    pub status: models::NetworkConnectionStatus,
    #[serde(rename = "localNetworkId")]
    pub local_network_id: models::NetworkId,
    #[serde(rename = "remoteNetworkId")]
    pub remote_network_id: models::NetworkId,
    #[serde(rename = "routingPolicy")]
    pub routing_policy:
        std::collections::HashMap<String, models::NetworkConnectionRoutingPolicyValue>,
}

impl NetworkConnectionResponse {
    pub fn new(
        id: String,
        status: models::NetworkConnectionStatus,
        local_network_id: models::NetworkId,
        remote_network_id: models::NetworkId,
        routing_policy: std::collections::HashMap<
            String,
            models::NetworkConnectionRoutingPolicyValue,
        >,
    ) -> NetworkConnectionResponse {
        NetworkConnectionResponse {
            id,
            local_channel: None,
            remote_channel: None,
            status,
            local_network_id,
            remote_network_id,
            routing_policy,
        }
    }
}
