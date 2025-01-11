// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkConnectionStatus {
    #[serde(rename = "WAITING_FOR_APPROVAL")]
    WaitingForApproval,
    #[serde(rename = "WAITING_FOR_PEER_APPROVAL")]
    WaitingForPeerApproval,
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "REJECTED_BY_PEER")]
    RejectedByPeer,
    #[serde(rename = "REMOVED")]
    Removed,
    #[serde(rename = "REMOVED_BY_PEER")]
    RemovedByPeer,
}

impl std::fmt::Display for NetworkConnectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::WaitingForApproval => write!(f, "WAITING_FOR_APPROVAL"),
            Self::WaitingForPeerApproval => write!(f, "WAITING_FOR_PEER_APPROVAL"),
            Self::Approved => write!(f, "APPROVED"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::Failed => write!(f, "FAILED"),
            Self::Rejected => write!(f, "REJECTED"),
            Self::RejectedByPeer => write!(f, "REJECTED_BY_PEER"),
            Self::Removed => write!(f, "REMOVED"),
            Self::RemovedByPeer => write!(f, "REMOVED_BY_PEER"),
        }
    }
}

impl Default for NetworkConnectionStatus {
    fn default() -> NetworkConnectionStatus {
        Self::WaitingForApproval
    }
}
