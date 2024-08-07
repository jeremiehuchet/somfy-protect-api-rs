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
pub struct InstallOptions {
    /// Support type of IntelliTAG.
    #[serde(rename = "tag_support_type", skip_serializing_if = "Option::is_none")]
    pub tag_support_type: Option<TagSupportType>,
    /// MAC address when installing a link.
    #[serde(rename = "mac", skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    /// Serial number when installing a HKP Gateway.
    #[serde(rename = "serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// Learning duration.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}

impl InstallOptions {
    pub fn new() -> InstallOptions {
        InstallOptions {
            tag_support_type: None,
            mac: None,
            serial: None,
            duration: None,
        }
    }
}
/// Support type of IntelliTAG.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TagSupportType {
    #[serde(rename = "externdoor")]
    Externdoor,
    #[serde(rename = "interndoor")]
    Interndoor,
    #[serde(rename = "window")]
    Window,
    #[serde(rename = "slidewindow")]
    Slidewindow,
    #[serde(rename = "slidedoor")]
    Slidedoor,
    #[serde(rename = "garage")]
    Garage,
    #[serde(rename = "other")]
    Other,
}

impl Default for TagSupportType {
    fn default() -> TagSupportType {
        Self::Externdoor
    }
}
