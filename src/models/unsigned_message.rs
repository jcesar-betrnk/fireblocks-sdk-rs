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
pub struct UnsignedMessage {
    #[serde(rename = "preHash", skip_serializing_if = "Option::is_none")]
    pub pre_hash: Option<models::PreHash>,
    /// Content to sign on.  Should be 32 bytes long for ECDSA (hash of the
    /// actual message to sign) or any length for EdDSA as prehashing is not
    /// required.
    #[serde(rename = "content")]
    pub content: serde_json::Value,
    // String,
    /// BIP44 address index
    #[serde(rename = "bip44addressIndex", skip_serializing_if = "Option::is_none")]
    pub bip44address_index: Option<i32>,
    /// BIP44 change index
    #[serde(rename = "bip44change", skip_serializing_if = "Option::is_none")]
    pub bip44change: Option<f64>,
    /// BIP44 full derivation path
    #[serde(rename = "derivationPath", skip_serializing_if = "Option::is_none")]
    pub derivation_path: Option<Vec<f64>>,
    /// Typed Message Signing - message type.  - EIP191 & EIP712: for ETH and
    /// all EVM based assets typed message signing - TIP191: For Tron (TRX)
    /// typed message signing - BTC_MESSAGE: For Bitcoin (BTC) typed message
    /// signing
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl UnsignedMessage {
    pub fn new(content: String) -> UnsignedMessage {
        let content: serde_json::Value = serde_json::to_value(content)
            .expect("failed to convert unsigend message to serde_json::Value");
        UnsignedMessage {
            pre_hash: None,
            content,
            bip44address_index: None,
            bip44change: None,
            derivation_path: None,
            r#type: None,
        }
    }
}
/// Typed Message Signing - message type.  - EIP191 & EIP712: for ETH and all
/// EVM based assets typed message signing - TIP191: For Tron (TRX) typed
/// message signing - BTC_MESSAGE: For Bitcoin (BTC) typed message signing
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "EIP191")]
    Eip191,
    #[serde(rename = "EIP712")]
    Eip712,
    #[serde(rename = "TIP191")]
    Tip191,
    #[serde(rename = "BTC_MESSAGE")]
    BtcMessage,
    #[serde(rename = "SOLANA_MESSAGE")]
    SolanaMessage,
    #[serde(rename = "ETH_MESSAGE")]
    EthMessage,
}

impl Default for Type {
    fn default() -> Type {
        Self::Eip191
    }
}
