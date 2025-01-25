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
pub struct Notification {
    /// The id of the Notification
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The creation date of the notification
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// The date when the notification was updated
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "status")]
    pub status: models::NotificationStatus,
    #[serde(rename = "eventType")]
    pub event_type: models::WebhookEvent,
    /// The event version of the Notification
    #[serde(rename = "eventVersion")]
    pub event_version: f64,
    /// The resource id of the event which the Notification is listen to
    #[serde(
        rename = "resourceId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_id: Option<Option<uuid::Uuid>>,
    /// The attempts related to Notification
    #[serde(rename = "attempts", skip_serializing_if = "Option::is_none")]
    pub attempts: Option<Vec<String>>,
}

impl Notification {
    pub fn new(
        id: uuid::Uuid,
        created_at: String,
        updated_at: String,
        status: models::NotificationStatus,
        event_type: models::WebhookEvent,
        event_version: f64,
    ) -> Notification {
        Notification {
            id,
            created_at,
            updated_at,
            status,
            event_type,
            event_version,
            resource_id: None,
            attempts: None,
        }
    }
}
