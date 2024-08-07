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

/// struct for passing parameters to the method [`change_security_level`]
#[derive(Clone, Debug)]
pub struct ChangeSecurityLevelParams {
    /// Site identifier
    pub site_id: String,
    /// Security object
    pub security_level: models::Security,
}

/// struct for passing parameters to the method [`create_site`]
#[derive(Clone, Debug)]
pub struct CreateSiteParams {
    /// Site object
    pub site: models::SiteInput,
}

/// struct for passing parameters to the method [`delete_site`]
#[derive(Clone, Debug)]
pub struct DeleteSiteParams {
    /// Site identifier
    pub site_id: String,
}

/// struct for passing parameters to the method [`device_updates_get`]
#[derive(Clone, Debug)]
pub struct DeviceUpdatesGetParams {
    /// Site identifier
    pub site_id: String,
    /// Device identifier
    pub device_id: String,
}

/// struct for passing parameters to the method [`get_site`]
#[derive(Clone, Debug)]
pub struct GetSiteParams {
    /// Site identifier
    pub site_id: String,
}

/// struct for passing parameters to the method [`get_site_lorawan_cover_test`]
#[derive(Clone, Debug)]
pub struct GetSiteLorawanCoverTestParams {
    /// Site identifier
    pub site_id: String,
}

/// struct for passing parameters to the method [`site_alarm_stop`]
#[derive(Clone, Debug)]
pub struct SiteAlarmStopParams {
    /// Site identifier
    pub site_id: String,
}

/// struct for passing parameters to the method [`site_domestic_alarm_stop`]
#[derive(Clone, Debug)]
pub struct SiteDomesticAlarmStopParams {
    /// Site identifier
    pub site_id: String,
    /// Domestic alarm identifier
    pub alarm_id: String,
}

/// struct for passing parameters to the method [`site_history`]
#[derive(Clone, Debug)]
pub struct SiteHistoryParams {
    /// Site identifier.
    pub site_id: String,
    /// ISO 8601 UTC timestamp (example: '2014-07-14T07:45:00.000Z').
    pub date_min: Option<String>,
    /// ISO 8601 UTC timestamp (example: '2014-07-14T07:45:00.000Z').
    pub date_max: Option<String>,
    /// Comma separated list of event types (example: 'securityAlarm,securityLevel').
    pub message_type: Option<String>,
    /// Set results order. '1' for ascending, '-1' for descending. Default: descending.
    pub order: Option<String>,
    /// Limit of results. Default: 50. Max: 100.
    pub limit: Option<i32>,
    /// ISO 8601 microseconds timestamp to get next page (based on 'occurred_at' value).
    pub start_after: Option<String>,
    /// ISO 8601 microseconds timestamp to get previous page (based on 'occurred_at' value).
    pub end_before: Option<String>,
}

/// struct for passing parameters to the method [`site_manual_alarm_trigger`]
#[derive(Clone, Debug)]
pub struct SiteManualAlarmTriggerParams {
    /// Site identifier
    pub site_id: String,
}

/// struct for passing parameters to the method [`site_panic_start`]
#[derive(Clone, Debug)]
pub struct SitePanicStartParams {
    /// Site identifier
    pub site_id: String,
    /// Panic mode details
    pub panic: models::Panic,
}

/// struct for passing parameters to the method [`site_set_privacy`]
#[derive(Clone, Debug)]
pub struct SiteSetPrivacyParams {
    /// Site identifier
    pub site_id: String,
    /// Privacy details
    pub privacy: models::SitePrivacy,
}

/// struct for passing parameters to the method [`update_site`]
#[derive(Clone, Debug)]
pub struct UpdateSiteParams {
    /// Site identifier
    pub site_id: String,
    /// Site object
    pub site: models::SiteInput,
}

