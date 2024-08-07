/*
 * Somfy Protect API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`site_core_scenario_get`]
#[derive(Clone, Debug)]
pub struct SiteCoreScenarioGetParams {
    /// Site identifier
    pub site_id: String,
    /// Core scenario trigger
    pub trigger: String,
}

/// struct for passing parameters to the method [`site_core_scenario_get_list`]
#[derive(Clone, Debug)]
pub struct SiteCoreScenarioGetListParams {
    /// Site identifier
    pub site_id: String,
}

/// struct for passing parameters to the method [`site_core_scenario_update`]
#[derive(Clone, Debug)]
pub struct SiteCoreScenarioUpdateParams {
    /// Site identifier
    pub site_id: String,
    /// Core scenario trigger
    pub trigger: String,
}

/// struct for typed errors of method [`site_core_scenario_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteCoreScenarioGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`site_core_scenario_get_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteCoreScenarioGetListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`site_core_scenario_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteCoreScenarioUpdateError {
    UnknownValue(serde_json::Value),
}

pub async fn site_core_scenario_get(
    configuration: &configuration::Configuration,
    params: SiteCoreScenarioGetParams,
) -> Result<(), Error<SiteCoreScenarioGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let trigger = params.trigger;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/scenario-core/{trigger}",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id),
        trigger = crate::apis::urlencode(trigger)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<SiteCoreScenarioGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn site_core_scenario_get_list(
    configuration: &configuration::Configuration,
    params: SiteCoreScenarioGetListParams,
) -> Result<(), Error<SiteCoreScenarioGetListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/scenario-core",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<SiteCoreScenarioGetListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn site_core_scenario_update(
    configuration: &configuration::Configuration,
    params: SiteCoreScenarioUpdateParams,
) -> Result<(), Error<SiteCoreScenarioUpdateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let trigger = params.trigger;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/scenario-core/{trigger}",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id),
        trigger = crate::apis::urlencode(trigger)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<SiteCoreScenarioUpdateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
