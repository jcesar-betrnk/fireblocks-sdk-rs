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
pub enum TransferPeerPathType {
    #[serde(rename = "VAULT_ACCOUNT")]
    VaultAccount,
    #[serde(rename = "EXCHANGE_ACCOUNT")]
    ExchangeAccount,
    #[serde(rename = "INTERNAL_WALLET")]
    InternalWallet,
    #[serde(rename = "EXTERNAL_WALLET")]
    ExternalWallet,
    #[serde(rename = "CONTRACT")]
    Contract,
    #[serde(rename = "NETWORK_CONNECTION")]
    NetworkConnection,
    #[serde(rename = "FIAT_ACCOUNT")]
    FiatAccount,
    #[serde(rename = "COMPOUND")]
    Compound,
    #[serde(rename = "GAS_STATION")]
    GasStation,
    #[serde(rename = "ONE_TIME_ADDRESS")]
    OneTimeAddress,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "END_USER_WALLET")]
    EndUserWallet,
    #[serde(rename = "PROGRAM_CALL")]
    ProgramCall,
}

impl std::fmt::Display for TransferPeerPathType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::VaultAccount => write!(f, "VAULT_ACCOUNT"),
            Self::ExchangeAccount => write!(f, "EXCHANGE_ACCOUNT"),
            Self::InternalWallet => write!(f, "INTERNAL_WALLET"),
            Self::ExternalWallet => write!(f, "EXTERNAL_WALLET"),
            Self::Contract => write!(f, "CONTRACT"),
            Self::NetworkConnection => write!(f, "NETWORK_CONNECTION"),
            Self::FiatAccount => write!(f, "FIAT_ACCOUNT"),
            Self::Compound => write!(f, "COMPOUND"),
            Self::GasStation => write!(f, "GAS_STATION"),
            Self::OneTimeAddress => write!(f, "ONE_TIME_ADDRESS"),
            Self::Unknown => write!(f, "UNKNOWN"),
            Self::EndUserWallet => write!(f, "END_USER_WALLET"),
            Self::ProgramCall => write!(f, "PROGRAM_CALL"),
        }
    }
}

impl Default for TransferPeerPathType {
    fn default() -> TransferPeerPathType {
        Self::VaultAccount
    }
}