/// struct for typed errors of method [`change_security_level`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeSecurityLevelError {
    Status404(models::ApiException),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_site`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSiteError {
    Status404(models::ApiException),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_site`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSiteError {
    Status404(models::ApiException),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`device_updates_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeviceUpdatesGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_site`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSiteError {
    Status404(models::ApiException),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_site_lorawan_cover_test`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSiteLorawanCoverTestError {
    Status404(models::ApiException),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`site_alarm_stop`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteAlarmStopError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`site_domestic_alarm_stop`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteDomesticAlarmStopError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`site_get_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteGetListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`site_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteHistoryError {
    Status404(models::ApiException),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`site_manual_alarm_trigger`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteManualAlarmTriggerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`site_panic_start`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SitePanicStartError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`site_set_privacy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteSetPrivacyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_site`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSiteError {
    Status404(models::ApiException),
    UnknownValue(serde_json::Value),
}

pub async fn change_security_level(
    configuration: &configuration::Configuration,
    params: ChangeSecurityLevelParams,
) -> Result<models::Job, Error<ChangeSecurityLevelError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let security_level = params.security_level;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/security",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id)
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
    local_var_req_builder = local_var_req_builder.json(&security_level);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChangeSecurityLevelError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_site(
    configuration: &configuration::Configuration,
    params: CreateSiteParams,
) -> Result<models::SiteOutput, Error<CreateSiteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site = params.site;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/site", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&site);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateSiteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_site(
    configuration: &configuration::Configuration,
    params: DeleteSiteParams,
) -> Result<(), Error<DeleteSiteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<DeleteSiteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn device_updates_get(
    configuration: &configuration::Configuration,
    params: DeviceUpdatesGetParams,
) -> Result<(), Error<DeviceUpdatesGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let device_id = params.device_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/device/{device_id}/update",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id),
        device_id = crate::apis::urlencode(device_id)
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
        let local_var_entity: Option<DeviceUpdatesGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_site(
    configuration: &configuration::Configuration,
    params: GetSiteParams,
) -> Result<models::SiteOutput, Error<GetSiteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}",
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSiteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_site_lorawan_cover_test(
    configuration: &configuration::Configuration,
    params: GetSiteLorawanCoverTestParams,
) -> Result<models::LoraWanCover, Error<GetSiteLorawanCoverTestError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/lorawan/covertest",
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSiteLorawanCoverTestError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn site_alarm_stop(
    configuration: &configuration::Configuration,
    params: SiteAlarmStopParams,
) -> Result<models::Job, Error<SiteAlarmStopError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/alarm/stop",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id)
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SiteAlarmStopError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn site_domestic_alarm_stop(
    configuration: &configuration::Configuration,
    params: SiteDomesticAlarmStopParams,
) -> Result<(), Error<SiteDomesticAlarmStopError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let alarm_id = params.alarm_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/domestic-alarm/{alarm_id}/stop",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id),
        alarm_id = crate::apis::urlencode(alarm_id)
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
        let local_var_entity: Option<SiteDomesticAlarmStopError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn site_get_list(
    configuration: &configuration::Configuration,
) -> Result<models::SiteCollection, Error<SiteGetListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/site", local_var_configuration.base_path);
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SiteGetListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn site_history(
    configuration: &configuration::Configuration,
    params: SiteHistoryParams,
) -> Result<models::SiteEventCollection, Error<SiteHistoryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let date_min = params.date_min;
    let date_max = params.date_max;
    let message_type = params.message_type;
    let order = params.order;
    let limit = params.limit;
    let start_after = params.start_after;
    let end_before = params.end_before;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/history",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = date_min {
        local_var_req_builder =
            local_var_req_builder.query(&[("date_min", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = date_max {
        local_var_req_builder =
            local_var_req_builder.query(&[("date_max", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = message_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("message_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder =
            local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_after {
        local_var_req_builder =
            local_var_req_builder.query(&[("start_after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_before {
        local_var_req_builder =
            local_var_req_builder.query(&[("end_before", &local_var_str.to_string())]);
    }
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SiteHistoryError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn site_manual_alarm_trigger(
    configuration: &configuration::Configuration,
    params: SiteManualAlarmTriggerParams,
) -> Result<models::Job, Error<SiteManualAlarmTriggerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/alarm/trigger_manual_alarm",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id)
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SiteManualAlarmTriggerError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn site_panic_start(
    configuration: &configuration::Configuration,
    params: SitePanicStartParams,
) -> Result<models::Job, Error<SitePanicStartError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let panic = params.panic;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/panic",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&panic);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SitePanicStartError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn site_set_privacy(
    configuration: &configuration::Configuration,
    params: SiteSetPrivacyParams,
) -> Result<models::Job, Error<SiteSetPrivacyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let privacy = params.privacy;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/privacy",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id)
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
    local_var_req_builder = local_var_req_builder.json(&privacy);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SiteSetPrivacyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_site(
    configuration: &configuration::Configuration,
    params: UpdateSiteParams,
) -> Result<(), Error<UpdateSiteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let site = params.site;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id)
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
    local_var_req_builder = local_var_req_builder.json(&site);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdateSiteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
