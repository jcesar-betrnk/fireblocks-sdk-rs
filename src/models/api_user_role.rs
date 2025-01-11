// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

/// ApiUserRole : The role of the API Key
/// The role of the API Key
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApiUserRole {
    #[serde(rename = "ADMIN")]
    Admin,
    #[serde(rename = "SIGNER")]
    Signer,
    #[serde(rename = "COLLATERALS_SIGNER")]
    CollateralsSigner,
    #[serde(rename = "EDITOR")]
    Editor,
    #[serde(rename = "APPROVER")]
    Approver,
    #[serde(rename = "VIEWER")]
    Viewer,
    #[serde(rename = "NON_SIGNING_ADMIN")]
    NonSigningAdmin,
    #[serde(rename = "AUDITOR")]
    Auditor,
    #[serde(rename = "NCW_ADMIN")]
    NcwAdmin,
    #[serde(rename = "NCW_SIGNER")]
    NcwSigner,
}

impl std::fmt::Display for ApiUserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Admin => write!(f, "ADMIN"),
            Self::Signer => write!(f, "SIGNER"),
            Self::CollateralsSigner => write!(f, "COLLATERALS_SIGNER"),
            Self::Editor => write!(f, "EDITOR"),
            Self::Approver => write!(f, "APPROVER"),
            Self::Viewer => write!(f, "VIEWER"),
            Self::NonSigningAdmin => write!(f, "NON_SIGNING_ADMIN"),
            Self::Auditor => write!(f, "AUDITOR"),
            Self::NcwAdmin => write!(f, "NCW_ADMIN"),
            Self::NcwSigner => write!(f, "NCW_SIGNER"),
        }
    }
}

impl Default for ApiUserRole {
    fn default() -> ApiUserRole {
        Self::Admin
    }
}
