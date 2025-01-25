// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

/// SolanaBlockchainData : Additional fields per blockchain - can be empty or
/// missing if not initialized or no additional info exists. The type depends on
/// the chainDescriptor value. For Solana (SOL), stake account address. For
/// Ethereum (ETH), an empty object is returned as no specific data is
/// available.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolanaBlockchainData {
    /// The stake account address matching the stakeAccountId.
    #[serde(rename = "stakeAccountAddress")]
    pub stake_account_address: String,
    /// The value of the change level in the BIP44 path which was used to derive
    /// the stake account address
    #[serde(rename = "stakeAccountDerivationChangeValue")]
    pub stake_account_derivation_change_value: f64,
}

impl SolanaBlockchainData {
    /// Additional fields per blockchain - can be empty or missing if not
    /// initialized or no additional info exists. The type depends on the
    /// chainDescriptor value. For Solana (SOL), stake account address. For
    /// Ethereum (ETH), an empty object is returned as no specific data is
    /// available.
    pub fn new(
        stake_account_address: String,
        stake_account_derivation_change_value: f64,
    ) -> SolanaBlockchainData {
        SolanaBlockchainData {
            stake_account_address,
            stake_account_derivation_change_value,
        }
    }
}
