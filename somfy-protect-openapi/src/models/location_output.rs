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
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationOutput {
    /// Is smart activation possible with fob.
    #[serde(
        rename = "fob_deactivation_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub fob_deactivation_status: Option<FobDeactivationStatus>,
    /// Latitude.
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f32>,
    /// Longitude.
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f32>,
    /// Trigger exit.
    #[serde(rename = "trigger_exit", skip_serializing_if = "Option::is_none")]
    pub trigger_exit: Option<bool>,
    /// Trigger exit if last.
    #[serde(
        rename = "trigger_exit_if_last",
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_exit_if_last: Option<bool>,
    /// Trigger enter.
    #[serde(rename = "trigger_enter", skip_serializing_if = "Option::is_none")]
    pub trigger_enter: Option<bool>,
    /// Mobile ID used for smart activation and presence. (nullable).
    #[serde(rename = "location_phone_id", skip_serializing_if = "Option::is_none")]
    pub location_phone_id: Option<String>,
}

impl LocationOutput {
    pub fn new() -> LocationOutput {
        LocationOutput {
            fob_deactivation_status: None,
            latitude: None,
            longitude: None,
            trigger_exit: None,
            trigger_exit_if_last: None,
            trigger_enter: None,
            location_phone_id: None,
        }
    }
}
/// Is smart activation possible with fob.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FobDeactivationStatus {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "missing.fob")]
    MissingPeriodFob,
    #[serde(rename = "missing.tag")]
    MissingPeriodTag,
}

impl Default for FobDeactivationStatus {
    fn default() -> FobDeactivationStatus {
        Self::Ok
    }
}
