// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaEntityResponse {
    /// Cached accessible URL
    #[serde(rename = "url")]
    pub url: String,
    /// Media type
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
}

impl MediaEntityResponse {
    pub fn new(url: String, content_type: ContentType) -> MediaEntityResponse {
        MediaEntityResponse { url, content_type }
    }
}
/// Media type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "VIDEO")]
    Video,
    #[serde(rename = "ANIMATION")]
    Animation,
    #[serde(rename = "THREE_D")]
    ThreeD,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "GIF")]
    Gif,
    #[serde(rename = "UNKNOWN_TYPE")]
    UnknownType,
    #[serde(rename = "SVG")]
    Svg,
    #[serde(rename = "AUDIO")]
    Audio,
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::Image
    }
}
