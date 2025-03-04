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
pub struct CreateTokenRequestDto {
    /// The id of the blockchain the request was initiated on
    #[serde(rename = "blockchainId", skip_serializing_if = "Option::is_none")]
    pub blockchain_id: Option<String>,
    /// The base asset identifier of the blockchain you want to deploy to
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// The id of the vault account that initiated the request to issue the
    /// token
    #[serde(rename = "vaultAccountId")]
    pub vault_account_id: String,
    #[serde(rename = "createParams")]
    pub create_params: models::CreateTokenRequestDtoCreateParams,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Indicates whether the token should be created in a gasless manner,
    /// utilizing the ERC-2771 standard. When set to true, the transaction will
    /// be relayed by a designated relayer. The workspace must be configured to
    /// use Fireblocks gasless relay.
    #[serde(rename = "useGasless", skip_serializing_if = "Option::is_none")]
    pub use_gasless: Option<bool>,
    /// Max fee amount for the write function transaction. interchangeable with
    /// the 'feeLevel' field
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    /// Fee level for the write function transaction. interchangeable with the
    /// 'fee' field
    #[serde(rename = "feeLevel", skip_serializing_if = "Option::is_none")]
    pub fee_level: Option<FeeLevel>,
}

impl CreateTokenRequestDto {
    pub fn new(
        vault_account_id: String,
        create_params: models::CreateTokenRequestDtoCreateParams,
    ) -> CreateTokenRequestDto {
        CreateTokenRequestDto {
            blockchain_id: None,
            asset_id: None,
            vault_account_id,
            create_params,
            display_name: None,
            use_gasless: None,
            fee: None,
            fee_level: None,
        }
    }
}
/// Fee level for the write function transaction. interchangeable with the 'fee'
/// field
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FeeLevel {
    #[serde(rename = "LOW")]
    Low,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "HIGH")]
    High,
}

impl Default for FeeLevel {
    fn default() -> FeeLevel {
        Self::Low
    }
}
