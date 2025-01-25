// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CollectionOwnershipResponse {
    /// Fireblocks collection id
    #[serde(rename = "id")]
    pub id: String,
    /// Collection name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Collection symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Collection contract standard
    #[serde(rename = "standard", skip_serializing_if = "Option::is_none")]
    pub standard: Option<String>,
    /// Collection's blockchain
    #[serde(rename = "blockchainDescriptor")]
    pub blockchain_descriptor: BlockchainDescriptor,
    /// Collection contract standard
    #[serde(rename = "contractAddress", skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,
}

impl CollectionOwnershipResponse {
    pub fn new(
        id: String,
        blockchain_descriptor: BlockchainDescriptor,
    ) -> CollectionOwnershipResponse {
        CollectionOwnershipResponse {
            id,
            name: None,
            symbol: None,
            standard: None,
            blockchain_descriptor,
            contract_address: None,
        }
    }
}
/// Collection's blockchain
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BlockchainDescriptor {
    #[serde(rename = "ETH")]
    Eth,
    #[serde(rename = "ETH_TEST3")]
    EthTest3,
    #[serde(rename = "ETH_TEST5")]
    EthTest5,
    #[serde(rename = "ETH_TEST6")]
    EthTest6,
    #[serde(rename = "POLYGON")]
    Polygon,
    #[serde(rename = "POLYGON_TEST_MUMBAI")]
    PolygonTestMumbai,
    #[serde(rename = "AMOY_POLYGON_TEST")]
    AmoyPolygonTest,
    #[serde(rename = "XTZ")]
    Xtz,
    #[serde(rename = "XTZ_TEST")]
    XtzTest,
    #[serde(rename = "BASECHAIN_ETH")]
    BasechainEth,
    #[serde(rename = "BASECHAIN_ETH_TEST3")]
    BasechainEthTest3,
    #[serde(rename = "BASECHAIN_ETH_TEST5")]
    BasechainEthTest5,
    #[serde(rename = "ETHERLINK")]
    Etherlink,
    #[serde(rename = "ETHERLINK_TEST")]
    EtherlinkTest,
    #[serde(rename = "MANTLE")]
    Mantle,
    #[serde(rename = "MANTLE_TEST")]
    MantleTest,
    #[serde(rename = "GUN_GUNZILLA_TEST")]
    GunGunzillaTest,
    #[serde(rename = "ETH_SONEIUM")]
    EthSoneium,
    #[serde(rename = "SONEIUM_MINATO_TEST")]
    SoneiumMinatoTest,
    #[serde(rename = "IOTX_IOTEX")]
    IotxIotex,
}

impl Default for BlockchainDescriptor {
    fn default() -> BlockchainDescriptor {
        Self::Eth
    }
}
