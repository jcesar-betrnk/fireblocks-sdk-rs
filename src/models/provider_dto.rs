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
pub struct ProviderDto {
    /// The ID of the provider
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the provider
    #[serde(rename = "providerName")]
    pub provider_name: String,
    /// An array of objects that includes chain descriptors and the
    /// corresponding fee percentages for validators supported by the provider
    #[serde(rename = "validators")]
    pub validators: Vec<models::ValidatorDto>,
    /// URL to the validator's icon
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    /// URL to the terms of service
    #[serde(rename = "termsOfServiceUrl")]
    pub terms_of_service_url: String,
    /// Indicates whether the terms of service are approved
    #[serde(rename = "isTermsOfServiceApproved")]
    pub is_terms_of_service_approved: bool,
}

impl ProviderDto {
    pub fn new(
        id: String,
        provider_name: String,
        validators: Vec<models::ValidatorDto>,
        icon_url: String,
        terms_of_service_url: String,
        is_terms_of_service_approved: bool,
    ) -> ProviderDto {
        ProviderDto {
            id,
            provider_name,
            validators,
            icon_url,
            terms_of_service_url,
            is_terms_of_service_approved,
        }
    }
}
