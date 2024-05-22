/*
 * Somfy Protect API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceOutput {
    /// The device unique identifier.
    #[serde(rename = "device_id")]
    pub device_id: String,
    /// The device box identifier.
    #[serde(rename = "box_id", skip_serializing_if = "Option::is_none")]
    pub box_id: Option<String>,
    /// The device site identifier.
    #[serde(rename = "site_id")]
    pub site_id: String,
    /// The device firmware version.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The device MAC address.
    #[serde(rename = "mac", skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(rename = "device_definition")]
    pub device_definition: Box<models::DeviceDefinition>,
    #[serde(rename = "status")]
    pub status: Box<models::DeviceStatus>,
    /// Device is into a subscription (uninstall will be forbidden).
    #[serde(rename = "into_subscription", skip_serializing_if = "Option::is_none")]
    pub into_subscription: Option<bool>,
    /// Camera has push to talk available.
    #[serde(
        rename = "push_to_talk_available",
        skip_serializing_if = "Option::is_none"
    )]
    pub push_to_talk_available: Option<bool>,
    /// Provider ID.
    #[serde(rename = "provider_id", skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[serde(rename = "update_available", skip_serializing_if = "Option::is_none")]
    pub update_available: Option<bool>,
    /// Is it master device.
    #[serde(rename = "master", skip_serializing_if = "Option::is_none")]
    pub master: Option<bool>,
    /// Somfy One Type.
    #[serde(rename = "somfy_one_type", skip_serializing_if = "Option::is_none")]
    pub somfy_one_type: Option<SomfyOneType>,
    /// Video Backend.
    #[serde(rename = "video_backend", skip_serializing_if = "Option::is_none")]
    pub video_backend: Option<String>,
    /// The device diagnosis.
    #[serde(rename = "diagnosis", skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<serde_json::Value>,
    #[serde(rename = "is_full_gsm", skip_serializing_if = "Option::is_none")]
    pub is_full_gsm: Option<bool>,
    /// The device label.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The room id.
    #[serde(rename = "roomId", skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Box<models::DeviceSettings>>,
}

impl DeviceOutput {
    pub fn new(
        device_id: String,
        site_id: String,
        device_definition: models::DeviceDefinition,
        status: models::DeviceStatus,
    ) -> DeviceOutput {
        DeviceOutput {
            device_id,
            box_id: None,
            site_id,
            version: None,
            mac: None,
            device_definition: Box::new(device_definition),
            status: Box::new(status),
            into_subscription: None,
            push_to_talk_available: None,
            provider_id: None,
            update_available: None,
            master: None,
            somfy_one_type: None,
            video_backend: None,
            diagnosis: None,
            is_full_gsm: None,
            label: None,
            room_id: None,
            settings: None,
        }
    }
}
/// Somfy One Type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SomfyOneType {
    #[serde(rename = "somfy_one")]
    One,
    #[serde(rename = "somfy_one_plus")]
    OnePlus,
}

impl Default for SomfyOneType {
    fn default() -> SomfyOneType {
        Self::One
    }
}
