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
pub struct HkpMode {
    #[serde(rename = "deterrence", skip_serializing_if = "Option::is_none")]
    pub deterrence: Option<serde_json::Value>,
}

impl HkpMode {
    pub fn new() -> HkpMode {
        HkpMode { deterrence: None }
    }
}
