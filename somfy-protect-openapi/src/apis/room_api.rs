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

/// struct for passing parameters to the method [`create_room`]
#[derive(Clone, Debug)]
pub struct CreateRoomParams {
    /// Site identifier
    pub site_id: String,
    /// Room
    pub room: models::RoomInput,
}

/// struct for passing parameters to the method [`delete_room`]
#[derive(Clone, Debug)]
pub struct DeleteRoomParams {
    /// Site identifier
    pub site_id: String,
    /// Room identifier
    pub room_id: String,
}

/// struct for passing parameters to the method [`list_rooms`]
#[derive(Clone, Debug)]
pub struct ListRoomsParams {
    /// Site identifier
    pub site_id: String,
}

/// struct for passing parameters to the method [`update_room`]
#[derive(Clone, Debug)]
pub struct UpdateRoomParams {
    /// Site identifier
    pub site_id: String,
    /// Room identifier
    pub room_id: String,
}

/// struct for typed errors of method [`create_room`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRoomError {
    Status400(models::ApiException),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_room`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRoomError {
    Status400(models::ApiException),
    Status404(models::ApiException),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_rooms`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRoomsError {
    Status400(models::ApiException),
    Status404(models::ApiException),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_room`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRoomError {
    Status400(models::ApiException),
    Status404(models::ApiException),
    UnknownValue(serde_json::Value),
}

pub async fn create_room(
    configuration: &configuration::Configuration,
    params: CreateRoomParams,
) -> Result<models::RoomOutput, Error<CreateRoomError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let room = params.room;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/room",
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
    local_var_req_builder = local_var_req_builder.json(&room);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateRoomError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_room(
    configuration: &configuration::Configuration,
    params: DeleteRoomParams,
) -> Result<(), Error<DeleteRoomError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let room_id = params.room_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/room/{room_id}",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id),
        room_id = crate::apis::urlencode(room_id)
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
        let local_var_entity: Option<DeleteRoomError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_rooms(
    configuration: &configuration::Configuration,
    params: ListRoomsParams,
) -> Result<models::RoomList, Error<ListRoomsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/room",
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
        let local_var_entity: Option<ListRoomsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_room(
    configuration: &configuration::Configuration,
    params: UpdateRoomParams,
) -> Result<models::RoomOutput, Error<UpdateRoomError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let room_id = params.room_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/room/{room_id}",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id),
        room_id = crate::apis::urlencode(room_id)
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
        let local_var_entity: Option<UpdateRoomError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
