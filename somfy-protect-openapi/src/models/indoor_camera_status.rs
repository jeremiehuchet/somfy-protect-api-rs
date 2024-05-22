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
pub struct IndoorCameraStatus {
    /// Power state.
    #[serde(rename = "power_state", skip_serializing_if = "Option::is_none")]
    pub power_state: Option<i32>,
    /// Wifi Level Percent.
    #[serde(rename = "wifi_level_percent", skip_serializing_if = "Option::is_none")]
    pub wifi_level_percent: Option<i32>,
    /// rLink Quality.
    #[serde(rename = "rlink_quality", skip_serializing_if = "Option::is_none")]
    pub rlink_quality: Option<i32>,
    /// rLink Quality Percent.
    #[serde(
        rename = "rlink_quality_percent",
        skip_serializing_if = "Option::is_none"
    )]
    pub rlink_quality_percent: Option<i32>,
    /// Last online status.
    #[serde(rename = "last_online_at", skip_serializing_if = "Option::is_none")]
    pub last_online_at: Option<String>,
    /// Last ofline status.
    #[serde(rename = "last_offline_at", skip_serializing_if = "Option::is_none")]
    pub last_offline_at: Option<String>,
    /// Shutter state.
    #[serde(rename = "shutter_state", skip_serializing_if = "Option::is_none")]
    pub shutter_state: Option<i32>,
    /// Last shutter opened.
    #[serde(
        rename = "last_shutter_opened_at",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_shutter_opened_at: Option<String>,
    /// Last shutter closed.
    #[serde(
        rename = "last_shutter_closed_at",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_shutter_closed_at: Option<String>,
    /// Power Mode.
    #[serde(rename = "power_mode", skip_serializing_if = "Option::is_none")]
    pub power_mode: Option<String>,
    /// Is device lost ? (Pir, Siren, OutdoorSiren, Remote, Tag, Extender).
    #[serde(rename = "device_lost", skip_serializing_if = "Option::is_none")]
    pub device_lost: Option<bool>,
    /// Last status date (Box, AIO, AIO+, Pir, Siren, OutdoorSiren, Remote, Tag, Extender).
    #[serde(rename = "last_status_at", skip_serializing_if = "Option::is_none")]
    pub last_status_at: Option<String>,
}

impl IndoorCameraStatus {
    pub fn new() -> IndoorCameraStatus {
        IndoorCameraStatus {
            power_state: None,
            wifi_level_percent: None,
            rlink_quality: None,
            rlink_quality_percent: None,
            last_online_at: None,
            last_offline_at: None,
            shutter_state: None,
            last_shutter_opened_at: None,
            last_shutter_closed_at: None,
            power_mode: None,
            device_lost: None,
            last_status_at: None,
        }
    }
}
