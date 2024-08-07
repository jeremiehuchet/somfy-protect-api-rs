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

/// struct for passing parameters to the method [`get_dictionary`]
#[derive(Clone, Debug)]
pub struct GetDictionaryParams {
    /// Dictionary identifier
    pub dictionary_id: String,
    /// Dictionary locale
    pub locale: String,
}

/// struct for passing parameters to the method [`get_user_job`]
#[derive(Clone, Debug)]
pub struct GetUserJobParams {
    /// Job uniquer identifier
    pub job_id: String,
}

/// struct for passing parameters to the method [`user_feedback_create`]
#[derive(Clone, Debug)]
pub struct UserFeedbackCreateParams {
    /// User identifier
    pub user_id: String,
    /// Feedback
    pub feedback: models::Feedback,
}

/// struct for typed errors of method [`get_cloud_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCloudInfoError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dictionary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDictionaryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserJobError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_feedback_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserFeedbackCreateError {
    UnknownValue(serde_json::Value),
}

pub async fn get_cloud_info(
    configuration: &configuration::Configuration,
) -> Result<(), Error<GetCloudInfoError>> {
    let local_var_configuration = configuration;

    // unbox the parameters

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/info", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetCloudInfoError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_dictionary(
    configuration: &configuration::Configuration,
    params: GetDictionaryParams,
) -> Result<(), Error<GetDictionaryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let dictionary_id = params.dictionary_id;
    let locale = params.locale;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/dictionary/{dictionary_id}/{locale}",
        local_var_configuration.base_path,
        dictionary_id = crate::apis::urlencode(dictionary_id),
        locale = crate::apis::urlencode(locale)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetDictionaryError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_user_job(
    configuration: &configuration::Configuration,
    params: GetUserJobParams,
) -> Result<models::Job, Error<GetUserJobError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let job_id = params.job_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/job/{job_id}",
        local_var_configuration.base_path,
        job_id = crate::apis::urlencode(job_id)
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
        let local_var_entity: Option<GetUserJobError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn user_feedback_create(
    configuration: &configuration::Configuration,
    params: UserFeedbackCreateParams,
) -> Result<(), Error<UserFeedbackCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let user_id = params.user_id;
    let feedback = params.feedback;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/user/{user_id}/feedback",
        local_var_configuration.base_path,
        user_id = crate::apis::urlencode(user_id)
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
    local_var_req_builder = local_var_req_builder.json(&feedback);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UserFeedbackCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
