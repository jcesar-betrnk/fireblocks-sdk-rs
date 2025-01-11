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

/// SmartTransferTicket : Data object with result data
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartTransferTicket {
    /// Unique id of Smart Transfer ticket
    #[serde(rename = "id")]
    pub id: String,
    /// Kind of Smart Transfer. Can be either `ASYNC` or `ATOMIC`
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Direction of Smart Transfer.
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    /// Current status of Smart Transfer ticket
    #[serde(rename = "status")]
    pub status: Status,
    /// Ticket terms (legs)
    #[serde(rename = "terms", skip_serializing_if = "Option::is_none")]
    pub terms: Option<Vec<models::SmartTransferTicketTerm>>,
    /// Number of hours for expiration.This data is valid only it ticket not in
    /// DRAFT state and it will be used to calculate expiresAt value
    #[serde(rename = "expiresIn", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<f64>,
    /// Date and time at which the ticket will expire if no funding is
    /// performed.
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Date and time when ticket is submitted.
    #[serde(rename = "submittedAt", skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<String>,
    /// Date and time when ticket is expired.
    #[serde(rename = "expiredAt", skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    /// Date and time when ticket is canceled.
    #[serde(rename = "canceledAt", skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    /// Date and time when ticket is fulfilled.
    #[serde(rename = "fulfilledAt", skip_serializing_if = "Option::is_none")]
    pub fulfilled_at: Option<String>,
    /// External Ref ID for Smart Transfer ticket.
    #[serde(rename = "externalRefId", skip_serializing_if = "Option::is_none")]
    pub external_ref_id: Option<String>,
    /// Note
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// ID of network profile that created ticket
    #[serde(rename = "createdByNetworkId")]
    pub created_by_network_id: String,
    /// Name of network profile that created ticket
    #[serde(rename = "createdByNetworkIdName")]
    pub created_by_network_id_name: String,
    /// Name of network profile that canceled ticket
    #[serde(
        rename = "canceledByNetworkIdName",
        skip_serializing_if = "Option::is_none"
    )]
    pub canceled_by_network_id_name: Option<String>,
    /// Date and time at which the ticket is created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Date and time of last ticket update.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "canceledByMe", skip_serializing_if = "Option::is_none")]
    pub canceled_by_me: Option<bool>,
    #[serde(rename = "createdByMe", skip_serializing_if = "Option::is_none")]
    pub created_by_me: Option<bool>,
}

impl SmartTransferTicket {
    /// Data object with result data
    pub fn new(
        id: String,
        r#type: Type,
        status: Status,
        created_by_network_id: String,
        created_by_network_id_name: String,
        created_at: String,
        updated_at: String,
    ) -> SmartTransferTicket {
        SmartTransferTicket {
            id,
            r#type,
            direction: None,
            status,
            terms: None,
            expires_in: None,
            expires_at: None,
            submitted_at: None,
            expired_at: None,
            canceled_at: None,
            fulfilled_at: None,
            external_ref_id: None,
            note: None,
            created_by_network_id,
            created_by_network_id_name,
            canceled_by_network_id_name: None,
            created_at,
            updated_at,
            canceled_by_me: None,
            created_by_me: None,
        }
    }
}
/// Kind of Smart Transfer. Can be either `ASYNC` or `ATOMIC`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ASYNC")]
    Async,
}

impl Default for Type {
    fn default() -> Type {
        Self::Async
    }
}
/// Direction of Smart Transfer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "EXCHANGE")]
    Exchange,
    #[serde(rename = "SEND")]
    Send,
    #[serde(rename = "RECEIVE")]
    Receive,
    #[serde(rename = "INTERMEDIATE")]
    Intermediate,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::Exchange
    }
}
/// Current status of Smart Transfer ticket
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "DRAFT")]
    Draft,
    #[serde(rename = "PENDING_APPROVAL")]
    PendingApproval,
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "IN_SETTLEMENT")]
    InSettlement,
    #[serde(rename = "FULFILLED")]
    Fulfilled,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "CANCELED")]
    Canceled,
}

impl Default for Status {
    fn default() -> Status {
        Self::Draft
    }
}
