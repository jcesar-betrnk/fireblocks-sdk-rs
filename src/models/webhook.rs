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
pub struct Webhook {
    /// The id of the webhook
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The url of the webhook where notifications will be sent. Must be a valid
    /// URL and https.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// description of the webhook of what it is used for
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The events that the webhook will be subscribed to
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<models::WebhookEvent>>,
    /// The status of the webhook
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The date and time the webhook was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The date and time the webhook was last updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl Webhook {
    pub fn new() -> Webhook {
        Webhook {
            id: None,
            url: None,
            description: None,
            events: None,
            status: None,
            created_at: None,
            updated_at: None,
        }
    }
}
/// The status of the webhook
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "ENABLED")]
    Enabled,
}

impl Default for Status {
    fn default() -> Status {
        Self::Disabled
    }
}
