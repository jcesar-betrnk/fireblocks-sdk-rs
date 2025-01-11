// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use {
    super::{configuration, Error},
    crate::{apis::ResponseContent, models},
    async_trait::async_trait,
    reqwest,
    serde::{Deserialize, Serialize},
    std::sync::Arc,
};

#[async_trait]
pub trait GasStationApi: Send + Sync {
    async fn get_gas_station_by_asset_id(
        &self,
        params: GetGasStationByAssetIdParams,
    ) -> Result<models::GasStationPropertiesResponse, Error<GetGasStationByAssetIdError>>;
    async fn get_gas_station_info(
        &self,
    ) -> Result<models::GasStationPropertiesResponse, Error<GetGasStationInfoError>>;
    async fn update_gas_station_configuration(
        &self,
        params: UpdateGasStationConfigurationParams,
    ) -> Result<
        models::EditGasStationConfigurationResponse,
        Error<UpdateGasStationConfigurationError>,
    >;
    async fn update_gas_station_configuration_by_asset_id(
        &self,
        params: UpdateGasStationConfigurationByAssetIdParams,
    ) -> Result<
        models::EditGasStationConfigurationResponse,
        Error<UpdateGasStationConfigurationByAssetIdError>,
    >;
}

pub struct GasStationApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl GasStationApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

/// struct for passing parameters to the method [`get_gas_station_by_asset_id`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetGasStationByAssetIdParams {
    /// The ID of the asset
    pub asset_id: String,
}

/// struct for passing parameters to the method
/// [`update_gas_station_configuration`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct UpdateGasStationConfigurationParams {
    pub gas_station_configuration: models::GasStationConfiguration,
    /// A unique identifier for the request. If the request is sent multiple
    /// times with the same idempotency key, the server will return the same
    /// response as the first request. The idempotency key is valid for 24
    /// hours.
    pub idempotency_key: Option<String>,
}

/// struct for passing parameters to the method
/// [`update_gas_station_configuration_by_asset_id`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct UpdateGasStationConfigurationByAssetIdParams {
    /// The ID of the asset
    pub asset_id: String,
    pub gas_station_configuration: models::GasStationConfiguration,
    /// A unique identifier for the request. If the request is sent multiple
    /// times with the same idempotency key, the server will return the same
    /// response as the first request. The idempotency key is valid for 24
    /// hours.
    pub idempotency_key: Option<String>,
}

#[async_trait]
impl GasStationApi for GasStationApiClient {
    /// Returns gas station settings and balances for a requested asset.
    async fn get_gas_station_by_asset_id(
        &self,
        params: GetGasStationByAssetIdParams,
    ) -> Result<models::GasStationPropertiesResponse, Error<GetGasStationByAssetIdError>> {
        let GetGasStationByAssetIdParams { asset_id } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/gas_station/{assetId}",
            local_var_configuration.base_path,
            assetId = crate::apis::urlencode(asset_id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetGasStationByAssetIdError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns gas station settings and ETH balance.
    async fn get_gas_station_info(
        &self,
    ) -> Result<models::GasStationPropertiesResponse, Error<GetGasStationInfoError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/gas_station", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetGasStationInfoError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Configures gas station settings for ETH.
    async fn update_gas_station_configuration(
        &self,
        params: UpdateGasStationConfigurationParams,
    ) -> Result<
        models::EditGasStationConfigurationResponse,
        Error<UpdateGasStationConfigurationError>,
    > {
        let UpdateGasStationConfigurationParams {
            gas_station_configuration,
            idempotency_key,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/gas_station/configuration",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder =
                local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.json(&gas_station_configuration);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateGasStationConfigurationError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Configures gas station settings for a requested asset.
    async fn update_gas_station_configuration_by_asset_id(
        &self,
        params: UpdateGasStationConfigurationByAssetIdParams,
    ) -> Result<
        models::EditGasStationConfigurationResponse,
        Error<UpdateGasStationConfigurationByAssetIdError>,
    > {
        let UpdateGasStationConfigurationByAssetIdParams {
            asset_id,
            gas_station_configuration,
            idempotency_key,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/gas_station/configuration/{assetId}",
            local_var_configuration.base_path,
            assetId = crate::apis::urlencode(asset_id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder =
                local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.json(&gas_station_configuration);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateGasStationConfigurationByAssetIdError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }
}

/// struct for typed errors of method [`get_gas_station_by_asset_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGasStationByAssetIdError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_gas_station_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGasStationInfoError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_gas_station_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGasStationConfigurationError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method
/// [`update_gas_station_configuration_by_asset_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGasStationConfigurationByAssetIdError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}
