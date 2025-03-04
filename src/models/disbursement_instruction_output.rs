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
pub struct DisbursementInstructionOutput {
    #[serde(rename = "amount")]
    pub amount: models::AssetAmount,
    #[serde(rename = "fee")]
    pub fee: models::AssetAmount,
    #[serde(rename = "payeeAccount")]
    pub payee_account: models::Destination,
}

impl DisbursementInstructionOutput {
    pub fn new(
        amount: models::AssetAmount,
        fee: models::AssetAmount,
        payee_account: models::Destination,
    ) -> DisbursementInstructionOutput {
        DisbursementInstructionOutput {
            amount,
            fee,
            payee_account,
        }
    }
}
